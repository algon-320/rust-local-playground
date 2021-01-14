//# serde = { version = "1.0", features = ["derive"] }
//# serde_json = "1.0"
//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Foo {
    a: String,
}

#[derive(Serialize, Deserialize)]
struct UnitLike;
impl UnitLike {
    pub fn new() -> Self {
        Self
    }
}

fn main() {
    let foo = Foo {
        a: r#""#.to_owned(),
    };
    assert_eq!(
        serde_json::to_string(&foo).unwrap(),
        r#"{"a":""}"#.to_owned()
    );

    let foo = Foo {
        a: r#"line1
line2"#
            .to_owned(),
    };
    assert_eq!(
        serde_json::to_string(&foo).unwrap(),
        r#"{"a":"line1\nline2"}"#.to_owned()
    );

    let ul = UnitLike::new();
    dbg!(serde_json::to_string(&ul));
}
