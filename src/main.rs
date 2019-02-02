extern crate clap;

use clap::{Arg, App};
use std::env;
use std::process::Command;
use std::io;

fn generateCal() {
    println!(" 
 ----------------------------------
              Calendar
 ----------------------------------
|  1 |  2 |  3 |  4 |  5 |  6 |  7 |
|----------------------------------|
|  8 |  9 | 10 | 11 | 12 | 13 | 14 |
|----------------------------------|
| 15 | 16 | 17 | 18 | 19 | 20 | 21 | 
|----------------------------------|  
| 22 | 23 | 24 | 25 | 26 | 27 | 28 |
|----------------------------------|
| 29 | 30 | 31 |  x |  x |  x |  x | 
 ----------------------------------  ")
}

fn work() {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    println!("What would you like to be notified about? ");
    let mut input = String::new();
     io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    Command::new("notify-send")
                    .arg("-t")
                    .arg("0")
                    .arg(input)
                    .output().unwrap_or_else(|e| {
                        panic!("falied to run: {}",e);
                    });
}


fn main() {

generateCal();

println!("What would you like to do?: ");

   let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    if input.trim() == "notify" {
        work();
    }else{
        println!("not a valid input");
    }
}