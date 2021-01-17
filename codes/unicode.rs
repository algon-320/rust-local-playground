//# unicode-normalization = "0.1.8"
//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

fn main() {
    println!("Wellcome to the playground!");
    let msg = "ヘ\u{3099}ストアルハ\u{3099}ム";
    println!("{}", msg);
    {
        use unicode_normalization::UnicodeNormalization;
        let msg_norm = msg.chars().nfc().collect::<String>();
        println!("{}", msg_norm);
    }

    let s = "が";
    println!("{:?} ==> {:?}", s, s.as_bytes());
    let s = "か\u{3099}";
    println!("{:?} ==> {:?}", s, s.as_bytes());
    let s = "か";
    println!("{:?} ==> {:?}", s, s.as_bytes());

    let c = unicode_normalization::char::compose('か', '\u{3099}');
    println!("{:?}", c);
}
