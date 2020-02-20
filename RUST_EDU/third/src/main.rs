#![allow(dead_code)]
/* << Cargo structure >>
 * ----------------------
 * .
 * ├── Cargo.toml
 * └── src
 *     └── main.rs
 * 1 directory, 2 files
 * ---------------------
 */
///

// without pub first
// ALL DEFAULT MOD IS PRIVATE!
mod learning_programming_language {
    pub mod self_study {
        pub fn buying_books() {}
        fn get_online_course() {}
    }
    mod learn_with_teachers {
        fn go_to_class() {}
        fn take_a_seat() {}
        fn do_homework() {}
    }
}

/* << Module structure >>
 * crate (where you are!)
 * └── learning_programming_language
 *     ├── self_study
 *     │   └── buying_books
 *     │   └── get_online_course
 *     └── learn_with_teachers
 *         └── go_to_class
 *         └── take_a_seat
 *         └── do_homework
 */
///

pub fn im_in_library() {
    // Absolute path
    // if you're in ~/Documents/Git
    // cd User/myname/Documents/Git/Rust
    crate::learning_programming_language::self_study::buying_books();
    // Relative path
    // if you're in ~/Documents/Git
    // cd ./Rust
    learning_programming_language::self_study::buying_books();
}
// ! stdErr: module `self_study` is private
// @ make module public

mod break_time {
    fn listening_to_music() {}

    fn go_to_library() {
        // super -> parent module (Like cd ../)
        super::im_in_library();
        listening_to_music();
    }

    #[derive(Debug)]
    pub struct Music {
        pub rockstyle: String,
        pub citypop: String,
        nedasship: String,
    }
    impl Music {
        pub fn what_music_you_hear(&self) {
            println!("I'm hearing rockstyle music named {}", self.rockstyle);
            println!("And I'm hearing citypop music named {}", self.citypop);
        }
        // after music_start function
        pub fn youre_music(x: String) -> Music {
            Music {
                rockstyle: String::from("rock"),
                citypop: String::from("city"),
                nedasship: String::from(x),
            }
        }
    }

    // enum is all public
}

pub fn music_start() {
    /*
    let me = crate::break_time::Music {
        rockstyle: "Rock".to_owned(),
        citypop: "City".to_owned(),
        nedasship: "nedasship".to_owned(),
    };
    */
    // ! Warning, cause nedasship is private

    let mut me = break_time::Music::youre_music(String::from("Bad Apple"));
    me.rockstyle = String::from("Queen");
    println!("{:?}", me);
}
// main() music_start and run

fn main() {
    {
        println!("Hello, world!");
        music_start();
    }

    // use
    // like namespace
    {
        crate::learning_programming_language::self_study::buying_books();
    }
    {
        use crate::learning_programming_language::self_study;
        self_study::buying_books();
    }
    {
        use crate::learning_programming_language::self_study::*;
        buying_books();
    }
    {
        let mut map = std::collections::HashMap::new();
        map.insert(1, "abs");
        println!("{:?}", map);
    }
    {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(1, 2);
        println!("{:?}", map);
    }
    {
        use std::fmt;
        use std::io;
        fn func2() ->io::Result<()>{
            Ok(())
        }
    }
    {
        use std::io::Result as IOResult;
        fn func2() ->IOResult<()> {
            Ok(())
        }
    }
}
