//Unix, standard output is identified by the number 1 => echo hello 1> file.txt === echo hello > file.txt

use std::fs::OpenOptions;
use std::io::Write;
use std::process::{Command, Stdio};

pub struct Redirection{
    pub clean_args: Vec<String>, 
    pub stdout_file: Option<String>,
}

impl Redirection{
    pub fn parse_streams(args: Vec<String>) -> Self{
        let mut clean_args = Vec::new();
        let mut stdout_file = None;

        let mut i =0;
        while i <args.len() {
            if args[i] == ">" || args[i] == "1>" {
                if i+1 < args.len(){
                    stdout_file = Some(args[i+1].clone());
                    i += 2;                                    //Skip BOTH the operator AND the filename, because we already handled them.
                } else {
                    eprintln!("syntax error: no file after >");
                    break;
                }
            } else {
                clean_args.push(args[i].clone());
                i += 1;
            }
        }

        Self {clean_args, stdout_file}
    }

    //echo hello > output.txt
    pub fn write_builtin_output(&self, text: &str){
        if let Some(file) = &self.stdout_file{
            if let Ok(mut f) = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(file)
                {
                    writeln!(f, "{}", text).ok();
                }else{
                    println!("{}", text);
                }
        }
    }

    pub fn apply_to_no_builtin(&self, cmd: &mut Command){
        if let Some(file) = &self.stdout_file{
            if let Ok(f) = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(file)
                {
                    cmd.stdout(Stdio::from(f));               //When this command runs, instead of printing to the terminal, write its output into this file.
                }
        }
    }
}
