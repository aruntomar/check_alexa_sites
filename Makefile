
build:
	cargo build

run:
	RUST_LOG=info ./target/debug/check_alexa_sites

release:
	cargo build --release

