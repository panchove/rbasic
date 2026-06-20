# RFC-0004: Grammar Specification

Status: Accepted
Version: 0.2
Author: RBASIC Project
Created: 2026-06-17
Last Updated: 2026-06-20

---

# 1. Summary

This RFC defines the grammar for RBASIC v0.2. It describes how the token stream from the lexer is organized into syntactic structures before semantic analysis. QuickBASIC is the source of truth for Phase 1 syntax.

---

# 2. Scope

Defines program structure, statements, function/subroutine declarations, control flow, data structures, expressions, operator precedence, type references, and parse errors. Does **not** cover lexical tokenization, type checking, semantic validation, code generation, or runtime behavior.

---

# 3. Program Structure

```ebnf
program ::= item* EOF
item    ::= function_decl | sub_decl | statement
```

Top‑level statements are allowed.

---

# 4. Statements

```ebnf
statement ::= variable_decl
            | print_stmt
            | input_stmt
            | return_stmt
            | if_stmt
            | while_stmt
            | for_stmt
            | do_stmt
            | select_case_stmt
            | dim_stmt
            | on_error_stmt
            | resume_stmt
            | goto_stmt
            | gosub_stmt
            | on_goto_stmt
            | on_gosub_stmt
            | do_events_stmt
            | option_explicit_stmt
            | option_base_stmt
            | exit_stmt
            | call_stmt
            | assign_stmt
            | compound_assign_stmt
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

# 6. Function Declarations (QuickBASIC)

```ebnf
function_decl ::= visibility? "FUNCTION" IDENTIFIER "(" parameters? ")" return_type? block "END" "FUNCTION"
visibility    ::= "PUBLIC" | "PRIVATE"
parameters    ::= parameter ("," parameter)*
parameter     ::= param_mod? IDENTIFIER ("AS" | ":") type_ref ("=" expression)?
param_mod     ::= "BYVAL" | "BYREF" | "OPTIONAL"
return_type   ::= "RETURNS" type_ref | "AS" type_ref
```

QuickBASIC functions return values by assigning to the function name. There is no `RETURN` statement inside functions.

Examples:
```basic
FUNCTION add(a AS INTEGER, b AS INTEGER) AS INTEGER
    add = a + b
END FUNCTION

PUBLIC FUNCTION DoubleIt(BYVAL x AS INTEGER) AS INTEGER
    DoubleIt = x * 2
END FUNCTION

FUNCTION Power(base AS INTEGER, OPTIONAL exp AS INTEGER = 2) AS INTEGER
    ' ...
END FUNCTION
```

---

# 7. Subroutine Declarations

```ebnf
sub_decl      ::= visibility? "SUB" IDENTIFIER "(" parameters? ")" block "END" "SUB"
```

Examples:
```basic
SUB Greet(name AS STRING)
    PRINT "Hello, " + name
END SUB

SUB Swap(BYREF a AS INTEGER, BYREF b AS INTEGER)
    DIM temp AS INTEGER
    temp = a
    a = b
    b = temp
END SUB

CALL Greet("World")
```

---

# 8. Blocks

```ebnf
block ::= statement*
```

Block termination depends on the enclosing construct:
- FUNCTION → END FUNCTION
- SUB → END SUB
- IF → END IF (ELSEIF/ELSE optional)
- WHILE → WEND
- FOR → NEXT [var]
- DO → LOOP
- SELECT CASE → END SELECT

---

# 9. Print Statement

```ebnf
print_stmt ::= "PRINT" (expression (","|";" expression)*)?
```

---

# 10. Input Statement

```ebnf
input_stmt ::= "INPUT" (STRING_LITERAL ","? )? IDENTIFIER
```

---

# 11. Return Statement

```ebnf
return_stmt ::= "RETURN" expression?
```

Used to return from GOSUB. Inside functions, use assignment to function name (QuickBASIC style).

---

# 12. If Statements

```ebnf
if_stmt     ::= "IF" expression "THEN" block elseif_clause? else_clause? "END" "IF"
elseif_clause ::= "ELSEIF" expression "THEN" block elseif_clause?
else_clause ::= "ELSE" block
```

Example:
```basic
IF score >= 90 THEN
    grade = "A"
