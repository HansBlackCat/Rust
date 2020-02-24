extern crate ansi_term;

use ansi_term::Colour::*;
// namespace

fn main() {
    println!("{}", RGB(12, 200, 30).paint("a red string"));
}
