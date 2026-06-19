pub mod codegen;
pub mod diagnostics;
pub mod lexer;
pub mod parser;
pub mod semantic;

pub use codegen::rust::generate_rust;
pub use lexer::{
    lex,
    token::{Token, TokenKind},
};
pub use parser::ast::*;
pub use parser::{ParseError, Parser};
pub use semantic::analyze;