ELSEIF score >= 80 THEN
    grade = "B"
ELSE
    grade = "F"
END IF
```

---

# 13. While Statement

```ebnf
while_stmt ::= "WHILE" expression block "WEND"
```

---

# 14. For Statement

```ebnf
for_stmt ::= "FOR" IDENTIFIER "=" expression "TO" expression ("STEP" expression)? block "NEXT" IDENTIFIER?
```

The STEP clause is optional (default 1). The loop variable is implicitly declared and mutable within the loop scope.

Examples:
```basic
FOR i = 1 TO 10
    PRINT i
NEXT i

FOR i = 0 TO 20 STEP 2
    PRINT i
NEXT
```

---

# 15. Do Loop Statements

```ebnf
do_while_pre  ::= "DO" "WHILE" expression block "LOOP"
do_until_pre  ::= "DO" "UNTIL" expression block "LOOP"
do_while_post ::= "DO" block "LOOP" "WHILE" expression
do_until_post ::= "DO" block "LOOP" "UNTIL" expression
```

---

# 16. Select Case Statement

```ebnf
select_case_stmt ::= "SELECT" "CASE" expression case_clause+ else_clause? "END" "SELECT"
case_clause      ::= "CASE" case_value ("," case_value)* block
case_value       ::= expression ("TO" expression)?
```

When `TO` is present, the case matches a range. A single expression matches that exact value.

Example:
```basic
SELECT CASE variable
    CASE 1
        PRINT "one"
    CASE 2 TO 5
        PRINT "two to five"
    CASE ELSE
        PRINT "other"
END SELECT
```

---

# 17. Dim Statements

```ebnf
dim_stmt  ::= "DIM" dim_decl ("," dim_decl)*
dim_decl  ::= IDENTIFIER "(" dim_bounds ")" ("AS" type_ref)?
dim_bounds ::= dim_bound ("," dim_bound)*
dim_bound ::= expression ("TO" expression)?
```

When `TO` is omitted, the lower bound is determined by `OPTION BASE` (default 0).

Examples:
```basic
DIM arr(10)
DIM a(0 TO 10)
DIM matrix(1 TO 3, 1 TO 4)
DIM arr(10) AS INTEGER
```

---

# 18. On Error / Resume Statements

```ebnf
on_error_stmt ::= "ON" "ERROR" "GOTO" IDENTIFIER
resume_stmt   ::= "RESUME" IDENTIFIER?
```

---

# 19. GOTO Statement

```ebnf
goto_stmt ::= "GOTO" IDENTIFIER
```

---

# 20. GOSUB Statement

```ebnf
gosub_stmt ::= "GOSUB" IDENTIFIER
```

---

# 21. ON...GOTO / ON...GOSUB

```ebnf
on_goto_stmt  ::= "ON" expression "GOTO" IDENTIFIER ("," IDENTIFIER)*
on_gosub_stmt ::= "ON" expression "GOSUB" IDENTIFIER ("," IDENTIFIER)*
```

---

# 22. DO EVENTS

```ebnf
do_events_stmt ::= "DO" "EVENTS"
```

---

# 23. OPTION Statements

```ebnf
option_explicit_stmt ::= "OPTION" "EXPLICIT"
option_base_stmt     ::= "OPTION" "BASE" INTEGER_LITERAL
```

---

# 24. EXIT Statement

```ebnf
exit_stmt ::= "EXIT" ("FOR" | "WHILE" | "DO")
```

---

# 25. CALL Statement

```ebnf
call_stmt ::= "CALL" IDENTIFIER "(" arguments? ")"
```

---

# 26. Expression Statements

```ebnf
expression_stmt ::= expression
```

Used for function calls.

---

# 27. Assignment Statements

```ebnf
assign_stmt ::= IDENTIFIER "=" expression
```

Standalone assignment mutates an existing variable. The variable must have been declared and be mutable.

---

# 28. Compound Assignment

```ebnf
compound_assign_stmt ::= IDENTIFIER compound_assign_op expression
compound_assign_op   ::= "+=" | "-=" | "*=" | "/=" | "\=" | "^=" | "MOD="
```

---

# 29. Expressions

```ebnf
expression ::= logical_or ("AS" type_ref)?

