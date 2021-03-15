//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

use std::marker::PhantomPinned;
use std::pin::Pin;

struct SelfRef {
    x: Vec<i32>,
    r: Option<*mut Vec<i32>>,
    _p: PhantomPinned,
}
impl SelfRef {
    pub fn new(x: i32) -> Pin<Box<Self>> {
        Box::pin(Self {
            x: vec![x],
            r: None,
            _p: PhantomPinned,
        })
    }
    fn init(self: Pin<&mut Self>) {
        unsafe {
            let this = self.get_unchecked_mut();
            let xref = &mut this.x as *mut _;
            this.r = Some(xref);
        }
    }
    fn use_r(self: Pin<&Self>) {
        println!("self.r = {:?}", self.r.map(|r| unsafe { &*r }));
    }
    fn use_mut_r(self: Pin<&mut Self>, y: i32) {
        if let Some(p) = self.r {
            unsafe { (*p).push(y) };
        }
    }
}

fn main() {
    let mut x = SelfRef::new(123);
    x.as_ref().use_r();
    x.as_mut().init();
    x.as_ref().use_r();
    x.as_mut().use_mut_r(456);
    x.as_ref().use_r();
}
