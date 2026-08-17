use core::ops::{Add, AddAssign, Sub, SubAssign};

// Similar to machine_index_t in harfbuzz, but Rust specific.
#[derive(Debug)]
pub struct MachineCursor<'a, T, F> {
    data: &'a [T],
    pred: F,
    pos: usize,
}

impl<'a, T, F> MachineCursor<'a, T, F>
where
    F: Fn(&[T], usize) -> bool,
{
    pub fn new(data: &'a [T], pred: F) -> Self {
        let pos = (0..data.len())
            .find(|i| pred(data, *i))
            .unwrap_or(data.len());
        Self { data, pred, pos }
    }

    fn advance1(&mut self) {
        self.pos = (self.pos + 1..self.data.len())
            .find(|q| (self.pred)(self.data, *q))
            .unwrap_or(self.data.len());
    }

    fn recede1(&mut self) {
        self.pos = (0..self.pos)
            .rev()
            .find(|q| (self.pred)(self.data, *q))
            .unwrap_or(0);
    }

    pub fn index(&self) -> usize {
        self.pos
    }

    pub fn end(&self) -> Self
    where
        F: Clone,
    {
        Self {
            data: self.data,
            pred: self.pred.clone(),
            pos: self.data.len(),
        }
    }
}

impl<T, F> Add<usize> for MachineCursor<'_, T, F>
where
    F: Fn(&[T], usize) -> bool,
{
    type Output = Self;

    fn add(mut self, rhs: usize) -> Self::Output {
        for _ in 0..rhs {
            self.advance1();
        }
        self
    }
}

impl<T, F> Sub<usize> for MachineCursor<'_, T, F>
where
    F: Fn(&[T], usize) -> bool,
{
    type Output = Self;

    fn sub(mut self, rhs: usize) -> Self::Output {
        for _ in 0..rhs {
            self.recede1();
        }
        self
    }
}

impl<T, F> AddAssign<usize> for MachineCursor<'_, T, F>
where
    F: Fn(&[T], usize) -> bool,
{
    fn add_assign(&mut self, rhs: usize) {
        for _ in 0..rhs {
            self.advance1();
        }
    }
}

impl<T, F> SubAssign<usize> for MachineCursor<'_, T, F>
where
    F: Fn(&[T], usize) -> bool,
{
    fn sub_assign(&mut self, rhs: usize) {
        for _ in 0..rhs {
            self.recede1();
        }
    }
}

impl<T, F> PartialEq for MachineCursor<'_, T, F> {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl<T, F> Clone for MachineCursor<'_, T, F>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data,
            pred: self.pred.clone(),
            pos: self.pos,
        }
    }
}

impl<T, F> Copy for MachineCursor<'_, T, F> where F: Copy {}
