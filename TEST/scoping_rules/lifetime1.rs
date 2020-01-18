use std::fmt::Debug;

fn print_rf<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
    let _x = 12;
    // let y: &'a i32 = &_x;
}

fn print_one<'a>(x: &'a i32) {
    println!("'print_one': x is {}", x);
}

fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("'print_multi' x is {}, y is {}", x, y);
}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) ->&'a i32 { x }

// fn invalid_output<'a>() -> &'a String { &String::from("foo") }

struct Owner(i32);
impl Owner {
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) { println!("'print': {}", self.0); }
}

#[derive(Debug)]
struct Borrowed<'a>(&'a i32);
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

#[derive(Debug)]
struct BorrowedTraits<'a> {
    x: &'a i32,
}
// Annotate lifetimes to impl
impl<'a> Default for BorrowedTraits<'a> {
    fn default () ->Self {
        Self {
            x: &10,
        }
    }
}

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
fn print<T>(t: T) where T: Debug {
    println!("'print': t is {:?}", t);
}
fn print_ref<'a, T>(t: &'a T) where T: Debug + 'a {
    println!("'print_ref': t is {:?}", t);
}

fn multiply<'a: 'b, 'b>(first: &'a i32, _: &'b i32) ->&'b i32 {
    first
}
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) ->&'b i32 {
    first
}

// static
static NUM: i32 = 32;

fn coerce_static<'a>(_: &'a i32) ->&'a i32 {
    &NUM
}

fn elided_input(x: &i32) {
    println!("'elidied_input': {}", x);
}
fn same_with_elided_input<'a>(x: &'a i32) {
    println!("'annotated_input': {}", x);
}

fn elided_pass(x: &i32) ->&i32 { x }
fn same_with_elided_pass<'a>(x: &'a i32) ->&'a i32 { x }

fn main() {
    let (four, nine) = (4, 9);
    print_rf(&four, &nine);

    failed_borrow();

    // ------------------- Functions --------------------
    println!("");
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);

    // ------------------- Methods --------------------
    println!("");
    let mut owner = Owner(22);
    owner.add_one();
    owner.print();

    // ------------------- Structs --------------------
    println!("");
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
    
    // ------------------- Traits --------------------
    println!("");
    let b: BorrowedTraits = Default::default();
    println!("b is {:?}", b);

    // ------------------- Bounds --------------------
    println!("");
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);

    // ------------------- Coercion --------------------
    println!("");
    let first = 2;
    {
        let second = 3;
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };

    // ------------------- Static --------------------
    println!("");

    {
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);
    }
    {
        let lifetime_num = 9;
        let coerce_ss = coerce_static(&lifetime_num);
        println!("coerced_static: {}", coerce_ss);
    }
    println!("NUM: {}", NUM);

    // ------------------- Elision --------------------
    let x = 3;
    elided_input(&x);
    same_with_elided_input(&x);

    println!("{}", elided_pass(&x));
    println!("{}", same_with_elided_pass(&x));
}
