// This declaration will look for a file named `_trait.rs` or `_trait/mod.rs` and will
// insert its contents inside a module named `_trait` under this scope
pub mod _trait;
use crate::_trait::init::*;

use std::ops;

/*
// ---------------------------------------------------------------------
// Derive
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Centimeters(f64);

#[derive(Debug)]
pub struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// ---------------------------------------------------------------------
// Dyn
struct Hush {}
struct Chunky {}

trait Chocolate{
    fn how_sweet(&self) -> &'static i32;
}
impl Chocolate for Hush {
    fn how_sweet(&self) -> &'static i32 {
        &5
    }
}
impl Chocolate for Chunky {
    fn how_sweet(&self) -> &'static i32 {
        &2
    }
}
fn random_chocolate(random_number: f64) -> Box<dyn Chocolate> {
    if random_number < 50 as f64 {
        Box::new(Hush {})
    } else {
        Box::new(Chunky {})
    }
}

// ---------------------------------------------------------------------
// Overloading
struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// By reversing the types, we end up implementing non-commutative addition.
// Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
// This block implements the operation: Bar + Foo = BarFoo
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

// ---------------------------------------------------------------------
// Drop
struct Droppable {
    name: &'static str,
}

// This trivial implementation of `drop` adds a print to console.
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

// ---------------------------------------------------------------------
// Iterator
// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
    type Item = u32;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
        // will never return `None`, and `Some` is always returned.
        Some(self.curr)
    }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

// ---------------------------------------------------------------------
// Iterator
*/

fn main() {
    println!("Hello, world!");

    let foot = Inches(3);
    println!("{:?}", foot);
    let centi = foot.to_centimeters();
    println!("{:?}", centi);

    /*
    for i in fibonacci().take(10) {
        print!("{} ", i);
    }
    */
}
