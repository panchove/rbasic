# RFC-0023: OPTION EXPLICIT and OPTION BASE

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add `OPTION EXPLICIT` and `OPTION BASE` directives to RBASIC. `OPTION EXPLICIT` enforces explicit variable declaration before use. `OPTION BASE` sets the default lower bound for array indices.

---

## 2. Syntax (EBNF)

```ebnf
option_explicit ::= "OPTION" "EXPLICIT"
option_base     ::= "OPTION" "BASE" INTEGER_LITERAL
```

- `OPTION`, `EXPLICIT`, `BASE` are case-insensitive reserved keywords.
- `OPTION EXPLICIT` takes no arguments.
- `OPTION BASE` takes `0` or `1` as the only valid values.
- These directives must appear at the top of the program, before any executable statements (excluding other `OPTION` directives).

Examples:

```basic
OPTION EXPLICIT
OPTION BASE 0

DIM arr(10) AS INTEGER
```

```basic
OPTION BASE 1
DIM matrix(3, 3) AS DOUBLE
```

---

## 3. Semantics

### OPTION EXPLICIT

1. When present, all variables must be declared with `DIM` or `LET` before use.
2. Using an undeclared variable emits `E1090`.
3. When absent, RBASIC follows traditional BASIC behavior: undeclared variables are implicitly typed (defaulting to `VARIANT` or the type from `DEFINT`, `DEFSNG`, etc.).
4. `OPTION EXPLICIT` is a file-level directive; it applies to the entire program.

### OPTION BASE

1. Sets the default lower bound for `DIM` array declarations.
2. Valid values are `0` and `1`. Other values emit `E1091`.
3. The default (if no `OPTION BASE` is specified) is `0`.
4. `OPTION BASE` must appear before any `DIM` statements. If it appears after a `DIM`, emit `E1092`.
5. `OPTION BASE` affects all subsequent `DIM` statements in the file.

---

## 4. AST (node definitions)

```text
OptionExplicit {}

OptionBase {
    base: i32,
}
```

These nodes are already defined in RFC-0005 §4.18, §4.19.

---

## 5. Parsing

When `OPTION` keyword is encountered at statement level:

1. Consume `OPTION`.
2. If the next token is `EXPLICIT`, produce `Statement::OptionExplicit`.
3. If the next token is `BASE`, consume it, then parse an integer literal. Produce `Statement::OptionBase { base }`.

```rust
fn parse_option() -> Result<Statement> {
    consume(Option);
    match peek() {
        Explicit => {
            advance();
            Ok(Statement::OptionExplicit {})
        }
        Base => {
            advance();
            let base = expect_integer_literal()?;
            Ok(Statement::OptionBase { base })
        }
        _ => Err("Expected EXPLICIT or BASE after OPTION"),
    }
}
```

---

## 6. Semantic Analysis

### OPTION EXPLICIT

1. Once `OPTION EXPLICIT` is encountered, all subsequent variable references must resolve to a declared variable. Undeclared variables emit `E1090`.
2. The directive is not validated for position (top of file); it is applied as soon as the analyzer encounters it.

### OPTION BASE

1. Validate that the value is `0` or `1`. Otherwise emit `E1091`.
2. Validate that no `DIM` statements have been processed before this directive. If `DIM` was already processed, emit `E1092`.
3. Store the base value in the analyzer state; it is used when processing `Dim` nodes to compute array index offsets.

---

## 7. Code Generation

### OPTION EXPLICIT

No direct codegen output. This is a compile-time directive only.

### OPTION BASE

Affects array initialization. The base value is used during `Dim` codegen to adjust index bounds:

```basic
OPTION BASE 1
DIM arr(5) AS INTEGER
```

Compiles to:

```rust
// Option Base 1: indices 1..=5, array has 5 elements
let mut arr = [0i32; 5];
// Access: arr[index - 1] in generated code
```

When `OPTION BASE 0`:

```basic
OPTION BASE 0
DIM arr(5) AS INTEGER
```

Compiles to:

```rust
let mut arr = [0i32; 6];
// Access: arr[index] in generated code
```

The codegen emits a helper function for bounds checking:

```rust
fn array_index(base: i32, index: i32, upper: i32) -> usize {
    let idx = index - base;
    if idx < 0 || idx > upper - base {
        panic!("Array index out of bounds");
    }
    idx as usize
}
```

---

## 8. Error Codes

| Code  | Description                                              |
|-------|----------------------------------------------------------|
| E1090 | Undeclared variable used with OPTION EXPLICIT            |
| E1091 | Invalid OPTION BASE value (must be 0 or 1)              |
| E1092 | OPTION BASE after DIM (must appear before array decls)   |

---

## 9. Acceptance Criteria

```text
✓ OPTION EXPLICIT parsed as OptionExplicit
✓ OPTION BASE 0 parsed as OptionBase { base: 0 }
✓ OPTION BASE 1 parsed as OptionBase { base: 1 }
✓ OPTION BASE with invalid value produces E1091
✓ OPTION BASE after DIM produces E1092
✓ Undeclared variable with OPTION EXPLICIT produces E1090
✓ OPTION BASE affects array index bounds in codegen
✓ Default OPTION BASE is 0
✓ Full test suite passes
```
