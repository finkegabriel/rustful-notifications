use std::fs::File;
use std::ffi::OsStr;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    event: u8,
    time: String,
    data: Vec<String>,
}

fn notify(msg: &str,time: &str){
    Command::new("notify-send")
    .arg("-t")
    .arg("0")
    .arg(msg)
    .arg(time)
    .output().unwrap_or_else(|e| {
        panic!("falied to run: {}",e);
    });
}

fn check_time(time: &str){
    println!("{} ",time);
}   

fn main() -> std::io::Result<()> {
    let mut file = File::open("src/todo.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let js = contents;
    let p: Person = serde_json::from_str(&js)?;
    let mut m = &p.data[0];
    println!("apple {}",p.data.len());
    check_time(&p.time);
    notify(m,&p.time);

    Ok(())
}