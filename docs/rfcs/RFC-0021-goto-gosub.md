# RFC-0021: GOTO and GOSUB Statements

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add `GOTO`, `GOSUB`, and `RETURN` statements to RBASIC, enabling classic BASIC-style unconditional jumps and subroutine calls. Labels provide jump targets within the same scope. `GOTO` transfers control unconditionally; `GOSUB` pushes a return address so that `RETURN` can resume after the `GOSUB`.

---

## 2. Syntax (EBNF)

```ebnf
goto_stmt   ::= "GOTO" IDENTIFIER
gosub_stmt  ::= "GOSUB" IDENTIFIER
return_stmt ::= "RETURN"
label_decl  ::= IDENTIFIER ":"
```

- `GOTO`, `GOSUB`, `RETURN` are case-insensitive reserved keywords.
- A label is declared by the pattern `identifier ":"` on its own line.
- Labels are valid as statement-level constructs anywhere in a program.
- `RETURN` transfers control back to the instruction after the most recent `GOSUB`.

Examples:

```basic
GOTO start
PRINT "skipped"
start:
PRINT "hello"
```

```basic
GOSUB helper
PRINT "back from helper"
GOTO end

helper:
    PRINT "in subroutine"
    RETURN

end:
```

---

## 3. Semantics

### GOTO

1. `GOTO label` transfers control to the statement following `label:`.
2. The label must be declared in the current scope.
3. `GOTO` is unconditional; no expression is evaluated.
4. Jumping into a loop body or block is undefined behavior (diagnostic emitted at compile time if detectable).

### GOSUB

1. `GOSUB label` pushes the next instruction address onto a return stack, then transfers control to `label:`.
2. `RETURN` (with no arguments) pops the return address and resumes execution after the `GOSUB` that called it.
3. `GOSUB` calls must be balanced: every `GOSUB` must eventually reach a `RETURN` before the enclosing function returns.
4. Nested `GOSUB` calls are supported; each `RETURN` matches the most recent `GOSUB`.

### Labels

1. Labels are identifiers followed by a colon, on their own line.
2. Label names are case-insensitive.
3. Duplicate labels in the same scope emit `E1070`.
4. Undeclared label references emit `E1071`.

---

## 4. AST (node definitions)

```text
Goto {
    label: String,
}

Gosub {
    label: String,
}

Return {
    expr: Option<Expression>,
}

Label {
    name: String,
}
```

These nodes are already defined in RFC-0005 §4.13, §4.14, §4.4, §4.29.

---

## 5. Parsing

### Labels

A label is recognized when an `IDENTIFIER` token is followed by a `Colon` token and the line ends. Labels are parsed as top-level statements:

```rust
fn parse_label_or_statement() -> Result<Statement> {
    match peek() {
        Identifier(name) if peek_ahead() == Colon => {
            advance(); // consume identifier
            advance(); // consume colon
            Ok(Statement::Label { name })
        }
        _ => parse_statement(),
    }
}
```

### GOTO / GOSUB / RETURN

- `GOTO` followed by an identifier produces `Statement::Goto`.
- `GOSUB` followed by an identifier produces `Statement::Gosub`.
- `RETURN` alone produces `Statement::Return { expr: None }`.

---

## 6. Semantic Analysis

1. **Label must exist** — `GOTO` and `GOSUB` targets must reference a declared label. Emit `E1071` if not found.
2. **No duplicate labels** — duplicate label names in the same scope emit `E1070`.
3. **GOSUB must have matching RETURN** — static analysis may warn if `GOSUB` has no reachable `RETURN` (warning, not error).
4. **RETURN outside GOSUB** — `RETURN` with no enclosing `GOSUB` in the same scope emits `E1072`.
5. **Jump validation** — jumping into a nested block (e.g., into a FOR loop body) emits `E1073` as a warning.

---

## 7. Code Generation

### GOTO

```basic
GOTO end
```

Compiles to a Rust `goto` equivalent (loop + break or labeled block):

```rust
// Using a labeled loop with break
'block: {
    // ... statements before GOTO ...
    break 'block;
    // ... skipped statements ...
    label_end:
    // ... statements after label ...
}
```

Alternatively, the codegen may use a `loop { break; }` idiom for the jump target.

### GOSUB / RETURN

```basic
GOSUB helper
' ...
helper:
    PRINT "sub"
    RETURN
```

Compiles to a function call:

```rust
fn helper() {
    println!("sub");
}
// ...
helper();
```

Each `GOSUB` target label generates an inline function, and `RETURN` maps to a `return` from that function.

---

## 8. Error Codes

| Code  | Description                                     |
|-------|-------------------------------------------------|
| E1070 | Duplicate label name in scope                   |
| E1071 | Undefined label (GOTO/GOSUB target not found)   |
| E1072 | RETURN outside GOSUB context                    |
| E1073 | GOTO into nested block (warning)                |

---

## 9. Acceptance Criteria

```text
✓ GOTO parsed as Statement::Goto
✓ GOSUB parsed as Statement::Gosub
✓ RETURN parsed as Statement::Return
✓ Label ":" syntax parsed as Statement::Label
✓ Undefined label produces E1071
✓ Duplicate label produces E1070
✓ RETURN outside GOSUB produces E1072
✓ GOTO correctly redirects control flow in codegen
✓ GOSUB + RETURN correctly pair in codegen
✓ Full test suite passes
```
