//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

#[derive(Debug)]
struct Foo {
    num: Option<i32>,
    text: Option<String>,
    bytes: Vec<u8>,
}

impl Foo {
    fn new() -> Self {
        Self {
            num: None,
            text: None,
            bytes: Vec::new(),
        }
    }

    fn set_num(mut self, num: i32) -> Self {
        println!("\t{:p}", &self);
        self.num = Some(num);
        self
    }
    fn set_text(mut self, text: String) -> Self {
        println!("\t{:p}", &self);
        self.text = Some(text);
        self
    }
    fn set_bytes(mut self, bytes: Vec<u8>) -> Self {
        println!("\t{:p}", &self);
        self.bytes = bytes;
        self
    }
}

fn main() {
    println!("size_of::<Foo>() = 0x{:x}", std::mem::size_of::<Foo>());
    println!("---------------");

    let foo = Foo::new();
    println!("{:p}: {:?}", &foo, foo);

    let foo = foo
        .set_num(123)
        .set_text("hello".into())
        .set_bytes(vec![4, 5, 6]);
    println!("{:p}: {:?}", &foo, foo);

    println!("---------------");

    let foo = Foo::new();
    println!("{:p}: {:?}", &foo, foo);
    let foo = foo.set_num(123);
    println!("{:p}: {:?}", &foo, foo);
}
