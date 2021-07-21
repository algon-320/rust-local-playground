//---- Put dependencies above ----
#![allow(dead_code, unused_variables)]

fn some_mut(v: &mut Vec<i32>) -> Option<&mut i32> {
    if let Some(x) = v.get_mut(1) {
        return Some(x);
    }
    v.get_mut(0)
}

fn main() {
    let mut v = vec![1, 2];
    dbg!(some_mut(&mut v));
}
