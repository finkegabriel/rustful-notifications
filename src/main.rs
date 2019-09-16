use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::ffi::OsStr;
extern crate chrono;
use chrono::{DateTime, Utc};
use chrono::prelude::*;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> =env::args().collect();
    let mut file = File::open(&args[1].to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let v: Value = serde_json::from_str(&contents)?;
    let m = v["event"].to_string() + " at " + &v["time"].to_string();
    println!("{}",&v["time"]);
    let now = Local::now();
    let cuTime = now.format("%I");
    
   if cuTime.to_string() == v["time"].to_string() {
                    Command::new("notify-send")
                    .arg("-t")
                    .arg("0")
                    .arg(m)
                    .output().unwrap_or_else(|e| {
                        panic!("falied to run: {}",e);
                    });

        
    }else{
        println!("{}","not the same");
     }

        Ok(())

}
