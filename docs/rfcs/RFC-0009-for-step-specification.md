# RFC-0009: FOR...NEXT...STEP Loop Specification

Status: Accepted
Version: 0.2
Author: RBASIC Project
Created: 2026-06-19
Last Updated: 2026-06-20

---

# 1. Summary

Define the FOR...NEXT loop with an optional STEP clause, matching QuickBASIC semantics. QuickBASIC is the source of truth for this feature.

---

# 2. Syntax

```ebnf
for_stmt    ::= "FOR" IDENTIFIER "=" expression "TO" expression ("STEP" expression)? block "NEXT" IDENTIFIER?
```

The STEP expression is optional. If omitted, the default increment is 1. The loop variable is implicitly declared and mutable within the loop scope.

Examples:
```basic
FOR i = 1 TO 10
    PRINT i
NEXT i

FOR i = 0 TO 20 STEP 2
    PRINT i
NEXT i

FOR i = 10 TO 1 STEP -1
    PRINT i
NEXT i
```

---

# 3. Semantics

- The step value is evaluated once before the loop begins.
- The step value must be numeric (I32 or F64).
- The step type must match the loop bounds type.
- After each iteration, the loop variable is incremented by the step value.
- For positive steps: loop continues while `var <= end`.
- For negative steps: loop continues while `var >= end`.
- The loop variable is accessible after the loop exits (holds the termination value).

---

# 4. EXIT FOR

`EXIT FOR` terminates the FOR loop early and transfers control to the statement after `NEXT`.

```basic
FOR i = 1 TO 100
    IF i > 50 THEN EXIT FOR
    PRINT i
NEXT i
```

---

# 5. Code Generation (Rust)

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

# 6. Error Codes

| Code  | Description                        |
|-------|------------------------------------|
| E1034 | Step value must be numeric (I32/F64) |

---

# 7. Acceptance Criteria

```
✓ FOR...NEXT parsed correctly (QuickBASIC syntax)
✓ Optional STEP clause supported
✓ Step type validated as numeric
✓ Step type matches bounds type
✓ Negative step generates descending loop
✓ EXIT FOR terminates loop early
✓ Codegen produces correct Rust with step variable
✓ Existing FOR (no STEP) unchanged
```
