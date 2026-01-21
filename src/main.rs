#[allow(unused_imports)] //Do not show warnings if some imports are not used.
use std::io::{self, Write};

fn main() {
    loop{
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        println!("{}: command not found", command.trim());
    }
    
}


// display $ 
//parse the exe command
//Display the error
//once again display $