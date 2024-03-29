//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

use core::mem::MaybeUninit;
struct FixedVec<T, const CAPACITY: usize> {
    buf: MaybeUninit<[T; CAPACITY]>,
    len: usize,
}
impl<T, const CAPACITY: usize> FixedVec<T, CAPACITY> {
    pub const fn new() -> Self {
        Self {
            buf: MaybeUninit::uninit(),
            len: 0,
        }
    }
    pub fn capacity(&self) -> usize {
        CAPACITY
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.len {
            let p = self.buf.as_ptr();
            Some(unsafe { &*(p as *const T).add(idx) })
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx < self.len {
            let p = self.buf.as_mut_ptr();
            Some(unsafe { &mut *(p as *mut T).add(idx) })
        } else {
            None
        }
    }
    pub fn push(&mut self, val: T) {
        assert!(self.len < CAPACITY);
        let p = self.buf.as_mut_ptr();
        unsafe { (p as *mut T).add(self.len).write(val) };
        self.len += 1;
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            let p = self.buf.as_mut_ptr();
            Some(unsafe { (p as *const T).add(self.len).read() })
        } else {
            None
        }
    }
}

static mut FIXED_VEC: FixedVec<i32, 10> = FixedVec::new();

fn main() {
    unsafe {
        assert_eq!(FIXED_VEC.len(), 0);

        FIXED_VEC.push(1);
        assert_eq!(FIXED_VEC.len(), 1);

        FIXED_VEC.push(2);
        assert_eq!(FIXED_VEC.len(), 2);

        let x = FIXED_VEC.pop();
        assert_eq!(FIXED_VEC.len(), 1);
        assert_eq!(x, Some(2));

        let x = FIXED_VEC.pop();
        assert_eq!(FIXED_VEC.len(), 0);
        assert_eq!(x, Some(1));

        let x = FIXED_VEC.pop();
        assert_eq!(FIXED_VEC.len(), 0);
        assert_eq!(x, None);

        FIXED_VEC.push(0);
        FIXED_VEC.push(1);
        FIXED_VEC.push(2);
        FIXED_VEC.push(3);
        FIXED_VEC.push(4);
        assert_eq!(FIXED_VEC.len(), 5);
        assert_eq!(FIXED_VEC.get(0), Some(&0));
        assert_eq!(FIXED_VEC.get(4), Some(&4));
        assert_eq!(FIXED_VEC.get(5), None);

        *FIXED_VEC.get_mut(3).unwrap() += 10;
        assert_eq!(FIXED_VEC.get(3), Some(&13));
    }
    println!("ok!");
}
