# RFC-0030: ENUM (Enumerations)

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add enumeration types to RBASIC via `ENUM...END ENUM`. Enumerations define a set of named integer constants, providing type-safe symbolic values. Members can have explicit integer values or auto-increment from a base of 0.

---

## 2. Syntax (EBNF)

```ebnf
enum_decl  ::= "ENUM" IDENTIFIER NEWLINE
               member_decl+
               "END" "ENUM"

member_decl ::= IDENTIFIER ["=" integer_literal]

dim_enum   ::= "DIM" IDENTIFIER "AS" IDENTIFIER
```

- `ENUM`, `END ENUM`, `AS`, `DIM` are case-insensitive reserved keywords.
- Enum members are identifiers, optionally assigned explicit integer values.
- Members without explicit values auto-increment from 0 (first member) or from the previous member + 1.
- Duplicate member names within an enum emit `E1400`.
- Duplicate enum type names in the same scope emit `E1401`.

Examples:

```basic
ENUM Color
    RED
    GREEN
    BLUE
END ENUM

DIM c AS Color
c = Color.RED
PRINT c
```

```basic
ENUM HttpStatus
    OK = 200
    NOT_FOUND = 404
    SERVER_ERROR = 500
END ENUM

DIM status AS HttpStatus
status = HttpStatus.NOT_FOUND
```

```basic
ENUM LogLevel
    DEBUG = 0
    INFO
    WARN
    ERROR
END ENUM

' INFO = 1, WARN = 2, ERROR = 3
```

---

## 3. Semantics

1. `ENUM name ... END ENUM` registers a new enumeration type with the given name.
2. Each member is an integer constant. Members without `= value` auto-increment.
3. The first member defaults to 0 if no explicit value is given.
4. Enum values are of type INTEGER internally.
5. `DIM x AS TypeName` declares a variable of the enum type.
6. `x = EnumName.Member` assigns a member value.
7. Enum members are accessed via qualified names: `EnumName.Member`.
8. Enums are assignment-compatible with INTEGER but not with other enums (no implicit conversion).
9. A member can be used in expressions wherever an INTEGER is expected.

---

## 4. AST (node definitions)

### EnumDecl

```text
EnumDecl {
    name:    String,
    members: Vec<EnumMember>,
}

EnumMember {
    name:  String,
    value: Option<i64>,
}
```

### EnumAccess (Expression)

```text
EnumAccess {
    enum_name: String,
    member:    String,
}
```

---

## 5. Parsing

### Enum Declaration

When `ENUM` is encountered at statement level:

1. Consume `ENUM`.
2. Parse the enum name (identifier).
3. Parse member declarations until `END ENUM`.
4. Consume `END ENUM`.
5. Produce `Statement::EnumDecl { name, members }`.

```rust
fn parse_enum_decl() -> Result<EnumDecl> {
    consume(Enum);
    let name = expect_identifier()?;
    let mut members = Vec::new();
    let mut next_value: i64 = 0;
    loop {
        if peek() == End && peek_ahead() == Enum {
            advance(); // END
            advance(); // ENUM
            break;
        }
        let member_name = expect_identifier()?;
        let value = if peek() == Assign {
            advance();
            let v = expect_integer_literal()?;
            next_value = v + 1;
            Some(v)
        } else {
            let v = next_value;
            next_value += 1;
            None
        };
        members.push(EnumMember { name: member_name, value });
    }
    Ok(EnumDecl { name, members })
}
```

### Enum Access

When an identifier is followed by a dot and another identifier:

1. Parse the left-hand side as an identifier (enum name).
2. Consume the dot.
3. Parse the right-hand side as a member name (identifier).
4. Produce `Expression::EnumAccess { enum_name, member }`.

```rust
fn parse_enum_access(enum_name: String) -> Result<Expression> {
    consume(Dot);
    let member = expect_identifier()?;
    Ok(Expression::EnumAccess { enum_name, member })
}
```

---

## 6. Semantic Analysis

1. **Type registration** — `ENUM name` registers the enum type. Duplicate enum names emit `E1401`.
2. **Member uniqueness** — duplicate member names within an enum emit `E1400`.
3. **Value assignment** — auto-increment assigns sequential integers starting from 0.
4. **Variable declaration** — `DIM x AS EnumName` validates the enum type exists. Unknown type emits `E1010`.
5. **Member access** — `EnumName.Member` validates the enum type and member exist. Unknown member emits `E1402`.
6. **Type compatibility** — enum assignments to INTEGER are allowed. Assigning a different enum type to an enum variable emits `E1403`.

---

## 7. Code Generation

### Enum Declaration

```basic
ENUM Color
    RED
    GREEN
    BLUE
END ENUM
```

Compiles to:

```rust
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}
```

### Enum Variable

```basic
DIM c AS Color
c = Color.RED
```

Compiles to:

```rust
let mut c: Color = Color::Red;
```

### Enum Access

```basic
PRINT Color.GREEN
```

Compiles to:

```rust
println!("{:?}", Color::Green);
```

---

## 8. Error Codes

| Code  | Description                                      |
|-------|--------------------------------------------------|
| E1400 | Duplicate member name in ENUM declaration        |
| E1401 | Duplicate ENUM type name in same scope           |
| E1402 | Unknown member in enum access                    |
| E1403 | Incompatible enum type in assignment             |
| E1010 | Unknown type in DIM AS (reused from RFC-0006)    |

---

## 9. Acceptance Criteria

```text
✓ ENUM ... END ENUM parsed as EnumDecl
✓ Members parsed with optional explicit values
✓ Auto-increment works correctly (0, 1, 2, ...)
✓ Explicit values override auto-increment
✓ Duplicate member name produces E1400
✓ Duplicate enum type produces E1401
✓ Unknown member access produces E1402
✓ Incompatible enum assignment produces E1403
✓ DIM AS EnumName parsed correctly
✓ Enum compiles to Rust enum with repr(i32)
✓ Enum access compiles to qualified variant
✓ Full test suite passes
```
