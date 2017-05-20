use getopts;
use std::fmt;
use getopts::Options;
use getopts::Fail::UnrecognizedOption;
use ::PalindromeParser;

#[derive(Debug)]
pub enum Fail {
    Help(String),
    ParseError(getopts::Fail)
}

impl fmt::Display for Fail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Fail::Help(ref msg) => {
                write!(f, "{}", msg)
            },
            Fail::ParseError(ref err) => {
                write!(f, "{}\n", err)
            }
        }
    }
}

impl From<getopts::Fail> for Fail {
    fn from(err: getopts::Fail) -> Fail {
        Fail::ParseError(err)
    }
}

pub fn parse<I: Iterator<Item=String>>(mut args: I) -> Result<PalindromeParser, Fail> {
    let program_name = match args.next() {
        Some(p) => p,
        None => From::from("ispalindrome")
    };

    let brief = format!("Usage {} [-s] [-i]", program_name);

    let mut opts = Options::new();

    opts.optflag("i", "", "ignore case");
    opts.optflag("s", "", "ignore spaces");
    opts.optflag("h", "help", "print this message");

    let matches = try!(opts.parse(args));

    if !matches.free.is_empty() {
        return Err(Fail::ParseError(UnrecognizedOption(matches.free.join(", "))));
    }

    if matches.opt_present("h") {
        return Err(Fail::Help(opts.usage(&brief)))
    }

    Ok(PalindromeParser {
        ignore_case: matches.opt_present("i"),
        ignore_spaces: matches.opt_present("s")
    })
}