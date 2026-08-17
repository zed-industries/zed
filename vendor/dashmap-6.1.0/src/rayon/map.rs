use crate::lock::RwLock;
use crate::mapref::multiple::{RefMulti, RefMutMulti};
use crate::{DashMap, HashMap};
use core::hash::{BuildHasher, Hash};
use crossbeam_utils::CachePadded;
use rayon::iter::plumbing::UnindexedConsumer;
use rayon::iter::{FromParallelIterator, IntoParallelIterator, ParallelExtend, ParallelIterator};
use std::sync::Arc;

impl<K, V, S> ParallelExtend<(K, V)> for DashMap<K, V, S>
where
    K: Send + Sync + Eq + Hash,
    V: Send + Sync,
    S: Send + Sync + Clone + BuildHasher,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = (K, V)>,
    {
        (&*self).par_extend(par_iter);
    }
}

// Since we don't actually need mutability, we can implement this on a
// reference, similar to `io::Write for &File`.
impl<K, V, S> ParallelExtend<(K, V)> for &'_ DashMap<K, V, S>
where
    K: Send + Sync + Eq + Hash,
    V: Send + Sync,
    S: Send + Sync + Clone + BuildHasher,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = (K, V)>,
    {
        let &mut map = self;
        par_iter.into_par_iter().for_each(move |(key, value)| {
            map.insert(key, value);
        });
    }
}

impl<K, V, S> FromParallelIterator<(K, V)> for DashMap<K, V, S>
where
    K: Send + Sync + Eq + Hash,
    V: Send + Sync,
    S: Send + Sync + Clone + Default + BuildHasher,
{
    fn from_par_iter<I>(par_iter: I) -> Self
    where
        I: IntoParallelIterator<Item = (K, V)>,
    {
        let map = Self::default();
        (&map).par_extend(par_iter);
        map
    }
}

// Implementation note: while the shards will iterate in parallel, we flatten
// sequentially within each shard (`flat_map_iter`), because the standard
// `HashMap` only implements `ParallelIterator` by collecting to a `Vec` first.
// There is real parallel support in the `hashbrown/rayon` feature, but we don't
// always use that map.

impl<K, V, S> IntoParallelIterator for DashMap<K, V, S>
where
    K: Send + Eq + Hash,
    V: Send,
    S: Send + Clone + BuildHasher,
{
    type Iter = OwningIter<K, V>;
    type Item = (K, V);

    fn into_par_iter(self) -> Self::Iter {
        OwningIter {
            shards: self.shards,
        }
    }
}

pub struct OwningIter<K, V> {
    pub(super) shards: Box<[CachePadded<RwLock<HashMap<K, V>>>]>,
}

impl<K, V> ParallelIterator for OwningIter<K, V>
where
    K: Send + Eq + Hash,
    V: Send,
{
    type Item = (K, V);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        Vec::from(self.shards)
            .into_par_iter()
            .flat_map_iter(|shard| {
                shard
                    .into_inner()
                    .into_inner()
                    .into_iter()
                    .map(|(k, v)| (k, v.into_inner()))
            })
            .drive_unindexed(consumer)
    }
}

// This impl also enables `IntoParallelRefIterator::par_iter`
impl<'a, K, V, S> IntoParallelIterator for &'a DashMap<K, V, S>
where
    K: Send + Sync + Eq + Hash,
    V: Send + Sync,
    S: Send + Sync + Clone + BuildHasher,
{
    type Iter = Iter<'a, K, V>;
    type Item = RefMulti<'a, K, V>;

    fn into_par_iter(self) -> Self::Iter {
        Iter {
            shards: &self.shards,
        }
    }
}

pub struct Iter<'a, K, V> {
    pub(super) shards: &'a [CachePadded<RwLock<HashMap<K, V>>>],
}

impl<'a, K, V> ParallelIterator for Iter<'a, K, V>
where
    K: Send + Sync + Eq + Hash,
    V: Send + Sync,
{
    type Item = RefMulti<'a, K, V>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.shards
            .into_par_iter()
            .flat_map_iter(|shard| unsafe {
                let guard = Arc::new(shard.read());
                guard.iter().map(move |b| {
                    let guard = Arc::clone(&guard);
                    let (k, v) = b.as_ref();
                    RefMulti::new(guard, k, v.get())
                })
            })
            .drive_unindexed(consumer)
    }
}

// This impl also enables `IntoParallelRefMutIterator::par_iter_mut`
impl<'a, K, V> IntoParallelIterator for &'a mut DashMap<K, V>
where
    K: Send + Sync + Eq + Hash,
    V: Send + Sync,
{
    type Iter = IterMut<'a, K, V>;
    type Item = RefMutMulti<'a, K, V>;

    fn into_par_iter(self) -> Self::Iter {
        IterMut {
            shards: &self.shards,
        }
    }
}

impl<K, V, S> DashMap<K, V, S>
where
    K: Send + Sync + Eq + Hash,
    V: Send + Sync,
{
    // Unlike `IntoParallelRefMutIterator::par_iter_mut`, we only _need_ `&self`.
    pub fn par_iter_mut(&self) -> IterMut<'_, K, V> {
        IterMut {
            shards: &self.shards,
        }
    }
}

pub struct IterMut<'a, K, V> {
    shards: &'a [CachePadded<RwLock<HashMap<K, V>>>],
}

impl<'a, K, V> ParallelIterator for IterMut<'a, K, V>
where
    K: Send + Sync + Eq + Hash,
    V: Send + Sync,
{
    type Item = RefMutMulti<'a, K, V>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.shards
            .into_par_iter()
            .flat_map_iter(|shard| unsafe {
                let guard = Arc::new(shard.write());
                guard.iter().map(move |b| {
                    let guard = Arc::clone(&guard);
                    let (k, v) = b.as_mut();
                    RefMutMulti::new(guard, k, v.get_mut())
                })
            })
            .drive_unindexed(consumer)
    }
}
