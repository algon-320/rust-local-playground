//# actix = "0.13.0"
//# tokio = { version = "1.17.0", features = ["full"] }
//# tokio-util = { version = "0.7.1", features = ["codec"] }
//# futures = "0.3.21"
//# lazy_static = "1.4.0"
//# pin-project = "1.0.10"
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

    struct Tcp {
        writer: LineWriter,
    }
    impl Actor for Tcp {
        type Context = Context<Self>;
    }

    impl Tcp {
        fn spawn(stream: TcpStream) -> Addr<Tcp> {
            let (reader, writer) = stream.into_split();
            let mut ctx = Context::new();

            let reader = LineReader::new(reader, LinesCodec::default());
            let reader = reader.map(|line| line.expect("codec"));
            Tcp::add_stream(reader, &mut ctx);

            let writer = LineWriter::new(writer, LinesCodec::default(), &mut ctx);

            let tcp = Tcp { writer };

            ctx.run(tcp)
        }
    }

    impl StreamHandler<String> for Tcp {
        fn handle(&mut self, line: String, ctx: &mut Self::Context) {
            println!("{}", line);
            self.writer.write(line);
        }
    }
    impl WriteHandler<LinesCodecError> for Tcp {}

    pub async fn main() {
        use std::str::FromStr;
        let addr = SocketAddr::from_str("127.0.0.1:12121").unwrap();

        println!("listening on {}", addr);
        let listener = TcpListener::bind(addr).await.expect("bind");

        while let Ok((stream, _)) = listener.accept().await {
            actix::spawn(async move {
                let addr = stream.peer_addr().unwrap();
                println!("new connection: {}", addr);

                Tcp::spawn(stream);
            });
        }
    }
}

mod lifecycle {
    use actix::prelude::*;

    #[derive(Debug)]
    struct MyActor {
        initial: usize,
        remaining: usize,
    }
    impl MyActor {
        fn new(count: usize) -> Self {
            Self {
                initial: count,
                remaining: count,
            }
        }
    }

    impl Actor for MyActor {
        type Context = Context<Self>;

        fn started(&mut self, ctx: &mut Self::Context) {
            println!("{:?} started", self);
        }

        fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
            println!("{:?} stopping", self);
            if self.remaining > 0 {
                self.remaining -= 1;
                Running::Continue
            } else {
                Running::Stop
            }
        }

        fn stopped(&mut self, ctx: &mut Self::Context) {
            println!("{:?} stopped", self);
        }
    }

    impl Drop for MyActor {
        fn drop(&mut self) {
            println!("{:?} dropped", self);
        }
    }

    impl Supervised for MyActor {
        fn restarting(&mut self, ctx: &mut Self::Context) {
            println!("{:?} restaring", self);
            self.remaining = self.initial;
        }
    }

    #[derive(Message)]
    #[rtype(result = "()")]
    struct Die;

    impl Handler<Die> for MyActor {
        type Result = ();
        fn handle(&mut self, die: Die, ctx: &mut Self::Context) {
            ctx.stop();
        }
    }

    pub async fn main() {
        let ator = Supervisor::start(|_| MyActor::new(2));

        let _ = actix::spawn(async move {
            let dur = std::time::Duration::from_secs(1);
            let mut interval = actix::clock::interval(dur);
            interval.tick().await;

            for _ in 0..10 {
                interval.tick().await;
                ator.send(Die).await.unwrap();
            }

            interval.tick().await;
        })
        .await;
    }
}

mod my_registry {
    use actix::prelude::*;

    mod lazy_registry {
        use actix::{Actor, Addr, Supervised};

        use std::any::{Any, TypeId};
        use std::collections::HashMap;
        use tokio::sync::watch;

        type AddrAny = Box<dyn Any + Send + Sync>;

        enum State {
            Waiting {
                tx: watch::Sender<Option<AddrAny>>,
                rx: watch::Receiver<Option<AddrAny>>,
            },
            Registered(AddrAny),
        }

