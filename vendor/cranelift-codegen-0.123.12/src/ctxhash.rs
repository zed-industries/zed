//! A hashmap with "external hashing": nodes are hashed or compared for
//! equality only with some external context provided on lookup/insert.
//! This allows very memory-efficient data structures where
//! node-internal data references some other storage (e.g., offsets into
//! an array or pool of shared data).

use hashbrown::hash_table::HashTable;
use std::hash::{Hash, Hasher};

/// Trait that allows for equality comparison given some external
/// context.
///
/// Note that this trait is implemented by the *context*, rather than
/// the item type, for somewhat complex lifetime reasons (lack of GATs
/// to allow `for<'ctx> Ctx<'ctx>`-like associated types in traits on
/// the value type).
pub trait CtxEq<V1: ?Sized, V2: ?Sized> {
    /// Determine whether `a` and `b` are equal, given the context in
    /// `self` and the union-find data structure `uf`.
    fn ctx_eq(&self, a: &V1, b: &V2) -> bool;
}

/// Trait that allows for hashing given some external context.
pub trait CtxHash<Value: ?Sized>: CtxEq<Value, Value> {
    /// Compute the hash of `value`, given the context in `self` and
    /// the union-find data structure `uf`.
    fn ctx_hash<H: Hasher>(&self, state: &mut H, value: &Value);
}

/// A null-comparator context type for underlying value types that
/// already have `Eq` and `Hash`.
#[derive(Default)]
pub struct NullCtx;

impl<V: Eq + Hash> CtxEq<V, V> for NullCtx {
    fn ctx_eq(&self, a: &V, b: &V) -> bool {
        a.eq(b)
    }
}
impl<V: Eq + Hash> CtxHash<V> for NullCtx {
    fn ctx_hash<H: Hasher>(&self, state: &mut H, value: &V) {
        value.hash(state);
    }
}

/// A bucket in the hash table.
///
/// Some performance-related design notes: we cache the hashcode for
/// speed, as this often buys a few percent speed in
/// interning-table-heavy workloads. We only keep the low 32 bits of
/// the hashcode, for memory efficiency: in common use, `K` and `V`
/// are often 32 bits also, and a 12-byte bucket is measurably better
/// than a 16-byte bucket.
struct BucketData<K, V> {
    hash: u32,
    k: K,
    v: V,
}

/// A HashMap that takes external context for all operations.
pub struct CtxHashMap<K, V> {
    raw: HashTable<BucketData<K, V>>,
}

impl<K, V> CtxHashMap<K, V> {
    /// Create an empty hashmap with pre-allocated space for the given
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            raw: HashTable::with_capacity(capacity),
        }
    }
}

fn compute_hash<Ctx, K>(ctx: &Ctx, k: &K) -> u32
where
    Ctx: CtxHash<K>,
{
    let mut hasher = rustc_hash::FxHasher::default();
    ctx.ctx_hash(&mut hasher, k);
    hasher.finish() as u32
}

impl<K, V> CtxHashMap<K, V> {
    /// Insert a new key-value pair, returning the old value associated
    /// with this key (if any).
    pub fn insert<Ctx>(&mut self, k: K, v: V, ctx: &Ctx) -> Option<V>
    where
        Ctx: CtxEq<K, K> + CtxHash<K>,
    {
        let hash = compute_hash(ctx, &k);
        match self.raw.find_mut(hash as u64, |bucket| {
            hash == bucket.hash && ctx.ctx_eq(&bucket.k, &k)
        }) {
            Some(bucket) => Some(std::mem::replace(&mut bucket.v, v)),
            None => {
                let data = BucketData { hash, k, v };
                self.raw
                    .insert_unique(hash as u64, data, |bucket| bucket.hash as u64);
                None
            }
        }
    }

    /// Look up a key, returning a borrow of the value if present.
    pub fn get<'a, Q, Ctx>(&'a self, k: &Q, ctx: &Ctx) -> Option<&'a V>
    where
        Ctx: CtxEq<K, Q> + CtxHash<Q> + CtxHash<K>,
    {
        let hash = compute_hash(ctx, k);
        self.raw
            .find(hash as u64, |bucket| {
                hash == bucket.hash && ctx.ctx_eq(&bucket.k, k)
            })
            .map(|bucket| &bucket.v)
    }

    /// Look up a key, returning an `Entry` that refers to an existing
    /// value or allows inserting a new one.
    pub fn entry<'a, Ctx>(&'a mut self, k: K, ctx: &Ctx) -> Entry<'a, K, V>
    where
        Ctx: CtxEq<K, K> + CtxHash<K>,
    {
        let hash = compute_hash(ctx, &k);
        let raw = self.raw.entry(
            hash as u64,
            |bucket| hash == bucket.hash && ctx.ctx_eq(&bucket.k, &k),
            |bucket| compute_hash(ctx, &bucket.k) as u64,
        );
        match raw {
            hashbrown::hash_table::Entry::Occupied(o) => Entry::Occupied(OccupiedEntry { raw: o }),
            hashbrown::hash_table::Entry::Vacant(v) => Entry::Vacant(VacantEntry {
                hash,
                key: k,
                raw: v,
            }),
        }
    }
}

/// A reference to an existing or vacant entry in the hash table.
pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

/// A reference to an occupied entry in the hash table.
pub struct OccupiedEntry<'a, K, V> {
    raw: hashbrown::hash_table::OccupiedEntry<'a, BucketData<K, V>>,
}

/// A reference to a vacant entry in the hash table.
pub struct VacantEntry<'a, K, V> {
    hash: u32,
    key: K,
    raw: hashbrown::hash_table::VacantEntry<'a, BucketData<K, V>>,
}

impl<'a, K, V> OccupiedEntry<'a, K, V> {
    /// Get the existing value.
    pub fn get(&self) -> &V {
        &self.raw.get().v
    }

    /// Get the existing value, mutably.
    pub fn get_mut(&mut self) -> &mut V {
        &mut self.raw.get_mut().v
    }
}

impl<'a, K, V> VacantEntry<'a, K, V> {
    /// Insert a new value.
    pub fn insert(self, v: V) {
        self.raw.insert(BucketData {
            hash: self.hash,
            k: self.key,
            v,
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone, Copy, Debug)]
    struct Key {
        index: u32,
    }
    struct Ctx {
        vals: &'static [&'static str],
    }
    impl CtxEq<Key, Key> for Ctx {
        fn ctx_eq(&self, a: &Key, b: &Key) -> bool {
            self.vals[a.index as usize].eq(self.vals[b.index as usize])
        }
    }
    impl CtxHash<Key> for Ctx {
        fn ctx_hash<H: Hasher>(&self, state: &mut H, value: &Key) {
            self.vals[value.index as usize].hash(state);
        }
    }

    #[test]
    fn test_basic() {
        let ctx = Ctx {
            vals: &["a", "b", "a"],
        };

        let k0 = Key { index: 0 };
        let k1 = Key { index: 1 };
        let k2 = Key { index: 2 };

        assert!(ctx.ctx_eq(&k0, &k2));
        assert!(!ctx.ctx_eq(&k0, &k1));
        assert!(!ctx.ctx_eq(&k2, &k1));

        let mut map: CtxHashMap<Key, u64> = CtxHashMap::with_capacity(4);
        assert_eq!(map.insert(k0, 42, &ctx), None);
        assert_eq!(map.insert(k2, 84, &ctx), Some(42));
        assert_eq!(map.get(&k1, &ctx), None);
        assert_eq!(*map.get(&k0, &ctx).unwrap(), 84);
    }
}
