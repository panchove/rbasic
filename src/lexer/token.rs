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

/// All possible token kinds for RBASIC.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Control flow
    If,
    Then,
    Else,
    ElseIf,
    End,
    While,
    Wend,
    For,
    Next,
    To,
    Step,
    Do,
    Loop,
    Until,
    Select,
    Case,
    On,
    Error,
    Goto,
    Gosub,
    Exit,
    Resume,
    Return,

    // Declarations
    Function,
    Returns,
    Sub,
    Call,
    Dim,
    Let,
    Mut,
    Option,
    Type,
    Public,
    Private,

    // Parameters
    ByVal,
    ByRef,
    Optional,

    // Keywords
    Input,
    Print,
    Not,
    As,
    And,
    Or,
    Xor,
    Shl,
    Shr,

    // File I/O
    Open,
    Close,
    Line,
    Append,
    Random,
    Binary,

    // System
    Peek,
    Poke,

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
    Hash, // #
    Semi, // ;
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
            If => "IF",
            Then => "THEN",
            Else => "ELSE",
            ElseIf => "ELSEIF",
            End => "END",
            While => "WHILE",
            Wend => "WEND",
            For => "FOR",
            Next => "NEXT",
            To => "TO",
            Step => "STEP",
            Do => "DO",
            Loop => "LOOP",
            Until => "UNTIL",
            Select => "SELECT",
            Case => "CASE",
            On => "ON",
            Error => "ERROR",
            Goto => "GOTO",
            Gosub => "GOSUB",
            Exit => "EXIT",
            Resume => "RESUME",
            Return => "RETURN",
            Function => "FUNCTION",
            Returns => "RETURNS",
            Sub => "SUB",
            Call => "CALL",
            Dim => "DIM",
            Let => "LET",
            Mut => "MUT",
            Option => "OPTION",
            Type => "TYPE",
            Public => "PUBLIC",
            Private => "PRIVATE",
            ByVal => "BYVAL",
            ByRef => "BYREF",
            Optional => "OPTIONAL",
            Input => "INPUT",
            Print => "PRINT",
            Not => "NOT",
            As => "AS",
            And => "AND",
            Or => "OR",
            Xor => "XOR",
            Shl => "SHL",
            Shr => "SHR",
            Open => "OPEN",
            Close => "CLOSE",
            Line => "LINE",
            Append => "APPEND",
            Random => "RANDOM",
            Binary => "BINARY",
            Peek => "PEEK",
            Poke => "POKE",
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
            Hash => "#",
            Semi => ";",
            LParen => "(",
            RParen => ")",
            Eof => "<EOF>",
        };
        write!(f, "{}", txt)
    }
}
