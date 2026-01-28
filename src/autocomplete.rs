use rustyline::completion::{Completer, Pair};
use rustyline::hint::Hinter;
use rustyline::highlight::Highlighter;
use rustyline::validate::Validator;
use rustyline::{Helper, Context, Result};

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
