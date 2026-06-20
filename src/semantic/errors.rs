use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticErrorCode {
    E1001,
    E1002,
    E1003,
    E1004,
    E1010,
    E1011,
    E1020,
    E1021,
    E1022,
    E1030,
    E1031,
    E1032,
    E1033,
    E1034,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticError {
    pub code: SemanticErrorCode,
    pub message: String,
    pub span: Option<(usize, usize)>,
}

impl fmt::Display for SemanticErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PartialEq<&'static str> for SemanticErrorCode {
    fn eq(&self, other: &&'static str) -> bool {
        matches!(
            (self, *other),
            (SemanticErrorCode::E1001, "E1001")
                | (SemanticErrorCode::E1002, "E1002")
                | (SemanticErrorCode::E1003, "E1003")
                | (SemanticErrorCode::E1004, "E1004")
                | (SemanticErrorCode::E1010, "E1010")
                | (SemanticErrorCode::E1011, "E1011")
                | (SemanticErrorCode::E1020, "E1020")
                | (SemanticErrorCode::E1021, "E1021")
                | (SemanticErrorCode::E1022, "E1022")
                | (SemanticErrorCode::E1030, "E1030")
                | (SemanticErrorCode::E1031, "E1031")
                | (SemanticErrorCode::E1032, "E1032")
                | (SemanticErrorCode::E1033, "E1033")
                | (SemanticErrorCode::E1034, "E1034")
        )
    }
}
