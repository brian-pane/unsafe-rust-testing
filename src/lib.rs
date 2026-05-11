use std::ffi::{c_char, c_long};

use libc::{calloc, free};

unsafe extern "C" {
    pub fn checksum(data: *const c_char) -> c_long;
}

pub fn get_data(size: usize) -> *const u8 {
    let buf = unsafe { calloc(size as _, 1) };
    unsafe { free(buf as _) };
    buf as _
}

pub fn get_length(data: &[u8]) -> usize {
    unsafe { libc::strlen(data.as_ptr().cast()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_data() {
        const SIZE: usize = 10;
        let mut data = get_data(SIZE);
        assert!(!data.is_null());
        for i in 0..SIZE {
            println!("data[{}] is {}", i, unsafe { *data });
            data = unsafe { data.add(1) };
        }
    }

    #[test]
    fn test_get_length() {
        let buf = ['a' as u8; 5];
        assert_eq!(get_length(&buf), 5);
    }

    #[test]
    #[cfg(not(miri))]
    fn test_checksum() {
        let s1: [c_char; 5] = [1, 2, 3, 4, 0];
        let s2: [c_char; 5] = [100; 5];
        let s3: [c_char; 5] = [0; 5];
        assert_eq!(unsafe { checksum(s1.as_ptr()) }, 10);
        assert_eq!(unsafe { checksum(s2.as_ptr()) }, 500);
        assert_eq!(unsafe { checksum(s3.as_ptr()) }, 0);
    }
}

#[cfg(kani)]
#[kani::proof]
fn check_get_data() {
    const SIZE: usize = 10;
    let mut data = get_data(SIZE);
    assert!(!data.is_null());
    for i in 0..SIZE {
        println!("data[{}] is {}", i, unsafe { *data });
        data = unsafe { data.add(1) };
    }
}
