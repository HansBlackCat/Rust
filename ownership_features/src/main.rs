
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    // let s_2 = s; println!("{}", s); -- Compiled error b/c ownership shifted
    let s_2 = s.clone();
    println!("s == {}, s_2 == {}", s, s_2);

    // << More and More >>
    let s_from = String::from("hello");

    take_ownership(s_from);
    // println!("{}", s_from); -- Of course, you can't

    let s_copy = 5; // int, double, char, tuple with same elem type is copy

    makes_copy(s_copy);
    println!("{}", s_copy);

    let s1 = give_ownership();
    let s2 = String::from("hello");
    let s3 = take_and_give_back(s2);
    // println!("{}", s2); -- borrow of moved value

    let make_int_fuck = i32::from(4);
    let _make_int_fuck2 = make_int_fuck;
    println!("{}", make_int_fuck); // Unfortunately you can't allocate ownership

    let mut how_about_this = String::from("how about this!");
    // let ninja_washabi = how_about_this.push_str("NINJA");
    // println!("{}", ninja_washabi); -- Can't maybe b/c .push.. is not return value?

    let (s3_val, s3_len) = calculate_length(s3);
    println!("Length of {} is {}", s3_val, s3_len);

    let s_ref = String::from("hello reference");
    // let s_temp = calculate_length2(s_ref); // Blow s_ref off
    // println!("{}", s_ref); // Here's a problem so..
    let _s_ref_uszie = calculate_length_v_pointer(&s_ref);
    println!("{}", s_ref);
    // let you_know_pointer_question_mark = calculate_length_v_pointer(&s_ref);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
fn give_ownership() ->String {
    let some_string = String::from("hello");
    some_string
}
fn take_and_give_back(some_string: String) ->String {
   some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let leng = s.len(); // .len() return length

    (s, leng)
}
fn calculate_length2(s: String) -> usize { s.len() } // Why not this?
fn calculate_length_v_pointer(s: &String) -> (usize) { s.len() }