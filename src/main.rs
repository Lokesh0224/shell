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

        if args[0] =="echo"{
            let output = args[1..].join(" ");
            println!("{}", output);
            continue;
        }
        
        println!("{}: command not found", command.trim());
    }
    
}


//implement echo => print the output with spaces and /n char at the end