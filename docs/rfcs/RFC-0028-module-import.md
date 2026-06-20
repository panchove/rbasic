# RFC-0028: MODULE/IMPORT (Module System)

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add a module system to RBASIC via `MODULE...END MODULE` and `IMPORT`. Modules encapsulate declarations (types, functions, subroutines, variables) and control visibility across compilation units. Each file implicitly defines a module matching its filename; explicit `MODULE name` overrides this.

---

## 2. Syntax (EBNF)

```ebnf
module_decl    ::= "MODULE" IDENTIFIER NEWLINE
                   module_body+
                   "END" "MODULE"

module_body    ::= type_decl
                 | function_decl
                 | sub_decl
                 | dim_stmt
                 | visibility_stmt
                 | module_decl

visibility_stmt ::= "PUBLIC" | "PRIVATE"

import_stmt    ::= "IMPORT" IDENTIFIER
                 | "IMPORT" IDENTIFIER "." "*"
                 | "IMPORT" IDENTIFIER "." IDENTIFIER

qualified_ref  ::= IDENTIFIER "." IDENTIFIER
```

- `MODULE`, `END MODULE`, `IMPORT`, `PUBLIC`, `PRIVATE` are case-insensitive reserved keywords.
- A module name is an identifier; modules may be nested.
- `IMPORT ModuleName` imports all public declarations from the named module.
- `IMPORT ModuleName.*` imports all public declarations explicitly.
- `IMPORT ModuleName.Item` imports a single named public declaration.
- Duplicate imports of the same item emit `E1200`.

Examples:

```basic
MODULE Math
    PUBLIC FUNCTION add(a AS DOUBLE, b AS DOUBLE) AS DOUBLE
        add = a + b
    END FUNCTION

    PRIVATE FUNCTION validate(x AS DOUBLE) AS BOOLEAN
        validate = x <> 0
    END FUNCTION
END MODULE

IMPORT Math

PRINT Math.add(2, 3)
```

```basic
MODULE Geometry
    MODULE Shapes
        PUBLIC FUNCTION area_circle(r AS DOUBLE) AS DOUBLE
            area_circle = 3.14159 * r * r
        END FUNCTION
    END MODULE
END MODULE

IMPORT Geometry.Shapes
PRINT Shapes.area_circle(5)
```

---

## 3. Semantics

1. `MODULE name ... END MODULE` creates a named scope. All declarations within are scoped to the module.
2. Declarations without an explicit visibility modifier default to `PUBLIC` within the module.
3. `PRIVATE` declarations are not accessible from outside the module.
4. `IMPORT ModuleName` makes all `PUBLIC` declarations of the module available in the importing scope.
5. `IMPORT ModuleName.Item` makes a single declaration available.
6. Modules may be nested; nested module names are qualified by their parent (e.g., `Geometry.Shapes`).
7. Circular imports emit `E1201`.
8. Importing a non-existent module emits `E1202`. Importing a non-existent item emits `E1203`.
9. Accessing a `PRIVATE` declaration from outside its module emits `E1204`.

---

## 4. AST (node definitions)

### ModuleDecl

```text
ModuleDecl {
    name:  String,
    body:  Vec<Statement>,
}
```

### Import

```text
Import {
    module: String,
    item:   Option<String>,
}
```

### QualifiedName (Expression)

```text
QualifiedName {
    module: String,
    name:   String,
}
```

---

## 5. Parsing

### Module Declaration

When `MODULE` is encountered at statement level:

1. Consume `MODULE`.
2. Parse the module name (identifier).
3. Parse statements until `END MODULE`.
4. Consume `END MODULE`.
5. Produce `Statement::ModuleDecl { name, body }`.

```rust
fn parse_module_decl() -> Result<ModuleDecl> {
    consume(Module);
    let name = expect_identifier()?;
    let mut body = Vec::new();
    loop {
        if peek() == End && peek_ahead() == Module {
            advance(); // END
            advance(); // MODULE
            break;
        }
        body.push(parse_statement()?);
    }
    Ok(ModuleDecl { name, body })
}
```

### Import Statement

When `IMPORT` is encountered:

1. Consume `IMPORT`.
2. Parse a dotted name (`ident.ident*`).
3. If the last segment is `*`, import all public items.
4. Produce `Statement::Import { module, item }`.

```rust
fn parse_import() -> Result<Import> {
    consume(Import);
    let module = expect_identifier()?;
    let item = if peek() == Dot {
        advance();
        if peek() == Star {
            advance();
            None
        } else {
            Some(expect_identifier()?)
        }
    } else {
        None
    };
    Ok(Import { module, item })
}
```

---

## 6. Semantic Analysis

1. **Module registration** — `MODULE name` registers the module in the module table. Duplicate module names in the same scope emit `E1205`.
2. **Visibility enforcement** — accessing `PRIVATE` items from outside the module emits `E1204`.
3. **Import resolution** — `IMPORT ModuleName` resolves the module and copies public names into the importing scope. Unknown modules emit `E1202`.
4. **Item resolution** — `IMPORT ModuleName.Item` validates the item exists and is public. Unknown items emit `E1203`.
5. **Circular dependency detection** — circular import chains emit `E1201`.
6. **Qualified access** — `Module.Item` validates that `Module` is imported and `Item` is public.

---

## 7. Code Generation

### Module Declaration

Modules are flattened during code generation. Public items compile to `pub` items; private items compile to non-pub items.

```basic
MODULE Math
    PUBLIC FUNCTION add(a AS DOUBLE, b AS DOUBLE) AS DOUBLE
        add = a + b
    END FUNCTION
END MODULE
```

Compiles to:

```rust
pub mod math {
    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }
}
```

### Import

```basic
IMPORT Math
```

Compiles to a `use` statement:

```rust
use math::*;
```

### Qualified Access

```basic
PRINT Math.add(2, 3)
```

Compiles to:

```rust
println!("{}", math::add(2.0, 3.0));
```

---

## 8. Error Codes

| Code  | Description                                          |
|-------|------------------------------------------------------|
| E1200 | Duplicate import of the same item                    |
| E1201 | Circular module dependency                           |
| E1202 | Unknown module in IMPORT                             |
| E1203 | Unknown item in IMPORT ModuleName.Item               |
| E1204 | Access to PRIVATE declaration from outside module     |
| E1205 | Duplicate MODULE declaration in same scope            |

---

## 9. Acceptance Criteria

```text
✓ MODULE ... END MODULE parsed as ModuleDecl
✓ IMPORT ModuleName parsed as Import
✓ IMPORT ModuleName.Item parsed as Import with item
✓ IMPORT ModuleName.* parsed as Import with wildcard
✓ PUBLIC visibility within module enforced
✓ PRIVATE visibility across modules enforced (E1204)
✓ Unknown module import produces E1202
✓ Unknown item import produces E1203
✓ Circular import produces E1201
✓ Duplicate import produces E1200
✓ Duplicate module name produces E1205
✓ Module compiles to Rust mod with correct visibility
✓ Import compiles to use statement
✓ Nested modules compile correctly
✓ Full test suite passes
```
