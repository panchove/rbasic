# RBASIC Project

RBASIC is a modern programming language inspired by the simplicity of BASIC, but built with contemporary principles of safety, performance, and maintainability. It is designed to be easy to learn while providing the power of a modern, statically-typed, and memory-safe language.

The project includes the `rbasic` compiler, currently implemented in Rust, which targets Rust code as an intermediate step to leverage LLVM via `rustc` for native compilation.

## Architecture

The compiler follows a traditional multi-stage pipeline:

1.  **Lexer:** Converts source code into a stream of tokens.
2.  **Parser:** Transforms tokens into an Abstract Syntax Tree (AST).
3.  **Semantic Analysis:** Performs type checking and symbol resolution on the AST.
4.  **Code Generation:** Translates the analyzed AST into Rust source code.
5.  **Compilation:** The generated Rust code is compiled by `rustc` into a native executable.

## Building and Running

### Development

The project uses `cargo` for dependency management and building, and a `Makefile` to provide convenient wrappers.

**Standard Commands:**

*   `make check` / `cargo check`: Verify code compilation and syntax.
*   `make test` / `cargo test`: Run the project's test suite.
*   `make build` / `cargo build`: Build the `rbasic` compiler.
*   `make fmt`: Format the Rust code using `cargo fmt`.
*   `make lint` / `cargo clippy`: Run linting checks.
*   `make verify`: A comprehensive command to run formatting, linting, and tests.

### Using the `rbasic` CLI

Once built, the `rbasic` executable can be used to interact with `.rbas` files.

**Usage:**
`rbasic <command> <file.rbas> [output.rs]`

**Commands:**

*   `check <file>`: Performs lexical, syntactic, and semantic analysis only.
*   `build <file> [output]`: Generates Rust code from the RBASIC source. If no output file is specified, it prints to stdout.
*   `run <file>`: Compiles the RBASIC source to a temporary Rust file, compiles it, and executes the resulting binary immediately.

## Directory Structure

*   `src/`: Core compiler implementation in Rust.
    *   `lexer/`: Lexical analysis logic.
    *   `parser/`: Syntax analysis and AST definitions.
    *   `semantic/`: Semantic analysis, type checking, and scope management.
    *   `codegen/`: Logic for generating Rust code from the AST.
    *   `diagnostics/`: Error and warning reporting mechanisms.
    *   `lib.rs`: Library entry point.
    *   `main.rs`: CLI entry point.
*   `runtime/`: C implementation of the RBASIC runtime (I/O, string handling, etc.).
*   `docs/`: Project documentation, including RFCs and intention documents.
*   `examples/`: Sample `.rbas` programs demonstrating language features.
*   `tests/`: Test suites for various compiler components (lexer, parser, semantic analysis, etc.) and integration tests.
*   `Makefile`: Automation script for common development tasks.
*   `Cargo.toml`: Rust package configuration.
