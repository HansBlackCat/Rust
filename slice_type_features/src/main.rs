
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);
    s.clear();

    let s = String::from("hello slicing");
    let hello = &s[..5];
    let slicing = &s[6..];

    let word2 = first_world_kai(&s);
    println!("{}", word2);

    let mut s_clear_test = String::from("hello world");
    let word3 = first_world_kai(&s_clear_test);
    println!("the first word is {}", word3);
    s_clear_test.clear();
    // println!("the first word is {}", word3); //already borrowed as immutable

    let s_literal = "Hello world without String_from";
    let s = String::from("Hello world with String_form");

    let word = first_world_kai_literal(&s[..]);
    let word = first_world_kai_literal(&s_literal);
    let word = first_world_kai_literal(&s_literal[..]);

    let a = [1,2,3,4,5];
    let sliced_a = &a[1..3];
}

fn first_word(s:&String) ->usize {
    let bytes = s.as_bytes();
    for (i, &items) in bytes.iter().enumerate() {
        if items == b' ' {
            return i;
        }
    }
    s.len()
}
fn first_world_kai(s: &String) ->&str {
    let bytes = s.as_bytes();
    for (i, &items) in bytes.iter().enumerate() {
        if items == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn first_world_kai_literal(s: &str) ->&str {
    let bytes = s.as_bytes();
    for (i, &items) in bytes.iter().enumerate() {
        if items == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
