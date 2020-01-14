
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let pair = (0, 2);

    println!("Tell me about {:?}", pair);
    match pair {
        (0, y) => println!("Right only {}", y),
        (x, 0) => println!("Left only {}", x),
        _ => println!("Both left and right exist"),
    }

    let color = Color::RGB(22, 2, 64);
    println!("What color is it?");
    match color {
        Color::Red|Color::Blue|Color::Green => println!("Pure RGB Color"),
        Color::RGB(r, g, b) => {
            println!("Red: {}, Green: {}, Blue: {}", r, g, b);
        }
        _ => println!("etc"),
    }

    // is it possible?

    // START -- Reference Test
    println!("-----------------------------------------------------");

    let outter_int_reference = &40;
    let outter_int_normal = 80;
    {
        let inner_literal_test = &10;
        let inner_static_test: &'static u32 = &10;
        fn simple_plus(a: &'static u32) ->u32{ a+1 }
        fn simple_plus2(a: u32) ->u32{ a+1 }
        simple_plus(outter_int_reference);
        simple_plus2(outter_int_normal);
        // outter_int_reference = &(outter_int_reference + 1);
        // outter_int_normal = outter_int_normal + 1;
        println!("Print ouuter ref inside: {}", outter_int_reference);
        println!("Print outter normal inside: {}", outter_int_normal);
    }
    println!("outter ref still availble? {}", outter_int_reference);
    println!("outter normal still available? {}", outter_int_normal);

    println!("-----------------------------------------------------");
    // END -- Reference Test
    let int_reference = &4;
    println!("{}", int_reference);
    match int_reference {
        &val => println!("Got a value via deconstructing: {:?}", val),
    }
    match *int_reference {
        val => println!("Got a value via dereferencing: {:?}\n", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("This is a reference of value: {}", r),
    }
    match mut_value {
        ref mut r => {
            *r += 10;
            println!("We added 10, `mut_value`: {:?}", r)
        }
    } println!("");


    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y),
        Foo { y: 2, x: i} => println!("y is 2, i = {:?}", i),
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        
    }

    // Guard
    let pair = (3, 4);
    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation...")
    }

    // Binding
    fn age() ->u32 { 17 }
    println!("\nTell me who you are!");
    match age() {
        0 => println!("I'm not born yet I guess"),
        n@1..=12 => println!("I'm a child of age {:?}", n),
        n@13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }

    fn some_number() ->Option<u32> { Some(42) }
    match some_number() {
        Some(n@42) => println!("The Answer: {}", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }

    // if let
    println!("");
    let optional = Some(7);
    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
        },
        _ => {}, // it seems like wasting space
    }

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}", i);
    }
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else { 
        println!("Can't match a number.");
    }

    let i_like_letter = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if let None = emoticon {
        println!("Didn't match a number");
    } else {
        println!("What is this?");
    }

    enum Fooo {
        Bar,
        Baz,
        Qux(u32),
    }
    let a = Fooo::Bar;
    let b = Fooo::Baz;
    let c = Fooo::Qux(100);
 
    if let Fooo::Bar = a {
        println!("Go to bar");
    }
    if let Fooo::Qux(val) = c {
        println!("Qux {}", val);
    }

    enum Foooo {Bar}
    let a = Foooo::Bar;
    // if Foooo::Bar == a { println!("Fus Rho Dah!"); }
    //  L Error cause no PartialEq
    if let Foooo::Bar = a { println!("Fus Rho Dah!"); } 

    // while_let
    let mut optional = Some(4);
    'outer: loop {
        match optional {
            Some(i) => {
                if i == 0 {
                    println!{"Emergency Exit!"}
                    break 'outer
                }
                if i>9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again", i);
                    optional = Some(i+1);
                }
            }
            _ => break
        }
    }

    let mut optional = Some(4);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit?");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again", i);
            optional = Some(i+1);
        }
    }
}