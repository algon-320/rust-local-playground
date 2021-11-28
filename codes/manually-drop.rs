//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

use std::mem::ManuallyDrop;

struct Bar(i32);
impl Drop for Bar {
    fn drop(&mut self) {
        println!("Bar dropped: {}", self.0);
    }
}

struct Foo {
    x: ManuallyDrop<Bar>,
    y: Bar,
}

fn main() {
    let foo = Foo {
        x: ManuallyDrop::new(Bar(1)),
        y: Bar(2),
    };
    drop(foo);
}
