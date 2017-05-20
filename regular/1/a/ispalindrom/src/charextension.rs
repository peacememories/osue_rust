use std::ascii::AsciiExt;

pub trait CharExtension {
    type Owned;

    fn is_whitespace(&self) -> bool;
    fn to_lower(self) -> Self::Owned;
}

impl <'a> CharExtension for &'a u8 {
    type Owned = u8;

    #[inline]
    fn is_whitespace(&self) -> bool {
        **self == b' '
    }

    #[inline]
    fn to_lower(self) -> Self::Owned {
        self.to_ascii_lowercase()
    }
}

impl CharExtension for u8 {
    type Owned = u8;

    #[inline]
    fn is_whitespace(&self) -> bool {
        *self == b' '
    }

    #[inline]
    fn to_lower(self) -> Self::Owned {
        self.to_ascii_lowercase()
    }
}

impl <'a> CharExtension for &'a str {
    type Owned = String;

    #[inline]
    fn is_whitespace(&self) -> bool {
        *self == " "
    }

    #[inline]
    fn to_lower(self) -> Self::Owned {
        self.to_lowercase()
    }
}