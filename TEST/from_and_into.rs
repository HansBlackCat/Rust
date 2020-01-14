use std::convert::{From, TryFrom, TryInto};
use std::fmt;

#[derive(Debug)]
struct Number {
    value: i32,
}
impl From<i32> for Number {
    fn from(item: i32) ->Self {
        Number { value: item }
    }
}

struct EvenNumber(i32);
impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) ->Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        }
        else {
            Err(())
        }
    }
}

struct Circle { radius: i32 }
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

#[test]
fn name() {
    unimplemented!();
}
fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);

    let num = Number::from(30);
    println!("My num is {:?}", num);

    let integ = 5;
    let num2: Number = integ.into();
    println!("My num2 is {:?}", num2);

    /* Need cargo test
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(10), Ok(EvenNumber(10)));

    let res1: Result<EvenNumber, ()> = 20i32.try_into();
    assert_eq!(Ok(EvenNumber(20)), res1);
    */    

    let cir = Circle { radius: 10 };
    println!("{}", cir);

    let trim_pared_str: i32 = " 13 ".trim().parse().unwrap();
    let trim_pared_str2 = " 19".trim().parse::<i32>().unwrap();
    println!("{}& {} sum: {}", trim_pared_str, trim_pared_str2, trim_pared_str+ trim_pared_str2);
}