#[derive(Copy, Clone, Debug)]
pub struct CharBag(u64);

impl CharBag {
    pub fn is_superset(self, other: CharBag) -> bool {
        self.0 & other.0 == other.0
    }

    fn insert(&mut self, c: char) {
        if c >= 'a' && c <= 'z' {
            let mut count = self.0;
            let idx = c as u8 - 'a' as u8;
            count = count >> (idx * 2);
            count = ((count << 1) | 1) & 3;
            count = count << idx * 2;
            self.0 |= count;
        } else if c >= '0' && c <= '9' {
            let idx = c as u8 - '0' as u8;
            self.0 |= 1 << (idx + 52);
        } else if c == '-' {
            self.0 |= 1 << 62;
        }
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
