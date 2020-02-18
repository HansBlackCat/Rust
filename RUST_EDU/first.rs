// Attribute
#![allow(unused_mut, unused_variables, dead_code)]


fn main() {

    // Print Hello world
    {
        println!("Hello World!\n");
    }

    // Immutability
    {
        let x = 5;
        println!("x is: {}\n", x);
        // Compile Error
        // x = 6;
    }

    // Mutability
    {
        let mut x = 5;
        println!("x is: {}", x);
        x = 3;
        println!("x is: {}\n", x);
    }

    // Shadowing
    {
        let x = 5;
        let x = 6;
        let x = x + 10;
        println!("x is: {}", x);
    }

    // Shadowing2
    {
        let some = "Hi there";
        let some = some.len();
        println!("some: {}\n", some);

        let mut some2 = "Hola";
        // expected `&str`, found `usize`
        // some2 = some2.len();
    }

    // Data Type
    {
        let guess: u32 = "42 ".trim().parse().expect("Not Num");
        // type annotation needed
        // let no_guess = "42 ".trim().parse().expect("Not Num");
    }

    // Integer Type
    {
        let x: u8 = 240;
        let y: u16 = 23;
        let z: u32 = 4;
        let a: i64 = -987;
        // Same as system size
        let b: isize = 98;
    }

    // Other type
    {
        let x: f64 = 2.0;
        let y: f32 = 3.0;
        let f: bool = false;
        let c: char = 'z';
        
        let tup: (i32, f64, char) = (500, 3.3, 'd');
        let (x, y, z) = tup;
        println!("snd of tup: {}", y);
        let fst_in_tup = tup.0;
        let last_in_tup = tup.2;

        let a: [i32; 5] = [0, 1, 2, 3, 4];
        println!("snd of list: {}", a[1]);

        let x = {
            let mut temp = 0;
            for i in 0..10 {
                temp = temp + 1;
            }
            temp
        };
        println!("x is: {}\n", x);
    }

    // Function
    {
        fn make_fn() {
            println!("Hello?");
        }
        fn another_function(x: i32) {
            println!("Hello? {}", x);
        }
        fn adder(x: i32, y:i32) ->i32 {
            x+y
        }

        make_fn();
        another_function(4);
        // return without ;
        let x = adder(3, 4);
        println!("x is: {}\n", x);
    }

    // Flow Contorl
    {
        let num = 5;
        let t: bool = {
            if num > 5 {
                true
            } else if num < 4 {
                true
            } else {
                false
            }
        };
        /* Error
        let err_t = {
            if num > 5 {
                true
            } else {
                3
            }
        };
        */
        println!("t is: {}", t);

        // Iterable
        let a = [1,2,3,4,5];
        for elem in a.iter() {
            println!("What is in a? {}", elem);
        }
        for num in (1..7).rev() {
            println!("{}!", num);
        }
    }

    // Ownership1
    {
        // `Copy` Prelude-type
        {
            let s = "string";
            // you can ues s here
            let t = s;
            let y = s.to_uppercase();
            println!("\ns here? {}", s);
        } // 's' removed here

        // &str vs String
        // &str is const, can't change it's size when running
        // String -> Heap allocated

        // String consist of... 
        // Pointer -> index/ Length/ Capacity
        // Double Free Error
        // Shallow Copy -> Copy Length/ Capacity
        // Deep Copy -> Copy all index
        // Rust's answer -> Shallow Copy and Move
        // in Rust Perspective, deep copy is not recommendable
        // .clone() method help deep copy

        // `Own` String, Vector, Class, TypeClass ...
        {
            let s = String::from("alloa!");
            // 's' removed here
            let t = s;
            // value borrowed here after move
            // println!("{}", s);
        } // `s` droped trait activate

        {
            fn take_ownership(some_string: String) {
                println!("{}", some_string);
            }

            let s: String = String::from("alloa!");
            take_ownership(s);
            // println!("{}", s);
        }

        {
            fn give_ownership() ->String {
                let some_string = String::from("hello");
                some_string
            }
        }
        // give_ownership();

        // ETC
        // Function as paramter
        {
            fn fun_test(value: i32, f: &dyn Fn(i32) -> i32) -> i32 {
                println!("{}", f(value));
                value
            }
            fn times2(value: i32) -> i32 {
                2 * value
            }
            fun_test(5, &times2);
            println!("\n");
        }
    }

    // Ownership2
    { 
        // Reference
        {
            fn calculate_length(s: &String) ->usize {
                s.len()
            }

            let s1 = String::from("hello");
            let len = calculate_length(&s1);
            println!("len is: {}", len);

            println!("s1 is? {}\n", s1);
        }

        // Reference 2
        {
            fn calculate_length(s: &String) ->usize {
                s.len()
            }
            fn calculate_length2(s: &String) ->usize {
                s.len()+1
            }



            let s1 = String::from("Howdy");
            let s1_clone = s1.clone();

            let s2 = &s1;
            let s3 = &s1;
            let s3_2 = &s2;

            // move action
            // remove all referenced things
            let s1_move = s1 + " world";
            println!("s1_move?: {}", s1_move);
            // println!("s1 alive?: {}", s1);
            // println!("s2 alive?: {}", s2);

        }

        // mut change
        {
            fn change(s: &mut String) {
                s.push_str(" alloa")
            }

            // Mut Ref
            // mutable 
            let mut p = String::from("Star");
            let p2 = &p;
            let p_mut = &mut p;

            // immutable borrow later used here
            // println!("p2?: {}", p2);
        }

        // double mut
        {
            let mut s = String::from("ahh");

            let s1 = &mut s;
            let s2 = &mut s;
            // second mutable borrow occurs here
            // println!("{}- {}", s1, s2);
        }

        // ref and mut ref
        {
            let mut s = String::from("hello");

            let s1 = &s;
            let s2 = &s;
            let s3 = &mut s;
            // immutable borrow later used here
            // println!("{}-{}-{}", s1, s2, s3);
        }

        {
            let mut s = String::from("gello");
            
            let s1 = &s;
            let s2 = &s;
            println!("{}-{}", s1, s2);

            let s3 = &mut s;
            println!("{}", s3);
        }

        // Dangling Pointer
        {/*
            fn some_func() -> &String {
                let s = String::from("hello");
                &s
            }
            // missing lifetime specifier
            // let ref_to_nothing = some_func();
        */
        }

    }

    // Slice type
    {
    }
}


