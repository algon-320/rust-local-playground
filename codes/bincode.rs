//#bincode = "1.3.3"
//#serde = { version = "1", features = ["derive"] }
//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum Foo {
    A,
    B(i32),
    C(String),
}

fn main() {
    println!("Wellcome to the playground!");

    let mut buf = vec![0; 1024];

    {
        let mut writer = &mut buf[..];
        dbg!(writer.len());

        let f = Foo::A;
        bincode::serialize_into(&mut writer, &f).expect("ser");
        dbg!(writer.len());

        let f = Foo::B(123);
        bincode::serialize_into(&mut writer, &f).expect("ser");
        dbg!(writer.len());

        let f = Foo::C("Hello!".into());
        bincode::serialize_into(&mut writer, &f).expect("ser");
        dbg!(writer.len());
    }

    {
        let mut reader = &buf[..];

        let f: Foo = bincode::deserialize_from(&mut reader).expect("de");
        dbg!(f, reader.len());

        let f: Foo = bincode::deserialize_from(&mut reader).expect("de");
        dbg!(f, reader.len());

        let f: Foo = bincode::deserialize_from(&mut reader).expect("de");
        dbg!(f, reader.len());
    }
}
