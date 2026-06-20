# RFC-0033: FOR EACH (Collection Iteration)

Status: Draft
Version: 0.1
Author: RBASIC Project
Created: 2026-06-20
Last Updated: 2026-06-20

---

## 1. Summary

Add `FOR EACH` iteration to RBASIC, enabling traversal of collections (arrays, strings, ranges) without manual index management. The loop variable receives each element value directly.

---

## 2. Syntax (EBNF)

```ebnf
for_each_stmt ::= "FOR" "EACH" IDENTIFIER "IN" expression NEWLINE
                  statement*
                  "END" "FOR"
```

- `FOR`, `EACH`, `IN`, `END`, `FOR` are case-insensitive reserved keywords.
- `IDENTIFIER` is the loop variable that receives each element.
- `expression` is the collection to iterate over (array, string, range).
- The loop variable is implicitly typed based on the collection element type.
- `EXIT FOR` is valid inside `FOR EACH` loops.

Examples:

```basic
DIM numbers(5) AS INTEGER
numbers(0) = 10
numbers(1) = 20
numbers(2) = 30
numbers(3) = 40
numbers(4) = 50

FOR EACH n IN numbers
    PRINT n
END FOR
```

```basic
DIM msg AS STRING
msg = "Hello"

FOR EACH ch IN msg
    PRINT ch
END FOR
```

```basic
DIM fruits(3) AS STRING
fruits(0) = "apple"
fruits(1) = "banana"
fruits(2) = "cherry"

FOR EACH fruit IN fruits
    PRINT "Fruit: " + fruit
END FOR
```

---

## 3. Semantics

1. `FOR EACH item IN collection` iterates over each element in `collection`.
2. The loop variable `item` is assigned the value of the current element on each iteration.
3. For arrays, `FOR EACH` iterates from index 0 to the upper bound.
4. For strings, `FOR EACH` iterates over each character.
5. For ranges, `FOR EACH` iterates over each value in the range.
6. The loop variable is read-only within the loop body (assignment to it emits `E1700`).
7. `EXIT FOR` is valid inside `FOR EACH` and terminates the loop.
8. Nested `FOR EACH` loops are supported.
9. The collection expression is evaluated once before the loop begins.

---

## 4. AST (node definitions)

### ForEach

```text
ForEach {
    var_name:  String,
    collection: Box<Expression>,
    body:      Vec<Statement>,
}
```

---

## 5. Parsing

When `FOR` is followed by `EACH`:

1. Consume `FOR`.
2. Check for `EACH` keyword.
3. Parse the loop variable name (identifier).
4. Consume `IN`.
5. Parse the collection expression.
6. Parse the loop body until `END FOR`.
7. Consume `END FOR`.
8. Produce `Statement::ForEach { var_name, collection, body }`.

```rust
fn parse_for_each() -> Result<ForEach> {
    consume(For);
    consume(Each);
    let var_name = expect_identifier()?;
    consume(In);
    let collection = parse_expression()?;
    let mut body = Vec::new();
    loop {
        if peek() == End && peek_ahead() == For {
            advance(); // END
            advance(); // FOR
            break;
        }
        body.push(parse_statement()?);
    }
    Ok(ForEach {
        var_name,
        collection: Box::new(collection),
        body,
    })
}
```

---

## 6. Semantic Analysis

1. **Collection type** — the collection expression must be an array, string, or range. Other types emit `E1701`.
2. **Element type inference** — the loop variable's type is inferred from the collection's element type.
3. **Read-only variable** — assignment to the loop variable emits `E1700`.
4. **Empty collection** — loops over empty collections execute zero iterations (no error).
5. **Nested iteration** — nested `FOR EACH` loops with the same variable name shadow the outer variable (warning emitted).

---

## 7. Code Generation

### Array Iteration

```basic
DIM numbers(3) AS INTEGER
numbers(0) = 10
numbers(1) = 20
numbers(2) = 30

FOR EACH n IN numbers
    PRINT n
END FOR
```

Compiles to:

```rust
let numbers = [10, 20, 30];
for n in numbers.iter() {
    println!("{}", n);
}
```

### String Iteration

```basic
DIM msg AS STRING
msg = "Hello"

FOR EACH ch IN msg
    PRINT ch
END FOR
```

Compiles to:

```rust
let msg = "Hello";
for ch in msg.chars() {
    println!("{}", ch);
}
```

### Range Iteration

```basic
FOR EACH i IN 1..10
    PRINT i
END FOR
```

Compiles to:

```rust
for i in 1..10 {
    println!("{}", i);
}
```

---

## 8. Error Codes

| Code  | Description                                           |
|-------|-------------------------------------------------------|
| E1700 | Assignment to read-only FOR EACH loop variable        |
| E1701 | FOR EACH applied to non-collection type               |

---

## 9. Acceptance Criteria

```text
✓ FOR EACH item IN collection parsed as ForEach
✓ Loop variable type inferred from collection element type
✓ Array iteration visits all elements
✓ String iteration visits all characters
✓ Range iteration visits all values
✓ Assignment to loop variable produces E1700
✓ FOR EACH on non-collection type produces E1701
✓ EXIT FOR inside FOR EACH terminates loop
✓ Empty collection results in zero iterations
✓ Array iteration compiles to for loop with iter()
✓ String iteration compiles to chars() iteration
✓ Full test suite passes
```
