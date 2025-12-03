# Runs 'all'
default:
    @just all

# Build
build:
    cargo build

# Run tests
test:
    cargo test

# Format code
fmt:
    cargo fmt --all

# Lint code (cargo check and clippy)
lint:
    cargo check
    cargo clippy --all-features -- -D warnings

# Format, lint, build and test
all: fmt lint build test
