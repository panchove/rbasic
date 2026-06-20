# RFC-0038: DO EVENTS (Cooperative Multitasking)

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add `DO EVENTS` to RBASIC for cooperative multitasking. `DO EVENTS` yields control to the runtime event loop, allowing pending I/O operations, timers, or other tasks to execute before resuming. This is essential for GUI applications and network servers.

---

## 2. Syntax (EBNF)

```ebnf
do_events ::= "DO" "EVENTS"
```

- `DO`, `EVENTS` are case-insensitive reserved keywords.
- `DO EVENTS` is a standalone statement (no loop body).
- It may appear anywhere a statement is valid.
- Multiple `DO EVENTS` calls in sequence yield multiple times.

Examples:

```basic
' Simple event loop
DO WHILE running
    process_input()
    DO EVENTS
    update_display()
WEND
```

```basic
' Network server
DIM server AS SOCKET
server = listen(8080)

DO WHILE TRUE
    DIM client AS SOCKET
    client = accept(server)
    IF client <> 0 THEN
        handle_client(client)
    END IF
    DO EVENTS
LOOP
```

```basic
' Timer-based loop
DIM elapsed AS DOUBLE
elapsed = 0

DO WHILE elapsed < 10.0
    render_frame()
    DO EVENTS
    elapsed = elapsed + delta_time()
LOOP
```

---

## 3. Semantics

1. `DO EVENTS` yields control to the runtime event loop.
2. The runtime processes pending events (I/O completions, timer callbacks, task notifications).
3. After all pending events are processed, execution resumes at the next statement.
4. `DO EVENTS` is a no-op if no events are pending.
5. `DO EVENTS` can appear inside `DO WHILE`, `FOR`, `WHILE`, or any block.
6. `DO EVENTS` does not change variable state or control flow.
7. In async contexts, `DO EVENTS` can process async task completions.
8. Nested `DO EVENTS` calls are allowed but may cause re-entrancy issues (warning emitted).

---

## 4. AST (node definitions)

```text
DoEvents {}
```

---

## 5. Parsing

When `DO` is followed by `EVENTS`:

1. Consume `DO`.
2. Consume `EVENTS`.
3. Produce `Statement::DoEvents {}`.

```rust
fn parse_do_events() -> Result<Statement> {
    consume(Do);
    consume(Events);
    Ok(Statement::DoEvents {})
}
```

---

## 6. Semantic Analysis

1. **Standalone statement** — `DO EVENTS` is a valid statement in any context.
2. **No return value** — `DO EVENTS` does not produce a value.
3. **Re-entrancy warning** — nested `DO EVENTS` calls emit a warning (E2200).
4. **Event loop tracking** — the semantic analyzer marks functions containing `DO EVENTS` as "event-aware" for code generation.

---

## 7. Code Generation

`DO EVENTS` compiles to a call to the runtime's event processing function.

### Basic Event Processing

```basic
DO EVENTS
```

Compiles to:

```rust
runtime::process_events();
```

### Inside Loop

```basic
DO WHILE running
    process_input()
    DO EVENTS
    update_display()
WEND
```

Compiles to:

```rust
while running {
    process_input();
    runtime::process_events();
    update_display();
}
```

### Async Event Processing

In async contexts, `DO EVENTS` processes pending async tasks:

```rust
runtime::process_events_async().await;
```

---

## 8. Error Codes

| Code  | Description                                              |
|-------|----------------------------------------------------------|
| E2200 | Nested DO EVENTS call (re-entrancy warning)              |

---

## 9. Acceptance Criteria

```text
✓ DO EVENTS parsed as DoEvents
✓ DO EVENTS is valid as standalone statement
✓ DO EVENTS inside loops works correctly
✓ DO EVENTS compiles to runtime::process_events()
✓ Nested DO EVENTS produces E2200 warning
✓ DO EVENTS does not affect control flow
✓ DO EVENTS allows event processing in GUI loops
✓ DO EVENTS works with async task completions
✓ Full test suite passes
```
