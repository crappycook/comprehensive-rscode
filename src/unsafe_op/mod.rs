#![allow(unused)]
mod ffi_wrapper;

#[test]
fn test_raw_pointers() {
    let mut s = String::from("value");
    let r1 = &mut s;
    let r2 = r1 as *const String;
    unsafe {
        println!("{}", *r1);
        *r1 = String::from("new val");
        println!("r2 is: {}", *r2);
    }

    let mut i = 10;
    let i1 = &mut i;
    let i2 = i1 as *mut i32;
    unsafe {
        // raw pointer dereferences
        *i2 = 20;
        assert_eq!(*i2, 20);
        assert_eq!(*i1, 20);
    }
}

extern "C" {
    // Absolute value of -3 according to C
    fn abs(input: i32) -> i32;
}

#[test]
fn test_unsafe_extern() {
    unsafe {
        // Undefined behavior if abs misbehaves.
        let res = abs(-3);
        assert_eq!(res, 3);
    }
}

/// Swaps the values pointed to by the given pointers.
///
/// # Safety
///
/// The pointers must be valid and properly aligned.
unsafe fn swap(x: &mut i32, y: &mut i32) {
    let tmp = *x;
    *x = *y;
    *y = tmp;
}

#[test]
fn test_unsafe_swap() {
    let mut x = 1;
    let mut y = 2;
    unsafe {
        swap(&mut x, &mut y);
    }
    assert_eq!(x, 2);
    assert_eq!(y, 1);
}
