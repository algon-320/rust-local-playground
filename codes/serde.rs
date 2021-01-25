//# serde = { version = "1.0", features = ["derive"] }
//# serde_json = "1.0"
//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Heavy(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FooRef<'a> {
    x: Cow<'a, Heavy>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FooOwn {
    x: Heavy,
}

use std::collections::HashSet;
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Set {
    c: HashSet<i32>,
}

fn main() {
    let h = Heavy("heavy".into());
    let f = FooRef {
        x: Cow::Borrowed(&h),
    };
    dbg!(serde_json::to_string(&f));

    let h = Heavy("heavy".into());
    let f = FooOwn { x: h };
    dbg!(serde_json::to_string(&f));

    let json = r#"{"c":[1,2,3]}"#;
    let s: Set = serde_json::from_str(json).unwrap();
    dbg!(s);
    let json = r#"{"c":[1,2,3,1,1,1,1]}"#;
    let s: Set = serde_json::from_str(json).unwrap();
    dbg!(s);
}
