use crate::lexer::token::{LexError, LexErrorCode};

/// Converts a byte offset in `source` to a 1-based (line, column) pair.
pub fn offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut col = 1usize;
    for (i, c) in source.char_indices() {
        if i >= offset {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

fn source_line(source: &str, offset: usize) -> &str {
    let line_start = source[..offset].rfind('\n').map(|p| p + 1).unwrap_or(0);
    let line_end = source[offset..]
        .find('\n')
        .map(|p| p + offset)
        .unwrap_or(source.len());
    &source[line_start..line_end]
}

fn lex_code_str(code: &LexErrorCode) -> &'static str {
    match code {
        LexErrorCode::InvalidChar => "L001",
        LexErrorCode::UnterminatedString => "L002",
        LexErrorCode::InvalidNumber => "L003",
    }
}

/// Formats a `LexError` as a human-readable diagnostic with file, line, column,
/// source snippet, and caret (RFC-0002 §17).
pub fn format_lex_error(error: &LexError, source: &str, file: &str) -> String {
    let (line, col) = offset_to_line_col(source, error.span.start);
    let code = lex_code_str(&error.code);
    let line_text = source_line(source, error.span.start);

    let span_end = error.span.end.min(source.len());
    let caret_len = source[error.span.start..span_end].chars().count().max(1);
    let caret = format!(
        "{}{}",
        " ".repeat(col.saturating_sub(1)),
        "^".repeat(caret_len)
    );

    let line_prefix = format!("{:>4} | ", line);
    let blank_prefix = format!("{:>4}   ", "");

    format!(
        "error[{code}] {file}:{line}:{col}: {msg}\n  {lp}{lt}\n  {bp}{caret}",
        code = code,
        file = file,
        line = line,
        col = col,
        msg = error.message,
        lp = line_prefix,
        lt = line_text,
        bp = blank_prefix,
        caret = caret,
    )
}
