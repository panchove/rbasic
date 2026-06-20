# RFC-0041: Result<T, E>

Status: Future
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add a `Result<T, E>` type to RBASIC, similar to Rust's `Result<T, E>`. This provides a type-safe way to represent the success or failure of an operation, replacing error codes and `ON ERROR RESUME` with explicit, composable error handling. `Result<T, E>` is a generic sum type that wraps either a success value of type `T` or a failure value of type `E`. Requires generics support from Phase 3.

---

## 2. Syntax (EBNF)

```ebnf
result_type    ::= "Result" "<" type_ref "," type_ref ">"

result_ctor    ::= "Ok" "(" expression ")"
                 | "Err" "(" expression ")"

method_call    ::= expression "." IDENTIFIER "(" [expression ("," expression)*] ")"
```

- `Result`, `Ok`, `Err` are case-insensitive reserved keywords.
- `Result<T, E>` declares a result variable with success type `T` and error type `E`.
- `Ok(value)` constructs a successful result.
- `Err(error)` constructs a failure result.
- `.IsOk()` returns `TRUE` if the result is a success.
- `.IsErr()` returns `TRUE` if the result is a failure.
- `.Unwrap()` returns the success value or panics at runtime if `Err`.
- `.UnwrapOr(default)` returns the success value or `default` if `Err`.
- `.Expect(msg)` returns the success value or panics with `msg` if `Err`.
- `.ExpectErr(msg)` returns the error value or panics with `msg` if `Ok`.
- `.Err()` returns the error value or panics if `Ok`.
- `.Map(fn)` transforms the success value, leaving `Err` unchanged.
- `.MapErr(fn)` transforms the error value, leaving `Ok` unchanged.

Examples:

```basic
DIM result AS Result<Integer, String>
result = Ok(42)
PRINT result.IsOk()       ' TRUE
PRINT result.Unwrap()     ' 42

DIM error AS Result<Integer, String>
error = Err("file not found")
PRINT error.IsErr()       ' TRUE
PRINT error.Expect("must succeed")  ' panics: must succeed
```

```basic
FUNCTION Divide(a AS INTEGER, b AS INTEGER) AS Result<Integer, String>
    IF b = 0 THEN
        RETURN Err("division by zero")
    ELSE
        RETURN Ok(a / b)
    END IF
END FUNCTION

DIM r AS Result<Integer, String>
r = Divide(10, 2)
IF r.IsOk() THEN
    PRINT r.Unwrap()      ' 5
END IF
```

```basic
DIM r AS Result<Integer, String>
r = Ok(10)
DIM doubled AS Result<Integer, String>
doubled = r.Map(FUNC(x) -> x * 2)
PRINT doubled.Unwrap()    ' 20
```

---

## 3. Semantics

1. `Result<T, E>` is a generic sum type with two variants: `Ok(T)` and `Err(E)`.
2. Variables declared as `Result<T, E>` can hold either a success value of type `T` or an error value of type `E`.
3. `Ok(expr)` wraps `expr` into `Result<T, E>` where `T` is the type of `expr`.
4. `Err(expr)` wraps `expr` into `Result<T, E>` where `E` is the type of `expr`.
5. `.IsOk()` returns `TRUE` if the result is a success.
6. `.IsErr()` returns `TRUE` if the result is a failure.
7. `.Unwrap()` extracts the success value. Panics with `E2100` if `Err`.
8. `.UnwrapOr(default)` returns the success value or `default` if `Err`.
9. `.Expect(msg)` extracts the success value. Panics with `msg` if `Err`.
10. `.ExpectErr(msg)` extracts the error value. Panics with `msg` if `Ok`.
11. `.Err()` extracts the error value. Panics with `E2101` if `Ok`.
12. `.Map(fn)` applies `fn` to the success value if `Ok`, returns `Err` unchanged.
13. `.MapErr(fn)` applies `fn` to the error value if `Err`, returns `Ok` unchanged.
14. `Result<T, E>` is assignment-compatible with `Result<T, E>` only (no implicit conversion).
15. Implicit unwrapping is not allowed. Explicit methods must be used.

---

## 4. AST (node definitions)

### ResultType (Type Reference)

```text
ResultType {
    ok_type:   Box<TypeRef>,
    err_type:  Box<TypeRef>,
}
```

### ResultLiteral (Expression)

```text
ResultOk {
    value: Box<Expression>,
}

ResultErr {
    value: Box<Expression>,
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

### MapExpression (Expression)

```text
MapCall {
    object:   Box<Expression>,
    function: Box<Expression>,
}

