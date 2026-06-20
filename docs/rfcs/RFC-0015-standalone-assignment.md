# RFC-0015: Standalone Assignment

Status: Accepted
Version: 0.2
Author: RBASIC Project
Created: 2026-06-19
Last Updated: 2026-06-19

---

## 1. Summary

Add standalone assignment statements (`x = expr`) to RBASIC, enabling mutation of previously declared variables. Currently, the only way to assign a value to a variable is via `LET` (which creates a new declaration). This RFC introduces a dedicated assignment statement that mutates an existing variable in place.

---

## 2. Motivation

v0.1 lacks any mutation mechanism. `LET MUT` creates a mutable *declaration* but the value can never change — there is no way to reassign. This makes loops (FOR, DO, WHILE) impractical for any non-trivial use case: loop counters cannot be modified, accumulators cannot accumulate, and algorithms requiring mutable state are impossible.

Standalone assignment unlocks:
- Mutable loop variables (`i = i + 1`)
- Accumulators (`sum = sum + x`)
- State machines and flags (`done = TRUE`)
- Realistic examples and tests

---

## 3. Syntax

```ebnf
assign_stmt ::= IDENTIFIER "=" expression
```

Examples:

```basic
x = 42
x = x + 1
sum = sum + i
name = "hello"
flag = TRUE
```

---

## 4. AST

### Assign Statement

```text
Assign {
    name: String,
    expr: Expression,
}
```

The `name` is the identifier being assigned to. The `expr` is the new value.

```rust
Statement::Assign {
    name: String,
    expr: Expression,
}
```

---

## 5. Parsing

In the `statement()` method, when an `IDENTIFIER` token is encountered, the parser peeks at the next token:
- If the next token is `Assign` (`=`), parse as an assignment statement.
- Otherwise, parse as an `ExpressionStmt` (function call, or expression statement).

This requires no new tokens — `Assign` already exists in the lexer.

Pseudocode:

```
fn statement() -> Result<Statement> {
    match peek() {
        ...
        Identifier(name) => {
            if next_is(Assign) {
                advance(); // consume identifier
                advance(); // consume =
                let expr = expression();
                Assign { name, expr }
            } else {
                ExpressionStmt { expr: identifier_or_call }
            }
        }
        ...
    }
}
```

### No `LET` keyword

The assignment is a plain identifier followed by `=`, without `LET`. This distinguishes it from declarations (`LET x = 10`). The absence of `LET` is the syntactic marker for mutation vs. declaration.

---

## 6. Semantic Analysis

1. The variable `name` must exist in the current scope.
2. The type of `expr` must be compatible with the declared type of `name`.
3. The variable must be mutable (declared with `LET MUT` or implicitly mutable — see §9).

### Error Codes

| Code  | Description                            |
|-------|----------------------------------------|
| E1040 | Assignment to undeclared variable      |
| E1041 | Type mismatch in assignment            |
| E1042 | Assignment to immutable variable       |

---

## 7. Code Generation

Assignment emits a straightforward Rust assignment:

```basic
x = 42
```

```rust
x = 42;
```

```basic
sum = sum + i
```

```rust
sum = sum + i;
```

The variable is already declared with `let mut` in Rust (from the original `LET MUT` declaration).

---

## 8. Interaction with Existing Features

### LET MUT + assignment

```basic
LET MUT x: I32 = 0
x = 10
```

```rust
let mut x: i32 = 0;
x = 10;
```

### FOR loop variables

FOR loop variables are implicitly mutable within the loop body. Assignment can be used to modify the loop counter:

```basic
FOR i = 1 TO 10
    i = i + 1  ' skip every other
END FOR
```

### String assignment

```basic
LET s: STRING = "hello"
s = "world"
```

```rust
let s: String = "hello".to_string();
s = "world".to_string();
```

---

## 9. Implicit Mutability

For v0.2, only variables declared with `LET MUT` or FOR loop variables are mutable. A future RFC may introduce implicit mutability for all variables (common in BASIC dialects). This RFC takes a conservative approach: explicit mutability only.

---

## 10. Open Questions

1. **`LET MUT` shorthand?** Some BASICs use `LET x = 1` for both declaration and assignment. This RFC keeps them separate for clarity. A future RFC may allow omitting `LET` for a re-declaration in the same scope.

2. **Compound assignment?** `+=`, `-=`, etc. would be a natural extension but are out of scope for this RFC.

---

## 11. Acceptance Criteria

```
✓ x = 42 parsed as Assign statement
✓ Assignment to mutable variable compiles and runs
✓ Assignment to undeclared variable produces E1040
✓ Type mismatch in assignment produces E1041
✓ Assignment to immutable variable produces E1042
✓ FOR loop variable can be assigned
✓ String assignment works correctly
✓ Full test suite passes
```
