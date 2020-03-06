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

    // String
    {
    println!("\n<< STRING >>\n");
    // String =/= &str
    // String is kind of `struct` contains &str, length, capacity

    // Method 1
    let mut s1 = String::new();
    let s1 = "Only for you~".to_owned();
    println!("{}", s1);

    // Method 2 (Easier)
    let mut s2 = String::from("반짝이는 그대에게로~");
    println!("{}", s2);

    // String support UTF-8
    let hello_fromJap = String::from("こんにちは");
    let hello_fromRus = String::from("Здравствуйте");
    println!("Hello Japan: {}", hello_fromJap);
    println!("Hello Rusia: {}", hello_fromRus);

    // Concat
    let mut s3 = s1 + " " + &s2;
    //                      ^~~~ don't forget `&`
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
        //scores.insert(12, true);

        // Mixing
        let teams = vec![String::from("Blue"), String::from("Red")];
        let scores = vec![10, 15];

        let scores: HashMap<_,_> = teams.iter().zip(scores.iter()).collect();
        println!("{:?}", scores);
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
        let vec = vec![1,2,3];
        // println!("{:?}", vec[99]);
        // thread 'main' panicked at 'index out of bounds

        // But why? we can't find this error in compile time?
        // Boundary check problem -> Make overhead
        // Bounds-check elimination
        

        fn give_gift(gift: String) {
            if gift == "moon".to_owned() { panic!("Fuck you") }
            println!("Oh. thanks you!");
        }
        give_gift(String::from("moone"));
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
        wrap_gift(Some(&String::from("Yooman")));
        wrap_gift(None);

        fn wrapper(wrappee: String) ->Option<String> {
            if wrappee.is_empty() { None }
            else { Some(wrappee) }
        }
        println!("{}", wrapper(String::from("hello")).unwrap());
    }
}
