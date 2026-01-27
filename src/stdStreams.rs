//Unix, standard output is identified by the number 1 => echo hello 1> file.txt === echo hello > file.txt

use std::fs::OpenOptions;
use std::io::Write;
use std::process::{Command, Stdio};

pub struct Redirection{
    pub clean_args: Vec<String>, 
    pub stdout_file: Option<String>,
    pub stderr_file: Option<String>,
    pub stdout_append: bool, 
    //pub stdapnderr_file: Option<String>,
}

impl Redirection{
    pub fn parse_streams(args: Vec<String>) -> Self{
        let mut clean_args = Vec::new();
        let mut stdout_file = None;
        let mut stderr_file = None;
        let mut stdout_append = false;
        //let mut stdapnderr_file = None;

        let mut i =0;
        while i <args.len() {
            match args[i].as_str() {
                ">" | "1>" =>{
                    if i+1 < args.len(){
                        stdout_file = Some(args[i+1].clone());
                        stdout_append = false;
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
                }, 

                ">>" | "1>>" => {
                    if i+1 <args.len(){
                        stdout_file = Some(args[i+1].clone());
                        stdout_append = true;

                        i += 2;
                    }
                },

                // "2>>" => {
                //     if i+1 <args.len(){
                //         stdapnderr_file = Some(args[i+1].clone());
                //         i += 2;
                //     }
                // }

                _ => {
                    clean_args.push(args[i].clone());
                    i += 1;
                }
            }
            
            
        }

        Self {clean_args, stdout_file, stderr_file, stdout_append}
    }

    pub fn prepare_redirections(&self){
        if let Some(file) = &self.stdout_file{
            let mut opts = OpenOptions::new();
            opts.create(true).write(true);

            if self.stdout_append{
                opts.append(true);
            }
            else{
                opts.truncate(true);
            }

            let _ = opts.open(file);
        }

        if let Some(file) = &self.stderr_file{
            let _ = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(file);
        }
    }

    //echo hello > output.txt
    pub fn write_builtin_output(&self, text: &str){
        if let Some(file) = &self.stdout_file{
            let mut opts = OpenOptions::new();
            opts.append(true); // always append now (file already truncated if needed)
            if let Ok(mut f) = opts.open(file){
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
            eprintln!("{}", text);
        }
    }

    //ls > out.txt
    //External commands produce streams.
    pub fn apply_to_no_builtin(&self, cmd: &mut Command){
        //cmd represents a child process being prepared
        if let Some(file) = &self.stdout_file{
            let mut opts = OpenOptions::new();
            opts.write(true).create(true);

            if self.stdout_append {
                opts.append(true);
            } else {
                opts.truncate(true);
            }

            if let Ok(f) = opts.open(file) {
                cmd.stdout(Stdio::from(f));
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
