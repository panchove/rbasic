# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog.

## [Unreleased]

### Added
- Explicit AS cast syntax: `expr AS TypeName` for numeric type conversions (i8, i16, i32, i64, u8, u16, u32, u64, f32, f64)
- String escape sequences processed in the lexer: `\\`, `\"`, `\n`, `\r`, `\t` (RFC-0002 §11)
- Structured lexer diagnostics with file, line, column, source snippet, and caret (RFC-0002 §17)
- `diagnostics::format_lex_error` and `diagnostics::offset_to_line_col` exported from crate root
- `Display` impl for `SemanticErrorCode` (clean `{}` formatting)
- Parse and semantic errors in the CLI now report `line:column` instead of byte offsets

### Planned (v0.2)
- Classic BASIC type aliases: `INTEGER`→I32, `LONG`→I64, `SINGLE`→F32, `DOUBLE`→F64, `BYTE`→U8, etc.
- `DIM` statement for array declarations
- Runtime error handling: `ON ERROR GOTO <LABEL>`, `RESUME <LABEL>` / `RESUME NEXT`
- Bitwise operators: `AND`, `OR`, `XOR`, `SHL`, `SHR`
- Logical operators: `AND`, `OR`, `XOR` (con short-circuit para BOOL)
- Semantic analyzer scaffolding
- Semantic Phase 1 verification test suite
- Type Compatibility engine (Phase 3) with diagnostics E1020‑E1022
- RFC‑0008 Type Checking Specification (draft)
- RFC‑0005 AST Specification (accepted)
- RFC‑0006 Semantic Analysis Specification (accepted)
- `NOT` keyword in lexer, token, parser and AST (UnaryOp::Not)
- Real Rust codegen (`generate_rust`) — genera funciones, variables, print, if/else, while, expresiones, llamadas
- 13 tests de codegen end-to-end
- `codegen::rust` module (replaces old `codegen::c`)
- CLI commands: `check` (validate) and `build` (generate Rust code to stdout or file)
- Phase 2 & 3 semantic analysis: type resolution, compatibility checking (E1020-E1022),
  argument count (E1030), return type (E1031), condition type (E1032), top-level return (E1033)
- 39 type checking tests (34 new)
- CLI command `run`: build and execute immediately (invokes rustc internally)
- FOR loop: `FOR var = start TO end ... END FOR` with codegen, semantic checks, and tests
- FOR...STEP loop: `FOR var = start TO end STEP expr ... END FOR` with codegen (direction-aware), semantic checks, and tests
- DO WHILE/UNTIL loop: `DO WHILE cond ... LOOP` and `DO UNTIL cond ... LOOP` (pre-test)
- DO...LOOP WHILE/UNTIL: `DO ... LOOP WHILE cond` and `DO ... LOOP UNTIL cond` (post-test)
- Power operator `^`: `x ^ y` (retorna F64, codegen con `powf`)
- Modulo operator `MOD`: `x MOD y` (solo I32, codegen con `%`)
- Integer division operator `\`: `x \ y` (solo I32, codegen con `/`)
- RFC-0009: FOR...STEP Specification (Accepted)
- RFC-0010: DO Loop Specification (Accepted)
- Examples: `examples/fizzbuzz.rbas` y `examples/fibonacci.rbas` (end-to-end validados)
- Integration tests (4 tests) que compilan y ejecutan ejemplos completos
- Integration tests (4 tests) que compilan y ejecutan ejemplos completos
- `make verify` now passes with **133 tests** (26 codegen + 1 lexer + 8 parser + 82 semantic‑type + 12 semantic + 4 integration)
- Example: `examples/for_step.rbas` and `examples/operators.rbas`
- I64 type support in type system, semantic analyzer, and codegen
- I8 and I16 type support with integer promotion (I8→I16→I32→I64) for all binary ops
- `Type::is_integer()`, `Type::is_numeric()`, `Type::widen_int()` helpers
- Implicit integer widening for var decl, function args, return types, and FOR bounds/step
- `types_compatible()` helper for implicit narrowing of I32 literals to I8/I16/I32/I64
- Test coverage: 15 new tests for I8/I16 (primitives, arithmetic, pow, mod, intdiv, neg, comparison, mixed)
- Codegen tests for I8, I16, I64 declarations
- F32 type support with float promotion (F32↔F64) for all binary ops and unary neg
- `types_compatible()` handles F32→F64 widening and F64 literal→F32 assignment
- 6 new tests for F32 (primitive, arithmetic, mixed F32/F64, neg, pow, comparison)
- Codegen test for F32 declaration
- Unsigned integer types: U8, U16, U32, U64 with full binary op support (arithmetic, pow, mod, intdiv, equality, relational)
- Signed/unsigned family separation: cross-family operations produce E1021, unary neg on unsigned produces E1022
- `Type::is_signed()` / `Type::is_unsigned()` helpers
- 30 new tests for unsigned types (primitives, arithmetic, pow, mod, intdiv, comparison, mixed unsigned, neg-fail, signed/unsigned mixed-fail)
- Codegen tests for U8, U16, U32, U64 declarations
- `make verify` now passes with **184 tests**
### Changed
- `rbasic run`: now cleans up temporary `.rs` and binary files after execution
- Semantic analyzer binary‑op match refactored: integer‑integer operations use automatic widening via `Type::widen_int()`, explicit match only for non‑integer types (F64, String, Bool)
- RFC-0002: token inventory includes `For`, `To`, `Step`, `Do`, `Loop`, `Until`
- Backend strategy: `RBASIC → Rust → rustc (LLVM)` replaces `RBASIC → C → GCC/Clang`
- `DOCUMENTO_DE_INTENCION.md`: backend section corrected, RBA section added
- RFC‑0001 rewritten to align with intention document (vision, architecture, RBA)
- RFC‑0002: status corrected to Accepted, duplicates removed, numbering fixed, NOT added
- RFC‑0003: "C code generation" → "code generation"
- RFC‑0004: backend reference corrected
- RFC‑1000: "C backend" → "backend (Rust/rustc)"
- RFC index: updated descriptions and statuses
- `src/lib.rs`: exposed `codegen` module
### Fixed
- Restored semantic module scaffolding required for successful compilation. Parser test failures for function declarations, control‑flow blocks, and syntax‑error handling.
- Missing `E1010` case in `PartialEq<&'static str>` for `SemanticErrorCode` — caused `unknown_type_error` test failure
### Removed
- `src/codegen/c.rs` (replaced by `rust.rs`)
## [0.1.0] - 2026-06-17

### Added
-
- Initial repository structure.
- AGENTS.md.
- README.md.
- RFC-0001 Vision and Goals.
- Rust compiler bootstrap project.
