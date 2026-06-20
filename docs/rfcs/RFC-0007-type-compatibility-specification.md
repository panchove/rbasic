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

RBASIC v0.1 uses **strict compatibility** with one exception: integer literals and same-family integer types support implicit widening (see §4a).

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

## 4a. Implicit Integer Widening

Within the same sign family, narrower integer types are implicitly compatible with wider ones:

```
Signed:   I8 → I16 → I32 → I64
Unsigned: U8 → U16 → U32 → U64
```

An integer literal (`Literal::Int`) resolves as `I32` by default and is implicitly compatible with any integer type. Signed and unsigned types are **not** mutually compatible (produces `E1021`).

F32 and F64 are mutually compatible (F32 widens to F64).

Explicit cross-type casts use the `AS` keyword and are governed by `can_cast_explicitly()` in the semantic analyzer.

## 11. Future Compatibility

Possible future additions:
- I32 → F64 promotion
- User‑defined conversions

## 12. Acceptance Criteria

- Compatibility matrix defined
- Diagnostics defined
- Unary and binary rules defined
- Function return and argument compatibility defined
- Examples provided
- No ambiguity remains
