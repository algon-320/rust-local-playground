//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

mod named {
    pub trait Named {
        fn name(&self) -> String;
    }

    struct Foo;
    impl Named for Foo {
        fn name(&self) -> String {
            "Foo".to_owned()
        }
    }

    struct Bar;
    impl Named for Bar {
        fn name(&self) -> String {
            "Bar".to_owned()
        }
    }

    pub fn f() -> impl Named {
        Foo
    }
    pub fn g() -> impl Named {
        Bar
    }
}

use named::{f, g, Named};

fn h(is_foo: bool) -> Box<dyn Named> {
    if is_foo {
        Box::new(f())
    } else {
        Box::new(g())
    }
}

fn show<T>(val: &T)
where
    T: Named + ?Sized,
{
    println!("{}", val.name());
}

struct Wrap<T: Named + ?Sized>(Box<T>);
impl<T: Named + ?Sized> Wrap<T> {
    fn show(&self) {
        println!("{}", self.0.name());
    }
}

fn main() {
    let x = h(true);
    show(&*x);

    let x = g();
    show(&x);

    let x = Wrap(h(false));
    x.show();

    let x = Wrap(Box::new(f()));
    x.show();
}
