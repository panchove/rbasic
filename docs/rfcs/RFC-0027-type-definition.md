# RFC-0027: TYPE...END TYPE (User-Defined Types)

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add user-defined composite types to RBASIC via `TYPE...END TYPE`. This allows programmers to define structured records with named fields, similar to C structs or classic BASIC `TYPE` blocks. Instances are declared with `DIM` and fields are accessed with dot notation.

---

## 2. Syntax (EBNF)

```ebnf
type_decl   ::= "TYPE" IDENTIFIER NEWLINE
                field_decl+
                "END" "TYPE"

field_decl  ::= IDENTIFIER "AS" type_ref

type_ref    ::= IDENTIFIER

dim_stmt    ::= "DIM" IDENTIFIER "AS" IDENTIFIER
            |  "DIM" IDENTIFIER "(" dimensions ")" "AS" IDENTIFIER

member_access ::= IDENTIFIER "." IDENTIFIER
```

- `TYPE`, `END TYPE`, `AS`, `DIM` are case-insensitive reserved keywords.
- A type name is an identifier registered as a user-defined type.
- Field names must be unique within a type. Duplicate field names emit `E1130`.
- Field types must be valid type names (primitives or other user-defined types).
- A type cannot contain itself (recursive types are not supported). Self-referencing emits `E1131`.

Examples:

```basic
TYPE Point
    x AS DOUBLE
    y AS DOUBLE
END TYPE

DIM origin AS Point
origin.x = 0
origin.y = 0
PRINT origin.x; origin.y
```

```basic
TYPE Person
    name AS STRING
    age AS INTEGER
    active AS BOOLEAN
END TYPE

DIM p AS Person
p.name = "Alice"
p.age = 30
p.active = TRUE
```

```basic
TYPE Rectangle
    top_left AS Point
    width AS DOUBLE
    height AS DOUBLE
END TYPE
```

---

## 3. Semantics

1. `TYPE name ... END TYPE` registers a new user-defined type with the given name.
2. Each `field AS type` declaration adds a named field to the type.
3. Instances are declared with `DIM instance AS TypeName`.
4. Fields are accessed with `instance.field` (dot notation).
5. Field access validates that the field exists on the type. Unknown fields emit `E1132`.
6. Assignment to a field respects mutability: the instance must be mutable, and the field type must match the assigned expression's type.
7. Types are resolved at compile time; no runtime type information is generated.
8. A type can reference other user-defined types as field types (composition, not nesting).

---

## 4. AST (node definitions)

### TypeDecl

```text
TypeDecl {
    name:   String,
    fields: Vec<Field>,
}

Field {
    name: String,
    typ:  TypeRef,
}
```

### MemberAccess (Expression)

```text
MemberAccess {
    object: Box<Expression>,
    field:  String,
}
```

### Updated TypeRef

The `TypeRef` node may reference a user-defined type name in addition to primitive types:

```text
TypeRef {
    name: String,
}
```

User-defined types are resolved during semantic analysis; the parser treats the type name as an opaque identifier.

---

## 5. Parsing

### TYPE Declaration

When `TYPE` keyword is encountered at statement level:

1. Consume `TYPE`.
2. Parse the type name (identifier).
3. Parse field declarations (`identifier AS type_ref`) until `END TYPE`.
4. Consume `END TYPE`.
5. Produce `Statement::TypeDecl { name, fields }`.

```rust
fn parse_type_decl() -> Result<TypeDecl> {
    consume(Type);
    let name = expect_identifier()?;
    let mut fields = Vec::new();
    loop {
        if peek() == End && peek_ahead() == Type {
            advance(); // END
            advance(); // TYPE
            break;
        }
        let field_name = expect_identifier()?;
        consume(As);
        let typ = parse_type_ref()?;
        fields.push(Field { name: field_name, typ });
    }
    Ok(TypeDecl { name, fields })
}
```

### Member Access

When an `IDENTIFIER` is followed by a `Dot`:

1. Parse the left-hand side as an expression.
2. Consume the dot.
3. Parse the right-hand side as a field name (identifier).
4. Produce `Expression::MemberAccess { object, field }`.

```rust
fn parse_member_access(object: Expression) -> Result<Expression> {
    consume(Dot);
    let field = expect_identifier()?;
    Ok(Expression::MemberAccess {
        object: Box::new(object),
        field,
    })
}
```

---

## 6. Semantic Analysis

1. **Type registration** — `TYPE name` registers the type in the type table. Duplicate type names emit `E1133`.
2. **Field uniqueness** — duplicate field names within a type emit `E1130`.
3. **Field type validity** — field types must be recognized types. Unknown types emit `E1010`.
4. **No self-referencing** — a type cannot have a field whose type is the type itself. Detected via dependency graph analysis. Self-reference emits `E1131`.
5. **Instance declaration** — `DIM x AS TypeName` validates that `TypeName` is a registered type. Unknown type emits `E1010`.
6. **Field access** — `x.field` validates that `x` is a type instance and `field` exists on that type. Unknown field emits `E1132`.
7. **Assignment to field** — `x.field = expr` validates type compatibility (E1020) and mutability (E1042).

---

## 7. Code Generation

### Type Declaration

```basic
TYPE Point
    x AS DOUBLE
    y AS DOUBLE
END TYPE
```

Compiles to a Rust struct:

```rust
#[derive(Clone, Copy, Default)]
struct Point {
    x: f64,
    y: f64,
}
```

### Instance Declaration

```basic
DIM origin AS Point
```

Compiles to:

```rust
let mut origin = Point::default();
```

### Field Access

```basic
origin.x = 5.0
PRINT origin.y
```

Compiles to:

```rust
origin.x = 5.0;
println!("{}", origin.y);
```

### Nested Types

```basic
TYPE Rectangle
    top_left AS Point
    width AS DOUBLE
    height AS DOUBLE
END TYPE
```

Compiles to:

```rust
#[derive(Clone, Default)]
struct Rectangle {
    top_left: Point,
    width: f64,
    height: f64,
}
```

---

## 8. Error Codes

| Code  | Description                                         |
|-------|-----------------------------------------------------|
| E1130 | Duplicate field name in TYPE declaration             |
| E1131 | Self-referencing TYPE (recursive type not allowed)  |
| E1132 | Unknown field on type instance                       |
| E1133 | Duplicate TYPE declaration in scope                  |
| E1010 | Unknown type in field annotation (reused from RFC-0006) |
| E1020 | Type mismatch in field assignment (reused from RFC-0007) |
| E1042 | Assignment to immutable field (reused from RFC-0015) |

---

## 9. Acceptance Criteria

```text
✓ TYPE ... END TYPE parsed as TypeDecl
✓ Field declarations parsed correctly
✓ Member access parsed as MemberAccess expression
✓ Duplicate field name produces E1130
✓ Self-referencing type produces E1131
✓ Unknown field access produces E1132
✓ Duplicate TYPE name produces E1133
✓ Unknown type in field annotation produces E1010
✓ TYPE compiles to Rust struct
✓ Field access compiles to dot notation
✓ Nested types compile correctly
✓ Default values via derive(Default)
✓ Full test suite passes
```
