# Agents

Proyecto: RBASIC

This repository is managed with OpenCode. Follow the local conventions below.

## CHANGELOG Policy

- Every significant change must be recorded in **CHANGELOG.md**.
- Follow the Keep a Changelog format with an *Unreleased* section.
- Update the file before closing any task.
- Do not delete historical entries or rewrite published versions.
- Agents must review the CHANGELOG before starting work and update it after completing modifications.
- The CHANGELOG complements RFCs: RFC → implementation → CHANGELOG entry.

## Language Version

Current Language Target: RBASIC v0.2

Status:
- **Implemented** — v0.2 features: Standalone Assignment (RFC-0015), DIM Array Codegen (RFC-0016), String Functions (RFC-0017)
- 363 tests passing (`make verify` succeeds)
- 17 RFCs (1–17) accepted and implemented

## RFC Requirement for Language Changes

### RFC Status Model

Official states for any RFC:

- **Draft** – initial proposal, not yet reviewed.
- **Review** – under community/agent review.
- **Accepted** – approved and ready for implementation.
- **Implemented** – code changes merged.
- **Deprecated** – feature should no longer be used.
- **Superseded** – replaced by a newer RFC.

Each RFC must contain a standard header, e.g.:

```markdown
# RFC-0002 Lexical Specification

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-06-18
Last Updated: 2026-06-18
```

Only **Accepted** RFCs may be used as a stable contract for implementation. Subsequent changes are only allowed via a new RFC that supersedes the old one.

No syntax, semantics, or type system change may be implemented without a prior approved RFC.

Flow:

RFC
 ↓
Approval
 ↓
Implementation
 ↓
CHANGELOG

This applies especially to debates about:
- Ref<T>
- MutRef<T>
- Optional<T>
- Result<T,E>
- Modules
- Generics
- Ownership

## Commands

- `make check` – cargo check
- `make test` – cargo test
- `make build` – cargo build
- `make run` – cargo run
- `make fmt` – cargo fmt
- `make lint` – cargo clippy with warnings as errors
- `make verify` – fmt‑check, lint, test
- Granular tests: `make test-lexer`, `make test-parser`, `make test-semantic`, `make test-types`, `make test-all`.
