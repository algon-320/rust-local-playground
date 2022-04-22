//# actix = "0.13.0"
//# actix-rt = "2.7.0"
//# tokio = { version = "1.17.0", features = ["full"] }
//# tokio-util = { version = "0.7.1", features = ["codec"] }
//# futures = "0.3.21"
//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

mod simple {
    use actix::prelude::*;

    struct Add(usize, usize);
    impl Message for Add {
        type Result = usize;
    }

    struct Calculator;
    impl Actor for Calculator {
        type Context = Context<Self>;
    }

    impl Handler<Add> for Calculator {
        type Result = <Add as Message>::Result;
        fn handle(&mut self, expr: Add, ctx: &mut Self::Context) -> Self::Result {
            let Add(lhs, rhs) = expr;
            lhs + rhs
        }
    }

    pub async fn main() {
        let addr = Calculator.start();
        let res = addr.send(Add(1, 2)).await;
        let _ = dbg!(res);
    }
}

mod tcp {
    use actix::io::{FramedWrite, WriteHandler};
    use actix::prelude::*;

    use futures::stream::StreamExt;
    use std::net::SocketAddr;
    use tokio::net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener, TcpStream,
    };
    use tokio_util::codec::{FramedRead, LinesCodec, LinesCodecError};

    type LineReader = FramedRead<OwnedReadHalf, LinesCodec>;
    type LineWriter = FramedWrite<String, OwnedWriteHalf, LinesCodec>;

    #[derive(Message)]
    #[rtype(result = "()")]
    struct Attach(TcpStream);

    #[derive(Default)]
    struct Tcp {
        writer: Option<LineWriter>,
    }
    impl Actor for Tcp {
        type Context = Context<Self>;
    }

    impl StreamHandler<String> for Tcp {
        fn handle(&mut self, line: String, ctx: &mut Self::Context) {
            if let Some(writer) = self.writer.as_mut() {
                println!("{}", line);
                writer.write(line);
            }
        }
    }
    impl WriteHandler<LinesCodecError> for Tcp {}

    impl Handler<Attach> for Tcp {
        type Result = ();
        fn handle(&mut self, Attach(stream): Attach, ctx: &mut Self::Context) {
            let (reader, writer) = stream.into_split();

            let writer = LineWriter::new(writer, LinesCodec::default(), ctx);
            let _ = self.writer.insert(writer);

            let reader = LineReader::new(reader, LinesCodec::default());
            let reader = reader.map(|line| line.expect("codec"));
            Self::add_stream(reader, ctx);
        }
    }

    pub async fn main() {
        use std::str::FromStr;
        let addr = SocketAddr::from_str("127.0.0.1:12121").unwrap();

        println!("listening on {}", addr);
        let listener = TcpListener::bind(addr).await.expect("bind");

        while let Ok((stream, _)) = listener.accept().await {
            actix::spawn(async move {
                let addr = stream.peer_addr().unwrap();
                println!("new connection: {}", addr);

                let actor = Tcp::default().start();
                actor.send(Attach(stream)).await.expect("attach");
            });
        }
    }
}

#[actix_rt::main]
async fn main() {
    tcp::main().await;
}
