# RBASIC

A modern BASIC language written for safety and simplicity, with its own compiler `rbasic` written in Rust.

## Building

```bash
cargo check
cargo test
```

## Examples

- `examples/hello.rbas`
- `examples/add.rbas`

## Development Commands

```bash
make help        # Show available make commands
make check       # Run `cargo check`
make test        # Run `cargo test`
make build       # Run `cargo build`
make fmt         # Run `cargo fmt`
make lint        # Run `cargo clippy` with warnings as errors
make verify      # Verify formatting, linting, and tests
```
