# RFC-0005: AST Specification

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-06-19
Last Updated: 2026-06-20

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
            | For
            | DoLoop
            | Dim
            | OnError
            | Resume
            | Assign
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

### 4.6 For

```text
For {
    var:   String,
    start: Expression,
    end:   Expression,
    step:  Option<Expression>,
    body:  Vec<Statement>,
}
```

For loop (`FOR var = start TO end [STEP step] block END FOR`). The loop variable is implicitly MUT within the loop body.

### 4.7 DoLoop

```text
DoLoop {
    variant:   DoLoopVariant,
    condition: Option<Expression>,
    body:      Vec<Statement>,
}
```

```text
DoLoopVariant ::= WhilePre | UntilPre | WhilePost | UntilPost
```

Four loop variants:
- **WhilePre**: `DO WHILE cond ... LOOP`
- **UntilPre**: `DO UNTIL cond ... LOOP`
- **WhilePost**: `DO ... LOOP WHILE cond`
- **UntilPost**: `DO ... LOOP UNTIL cond`

### 4.8 Dim

```text
Dim {
    declarations: Vec<ArrayDecl>,
}
```

```text
ArrayDecl {
    name:       String,
    array_type: ArrayType,
    init:       Option<Expression>,   // reserved for future use
}
```

```text
ArrayType {
    base_type:  Box<TypeRef>,
    dimensions: Vec<Expression>,       // array sizes
}
```

Array declaration (`DIM name(dim1, dim2, ...)`). Base type defaults to `INTEGER` (I32). Code generation is deferred to a future version.

### 4.9 OnError

```text
OnError {
    label: String,
}
```

Error handler directive (`ON ERROR GOTO label`).

### 4.10 Resume

```text
Resume {
    label: Option<String>,
}
```

Error recovery (`RESUME` or `RESUME label`).

### 4.11 Assign

```text
Assign {
    name: String,
    expr: Expression,
}
```

Standalone assignment (`x = expression`). The variable must be declared and mutable.

### 4.12 ExpressionStmt

```text
ExpressionStmt {
    expr: Expression,
}
```

An expression used as a statement (typically function calls).

### 4.13 FunctionDecl

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

| Variant | Syntax | Semantics |
|---------|--------|-----------|
| `Neg`   | `-expr` | Numeric negation (signed integers and floats) |
| `Not`   | `NOT expr` | Logical NOT (BOOL only) |

### 5.4 Binary

```text
BinaryOp ::= Add | Sub | Mul | Div | Pow | IntDiv | Mod
           | Eq | NotEq
           | Lt | Lte | Gt | Gte
           | And | Or | Xor
           | Shl | Shr
```

| Category         | Operators                                          |
|------------------|----------------------------------------------------|
| Arithmetic       | `+`, `-`, `*`, `/`, `^` (Pow), `\` (IntDiv), `MOD` |
| Equality         | `==`, `!=`                                         |
| Relational       | `<`, `<=`, `>`, `>=`                               |
| Logical          | `AND`, `OR`, `XOR`                                 |
| Bitwise shift    | `SHL`, `SHR`                                       |

### 5.5 Grouping

Parenthesized expression (`(expression)`). Used to override operator precedence.

### 5.6 Cast

```text
Cast {
    expr:        Box<Expression>,
    target_type: String,
}
```

Postfix explicit type cast (`expr AS TypeName`). Valid for all numeric types (signed integers, unsigned integers, floats).

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

A type annotation is represented as a string identifier. Valid primitive types are validated during semantic analysis, not during parsing. The full v0.1 type set: `BOOL`, `I8`, `I16`, `I32`, `I64`, `U8`, `U16`, `U32`, `U64`, `F32`, `F64`, `STRING`, plus classic BASIC aliases (`INTEGER`, `LONG`, `DOUBLE`, `SINGLE`, `BOOLEAN`, `BYTE`, `WORD`, `LONGLONG`). All type names are case‑insensitive.

---

## 7. Traversal Order

Semantic analysis and code generation traverse the AST as follows:

1. **Program**: iterate statements in order
2. **FunctionDecl**: collect name/params/ret_type, then recurse into body
3. **If**: visit condition, then recurse into then_branch and else_branch
4. **While**: visit condition, then recurse into body
5. **For**: visit start, end, step (if present), then recurse into body
6. **DoLoop**: visit condition (if present), then recurse into body
7. **VarDecl**: visit init expression
8. **Print/Return/ExpressionStmt/Dim/OnError/Resume**: visit inner expression(s)
9. **Cast**: visit inner expression
10. **Binary**: left then right
11. **Unary**: inner expression
12. **Call**: visit all arguments
13. **Literal/Identifier**: leaf nodes

---

## 8. Acceptance Criteria

```text
✓ Program node with statement list
✓ All 13 statement variants defined (VarDecl, Print, Return, If, While, For, DoLoop, Dim, OnError, Resume, Assign, ExpressionStmt, FunctionDecl)
✓ All 7 expression variants defined (Literal, Identifier, Unary, Binary, Grouping, Cast, Call)
✓ All 4 literal types defined (Int, Float, String, Bool)
✓ Unary operators: Neg, Not
✓ Binary operators: Add, Sub, Mul, Div, Pow, IntDiv, Mod, Eq, NotEq, Lt, Lte, Gt, Gte, And, Or, Xor, Shl, Shr
✓ TypeRef defined
✓ DoLoopVariant defined (WhilePre, UntilPre, WhilePost, UntilPost)
✓ ArrayDecl / ArrayType defined
✓ Traversal order specified
✓ Matches parser implementation in src/parser/ast.rs
```
