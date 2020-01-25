// Reference Counting

// RC never do deep copy

use std::rc::Rc;

fn main() {
    let rc_example = "Rc examples".to_string();
    {
        println!("----- rc_a is created -----");
        let rc_a: Rc<String> = Rc::new(rc_example);

        println!("Reference Count of rc_a: {}\n", Rc::strong_count(&rc_a));

        {
            println!("----- rc_a is cloned to rc_b -----");

            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
            println!("Reference Count of rc_b: {}\n", Rc::strong_count(&rc_b));
            
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}\n", rc_b);

            println!("----- rc_b is dropped out of scope -----");
        }

        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        println!("----- rc_a is dropped out of scope -----");
    }

    // println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
    // println!("rc_examples: {}", rc_example);
}