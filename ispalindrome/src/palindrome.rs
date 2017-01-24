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

    pub fn parse_line(&self, line: &str) -> bool {
        let comp_ci = |(a, b): (&str, &str)| {
            a.to_lowercase() == b.to_lowercase()
        };

        let comp_cs = |(a,b): (&str, &str)| {
            a == b
        };

        let comp: &Fn((&str, &str))->bool = if self.ignore_case {
            &comp_ci
        } else {
            &comp_cs
        };

        if self.ignore_spaces {
            UnicodeSegmentation::graphemes(line, true).filter(|&c| c != " ").self_zip().all(comp)
        } else {
            UnicodeSegmentation::graphemes(line, true).self_zip().all(comp)
        }
    }
}