.PHONY: all build run test clean fmt clippy doc release

all: build

build:
	docker build -t factrs .

run: build
	docker run factrs

release:
	cargo build --release

clean:
	cargo clean
