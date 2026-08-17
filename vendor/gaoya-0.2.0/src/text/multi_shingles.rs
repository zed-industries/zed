use std::cmp;

/// MultiShingles produce shingles for a given range

#[derive(Clone)]
pub struct MultiShingles<'a, T: ?Sized + 'a> {
    data: &'a T,
    from_size: usize,
    to_size: usize,
    size: usize,
    step: usize,
}

impl<'a, T: ?Sized> MultiShingles<'a, T> {

    pub fn new(data: &'a T, from_size: usize, to_size: usize) -> Self {
        MultiShingles {
            data,
            from_size,
            to_size,
            size: from_size,
            step: 1,
        }
    }
}

impl<'a, T> Iterator for MultiShingles<'a, [T]> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() >= self.size {
            let ret = &self.data[0..self.size];
            self.size += 1;
            if self.size > self.to_size {
                self.size = self.from_size;
                self.data = &self.data[self.step..];
            }
            Some(ret)
        } else {
            None
        }
    }
}

impl<'a> Iterator for MultiShingles<'a, str> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pos_next: usize = 0;
        let mut pos_end: usize = 0;
        let mut chars = 0;

        // iterator reproduces char boundary positions and its appropriate bytes
        let iter = self.data.as_bytes().iter().enumerate()
            // only char boundaries
            .filter(|&(_, &b)| !(128..192).contains(&b));

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
        self.size += 1;
        if self.size > self.to_size {
            self.size = self.from_size;
            if pos_next != 0 {
                self.data = &self.data[pos_next..];
            } else {
                self.data = &self.data[self.data.len()..];
            }
        }

        ret
    }
}
pub trait AsShingles<'a, T: ?Sized + 'a> {
    fn as_shingles(&'a self, from_size: usize, to_size: usize) -> MultiShingles<'a, T>;
}

impl<'a, T: 'a> AsShingles<'a, [T]> for [T] {
    fn as_shingles(&'a self, from_size: usize, to_size: usize) -> MultiShingles<'a, [T]> {
        MultiShingles::new(self, from_size, to_size)
    }
}

impl<'a> AsShingles<'a, str> for str {
    fn as_shingles(&'a self, from_size: usize, to_size: usize) -> MultiShingles<'a, str> {
        MultiShingles::new(self, from_size, to_size)
    }
}