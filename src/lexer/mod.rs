pub mod token;

use token::{Span, Token, TokenKind};

/// Very simple lexer sufficient for the current test suite.
/// It tokenises identifiers, integer literals, keywords, and the
/// punctuation required by the parser (colon, comma, parentheses,
/// arithmetic operators, assignment, comparison operators and the
/// `END`/`IF`/`WHILE`/`FUNCTION`/`LET`/`PRINT`/`RETURN` keywords).
///
/// The implementation is deliberately straightforward – it scans the
/// input byte‑by‑byte, builds tokens and records their span (start‑
/// inclusive, end‑exclusive). Only the token kinds needed by the
/// parser are produced; any unrecognised character is ignored.
pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();
    while let Some((start, ch)) = chars.next() {
        // Skip whitespace
        if ch.is_whitespace() {
            continue;
        }
        // Single-line comment: skip everything until end of line
        if ch == '\'' {
            for (_, c) in chars.by_ref() {
                if c == '\n' {
                    break;
                }
            }
            continue;
        }
        // Simple single‑character tokens
        let kind = match ch {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '^' => TokenKind::Caret,
            '\\' => TokenKind::Backslash,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '=' => {
                // Could be "==" or "="; look ahead
                if let Some('=') = chars.peek().map(|(_, c)| *c) {
                    chars.next(); // consume second '='
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Assign
                }
            }
            '!' => {
                if let Some('=') = chars.peek().map(|(_, c)| *c) {
                    chars.next();
                    TokenKind::NotEqual
                } else {
                    // unsupported token, skip
                    continue;
                }
            }
            '<' => {
                if let Some('=') = chars.peek().map(|(_, c)| *c) {
                    chars.next();
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            '>' => {
                if let Some('=') = chars.peek().map(|(_, c)| *c) {
                    chars.next();
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            '"' => {
                // string literal until next quote
                let mut s = String::new();
                for (_, c) in chars.by_ref() {
                    if c == '"' {
                        break;
                    }
                    s.push(c);
                }
                let len = s.len();
                tokens.push(Token {
                    kind: TokenKind::StringLit(s.clone()),
                    span: Span::new(start, start + 1 + len + 1),
                });
                continue;
            }
            c if c.is_ascii_digit() => {
                // Number literal: may be int or float
                let mut num = c.to_string();
                let mut end = start + c.len_utf8();
                let mut is_float = false;
                while let Some(&(idx, nxt)) = chars.peek() {
                    if nxt.is_ascii_digit() {
                        num.push(nxt);
                        end = idx + nxt.len_utf8();
                        chars.next();
                    } else if nxt == '.' && !is_float {
                        // start of fractional part
                        is_float = true;
                        num.push('.');
                        end = idx + nxt.len_utf8();
                        chars.next();
                    } else {
                        break;
                    }
                }
                if is_float {
                    let value = num.parse::<f64>().unwrap_or(0.0);
                    tokens.push(Token {
                        kind: TokenKind::Float(value),
                        span: Span::new(start, end),
                    });
                } else {
                    let value = num.parse::<i64>().unwrap_or(0);
                    tokens.push(Token {
                        kind: TokenKind::Int(value),
                        span: Span::new(start, end),
                    });
                }
                continue;
            }
            _ => {
                // identifiers / keywords and boolean literals
                let mut ident = String::new();
                ident.push(ch);
                let mut end = start + ch.len_utf8();
                while let Some(&(idx, nxt)) = chars.peek() {
                    if nxt.is_alphanumeric() || nxt == '_' {
                        ident.push(nxt);
                        end = idx + nxt.len_utf8();
                        chars.next();
                    } else {
                        break;
                    }
                }
                // Check for boolean literals (case‑insensitive)
                match ident.to_ascii_uppercase().as_str() {
                    "TRUE" => {
                        tokens.push(Token {
                            kind: TokenKind::Bool(true),
                            span: Span::new(start, end),
                        });
                        continue;
                    }
                    "FALSE" => {
                        tokens.push(Token {
                            kind: TokenKind::Bool(false),
                            span: Span::new(start, end),
                        });
                        continue;
                    }
                    _ => {}
                }
                // Match other keywords (case‑insensitive)
                let kind = match ident.to_ascii_uppercase().as_str() {
                    "AND" => TokenKind::And,
                    "OR" => TokenKind::Or,
                    "XOR" => TokenKind::Xor,
                    "LET" => TokenKind::Let,
                    "MUT" => TokenKind::Mut,
                    "FUNCTION" => TokenKind::Function,
                    "RETURNS" => TokenKind::Returns,
                    "RETURN" => TokenKind::Return,
                    "IF" => TokenKind::If,
                    "THEN" => TokenKind::Then,
                    "ELSE" => TokenKind::Else,
                    "END" => TokenKind::End,
                    "WHILE" => TokenKind::While,
                    "PRINT" => TokenKind::Print,
                    "NOT" => TokenKind::Not,
                    "FOR" => TokenKind::For,
                    "TO" => TokenKind::To,
                    "STEP" => TokenKind::Step,
                    "DO" => TokenKind::Do,
                    "LOOP" => TokenKind::Loop,
                    "UNTIL" => TokenKind::Until,
                    "AS" => TokenKind::As,
                    "MOD" => TokenKind::Mod,
                    "DIM" => TokenKind::Dim,
                    "ON" => TokenKind::On,
                    "ERROR" => TokenKind::Error,
                    "GOTO" => TokenKind::Goto,
                    "RESUME" => TokenKind::Resume,
                    "SHL" => TokenKind::Shl,
                    "SHR" => TokenKind::Shr,
                    _ => TokenKind::Identifier(ident),
                };
                let token = Token {
                    kind,
                    span: Span::new(start, end),
                };
                tokens.push(token);
                continue;
            }
        };
        // For the identifier/keyword branch we need custom handling, handled above.
        // We'll reconstruct token for non‑identifier kinds.
        let end = start + ch.len_utf8();
        tokens.push(Token {
            kind,
            span: Span::new(start, end),
        });
    }
    tokens.push(Token {
        kind: TokenKind::Eof,
        span: Span::new(input.len(), input.len()),
    });
    tokens
}
