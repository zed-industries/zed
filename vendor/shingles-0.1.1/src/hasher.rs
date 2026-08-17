use std::hash::{Hash, Hasher, SipHasher};

/// ShingleHasher consumes iterator and reproduces hashes of it items.
///
/// ```
/// use shingles::{ShingleHasher, Shingles};
///
/// let str_sh = Shingles::new("hello", 4);
///
/// for h in ShingleHasher::new(str_sh) {
///     // prints hash of each shingle
///     println!("{}", h);
/// }
/// ```
pub struct ShingleHasher<T, K>
    where T: Iterator<Item = K>,
          K: Hash
{
    iter: T,
}

impl<T, K> ShingleHasher<T, K>
    where T: Iterator<Item = K>,
          K: Hash
{
    pub fn new(iter: T) -> Self {
        ShingleHasher { iter: iter }
    }
}

impl<T, K> Iterator for ShingleHasher<T, K>
    where T: Iterator<Item = K>,
          K: Hash
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next() {
            let mut h = SipHasher::new();
            item.hash(&mut h);
            Some(h.finish())
        } else {
            None
        }
    }
}