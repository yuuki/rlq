BUILD = target/release/rlq

.PHONY: build
build:
	@which rustc > /dev/null || { echo "rlq requires Rust to compile. For installation instructions, please visit http://rust-lang.org/"; exit 1; }
	cargo build --release

.PHONY: test
test:
	cargo test
