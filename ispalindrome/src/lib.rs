extern crate getopts;

mod selfzip;
mod charextension;
pub mod args;

use selfzip::IntoSelfZip;
use charextension::CharExtension;

pub struct PalindromeParser {
    pub ignore_case: bool,
    pub ignore_spaces: bool
}

impl PalindromeParser {
    fn compare_iter<L, C, T>(&self, mut iter: T) -> bool
        where
            L: PartialEq,
            C: PartialEq<C> + CharExtension<Owned=L>,
            T: Iterator<Item=(C, C)>
    {
        if self.ignore_case {
            iter.all(|(a, b)| a.to_lower() == b.to_lower())
        } else {
            iter.all(|(a, b)| a == b)
        }
    }

    pub fn parse_line<L, C, T, I>(&self, line: I) -> bool
    where
        L: PartialEq,
        C: PartialEq<C> + CharExtension<Owned=L>,
        T: DoubleEndedIterator<Item=C>,
        I: IntoIterator<Item=C, IntoIter=T>
    {
        let iter = line.into_iter();

        if self.ignore_spaces {
            self.compare_iter(iter.filter(|c| !c.is_whitespace()).self_zip())
        } else {
            self.compare_iter(iter.self_zip())
        }
    }
}