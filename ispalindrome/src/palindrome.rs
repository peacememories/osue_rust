use selfzip::IntoSelfZip;
use unicode_segmentation::UnicodeSegmentation;

pub struct PalindromeParser {
    ignore_case: bool,
    ignore_spaces: bool
}

impl PalindromeParser {
    pub fn new(ignore_case: bool, ignore_spaces: bool) -> PalindromeParser {
        PalindromeParser {
            ignore_case: ignore_case,
            ignore_spaces: ignore_spaces
        }
    }

    fn compare_iter<'a, I: 'a + Iterator<Item=(&'a str, &'a str)>>(&self, mut iter: I) -> bool {
        if self.ignore_case {
            iter.all(|(a, b)| a.to_lowercase() == b.to_lowercase())
        } else {
            iter.all(|(a, b)| a == b)
        }
    }

    pub fn parse_line(&self, line: &str) -> bool {
        let iter = UnicodeSegmentation::graphemes(line, true);
        if self.ignore_spaces {
            self.compare_iter(iter.filter(|&c| c != " ").self_zip())
        } else {
            self.compare_iter(iter.self_zip())
        }
    }
}