        impl Default for State {
            fn default() -> Self {
                let (tx, rx) = watch::channel(None);
                Self::Waiting { tx, rx }
            }
        }

        use std::sync::Mutex;
        lazy_static::lazy_static! {
            static ref REGISTRY: Mutex<HashMap<TypeId, State>> = {
                Mutex::new(HashMap::new())
            };
        }

        pub async fn lookup<A>() -> Addr<A>
        where
            A: Actor,
        {
            let actor_id = TypeId::of::<A>();

            let mut rx = {
                // There is no `await` in this block
                // NOTE: awaiting some future while having synchronous locks
                //       (such as std::sync::Mutex) can cause deadlock.
                let mut registry = REGISTRY.lock().expect("poisoned");
                let state = registry.entry(actor_id).or_default();
                match state {
                    State::Registered(any) => {
                        let addr: &Addr<A> = any.downcast_ref().expect("it's not an Addr<A>");
                        return addr.clone();
                    }
                    State::Waiting { rx, .. } => rx.clone(),
                }
            };

            // Wait for A' address
            while rx.borrow().is_none() {
                rx.changed().await.expect("sender dropped");
            }

            // Now the watch buffer should be updated into a legitimate address
            let option_any = rx.borrow();
            let addr: &Addr<A> = option_any
                .as_ref()
                .expect("changed but None")
                .downcast_ref()
                .expect("it's not an Addr<A>");

            addr.clone()
        }

        pub fn register<A>(addr: Addr<A>)
        where
            A: Actor + Supervised,
        {
            let actor_id = TypeId::of::<A>();

            let mut registry = REGISTRY.lock().expect("poisoned");
            let state = registry.entry(actor_id).or_default();

            match state {
                State::Registered(_) => {
                    panic!("An address of `A` has already been registered.");
                }

                State::Waiting { .. } => {
                    let addr_any: AddrAny = Box::new(addr.clone());
                    let new_state = State::Registered(addr_any);
                    let old_state = std::mem::replace(state, new_state);

                    if let State::Waiting { tx, .. } = old_state {
                        let addr_any: AddrAny = Box::new(addr);
                        // Report the addr to all waiters
                        tx.send_replace(Some(addr_any));
                    } else {
                        unreachable!();
                    }
                }
            }
        }
    }

    enum Expr {
        Add(i32, i32),
        Sub(i32, i32),
        Div(i32, i32),
    }
    impl Message for Expr {
        type Result = i32;
    }

    struct Calculator;
    impl Actor for Calculator {
        type Context = Context<Self>;
    }
    impl Supervised for Calculator {
        fn restarting(&mut self, _: &mut Self::Context) {
            println!("restarting");
        }
    }

    impl Handler<Expr> for Calculator {
        type Result = i32;
        fn handle(&mut self, expr: Expr, ctx: &mut Self::Context) -> Self::Result {
            match expr {
                Expr::Add(lhs, rhs) => lhs + rhs,
                Expr::Sub(lhs, rhs) => lhs - rhs,
                Expr::Div(lhs, rhs) => {
                    if rhs == 0 {
                        ctx.stop();
                        0
                    } else {
                        lhs / rhs
                    }
                }
            }
        }
    }

    struct Double;
    impl Actor for Double {
        type Context = Context<Self>;
    }

    impl Handler<Expr> for Double {
        type Result = ResponseFuture<i32>;
        fn handle(&mut self, expr: Expr, ctx: &mut Self::Context) -> Self::Result {
            Box::pin(async {
                let addr = lazy_registry::lookup::<Calculator>().await;
                let res = addr.send(expr).await.unwrap();
                res * 2
            })
        }
    }

