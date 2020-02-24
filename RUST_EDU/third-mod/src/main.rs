mod learning_programming_language;
// 1. something.rs
// 2. folder/mod.rs
mod something_new;

// main.rs -> crate

// pub use can re-export 
pub use crate::learning_programming_language::self_study::*;
fn main() {
    println!("Hello, world!");
    crate::learning_programming_language::self_study::buying_books();
    buying_books();

    crate::something_new::zzarithea::choigoya();
    crate::something_new::treetree::namunamu();
}
