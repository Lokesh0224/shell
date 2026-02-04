use std::process::{Command, Output};
use std::os::unix::process::CommandExt;
use nix::unistd::{pipe, fork, ForkResult, close, dup2};
use nix::sys::wait::waitpid;
use std::env;
use std::path::Path;
use std::fs;
use std::os::unix::fs::PermissionsExt;


pub fn execute_pipeline(commands: Vec<Vec<String>>) -> std::io::Result<()> {
    if commands.is_empty() {
        return Ok(());
    }
    
    // For a single command (no pipe), execute normally
    if commands.len() == 1 {
        execute_single_command(&commands[0])?;
        return Ok(());
    }
    
    // For two commands with a pipe
    // if commands.len() == 2 {
    //     execute_two_command_pipeline(&commands[0], &commands[1])?;
    //     return Ok(());
    // }

    execute_multi_command_pipeline(&commands)?;
    
    Ok(())
}

fn is_builtin(cmd: &str) -> bool {
    matches!(cmd, "echo" | "exit" | "type" | "pwd" | "cd" | "history")
}

fn find_executable(cmd: &str) -> Option<String> {
    if cmd.contains('/') {
        if Path::new(cmd).exists() {
            return Some(cmd.to_string());
        }
        return None;
    }
    
    if let Ok(path_var) = env::var("PATH") {
        for dir in path_var.split(':') {
            let full_path = Path::new(dir).join(cmd);
            if let Ok(metadata) = fs::metadata(&full_path) {
                if metadata.permissions().mode() & 0o111 != 0 {
                    return Some(full_path.to_string_lossy().to_string());
                }
            }
        }
    }
    
    None
}


//to execute the builtins 
fn execute_builtin(args: &[String]) -> String {
    if args.is_empty() {
        return String::new();
    }
    
    match args[0].as_str() {
        "echo" => {
            args[1..].join(" ")
        },
        "pwd" => {
            if let Ok(path) = std::env::current_dir() {
                path.display().to_string()
            } else {
                String::new()
            }
        },
        "type" => {
            if args.len() < 2 {
                return "mention the command.".to_string();
            }
            
            let cmd = &args[1];
            
            // Check if it's a builtin
            if is_builtin(cmd) {
                return format!("{} is a shell builtin", cmd);
            }
            
            // Search in PATH
            if let Some(path) = find_executable(cmd) {
                return format!("{} is {}", cmd, path);
            }
            
            format!("{}: not found", cmd)
        },

        // "history" => {

        // },
        _ => String::new()
    }
}

fn execute_single_command(args: &[String]) -> std::io::Result<()> {
    if args.is_empty() {
        return Ok(());
    }

    if is_builtin(&args[0]){
        let output = execute_builtin(args);
        if !output.is_empty(){
            println!("{}", output);
        }
        return Ok(());
    }
    
    if let Some(executable) = find_executable(&args[0]){
        let mut cmd = Command::new(executable);//creates a new process
        if args.len() > 1 { //this coluld be possibly none, we already did a check
            cmd.args(&args[1..]);
        }
        cmd.arg0(&args[0]);
        cmd.status()?;//run it
    } else{
        eprintln!("{}: command not found", args[0]);
    }
    Ok(())
}


//to better understand go through execute_two_command_pipeline fn

//now multi-command pipeline

// ┌──────┐ pipe1 ┌──────┐ pipe2 ┌──────┐
// │ cat  │──────>│ head │──────>│  wc  │──────> Terminal
// └──────┘       └──────┘       └──────┘

// Each command (except first and last) needs:
// stdin from the previous pipe
// stdout to the next pipe

// N commands:
//      N-1 pipes
//      N child processes (one for each command)


// cmd1 | cmd2 | cmd3

// Pipes needed: 2
// - pipe0: connects cmd1 → cmd2
// - pipe1: connects cmd2 → cmd3

// Processes:
// - Child 0: cmd1, stdout → pipe0[write]
// - Child 1: cmd2, stdin ← pipe0[read], stdout → pipe1[write]
// - Child 2: cmd3, stdin ← pipe1[read]

