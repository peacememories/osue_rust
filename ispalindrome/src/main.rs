extern crate getopts;
extern crate unicode_segmentation;

mod selfzip;
mod palindrome;
mod args;

use std::io;
use std::io::prelude::*;
use std::env;
use std::process;

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
            if parser.parse_line(&ln) {
                println!("{} is a palindrome", ln);
            } else {
                println!("{} is not a palindrome", ln);
            }
        }
    }
}
