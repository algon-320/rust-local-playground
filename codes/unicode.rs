//# unicode-normalization = "0.1.8"

fn main() {
    let s = "がか\u{3099}\u{00A0}\u{3099}";
    println!("{}: {:?}", s, s);
    let s = "ぞそ\u{3099}\u{00A0}\u{3099}";
    println!("{}: {:?}", s, s);
    let s = "に\u{3099}\u{00A0}\u{3099}";
    println!("{}: {:?}", s, s);

    println!();
    let c = unicode_normalization::char::compose('か', '\u{3099}').unwrap();
    println!("{}: {:?}", c, c);
}
