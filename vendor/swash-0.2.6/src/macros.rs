/// Macro for consistent implementation of font resource iterators.
macro_rules! impl_iter {
    ($name:ident, $item:ident) => {
        impl<'a> Iterator for $name<'a> {
            type Item = $item<'a>;

            fn size_hint(&self) -> (usize, Option<usize>) {
                let remaining = self.len - self.pos;
                (remaining, Some(remaining))
            }

            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                let pos = self.pos.checked_add(n)?;
                if pos >= self.len {
                    self.pos = self.len;
                    None
                } else {
                    self.pos = pos + 1;
                    self.get(pos)
                }
            }

            fn next(&mut self) -> Option<Self::Item> {
                if self.pos >= self.len {
                    None
                } else {
                    let pos = self.pos;
                    self.pos += 1;
                    self.get(pos)
                }
            }
        }

        impl<'a> ExactSizeIterator for $name<'a> {
            fn len(&self) -> usize {
                self.len - self.pos
            }
        }
    };
}
