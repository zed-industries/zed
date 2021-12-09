use rand::prelude::*;

pub struct RandomCharIter<T: Rng>(T);

impl<T: Rng> RandomCharIter<T> {
    pub fn new(rng: T) -> Self {
        Self(rng)
    }
}

impl<T: Rng> Iterator for RandomCharIter<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.gen_range(0..100) {
            // whitespace
            0..=5 => ['\n'].choose(&mut self.0).copied(),
            // two-byte greek letters
            // 20..=32 => char::from_u32(self.0.gen_range(('α' as u32)..('ω' as u32 + 1))),
            // // three-byte characters
            // 33..=45 => ['✋', '✅', '❌', '❎', '⭐'].choose(&mut self.0).copied(),
            // // four-byte characters
            // 46..=58 => ['🍐', '🏀', '🍗', '🎉'].choose(&mut self.0).copied(),
            // ascii letters
            _ => Some(self.0.gen_range(b'a'..b'z' + 1).into()),
        }
    }
}
