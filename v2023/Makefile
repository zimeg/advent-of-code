.PHONY: clean lint test

test: lint
	cargo test --release

lint:
	cargo fmt --check --all
	cargo clippy --no-deps -- -D warnings

clean:
	rm -rf target
