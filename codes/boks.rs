//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]
#![feature(dropck_eyepatch)]

use std::marker::PhantomData;

struct MyPhantom<T>([T; 0]);
impl<T> MyPhantom<T> {
    fn new() -> Self {
        Self([])
    }
}

struct Boks<T> {
    p: *mut T,
    _t: MyPhantom<T>,
}

unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.p) };
    }
}

impl<T> Boks<T> {
    fn new(t: T) -> Self {
        Self {
            p: Box::into_raw(Box::new(t)),
            // _t: PhantomData,
            _t: MyPhantom::new(),
        }
    }
}

impl<T> std::ops::Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.p }
    }
}
impl<T> std::ops::DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.p }
    }
}

use std::fmt::Debug;
struct Foo<T: Debug>(T);
impl<T: Debug> Drop for Foo<T> {
    fn drop(&mut self) {
        let r: &mut T = &mut self.0;
        println!("dropping {:?}", r);
    }
}

fn main() {
    assert_eq!(std::mem::size_of::<MyPhantom<i32>>(), 0);

    let mut x = 123;
    let b = Boks::new(&mut x);
    println!("{:?}", x);

    let mut x = 456;
    let b = Boks::new(Foo(&mut x));
    // println!("{:?}", x);
}
