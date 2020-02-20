mod learning_programming_language;
mod something_new;

// pub use can re-export 
pub use crate::learning_programming_language::self_study;
fn main() {
    println!("Hello, world!");
    crate::learning_programming_language::self_study::buying_books();
    self_study::buying_books();

    crate::something_new::zzarithea::choigoya();
    crate::something_new::treetree::namunamu();
}
