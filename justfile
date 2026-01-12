# List available commands
default:
    @just --list

# Run all tests
test:
    cargo nextest run --all-features

# Run clippy lints
check:
    cargo clippy --all-targets --all-features -- -D warnings

# Format code
fmt:
    cargo fmt --all

# Check formatting without changes
fmt-check:
    cargo fmt --all --check

# Build release binary
build:
    cargo build --release

# Run all CI checks locally
ci: fmt-check check test build
