# RFC-0040: Optional<T>

Status: Future
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add an `Optional<T>` type to RBASIC, similar to Rust's `Option<T>`. This provides a type-safe way to represent the presence or absence of a value, eliminating null pointer exceptions and forced unwraps. `Optional<T>` is a generic type that wraps a value of type `T` and exposes methods to inspect and extract the contained value. Requires generics support from Phase 3.

---

## 2. Syntax (EBNF)

```ebnf
optional_type   ::= "Optional" "<" type_ref ">"

optional_ctor   ::= "Some" "(" expression ")"
                  | "None"

method_call     ::= expression "." IDENTIFIER "(" [expression ("," expression)*] ")"
```

- `Optional`, `Some`, `None` are case-insensitive reserved keywords.
- `Optional<T>` declares an optional variable with inner type `T`.
- `Some(value)` constructs a present optional.
- `None` constructs an absent optional.
- `.IsSome()` returns `true` if the optional contains a value.
- `.IsNone()` returns `true` if the optional is absent.
- `.Unwrap()` returns the inner value or panics at runtime if absent.
- `.UnwrapOr(default)` returns the inner value or the provided default.
- `.Expect(msg)` returns the inner value or panics with `msg` if absent.

Examples:

```basic
DIM name AS Optional<String>
name = Some("Alice")
PRINT name.IsSome()       ' TRUE
PRINT name.Unwrap()       ' Alice

DIM empty AS Optional<String>
empty = None
PRINT empty.IsNone()      ' TRUE
PRINT empty.UnwrapOr("N/A")  ' N/A
```

```basic
FUNCTION FindUser(id AS INTEGER) AS Optional<String>
    IF id = 1 THEN
        RETURN Some("Alice")
    ELSE
        RETURN None
    END IF
END FUNCTION

DIM result AS Optional<String>
result = FindUser(1)
IF result.IsSome() THEN
    PRINT result.Unwrap()
END IF
```

---

## 3. Semantics

1. `Optional<T>` is a generic sum type with two variants: `Some(T)` and `None`.
2. Variables declared as `Optional<T>` can hold either a value of type `T` or `None`.
3. `Some(expr)` wraps `expr` into an `Optional<T>` where `T` is the type of `expr`.
4. `None` represents the absence of a value; its type is inferred from context.
5. `.IsSome()` returns `TRUE` if the optional contains a value.
6. `.IsNone()` returns `TRUE` if the optional is absent.
7. `.Unwrap()` extracts the inner value. Panics with `E2000` if `None`.
8. `.UnwrapOr(default)` returns the inner value or `default` if `None`.
9. `.Expect(msg)` extracts the inner value. Panics with `msg` if `None`.
10. `Optional<T>` is assignment-compatible with `Optional<T>` only (no implicit conversion).
11. `Optional<T>` is not assignment-compatible with `T`. Explicit unwrapping is required.

---

## 4. AST (node definitions)

### OptionalType (Type Reference)

```text
OptionalType {
    inner: Box<TypeRef>,
}
```

### OptionalLiteral (Expression)

```text
OptionalSome {
    value: Box<Expression>,
}

OptionalNone {
    inferred_type: Option<TypeRef>,
}
```

### MethodCall (Expression)

```text
MethodCall {
    object:   Box<Expression>,
    method:   String,
    args:     Vec<Expression>,
}
```

### ReturnStatement (Expression)

```text
Return {
    value: Option<Box<Expression>>,
}
```

---

## 5. Parsing

### Optional Type Declaration

When `DIM` is followed by an identifier and `AS`:

1. Consume `DIM`.
2. Parse the variable name (identifier).
3. Consume `AS`.
4. Parse `Optional<type>`.
5. Produce `Statement::Dim { name, type_ref: OptionalType(inner) }`.

```rust
fn parse_optional_type() -> Result<TypeRef> {
    consume(Optional);
    consume(Lt);
    let inner = parse_type_ref()?;
    consume(Gt);
    Ok(TypeRef::Optional(Box::new(inner)))
}
```

### Some Expression

When `Some` is followed by `(`:

1. Consume `Some`.
2. Consume `(`.
3. Parse the inner expression.
4. Consume `)`.
5. Produce `Expression::OptionalSome { value }`.

