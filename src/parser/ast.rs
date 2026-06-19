#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VarDecl {
        name: String,
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
    FunctionDecl {
        name: String,
        params: Vec<Param>,
        ret_type: Option<TypeRef>,
        body: Vec<Statement>,
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
