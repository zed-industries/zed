use rand::prelude::*;
use std::cmp::Ordering;

pub fn pre_inc(value: &mut usize) -> usize {
    *value += 1;
    *value
}

pub fn post_inc(value: &mut usize) -> usize {
    let prev = *value;
    *value += 1;
    prev
}

pub fn find_insertion_index<'a, F, T, E>(slice: &'a [T], mut f: F) -> Result<usize, E>
where
    F: FnMut(&'a T) -> Result<Ordering, E>,
{
    use Ordering::*;

    let s = slice;
    let mut size = s.len();
    if size == 0 {
        return Ok(0);
    }
    let mut base = 0usize;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        // mid is always in [0, size), that means mid is >= 0 and < size.
        // mid >= 0: by definition
        // mid < size: mid = size / 2 + size / 4 + size / 8 ...
        let cmp = f(unsafe { s.get_unchecked(mid) })?;
        base = if cmp == Greater { base } else { mid };
        size -= half;
    }
    // base is always in [0, size) because base <= mid.
    let cmp = f(unsafe { s.get_unchecked(base) })?;
    if cmp == Equal {
        Ok(base)
    } else {
        Ok(base + (cmp == Less) as usize)
    }
}

pub struct RandomCharIter<T: Rng>(T);

impl<T: Rng> RandomCharIter<T> {
    pub fn new(rng: T) -> Self {
        Self(rng)
    }
}

impl<T: Rng> Iterator for RandomCharIter<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.gen_bool(1.0 / 5.0) {
            Some('\n')
        } else {
            Some(self.0.gen_range(b'a'..b'z' + 1).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_insertion_index() {
        assert_eq!(
            find_insertion_index(&[0, 4, 8], |probe| Ok::<Ordering, ()>(probe.cmp(&2))),
            Ok(1)
        );
    }
}
