# RFC-0017: String Functions

Status: Accepted
Version: 0.2
Author: RBASIC Project
Created: 2026-06-19
Last Updated: 2026-06-19

---

## 1. Summary

Add classic BASIC string manipulation functions to RBASIC: `LEN`, `MID$`, `LEFT$`, `RIGHT$`, `CHR$`, `ASC`, `INSTR`, `VAL`, `STR$`, `UCASE$`, `LCASE$`, `TRIM$`, `LTRIM$`, `RTRIM$`, `SPACE$`, and `STRING$`. These are built-in functions (not user-defined), recognized by name at parse/semantic time, and mapped to corresponding Rust standard library equivalents in codegen.

---

## 2. Syntax

All string functions use the standard function call syntax already supported by RBASIC:

```basic
result = LEN(s)
result = MID$(s, start, length)
result = LEFT$(s, n)
result = RIGHT$(s, n)
result = CHR$(n)
result = ASC(s)
result = INSTR(s, search)
result = INSTR(start, s, search)
result = VAL(s)
result = STR$(n)
result = UCASE$(s)
result = LCASE$(s)
result = TRIM$(s)
result = LTRIM$(s)
result = RTRIM$(s)
result = SPACE$(n)
result = STRING$(n, ch)
```

The `$` suffix in function names follows classic BASIC convention, indicating the function returns a `STRING`.

---

## 3. Function Table

| Function | Args | Return  | Description                           | Rust equivalent               |
|----------|------|---------|---------------------------------------|-------------------------------|
| `LEN`    | s    | `I32`   | Length of string in characters        | `s.len() as i32`              |
| `MID$`   | s, start, len | `STRING` | Extract substring | `s[start..start+len].to_string()` |
| `LEFT$`  | s, n | `STRING` | Leftmost n characters                 | `s[..n].to_string()`          |
| `RIGHT$` | s, n | `STRING` | Rightmost n characters                | `s[s.len()-n..].to_string()`  |
| `CHR$`   | n    | `STRING` | Character from ASCII/Unicode code     | `char::from_u32(n).to_string()` |
| `ASC`    | s    | `I32`    | ASCII code of first character         | `s.chars().next() as i32`     |
| `INSTR`  | [start,] s, search | `I32` | Find substring position (1-based) | `s[start..].find(search)` |
| `VAL`    | s    | `F64`    | Convert string to number              | `s.parse::<f64>()`            |
| `STR$`   | n    | `STRING` | Convert number to string              | `n.to_string()`               |
| `UCASE$` | s    | `STRING` | Convert to uppercase                  | `s.to_uppercase()`            |
| `LCASE$` | s    | `STRING` | Convert to lowercase                  | `s.to_lowercase()`            |
| `TRIM$`  | s    | `STRING` | Remove leading and trailing whitespace| `s.trim().to_string()`        |
| `LTRIM$` | s    | `STRING` | Remove leading whitespace             | `s.trim_start().to_string()`  |
| `RTRIM$` | s    | `STRING` | Remove trailing whitespace            | `s.trim_end().to_string()`    |
| `SPACE$` | n    | `STRING` | String of n spaces                    | `" ".repeat(n)`               |
| `STRING$`| n, ch| `STRING` | String of n repeated characters       | `ch.repeat(n)`                |

### Edge Cases

| Function | Edge case | Behaviour |
|----------|-----------|-----------|
| `LEN`    | Empty string | Returns 0 |
| `MID$`   | start or length out of bounds | Returns empty string (no panic) |
| `LEFT$`  | n > string length | Returns full string |
| `RIGHT$` | n > string length | Returns full string |
| `CHR$`   | Invalid Unicode code point | Returns empty string |
| `ASC`    | Empty string | Returns 0 |
| `INSTR`  | Not found | Returns 0 (1-based, 0 = not found) |
| `VAL`    | Non-numeric string | Returns 0 (silent, matching BASIC convention) |
| `STR$`   | Integer | Returns string representation |
| `STR$`   | Float | Returns string representation with decimal |

---

## 4. AST

No new AST nodes are needed. Built-in string functions reuse the existing `Expression::Call { callee, args }` node. The semantic analyzer distinguishes built-in functions from user-defined functions by checking against a known function name table.

---

## 5. Parsing

No parser changes needed. The existing function call syntax `name(args...)` already parses correctly into `Expression::Call`. Callee names with `$` suffix (e.g., `MID$`) are valid identifiers in the lexer.

---

## 6. Semantic Analysis

### 6.1 Built-in Function Registry

The semantic analyzer maintains a registry of built-in function signatures. When resolving `Expression::Call`, if the callee is not found in user-defined functions, the built-in registry is checked before producing E1003.

### 6.2 Signatures

