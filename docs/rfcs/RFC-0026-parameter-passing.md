# RFC-0026: Parameter Passing (BYVAL/BYREF/OPTIONAL)

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add explicit parameter passing modes (`BYVAL`, `BYREF`, `OPTIONAL`) to RBASIC subroutines and functions. `BYVAL` passes a copy of the argument, `BYREF` passes a reference to the caller's variable, and `OPTIONAL` allows a parameter to be omitted with a default value.

---

## 2. Syntax (EBNF)

```ebnf
param_list ::= [param ("," param)*]
param      ::= ["BYVAL" | "BYREF" | "OPTIONAL"] IDENTIFIER ["AS" type_ref] ["=" expression]

arg_list   ::= [expression ("," expression)*]
```

- `BYVAL`, `BYREF`, `OPTIONAL`, `AS` are case-insensitive reserved keywords.
- If no passing mode is specified, the default is `BYREF` (matching classic BASIC convention).
- `OPTIONAL` parameters must come after all required parameters.
- `OPTIONAL` parameters may have a default value (`= expression`). If no default is given, the parameter defaults to the type's zero value.
- Only the last parameters in the list may be `OPTIONAL`.

Examples:

```basic
SUB print_value(ByVal x AS INTEGER)
    PRINT x
END SUB

SUB swap(ByRef a AS INTEGER, ByRef b AS INTEGER)
    DIM temp AS INTEGER
    temp = a
    a = b
    b = temp
END SUB

FUNCTION power(base AS INTEGER, Optional exponent AS INTEGER = 2) AS INTEGER
    DIM result AS INTEGER
    result = 1
    FOR i = 1 TO exponent
        result = result * base
    NEXT i
    power = result
END FUNCTION
```

---

## 3. Semantics

### BYVAL

1. `ByVal` passes the argument by value. The function receives a copy.
2. Modifications to the parameter inside the function do not affect the caller's variable.
3. The argument expression is evaluated and the result is assigned to the parameter.

### BYREF

1. `ByRef` passes the argument by reference. The function receives a reference to the caller's variable.
2. Modifications to the parameter inside the function affect the caller's variable.
3. The argument must be an l-value (a variable, not a literal or expression).
4. Passing a non-l-value to a `ByRef` parameter emits `E1120`.

### OPTIONAL

1. `Optional` parameters may be omitted from the call.
2. If omitted, the parameter receives its default value (specified by `= expression`) or the type's zero value.
3. `Optional` parameters must follow all required parameters. A non-optional parameter after an optional parameter emits `E1121`.
4. Default values are evaluated once at the point of declaration (not at each call).

### Default (no modifier)

If no modifier is specified, the parameter defaults to `ByRef` behavior. This matches classic BASIC convention and preserves compatibility with existing code.

---

## 4. AST (node definitions)

```text
Param {
    name:    String,
    typ:     TypeRef,
    passing: ParamPassing,
    default: Option<Expression>,
}

ParamPassing ::= ByVal | ByRef | Optional
```

This extends the existing `Param` node from RFC-0005 §4.23 by adding the optional `default` field.

```rust
pub enum ParamPassing {
    ByVal,
    ByRef,
    Optional,
}
```

---

## 5. Parsing

When parsing a parameter list:

1. Check for `BYVAL`, `BYREF`, or `OPTIONAL` keyword at the start of each parameter.
2. Parse the parameter name.
3. Optionally parse `AS type_ref`.
4. Optionally parse `= expression` for the default value.
5. Produce the `Param` node with the appropriate passing mode and default.

```rust
fn parse_param() -> Result<Param> {
    let passing = match peek() {
        ByVal    => { advance(); ParamPassing::ByVal }
        ByRef    => { advance(); ParamPassing::ByRef }
        Optional => { advance(); ParamPassing::Optional }
        _        => ParamPassing::ByRef,
    };
    let name = expect_identifier()?;
    let typ = if peek() == As {
        advance();
        parse_type_ref()?
    } else {
        TypeRef { name: "VARIANT".to_string() }
    };
    let default = if peek() == Assign {
        advance();
        Some(expression()?)
    } else {
        None
    };
    Ok(Param { name, typ, passing, default })
}
```

---

## 6. Semantic Analysis

1. **ByRef requires l-value** — if a `ByRef` parameter receives a non-l-value (literal, expression), emit `E1120`.
2. **Optional parameter ordering** — `Optional` parameters must be after all required parameters. Non-optional after optional emits `E1121`.
3. **Default type compatibility** — the default value's type must be compatible with the parameter's declared type. Incompatible types emit `E1020`.
4. **Argument count validation** — arguments must match required parameters. If fewer arguments than required parameters, emit `E1030`. If more arguments than total parameters, emit `E1030`.
5. **Optional argument filling** — missing optional arguments are filled with their default values during call resolution.

---

## 7. Code Generation

### BYVAL

```basic
SUB print_value(ByVal x AS INTEGER)
    PRINT x
END SUB
```

Compiles to:

```rust
fn print_value(x: i32) {
    println!("{}", x);
}
```

### BYREF

```basic
SUB swap(ByRef a AS INTEGER, ByRef b AS INTEGER)
    DIM temp AS INTEGER
    temp = a
    a = b
    b = temp
END SUB
```

Compiles to:

```rust
fn swap(a: &mut i32, b: &mut i32) {
    let mut temp: i32 = 0;
    temp = *a;
    *a = *b;
    *b = temp;
}
```

### OPTIONAL with default

```basic
FUNCTION power(base AS INTEGER, Optional exponent AS INTEGER = 2) AS INTEGER
    ' ...
END FUNCTION
```

Compiles to:

```rust
fn power(base: i32, exponent: Option<i32>) -> i32 {
    let exponent = exponent.unwrap_or(2);
    // ...
}
```

Or alternatively:

```rust
fn power(base: i32, exponent: i32) -> i32 {
    // default applied at call site
}

// Call with default:
power(5, 2)
```

---

## 8. Error Codes

| Code  | Description                                              |
|-------|----------------------------------------------------------|
| E1120 | ByRef argument must be an l-value                        |
| E1121 | Optional parameter must be last in parameter list        |
| E1030 | Argument count mismatch (reused from RFC-0008)           |
| E1020 | Type mismatch in default value (reused from RFC-0007)    |

---

## 9. Acceptance Criteria

```text
✓ ByVal parameter parsed with ByVal passing mode
✓ ByRef parameter parsed with ByRef passing mode
✓ Optional parameter parsed with Optional passing mode
✓ Default parameter (no modifier) is ByRef
✓ Optional parameter with default value parsed correctly
✓ ByRef with non-l-value argument produces E1120
✓ Optional after non-optional produces E1121
✓ Default type compatibility validated
✓ Argument count validated (required + optional)
✓ ByVal compiles to value parameter in Rust
✓ ByRef compiles to &mut reference in Rust
✓ Optional compiles to Option<T> or default at call site
✓ Full test suite passes
```
