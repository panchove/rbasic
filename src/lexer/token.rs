use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexErrorCode {
    InvalidChar,
    UnterminatedString,
    InvalidNumber,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    pub code: LexErrorCode,
    pub message: String,
    pub span: Span,
}

/// Position in the source code (byte offset).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// All possible token kinds for RBASIC v0.1.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Keywords (case‑insensitive)
    Let,
    Mut,
    Function,
    Returns,
    Return,
    If,
    Then,
    Else,
    End,
    While,
    Print,
    Not,
    For,
    To,
    Step,
    Do,
    Loop,
    Until,
    As,
    And,
    Or,
    Xor,
    Dim,
    On,
    Error,
    Goto,
    Resume,
    Shl, // Shift left
    Shr, // Shift right

    // Literals
    Identifier(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    StringLit(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Caret,          // ^
    Backslash,      // \
    Mod,            // MOD
    Assign,         // =
    PlusEqual,      // +=
    MinusEqual,     // -=
    StarEqual,      // *=
    SlashEqual,     // /=
    BackslashEqual, // \=
    ModEqual,       // MOD=
    EqualEqual,     // ==
    NotEqual,       // !=
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Delimiters
    Colon,
    Comma,
    LParen,
    RParen,

    Eof,
}

/// Full token with its span.
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenKind::*;
        let txt = match self {
            Let => "LET",
            Mut => "MUT",
            Function => "FUNCTION",
            Returns => "RETURNS",
            Return => "RETURN",
            If => "IF",
            Then => "THEN",
            Else => "ELSE",
            End => "END",
            While => "WHILE",
            Print => "PRINT",
            Not => "NOT",
            For => "FOR",
            To => "TO",
            Step => "STEP",
            Do => "DO",
            Loop => "LOOP",
            Until => "UNTIL",
            Identifier(s) => s,
            Int(i) => return write!(f, "{}", i),
            Float(fl) => return write!(f, "{}", fl),
            Bool(b) => return write!(f, "{}", b),
            StringLit(s) => return write!(f, "\"{}\"", s),
            Plus => "+",
            Minus => "-",
            Star => "*",
            Slash => "/",
            Caret => "^",
            Backslash => "\\",
            Mod => "MOD",
            As => "AS",
            And => "AND",
            Or => "OR",
            Xor => "XOR",
            Dim => "DIM",
            On => "ON",
            Error => "ERROR",
            Goto => "GOTO",
            Resume => "RESUME",
            Shl => "SHL",
            Shr => "SHR",
            Assign => "=",
            PlusEqual => "+=",
            MinusEqual => "-=",
            StarEqual => "*=",
            SlashEqual => "/=",
            BackslashEqual => "\\=",
            ModEqual => "MOD=",
            EqualEqual => "==",
            NotEqual => "!=",
            Less => "<",
            LessEqual => "<=",
            Greater => ">",
            GreaterEqual => ">=",
            Colon => ":",
            Comma => ",",
            LParen => "(",
            RParen => ")",
            Eof => "<EOF>",
        };
        write!(f, "{}", txt)
    }
}
