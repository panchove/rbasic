# RFC-0018: Compound Assignment Operators

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-06-19
Last Updated: 2026-06-19

---

## 1. Summary

Add compound assignment operators (`+=`, `-=`, `*=`, `/=`, `\=`, `MOD=`) to RBASIC, providing shorthand for common mutation patterns.

---

## 2. Motivation

Standalone assignment (RFC-0015) is implemented but requires full expression repetition for common patterns:

```basic
counter = counter + 1
total = total * value
```

Compound assignment reduces verbosity and matches classic BASIC convention:

```basic
counter += 1
total *= value
```

---

## 3. Syntax

### 3.1 Grammar

```ebnf
compound_assign_stmt ::= IDENTIFIER compound_assign_op expression
compound_assign_op   ::= "+=" | "-=" | "*=" | "/=" | "\=" | "MOD="
```

### 3.2 Token Inventory

New tokens:

```text
PlusEqual    (+=)
MinusEqual   (-=)
StarEqual    (*=)
SlashEqual   (/=)
BackslashEqual (\=)
ModEqual     (MOD=)
```

The lexer must recognize these as two‑character tokens via the longest‑match rule (e.g., `+=` is `PlusEqual`, not `Plus` + `Assign`).

### 3.3 Examples

```basic
counter += 1
total -= discount
price *= 1.1
count /= 2
quotient \= 3
remainder MOD= 10
```

---

## 4. Semantics

Each compound assignment is syntactic sugar for read‑modify‑write:

| Operator | Semantics            | Type Requirements                    |
|----------|----------------------|--------------------------------------|
| `x += y` | `x = x + y`         | Numeric or STRING (concatenation)    |
| `x -= y` | `x = x - y`         | Numeric only                         |
| `x *= y` | `x = x * y`         | Numeric only                         |
| `x /= y` | `x = x / y`         | Numeric only (float division)        |
| `x \= y` | `x = x \ y`         | Integer only (integer division)      |
| `x MOD= y`| `x = x MOD y`       | Integer only (modulo)                |

### 4.1 Type Rules

- **Numeric operators** (`+=`, `-=`, `*=`, `/=`): Same rules as the corresponding binary operator (RFC-0007 §9). Operands must be compatible; result type matches the expression type.
- **String concatenation** (`+=`): `STRING += STRING` is valid. `STRING += non‑STRING` is rejected (E1041).
- **Integer operators** (`\=`, `MOD=`): Both operands must be integer types. Float operands are rejected with E1021.

### 4.2 Mutability Requirements

Same as standalone assignment (RFC-0015):

1. The variable must exist in the current scope (E1040 if undeclared).
2. The variable must be mutable (E1042 if immutable).
3. The type of the expression must be compatible with the variable's declared type (E1041).

---

## 5. AST

```text
AssignOp {
    target: String,
    op:     CompoundAssignOp,
    expr:   Expression,
}
```

```rust
pub enum CompoundAssignOp {
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    IntDivEq,
    ModEq,
}
```

The existing `Statement::Assign` may be refactored to a unified `Assign { target, op: AssignOp, expr }` where `AssignOp` is either `Plain` or a compound variant. Alternatively, a new `Statement::AssignOp` variant may be added.

---

## 6. Parsing

In the `statement()` method, after peeking an `Identifier`, check whether the next token is a compound assignment operator:

```rust
fn statement() -> Result<Statement> {
    match peek() {
        Identifier(name) => {
            if next_is(Assign) {
                // standalone assignment
            } else if next_is(PlusEqual | MinusEqual | ...) {
                // compound assignment
            } else {
                // expression statement
            }
        }
    }
}
```

This extends the existing lookahead logic used for standalone assignment.

---

## 7. Semantic Analysis

The compiler should desugar compound assignment into the equivalent binary expression:

```basic
x += y   ⇒   x = x + y
```

During semantic analysis:

1. Resolve `x` — must exist (E1040), must be mutable (E1042).
2. Validate `x + y` using existing binary operator rules (E1021 for invalid ops).
3. Validate that the result is compatible with `x`'s type (E1041).

---

## 8. Code Generation

Emit the desugared form:

```rust
// x += y  ⇒  x = x + y;
x = x + y;
```

No new codegen logic is required beyond the existing assignment and binary expression emitters.

---

## 9. Diagnostics

| Code  | Description                    | When Triggered                        |
|-------|--------------------------------|---------------------------------------|
| E1043 | Compound assign undeclared     | Target variable not in scope          |
| E1044 | Compound assign immutable      | Target variable is not mutable        |
| E1045 | Compound assign type mismatch  | Expression type incompatible with var |

These are reserved in the `E1040–E1049` assignment block.

---

## 10. Exclusions

### 10.1 Pre‑/Post‑Increment (`++`/`--`)

The operators `++` and `--` are **explicitly excluded** from this RFC. Rationale:

- They are not part of classic BASIC tradition.
- They conflate expression and statement semantics (pre‑ vs. post‑increment).
- `+= 1` covers the common use case unambiguously.
- They would require introducing new expression forms beyond the statement‑level compound operators.

If `++`/`--` are desired in a future version, they require a separate RFC.

### 10.2 Bitwise Compound Assignment

`SHL=`, `SHR=`, `AND=`, `OR=`, `XOR=` are not included. They may be proposed in a future RFC if real‑world BASIC code requires them.

---

## 11. Impact on Existing RFCs

| RFC            | Impact                                            |
|----------------|---------------------------------------------------|
| RFC-0002       | New tokens: `PlusEqual`, `MinusEqual`, `StarEqual`, `SlashEqual`, `BackslashEqual`, `ModEqual` |
| RFC-0004       | New grammar rule: `compound_assign_stmt`; update `statement` rule |
| RFC-0005       | New AST node or variant for compound assignment   |
| RFC-0006       | New diagnostic codes E1043–E1045                  |
| RFC-0007       | No change (reuses existing binary op rules)       |
| RFC-0015       | Extends assignment system; shared mutability rules |

---

## 12. Open Questions

1. **Separate AST node or unified `Assign`?** The current `Statement::Assign` has `name` and `expr`. Adding an `op` field would unify both plain and compound assignment. Alternatively, a new variant avoids refactoring the existing code. Decision: use a separate `Statement::AssignOp` variant to minimize disruption to the existing codebase.

2. **String `+=`?** `STRING += STRING` is valid (concatenation). `STRING += I32` should be rejected, not implicitly converted. This matches the strict type policy of v0.2.

---

## 13. Acceptance Criteria

```text
✓ `x += 1` parsed as compound assignment
✓ `x -= 1`, `x *= 2`, `x /= 2`, `x \= 2`, `x MOD= 2` parsed
✓ Undeclared target produces E1043
✓ Immutable target produces E1044
✓ Type mismatch produces E1045
✓ `STRING += STRING` valid
✓ `STRING += I32` produces error
✓ `++` and `--` not implemented
✓ Full test suite passes
```
