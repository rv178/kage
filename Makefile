BINARY=chess-engine
BUILD_FLAGS=--release
all: build

build:
	@cargo build $(BUILD_FLAGS)
	@cp ./target/release/$(BINARY) ./bin/$(BINARY)

check:
	@cargo fmt
	@cargo check

clean:
	@cargo clean

setup:
	@mkdir bin
	@rustup install stable
	@rustup default stable

.PHONY: check clean setup all build
