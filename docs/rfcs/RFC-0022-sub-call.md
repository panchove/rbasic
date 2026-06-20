# RFC-0022: SUB/CALL Statements

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add named subroutine declarations (`SUB ... END SUB`) and explicit call statements (`CALL name(args)`) to RBASIC. Subroutines encapsulate reusable blocks of code that accept parameters but return no value. This distinguishes them from functions (RFC-0005 §4.28) which return a value.

---

## 2. Syntax (EBNF)

```ebnf
sub_decl   ::= "SUB" IDENTIFIER "(" param_list ")" NEWLINE
               statement*
               "END" "SUB"

call_stmt  ::= "CALL" IDENTIFIER "(" [arg_list] ")"
            |  IDENTIFIER "(" [arg_list] ")"

param_list ::= [param ("," param)*]
param      ::= IDENTIFIER [":" type_ref] ["AS" type_ref]

arg_list   ::= expression ("," expression)*
```

- `SUB`, `END SUB`, `CALL`, `AS` are case-insensitive reserved keywords.
- A `SUB` has no return type; it is invoked with `CALL` or as a statement-level expression.
- Parameters default to `ByRef` passing unless `ByVal` is specified (see RFC-0026).
- `END SUB` must match the opening `SUB`.

Examples:

```basic
SUB greet(name AS STRING)
    PRINT "Hello, "; name
END SUB

CALL greet("World")
greet("Alice")
```

```basic
SUB swap(ByRef a AS INTEGER, ByRef b AS INTEGER)
    DIM temp AS INTEGER
    temp = a
    a = b
    b = temp
END SUB

DIM x AS INTEGER
DIM y AS INTEGER
x = 10
y = 20
CALL swap(x, y)
PRINT x; y
```

---

## 3. Semantics

1. A `SUB` declaration registers a subroutine with the given name and parameter list.
2. Subroutines have no return value; attempting to use a `SUB` call as an expression emits `E1080`.
3. `CALL name(args)` invokes the subroutine, evaluating each argument and binding to the corresponding parameter.
4. `name(args)` without `CALL` is syntactic sugar for the same invocation (statement context only).
5. Argument count must match parameter count (unless parameters are `OPTIONAL`). Mismatch emits `E1030`.
6. Argument types must be compatible with parameter types. Incompatible types emit `E1020`.
7. Subroutines may not be redeclared in the same scope. Redeclaration emits `E1081`.
8. Subroutines may call other subroutines (including recursive calls, subject to stack limits).

---

## 4. AST (node definitions)

```text
SubDecl {
    name:   String,
    params: Vec<Param>,
    body:   Vec<Statement>,
}

Call {
    name: String,
    args: Vec<Expression>,
}

Param {
    name:    String,
    typ:     TypeRef,
    passing: ParamPassing,
}

ParamPassing ::= ByVal | ByRef | Optional
```

These nodes are already defined in RFC-0005 §4.23, §4.24.

---

## 5. Parsing

### SUB Declaration

When `SUB` keyword is encountered at statement level:

1. Consume `SUB`.
2. Parse identifier (subroutine name).
3. Consume `(`, parse parameter list, consume `)`.
4. Parse statement body until `END SUB`.
5. Consume `END SUB`.

```rust
fn parse_sub_decl() -> Result<SubDecl> {
    consume(Sub);
    let name = expect_identifier()?;
    consume(OpenParen);
    let params = parse_param_list()?;
    consume(CloseParen);
    let body = parse_statements_until(EndSub);
    consume(EndSub);
    Ok(SubDecl { name, params, body })
}
```

### CALL Statement

When `CALL` keyword is encountered:

1. Consume `CALL`.
2. Parse identifier (subroutine name).
3. Consume `(`, parse argument list, consume `)`.
4. Produce `Statement::Call { name, args }`.

Alternatively, `IDENTIFIER "(" args ")"` at statement level is parsed as a `Call`.

---

## 6. Semantic Analysis

1. **Subroutine must exist** — calling an undeclared subroutine emits `E1003`.
2. **No duplicate subroutines** — redeclaring a subroutine in the same scope emits `E1081`.
3. **Argument count** — must match parameter count (E1030).
4. **Argument types** — must be compatible with parameter types (E1020).
5. **No return value** — using a `CALL` expression in an expression context emits `E1080`.
6. **Parameter types** — parameter type annotations must be valid types (E1010).
7. **ByRef mutability** — `ByRef` parameters reference mutable bindings; the caller must pass an l-value.

---

## 7. Code Generation

`SUB` declarations compile to Rust functions with no return value:

```basic
SUB greet(name AS STRING)
    PRINT "Hello, "; name
END SUB
```

Compiles to:

```rust
fn greet(name: &str) {
    println!("Hello, {}", name);
}
```

`CALL` invokes the function:

```basic
CALL greet("World")
```

Compiles to:

```rust
greet("World");
```

Parameters default to `ByRef`, so they compile as references:

```rust
fn swap(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}
```

---

## 8. Error Codes

| Code  | Description                                        |
|-------|----------------------------------------------------|
| E1080 | SUB has no return value (used in expression)       |
| E1081 | Duplicate SUB declaration in scope                 |
| E1003 | Unknown subroutine (call to undeclared SUB)        |
| E1030 | Argument count mismatch (reused from RFC-0008)     |
| E1020 | Type mismatch in argument (reused from RFC-0007)   |

---

## 9. Acceptance Criteria

```text
✓ SUB ... END SUB parsed as SubDecl
✓ CALL name(args) parsed as Call
✓ name(args) without CALL parsed as Call (statement context)
✓ SUB must not return a value (E1080 if used as expression)
✓ Duplicate SUB in scope produces E1081
✓ Unknown SUB call produces E1003
✓ Argument count mismatch produces E1030
✓ Type mismatch in arguments produces E1020
✓ ByRef parameters compiled as references
✓ Full test suite passes
```
