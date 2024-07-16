use std::iter::FromIterator;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CharBag(u64);

impl CharBag {
    pub fn is_superset(self, other: CharBag) -> bool {
        self.0 & other.0 == other.0
    }

    fn insert(&mut self, c: char) {
        let c = c.to_ascii_lowercase();
        if c.is_ascii_lowercase() {
            let mut count = self.0;
            let idx = c as u8 - b'a';
            count >>= idx * 2;
            count = ((count << 1) | 1) & 3;
            count <<= idx * 2;
            self.0 |= count;
        } else if c.is_ascii_digit() {
            let idx = c as u8 - b'0';
            self.0 |= 1 << (idx + 52);
        } else if c == '-' {
            self.0 |= 1 << 62;
        }
    }
}

impl Extend<char> for CharBag {
    fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T) {
        for c in iter {
            self.insert(c);
        }
    }
}

impl FromIterator<char> for CharBag {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut result = Self::default();
        result.extend(iter);
        result
    }
}

impl From<&str> for CharBag {
    fn from(s: &str) -> Self {
        let mut bag = Self(0);
        for c in s.chars() {
            bag.insert(c);
        }
        bag
    }
}

impl From<&[char]> for CharBag {
    fn from(chars: &[char]) -> Self {
        let mut bag = Self(0);
        for c in chars {
            bag.insert(*c);
        }
        bag
    }
}
