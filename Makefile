.PHONY: test fmt lint check run shell

TARGET ?= projects/cli_calculator

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

check:
	cargo fmt --check
	cargo clippy -- -D warnings
	cargo test

run:
	cargo run -p $(notdir $(TARGET))

shell:
	docker compose run --rm rust-learning bash

