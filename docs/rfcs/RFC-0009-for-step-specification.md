# RFC-0009: FOR...STEP Loop Specification

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-06-19
Last Updated: 2026-06-19

---

# 1. Summary

Extend the FOR loop with an optional STEP clause to specify a custom increment value, matching classic BASIC semantics.

---

# 2. Syntax

```ebnf
for_stmt ::= "FOR" IDENTIFIER "=" expression "TO" expression ("STEP" expression)? block "END" "FOR"
```

The STEP expression is optional. If omitted, the default increment is 1 (current behavior).

---

# 3. Semantics

- The step value is evaluated once before the loop begins.
- The step value must be numeric (I32 or F64).
- The step type must match the loop bounds type.
- After each iteration, the loop variable is incremented by the step value.
- The loop condition `var <= end` applies only for positive steps. If the step is negative, the condition should be `var >= end`.

---

# 4. Code Generation (Rust)

```rust
{
    let mut var = start;
    let step = <step_expr>;
    if step >= 0 {
        while var <= end {
            <body>
            var += step;
        }
    } else {
        while var >= end {
            <body>
            var += step;
        }
    }
}
```

---

# 5. Error Codes

| Code  | Description                        |
|-------|------------------------------------|
| E1034 | Step value must be numeric (I32/F64) |

---

# 6. Acceptance Criteria

```
✓ FOR...STEP parsed correctly (optional STEP clause)
✓ Step type validated as numeric
✓ Step type matches bounds type
✓ Negative step generates descending loop
✓ Codegen produces correct Rust with step variable
✓ Existing FOR (no STEP) unchanged
```
