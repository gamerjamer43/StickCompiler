pub mod errors;
pub mod diagnostic;

pub use errors::{SyntaxError, ParseError, lex_err};
pub use diagnostic::{Diagnostic, dump};