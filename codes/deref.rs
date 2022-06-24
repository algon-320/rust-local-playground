//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

#[derive(Debug)]
struct X;

impl std::ops::Deref for X {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &123
    }
}

fn main() {
    let x = X;
    let val: &i32 = &x;
    assert_eq!(&123_i32, val);
}
