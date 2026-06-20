# RFC-0003: Language Feature Phases

Status: Accepted
Version: 0.2
Author: RBASIC Project
Created: 2026-06-17
Last Updated: 2026-06-20

---

## 1. Summary

This RFC defines the feature phases for RBASIC. Features are organized in three phases: Phase 1 (QuickBASIC-compatible core), Phase 2 (modern extensions), and Phase 3 (future features requiring generics). QuickBASIC is the source of truth for Phase 1 syntax and semantics.

---

## 2. Phase 1 — QuickBASIC Core

The foundation. All syntax and semantics are defined by QuickBASIC as the canonical reference.

### 2.1 Variables

- **✓** `LET` (optional keyword)
- **✓** Standalone assignment (`x = expr`)
- **✓** `OPTION EXPLICIT` (force variable declaration)
- **✓** `OPTION BASE 1` (change default array lower bound)

### 2.2 Functions and Subroutines

- **✓** `FUNCTION...END FUNCTION` (assign to function name, no `RETURN`)
- **✓** `SUB...END SUB` / `CALL`
- **✓** `BYVAL` / `BYREF` / `OPTIONAL` parameter passing
- **✓** `PUBLIC` / `PRIVATE` (module visibility)

### 2.3 Control Flow

- **✓** `PRINT`
- **✓** `INPUT`
- **✓** `IF...THEN...ELSEIF...ELSE...END IF`
- **✓** `SELECT CASE...CASE...END SELECT`
- **✓** `WHILE...WEND`
- **✓** `FOR...NEXT...STEP`
- **✓** `DO/LOOP` (4 variants: WHILE/UNTIL, pre/post)
- **✓** `EXIT FOR` / `EXIT WHILE` / `EXIT DO`
- **✓** `GOTO` / `GOSUB` / `RETURN` (GOSUB)
- **✓** `ON...GOTO` / `ON...GOSUB`
- **✓** `DO EVENTS`

### 2.4 Data Structures

- **✓** `DIM` (arrays, with `TO` bounds)
- **✓** `TYPE...END TYPE` (user-defined types)
- **✓** Fixed-length strings (`AS STRING * 10`)

### 2.5 Error Handling

- **✓** `ON ERROR GOTO` / `RESUME` / `RESUME label`

### 2.6 Types

- **✓** `BOOL`, `I8`, `I16`, `I32`, `I64`
- **✓** `U8`, `U16`, `U32`, `U64`
- **✓** `F32`, `F64`
- **✓** `STRING`
- **✓** Type aliases (`INTEGER`, `LONG`, `DOUBLE`, `SINGLE`, `BOOLEAN`, `BYTE`, `WORD`, `LONGLONG`, `DWORD`, `QWORD`)

### 2.7 Operators

- **✓** Arithmetic: `+`, `-`, `*`, `/`, `^`, `\`, `MOD`
- **✓** Bitwise: `SHL`, `SHR`
- **✓** Logical: `AND`, `OR`, `XOR`, `NOT`
- **✓** Relational: `=`, `<>`, `<`, `>`, `<=`, `>=`
- **✓** String concatenation: `+`, `&`

### 2.8 Standard Library

- **✓** String functions: `LEN`, `MID$`, `LEFT$`, `RIGHT$`, `CHR$`, `ASC`, `INSTR`, `VAL`, `STR$`, `UCASE$`, `LCASE$`, `TRIM$`, `LTRIM$`, `RTRIM$`, `SPACE$`, `STRING$`
- **✓** Math functions: `ABS`, `SQR`, `SIN`, `COS`, `TAN`, `ATN`, `LOG`, `EXP`, `INT`, `FIX`, `SGN`, `RND`, `ROUND`
- **✓** Type conversion: `CINT`, `CLNG`, `CSNG`, `CDBL`, `CSTR`, `CBOOL`, `CVI`, `CVS`, `CVD`
- **✓** File I/O: `OPEN`, `CLOSE`, `INPUT#`, `PRINT#`, `LINE INPUT#`, `EOF`, `LOF`, `FREEFILE`, `SEEK`, `MKI$`/`MKS$`/`MKD$`, `FILEATTR`
- **✓** Memory: `PEEK`, `POKE`
- **✓** System: `TIMER`, `DATE$`, `TIME$`, `ENVIRON$`, `COMMAND$`, `SHELL`, `SYSTEM`, `END`

