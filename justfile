build:
	cargo build --release

# Run a specific example
run example:
	cargo run --release --example {{example}}

# Runs a clippy check
check *args:
    cargo clippy {{args}} -- -W clippy::pedantic
