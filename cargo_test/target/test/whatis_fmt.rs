use std::fmt;

#[derive(Debug)]
struct MinMax (i64, i64);
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Veclist(Vec<i32>);
impl fmt::Display for Veclist {
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        write!(f, "]");
    }
}

fn main() {
    let minmax = MinMax(3,6);
    println!("Fmt   :  {}", minmax);
    println!("Debug : {:?}\n", minmax);

    let pointtod = Point2D {x:3.4, y:5.9};
    println!("Fmt   : {}", pointtod);
    println!("Debug : {:?}", pointtod);

    let veclist = Veclist(vec![1,3,5]);
    println!("{}", veclist);
}