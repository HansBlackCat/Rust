use std::fmt::Display;


struct ImportantExcept<'a> {
    part: &'a str,
}
impl<'a> ImportantExcept<'a> { fn level(&self) ->i32 { 3 } }

fn main() {

    let r;
    {
        let x = 5;
        r = &x;
        println!("r in scope: {}", r);
    }
    // println!("r: {}", r); // x out of scope
    {
        let string1 = String::from("abcd");
        let string2 = "xyz";
        let string3 = String::from(string2);

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let string1 = String::from("AC");
    let temp;
    {
        let string2 = String::from("DC");
        temp = longest_v2(string1.as_str(), string2.as_str());
    }
    println!("{}", temp);

    let fnt = |x|x*x;
    let x_v: i64 = 64;
    println!("{}", test(x_v, fnt));

    //
    let novel = String::from("I'm your father. Or not");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcept { part: first_sentence };

    //
    let s: &'static str = "I Live FOREVER!!"; // Reference can live forever

    //
    let _temp = longest_with_an_announcement("AC", "DC", "Kung Fury");
}
fn longest<'a>(x: &'a str, y: &'a str) ->&'a str { if x.len() > y.len() { x } else { y } }
fn longest_v2<'a>(x: &'a str, y:& str) ->&'a str { x }
fn test(x: i64, y: fn(i64) ->i64) ->i64 { y(x) }

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display { println!("Announcement! {}", ann); if x.len() > y.len() { x } else { y } }

