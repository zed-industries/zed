use super::noop::NoopConsumer;
use super::plumbing::{Consumer, Folder, Reducer, UnindexedConsumer};
use super::{IntoParallelIterator, ParallelExtend, ParallelIterator};

use either::Either;
use std::borrow::Cow;
use std::collections::LinkedList;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::collections::{BinaryHeap, VecDeque};
use std::ffi::{OsStr, OsString};
use std::hash::{BuildHasher, Hash};

/// Performs a generic `par_extend` by collecting to a `LinkedList<Vec<_>>` in
/// parallel, then extending the collection sequentially.
macro_rules! extend {
    ($self:ident, $par_iter:ident) => {
        extend!($self <- fast_collect($par_iter))
    };
    ($self:ident <- $vecs:expr) => {
        match $vecs {
            Either::Left(vec) => $self.extend(vec),
            Either::Right(list) => {
                for vec in list {
                    $self.extend(vec);
                }
            }
        }
    };
}
macro_rules! extend_reserved {
    ($self:ident, $par_iter:ident, $len:ident) => {
        let vecs = fast_collect($par_iter);
        $self.reserve($len(&vecs));
        extend!($self <- vecs)
    };
    ($self:ident, $par_iter:ident) => {
        extend_reserved!($self, $par_iter, len)
    };
}

/// Computes the total length of a `fast_collect` result.
fn len<T>(vecs: &Either<Vec<T>, LinkedList<Vec<T>>>) -> usize {
    match vecs {
        Either::Left(vec) => vec.len(),
        Either::Right(list) => list.iter().map(Vec::len).sum(),
    }
}

/// Computes the total string length of a `fast_collect` result.
fn string_len<T: AsRef<str>>(vecs: &Either<Vec<T>, LinkedList<Vec<T>>>) -> usize {
    let strs = match vecs {
        Either::Left(vec) => Either::Left(vec.iter()),
        Either::Right(list) => Either::Right(list.iter().flatten()),
    };
    strs.map(AsRef::as_ref).map(str::len).sum()
}

/// Computes the total OS-string length of a `fast_collect` result.
fn osstring_len<T: AsRef<OsStr>>(vecs: &Either<Vec<T>, LinkedList<Vec<T>>>) -> usize {
    let osstrs = match vecs {
        Either::Left(vec) => Either::Left(vec.iter()),
        Either::Right(list) => Either::Right(list.iter().flatten()),
    };
    osstrs.map(AsRef::as_ref).map(OsStr::len).sum()
}

pub(super) fn fast_collect<I, T>(pi: I) -> Either<Vec<T>, LinkedList<Vec<T>>>
where
    I: IntoParallelIterator<Item = T>,
    T: Send,
{
    let par_iter = pi.into_par_iter();
    match par_iter.opt_len() {
        Some(len) => {
            // Pseudo-specialization. See impl of ParallelExtend for Vec for more details.
            let mut vec = Vec::new();
            super::collect::special_extend(par_iter, len, &mut vec);
            Either::Left(vec)
        }
        None => Either::Right(par_iter.drive_unindexed(ListVecConsumer)),
    }
}

struct ListVecConsumer;

struct ListVecFolder<T> {
    vec: Vec<T>,
}

impl<T: Send> Consumer<T> for ListVecConsumer {
    type Folder = ListVecFolder<T>;
    type Reducer = ListReducer;
    type Result = LinkedList<Vec<T>>;

    fn split_at(self, _index: usize) -> (Self, Self, Self::Reducer) {
        (Self, Self, ListReducer)
    }

    fn into_folder(self) -> Self::Folder {
        ListVecFolder { vec: Vec::new() }
    }

    fn full(&self) -> bool {
        false
    }
}

impl<T: Send> UnindexedConsumer<T> for ListVecConsumer {
    fn split_off_left(&self) -> Self {
        Self
    }

    fn to_reducer(&self) -> Self::Reducer {
        ListReducer
    }
}

impl<T> Folder<T> for ListVecFolder<T> {
    type Result = LinkedList<Vec<T>>;

    fn consume(mut self, item: T) -> Self {
        self.vec.push(item);
        self
    }

    fn consume_iter<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        self.vec.extend(iter);
        self
    }

    fn complete(self) -> Self::Result {
        let mut list = LinkedList::new();
        if !self.vec.is_empty() {
            list.push_back(self.vec);
        }
        list
    }

    fn full(&self) -> bool {
        false
    }
}

/// Extends a binary heap with items from a parallel iterator.
impl<T> ParallelExtend<T> for BinaryHeap<T>
where
    T: Ord + Send,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = T>,
    {
        extend_reserved!(self, par_iter);
    }
}

/// Extends a binary heap with copied items from a parallel iterator.
impl<'a, T> ParallelExtend<&'a T> for BinaryHeap<T>
where
    T: 'a + Copy + Ord + Send + Sync,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = &'a T>,
    {
        extend_reserved!(self, par_iter);
    }
}

