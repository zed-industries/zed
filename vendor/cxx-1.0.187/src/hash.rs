use core::hash::{BuildHasher as _, Hash};

#[doc(hidden)]
pub fn hash<V: Hash>(value: &V) -> usize {
    foldhash::quality::FixedState::default().hash_one(value) as usize
}
