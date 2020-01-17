use std::fmt;

fn printer<T: Display>(t: T) {
    println!("{}", t);
}
struct S<T: Display>(T);

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[derive(Debug)]
struct Triangle { length: f64, height: f64 }

trait HasArea {
    fn area(&self) ->f64;
}
impl HasArea for Rectangle {
    fn area(&self) ->f64 { self.length * self.height }
}
impl HasArea for Triangle {
    fn area(&self) ->f64 { self.height * self.length / 2.0 }
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}
fn area<T: HasArea>(t: &T) ->f64 { t.area() }

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Red for Turkey {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T) ->&'static str { "red" }
fn blue<T: Blue>(_: &T) ->&str { "blue" }

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: '{:?}'", t);
    println!("Display: '{}'", t);
}
fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: '{:?}'", t);
    println!("u: '{:?}'", u);
}

/*
impl fmt::Display for array {
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt:;Result {
        write!(f, )
    }
}
*/

fn main() {
    // example) fail cause vector can't Display
    // let s = S(vec![1,2,3]);

    let rec = Rectangle { length: 3.0, height: 4.0 };
    let tri = Triangle { length: 3.0, height: 2.7 };

    print_debug(&rec);
    println!("Area: {}", area(&rec));

    print_debug(&tri);
    println!("Area: {}", area(&tri));

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let turkey = Turkey;
    
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    println!("A turkey is {}", red(&turkey));

    let string = "words";
    let array = [1,2,3];
    let vec = vec![1,2,3];

    compare_prints(&string);
    compare_prints(&array);

}
