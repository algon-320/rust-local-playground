//# servo-fontconfig = "0.5.1"
//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

struct FontConfig {
    fc: *mut fontconfig::fontconfig::FcConfig,
}
impl Fontconfig {
    fn new() -> Self {
        let fc = unsafe { fontconfig::fontconfig::FcInitLoadConfigAndFonts() };
        Self { fc }
    }
}
impl std::ops::Drop for FontConfig {
    fn drop(&mut self) {
        unsafe { fontconfig::fontconfig::FcFini() };
    }
}

fn main() {
    let fc = Fontconfig::new();
}
