# RFC-0037: OPTION IMPLICIT (Implicit Variable Declaration)

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add `OPTION IMPLICIT` to RBASIC for backward compatibility with classic BASIC programs. When enabled, variables do not need explicit `DIM` declarations and are automatically declared as `VARIANT` (or `INTEGER` for simple assignments) on first use. This is a legacy mode opt-in and contradicts `OPTION EXPLICIT` (RFC-0023). Using both in the same file emits an error.

---

## 2. Syntax (EBNF)

```ebnf
option_implicit ::= "OPTION" "IMPLICIT"
option_explicit ::= "OPTION" "EXPLICIT"
```

- `OPTION`, `IMPLICIT`, `EXPLICIT` are case-insensitive reserved keywords.
- `OPTION IMPLICIT` must appear at the top of the file (before any executable statements).
- `OPTION IMPLICIT` and `OPTION EXPLICIT` are mutually exclusive.

Examples:

```basic
OPTION IMPLICIT

' No DIM needed - variables auto-declared
x = 10
name = "Alice"
PRINT x, name
```

```basic
OPTION IMPLICIT

' Variables auto-declared with type inference
FOR i = 1 TO 10
    PRINT i * 2
NEXT i
```

```basic
OPTION EXPLICIT

' This would produce an error - must DIM first
' x = 10  ' E1001: Undeclared variable

DIM x AS INTEGER
x = 10
PRINT x
```

---

## 3. Semantics

1. `OPTION IMPLICIT` enables implicit variable declaration. Any identifier used in an expression is automatically declared as a new variable.
2. The type of implicitly declared variables is inferred from the first assignment:
   - Integer literal → `INTEGER`
   - Float literal → `DOUBLE`
   - String literal → `STRING`
   - Boolean literal → `BOOLEAN`
   - Function call → function's return type
3. If no assignment determines a type, the variable defaults to `VARIANT`.
4. `OPTION IMPLICIT` and `OPTION EXPLICIT` cannot both appear in the same file. Emit `E2100`.
5. `OPTION IMPLICIT` must appear before any executable statements. Emit `E2101` if placed after code.
6. Implicit variables are added to the global scope.
7. Implicit declaration does not apply to function/sub parameters (which are always explicit).

---

## 4. AST (node definitions)

### OptionImplicit (Statement)

```text
OptionImplicit {
    enabled: bool,
}
```

This node is a marker in the AST that affects the semantic analysis phase. No code is generated.

---

## 5. Parsing

When `OPTION` is encountered:

1. Consume `OPTION`.
2. Check for `IMPLICIT` or `EXPLICIT`.
3. Produce `Statement::OptionImplicit { enabled: true }` or `Statement::OptionExplicit`.

```rust
fn parse_option() -> Result<Statement> {
    consume(Option);
    match peek() {
        Implicit => {
            advance();
            Ok(Statement::OptionImplicit { enabled: true })
        }
        Explicit => {
            advance();
            Ok(Statement::OptionExplicit)
        }
        _ => Err("Expected IMPLICIT or EXPLICIT after OPTION"),
    }
}
```

---

## 6. Semantic Analysis

1. **Mode tracking** — the semantic analyzer tracks whether `OPTION IMPLICIT` or `OPTION EXPLICIT` is active. If both are present, emit `E2100`.
2. **Position validation** — `OPTION IMPLICIT` must appear before any executable statements. Emit `E2101` if placed after code.
3. **Implicit declaration** — when `OPTION IMPLICIT` is active and an undeclared identifier is encountered, automatically declare it with inferred type.
4. **Type inference** — the first assignment to an implicit variable determines its type.
5. **Scope** — implicit variables are added to the global scope.
6. **No conflict with explicit** — if a variable is both implicitly declared and later explicitly `DIM`'d, the explicit declaration takes precedence (no error).

---

## 7. Code Generation

`OPTION IMPLICIT` does not generate any Rust code. It only affects the semantic analysis phase by enabling implicit variable declaration.

### Implicit Variable

```basic
OPTION IMPLICIT
x = 10
```

Compiles to:

```rust
let mut x: i32 = 10;
```

### Implicit String

```basic
OPTION IMPLICIT
name = "Alice"
PRINT "Hello, " + name
```

Compiles to:

```rust
let mut name: String = "Alice".to_string();
println!("Hello, {}", name);
```

---

## 8. Error Codes

| Code  | Description                                             |
|-------|---------------------------------------------------------|
| E2100 | Both OPTION IMPLICIT and OPTION EXPLICIT in same file   |
| E2101 | OPTION IMPLICIT placed after executable statements       |

---

## 9. Acceptance Criteria

```text
✓ OPTION IMPLICIT parsed as OptionImplicit
✓ OPTION EXPLICIT parsed as OptionExplicit
✓ Both in same file produces E2100
✓ OPTION IMPLICIT after code produces E2101
✓ Implicit variable declaration works (no DIM required)
✓ Integer literal assignment infers INTEGER type
✓ Float literal assignment infers DOUBLE type
✓ String literal assignment infers STRING type
✓ Implicit variables compile to let mut with inferred type
✓ OPTION IMPLICIT has no runtime overhead
✓ Full test suite passes
```
