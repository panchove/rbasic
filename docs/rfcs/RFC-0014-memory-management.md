# RFC-0014: Hybrid Stack + ARC Memory Management

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-06-19
Last Updated: 2026-06-19

---

## 1. Summary

Define a memory management strategy for RBASIC using a hybrid model: primitive types live on the stack with copy semantics, while heap-allocated types (`STRING`, arrays) use Automatic Reference Counting (ARC). No garbage collector, no borrow checker — the model is simple, deterministic, and familiar to BASIC programmers.

---

## 2. Memory Model

### 2.1 Value Types (stack, copy semantics)

| Type      | Rust repr | Size    | Behaviour                      |
|-----------|-----------|---------|--------------------------------|
| `BOOL`    | `bool`    | 1 byte  | Copied on assignment           |
| `I8`      | `i8`      | 1 byte  | Copied on assignment           |
| `I16`     | `i16`     | 2 bytes | Copied on assignment           |
| `I32`     | `i32`     | 4 bytes | Copied on assignment           |
| `I64`     | `i64`     | 8 bytes | Copied on assignment           |
| `U8`      | `u8`      | 1 byte  | Copied on assignment           |
| `U16`     | `u16`     | 2 bytes | Copied on assignment           |
| `U32`     | `u32`     | 4 bytes | Copied on assignment           |
| `U64`     | `u64`     | 8 bytes | Copied on assignment           |
| `F32`     | `f32`     | 4 bytes | Copied on assignment           |
| `F64`     | `f64`     | 8 bytes | Copied on assignment           |

Assignment between value types always produces an independent copy:

```basic
DIM a AS INTEGER = 42
DIM b AS INTEGER = a   ' b = 42, independent copy
a = 10                 ' b still 42
```

### 2.2 Reference Types (heap, ARC semantics)

| Type     | Rust repr              | Behaviour                           |
|----------|------------------------|-------------------------------------|
| `STRING` | `Rc<String>`           | ARC; clone increments refcount      |
| Array    | `Rc<Vec<Value>>`       | ARC; clone increments refcount      |

Assignment between reference types shares the heap allocation and increments the reference count:

```basic
DIM s AS STRING = "hello"
DIM t AS STRING = s     ' s and t share heap; refcount = 2
s = "world"             ' s detached; refcount for "hello" drops to 1
```

---

## 3. ARC Strategy

- Every heap-allocated value is wrapped in `std::rc::Rc<T>`.
- Assignment of a reference type performs `Rc::clone()` — O(1), increments counter.
- Leaving scope runs `Drop`, which decrements the counter. When it reaches 0, the heap memory is freed.
- No tracing, no stop-the-world pauses — memory reclamation is deterministic.
- **Cycles**: The language is designed to make accidental cycles unlikely (strings and arrays only). If cycles become a problem in practice, `Weak<T>` support can be added in a future RFC.

---

## 4. Code Generation (Rust)

### 4.1 STRING (v0.1)

In v0.1, every RBASIC `STRING` variable is emitted as Rust `String`. Rust's `String` is already heap-allocated with RAII semantics (deterministic cleanup via `Drop`). This satisfies the abstract ARC model — each string owns its heap allocation, and scope exit frees it. `Rc<String>` will be introduced in a future version when reference semantics (shared ownership across assignments) are needed.

```basic
DIM x AS STRING = "hello"
```

```rust
let x: String = "hello".to_string();
```

String literals in expressions: `"literal".to_string()`.

### 4.2 Arrays (DIM, v0.1)

Array variables are emitted as `Vec<T>` (owns its heap storage). `Rc<Vec<T>>` will be introduced when shared array semantics are needed.

```basic
DIM arr(10) AS INTEGER
```

```rust
let arr: Vec<i32> = vec![0i32; 11];
```

Array access and mutation require copy-on-write semantics (deferred to a future version).

---

## 5. Runtime

The runtime library (`runtime/`) is not needed for v0.1 code generation (all string and array allocation is emitted as inline Rust). Stub files remain as placeholders for future C FFI requirements.

---

## 6. Impact on Existing RFCs

| RFC            | Impact                                           |
|----------------|--------------------------------------------------|
| RFC-0005 (AST) | No change — `Type::String` and `ArrayType` unchanged |
| RFC-0007 (Types) | No change — compatibility matrix unaffected   |
| RFC-0012 (DIM) | Codegen emits `Vec<T>` in v0.1; `Rc<Vec<T>>` deferred |

---

## 7. Excluded from this RFC

- `Ref<T>`, `MutRef<T>` — require a separate RFC (reserved keywords exist)
- `Optional<T>`, `Result<T, E>` — separate RFC (reserved keywords exist)
- `Weak<T>` — may be added later if cycle-breaking is needed
- Tracing GC — explicitly excluded

---

## 8. Acceptance Criteria

```
✓ Value types copied on assignment (stack)
✓ STRING uses Rust String (heap-allocated, RAII cleanup) in v0.1
✓ DIM arrays use Vec<T> (heap-allocated, RAII cleanup) in v0.1
✓ Scope exit runs Drop; heap memory freed when variable goes out of scope
✓ Codegen emits "string".to_string() and Vec::new() correctly
✓ Full test suite passes
```

---

## 9. Open Questions

1. **Rc<T> migration?** When assignment sharing is needed (v0.2+), STRING → `Rc<String>` and arrays → `Rc<Vec<T>>`.
2. **Copy-on-write for arrays?** `Rc::make_mut()` provides COW semantics automatically — recommended for the Rc migration version.
3. **C FFI?** Inline Rust emission is sufficient for v0.1; C runtime helpers may be added when the language is self-hosting or needs C interop.
