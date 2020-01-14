
type NanoSecond = u64;

fn main() {
    let x = 1u8;
    let y = 2u32;
    let z: NanoSecond = 3 as NanoSecond;
    let a = 9.8f64;

    let i = 1;
    let j = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `a` in bytes: {}", std::mem::size_of_val(&a));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `j` in bytes: {}", std::mem::size_of_val(&j));
    
    let elem = 5u8;
    let mut vec: Vec<_> = Vec::new();

    vec.push(elem);
    println!("{:?}", vec);   
}