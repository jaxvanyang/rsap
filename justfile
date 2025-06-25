build:
	cargo build --release

# Run a specific app
run app:
	cargo run --release --bin {{app}}

# Runs a clippy check
check *args:
    cargo clippy {{args}}
