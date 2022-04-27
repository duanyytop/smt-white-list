build:
	cargo fmt
	cargo build

build-release:
	cargo fmt
	cargo build --release

run:
	cargo fmt
	RUST_LOG=info cargo run

run-release:
	RUST_LOG=info ./target/release/cota-aggregator

.PHONY: build run