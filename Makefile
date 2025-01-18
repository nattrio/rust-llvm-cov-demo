.PHONY: lcov lcov-watch lcov-html
lcov:
	cargo llvm-cov --lcov --output-path ./target/lcov.info
lcov-watch:
	cargo watch -x 'llvm-cov --lcov --output-path ./target/lcov.info' -w src
lcov-html:
	cargo llvm-cov --open