#![allow(dead_code)]

mod single {
    use std::sync::{Condvar, Mutex};

    struct CircularBufferState {
        /// データを保存するバッファ
        data: Vec<i32>,
        /// 読み出す位置
        rp: usize,
        /// 書き込む位置
        wp: usize,
        /// バッファ内の要素数
        used: usize,
    }
    impl CircularBufferState {
        const BUFFER_SIZE: usize = 4;

        pub fn new() -> CircularBufferState {
            CircularBufferState {
                data: vec![0; Self::BUFFER_SIZE],
                rp: 0,
                wp: 0,
                used: 0,
            }
        }

        pub fn put(&mut self, x: i32) {
            self.data[self.wp] = x;
            self.wp = (self.wp + 1) % Self::BUFFER_SIZE;
            self.used += 1;
        }

        pub fn get(&mut self) -> i32 {
            let x = self.data[self.rp];
            self.rp = (self.rp + 1) % Self::BUFFER_SIZE;
            self.used -= 1;
            x
        }

        pub fn is_empty(&self) -> bool {
            self.used == 0
        }

        pub fn is_full(&self) -> bool {
            self.used == Self::BUFFER_SIZE
        }
    }

    struct CircularBuffer {
        /// 相互排除が必要な部分
        mtx: Mutex<CircularBufferState>,
        /// バッファに空きがあることを示す状態変数
        not_full: Condvar,
        /// バッファに要素があることを示す状態変数
        not_empty: Condvar,
    }
    impl CircularBuffer {
        pub fn new() -> CircularBuffer {
            CircularBuffer {
                mtx: Mutex::new(CircularBufferState::new()),
                not_full: Condvar::new(),
                not_empty: Condvar::new(),
            }
        }

        pub fn put(&self, x: i32) {
            let mut state = self.mtx.lock().unwrap();
            while state.is_full() {
                state = self.not_full.wait(state).unwrap();
            }
            state.put(x);
            self.not_empty.notify_all();
        }

        pub fn get(&self) -> i32 {
            let mut state = self.mtx.lock().unwrap();
            while state.is_empty() {
                state = self.not_empty.wait(state).unwrap();
            }
            let x = state.get();
            self.not_full.notify_all();
            x
        }
    }

    fn thread_a(b: &CircularBuffer) {
        for x in 0..10 {
            println!("thread_a(): put( {} )", x);
            b.put(x);
        }
    }
    fn thread_b(b: &CircularBuffer) {
        for _ in 0..10 {
            let x = b.get();
            println!("thread_b(): got() {}", x);
        }
    }

    fn main() {
        // スレッド間でオブジェクトを共有するためにstd::sync::Arcで包む
        //     CircularBufferはヒープメモリ上に確保される。
        //     Arcはスレッド安全な方法で参照カウントされるポインタを表す。
        let cbuf = std::sync::Arc::new(CircularBuffer::new());

        let th_a = {
            let cbuf = cbuf.clone(); // ポインタのコピー(参照カウント+1)
            std::thread::spawn(move || thread_a(&cbuf))
        };
        let th_b = {
            let cbuf = cbuf.clone(); // ポインタのコピー(参照カウント+1)
            std::thread::spawn(move || thread_b(&cbuf))
        };

        th_a.join().unwrap();
        th_b.join().unwrap();
    }
}

mod multiple {
    use std::sync::{Condvar, Mutex};

    struct CircularBufferState {
        /// データを保存するバッファ
        data: Vec<i32>,
        /// 読み出す位置
        rp: usize,
        /// 書き込む位置
        wp: usize,
        /// バッファ内の要素数
        used: usize,
    }
    impl CircularBufferState {
        const BUFFER_SIZE: usize = 4;

        pub fn new() -> CircularBufferState {
            CircularBufferState {
                data: vec![0; Self::BUFFER_SIZE],
                rp: 0,
                wp: 0,
                used: 0,
            }
        }

        pub fn put(&mut self, x: i32) {
            self.data[self.wp] = x;
            self.wp = (self.wp + 1) % Self::BUFFER_SIZE;
            self.used += 1;
        }

