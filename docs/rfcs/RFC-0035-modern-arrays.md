# RFC-0035: Modern Arrays

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Modernize RBASIC array syntax with type-parameterized declarations, initializer lists, fixed-size array types, and slicing. This extends the existing `DIM` array support (RFC-0012, RFC-0016) with a more expressive type system while preserving backward compatibility.

---

## 2. Syntax (EBNF)

```ebnf
dim_array_modern ::= "DIM" IDENTIFIER ":" "array" "<" type_ref "," integer_literal ">"
                  |  "DIM" IDENTIFIER "=" array_init
                  |  "DIM" IDENTIFIER "(" dimensions ")" "AS" type_ref

array_init       ::= "{" [expression ("," expression)*] "}"

dimensions       ::= integer_literal ("," integer_literal)*

array_type       ::= "array" "<" type_ref "," integer_literal ">"

slice_expr       ::= expression "[" expression ".." expression "]"

index_expr       ::= expression "(" expression ")"
```

- `DIM`, `AS`, `array` are case-insensitive reserved keywords.
- `array<T, N>` declares an array of type `T` with `N` elements.
- `DIM arr = {1, 2, 3}` infers the type from the initializer.
- `arr[start..end]` returns a slice of the array.
- Existing `DIM arr(N) AS T` syntax remains supported.

Examples:

```basic
DIM arr: array<i32, 11>
arr(0) = 10
PRINT arr(0)
```

```basic
DIM numbers = {1, 2, 3, 4, 5}
PRINT numbers(2)
```

```basic
DIM names: array<string, 3>
names(0) = "Alice"
names(1) = "Bob"
names(2) = "Charlie"

DIM subset = names(0..1)
PRINT subset(0)
PRINT subset(1)
```

```basic
DIM matrix: array<i32, 9>
' 3x3 matrix stored flat
matrix(0) = 1
matrix(4) = 5
matrix(8) = 9
```

---

## 3. Semantics

1. `DIM arr: array<T, N>` declares a fixed-size array of `N` elements of type `T`.
2. `DIM arr = {e1, e2, ...}` infers the element type and size from the initializer.
3. Array indexing uses parentheses: `arr(i)`. Index out of range emits `E1900`.
4. `arr[start..end]` returns a slice from index `start` to `end` (exclusive). Invalid slice emits `E1901`.
5. Array types are value types (copy on assignment). To share data, use references.
6. The `array<T, N>` type is parameterized: `T` is the element type, `N` is the compile-time size.
7. Multi-dimensional arrays use flat storage: `arr(i, j)` maps to `arr(i * cols + j)`.
8. Existing `DIM arr(N) AS T` syntax is equivalent to `DIM arr: array<T, N>`.

---

## 4. AST (node definitions)

### DimArray (Statement)

```text
DimArray {
    name:    String,
    elem_type: Option<TypeRef>,
    size:    Option<Box<Expression>>,
    init:    Option<Vec<Expression>>,
}
```

### ArrayIndex (Expression)

```text
ArrayIndex {
    array: Box<Expression>,
    index: Box<Expression>,
}
```

### ArraySlice (Expression)

```text
ArraySlice {
    array: Box<Expression>,
    start: Box<Expression>,
    end:   Box<Expression>,
}
```

### ArrayInit (Expression)

```text
ArrayInit {
    elements: Vec<Expression>,
}
```

---

## 5. Parsing

### Modern DIM with Type Parameter

When `DIM` is followed by an identifier and `:`:

1. Consume `DIM`.
2. Parse the variable name (identifier).
3. Consume `:`.
4. Parse `array<type, size>`.
5. Produce `Statement::DimArray { name, elem_type, size, init: None }`.

```rust
fn parse_dim_array_modern() -> Result<Statement> {
    consume(Dim);
    let name = expect_identifier()?;
    consume(Colon);
    consume(Array);
    consume(Lt);
    let elem_type = parse_type_ref()?;
    consume(Comma);
    let size = parse_expression()?;
    consume(Gt);
    Ok(Statement::DimArray {
        name,
        elem_type: Some(elem_type),
        size: Some(Box::new(size)),
        init: None,
    })
}
```

### Initializer List

When `DIM` is followed by an identifier and `=`:

1. Consume `DIM`.
2. Parse the variable name (identifier).
3. Consume `=`.
4. Parse `{ expr, expr, ... }`.
5. Produce `Statement::DimArray { name, elem_type: None, size: None, init: Some(elements) }`.

```rust
fn parse_dim_array_init() -> Result<Statement> {
    consume(Dim);
    let name = expect_identifier()?;
    consume(Assign);
    consume(LBrace);
    let mut elements = Vec::new();
    while peek() != RBrace {
        elements.push(parse_expression()?);
        if peek() == Comma { advance(); }
    }
    consume(RBrace);
    Ok(Statement::DimArray {
        name,
        elem_type: None,
        size: None,
        init: Some(elements),
    })
}
```

### Slice Expression

When an expression is followed by `[`:

1. Parse the left-hand side expression.
2. Consume `[`.
3. Parse start expression.
4. Consume `..`.
5. Parse end expression.
6. Consume `]`.
7. Produce `Expression::ArraySlice { array, start, end }`.

---

## 6. Semantic Analysis

1. **Type parameterization** — `array<T, N>` validates that `T` is a known type and `N` is a positive integer.
2. **Initializer inference** — `DIM arr = {1, 2, 3}` infers `array<i32, 3>`.
3. **Index bounds** — array index must be within `0..N`. Static bounds checking emits `E1900` when detectable at compile time.
4. **Slice validation** — `start` must be less than `end`, both within bounds. Invalid slice emits `E1901`.
5. **Type compatibility** — element types in initializer must match the declared element type. Mismatch emits `E1902`.
6. **Multi-dimensional** — `arr(i, j)` is syntactic sugar for `arr(i * cols + j)`.

---

## 7. Code Generation

### Type-Parameterized Array

```basic
DIM arr: array<i32, 11>
```

Compiles to:

```rust
let mut arr: [i32; 11] = [0; 11];
```

### Initializer Array

```basic
DIM numbers = {1, 2, 3, 4, 5}
```

Compiles to:

```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
```

### Slice

```basic
DIM subset = arr(2..5)
```

Compiles to:

```rust
let subset: &[i32] = &arr[2..5];
```

### Array Index

```basic
PRINT arr(3)
```

Compiles to:

```rust
println!("{}", arr[3]);
```

---

## 8. Error Codes

| Code  | Description                                           |
|-------|-------------------------------------------------------|
| E1900 | Array index out of bounds                             |
| E1901 | Invalid slice range                                   |
| E1902 | Type mismatch in array initializer                    |

---

## 9. Acceptance Criteria

```text
✓ DIM arr: array<T, N> parsed as DimArray with type and size
✓ DIM arr = {1,2,3} parsed as DimArray with initializer
✓ Array type inferred from initializer
✓ Array indexing parsed as ArrayIndex
✓ Slice parsed as ArraySlice
✓ Index out of bounds produces E1900
✓ Invalid slice range produces E1901
✓ Type mismatch in initializer produces E1902
✓ Modern DIM compiles to Rust fixed-size array
✓ Initializer compiles to array literal
✓ Slice compiles to Rust slice
✓ Existing DIM arr(N) AS T syntax preserved
✓ Full test suite passes
```
