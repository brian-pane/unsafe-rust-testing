#![no_main]

use core::hint::black_box;
use libfuzzer_sys::fuzz_target;
use unsafe_rust_testing::*;

fuzz_target!(|size: usize| {
    let buf1 = [0, 1, 2, 3, 4];
    black_box(unsafe { read_buffer_c(buf1.as_ptr(), size) });
});
