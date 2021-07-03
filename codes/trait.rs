//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

trait A {
    fn a(&self) -> i32;
}

trait B {
    fn b(&self) -> &'static str;
}

struct X;
impl A for X {
    fn a(&self) -> i32 {
        1
    }
}
impl B for X {
    fn b(&self) -> &'static str {
        "hello"
    }
}

fn foo() -> impl A {
    X
}
fn bar() -> impl A + B {
    X
}

fn main() {
    let x = foo();
    println!("{:?}", x.a());
    // println!("{:?}", x.b());

    let x = bar();
    println!("{:?}", x.a());
    println!("{:?}", x.b());
}
