

fn main() {
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram:\n {}\n", pangram);

    for word in pangram.split_whitespace().rev() {
        print!("{} ", word);
    }
    println!("");

    for i in pangram.chars().rev() {
        print!("{}", i);
    }
    println!("");

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();
    println!("{:?}\n", chars);

    let mut strkk = String::new();
    for c in chars {
        strkk.push(c);
        strkk.push_str(", ");
    }
    println!("{:?}\n", strkk);

    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_string: &str = strkk.trim_matches(chars_to_trim);
    println!("{:?}", trimmed_string);

    let byte_escape = "I'm writing \x52\x75\x73\x74?";
    println!("What is this\x3F (\\x3F == ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let cjaracter_name = "\"CAPPA\"";
    println!("Unicode: {}", unicode_codepoint);
    println!("jarter: {}", cjaracter_name);

    let raw_string = r"Escape NOPE \u{211D} \xFF";
    println!("{:?}", raw_string);

    let quotes = r#"And there I said: "No Escape!""#;
    println!("{}", quotes);

    let longer_quotes = r###"A string with "# in it. even "##!""###;
    println!("{}", longer_quotes);
    println!("");

    {    
        use std::str;
        let bytestring: &[u8; 21] = b"this is a byte string";
        println!("A byte string: {:?}", bytestring);

        let escaped = b"\x52\x75\x73\x74 as bytes";
        println!("Some escaped bytes: {:?}", escaped);

        let raw_bytestring = br"\u{211D} is not escaped";
        println!("{:?}", raw_bytestring);

        if let Ok(my_str) = str::from_utf8(raw_bytestring) {
            println!("And the same as text: '{}'", my_str);
        }

        let _quotes = br#"You can also use "fancier" formatting, \
            like with normal raw string"#;
        
        let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82";
        println!("{:?}", &shift_jis);

        match str::from_utf8(shift_jis) {
            Ok(my_str) => println!("Conv: {:?}", my_str),
            Err(e) => println!("Conv fail: {:?}", e),
        };
    }

}