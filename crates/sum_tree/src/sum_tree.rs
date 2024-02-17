mod cursor;
mod tree_map;

use arrayvec::ArrayVec;
pub use cursor::{Cursor, FilterCursor, Iter};
use rayon::prelude::*;
use std::marker::PhantomData;
use std::mem;
use std::{cmp::Ordering, fmt, iter::FromIterator, sync::Arc};
pub use tree_map::{MapSeekTarget, TreeMap, TreeSet};

#[cfg(test)]
pub const TREE_BASE: usize = 2;
#[cfg(not(test))]
pub const TREE_BASE: usize = 6;

/// An item that can be stored in a [`SumTree`]
///
/// Must be summarized by a type that implements [`Summary`]
pub trait Item: Clone {
    type Summary: Summary;

    fn summary(&self) -> Self::Summary;
}

/// An [`Item`] whose summary has a specific key that can be used to identify it
pub trait KeyedItem: Item {
    type Key: for<'a> Dimension<'a, Self::Summary> + Ord;

    fn key(&self) -> Self::Key;
}

/// A type that describes the Sum of all [`Item`]s in a subtree of the [`SumTree`]
///
/// Each Summary type can have multiple [`Dimensions`] that it measures,
/// which can be used to navigate the tree
pub trait Summary: Default + Clone + fmt::Debug {
    type Context;

    fn add_summary(&mut self, summary: &Self, cx: &Self::Context);
}

/// Each [`Summary`] type can have more than one [`Dimension`] type that it measures.
///
/// You can use dimensions to seek to a specific location in the [`SumTree`]
///
/// # Example:
/// Zed's rope has a `TextSummary` type that summarizes lines, characters, and bytes.
/// Each of these are different dimensions we may want to seek to
pub trait Dimension<'a, S: Summary>: Clone + fmt::Debug + Default {
    fn add_summary(&mut self, _summary: &'a S, _: &S::Context);

    fn from_summary(summary: &'a S, cx: &S::Context) -> Self {
        let mut dimension = Self::default();
        dimension.add_summary(summary, cx);
        dimension
    }
}

impl<'a, T: Summary> Dimension<'a, T> for T {
    fn add_summary(&mut self, summary: &'a T, cx: &T::Context) {
        Summary::add_summary(self, summary, cx);
    }
}

pub trait SeekTarget<'a, S: Summary, D: Dimension<'a, S>>: fmt::Debug {
    fn cmp(&self, cursor_location: &D, cx: &S::Context) -> Ordering;
}

impl<'a, S: Summary, D: Dimension<'a, S> + Ord> SeekTarget<'a, S, D> for D {
    fn cmp(&self, cursor_location: &Self, _: &S::Context) -> Ordering {
        Ord::cmp(self, cursor_location)
    }
}

impl<'a, T: Summary> Dimension<'a, T> for () {
    fn add_summary(&mut self, _: &'a T, _: &T::Context) {}
}

impl<'a, T: Summary, D1: Dimension<'a, T>, D2: Dimension<'a, T>> Dimension<'a, T> for (D1, D2) {
    fn add_summary(&mut self, summary: &'a T, cx: &T::Context) {
        self.0.add_summary(summary, cx);
        self.1.add_summary(summary, cx);
    }
}

impl<'a, S: Summary, D1: SeekTarget<'a, S, D1> + Dimension<'a, S>, D2: Dimension<'a, S>>
    SeekTarget<'a, S, (D1, D2)> for D1
{
    fn cmp(&self, cursor_location: &(D1, D2), cx: &S::Context) -> Ordering {
        self.cmp(&cursor_location.0, cx)
    }
}

struct End<D>(PhantomData<D>);

impl<D> End<D> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'a, S: Summary, D: Dimension<'a, S>> SeekTarget<'a, S, D> for End<D> {
    fn cmp(&self, _: &D, _: &S::Context) -> Ordering {
        Ordering::Greater
    }
}

impl<D> fmt::Debug for End<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("End").finish()
    }
}

/// Bias is used to settle ambiguities when determining positions in an ordered sequence.
///
/// The primary use case is for text, where Bias influences
/// which character an offset or anchor is associated with.
///
/// # Examples
/// Given the buffer `AˇBCD`:
/// - The offset of the cursor is 1
/// - [Bias::Left] would attach the cursor to the character `A`
/// - [Bias::Right] would attach the cursor to the character `B`
///
/// Given the buffer `A«BCˇ»D`:
/// - The offset of the cursor is 3, and the selection is from 1 to 3
/// - The left anchor of the selection has [Bias::Right], attaching it to the character `B`
/// - The right anchor of the selection has [Bias::Left], attaching it to the character `C`
///
/// Given the buffer `{ˇ<...>`, where `<...>` is a folded region:
/// - The display offset of the cursor is 1, but the offset in the buffer is determined by the bias
/// - [Bias::Left] would attach the cursor to the character `{`, with a buffer offset of 1
/// - [Bias::Right] would attach the cursor to the first character of the folded region,
///   and the buffer offset would be the offset of the first character of the folded region
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash, Default)]
pub enum Bias {
    /// Attach to the character on the left
    #[default]
    Left,
    /// Attach to the character on the right
    Right,
}

