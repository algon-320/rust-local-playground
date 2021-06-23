//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

enum A {
    B(B),
}

enum B {
    C(C),
}

struct C;

// impl From<B> for A {
//     fn from(b: B) -> A {
//         A::B(b)
//     }
// }

impl From<C> for B {
    fn from(c: C) -> B {
        B::C(c)
    }
}

impl<T: Into<B>> From<T> for A {
    fn from(x: T) -> Self {
        A::B(Into::<B>::into(x))
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
