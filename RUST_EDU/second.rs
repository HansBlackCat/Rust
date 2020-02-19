// Attribute
// Meaning: allow that `warning`
#![allow(unused_mut, unused_variables, dead_code, non_snake_case, irrefutable_let_patterns)]



fn main() {

    // Difference between &str and String
    {
        // String contain <&str, length, capacity>
        // &str is just list of [char]
        let test: &str = "hello";
        let mut test2: String = String::new();
        test2.push_str("Holla~");
        println!("&str: {}\nString: {}\n", test, test2);
    }

    // Death reference
    {
        // @Question why error?
        /* Dangling Pointer
        fn dangle() ->&String {
            let s = String::from("hello");
            &s // return s's REFERENCE
        } // BUT original s drop here
        let t = dangle();
        */
    }

    // Slice Type
    {
        let mut s = String::from("Hello World");

        let s1 = &s[0..5];
        // == &s[..5];
        let s2 = &s[6..s.len()];
        // == &s[6..];
        println!("{}-{}", s1, s2);
        // *BUT ONLY ASCII!!*

        fn first_word(s: &String) ->&str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[..i];
                }
            }

            &s[..]
        }

        let t = first_word(&s);
        
        // Review
        // Block String from modification
        // s.clear();

        println!("{}", t);

        // What is &str?
        // String without length and capacity
    }

    // Struct
    {
        // Already saw in C
        // derive Debug 
        // @3
        // Struct 1 (formal struct)
        #[derive(Debug)]
        struct Student {
            name: String,
            email: String,
            age: u32,
            // allow last comma(,)
            undergraduated: bool,
        }

        let mut me = Student {
            name: String::from("Lee"),
            email: String::from("abc@temp.com"),
            age: 23 as u32,
            undergraduated: false,
        };
        // Strong restriction
        // Other language such as Scala allow mut even when its const
        // == permit element change in Object
        me.age = 24;
        // Debug print
        println!("{:?}", me);

        let you = Student {
            name: String::from("Kim"),
            email: String::from("test@test"),
            // &me
            ..me
        };
    }

    // Struct 2 (tuple style)
    {
        // @13
        // Struct 2 (Tuple style struct)
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(0,0,0);
        let origin = Point(0,0,0);
        println!("{:?}", black.0);
    }

    // @Question
    // Is it possible?
    {
        // Error
        // missing lifetime specifier
        /*
        struct Student {
            name: &str,
            email: &str,
            age: u32,
            // allow last comma(,)
            undergraduated: bool,
        }
        */
    }

    // Struct 3
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let rect1 = Rectangle {width: 30, height: 50};
        let rect2 = Rectangle {width: 10, height: 20};
        println!("rect1 is {:?}", rect1);
        println!("rect2 is {:#?}", rect2);
        println!("rect2's width is? {}", rect2.width);

        // Defining Method
        // implementation
        impl Rectangle {
            // What is self?
            // something.method() 
            // ^~~~self  ^~~~method
            fn area(&self) ->u32 {
                self.width * self.height
            }
        }
        println!("rect1 area is? {}", rect1.area());

        // Multiple impl is OK
        impl Rectangle {
            // Parameter
            fn smaller(&self, other: &Rectangle) ->bool {
                self.width > other.width && self.height > other.height
            }
            // Act like constructor
            fn square_maker(width: u32, height: u32) ->Rectangle {
                Rectangle {width, height}
            }
        }
        println!("{:?}", rect1.smaller(&rect2));
        // Have you seen this before?
        // It's look like String::new(), String::from()
        let square_make = Rectangle::square_maker(20, 30);
        println!("{:?}", square_make);
    }

    // Impl PaYoEn
    {
        struct Bungi {
            data: i32,
        }
        impl Drop for Bungi {
            fn drop(&mut self) {
                println!("> Drop {}", self.data);
            }
        }

        {
            let mut t = Bungi {data: 0};
            {
                let mut s = Bungi {data:1};
                {
                    let k = Bungi {data:2};
                } // Drop 2
            } // Drop 1
        } // Drop 0
    }
    // Impl PaYoEn2
    {
        // What is Iterator?
        // Like linked list
        let mut seq = 0..2;
        println!("> {:?}", seq.next()); // Some(0)
        println!("> {:?}", seq.next()); // Some(1)
        println!("> {:?}", seq.next()); // None

        struct Fib {curr: u32, next: u32};
        // Simple Trait teach -- Like BluePrint
        /*
         * In Trait
         * pub trait Iterator {
         *     type Item;
         *     fn next(&mut self) ->Option<Self::Item>;
         *     ...etc 
         * }
         */

        impl Iterator for Fib {
            type Item = u32;
            // @Question?
            // Why &mut self? even we don't change
            // -> if held &mut, Locked => prevent iterator crash
            fn next(&mut self) ->Option<u32> {
                let new_next = self.curr + self.next;
                self.curr = self.next;
                self.next = new_next;
                Some(self.curr)
            }
        }

        let mut fib_test = Fib {curr: 1, next: 1};
        println!(">> {:?}", fib_test.next()); // Some(1)
        println!(">> {:?}", fib_test.next()); // Some(2)
        println!(">> {:?}", fib_test.next()); // Some(3)
        println!(">> {:?}", fib_test.next()); // Some(5)
    }

    // It's enum time!
    {
        // Enum is differenct with struct
        // It's like F#, Ocaml, Haskell's ADT

        #[derive(Debug)]
        enum IpAddress {
            V4,
            V6,
        }
        // check diff between . and :: 
        // . connect between (thing and function)
        // <<Apply front to Back>>
        // tuple.0; -- function(find 0th) to tuple
        // "42 ".trim().parse(); -- function(trim and parse) to &str

        // :: means call inside method or things
        // String::new(); -- call Struct String's function new()
        // IpAddress::V4; -- call IpAddress's V4
        
        // @Question
        // What is type of four and six?
        // Ans: IpAddress
        let four = IpAddress::V4;
        let six = IpAddress::V6;

        #[derive(Debug)]
        struct IpAddr {
            kind: IpAddress,
            address: String,
        }
        let my_home = IpAddr {
            kind: IpAddress::V4,
            address: String::from("127.0.0.1"),
        };
        println!("{:?}", my_home);

        enum SimpleIpAddress {
            V4(String),
            V6(String),
        }
        // @Question
        // let make fake_home, with V4 ip 127.0.0.1
        let fake_home = SimpleIpAddress::V4(String::from("127.0.0.1"));
    }

    // More enum
    {
        #[derive(Debug)]
        struct MessageLoc {
            sendfrom: String,
            sendto: String,
        }
        #[derive(Debug)]
        enum Message {
            // Abstract Type
            End,
            // enum type with inited
            Message(String),
            // multi type
            Color(i32, i32, i32),
            // struct-like type
            Os {yourOS: String, oppOS: String},
            // struct type
            Location(MessageLoc),
        }
        
        let mestest = MessageLoc {
            // to_owned() make &str -> String
            sendfrom: String::from("a"), 
            sendto: "b".to_owned(),
        };
        // @Question
        // make variable have type of Location, data mestest
        let test = Message::Location(mestest);
        println!("{:?}", test);
    }

    // Important 
    /* 
    enum Option<T> {
        Some(T),
        None,
    }
    */
    {
        let some_number = Some(5);
        let some_string = Some(String::from("hello"));
        println!("{:?}", some_number);
        println!("{:?}", some_string);
        let none_bool: Option<bool> = None;
        println!("{:?}\n", none_bool);
    }

    // Pattern matching
    {
        enum Grade {
            // Each is type
            A,
            B,
            C,
            D,
        }

        fn gradee(x: Grade) {
            match x {
                Grade::A => println!("Great!"),
                Grade::B => println!("Good"),
                Grade::C => println!("Well.."),
                // Try to comment below
                // non-exhaustive patterns: `D` not covered
                Grade::D => println!("Need some try"),
            }
        }
        let my_grade = Grade::D;
        gradee(my_grade);

        fn choose_onlyone(x: Grade) {
            match x {
                Grade::A => println!("You're Best!"),
                _        => println!("Try next time"),
            }
        }
    }

    // @Question
    // When input is Some<i32>, make plusOne function
    {
        let a: Option<i32> = None; 
        let b = Some(4);
        //.. Make Function
        fn plus_one(x: Option<i32>) ->Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i+1),
            }
        }
        //.. 
        println!("{:?}", plus_one(a)); // None
        println!("{:?}", plus_one(b)); // Some(5)
    }

    // Unwrapping
    {
        let t = Some(32);
        let s = Some("hello".to_owned());
        let w: Option<bool> = None;
        println!("{:?}", t.unwrap());
        println!("{:?}", s.unwrap());
        // thread panic!
        // Unwrap None,
        // println!("{:?}", w.unwrap());
        println!("{:?}", w.unwrap_or(true));
    }

    // if let
    {
        let some = Some(3);
        /*
        match some {
            Some(3) => println!("Youre 3");
            _ => (),
        }
        */
        if let some = Some(3) {
            println!("Youre 3");
        }
    }

    // Advanced
    // Guard 
    {
        let pair = (2,-4);
        match pair {
            (x,y) if x == y           => println!("x and y are same"),
            (x,y) if x + y == 0       => println!("x + y = 0"),
            (x,y) if x%2==0 && y%2==0 => println!("x and y are even number"),
            (x,_) if x == 7           => println!("Lucky Seven"),
            _                         => println!("Well.. no match!"),
        }
        // What if pair = (2,-2)?
        // (7,7)?
        // Second guard
    }

    // Binding
    {
        fn age() ->u32 {
            15
        }
        match age() {
            0           => println!("New"),
            n @ 1..=12  => println!("child of {:?}", n),
            n @ 13..=19 => println!("teen of {:?}", n),
            _           => println!("adult"),
        }
    }

    // @Question or Homework
    {
        // Make function `calculating` that calculate enum
        enum SimCal {
            Cal(i32, i32, char)
        }
        let x = SimCal::Cal(3, 4, '+'); // calculating(x) = 7
        let y = SimCal::Cal(10, 4, '-'); // calculating(y) = 6
        let z = SimCal::Cal(3, 10, '*'); // calculating(z) = 30
        let a = SimCal::Cal(12, 3, '/'); // calculating(a) = 4
        let a = SimCal::Cal(12, 3, '.'); // calculating(b) = 4

        fn calculating(t: SimCal) ->i32 {
            match t {
                SimCal::Cal(a, b, '+') => a+b,
                SimCal::Cal(a, b, '-') => a-b,
                SimCal::Cal(a, b, '*') => a*b,
                SimCal::Cal(a, b, '/') => a/b,
                _ => -1,
            }
        }
        println!("{:?}", calculating(x));
        println!("{:?}", calculating(y));
        println!("{:?}", calculating(z));
        println!("{:?}", calculating(a));
    }
}