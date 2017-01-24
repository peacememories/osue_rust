use std::io::Read;

struct Compressor<C, I> {
    last_char: Option<C>,
    iter: I
}

impl <C: PartialEq<C> + Clone, I: Iterator<Item=C>> Iterator for Compressor<C, I> {
    type Item = (usize, C);

    fn next(&mut self) -> Option<(usize, C)> {
        let mut count = 1;
        for new_val in self.iter.by_ref() {
            match self.last_char {
                Some(ref old_val) if &new_val == old_val => {
                    count = count+1;
                },
                Some(_) => {
                    self.last_char = Some(new_val.clone());
                    return Some((count, new_val));
                }
                None => {
                    self.last_char = Some(new_val);
                }
            }
        }
        match self.last_char {
            Some(ref old_char) => Some((count, old_char.clone())),
            None => None
        }
    }
}