use crate::attribute::OwnedAttribute;
use crate::name::OwnedName;

use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::hash::{BuildHasher, Hash, Hasher};

/// An ordered set
pub struct AttributesSet {
    vec: Vec<OwnedAttribute>,
    /// Uses a no-op hasher, because these u64s are hashes already
    may_contain: HashSet<u64, U64HasherBuilder>,
    /// This is real hasher for the `OwnedName`
    hasher: RandomState,
}

/// Use linear search and don't allocate `HashSet` if there are few attributes,
/// because allocation costs more than a few comparisons.
const HASH_THRESHOLD: usize = 8;

impl AttributesSet {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            hasher: RandomState::new(),
            may_contain: HashSet::default(),
        }
    }

    fn hash(&self, val: &OwnedName) -> u64 {
        let mut h = self.hasher.build_hasher();
        val.hash(&mut h);
        h.finish()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn contains(&self, name: &OwnedName) -> bool {
        // fall back to linear search only on duplicate or hash collision
        (self.vec.len() < HASH_THRESHOLD || self.may_contain.contains(&self.hash(name))) &&
            self.vec.iter().any(move |a| &a.name == name)
    }

    pub fn push(&mut self, attr: OwnedAttribute) {
        if self.vec.len() >= HASH_THRESHOLD {
            if self.vec.len() == HASH_THRESHOLD {
                self.may_contain.reserve(HASH_THRESHOLD * 2);
                for attr in &self.vec {
                    self.may_contain.insert(self.hash(&attr.name));
                }
            }
            self.may_contain.insert(self.hash(&attr.name));
        }
        self.vec.push(attr);
    }

    pub fn into_vec(self) -> Vec<OwnedAttribute> {
        self.vec
    }
}

#[test]
fn indexset() {
    let mut s = AttributesSet::new();
    let not_here = OwnedName {
        local_name: "attr1000".into(),
        namespace: Some("test".into()),
        prefix: None,
    };

    // this test will take a lot of time if the `contains()` is linear, and the loop is quadratic
    for i in 0..50000 {
        let name = OwnedName {
            local_name: format!("attr{i}"), namespace: None, prefix: None,
        };
        assert!(!s.contains(&name));

        s.push(OwnedAttribute { name, value: String::new() });
        assert!(!s.contains(&not_here));
    }

    assert!(s.contains(&OwnedName {
        local_name: "attr1234".into(), namespace: None, prefix: None,
    }));
    assert!(s.contains(&OwnedName {
        local_name: "attr0".into(), namespace: None, prefix: None,
    }));
    assert!(s.contains(&OwnedName {
        local_name: "attr49999".into(), namespace: None, prefix: None,
    }));
}

/// Hashser that does nothing except passing u64 through
struct U64Hasher(u64);

impl Hasher for U64Hasher {
    fn finish(&self) -> u64 { self.0 }
    fn write(&mut self, slice: &[u8]) {
        for &v in slice { self.0 ^= u64::from(v) } // unused in practice
    }
    fn write_u64(&mut self, i: u64) {
        self.0 ^= i;
    }
}

#[derive(Default)]
struct U64HasherBuilder;

impl BuildHasher for U64HasherBuilder {
    type Hasher = U64Hasher;
    fn build_hasher(&self) -> U64Hasher { U64Hasher(0) }
}
