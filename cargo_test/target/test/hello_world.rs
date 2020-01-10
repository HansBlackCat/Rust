use std::fmt;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Struc(i32);
impl fmt::Display for Struc {
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let name = "JB";
    let age = 23;
    let lee = Person {name, age};
    //println!("{}",Lee);
    println!("{:?}",lee);
    println!("{:#?}",lee);

}

