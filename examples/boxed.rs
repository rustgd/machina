// This example shows how to use dynamically typed states.
// While this allows easier extension from the outside, it's not
// as performant as using an enum.

extern crate machinae;

use machinae::{DynMachine, DynResult, DynState, Trans};

struct State1;

impl DynState<i32, (), ()> for State1 {
    fn start(&mut self, args: i32) -> DynResult<i32, (), ()> {
        println!("Starting with {}", args);

        Ok(Trans::None)
    }

    fn update(&mut self, args: i32) -> DynResult<i32, (), ()> {
        Ok(Trans::Switch(Box::new(State2(args))))
    }
}

struct State2(i32);

impl DynState<i32, (), ()> for State2 {
    fn start(&mut self, args: i32) -> DynResult<i32, (), ()> {
        println!("Started State2 with {}", args);
        println!("Quitting");

        Ok(Trans::Quit)
    }
}

fn run() -> Result<(), ()> {
    let mut machine = DynMachine::new(Box::new(State1));

    machine.start(1)?;
    machine.update(2)?;

    assert!(!machine.running());

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error occurred: {:?}", e);
    }
}
