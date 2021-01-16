//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

fn curry<'a, 'b, F, X, Y, Z>(f: F, x: X) -> impl Fn(Y) -> Z
where
    X: 'a + Copy,
    Y: 'b,
    'a: 'b,
    F: Fn(X, Y) -> Z,
{
    move |y: Y| f(x, y)
}

fn concat1<'a, 'b>(s: &'a String, t: &'b String) -> String
where
    'a: 'b,
{
    format!("{}{}", s, t)
}

fn concat2<'a, 'b>(s: &'a str, t: &'b str) -> String
where
    'a: 'b,
{
    format!("{}{}", s, t)
}

fn main() {
    // type S = String;
    // let cat = concat1;

    type S = &'static str;
    let cat = concat2;

    let s: S = "hello, ".into();
    let hello = curry(cat, &s);
    {
        let t: S = "world".into();
        let r: String = hello(&t);
        dbg!(r);
    }
    {
        let t: S = "rustaceans".into();
        let r: String = hello(&t);
        dbg!(r);
    }
}
