#[allow(unused_imports)] //Do not show warnings if some imports are not used.
use std::io::{self, Write};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path};
use std::process::Command;
use std::os::unix::process::CommandExt;

// Rustyline imports for version 17.x
use rustyline::error::ReadlineError;
use rustyline::Config;
use rustyline::config::CompletionType;

mod parser;
use parser::parse_input;

mod stdStreams;
use stdStreams::Redirection;

mod autocomplete;
use autocomplete::ShellCompleter;

fn main() -> std::io::Result<()> {
    let config = Config::builder()
                        .completion_type(CompletionType::List)
                        .build();

    // Create editor with helper directly
    let h = ShellCompleter;
    let mut rl = rustyline::Editor::with_config(config).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;
    rl.set_helper(Some(h)); 


    loop{
        // print!("$ ");
        // io::stdout().flush().unwrap();

        // // Wait for user input
        // let mut command = String::new();
        // //read_line will also consider /n character.
        // let bytes_read = io::stdin().read_line(&mut command).unwrap();

        // if bytes_read == 0{
        //     break;
        // }

        let readline = rl.readline("$ ");

        match readline {
            Ok(command) => {

                if command.trim().is_empty() {
                    continue;
                }
                //passing arguments to parser.rs
                let parsed = parse_input(command.trim());

                //when just enter is pressed
                if parsed.is_empty(){
                    continue;
                }

                //Handle stdout redirection
                let redir= Redirection::parse_streams(parsed);
                redir.prepare_redirections(); 
            
                // if command.trim() == "exit"{
                //     break;
                // }

                //let args: Vec<&str> = parsed.iter().map(|s| s.as_str()).collect();
                let args = &redir.clean_args;

                if args.is_empty() {
                    continue;
                }

                let zeroth = args[0].as_str();
                

                match zeroth {
                    "exit" => {
                        break
                    },

                    "echo" => {
                        let output = args[1..].join(" ");
                        redir.write_builtin_output(&output);
                        continue;
                    }, 

                    "type" => {
                        if args.len() < 2 {
                            redir.write_builtin_output("mention the command.");
                            continue;
                        }

                        let cmd = args[1].as_str();

                        // 1. Check builtins
                        if matches!(cmd, "echo" | "exit" | "type" | "pwd" | "cd"){
                            let msg = format!("{} is a shell builtin", cmd);
                            redir.write_builtin_output(&msg);
                            continue;
                        }

                        // 2. Search in PATH
                        //PATH is about where the OS keeps programs.
                        let mut found = false;
                        if let Ok(path_var) = env::var("PATH") {//will give the path of the command
                            for dir in path_var.split(':') {
                                let full_path = Path::new(dir).join(cmd);

                                if let Ok(metadata) = fs::metadata(&full_path) {
                                    // Check execute permission
                                    // 0o100 → owner execute
                                    // 0o010 → group execute
                                    // 0o001 → others execute

                                    if metadata.permissions().mode() & 0o111 != 0 {
                                        let msg = format!("{} is {}", cmd, full_path.display());
                                        redir.write_builtin_output(&msg);
                                        found = true;
                                        break;
                                    }
                                }
                            }
                        }

                        // 3. Not found
                        if !found {
                            let msg = format!("{}: not found", cmd);
                            redir.write_builtin_output(&msg);
                        }

                        
                        ////11.
        
                    }, 

                    "pwd" =>{
                        let path = env::current_dir()?;
                        let msg = path.display().to_string();
                        redir.write_builtin_output(&msg);
                        continue;
                    }, 
                    
                    //Absolute paths, like /usr/local/bin
                    //Relative paths, like ./, ../, ./dir
                    //The ~ (tilde) character
                    "cd" => {
                        if args.len() < 2 {
                            redir.write_builtin_err("cd: missing argument");
                            continue;
                        }

                        let path = Path::new(&args[1]);
                        let tilde = &args[1];

                        if tilde == "~"{
                            if let Ok(home) = env::var("HOME"){
                                env::set_current_dir(home)?;
                                continue;
                            }

                        }
                        
                        if path.is_dir() {
                            env::set_current_dir(&path)?;
                            continue;
                        }
                        else {
                            redir.write_builtin_err(&format!("{}: {}: No such file or directory", &args[0], &args[1]));
                            continue;
                        }
                        

                    }

                    _ => {
                        let mut found = false;
                        if let Ok(path_var) = env::var("PATH"){
                            for dir in path_var.split(":"){
                                let full_path = Path::new(dir).join(zeroth);

                                //0o111 everyone can execute the file
                                if let Ok(metadata) =  fs::metadata(&full_path){                //checks if the file exist in the path
                                    if metadata.permissions().mode() & 0o111 !=0{                         //checks if the file is executable
                                        let mut cmd = Command::new(full_path);                 //Prepare to execute the file /tmp/custom_exe_7592
                                        cmd.args(&args[1..]);                                               //are the arguments
                                        cmd.arg0(&zeroth);                                             //overrides the {argv[0](/tmp/custom_exe_7592)} to custom_exe_7592
                                                        //.status();      
                                                                                        
                                        redir.apply_to_no_builtin(&mut cmd);

                                        let _ = cmd.status();                                  //run it
                                        found = true;
                                        break;
                                    }
                                }
                            }
                        }

                        //this err is produced by the shell so we're using builtin err fn.
                        if !found {
                            redir.write_builtin_err(&format!("{}: command not found", zeroth));
                        }

                    },
                    
                }
            }, 

            Err(ReadlineError::Interrupted) => {
                // Ctrl-C was pressed
                continue;
            }, 

            Err(ReadlineError::Eof) => {
                // Ctrl-D was pressed
                break;
            }, 

            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            },
        }
        // if args[0] =="echo"{
        //     let output = args[1..].join(" ");
        //     println!("{}", output);
        //     continue;
        // }
    }
    Ok(())
    
}





//11.
// match cmd {
                //     "echo" | "exit" | "type" => {
                //         println!("{} is a shell builtin", cmd);
                //         continue;
                //     }, 

                    
                //     _ => {
                //         //PATH is about where the OS keeps programs.
                //         let mut found = false;
                //         if let Ok(path_var) = env::var("PATH") {//will give the path of the command
                //             for dir in path_var.split(':') {
                //                 let full_path = Path::new(dir).join(cmd);

                //                 if let Ok(metadata) = fs::metadata(&full_path) {
                //                     // Check execute permission
                //                     // 0o100 → owner execute
                //                     // 0o010 → group execute
                //                     // 0o001 → others execute

                //                     if metadata.permissions().mode() & 0o111 != 0 {
                //                         println!("{} is {}", cmd, full_path.display());
                //                         found = true;
                //                         break;
                //                     }
                //                 }
                //             }
                //         }

                //         // 3. Not found
                //         if !found {
                //             println!("{}: not found", cmd);
                //         }
                //         continue;
                //         }
                // }