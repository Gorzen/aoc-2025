set shell := ["bash", "-uc"]

# Runs 'all'
default:
    @just all

# Build
build:
    cargo build

# Run tests
test:
    cargo test

# Check code format
fmt-check:
    cargo fmt --all -- --check

# Format code
fmt:
    cargo fmt --all

# Lint code (cargo check and clippy)
lint:
    cargo check
    cargo clippy --all-features -- -D warnings

# Run all days, one by one
run-all:
    for i in {1..6}; do echo "Running day_$i"; cargo run -- day_$i; done

# Format, lint, build and test
all: fmt lint build test
