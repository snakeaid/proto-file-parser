.PHONY: build test format lint doc clean pre-commit

build:
	cargo build --release

test:
	cargo test --all

format:
	cargo fmt --all

lint:
	cargo clippy -- -D warnings

doc:
	cargo doc --no-deps

clean:
	cargo clean

run:
	cargo run

pre-commit: format lint test
	@echo "All checks passed!"

publish:
	cargo publish

.DEFAULT_GOAL := build