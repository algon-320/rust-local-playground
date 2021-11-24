//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

fn main() {
    singly::main();
}

mod singly {
    #[derive(Clone)]
    struct Node {
        value: i32,
        next: Option<Box<Node>>,
    }

    impl Node {
        fn new(value: i32) -> Self {
            Self { value, next: None }
        }

        fn insert_next(&mut self, value: i32) {
            let another = Node {
                value,
                next: self.next.take(),
            };
            self.next = Some(Box::new(another));
        }

        fn delete_next(&mut self) -> Option<i32> {
            if let Some(mut next) = self.next.take() {
                self.next = next.next.take();
                Some(next.value)
            } else {
                None
            }
        }
    }

    impl IntoIterator for Node {
        type IntoIter = NodeIter;
        type Item = i32;

        fn into_iter(self) -> Self::IntoIter {
            NodeIter(Some(Box::new(self)))
        }
    }

    struct NodeIter(Option<Box<Node>>);

    impl Iterator for NodeIter {
        type Item = i32;

        fn next(&mut self) -> Option<i32> {
            if let Some(node) = self.0.take() {
                let value = node.value;
                *self = NodeIter(node.next);
                Some(value)
            } else {
                None
            }
        }
    }

    pub fn main() {
        let mut root = Node::new(-1);
        root.insert_next(3);
        root.insert_next(2);
        root.insert_next(1);

        // root --> v=1 --> v=2 --> v=3

        let vec: Vec<i32> = root.clone().into_iter().collect();
        let expected = vec![-1, 1, 2, 3];
        assert_eq!(expected, vec);

        let x = root.delete_next();
        // root --> v=2 --> v=3

        let vec: Vec<i32> = root.into_iter().collect();
        let expected = vec![-1, 2, 3];
        assert_eq!(expected, vec);
        assert_eq!(Some(1), x);
    }
}
