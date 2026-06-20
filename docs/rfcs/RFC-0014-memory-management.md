# RFC-0014: Hybrid Stack + ARC Memory Management

Status: Accepted
Version: 0.2
Author: RBASIC Project
Created: 2026-06-19
Last Updated: 2026-06-20

---

## 1. Summary

Define a memory management strategy for RBASIC using a hybrid model: primitive types live on the stack with copy semantics, while heap-allocated types (`STRING`, arrays) use Automatic Reference Counting (ARC). Phase 2 adds `Ref<T>` (strong reference) and `Weak<T>` (weak reference) to break cycles. No garbage collector, no borrow checker — the model is simple, deterministic, and familiar to BASIC programmers.

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

### 2.3 Strong and Weak References (Phase 2)

| Type      | Rust repr     | Behaviour                                    |
|-----------|---------------|----------------------------------------------|
| `Ref<T>`  | `Rc<T>`       | Strong reference; keeps object alive          |
| `Weak<T>` | `Weak<T>`     | Weak reference; does not keep object alive    |

```basic
DIM parent: Ref<Node> = Node()
DIM child: Ref<Node> = Node()
child.parent = Weak(parent)    ' does not increment refcount

' Check if object is still alive
IF child.parent.IsSome() THEN
    DIM p = child.parent.Unwrap()
END IF
```

Weak references break reference cycles:

```basic
DIM a: Ref<Node> = Node()
DIM b: Ref<Node> = Node()
a.next = Ref(b)        ' strong: refcount = 2
b.next = Weak(a)       ' weak: does not increment refcount
' No cycle: when a goes out of scope, refcount drops to 0, both freed
```

---

## 3. Lifecycle

- **Entering scope** allocates the variable.
- **Leaving scope** runs `Drop`, which decrements the reference count.
- **Refcount reaches 0**: heap memory is freed.
- **`Weak` references** do not prevent deallocation. Use `IsSome()` to check if the object is alive, `Unwrap()` to access it.
- **Deterministic cleanup**, no stop‑the‑world pauses.

---

## 4. ARC Strategy

- Every heap-allocated value is wrapped in `std::rc::Rc<T>`.
- Assignment of a reference type performs `Rc::clone()` — O(1), increments counter.
- Leaving scope runs `Drop`, which decrements the counter. When it reaches 0, the heap memory is freed.
- `Weak<T>` uses `std::rc::Weak<T>` — does not increment the strong reference count.
- No tracing, no stop-the-world pauses — memory reclamation is deterministic.

---

## 5. Code Generation (Rust)

### 5.1 STRING (Phase 1)

In Phase 1, every RBASIC `STRING` variable is emitted as Rust `String`. Rust's `String` is already heap-allocated with RAII semantics (deterministic cleanup via `Drop`). This satisfies the abstract ARC model — each string owns its heap allocation, and scope exit frees it. `Rc<String>` will be introduced in Phase 2 when reference semantics (shared ownership across assignments) are needed.

```basic
DIM x AS STRING = "hello"
```

```rust
let x: String = "hello".to_string();
```

String literals in expressions: `"literal".to_string()`.

### 5.2 Arrays (DIM, Phase 1)

Array variables are emitted as `Vec<T>` (owns its heap storage). `Rc<Vec<T>>` will be introduced when shared array semantics are needed.

```basic
DIM arr(10) AS INTEGER
```

```rust
let arr: Vec<i32> = vec![0i32; 11];
```

### 5.3 Ref<T> and Weak<T> (Phase 2)

```basic
DIM a: Ref<Node> = Node()
DIM b: Weak<Node> = Weak(a)
```

```rust
let a: Rc<Node> = Rc::new(Node::new());
let b: Weak<Node> = Rc::downgrade(&a);
```

---

## 6. Runtime

The runtime library (`runtime/`) is not needed for Phase 1 code generation (all string and array allocation is emitted as inline Rust). Stub files remain as placeholders for future C FFI requirements.

---

## 7. Impact on Existing RFCs

| RFC            | Impact                                           |
|----------------|--------------------------------------------------|
| RFC-0005 (AST) | No change — `Type::String` and `ArrayType` unchanged |
| RFC-0007 (Types) | No change — compatibility matrix unaffected   |
| RFC-0012 (DIM) | Codegen emits `Vec<T>` in Phase 1; `Rc<Vec<T>>` deferred |

---

## 8. Phase Summary

| Phase | Feature | Status |
|-------|---------|--------|
| Phase 1 | Value types (stack, copy) | Implemented |
| Phase 1 | STRING as Rust `String` | Implemented |
| Phase 1 | DIM arrays as `Vec<T>` | Implemented |
| Phase 2 | STRING as `Rc<String>` | Planned |
| Phase 2 | DIM arrays as `Rc<Vec<T>>` | Planned |
| Phase 2 | `Ref<T>` (strong reference) | Planned |
| Phase 2 | `Weak<T>` (weak reference, breaks cycles) | Planned |

---

## 9. Acceptance Criteria

```
✓ Value types copied on assignment (stack)
✓ STRING uses Rust String (heap-allocated, RAII cleanup) in Phase 1
✓ DIM arrays use Vec<T> (heap-allocated, RAII cleanup) in Phase 1
✓ Scope exit runs Drop; heap memory freed when variable goes out of scope
✓ Ref<T> uses Rc<T> in Phase 2
✓ Weak<T> uses Weak<T> in Phase 2
✓ Weak references do not prevent deallocation
✓ IsSome() and Unwrap() work correctly on Weak<T>
✓ Codegen emits correct Rust for all types
✓ Full test suite passes
```

---

## 10. Open Questions

1. **Rc<T> migration?** When assignment sharing is needed (Phase 2), STRING → `Rc<String>` and arrays → `Rc<Vec<T>>`.
2. **Copy-on-write for arrays?** `Rc::make_mut()` provides COW semantics automatically — recommended for the Rc migration version.
3. **C FFI?** Inline Rust emission is sufficient for Phase 1; C runtime helpers may be added when the language is self-hosting or needs C interop.
