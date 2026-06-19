# RFC-0005: AST Specification

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-06-19
Last Updated: 2026-06-19

---

## 1. Summary

This RFC defines the Abstract Syntax Tree (AST) for RBASIC v0.1. The AST is the output of the parser (RFC-0004) and the input to semantic analysis (RFC-0006) and code generation.

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

A program is a sequence of statements. Statements may be top-level or nested inside control flow and function bodies.

---

## 4. Statements

```text
Statement ::= VarDecl
            | Print
            | Return
            | If
            | While
            | ExpressionStmt
            | FunctionDecl
```

### 4.1 VarDecl

```text
VarDecl {
    name: String,
    typ:  Option<TypeRef>,
    init: Expression,
}
```

Variable declaration (`LET name [":" type] "=" expression`). The initializer is mandatory. The type annotation is optional.

### 4.2 Print

```text
Print {
    expr: Expression,
}
```

Print statement (`PRINT expression`).

### 4.3 Return

```text
Return {
    expr: Option<Expression>,
}
```

Return statement (`RETURN expression?`). The expression is optional, allowing bare `RETURN` (defaults to unit).

### 4.4 If

```text
If {
    condition:   Expression,
    then_branch: Vec<Statement>,
    else_branch: Option<Vec<Statement>>,
}
```

Multi-line if statement (`IF condition THEN block [ELSE block] END IF`).

### 4.5 While

```text
While {
    condition: Expression,
    body:      Vec<Statement>,
}
```

While loop (`WHILE condition block END WHILE`).

### 4.6 ExpressionStmt

```text
ExpressionStmt {
    expr: Expression,
}
```

An expression used as a statement (typically function calls).

### 4.7 FunctionDecl

```text
FunctionDecl {
    name:     String,
    params:   Vec<Param>,
    ret_type: Option<TypeRef>,
    body:     Vec<Statement>,
}
```

Function declaration (`FUNCTION name "(" params ")" [RETURNS type] block END FUNCTION`).

```text
Param {
    name: String,
    typ:  TypeRef,
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
UnaryOp ::= Neg
```

Unary minus (`-expression`). Future RFCs may extend this with `NOT`.

### 5.4 Binary

```text
BinaryOp ::= Add | Sub | Mul | Div
           | Eq | NotEq
           | Lt | Lte | Gt | Gte
```

Arithmetic: `+`, `-`, `*`, `/`
Equality: `==`, `!=`
Relational: `<`, `<=`, `>`, `>=`

### 5.5 Grouping

Parenthesized expression (`(expression)`). Used to override operator precedence.

### 5.6 Call

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

A type annotation is represented as a string identifier. Valid primitive types (`BOOL`, `I32`, `F64`, `STRING`) are validated during semantic analysis, not during parsing.

---

## 7. Traversal Order

Semantic analysis and code generation traverse the AST as follows:

1. **Program**: iterate statements in order
2. **FunctionDecl**: collect name/params/ret_type, then recurse into body
3. **If**: visit condition, then recurse into then_branch and else_branch
4. **While**: visit condition, then recurse into body
5. **VarDecl**: visit init expression
6. **Print/Return/ExpressionStmt**: visit inner expression(s)
7. **Binary**: left then right
8. **Unary**: inner expression
9. **Call**: visit all arguments
10. **Literal/Identifier**: leaf nodes

---

## 8. Acceptance Criteria

```text
✓ Program node with statement list
✓ All 7 statement variants defined
✓ All 6 expression variants defined
✓ All 4 literal types defined
✓ Unary and binary operators defined
✓ TypeRef defined
✓ Traversal order specified
✓ Matches parser implementation in src/parser/ast.rs
```