MapErrCall {
    object:   Box<Expression>,
    function: Box<Expression>,
}
```

---

## 5. Parsing

### Result Type Declaration

When `DIM` is followed by an identifier and `AS`:

1. Consume `DIM`.
2. Parse the variable name (identifier).
3. Consume `AS`.
4. Parse `Result<T, E>`.
5. Produce `Statement::Dim { name, type_ref: ResultType(ok_type, err_type) }`.

```rust
fn parse_result_type() -> Result<TypeRef> {
    consume(Result);
    consume(Lt);
    let ok_type = parse_type_ref()?;
    consume(Comma);
    let err_type = parse_type_ref()?;
    consume(Gt);
    Ok(TypeRef::Result(ok_type, err_type))
}
```

### Ok Expression

When `Ok` is followed by `(`:

1. Consume `Ok`.
2. Consume `(`.
3. Parse the inner expression.
4. Consume `)`.
5. Produce `Expression::ResultOk { value }`.

```rust
fn parse_ok_expr() -> Result<Expression> {
    consume(Ok);
    consume(LParen);
    let value = parse_expression()?;
    consume(RParen);
    Ok(Expression::ResultOk { value: Box::new(value) })
}
```

### Err Expression

When `Err` is followed by `(`:

1. Consume `Err`.
2. Consume `(`.
3. Parse the inner expression.
4. Consume `)`.
5. Produce `Expression::ResultErr { value }`.

```rust
fn parse_err_expr() -> Result<Expression> {
    consume(Err);
    consume(LParen);
    let value = parse_expression()?;
    consume(RParen);
    Ok(Expression::ResultErr { value: Box::new(value) })
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

### Map / MapErr

When `.Map(` or `.MapErr(` is encountered:

1. Parse the object expression.
2. Consume `.Map(` or `.MapErr(`.
3. Parse the transformation function/expression.
4. Consume `)`.
5. Produce `Expression::MapCall { object, function }` or `Expression::MapErrCall { object, function }`.

---

## 6. Semantic Analysis

1. **Type registration** — `Result<T, E>` validates that both `T` and `E` are known types. Unknown type emits `E1010`.
2. **Ok construction** — `Ok(expr)` wraps the value. The inferred type is `Result<T, E>` where `T` is the type of `expr`.
3. **Err construction** — `Err(expr)` wraps the error. The inferred type is `Result<T, E>` where `E` is the type of `expr`.
4. **Method validation** — `.IsOk()`, `.IsErr()`, `.Unwrap()`, `.UnwrapOr()`, `.Expect()`, `.ExpectErr()`, `.Err()`, `.Map()`, `.MapErr()` are valid on `Result<T, E>`. Calling on non-result emits `E2102`.
5. **Map signature** — `.Map(fn)` requires `fn` to accept one argument of type `T` and return a value. Return type becomes `Result<R, E>`.
6. **MapErr signature** — `.MapErr(fn)` requires `fn` to accept one argument of type `E` and return a value. Return type becomes `Result<T, R>`.
7. **Unwrap safety** — `.Unwrap()`, `.Expect()`, `.ExpectErr()`, `.Err()` are unchecked at compile time. Runtime safety is the programmer's responsibility.
8. **Type compatibility** — assigning `Result<T, E>` to `T` is not allowed. Must use explicit unwrapping.
9. **Return type** — `RETURN Ok(expr)` and `RETURN Err(expr)` must match the function's declared return type.

---

## 7. Code Generation

### Result Declaration

```basic
DIM result AS Result<Integer, String>
result = Ok(42)
```

Compiles to:

```rust
let mut result: Result<i32, String> = Ok(42);
```

### Err Assignment

```basic
DIM error AS Result<Integer, String>
error = Err("something went wrong")
```

Compiles to:

```rust
let mut error: Result<i32, String> = Err(String::from("something went wrong"));
```

### IsOk / IsErr

```basic
IF result.IsOk() THEN
    PRINT "success"
END IF
```

Compiles to:

```rust
if result.is_ok() {
    println!("success");
}
```

### Unwrap

```basic
PRINT result.Unwrap()
```

Compiles to:

```rust
println!("{}", result.unwrap());
```

### UnwrapOr

```basic
PRINT result.UnwrapOr(0)
```

Compiles to:

```rust
println!("{}", result.unwrap_or(0));
```

### Expect

```basic
PRINT result.Expect("operation must succeed")
```

Compiles to:

```rust
println!("{}", result.expect("operation must succeed"));
```

### Map

```basic
DIM doubled = result.Map(FUNC(x) -> x * 2)
```

Compiles to:

```rust
let doubled = result.map(|x| x * 2);
```

### MapErr

```basic
DIM mapped = error.MapErr(FUNC(e) -> "Error: " + e)
```

Compiles to:

```rust
let mapped = error.map_err(|e| format!("Error: {}", e));
```

### Full Function Example

```basic
FUNCTION Divide(a AS INTEGER, b AS INTEGER) AS Result<Integer, String>
    IF b = 0 THEN
        RETURN Err("division by zero")
    ELSE
        RETURN Ok(a / b)
    END IF
END FUNCTION
```

Compiles to:

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}
```

---

## 8. Error Codes

| Code  | Description                                           |
|-------|-------------------------------------------------------|
| E2100 | Runtime unwrap of Err value                           |
| E2101 | Runtime .Err() call on Ok value                       |
| E2102 | Method called on non-Result type                      |
| E2103 | Type mismatch: expected Result<T,E>, got T            |
| E2104 | Map function signature mismatch                       |
| E2105 | MapErr function signature mismatch                    |
| E1010 | Unknown type in type reference (reused from RFC-0006) |

---

## 9. Acceptance Criteria

```text
✓ Result<T, E> parsed as ResultType with ok and err types
✓ Ok(expr) parsed as ResultOk
✓ Err(expr) parsed as ResultErr
✓ DIM x AS Result<T, E> parsed correctly
✓ .IsOk() method call parsed and validated
✓ .IsErr() method call parsed and validated
✓ .Unwrap() method call parsed and validated
✓ .UnwrapOr(default) method call parsed and validated
✓ .Expect(msg) method call parsed and validated
✓ .ExpectErr(msg) method call parsed and validated
✓ .Err() method call parsed and validated
✓ .Map(fn) method call parsed and validated
✓ .MapErr(fn) method call parsed and validated
✓ Calling method on non-Result produces E2102
✓ Unwrap of Err at runtime produces E2100
✓ .Err() on Ok at runtime produces E2101
✓ Result<T, E> compiles to Rust Result<T, E>
✓ Ok(expr) compiles to Ok(expr)
✓ Err(expr) compiles to Err(expr)
✓ IsOk/IsErr compile to is_ok()/is_err()
✓ Unwrap compiles to unwrap()
✓ UnwrapOr compiles to unwrap_or()
✓ Expect compiles to expect()
✓ Map compiles to map()
✓ MapErr compiles to map_err()
✓ Full test suite passes
```
