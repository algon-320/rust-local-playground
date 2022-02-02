//# rand = "0.8.4"
//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

mod pq {
    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct Pq<T: Ord> {
        buf: Vec<T>,
    }

    impl<T: Ord> Pq<T> {
        pub fn len(&self) -> usize {
            self.buf.len()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        pub fn pop(&mut self) -> Option<T> {
            let len = self.len();
            if len <= 1 {
                return self.buf.pop();
            }

            self.buf.swap(0, len - 1);
            let ret = self.buf.pop();
            let len = len - 1;

            let mut v = 0;
            loop {
                let l = left_child(v);
                let r = right_child(v);

                let mx = if l < len && r < len {
                    if self.buf[l] < self.buf[r] {
                        r
                    } else {
                        l
                    }
                } else if l < len {
                    l
                } else {
                    break;
                };

                if self.buf[v] < self.buf[mx] {
                    self.buf.swap(v, mx);
                    v = mx;
                } else {
                    break;
                }
            }

            ret
        }

        pub fn push(&mut self, value: T) {
            self.buf.push(value);

            let mut v = self.buf.len() - 1;
            while v > 0 {
                if self.buf[parent(v)] < self.buf[v] {
                    self.buf.swap(parent(v), v);
                    v = parent(v);
                } else {
                    break;
                }
            }
        }
    }

    impl<T: Ord + Clone> Pq<T> {
        pub fn top(&self) -> Option<T> {
            if self.is_empty() {
                None
            } else {
                Some(self.buf[0].clone())
            }
        }
    }

    fn parent(node: usize) -> usize {
        (node - 1) / 2
    }
    fn left_child(node: usize) -> usize {
        2 * node + 1
    }
    fn right_child(node: usize) -> usize {
        2 * node + 2
    }
}

fn main() {
    let mut values = Vec::new();
    for _ in 0..10000 {
        let x: i32 = rand::random();
        values.push(x);
    }

    use pq::Pq;
    let mut pq: Pq<i32> = Pq::default();

    use std::collections::BinaryHeap;
    let mut expected: BinaryHeap<i32> = BinaryHeap::default();

    while values.len() > 0 || expected.len() > 0 {
        if rand::random() {
            if values.is_empty() {
                continue;
            }
            let x = values.pop().unwrap();
            pq.push(x);
            expected.push(x);
        } else {
            let x = pq.pop();
            let y = expected.pop();
            assert_eq!(x, y);
        }
    }
}
