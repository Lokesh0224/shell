//Unix, standard output is identified by the number 1 => echo hello 1> file.txt === echo hello > file.txt

use std::fs::OpenOptions;
use std::io::Write;
use std::process::{Command, Stdio};

pub struct Redirection{
    pub clean_args: Vec<String>, 
    pub stdout_file: Option<String>,
    pub stderr_file: Option<String>,
}

impl Redirection{
    pub fn parse_streams(args: Vec<String>) -> Self{
        let mut clean_args = Vec::new();
        let mut stdout_file = None;
        let mut stderr_file = None;

        let mut i =0;
        while i <args.len() {
            match args[i].as_str() {
                ">" | "1>" =>{
                    if i+1 < args.len(){
                        stdout_file = Some(args[i+1].clone());
                        i+=2;
                    }else{
                        eprintln!("Syntax error: no file after >");
                        break;
                    }
                }, 

                "2>" => {
                    if i+1 < args.len(){
                        stderr_file = Some(args[i+1].clone());
                        i += 2;
                    }
                    else{
                        eprintln!("Syntax error: no file after 2>");
                        break;
                    }
                }

                _ => {
                    clean_args.push(args[i].clone());
                    i += 1;
                }
            }
            
            
        }

        Self {clean_args, stdout_file, stderr_file}
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
                }
        }
        else{
            println!("{}", text);
        }
    }

    //ex: echo hello > out.txt
    //Built-ins produce strings.
    pub fn write_builtin_err(&self, text: &str){
        if let Some(file) = &self.stderr_file{
            if let Ok(mut f) = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(file)
                {
                    writeln!(f, "{}", text).ok(); //here we know the output string, so we write directly into the file
                }
        }
        else{
            println!("{}", text);
        }
    }

    //ls > out.txt
    //External commands produce streams.
    pub fn apply_to_no_builtin(&self, cmd: &mut Command){
        //cmd represents a child process being prepared
        if let Some(file) = &self.stdout_file{
            if let Ok(f) = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(file)
                {
                    //but here we don't know what is the output string, so run the child process and then 
                    cmd.stdout(Stdio::from(f));               //When this command runs, instead of printing to the terminal, write its output into this file.
                }
        }

        if let Some(file) = &self.stderr_file{
            if let Ok(f) = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(file)
                {
                    cmd.stderr(Stdio::from(f));                 //When this program runs, anything it writes to stderr should go into this file instead of the terminal.
                }
        }
    }
}
