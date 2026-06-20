pub mod codegen;
pub mod diagnostics;
pub mod lexer;
pub mod parser;
pub mod runtime;
pub mod semantic;

pub use codegen::rust::generate_rust;
pub use diagnostics::{format_lex_error, offset_to_line_col};
pub use lexer::{
    lex,
    token::{LexError, LexErrorCode, Token, TokenKind},
};
pub use parser::ast::*;
pub use parser::{ParseError, Parser};
pub use semantic::analyze;
