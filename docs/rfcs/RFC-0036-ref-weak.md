# RFC-0036: Ref<T> / Weak<T> (Smart Pointers)

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add `Ref<T>` and `Weak<T>` smart pointer types to RBASIC for shared and weak ownership semantics. `Ref<T>` provides reference-counted shared ownership (maps to `Rc<T>`/`Arc<T>`). `Weak<T>` provides non-owning references that do not prevent deallocation (maps to `Weak<T>`). This RFC depends on RFC-0014 (Memory Management).

---

## 2. Syntax (EBNF)

```ebnf
dim_ref       ::= "DIM" IDENTIFIER ":" "Ref" "<" type_ref ">"
dim_weak      ::= "DIM" IDENTIFIER ":" "Weak" "<" type_ref ">"
create_ref    ::= "Ref" "<" type_ref ">" "(" expression ")"
create_weak   ::= IDENTIFIER ".Downgrade" "(" ")"

method_call   ::= IDENTIFIER "." IDENTIFIER "(" [arg_list] ")"

is_some       ::= IDENTIFIER ".IsSome" "(" ")"
unwrap        ::= IDENTIFIER ".Unwrap" "(" ")"
upgrade       ::= IDENTIFIER ".Upgrade" "(" ")"
```

- `Ref`, `Weak`, `Downgrade`, `IsSome`, `Unwrap`, `Upgrade` are case-insensitive identifiers.
- `Ref<T>` creates a new reference-counted value.
- `x.Downgrade()` creates a `Weak<T>` from a `Ref<T>`.
- `w.Upgrade()` attempts to upgrade a `Weak<T>` to a `Ref<T>`.
- `w.IsSome()` returns `TRUE` if the weak reference is still valid.
- `w.Unwrap()` returns the value if valid, otherwise panics.

Examples:

```basic
DIM shared: Ref<i32>
shared = Ref<i32>(42)

DIM weak: Weak<i32>
weak = shared.Downgrade()

IF weak.IsSome() THEN
    DIM val = weak.Unwrap()
    PRINT val
END IF

DIM upgraded: Ref<i32>
upgraded = weak.Upgrade()
IF upgraded.IsSome() THEN
    PRINT upgraded.Unwrap()
END IF
```

---

## 3. Semantics

1. `Ref<T>` creates a reference-counted value with shared ownership. When all `Ref<T>` handles are dropped, the value is deallocated.
2. `Weak<T>` is a non-owning reference. It does not prevent deallocation of the value.
3. `Ref<T>(value)` creates a new `Ref<T>` containing `value`.
4. `x.Downgrade()` creates a `Weak<T>` from `Ref<T>`.
5. `w.Upgrade()` returns a new `Ref<T>` if the value is still alive, otherwise returns `NULL`.
6. `w.IsSome()` returns `TRUE` if the value is still alive.
7. `w.Unwrap()` returns the value if alive, otherwise panics at runtime.
8. `Ref<T>` is copyable (clone increments reference count).
9. `Weak<T>` is copyable (clone creates another weak reference).
10. `Ref<T>` maps to `Rc<T>` (single-threaded) or `Arc<T>` (multi-threaded).
11. `Weak<T>` maps to `std::rc::Weak<T>` or `std::sync::Weak<T>`.

---

## 4. AST (node definitions)

### DimRef (Statement)

```text
DimRef {
    name:     String,
    elem_type: TypeRef,
}
```

### DimWeak (Statement)

```text
DimWeak {
    name:     String,
    elem_type: TypeRef,
}
```

### CreateRef (Expression)

```text
CreateRef {
    elem_type: TypeRef,
    value:     Box<Expression>,
}
```

### MethodCall (Expression)

```text
MethodCall {
    object: Box<Expression>,
    method: String,
    args:   Vec<Expression>,
}
```

---

## 5. Parsing

### Ref Declaration

```rust
fn parse_dim_ref() -> Result<Statement> {
    consume(Dim);
    let name = expect_identifier()?;
    consume(Colon);
    consume(Ref);
    consume(Lt);
    let elem_type = parse_type_ref()?;
    consume(Gt);
    Ok(Statement::DimRef { name, elem_type })
}
```

