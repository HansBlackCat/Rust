#![allow(unused_variables, dead_code)]
use std::mem;

fn main() {
    fn n_function    (i: i32) ->i32 { i+ 1 }
    // inferred type - closure
    let c_function = |i| i+1;
    // This also possible
    let etc_c_function = |i: i32| ->i32{i+1};
    
    let i = 1;
    println!("normal function: {}", n_function(i));
    println!("closure function: {}", c_function(i));

    let one = ||1;
    println!("closure returning one: {}", one());

    let color = "green";
    let print = || println!("`color`: {}", color);
    let compilcated_color = String::from("red");
    let compilcated_print = || println!("`c_color`: {}", compilcated_color);
    print();
    compilcated_print();

    let reborrow = &color;
    let comp_reborrow = &compilcated_color;
    print();
    compilcated_print();

    let takeit = color;
    let comp_takit = compilcated_color;
    print();
    // Ownership shifted
    // compilcated_print();

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    // let reborrow = &count;
    inc();

    let count_reborrowed = &mut count;

    let movable = Box::new(3);
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    consume();

    // ----------------------------------------------------
    // Multi ref is OK
    let a = 3;
    let a_p = &a;
    let a_pp = &a;

    let b = String::from("hello");
    let b_p = &b;
    let b_pp = &b;
    // Multi mutable borrow is little complicated 
    // Can DECLARE, then can only use 2nd mut borrow
    let mut c = 2;
    let c_p = &mut c;
    *c_p += 1;
    let c_pp = &mut c;
    // let c_p_p = &mut c_pp; -> cannot borrow as muable, &mut's &mut impossible
    // *c_p += 1; -> can't use 1st &mut borrow 
    c += 1;
    let d = c + 1;
    println!("\nc is: {}\n", c);
    // ----------------------------------------------------
    
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));
    println!("{}", contains(&1));

    // B.c closure use 'move' to take authority following line can't compile
    // println!("There're {} elements in vec", haystack.len());

    // ----------------------------------------------------
    // << As Input Parameters >>
    println!("");
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();
    let diary = || {
        // Fn
        println!("I said {}.", greeting);
        // FnMut
        farewell.push_str("!!!");
        println!("Then I screamed {}", farewell);
        println!("Then I sleep. zzz");
        println!("");
        // FnOnce
        mem::drop(farewell);
    };
    apply(diary);
    let double = |x| 2*x;
    println!("3 doubled: {}", appy_to_3(double));

    // ----------------------------------------------------
    // << Type Anonymity >>
    let x = 7;
    let print = || println!("{}", x);
    apply2(print);

    // ----------------------------------------------------
    // << Input Function >>
    let closure = ||println!("I'm a closure!");
    call_me(closure);
    call_me(function_call);
    println!("");

    // ----------------------------------------------------
    // << As Output Parameters >>
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();

}

fn apply<F>(f: F) where F: FnOnce() { f(); }
fn appy_to_3<F>(f: F) ->i32 where F: Fn(i32) ->i32 { f(3) }

// F: Generic
fn apply2<F>(f: F) where F: FnOnce() { f(); }

fn call_me<F: Fn()>(f: F) { f(); }
fn function_call() { println!("I'm a function!") }

fn create_fn() ->impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {}", text)
}
fn create_fnmut() ->impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a: {}", text)
}
fn create_fnonce() ->impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This is a: {}", text)
}