```rust
fn parse_some_expr() -> Result<Expression> {
    consume(Some);
    consume(LParen);
    let value = parse_expression()?;
    consume(RParen);
    Ok(Expression::OptionalSome { value: Box::new(value) })
}
```

### None Expression

When `None` is encountered:

1. Consume `None`.
2. Produce `Expression::OptionalNone { inferred_type: None }`.

```rust
fn parse_none_expr() -> Result<Expression> {
    consume(None);
    Ok(Expression::OptionalNone { inferred_type: None })
}
```

### Method Call

When an expression is followed by `.` and an identifier with `(`:

1. Parse the left-hand side expression.
2. Consume `.`.
3. Parse the method name (identifier).
4. Consume `(`.
5. Parse argument list (optional).
6. Consume `)`.
7. Produce `Expression::MethodCall { object, method, args }`.

---

## 6. Semantic Analysis

1. **Type registration** — `Optional<T>` validates that `T` is a known type. Unknown type emits `E1010`.
2. **Some construction** — `Some(expr)` wraps the value. The inferred type is `Optional<T>` where `T` is the type of `expr`.
3. **None inference** — `None` type is inferred from the target variable or context. Unresolvable type emits `E2001`.
4. **Method validation** — `.IsSome()`, `.IsNone()`, `.Unwrap()`, `.UnwrapOr()`, `.Expect()` are valid on `Optional<T>`. Calling on non-optional emits `E2002`.
5. **Unwrap safety** — `.Unwrap()` and `.Expect()` are unchecked at compile time. Runtime safety is the programmer's responsibility.
6. **Type compatibility** — assigning `Optional<T>` to `T` is not allowed. Must use explicit unwrapping.
7. **Return type** — `RETURN Some(expr)` and `RETURN None` must match the function's declared return type.

---

## 7. Code Generation

### Optional Declaration

```basic
DIM name AS Optional<String>
name = Some("Alice")
```

Compiles to:

```rust
let mut name: Option<String> = Some(String::from("Alice"));
```

### None Assignment

```basic
DIM val AS Optional<Integer>
val = None
```

Compiles to:

```rust
let mut val: Option<i32> = None;
```

### IsSome / IsNone

```basic
IF name.IsSome() THEN
    PRINT "has value"
END IF
```

Compiles to:

```rust
if name.is_some() {
    println!("has value");
}
```

### Unwrap

```basic
PRINT name.Unwrap()
```

Compiles to:

```rust
println!("{}", name.unwrap());
```

### UnwrapOr

```basic
PRINT name.UnwrapOr("default")
```

Compiles to:

```rust
println!("{}", name.unwrap_or(String::from("default")));
```

### Expect

```basic
PRINT name.Expect("name should exist")
```

Compiles to:

```rust
println!("{}", name.expect("name should exist"));
```

---

## 8. Error Codes

| Code  | Description                                           |
|-------|-------------------------------------------------------|
| E2000 | Runtime unwrap of None value                          |
| E2001 | Cannot infer type of None from context                |
| E2002 | Method called on non-Optional type                    |
| E2003 | Type mismatch: expected Optional<T>, got T            |
| E1010 | Unknown type in type reference (reused from RFC-0006) |

---

## 9. Acceptance Criteria

```text
✓ Optional<T> parsed as OptionalType with inner type
✓ Some(expr) parsed as OptionalSome
✓ None parsed as OptionalNone
✓ DIM x AS Optional<T> parsed correctly
✓ .IsSome() method call parsed and validated
✓ .IsNone() method call parsed and validated
✓ .Unwrap() method call parsed and validated
✓ .UnwrapOr(default) method call parsed and validated
✓ .Expect(msg) method call parsed and validated
✓ Calling method on non-Optional produces E2002
✓ Unwrap of None at runtime produces E2000
✓ Optional<T> compiles to Rust Option<T>
✓ Some(expr) compiles to Some(expr)
✓ None compiles to None
✓ IsSome/IsNone compile to is_some()/is_none()
✓ Unwrap compiles to unwrap()
✓ UnwrapOr compiles to unwrap_or()
✓ Expect compiles to expect()
✓ Full test suite passes
```
