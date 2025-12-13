# Default recipe - show available recipes
default:
    @just --list

# Run all checks (similar to full CI)
ci: install-deps check check-all-features check-examples check-minimal-versions test-all-features fmt clippy deny

install-deps:
    @cargo install cargo-semver-checks

# Check with default features
check:
    cargo check

# Check with all features
check-all-features:
    cargo check --all-features
    cargo check --features strict-dates

# Check that all examples build with various feature combinations
check-examples:
    cargo check --examples --all-features
    cargo check --examples --no-default-features
    cargo check --examples --no-default-features --features parser

# Check with beta toolchain
check-beta:
    cargo +beta check
    cargo +beta check --all-features

# Check minimal versions (requires nightly)
check-minimal-versions:
    cargo +nightly update -Z minimal-versions
    cargo +nightly check
    cargo +nightly check --all-features

# Run all tests (default features)
test:
    cargo test

# Run tests with all feature combinations from CI matrix
test-all-features:
    cargo test --no-default-features
    cargo test --no-default-features --features parser
    cargo test --no-default-features --features "parser,strict-dates"
    cargo test --no-default-features --features "parser,serde_json"
    cargo test --no-default-features --features "parser,chrono-tz"
    cargo test --no-default-features --features "parser,time"
    cargo test --no-default-features --features chrono-tz
    cargo test --no-default-features --features time
    cargo test --no-default-features --features "parser,serde_json,chrono-tz,time"

# Run tests with parser feature
test-parser:
    cargo test --features parser

# Run tests with chrono-tz feature
test-tz:
    cargo test --features chrono-tz

# Run all test variants (includes feature matrix)
test-all: test test-all-features test-parser test-tz

# Format check
fmt:
    cargo fmt --all --check

# Format code
fmt-fix:
    cargo fmt --all

# Run clippy
clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run clippy and fix issues automatically
clippy-fix:
    cargo clippy --workspace --all-targets --all-features --fix

# Run cargo-deny checks
deny:
    cargo deny check advisories
    cargo deny check bans licenses sources

# Run only advisory checks
deny-advisories:
    cargo deny check advisories

# Run bans, licenses, and sources checks
deny-bls:
    cargo deny check bans licenses sources

# Clean build artifacts
clean:
    cargo clean

# Build the project
build:
    cargo build

# Build with all features
build-all:
    cargo build --all-features

# Build release version
build-release:
    cargo build --release
