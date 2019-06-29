extern crate chrono;
extern crate date_time;
use serde_json::{Result, Value};
use std::thread::sleep;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::ffi::OsStr;
use chrono::prelude::*;
use date_time::time_tuple::{Duration, Time, TimeTuple};

fn main() -> std::io::Result<()> {  
    let mut file = File::open("todo.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //-----------------------------------------------------------------------------// 
    let mut foo = "".to_string();
    let js = contents;
    let v: Value = serde_json::from_str(&js)?;
    let data = &v["event"][0].to_string();
    let time = &v["event"][1].to_string();

    let tuple = TimeTuple::new(3, 0, 39);
    // assert_eq!(String::from("03:00"), tuple.to_hhmm_string())

    println!("{:?}",tuple);

    foo.push_str(&data);
    foo.push_str(" @ ");
    foo.push_str(&time);
    
    Command::new("notify-send")
                    .arg("-t")
                    .arg("0")
                    .arg(foo)
                    .output().unwrap_or_else(|e| {
                        panic!("falied to run: {}",e);
                    });

    Ok(())
}
