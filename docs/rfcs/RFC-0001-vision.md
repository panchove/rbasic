# RFC-0001: Vision and Goals

Status: Accepted
Version: 0.2
Author: RBASIC Project
Created: 2026-06-17
Last Updated: 2026-06-20

---

## 1. Summary

RBASIC is a modern programming language inspired by the readability of BASIC, the safety of Rust, the operational simplicity of Go, the hardware proximity of C, and the productivity of modern languages. Its compiler is written in Rust and generates Rust code, using rustc (LLVM) as the backend.

**QuickBASIC is the source of truth** for the language features that RBASIC aims to implement. All syntax, semantics, and control flow constructs are defined by QuickBASIC as the canonical reference. RBASIC extends this foundation with modern safety features, but the core language behavior must remain faithful to QuickBASIC.

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
- Memory safety (stack + ARC + Weak, no garbage collector)
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

### Compilation Modes

- **32‑bit** (`--target i686`) — generates 32‑bit executables
- **64‑bit** (`--target x86_64`) — default. Generates 64‑bit executables

---

## 5. Implementation Phases

RBASIC features are organized in three phases (see RFC-0003):

| Phase | Description |
|-------|-------------|
| Phase 1 | QuickBASIC-compatible core |
| Phase 2 | Modern extensions (safety, expressiveness) |
| Phase 3 | Future features (generics) |

---

## 6. Long-term Goals

RBASIC aims to be a modern, safe, and sustainable language suitable for:

- CLI applications
- Automation tooling
- Embedded systems
- Backend services
- Cross-platform applications
- Compilers and developer tools
- Office suite automation via **RBA (RBasic for Applications)**, as a modern VBA replacement in LibreOffice, FreeOffice and OnlyOffice
- Automation scripting via **RBScript**, as a modern VBScript replacement

---

## 7. Evolution Strategy

| Stage | Phase | Description |
|-------|-------|-------------|
| 1 | Phase 1 | QuickBASIC-compatible core complete |
| 2 | Phase 2 | Modern extensions begin + standard library |
| 3 | Phase 2 | Phase 2 complete, compiler processes real projects |
| 4 | Phase 1+2 | Progressive compiler rewrite in RBASIC |
| 5 | Phase 1+2 | Self-hosting (autocompilation) |
| 6 | — | Native LLVM/Cranelift backend |
| 7 | Phase 3 | Generics, Optional, Result |

---

## 8. RBA — RBasic for Applications

RBA is an embedded variant of RBASIC designed to act as a scripting engine in open-source office suites:

- **LibreOffice** — Integration via UNO API
- **FreeOffice** — Integration via native API
- **OnlyOffice** — Integration via plugin/scripting mechanisms

RBA shares the same linguistic core as RBASIC but includes a standard library oriented toward document manipulation, spreadsheets, presentations, and office task automation. The goal is to offer a modern, safe, cross-platform alternative to VBA (Visual Basic for Applications), eliminating dependency on Windows and Microsoft Office.

---

## 9. RBScript

RBScript is a scripting language inspired by VBScript, designed to provide automation and scripting capabilities within the RBASIC ecosystem. It shares the same core language as RBASIC and RBA, targeting environments where a lightweight, embeddable scripting engine is needed.

---

## 10. Acceptance Criteria

```text
✓ Vision and goals documented
✓ QuickBASIC is source of truth for core features
✓ Architecture defined
✓ Backend strategy defined
✓ Compilation modes defined
✓ Three-phase implementation model defined
✓ Evolution stages defined (7 stages)
✓ RBA vision defined
✓ RBScript defined
✓ Aligned with DOCUMENTO_DE_INTENCION.md
```
