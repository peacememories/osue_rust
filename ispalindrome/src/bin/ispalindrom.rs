extern crate ispalindrome;

use std::io;
use std::io::prelude::*;
use std::env;
use std::process;
use ispalindrome::args;

fn main() {
    let parser = match args::parse(env::args()) {
        Ok(p) => p,
        Err(err) => {
            writeln!(&mut io::stderr(), "{}", err).unwrap();
            process::exit(1);
        }
    };


    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let ln = line.unwrap();
        if !ln.is_empty() {
            if parser.parse_line(ln.bytes()) {
                println!("{} is a palindrome", ln);
            } else {
                println!("{} is not a palindrome", ln);
            }
        }
    }
}
