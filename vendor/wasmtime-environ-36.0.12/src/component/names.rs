use crate::prelude::*;
use anyhow::{Result, bail};
use core::hash::Hash;
use semver::Version;
use serde_derive::{Deserialize, Serialize};

/// A semver-aware map for imports/exports of a component.
///
/// This data structure is used when looking up the names of imports/exports of
/// a component to enable semver-compatible matching of lookups. This will
/// enable lookups of `a:b/c@0.2.0` to match entries defined as `a:b/c@0.2.1`
/// which is currently considered a key feature of WASI's compatibility story.
///
/// On the outside this looks like a map of `K` to `V`.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NameMap<K: Clone + Hash + Eq + Ord, V> {
    /// A map of keys to the value that they define.
    ///
    /// Note that this map is "exact" where the name here is the exact name that
    /// was specified when the `insert` was called. This doesn't have any
    /// semver-mangling or anything like that.
    ///
    /// This map is always consulted first during lookups.
    definitions: IndexMap<K, V>,

    /// An auxiliary map tracking semver-compatible names. This is a map from
    /// "semver compatible alternate name" to a name present in `definitions`
    /// and the semver version it was registered at.
    ///
    /// An example map would be:
    ///
    /// ```text
    /// {
    ///     "a:b/c@0.2": ("a:b/c@0.2.1", 0.2.1),
    ///     "a:b/c@2": ("a:b/c@2.0.0+abc", 2.0.0+abc),
    /// }
    /// ```
    ///
    /// As names are inserted into `definitions` each name may have up to one
    /// semver-compatible name with extra numbers/info chopped off which is
    /// inserted into this map. This map is the lookup table from `@0.2` to
    /// `@0.2.x` where `x` is what was inserted manually.
    ///
    /// The `Version` here is tracked to ensure that when multiple versions on
    /// one track are defined that only the maximal version here is retained.
    alternate_lookups: IndexMap<K, (K, Version)>,
}

impl<K, V> NameMap<K, V>
where
    K: Clone + Hash + Eq + Ord,
{
    /// Inserts the `name` specified into this map.
    ///
    /// The name is intern'd through the `cx` argument and shadowing is
    /// controlled by the `allow_shadowing` variable.
    ///
    /// This function will automatically insert an entry in
    /// `self.alternate_lookups` if `name` is a semver-looking name.
    ///
    /// Returns an error if `allow_shadowing` is `false` and the `name` is
    /// already present in this map (by exact match). Otherwise returns the
    /// intern'd version of `name`.
    pub fn insert<I>(&mut self, name: &str, cx: &mut I, allow_shadowing: bool, item: V) -> Result<K>
    where
        I: NameMapIntern<Key = K>,
    {
        // Always insert `name` and `item` as an exact definition.
        let key = cx.intern(name);
        if let Some(prev) = self.definitions.insert(key.clone(), item) {
            if !allow_shadowing {
                self.definitions.insert(key, prev);
                bail!("map entry `{name}` defined twice")
            }
        }

        // If `name` is a semver-looking thing, like `a:b/c@1.0.0`, then also
        // insert an entry in the semver-compatible map under a key such as
        // `a:b/c@1`.
        //
        // This key is used during `get` later on.
        if let Some((alternate_key, version)) = alternate_lookup_key(name) {
            let alternate_key = cx.intern(alternate_key);
            if let Some((prev_key, prev_version)) = self
                .alternate_lookups
                .insert(alternate_key.clone(), (key.clone(), version.clone()))
            {
                // Prefer the latest version, so only do this if we're
                // greater than the prior version.
                if version < prev_version {
                    self.alternate_lookups
                        .insert(alternate_key, (prev_key, prev_version));
                }
            }
        }
        Ok(key)
    }

    /// Looks up `name` within this map, using the interning specified by
    /// `cx`.
    ///
    /// This may return a definition even if `name` wasn't exactly defined in
    /// this map, such as looking up `a:b/c@0.2.0` when the map only has
    /// `a:b/c@0.2.1` defined.
    pub fn get<I>(&self, name: &str, cx: &I) -> Option<&V>
    where
        I: NameMapIntern<Key = K>,
    {
        // First look up an exact match and if that's found return that. This
        // enables defining multiple versions in the map and the requested
        // version is returned if it matches exactly.
        let candidate = cx.lookup(name).and_then(|k| self.definitions.get(&k));
        if let Some(def) = candidate {
            return Some(def);
        }

        // Failing that, then try to look for a semver-compatible alternative.
        // This looks up the key based on `name`, if any, and then looks to see
        // if that was intern'd in `strings`. Given all that look to see if it
        // was defined in `alternate_lookups` and finally at the end that exact
        // key is then used to look up again in `self.definitions`.
        let (alternate_name, _version) = alternate_lookup_key(name)?;
        let alternate_key = cx.lookup(alternate_name)?;
        let (exact_key, _version) = self.alternate_lookups.get(&alternate_key)?;
        self.definitions.get(exact_key)
    }

    /// Returns an iterator over inserted values in this map.
    ///
    /// Note that the iterator return yields intern'd keys and additionally does
    /// not do anything special with semver names and such, it only literally
    /// yields what's been inserted with [`NameMap::insert`].
    pub fn raw_iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.definitions.iter()
    }

    /// TODO
    pub fn raw_get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.definitions.get_mut(key)
    }
}

