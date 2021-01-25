//# serde = { version = "1.0", features = ["derive"] }
//# serde_json = "1.0"
//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Heavy(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FooRef<'a> {
    x: &'a str,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FooOwn {
    x: String,
}

fn main() {
    let h = Heavy("heavy".into());
    let f = FooRef { x: &h.0 };
    dbg!(serde_json::to_string(&f));

    let h = Heavy("heavy".into());
    let f = FooOwn { x: h.0 };
    dbg!(serde_json::to_string(&f));
}
