use std::cmp;
use std::hash::Hash;

use hasher::ShingleHasher;

/// Positions enum
/// None   - initial state.
/// Val()  - for constant size types.
/// List() - for dynamic size types(like strings). Keeps positions for each shingle row.
#[derive(Clone)]
enum Pos {
    None,
    Val(usize),
    List(Vec<usize>),
}

/// Two dimensional shingles structure, has different sizes and steps in each dimension.
/// Can be useful in picture analysis or while considering some text as a single sheet etc.
///
/// # Examples
///
/// ```
/// use shingles::Shingles2D;
///
/// let v: Vec<_> = "abcd\n\
///                  efgh\n\
///                  ijkl"
///     .split_terminator("\n")
///     .collect();
///
/// let mut sh_2d = Shingles2D::new(&v[..], [3, 3]);
///
/// assert_eq!(
///     Some(vec![&v[0][0..3], &v[1][0..3], &v[2][0..3]]),
///     sh_2d.next()
/// );
///
/// // You can easily get hashes from 2D-shingles
/// for h in Shingles2D::new(&v[..], [3, 3]).hashes() {
///     // print u64 hash value for each 2D-shingle
///     println!("{}", h);
/// }
/// ```
#[derive(Clone)]
pub struct Shingles2D<'a, T: ?Sized + 'a> {
    data: &'a [&'a T],
    pos_x: Pos,
    size: [usize; 2],
    step: [usize; 2],
}

impl<'a, T: ?Sized> Shingles2D<'a, T> {
    pub fn new(data: &'a [&'a T], size: [usize; 2]) -> Self {
        Self::new_with_step(data, size, [1, 1])
    }

    pub fn new_with_step(data: &'a [&'a T], size: [usize; 2], step: [usize; 2]) -> Self {
        Shingles2D {
            data: data,
            size: size,
            pos_x: Pos::None,
            step: step,
        }
    }

    /// Returns iterator, which reproduces hashes of 2D shingles.
    pub fn hashes<K>(self) -> ShingleHasher<Self, K>
        where Self: Iterator<Item = K>,
              K: Hash
    {
        ShingleHasher::new(self)
    }
}

impl<'a, T> Iterator for Shingles2D<'a, [T]> {
    type Item = Vec<&'a [T]>;

    fn next(&mut self) -> Option<Self::Item> {
        // initialize position at first time or get current position
        let mut pos_x = match self.pos_x {
            Pos::Val(p) => p,
            Pos::None => 0,
            _ => unreachable!(),
        };

        let mut ret = Vec::with_capacity(self.size[1]);

        // In this case X space varies at every Y
        // so we'll find a first rectangular where we can build a shingle.
        // Suppose shingle was build successfully if it have sufficient height
        // and at least one row have sufficient width.
        while self.data.len() >= self.size[1] {
            let mut has_sufficient_width = false;

            for data_x in self.data[0..self.size[1]].iter() {
                if pos_x + self.size[0] <= data_x.len() {
                    has_sufficient_width = true;
                }
                let from = cmp::min(pos_x, data_x.len());
                let to = cmp::min(pos_x + self.size[0], data_x.len());

                ret.push(&data_x[from..to]);
            }

            // suitable shingle
            if has_sufficient_width && ret.len() == self.size[1] {
                pos_x += self.step[0];
                break;
            }

            // try with next row
            let step = cmp::min(self.step[1], self.data.len());
            self.data = &self.data[step..];
            pos_x = 0;
            ret.clear();
        }

        self.pos_x = Pos::Val(pos_x);

        if ret.len() == self.size[1] {
            Some(ret)
        } else {
            None
        }
    }
}

impl<'a> Iterator for Shingles2D<'a, str> {
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        // initialize positions at first time or get current positions
        let mut pos_x_list = match self.pos_x {
            Pos::List(ref v) => v.clone(),
            Pos::None => vec![0; self.size[1]],
            _ => unreachable!(),
        };

        let mut ret = Vec::with_capacity(self.size[1]);

        // Same logic as with [T] type
        while self.data.len() >= self.size[1] {
            let mut has_sufficient_width = false;

            for (y, data_x) in self.data[0..self.size[1]].iter().enumerate() {
                let mut pos_end: usize = 0;
                let mut pos_next: usize = 0;
                let mut chars = 0;
                let pos_x = pos_x_list[y];

                // iterator reproduces char boundary positions and its appropriate bytes
                let iter = data_x[pos_x..].as_bytes()
                    .iter().enumerate()
                    // only char boundaries
                    .filter(|&(_, &b)| b < 128 || b >= 192);

                // get end position of x-shingle part and next x-step position at once
                for (i, _) in iter {
                    if chars == self.step[0] {
                        pos_next = pos_x + i;
                    }
                    if chars == self.size[0] {
                        pos_end = pos_x + i;
                    }
                    if pos_next != 0 && pos_end != 0 {
                        break;
                    }
                    chars += 1;
                }

                if pos_end != 0 || chars == self.size[0] {
                    has_sufficient_width = true;
                }

                if pos_end == 0 {
                    pos_end = data_x.len();
                }

                ret.push(&data_x[pos_x..pos_end]);
                pos_x_list[y] = if pos_next != 0 { pos_next } else { data_x.len() };
            }

            // suitable shingle was found
            if has_sufficient_width && ret.len() == self.size[1] {
                break;
            }

            // go to next row
            let step = cmp::min(self.step[1], self.data.len());
            self.data = &self.data[step..];
            pos_x_list = vec![0; self.size[1]];
            ret.clear();
        }

        self.pos_x = Pos::List(pos_x_list);

        if ret.len() == self.size[1] {
            Some(ret)
        } else {
            None
        }
    }
}

/// An interface for getting 2D shingles from other types.
///
/// # Examples
///
/// ```
/// use shingles::AsShingles2D;
///
/// let v: Vec<_> = "abcd\n\
///                  efgh\n\
///                  ijkl"
///     .split_terminator("\n")
///     .collect();
///
/// let mut sh_2d = v[..].as_shingles_2d([3, 3]);
///
/// assert_eq!(
///     Some(vec![&v[0][0..3], &v[1][0..3], &v[2][0..3]]),
///     sh_2d.next()
/// );
/// ```
pub trait AsShingles2D<'a, T: ?Sized + 'a> {
    fn as_shingles_2d(&'a self, size: [usize; 2]) -> Shingles2D<'a, T>;
    fn as_shingles_2d_with_step(&'a self, size: [usize; 2], step: [usize; 2]) -> Shingles2D<'a, T>;
}

impl<'a, T> AsShingles2D<'a, [T]> for [&'a [T]] {
    fn as_shingles_2d(&'a self, size: [usize; 2]) -> Shingles2D<'a, [T]> {
        Shingles2D::new(self, size)
    }

    fn as_shingles_2d_with_step(&'a self, size: [usize; 2], step: [usize; 2]) -> Shingles2D<'a, [T]> {
        Shingles2D::new_with_step(self, size, step)
    }
}

impl<'a> AsShingles2D<'a, str> for [&'a str] {
    fn as_shingles_2d(&'a self, size: [usize; 2]) -> Shingles2D<'a, str> {
        Shingles2D::new(self, size)
    }

    fn as_shingles_2d_with_step(&'a self, size: [usize; 2], step: [usize; 2]) -> Shingles2D<'a, str> {
        Shingles2D::new_with_step(self, size, step)
    }
}