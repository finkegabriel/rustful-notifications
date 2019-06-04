extern crate chrono;
use chrono::{Datelike, Timelike, Utc};
use serde_json::{Result, Value};
use std::thread::sleep;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::ffi::OsStr;
use chrono::prelude::*;

fn main() -> std::io::Result<()> {  
    let mut file = File::open("todo.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //-----------------------------------------------------------------------------// 
    let mut foo = "".to_string();
    let js = contents;
    let v: Value = serde_json::from_str(&js)?;
    let data = &v["event"][0].to_string();
    let date = &v["event"][1].to_string();

    foo.push_str(&data);
    foo.push_str(" @ ");
    foo.push_str(&date);
    
    Command::new("notify-send")
                    .arg("-t")
                    .arg("0")
                    .arg(foo)
                    .output().unwrap_or_else(|e| {
                        panic!("falied to run: {}",e);
                    });
    Ok(())
}
