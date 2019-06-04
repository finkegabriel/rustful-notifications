use serde_json::{Result, Value};
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::ffi::OsStr;

fn main() -> std::io::Result<()> {
    let mut file = File::open("todo.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //-----------------------------------------------------------------------------// 
    let js = contents;
    let v: Value = serde_json::from_str(&js)?;
    let data =&v["event"][0].to_string(); 

    Command::new("notify-send")
                    .arg("-t")
                    .arg("0")
                    .arg(data)
                    .output().unwrap_or_else(|e| {
                        panic!("falied to run: {}",e);
                    });
    Ok(())
}
