#![allow(unused_variables, dead_code)]

use std::mem;

fn main() {
    // RAII
    {
        // stack allocate
        let a: u32 = 3;
        println!("{}", a);
    }

    {
        // heap allocate
        // similar with malloc in C
        let a = Box::new(3u32);
        println!("{}", a);
    }

    {
        let a = 5u32;
        let b = a;
        println!("{}-{}", a, b);

        let x = Box::new(12u32);
        let y = x;
        // println!("{}-{}", x, y);
        // borrow of moved value
    }

    // Error Handling
    {
        fn give_gift(gift: String) {
            if gift == "moon".to_owned() { panic!("Fuck you") }
            println!("Oh. thanks you!");
        }
        give_gift(String::from("cake"));
        // give_gift(String::from("moon"));
        // thread 'main' paniced at 'Fuck you'

        // panic! has type `!` which can be anything
        fn div_number(divd: i32, divby: i32) ->i32 {
            if divby == 0 { panic!("Divide by zero") }
            divd / divby
        }
        println!("{}", div_number(10, 5));
        // println!("{}", div_number(3, 0));
        // thread 'main' paniced at 'Divide by zero'
    }

    {
        // error and Option<T>
        fn wrap_gift(gift: Option<&str>) {
            match gift {
                Some("moon") => println!("Fuck you"),
                Some(a)      => println!("It's {}!", a),
                None         => println!("Fuck you"),
            }
        }
        wrap_gift(Some("moon"));
        wrap_gift(Some(&String::from("sun")));
        wrap_gift(None);

        fn wrapper(wrappee: String) ->Option<String> {
            if wrappee.is_empty() { None }
            else { Some(wrappee) }
        }
        println!("{}", wrapper(String::from("hello")).unwrap());
    }

    // Closure
    // use std::mem;
    {
        fn func           (i: u32) ->u32 { i+2 }
        let closure_ann = |i: u32| ->u32 { i+2 };

        let always_one = ||1;
    }

    {

    }
}