    pub async fn main() {
        use actix::clock::sleep;
        use std::time::Duration;

        let task1 = actix::spawn(async move {
            sleep(Duration::from_secs(2)).await;

            let addr = Supervisor::start(|_| Calculator);
            lazy_registry::register(addr.clone());
        });

        let task2 = actix::spawn(async move {
            let addr = lazy_registry::lookup::<Calculator>().await;
            let res = addr.send(Expr::Add(1, 2)).await;
            assert_eq!(res.unwrap(), 3);
            let _ = dbg!(res);
        });

        let task3 = actix::spawn(async move {
            let double = Double.start();

            let res = double.send(Expr::Add(3, 4)).await;
            assert_eq!(res.unwrap(), 14);
            let _ = dbg!(res);

            let res = double.send(Expr::Sub(10, 6)).await;
            assert_eq!(res.unwrap(), 8);
            let _ = dbg!(res);

            let res = double.send(Expr::Div(10, 0)).await;
            let _ = dbg!(res);
        });

        let _ = task1.await;
        let _ = task2.await;
        let _ = task3.await;
    }
}

mod mutual_ref {
    use actix::prelude::*;

    mod drop_future {
        use tokio::sync::oneshot;

        #[pin_project::pin_project]
        struct DropFuture {
            #[pin]
            rx: oneshot::Receiver<()>,
        }
        impl std::future::Future for DropFuture {
            type Output = ();
            fn poll(
                self: std::pin::Pin<&mut Self>,
                ctx: &mut std::task::Context,
            ) -> std::task::Poll<()> {
                self.project().rx.poll(ctx).map(|res| res.unwrap_or(()))
            }
        }
        pub struct DropFutureAnchor {
            tx: Option<oneshot::Sender<()>>,
        }
        impl DropFutureAnchor {
            fn drop(&mut self) {
                let _ = self.tx.take().unwrap().send(());
            }
        }
        pub fn pair() -> (DropFutureAnchor, impl std::future::Future<Output = ()>) {
            let (tx, rx) = oneshot::channel();
            let anchor = DropFutureAnchor { tx: tx.into() };
            let fut = DropFuture { rx };
            (anchor, fut)
        }
    }
    use drop_future::DropFutureAnchor;

    struct Actor1 {
        peer: Addr<Actor2>,
        _anchor: DropFutureAnchor,
    }
    impl Actor for Actor1 {
        type Context = Context<Self>;
    }

    struct Actor2 {
        peer: Addr<Actor1>,
        _anchor: DropFutureAnchor,
    }
    impl Actor for Actor2 {
        type Context = Context<Self>;
    }

    #[derive(Message)]
    #[rtype(result = "()")]
    struct Foo(isize);

    impl Handler<Foo> for Actor1 {
        type Result = ();
        fn handle(&mut self, Foo(x): Foo, ctx: &mut Self::Context) {
            println!("Actor1: {}", x);
            if x == 0 {
                ctx.stop();
                self.peer.do_send(Foo(0));
            } else {
                self.peer.do_send(Foo(x - 1));
            }
        }
    }
    impl Handler<Foo> for Actor2 {
        type Result = ();
        fn handle(&mut self, Foo(x): Foo, ctx: &mut Self::Context) {
            println!("Actor2: {}", x);
            if x == 0 {
                ctx.stop();
                self.peer.do_send(Foo(0));
            } else {
                self.peer.do_send(Foo(x - 1));
            }
        }
    }

    pub async fn main() {
        let c1 = Context::new();
        let c2 = Context::new();

        let (anchor, a1_drop) = drop_future::pair();
        let a1 = Actor1 {
            peer: c2.address(),
            _anchor: anchor,
        };
        let (anchor, a2_drop) = drop_future::pair();
        let a2 = Actor2 {
            peer: c1.address(),
            _anchor: anchor,
        };

        let addr1 = c1.run(a1);
        c2.run(a2);

        addr1.do_send(Foo(5));

        let _ = a1_drop.await;
        let _ = a2_drop.await;
    }
}

#[actix::main]
async fn main() {
    mutual_ref::main().await;
}
