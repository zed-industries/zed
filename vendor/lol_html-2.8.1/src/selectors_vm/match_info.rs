use std::num::NonZeroU32;

pub(crate) type MatchId = u32;

pub(crate) struct MatchInfo {
    pub match_id: MatchId,
    pub with_content: bool,
}

#[derive(Eq, Debug, Clone, PartialEq)]
pub(crate) enum DenseHashSet {
    Inline(u32),
    Heap(Box<[u32]>),
}

impl DenseHashSet {
    pub fn new() -> Self {
        Self::Inline(0)
    }

    pub fn insert(&mut self, value: MatchId) {
        let int_idx = (value / 32) as usize;
        let bit_idx = (value & 31) as u8;
        let b = if let Some(b) = self.slice_mut().get_mut(int_idx) {
            b
        } else if let Some(b) = self.resize(int_idx * 2).get_mut(int_idx) {
            b
        } else {
            debug_assert!(false);
            return;
        };
        *b |= 1 << bit_idx;
    }

    pub fn union(&mut self, other: &Self) {
        let mut bits = self.slice_mut();
        let other = other.slice();
        if bits.len() < other.len() {
            bits = self.resize(other.len());
        }
        for (a, b) in bits.iter_mut().zip(other) {
            *a |= *b;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = MatchId> {
        self.slice()
            .iter()
            .copied()
            .enumerate()
            .skip_while(|&(_, b)| b == 0)
            .flat_map(|(int_idx, mut current_byte)| {
                let base_bit_idx = 32 * int_idx as MatchId;
                std::iter::from_fn(move || {
                    let byte = NonZeroU32::new(current_byte)?;
                    let match_id = base_bit_idx + byte.trailing_zeros();
                    current_byte &= byte.get() - 1;

                    Some(match_id)
                })
            })
    }

    fn slice_mut(&mut self) -> &mut [u32] {
        match self {
            Self::Heap(v) => v,
            Self::Inline(v) => std::slice::from_mut(v),
        }
    }

    fn slice(&self) -> &[u32] {
        match self {
            Self::Heap(v) => v,
            Self::Inline(v) => std::slice::from_ref(v),
        }
    }

    #[cold]
    fn resize(&mut self, new_len: usize) -> &mut [u32] {
        let bits = self.slice_mut();
        let mut new = Vec::with_capacity(new_len);
        new.extend_from_slice(bits);
        let cap = new.capacity();
        new.resize(cap, 0);
        *self = Self::Heap(new.into_boxed_slice());
        self.slice_mut()
    }

    #[cfg(test)]
    pub fn from(values: impl IntoIterator<Item = MatchId>) -> Self {
        let mut new = Self::new();
        for v in values {
            new.insert(v);
        }
        new
    }
}

#[test]
fn size() {
    assert_eq!(16, size_of::<DenseHashSet>());
}
