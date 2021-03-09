//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

fn main() {
    let s = &[1, 2, 3, 4, 5];
    assert_eq!(s.len(), 5);
    let mut it = s.iter();
    assert_eq!(it.as_slice(), s);
    it.next().unwrap();
    assert_eq!(it.as_slice(), &s[1..]);
    it.next().unwrap();
    assert_eq!(it.as_slice(), &s[2..]);
}