/// Extends a B-tree map with items from a parallel iterator.
impl<K, V> ParallelExtend<(K, V)> for BTreeMap<K, V>
where
    K: Ord + Send,
    V: Send,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = (K, V)>,
    {
        extend!(self, par_iter);
    }
}

/// Extends a B-tree map with copied items from a parallel iterator.
impl<'a, K: 'a, V: 'a> ParallelExtend<(&'a K, &'a V)> for BTreeMap<K, V>
where
    K: Copy + Ord + Send + Sync,
    V: Copy + Send + Sync,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = (&'a K, &'a V)>,
    {
        extend!(self, par_iter);
    }
}

/// Extends a B-tree set with items from a parallel iterator.
impl<T> ParallelExtend<T> for BTreeSet<T>
where
    T: Ord + Send,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = T>,
    {
        extend!(self, par_iter);
    }
}

/// Extends a B-tree set with copied items from a parallel iterator.
impl<'a, T> ParallelExtend<&'a T> for BTreeSet<T>
where
    T: 'a + Copy + Ord + Send + Sync,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = &'a T>,
    {
        extend!(self, par_iter);
    }
}

/// Extends a hash map with items from a parallel iterator.
impl<K, V, S> ParallelExtend<(K, V)> for HashMap<K, V, S>
where
    K: Eq + Hash + Send,
    V: Send,
    S: BuildHasher + Send,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = (K, V)>,
    {
        // See the map_collect benchmarks in rayon-demo for different strategies.
        extend_reserved!(self, par_iter);
    }
}

/// Extends a hash map with copied items from a parallel iterator.
impl<'a, K: 'a, V: 'a, S> ParallelExtend<(&'a K, &'a V)> for HashMap<K, V, S>
where
    K: Copy + Eq + Hash + Send + Sync,
    V: Copy + Send + Sync,
    S: BuildHasher + Send,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = (&'a K, &'a V)>,
    {
        extend_reserved!(self, par_iter);
    }
}

/// Extends a hash set with items from a parallel iterator.
impl<T, S> ParallelExtend<T> for HashSet<T, S>
where
    T: Eq + Hash + Send,
    S: BuildHasher + Send,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = T>,
    {
        extend_reserved!(self, par_iter);
    }
}

/// Extends a hash set with copied items from a parallel iterator.
impl<'a, T, S> ParallelExtend<&'a T> for HashSet<T, S>
where
    T: 'a + Copy + Eq + Hash + Send + Sync,
    S: BuildHasher + Send,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = &'a T>,
    {
        extend_reserved!(self, par_iter);
    }
}

/// Extends a linked list with items from a parallel iterator.
impl<T> ParallelExtend<T> for LinkedList<T>
where
    T: Send,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = T>,
    {
        let mut list = par_iter.into_par_iter().drive_unindexed(ListConsumer);
        self.append(&mut list);
    }
}

/// Extends a linked list with copied items from a parallel iterator.
impl<'a, T> ParallelExtend<&'a T> for LinkedList<T>
where
    T: 'a + Copy + Send + Sync,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = &'a T>,
    {
        self.par_extend(par_iter.into_par_iter().copied())
    }
}

struct ListConsumer;

struct ListFolder<T> {
    list: LinkedList<T>,
}

struct ListReducer;

impl<T: Send> Consumer<T> for ListConsumer {
    type Folder = ListFolder<T>;
    type Reducer = ListReducer;
    type Result = LinkedList<T>;

    fn split_at(self, _index: usize) -> (Self, Self, Self::Reducer) {
        (Self, Self, ListReducer)
    }

    fn into_folder(self) -> Self::Folder {
        ListFolder {
            list: LinkedList::new(),
        }
    }

    fn full(&self) -> bool {
        false
    }
}

impl<T: Send> UnindexedConsumer<T> for ListConsumer {
    fn split_off_left(&self) -> Self {
        Self
    }

    fn to_reducer(&self) -> Self::Reducer {
        ListReducer
    }
}

impl<T> Folder<T> for ListFolder<T> {
    type Result = LinkedList<T>;

    fn consume(mut self, item: T) -> Self {
        self.list.push_back(item);
        self
    }

    fn consume_iter<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        self.list.extend(iter);
        self
    }

    fn complete(self) -> Self::Result {
        self.list
    }

    fn full(&self) -> bool {
        false
    }
}

impl<T> Reducer<LinkedList<T>> for ListReducer {
    fn reduce(self, mut left: LinkedList<T>, mut right: LinkedList<T>) -> LinkedList<T> {
        left.append(&mut right);
        left
    }
}

/// Extends an OS-string with string slices from a parallel iterator.
impl<'a> ParallelExtend<&'a OsStr> for OsString {
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = &'a OsStr>,
    {
        extend_reserved!(self, par_iter, osstring_len);
    }
}

