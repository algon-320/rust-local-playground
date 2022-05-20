//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

fn main() {
    for point in 0x2500..=0x257F {
        let ch = char::from_u32(point).unwrap();
        println!("{}: U+{:4X}", ch, point);
    }
}
