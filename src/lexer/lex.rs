use super::{
    diagnostic::{Diagnostic, dump},
    token::Token,
};
use logos::{Lexer, Logos};
use std::{ops::Range, result::Result, time::Instant};

pub fn lex<'a>(
    path: &'a str,
    src: &'a str,
    debug: bool,
) -> Result<Vec<Token<'a>>, Vec<Diagnostic<'a, 'a>>> {
    let mut errors: Vec<Diagnostic> = vec![];
    let mut tokens: Vec<Token<'_>> = vec![];
    let start: Instant = Instant::now();

    let linecount: usize = src.lines().count();
    let mut lex: Lexer<'_, Token> = Token::lexer(&src);
    while let Some(res) = lex.next() {
        match res {
            Ok(tok) => {
                if debug == false {
                    tokens.push(tok);
                    continue;
                }

                // print token specs for debug (cargo fmt is working against me)
                let span: Range<usize> = lex.span();
                let slice: &str = lex.slice();
                println!(
                    "[{:>6}..{:>6}] {:<18} {:?}",
                    span.start,
                    span.end,
                    format!("{tok:?}"),
                    slice
                );

                // THEN push
                tokens.push(tok);
            }

            // any errors have types in the LexError enum, Unknown by default
            Err(e) => {
                let span: Range<usize> = lex.span();
                let diagnostic: Diagnostic<'_, '_> = Diagnostic {
                    path: &path,
                    src: &src,
                    span: span.start..span.end,
                    err: e,
                };
                errors.push(diagnostic);
            }
        }
    }

    // dump errors
    let errorcount: usize = errors.len();
    if debug {
        println!(
            "Lexed {} bytes, {linecount} lines into {} tokens. Took {}s.",
            src.len(),
            tokens.len(),
            start.elapsed().as_secs_f64()
        );

        // dump to log
        if !errors.is_empty() {
            dump(&errors, "lastrun.log").unwrap_or_else(|_| eprintln!("Failed to dump errors."));
            println!("Dumped all errors to a log file.");
        }
    }

    // result handling
    if errorcount == 0 {
        Ok(tokens)
    } else {
        Err(errors)
    }
}
