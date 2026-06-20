// Runtime I/O utilities for RBASIC (pure Rust implementation)

use std::io::{self, Write};

/// Reads a line from stdin, prints the optional prompt first.
/// Returns the trimmed line as a `String`.
pub fn read_line(prompt: Option<&str>) -> String {
    if let Some(p) = prompt {
        // Print prompt without a newline and flush stdout immediately.
        print!("{}", p);
        // Ensure the prompt appears before we block for input.
        let _ = io::stdout().flush();
    }
    let mut buffer = String::new();
    // Read until newline (or EOF). In case of error, panic – consistent with current runtime behaviour.
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read from stdin");
    buffer.trim_end().to_string()
}

/// Reads a `bool` from stdin, optionally displaying a prompt.
/// Interprets "TRUE" (case-insensitive) as true, anything else as false.
pub fn input_bool(prompt: Option<&str>) -> bool {
    let line = read_line(prompt);
    line.trim().eq_ignore_ascii_case("TRUE")
}

/// Reads an `i32` from stdin, optionally displaying a prompt.
/// Panics if the input cannot be parsed as `i32`.
pub fn input_i32(prompt: Option<&str>) -> i32 {
    let line = read_line(prompt);
    line.parse::<i32>()
        .expect("Failed to parse integer from INPUT statement")
}

/// Reads a `String` from stdin, optionally displaying a prompt.
/// The returned string does **not** contain the trailing newline.
pub fn input_string(prompt: Option<&str>) -> String {
    read_line(prompt)
}

/// Simple wrapper for printing any `Display` value, mirroring BASIC's PRINT.
pub fn print<T: std::fmt::Display>(value: T) {
    println!("{}", value);
}