        pub fn get(&mut self) -> i32 {
            let x = self.data[self.rp];
            self.rp = (self.rp + 1) % Self::BUFFER_SIZE;
            self.used -= 1;
            x
        }

        pub fn is_empty(&self) -> bool {
            self.used == 0
        }

        pub fn is_full(&self) -> bool {
            self.used == Self::BUFFER_SIZE
        }
    }

    struct CircularBuffer {
        /// 相互排除が必要な部分
        mtx: Mutex<CircularBufferState>,
        /// バッファに空きがあることを示す状態変数
        not_full: Condvar,
        /// バッファに要素があることを示す状態変数
        not_empty: Condvar,

        /// インターリーブを避けるためのロック
        mtx_put: Mutex<()>,
        mtx_get: Mutex<()>,
    }
    impl CircularBuffer {
        pub fn new() -> CircularBuffer {
            CircularBuffer {
                mtx: Mutex::new(CircularBufferState::new()),
                not_full: Condvar::new(),
                not_empty: Condvar::new(),
                mtx_put: Mutex::new(()),
                mtx_get: Mutex::new(()),
            }
        }

        pub fn put(&self, xs: &[i32]) {
            let _lock = self.mtx_put.lock().unwrap();

            let mut state = self.mtx.lock().unwrap();
            let mut i = 0;
            while i < xs.len() {
                while state.is_full() {
                    state = self.not_full.wait(state).unwrap();
                }
                while i < xs.len() && !state.is_full() {
                    state.put(xs[i]);
                    i += 1;
                }
                self.not_empty.notify_all();
            }
        }

        pub fn get(&self, xs: &mut [i32]) {
            let _lock = self.mtx_get.lock().unwrap();

            let mut state = self.mtx.lock().unwrap();
            let mut i = 0;
            while i < xs.len() {
                while state.is_empty() {
                    state = self.not_empty.wait(state).unwrap();
                }
                while i < xs.len() && !state.is_empty() {
                    xs[i] = state.get();
                    i += 1;
                }
                self.not_full.notify_all();
            }
        }
    }

    /// 1から10までの値をCircularBufferに追加する
    fn thread_a(b: &CircularBuffer) {
        let mut xs = Vec::new();
        for x in 1..=10 {
            xs.push(x);
        }
        println!("thread_a(): put( {:?} )", xs);
        b.put(&xs[..]);
    }
    /// -1から-10までの値をCircularBufferに追加する
    fn thread_b(b: &CircularBuffer) {
        let mut xs = Vec::new();
        for x in 1..=10 {
            xs.push(-x);
        }
        println!("thread_b(): put( {:?} )", xs);
        b.put(&xs[..]);
    }
    /// CircularBufferから10個の要素を取り出す
    fn thread_c(b: &CircularBuffer) {
        let mut xs = vec![0; 10];
        b.get(&mut xs[..]);
        println!("thread_c(): got() {:?}", xs);
    }
    /// CircularBufferから10個の要素を取り出す
    fn thread_d(b: &CircularBuffer) {
        let mut xs = vec![0; 10];
        b.get(&mut xs[..]);
        println!("thread_d(): got() {:?}", xs);
    }

    pub fn main() {
        let cbuf = std::sync::Arc::new(CircularBuffer::new());

        let th_a = {
            let cbuf = cbuf.clone();
            std::thread::spawn(move || thread_a(&cbuf))
        };
        let th_b = {
            let cbuf = cbuf.clone();
            std::thread::spawn(move || thread_b(&cbuf))
        };

        let th_c = {
            let cbuf = cbuf.clone();
            std::thread::spawn(move || thread_c(&cbuf))
        };
        let th_d = {
            let cbuf = cbuf.clone();
            std::thread::spawn(move || thread_d(&cbuf))
        };

        th_a.join().unwrap();
        th_b.join().unwrap();
        th_c.join().unwrap();
        th_d.join().unwrap();
    }
}

pub fn main() {
    multiple::main();
}
