//---- Put dependencies above ----
#![allow(dead_code, unused_variables)]

fn some_mut(v: &mut Vec<i32>) -> Option<&mut i32> {
    todo!()
    // if let Some(x) = v.get_mut(1) {
    //     return Some(x);
    // }
    // v.get_mut(0)
}

fn some_mut_slice(v: &mut Vec<i32>) -> Option<&mut i32> {
    let sl = v.as_mut_slice();

    let (first, sl) = sl.split_first_mut()?;
    let second = sl.split_first_mut().map(|(x, xs)| x);
    Some(second.unwrap_or(first))
}

fn main() {
    let mut v = vec![1, 2];
    dbg!(some_mut_slice(&mut v));
}
