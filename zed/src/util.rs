use rand::prelude::*;
use std::cmp::Ordering;

pub fn post_inc(value: &mut usize) -> usize {
    let prev = *value;
    *value += 1;
    prev
}

/// Extend a sorted vector with a sorted sequence of items, maintaining the vector's sort order and
/// enforcing a maximum length. Sort the items according to the given callback. Before calling this,
/// both `vec` and `new_items` should already be sorted according to the `cmp` comparator.
pub fn extend_sorted<T, I, F>(vec: &mut Vec<T>, new_items: I, limit: usize, mut cmp: F)
where
    I: IntoIterator<Item = T>,
    F: FnMut(&T, &T) -> Ordering,
{
    let mut start_index = 0;
    for new_item in new_items {
        if let Err(i) = vec[start_index..].binary_search_by(|m| cmp(m, &new_item)) {
            let index = start_index + i;
            if vec.len() < limit {
                vec.insert(index, new_item);
            } else if index < vec.len() {
                vec.pop();
                vec.insert(index, new_item);
            }
            start_index = index;
        }
    }
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

    #[test]
    fn test_extend_sorted() {
        let mut vec = vec![];

        extend_sorted(&mut vec, vec![21, 17, 13, 8, 1, 0], 5, |a, b| b.cmp(a));
        assert_eq!(vec, &[21, 17, 13, 8, 1]);

        extend_sorted(&mut vec, vec![101, 19, 17, 8, 2], 8, |a, b| b.cmp(a));
        assert_eq!(vec, &[101, 21, 19, 17, 13, 8, 2, 1]);

        extend_sorted(&mut vec, vec![1000, 19, 17, 9, 5], 8, |a, b| b.cmp(a));
        assert_eq!(vec, &[1000, 101, 21, 19, 17, 13, 9, 8]);
    }
}
