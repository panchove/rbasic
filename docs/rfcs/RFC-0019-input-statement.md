# RFC-0019 INPUT Statement

**Status:** Accepted

## Purpose
Add interactive user input capability to RBASIC, allowing programs to read values from standard input and assign them to mutable variables.

## Syntax (EBNF)
```
input_stmt ::= INPUT identifier
input_stmt ::= INPUT string_literal "," identifier
```
- `INPUT` is a case‑insensitive reserved keyword.
- `identifier` must refer to a previously declared mutable variable.
- An optional prompt string may be supplied; the prompt is written to stdout before reading.

## Semantic Rules
1. **Target must be declared** – otherwise emit `E1050` (Unknown Input Target).
2. **Target must be mutable** – otherwise emit `E1051` (Immutable Input Target).
3. **Target type must be a supported primitive** (`STRING`, `INTEGER`, `LONG`, `DOUBLE`, `BOOLEAN`, `BYTE`). If not, emit `E1052` (Unsupported Input Type).
4. The input value is converted according to the target type (see Type Semantics table).

## Type Semantics
| Target Type | Accepted Input |
|-------------|----------------|
| STRING      | Any text       |
| INTEGER     | Valid integer  |
| LONG        | Valid integer  |
| DOUBLE      | Valid floating‑point value |
| BOOLEAN     | `TRUE` / `FALSE` (case‑insensitive) |
| BYTE        | Integer in range 0‑255 |

Invalid conversions result in a runtime diagnostic (not a compile‑time error).

## Code Generation (Rust Backend)
For each `INPUT` statement the generator emits something equivalent to:
```rust
let mut buffer = String::new();
std::io::stdout().write_all(prompt.as_bytes()).unwrap(); // if prompt present
std::io::stdin().read_line(&mut buffer).unwrap();
```
Followed by a type‑specific conversion:
- **STRING** – `buffer.trim().to_string();`
- **INTEGER / LONG** – `buffer.trim().parse::<i32>().unwrap();`
- **DOUBLE** – `buffer.trim().parse::<f64>().unwrap();`
- **BOOLEAN** – `matches!(buffer.trim().to_ascii_uppercase().as_str(), "TRUE")`
- **BYTE** – `buffer.trim().parse::<u8>().unwrap();`

## Examples
```basic
DIM name AS STRING
DIM age AS INTEGER
INPUT "Name: ", name
INPUT age
```

## Diagnostics
| Code | Description | RFC |
|------|-------------|-----|
| E1050 | Unknown input target (variable not declared) | RFC‑0019 |
| E1051 | Immutable input target (cannot assign) | RFC‑0019 |
| E1052 | Unsupported input type for INPUT statement | RFC‑0019 |

## Acceptance Criteria
- Lexer recognizes `INPUT` keyword (case‑insensitive) and reserves it.
- Parser produces `Statement::Input { prompt: Option<String>, target: String }` nodes.
- Semantic analysis validates the three rules above and emits the corresponding diagnostics.
- Rust codegen generates stdin reading, optional prompt output, and correct type conversion.
- Tests cover parser, semantic validation (valid, undeclared, immutable, unsupported type) and codegen output.
- `make verify` passes with no warnings or clippy errors.
