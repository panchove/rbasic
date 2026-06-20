# ARCH-002: Semantic Analyzer Output Is Acting As Metadata Transport

## Current State
```rust
pub type ArrayInfo = HashMap<String, (Type, usize)>;

pub fn analyze(prog: &Program) -> Result<ArrayInfo, Vec<SemanticError>>
```
*Source:* `src/semantic/analyzer.rs`

The semantic analyzer performs full semantic validation but only returns `ArrayInfo`.

### Semantic Responsibilities
- symbol resolution
- scope validation
- type resolution
- type compatibility
- type checking
- assignment validation
- array validation

### Concern
Returning only `ArrayInfo` is functional for v0.2 but is too narrow as a long‑term semantic contract. Future language extensions (modules, function references, INPUT, SUB, SELECT CASE) will increase the mismatch between work performed and data exported.

### Recommendation
Do not change this in v0.2.

**Future RFC**: `RFC-0024 Semantic Information Model`

**Suggested future API**
```rust
pub struct SemanticInfo {
    pub arrays: ArrayInfo,
    // future:
    // pub globals: GlobalTable,
    // pub functions: FunctionTable,
    // pub inferred_types: TypeTable,
}

pub fn analyze(prog: &Program) -> Result<SemanticInfo, Vec<SemanticError>>
```

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
