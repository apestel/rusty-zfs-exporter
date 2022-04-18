all: build

build:
	cargo build --release --target=x86_64-unknown-linux-musl

deploy:
	scp ./target/x86_64-unknown-linux-musl/release/rusty-zfs-exporter orion:~/