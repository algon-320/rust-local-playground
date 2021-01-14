//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

use std::io::prelude::*;
use std::io::{BufRead, BufReader, BufWriter};
use std::net::TcpStream;

fn main() {
    let mut stream = BufWriter::new(TcpStream::connect("127.0.0.1:34254").unwrap());
    for i in 0..10 {
        stream.write(format!("hello {}\n", i).as_bytes()).unwrap();
        dbg!(stream.buffer().len());
    }
    stream.flush().unwrap();

    let mut rd = BufReader::new(stream.get_ref());
    let mut line = String::new();
    rd.read_line(&mut line).unwrap();
    dbg!(line);
}
