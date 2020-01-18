#[allow(dead_code)]
#[derive(Clone, Copy)]

struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

// take reference to a book
fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition",
        book.title, book.year);
}
fn new_edition(book: &mut Book) {
    book.year = 2020;
    println!("I mutably borrowed {} - {} edition",
        book.title, book.year);
}

struct Point { x: i32, y: i32, z: i32 }
impl Point {
    fn new() ->Point{
        Point { x: 0, y: 0, z: 0 }
    }
}

fn main() {
    // ------------------------- Mutability -------------------------
    let immutabook = Book {
        author: "Lee",
        title: "Rustcean",
        year: 2018,
    };
    // copy
    let mut mutabook = immutabook;

    borrow_book(&immutabook);
    borrow_book(&mutabook);

    new_edition(&mut mutabook);
    // new_edition(&mut immutabook)

    // ------------------------- Frozen -------------------------
    let mut _mutable_integer = 7i32;
    {
        let large_integer = &_mutable_integer;

        // _mutable_integer = 50;

        println!("Immutably borrowed {}", large_integer);
    }
    _mutable_integer = 3;
    
    // ------------------------- Aliasing -------------------------
    println!("");
    let mut point = Point::new();

    let borrowed_point = &point;
    let another_borrow = &point;
    println!("Point has coordinates: ({}, {}, {})",
        borrowed_point.x, borrowed_point.y, borrowed_point.z
    );

    // Error bc line 
    // let mutable_borrow = &mut point;

    println!("Point has coordinates: ({}, {}, {})",
        borrowed_point.x, borrowed_point.y, borrowed_point.z
    );

    let mutable_borrow = &mut point;
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // let y = &point.y;
    // println!("Point Z coordinate is {}", point.z);

    println!("Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}

