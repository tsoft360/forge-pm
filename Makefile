main:
	cargo build --release
	sudo mv target/release/forge-pm /usr/bin