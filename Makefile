
build:
	cargo build

run:
	RUST_LOG=info ./target/debug/check_alexa_sites

release:
	cargo build --release

linux-build:
	cross build --target x86_64-unknown-linux-gnu