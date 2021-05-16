//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

mod unit {
    pub struct A;
    pub struct B();
    pub struct C(());
    pub struct Cp(pub ());
    pub struct D {}
    pub struct E { e: () }
    pub struct Ep { pub e: () }
}

fn main() {
    let a = unit::A;
    let b = unit::B();
//  let c = unit::C(()); // error
    let c = unit::Cp(());
    let d = unit::D {};
//  let e = unit::E { e: () }; // error
    let e = unit::Ep { e: () };
}
