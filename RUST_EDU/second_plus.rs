#![allow(unused_mut, unused_variables, dead_code, non_snake_case, irrefutable_let_patterns)]

// Practice Vector and String1

// Vector
fn main() {
    let mut v1: Vec<i32> = Vec::new();
    //             ^~~~ Need to write 'what type' you want to push in vector
    println!("{:?}", &v1);
    //               ^~~~ Don't forget to write `&`
    v1.push(1);
    v1.push(2);
    println!("{:?}", &v1);

    // More simple way: Using MACRO
    let v2: Vec<i32> = vec![1,2,3,4,5];
    println!("{:?}", &v2);
    
    // print third element
    println!("{:?}", &v2[2]);
    match &v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // TODO!!
    // What is difference between v2[2] and v2.get(2) ??

    // Print all
    for i in &v2 {
        print!("{} ", i);
        // println! -> print and \n
        // print!   -> print
    }
    

}