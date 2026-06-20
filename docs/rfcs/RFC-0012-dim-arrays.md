# RFC-0012: DIM Array Declarations

Status: Accepted
Version: 0.2
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add array declaration support to RBASIC using the classic BASIC `DIM` keyword. Arrays support explicit bounds with `TO`, multiple dimensions, and `OPTION BASE` to change the default lower bound. QuickBASIC is the source of truth for this feature.

---

## 2. Syntax

```ebnf
dim_stmt     ::= "DIM" dim_decl ("," dim_decl)*
dim_decl     ::= IDENTIFIER "(" dim_bounds ")" ("AS" type_ref)?
dim_bounds   ::= dim_bound ("," dim_bound)*
dim_bound    ::= expression ("TO" expression)?
```

When `TO` is omitted, the lower bound is determined by `OPTION BASE` (default 0).

Examples:

```basic
' Default bounds (0-based by default)
DIM arr(10)              ' indices 0 to 10 (11 elements)

' Explicit bounds
DIM a(0 TO 10)           ' indices 0 to 10
DIM b(1 TO 10)           ' indices 1 to 10

' Multi-dimensional
DIM matrix(5, 5)         ' 6x6 (0‑5, 0‑5)
DIM grid(1 TO 3, 1 TO 4) ' 3x4 (1‑3, 1‑4)

' Multiple declarations
DIM a(100), b(200)

' With type annotation
DIM arr(10) AS INTEGER

' OPTION BASE changes default lower bound
OPTION BASE 1
DIM c(10)                ' indices 1 to 10 (10 elements)
```

---

## 3. OPTION BASE

```ebnf
option_base_stmt ::= "OPTION" "BASE" INTEGER_LITERAL
```

- `OPTION BASE 0` — default. Arrays are 0-based when `TO` is omitted.
- `OPTION BASE 1` — Arrays are 1-based when `TO` is omitted.
- Must appear before any `DIM` statements that rely on the default.
- Only `0` and `1` are valid values.

---

## 4. AST

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
    dimensions: Vec<DimBound>,
}
```

### DimBound

```text
DimBound {
    lower: Option<Expression>,   // None if TO omitted (use OPTION BASE)
    upper: Expression,
}
```

The default base type is `INTEGER` (I32). When `TO` is omitted, the lower bound is determined by the current `OPTION BASE` setting.

---

## 5. Parsing

- `DIM` is a keyword token (already in the lexer).
- The parser consumes `DIM`, then parses one or more comma-separated array declarations.
- Each array declaration has a name, parenthesized dimension list, optional `AS` type annotation, and an optional initializer (reserved for future use).
- `TO` keyword specifies explicit bounds; when omitted, bounds are inferred from `OPTION BASE`.

---

## 6. Semantic Analysis

- Array names are added to the current scope (global or local).
- Duplicate array names within the same scope produce E1003.
- The base type is read from the optional type annotation; if absent, defaults to I32.
- Dimension expressions are type-checked (must be numeric).
- `OPTION BASE` is tracked per scope; only values 0 and 1 are accepted.

---

## 7. Code Generation

Code generation for DIM arrays is **deferred** to a future version. The codegen emits nothing for `Statement::Dim` nodes.

---

## 8. Error Codes

| Code  | Description                              |
|-------|------------------------------------------|
| E1003 | Duplicate array variable name in scope   |
| E1010 | Unknown type in type annotation          |
| E1011 | Invalid OPTION BASE value (must be 0 or 1) |

---

## 9. Acceptance Criteria

```text
✓ DIM parsed correctly (single and multiple arrays)
✓ Explicit bounds with TO supported
✓ OPTION BASE 0 and OPTION BASE 1 supported
✓ Array AST nodes defined (ArrayDecl, ArrayType, DimBound)
✓ Semantic analysis registers array names in scope
✓ Duplicate detection on array names
✓ OPTION BASE validation (0 or 1 only)
✓ Codegen emits nothing (deferred)
✓ Full test suite passes
```
