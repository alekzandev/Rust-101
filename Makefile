SHELL := /bin/bash

.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: build test

build: ## Build the project using cargo
	@cargo build

check: ## Check the project using cargo
	@cargo check

clean: ## Clean the project using cargo
	@cargo clean

doc: ## Generate the documentation using cargo
	@cargo doc

test: ## Run the tests using cargo
	@cargo test
	@cargo test --all-features

format: ## Format the code using cargo
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all

format-check: ## Check the code formatting using cargo
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all -- --check

lint: ## Lint the code using cargo and clippy
	@rustup component add clippy 2> /dev/null
	@cargo clippy

verify-makefile-format: ## Verify that the Makefile is formatted correctly
	cat -e -t -v Makefile | grep '\t' && echo "Makefile contains tabs" && exit 1 || echo "Makefile is formatted correctly"

bump: ## Bump the version of the project
	@echo "Current version: $(shell cargo pkgid | cut -d# -f2)"
	@read -p "Enter the new version: " version; \
	updated_version=$$(cargo pkgid | cut -d# -f2 | sed -E "s/([0-9]+\.[0-9]+\.[0-9]+)$$/$$version/"); \
	sed -i -E "s/^version = .*/version = \"$$updated_version\"/" Cargo.toml; \
	@echo "Updated version: $(shell cargo pkgid | cut -d# -f2)"%
