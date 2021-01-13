//# rayon = "1.5"
//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]
//! いい感じに並列化してくれるrayon

use rayon::prelude::*;

fn main() {
    let v: Vec<u64> = (0..10).map(|x| x + 1).collect();
    let s: u64 = v
        //.into_iter() // single threaded
        .into_par_iter() // automatically multi-threaded
        .map(|x| {
            println!(
                "running on the thread with id {:?}",
                std::thread::current().id()
            );
            std::thread::sleep(std::time::Duration::new(x, 0));
            x
        })
        .sum();
    dbg!(s);
}
