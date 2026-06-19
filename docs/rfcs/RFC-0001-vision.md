# RFC-0001: Vision and Goals

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-06-17
Last Updated: 2026-06-19

---

## 1. Summary

RBASIC is a modern programming language inspired by the readability of BASIC, the safety of Rust, the operational simplicity of Go, the hardware proximity of C, and the productivity of modern languages. Its compiler is written in Rust and generates Rust code, using rustc (LLVM) as the backend.

This RFC defines the long-term vision and strategic goals of the RBASIC project as established in the project's intention document (DOCUMENTO_DE_INTENCION.md).

---

## 2. Strategic Objectives

### Primary Objective

Develop a complete ecosystem composed of:

- RBASIC language
- RBASIC compiler
- Development tooling
- Package system
- Standard library
- Official documentation

### Technical Objectives

- Native compilation via Rust backend (rustc/LLVM)
- Static typing
- Memory safety
- Explicit error handling
- Cross-platform portability
- C interoperability
- WebAssembly support (future)
- Self-hosting capability (future)

---

## 3. Core Principles

### 3.1 Safety by Default

Every RBASIC program must be safe by default. The programmer must explicitly opt into unsafe operations through controlled mechanisms.

### 3.2 Readability First

Code must be understandable before being clever. Language constructs favor clarity over complexity.

### 3.3 Evolutionary Simplicity

The language core must remain small and stable. Advanced features are built on a minimal, consistent foundation.

### 3.4 Zero Cost When Possible

Language abstractions must not introduce unnecessary runtime penalties.

---

## 4. Compiler Architecture

```text
Lexer
 ↓
Parser
 ↓
AST
 ↓
Semantic Analysis
 ↓
Typed AST
 ↓
Code Generation (Rust)
 ↓
rustc (LLVM)
```

### Backend Strategy

RBASIC → Rust → rustc (LLVM)

This strategy provides:
- LLVM optimization infrastructure via rustc
- Single language for the compiler (Rust)
- Future bootstrapping path via codegen rewrite in RBASIC
- Portability to all Rust-supported platforms

---

## 5. Long-term Goals

RBASIC aims to be a modern, safe, and sustainable language suitable for:

- CLI applications
- Automation tooling
- Embedded systems
- Backend services
- Cross-platform applications
- Compilers and developer tools
- Office suite automation via **RBA (RBasic for Applications)**, as a modern VBA replacement in LibreOffice, FreeOffice and OnlyOffice

---

## 6. Evolution Strategy

| Stage | Description |
|-------|-------------|
| 1 | Minimal functional RBASIC |
| 2 | Basic standard library |
| 3 | Production-ready compiler |
| 4 | Progressive compiler rewrite in RBASIC |
| 5 | Self-hosting |
| 6 | Native LLVM/Cranelift backend |

---

## 7. RBA — RBasic for Applications

RBA is an embedded variant of RBASIC designed to act as a scripting engine in open-source office suites:

- **LibreOffice** — Integration via UNO API
- **FreeOffice** — Integration via native API
- **OnlyOffice** — Integration via plugin/scripting mechanisms

RBA shares the same linguistic core as RBASIC but includes a standard library oriented toward document manipulation, spreadsheets, presentations, and office task automation. The goal is to offer a modern, safe, cross-platform alternative to VBA (Visual Basic for Applications), eliminating dependency on Windows and Microsoft Office.

---

## 8. Acceptance Criteria

```text
✓ Vision and goals documented
✓ Architecture defined
✓ Backend strategy defined
✓ Evolution stages defined
✓ RBA vision defined
✓ Aligned with DOCUMENTO_DE_INTENCION.md
```
