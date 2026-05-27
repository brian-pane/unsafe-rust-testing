use libc::{calloc, free, size_t};

unsafe extern "C" {
    pub fn read_buffer_c(data: *const u8, size: size_t) -> u64;
    pub fn write_buffer_c(data: *mut u8, size: size_t, pattern: u8);
}

pub fn allocate_buffer(size: usize) -> *const u8 {
    unsafe { calloc(size as _, 1) }.cast()
}

pub fn free_buffer(buf: *const u8) {
    unsafe { free(buf as _) };
}

/// Read the bytes of a buffer and return a sum of their values.
///
/// # Safety
///
/// `addr` must point to at least `size` initialized and readable bytes.
pub unsafe fn read_buffer(addr: *const u8, size: usize) -> u64 {
    let mut sum = 0;
    let mut dest = addr;
    for _ in 0..size {
        sum += unsafe { *dest } as u64;
        dest = unsafe { dest.add(1) };
    }
    sum
}

/// Overwrite each byte of a buffer with a specified value.
///
/// # Safety
///
/// `addr` must point to at least `size` writable bytes.
pub unsafe fn write_buffer(addr: *mut u8, size: usize, pattern: u8) {
    let mut dest = addr;
    for _ in 0..size {
        unsafe { *dest = pattern };
        dest = unsafe { dest.add(1) };
    }
}

#[cfg(test)]
mod tests {
    use core::hint::black_box;

    use super::*;

    #[test]
    fn read_overflow_stack_rust() {
        let buf1 = [0, 1, 2, 3, 4];
        let _buf2 = [0, 0, 0, 0, 0];
        assert_eq!(unsafe { read_buffer(buf1.as_ptr(), 6) }, 10);
    }

    #[test]
    fn read_overflow_stack_c() {
        let buf1 = [0, 1, 2, 3, 4];
        let _buf2 = [0, 0, 0, 0, 0];
        assert_eq!(unsafe { read_buffer_c(buf1.as_ptr(), 6) }, 10);
    }

    #[test]
    fn read_overflow_heap_rust() {
        let buf1 = Box::new([0, 1, 2, 3, 4]);
        let _buf2 = Box::new([0, 0, 0, 0, 0]);
        assert_eq!(unsafe { read_buffer(buf1.as_ptr(), 6) }, 10);
    }

    #[test]
    fn read_overflow_heap_c() {
        let buf1 = Box::new([0, 1, 2, 3, 4]);
        let _buf2 = Box::new([0, 0, 0, 0, 0]);
        assert_eq!(unsafe { read_buffer_c(buf1.as_ptr(), 6) }, 10);
    }

    #[test]
    fn write_overflow_stack_rust() {
        let mut buf1 = [0, 1, 2, 3, 4];
        let _buf2 = [0, 0, 0, 0, 0];
        unsafe { write_buffer(buf1.as_mut_ptr(), 6, b'!') };
        assert_eq!(buf1, [b'!'; 5]);
    }

    #[test]
    fn write_overflow_stack_c() {
        let mut buf1 = [0, 1, 2, 3, 4];
        let _buf2 = [0, 0, 0, 0, 0];
        unsafe { write_buffer_c(buf1.as_mut_ptr(), 6, b'!') };
        assert_eq!(buf1, [b'!'; 5]);
    }

    #[test]
    fn write_overflow_heap_rust() {
        let mut buf1 = Box::new([0, 1, 2, 3, 4]);
        let _buf2 = Box::new([0, 0, 0, 0, 0]);
        unsafe { write_buffer(buf1.as_mut_ptr(), 6, b'!') };
        assert_eq!(*buf1, [b'!'; 5]);
    }

    #[test]
    fn write_overflow_heap_c() {
        let mut buf1 = Box::new([0, 1, 2, 3, 4]);
        let _buf2 = Box::new([0, 0, 0, 0, 0]);
        unsafe { write_buffer_c(buf1.as_mut_ptr(), 6, b'!') };
        assert_eq!(*buf1, [b'!'; 5]);
    }

    fn freed_stack_ref() -> *const u8 {
        let buf = [0, 1, 2, 3, 4];
        buf.as_ptr()
    }

    #[test]
    fn use_after_free_stack_rust() {
        let addr = freed_stack_ref();
        black_box(unsafe { read_buffer(addr, 5) });
    }

    #[test]
    fn use_after_free_stack_c() {
        let addr = freed_stack_ref();
        black_box(unsafe { read_buffer_c(addr, 5) });
    }

    #[test]
    fn use_after_free_heap_rust() {
        let buf1 = Box::new([0, 1, 2, 3, 4]);
        let addr = buf1.as_ptr();
        drop(buf1);
        black_box(unsafe { read_buffer(addr, 5) });
    }

    #[test]
    fn use_after_free_heap_c() {
        let buf1 = Box::new([0, 1, 2, 3, 4]);
        let addr = buf1.as_ptr();
        drop(buf1);
        black_box(unsafe { read_buffer_c(addr, 5) });
    }

    /*
    #[test]
    fn test_use_after_free() {
        const SIZE: usize = 10;
        let mut data = allocate_buffer(SIZE);
        assert!(!data.is_null());
        for i in 0..SIZE {
            println!("data[{}] is {}", i, unsafe { *data });
            data = unsafe { data.add(1) };
        }
    }
     */
}

#[cfg(kani)]
#[kani::proof]
fn check_use_after_free() {
    let size = kani::any();
    let mut data = allocate_buffer(size);
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
