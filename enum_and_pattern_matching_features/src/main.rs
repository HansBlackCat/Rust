
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
enum Message {
    Quit,
    Move { x:i32, y:i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {

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

}