impl<K, V> Default for NameMap<K, V>
where
    K: Clone + Hash + Eq + Ord,
{
    fn default() -> NameMap<K, V> {
        NameMap {
            definitions: Default::default(),
            alternate_lookups: Default::default(),
        }
    }
}

/// A helper trait used in conjunction with [`NameMap`] to optionally intern
/// keys to non-strings.
pub trait NameMapIntern {
    /// The key that this interning context generates.
    type Key;

    /// Inserts `s` into `self` and returns the intern'd key `Self::Key`.
    fn intern(&mut self, s: &str) -> Self::Key;

    /// Looks up `s` in `self` returning `Some` if it was found or `None` if
    /// it's not present.
    fn lookup(&self, s: &str) -> Option<Self::Key>;
}

/// For use with [`NameMap`] when no interning should happen and instead string
/// keys are copied as-is.
pub struct NameMapNoIntern;

impl NameMapIntern for NameMapNoIntern {
    type Key = String;

    fn intern(&mut self, s: &str) -> String {
        s.to_string()
    }

    fn lookup(&self, s: &str) -> Option<String> {
        Some(s.to_string())
    }
}

/// Determines a version-based "alternate lookup key" for the `name` specified.
///
/// Some examples are:
///
/// * `foo` => `None`
/// * `foo:bar/baz` => `None`
/// * `foo:bar/baz@1.1.2` => `Some(foo:bar/baz@1)`
/// * `foo:bar/baz@0.1.0` => `Some(foo:bar/baz@0.1)`
/// * `foo:bar/baz@0.0.1` => `None`
/// * `foo:bar/baz@0.1.0-rc.2` => `None`
///
/// This alternate lookup key is intended to serve the purpose where a
/// semver-compatible definition can be located, if one is defined, at perhaps
/// either a newer or an older version.
fn alternate_lookup_key(name: &str) -> Option<(&str, Version)> {
    let at = name.find('@')?;
    let version_string = &name[at + 1..];
    let version = Version::parse(version_string).ok()?;
    if !version.pre.is_empty() {
        // If there's a prerelease then don't consider that compatible with any
        // other version number.
        None
    } else if version.major != 0 {
        // If the major number is nonzero then compatibility is up to the major
        // version number, so return up to the first decimal.
        let first_dot = version_string.find('.')? + at + 1;
        Some((&name[..first_dot], version))
    } else if version.minor != 0 {
        // Like the major version if the minor is nonzero then patch releases
        // are all considered to be on a "compatible track".
        let first_dot = version_string.find('.')? + at + 1;
        let second_dot = name[first_dot + 1..].find('.')? + first_dot + 1;
        Some((&name[..second_dot], version))
    } else {
        // If the patch number is the first nonzero entry then nothing can be
        // compatible with this patch, e.g. 0.0.1 isn't' compatible with
        // any other version inherently.
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{NameMap, NameMapNoIntern};

    #[test]
    fn alternate_lookup_key() {
        fn alt(s: &str) -> Option<&str> {
            super::alternate_lookup_key(s).map(|(s, _)| s)
        }

        assert_eq!(alt("x"), None);
        assert_eq!(alt("x:y/z"), None);
        assert_eq!(alt("x:y/z@1.0.0"), Some("x:y/z@1"));
        assert_eq!(alt("x:y/z@1.1.0"), Some("x:y/z@1"));
        assert_eq!(alt("x:y/z@1.1.2"), Some("x:y/z@1"));
        assert_eq!(alt("x:y/z@2.1.2"), Some("x:y/z@2"));
        assert_eq!(alt("x:y/z@2.1.2+abc"), Some("x:y/z@2"));
        assert_eq!(alt("x:y/z@0.1.2"), Some("x:y/z@0.1"));
        assert_eq!(alt("x:y/z@0.1.3"), Some("x:y/z@0.1"));
        assert_eq!(alt("x:y/z@0.2.3"), Some("x:y/z@0.2"));
        assert_eq!(alt("x:y/z@0.2.3+abc"), Some("x:y/z@0.2"));
        assert_eq!(alt("x:y/z@0.0.1"), None);
        assert_eq!(alt("x:y/z@0.0.1-pre"), None);
        assert_eq!(alt("x:y/z@0.1.0-pre"), None);
        assert_eq!(alt("x:y/z@1.0.0-pre"), None);
    }

    #[test]
    fn name_map_smoke() {
        let mut map = NameMap::default();
        let mut intern = NameMapNoIntern;

        map.insert("a", &mut intern, false, 0).unwrap();
        map.insert("b", &mut intern, false, 1).unwrap();

        assert!(map.insert("a", &mut intern, false, 0).is_err());
        assert!(map.insert("a", &mut intern, true, 0).is_ok());

        assert_eq!(map.get("a", &intern), Some(&0));
        assert_eq!(map.get("b", &intern), Some(&1));
        assert_eq!(map.get("c", &intern), None);

        map.insert("a:b/c@1.0.0", &mut intern, false, 2).unwrap();
        map.insert("a:b/c@1.0.1", &mut intern, false, 3).unwrap();
        assert_eq!(map.get("a:b/c@1.0.0", &intern), Some(&2));
        assert_eq!(map.get("a:b/c@1.0.1", &intern), Some(&3));
        assert_eq!(map.get("a:b/c@1.0.2", &intern), Some(&3));
        assert_eq!(map.get("a:b/c@1.1.0", &intern), Some(&3));
    }
}
