# RFC-0016: DIM Array Code Generation

Status: Accepted
Version: 0.2
Author: RBASIC Project
Created: 2026-06-19
Last Updated: 2026-06-19

---

## 1. Summary

Implement code generation for `DIM` array declarations. In v0.1, `DIM` is parsed and semantically analyzed but the codegen emits nothing. This RFC specifies how `DIM` arrays are translated to Rust, including initialization, default values, and memory layout.

---

## 2. Background

RFC-0012 defined the syntax, AST, and semantic analysis for `DIM`. RFC-0014 established the hybrid stack + ARC memory model, with arrays using heap-allocated storage.

In v0.2, arrays are emitted as `Vec<T>` (heap-allocated, RAII-managed). The `Rc<Vec<T>>` wrapper is deferred until shared ownership semantics are needed.

---

## 3. Memory Layout

`DIM arr(n)` creates an array with `n + 1` elements, indexed from 0 to n (inclusive). This matches both Rust's 0-based indexing and classic BASIC's convention where `DIM arr(10)` allocates 11 elements with indices 0–10.

```
DIM arr(10)    →  Vec with 11 elements, indices 0..=10
DIM matrix(5, 5)  →  Vec<Vec<T>> with 6×6 = 36 elements (future)
```

### Multi-dimensional arrays

Multi-dimensional arrays are represented as nested `Vec`s:

```basic
DIM matrix(5, 5)
```

```rust
let matrix: Vec<Vec<i32>> = vec![vec![0i32; 6]; 6];
```

---

## 4. Default Values

Each array element is initialized to the default value for its base type:

| Base type | Default  | Rust expression           |
|-----------|----------|---------------------------|
| BOOL      | FALSE    | `false`                   |
| I8        | 0        | `0i8`                     |
| I16       | 0        | `0i16`                    |
| I32       | 0        | `0i32`                    |
| I64       | 0        | `0i64`                    |
| U8        | 0        | `0u8`                     |
| U16       | 0        | `0u16`                    |
| U32       | 0        | `0u32`                    |
| U64       | 0        | `0u64`                    |
| F32       | 0.0      | `0.0f32`                  |
| F64       | 0.0      | `0.0f64`                  |
| STRING    | ""       | `String::new()`           |

If no type annotation is provided, the default base type is `INTEGER` (I32), matching classic BASIC convention.

---

## 5. Code Generation

### Single dimension

```basic
DIM arr(10)
```

```rust
let arr: Vec<i32> = vec![0i32; 11];
```

```basic
DIM arr(10) AS STRING
```

```rust
let arr: Vec<String> = vec![String::new(); 11];
```

### Multi-dimensional

```basic
DIM matrix(5, 5)
```

```rust
let matrix: Vec<Vec<i32>> = vec![vec![0i32; 6]; 6];
```

### Static lifetime

All `DIM` variables are `let` (immutable binding) in v0.2. The underlying `Vec` is mutable (`let mut`) only if `LET MUT` is... actually, `DIM` does not use `LET`. In v0.2, all array bindings are `let` (immutable reference) but the Vec itself is heap-allocated and can be mutated via future array access syntax.

---

## 6. Type Information in AST

The current `ArrayDecl` stores the base type via `array_type.base_type`. The semantic analyzer already resolves this. The codegen reads the resolved type to emit the correct default value and type annotation.

---

## 7. Interaction with Assignment (RFC-0015)

Once RFC-0015 (standalone assignment) is implemented, array variables can be reassigned:

```basic
DIM arr(10)
arr = another_array    ' only if another_array has compatible type
```

Full array assignment is straightforward in Rust:

```rust
arr = another_array;
```

---

## 8. Deferred Features

The following are explicitly **out of scope** for v0.2:

| Feature | Reason |
|---------|--------|
| Array element access (`arr(i)`) | Requires new syntax, grammar, and AST node |
| Array slicing or iteration | Requires language-level iteration protocol |
| `Rc<Vec<T>>` shared ownership | Deferred until reference semantics are needed |
| Dynamic resizing | BASIC arrays are fixed-size after DIM |
| Array literals (`[1, 2, 3]`) | No syntax defined yet |

---

## 9. Acceptance Criteria

```
✓ DIM arr(10) emits let arr: Vec<i32> = vec![0i32; 11];
✓ DIM arr(5) AS STRING emits Vec<String> with empty strings
✓ DIM matrix(2, 3) emits Vec<Vec<i32>> with outer length 3, inner length 4
✓ Type annotation defaults to I32 when omitted
✓ All numeric default values are correct (0, 0.0, false)
✓ Full test suite passes
```
