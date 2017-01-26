use std::io::Read;
use std::iter::Peekable;

struct Compressor<I: Iterator> {
    iter: Peekable<I>
}

impl <C: PartialEq, I: Iterator<Item=C>> Iterator for Compressor<I> {
    type Item = (usize, C);

    fn next(&mut self) -> Option<(usize, C)> {
        let mut count = 0;
        loop {
            match self.iter.next() {
                Some(value) => {
                    match self.iter.peek() {
                        Some(new_value) if new_value == &value => {
                            count = count + 1;
                        },
                        _ => {
                            return Some((count+1, value))
                        }
                    }
                },
                None => {
                    break
                }
            }
        }
        None
    }
}