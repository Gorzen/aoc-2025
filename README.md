# ⭐ Advent of Code 2025 ⭐

https://adventofcode.com/2025

My repository for the Advent of Code 2025 challenge, in Rust 🦀

## Runing & testing

- To run a specific day: `cargo run -- day_n`
- To test a specific day `cargo test day_n`

There is also a `justfile` to run other useful commands

```
$ just --list
Available recipes:
    all       # Format, lint, build and test
    build     # Build
    default   # Runs 'all'
    fmt       # Format code
    fmt-check # Check code format
    lint      # Lint code (cargo check and clippy)
    run-all   # Run all days, one by one
    test      # Run tests
```
