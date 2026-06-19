# RFC-0002: Lexical Specification

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-06-17
Last Updated: 2026-06-19

---

# 1. Summary

This RFC defines the lexical structure of RBASIC v0.1.

The lexical specification describes how source code characters are transformed into tokens before parsing.

This document serves as the authoritative contract for lexer implementation during RBASIC v0.1.

---

# 2. Scope

This RFC defines:

* Source file encoding
* Whitespace rules
* Line terminators
* Comments
* Keywords
* Identifiers
* Literals
* Operators
* Delimiters
* Reserved words
* Lexical errors
* Tokenization behavior

This RFC does not define:

* Grammar (covered by RFC-0004)
* AST structure (covered by RFC-0005)
* Semantic analysis (covered by RFC-0006)
* Type checking (covered by RFC-0008)
* Code generation

Those topics are covered by separate RFCs.

---

# 3. Source File Encoding

RBASIC source files SHALL use:

```text
UTF-8
```

without byte order mark (BOM).

File extension:

```text
.rbas
```

Example:

```text
main.rbas
hello.rbas
```

Identifiers are limited to ASCII characters for v0.1; Unicode characters may appear only inside string literals.

---

# 4. Whitespace

The following characters are considered whitespace:

```text
Space (U+0020)
Tab (U+0009)
Carriage Return (U+000D)
Line Feed (U+000A)
```

Whitespace is ignored except when separating tokens.

Example:

```basic
LET x = 10
```

is equivalent to:

```basic
LET     x      =      10
```

---

# 5. Line Terminators

The lexer SHALL recognize:

```text
LF   (\n)
CRLF (\r\n)
```

as valid line endings.

All internal diagnostics shall normalize line tracking.

---

# 6. Comments

RBASIC v0.1 supports single-line comments only.

Comment marker:

```basic
'
```

Everything after the apostrophe until the end of the line is ignored.

Example:

```basic
LET x = 10

' This is a comment

PRINT x
```

Block comments are not part of v0.1.

---

# 7. Case Insensitivity

Keywords are case‑insensitive, while identifiers preserve the case used in the source. The language is case‑insensitive overall but source code retains the original casing (case‑preserving).

```basic
LET Name = "Pancho"
PRINT name   ' refers to the same identifier
```

Consequently `Name` and `name` refer to the same binding.

---

# 8. Identifiers

Identifiers represent user-defined names.

Allowed characters:

```text
A-Z
a-z
0-9
_
```

Rules:

* Must start with a letter or underscore.
* Cannot start with a digit.
* Cannot match reserved keywords.

Valid:

```text
counter
_total
user_name
value123
```

Invalid:

```text
123value
LET
FUNCTION
```

**Reserved primitive type identifiers** (treated as identifiers but reserved for future use):

```text
BOOL
I32
F64
STRING
```

They are tokenised as `Identifier` tokens now, but their spelling is reserved to avoid later lexical changes.

---

# 9. Integer Literals

RBASIC v0.1 supports decimal integers.

Examples:

```basic
0
1
42
1000
```

Negative values are represented using unary minus:

```basic
-42
```

The minus sign is not part of the literal token.

---

# 10. Floating Point Literals

Examples:

```basic
3.14
0.5
10.0
100.25
```

Invalid:

```basic
.
10.
```

Scientific notation is not part of v0.1.

---

# 11. String Literals

Strings are enclosed by double quotes.

Example:

```basic
"Hello"
"RBASIC"
"Hello World"
```

Supported escapes:

```text
\\
\"
\n
\r
\t
```

Example:

```basic
"Line 1\nLine 2"
```

Undetermined strings produce a lexical error.

---

# 12. Operators

Arithmetic operators:

```text
+
-
*
/
```

Assignment operator:

```text
=
```

Comparison operators:

```text
==
!=
<
<=
>
>=
```

---

# 13. Delimiters

The following delimiters are recognized:

```text
(
)
:
,
```

Examples:

```basic
FUNCTION add(a: i32, b: i32)
```

---

# 14. Reserved Words

The following words are reserved for future language evolution and SHALL NOT be used as identifiers:

```text
NEXT
MODULE
IMPORT
EXPORT
TYPE
ENUM
STRUCT
OPTIONAL
RESULT
REF
MUTREF
UNSAFE
```

These words do not produce valid language constructs in v0.1.

---

# 15. Keywords

The following keywords are recognized:

```text
LET
MUT
FUNCTION
RETURNS
RETURN
IF
THEN
ELSE
END
WHILE
PRINT
NOT
FOR
TO
```

Keywords are case-insensitive.

The following forms are equivalent:

```basic
LET
let
Let
lEt
```

The lexer SHALL normalize keyword tokens.

---

# 16. Tokenization Rules

The lexer SHALL apply longest-match behavior.

Example:

```basic
>=
```

must produce:

```text
GreaterEqual
```

and not:

```text
Greater
Assign
```

Similarly:

```basic
==
```

must be emitted as a single token.

---

# 17. Lexical Errors

The lexer SHALL report diagnostics for:

## Invalid Character

Example:

```basic
LET x = §
```

## Unterminated String

Example:

```basic
PRINT "Hello
```

## Invalid Number

Example:

```basic
12.34.56
```

Diagnostics should include:

```text
error code
file
line
column
source snippet
message
```

---

# 18. Token Inventory

Minimum token set for RBASIC v0.1:

```text
Identifier
IntegerLiteral
FloatLiteral
StringLiteral

Let
Mut
Function
Returns
Return
If
Then
Else
End
While
Print
Not
For
To

Plus
Minus
Star
Slash

Assign

EqualEqual
BangEqual

Less
LessEqual

Greater
GreaterEqual

LeftParen
RightParen

Colon
Comma

EOF
```

The lexer shall always emit exactly one `EOF` token at the end of every source file.

---

# 19. Examples

Example:

```basic
FUNCTION add(a: i32, b: i32) RETURNS i32
    RETURN a + b
END FUNCTION
```

Expected token sequence:

```text
Function
Identifier(add)
LeftParen
Identifier(a)
Colon
Identifier(i32)
Comma
Identifier(b)
Colon
Identifier(i32)
RightParen
Returns
Identifier(i32)
Return
Identifier(a)
Plus
Identifier(b)
End
Function
EOF
```

---

# 20. Acceptance Criteria

This RFC shall be considered implemented when:

```text
✓ Lexer implemented
✓ Token inventory complete
✓ Keyword recognition complete
✓ Comment handling complete
✓ UTF-8 support verified
✓ Lexical diagnostics implemented
✓ Unit tests passing
✓ MILESTONE-001 completed
```
