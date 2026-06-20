# RBASIC Roadmap: v0.2 to v1.0

**Status:** Current
**Last Updated:** 2026-06-19

---

## Overview

This document defines the projected development roadmap from the current v0.2 (Semantic Foundation) toward v1.0 (Production Compiler). Milestones and versions are approximate and subject to change based on community feedback and implementation experience.

---

## v0.2 — Semantic Foundation (Current)

**Status:** Complete

**Features:**
- Standalone assignment (RFC-0015)
- DIM array codegen (RFC-0016)
- Built-in string functions (RFC-0017)
- Integer‑to‑float widening
- String `$` suffix convention
- Persistent locals
- 17 RFCs accepted
- 363 tests passing

**Artifacts:**
- CLI: `rbasic build`, `rbasic run`
- Rust codegen pipeline operational
- All semantic phases (1–3) implemented

---

## v0.3 — Type Checking Completion

**Target:** Next milestone

### Planned Features

1. **Compound Assignment Operators** (RFC-0018)
   - `+=`, `-=`, `*=`, `/=`, `\=`, `MOD=`
   - Lexer tokens, parser, AST, semantic, codegen
   - Diagnostics E1043–E1045

2. **Array Indexing** (RFC-0019)
   - `arr(i)` read access in expressions
   - `arr(i) = expr` write access
   - 1‑based to 0‑based conversion
   - Diagnostics E1060–E1062

3. **Function References** (RFC-0020)
   - `ADDRESSOF` operator or `FUNC()` syntax
   - Function pointer type system
   - Dynamic dispatch for callbacks

4. **RFC Remediation**
   - Update RFC-0002/0004/0005/0006/0007 to match implementation
   - Create diagnostic registry (`docs/diagnostics/README.md`)

### Dependencies
- RFC-0014 (Memory Management) must be resolved before function references
- Array indexing depends on compound assignment grammar structure

### Verification
- `make verify` must pass
- No RFC‑to‑code drift

---

## v0.4 — Code Generation Expansion

### Planned Features

1. **Optimized Codegen**
   - Dead code elimination for unreachable statements
   - Constant folding for compile‑time expressions
   - Loop invariant hoisting (simple cases)

2. **String Optimization**
   - `Rc<String>` adoption (RFC-0014 migration)
   - COW semantics for string assignment

3. **Array Runtime**
   - Bounds checking (optional, debug mode)
   - Dynamic resizing (`REDIM PRESERVE`)

4. **Error Runtime**
   - Runtime error codes (E1070–E1079)
   - `ON ERROR GOTO` runtime support

### Verification
- Integration tests for generated code correctness
- Benchmark suite for codegen quality

---

## v0.5 — Runtime Layer

### Planned Features

1. **RBA Runtime Stub**
   - `runtime/` module bootstrap
   - C FFI bridge for runtime helpers
   - String allocation abstraction

2. **Runtime Library**
   - `rba::string` with ARC semantics
   - `rba::array` with COW semantics
   - Error handling runtime (`rba::error`)

3. **Runtime Initialization**
   - `__rba_init()` preamble in generated code
   - Runtime version checks

### Verification
- `cargo test` with runtime linked
- Runtime unit tests

---

## v0.6 — Module System

### Planned Features

1. **Module Declarations**
   - `MODULE` keyword and grammar
   - Module‑level scope isolation
   - `IMPORT`/`EXPORT` for cross‑module visibility

2. **Multi‑File Compilation**
   - File‑as‑module mapping
   - Module dependency graph resolution
   - Separate compilation units

3. **Namespace Management**
   - Fully qualified names (`Module::name`)
   - Name collision resolution
   - Diagnostics for module errors

### Dependencies
- Reserved words already present (`MODULE`, `IMPORT`, `EXPORT`)
- RFC required before implementation

---

## v0.7 — RBA Runtime Foundation

### Planned Features

1. **Office Integration API**
   - Application stubs: `MsgBox`, `InputBox`
   - Excel stubs: `Range`, `Cells`, `Worksheet`
   - Word stubs: `Document`, `Selection`

2. **COM Interop Layer**
   - Interface definition in Rust
   - SAFEARRAY and BSTR marshaling
   - Error HRESULT mapping

3. **Runtime Host**
   - Standalone executable mode
   - Office add‑in mode
   - Configuration file format

### Dependencies
- RFC-1000 (RBA Vision) must be finalized

---

## v0.8 — Office Integration

### Planned Features

1. **Excel Automation**
   - Range read/write
   - Formula assignment
   - Worksheet management
   - Workbook operations

2. **Word Automation**
   - Document creation/modification
   - Selection and range operations
   - Template processing

3. **Outlook Automation** (if scoped)
   - Mail item creation
   - Calendar operations

### Verification
- Integration tests against Office application
- Example macros and templates

---

## v0.9 — Self‑Hosting Preparation

### Planned Features

1. **Bootstrap Compiler**
   - RBASIC‑to‑RBASIC compilation
   - Lexer, parser, semantic analysis in RBASIC
   - Codegen delegating to Rust backend

2. **Standard Library**
   - `rbas/std` in pure RBASIC
   - File I/O, string processing, collections
   - Date/time, math, random

3. **Package Manager**
   - Module registry format
   - Dependency resolution
   - Build tool integration

### Dependencies
- v0.6 module system
- v0.5 runtime layer
- Sufficient language expressiveness

---

## v1.0 — Production Compiler

### Requirements

- Self‑hosting compiler
- Office integration operational
- Runtime library stable
- Test suite > 2000 tests
- Documentation complete (RFCs, API docs, tutorial)
- Community review completed
- Backward compatibility policy documented

### Non‑Goals (for v1.0)

- Generics (reserved keyword exists)
- `Ref<T>` / `MutRef<T>`
- `Optional<T>` / `Result<T, E>`
- Ownership and borrow checking
- User‑defined types (`TYPE`/`STRUCT`/`ENUM`)
- Async/parallel execution
- GUI framework

---

## Risk Register

| Risk                                      | Impact | Likelihood | Mitigation                                    |
|-------------------------------------------|--------|------------|-----------------------------------------------|
| RFC-0014 unimplemented blocks v0.3        | High   | Medium     | Downgrade to Draft; implement before v0.3 FR  |
| Office API drift (COM changes)            | Medium | Low        | Abstraction layer isolates COM bindings       |
| Self‑hosting compiler complexity          | High   | Medium     | Phased approach: lexer first, then parser     |
| Community/contributor bandwidth           | Medium | Medium     | Documentation and onboarding improvements     |
| Test suite fragility                      | Low    | Low        | CI enforcement of `make verify`               |

---

## Milestone Tracking

| Version | Status       | RFCs | Tests   |
|---------|--------------|------|---------|
| v0.1    | Complete     | 1–14 | 229     |
| v0.2    | Complete     | 15–17| 363     |
| v0.3    | Not started  | 18–20| —       |
| v0.4    | Not started  | —    | —       |
| v0.5    | Not started  | —    | —       |
| v0.6    | Not started  | —    | —       |
| v0.7    | Not started  | —    | —       |
| v0.8    | Not started  | —    | —       |
| v0.9    | Not started  | —    | —       |
| v1.0    | Not started  | —    | —       |
