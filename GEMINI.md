# RBASIC Project

RBASIC is a modern programming language inspired by the simplicity of BASIC, built with contemporary principles of safety, performance, and maintainability. It is designed to be easy to learn while providing the power of a modern, statically‑typed, and memory‑safe language.

The project includes the `rbasic` compiler, currently implemented in Rust, which targets Rust code as an intermediate step to leverage LLVM via `rustc` for native compilation.

## Current Status

- **Version:** 0.2
- **Tests:** 363 passing (`make verify` succeeds)
- **RFCs:** 19 accepted and implemented (RFC‑0001 through RFC‑0019)
- **Roadmap:** v0.2 complete; v0.3 planned (modules, `Result`, `Optional`, `Ref`).

## Architecture

The compiler follows a traditional multi‑stage pipeline:

1. **Lexer** – Converts source code into a stream of tokens.
2. **Parser** – Transforms tokens into an Abstract Syntax Tree (AST).
3. **Semantic Analysis** – Performs type checking and symbol resolution.
4. **Code Generation** – Translates the analysed AST into Rust source code.
5. **Compilation** – The generated Rust code is compiled by `rustc` (LLVM) into a native executable.

The backend strategy is **RBASIC → Rust → rustc (LLVM)**. Memory management uses a hybrid stack + ARC model: primitive types are copied on assignment, while `STRING` and arrays use heap‑allocated `String`/`Vec<T>` with RAII.

## Implemented Features (v0.2)

### Language Constructs

- **Variables:** `LET name [":" type] "=" expr`, `LET MUT` for mutability.
- **Assignment:** standalone (`x = expr`) and compound (`+=`, `-=`, `*=`, `/=`, `\=`, `MOD=`).
- **Functions:** `FUNCTION name(params) [RETURNS type] ... END FUNCTION`.
- **Control flow:** `IF/ELSE`, `WHILE`, `FOR var = start TO end [STEP step]`, `DO WHILE/UNTIL ... LOOP` (4 variants).
- **Arrays:** `DIM name(dim1, dim2, ...)` – codegen emits `Vec<T>`.
- **Error handling:** `ON ERROR GOTO label` and `RESUME [label]` (parsed, codegen deferred).
- **I/O:** `PRINT expr`, `INPUT [prompt ","] variable`.
- **Comments:** single‑line with `'`.

### Built‑in Functions (String & Conversion)

`LEN`, `MID$`, `LEFT$`, `RIGHT$`, `CHR$`, `ASC`, `INSTR`, `VAL`, `STR$`, `UCASE$`, `LCASE$`, `TRIM$`, `LTRIM$`, `RTRIM$`, `SPACE$`, `STRING$`. Non‑`$` aliases (e.g., `MID`) are also accepted.

### Types

- **Primitives:** `BOOL`, `I8`, `I16`, `I32`, `I64`, `U8`, `U16`, `U32`, `U64`, `F32`, `F64`, `STRING`.
- **Classic BASIC aliases:** `INTEGER`→I32, `LONG`/`LONGLONG`→I64, `DOUBLE`→F64, `SINGLE`→F32, `BOOLEAN`→BOOL, `BYTE`→U8, `WORD`→U16, `DWORD`→U32, `QWORD`→U64.

## Building and Running

### Development Commands

| Command | Description |
|---------|-------------|
| `make check` / `cargo check` | Verify code compilation and syntax |
| `make test` / `cargo test` | Run the project's test suite |
| `make build` / `cargo build` | Build the `rbasic` compiler |
| `make fmt` | Format Rust code |
| `make lint` / `cargo clippy` | Run linting (warnings as errors) |
| `make verify` | Run `fmt`‑check, `lint`, and `test` together |
| `make test-lexer`, `make test-parser`, `make test-semantic`, `make test-types`, `make test-all` | Granular test suites |

### Installing and Using the CLI

Install the compiler:

```bash
cargo install --path .
```

Usage:

```
rbasic <command> <file.rbas> [output.rs]
```

Commands:

- `check <file>` – lexical, syntactic, and semantic analysis only.
- `build <file> [output]` – generate Rust code; if no output, prints to stdout.
- `run <file>` – compile to a temporary Rust file, build, and execute.

Example:

```bash
rbasic run examples/hello.rbas
```

## Directory Structure

```
.
├── src/                     # Core compiler (Rust)
│   ├── lexer/               # Lexical analysis
│   ├── parser/              # Syntax analysis and AST
│   ├── semantic/            # Semantic analysis, type checking, scopes
│   ├── codegen/             # Rust code generation
│   ├── diagnostics/         # Error/warning reporting
│   ├── lib.rs               # Library entry
│   └── main.rs              # CLI entry
├── runtime/                 # C runtime (stubs)
├── docs/                    # Documentation (RFCs, intention document)
├── examples/                # Sample .rbas programs
├── tests/                   # Test suites
├── Makefile                 # Build automation
├── Cargo.toml               # Rust package manifest
├── CHANGELOG.md             # Changelog
├── AGENTS.md                # Developer guidelines
├── README.md                # RFC index and status
└── GEMINI.md                # This file
```

## Documentation & RFCs

All language design decisions are documented in **RFCs** (Requests for Comments) in the `docs/` directory. See [README.md](README.md) for the full index and status.

- **RFC‑0001** Vision and Goals (Accepted)
- **RFC‑0002** Lexical Specification (Accepted)
- **RFC‑0003** MVP Definition (Accepted)
- **RFC‑0004** Grammar Specification (Accepted)
- **RFC‑0005** AST Specification (Accepted)
- **RFC‑0006** Semantic Analysis (Accepted)
- **RFC‑0007** Type Compatibility (Accepted)
- **RFC‑0008** Type Checking (Accepted)
- **RFC‑0009** FOR…STEP (Accepted)
- **RFC‑0010** DO Loop (Accepted)
- **RFC‑0011** Type Aliases (Accepted)
- **RFC‑0012** DIM Arrays (Accepted)
- **RFC‑0013** ON ERROR / RESUME (Accepted)
- **RFC‑0014** Memory Management (Accepted)
- **RFC‑0015** Standalone Assignment (Accepted)
- **RFC‑0016** DIM Array Codegen (Accepted)
- **RFC‑0017** String Functions (Accepted)
- **RFC‑0018** Compound Assignment (Accepted)
- **RFC‑0019** INPUT Statement (Accepted)
- **RFC‑1000** RBA Vision (Draft)

## Contributing

Please read [AGENTS.md](AGENTS.md) for detailed guidelines.

**Key rules:**

- All language changes require an **Accepted** RFC first.
- Update the **CHANGELOG** with every significant change.
- Run `make verify` before submitting a pull request.
- Follow the code style (`make fmt`, `make lint`).

## License

This project is licensed under the MIT License (see LICENSE file).

## Roadmap

- **v0.1** ✅ – MVP (core types, control flow, functions, `PRINT`)
- **v0.2** ✅ – Arrays, string functions, assignment, input, compound ops
- **v0.3** – Modules, `Result<T,E>`, `Optional<T>`, `Ref<T>`
- **Future** – Standard library, package system, self‑hosting, RBA (office automation)

Last updated: 2026‑06‑20*
