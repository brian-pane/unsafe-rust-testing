use std::ffi::c_char;

use libc::{calloc, free, size_t};

unsafe extern "C" {
    pub fn write_buffer(data: *mut c_char, size: size_t, pattern: c_char);
}

pub fn get_buffer(size: usize) -> *const u8 {
    let buf = unsafe { calloc(size as _, 1) };
    unsafe { free(buf as _) };
    buf as _
}

/// # Safety
///
/// `addr` must point to at least `size` initialized and readable bytes.
pub unsafe fn read_buffer(addr: *const u8, size: usize) -> Vec<u8> {
    let mut copy = Vec::with_capacity(size);
    let dest = copy.as_mut_ptr();
    unsafe { core::ptr::copy_nonoverlapping(addr, dest, size) };
    unsafe { copy.set_len(size) };
    copy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_after_free() {
        const SIZE: usize = 10;
        let mut data = get_buffer(SIZE);
        assert!(!data.is_null());
        for i in 0..SIZE {
            println!("data[{}] is {}", i, unsafe { *data });
            data = unsafe { data.add(1) };
        }
    }

    #[test]
    fn test_read_overflow() {
        let buf1 = [0, 1, 2, 3, 4];
        let _buf2 = [0, 0, 0, 0, 0];

        // Read beyond the bounds of a slice
        assert_eq!(
            unsafe { read_buffer(buf1.as_ptr(), 6) }.as_slice(),
            &[0, 1, 2, 3, 4, 0]
        );
    }

    #[test]
    fn test_write_overflow() {
        let mut buf1 = [0u8; 5];

        // Write beyond the bounds of a slice
        unsafe { write_buffer(buf1.as_mut_ptr().cast(), 6, 'c' as _) };
        assert_eq!(buf1, ['c' as u8; 5]);
    }
}

#[cfg(kani)]
#[kani::proof]
fn check_use_after_free() {
    let size = kani::any();
    let mut data = get_buffer(size);
    assert!(!data.is_null());
    let mut sum = 0i32;
    for _ in 0..size {
        sum += unsafe { *data } as i32;
        data = unsafe { data.add(1) };
    }
    assert!(sum >= 0);
}

#[cfg(kani)]
#[kani::proof]
fn check_read_overflow() {
    let buf1 = [0, 1, 2, 3, 4];
    let size = kani::any();
    unsafe { read_buffer(buf1.as_ptr(), size) };
}

#[cfg(kani)]
#[kani::proof]
fn check_write_overflow() {
    let mut buf1 = [0, 1, 2, 3, 4];
    let size = kani::any();
    unsafe { write_buffer(buf1.as_mut_ptr().cast(), size, 'c' as _) };
}
