#![allow(unused_mut, unused_variables, dead_code, non_snake_case, irrefutable_let_patterns)]

// Practice Vector and String2

// String
fn main() {
    // String =/= &str
    // String is kind of `struct` contains &str, length, capacity

    // Method 1
    let mut s1 = String::new();
    let s1 = "Only for you~".to_owned();
    println!("{}", s1);

    // Method 2 (Easier)
    let mut s2 = String::from("반짝이는 그대에게로~");
    println!("{}", s2);

    // String support UTF-8
    let hello_fromJap = String::from("こんにちは");
    let hello_fromRus = String::from("Здравствуйте");
    println!("Hello Japan: {}", hello_fromJap);
    println!("Hello Rusia: {}", hello_fromRus);

    // Concat
    let mut s3 = s1 + " " + &s2;
    //                      ^~~~ don't forget `&`
    s3.push_str(" 살며시 다가가 그대와~");
    println!("{}", s3);

    // Bad CJK
    let test = String::from("안녕");
    println!("{}", test.len());
    // TODO!!
    // Why this len is not 2 but 6??
}