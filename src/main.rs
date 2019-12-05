extern crate oss_advent_code_19;

use std::env;
use std::process;

// Will have menu control and call of different functions from the lib.rs
// need flexibility on calls
// Answer: Lets just make a big match
fn main() {
    println!("Welcome to OssAdventOfCode 2019");
    println!("Have fun!\n");

    let args: Vec<String> = env::args().collect();
    if let Err(e) = oss_advent_code_19::work_to_do(args) {
        println!("Application error:\n {}", e);
        process::exit(1);
    }
}
