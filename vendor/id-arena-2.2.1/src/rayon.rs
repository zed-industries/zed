extern crate rayon;

use self::rayon::iter::plumbing::{Consumer, UnindexedConsumer};
use self::rayon::iter::plumbing::ProducerCallback;
use self::rayon::prelude::*;
use super::*;

impl<T, A> Arena<T, A>
where
    A: ArenaBehavior,
{
    /// Returns an iterator of shared references which can be used to iterate
    /// over this arena in parallel with the `rayon` crate.
    ///
    /// # Features
    ///
    /// This API requires the `rayon` feature of this crate to be enabled.
    pub fn par_iter(&self) -> ParIter<T, A>
    where
        T: Sync,
        A::Id: Send,
    {
        ParIter {
            arena_id: self.arena_id,
            iter: self.items.par_iter().enumerate(),
            _phantom: PhantomData,
        }
    }

    /// Returns an iterator of mutable references which can be used to iterate
    /// over this arena in parallel with the `rayon` crate.
    ///
    /// # Features
    ///
    /// This API requires the `rayon` feature of this crate to be enabled.
    pub fn par_iter_mut(&mut self) -> ParIterMut<T, A>
    where
        T: Send + Sync,
        A::Id: Send,
    {
        ParIterMut {
            arena_id: self.arena_id,
            iter: self.items.par_iter_mut().enumerate(),
            _phantom: PhantomData,
        }
    }
}

/// A parallel iterator over shared references in an arena.
///
/// See `Arena::par_iter` for more information.
#[derive(Debug)]
pub struct ParIter<'a, T, A>
where
    T: Sync,
{
    arena_id: u32,
    iter: rayon::iter::Enumerate<rayon::slice::Iter<'a, T>>,
    _phantom: PhantomData<fn() -> A>,
}

impl<'a, T, A> ParallelIterator for ParIter<'a, T, A>
where
    T: Sync,
    A: ArenaBehavior,
    A::Id: Send,
{
    type Item = (A::Id, &'a T);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let arena_id = self.arena_id;
        self.iter.map(|(i, item)| (A::new_id(arena_id, i), item))
            .drive_unindexed(consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        self.iter.opt_len()
    }
}

impl<'a, T, A> IndexedParallelIterator for ParIter<'a, T, A>
where
    T: Sync,
    A: ArenaBehavior,
    A::Id: Send,
{
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        let arena_id = self.arena_id;
        self.iter.map(|(i, item)| (A::new_id(arena_id, i), item))
            .drive(consumer)
    }

    fn len(&self) -> usize {
        self.iter.len()
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        let arena_id = self.arena_id;
        self.iter.map(|(i, item)| (A::new_id(arena_id, i), item))
            .with_producer(callback)
    }
}

impl<'data, T, A> IntoParallelIterator for &'data Arena<T, A>
    where A: ArenaBehavior,
          A::Id: Send,
          T: Sync,
{
    type Item = (A::Id, &'data T);
    type Iter = ParIter<'data, T, A>;

    fn into_par_iter(self) -> Self::Iter {
        self.par_iter()
    }
}

/// A parallel iterator over mutable references in an arena.
///
/// See `Arena::par_iter_mut` for more information.
#[derive(Debug)]
pub struct ParIterMut<'a, T, A>
where
    T: Send + Sync,
{
    arena_id: u32,
    iter: rayon::iter::Enumerate<rayon::slice::IterMut<'a, T>>,
    _phantom: PhantomData<fn() -> A>,
}

impl<'a, T, A> ParallelIterator for ParIterMut<'a, T, A>
where
    T: Send + Sync,
    A: ArenaBehavior,
    A::Id: Send,
{
    type Item = (A::Id, &'a mut T);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let arena_id = self.arena_id;
        self.iter.map(|(i, item)| (A::new_id(arena_id, i), item))
            .drive_unindexed(consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        self.iter.opt_len()
    }
}

impl<'a, T, A> IndexedParallelIterator for ParIterMut<'a, T, A>
where
    T: Send + Sync,
    A: ArenaBehavior,
    A::Id: Send,
{
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        let arena_id = self.arena_id;
        self.iter.map(|(i, item)| (A::new_id(arena_id, i), item))
            .drive(consumer)
    }

    fn len(&self) -> usize {
        self.iter.len()
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        let arena_id = self.arena_id;
        self.iter.map(|(i, item)| (A::new_id(arena_id, i), item))
            .with_producer(callback)
    }
}

impl<'data, T, A> IntoParallelIterator for &'data mut Arena<T, A>
    where A: ArenaBehavior,
          A::Id: Send,
          T: Send + Sync,
{
    type Item = (A::Id, &'data mut T);
    type Iter = ParIterMut<'data, T, A>;

    fn into_par_iter(self) -> Self::Iter {
        self.par_iter_mut()
    }
}

/// A parallel iterator over items in an arena.
///
/// See `Arena::into_par_iter` for more information.
#[derive(Debug)]
pub struct IntoParIter<T, A>
where
    T: Send,
{
    arena_id: u32,
    iter: rayon::iter::Enumerate<rayon::vec::IntoIter<T>>,
    _phantom: PhantomData<fn() -> A>,
}

impl<T, A> ParallelIterator for IntoParIter<T, A>
where
    T: Send,
    A: ArenaBehavior,
    A::Id: Send,
{
    type Item = (A::Id, T);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let arena_id = self.arena_id;
        self.iter.map(|(i, item)| (A::new_id(arena_id, i), item))
            .drive_unindexed(consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        self.iter.opt_len()
    }
}

impl<T, A> IndexedParallelIterator for IntoParIter<T, A>
where
    T: Send,
    A: ArenaBehavior,
    A::Id: Send,
{
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        let arena_id = self.arena_id;
        self.iter.map(|(i, item)| (A::new_id(arena_id, i), item))
            .drive(consumer)
    }

    fn len(&self) -> usize {
        self.iter.len()
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        let arena_id = self.arena_id;
        self.iter.map(|(i, item)| (A::new_id(arena_id, i), item))
            .with_producer(callback)
    }
}

impl<T, A> IntoParallelIterator for Arena<T, A>
    where A: ArenaBehavior,
          A::Id: Send,
          T: Send,
{
    type Item = (A::Id, T);
    type Iter = IntoParIter<T, A>;

    fn into_par_iter(self) -> Self::Iter {
        IntoParIter {
            arena_id: self.arena_id,
            iter: self.items.into_par_iter().enumerate(),
            _phantom: PhantomData,
        }
    }
}
