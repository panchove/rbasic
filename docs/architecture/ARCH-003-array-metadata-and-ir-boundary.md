# ARCH-003: Array Metadata Ownership and Future IR Boundary

## Current Pipeline
```
AST
 ↓
Semantic Analysis
 ↓
ArrayInfo
 ↓
Rust Codegen
```
*ArrayInfo* is produced by the semantic analyzer and consumed directly by the Rust backend.

### Concern
While acceptable with a single backend, this tight coupling may become limiting when additional backends, optimization passes, or the RBA runtime are introduced. Transporting backend‑specific metadata directly from the semantic phase can hinder flexibility.

### IR Not Required for v0.2 / v0.3
- No intermediate representation is needed for the current Rust code generator.
- The architecture deliberately avoids an IR until after v0.3 features stabilize.

### Trigger Conditions for Introducing an IR
- Introduction of a second backend (e.g., C, generic IR).
- Addition of optimization passes that need a richer program model.
- Module system requiring richer semantic transport across compilation units.
- RBA runtime backend integration.
- Self‑hosting preparation (compiler written in RBASIC).

### Recommendation
Future work should define an intermediate representation.

**Future RFC**: `RFC-0025 Intermediate Representation Architecture`

**Metadata**
- Status: Observation
- Implementation: Deferred
- Target: v0.6+

## Governance Note

This observation is recorded for long‑term architectural planning only.

It is **NOT** an approved implementation task.

It does **NOT** supersede the v0.3 roadmap.

Implementation requires:

* Dedicated RFC
* Architectural review
* RFC acceptance
* Migration strategy
* Successful `make verify`

Current status:

```
Status: Observation
Implementation: Deferred
Target: v0.6+
```
