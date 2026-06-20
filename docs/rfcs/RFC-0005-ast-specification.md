# RFC-0005: AST Specification

Status: Accepted
Version: 0.2
Author: RBASIC Project
Created: 2026-06-19
Last Updated: 2026-06-20

---

## 1. Summary

This RFC defines the Abstract Syntax Tree (AST) for RBASIC v0.2. The AST is the output of the parser (RFC-0004) and the input to semantic analysis (RFC-0006) and code generation.

---

## 2. Scope

Defines all AST node types: program, statements, expressions, literals, operators, and type references. Does **not** cover parsing, tokenization, semantic validation, or code generation.

---

## 3. Program

```text
Program {
    statements: Vec<Statement>
}
```

A program is a sequence of statements. Statements may be top-level or nested inside control flow and function/subroutine bodies.

---

## 4. Statements

```text
Statement ::= VarDecl
            | Print
            | Input
            | Return
            | If
            | While
            | For
            | DoLoop
            | SelectCase
            | Dim
            | OnError
            | Resume
            | Goto
            | Gosub
            | OnGoto
            | OnGosub
            | DoEvents
            | OptionExplicit
            | OptionBase
            | ExitFor
            | ExitWhile
            | ExitDo
            | SubDecl
            | Call
            | Assign
            | AssignOp
            | ExpressionStmt
            | FunctionDecl
            | Label
```

### 4.1 VarDecl

```text
VarDecl {
    name: String,
    typ:  Option<TypeRef>,
    init: Expression,
}
```

Variable declaration (`LET name [":" type] "=" expression`). The initializer is mandatory.

### 4.2 Print

```text
Print {
    exprs: Vec<Expression>,
}
```

Print statement (`PRINT expression`). Multiple expressions separated by `,` or `;`.

### 4.3 Input

```text
Input {
    prompt: Option<Expression>,
    target: String,
}
```

Input statement (`INPUT "prompt", variable` or `INPUT variable`).

### 4.4 Return

```text
Return {
    expr: Option<Expression>,
}
```

Return from GOSUB (`RETURN` or `RETURN expression`).

### 4.5 If

```text
If {
    condition:   Expression,
    then_branch: Vec<Statement>,
    elseif_clauses: Vec<ElseIfClause>,
    else_branch: Option<Vec<Statement>>,
}

ElseIfClause {
    condition: Expression,
    body:      Vec<Statement>,
}
```

### 4.6 While

```text
While {
    condition: Expression,
    body:      Vec<Statement>,
}
```

While loop (`WHILE condition ... WEND`).

### 4.7 For

```text
For {
    var:   String,
    start: Expression,
    end:   Expression,
    step:  Option<Expression>,
    body:  Vec<Statement>,
}
```

For loop (`FOR var = start TO end [STEP step] ... NEXT [var]`).

### 4.8 DoLoop

```text
DoLoop {
    variant:   DoLoopVariant,
    condition: Option<Expression>,
    body:      Vec<Statement>,
}

DoLoopVariant ::= WhilePre | UntilPre | WhilePost | UntilPost
```

### 4.9 SelectCase

```text
SelectCase {
    expr:     Expression,
    cases:    Vec<CaseClause>,
    else_case: Option<Vec<Statement>>,
}

CaseClause {
    values: Vec<CaseValue>,
    body:   Vec<Statement>,
}

CaseValue ::= Single(Expression)
            | Range(Expression, Expression)
```

### 4.10 Dim

```text
Dim {
    declarations: Vec<ArrayDecl>,
}

ArrayDecl {
    name:       String,
    array_type: ArrayType,
    init:       Option<Expression>,
}

ArrayType {
    base_type:  Box<TypeRef>,
    dimensions: Vec<DimBound>,
}

DimBound {
    lower: Option<Expression>,
    upper: Expression,
}
```

### 4.11 OnError

```text
OnError {
    label: String,
}
```

### 4.12 Resume

```text
Resume {
    label: Option<String>,
}
```

### 4.13 Goto

```text
Goto {
    label: String,
}
```

### 4.14 Gosub

```text
Gosub {
    label: String,
}
```

### 4.15 OnGoto

```text
OnGoto {
    expr:   Expression,
    labels: Vec<String>,
}
```

### 4.16 OnGosub

```text
OnGosub {
    expr:   Expression,
    labels: Vec<String>,
}
```

### 4.17 DoEvents

```text
DoEvents {}
```

### 4.18 OptionExplicit

```text
OptionExplicit {}
```

### 4.19 OptionBase

```text
OptionBase {
    base: i32,
}
```

### 4.20 ExitFor

```text
ExitFor {}
```

### 4.21 ExitWhile

```text
ExitWhile {}
```

### 4.22 ExitDo

