//# fontconfig = "0.2.0"
//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

fn main() {
    use fontconfig::Fontconfig;
    let fc = Fontconfig::new().unwrap();
    let font = fc.find("sans-serif", Some("Bold")).unwrap();
    println!("Name: {}", font.name);
    println!("Path: {:?}", font.path);
}
