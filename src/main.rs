extern crate chrono;
extern crate date_time;
use std::fs::File;
use std::ffi::OsStr;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use date_time::time_tuple::{Duration, Time, TimeTuple};
use std::process::Command;
use serde_json::{Result, Value};
use chrono::prelude::*;
use chrono::Local;

#[derive(Serialize, Deserialize)]
struct Data {
    event:Vec<String>,
    time:Vec<String>
}

fn notify(msg: &str,time: &str){
    let m = msg.to_string()+" at "+time;
    Command::new("notify-send")
    .arg("-t")
    .arg("0")
    .arg(m)
    .output().unwrap_or_else(|e| {
        panic!("falied to run: {}",e);
    });
}  

fn todaysData(){
    
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("src/todo.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let js = contents;
    let d: Data = serde_json::from_str(&js)?;
    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));
    let date = Local::now();
    let systemTime: String = date.format("%H:%M").to_string();  
    let timeRn = [&date.format("%H:%M").to_string()];
    
        if d.event.len() != 0 {
                for (index) in d.time.iter().enumerate() {
                        // println!("index: {:?}", index); //uncomment to debug
                        if(index.1 == timeRn[0]){
                             notify(&d.event[index.0],&d.time[index.0]);
                        }
                }
        }else{
            notify("You're free","")
        }
    

    Ok(())
}