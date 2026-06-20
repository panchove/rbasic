# RFC-0031: ASYNC/AWAIT/GO (Concurrency)

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add lightweight concurrency to RBASIC via `ASYNC`, `AWAIT`, and `GO`. `ASYNC` spawns a concurrent task, `AWAIT` waits for its result, and `GO` launches a fire-and-forget task. This maps to Rust's `tokio` async runtime and green threads.

---

## 2. Syntax (EBNF)

```ebnf
async_stmt    ::= "ASYNC" IDENTIFIER "=" expression
await_stmt    ::= "AWAIT" IDENTIFIER
go_stmt       ::= "GO" expression
```

- `ASYNC`, `AWAIT`, `GO` are case-insensitive reserved keywords.
- `ASYNC task = expr` evaluates `expr` concurrently and stores the handle in `task`.
- `AWAIT task` blocks until `task` completes and returns its result.
- `GO expr` evaluates `expr` concurrently without waiting for the result.
- `expr` in `ASYNC` and `GO` must be a function call or parenthesized expression.
- `AWAIT` on a non-async task emits `E1500`.

Examples:

```basic
ASYNC result = fetch_data("https://api.example.com")
PRINT "Request sent"
DIM data AS STRING
data = AWAIT result
PRINT data
```

```basic
GO log_event("User logged in")
GO send_email("admin@example.com", "Welcome")
PRINT "Tasks launched"
```

```basic
ASYNC t1 = compute(10)
ASYNC t2 = compute(20)
DIM r1 AS INTEGER
DIM r2 AS INTEGER
r1 = AWAIT t1
r2 = AWAIT t2
PRINT r1 + r2
```

---

## 3. Semantics

1. `ASYNC task = expr` spawns `expr` as an async task. The task handle is stored in `task` (type: `Task<T>` where `T` is the expression's return type).
2. `AWAIT task` suspends the current execution until `task` completes, then returns the result.
3. `GO expr` spawns `expr` as a detached task. No handle is returned; the result is discarded.
4. Tasks execute concurrently on the async runtime. The runtime manages scheduling.
5. `AWAIT` can only be used on a `Task<T>` value. Using `AWAIT` on a non-task value emits `E1500`.
6. `ASYNC` cannot be used on `SUB` calls (which return void). Emit `E1501`.
7. `GO` can be used on any expression, but the result is discarded.
8. Task variables are consumed by `AWAIT`; using a consumed task emits `E1502`.

---

## 4. AST (node definitions)

### AsyncStmt

```text
AsyncStmt {
    var_name: String,
    expr:     Box<Expression>,
}
```

### AwaitExpr (Expression)

```text
AwaitExpr {
    task: Box<Expression>,
}
```

### GoStmt

```text
GoStmt {
    expr: Box<Expression>,
}
```

---

## 5. Parsing

### Async Statement

When `ASYNC` is encountered:

1. Consume `ASYNC`.
2. Parse the variable name (identifier).
3. Consume `=`.
4. Parse the expression.
5. Produce `Statement::AsyncStmt { var_name, expr }`.

```rust
fn parse_async() -> Result<Statement> {
    consume(Async);
    let var_name = expect_identifier()?;
    consume(Assign);
    let expr = parse_expression()?;
    Ok(Statement::AsyncStmt {
        var_name,
        expr: Box::new(expr),
    })
}
```

### Await Expression

When `AWAIT` is encountered in expression position:

1. Consume `AWAIT`.
2. Parse the task expression (identifier or member access).
3. Produce `Expression::AwaitExpr { task }`.

```rust
fn parse_await() -> Result<Expression> {
    consume(Await);
    let task = parse_primary()?;
    Ok(Expression::AwaitExpr {
        task: Box::new(task),
    })
}
```

### Go Statement

When `GO` is encountered:

1. Consume `GO`.
2. Parse the expression.
3. Produce `Statement::GoStmt { expr }`.

```rust
fn parse_go() -> Result<Statement> {
    consume(Go);
    let expr = parse_expression()?;
    Ok(Statement::GoStmt {
        expr: Box::new(expr),
    })
}
```

---

## 6. Semantic Analysis

1. **Task type inference** — `ASYNC task = expr` infers the task type as `Task<T>` where `T` is the type of `expr`.
2. **Await validation** — `AWAIT task` validates that `task` is of type `Task<T>`. Non-task types emit `E1500`.
3. **Async expression** — `ASYNC` requires the expression to be a function call (SUB calls are invalid). Emit `E1501`.
4. **Task consumption** — `AWAIT` consumes the task handle. Subsequent `AWAIT` on the same handle emits `E1502`.
5. **Go expression** — `GO` accepts any expression. The return type is discarded.
6. **Concurrency safety** — tasks cannot share mutable state without synchronization. The compiler warns on shared mutable access.

---

## 7. Code Generation

### Async/Await

```basic
ASYNC result = fetch_data("url")
DIM data AS STRING
data = AWAIT result
```

Compiles to:

```rust
let result = tokio::spawn(async { fetch_data("url").await });
let data: String = result.await.unwrap();
```

### Go (Fire and Forget)

```basic
GO log_event("User logged in")
```

Compiles to:

```rust
tokio::spawn(async { log_event("User logged in").await });
```

### Multiple Async Tasks

```basic
ASYNC t1 = compute(10)
ASYNC t2 = compute(20)
DIM r1 AS INTEGER
DIM r2 AS INTEGER
r1 = AWAIT t1
r2 = AWAIT t2
```

Compiles to:

```rust
let t1 = tokio::spawn(async { compute(10).await });
let t2 = tokio::spawn(async { compute(20).await });
let r1: i32 = t1.await.unwrap();
let r2: i32 = t2.await.unwrap();
```

---

## 8. Error Codes

| Code  | Description                                            |
|-------|--------------------------------------------------------|
| E1500 | AWAIT used on non-task value                           |
| E1501 | ASYNC used on SUB call (void expression)               |
| E1502 | AWAIT used on already-consumed task                    |

---

## 9. Acceptance Criteria

```text
✓ ASYNC task = expr parsed as AsyncStmt
✓ AWAIT task parsed as AwaitExpr
✓ GO expr parsed as GoStmt
✓ AWAIT on non-task value produces E1500
✓ ASYNC on SUB call produces E1501
✓ AWAIT on consumed task produces E1502
✓ Task type inferred as Task<T>
✓ Async compiles to tokio::spawn
✓ Await compiles to .await
✓ Go compiles to detached tokio::spawn
✓ Multiple async tasks work correctly
✓ Full test suite passes
```
