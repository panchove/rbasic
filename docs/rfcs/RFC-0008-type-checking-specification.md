# RFC-0008: Type Checking Specification

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-07-01
Last Updated: 2026-06-19

---

# 1. Summary

This RFC defines static type checking rules for RBASIC v0.1. It builds upon RFC-0007 (Type Compatibility) to enforce type safety during semantic analysis.

---

# 2. Scope

This RFC defines:

* Argument count validation for function calls
* Return type validation for function bodies
* Boolean condition validation for IF/WHILE statements
* Top-level return statement rejection
* Final semantic analyzer acceptance criteria

---

# 3. Argument Count Validation

Every function call must provide exactly the number of arguments declared in the function signature.

```ebnf
call ::= IDENTIFIER "(" arguments ")"
arguments ::= expression ("," expression)*
```

The semantic analyzer must validate that the number of arguments matches the parameter count.

---

# 4. Return Type Validation

A function's return statements must:

* Return a value of the declared return type
* Not appear at top-level (only inside functions)

```ebnf
return_stmt ::= "RETURN" expression?
```

The return expression must be type-compatible with the declared return type.

---

# 5. Boolean Condition Validation

IF and WHILE conditions must evaluate to BOOL.

```ebnf
if_stmt ::= "IF" expression "THEN" block ...
while_stmt ::= "WHILE" expression block ...
```

The expression in the condition position must have type BOOL.

---

# 6. Top-Level Return Rejection

RETURN statements at the top level are invalid. They must only appear inside function bodies.

---

# 7. Semantic Analyzer Acceptance Criteria

The semantic analyzer must:

* Validate all argument counts
* Validate all return types
* Validate all control flow conditions
* Reject top-level returns
* Produce structured diagnostics for all violations

---

# 8. Error Codes

| Code  | Description |
|-------|-------------|
| E1030 | Argument count mismatch |
| E1031 | Return type mismatch |
| E1032 | Invalid condition type (expected BOOL) |
| E1033 | Return outside function body |

---

# 9. Acceptance Criteria

```text
✓ Argument count validation implemented
✓ Return type validation implemented
✓ Boolean condition validation implemented
✓ Top-level return rejection implemented
✓ All error codes E1030-E1033 implemented
✓ Semantic compatibility tests passing
```