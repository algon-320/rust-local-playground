//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]
#![feature(new_uninit)]

fn main() {
    doubly::main();
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

mod doubly {
    use std::ptr::NonNull;

    struct Node<T> {
        value: T,
        next: NonNull<Node<T>>,
        prev: NonNull<Node<T>>,
    }

    impl<T> Node<T> {
        fn new(value: T) -> NonNull<Node<T>> {
            use std::mem::MaybeUninit;
            use std::ptr::addr_of_mut;

            let bx: Box<MaybeUninit<Node<T>>> = Box::new(MaybeUninit::uninit());
            let bx = Box::into_raw(bx);

            // safe because bx is obtained from Box::into_raw
            let node: NonNull<Node<T>> = unsafe { NonNull::new_unchecked(bx) }.cast();

            // initialize a Node through raw pointer
            unsafe {
                let uninit_ptr = (*bx).as_mut_ptr();
                addr_of_mut!((*uninit_ptr).value).write(value);
                addr_of_mut!((*uninit_ptr).next).write(node);
                addr_of_mut!((*uninit_ptr).prev).write(node);
            }

            // now it points to a initialized Node
            node
        }
    }

    struct List<T> {
        ptr: NonNull<Node<T>>,
        len: usize,
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            println!("List dropped");
            let origin = self.ptr;
            let mut p = self.ptr;
            loop {
                let next = unsafe { p.as_ref().next };
                unsafe { std::ptr::drop_in_place(p.as_ptr()) };
                if next == origin {
                    break;
                }
                p = next;
            }
        }
    }

    impl<T> List<T> {
        fn new(value: T) -> Self {
            let ptr = Node::new(value);
            Self { ptr, len: 1 }
        }

        fn insert_next(&mut self, mut other: Self) {
            // update length
            self.len += other.len;

            let node_ptr = self.ptr;
            let other_ptr = other.ptr;

            let node: &mut Node<T> = &mut *self;
            let other: &mut Node<T> = &mut *other;

            let mut node_next = node.next;
            let mut other_prev = other.prev;

            node.next = other_ptr;
            other.prev = node_ptr;

            drop(node);
            drop(other);

            let node = unsafe { node_next.as_mut() };
            node.prev = other_prev;

            let other = unsafe { other_prev.as_mut() };
            other.next = node_next;
        }

        fn move_forward(&mut self) {
            let next = (*self).next;
            self.ptr = next;
        }

        fn move_backward(&mut self) {
            let prev = (*self).prev;
            self.ptr = prev;
        }
    }

    impl<T: Clone> From<&[T]> for List<T> {
        fn from(values: &[T]) -> List<T> {
            if values.is_empty() {
                panic!("construct from empty slice");
            }

            let mut list = List::new(values[0].clone());
            if values.len() > 1 {
                list.insert_next(List::from(&values[1..]));
            }
            list
        }
    }

    impl<T> std::ops::Deref for List<T> {
        type Target = Node<T>;
        fn deref(&self) -> &Self::Target {
            // SAFETY: &self assures us that the dereferencing is safe.
            unsafe { self.ptr.as_ref() }
        }
    }
    impl<T> std::ops::DerefMut for List<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            // SAFETY: &mut self assures us that the dereferencing is safe.
            unsafe { self.ptr.as_mut() }
        }
    }

    impl<T: Clone> Iterator for List<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            let val = (*self).value.clone();
            self.move_forward();
            Some(val)
        }
    }
    impl<T: Clone> std::iter::DoubleEndedIterator for List<T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            let val = (*self).value.clone();
            self.move_backward();
            Some(val)
        }
    }

    pub fn main() {
        let list: List<i32> = List::from(&[1, 2, 3, 4, 5] as &[i32]);
        for x in list.take(10) {
            dbg!(x);
        }

        let list: List<&'static str> = List::from(&["a", "b", "c"] as &[&str]);
        for x in list.take(5) {
            dbg!(x);
        }

        #[derive(Debug, Clone)]
        struct Foo(i32);
        impl Drop for Foo {
            fn drop(&mut self) {
                println!("{:?} dropped", self);
            }
        }
        let foos = [Foo(1), Foo(2), Foo(3)];
        let list = List::from(&foos as &[Foo]);
        for x in list.take(5) {
            println!("====");
            println!("{:?}", x);
        }
        println!("--------");
    }
}
