extern crate chrono;

use std::io;
use std::{env, fs};
use chrono::prelude::*;
use std::process::Command;

fn main() ->io::Result<()> {
    let local: String = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("{:?}", local);

    let current_dir = env::current_dir()?;
    println!("Current directory is: \n{:?}\n", current_dir);

    for entry in fs::read_dir(current_dir)? {
        let entry_in_string = entry.unwrap().path();
        println!("{:?}", entry_in_string);
        let metadata = fs::metadata(entry_in_string)?;
        println!("{:?}", metadata.is_dir());
    }

    let dir_list = 
        Command::new("sh")
            .arg("-c")
            .arg("ls > list.txt")
            .output()
            .expect("failed to execute `ls`");
    dir_list.stdout;

    let output = if cfg!(target_os = "macos") {
        Command::new("sh")
            .arg("-c")
            .arg("git log --oneline > hello.txt")
            // pub fn output(&mut self) -> Result<Output>
            .output()
            .expect("failed to execute process")
    } else {
        panic!("Only in MacOs")
    };
   // output.stdout;

   Ok(())
}
