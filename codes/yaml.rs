//# serde_yaml = "0.8"
//# serde = { version = "1", features = ["derive"] }
//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum Command {
    Foo(usize),
}

fn main() {
    println!("Wellcome to the playground!");
    let cmd = Command::Foo(123);
    println!("{}", serde_yaml::to_string(&cmd).unwrap());
}
