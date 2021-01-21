//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

trait Foo {
    type T;
    fn foo(&self, t: Self::T);
}

struct Bar;
impl Foo for Bar {
    type T = i32;
    fn foo(&self, t: Self::T) {
        dbg!(t);
    }
}
// impl Foo for Bar {
//     type T = &'static str;
//     fn foo(&self, t: Self::T) {
//         dbg!(t);
//     }
// }
fn main() {
    println!("Wellcome to the playground!");
    let b = Bar;
    b.foo(123);
    // b.foo("bar");
}
