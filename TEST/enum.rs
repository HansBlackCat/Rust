#![allow(dead_code)]

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

enum GasGasGasIGonnaStepOnAGas {
    Add,
    Subtract,
}
impl GasGasGasIGonnaStepOnAGas {
    fn run(&self, x: i64, y: i64) ->i64 {
        match self {
            Self::Add => x+y,
            Self::Subtract => x-y,
        }
    }
}
type GGG = GasGasGasIGonnaStepOnAGas;

enum ClikeEnum {
    Zero,
    One,
    Two,
}
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad    => println!("page loading"),
        WebEvent::PageUnload  => println!("can't load page"),
        WebEvent::KeyPress(c) => println!("pressing {}", c),
        WebEvent::Paste(s)    => println!("pasted {}", s),
        WebEvent::Click {x, y}    => println!("click {}, {}", x, y),  
    }
}

fn main() {
    use crate::GasGasGasIGonnaStepOnAGas;

    let press = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("Curry".to_owned());
    let click = WebEvent::Click {x: 10, y: 255};
    let load = WebEvent::PageLoad;

    let ad = GGG::Add;
    let ad2 = ad.run(10, 40);
    println!("add.run {}", ad2);

    inspect(press);

    println!("zero is {}", ClikeEnum::Zero as i32);
    println!("one is {}", ClikeEnum::One as i32);
    println!("two is? {}", ClikeEnum::One as i32 +1);
    // Can't cast as char
    // println!("two as chr is {}", ClikeEnum::Two as char);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("roses pure print {}", Color::Red as i32);
    println!("what is :06? {:10}", Color::Red as i32);
}