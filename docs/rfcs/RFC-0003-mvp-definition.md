# MVP Definition

This document defines the Minimum Viable Product (MVP) for the RBASIC language and its compiler. Features listed as **✓** are included in the MVP; features marked **✗** are excluded and require a separate RFC before they can be considered.

## Language Features

- **✓** `LET`
- **✓** `LET MUT`
- **✓** `FUNCTION`
- **✓** `RETURN`
- **✓** `PRINT`
- **✓** `IF`
- **✓** `ELSE`
- **✓** `WHILE`

## Types

- **✓** `bool`
- **✓** `i32`
- **✓** `f64`
- **✓** `string`

## Excluded Features (require RFC)

- **✗** arrays
- **✗** modules
- **✗** `Optional`
- **✗** `Result`
- **✗** `Ref`
- **✗** `MutRef`
- **✗** generics
- **✗** ownership semantics

## Rationale

The MVP focuses on a small, well‑defined core that allows writing simple procedural programs, exercising the lexer, parser, semantic analysis, and code generation pipelines without the complexity of advanced type system features. Once the MVP is stable, the excluded features can be incrementally added following the RFC → approval → implementation → CHANGELOG process.
