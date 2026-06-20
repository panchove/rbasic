# RFC-0020: SELECT CASE Statement

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add multi-way conditional branching via `SELECT CASE` to RBASIC. This replaces chained `IF/ELSEIF` blocks with a cleaner pattern-matching construct. The expression is evaluated once and compared against one or more `CASE` clauses until a match is found; the matching clause's body executes. A `CASE ELSE` clause serves as the fallback.

---

## 2. Syntax (EBNF)

```ebnf
select_case_stmt ::= "SELECT" "CASE" expression NEWLINE
                     case_clause+
                     [case_else_clause]
                     "END" "SELECT"

case_clause       ::= "CASE" case_values NEWLINE statement*
case_else_clause  ::= "CASE" "ELSE" NEWLINE statement*

case_values       ::= case_value ("," case_value)*
case_value        ::= expression
                    | expression "TO" expression
```

- `SELECT`, `CASE`, `TO`, `ELSE`, `END` are case-insensitive reserved keywords.
- The expression after `SELECT CASE` is evaluated once and compared against each `CASE`.
- Each `CASE` may have one or more comma-separated values.
- A single value matches equality; `value1 TO value2` matches inclusive range.
- `CASE ELSE` is optional; if present, it must be the last clause.
- `END SELECT` terminates the construct.

Examples:

```basic
SELECT CASE grade
    CASE 90 TO 100
        PRINT "A"
    CASE 80 TO 89
        PRINT "B"
    CASE 70 TO 79
        PRINT "C"
    CASE ELSE
        PRINT "F"
END SELECT
```

```basic
SELECT CASE command$
    CASE "QUIT", "EXIT"
        END
    CASE "HELP"
        PRINT "No help available"
END SELECT
```

---

## 3. Semantics

1. The `SELECT CASE` expression is evaluated once.
2. Case clauses are evaluated **in order** from top to bottom.
3. For a single-value `CASE`, the clause body executes if the value equals the select expression (using `=` semantics).
4. For a `CASE value1 TO value2`, the clause body executes if the select expression is between `value1` and `value2` **inclusive**. The range is valid only if `value1 <= value2`; otherwise, emit `E1060`.
5. The first matching clause wins; subsequent clauses are skipped.
6. If no clause matches and `CASE ELSE` exists, the else body executes.
7. If no clause matches and no `CASE ELSE` exists, the `SELECT` block is skipped.
8. All `CASE` values and ranges must be type-compatible with the select expression.

---

## 4. AST (node definitions)

```text
SelectCase {
    expr:     Expression,
    cases:    Vec<CaseClause>,
    else_case: Option<Vec<Statement>>,
}

CaseClause {
    values: Vec<CaseValue>,
    body:   Vec<Statement>,
}

CaseValue ::= Single(Expression)
            | Range(Expression, Expression)
```

These nodes are already defined in RFC-0005 §4.9.

---

## 5. Parsing

The parser recognizes `SELECT CASE` as a compound statement:

1. Consume `SELECT`, then `CASE`, then parse the select expression.
2. Loop: parse `CASE` clauses until `END SELECT` is reached.
   - Each `CASE` starts with the `CASE` keyword.
   - Parse comma-separated case values:
     - If `TO` follows a value, parse as `CaseValue::Range(expr1, expr2)`.
     - Otherwise, parse as `CaseValue::Single(expr)`.
   - Parse the clause body (statements until the next `CASE` or `END SELECT`).
3. A standalone `CASE ELSE` (detected by `CASE` + `ELSE` token) produces `else_case`.
4. Consume `END SELECT`.

Pseudocode:

```rust
fn parse_select_case() -> Result<SelectCase> {
    consume(Select);
    consume(Case);
    let expr = expression()?;
    let mut cases = Vec::new();
    let mut else_case = None;

    loop {
        match peek() {
            Case => {
                advance();
                if peek() == Else {
                    advance();
                    else_case = Some(statements_until(EndSelect));
                } else {
                    let values = parse_case_values()?;
                    let body = statements_until_next_case_or_end();
                    cases.push(CaseClause { values, body });
                }
            }
            EndSelect => { advance(); break; }
            EOF => return Err("Expected END SELECT"),
        }
    }

    Ok(SelectCase { expr, cases, else_case })
}
```

---

## 6. Semantic Analysis

1. The select expression must be of a comparable type (INTEGER, LONG, DOUBLE, STRING, BOOLEAN, BYTE).
2. All `CaseValue` expressions must be type-compatible with the select expression. Incompatible types emit `E1020`.
3. For `Range(expr1, expr2)`, both expressions must be type-compatible with the select expression.
4. `CASE ELSE` must be the last clause. If `CASE ELSE` appears before other `CASE` clauses, emit `E1061`.
5. Duplicate case values (exact equality) emit `E1062` as a warning.
6. The `END SELECT` must match the opening `SELECT CASE`. Mismatched nesting emits `E1063`.

---

## 7. Code Generation

`SELECT CASE` compiles to an if-else-if chain:

```basic
SELECT CASE x
    CASE 1
        PRINT "one"
    CASE 2 TO 5
        PRINT "small"
    CASE ELSE
        PRINT "other"
END SELECT
```

Compiles to:

```rust
if x == 1 {
    println!("one");
} else if x >= 2 && x <= 5 {
    println!("small");
} else {
    println!("other");
}
```

For range cases, the codegen emits `&&` of `>=` and `<=`. For single values, it emits `==`.

---

## 8. Error Codes

| Code  | Description                                   |
|-------|-----------------------------------------------|
| E1060 | Invalid range: lower bound greater than upper |
| E1061 | CASE ELSE must be the last clause             |
| E1062 | Duplicate case value (warning)                |
| E1063 | Unmatched END SELECT                         |
| E1020 | Type mismatch (reused from RFC-0007)          |

---

## 9. Acceptance Criteria

```text
✓ SELECT CASE with single values parsed correctly
✓ SELECT CASE with TO ranges parsed correctly
✓ SELECT CASE with CASE ELSE parsed correctly
✓ CASE ELSE must be last clause (E1061 if not)
✓ Range bounds validated (E1060 if invalid)
✓ Type compatibility checked between select expr and case values
✓ Codegen produces correct if-else-if chain
✓ All 4 variants (single, range, multiple, else) tested
✓ Full test suite passes
```
