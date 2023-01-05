_default:
	@just --list

# Build
build:
	@cargo build

# Format
fmt:
	@cargo fmt --all

# Lint
lint:
	@cargo fmt --all -- --check
	@cargo clippy --all --tests -- -D warnings

# Test
test:
	@cargo test --all

# Run one day
run-day day:
	@cargo run --release -- run-day {{ day }}

# Run all days
run-all:
	@cargo run --release -- run-all
