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
    Resume {
        label: Option<String>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Param {
    pub name: String,
    pub typ: TypeRef,
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
    Shl, // Shift left
    Shr, // Shift right
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
    pub dimensions: Vec<Expression>, // Array sizes
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayDecl {
    pub name: String,
    pub array_type: ArrayType,
    pub init: Option<Expression>, // Optional initialization
}
