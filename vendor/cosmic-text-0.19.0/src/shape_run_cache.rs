#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};
use core::ops::Range;

use crate::{AttrsOwned, HashMap, ShapeGlyph};

/// Key for caching shape runs.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ShapeRunKey {
    pub text: String,
    pub default_attrs: AttrsOwned,
    pub attrs_spans: Vec<(Range<usize>, AttrsOwned)>,
}

/// A helper structure for caching shape runs.
#[derive(Clone, Default)]
pub struct ShapeRunCache {
    age: u64,
    cache: HashMap<ShapeRunKey, (u64, Vec<ShapeGlyph>)>,
}

impl ShapeRunCache {
    /// Get cache item, updating age if found
    pub fn get(&mut self, key: &ShapeRunKey) -> Option<&Vec<ShapeGlyph>> {
        self.cache.get_mut(key).map(|(age, glyphs)| {
            *age = self.age;
            &*glyphs
        })
    }

    /// Insert cache item with current age
    pub fn insert(&mut self, key: ShapeRunKey, glyphs: Vec<ShapeGlyph>) {
        self.cache.insert(key, (self.age, glyphs));
    }

    /// Remove anything in the cache with an age older than `keep_ages`
    pub fn trim(&mut self, keep_ages: u64) {
        self.cache
            .retain(|_key, (age, _glyphs)| *age + keep_ages >= self.age);
        // Increase age
        self.age += 1;
    }
}

impl core::fmt::Debug for ShapeRunCache {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ShapeRunCache").finish()
    }
}
