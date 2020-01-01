use std::io;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(err) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem Creating Files: {:?}", e),
            }
            other_error => panic!("Problem Opening Files: {:?}", other_error),
        }
    };
}
