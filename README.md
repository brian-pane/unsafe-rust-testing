# Unsafe Rust Testing Example

This repository contains some intentionally buggy Rust and C code.
It was written for use in a tech talk about how to debug unsafe code.

## Build and Test
```
# One-time setup
cargo +stable install cargo-llvm-cov --locked

# Run a static lint analysis
cargo clippy

# Compile the code and run the tests
cargo build
cargo test

# Measure the test coverage
cargo llvm-cov --html
```
The tests may pass, depending on platform implementation details and
some random chance. But that doesn't mean the code is sound.

## Test with Miri
```
# One-time setup
rustup +nightly component add miri

# Run the tests
MIRIFLAGS="-Zmiri-disable-isolation" cargo +nightly miri test --no-fail-fast
```
This should catch a use-after-free and an array bounds overread in the code.

## Test with Valgrind
```
env CARGO_TARGET_$(rustc -vV | sed -n 's|host: ||p' | tr '[a-z]-' '[A-Z]_')_RUNNER="valgrind --track-origins=yes --error-exitcode=1" cargo test
```

## Test with ASAN
```angular2html
# One-time setup
rustup toolchain install nightly

# Run the tests
# You may need to change "llvm-symbolizer-21" to a different version, depending
# on the LLVM version installed on your system.
env ASAN_SYMBOLIZER_PATH=$(which llvm-symbolizer-21) ASAN_OPTIONS=halt_on_error=0 RUSTFLAGS=-Zsanitizer=address cargo +nightly test
```

## Analyze with Kani
```
# One-time setup
cargo install --locked kani-verifier
cargo kani setup

# Run the analyzer
cargo kani
```