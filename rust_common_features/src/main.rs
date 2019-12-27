//use std::io;

// let CONST_A = 3;
const CONST_A: u32 = 3; // const -> ignore all scopes, can declare anywhere, must annotate type

fn main() {
    // << Mutable and Immutable, Const >>
    let immutable_x = 5;
    println!("let x == {}", immutable_x);
    //  immutable_x = 7; // Not OK

    let mut mutable_x = 5;
    println!("let mut x == {}", mutable_x);
    mutable_x = 6;
    println!("new mut x == {}", mutable_x);

    println!("globally declared CONST_A == {}", CONST_A);
    println!("\n");

    // << Shadowing >>
    let _shadowing_a = 5;
    let _shadowing_a = 4;  // OK
    //  _shadowing_a = 3; // Not OK
    let _shadowing_a = _shadowing_a+3;
    let _shadowing_a = _shadowing_a*3;
    println!("shadowing_a == {}", _shadowing_a);

    let _shadowing_b = "Dormitory";
    let _shadowing_b = _shadowing_b.len(); // let can mutate type if shadowed
    let mut mut_b = "Dormitory";
    // mut_b = mut_b.len(); // but mut can't  -- expected &str, found usize
    let mut mut_b = mut_b.len();
    println!("_shadowing_b == {}\nmut_b == {}", _shadowing_b, mut_b);
    println!("\n");

    // << Data Type >>
    let guess: &str = "007";
    let guess: u32 = guess.trim().parse().expect("Is it number?");
    println!("guess == {}", guess);
    let float_x = 2.0;
    let boolean_x = true;
    let char_x = 'ℤ';
    let char_howaboutthis = '😻';
    println!("char Family! {} {}", char_x, char_howaboutthis);

    let decimal_x: i32 = 98_222;
    let hex_x: i32 = 0xff;
    let octal_x: i32 = 0o77;
    let binary_x: i32 = 0b1111_0000;
    let byte_x: u8 = b'A'; // Only u8 available
    println!("decimal_x == {}\n\
              hex_x == {}\n\
              octal_x == {}\n\
              binary_x == {}\n\
              byte_x: u8 == {}", decimal_x, hex_x, octal_x, binary_x, byte_x);

    let tup_x = (500, 40.3, 'ℤ');
    let (intup_x, intup_y, intup_z) = tup_x;
    println!("First component in tuple: {}", intup_x);
    let fst_tup = tup_x.0;
    let snd_tup = tup_x.1;
    println!("Access through dot -> {} {}\n", fst_tup, snd_tup);

    let list_x = [1,2,3,4];
    let fst_list_x = list_x[0]; // Cannot access through dot
    let list_y = [3..5];
    let list_z = [3; 5];

    // << Functions >>
    lets_print_hello_world();
    what_is_x(4);
    // println!("{}", fnx); -- Of course, can't
    define_fnx_to_z();
    let five_with_return = this_is_return_five();
    let mut five_without_return = this_is_nonreturn_five();
    println!("{}", five_with_return==five_without_return);
}

fn lets_print_hello_world() { println!("Hello world!"); }
fn what_is_x(x: i32) { println!("x is {}", x); }
fn define_fnx_to_z() {
    let fnx = 30;
    let fny = {
        let fnz = 3;
        fnz*5 // if add ; at suffix it considered like statement
    };
    println!("fnx-z {} {}", fnx, fny); // can't call fnz(not in scope)
}
fn this_is_return_five()-> i32 { return 5 } // need ->i32
fn this_is_nonreturn_five()-> i32 { 5 } // need -> i32

