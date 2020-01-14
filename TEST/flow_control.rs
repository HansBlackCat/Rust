// #[allow(unreachable_code)]

fn main() {
    let n = 5;
 
    let _big_n = 
        if n>6 {
            println!("Big");
            n*10
        }
        else { 
            println!("Small");
            n/2
        };
    
    let mut cnt = 0u32;
    loop {
        cnt += 1;
        if cnt==5 { println!("50!"); break }
        else { println!("Not yet"); continue; } // Maybe) break, continue's type is ()?

    }
    println!("");
    
    'outer: loop {
        println!("Here is outer loop");

        'inner: loop {
            println!("Entering to inner loop");
            // break 'outer; ->Go to Evacuate!
            break;
        }
        println!("Can you print this?");
        break;
    }
    println!("Evacuate!\n");

    let mut cnt = 0u32;
    let result = loop {
        cnt += 1;
        if cnt == 5 {
            break cnt*cnt;
        }
            };
println!("result is: {}\n", result);

    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 { println!("fizzbuzz") }
        else if n % 3 == 0 { println!("fizz") }
        else if n % 5 == 0 { println!("buzz") }
        else { println!("{}", n) }
        n += 1;
    } println!("\n");

    // iter
    // .iter() => borrow, can reuse iterated object after loop
    let names = vec!["WG", "WC", "GD", "SH", "JB", "DW"];
    for name in names.iter() {
        match name {
            &"JB" => println!("{} use rust", name),
            _ => println!("And.."),
        }
    } println!("");
    // .into_iter() => move, can't reuse, move ownership
    for name in names.into_iter() {
        match name {
            "WG" => println!("{} use C++", name),
            _ => println!("And.."),
        }
    } println!("");
    // println!("{:?}", names);
    // .iter_mut() => mut, can change value within borrowing
    let mut names = vec!["WG", "WC", "GD", "SH", "JB", "DW"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "WC" => "He want to learn Typescript",
            _ => "No Change",
        }
    }
    println!("{:?}\n", names)

}