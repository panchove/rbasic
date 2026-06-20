# RFC-0004: Grammar Specification

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-06-17
Last Updated: 2026-06-20

---

# 1. Summary

This RFC defines the grammar for RBASIC v0.1. It describes how the token stream from the lexer is organized into syntactic structures before semantic analysis. It serves as the authoritative contract for the parser implementation during MILESTONE‑002.

---

# 2. Scope

Defines program structure, statements, function/variable declarations, print/return/if/while statements, expressions, operator precedence, function calls, type references, block endings, and parse errors. Does **not** cover lexical tokenization, type checking, semantic validation, code generation, or runtime behavior.

---

# 3. Program Structure

```ebnf
program ::= item* EOF
item    ::= function_decl | statement
```
Top‑level statements are allowed.

---

# 4. Statements

```ebnf
statement ::= variable_decl
            | print_stmt
            | return_stmt
            | if_stmt
            | while_stmt
            | for_stmt
            | do_stmt
            | dim_stmt
            | on_error_stmt
            | resume_stmt
            | assign_stmt
            | expression_stmt
```

---

# 5. Variable Declarations

```ebnf
variable_decl ::= "LET" mutability? IDENTIFIER type_annotation? "=" expression
mutability     ::= "MUT"
type_annotation ::= ":" type_ref
```
Examples:
```
LET x = 10
LET y: i32 = 20
LET MUT counter: i32 = 0
```
Initializer is mandatory.

---

# 6. Function Declarations

```ebnf
function_decl ::= "FUNCTION" IDENTIFIER "(" parameters? ")" return_type? block "END" "FUNCTION"
parameters    ::= parameter ("," parameter)*
parameter     ::= IDENTIFIER ":" type_ref
return_type   ::= "RETURNS" type_ref   // optional in v0.1
```
Example:
```
FUNCTION add(a: i32, b: i32) RETURNS i32
    RETURN a + b
END FUNCTION
```

---

# 7. Blocks

```ebnf
block ::= statement*
```
Block termination depends on the enclosing construct:
- FUNCTION → END FUNCTION
- IF       → END IF (ELSE optional)
- WHILE    → END WHILE
- FOR      → END FOR

---

# 8. Print Statements

```ebnf
print_stmt ::= "PRINT" expression
```
---

# 9. Return Statements

```ebnf
return_stmt ::= "RETURN" expression?
```
---

# 10. If Statements (multi‑line only)

```ebnf
if_stmt ::= "IF" expression "THEN" block else_clause? "END" "IF"
else_clause ::= "ELSE" block
```
Single‑line `IF … THEN …` is **not** part of v0.1.
---

# 11. While Statements

```ebnf
while_stmt ::= "WHILE" expression block "END" "WHILE"
```
---

# 12. For Statements

```ebnf
for_stmt ::= "FOR" IDENTIFIER "=" expression "TO" expression ("STEP" expression)? block "END" "FOR"
```

The `STEP` clause is optional. If omitted, the default increment is 1. The loop variable is implicitly declared as `MUT` within the loop scope.

---

# 13. Do Loop Statements

```ebnf
do_while_pre  ::= "DO" "WHILE" expression block "LOOP"
do_until_pre  ::= "DO" "UNTIL" expression block "LOOP"
do_while_post ::= "DO" block "LOOP" "WHILE" expression
do_until_post ::= "DO" block "LOOP" "UNTIL" expression
```

Four variants matching classic BASIC:
- **DO WHILE ... LOOP**: pre-test, executes while condition is TRUE
- **DO UNTIL ... LOOP**: pre-test, executes while condition is FALSE (inverse)
- **DO ... LOOP WHILE**: post-test, executes at least once while condition is TRUE
- **DO ... LOOP UNTIL**: post-test, executes at least once until condition is TRUE

---

# 14. Dim Statements

```ebnf
dim_stmt ::= "DIM" IDENTIFIER "(" expression ("," expression)* ")" ("," IDENTIFIER "(" expression ("," expression)* ")")*
```

Array declarations. The expressions inside parentheses are dimension sizes. Multiple arrays may be declared in a single DIM statement separated by commas.

---

# 15. On Error / Resume Statements

