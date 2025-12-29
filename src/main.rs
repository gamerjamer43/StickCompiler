//! docs here soon i'm lazy as fuck
mod lexer;

// gotta work on this name but now im tired
use crate::lexer::{diagnostic::dump, lex::lex, token::Token};

use std::{env::{Args, args}, fs::read_to_string, iter::Skip, process::exit};

fn main() {
    // handle cl args
    let mut args: Skip<Args> = args().skip(1);
    let mut path: Option<String> = None;
    let mut debug: bool = false;
    while let Some(a) = args.next() {
        match a.as_str() {
            "-d" | "--debug" => debug = true,
            "--" => {
                if let Some(p) = args.next() { path = Some(p); }
                break;
            }

            // unknown args
            s if s.starts_with('-') => {
                eprintln!("unknown flag: {}", s);
                exit(2);
            }
            s => if path.is_none() { path = Some(s.to_string()) },
        }
    }

    let path = path.unwrap_or_else(|| {
        eprintln!("usage: lexer <file>");
        exit(2);
    });

    let src = read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("failed to read {path}: {e}");
        exit(2);
    });

    // TODO: parse the returned tokens into an AST
    let _tokens: Vec<Token<'_>> = match lex(&path, &src, debug) {
        Ok(tokens) => tokens,

        // any errors
        Err(errors) => {
            for d in &errors {
                eprintln!("{d}");
            }

            if debug {
                dump(&errors, "lastrun.log")
                    .unwrap_or_else(|_| eprintln!("Failed to dump errors."));
            }

            println!("\n(!) {} errors found.", errors.len());
            exit(0);
        }
    };

    // parsing down here.
}
