pub mod diagnostic;
pub mod errors;

pub use diagnostic::{Diagnostic, dump};
pub use errors::{ParseError, SyntaxError, lex_err};
