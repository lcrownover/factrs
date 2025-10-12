.PHONY: all build run test clean fmt clippy doc release

all: build

build:
	docker build -t factors .

run: build
	docker run factors

release:
	cargo build --release

clean:
	cargo clean

