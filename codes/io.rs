//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

fn main() {
    println!("Wellcome to the playground!");

    let b = b"hello\nworld\n";
    use std::io::{BufRead, BufReader};
    {
        let mut reader = &b[..];

        let mut s = String::new();
        let sz = reader.read_line(&mut s).unwrap();
        dbg!(sz, s);

        let mut s = String::new();
        let sz = reader.read_line(&mut s).unwrap();
        dbg!(sz, s);

        let mut s = String::new();
        let sz = reader.read_line(&mut s).unwrap();
        dbg!(sz, s);
    }
}
