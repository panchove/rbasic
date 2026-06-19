# RFC-0010: DO Loop Specification

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-06-19
Last Updated: 2026-06-19

---

# 1. Summary

Add DO/LOOP loops with WHILE and UNTIL variants (pre-test and post-test), matching classic BASIC semantics.

---

# 2. Syntax

```ebnf
do_while_pre  ::= "DO" "WHILE" expression block "LOOP"
do_until_pre  ::= "DO" "UNTIL" expression block "LOOP"
do_while_post ::= "DO" block "LOOP" "WHILE" expression
do_until_post ::= "DO" block "LOOP" "UNTIL" expression
```

---

# 3. Semantics

- **DO WHILE ... LOOP**: Pre-test loop. Condition is checked before each iteration. Body executes zero or more times.
- **DO UNTIL ... LOOP**: Pre-test loop. Body executes while condition is FALSE (inverse of DO WHILE).
- **DO ... LOOP WHILE**: Post-test loop. Body executes at least once. Condition checked after each iteration.
- **DO ... LOOP UNTIL**: Post-test loop. Body executes at least once. Stops when condition is TRUE.

In all variants, the condition must evaluate to BOOL.

---

# 4. Code Generation (Rust)

```rust
// DO WHILE cond ... LOOP
while cond {
    <body>
}

// DO UNTIL cond ... LOOP
while !cond {
    <body>
}

// DO ... LOOP WHILE cond
loop {
    <body>
    if !cond { break; }
}

// DO ... LOOP UNTIL cond
loop {
    <body>
    if cond { break; }
}
```

---

# 5. Error Codes

| Code  | Description |
|-------|-------------|
| E1032 | DO loop condition must be BOOL (reused from IF/WHILE) |

---

# 6. Acceptance Criteria

```
✓ DO WHILE parsed and codegened correctly
✓ DO UNTIL parsed and codegened correctly
✓ LOOP WHILE parsed and codegened correctly
✓ LOOP UNTIL parsed and codegened correctly
✓ DO loop condition validated as BOOL
✓ All four variants produce correct Rust code
```
