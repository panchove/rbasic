# RFC-0011: Classic BASIC Type Aliases

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-06-19
Last Updated: 2026-06-19

---

## 1. Summary

Define a set of traditional BASIC type aliases that map to the canonical RBASIC primitive types. Aliases are accepted anywhere a type name appears (variable declarations, function parameters, return types, AS casts).

---

## 2. Motivation

BASIC programmers are familiar with names like `INTEGER`, `LONG`, `DOUBLE`, and `BOOLEAN`. Supporting these aliases lowers the barrier to entry and makes RBASIC more legible to users coming from BASIC or VBA backgrounds, without adding new types to the type system.

---

## 3. Alias Table

| Alias      | Canonical type | Width   | Notes                        |
|------------|---------------|---------|------------------------------|
| `BOOLEAN`  | `BOOL`        | 1 bit   | VBA / QuickBASIC name        |
| `BYTE`     | `U8`          | 8-bit   | Unsigned byte                |
| `WORD`     | `U16`         | 16-bit  | Unsigned 16-bit integer      |
| `INTEGER`  | `I32`         | 32-bit  | Classic BASIC INTEGER        |
| `LONG`     | `I64`         | 64-bit  | Classic BASIC LONG           |
| `LONGLONG` | `I64`         | 64-bit  | VBA 7.0 / 64-bit alias       |
| `SINGLE`   | `F32`         | 32-bit  | Single-precision float       |
| `DOUBLE`   | `F64`         | 64-bit  | Double-precision float       |
| `STRING`   | `STRING`      | —       | Already canonical; listed for completeness |

The canonical names (`BOOL`, `I8`, `I16`, `I32`, `I64`, `U8`, `U16`, `U32`, `U64`, `F32`, `F64`, `STRING`) remain valid and are preferred in new code.

---

## 4. Lookup Rules

- Type name resolution is **case-insensitive**: `integer`, `Integer`, `INTEGER` all resolve to `I32`.
- Aliases are purely a front-end concern: the semantic analyzer and code generator never see the alias name — only the resolved `Type` variant.
- An unknown type name continues to produce `E1010`.

---

## 5. Examples

```basic
LET age: INTEGER = 25
LET ratio: DOUBLE = 3.14
LET flag: BOOLEAN = TRUE
LET b: BYTE = 255
LET w: WORD = 1024
LET big: LONGLONG = 9999999999
```

Each is semantically identical to:

```basic
LET age: I32 = 25
LET ratio: F64 = 3.14
LET flag: BOOL = TRUE
LET b: U8 = 255
LET w: U16 = 1024
LET big: I64 = 9999999999
```

---

## 6. Implementation

Only `Type::from_name()` in `src/semantic/types.rs` requires changes. No AST, parser, or codegen changes are needed.

---

## 7. Acceptance Criteria

- All aliases in §3 resolve to the correct canonical type
- Aliases are case-insensitive
- Unknown type names still produce E1010
- Integration tests pass using alias names
- CHANGELOG updated