```text
ExitDo {}
```

### 4.23 SubDecl

```text
SubDecl {
    name:   String,
    params: Vec<Param>,
    body:   Vec<Statement>,
}
```

```text
Param {
    name:    String,
    typ:     TypeRef,
    passing: ParamPassing,
}

ParamPassing ::= ByVal | ByRef | Optional
```

### 4.24 Call

```text
Call {
    name: String,
    args: Vec<Expression>,
}
```

### 4.25 Assign

```text
Assign {
    name: String,
    expr: Expression,
}
```

### 4.26 AssignOp

```text
AssignOp {
    target: String,
    op:     CompoundAssignOp,
    expr:   Expression,
}

CompoundAssignOp ::= AddEq | SubEq | MulEq | DivEq | IntDivEq | PowerEq | ModEq
```

### 4.27 ExpressionStmt

```text
ExpressionStmt {
    expr: Expression,
}
```

### 4.28 FunctionDecl

```text
FunctionDecl {
    name:     String,
    params:   Vec<Param>,
    ret_type: Option<TypeRef>,
    body:     Vec<Statement>,
}
```

### 4.29 Label

```text
Label {
    name: String,
}
```

---

## 5. Expressions

```text
Expression ::= Literal
             | Identifier(String)
             | Unary { op: UnaryOp, expr: Box<Expression> }
             | Binary { left: Box<Expression>, op: BinaryOp, right: Box<Expression> }
             | Grouping(Box<Expression>)
             | Cast { expr: Box<Expression>, target_type: String }
             | Call { callee: String, args: Vec<Expression> }
```

### 5.1 Literal

```text
Literal ::= Int(i64)
          | Float(f64)
          | String(String)
          | Bool(bool)
```

### 5.2 Identifier

A simple name reference, resolved later during semantic analysis.

### 5.3 Unary

```text
UnaryOp ::= Neg | Not
```

### 5.4 Binary

```text
BinaryOp ::= Add | Sub | Mul | Div | Pow | IntDiv | Mod
           | Eq | NotEq
           | Lt | Lte | Gt | Gte
           | And | Or | Xor
           | Shl | Shr
```

### 5.5 Grouping

Parenthesized expression (`(expression)`).

### 5.6 Cast

```text
Cast {
    expr:        Box<Expression>,
    target_type: String,
}
```

Postfix explicit type cast (`expr AS TypeName`).

### 5.7 Call

```text
Call {
    callee: String,
    args:   Vec<Expression>,
}
```

Function call (`identifier "(" [expression ("," expression)*] ")"`).

---

## 6. Type References

```text
TypeRef {
    name: String,
}
```

A type annotation is represented as a string identifier. Valid primitive types are validated during semantic analysis.

---

## 7. Traversal Order

1. **Program**: iterate statements in order
2. **FunctionDecl/SubDecl**: collect name/params/ret_type, then recurse into body
3. **If**: visit condition, then recurse into then_branch, elseif_clauses, else_branch
4. **While**: visit condition, then recurse into body
5. **For**: visit start, end, step, then recurse into body
6. **DoLoop**: visit condition, then recurse into body
7. **SelectCase**: visit expr, then each case value and body
8. **VarDecl/Assign/AssignOp**: visit init/target expression
9. **Print/Input/Return/ExpressionStmt/Dim/OnError/Resume**: visit inner expression(s)
10. **Goto/Gosub/OnGoto/OnGosub/DoEvents/OptionExplicit/OptionBase/ExitFor/ExitWhile/ExitDo/Label**: leaf nodes
11. **Call**: visit all arguments
12. **Cast**: visit inner expression
13. **Binary**: left then right
14. **Unary**: inner expression
15. **Literal/Identifier**: leaf nodes

---

## 8. Acceptance Criteria

```text
✓ Program node with statement list
✓ All 29 statement variants defined
✓ All 7 expression variants defined
✓ All 4 literal types defined
✓ Unary operators: Neg, Not
✓ Binary operators: Add, Sub, Mul, Div, Pow, IntDiv, Mod, Eq, NotEq, Lt, Lte, Gt, Gte, And, Or, Xor, Shl, Shr
✓ TypeRef defined
✓ DoLoopVariant defined (WhilePre, UntilPre, WhilePost, UntilPost)
✓ SelectCase / CaseClause / CaseValue defined
✓ Dim / ArrayDecl / ArrayType / DimBound defined
✓ Param / ParamPassing defined (ByVal, ByRef, Optional)
✓ CompoundAssignOp defined (AddEq, SubEq, MulEq, DivEq, IntDivEq, PowerEq, ModEq)
✓ Label node defined
✓ Traversal order specified
✓ Matches parser implementation in src/parser/ast.rs
```
