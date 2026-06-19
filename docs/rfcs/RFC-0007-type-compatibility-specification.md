# RFC-0007: Type Compatibility Specification

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-06-18
Last Updated: 2026-06-18

## 1. Purpose

Define how RBASIC determines whether two types are compatible. This RFC builds on:
- RFC-0005 AST Specification
- RFC-0006 Semantic Analysis Specification

## 2. Scope

Applies to:
- Variable initialization
- Expression evaluation
- Function return values
- Function arguments
- Unary operators
- Binary operators

Does **not** define:
- Generics, Modules, Optional, Result, Ref, MutRef, Ownership, Borrow checking, User‑defined types.

## 3. Supported Primitive Types

- BOOL
- I32
- F64
- STRING

All type names are case‑insensitive.

## 4. Compatibility Matrix

```
              BOOL   I32   F64   STRING
BOOL          ✓     ✗     ✗      ✗
I32           ✗     ✓     ✗      ✗
F64           ✗     ✗     ✓      ✗
STRING        ✗     ✗     ✗      ✓
```

RBASIC v0.1 uses **strict compatibility** – no implicit conversions.

## 5. Variable Initialization Rules

Valid example:
```
LET age: I32 = 42
```
Invalid example (produces `E1020 Type Mismatch`):
```
LET age: I32 = "forty-two"
```

## 6. Function Return Compatibility

Valid:
```
FUNCTION GetAge() RETURNS I32
    RETURN 42
END FUNCTION
```
Invalid (produces `E1020 Type Mismatch`):
```
FUNCTION GetAge() RETURNS I32
    RETURN "forty-two"
END FUNCTION
```

## 7. Function Argument Compatibility

Valid call:
```
PrintAge(42)
```
Invalid call (produces `E1020 Type Mismatch`):
```
PrintAge("forty-two")
```

## 8. Unary Operators

Allowed:
- `NOT BOOL → BOOL`

Any other combination produces `E1022 Invalid Unary Operation`.

## 9. Binary Operators

### Arithmetic
- `I32 + I32 → I32`
- `I32 - I32 → I32`
- `I32 * I32 → I32`
- `I32 / I32 → I32`
- `F64 + F64 → F64`
- `F64 - F64 → F64`
- `F64 * F64 → F64`
- `F64 / F64 → F64`

### Equality
- `BOOL == BOOL → BOOL`
- `I32 == I32 → BOOL`
- `F64 == F64 → BOOL`
- `STRING == STRING → BOOL`

### Relational (ordering)
- `I32 < I32, <=, >, >= → BOOL`
- `F64 < F64, <=, >, >= → BOOL`

All other binary combinations produce `E1021 Invalid Binary Operation`.

## 10. Diagnostics

- **E1020 Type Mismatch** – when an expression’s actual type does not match the expected type.
- **E1021 Invalid Binary Operation** – when a binary operator is applied to incompatible operand types.
- **E1022 Invalid Unary Operation** – when a unary operator is applied to an incompatible operand type.

Each diagnostic must include:
- Code
- Message
- Expected Type(s)
- Actual Type
- Span (source location)

## 11. Future Compatibility

No implicit conversions are permitted in v0.1. Future RFCs may introduce:
- I32 → F64 promotion
- Explicit `CAST` operations
- User‑defined conversions

These are **not** part of RBASIC v0.1.

## 12. Acceptance Criteria

- Compatibility matrix defined
- Diagnostics defined
- Unary and binary rules defined
- Function return and argument compatibility defined
- Examples provided
- No ambiguity remains
