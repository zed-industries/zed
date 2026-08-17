use std::cmp;
use std::hash::Hash;

use hasher::ShingleHasher;

/// Shingles works well with slices and utf8 strings.
/// This structure can generate shingles with different steps and hashes of thees shingles.
///
/// # Examples
///
/// ```
/// use shingles::Shingles;
///
/// let v = [1, 2, 3, 4];
/// let mut num_sh = Shingles::new(&v[..], 3);
/// let mut str_sh = Shingles::new_with_step("привет!", 4, 2);
///
/// assert_eq!(Some(&v[0..3]), num_sh.next());
/// assert_eq!(Some(&v[1..4]), num_sh.next());
///
/// assert_eq!(Some("прив"), str_sh.next());
/// assert_eq!(Some("ивет"), str_sh.next());
///
/// // get vector of shingles, represented as u64 values
/// let sh_hashes: Vec<_> = Shingles::new("привет!", 4).hashes().collect();
/// ```
#[derive(Clone)]
pub struct Shingles<'a, T: ?Sized + 'a> {
    data: &'a T,
    size: usize,
    step: usize,
}

impl<'a, T: ?Sized> Shingles<'a, T> {
    pub fn new(data: &'a T, size: usize) -> Self {
        Shingles::new_with_step(data, size, 1)
    }

    pub fn new_with_step(data: &'a T, size: usize, step: usize) -> Self {
        Shingles {
            data: data,
            size: size,
            step: step,
        }
    }

    /// Returns iterator, which reproduces hashes of shingles.
    pub fn hashes<K>(self) -> ShingleHasher<Self, K>
        where Self: Iterator<Item = K>,
              K: Hash
    {
        ShingleHasher::new(self)
    }
}

impl<'a, T> Iterator for Shingles<'a, [T]> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() >= self.size {
            let ret = &self.data[0..self.size];
            let step = cmp::min(self.step, self.data.len());
            self.data = &self.data[step..];
            Some(ret)
        } else {
            None
        }
    }
}

impl<'a> Iterator for Shingles<'a, str> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pos_next: usize = 0;
        let mut pos_end: usize = 0;
        let mut chars = 0;

        // iterator reproduces char boundary positions and its appropriate bytes
        let iter = self.data.as_bytes().iter().enumerate()
            // only char boundaries
            .filter(|&(_, &b)| b < 128 || b >= 192);

        // get shingle end pos end step next pos at once
        for (i, _) in iter {
            if chars == self.step {
                pos_next = i;
            }
            if chars == self.size {
                pos_end = i;
            }
            if pos_next != 0 && pos_end != 0 {
                break;
            }
            chars += 1;
        }

        // try get shingle from data
        let ret = if pos_end != 0 {
            Some(&self.data[0..pos_end])
        } else if chars == self.size {
            Some(self.data)
        } else {
            None
        };

        // move data slice to next step position
        if pos_next != 0 {
            self.data = &self.data[pos_next..];
        } else {
            self.data = &self.data[self.data.len()..];
        }

        ret
    }
}

/// An interface for getting shingles from other types.
///
/// # Examples
///
/// ```
/// use shingles::AsShingles;
///
/// let v = [1, 2, 3, 4, 5];
/// let mut sh = v.as_shingles(3);
/// let mut s = "привет!".as_shingles_with_step(4, 2);
///
/// assert_eq!(Some(&v[0..3]), sh.next());
/// assert_eq!(Some(&v[1..4]), sh.next());
///
/// assert_eq!(Some("прив"), s.next());
/// assert_eq!(Some("ивет"), s.next());
/// ```
pub trait AsShingles<'a, T: ?Sized + 'a> {
    fn as_shingles(&'a self, size: usize) -> Shingles<'a, T>;
    fn as_shingles_with_step(&'a self, size: usize, step: usize) -> Shingles<'a, T>;
}

impl<'a, T: 'a> AsShingles<'a, [T]> for [T] {
    fn as_shingles(&'a self, size: usize) -> Shingles<'a, [T]> {
        Shingles::new(self, size)
    }

    fn as_shingles_with_step(&'a self, size: usize, step: usize) -> Shingles<'a, [T]> {
        Shingles::new_with_step(self, size, step)
    }
}

impl<'a> AsShingles<'a, str> for str {
    fn as_shingles(&'a self, size: usize) -> Shingles<'a, str> {
        Shingles::new(self, size)
    }

    fn as_shingles_with_step(&'a self, size: usize, step: usize) -> Shingles<'a, str> {
        Shingles::new_with_step(self, size, step)
    }
}