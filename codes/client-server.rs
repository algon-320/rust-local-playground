mod rpc {
    use std::collections::HashMap;
    use std::sync::mpsc;

    struct Request {
        id: u32,
        body: RequestBody,
    }

    enum RequestBody {
        Add(i32, i32),
        Disconnect,
    }

    enum Response {
        Answer(i32),
    }

    pub struct Server {
        request_rx: mpsc::Receiver<Request>,
        request_tx: mpsc::Sender<Request>,
        clients: HashMap<u32, mpsc::Sender<Response>>,
        next_client_id: u32,
    }

    impl Server {
        pub fn new() -> Server {
            let (tx, rx) = mpsc::channel();
            Server {
                request_tx: tx,
                request_rx: rx,
                clients: HashMap::new(),
                next_client_id: 1,
            }
        }

        pub fn new_client(&mut self) -> Client {
            let id = self.next_client_id;
            self.next_client_id += 1;

            let (tx, rx) = mpsc::channel();

            self.clients.insert(id, tx);

            Client {
                id,
                request_tx: self.request_tx.clone(),
                response_rx: rx,
            }
        }

        pub fn start(mut self) {
            loop {
                if self.clients.is_empty() {
                    break;
                }

                let req = self.request_rx.recv().unwrap();

                if !self.clients.contains_key(&req.id) {
                    continue;
                }

                match req.body {
                    RequestBody::Add(lhs, rhs) => {
                        let sum = self.add(lhs, rhs);
                        let resp = Response::Answer(sum);

                        let response_tx = self.clients.get(&req.id).unwrap();
                        response_tx.send(resp).unwrap();
                    }

                    RequestBody::Disconnect => {
                        self.clients.remove(&req.id);
                    }
                }
            }
        }

        fn add(&mut self, lhs: i32, rhs: i32) -> i32 {
            lhs + rhs
        }
    }

    pub struct Client {
        id: u32,
        request_tx: mpsc::Sender<Request>,
        response_rx: mpsc::Receiver<Response>,
    }

    impl Client {
        pub fn add(&self, lhs: i32, rhs: i32) -> i32 {
            let req = Request {
                id: self.id,
                body: RequestBody::Add(lhs, rhs),
            };
            self.request_tx.send(req).unwrap();

            if let Response::Answer(ans) = self.response_rx.recv().unwrap() {
                ans
            } else {
                unreachable!();
            }
        }
    }

    impl Drop for Client {
        fn drop(&mut self) {
            let _ = self.request_tx.send(Request {
                id: self.id,
                body: RequestBody::Disconnect,
            });
        }
    }
}

fn main() {
    let mut server = rpc::Server::new();

    let c1 = server.new_client();
    std::thread::spawn(move || {
        println!("c1: 1+2={}", c1.add(1, 2));
    });

    let c2 = server.new_client();
    std::thread::spawn(move || {
        println!("c2: 3+4={}", c2.add(3, 4));
    });

    server.start();
}
