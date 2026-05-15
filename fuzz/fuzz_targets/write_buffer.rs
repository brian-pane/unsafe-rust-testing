#![no_main]

use core::cmp::min;

use libfuzzer_sys::fuzz_target;

use unsafe_rust_testing::*;

fuzz_target!(|input: (u16, u16)| {
    let (buf_size, write_size) = input;
    let buf_size = buf_size as usize;
    let write_size = write_size as usize;
    let mut buf = vec![1u8; buf_size];
    unsafe { write_buffer(buf.as_mut_ptr().cast(), write_size, 2) };
    for i in 0..min(buf_size, write_size) {
        assert_eq!(buf[i], 2);
    }
});
