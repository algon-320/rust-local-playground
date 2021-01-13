//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]
//! 構造体メンバの所有権
//! ref: https://doc.rust-lang.org/stable/reference/expressions/field-expr.html

#[derive(Debug, Clone)]
struct NonCopiableStruct;
impl NonCopiableStruct {
    fn consume(self) {
        println!("consumed: {:?}", self);
    }
}

#[derive(Debug, Clone)]
struct Foo {
    x: NonCopiableStruct,
    y: NonCopiableStruct,
}

#[derive(Debug, Clone)]
struct Bar {
    x: NonCopiableStruct,
    y: NonCopiableStruct,
}
impl Drop for Bar {
    fn drop(&mut self) {
        println!("dropped: {:?}", self);
    }
}

fn main() {
    {
        let f = Foo {
            x: NonCopiableStruct,
            y: NonCopiableStruct,
        };
        f.x.consume();
        f.y.consume();
        // this is the same as following:
        let f = Foo {
            x: NonCopiableStruct,
            y: NonCopiableStruct,
        };
        let Foo { x, y } = f;
        x.consume();
        y.consume();
    }

    {
        let b = Bar {
            x: NonCopiableStruct,
            y: NonCopiableStruct,
        };
        // error
        // b.x.consume();
        // b.y.consume();
    }
}
