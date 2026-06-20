# RFC-0025: PUBLIC/PRIVATE Visibility

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add `PUBLIC` and `PRIVATE` visibility modifiers to RBASIC subroutine and function declarations. These keywords control whether a `SUB` or `FUNCTION` is accessible from other modules or only from within the declaring module.

---

## 2. Syntax (EBNF)

```ebnf
visibility    ::= "PUBLIC" | "PRIVATE"
sub_decl      ::= [visibility] "SUB" IDENTIFIER "(" param_list ")" NEWLINE
                  statement*
                  "END" "SUB"

function_decl ::= [visibility] "FUNCTION" IDENTIFIER "(" param_list ")" ["AS" type_ref]
                  NEWLINE
                  statement*
                  "END" "FUNCTION"
```

- `PUBLIC`, `PRIVATE`, `SUB`, `FUNCTION`, `END` are case-insensitive reserved keywords.
- `PUBLIC` is the default if no visibility modifier is specified.
- `PRIVATE` restricts visibility to the declaring module only.
- Visibility modifiers may also apply to `DIM` declarations in a future RFC.

Examples:

```basic
PUBLIC SUB init()
    PRINT "initialized"
END SUB

PRIVATE SUB helper()
    PRINT "internal only"
END SUB

PUBLIC FUNCTION add(a AS INTEGER, b AS INTEGER) AS INTEGER
    add = a + b
END FUNCTION

PRIVATE FUNCTION validate(s AS STRING) AS BOOLEAN
    validate = LEN(s) > 0
END FUNCTION
```

---

## 3. Semantics

### PUBLIC

1. A `PUBLIC` subroutine or function is accessible from any module in the program.
2. This is the default behavior when no modifier is specified.
3. `PUBLIC` declarations are visible in the global symbol table.

### PRIVATE

1. A `PRIVATE` subroutine or function is only accessible from within the declaring module.
2. Calling a `PRIVATE` function from another module emits `E1110`.
3. `PRIVATE` declarations are added to a module-scoped symbol table that is not shared across modules.

### Module System (Future)

Full module semantics (imports, module boundaries) are deferred to a future RFC. For v0.1, `PUBLIC`/`PRIVATE` applies within a single compilation unit. All `PRIVATE` declarations are visible to all code within the same file.

---

## 4. AST (node definitions)

Visibility modifies existing declaration nodes. Two approaches:

### Approach A: Wrapper Node

```text
VisibilityModifier {
    visibility: Visibility,
    decl: Box<Statement>,
}

Visibility ::= Public | Private
```

### Approach B: Extended Declaration Nodes (Preferred)

```text
SubDecl {
    name:       String,
    params:     Vec<Param>,
    body:       Vec<Statement>,
    visibility: Visibility,
}

FunctionDecl {
    name:       String,
    params:     Vec<Param>,
    ret_type:   Option<TypeRef>,
    body:       Vec<Statement>,
    visibility: Visibility,
}

Visibility ::= Public | Private
```

Approach B is preferred as it avoids wrapping existing nodes and matches the existing `SubDecl`/`FunctionDecl` structure from RFC-0005.

---

## 5. Parsing

When a `PUBLIC` or `PRIVATE` keyword is encountered, the parser reads the modifier and applies it to the following declaration:

```rust
fn parse_declaration() -> Result<Statement> {
    let visibility = match peek() {
        Public  => { advance(); Visibility::Public }
        Private => { advance(); Visibility::Private }
        _       => Visibility::Public,
    };

    match peek() {
        Sub      => parse_sub_decl(visibility),
        Function => parse_function_decl(visibility),
        _        => Err("Expected SUB or FUNCTION after visibility modifier"),
    }
}
```

---

## 6. Semantic Analysis

1. **Visibility scope** — `PRIVATE` declarations are added to the current module scope. Other modules cannot resolve `PRIVATE` names.
2. **Cross-module access** — calling a `PRIVATE` sub/function from outside the declaring module emits `E1110`.
3. **Duplicate declarations** — `PUBLIC` duplicate in the same scope emits `E1004`. `PRIVATE` duplicate in the same module emits `E1111`.
4. **Default visibility** — declarations without a modifier default to `PUBLIC`.

---

## 7. Code Generation

### PUBLIC

`PUBLIC` functions compile to public Rust functions:

```rust
pub fn init() {
    println!("initialized");
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### PRIVATE

`PRIVATE` functions compile to private (non-`pub`) Rust functions:

```rust
fn helper() {
    println!("internal only");
}

fn validate(s: &str) -> bool {
    s.len() > 0
}
```

---

## 8. Error Codes

| Code  | Description                                       |
|-------|---------------------------------------------------|
| E1110 | Access to PRIVATE declaration from another module  |
| E1111 | Duplicate PRIVATE declaration in same module       |

---

## 9. Acceptance Criteria

```text
✓ PUBLIC SUB parsed with Public visibility
✓ PRIVATE SUB parsed with Private visibility
✓ PUBLIC FUNCTION parsed with Public visibility
✓ PRIVATE FUNCTION parsed with Private visibility
✓ Default (no modifier) is Public
✓ PRIVATE declaration in same module compiles correctly
✓ Access to PRIVATE from outside module produces E1111
✓ Duplicate PUBLIC declaration produces E1004
✓ Duplicate PRIVATE declaration produces E1111
✓ PUBLIC compiles to pub fn in Rust
✓ PRIVATE compiles to fn (no pub) in Rust
✓ Full test suite passes
```