| Function | Parameter types | Return type |
|----------|----------------|-------------|
| `LEN`    | `(STRING)`     | `I32`       |
| `MID$`   | `(STRING, I32, I32)` | `STRING` |
| `LEFT$`  | `(STRING, I32)` | `STRING`   |
| `RIGHT$` | `(STRING, I32)` | `STRING`   |
| `CHR$`   | `(I32)`        | `STRING`    |
| `ASC`    | `(STRING)`     | `I32`       |
| `INSTR`  | `(I32, STRING, STRING)` or `(STRING, STRING)` | `I32` |
| `VAL`    | `(STRING)`     | `F64`       |
| `STR$`   | `(F64)`        | `STRING`    |
| `UCASE$` | `(STRING)`     | `STRING`    |
| `LCASE$` | `(STRING)`     | `STRING`    |
| `TRIM$`  | `(STRING)`     | `STRING`    |
| `LTRIM$` | `(STRING)`     | `STRING`    |
| `RTRIM$` | `(STRING)`     | `STRING`    |
| `SPACE$` | `(I32)`        | `STRING`    |
| `STRING$`| `(I32, STRING)` | `STRING`   |

### 6.3 Type Checking

Argument types are validated against the registered signature using the existing `types_compatible` function. Implicit widening rules apply (e.g., `I32` literal can be passed where `F64` is expected, per RFC-0007).

### 6.4 Error Codes

No new error codes needed. Existing codes cover all cases:
- E1030: Wrong number of arguments
- E1020: Type mismatch in argument

### 6.5 Name Resolution

Built-in function names take precedence over... they don't conflict: user-defined functions have names without `$` suffix typically. If a user defines a function with the same name as a built-in, the user-defined function is shadowed by the built-in (matching standard BASIC practice).

Actually, the `$` suffix convention makes conflicts very unlikely. A user would need to deliberately define `FUNCTION LEN(...)` to clash. We shadow user functions with built-ins for safety.

---

## 7. Code Generation

The codegen needs a new helper to translate built-in function calls to Rust expressions, since the Rust equivalents use method syntax or non-trivial expressions rather than simple function calls.

### Codegen Table

```rust
// LEN(s)       → s.len() as i32
// MID$(s, i, l) → s.chars().skip(i-1).take(l).collect::<String>()
// LEFT$(s, n)   → s.chars().take(n).collect::<String>()
// RIGHT$(s, n)  → s.chars().rev().take(n).collect::<String>().chars().rev().collect()
// CHR$(n)       → String::from(char::from_u32(n as u32).unwrap_or(' '))
// ASC(s)        → s.chars().next().map(|c| c as i32).unwrap_or(0)
// INSTR(s, sub) → s.find(&sub).map(|i| i as i32 + 1).unwrap_or(0)
// INSTR(st, s, sub) → s[st as usize..].find(&sub).map(|i| (i + st) as i32 + 1).unwrap_or(0)
// VAL(s)        → s.trim().parse::<f64>().unwrap_or(0.0)
// STR$(n)       → n.to_string()
// UCASE$(s)     → s.to_uppercase()
// LCASE$(s)     → s.to_lowercase()
// TRIM$(s)      → s.trim().to_string()
// LTRIM$(s)     → s.trim_start().to_string()
// RTRIM$(s)     → s.trim_end().to_string()
// SPACE$(n)     → " ".repeat(n as usize)
// STRING$(n, ch)→ ch.repeat(n as usize)
```

For `INSTR`, the two-argument form (`INSTR(s, sub)`) and three-argument form (`INSTR(start, s, sub)`) are both supported, with start being 1-based.

---

## 8. Interaction with Existing Features

### 8.1 Assignment

String functions work naturally with standalone assignment (RFC-0015):

```basic
LET MUT s: STRING = "hello"
PRINT LEN(s)           ' 5
s = UCASE$(s)          ' "HELLO"
s = s + SPACE$(1) + "world"  ' "HELLO world"
```

### 8.2 Nested Calls

```basic
PRINT LEFT$(UCASE$("hello"), 3)  ' "HEL"
PRINT VAL(MID$("42abc", 1, 2))   ' 42.0
```

### 8.3 Non-$ aliases

For programmers who prefer non-$ naming, lowercase aliases without `$` are also accepted: `MID`, `LEFT`, `RIGHT`, `UCASE`, `LCASE`, `TRIM`, `LTRIM`, `RTRIM`, `STR`, `SPACE`, `STRING`.

---

## 9. Implementation Plan

1. Add built-in function registry to semantic analyzer
2. Modify `Expression::Call` resolution to check built-in names before E1003
3. Add codegen helper `gen_builtin_call(callee, args, out)` that translates each function to Rust
4. Add tests: semantic (correct usage, wrong args, type mismatches), codegen (correct Rust emission), integration (runtime verification)

---

## 10. Acceptance Criteria

```
✓ LEN("hello") returns 5
✓ MID$("hello", 2, 3) returns "ell"
✓ LEFT$("hello", 2) returns "he"
✓ RIGHT$("hello", 2) returns "lo"
✓ CHR$(65) returns "A"
✓ ASC("A") returns 65
✓ INSTR("hello", "ll") returns 3
✓ VAL("42.5") returns 42.5
✓ STR$(42) returns "42"
✓ UCASE$("hello") returns "HELLO"
✓ LCASE$("HELLO") returns "hello"
✓ TRIM$("  hi  ") returns "hi"
✓ SPACE$(3) returns "   "
✓ Functions work in expressions (nested calls, with assignment)
✓ Wrong argument types produce semantic errors
✓ Wrong argument count produces semantic errors
✓ Full test suite passes
```
