
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x:i32, y:i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message { fn call(&self) { } }

enum Coin {
    Penny, Nickel, Dime, Quarter(Message),
}
fn value_in_cents(coin: Coin) ->u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => { println!("And largest"); 25 },
    }
}
fn plus_one(x: Option<i64>) -> Option<i64> {
    match x {
        Some(x) => Some(x+1),
        None => None
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    let home_u8 = IpAddr::V4(127, 0, 0, 1);

    let meswrite = Message::Write(String::from("hello?"));
    meswrite.call();

    let maybe_number = Some(50);
    let maybe_string = Some("Maybe String");
    let absent_number: Option<i32> = None;

    let penny_parker:Coin = Coin::Penny;
    value_in_cents(penny_parker);

    let op_64: Option<i64> = Some(64);
    plus_one(op_64);

    let some_u8_value = Some(4);
    if let Some(3) = some_u8_value {
        println!("Three");
    }

    let coin = Coin::Quarter(Message::Quit);
    let mut count = 0;
    match coin {
        Coin::Quarter(Quit) => println!("Message quarter from {:?}", Message::Quit),
        _ => count += 1,
    }

}