//---- Put dependencies above ----
#![allow(dead_code, unused_variables)]

#[derive(Debug, Clone)]
struct Foo;

#[derive(Debug)]
struct Bar(Foo);

impl Drop for Bar {
    fn drop(&mut self) {
        println!("bar dropped");
    }
}

fn main() {
    {
        let bar = Bar(Foo);
        dbg!(&bar);

        // let Bar(foo) = bar; // Error!
        // dbg!(&foo);
    }

    {
        let bar = Bar(Foo);
        dbg!(&bar);

        let foo = bar.0.clone();
        dbg!(&foo);

        std::mem::forget(bar);
    }
}
