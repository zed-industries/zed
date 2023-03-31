#[cfg(feature = "test-support")]
#[derive(Clone, Default)]
pub struct DeterministicState;

#[cfg(feature = "test-support")]
impl std::hash::BuildHasher for DeterministicState {
    type Hasher = seahash::SeaHasher;

    fn build_hasher(&self) -> Self::Hasher {
        seahash::SeaHasher::new()
    }
}

#[cfg(feature = "test-support")]
pub type HashMap<K, V> = std::collections::HashMap<K, V, DeterministicState>;

#[cfg(feature = "test-support")]
pub type HashSet<T> = std::collections::HashSet<T, DeterministicState>;

#[cfg(not(feature = "test-support"))]
pub type HashMap<K, V> = std::collections::HashMap<K, V>;

#[cfg(not(feature = "test-support"))]
pub type HashSet<T> = std::collections::HashSet<T>;

pub use std::collections::*;

// NEW TYPES

#[derive(Default)]
pub struct CommandPaletteFilter {
    pub filtered_namespaces: HashSet<&'static str>,
}
