pub trait IterExt: Iterator {
    fn just_one(&mut self) -> Option<Self::Item> {
        let one = self.next()?;
        match self.next() {
            Some(_) => None,
            None => Some(one),
        }
    }
}

impl<T: Iterator> IterExt for T {}
