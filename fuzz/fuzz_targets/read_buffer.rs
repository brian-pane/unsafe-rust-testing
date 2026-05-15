#![no_main]

use core::cmp::min;

use libfuzzer_sys::fuzz_target;

use unsafe_rust_testing::*;

fuzz_target!(|input: (u16, u16)| {
    let (buf_size, read_size) = input;
    let buf_size = buf_size as usize;
    let read_size = read_size as usize;
    let src_buf = vec![1u8; buf_size];
    let out_buf = unsafe { read_buffer(src_buf.as_ptr(), read_size) };
    assert_eq!(out_buf.as_slice()[..min(buf_size, read_size)], src_buf.as_slice()[..min(buf_size, read_size)]);
});
