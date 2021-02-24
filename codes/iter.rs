//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

fn main() {
    let v: Vec<Option<Vec<i32>>> = vec![Some(vec![1, 2, 3]), Some(vec![4, 5, 6])];
    let w: Option<Vec<i32>> = v
        .into_iter() /* ... */
        .collect();
    assert_eq!(w, Some(vec![1, 2, 3, 4, 5, 6]));

    let v: Vec<Option<Vec<i32>>> = vec![None, Some(vec![4, 5, 6])];
    let w: Option<Vec<i32>> = v
        .into_iter() /* ... */
        .collect();
    assert_eq!(w, None);
}