fn execute_multi_command_pipeline(commands: &[Vec<String>]) -> std::io::Result<()> {
    let num_commands = commands.len();

    if num_commands == 0 {
        return Ok(());
    }

    // Create all pipes (N-1 pipes for N commands)
    let mut pipes = Vec::new();
    for _ in 0..(num_commands -1){
        let (read_fd, write_fd) = pipe().map_err(|e|{
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;
        pipes.push((read_fd, write_fd));//pushing fd along with index
    }

    let mut children = Vec::new();

    //fork a process for each command
    //iter through these commands 0, 1, 2 (&Vec<String>)
    // [
    // ["ls", "-l"],
    // ["grep", "foo"],
    // ["sort"]
    // ]
    for (i, cmd_args) in commands.iter().enumerate(){
        if cmd_args.is_empty(){
            continue;
        }

        match unsafe { fork() }{
            Ok(ForkResult::Child) => {
                // Set up stdin from previous pipe
                if i > 0 {
                    let (prev_read_fd, prev_write_fd) = pipes[i-1];
                    close(prev_write_fd).ok();
                    dup2(prev_read_fd, 0).ok(); //0=stdin
                    close(prev_read_fd).ok();
                }

                // Set up stdout to next pipe
                if i < num_commands -1{
                    let (next_read_fd, next_write_fd) = pipes[i];
                    close(next_read_fd).ok();
                    dup2(next_write_fd, 1).ok();//1=stdout
                    close(next_write_fd).ok();
                }

                // Close all remaining pipes
                for (read_fd, write_fd) in &pipes {
                    close(*read_fd).ok();
                    close(*write_fd).ok();
                }


                // Execute command
                let is_builtin_cmd = is_builtin(&cmd_args[0]);
                
                if is_builtin_cmd {
                    let output = execute_builtin(cmd_args);
                    println!("{}", output);
                    std::process::exit(0);
                } else {
                    if let Some(executable) = find_executable(&cmd_args[0]) {
                        let mut cmd = Command::new(&executable);
                        if cmd_args.len() > 1 {
                            cmd.args(&cmd_args[1..]);
                        }
                        cmd.arg0(&cmd_args[0]);
                        cmd.exec();
                    }
                    eprintln!("{}: command not found", cmd_args[0]);
                    std::process::exit(1);
                }
            }

            Ok(ForkResult::Parent { child }) => {
                children.push(child);
            }
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
            }
        }
    }


    // Parent closes all pipes
    for (read_fd, write_fd) in pipes {
        close(read_fd).ok();
        close(write_fd).ok();
    }
    
    // Wait for all children
    for child in children {
        waitpid(child, None).ok();
    }
    
    Ok(())
}


// fn execute_two_command_pipeline(cmd1_args: &[String], cmd2_args: &[String]) -> std::io::Result<()> {
    
    
//     if cmd1_args.is_empty() || cmd2_args.is_empty() {
//         return Ok(());
//     }

//     let cmd1_is_builtin = is_builtin(&cmd1_args[0]);
//     let cmd2_is_builtin = is_builtin(&cmd2_args[0]);
    
//     // 1.Create a pipe 
//     //command1 --> (stdout) ----> [PIPE] ----> (stdin) --> command2
//     //       write_fd  ----> [PIPE] ----> read_fd
//     let (read_fd, write_fd) = pipe().map_err(|e| {//pipe is a byte stream between two process
//         std::io::Error::new(std::io::ErrorKind::Other, e)
//     })?;
    
//     // Fork first command
//     match unsafe { fork() } { //Parent, Child
//         Ok(ForkResult::Child) => {
//             // In child process for cmd1
//             // Close read end (initially we need to write)
//             nix::unistd::close(read_fd).ok();
            
//             // Redirect stdout to write end of pipe
//             nix::unistd::dup2(write_fd, 1).ok(); // 1 = stdout
//             nix::unistd::close(write_fd).ok();
            
//             if cmd1_is_builtin {
//                 let output = execute_builtin(cmd1_args);
//                 println!("{}", output);
//                 std::process::exit(0); //code 0 means success
//             }else{
//                 if let Some(executable) = find_executable(&cmd1_args[0]){
//                     // Execute command
//                     let mut cmd = Command::new(&executable);
//                     if cmd1_args.len() > 1 {
//                         cmd.args(&cmd1_args[1..]);
//                     }
//                     cmd.arg0(&cmd1_args[0]);
//                     //replaces the current process (this child created by fork) with the new program
//                     cmd.exec(); 
//                 }
//             }
            
//             std::process::exit(1); // Should never reach here means failure
//         }
//         Ok(ForkResult::Parent { child: child1 }) => {
//             // Fork second command
//             match unsafe { fork() } {//Parent, Child
//                 Ok(ForkResult::Child) => {
//                     // In child process for cmd2
//                     // Close write end (we only read)
//                     nix::unistd::close(write_fd).ok();
                    
//                     // Redirect stdin to read end of pipe
//                     nix::unistd::dup2(read_fd, 0).ok(); // 0 = stdin
//                     nix::unistd::close(read_fd).ok();
                    
//                     if cmd2_is_builtin{
//                         let output = execute_builtin(cmd2_args);
//                         println!("{}", output);
//                         std::process::exit(0);
//                     }else{
//                         if let Some(executable) = find_executable(&cmd2_args[0]){
//                             // Execute command
//                             let mut cmd = Command::new(&cmd2_args[0]); //new process 
//                             if cmd2_args.len() > 1 {
//                                 cmd.args(&cmd2_args[1..]);
//                             }
//                             cmd.arg0(&cmd2_args[0]);
//                             //replaces the current process (this child created by fork) with the new program 
//                             cmd.exec(); //maps to the OS execvp call
//                         }
//                     }
                    
//                     //safety brake so a failed exec() child doesn’t accidentally keep running your shell logic.
//                     std::process::exit(1);
//                 }
//                 Ok(ForkResult::Parent { child: child2 }) => {
                    
//                     // In parent process
//                     // Close both ends of pipe
//                     nix::unistd::close(read_fd).ok();
//                     nix::unistd::close(write_fd).ok();
                    
//                     // Wait for both children
//                     waitpid(child1, None).ok();
//                     waitpid(child2, None).ok();
//                 }
//                 Err(e) => {
//                     return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
//                 }
//             }
//         }
//         Err(e) => {
//             return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
//         }
//     }
    
//     Ok(())
// }

//          KERNEL PIPE BUFFER
//      ┌────────────────────────┐
// cmd1 │ write_fd →  [ bytes ]  │ read_fd → cmd2
//      └────────────────────────┘

// | Step                | What happens                               |
// | ------------------- | ------------------------------------------ |
// | `pipe()`            | Kernel creates shared buffer               |
// | `dup2(write_fd, 1)` | cmd1 writes into that buffer               |
// | `dup2(read_fd, 0)`  | cmd2 reads from that buffer                |
// | `exec()`            | Programs start using stdin/stdout normally |


//so the cmd1 and cmd2 exactly meets after both dup2 calls have executed.