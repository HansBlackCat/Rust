

fn main() {
    let long_lived_binding = 1;
    {
        println!("inner init long: {}", long_lived_binding);

        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        let long_lived_binding = 5f32;
        println!("inner long {}", long_lived_binding);
    }
    println!("outter long: {}", long_lived_binding);

    let a_binding;
    {
        let c = 10;
        a_binding = c * c;
    }
    println!("a binding: {}", a_binding);
}