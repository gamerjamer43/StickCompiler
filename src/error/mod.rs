pub mod errors;
pub mod diagnostic;

pub use errors::{SyntaxError, lex_err};
pub use diagnostic::{Diagnostic, dump};