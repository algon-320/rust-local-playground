//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

use std::iter::Peekable;
fn numeric<I>(itr: &mut Peekable<I>) -> Option<i64>
where
    I: Iterator<Item = char>,
{
    let mut tmp = None;
    while let Some(d) = itr.peek().and_then(|c| c.to_digit(10)) {
        itr.next().unwrap();
        tmp = Some(tmp.unwrap_or(0i64) * 10 + d as i64);
    }
    tmp
}

fn main() {
    let s = "1;23;;";
    let mut itr = s.chars().peekable();
    let x = numeric(&mut itr);
    assert_eq!(x, Some(1));
    assert_eq!(itr.next(), Some(';'));
    let x = numeric(&mut itr);
    assert_eq!(x, Some(23));
    assert_eq!(itr.next(), Some(';'));
    let x = numeric(&mut itr);
    assert_eq!(x, None);
    assert_eq!(itr.next(), Some(';'));
    let x = numeric(&mut itr);
    assert_eq!(x, None);
}
