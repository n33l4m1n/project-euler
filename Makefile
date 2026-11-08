# `check` is what CI (gates.yml) and agent-pr.sh both run.
.PHONY: check fmt lint test
check: fmt lint test

fmt:
	cargo fmt --check

lint:
	cargo clippy --all-targets --locked -- -D warnings

test:
	cargo test --locked