```ebnf
on_error_stmt ::= "ON" "ERROR" "GOTO" IDENTIFIER
resume_stmt   ::= "RESUME" IDENTIFIER?
```

`ON ERROR GOTO label` sets the error handler to a label. `RESUME` (with or without a target label) transfers control after an error is handled.

---

# 16. Expression Statements

```ebnf
expression_stmt ::= expression
```
Used for function calls.
---

# 17. Assignment Statements

```ebnf
assign_stmt ::= IDENTIFIER "=" expression
```

Standalone assignment mutates an existing variable. The variable must have been declared with `LET MUT` (or be an implicitly mutable FOR loop variable). The type of the expression must be compatible with the variable's declared type.

Examples:

```basic
x = 42
counter = counter + 1
name = "hello"
flag = TRUE
```

---

# 18. Expressions

```ebnf
expression ::= logical_or ("AS" type_ref)?

logical_or  ::= logical_and (("OR" | "XOR") logical_and)*
logical_and ::= equality ("AND" equality)*
equality    ::= comparison (("==" | "!=") comparison)*
comparison  ::= term (("<" | "<=" | ">" | ">=") term)*
term        ::= factor (("+" | "-") factor)*
factor      ::= power (("*" | "/" | "\\" | "MOD" | "SHL" | "SHR") power)*
power       ::= unary ("^" power)*     ' right‑associative
unary       ::= "-" unary | "NOT" unary | primary
primary     ::= INTEGER_LITERAL | FLOAT_LITERAL | STRING_LITERAL | BOOL_LITERAL
              | IDENTIFIER | function_call | "(" expression ")"
function_call ::= IDENTIFIER "(" arguments? ")"
arguments   ::= expression ("," expression)*
```

Notes:
- `AS type_ref` at the expression level allows postfix casts: `expr AS I32`
- `\` is integer division (Backslash), `MOD` is modulo, `^` is power
- `SHL`/`SHR` are bitwise shift operators
- `NOT` is a unary prefix operator for logical negation
---

# 19. Operator Precedence (high → low)
1. Function call / grouping, Bool literal
2. Unary `-` `NOT`
3. `^` (right-associative)
4. `*` `/` `\` `MOD` `SHL` `SHR`
5. `+` `-`
6. Comparison `<` `<=` `>` `>=`
7. Equality `==` `!=`
8. `AND`
9. `OR` `XOR`
10. `AS` (postfix cast)
All binary operators are left‑associative except `^` which is right‑associative.
---

# 20. Type References

```ebnf
type_ref ::= IDENTIFIER
```
Valid primitive type identifiers (checked later in semantic analysis): `BOOL`, `I8`, `I16`, `I32`, `I64`, `U8`, `U16`, `U32`, `U64`, `F32`, `F64`, `STRING`, plus classic BASIC aliases (`INTEGER`, `LONG`, `DOUBLE`, `SINGLE`, `BOOLEAN`, `BYTE`, `WORD`, `LONGLONG`). All are case‑insensitive.
---

# 22. Assignment

Standalone assignment (`x = 10`) is supported as a statement (see §16). Assignment is also part of variable declarations (`LET … = …`, see §5) and FOR loop initialization (`FOR var = …`, see §12).
---

# 23. Error Handling

Parser must emit structured `ParseError` with file, line, column, span, message, expected token/construct, and actual token.
---

# 24. Parser Recovery

Minimal recovery: stop at first syntax error.
---

# 25. Examples

## Full Program
```
FUNCTION add(a: i32, b: i32) RETURNS i32
    RETURN a + b
END FUNCTION

LET result: i32 = add(10, 20)

IF result > 20 THEN
    PRINT "greater"
ELSE
    PRINT "smaller"
END IF
```

## Minimal Program
```
PRINT "Hello, RBASIC"
```
---

# 26. Acceptance Criteria

Parser must:
- Accept all forms defined above.
- Reject out‑of‑scope forms (single‑line IF, inline assignment, etc.) with structured errors.
- Produce an AST containing nodes for Program, FunctionDecl, VarDecl, PrintStmt, ReturnStmt, IfStmt, WhileStmt, Expression, etc.
- Pass parser unit tests.
- `cargo test` succeeds.
- Update CHANGELOG.
