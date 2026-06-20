# RFC-0012: DIM Array Declarations

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add array declaration support to RBASIC using the classic BASIC `DIM` keyword. Arrays are declared with a fixed number of dimensions and default base type. Code generation is deferred to a future version.

---

## 2. Syntax

```ebnf
dim_stmt ::= "DIM" IDENTIFIER "(" expression ("," expression)* ")"
             ("," IDENTIFIER "(" expression ("," expression)* ")")*
```

Examples:

```basic
DIM arr(10)
DIM matrix(5, 5)
DIM a(100), b(200)
```

---

## 3. AST

### Dim Statement

```text
Dim {
    declarations: Vec<ArrayDecl>,
}
```

### ArrayDecl

```text
ArrayDecl {
    name:       String,
    array_type: ArrayType,
    init:       Option<Expression>,   // reserved for future use
}
```

### ArrayType

```text
ArrayType {
    base_type:  Box<TypeRef>,
    dimensions: Vec<Expression>,
}
```

The default base type is `INTEGER` (I32). Dimension expressions are evaluated at runtime.

---

## 4. Parsing

- `DIM` is a keyword token (already in the lexer).
- The parser consumes `DIM`, then parses one or more comma-separated array declarations.
- Each array declaration has a name, parenthesized dimension list, and an optional initializer (reserved for future use).
- Ast node: `Statement::Dim { declarations: Vec<ArrayDecl> }`.

---

## 5. Semantic Analysis

- Array names are added to the current scope (global or local).
- Duplicate array names within the same scope produce E1003.
- The base type is read from the optional type annotation; if absent, defaults to I32.
- Dimension expressions are type-checked (must be numeric).

---

## 6. Code Generation

Code generation for DIM arrays is **deferred** to a future version. The codegen emits nothing for `Statement::Dim` nodes.

---

## 7. Error Codes

| Code  | Description                              |
|-------|------------------------------------------|
| E1003 | Duplicate array variable name in scope   |
| E1010 | Unknown type in type annotation          |

---

## 8. Acceptance Criteria

```text
✓ DIM parsed correctly (single and multiple arrays)
✓ Array AST nodes defined (ArrayDecl, ArrayType)
✓ Semantic analysis registers array names in scope
✓ Duplicate detection on array names
✓ Codegen emits nothing (deferred)
✓ Full test suite passes
```
