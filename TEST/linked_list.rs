use crate::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() ->List {
        Nil
    }
    // &self vs self
    fn prepend(self, elem: u32) ->List {
        Cons(elem, Box::new(self))
    }
    fn len(&self) ->u32 {
        // self == &List, *self == List
        match *self {
            // can't take ownership from tail bc already borrow ref from self
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => { format!("Nil!") },
        }
    }
}


fn main() {
    // << Practice Boc >>
    let b = Box::new(5);
    println!("b = {}", b);

    // Main
    let mut list = List::new();

    list = list.prepend(3);
    list = list.prepend(10);
    list = list.prepend(9);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

}