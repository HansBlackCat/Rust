use std::fmt;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// Unit struct
struct Nil;

// Tuples struct
struct Pair(i32, i32);

// Struct with two point
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result {
        let (a, b, c, d) = (self.top_left.x, self.top_left.y, self.bottom_right.x, self.bottom_right.y);
        write!(f, "({}, {})\n({}, {})", a, b, c, d)
    }
}

// impl Rectangle {
fn rect_area(rec_info: Rectangle) ->f32 {
    let (a, b) = (rec_info.top_left, rec_info.bottom_right);
    let delta_x = b.x - a.x;
    let delta_y = a.y - b.y;

    delta_x * delta_y
}

fn square(pnt: Point, vol: f32) ->Rectangle {
    Rectangle {
        top_left: Point { x: pnt.x, y: pnt.y + vol },
        bottom_right : Point { x: pnt.x + vol, y: pnt.y },
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);
    
    let point: Point = Point { x: 3.5, y: 10.5 };
    println!("Point x: {}, Point y: {}", point.x, point.y);

    let bottom_right = Point { x: 5.5, ..point };
    println!("Bottom_right x: {}, y: {}", bottom_right.x, bottom_right.y);

    let _rectangle = Rectangle {
        top_left: Point { x: 3.0, y: 5.5 },
        bottom_right: Point { x:5.0, y: 1.5 },
    };

    let _nil = Nil;
    // println!("what is unit struct?: {:?}", _nil);

    let pair = Pair(1, 9);

    let Pair(integer, decimal) = pair;
    println!("Destructure a tuple struct: {} &{}", integer, decimal);
    
    let area = rect_area(_rectangle);
    println!("{}", area);

    let square_init = Point { x: 1.0, y: 2.0 };
    let vol_multi: f32 = 3.5;
    let sq = square(square_init, vol_multi);
    println!("{}", sq);
}
