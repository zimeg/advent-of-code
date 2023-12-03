.PHONY: clean lint test

test: lint
	cargo test

lint:
	cargo fmt --check --all
	cargo clippy --no-deps -- -D warnings

clean:
	rm -rf target
