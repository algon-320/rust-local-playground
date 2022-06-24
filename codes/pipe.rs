//# nix = "0.24.1"
//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

use nix::unistd::{pipe, read, write};
use std::os::unix::io::RawFd;

fn another(fd: RawFd) {
    println!("another: waiting...");
    let mut buf = vec![0_u8; 0x1000];
    let nb = read(fd, &mut buf[..]).unwrap();
    println!("another: {:?}", &buf[..nb]);
}

fn main() {
    let (rfd, wfd) = pipe().unwrap();

    let nb = write(wfd, b"abcdef").unwrap();
    println!("main: send {} bytes", nb);
    std::thread::sleep(std::time::Duration::from_secs(1));

    another(rfd);

    // let handle = std::thread::spawn(move || another(rfd));
    // handle.join().unwrap();
}
