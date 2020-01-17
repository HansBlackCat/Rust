

struct Empty;
struct Null;


trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}


// generic parameter 'T', caller 'U'
impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    // Deallocate 'empty' and 'null'
    // Both OK
    //empty.double_drop(null);
    null.double_drop(empty);

    // empty;
    // null;
}