//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

struct Foo;

pub mod a {
    use super::Foo;

    pub mod b {
        use super::Foo;

        pub fn foo() {
            let _ = Foo;
        }
    }
}

fn main() {
    println!("Wellcome to the playground!");

    let _ = a::b::foo();
}
