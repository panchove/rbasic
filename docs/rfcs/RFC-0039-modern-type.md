# RFC-0039: Modern TYPE Syntax

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Extend RFC-0027 (TYPE...END TYPE) with modern syntax: colon-delimited field declarations with default values. This provides a more concise and expressive way to define user-defined types while maintaining backward compatibility with the existing `AS` syntax.

---

## 2. Syntax (EBNF)

```ebnf
type_decl_modern ::= "TYPE" IDENTIFIER NEWLINE
                     type_field_modern+
                     "END" "TYPE"

type_field_modern ::= IDENTIFIER ":" type_ref ["=" expression]
                   |  IDENTIFIER "AS" type_ref

type_ref_modern  ::= IDENTIFIER
                   | "i32" | "i64" | "f32" | "f64"
                   | "string" | "bool" | "char"
                   | "usize" | "isize"
```

- `TYPE`, `END TYPE`, `AS` are case-insensitive reserved keywords.
- Modern syntax uses `:` instead of `AS` for type annotations.
- Default values follow `= expression` after the type annotation.
- Both `AS` and `:` syntax may coexist in the same TYPE block.
- Default values must be compatible with the declared type.
- Duplicate field names still emit `E1130`.

Examples:

```basic
TYPE Point
    x: f64 = 0.0
    y: f64 = 0.0
END TYPE

DIM origin AS Point
PRINT origin.x, origin.y
' Output: 0.0 0.0
```

```basic
TYPE Person
    name: string = "Unknown"
    age: i32 = 0
    active: bool = TRUE
END TYPE

DIM p AS Person
p.name = "Alice"
p.age = 30
PRINT p.name, p.age
```

```basic
TYPE Config
    host AS STRING = "localhost"
    port: i32 = 8080
    debug: bool = FALSE
END TYPE

' Mixed AS and : syntax allowed
DIM cfg AS Config
PRINT cfg.host, cfg.port
```

```basic
TYPE Rectangle
    top_left: Point
    width: f64
    height: f64

    FUNCTION area(self) AS f64
        area = self.width * self.height
    END FUNCTION
END TYPE
```

---

## 3. Semantics

1. `TYPE name ... END TYPE` registers a user-defined type (extends RFC-0027).
2. Modern syntax `field: type = default` declares a field with a default value.
3. Legacy syntax `field AS type` declares a field without a default value (zero-initialized).
4. Both syntaxes may coexist in the same TYPE block.
5. Default values are applied when an instance is created without explicit initialization.
6. Default values must be type-compatible with the declared field type. Mismatch emits `E2300`.
7. Default values are evaluated at compile time when possible, runtime otherwise.
8. The `self` parameter in methods refers to the current instance (future extension).
9. Fields without defaults are zero-initialized (0 for integers, 0.0 for floats, "" for strings, FALSE for booleans).

---

## 4. AST (node definitions)

### TypeDecl (Extended)

```text
TypeDecl {
    name:    String,
    fields:  Vec<Field>,
}

Field {
    name:         String,
    typ:          TypeRef,
    default_value: Option<Box<Expression>>,
}
```

### InstanceInit (Expression)

```text
InstanceInit {
    type_name: String,
    fields:    Vec<(String, Expression)>,
}
```

---

## 5. Parsing

### Modern Field Declaration

When a field uses `:` syntax:

1. Parse the field name (identifier).
2. Consume `:`.
3. Parse the type reference.
4. If `=` follows, parse the default value.
5. Produce `Field { name, typ, default_value }`.

```rust
fn parse_type_field_modern() -> Result<Field> {
    let name = expect_identifier()?;
    if peek() == Colon {
        advance(); // ':'
        let typ = parse_type_ref()?;
        let default_value = if peek() == Assign {
            advance();
            Some(Box::new(parse_expression()?))
        } else {
            None
        };
        Ok(Field { name, typ, default_value })
    } else if peek() == As {
        // Legacy syntax
        advance();
        let typ = parse_type_ref()?;
        Ok(Field { name, typ, default_value: None })
    } else {
        Err("Expected ':' or 'AS' after field name")
    }
}
```

### TYPE Block (Extended)

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
        fields.push(parse_type_field_modern()?);
    }
    Ok(TypeDecl { name, fields })
}
```

---

## 6. Semantic Analysis

1. **Field registration** — fields are registered with their names and types. Duplicate field names emit `E1130`.
2. **Default value validation** — default values must be type-compatible with the declared field type. Mismatch emits `E2300`.
3. **Default value evaluation** — compile-time defaults are evaluated during semantic analysis. Runtime defaults are deferred to code generation.
4. **Zero initialization** — fields without defaults are zero-initialized in the generated code.
5. **Mixed syntax** — `AS` and `:` syntax may coexist without conflict.
6. **Type name resolution** — type references in `:` syntax are resolved the same as in `AS` syntax.

---

## 7. Code Generation

### Modern TYPE with Defaults

```basic
TYPE Point
    x: f64 = 0.0
    y: f64 = 0.0
END TYPE
```

Compiles to:

```rust
#[derive(Clone, Copy, Default)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
```

### Mixed Syntax

```basic
TYPE Config
    host AS STRING = "localhost"
    port: i32 = 8080
END TYPE
```

Compiles to:

```rust
#[derive(Clone, Default)]
struct Config {
    host: String,
    port: i32,
}

impl Config {
    fn new() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
        }
    }
}
```

### Instance with Defaults

```basic
DIM p AS Point
PRINT p.x, p.y
```

Compiles to:

```rust
let p = Point::new();
println!("{} {}", p.x, p.y);
```

### Instance with Override

```basic
DIM p AS Point
p.x = 5.0
```

Compiles to:

```rust
let mut p = Point::new();
p.x = 5.0;
```

---

## 8. Error Codes

| Code  | Description                                              |
|-------|----------------------------------------------------------|
| E1130 | Duplicate field name in TYPE declaration (from RFC-0027)  |
| E2300 | Type mismatch in field default value                      |

---

## 9. Acceptance Criteria

```text
✓ TYPE with : syntax parsed correctly
✓ TYPE with default values parsed correctly
✓ Mixed AS and : syntax in same TYPE block works
✓ Default value type compatibility validated
✓ Default values compiled to struct initialization
✓ Fields without defaults zero-initialized
✓ Existing TYPE syntax (RFC-0027) preserved
✓ Duplicate field name produces E1130
✓ Type mismatch in default produces E2300
✓ Instance created with defaults via ::new()
✓ Override of default values works
✓ Full test suite passes
```