logical_or  ::= logical_and (("OR" | "XOR") logical_and)*
logical_and ::= equality ("AND" equality)*
equality    ::= comparison (("=" | "<>") comparison)*
comparison  ::= term (("<" | "<=" | ">" | ">=") term)*
term        ::= factor (("+" | "-") factor)*
factor      ::= power (("*" | "/" | "\\" | "MOD" | "SHL" | "SHR") power)*
power       ::= unary ("^" power)?     ' right‑associative
unary       ::= "-" unary | "NOT" unary | primary
primary     ::= INTEGER_LITERAL | FLOAT_LITERAL | STRING_LITERAL | BOOL_LITERAL
              | IDENTIFIER | function_call | "(" expression ")"
function_call ::= IDENTIFIER "(" arguments? ")"
arguments   ::= expression ("," expression)*
```

---

# 30. Operator Precedence (high → low)

1. Function call / grouping, Bool literal
2. Unary `-` `NOT`
3. `^` (right-associative)
4. `*` `/` `\` `MOD` `SHL` `SHR`
5. `+` `-`
6. Comparison `<` `<=` `>` `>=`
7. Equality `=` `<>`
8. `AND`
9. `OR` `XOR`
10. `AS` (postfix cast)

All binary operators are left‑associative except `^` which is right‑associative.

---

# 31. Type References

```ebnf
type_ref ::= IDENTIFIER
```

Valid primitive type identifiers (checked later in semantic analysis): `BOOL`, `I8`, `I16`, `I32`, `I64`, `U8`, `U16`, `U32`, `U64`, `F32`, `F64`, `STRING`, plus classic BASIC aliases (`INTEGER`, `LONG`, `DOUBLE`, `SINGLE`, `BOOLEAN`, `BYTE`, `WORD`, `LONGLONG`, `DWORD`, `QWORD`). All are case‑insensitive.

---

# 32. Labels

```ebnf
label ::= IDENTIFIER ":"
```

Labels are used as targets for GOTO, GOSUB, ON ERROR GOTO, and ON...GOTO.

Example:
```basic
GOTO label1

label1:
    PRINT "Jumped here"
```

---

# 33. Error Handling

Parser must emit structured `ParseError` with file, line, column, span, message, expected token/construct, and actual token.

---

# 34. Parser Recovery

Minimal recovery: stop at first syntax error.

---

# 35. Examples

## Full Program
```basic
OPTION EXPLICIT

FUNCTION add(a AS INTEGER, b AS INTEGER) AS INTEGER
    add = a + b
END FUNCTION

DIM result AS INTEGER
result = add(10, 20)

IF result > 20 THEN
    PRINT "greater"
ELSE
    PRINT "smaller"
END IF
```

## SELECT CASE
```basic
SELECT CASE day
    CASE 1
        PRINT "Monday"
    CASE 2 TO 5
        PRINT "Weekday"
    CASE ELSE
        PRINT "Weekend"
END SELECT
```

## GOTO / GOSUB
```basic
GOTO Start

SubRoutine:
    PRINT "In subroutine"
RETURN

Start:
    GOSUB SubRoutine
    PRINT "Back from subroutine"
```

## Minimal Program
```
PRINT "Hello, RBASIC"
```

---

# 36. Acceptance Criteria

Parser must:
- Accept all forms defined above.
- Reject out‑of‑scope forms with structured errors.
- Produce an AST containing nodes for all statement and expression types.
- Pass parser unit tests.
- `cargo test` succeeds.
- Update CHANGELOG.
