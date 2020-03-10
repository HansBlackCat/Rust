#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    non_snake_case,
    unused_must_use
)]

use std::fs::*;
use std::io;
use std::io::{ErrorKind, Read};

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
        let a = Box::new(3u32); // malloc int space -> 3
        let b = Box::new(3);
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

    // String::from("as");
    // String::new();
    // "asd".to_owned();
    // String -> &str, length, capacity

    // Vector
    {
        println!("\n<< VECTOR >>\n");

        let mut v1: Vec<i32> = Vec::new();
        //             ^~~~ Need to write 'what type' you want to push in vector
        println!("{:?}", &v1);
        //               ^~~~ Don't forget to write `&`
        v1.push(1);
        v1.push(2);
        println!("{:?}", &v1);
        // [1,3,5,7,9]

        // More simple way: Using MACRO
        let v2: Vec<i32> = vec![1, 2, 3, 4, 5];

        println!("{:?}", &v2);

        // print third element
        println!("{:?}", &v2[2]);
        // v2[999]; ? -> SegFault
        // enum -> Error Some | None

        match &v2.get(999) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no element."),
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

    // String
    {
        println!("\n<< STRING >>\n");
        // String =/= &str
        // String is kind of `struct` contains &str, length, capacity

        // "hook" -> length: 4 , Capacity: 8
        // hook + tt ++ sa = "hookttsa" -> length: 8
        // - - - - 끝
        // - - - - -
        // - - - - 0 0 0 0
        // - - - - - - - -

        // Method 1
        let mut s1 = String::new();
        let s1 = "Only for you~".to_owned();
        println!("{}", s1);

        // Method 2 (Easier)
        let mut s2 = String::from("반짝이는 그대에게로~");
        println!("{}", s2);

        // String vs &str
        // String support UTF-8
        let hello_fromJap = String::from("こんにちは");
        let hello_fromRus = String::from("Здравствуйте");
        println!("Hello Japan: {}", hello_fromJap);
        println!("Hello Rusia: {}", hello_fromRus);

        // Concat
        let mut s3 = s1 + " " + &s2;
        //                      ^~~~ don't forget `&`

        // s3 = s1 + &s2 + &s4 + &s7
        s3.push_str(" 살며시 다가가 그대와~");
        println!("{}", s3);

        // Bad CJK
        let test = String::from("안녕");
        println!("{}", test.len());
        // TODO!!
        // Why this len is not 2 but 6??
    }

    // Hash map
    {
        println!("\n<< HASPMAP >>\n");
        // Many different name
        // Hash, Map, Object, HashTable, Dictionary, Associative Array

        // Fast at finding
        // Slow for Saving
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        // .insert(Key, Value) -> Can be any type
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Red"), 15);
        println!("{:?}", scores);
        // mismatched types
        // scores.insert(12, true);

        // Mixing
        let teams = vec![String::from("Blue"), String::from("Red")];
        let scores = vec![10, 15];

        let scores: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
        println!(">>> {:?}", scores);

        let scores2 = teams.iter().zip(scores.iter());
        println!("{:?}", scores2);
        // Zip { a: Iter(["Blue", "Red"]), b: [("Blue", 10), ("Red", 15)], index: 0, len: 0 }
        // -> {"Blue": 10, "Red": 15}
        // .collect() do Unwraping zip and make b to `something`

        // if ommiting Haspmap<_,_>? -> ERROR `consider giving `scores` a type`

        // zip -> matching
        // [1,2,3] zip [4,5,6] == [(1,4),(2,5),(4,6)]

        // Finding with key
        // .get()
        let finding_team_name = String::from("Blue");
        let score_of = scores.get(&finding_team_name);
        // Guessing
        println!("{:?}", score_of);
        // == Some(10)
        // None
        // WHY?

        // Only Reference Accepted!
        //let score_of = scores.get(finding_team_name);
        //println!("{:?}", score_of);

        let mut new_score = HashMap::new();
        new_score.insert(String::from("Blue"), 10);
        new_score.insert(String::from("Blue"), 15);
        println!("{:?}", new_score);
        // == {"Blue": 15}

        new_score.entry(String::from("Yellow")).or_insert(50);
        new_score.entry(String::from("Blue")).or_insert(30);
        println!("{:?}", new_score);
        // == {"Blue": 15, "Yellow": 50}

        let tmp = new_score.entry("Blue".to_owned());
        println!("{:?}", tmp);
        // Entry(OccupiedEntry { key: "Blue", value: 15 })

        // https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
        // pub enum Entry<'a, K: 'a, V: 'a> {
        //     Occupied(OccupiedEntry<'a, K, V>),
        //     Vacant(VacantEntry<'a, K, V>),
        // }
        // Like Some | None -> Check if empty

        let text = "hello my world and hello my school and no hello Zzang";
        // hello -> 3times
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1
        }
        println!("{:?}", map);

        let a1 = "ab bs ds";
        println!("{:?}", a1.split_whitespace());
        // SplitWhitespace { inner: Filter { iter: Split(SplitInternal { start: 0, end: 8, matcher: CharPredicateSearcher { haystack: "ab bs ds", char_indices: CharIndices { front_offset: 0, iter: Chars(['a', 'b', ' ', 'b', 's', ' ', 'd', 's']) } }, allow_trailing_empty: true, finished: false }) } }

        // {"my": 2, "hello": 3, "and": 2, "school,": 1, "world,": 1, "no": 1, "Zzang": 1}
    }

    // Unrecoverable Error Handling
    {
        // Recoverable Error Handling -> enum Some | None
        // enum Result<T,E> {
        //     Ok(T),
        //     Err(E),
        // }

        // Different between SegFault and panic!
        // Don't stop applications but release stack
        // Stop applications -> OS clean memory
        // release stack     -> Language manage memory
        // Pros of panic! -> can find where error occures
        println!("\n<< ERROR HANDLING >>\n");

        // Easy way to see panic!
        let vec = vec![1, 2, 3];
        // println!("{:?}", vec[99]);
        // thread 'main' panicked at 'index out of bounds

        // But why? we can't find this error in compile time?
        // Boundary check problem -> Make overhead
        // Bounds-check elimination

        fn give_gift(gift: String) {
            if gift == "moon".to_owned() {
                panic!("Fuck you")
            }
            println!("Oh. thanks you!");
        }
        give_gift(String::from("moone"));
        // give_gift(String::from("moon"));
        // thread 'main' paniced at 'Fuck you'

        // panic! ?
        // `!`
        // panic! has type `!` which can be anything
        fn div_number(divd: i32, divby: i32) -> i32 {
            if divby == 0 {
                panic!("Divide by zero")
            }
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
                Some(a) => println!("It's {}!", a),
                None => println!("Fuck you"),
            }
        }
        wrap_gift(Some("moon"));
        wrap_gift(Some(&String::from("Yooman")));
        wrap_gift(None);

        fn wrapper(wrappee: String) -> Option<String> {
            if wrappee.is_empty() {
                None
            } else {
                Some(wrappee)
            }
        }
        println!("{}", wrapper(String::from("hello")).unwrap());
    }

    // Error 2
    {
        // first `use`
        // use std::fs::*;
        // fs -> FileSystem
        // use std::io;
        // use std::io::{ErrorKind, Read};
        println!(
            "\n\n------------------------------------------------------------------------------"
        );

        // First Run
        let f = File::open("hello.txt");
        // NO Error message

        // Manage Error with type matching

        // Some(T) | None

        // pub enum Result<T, E> {
        //     Ok(T),
        //     Err(E),
        // }

        let f = match f {
            Ok(file) => file,
            Err(ref error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Can't create file {:?}", e),
                },
                other_error => panic!("Can't open file {:?}", other_error),
            },
        };

        // File::open("something") -> Result<T, E>
        // unwraping (similar things with matching, not create new file)

        // TRY
        // let f = File::open("bonjour.txt").unwrap();
        // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ~

        // TRY
        // let f = File::open("gutentag.txt").expect("Where is file?");
        // thread 'main' panicked at 'Where is file?: ~

        // `?`
        fn reader() -> Result<String, io::Error> {
            let mut f = File::open("hello.txt")?;
            let mut s = String::new();
            f.read_to_string(&mut s);
            Ok(s)
        }
        // Ok(/*hello.txt*/)

        fn reader2() -> Result<String, io::Error> {
            let mut s = String::new();
            File::open("hello.txt")?.read_to_string(&mut s);
            Ok(s)
        }
    }

    // Standard Development style
    {
        use std::error::Error;
        fn main() ->Result<(), Box<dyn Error>> {
            let f = File::open("hello.txt")?;
            Ok(())
        }
        // Box<dyn Error>> -> trait object == simply means all error
        // () is like empty set
    }

    // Simple linked list
    {
        println!("\n<< Simple Linked List >> \n");
        /*
        enum List {
            Nil,
            Cons(i32, List)
        }
        */
        // Error: recursive type has infinite size

        // (data, pointer(Box)) -> (data, pointer(Box)) -> (data, pointer) -> ...
        #[derive(Debug)]
        enum List {
            Nil,
            Cons(i32, Box<List>)
        }
        // Heap allocated
        // Box like (smart) pointer

        impl List {
            fn mysingleton(num: i32) {
                // ..TODO..
            }
            fn mypush(self, num:i32) {
                // ..TODO..
            }
        }
        // let test = List::mysingleton(3) ==> Cons(3, Nil)
        // test.push(4)                    ==> Cons(3, Cons(4, Nil))





        impl List {
            fn singleton(num: i32) ->List {
                List::Cons(num, Box::new(List::Nil))
            }

            fn push(self, num:i32) ->List {
                List::Cons(num, Box::new(self))
            }
        }
        let test = List::singleton(10);
        println!("{:?}", test);
        println!("{:?}", test.push(20).push(3).push(6).push(100));

        // It's not perfect linked list
        // Because of Ownership
    }

    // Generic, Trait, and Lifetime
    // Super Duper Important
    {
        println!("\n<<Generic 1>>\n");

        // What is generic?

        let vec1: Vec<i32> = vec![1, 2, 3];
        let vec2: Vec<char> = vec!['a', 'b', 'c'];

        std::mem::drop(vec1);
        std::mem::drop(vec2);

        // Largest
        let number_list = vec![1, 2, 3, 4, 5];

        // C-style
        fn largest_int(number_list: &[i32]) -> i32 {
            let mut largest = number_list[0];
            for &num in number_list.iter() {
                if num > largest {
                    largest = num;
                }
            }
            largest
        }
        // Go? ->implicit type trans
        println!("Largest number is: {:?}", largest_int(&number_list));

        let char_list = vec!['a', 'b', 'c', 'd'];
        // println!("{:?}", largest_int(char_list));
        // Error: mismatched type
    }

    // Generic 1 -- Generic in function
    {
        /*
        fn largest<T>(list: &[T]) ->T {
            let mut largest = list[0];

            for &item in list.iter() {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }
        */
        
        // Error: binary operation `>` cannot be applied to type `T`
        // Arbitrary type T can't Ordering
        // std::cmp::PartialOrd
    }

    // Generic 2 -- Generic in struct, and enum
    {
        println!("\n<<Generic 2>>\n");

        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        let point_int = Point { x: 5, y: 10 };
        let point_float = Point { x: 1.5, y: 5. };
        let point_test = Point { x: "St", y: "sd" };

        // let point_mix = Point { x: 3, y: 3.5 };
        // expected integer, found floating-point number

        #[derive(Debug)]
        struct Point2<T, U> {
            x: T,
            y: U,
        }
        let new_point_mix = Point2 {x: 3, y: 3.5};
        println!("{:?}", new_point_mix);

        /*
        enum Option<T> {
            Some(T),
            None,
        }
        */
        let a = Some(3);
        let b = Some("King");
        let c = Some('a');
        let d = Some(new_point_mix);

        // -----
        // impl with generic type
        impl<T> Point<T> {
        //   ^~~ Ready for Generic
            fn first(&self) ->&T {
                &self.x
            }
        }

        /*
        impl<T> Point<i32> {
            fn ipriny(&self) ->() {
                println!("iprinty - {:?}", &self.x);
            }
        }
        */
        // Error: the type parameter `T` is not constrained by the impl trait, self type, or predicates

        /*
        impl<T> Point {
            fn tpriny(&self) ->() {
                println!("tprinty - {:?}", &self.x);
            }
        }
        */
        // Error: wrong number of type arguments: expected 1, found 0

        impl Point<f32> {
            fn distance_from_origin(&self) ->f32 {
                (&self.x.powi(2) + &self.y.powi(2)).sqrt()
            }
        }


        // ---
        /*
        struct Point2<T, U> {
            x: T,
            y: U,
        }
        */

        // Use other generic types that defined
        impl<T, U> Point2<T, U> {
            fn mixup<V, W> (self, other: Point2<V, W>) ->Point2<T, W> {
                Point2 {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point2 {x: 5, y: "Hello"};
        let p2 = Point2 {x: 8.5, y: vec![1,2,3]};
        let p3 = p1.mixup(p2);
        println!("{:?}", p3);

        // STL -> C++?
        // Standard Template Library

        

    }
}
