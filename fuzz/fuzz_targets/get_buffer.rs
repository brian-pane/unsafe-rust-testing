#![no_main]

use libfuzzer_sys::fuzz_target;

use unsafe_rust_testing::*;

fuzz_target!(|input: u16| {
    let size = input as usize;
    let mut data = get_buffer(size);
    assert!(!data.is_null());
    let mut sum = 0i32;
    for _ in 0..size {
        sum += unsafe { *data } as i32;
        data = unsafe { data.add(1) };
    }
    assert!(sum >= 0);

});
