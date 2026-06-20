# RFC-0032: CONTINUE FOR/WHILE/DO

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add `CONTINUE FOR`, `CONTINUE WHILE`, and `CONTINUE DO` statements to RBASIC, enabling premature advancement to the next iteration of a loop. Each `CONTINUE` statement skips the remaining body of the innermost matching loop and proceeds to the next iteration.

---

## 2. Syntax (EBNF)

```ebnf
continue_for   ::= "CONTINUE" "FOR"
continue_while ::= "CONTINUE" "WHILE"
continue_do    ::= "CONTINUE" "DO"
```

- `CONTINUE`, `FOR`, `WHILE`, `DO` are case-insensitive reserved keywords.
- `CONTINUE FOR` continues the innermost `FOR` loop.
- `CONTINUE WHILE` continues the innermost `WHILE` loop.
- `CONTINUE DO` continues the innermost `DO` loop.

Examples:

```basic
FOR i = 1 TO 10
    IF i MOD 2 = 0 THEN
        CONTINUE FOR
    END IF
    PRINT i
NEXT i
```

```basic
DO WHILE TRUE
    INPUT "Enter value: ", v
    IF v = "" THEN
        CONTINUE DO
    END IF
    process(v)
LOOP
```

```basic
WHILE running
    IF should_skip() THEN
        CONTINUE WHILE
    END IF
    do_work()
WEND
```

---

## 3. Semantics

1. `CONTINUE FOR` skips the remaining body of the innermost `FOR` loop and proceeds to the `NEXT` statement, which advances the loop variable.
2. `CONTINUE WHILE` skips the remaining body of the innermost `WHILE` loop and proceeds to re-evaluate the condition at `WEND`.
3. `CONTINUE DO` skips the remaining body of the innermost `DO` loop and proceeds to the `LOOP` statement.
4. `CONTINUE` statements are only valid inside their matching loop type. Using `CONTINUE FOR` outside a `FOR` loop emits `E1600`.
5. `CONTINUE` may appear inside nested loops, conditionals, or subroutines within the loop.
6. Only one `CONTINUE` per statement is allowed.

---

## 4. AST (node definitions)

```text
ContinueFor {}
ContinueWhile {}
ContinueDo {}
```

---

## 5. Parsing

When `CONTINUE` keyword is encountered:

1. Consume `CONTINUE`.
2. The next token must be `FOR`, `WHILE`, or `DO`.
3. Produce the corresponding AST node.

```rust
fn parse_continue() -> Result<Statement> {
    consume(Continue);
    match peek() {
        For  => { advance(); Ok(Statement::ContinueFor {}) }
        While => { advance(); Ok(Statement::ContinueWhile {}) }
        Do   => { advance(); Ok(Statement::ContinueDo {}) }
        _    => Err("Expected FOR, WHILE, or DO after CONTINUE"),
    }
}
```

---

## 6. Semantic Analysis

1. **Context validation** — `CONTINUE FOR` must be inside a `FOR` loop, `CONTINUE WHILE` inside a `WHILE` loop, `CONTINUE DO` inside a `DO` loop. Mismatched context emits `E1600`.
2. **Loop tracking** — the semantic analyzer maintains a loop nesting stack that tracks the current loop type. `CONTINUE` pops the stack for the matching type.
3. **Valid in any nesting depth** — `CONTINUE` inside nested loops only affects the innermost matching loop.

---

## 7. Code Generation

Each `CONTINUE` statement compiles to a `continue` in the corresponding Rust loop construct:

### CONTINUE FOR

```basic
FOR i = 1 TO 10
    IF i MOD 2 = 0 THEN CONTINUE FOR
    PRINT i
NEXT i
```

Compiles to:

```rust
for i in 1..=10 {
    if i % 2 == 0 {
        continue;
    }
    println!("{}", i);
}
```

### CONTINUE WHILE

```basic
WHILE running
    IF should_skip() THEN CONTINUE WHILE
    do_work()
WEND
```

Compiles to:

```rust
while running {
    if should_skip() {
        continue;
    }
    do_work();
}
```

### CONTINUE DO

```basic
DO WHILE TRUE
    IF v = "" THEN CONTINUE DO
    process(v)
LOOP
```

Compiles to:

```rust
while true {
    if v == "" {
        continue;
    }
    process(&v);
}
```

---

## 8. Error Codes

| Code  | Description                                         |
|-------|-----------------------------------------------------|
| E1600 | CONTINUE used outside matching loop type             |

---

## 9. Acceptance Criteria

```text
✓ CONTINUE FOR parsed as ContinueFor
✓ CONTINUE WHILE parsed as ContinueWhile
✓ CONTINUE DO parsed as ContinueDo
✓ CONTINUE FOR outside FOR loop produces E1600
✓ CONTINUE WHILE outside WHILE loop produces E1600
✓ CONTINUE DO outside DO loop produces E1600
✓ CONTINUE inside nested loop affects only innermost loop
✓ CONTINUE compiles to correct Rust continue statement
✓ Full test suite passes
```
