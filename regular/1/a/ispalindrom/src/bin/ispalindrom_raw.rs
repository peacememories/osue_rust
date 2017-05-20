extern crate ispalindrome;

use ispalindrome::PalindromeParser;

fn main() {
    let line = b"teset";
    let parser = PalindromeParser {
        ignore_case: false,
        ignore_spaces: false
    };

    parser.parse_line(line);
}