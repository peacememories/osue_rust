extern crate getopts;
extern crate multi_reader;

use getopts::Options;
use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::env;
use multi_reader::MultiReader;

macro_rules! panic_userfacing {
    ($($stuff:tt)*) => {
        {
            use std::io::Write;
            let _ = writeln!(::std::io::stderr(), $($stuff)*);
            ::std::process::exit(1);
        }
    }
}

fn usage(program_name: &str, opts: &Options) -> String {
    format!("{} [infile...]", opts.short_usage(program_name))
}

fn main() {
    let mut args = env::args();
    let program_name = args.next()
        .map(Cow::from)
        .unwrap_or("mycompression".into());

    let mut opts = Options::new();

    opts.optopt("o", "", "Write output to a file", "outfile");
    
    let matches = opts.parse(args).unwrap_or_else(
        |err| {
            panic_userfacing!("{}\n{}", err, usage(&program_name, &opts));
        }
    );

    let infiles = matches.free
        .into_iter()
        .map(File::open)
        .collect::<Result<Vec<File>,_>>()
        .unwrap_or_else(
            |err| {
                panic_userfacing!("Failed to open file: {}", err);
            }
        );

    if infiles.is_empty() {
        
    } else {
        
    }

    println!("Hello, world!");
}
