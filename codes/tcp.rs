//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

fn main() {
    use std::net::{TcpListener, TcpStream};

    const BIND_ADDR: &str = "localhost:12345";

    match std::env::args().nth(1).as_deref() {
        // $ cargo play tcp.rs -- server
        Some("server") => {
            let listener = TcpListener::bind(BIND_ADDR).expect("bind");
            let (mut stream, _) = listener.accept().expect("accept");
            send(&mut stream, "line 1");
            send(&mut stream, "line 2");
        }

        // $ cargo play tcp.rs -- client
        Some("client") => {
            let stream = TcpStream::connect(BIND_ADDR).expect("connect");
            let mut br = std::io::BufReader::new(stream);
            receive(&mut br);
            receive(&mut br);
        }

        _ => panic!("<server|client>"),
    }
}

fn send<W: std::io::Write>(mut writer: W, msg: &str) {
    writer.write_all(msg.as_bytes()).expect("write_all");
    writer.write_all(b"\n").expect("write_all");
    writer.flush().expect("flush");
}

fn receive<R: std::io::BufRead>(mut reader: R) {
    let mut line = String::new();
    let res = reader.read_line(&mut line).expect("read_line");
    dbg!(res, line);
}
