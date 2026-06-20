# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog.

## [Unreleased]

### Added

- RFC-0018: Compound Assignment Operators (Accepted + implemented)
- Compound assignment operators: `+=`, `-=`, `*=`, `/=`, `\=`, `MOD=`
- Lexer: longest-match tokens `PlusEqual`, `MinusEqual`, `StarEqual`, `SlashEqual`, `BackslashEqual`, `ModEqual`
- AST: `Statement::AssignOp { name, op: CompoundAssignOp, expr }` variant
- Semantic: E1043 (undeclared target), E1044 (immutable target), E1045 (type mismatch); reuses binary op rules
- Codegen: desugared to `x = x OP expr`
- 28 new tests: 8 lexer, 2 parser, 11 semantic, 7 codegen (405 total)
- Diagnostic registry: `docs/diagnostics/README.md`
- Architecture documents: `docs/architecture/array-indexing-gap-analysis.md`, `docs/architecture/roadmap-v0.2-to-v1.0.md`

### Compliance Fixes (RFC Audit)

- **P0**: Lexer now restricts `$` to trailing-only suffix — `na$me`, `foo$$`, `abc$def` produce lex errors (RFC-0002 compliance)
- **P0**: Parser now accepts `DIM arr(n) AS <TYPE>` syntax; default remains INTEGER when omitted (RFC-0012/RFC-0016 compliance)
- **P1**: Duplicate DIM array variable now emits E1002 instead of E1003, matching duplicate variable convention (RFC-0006 compliance)
- **P2**: Added E1034 to RFC-0008 §8 error code table; updated scope to include FOR…STEP value type validation (documentation gap closed)
- 14 new tests: 4 lexer, 3 parser, 4 semantic type, 3 codegen (377 total, up from 363)

### Changed

- RFC-0002: Updated identifier rules to allow optional trailing `$` suffix; added `identifier ::= [A-Za-z_][A-Za-z0-9_]*[$]?` grammar
- RFC-0004: Added `assign_stmt` grammar rule; updated `statement` production to include standalone assignment; renumbered sections
- RFC-0005: Added `Statement::Assign` variant to AST specification; updated variant count from 12 to 13
- RFC-0006: Added E1040–E1042 to error code table; updated implementation status to v0.2
- RFC-0007: Added integer‑to‑float widening rules (any integer → F32/F64); updated cross‑family compatibility matrix
- docs/rfcs/README.md: Added RFC-0018 (Draft)
- Type aliases `DWORD` (→U32) and `QWORD` (→U64), matching Windows API convention
- RFC-0014: Hybrid Stack + ARC Memory Management (Accepted)
- RFC-0015: Standalone Assignment (Accepted + implemented)
- RFC-0016: DIM Array Code Generation (Accepted + implemented)
- RFC-0017: String Functions (Accepted + implemented)
- Built-in string functions: LEN, MID$/MID, LEFT$/LEFT, RIGHT$/RIGHT, CHR$/CHR, ASC, INSTR, VAL, STR$/STR, UCASE$/UCASE, LCASE$/LCASE, TRIM$/TRIM, LTRIM$/LTRIM, RTRIM$/RTRIM, SPACE$/SPACE, STRING$/STRING
- String literals emit `.to_string()` in codegen (heap allocation)
- Codegen tests for DWORD/QWORD type aliases
- Standalone assignment (`x = expr`) — parses, validates, codegen
- DIM array codegen — emits `Vec<T>` with `vec![]` initialization
- Mutability tracking for variables (E1042: assignment to immutable)
- New error codes: E1040 (undeclared), E1041 (type mismatch), E1042 (immutable)
- `$` allowed in identifiers (BASIC string function naming convention)
- Integer-to-float implicit widening (I32→F64, etc.) in type compatibility
- Persistent top-level locals across statements in semantic analyzer
- `default_for_type()` and `gen_dim_init()` helpers for array codegen
- `check_function_call()`, `check_builtin_call()`, `builtin_sig()` helpers for built-in function validation
- `is_builtin()`, `gen_builtin_call()` helpers for built-in function codegen
- 20 semantic tests + 20 codegen tests for string functions

### Changed

- Updated RFC-0002 token inventory with all v0.1 tokens (Step, Do, Loop, Until, As, And, Or, Xor, Dim, On, Error, Goto, Resume, Shl, Shr, Caret, Backslash, Mod, BoolLiteral)
- Updated RFC-0004 grammar with FOR/STEP, DO/LOOP variants, DIM, ON ERROR, RESUME, extended operators (^, \, MOD, SHL, SHR), AS cast, NOT unary, logical AND/OR/XOR
- Updated RFC-0005 AST with For, DoLoop, Dim, OnError, Resume, Cast expression, Not unary, extended BinaryOp
- Updated RFC-0006 semantic analysis: marked all phases 1–3 as implemented
- Updated RFC-0007 type compatibility with complete v0.1 type set (I8–I64, U8–U64, F32) and extended operator rules
- Updated RFC-0006 error table: E1034 is now non-numeric step (was Reserved)
- Fixed RFC-0008 creation date (2026-07-01 → 2026-06-19)

### Added

- RFC-0012: DIM Array Declarations (Accepted)
- RFC-0013: ON ERROR / RESUME Statements (Accepted)
- RFC-0011 added to docs/rfcs/README.md index
- Updated AGENTS.md: v0.1 marked as complete (229 tests, 13 RFCs)
- Updated CLAUDE.md with v0.1 completion status

### Added

- 74 new tests covering previously untested v0.1 features (303 total, up from 229):
  - Lexer: bool literals, all extended keywords (STEP, DO, LOOP, UNTIL, AS, AND, OR, XOR, DIM, ON, ERROR, GOTO, RESUME, SHL, SHR, MOD), operators (^, \\), UTF-8 encoding, reserved word handling, END sequences, identifier with underscore, case-insensitive extended keywords
  - Parser: DIM (single, multiple, multi-dim), ON ERROR GOTO, RESUME (bare/label), DO UNTIL/LOOP UNTIL, AND/OR/XOR binary, SHL/SHR, NOT unary, TRUE/FALSE literals, POW/MOD/IntDiv operators, AS cast, multiple functions, operator precedence
  - Codegen: AND/OR Rust output, DIM/ON ERROR/RESUME no-crash, multiple functions, DO UNTIL/LOOP UNTIL codegen, FALSE literal, NOT expression, STRING type decl, AS cast with alias
  - Integration: DO loops (terminating via const conditions), AS casts (F64→I32, I32→F32), unsigned type addition, power operator, multiple functions
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
