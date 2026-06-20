# RFC-0024: EXIT FOR/WHILE/DO

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add `EXIT FOR`, `EXIT WHILE`, and `EXIT DO` statements to RBASIC, enabling premature termination of loop constructs. Each `EXIT` statement breaks out of the innermost matching loop type.

---

## 2. Syntax (EBNF)

```ebnf
exit_for   ::= "EXIT" "EXIT" "FOR"
exit_while ::= "EXIT" "WHILE"
exit_do    ::= "EXIT" "DO"
```

- `EXIT`, `FOR`, `WHILE`, `DO` are case-insensitive reserved keywords.
- `EXIT FOR` exits the innermost `FOR` loop.
- `EXIT WHILE` exits the innermost `WHILE` loop.
- `EXIT DO` exits the innermost `DO` loop.

Examples:

```basic
FOR i = 1 TO 100
    IF i = 50 THEN
        EXIT FOR
    END IF
    PRINT i
NEXT i
```

```basic
DO WHILE TRUE
    INPUT "Enter Q to quit: ", cmd$
    IF cmd$ = "Q" THEN
        EXIT DO
    END IF
LOOP
```

```basic
WHILE running
    process()
    IF done THEN
        EXIT WHILE
    END IF
WEND
```

---

## 3. Semantics

1. `EXIT FOR` terminates the innermost enclosing `FOR` loop. Execution resumes at the statement following `NEXT`.
2. `EXIT WHILE` terminates the innermost enclosing `WHILE` loop. Execution resumes at the statement following `WEND`.
3. `EXIT DO` terminates the innermost enclosing `DO` loop. Execution resumes at the statement following `LOOP`.
4. `EXIT` statements are only valid inside their matching loop type. Using `EXIT FOR` outside a `FOR` loop emits `E1100`.
5. `EXIT` may appear inside nested loops, conditionals, or subroutines within the loop.
6. Only one `EXIT` per statement is allowed; `EXIT FOR WHILE` is a syntax error.

---

## 4. AST (node definitions)

```text
ExitFor {}
ExitWhile {}
ExitDo {}
```

These nodes are already defined in RFC-0005 §4.20, §4.21, §4.22.

---

## 5. Parsing

When `EXIT` keyword is encountered:

1. Consume `EXIT`.
2. The next token must be `FOR`, `WHILE`, or `DO`.
3. Produce the corresponding AST node.

```rust
fn parse_exit() -> Result<Statement> {
    consume(Exit);
    match peek() {
        For  => { advance(); Ok(Statement::ExitFor {}) }
        While => { advance(); Ok(Statement::ExitWhile {}) }
        Do   => { advance(); Ok(Statement::ExitDo {}) }
        _    => Err("Expected FOR, WHILE, or DO after EXIT"),
    }
}
```

---

## 6. Semantic Analysis

1. **Context validation** — `EXIT FOR` must be inside a `FOR` loop, `EXIT WHILE` inside a `WHILE` loop, `EXIT DO` inside a `DO` loop. Mismatched context emits `E1100`.
2. **Loop tracking** — the semantic analyzer maintains a loop nesting stack that tracks the current loop type. `EXIT` pops the stack for the matching type.
3. **Valid in any nesting depth** — `EXIT` inside nested loops only affects the innermost matching loop.

---

## 7. Code Generation

Each `EXIT` statement compiles to a `break` in the corresponding Rust loop construct:

### EXIT FOR

```basic
FOR i = 1 TO 100
    IF i = 50 THEN EXIT FOR
    PRINT i
NEXT i
```

Compiles to:

```rust
for i in 1..=100 {
    if i == 50 {
        break;
    }
    println!("{}", i);
}
```

### EXIT WHILE

```basic
WHILE running
    IF done THEN EXIT WHILE
WEND
```

Compiles to:

```rust
while running {
    if done {
        break;
    }
}
```

### EXIT DO

```basic
DO WHILE TRUE
    IF quit THEN EXIT DO
LOOP
```

Compiles to:

```rust
while true {
    if quit {
        break;
    }
}
```

---

## 8. Error Codes

| Code  | Description                                    |
|-------|------------------------------------------------|
| E1100 | EXIT used outside matching loop type            |

---

## 9. Acceptance Criteria

```text
✓ EXIT FOR parsed as ExitFor
✓ EXIT WHILE parsed as ExitWhile
✓ EXIT DO parsed as ExitDo
✓ EXIT FOR outside FOR loop produces E1100
✓ EXIT WHILE outside WHILE loop produces E1100
✓ EXIT DO outside DO loop produces E1100
✓ EXIT inside nested loop affects only innermost loop
✓ EXIT compiles to correct Rust break statement
✓ Full test suite passes
```
