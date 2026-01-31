use rustyline::completion::{Completer, Pair};
use rustyline::hint::Hinter;
use rustyline::highlight::Highlighter;
use rustyline::validate::Validator;
use rustyline::{Helper, Context, Result};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::collections::HashSet;


pub struct ShellCompleter;

impl ShellCompleter {
    // Helper function to get all matching candidates
    pub fn get_candidates(&self, word: &str) -> Vec<String> {
        let mut candidates = HashSet::new();    
        // Check builtins
        let builtins = ["echo", "exit"];
        for builtin in builtins {
            if builtin.starts_with(word) && builtin != word {
                candidates.insert(builtin.to_string());
            }
        }
        
        // Check PATH for executables
        if let Ok(path_var) = env::var("PATH") {
            for dir in path_var.split(':') {
                if let Ok(entries) = fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        if let Ok(file_name) = entry.file_name().into_string() {
                            if file_name.starts_with(word) && file_name != word {
                                // Check if executable
                                if let Ok(metadata) = entry.metadata() {
                                    if metadata.permissions().mode() & 0o111 != 0 {
                                        candidates.insert(file_name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        let mut result: Vec<String> = candidates.into_iter().collect();
        result.sort();
        result
    }
}


impl Completer for ShellCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> Result<(usize, Vec<Pair>)> {
        // Get the word being completed (before cursor)
        let word = &line[..pos];
        
        // Only complete if it's the first word (no spaces before)
        if word.contains(' ') {
            return Ok((pos, vec![]));
        }
        
        let mut candidates = self.get_candidates(word);
        
        let pairs: Vec<Pair> = candidates
            .into_iter()
            .map(|c| Pair {
                display: c.clone(),
                replacement: format!("{} ", c),
            })
            .collect();
        
        Ok((0, pairs)) //(start, matches)
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
