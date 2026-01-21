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
        println!("{}: command not found", command.trim());
    }
    
}


//if exit, terminate the shell immediately 