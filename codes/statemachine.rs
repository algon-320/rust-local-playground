//---- Put dependencies above ----

#![allow(dead_code, unused_variables)]

#[derive(Debug)]
enum Error {
    InvalidTransfer,
}

trait StateMachine {
    type State;
    type Input;

    fn delta(state: Self::State, input: Self::Input) -> Option<Self::State>;
    fn state(&mut self) -> &mut Self::State;
}

trait StateMachineStepExt: StateMachine
where
    Self::State: Default,
{
    fn step(&mut self, input: Self::Input) -> Result<(), Error> {
        let dummy = Self::State::default();
        let state = std::mem::replace(self.state(), dummy);
        *self.state() = Self::delta(state, input).ok_or(Error::InvalidTransfer)?;
        Ok(())
    }

    fn step_iter<Iter>(&mut self, iter: Iter) -> Result<(), Error>
    where
        Iter: IntoIterator<Item = Self::Input>,
    {
        for input in iter {
            self.step(input)?;
        }
        Ok(())
    }
}

mod vending_machine {
    use super::*;

    enum Operation {
        Add(i32),
        Buy,
    }

    struct VendingMachine {
        current_coin: i32,
    }

    impl StateMachine for VendingMachine {
        type State = i32;
        type Input = Operation;

        fn state(&mut self) -> &mut i32 {
            &mut self.current_coin
        }
        fn delta(state: i32, input: Operation) -> Option<i32> {
            match input {
                Operation::Add(x) => Some(state + x),
                Operation::Buy => Some(0),
            }
        }
    }
    impl StateMachineStepExt for VendingMachine {}

    pub fn main() {
        let mut vending_machine = VendingMachine { current_coin: 0 };
        vending_machine.step(Operation::Add(10)).unwrap();
        vending_machine.step(Operation::Add(10)).unwrap();
        vending_machine.step(Operation::Add(100)).unwrap();
        assert_eq!(vending_machine.current_coin, 120);
        vending_machine.step(Operation::Buy).unwrap();
        assert_eq!(vending_machine.current_coin, 0);

        let mut vending_machine = VendingMachine { current_coin: 0 };
        let ops = vec![Operation::Add(10), Operation::Add(100), Operation::Add(10)];
        vending_machine.step_iter(ops).unwrap();
        assert_eq!(vending_machine.current_coin, 120);
    }
}

mod adder {
    use super::*;

    struct Adder {
        x: i32,
    }

    impl StateMachine for Adder {
        type State = i32;
        type Input = i32;

        fn state(&mut self) -> &mut i32 {
            &mut self.x
        }
        fn delta(current: i32, d: i32) -> Option<i32> {
            Some(current + d)
        }
    }
    impl StateMachineStepExt for Adder {}

    pub fn main() {
        let mut addr = Adder { x: 0 };
        addr.step_iter(std::iter::repeat(1).take(10)).unwrap();
        assert_eq!(addr.x, 10);

        let mut addr = Adder { x: 0 };
        addr.step_iter((1..).take(10)).unwrap();
        assert_eq!(addr.x, 55);
    }
}

mod unclonable {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum Foo {
        X,
        Y,
        Z,
    }
    impl Default for Foo {
        fn default() -> Self {
            Foo::X
        }
    }

    struct Rot {
        f: Foo,
    }
    impl StateMachine for Rot {
        type State = Foo;
        type Input = ();

        fn state(&mut self) -> &mut Foo {
            &mut self.f
        }
        fn delta(f: Foo, _: ()) -> Option<Foo> {
            let nx = match f {
                Foo::X => Foo::Y,
                Foo::Y => Foo::Z,
                Foo::Z => Foo::X,
            };
            Some(nx)
        }
    }
    impl StateMachineStepExt for Rot {}

    pub fn main() {
        use std::iter::repeat;
        let mut rot = Rot { f: Foo::X };
        rot.step_iter(repeat(()).take(1)).unwrap();
        assert_eq!(rot.f, Foo::Y);
        rot.step_iter(repeat(()).take(2)).unwrap();
        assert_eq!(rot.f, Foo::X);
        rot.step_iter(repeat(()).take(10)).unwrap();
        assert_eq!(rot.f, Foo::Y);
    }
}

fn main() {
    vending_machine::main();
    adder::main();
    unclonable::main();
}
