#[allow(unused_imports)] //Do not show warnings if some imports are not used.
use std::io::{self, Write};

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
                //.len() = no.of elements not the index
                if args.len() <2{ //if its just "type" and nothing else typed in terminal
                    println!("mention the command.");
                    continue;
                }

                let cmd = args[1];

                match cmd{
                    "echo" | "exit" | "type" => {
                        println!("{} is a shell builtin", cmd);
                    }, 
                    
                    _ => {
                        println!("{}: not found", cmd);
                    },
                }
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
