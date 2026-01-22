#[allow(unused_imports)] //Do not show warnings if some imports are not used.
use std::io::{self, Write};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;


fn main() {
    loop{
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut command = String::new();
        //read_line will also consider /n character.
        let bytes_read = io::stdin().read_line(&mut command).unwrap();

        if bytes_read == 0{
            break;
        }
         
        if command.trim() == "exit"{
            break;
        }

        let args: Vec<&str> = command.split_whitespace().collect();

        if args.is_empty(){
            continue;
        }

        let zeroth = args[0];

        match zeroth {
            "echo" => {
                let output = args[1..].join(" ");
                println!("{}", output);
                continue;
            }, 

            "type" => {
                if args.len() < 2 {
                    println!("mention the command.");
                    continue;
                }

                let cmd = args[1];

                // 1. Check builtins
                match cmd {
                    "echo" | "exit" | "type" => {
                        println!("{} is a shell builtin", cmd);
                        continue;
                    }
                    _ => {}
                }

                // 2. Search in PATH
                //PATH is about where the OS keeps programs.
                if let Ok(path_var) = env::var("PATH") {//will give the path of the command
                    for dir in path_var.split(':') {
                        let full_path = Path::new(dir).join(cmd);

                        if let Ok(metadata) = fs::metadata(&full_path) {
                            // Check execute permission
                            if metadata.permissions().mode() & 0o111 != 0 {
                                println!("{} is {}", cmd, full_path.display());
                                continue;
                            }
                        }
                    }
                }

                // 3. Not found
                println!("{}: not found", cmd);
                continue;
            }, 

            _ => {},
            
        }

        // if args[0] =="echo"{
        //     let output = args[1..].join(" ");
        //     println!("{}", output);
        //     continue;
        // }
        
        println!("{}: command not found", command.trim());
    }
    
}