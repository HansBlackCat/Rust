#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;
    println!("{} -> integer: {} -> character: {}", decimal, integer, character);

    // Need #![allow(overflowing_literals)]
    println!("1000000 as u16: {}", 1000000 as u16);
    println!("257 as u8: {}", 257 as u8);
    println!("513 as u8: {}", 518 as u8);

    println!("-1 as u8: {}", (-1i32) as u8);
}