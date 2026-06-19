# RFC-0006: Semantic Analysis Specification

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-06-19
Last Updated: 2026-06-19

---

## 1. Summary

This RFC defines the semantic analysis phase for RBASIC v0.1. The semantic analyzer takes the AST (RFC-0005) and performs name resolution, duplicate detection, type validation, and control flow checks. It serves as the bridge between the parser and code generation.

---

## 2. Scope

### Phase 1 — Name Resolution & Duplicate Detection (this RFC)

- Global variable resolution
- Local variable resolution
- Function resolution
- Nested scope resolution
- Case-insensitive symbol lookup
- Duplicate declaration detection (globals, locals, functions, parameters)
- Unknown type validation

### Phase 2 — Type Resolution (RFC-0007)

- Type annotation validation
- Primitive type matching

### Phase 3 — Type Compatibility (RFC-0008)

- Operator type checking
- Argument count and type matching
- Return type validation
- Boolean condition validation

---

## 3. Two-Pass Architecture

### Pass 1: Declaration Collection

The analyzer first scans all top-level statements to collect:

- **Global variables**: name → () mapping (case-insensitive)
- **Function declarations**: name → AST node mapping (case-insensitive)

During this pass, type annotations on variable and function declarations are validated against the set of known primitive types: `BOOL`, `F64`, `I32`, `STRING`.

### Pass 2: Recursive Resolution

The analyzer walks all statements recursively, maintaining a local symbol table at each scope level. For each expression, it resolves identifiers against the current local scope, then falls back to the global scope.

---

## 4. Scopes

### 4.1 Global Scope

Top-level variables are registered during the first pass and are visible everywhere, including inside functions.

### 4.2 Function Scope

Each function introduces a new scope. Parameters are registered as local variables before the function body is analyzed.

### 4.3 Control Flow Scopes

`IF`/`ELSE` and `WHILE` bodies each receive a **copy** of the parent scope. Variables declared inside a branch do not leak to the enclosing scope.

### 4.4 Shadowing

Local variables shadow globals within the function/block scope.

---

## 5. Error Codes

| Code  | Description                                        | Phase |
|-------|----------------------------------------------------|-------|
| E1001 | Unknown variable (not declared in any visible scope) | 1   |
| E1002 | Duplicate variable (already declared in current scope) | 1 |
| E1003 | Unknown function (call to undeclared function)     | 1     |
| E1004 | Duplicate function (redeclared at global scope)    | 1     |
| E1010 | Unknown type (type annotation not recognized)      | 1     |
| E1011 | Duplicate parameter (same name in parameter list)  | 1     |
| E1020 | Type mismatch (RFC-0007)                           | 3     |
| E1021 | Invalid binary operation (RFC-0007)                | 3     |
| E1022 | Invalid unary operation (RFC-0007)                 | 3     |
| E1030 | Argument count mismatch (RFC-0008)                 | 3     |
| E1031 | Return type mismatch (RFC-0008)                    | 3     |
| E1032 | Invalid condition type (RFC-0008)                  | 3     |
| E1033 | Return outside function body (RFC-0008)            | 3     |
| E1034 | Reserved                                           | —     |

---

## 6. Case Insensitivity

All symbol lookups are case-insensitive. Identifiers are lowercased before being stored in and retrieved from symbol tables.

```basic
LET Counter = 10
PRINT counter   ' resolves to Counter
```

```basic
FUNCTION Greet()
END FUNCTION

greet()   ' resolves to Greet
```

---

## 7. Type Validation

### 7.1 Known Primitive Types

```text
BOOL
I32
F64
STRING
```

Type names are case-insensitive (`i32`, `I32`, `I32` are all valid).

### 7.2 Type Annotations

Variable declarations and function parameters with type annotations are validated against the known type set. An unknown type produces **E1010**.

Function return type annotations are validated the same way.

---

## 8. Current Implementation Status

### Implemented (Phase 1)

- Global variable collection and duplicate detection (E1002)
- Function collection and duplicate detection (E1004)
- Parameter duplicate detection (E1011)
- Unknown variable resolution (E1001)
- Unknown function resolution (E1003)
- Unknown type validation (E1010)
- Case-insensitive lookup
- Nested scope with local shadowing
- Control flow scope isolation (IF/WHILE bodies use cloned scopes)

### Not Yet Implemented (Phases 2-3)

- Type inference for expressions
- Operator type compatibility (RFC-0007)
- Argument count validation (RFC-0008)
- Return type checking (RFC-0008)
- Boolean condition validation (RFC-0008)
- Top-level return rejection (RFC-0008)

---

## 9. Acceptance Criteria

```text
✓ Global variable resolution
✓ Local variable resolution
✓ Function resolution
✓ Case-insensitive symbol lookup
✓ Duplicate variable detection (E1002)
✓ Duplicate function detection (E1004)
✓ Duplicate parameter detection (E1011)
✓ Unknown variable error (E1001)
✓ Unknown function error (E1003)
✓ Unknown type error (E1010)
✓ Nested scope resolution
✓ Parameter resolution inside function bodies
✓ Phase 1 unit tests passing
```
