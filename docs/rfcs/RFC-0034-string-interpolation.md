# RFC-0034: String Interpolation and Raw Strings

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add string interpolation (`f"..."`) and raw string literals (`raw"..."`) to RBASIC. Interpolation allows embedding expressions inside string literals using `{expr}` syntax. Raw strings preserve backslashes literally, simplifying file paths and regex patterns. Multi-line strings are also supported.

---

## 2. Syntax (EBNF)

```ebnf
fstring        ::= "f" "\"" fstring_body "\""
                 | "f" "'" fstring_body "'"

fstring_body   ::= (fchar | interp)*

fchar          ::= <any character except '"', "'", "{", "}">

interp         ::= "{" expression "}"

raw_string     ::= "raw" "\"" raw_body "\""
                 | "raw" "'" raw_body "'"

raw_body       ::= <any character except quote character>

multiline_string ::= "\"" multiline_body "\""
                   | "'" multiline_body "'"

multiline_body   ::= <any character, including newlines, except quote character>

string_literal   ::= "\"" <any character except '"', "\\", "{">* "\""
```

- `f`, `raw` prefixes are case-insensitive.
- `{expr}` inside `f"..."` evaluates `expr` and embeds the result as a string.
- Nested braces `{{` and `}}` produce literal `{` and `}`.
- `raw"..."` treats all characters literally (no escape sequences).
- Multi-line strings span multiple lines without escape characters.
- Escape sequences in regular strings: `\n`, `\t`, `\\`, `\"`, `\'`.

Examples:

```basic
DIM name AS STRING
name = "Alice"
DIM age AS INTEGER
age = 30

PRINT f"Hello, {name}! You are {age} years old."
' Output: Hello, Alice! You are 30 years old.
```

```basic
DIM path AS STRING
path = raw"C:\Users\admin\Documents"
PRINT path
' Output: C:\Users\admin\Documents
```

```basic
DIM msg AS STRING
msg = "Line 1\nLine 2\nLine 3"
PRINT msg
' Output:
' Line 1
' Line 2
' Line 3
```

```basic
DIM sql AS STRING
sql = "
SELECT *
FROM users
WHERE active = 1
"
PRINT sql
```

---

## 3. Semantics

1. `f"..."` strings are evaluated at runtime. Each `{expr}` is replaced with the string representation of `expr`.
2. The expression inside `{}` must be a valid expression. Nested braces are escaped with `{{` and `}}`.
3. `raw"..."` strings have no escape processing. Backslashes are literal characters.
4. Multi-line strings preserve newlines and whitespace exactly as written.
5. Regular strings support standard escape sequences: `\n` (newline), `\t` (tab), `\\` (backslash), `\"` (double quote), `\'` (single quote).
6. `f"..."` with `raw` prefix is not supported in v0.2. Combined prefixes emit `E1800`.
7. String interpolation types are converted to string via `STR()` or `TOSTR()`.

---

## 4. AST (node definitions)

### FString (Expression)

```text
FString {
    parts: Vec<FStringPart>,
}

FStringPart {
    kind:  FStringPartKind,
    value: String,        // for Literal
    expr:  Option<Box<Expression>>,  // for Interpolation
}

FStringPartKind ::= Literal | Interpolation
```

### RawString (Expression)

```text
RawString {
    value: String,
}
```

### MultiLineString (Expression)

```text
MultiLineString {
    value: String,
}
```

---

## 5. Parsing

### FString

When `f"` or `f'` is encountered:

1. Consume `f` and the opening quote.
2. Parse characters until the closing quote.
3. When `{` is encountered, parse the expression until `}`.
4. Produce `Expression::FString { parts }`.

```rust
fn parse_fstring() -> Result<Expression> {
    consume(FPrefix);
    let quote = consume_quote()?;
    let mut parts = Vec::new();
    let mut buf = String::new();
    loop {
        match peek() {
            CloseQuote(q) if q == quote => {
                advance();
                if !buf.is_empty() {
                    parts.push(FStringPart::Literal(buf));
                }
                break;
            }
            LBrace => {
                advance();
                if !buf.is_empty() {
                    parts.push(FStringPart::Literal(buf.clone()));
                    buf.clear();
                }
                let expr = parse_expression()?;
                expect(RBrace)?;
                parts.push(FStringPart::Interpolation(Box::new(expr)));
            }
            _ => {
                buf.push(current_char());
                advance();
            }
        }
    }
    Ok(Expression::FString { parts })
}
```

### RawString

When `raw"` or `raw'` is encountered:

1. Consume `raw` and the opening quote.
2. Parse all characters literally until the closing quote.
3. Produce `Expression::RawString { value }`.

```rust
fn parse_raw_string() -> Result<Expression> {
    consume(Raw);
    let quote = consume_quote()?;
    let mut value = String::new();
    loop {
        match peek() {
            CloseQuote(q) if q == quote => {
                advance();
                break;
            }
            _ => {
                value.push(current_char());
                advance();
            }
        }
    }
    Ok(Expression::RawString { value })
}
```

### MultiLineString

When a string literal contains newlines:

1. Consume the opening quote.
2. Parse all characters (including newlines) until the closing quote.
3. Produce `Expression::MultiLineString { value }`.

---

## 6. Semantic Analysis

1. **Expression type** — `{expr}` inside `f"..."` accepts any expression type. Types are converted to string via `STR()` or `TOSTR()`.
2. **Raw string type** — `raw"..."` is always of type `STRING`.
3. **Multi-line string type** — multi-line strings are always of type `STRING`.
4. **Escape validation** — invalid escape sequences in regular strings emit `E1801`.
5. **Combined prefixes** — `fraw"..."` or `rawf"..."` emit `E1800`.

---

## 7. Code Generation

### FString

```basic
PRINT f"Hello, {name}!"
```

Compiles to:

```rust
println!("Hello, {}!", name);
```

### RawString

```basic
DIM path AS STRING
path = raw"C:\Users\admin"
```

Compiles to:

```rust
let path: String = r"C:\Users\admin".to_string();
```

### MultiLineString

```basic
DIM sql AS STRING
sql = "SELECT * FROM users"
```

Compiles to:

```rust
let sql: String = "SELECT * FROM users".to_string();
```

---

## 8. Error Codes

| Code  | Description                                           |
|-------|-------------------------------------------------------|
| E1800 | Combined f/raw prefixes not supported                 |
| E1801 | Invalid escape sequence in string literal              |

---

## 9. Acceptance Criteria

```text
✓ f"..." parsed as FString with literal and interpolation parts
✓ {expr} inside f-string evaluates and converts to string
✓ {{ and }} produce literal { and }
✓ raw"..." parsed as RawString with literal backslashes
✓ Multi-line strings preserve newlines
✓ Regular string escape sequences work (\n, \t, \\, \")
✓ fraw"..." produces E1800
✓ Invalid escape sequence produces E1801
✓ FString compiles to format string with {}
✓ RawString compiles to r"..." literal
✓ Full test suite passes
```
