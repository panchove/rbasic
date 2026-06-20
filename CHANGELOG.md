# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog.

## [Unreleased]

### Added
- RFC-0011: Classic BASIC type aliases — `BOOLEAN`, `BYTE`, `WORD`, `INTEGER`, `LONG`, `LONGLONG`, `SINGLE`, `DOUBLE` resolve to their canonical types (case-insensitive, zero AST/parser changes)
- 12 new semantic type tests covering all aliases, case-insensitivity, function params, casts, and unknown alias rejection
- String escape sequences processed in the lexer: `\\`, `\"`, `\n`, `\r`, `\t` (RFC-0002 §11)
- Structured lexer diagnostics with file, line, column, source snippet, and caret (RFC-0002 §17)
- `diagnostics::format_lex_error` and `diagnostics::offset_to_line_col` exported from crate root
- `Display` impl for `SemanticErrorCode` (clean `{}` formatting)
- Parse and semantic errors in the CLI now report `line:column` instead of byte offsets
- Explicit AS cast syntax: `expr AS TypeName` for numeric type conversions
- CLI command `run`: build and execute immediately (invokes rustc internally)
- FOR loop: `FOR var = start TO end ... END FOR` with codegen, semantic checks, and tests
- FOR...STEP loop: `FOR var = start TO end STEP expr ... END FOR` with direction-aware codegen
- DO WHILE/UNTIL (pre-test) and DO...LOOP WHILE/UNTIL (post-test) loop variants
- Power operator `^`, modulo `MOD`, integer division `\`, bitwise `SHL`/`SHR`, logical `AND`/`OR`/`XOR`
- Unsigned integer types: U8, U16, U32, U64 with full operator support
- F32 type with float promotion (F32↔F64)
- I8, I16 types with integer widening (I8→I16→I32→I64)
- RFC-0009: FOR...STEP Specification (Accepted)
- RFC-0010: DO Loop Specification (Accepted)
- RFC-0011: Type Aliases Specification (Accepted)
- Examples: `hello.rbas`, `add.rbas`, `fibonacci.rbas`, `fizzbuzz.rbas`, `for_step.rbas`, `operators.rbas`
- Integration tests: compile and execute all examples end-to-end

### Changed
- RFC-0002: `BangEqual` → `NotEqual` in token inventory (aligns with `TokenKind::NotEqual`)
- RFC-0007: documented implicit integer widening rules (I8→I64, U8→U64, F32↔F64)
- `rbasic run`: cleans up temporary `.rs` and binary files after execution

### Fixed
- Restored semantic module scaffolding required for successful compilation

### Removed
- `src/codegen/c.rs` (replaced by `rust.rs`)

## [0.1.0] - 2026-06-17

### Added
- Initial repository structure
- AGENTS.md, README.md
- RFC-0001 Vision and Goals
- Rust compiler bootstrap project
