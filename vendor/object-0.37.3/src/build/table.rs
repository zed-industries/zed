use alloc::vec::Vec;

/// An item in a [`Table`].
pub trait Item {
    /// The type of identifier for the item.
    type Id: Id;

    /// Return `True` if the item is deleted.
    fn is_deleted(&self) -> bool;
}

/// An identifier for referring to an item in a [`Table`].
pub trait Id: IdPrivate {
    /// Return the index of the item in the table.
    fn index(&self) -> usize;
}

mod id_private {
    pub trait IdPrivate {
        fn new(id: usize) -> Self;
    }
}
pub(super) use id_private::IdPrivate;

/// A table of items.
///
/// Each item has a unique identifier.
/// Items can be deleted without changing the identifiers of other items.
#[derive(Debug)]
pub struct Table<T>(Vec<T>);

impl<T> Table<T> {
    pub(super) fn new() -> Self {
        Table(Vec::new())
    }
}

impl<T: Item> Table<T> {
    pub(super) fn next_id(&self) -> T::Id {
        T::Id::new(self.0.len())
    }

    pub(super) fn push(&mut self, item: T) -> &mut T {
        self.0.push(item);
        self.0.last_mut().unwrap()
    }

    /// Number of items, including deleted items.
    pub(super) fn len(&self) -> usize {
        self.0.len()
    }

    /// Return `True` if there are no non-deleted items.
    pub fn is_empty(&self) -> bool {
        self.into_iter().next().is_none()
    }

    /// Number of non-deleted items.
    pub fn count(&self) -> usize {
        self.into_iter().count()
    }

    /// Return a reference to an item.
    pub fn get(&self, id: T::Id) -> &T {
        self.0.get(id.index()).unwrap()
    }

    /// Return a mutable reference to a segment.
    pub fn get_mut(&mut self, id: T::Id) -> &mut T {
        self.0.get_mut(id.index()).unwrap()
    }

    /// Return an iterator for the segments.
    pub fn iter(&self) -> TableIter<'_, T> {
        self.into_iter()
    }

    /// Return a mutable iterator for the segments.
    pub fn iter_mut(&mut self) -> TableIterMut<'_, T> {
        self.into_iter()
    }
}

impl<'a, T: Item> IntoIterator for &'a Table<T> {
    type Item = &'a T;
    type IntoIter = TableIter<'a, T>;
    fn into_iter(self) -> TableIter<'a, T> {
        TableIter {
            iter: self.0.iter(),
        }
    }
}

impl<'a, T: Item> IntoIterator for &'a mut Table<T> {
    type Item = &'a mut T;
    type IntoIter = TableIterMut<'a, T>;
    fn into_iter(self) -> TableIterMut<'a, T> {
        TableIterMut {
            iter: self.0.iter_mut(),
        }
    }
}

/// An iterator for non-deleted items in a [`Table`].
#[derive(Debug)]
pub struct TableIter<'a, T> {
    iter: core::slice::Iter<'a, T>,
}

impl<'a, T: Item> Iterator for TableIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        self.iter.find(|item| !item.is_deleted())
    }
}

/// An iterator for non-deleted items in a [`Table`].
#[derive(Debug)]
pub struct TableIterMut<'a, T> {
    iter: core::slice::IterMut<'a, T>,
}

impl<'a, T: Item> Iterator for TableIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        self.iter.find(|item| !item.is_deleted())
    }
}
