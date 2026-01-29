use rustyline::completion::{Completer, Pair};
use rustyline::hint::Hinter;
use rustyline::highlight::Highlighter;
use rustyline::validate::Validator;
use rustyline::{Helper, Context, Result};
use std::env;
use std::fs;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

pub struct ShellCompleter;

impl Completer for ShellCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        // Get the word being completed (before cursor)
        let word = &line[..pos];
        
        // Only complete if it's the first word (no spaces before)
        if word.contains(' ') {
            return Ok((pos, vec![]));
        }
        
        let builtins = ["echo", "exit"];
        let mut candidates = Vec::new();
        
        for builtin in builtins {
            if builtin.starts_with(word) {
                candidates.push(Pair {
                    display: builtin.to_string(),
                    replacement: format!("{} ", builtin),
                });
            }
        }

        if let Ok(path_var) = env::var("PATH") {
            for dir in path_var.split(':') {
                
                if let Ok(entries) = fs::read_dir(dir) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let file_name = entry.file_name();
                            
                            if let Some(name) = file_name.to_str() {
                                if name.starts_with(word) && name != word {
                                    
                                    
                                    if let Ok(metadata) = entry.metadata() {
                                        if metadata.permissions().mode() & 0o111 != 0 {
                                            candidates.push(Pair {
                                                display: name.to_string(),
                                                replacement: name.to_string(),
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
        }
    }   
        
        Ok((0, candidates))
    }
}

impl Helper for ShellCompleter {}

impl Hinter for ShellCompleter {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}

impl Highlighter for ShellCompleter {}

impl Validator for ShellCompleter {}
