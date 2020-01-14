use std::fs::File;
use std::ffi::OsStr;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("src/todo.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // //-----------------------------------------------------------------------------// 
    let js = contents;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(&js)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}