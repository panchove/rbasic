# RFC-0007: Type Compatibility Specification

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-06-18
Last Updated: 2026-06-20

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

| Type     | Category   | Rust equivalent | Description               |
|----------|------------|-----------------|---------------------------|
| BOOL     | Boolean    | `bool`          | True/false                |
| I8       | Signed int | `i8`            | 8-bit signed integer      |
| I16      | Signed int | `i16`           | 16-bit signed integer     |
| I32      | Signed int | `i32`           | 32-bit signed integer     |
| I64      | Signed int | `i64`           | 64-bit signed integer     |
| U8       | Unsigned   | `u8`            | 8-bit unsigned integer    |
| U16      | Unsigned   | `u16`           | 16-bit unsigned integer   |
| U32      | Unsigned   | `u32`           | 32-bit unsigned integer   |
| U64      | Unsigned   | `u64`           | 64-bit unsigned integer   |
| F32      | Float      | `f32`           | 32-bit single precision   |
| F64      | Float      | `f64`           | 64-bit double precision   |
| STRING   | String     | `String`        | UTF-8 string              |

All type names are case‑insensitive. Classic BASIC aliases (RFC-0011) map to these canonical types: `INTEGER`→I32, `LONG`/`LONGLONG`→I64, `SINGLE`→F32, `DOUBLE`→F64, `BOOLEAN`→BOOL, `BYTE`→U8, `WORD`→U16.

## 4. Compatibility Rules

RBASIC v0.1 uses **strict compatibility** within each type family, with implicit widening for same-family integer types and float mutual promotion.

### 4.1 Same‑family compatibility

Types within the same category are compatible:

```text
Signed integers:   I8 ↔ I16 ↔ I32 ↔ I64   (widening allowed)
Unsigned integers: U8 ↔ U16 ↔ U32 ↔ U64   (widening allowed)
Floats:            F32 ↔ F64               (mutual promotion)
Boolean:           BOOL → BOOL only
String:            STRING → STRING only
```

Signed ↔ unsigned mixing is **always rejected** (produces E1020/E1021).

### 4.2 Cross‑family compatibility

```
              BOOL   I8/I16/I32/I64   U8/U16/U32/U64   F32/F64   STRING
BOOL          ✓      ✗                ✗                ✗         ✗
Signed int    ✗      ✓ (same-family)  ✗                ✓ (→F32/F64) ✗
Unsigned int  ✗      ✗                ✓ (same-family)  ✓ (→F32/F64) ✗
Float         ✗      ✗                ✗                ✓         ✗
STRING        ✗      ✗                ✗                ✗         ✓
```

Integer‑to‑float widening (i.e., any integer to F32 or F64) is allowed implicitly — see §4.3.

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

| Operator | Operand type | Result | Description           |
|----------|-------------|--------|-----------------------|
| `-`      | Signed int  | Same   | Numeric negation      |
| `-`      | Float       | Same   | Numeric negation      |
| `-`      | Unsigned    | ✗ E1022| Unsigned negation     |
| `NOT`    | BOOL        | BOOL   | Logical NOT           |
| `NOT`    | Non‑BOOL    | ✗ E1022| Invalid operand       |

## 9. Binary Operators

### Arithmetic (`+`, `-`, `*`, `/`, `^`, `\`, `MOD`)

Valid for same-family numeric types:
```
Signed int op Signed int → wider of the two types
Unsigned int op Unsigned int → wider of the two types
Float op Float → wider of the two (F32↔F64 → F64)
Signed int op Unsigned int → ✗ E1021
```

Special notes:
- `^` (power) always returns `F64`
- `\` (integer division) and `MOD` require integer types (not F32/F64)
- `+` on STRING concatenates: `STRING + STRING → STRING`

### Bitwise shift (`SHL`, `SHR`)
```
Integer op Integer → left operand's type
```
Both operands must be integers (signed or unsigned, same family not required).

### Equality (`==`, `!=`)
Same‑type comparison always valid, result is `BOOL`:
```
BOOL == BOOL → BOOL
Signed int == Signed int → BOOL
Unsigned int == Unsigned int → BOOL
Float == Float → BOOL
STRING == STRING → BOOL
```

### Relational (`<`, `<=`, `>`, `>=`)
Valid for numeric types and STRING, result is `BOOL`:
```
Signed int < Signed int → BOOL
Unsigned int < Unsigned int → BOOL
Float < Float → BOOL
STRING < STRING → BOOL
```

### Logical (`AND`, `OR`, `XOR`)
```
BOOL AND BOOL → BOOL
BOOL OR BOOL → BOOL
BOOL XOR BOOL → BOOL
```

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

### 4.3 Integer‑to‑Float Widening

Any integer type (signed or unsigned) is implicitly compatible with F32 and F64:

```text
Signed integer (I8/I16/I32/I64) → F32
Signed integer (I8/I16/I32/I64) → F64
Unsigned integer (U8/U16/U32/U64) → F32
Unsigned integer (U8/U16/U32/U64) → F64
```

This enables patterns such as:

```basic
LET x: F64 = 42       ' I32 literal widens to F64
LET y: F64 = x / 3    ' integer division result promotes to F64
```

This widening is consistent with classic BASIC semantics where numeric types promote uniformly to floating point.

Explicit cross-type casts use the `AS` keyword and are governed by `can_cast_explicitly()` in the semantic analyzer.

## 11. Future Compatibility

Possible future additions:
- User‑defined conversions

## 12. Acceptance Criteria

- Compatibility matrix defined
- Diagnostics defined
- Unary and binary rules defined
- Function return and argument compatibility defined
- Examples provided
- No ambiguity remains
