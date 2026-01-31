use std::process::{Command, Stdio};
use std::os::unix::process::CommandExt;
use std::os::unix::io::{AsRawFd, FromRawFd};

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
    if commands.len() == 2 {
        execute_two_command_pipeline(&commands[0], &commands[1])?;
        return Ok(());
    }
    
    Ok(())
}

fn execute_single_command(args: &[String]) -> std::io::Result<()> {
    if args.is_empty() {
        return Ok(());
    }
    
    let mut cmd = Command::new(&args[0]);//creates a new process
    if args.len() > 1 { //this coluld be possibly none, we already did a check
        cmd.args(&args[1..]);
    }
    cmd.status()?;//run it
    Ok(())
}

fn execute_two_command_pipeline(cmd1_args: &[String], cmd2_args: &[String]) -> std::io::Result<()> {
    use nix::unistd::{pipe, fork, ForkResult};
    use nix::sys::wait::waitpid;
    
    if cmd1_args.is_empty() || cmd2_args.is_empty() {
        return Ok(());
    }
    
    // Create a pipe
    let (read_fd, write_fd) = pipe().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;
    
    // Fork first command
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            // In child process for cmd1
            // Close read end (we only write)
            nix::unistd::close(read_fd).ok();
            
            // Redirect stdout to write end of pipe
            nix::unistd::dup2(write_fd, 1).ok(); // 1 = stdout
            nix::unistd::close(write_fd).ok();
            
            // Execute command
            let mut cmd = Command::new(&cmd1_args[0]);
            if cmd1_args.len() > 1 {
                cmd.args(&cmd1_args[1..]);
            }
            cmd.exec(); // This replaces the process
            
            std::process::exit(1); // Should never reach here
        }
        Ok(ForkResult::Parent { child: child1 }) => {
            // Fork second command
            match unsafe { fork() } {
                Ok(ForkResult::Child) => {
                    // In child process for cmd2
                    // Close write end (we only read)
                    nix::unistd::close(write_fd).ok();
                    
                    // Redirect stdin to read end of pipe
                    nix::unistd::dup2(read_fd, 0).ok(); // 0 = stdin
                    nix::unistd::close(read_fd).ok();
                    
                    // Execute command
                    let mut cmd = Command::new(&cmd2_args[0]);
                    if cmd2_args.len() > 1 {
                        cmd.args(&cmd2_args[1..]);
                    }
                    cmd.exec();
                    
                    std::process::exit(1);
                }
                Ok(ForkResult::Parent { child: child2 }) => {
                    // In parent process
                    // Close both ends of pipe
                    nix::unistd::close(read_fd).ok();
                    nix::unistd::close(write_fd).ok();
                    
                    // Wait for both children
                    waitpid(child1, None).ok();
                    waitpid(child2, None).ok();
                }
                Err(e) => {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
                }
            }
        }
        Err(e) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    }
    
    Ok(())
}