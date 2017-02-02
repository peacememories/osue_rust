extern crate mycompression;

use std::io;
use std::io::Read;
use std::io::Write;
use mycompression::Compressor;
use mycompression::Counter;

struct Stat {
    read: usize,
    written: usize
}

fn compress<R: Read, W: Write>(reader: R, writer: W) -> io::Result<Stat> {
    let mut read = 0;
    let mut counter = Counter::new(writer);
    {
        let mut compressor = Compressor::new(&mut counter);
        for res in reader.bytes() {
            let byte = try!(res);
            read += 1;
            try!(compressor.write(&[byte]));
        }
        try!(compressor.flush());
    }
    Ok(Stat{
        read: read,
        written: counter.written()
    })
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let res = compress(stdin.lock(), stdout.lock()).unwrap();
    println!("");
    println!("{}, {}", res.read, res.written);
}