/// Extends an OS-string with strings from a parallel iterator.
impl ParallelExtend<OsString> for OsString {
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = OsString>,
    {
        extend_reserved!(self, par_iter, osstring_len);
    }
}

/// Extends an OS-string with string slices from a parallel iterator.
impl<'a> ParallelExtend<Cow<'a, OsStr>> for OsString {
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = Cow<'a, OsStr>>,
    {
        extend_reserved!(self, par_iter, osstring_len);
    }
}

/// Extends a string with characters from a parallel iterator.
impl ParallelExtend<char> for String {
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = char>,
    {
        // This is like `extend`, but `Vec<char>` is less efficient to deal
        // with than `String`, so instead collect to `LinkedList<String>`.
        let list = par_iter.into_par_iter().drive_unindexed(ListStringConsumer);
        self.reserve(list.iter().map(String::len).sum());
        self.extend(list);
    }
}

/// Extends a string with copied characters from a parallel iterator.
impl<'a> ParallelExtend<&'a char> for String {
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = &'a char>,
    {
        self.par_extend(par_iter.into_par_iter().copied())
    }
}

struct ListStringConsumer;

struct ListStringFolder {
    string: String,
}

impl Consumer<char> for ListStringConsumer {
    type Folder = ListStringFolder;
    type Reducer = ListReducer;
    type Result = LinkedList<String>;

    fn split_at(self, _index: usize) -> (Self, Self, Self::Reducer) {
        (Self, Self, ListReducer)
    }

    fn into_folder(self) -> Self::Folder {
        ListStringFolder {
            string: String::new(),
        }
    }

    fn full(&self) -> bool {
        false
    }
}

impl UnindexedConsumer<char> for ListStringConsumer {
    fn split_off_left(&self) -> Self {
        Self
    }

    fn to_reducer(&self) -> Self::Reducer {
        ListReducer
    }
}

impl Folder<char> for ListStringFolder {
    type Result = LinkedList<String>;

    fn consume(mut self, item: char) -> Self {
        self.string.push(item);
        self
    }

    fn consume_iter<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = char>,
    {
        self.string.extend(iter);
        self
    }

    fn complete(self) -> Self::Result {
        let mut list = LinkedList::new();
        if !self.string.is_empty() {
            list.push_back(self.string);
        }
        list
    }

    fn full(&self) -> bool {
        false
    }
}

/// Extends a string with string slices from a parallel iterator.
impl<'a> ParallelExtend<&'a str> for String {
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = &'a str>,
    {
        extend_reserved!(self, par_iter, string_len);
    }
}

/// Extends a string with strings from a parallel iterator.
impl ParallelExtend<String> for String {
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = String>,
    {
        extend_reserved!(self, par_iter, string_len);
    }
}

/// Extends a string with boxed strings from a parallel iterator.
impl ParallelExtend<Box<str>> for String {
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = Box<str>>,
    {
        extend_reserved!(self, par_iter, string_len);
    }
}

/// Extends a string with string slices from a parallel iterator.
impl<'a> ParallelExtend<Cow<'a, str>> for String {
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = Cow<'a, str>>,
    {
        extend_reserved!(self, par_iter, string_len);
    }
}

/// Extends a deque with items from a parallel iterator.
impl<T> ParallelExtend<T> for VecDeque<T>
where
    T: Send,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = T>,
    {
        extend_reserved!(self, par_iter);
    }
}

/// Extends a deque with copied items from a parallel iterator.
impl<'a, T> ParallelExtend<&'a T> for VecDeque<T>
where
    T: 'a + Copy + Send + Sync,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = &'a T>,
    {
        extend_reserved!(self, par_iter);
    }
}

/// Extends a vector with items from a parallel iterator.
impl<T> ParallelExtend<T> for Vec<T>
where
    T: Send,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = T>,
    {
        // See the vec_collect benchmarks in rayon-demo for different strategies.
        let par_iter = par_iter.into_par_iter();
        match par_iter.opt_len() {
            Some(len) => {
                // When Rust gets specialization, we can get here for indexed iterators
                // without relying on `opt_len`.  Until then, `special_extend()` fakes
                // an unindexed mode on the promise that `opt_len()` is accurate.
                super::collect::special_extend(par_iter, len, self);
            }
            None => {
                // This works like `extend`, but `Vec::append` is more efficient.
                let list = par_iter.drive_unindexed(ListVecConsumer);
                self.reserve(list.iter().map(Vec::len).sum());
                for mut other in list {
                    self.append(&mut other);
                }
            }
        }
    }
}

/// Extends a vector with copied items from a parallel iterator.
impl<'a, T> ParallelExtend<&'a T> for Vec<T>
where
    T: 'a + Copy + Send + Sync,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = &'a T>,
    {
        self.par_extend(par_iter.into_par_iter().copied())
    }
}

/// Collapses all unit items from a parallel iterator into one.
impl ParallelExtend<()> for () {
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = ()>,
    {
        par_iter.into_par_iter().drive_unindexed(NoopConsumer)
    }
}
