//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

fn main() {
    use std::net::{SocketAddr, ToSocketAddrs};
    let addrs = "localhost:8000".to_socket_addrs().unwrap();
    let mut v4_addr = None;
    for addr in addrs {
        match addr {
            SocketAddr::V4(v4) => {
                v4_addr = Some(v4);
            }
            _ => continue,
        }
    }
    dbg!(v4_addr);
}
