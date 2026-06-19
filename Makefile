# Makefile for RBASIC project

# Default target
.PHONY: help
help:
	@echo "Available commands:"
	@echo "  make help               - Show this help"
	@echo "  make check              - Run cargo check"
	@echo "  make test               - Run cargo test"
	@echo "  make build              - Run cargo build"
	@echo "  make run                - Run cargo run"
	@echo "  make fmt                - Run cargo fmt"
	@echo "  make lint               - Run cargo clippy with warnings as errors"
	@echo "  make clean              - Run cargo clean"
	@echo "  make verify             - Verify formatting, linting and tests"
	@echo "  make milestone-lexer    - Verify lexer milestone (cargo check & test)"
	@echo "  make milestone-parser   - Verify parser milestone (cargo check & test)"

.PHONY: check
default: check
check:
	cargo check

.PHONY: test
test:
	cargo test

.PHONY: build
build:
	cargo build

.PHONY: run
run:
	cargo run

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: clean
clean:
	cargo clean

.PHONY: verify
verify: fmt-check lint test

.PHONY: fmt-check
fmt-check:
	cargo fmt --check

.PHONY: milestone-lexer
milestone-lexer:
	$(MAKE) check
	$(MAKE) test

.PHONY: milestone-parser
milestone-parser:
	$(MAKE) check
	$(MAKE) test

# -----------------------------------------------------------------
# Semantic and granular test targets
.PHONY: test-lexer test-parser test-semantic test-types test-compatibility test-all

test-lexer:
	cargo test --test lexer_tests

test-parser:
	cargo test --test parser_tests

test-semantic:
	cargo test --test semantic_tests

# Phase‑2 type‑resolution tests

test-types:
	cargo test --test semantic_type_tests

# Phase‑3 type‑compatibility tests
# TODO: create semantic_compatibility_tests.rs

# test-compatibility:
# 	cargo test --test semantic_compatibility_tests

# Run every test suite

test-all: test-lexer test-parser test-semantic test-types
	@echo "All test suites executed."
# -----------------------------------------------------------------
