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
    fastfail: bool,
) -> Result<Vec<Token<'a>>, Vec<Diagnostic<'a, 'a>>> {
    let mut errors: Vec<Diagnostic> = vec![];
    let mut tokens: Vec<Token<'_>> = vec![];
    let start: Instant = Instant::now();

    let linecount: usize = src.lines().count();
    let mut lex: Lexer<'_, Token> = Token::lexer(src);
    while let Some(res) = lex.next() {
        match res {
            Ok(tok) => tokens.push(tok),

            // any errors have types in the SyntaxError enum, Unknown by default
            Err(err) => {
                let span: Range<usize> = lex.span();
                let diagnostic: Diagnostic<'_, '_> = Diagnostic {
                    path,
                    src,
                    span,
                    err,
                };
                errors.push(diagnostic);

                // fail immediately on ff
                if fastfail {
                    break;
                }
                continue;
            }
        };
    }

    // handle debug prints
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

    // any errors stop at the lexing stage
    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors)
    }
}
