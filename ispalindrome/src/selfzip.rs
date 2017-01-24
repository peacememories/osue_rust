pub struct SelfZip<I> {
    iter: I
}

impl <I: DoubleEndedIterator> Iterator for SelfZip<I>
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.iter.next(), self.iter.next_back()) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None
        }
    }
}

pub trait IntoSelfZip : DoubleEndedIterator {
    fn self_zip(self) -> SelfZip<Self> where Self: Sized {
        SelfZip {iter: self}
    }
}

impl <I> IntoSelfZip for I where I: DoubleEndedIterator {}