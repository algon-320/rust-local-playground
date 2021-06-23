//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

enum A {
    B(B),
}

enum B {
    C(C),
}

struct C;

impl From<B> for A {
    fn from(b: B) -> A {
        A::B(b)
    }
}

impl From<C> for B {
    fn from(c: C) -> B {
        B::C(c)
    }
}

fn main() {
    let c = C;
    let b = B::from(c);
    let a = A::from(b);

    let c = C;
    let a = A::from(c);
    println!("Wellcome to the playground!");
}
