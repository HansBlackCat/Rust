#![allow(unused_variables)]
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
    let inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    //let reborrow = &mut count;
    inc();

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
    

}