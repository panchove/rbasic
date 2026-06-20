pub mod token;

use token::{LexError, LexErrorCode, Span, Token, TokenKind};

pub fn lex(input: &str) -> (Vec<Token>, Vec<LexError>) {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
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
        // Simple single-character tokens
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
                if let Some('=') = chars.peek().map(|(_, c)| *c) {
                    chars.next();
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
                    errors.push(LexError {
                        code: LexErrorCode::InvalidChar,
                        message: "invalid character '!' (did you mean '!='?)".to_string(),
                        span: Span::new(start, start + 1),
                    });
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
                let mut s = String::new();
                let mut terminated = false;
                let mut end_pos = start + 1;
                loop {
                    match chars.next() {
                        None => break,
                        Some((idx, '"')) => {
                            terminated = true;
                            end_pos = idx + 1;
                            break;
                        }
                        Some((_, '\\')) => match chars.next() {
                            Some((idx, '\\')) => {
                                s.push('\\');
                                end_pos = idx + 1;
                            }
                            Some((idx, '"')) => {
                                s.push('"');
                                end_pos = idx + 1;
                            }
                            Some((idx, 'n')) => {
                                s.push('\n');
                                end_pos = idx + 1;
                            }
                            Some((idx, 'r')) => {
                                s.push('\r');
                                end_pos = idx + 1;
                            }
                            Some((idx, 't')) => {
                                s.push('\t');
                                end_pos = idx + 1;
                            }
                            Some((idx, c)) => {
                                s.push('\\');
                                s.push(c);
                                end_pos = idx + c.len_utf8();
                            }
                            None => break,
                        },
                        Some((idx, c)) => {
                            s.push(c);
                            end_pos = idx + c.len_utf8();
                        }
                    }
                }
                if !terminated {
                    errors.push(LexError {
                        code: LexErrorCode::UnterminatedString,
                        message: format!("unterminated string literal starting with \"{}\"", s),
                        span: Span::new(start, input.len()),
                    });
                    continue;
                }
                tokens.push(Token {
                    kind: TokenKind::StringLit(s),
                    span: Span::new(start, end_pos),
                });
                continue;
            }
            c if c.is_ascii_digit() => {
                let mut num = c.to_string();
                let mut end = start + c.len_utf8();
                let mut is_float = false;
                let mut invalid = false;
                while let Some(&(idx, nxt)) = chars.peek() {
                    if nxt.is_ascii_digit() {
                        num.push(nxt);
                        end = idx + nxt.len_utf8();
                        chars.next();
                    } else if nxt == '.' && !is_float {
                        is_float = true;
                        num.push('.');
                        end = idx + nxt.len_utf8();
                        chars.next();
                    } else if nxt == '.' && is_float {
                        // Second decimal point: invalid number
                        invalid = true;
                        end = idx + nxt.len_utf8();
                        chars.next();
                        // consume remainder of the malformed literal
                        while let Some(&(idx2, nxt2)) = chars.peek() {
                            if nxt2.is_ascii_digit() || nxt2 == '.' {
                                end = idx2 + nxt2.len_utf8();
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        break;
                    } else {
                        break;
                    }
                }
                if invalid {
                    errors.push(LexError {
                        code: LexErrorCode::InvalidNumber,
                        message: format!("invalid numeric literal '{}'", num),
                        span: Span::new(start, end),
                    });
                    continue;
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
                // Identifiers and keywords must start with a letter or underscore
                if !ch.is_ascii_alphabetic() && ch != '_' {
                    errors.push(LexError {
                        code: LexErrorCode::InvalidChar,
                        message: format!("invalid character '{}'", ch),
                        span: Span::new(start, start + ch.len_utf8()),
                    });
                    continue;
                }
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
                // Boolean literals (case-insensitive)
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
                // Keywords (case-insensitive)
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
                tokens.push(Token {
                    kind,
                    span: Span::new(start, end),
                });
                continue;
            }
        };
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
    (tokens, errors)
}
