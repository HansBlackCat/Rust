
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
    let you_know_pointer_question_mark = calculate_length_v_pointer(&s_ref); // you can use again

    // << HOW to use neighbor's things >>
    let mut s_kind_neighbor = String::from("Apple pie");
    modify_borrowed_string_v_mutable(&mut s_kind_neighbor);
    println!("{}", s_kind_neighbor);

    let pie_s_pointer = &mut s_kind_neighbor;
    let another_pie = &mut s_kind_neighbor; // you can allocate two pointer
    // println!("{}, {}", pie_s_pointer, another_pie); // can't borrow two mutable

    let mut peanut_sandwich = String::from("Delicious Sandwich");
    let ps_pointer1 = &peanut_sandwich;
    let ps_pointer2 = &peanut_sandwich; // It's OK
    println!("{}, {}", ps_pointer1, ps_pointer2); // and can borrow two immutable
    let ps_pointer3 = &mut peanut_sandwich; // ps_pointer1,2 now out of scope
    println!("{}", ps_pointer3);

    // let ref_of_nothing = dangle();  //reference of nothing
    let ref_of_yesthing = no_dangle();
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
// fn modify_borrowed_string(some_string: &String) { some_string.push_str(", You understand?"); }
// L you can't modify borrowed value, you thief!
fn modify_borrowed_string_v_mutable(some_string: &mut String) { some_string.push_str(", But everything has exception"); }
// fn dangle() ->&String { let s = String::from("dangle dangle"); &s } // s out of scope so &s==null
fn no_dangle() ->String {
    let s = String::from("no dangle dangle");
    s
}