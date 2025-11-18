#[allow(unused_imports)] //Do not show warnings if some imports are not used.
use std::io::{self, Write};

fn main() {

    print!("$ ");
    io::stdout().flush().unwrap();

    //To get user input
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    println!("{}, Command not found!", command.trim());
}
