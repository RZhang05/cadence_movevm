ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

.PHONY: run-all
run-all: run-dynamic

.PHONY: build-all
build-all: build-dynamic

.PHONY: build-dynamic
build-dynamic:
	@cd lib/sample/go_lib && go build -o libmain.so -buildmode=c-shared .
	@cd lib/sample && cargo build --release
	@cp lib/sample/target/release/libsample.so lib/
	go build -ldflags="-r $(ROOT_DIR)lib" main.go

.PHONY: run-dynamic
run-dynamic: build-dynamic
	@./main
	@cd lib/sample/go_lib && go test --bench=.
	go test -ldflags="-r $(ROOT_DIR)lib" --bench=.

# This is just for running the Rust lib tests natively via cargo
.PHONY: test-rust-lib
test-rust-lib:
	@cd lib/sample && cargo test -- --nocapture

.PHONY: clean
clean:
	rm -rf main lib/libsample.so lib/libsample.a lib/sample/target lib/sample/Cargo.lock

