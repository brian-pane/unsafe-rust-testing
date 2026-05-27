#![no_main]

use core::hint::black_box;
use libfuzzer_sys::fuzz_target;
use unsafe_rust_testing::*;

fuzz_target!(|size: usize| {
    let mut buf1 = Box::new([0, 1, 2, 3, 4]);
    black_box(unsafe { write_buffer(buf1.as_mut_ptr(), size, b'!') });
});
