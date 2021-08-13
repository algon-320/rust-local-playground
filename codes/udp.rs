//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

use std::io::stdin;
use std::net::{ToSocketAddrs, UdpSocket};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("bind address:");
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    let listen = buf
        .trim()
        .to_socket_addrs()
        .ok()
        .and_then(|mut itr| itr.next())
        .unwrap();
    let sock = UdpSocket::bind(listen)?;
    println!("sock = {:?}", sock.local_addr());

    println!("destination address:");
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    if let Some(dst) = buf
        .trim()
        .to_socket_addrs()
        .ok()
        .and_then(|mut itr| itr.next())
    {
        sock.send_to(b"hello,world!", dst)?;
        println!("Sent to {:?}", dst);
    }

    let mut buf = vec![0; 1024];
    let (sz, from) = sock.recv_from(&mut buf)?;
    println!("Received from {:?}: {:?}", from, &buf[..sz]);

    Ok(())
}
