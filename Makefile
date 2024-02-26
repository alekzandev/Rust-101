.PHONY: all build check doc test format format-check lint

all: build test

build:
	@cargo build

check:
	@cargo check

doc:
	@cargo doc

test:
	@cargo test
	@cargo test --all-features

format:
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all

format-check:
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	@cargo clippy

verify-makefile-format:
	cat -e -t -v Makefile | grep '\t' && echo "Makefile contains tabs" && exit 1 || echo "Makefile is formatted correctly"