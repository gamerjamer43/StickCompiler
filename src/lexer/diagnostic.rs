use super::error::LexError;
use std::{
    fmt::{Display, Formatter, Result},
    fs::File,
    io::{self, BufWriter, Write},
    ops::Range,
};
use ariadne::{Label, Report, ReportKind, Source};
use strip_ansi_escapes::strip;

/// a structured way to print diagnostics, as i want to hide this from the world
/// - path = the file path, displayed in the error message
/// - src = the source file, to scan for the error message
/// - span = the range of chars the error lies in
/// - err = the accompanying LexError
///
/// - 'a the lifetime of this Diagnostic
/// - 'src the lifetime of the source file
pub struct Diagnostic<'a, 'src> {
    pub path: &'a str,
    pub src: &'src str,
    pub span: Range<usize>,
    pub err: LexError<'src>,
}

// so much fucking cleaner saves me a lot of pain
impl<'a, 'src> Display for Diagnostic<'a, 'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut buf = Vec::new();

        // main report with a short, human-friendly header
        Report::build(ReportKind::Error, self.path, self.span.start)
            .with_message(format!("lex error: {}", self.err)) // concise header
            // primary label points to the offending span
            .with_label(
                Label::new((self.path, self.span.clone()))
                    .with_message("error here")
            )
            // optional secondary label: show a little context before the span (if available)
            .with_label(
                Label::new((
                    self.path,
                    (self.span.start.saturating_sub(1)..self.span.start),
                ))
                .with_message("context")
            )
            .with_note("hint: check for unterminated string/char or an invalid token") // short hint
            .finish()
            .write((self.path, Source::from(self.src)), &mut buf)
            .unwrap();

        write!(f, "{}", String::from_utf8_lossy(&buf))
    }
}

// dump any found errors
pub fn dump(errors: &[Diagnostic<'_, '_>], path: &str) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for diag in errors {
        // render diagnostic (this includes ANSI color codes)
        let rendered = diag.to_string();

        // strip ANSI escape sequences
        let stripped = strip(rendered.as_bytes());

        // write clean UTF-8 (lossy is fine for logs)
        writeln!(writer, "{}", String::from_utf8_lossy(&stripped))?;
    }

    writer.flush()
}
