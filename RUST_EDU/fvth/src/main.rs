#![allow(dead_code,unused_variables)]

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {}
// not all trait items implemented, missing: `summarize`

impl Summary for NewsArticle {
    fn summarize(&self) ->String {
        format!("{}, by {} ({})",self.headline,self.author,self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) ->String {
        format!("{}: {}",self.username,self.content)
    }
}
// GOTO 1st

// --------------------------------------------------------------------

// trait with already completed
pub trait Summary2 {
    fn summarize2(&self) ->String {
        String::from("(Read more...)")
    }

}
impl Summary2 for Tweet {}
// GOTO 2nd

// --------------------------------------------------------------------

// trait as function arguments
pub fn easy_summarize<T:Summary>(item: T) {
    println!("Eassssy2 - {}", item.summarize());
}

// More simple case (do same thing)
pub fn easy_summarize2(item: impl Summary) {
    println!("Eassssy - {}", item.summarize());
}
// GOTO 3rd

// --------------------------------------------------------------------

// when you use many traits + generics + etc
// multiple generics with `+`
fn example_func<T, U>(t: T, u: U)
    where T: Clone,
          U: Clone + PartialEq + PartialOrd
{}

// resulting impl is also allowed
fn machine(b: bool) ->impl Summary {
    Tweet {
        username: String::from("Machine"),
        content: String::from("Learning"),
        reply: false,
        retweet: false,
    }
}

// Remember Generic function?
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
// `>` error
// `T` might need a bound for `std::cmp::PartialOrd`

fn largest<T: PartialOrd + Copy>(list: &[T]) ->T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// `T` is bound for PartialOrd and Copy

// GOTO 4th

// --------------------------------------------------------------------

use std::fmt::Display;
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) ->Self {
        Self {
            x,
            y,
        }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("Largest mem in x: {}", self.x);
        } else {
            println!("Largest mem in y: {}", self.y);
        }
    }
}

// GOTO 5th

// --------------------------------------------------------------------
#[derive(Debug)]
pub struct Num<T> {
    x: T,
    y: T,
}

pub trait ToFloat {
    fn to_float(&self) ->Num<f32>;
}
pub trait Plus {
    fn plus(&self) ->f32;
}
pub trait Purification {
    fn purge(&self) ->f32;
}

impl ToFloat for Num<i32> {
    fn to_float(&self) ->Num<f32> {
        Num {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}
impl Plus for Num<f32> {
    fn plus(&self) ->f32 {
        self.x + self.y
    }
}
impl<T: ToFloat> Purification for T {
    fn purge(&self) ->f32 {0.0}
}

// GOTO 6th



fn main() {
    // 1st
    println!("\n<1st>\n");
    let tweet = Tweet {
        username: String::from("ZUN"),
        content: String::from("Touhou"),
        reply: false,
        retweet: false,
    };
    println!("Summarize - {}",tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Seoul Cyber Univ"),
        location: String::from("Gwan-Ak"),
        author: String::from("YEB"),
        content: String::from("Onlne-class"),
    };
    println!("Summarize - {}",article.summarize());
    // BACK 1st
    
    // --------------------------------------------------------------------
    // 2nd
    println!("\n<2nd>\n");
    println!("{}",tweet.summarize2());
    // Back 2nd

    // --------------------------------------------------------------------
    // 3rd
    println!("\n<3rd>\n");
    easy_summarize(article);
    easy_summarize2(tweet);
    // BACK 3rd

    // --------------------------------------------------------------------
    // 4th
    println!("\n<4th>\n");
    let number_list = vec![34,50,25,10,77];
    println!("{}",largest(&number_list));

    let char_list = vec!['a','f','e','l'];
    println!("{}",largest(&char_list));

    let string_list = vec!["as","ert","gdf","plo"];
    println!("{}",largest(&string_list));
    // BACK 4th
    
    // --------------------------------------------------------------------
    // 5th
    println!("\n<5th>\n");
    let test_pair = Pair::new(30, 10);
    test_pair.cmp_display();
    // BACK 5th

    // --------------------------------------------------------------------
    // 6th
    println!("\n<6th>\n");
    let test_num = Num {
        x: 10,
        y: 12,
    };
    println!("{:#?}", test_num.to_float());
    println!("{:#?}", test_num.purge());

    // --------------------------------------------------------------------
    // --------------------------------------------------------------------
    // <LifeTime>
    println!("\n-- << LIFETIME >> --\n");

    {
        let r;
        {
            let x = 5;
            r = &x;
        }
        // println!("r is? {}", r);
    }

    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let we_want = longest(string1.as_str(),string2);
        println!("what we want: {}", we_want);

        /*
        fn longest2(x: &str, y: &str) ->&str {
            //                         ^~~ It's borred from x? or y?
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        // Error: expected lifetime parameter
        */

        { // Example
            let s = String::from("a");
            {
                let s2 = String::from("b");
                // let result = longest2(s.as_str(), s2.as_str());

                // s2 is dropped here
            }
            // s is dropped here
            // where `result` will be dropped?

        } // EndExample

        fn longest<'a>(x: &'a str, y: &'a str) ->&'a str {
            //    ^~~ Here ^~~ Here    ^~~ Here   ^~~ Here
            // Tot 4
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        { // Example
            let s = String::from("a");
            {
                let s2 = String::from("b");
                let result = longest(s.as_str(), s2.as_str());

                println!("longest: {}", result);
                // s2 is dropped here
            }
            // println!("longest: {}", result);
            // Error
            // `reult` already dropped! 

            // s is dropped here
        } // EndExample
        
        /*
        fn longest3<'a>(x: &'a str, y: &str) ->&'a str {
            //    ^~~ Here ^~~ Here             ^~~ Here
            // Tot 3
            if x.len() > y.len() {
                x
            } else {
                y
                // Error explicit lifetime required in the type of `y` lifetime `'a` required
                // don't know whether `y` dropped 
            }
        }
        */

        // longest3 function is like using value declared in function in C
        {
            { // Scope

            }
        }

        fn print_first<'a,'b>(x: &'a String, y: &'b String) ->&'a String {
            x
        }

        {
            let a = "a".to_owned();
            let b = "b".to_owned();
            let r = print_first(&a, &b);

            std::mem::drop(b);
            // value moved here

            // println!("{}", b);
            // Err: value borrowed here after move

            println!("{}", r);
        }
    }
}