impl Bias {
    pub fn invert(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

/// A B-tree where each leaf node contains an [`Item`] of type `T`,
/// and each internal node contains a [`Summary`] of the items in its subtree.
///
/// Any [`Dimension`] supported by the [`Summary`] type can be used to seek to a specific location in the tree.
#[derive(Debug, Clone)]
pub struct SumTree<T: Item>(Arc<Node<T>>);

impl<T: Item> SumTree<T> {
    pub fn new() -> Self {
        SumTree(Arc::new(Node::Leaf {
            summary: T::Summary::default(),
            items: ArrayVec::new(),
            item_summaries: ArrayVec::new(),
        }))
    }

    pub fn from_item(item: T, cx: &<T::Summary as Summary>::Context) -> Self {
        let mut tree = Self::new();
        tree.push(item, cx);
        tree
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(
        iter: I,
        cx: &<T::Summary as Summary>::Context,
    ) -> Self {
        let mut nodes = Vec::new();

        let mut iter = iter.into_iter().peekable();
        while iter.peek().is_some() {
            let items: ArrayVec<T, { 2 * TREE_BASE }> = iter.by_ref().take(2 * TREE_BASE).collect();
            let item_summaries: ArrayVec<T::Summary, { 2 * TREE_BASE }> =
                items.iter().map(|item| item.summary()).collect();

            let mut summary = item_summaries[0].clone();
            for item_summary in &item_summaries[1..] {
                <T::Summary as Summary>::add_summary(&mut summary, item_summary, cx);
            }

            nodes.push(Node::Leaf {
                summary,
                items,
                item_summaries,
            });
        }

        let mut parent_nodes = Vec::new();
        let mut height = 0;
        while nodes.len() > 1 {
            height += 1;
            let mut current_parent_node = None;
            for child_node in nodes.drain(..) {
                let parent_node = current_parent_node.get_or_insert_with(|| Node::Internal {
                    summary: T::Summary::default(),
                    height,
                    child_summaries: ArrayVec::new(),
                    child_trees: ArrayVec::new(),
                });
                let Node::Internal {
                    summary,
                    child_summaries,
                    child_trees,
                    ..
                } = parent_node
                else {
                    unreachable!()
                };
                let child_summary = child_node.summary();
                <T::Summary as Summary>::add_summary(summary, child_summary, cx);
                child_summaries.push(child_summary.clone());
                child_trees.push(Self(Arc::new(child_node)));

                if child_trees.len() == 2 * TREE_BASE {
                    parent_nodes.extend(current_parent_node.take());
                }
            }
            parent_nodes.extend(current_parent_node.take());
            mem::swap(&mut nodes, &mut parent_nodes);
        }

        if nodes.is_empty() {
            Self::new()
        } else {
            debug_assert_eq!(nodes.len(), 1);
            Self(Arc::new(nodes.pop().unwrap()))
        }
    }

    pub fn from_par_iter<I, Iter>(iter: I, cx: &<T::Summary as Summary>::Context) -> Self
    where
        I: IntoParallelIterator<Iter = Iter>,
        Iter: IndexedParallelIterator<Item = T>,
        T: Send + Sync,
        T::Summary: Send + Sync,
        <T::Summary as Summary>::Context: Sync,
    {
        let mut nodes = iter
            .into_par_iter()
            .chunks(2 * TREE_BASE)
            .map(|items| {
                let items: ArrayVec<T, { 2 * TREE_BASE }> = items.into_iter().collect();
                let item_summaries: ArrayVec<T::Summary, { 2 * TREE_BASE }> =
                    items.iter().map(|item| item.summary()).collect();
                let mut summary = item_summaries[0].clone();
                for item_summary in &item_summaries[1..] {
                    <T::Summary as Summary>::add_summary(&mut summary, item_summary, cx);
                }
                SumTree(Arc::new(Node::Leaf {
                    summary,
                    items,
                    item_summaries,
                }))
            })
            .collect::<Vec<_>>();

        let mut height = 0;
        while nodes.len() > 1 {
            height += 1;
            nodes = nodes
                .into_par_iter()
                .chunks(2 * TREE_BASE)
                .map(|child_nodes| {
                    let child_trees: ArrayVec<SumTree<T>, { 2 * TREE_BASE }> =
                        child_nodes.into_iter().collect();
                    let child_summaries: ArrayVec<T::Summary, { 2 * TREE_BASE }> = child_trees
                        .iter()
                        .map(|child_tree| child_tree.summary().clone())
                        .collect();
                    let mut summary = child_summaries[0].clone();
                    for child_summary in &child_summaries[1..] {
                        <T::Summary as Summary>::add_summary(&mut summary, child_summary, cx);
                    }
                    SumTree(Arc::new(Node::Internal {
                        height,
                        summary,
                        child_summaries,
                        child_trees,
                    }))
                })
                .collect::<Vec<_>>();
        }

        if nodes.is_empty() {
            Self::new()
        } else {
            debug_assert_eq!(nodes.len(), 1);
            nodes.pop().unwrap()
        }
    }

    #[allow(unused)]
    pub fn items(&self, cx: &<T::Summary as Summary>::Context) -> Vec<T> {
        let mut items = Vec::new();
        let mut cursor = self.cursor::<()>();
        cursor.next(cx);
        while let Some(item) = cursor.item() {
            items.push(item.clone());
            cursor.next(cx);
        }
        items
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }

    pub fn cursor<'a, S>(&'a self) -> Cursor<T, S>
    where
        S: Dimension<'a, T::Summary>,
    {
        Cursor::new(self)
    }

    /// Note: If the summary type requires a non `()` context, then the filter cursor
    /// that is returned cannot be used with Rust's iterators.
    pub fn filter<'a, F, U>(&'a self, filter_node: F) -> FilterCursor<F, T, U>
    where
        F: FnMut(&T::Summary) -> bool,
        U: Dimension<'a, T::Summary>,
    {
        FilterCursor::new(self, filter_node)
    }

    #[allow(dead_code)]
    pub fn first(&self) -> Option<&T> {
        self.leftmost_leaf().0.items().first()
    }

    pub fn last(&self) -> Option<&T> {
        self.rightmost_leaf().0.items().last()
    }

    pub fn update_last(&mut self, f: impl FnOnce(&mut T), cx: &<T::Summary as Summary>::Context) {
        self.update_last_recursive(f, cx);
    }

    fn update_last_recursive(
        &mut self,
        f: impl FnOnce(&mut T),
        cx: &<T::Summary as Summary>::Context,
    ) -> Option<T::Summary> {
        match Arc::make_mut(&mut self.0) {
            Node::Internal {
                summary,
                child_summaries,
                child_trees,
                ..
            } => {
                let last_summary = child_summaries.last_mut().unwrap();
                let last_child = child_trees.last_mut().unwrap();
                *last_summary = last_child.update_last_recursive(f, cx).unwrap();
                *summary = sum(child_summaries.iter(), cx);
                Some(summary.clone())
            }
            Node::Leaf {
                summary,
                items,
                item_summaries,
            } => {
                if let Some((item, item_summary)) = items.last_mut().zip(item_summaries.last_mut())
                {
                    (f)(item);
                    *item_summary = item.summary();
                    *summary = sum(item_summaries.iter(), cx);
                    Some(summary.clone())
                } else {
                    None
                }
            }
        }
    }

    pub fn extent<'a, D: Dimension<'a, T::Summary>>(
        &'a self,
        cx: &<T::Summary as Summary>::Context,
    ) -> D {
        let mut extent = D::default();
        match self.0.as_ref() {
            Node::Internal { summary, .. } | Node::Leaf { summary, .. } => {
                extent.add_summary(summary, cx);
            }
        }
        extent
    }

    pub fn summary(&self) -> &T::Summary {
        match self.0.as_ref() {
            Node::Internal { summary, .. } => summary,
            Node::Leaf { summary, .. } => summary,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.0.as_ref() {
            Node::Internal { .. } => false,
            Node::Leaf { items, .. } => items.is_empty(),
        }
    }

    pub fn extend<I>(&mut self, iter: I, cx: &<T::Summary as Summary>::Context)
    where
        I: IntoIterator<Item = T>,
    {
        self.append(Self::from_iter(iter, cx), cx);
    }

    pub fn par_extend<I, Iter>(&mut self, iter: I, cx: &<T::Summary as Summary>::Context)
    where
        I: IntoParallelIterator<Iter = Iter>,
        Iter: IndexedParallelIterator<Item = T>,
        T: Send + Sync,
        T::Summary: Send + Sync,
        <T::Summary as Summary>::Context: Sync,
    {
        self.append(Self::from_par_iter(iter, cx), cx);
    }

    pub fn push(&mut self, item: T, cx: &<T::Summary as Summary>::Context) {
        let summary = item.summary();
        self.append(
            SumTree(Arc::new(Node::Leaf {
                summary: summary.clone(),
                items: ArrayVec::from_iter(Some(item)),
                item_summaries: ArrayVec::from_iter(Some(summary)),
            })),
            cx,
        );
    }

    pub fn append(&mut self, other: Self, cx: &<T::Summary as Summary>::Context) {
        if self.is_empty() {
            *self = other;
        } else if !other.0.is_leaf() || !other.0.items().is_empty() {
            if self.0.height() < other.0.height() {
                for tree in other.0.child_trees() {
                    self.append(tree.clone(), cx);
                }
            } else if let Some(split_tree) = self.push_tree_recursive(other, cx) {
                *self = Self::from_child_trees(self.clone(), split_tree, cx);
            }
        }
    }

    fn push_tree_recursive(
        &mut self,
        other: SumTree<T>,
        cx: &<T::Summary as Summary>::Context,
    ) -> Option<SumTree<T>> {
        match Arc::make_mut(&mut self.0) {
            Node::Internal {
                height,
                summary,
                child_summaries,
                child_trees,
                ..
            } => {
                let other_node = other.0.clone();
                <T::Summary as Summary>::add_summary(summary, other_node.summary(), cx);

                let height_delta = *height - other_node.height();
                let mut summaries_to_append = ArrayVec::<T::Summary, { 2 * TREE_BASE }>::new();
                let mut trees_to_append = ArrayVec::<SumTree<T>, { 2 * TREE_BASE }>::new();
                if height_delta == 0 {
                    summaries_to_append.extend(other_node.child_summaries().iter().cloned());
                    trees_to_append.extend(other_node.child_trees().iter().cloned());
                } else if height_delta == 1 && !other_node.is_underflowing() {
                    summaries_to_append.push(other_node.summary().clone());
                    trees_to_append.push(other)
                } else {
                    let tree_to_append = child_trees
                        .last_mut()
                        .unwrap()
                        .push_tree_recursive(other, cx);
                    *child_summaries.last_mut().unwrap() =
                        child_trees.last().unwrap().0.summary().clone();

                    if let Some(split_tree) = tree_to_append {
                        summaries_to_append.push(split_tree.0.summary().clone());
                        trees_to_append.push(split_tree);
                    }
                }

                let child_count = child_trees.len() + trees_to_append.len();
                if child_count > 2 * TREE_BASE {
                    let left_summaries: ArrayVec<_, { 2 * TREE_BASE }>;
                    let right_summaries: ArrayVec<_, { 2 * TREE_BASE }>;
                    let left_trees;
                    let right_trees;

                    let midpoint = (child_count + child_count % 2) / 2;
                    {
                        let mut all_summaries = child_summaries
                            .iter()
                            .chain(summaries_to_append.iter())
                            .cloned();
                        left_summaries = all_summaries.by_ref().take(midpoint).collect();
                        right_summaries = all_summaries.collect();
                        let mut all_trees =
                            child_trees.iter().chain(trees_to_append.iter()).cloned();
                        left_trees = all_trees.by_ref().take(midpoint).collect();
                        right_trees = all_trees.collect();
                    }
                    *summary = sum(left_summaries.iter(), cx);
                    *child_summaries = left_summaries;
                    *child_trees = left_trees;

                    Some(SumTree(Arc::new(Node::Internal {
                        height: *height,
                        summary: sum(right_summaries.iter(), cx),
                        child_summaries: right_summaries,
                        child_trees: right_trees,
                    })))
                } else {
                    child_summaries.extend(summaries_to_append);
                    child_trees.extend(trees_to_append);
                    None
                }
            }
            Node::Leaf {
                summary,
                items,
                item_summaries,
            } => {
                let other_node = other.0;

                let child_count = items.len() + other_node.items().len();
                if child_count > 2 * TREE_BASE {
                    let left_items;
                    let right_items;
                    let left_summaries;
                    let right_summaries: ArrayVec<T::Summary, { 2 * TREE_BASE }>;

                    let midpoint = (child_count + child_count % 2) / 2;
                    {
                        let mut all_items = items.iter().chain(other_node.items().iter()).cloned();
                        left_items = all_items.by_ref().take(midpoint).collect();
                        right_items = all_items.collect();

                        let mut all_summaries = item_summaries
                            .iter()
                            .chain(other_node.child_summaries())
                            .cloned();
                        left_summaries = all_summaries.by_ref().take(midpoint).collect();
                        right_summaries = all_summaries.collect();
                    }
                    *items = left_items;
                    *item_summaries = left_summaries;
                    *summary = sum(item_summaries.iter(), cx);
                    Some(SumTree(Arc::new(Node::Leaf {
                        items: right_items,
                        summary: sum(right_summaries.iter(), cx),
                        item_summaries: right_summaries,
                    })))
                } else {
                    <T::Summary as Summary>::add_summary(summary, other_node.summary(), cx);
                    items.extend(other_node.items().iter().cloned());
                    item_summaries.extend(other_node.child_summaries().iter().cloned());
                    None
                }
            }
        }
    }

    fn from_child_trees(
        left: SumTree<T>,
        right: SumTree<T>,
        cx: &<T::Summary as Summary>::Context,
    ) -> Self {
        let height = left.0.height() + 1;
        let mut child_summaries = ArrayVec::new();
        child_summaries.push(left.0.summary().clone());
        child_summaries.push(right.0.summary().clone());
        let mut child_trees = ArrayVec::new();
        child_trees.push(left);
        child_trees.push(right);
        SumTree(Arc::new(Node::Internal {
            height,
            summary: sum(child_summaries.iter(), cx),
            child_summaries,
            child_trees,
        }))
    }

    fn leftmost_leaf(&self) -> &Self {
        match *self.0 {
            Node::Leaf { .. } => self,
            Node::Internal {
                ref child_trees, ..
            } => child_trees.first().unwrap().leftmost_leaf(),
        }
    }

    fn rightmost_leaf(&self) -> &Self {
        match *self.0 {
            Node::Leaf { .. } => self,
            Node::Internal {
                ref child_trees, ..
            } => child_trees.last().unwrap().rightmost_leaf(),
        }
    }

    #[cfg(debug_assertions)]
    pub fn _debug_entries(&self) -> Vec<&T> {
        self.iter().collect::<Vec<_>>()
    }
}

impl<T: Item + PartialEq> PartialEq for SumTree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T: Item + Eq> Eq for SumTree<T> {}

impl<T: KeyedItem> SumTree<T> {
    pub fn insert_or_replace(
        &mut self,
        item: T,
        cx: &<T::Summary as Summary>::Context,
    ) -> Option<T> {
        let mut replaced = None;
        *self = {
            let mut cursor = self.cursor::<T::Key>();
            let mut new_tree = cursor.slice(&item.key(), Bias::Left, cx);
            if let Some(cursor_item) = cursor.item() {
                if cursor_item.key() == item.key() {
                    replaced = Some(cursor_item.clone());
                    cursor.next(cx);
                }
            }
            new_tree.push(item, cx);
            new_tree.append(cursor.suffix(cx), cx);
            new_tree
        };
        replaced
    }

    pub fn remove(&mut self, key: &T::Key, cx: &<T::Summary as Summary>::Context) -> Option<T> {
        let mut removed = None;
        *self = {
            let mut cursor = self.cursor::<T::Key>();
            let mut new_tree = cursor.slice(key, Bias::Left, cx);
            if let Some(item) = cursor.item() {
                if item.key() == *key {
                    removed = Some(item.clone());
                    cursor.next(cx);
                }
            }
            new_tree.append(cursor.suffix(cx), cx);
            new_tree
        };
        removed
    }

    pub fn edit(
        &mut self,
        mut edits: Vec<Edit<T>>,
        cx: &<T::Summary as Summary>::Context,
    ) -> Vec<T> {
        if edits.is_empty() {
            return Vec::new();
        }

        let mut removed = Vec::new();
        edits.sort_unstable_by_key(|item| item.key());

        *self = {
            let mut cursor = self.cursor::<T::Key>();
            let mut new_tree = SumTree::new();
            let mut buffered_items = Vec::new();

            cursor.seek(&T::Key::default(), Bias::Left, cx);
            for edit in edits {
                let new_key = edit.key();
                let mut old_item = cursor.item();

                if old_item
                    .as_ref()
                    .map_or(false, |old_item| old_item.key() < new_key)
                {
                    new_tree.extend(buffered_items.drain(..), cx);
                    let slice = cursor.slice(&new_key, Bias::Left, cx);
                    new_tree.append(slice, cx);
                    old_item = cursor.item();
                }

                if let Some(old_item) = old_item {
                    if old_item.key() == new_key {
                        removed.push(old_item.clone());
                        cursor.next(cx);
                    }
                }

                match edit {
                    Edit::Insert(item) => {
                        buffered_items.push(item);
                    }
                    Edit::Remove(_) => {}
                }
            }

            new_tree.extend(buffered_items, cx);
            new_tree.append(cursor.suffix(cx), cx);
            new_tree
        };

        removed
    }

    pub fn get(&self, key: &T::Key, cx: &<T::Summary as Summary>::Context) -> Option<&T> {
        let mut cursor = self.cursor::<T::Key>();
        if cursor.seek(key, Bias::Left, cx) {
            cursor.item()
        } else {
            None
        }
    }
}

impl<T: Item> Default for SumTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub enum Node<T: Item> {
    Internal {
        height: u8,
        summary: T::Summary,
        child_summaries: ArrayVec<T::Summary, { 2 * TREE_BASE }>,
        child_trees: ArrayVec<SumTree<T>, { 2 * TREE_BASE }>,
    },
    Leaf {
        summary: T::Summary,
        items: ArrayVec<T, { 2 * TREE_BASE }>,
        item_summaries: ArrayVec<T::Summary, { 2 * TREE_BASE }>,
    },
}

impl<T: Item> Node<T> {
    fn is_leaf(&self) -> bool {
        matches!(self, Node::Leaf { .. })
    }

    fn height(&self) -> u8 {
        match self {
            Node::Internal { height, .. } => *height,
            Node::Leaf { .. } => 0,
        }
    }

    fn summary(&self) -> &T::Summary {
        match self {
            Node::Internal { summary, .. } => summary,
            Node::Leaf { summary, .. } => summary,
        }
    }

    fn child_summaries(&self) -> &[T::Summary] {
        match self {
            Node::Internal {
                child_summaries, ..
            } => child_summaries.as_slice(),
            Node::Leaf { item_summaries, .. } => item_summaries.as_slice(),
        }
    }

    fn child_trees(&self) -> &ArrayVec<SumTree<T>, { 2 * TREE_BASE }> {
        match self {
            Node::Internal { child_trees, .. } => child_trees,
            Node::Leaf { .. } => panic!("Leaf nodes have no child trees"),
        }
    }

    fn items(&self) -> &ArrayVec<T, { 2 * TREE_BASE }> {
        match self {
            Node::Leaf { items, .. } => items,
            Node::Internal { .. } => panic!("Internal nodes have no items"),
        }
    }

    fn is_underflowing(&self) -> bool {
        match self {
            Node::Internal { child_trees, .. } => child_trees.len() < TREE_BASE,
            Node::Leaf { items, .. } => items.len() < TREE_BASE,
        }
    }
}

#[derive(Debug)]
pub enum Edit<T: KeyedItem> {
    Insert(T),
    Remove(T::Key),
}

impl<T: KeyedItem> Edit<T> {
    fn key(&self) -> T::Key {
        match self {
            Edit::Insert(item) => item.key(),
            Edit::Remove(key) => key.clone(),
        }
    }
}

fn sum<'a, T, I>(iter: I, cx: &T::Context) -> T
where
    T: 'a + Summary,
    I: Iterator<Item = &'a T>,
{
    let mut sum = T::default();
    for value in iter {
        sum.add_summary(value, cx);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions, prelude::*};
    use std::cmp;

    #[ctor::ctor]
    fn init_logger() {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::init();
        }
    }

    #[test]
    fn test_extend_and_push_tree() {
        let mut tree1 = SumTree::new();
        tree1.extend(0..20, &());

        let mut tree2 = SumTree::new();
        tree2.extend(50..100, &());

        tree1.append(tree2, &());
        assert_eq!(
            tree1.items(&()),
            (0..20).chain(50..100).collect::<Vec<u8>>()
        );
    }

    #[test]
    fn test_random() {
        let mut starting_seed = 0;
        if let Ok(value) = std::env::var("SEED") {
            starting_seed = value.parse().expect("invalid SEED variable");
        }
        let mut num_iterations = 100;
        if let Ok(value) = std::env::var("ITERATIONS") {
            num_iterations = value.parse().expect("invalid ITERATIONS variable");
        }
        let num_operations = std::env::var("OPERATIONS")
            .map_or(5, |o| o.parse().expect("invalid OPERATIONS variable"));

        for seed in starting_seed..(starting_seed + num_iterations) {
            eprintln!("seed = {}", seed);
            let mut rng = StdRng::seed_from_u64(seed);

            let rng = &mut rng;
            let mut tree = SumTree::<u8>::new();
            let count = rng.gen_range(0..10);
            if rng.gen() {
                tree.extend(rng.sample_iter(distributions::Standard).take(count), &());
            } else {
                let items = rng
                    .sample_iter(distributions::Standard)
                    .take(count)
                    .collect::<Vec<_>>();
                tree.par_extend(items, &());
            }

            for _ in 0..num_operations {
                let splice_end = rng.gen_range(0..tree.extent::<Count>(&()).0 + 1);
                let splice_start = rng.gen_range(0..splice_end + 1);
                let count = rng.gen_range(0..10);
                let tree_end = tree.extent::<Count>(&());
                let new_items = rng
                    .sample_iter(distributions::Standard)
                    .take(count)
                    .collect::<Vec<u8>>();

                let mut reference_items = tree.items(&());
                reference_items.splice(splice_start..splice_end, new_items.clone());

                tree = {
                    let mut cursor = tree.cursor::<Count>();
                    let mut new_tree = cursor.slice(&Count(splice_start), Bias::Right, &());
                    if rng.gen() {
                        new_tree.extend(new_items, &());
                    } else {
                        new_tree.par_extend(new_items, &());
                    }
                    cursor.seek(&Count(splice_end), Bias::Right, &());
                    new_tree.append(cursor.slice(&tree_end, Bias::Right, &()), &());
                    new_tree
                };

                assert_eq!(tree.items(&()), reference_items);
                assert_eq!(
                    tree.iter().collect::<Vec<_>>(),
                    tree.cursor::<()>().collect::<Vec<_>>()
                );

                log::info!("tree items: {:?}", tree.items(&()));

                let mut filter_cursor = tree.filter::<_, Count>(|summary| summary.contains_even);
                let expected_filtered_items = tree
                    .items(&())
                    .into_iter()
                    .enumerate()
                    .filter(|(_, item)| (item & 1) == 0)
                    .collect::<Vec<_>>();

                let mut item_ix = if rng.gen() {
                    filter_cursor.next(&());
                    0
                } else {
                    filter_cursor.prev(&());
                    expected_filtered_items.len().saturating_sub(1)
                };
                while item_ix < expected_filtered_items.len() {
                    log::info!("filter_cursor, item_ix: {}", item_ix);
                    let actual_item = filter_cursor.item().unwrap();
                    let (reference_index, reference_item) = expected_filtered_items[item_ix];
                    assert_eq!(actual_item, &reference_item);
                    assert_eq!(filter_cursor.start().0, reference_index);
                    log::info!("next");
                    filter_cursor.next(&());
                    item_ix += 1;

                    while item_ix > 0 && rng.gen_bool(0.2) {
                        log::info!("prev");
                        filter_cursor.prev(&());
                        item_ix -= 1;

                        if item_ix == 0 && rng.gen_bool(0.2) {
                            filter_cursor.prev(&());
                            assert_eq!(filter_cursor.item(), None);
                            assert_eq!(filter_cursor.start().0, 0);
                            filter_cursor.next(&());
                        }
                    }
                }
                assert_eq!(filter_cursor.item(), None);

                let mut before_start = false;
                let mut cursor = tree.cursor::<Count>();
                let start_pos = rng.gen_range(0..=reference_items.len());
                cursor.seek(&Count(start_pos), Bias::Right, &());
                let mut pos = rng.gen_range(start_pos..=reference_items.len());
                cursor.seek_forward(&Count(pos), Bias::Right, &());

                for i in 0..10 {
                    assert_eq!(cursor.start().0, pos);

                    if pos > 0 {
                        assert_eq!(cursor.prev_item().unwrap(), &reference_items[pos - 1]);
                    } else {
                        assert_eq!(cursor.prev_item(), None);
                    }

                    if pos < reference_items.len() && !before_start {
                        assert_eq!(cursor.item().unwrap(), &reference_items[pos]);
                    } else {
                        assert_eq!(cursor.item(), None);
                    }

                    if before_start {
                        assert_eq!(cursor.next_item(), reference_items.get(0));
                    } else if pos + 1 < reference_items.len() {
                        assert_eq!(cursor.next_item().unwrap(), &reference_items[pos + 1]);
                    } else {
                        assert_eq!(cursor.next_item(), None);
                    }

                    if i < 5 {
                        cursor.next(&());
                        if pos < reference_items.len() {
                            pos += 1;
                            before_start = false;
                        }
                    } else {
                        cursor.prev(&());
                        if pos == 0 {
                            before_start = true;
                        }
                        pos = pos.saturating_sub(1);
                    }
                }
            }

            for _ in 0..10 {
                let end = rng.gen_range(0..tree.extent::<Count>(&()).0 + 1);
                let start = rng.gen_range(0..end + 1);
                let start_bias = if rng.gen() { Bias::Left } else { Bias::Right };
                let end_bias = if rng.gen() { Bias::Left } else { Bias::Right };

                let mut cursor = tree.cursor::<Count>();
                cursor.seek(&Count(start), start_bias, &());
                let slice = cursor.slice(&Count(end), end_bias, &());

                cursor.seek(&Count(start), start_bias, &());
                let summary = cursor.summary::<_, Sum>(&Count(end), end_bias, &());

                assert_eq!(summary.0, slice.summary().sum);
            }
        }
    }

    #[test]
    fn test_cursor() {
        // Empty tree
        let tree = SumTree::<u8>::new();
        let mut cursor = tree.cursor::<IntegersSummary>();
        assert_eq!(
            cursor.slice(&Count(0), Bias::Right, &()).items(&()),
            Vec::<u8>::new()
        );
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 0);
        cursor.prev(&());
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 0);
        cursor.next(&());
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 0);

        // Single-element tree
        let mut tree = SumTree::<u8>::new();
        tree.extend(vec![1], &());
        let mut cursor = tree.cursor::<IntegersSummary>();
        assert_eq!(
            cursor.slice(&Count(0), Bias::Right, &()).items(&()),
            Vec::<u8>::new()
        );
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 0);

        cursor.next(&());
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 1);

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 0);

        let mut cursor = tree.cursor::<IntegersSummary>();
        assert_eq!(cursor.slice(&Count(1), Bias::Right, &()).items(&()), [1]);
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 1);

        cursor.seek(&Count(0), Bias::Right, &());
        assert_eq!(
            cursor
                .slice(&tree.extent::<Count>(&()), Bias::Right, &())
                .items(&()),
            [1]
        );
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 1);

        // Multiple-element tree
        let mut tree = SumTree::new();
        tree.extend(vec![1, 2, 3, 4, 5, 6], &());
        let mut cursor = tree.cursor::<IntegersSummary>();

        assert_eq!(cursor.slice(&Count(2), Bias::Right, &()).items(&()), [1, 2]);
        assert_eq!(cursor.item(), Some(&3));
        assert_eq!(cursor.prev_item(), Some(&2));
        assert_eq!(cursor.next_item(), Some(&4));
        assert_eq!(cursor.start().sum, 3);

        cursor.next(&());
        assert_eq!(cursor.item(), Some(&4));
        assert_eq!(cursor.prev_item(), Some(&3));
        assert_eq!(cursor.next_item(), Some(&5));
        assert_eq!(cursor.start().sum, 6);

        cursor.next(&());
        assert_eq!(cursor.item(), Some(&5));
        assert_eq!(cursor.prev_item(), Some(&4));
        assert_eq!(cursor.next_item(), Some(&6));
        assert_eq!(cursor.start().sum, 10);

        cursor.next(&());
        assert_eq!(cursor.item(), Some(&6));
        assert_eq!(cursor.prev_item(), Some(&5));
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 15);

        cursor.next(&());
        cursor.next(&());
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&6));
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 21);

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&6));
        assert_eq!(cursor.prev_item(), Some(&5));
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 15);

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&5));
        assert_eq!(cursor.prev_item(), Some(&4));
        assert_eq!(cursor.next_item(), Some(&6));
        assert_eq!(cursor.start().sum, 10);

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&4));
        assert_eq!(cursor.prev_item(), Some(&3));
        assert_eq!(cursor.next_item(), Some(&5));
        assert_eq!(cursor.start().sum, 6);

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&3));
        assert_eq!(cursor.prev_item(), Some(&2));
        assert_eq!(cursor.next_item(), Some(&4));
        assert_eq!(cursor.start().sum, 3);

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&2));
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.next_item(), Some(&3));
        assert_eq!(cursor.start().sum, 1);

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.next_item(), Some(&2));
        assert_eq!(cursor.start().sum, 0);

        cursor.prev(&());
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.next_item(), Some(&1));
        assert_eq!(cursor.start().sum, 0);

        cursor.next(&());
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.next_item(), Some(&2));
        assert_eq!(cursor.start().sum, 0);

        let mut cursor = tree.cursor::<IntegersSummary>();
        assert_eq!(
            cursor
                .slice(&tree.extent::<Count>(&()), Bias::Right, &())
                .items(&()),
            tree.items(&())
        );
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&6));
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 21);

        cursor.seek(&Count(3), Bias::Right, &());
        assert_eq!(
            cursor
                .slice(&tree.extent::<Count>(&()), Bias::Right, &())
                .items(&()),
            [4, 5, 6]
        );
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&6));
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 21);

        // Seeking can bias left or right
        cursor.seek(&Count(1), Bias::Left, &());
        assert_eq!(cursor.item(), Some(&1));
        cursor.seek(&Count(1), Bias::Right, &());
        assert_eq!(cursor.item(), Some(&2));

        // Slicing without resetting starts from where the cursor is parked at.
        cursor.seek(&Count(1), Bias::Right, &());
        assert_eq!(
            cursor.slice(&Count(3), Bias::Right, &()).items(&()),
            vec![2, 3]
        );
        assert_eq!(
            cursor.slice(&Count(6), Bias::Left, &()).items(&()),
            vec![4, 5]
        );
        assert_eq!(
            cursor.slice(&Count(6), Bias::Right, &()).items(&()),
            vec![6]
        );
    }

    #[test]
    fn test_edit() {
        let mut tree = SumTree::<u8>::new();

        let removed = tree.edit(vec![Edit::Insert(1), Edit::Insert(2), Edit::Insert(0)], &());
        assert_eq!(tree.items(&()), vec![0, 1, 2]);
        assert_eq!(removed, Vec::<u8>::new());
        assert_eq!(tree.get(&0, &()), Some(&0));
        assert_eq!(tree.get(&1, &()), Some(&1));
        assert_eq!(tree.get(&2, &()), Some(&2));
        assert_eq!(tree.get(&4, &()), None);

        let removed = tree.edit(vec![Edit::Insert(2), Edit::Insert(4), Edit::Remove(0)], &());
        assert_eq!(tree.items(&()), vec![1, 2, 4]);
        assert_eq!(removed, vec![0, 2]);
        assert_eq!(tree.get(&0, &()), None);
        assert_eq!(tree.get(&1, &()), Some(&1));
        assert_eq!(tree.get(&2, &()), Some(&2));
        assert_eq!(tree.get(&4, &()), Some(&4));
    }

    #[derive(Clone, Default, Debug)]
    pub struct IntegersSummary {
        count: usize,
        sum: usize,
        contains_even: bool,
        max: u8,
    }

    #[derive(Ord, PartialOrd, Default, Eq, PartialEq, Clone, Debug)]
    struct Count(usize);

    #[derive(Ord, PartialOrd, Default, Eq, PartialEq, Clone, Debug)]
    struct Sum(usize);

    impl Item for u8 {
        type Summary = IntegersSummary;

        fn summary(&self) -> Self::Summary {
            IntegersSummary {
                count: 1,
                sum: *self as usize,
                contains_even: (*self & 1) == 0,
                max: *self,
            }
        }
    }

    impl KeyedItem for u8 {
        type Key = u8;

        fn key(&self) -> Self::Key {
            *self
        }
    }

    impl Summary for IntegersSummary {
        type Context = ();

        fn add_summary(&mut self, other: &Self, _: &()) {
            self.count += other.count;
            self.sum += other.sum;
            self.contains_even |= other.contains_even;
            self.max = cmp::max(self.max, other.max);
        }
    }

    impl<'a> Dimension<'a, IntegersSummary> for u8 {
        fn add_summary(&mut self, summary: &IntegersSummary, _: &()) {
            *self = summary.max;
        }
    }

    impl<'a> Dimension<'a, IntegersSummary> for Count {
        fn add_summary(&mut self, summary: &IntegersSummary, _: &()) {
            self.0 += summary.count;
        }
    }

    impl<'a> SeekTarget<'a, IntegersSummary, IntegersSummary> for Count {
        fn cmp(&self, cursor_location: &IntegersSummary, _: &()) -> Ordering {
            self.0.cmp(&cursor_location.count)
        }
    }

    impl<'a> Dimension<'a, IntegersSummary> for Sum {
        fn add_summary(&mut self, summary: &IntegersSummary, _: &()) {
            self.0 += summary.sum;
        }
    }
}
