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
use std::collections::HashMap;
use std::mem;


fn main() -> std::io::Result<()> {
    let args: Vec<String> =env::args().collect();
    let mut file = File::open(&args[1].to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let v: Value = serde_json::from_str(&contents)?;
     let now = Local::now();
    let cuTime = now.format("%I");

    for x in 0..2 {
        let mut y = &v[format!("{}",x)];
        let mut event = &y["event"];
        let mut time = &y["time"];
        let m = event.to_string();
        let l = time.to_string();
            
        if &cuTime.to_string() == &time.to_string() {
            println!("{}",&event);
                    Command::new("notify-send")
                    .arg("-t")
                    .arg("0")
                    .arg(m + " at " +&l)
                    .output().unwrap_or_else(|e| {
                        panic!("falied to run: {}",e);
                    });

        
    }else{
        println!("{}","not the same");
     }

    }
    println!("{}",&v["1"]["time"]);
   
        Ok(())

}
