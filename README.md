# Unsafe Rust Testing Example

This repository contains some intentionally buggy Rust and C code.
It was written for use in a tech talk about how to debug unsafe code.

## Build and Test
```
cargo build
cargo test
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
