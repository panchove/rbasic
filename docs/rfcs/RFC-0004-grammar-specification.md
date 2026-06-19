# RFC-0004: Grammar Specification

Status: Accepted
Version: 0.1
Author: RBASIC Project
Created: 2026-06-17
Last Updated: 2026-06-17

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

# 12. Expression Statements

```ebnf
expression_stmt ::= expression
```
Used for function calls.
---

# 13. Expressions

```ebnf
expression ::= equality

equality   ::= comparison (("==" | "!=") comparison)*
comparison ::= term (("<" | "<=" | ">" | ">=") term)*
term       ::= factor (("+" | "-") factor)*
factor     ::= unary (("*" | "/") unary)*
unary      ::= "-" unary | primary
primary    ::= INTEGER_LITERAL | FLOAT_LITERAL | STRING_LITERAL | IDENTIFIER | function_call | "(" expression ")"
function_call ::= IDENTIFIER "(" arguments? ")"
arguments  ::= expression ("," expression)*
```
---

# 14. Operator Precedence (high → low)
1. Function call / grouping
2. Unary minus
3. `*` `/`
4. `+` `-`
5. Comparison `<` `<=` `>` `>=`
6. Equality `==` `!=`
All binary operators are left‑associative; unary minus is right‑associative.
---

# 15. Type References

```ebnf
type_ref ::= IDENTIFIER
```
Valid primitive type identifiers (checked later in semantic analysis): `bool`, `i32`, `f64`, `string`.
---

# 16. Assignment

Assignment is only allowed in variable declarations (`LET … = …`). Plain assignment (`x = 10`) is **not** in the v0.1 grammar.
---

# 17. Error Handling

Parser must emit structured `ParseError` with file, line, column, span, message, expected token/construct, and actual token.
---

# 18. Parser Recovery

Minimal recovery: stop at first syntax error.
---

# 19. Examples

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

# 20. Acceptance Criteria

Parser must:
- Accept all forms defined above.
- Reject out‑of‑scope forms (single‑line IF, inline assignment, etc.) with structured errors.
- Produce an AST containing nodes for Program, FunctionDecl, VarDecl, PrintStmt, ReturnStmt, IfStmt, WhileStmt, Expression, etc.
- Pass parser unit tests.
- `cargo test` succeeds.
- Update CHANGELOG.
