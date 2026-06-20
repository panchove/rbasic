#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VarDecl {
        name: String,
        is_mut: bool,
        typ: Option<TypeRef>,
        init: Expression,
    },
    Print {
        expr: Expression,
    },
    Return {
        expr: Option<Expression>,
    },
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_ifs: Vec<ElseIfClause>,
        else_branch: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        var: String,
        start: Expression,
        end: Expression,
        step: Option<Expression>,
        body: Vec<Statement>,
    },
    DoLoop {
        variant: DoLoopVariant,
        condition: Option<Expression>,
        body: Vec<Statement>,
    },
    ExpressionStmt {
        expr: Expression,
    },
    Assign {
        name: String,
        expr: Expression,
    },
    AssignOp {
        name: String,
        op: CompoundAssignOp,
        expr: Expression,
    },
    ArrayAssign {
        name: String,
        indices: Vec<Expression>,
        value: Expression,
    },
    FunctionDecl {
        name: String,
        params: Vec<Param>,
        ret_type: Option<TypeRef>,
        body: Vec<Statement>,
    },
    Dim {
        declarations: Vec<ArrayDecl>,
    },
    OnError {
        label: String,
    },
    Input {
        prompt: Option<String>,
        target: String,
    },
    SubDecl {
        name: String,
        params: Vec<Param>,
        body: Vec<Statement>,
    },
    Call {
        name: String,
        args: Vec<Expression>,
    },
    Resume {
        label: Option<String>,
    },
    // File I/O
    Open {
        filename: Expression,
        mode: FileMode,
        handle: Expression,
        record_len: Option<Expression>,
    },
    Close {
        handles: Vec<Expression>,
    },
    InputHash {
        handle: Expression,
        targets: Vec<String>,
    },
    PrintHash {
        handle: Expression,
        items: Vec<PrintItem>,
    },
    LineInputHash {
        handle: Expression,
        target: String,
    },
    // SELECT CASE
    SelectCase {
        expr: Expression,
        cases: Vec<CaseClause>,
        else_case: Option<Vec<Statement>>,
    },
    // Control flow
    ExitFor,
    ExitWhile,
    ExitDo,
    Goto {
        label: String,
    },
    Gosub {
        label: String,
    },
    Label {
        name: String,
    },
    // OPTION
    OptionExplicit,
    OptionBase {
        base: i64,
    },
    // TYPE...END TYPE
    TypeDecl {
        name: String,
        fields: Vec<TypeField>,
    },
    // PEEK/POKE
    Poke {
        addr: Expression,
        value: Expression,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElseIfClause {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaseClause {
    pub values: Vec<CaseValue>,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CaseValue {
    Single(Expression),
    Range(Expression, Expression),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FileMode {
    Input,
    Output,
    Append,
    Random,
    Binary,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrintItem {
    Expr(Expression),
    Comma, // Tab to next zone
    Semi,  // No separator
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParamKind {
    ByVal,
    ByRef,
    Optional,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Param {
    pub name: String,
    pub typ: TypeRef,
    pub kind: ParamKind,
    pub default: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Unary {
        op: UnaryOp,
        expr: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    Grouping(Box<Expression>),
    Cast {
        expr: Box<Expression>,
        target_type: String,
    },
    Call {
        callee: String,
        args: Vec<Expression>,
    },
    ArrayAccess {
        name: String,
        indices: Vec<Expression>,
    },
    Peek {
        addr: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    IntDiv,
    Mod,
    Eq,
    NotEq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    Xor,
    Shl,
    Shr,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CompoundAssignOp {
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    IntDivEq,
    ModEq,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DoLoopVariant {
    WhilePre,
    UntilPre,
    WhilePost,
    UntilPost,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypeRef {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayType {
    pub base_type: Box<TypeRef>,
    pub dimensions: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayDecl {
    pub name: String,
    pub array_type: ArrayType,
    pub init: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeField {
    pub name: String,
    pub typ: TypeRef,
    pub visibility: Option<FieldVisibility>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FieldVisibility {
    Public,
    Private,
}