### Weak Declaration

```rust
fn parse_dim_weak() -> Result<Statement> {
    consume(Dim);
    let name = expect_identifier()?;
    consume(Colon);
    consume(Weak);
    consume(Lt);
    let elem_type = parse_type_ref()?;
    consume(Gt);
    Ok(Statement::DimWeak { name, elem_type })
}
```

### Ref Construction

```rust
fn parse_create_ref() -> Result<Expression> {
    consume(Ref);
    consume(Lt);
    let elem_type = parse_type_ref()?;
    consume(Gt);
    consume(LParen);
    let value = parse_expression()?;
    consume(RParen);
    Ok(Expression::CreateRef {
        elem_type,
        value: Box::new(value),
    })
}
```

### Method Calls

When an identifier is followed by `.` and a method name:

```rust
fn parse_method_call(object: Expression) -> Result<Expression> {
    consume(Dot);
    let method = expect_identifier()?;
    consume(LParen);
    let mut args = Vec::new();
    if peek() != RParen {
        args.push(parse_expression()?);
        while peek() == Comma {
            advance();
            args.push(parse_expression()?);
        }
    }
    consume(RParen);
    Ok(Expression::MethodCall {
        object: Box::new(object),
        method,
        args,
    })
}
```

---

## 6. Semantic Analysis

1. **Ref type validation** — `Ref<T>` validates that `T` is a known type. Unknown type emits `E2000`.
2. **Weak type validation** — `Weak<T>` validates that `T` is a known type. Unknown type emits `E2000`.
3. **Downgrade source** — `x.Downgrade()` requires `x` to be `Ref<T>`. Other types emit `E2001`.
4. **Upgrade source** — `w.Upgrade()` requires `w` to be `Weak<T>`. Other types emit `E2002`.
5. **IsSome/Unwrap source** — requires a `Weak<T>` receiver. Other types emit `E2003`.
6. **Type propagation** — `Ref<T>.Downgrade()` returns `Weak<T>`. `Weak<T>.Upgrade()` returns `Ref<T>` (or NULL).

---

## 7. Code Generation

### Ref Declaration

```basic
DIM shared: Ref<i32>
shared = Ref<i32>(42)
```

Compiles to:

```rust
use std::rc::Rc;
let shared: Rc<i32> = Rc::new(42);
```

### Weak Declaration

```basic
DIM weak: Weak<i32>
weak = shared.Downgrade()
```

Compiles to:

```rust
use std::rc::Weak;
let weak: Weak<i32> = Rc::downgrade(&shared);
```

### Upgrade

```basic
DIM val = weak.Upgrade()
```

Compiles to:

```rust
let val: Option<Rc<i32>> = weak.upgrade();
```

### IsSome / Unwrap

```basic
IF weak.IsSome() THEN
    PRINT weak.Unwrap()
END IF
```

Compiles to:

```rust
if let Some(val) = weak.upgrade() {
    println!("{}", val);
}
```

---

## 8. Error Codes

| Code  | Description                                            |
|-------|--------------------------------------------------------|
| E2000 | Unknown type in Ref<T> or Weak<T>                      |
| E2001 | Downgrade called on non-Ref type                       |
| E2002 | Upgrade called on non-Weak type                        |
| E2003 | IsSome/Unwrap called on non-Weak type                  |

---

## 9. Acceptance Criteria

```text
✓ DIM x: Ref<T> parsed as DimRef
✓ DIM x: Weak<T> parsed as DimWeak
✓ Ref<T>(value) parsed as CreateRef
✓ x.Downgrade() parsed as MethodCall
✓ w.Upgrade() parsed as MethodCall
✓ w.IsSome() parsed as MethodCall
✓ w.Unwrap() parsed as MethodCall
✓ Downgrade on non-Ref produces E2001
✓ Upgrade on non-Weak produces E2002
✓ Ref<T> compiles to Rc::new
✓ Weak<T> compiles to Rc::downgrade
✓ Upgrade compiles to .upgrade()
✓ Full test suite passes
```
