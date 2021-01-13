//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]
//! ref: https://doc.rust-lang.org/stable/reference/expressions/path-expr.html

mod x {
    pub struct A;
    pub struct B();
    pub struct C {}
    pub enum D {}
    pub enum E {
        F,
        G(),
        H(i32),
    }
}
mod y {
    pub struct A;
}

fn main() {
    // let _x = a;
    // let _y = x;
    let _x_a: x::A = x::A;
    let _x_b: fn() -> x::B = x::B;
    // let _x_c  = x::C;
    // let _x_d = x::D;
    // let _x_e = x::E;
    let _x_e_f: x::E = x::E::F;
    let _x_e_g: fn() -> x::E = x::E::G;
    let _x_e_h: fn(i32) -> x::E = x::E::H;
}