### 2.9 Lexical

- **✓** UTF‑8 encoding
- **✓** `.rbas` file extension
- **✓** Case‑insensitive keywords, case‑preserving identifiers
- **✓** Single‑line comments with `'`
- **✓** `$` suffix for string identifiers

---

## 3. Phase 2 — Modern Extensions

Extends QuickBASIC with safety, expressiveness, and modern features.

### 3.1 Variables

- **✓** `LET MUT` (explicit mutability)

### 3.2 Functions

- **✓** Typed declarations with `:` syntax and `RETURNS`
- **✓** `RETURN` in functions (early return)

### 3.3 Types

- **✓** Expanded primitive types (I8–I64, U8–U64, F32, F64)
- **✓** `ENUM...END ENUM`
- **✓** `MODULE...END MODULE` / `IMPORT`

### 3.4 Control Flow

- **✓** `CONTINUE FOR` / `CONTINUE WHILE` / `CONTINUE DO`
- **✓** `FOR EACH...END FOR`

### 3.5 Operators

- **✓** Compound assignment: `+=`, `-=`, `*=`, `/=`, `^=`, `\=`
- **✓** `AS` type cast

### 3.6 Strings

- **✓** String interpolation (`f"..."`)
- **✓** Raw strings (`raw"..."`)
- **✓** Multi‑line strings

### 3.7 Arrays

- **✓** `array<T, N>` generic syntax
- **✓** `[T; N]` fixed‑size arrays (stack‑allocated)
- **✓** Array literals `{}` with type inference
- **✓** Slicing `arr[2 TO 5]`

### 3.8 TYPE (Modern)

- **✓** `:` syntax for field types
- **✓** Default values in fields

### 3.9 Visibility (Extended)

- **✓** `PUBLIC`/`PRIVATE` on STRUCT fields
- **✓** `PUBLIC`/`PRIVATE` on MODULE exports

### 3.10 C Interop

- **✓** `DECLARE LIBRARY` (static linking)
- **✓** `DECLARE DYNAMIC LIBRARY` (runtime loading)

### 3.11 Concurrency

- **✓** `ASYNC` / `AWAIT`
- **✓** `GO` (fire‑and‑forget)

### 3.12 Memory

- **✓** `Ref<T>` (strong reference, ARC)
- **✓** `Weak<T>` (weak reference, breaks cycles)

### 3.13 Option

- **✓** `OPTION IMPLICIT` (legacy mode opt‑in)

---

## 4. Phase 3 — Future Features

Requires generics.

- **✓** `Optional<T>`
- **✓** `Result<T, E>`

---

## 5. Excluded Features

- Classic inheritance.
- Complex classes.
- Garbage collection.
- Advanced macros.
- Reflection.
- Metaprogramming.
- Advanced concurrency (beyond ASYNC/AWAIT/GO).
- Complex generics (beyond Phase 3).

---

## 6. Rationale

The three‑phase model allows incremental implementation: Phase 1 delivers a complete QuickBASIC-compatible language, Phase 2 adds modern safety and expressiveness, and Phase 3 adds advanced type system features. Each phase builds on the previous without breaking compatibility.

---

## 7. Acceptance Criteria

```
✓ Phase 1 features match QuickBASIC semantics
✓ Phase 2 features extend Phase 1 without breaking it
✓ Phase 3 features are blocked on generics
✓ All features have corresponding RFCs or are covered by this RFC
✓ Documented in DOCUMENTO_DE_INTENCION.md Section 9
```
