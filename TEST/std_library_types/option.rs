

fn checked_division(dividend: i32, divisor: i32) ->Option<i32> {
    if divisor == 0 { None }
    else { Some(dividend/divisor) }
}

fn try_division(dividend: i32, divisor: i32) {
    match checked_division(dividend, divisor) {
        None => println!("DIV by zero"),
        Some(anw) => println!("{}/{} == {}", dividend, divisor, anw),
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    let option_float = Some(0f32);
    let option_float_n: Option<f32> = None;

    println!("{:?} => {:?}", option_float, option_float.unwrap());
    // Panic b/c unwarp None
    // println!("{:?} => {:?}", option_float_n, option_float_n.unwrap());
}