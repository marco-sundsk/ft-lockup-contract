RFLAGS="-C link-arg=-s"

build: src
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RFLAGS) cargo build -p ft-lockup --target wasm32-unknown-unknown --release
	mkdir -p res
	cp target/wasm32-unknown-unknown/release/ft_lockup.wasm ./res/ft_lockup.wasm


test: build
	RUSTFLAGS=$(RFLAGS) cargo test wtest_ -- --nocapture