prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cd managed_counter && cargo build --release --target wasm32-unknown-unknown

lint:
	cd tests && cargo fmt
	cd managed_counter && cargo fmt
	cd tests && cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints
	cd managed_counter && cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints

clean:
	cd tests && cargo clean
	cd managed_counter && cargo clean

copy-wasm-file-to-test:
	cp managed_counter/target/wasm32-unknown-unknown/release/managed_counter.wasm tests/wasm/

test-only:
	cd tests && cargo test -p tests

test: build-contract copy-wasm-file-to-test test-only
