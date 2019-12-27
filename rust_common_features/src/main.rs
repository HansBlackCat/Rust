//use std::io;

fn main() {
    let immutable_x = 5;
    println!("let x == {}", immutable_x);

    let mut mutable_x = 5;
    println!("let mut x == {}", mutable_x);
    mutable_x = 6;
    println!("new mut x == {}", mutable_x);

}
