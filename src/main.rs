#[allow(unused_imports)] //Do not show warnings if some imports are not used.
use std::io::{self, Write};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use std::os::unix::process::CommandExt;


fn main() -> std::io::Result<()> {
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
                if matches!(cmd, "echo" | "exit" | "type" | "pwd"){
                    println!("{} is a shell builtin", cmd);
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
                                println!("{} is {}", cmd, full_path.display());
                                found = true;
                                break;
                            }
                        }
                    }
                }

                // 3. Not found
                if !found {
                    println!("{}: not found", cmd);
                }

                
                ////11.
 
            }, 

            "pwd" =>{
                let path = env::current_dir()?;
                // println!("{}", path.display());
            }

            _ => {
                let mut found = false;
                if let Ok(path_var) = env::var("PATH"){
                    for dir in path_var.split(":"){
                        let full_path = Path::new(dir).join(zeroth);

                        //0o111 everyone can execute the file
                        if let Ok(metadata) =  fs::metadata(&full_path){                //checks if the file exist in the path
                            if metadata.permissions().mode() & 0o111 !=0{                         //checks if the file is executable
                                let _ = Command::new(&full_path)                  //Prepare to execute the file /tmp/custom_exe_7592
                                                .args(&args[1..])                     //are the arguments
                                                .arg0(zeroth)                    //overrides the {argv[0](/tmp/custom_exe_7592)} to custom_exe_7592
                                                .status();                                        //run it

                                found = true;
                                break;
                            }
                        }
                    }
                }

                if !found {
                    println!("{}: command not found", zeroth);
                }

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