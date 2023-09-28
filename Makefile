run: check test
	cargo run
test: 
	cargo test
check:
	cargo check
	cargo clippy
up:
	cargo run -- up
down:
	cargo run -- down
