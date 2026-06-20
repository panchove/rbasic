# Array Indexing Gap Analysis

## Current State

RBASIC v0.2 supports array **declaration** via `DIM` but does **not** support array **access** (reading or writing elements). Arrays are currently dead code — they can be declared and codegen emits `Vec<T>` but no syntax exists to interact with individual elements.

### What Works

```basic
DIM arr(10) AS INTEGER          ' ✓ Parser: Dim statement
DIM matrix(5, 5)                ' ✓ Parser: multi-dimensional
DIM a(100), b(200)              ' ✓ Parser: multi-declaration
```

```rust
// Codegen output for the above:
let arr: Vec<i32> = vec![0i32; 11];
let matrix: Vec<Vec<i32>> = vec![vec![0i32; 6]; 6];
let a: Vec<i32> = vec![0i32; 101];
let b: Vec<i32> = vec![0i32; 201];
```

### What Does NOT Work

```basic
DIM arr(10) AS INTEGER
arr(0) = 42        ' ✗ No syntax for indexed assignment
PRINT arr(0)       ' ✗ No syntax for indexed read
LET x = arr(5)     ' ✗ No syntax for indexed expression
```

There is no `IndexExpr` node in the AST, no parser rule for index access, and no semantic or codegen support.

## Missing Grammar

The primary expression rule currently has no array‑index production:

```ebnf
' Current (no array indexing):
primary ::= INTEGER_LITERAL | FLOAT_LITERAL | STRING_LITERAL | BOOL_LITERAL
          | IDENTIFIER | function_call | "(" expression ")"

' Required:
primary ::= INTEGER_LITERAL | FLOAT_LITERAL | STRING_LITERAL | BOOL_LITERAL
          | IDENTIFIER | function_call | index_access | "(" expression ")"
index_access ::= IDENTIFIER "(" expression ")"
```

### Ambiguity with Function Calls

`arr(0)` is syntactically identical to a function call `f(0)`. The disambiguation must happen during semantic analysis:

- If `arr` refers to an array variable → index access
- If `arr` refers to a function → function call

This is the same approach used in classic BASIC and requires the semantic analyzer to distinguish array names from function names during expression resolution.

## Missing AST Nodes

The following new AST node is required:

```text
Expression ::= ...
             | Index {
                   target: Box<Expression>,
                   index: Box<Expression>,
               }
```

For multi‑dimensional arrays, chained index expressions would be used:

```basic
matrix(2, 3)  →  Index { target: Index { target: Identifier("matrix"), index: Int(2) }, index: Int(3) }
```

Alternatively, a variadic `indices: Vec<Expression>` field could be used:

```text
Expression ::= ...
             | Index {
                   target: Box<Expression>,
                   indices: Vec<Expression>,
               }
```

The variadic approach is recommended for better error reporting and simpler codegen.

## Missing Semantic Rules

1. **Name resolution**: When resolving `arr(i)` where `arr` is an array, the semantic analyzer must return the array's element type (e.g., `I32` for `DIM arr(10) AS INTEGER`), not the array type itself (`Vec<i32>`).
2. **Index type validation**: The index expression `i` must be an integer type. Float, bool, or string indices produce E1061.
3. **Dimension counting**: The number of indices must match the declared dimensionality. `DIM matrix(5, 5)` requires exactly 2 indices. Mismatch produces E1062.
4. **Bounds checking**: Static bounds checking is optional for v0.3 (deferred to runtime). The semantic analyzer should only validate type and dimension count.
5. **Mutable access**: Indexed assignment (`arr(i) = expr`) requires the array variable to be mutable.

## Missing Code Generation

For array read access:

```basic
PRINT arr(0)
```
```rust
println!("{}", arr[0usize]);
```

For array write access:

```basic
arr(0) = 42
```
```rust
arr[0usize] = 42;
```

The index expression must be converted from 1‑based (BASIC convention) to 0‑based (Rust convention). This applies to both read and write:

```basic
arr(1) = 10   ' BASIC: first element
```
```rust
arr[0usize] = 10;   ' Rust: first element
```

The index conversion is `index - 1`. This must be emitted as a runtime subtraction.

## Indexed Assignment Grammar

The current assignment grammar only supports simple identifiers as targets:

```ebnf
assign_stmt ::= IDENTIFIER "=" expression
```

This must be extended to support array element targets:

```ebnf
assign_target ::= IDENTIFIER | index_access
assign_stmt   ::= assign_target "=" expression
```

## Proposed Diagnostics

| Code  | Description                         | When Triggered                           |
|-------|-------------------------------------|------------------------------------------|
| E1060 | Index out of bounds                 | Static analysis detects out‑of‑range (optional) |
| E1061 | Invalid index type                  | Index expression is not integer          |
| E1062 | Dimension mismatch                  | Number of indices does not match DIM     |

These are reserved in the diagnostic registry (`docs/diagnostics/README.md`) and require an RFC for formal adoption.

## Dependency Chain

```
┌──────────────────────────────────────────────────┐
│ RFC-0019 (Array Indexing, new)                    │
│   ├── Grammar: index_access in primary            │
│   ├── AST: Expression::Index or IndexExpr         │
│   ├── Semantic: array vs function disambiguation   │
│   ├── Codegen: arr[i-1] emission                  │
│   └── Diagnostics: E1060, E1061, E1062            │
└──────────────────────────────────────────────────┘
                       ↑ depends on
┌──────────────────────────────────────────────────┐
│ RFC-0018 (Compound Assignment)                    │
│   (assign_target extension is shared)             │
└──────────────────────────────────────────────────┘
```

## Recommendation

Array indexing should be the subject of a dedicated RFC (RFC-0019) targeting v0.3. It requires:

- 1 new grammar rule (`index_access` in `primary`)
- 1 new AST node (`Expression::Index`)
- Parser changes (disambiguation from function calls deferred to semantic analysis)
- Semantic analysis (array type resolution, index validation, dimension counting)
- Codegen (1‑based to 0‑based conversion, `Vec` indexing emit)
- 3 new diagnostics (E1060–E1062)
- Updates to assignment grammar to support indexed targets
