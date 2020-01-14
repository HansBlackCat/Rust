
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
        &val => println!("Got a value via dereferencing: {:?}", val),
        // _ => println!("etc"),
    }
}