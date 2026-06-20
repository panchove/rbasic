# RFC-0013: ON ERROR / RESUME Statements

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add error handling directives to RBASIC using the classic BASIC pattern: `ON ERROR GOTO label` sets an error handler, and `RESUME` transfers control after error recovery. Parsing and semantic analysis are implemented in v0.1; code generation is deferred.

---

## 2. Syntax

```ebnf
on_error_stmt ::= "ON" "ERROR" "GOTO" IDENTIFIER
resume_stmt   ::= "RESUME" IDENTIFIER?
```

Examples:

```basic
ON ERROR GOTO handler
' ... code that may error ...
RESUME
```

---

## 3. AST

### OnError Statement

```text
OnError {
    label: String,
}
```

Sets the error handler to the given label name.

### Resume Statement

```text
Resume {
    label: Option<String>,
}
```

Resumes execution after error handling. Optional label for RESUME NEXT / RESUME label patterns.

---

## 4. Parsing

- `ON`, `ERROR`, `GOTO`, `RESUME` are keyword tokens (already in the lexer).
- `ON ERROR GOTO` followed by an identifier (the label name).
- `RESUME` optionally followed by an identifier.

---

## 5. Semantic Analysis

- `ON ERROR GOTO` and `RESUME` are accepted without validation of the target label in v0.1.
- No errors are produced for undeclared labels (label resolution is out of scope for v0.1).
- These statements are passive in the semantic analyzer — they do not affect type checking.

---

## 6. Code Generation

Code generation for ON ERROR / RESUME is **deferred** to a future version. The codegen emits nothing for these nodes.

---

## 7. Error Codes

None in v0.1. Future versions may add label resolution errors.

---

## 8. Acceptance Criteria

```text
✓ ON ERROR GOTO parsed correctly
✓ RESUME / RESUME label parsed correctly
✓ AST nodes defined (OnError, Resume)
✓ Semantic analysis accepts both forms without error
✓ Codegen emits nothing (deferred)
✓ Full test suite passes
```
