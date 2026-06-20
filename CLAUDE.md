# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

Use `make` targets rather than invoking `cargo` directly:

```sh
make verify          # canonical validation: fmt-check + clippy + all tests (run before closing any task)
make test            # run all tests
make lint            # cargo clippy --all-targets --all-features -- -D warnings
make fmt             # format source
make build           # cargo build
make check           # cargo check

# Granular test targets
make test-lexer      # cargo test --test lexer_tests
make test-parser     # cargo test --test parser_tests
make test-semantic   # cargo test --test semantic_tests
make test-types      # cargo test --test semantic_type_tests

# Run a single test by name
cargo test --test semantic_tests test_name
```

### Using the compiler

```sh
cargo run -- check   examples/hello.rbas          # lex + parse + semantic check only
cargo run -- build   examples/hello.rbas out.rs   # emit Rust source
cargo run -- run     examples/hello.rbas           # compile via rustc and execute
```

## Architecture

The compiler is a classic single-pass pipeline. Each stage is a separate module under `src/`:

```text
.rbas source
  └─ lexer::lex()             → Vec<Token>               (src/lexer/)
       └─ Parser::parse_program()  → Program (AST)        (src/parser/)
            └─ semantic::analyze() → Ok / Err<SemanticError>  (src/semantic/)
                 └─ codegen::generate_rust() → String        (src/codegen/)
                      └─ rustc invocation (only for `run` command)
```

**Lexer** (`src/lexer/`): single-pass byte scanner; keywords are case-insensitive. Emits `Token { kind: TokenKind, span: Span }`. Unrecognised characters are silently skipped.

**Parser** (`src/parser/`): recursive-descent; produces the AST types defined in `src/parser/ast.rs`. `src/parser/token.rs` is the parser-side token wrapper (separate from the lexer's `src/lexer/token.rs`). Returns `ParseError { message, span }` on failure.

**AST** (`src/parser/ast.rs`): key types are `Program`, `Statement` (enum), `Expression` (enum), `TypeRef`, `Literal`, `BinaryOp`, `UnaryOp`, `DoLoopVariant`. All `Clone + PartialEq`.

**Semantic analyzer** (`src/semantic/analyzer.rs`): two-pass over the AST. Pass 1 collects all top-level `VarDecl` and `FunctionDecl` names/signatures into `globals` and `functions` maps. Pass 2 walks every statement, tracking a per-scope `locals: HashMap<String, Type>`. `resolve_expr` returns `Option<Type>` (None means a prior error was already recorded). Branch scopes are cloned (`locals.clone()`), so variables declared inside `if`/`while`/`for` bodies are not visible in outer scope.

**Type system** (`src/semantic/types.rs`): `Type` enum covers `Bool`, `I8/I16/I32/I64`, `U8/U16/U32/U64`, `F32/F64`, `String`. RBASIC type names (e.g. `INTEGER`, `LONG`, `SINGLE`, `DOUBLE`, `BOOLEAN`) map to Rust-equivalent `Type` variants via `Type::from_name()`. Integer widening is automatic within the same sign family; signed/unsigned mixing is `E1021`. Integer literals (`Literal::Int`) resolve as `Type::I32` by default and are implicitly compatible with any integer type. `F32` and `F64` are mutually promotable.

**Error codes** (`src/semantic/errors.rs`): `SemanticErrorCode` enum (E1001–E1034). E1001 = unknown variable, E1002 = duplicate global, E1003 = duplicate local/param/unknown function, E1004 = duplicate function, E1010 = unknown type, E1011 = duplicate parameter, E1020 = type mismatch, E1021 = invalid/binary operation (including signed/unsigned mixing), E1022 = invalid unary op, E1030 = wrong arg count, E1031 = return type mismatch, E1032 = non-bool condition, E1033 = return outside function, E1034 = non-numeric step.

**Code generation** (`src/codegen/rust.rs`): walks the `Program` AST and emits a single Rust `fn main() { … }` string. Functions declared in RBASIC are emitted as top-level Rust functions before `main`. Supported constructs: LET, LET MUT, PRINT, RETURN, IF/ELSE, WHILE, FOR/STEP, DO/LOOP variants, function calls, AS casts, all operators. No codegen for DIM, ON ERROR, or RESUME (deferred to future versions). No intermediate IR; direct AST-to-text translation.

## Process rules (from AGENTS.md)

**Every language change requires an RFC first.** The flow is: RFC Draft → Review → Accepted → Implementation → CHANGELOG entry. Only `Accepted` RFCs may be used as an implementation contract. This applies especially to: `Ref<T>`, `MutRef<T>`, `Optional<T>`, `Result<T,E>`, Modules, Generics, Ownership.

**CHANGELOG.md must be updated** before closing any task (Keep a Changelog format, `[Unreleased]` section at the top). Review the CHANGELOG before starting work.

**`make verify` must pass** before a milestone is considered complete.

**v0.1 is 100% implemented.** All 13 RFCs (0001–0013) are Accepted. 229 tests pass. The codebase is ready for v0.2 planning. DIM, ON ERROR, and RESUME are parsed but have no codegen yet — they are placeholders for future work.

## Key conventions

- Variable/function name lookup is always case-insensitive (`name.to_lowercase()` before map access).
- The analyzer clones `locals` when entering branch bodies — mutations inside branches do not propagate outward.
- `Literal::Int(_)` always resolves to `Type::I32` regardless of value; the compatibility rules in `types_compatible()` then allow widening to larger integer types.
- `can_cast_explicitly()` in the analyzer governs explicit `AS <type>` casts; implicit narrowing is rejected.
- Test files live in `tests/` as integration tests; they call into the public API exposed by `src/lib.rs`.
