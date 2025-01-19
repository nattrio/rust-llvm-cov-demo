.PHONY: setup
setup:
	which cargo-nextest || cargo install cargo-nextest --locked
	which  cargo-llvm-cov || cargo +stable install cargo-llvm-cov --locked
	cargo add rstest

.PHONY: lcov lcov-watch lcov-html
lcov:
	cargo llvm-cov nextest --lcov --output-path ./target/lcov.info
lcov-watch:
	cargo watch -x 'llvm-cov nextest --lcov --output-path ./target/lcov.info' -w src
lcov-html:
	cargo llvm-cov nextest --open