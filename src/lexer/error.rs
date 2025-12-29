use std::fmt::{Display, Formatter, Result};

use super::token::Token;
use logos::Lexer;

// originally borrowed from https://docs.rs/logos/latest/logos/struct.Lexer.html
/// a generic error for anything that may happen during lexing.
#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexError<'src> {
    UnterminatedString(&'src str),
    UnterminatedChar(&'src str),
    UnknownToken(&'src str),

    #[default]
    Unknown,
}

impl<'src> Display for LexError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            // &&str confused me. but we borrow on lines 10/11, and because of that the match borrows again.
            // this shit just double referencing
            LexError::UnterminatedString(s) => write!(
                f, "\x1b[1mUnterminatedString:\x1b[22m Strings must be properly terminated. Afflicted: {s}"
            ),
            LexError::UnterminatedChar(s) => write!(
                f, "\x1b[1mUnterminatedChar:\x1b[22m Chars must be properly terminated. Afflicted: {s}"
            ),
            LexError::UnknownToken(s) => write!(
                f, "\x1b[1mUnknownToken:\x1b[22m The character '{s}' is not in the grammar for this language."
            ),

            // catchall == unknown
            LexError::Unknown => write!(f, "unknown lexer error"),
        }
    }
}

/// a general error callback that will be done on any error that happens in lexing,
/// pulling from LexError (rn just untermed "" or '', but there may be a couple others).
/// most errors will throw in parsing, but some will have to throw here
///
/// # Basic Usage
/// when a lexer error occurs, this will also be fired with it (attached to Logos).
/// this will match an error to one of the following or unknown (default).
///
/// # Returns
/// an option from the LexError enum if matched, or unknown by default
pub fn err<'src>(lex: &mut Lexer<'src, Token<'src>>) -> LexError<'src> {
    let slice: &str = lex.slice();
    match slice.as_bytes().first() {
        Some(b'"') => LexError::UnterminatedString(slice),
        Some(b'\'') => LexError::UnterminatedChar(slice),
        Some(_) => LexError::UnknownToken(slice),

        // catch all in case none matched
        None => LexError::Unknown,
    }
}
