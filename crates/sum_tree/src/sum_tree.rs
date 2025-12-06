mod cursor;
mod tree_map;

use arrayvec::ArrayVec;
pub use cursor::{Cursor, FilterCursor, Iter};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator as _};
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

    fn summary(&self, cx: <Self::Summary as Summary>::Context<'_>) -> Self::Summary;
}

/// An [`Item`] whose summary has a specific key that can be used to identify it
pub trait KeyedItem: Item {
    type Key: for<'a> Dimension<'a, Self::Summary> + Ord;

    fn key(&self) -> Self::Key;
}

/// A type that describes the Sum of all [`Item`]s in a subtree of the [`SumTree`]
///
/// Each Summary type can have multiple [`Dimension`]s that it measures,
/// which can be used to navigate the tree
pub trait Summary: Clone {
    type Context<'a>: Copy;
    fn zero<'a>(cx: Self::Context<'a>) -> Self;
    fn add_summary<'a>(&mut self, summary: &Self, cx: Self::Context<'a>);
}

pub trait ContextLessSummary: Clone {
    fn zero() -> Self;
    fn add_summary(&mut self, summary: &Self);
}

impl<T: ContextLessSummary> Summary for T {
    type Context<'a> = ();

    fn zero<'a>((): ()) -> Self {
        T::zero()
    }

    fn add_summary<'a>(&mut self, summary: &Self, (): ()) {
        T::add_summary(self, summary)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoSummary;

/// Catch-all implementation for when you need something that implements [`Summary`] without a specific type.
/// We implement it on a `NoSummary` instead of re-using `()`, as that avoids blanket impl collisions with `impl<T: Summary> Dimension for T`
/// (as we also need unit type to be a fill-in dimension)
impl ContextLessSummary for NoSummary {
    fn zero() -> Self {
        NoSummary
    }

    fn add_summary(&mut self, _: &Self) {}
}

/// Each [`Summary`] type can have more than one [`Dimension`] type that it measures.
///
/// You can use dimensions to seek to a specific location in the [`SumTree`]
///
/// # Example:
/// Zed's rope has a `TextSummary` type that summarizes lines, characters, and bytes.
/// Each of these are different dimensions we may want to seek to
pub trait Dimension<'a, S: Summary>: Clone {
    fn zero(cx: S::Context<'_>) -> Self;

    fn add_summary(&mut self, summary: &'a S, cx: S::Context<'_>);
    #[must_use]
    fn with_added_summary(mut self, summary: &'a S, cx: S::Context<'_>) -> Self {
        self.add_summary(summary, cx);
        self
    }

    fn from_summary(summary: &'a S, cx: S::Context<'_>) -> Self {
        let mut dimension = Self::zero(cx);
        dimension.add_summary(summary, cx);
        dimension
    }
}

impl<'a, T: Summary> Dimension<'a, T> for T {
    fn zero(cx: T::Context<'_>) -> Self {
        Summary::zero(cx)
    }

    fn add_summary(&mut self, summary: &'a T, cx: T::Context<'_>) {
        Summary::add_summary(self, summary, cx);
    }
}

pub trait SeekTarget<'a, S: Summary, D: Dimension<'a, S>> {
    fn cmp(&self, cursor_location: &D, cx: S::Context<'_>) -> Ordering;
}

impl<'a, S: Summary, D: Dimension<'a, S> + Ord> SeekTarget<'a, S, D> for D {
    fn cmp(&self, cursor_location: &Self, _: S::Context<'_>) -> Ordering {
        Ord::cmp(self, cursor_location)
    }
}

impl<'a, T: Summary> Dimension<'a, T> for () {
    fn zero(_: T::Context<'_>) -> Self {}

    fn add_summary(&mut self, _: &'a T, _: T::Context<'_>) {}
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dimensions<D1, D2, D3 = ()>(pub D1, pub D2, pub D3);

impl<'a, T: Summary, D1: Dimension<'a, T>, D2: Dimension<'a, T>, D3: Dimension<'a, T>>
    Dimension<'a, T> for Dimensions<D1, D2, D3>
{
    fn zero(cx: T::Context<'_>) -> Self {
        Dimensions(D1::zero(cx), D2::zero(cx), D3::zero(cx))
    }

    fn add_summary(&mut self, summary: &'a T, cx: T::Context<'_>) {
        self.0.add_summary(summary, cx);
        self.1.add_summary(summary, cx);
        self.2.add_summary(summary, cx);
    }
}

impl<'a, S, D1, D2, D3> SeekTarget<'a, S, Dimensions<D1, D2, D3>> for D1
where
    S: Summary,
    D1: SeekTarget<'a, S, D1> + Dimension<'a, S>,
    D2: Dimension<'a, S>,
    D3: Dimension<'a, S>,
{
    fn cmp(&self, cursor_location: &Dimensions<D1, D2, D3>, cx: S::Context<'_>) -> Ordering {
        self.cmp(&cursor_location.0, cx)
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

/// A B+ tree in which each leaf node contains `Item`s of type `T` and a `Summary`s for each `Item`.
/// Each internal node contains a `Summary` of the items in its subtree.
///
/// The maximum number of items per node is `TREE_BASE * 2`.
///
/// Any [`Dimension`] supported by the [`Summary`] type can be used to seek to a specific location in the tree.
#[derive(Clone)]
pub struct SumTree<T: Item>(Arc<Node<T>>);

impl<T> fmt::Debug for SumTree<T>
where
    T: fmt::Debug + Item,
    T::Summary: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("SumTree").field(&self.0).finish()
    }
}

impl<T: Item> SumTree<T> {
    pub fn new(cx: <T::Summary as Summary>::Context<'_>) -> Self {
        SumTree(Arc::new(Node::Leaf {
            summary: <T::Summary as Summary>::zero(cx),
            items: ArrayVec::new(),
            item_summaries: ArrayVec::new(),
        }))
    }

    /// Useful in cases where the item type has a non-trivial context type, but the zero value of the summary type doesn't depend on that context.
    pub fn from_summary(summary: T::Summary) -> Self {
        SumTree(Arc::new(Node::Leaf {
            summary,
            items: ArrayVec::new(),
            item_summaries: ArrayVec::new(),
        }))
    }

    pub fn from_item(item: T, cx: <T::Summary as Summary>::Context<'_>) -> Self {
        let mut tree = Self::new(cx);
        tree.push(item, cx);
        tree
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(
        iter: I,
        cx: <T::Summary as Summary>::Context<'_>,
    ) -> Self {
        let mut nodes = Vec::new();

        let mut iter = iter.into_iter().fuse().peekable();
        while iter.peek().is_some() {
            let items: ArrayVec<T, { 2 * TREE_BASE }> = iter.by_ref().take(2 * TREE_BASE).collect();
            let item_summaries: ArrayVec<T::Summary, { 2 * TREE_BASE }> =
                items.iter().map(|item| item.summary(cx)).collect();

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
                    summary: <T::Summary as Summary>::zero(cx),
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
            Self::new(cx)
        } else {
            debug_assert_eq!(nodes.len(), 1);
            Self(Arc::new(nodes.pop().unwrap()))
        }
    }

    pub fn from_par_iter<I, Iter>(iter: I, cx: <T::Summary as Summary>::Context<'_>) -> Self
    where
        I: IntoParallelIterator<Iter = Iter>,
        Iter: IndexedParallelIterator<Item = T>,
        T: Send + Sync,
        T::Summary: Send + Sync,
        for<'a> <T::Summary as Summary>::Context<'a>: Sync,
    {
        let mut nodes = iter
            .into_par_iter()
            .chunks(2 * TREE_BASE)
            .map(|items| {
                let items: ArrayVec<T, { 2 * TREE_BASE }> = items.into_iter().collect();
                let item_summaries: ArrayVec<T::Summary, { 2 * TREE_BASE }> =
                    items.iter().map(|item| item.summary(cx)).collect();
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
            Self::new(cx)
        } else {
            debug_assert_eq!(nodes.len(), 1);
            nodes.pop().unwrap()
        }
    }

    #[allow(unused)]
    pub fn items<'a>(&'a self, cx: <T::Summary as Summary>::Context<'a>) -> Vec<T> {
        let mut items = Vec::new();
        let mut cursor = self.cursor::<()>(cx);
        cursor.next();
        while let Some(item) = cursor.item() {
            items.push(item.clone());
            cursor.next();
        }
        items
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self)
    }

    /// A more efficient version of `Cursor::new()` + `Cursor::seek()` + `Cursor::item()`.
    ///
    /// Only returns the item that exactly has the target match.
    pub fn find_exact<'a, 'slf, D, Target>(
        &'slf self,
        cx: <T::Summary as Summary>::Context<'a>,
        target: &Target,
        bias: Bias,
    ) -> (D, D, Option<&'slf T>)
    where
        D: Dimension<'slf, T::Summary>,
        Target: SeekTarget<'slf, T::Summary, D>,
    {
        let tree_end = D::zero(cx).with_added_summary(self.summary(), cx);
        let comparison = target.cmp(&tree_end, cx);
        if comparison == Ordering::Greater || (comparison == Ordering::Equal && bias == Bias::Right)
        {
            return (tree_end.clone(), tree_end, None);
        }

        let mut pos = D::zero(cx);
        return match Self::find_recurse::<_, _, true>(cx, target, bias, &mut pos, self) {
            Some((item, end)) => (pos, end, Some(item)),
            None => (pos.clone(), pos, None),
        };
    }

    /// A more efficient version of `Cursor::new()` + `Cursor::seek()` + `Cursor::item()`
    pub fn find<'a, 'slf, D, Target>(
        &'slf self,
        cx: <T::Summary as Summary>::Context<'a>,
        target: &Target,
        bias: Bias,
    ) -> (D, D, Option<&'slf T>)
    where
        D: Dimension<'slf, T::Summary>,
        Target: SeekTarget<'slf, T::Summary, D>,
    {
        let tree_end = D::zero(cx).with_added_summary(self.summary(), cx);
        let comparison = target.cmp(&tree_end, cx);
        if comparison == Ordering::Greater || (comparison == Ordering::Equal && bias == Bias::Right)
        {
            return (tree_end.clone(), tree_end, None);
        }

        let mut pos = D::zero(cx);
        return match Self::find_recurse::<_, _, false>(cx, target, bias, &mut pos, self) {
            Some((item, end)) => (pos, end, Some(item)),
            None => (pos.clone(), pos, None),
        };
    }

    fn find_recurse<'tree, 'a, D, Target, const EXACT: bool>(
        cx: <T::Summary as Summary>::Context<'a>,
        target: &Target,
        bias: Bias,
        position: &mut D,
        this: &'tree SumTree<T>,
    ) -> Option<(&'tree T, D)>
    where
        D: Dimension<'tree, T::Summary>,
        Target: SeekTarget<'tree, T::Summary, D>,
    {
        match &*this.0 {
            Node::Internal {
                child_summaries,
                child_trees,
                ..
            } => {
                for (child_tree, child_summary) in child_trees.iter().zip(child_summaries) {
                    let child_end = position.clone().with_added_summary(child_summary, cx);

                    let comparison = target.cmp(&child_end, cx);
                    let target_in_child = comparison == Ordering::Less
                        || (comparison == Ordering::Equal && bias == Bias::Left);
                    if target_in_child {
                        return Self::find_recurse::<D, Target, EXACT>(
                            cx, target, bias, position, child_tree,
                        );
                    }
                    *position = child_end;
                }
            }
            Node::Leaf {
                items,
                item_summaries,
                ..
            } => {
                for (item, item_summary) in items.iter().zip(item_summaries) {
                    let mut child_end = position.clone();
                    child_end.add_summary(item_summary, cx);

                    let comparison = target.cmp(&child_end, cx);
                    let entry_found = if EXACT {
                        comparison == Ordering::Equal
                    } else {
                        comparison == Ordering::Less
                            || (comparison == Ordering::Equal && bias == Bias::Left)
                    };
                    if entry_found {
                        return Some((item, child_end));
                    }

                    *position = child_end;
                }
            }
        }
        None
    }

    pub fn cursor<'a, 'b, D>(
        &'a self,
        cx: <T::Summary as Summary>::Context<'b>,
    ) -> Cursor<'a, 'b, T, D>
    where
        D: Dimension<'a, T::Summary>,
    {
        Cursor::new(self, cx)
    }

    /// Note: If the summary type requires a non `()` context, then the filter cursor
    /// that is returned cannot be used with Rust's iterators.
    pub fn filter<'a, 'b, F, U>(
        &'a self,
        cx: <T::Summary as Summary>::Context<'b>,
        filter_node: F,
    ) -> FilterCursor<'a, 'b, F, T, U>
    where
        F: FnMut(&T::Summary) -> bool,
        U: Dimension<'a, T::Summary>,
    {
        FilterCursor::new(self, cx, filter_node)
    }

    #[allow(dead_code)]
    pub fn first(&self) -> Option<&T> {
        self.leftmost_leaf().0.items().first()
    }

    pub fn last(&self) -> Option<&T> {
        self.rightmost_leaf().0.items().last()
    }

    pub fn update_last(
        &mut self,
        f: impl FnOnce(&mut T),
        cx: <T::Summary as Summary>::Context<'_>,
    ) {
        self.update_last_recursive(f, cx);
    }

    fn update_last_recursive(
        &mut self,
        f: impl FnOnce(&mut T),
        cx: <T::Summary as Summary>::Context<'_>,
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
                    *item_summary = item.summary(cx);
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
        cx: <T::Summary as Summary>::Context<'_>,
    ) -> D {
        let mut extent = D::zero(cx);
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

    pub fn extend<I>(&mut self, iter: I, cx: <T::Summary as Summary>::Context<'_>)
    where
        I: IntoIterator<Item = T>,
    {
        self.append(Self::from_iter(iter, cx), cx);
    }

    pub fn par_extend<I, Iter>(&mut self, iter: I, cx: <T::Summary as Summary>::Context<'_>)
    where
        I: IntoParallelIterator<Iter = Iter>,
        Iter: IndexedParallelIterator<Item = T>,
        T: Send + Sync,
        T::Summary: Send + Sync,
        for<'a> <T::Summary as Summary>::Context<'a>: Sync,
    {
        self.append(Self::from_par_iter(iter, cx), cx);
    }

    pub fn push(&mut self, item: T, cx: <T::Summary as Summary>::Context<'_>) {
        let summary = item.summary(cx);
        self.append(
            SumTree(Arc::new(Node::Leaf {
                summary: summary.clone(),
                items: ArrayVec::from_iter(Some(item)),
                item_summaries: ArrayVec::from_iter(Some(summary)),
            })),
            cx,
        );
    }

    pub fn append(&mut self, mut other: Self, cx: <T::Summary as Summary>::Context<'_>) {
        if self.is_empty() {
            *self = other;
        } else if !other.0.is_leaf() || !other.0.items().is_empty() {
            if self.0.height() < other.0.height() {
                if let Some(tree) = Self::append_large(self.clone(), &mut other, cx) {
                    *self = Self::from_child_trees(tree, other, cx);
                } else {
                    *self = other;
                }
            } else if let Some(split_tree) = self.push_tree_recursive(other, cx) {
                *self = Self::from_child_trees(self.clone(), split_tree, cx);
            }
        }
    }

    fn push_tree_recursive(
        &mut self,
        other: SumTree<T>,
        cx: <T::Summary as Summary>::Context<'_>,
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

    // appends the `large` tree to a `small` tree, assumes small.height() <= large.height()
    fn append_large(
        small: Self,
        large: &mut Self,
        cx: <T::Summary as Summary>::Context<'_>,
    ) -> Option<Self> {
        if small.0.height() == large.0.height() {
            if !small.0.is_underflowing() {
                Some(small)
            } else {
                Self::merge_into_right(small, large, cx)
            }
        } else {
            debug_assert!(small.0.height() < large.0.height());
            let Node::Internal {
                height,
                summary,
                child_summaries,
                child_trees,
            } = Arc::make_mut(&mut large.0)
            else {
                unreachable!();
            };
            let mut full_summary = small.summary().clone();
            Summary::add_summary(&mut full_summary, summary, cx);
            *summary = full_summary;

            let first = child_trees.first_mut().unwrap();
            let res = Self::append_large(small, first, cx);
            *child_summaries.first_mut().unwrap() = first.summary().clone();
            if let Some(tree) = res {
                if child_trees.len() < 2 * TREE_BASE {
                    child_summaries.insert(0, tree.summary().clone());
                    child_trees.insert(0, tree);
                    None
                } else {
                    let new_child_summaries = {
                        let mut res = ArrayVec::from_iter([tree.summary().clone()]);
                        res.extend(child_summaries.drain(..TREE_BASE));
                        res
                    };
                    let tree = SumTree(Arc::new(Node::Internal {
                        height: *height,
                        summary: sum(new_child_summaries.iter(), cx),
                        child_summaries: new_child_summaries,
                        child_trees: {
                            let mut res = ArrayVec::from_iter([tree]);
                            res.extend(child_trees.drain(..TREE_BASE));
                            res
                        },
                    }));

                    *summary = sum(child_summaries.iter(), cx);
                    Some(tree)
                }
            } else {
                None
            }
        }
    }

    // Merge two nodes into `large`.
    //
    // `large` will contain the contents of `small` followed by its own data.
    // If the combined data exceed the node capacity, returns a new node that
    // holds the first half of the merged items and `large` is left with the
    // second half
    //
    // The nodes must be on the same height
    // It only makes sense to call this when `small` is underflowing
    fn merge_into_right(
        small: Self,
        large: &mut Self,
        cx: <<T as Item>::Summary as Summary>::Context<'_>,
    ) -> Option<SumTree<T>> {
        debug_assert_eq!(small.0.height(), large.0.height());
        match (small.0.as_ref(), Arc::make_mut(&mut large.0)) {
            (
                Node::Internal {
                    summary: small_summary,
                    child_summaries: small_child_summaries,
                    child_trees: small_child_trees,
                    ..
                },
                Node::Internal {
                    summary,
                    child_summaries,
                    child_trees,
                    height,
                },
            ) => {
                let total_child_count = child_trees.len() + small_child_trees.len();
                if total_child_count <= 2 * TREE_BASE {
                    let mut all_trees = small_child_trees.clone();
                    all_trees.extend(child_trees.drain(..));
                    *child_trees = all_trees;

                    let mut all_summaries = small_child_summaries.clone();
                    all_summaries.extend(child_summaries.drain(..));
                    *child_summaries = all_summaries;

                    let mut full_summary = small_summary.clone();
                    Summary::add_summary(&mut full_summary, summary, cx);
                    *summary = full_summary;
                    None
                } else {
                    let midpoint = total_child_count.div_ceil(2);
                    let mut all_trees = small_child_trees.iter().chain(child_trees.iter()).cloned();
                    let left_trees = all_trees.by_ref().take(midpoint).collect();
                    *child_trees = all_trees.collect();

                    let mut all_summaries = small_child_summaries
                        .iter()
                        .chain(child_summaries.iter())
                        .cloned();
                    let left_summaries: ArrayVec<_, { 2 * TREE_BASE }> =
                        all_summaries.by_ref().take(midpoint).collect();
                    *child_summaries = all_summaries.collect();

                    *summary = sum(child_summaries.iter(), cx);
                    Some(SumTree(Arc::new(Node::Internal {
                        height: *height,
                        summary: sum(left_summaries.iter(), cx),
                        child_summaries: left_summaries,
                        child_trees: left_trees,
                    })))
                }
            }
            (
                Node::Leaf {
                    summary: small_summary,
                    items: small_items,
                    item_summaries: small_item_summaries,
                },
                Node::Leaf {
                    summary,
                    items,
                    item_summaries,
                },
            ) => {
                let total_child_count = small_items.len() + items.len();
                if total_child_count <= 2 * TREE_BASE {
                    let mut all_items = small_items.clone();
                    all_items.extend(items.drain(..));
                    *items = all_items;

                    let mut all_summaries = small_item_summaries.clone();
                    all_summaries.extend(item_summaries.drain(..));
                    *item_summaries = all_summaries;

                    let mut full_summary = small_summary.clone();
                    Summary::add_summary(&mut full_summary, summary, cx);
                    *summary = full_summary;
                    None
                } else {
                    let midpoint = total_child_count.div_ceil(2);
                    let mut all_items = small_items.iter().chain(items.iter()).cloned();
                    let left_items = all_items.by_ref().take(midpoint).collect();
                    *items = all_items.collect();

                    let mut all_summaries = small_item_summaries
                        .iter()
                        .chain(item_summaries.iter())
                        .cloned();
                    let left_summaries: ArrayVec<_, { 2 * TREE_BASE }> =
                        all_summaries.by_ref().take(midpoint).collect();
                    *item_summaries = all_summaries.collect();

                    *summary = sum(item_summaries.iter(), cx);
                    Some(SumTree(Arc::new(Node::Leaf {
                        items: left_items,
                        summary: sum(left_summaries.iter(), cx),
                        item_summaries: left_summaries,
                    })))
                }
            }
            _ => unreachable!(),
        }
    }

    fn from_child_trees(
        left: SumTree<T>,
        right: SumTree<T>,
        cx: <T::Summary as Summary>::Context<'_>,
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
}

impl<T: Item + PartialEq> PartialEq for SumTree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T: Item + Eq> Eq for SumTree<T> {}

impl<T: KeyedItem> SumTree<T> {
    pub fn insert_or_replace<'a, 'b>(
        &'a mut self,
        item: T,
        cx: <T::Summary as Summary>::Context<'b>,
    ) -> Option<T> {
        let mut replaced = None;
        {
            let mut cursor = self.cursor::<T::Key>(cx);
            let mut new_tree = cursor.slice(&item.key(), Bias::Left);
            if let Some(cursor_item) = cursor.item()
                && cursor_item.key() == item.key()
            {
                replaced = Some(cursor_item.clone());
                cursor.next();
            }
            new_tree.push(item, cx);
            new_tree.append(cursor.suffix(), cx);
            drop(cursor);
            *self = new_tree
        };
        replaced
    }

    pub fn remove(&mut self, key: &T::Key, cx: <T::Summary as Summary>::Context<'_>) -> Option<T> {
        let mut removed = None;
        *self = {
            let mut cursor = self.cursor::<T::Key>(cx);
            let mut new_tree = cursor.slice(key, Bias::Left);
            if let Some(item) = cursor.item()
                && item.key() == *key
            {
                removed = Some(item.clone());
                cursor.next();
            }
            new_tree.append(cursor.suffix(), cx);
            new_tree
        };
        removed
    }

    pub fn edit(
        &mut self,
        mut edits: Vec<Edit<T>>,
        cx: <T::Summary as Summary>::Context<'_>,
    ) -> Vec<T> {
        if edits.is_empty() {
            return Vec::new();
        }

        let mut removed = Vec::new();
        edits.sort_unstable_by_key(|item| item.key());

        *self = {
            let mut cursor = self.cursor::<T::Key>(cx);
            let mut new_tree = SumTree::new(cx);
            let mut buffered_items = Vec::new();

            cursor.seek(&T::Key::zero(cx), Bias::Left);
            for edit in edits {
                let new_key = edit.key();
                let mut old_item = cursor.item();

                if old_item
                    .as_ref()
                    .is_some_and(|old_item| old_item.key() < new_key)
                {
                    new_tree.extend(buffered_items.drain(..), cx);
                    let slice = cursor.slice(&new_key, Bias::Left);
                    new_tree.append(slice, cx);
                    old_item = cursor.item();
                }

                if let Some(old_item) = old_item
                    && old_item.key() == new_key
                {
                    removed.push(old_item.clone());
                    cursor.next();
                }

                match edit {
                    Edit::Insert(item) => {
                        buffered_items.push(item);
                    }
                    Edit::Remove(_) => {}
                }
            }

            new_tree.extend(buffered_items, cx);
            new_tree.append(cursor.suffix(), cx);
            new_tree
        };

        removed
    }

    pub fn get<'a>(
        &'a self,
        key: &T::Key,
        cx: <T::Summary as Summary>::Context<'a>,
    ) -> Option<&'a T> {
        if let (_, _, Some(item)) = self.find_exact::<T::Key, _>(cx, key, Bias::Left) {
            Some(item)
        } else {
            None
        }
    }
}

impl<T, S> Default for SumTree<T>
where
    T: Item<Summary = S>,
    S: for<'a> Summary<Context<'a> = ()>,
{
    fn default() -> Self {
        Self::new(())
    }
}

#[derive(Clone)]
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

impl<T> fmt::Debug for Node<T>
where
    T: Item + fmt::Debug,
    T::Summary: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Internal {
                height,
                summary,
                child_summaries,
                child_trees,
            } => f
                .debug_struct("Internal")
                .field("height", height)
                .field("summary", summary)
                .field("child_summaries", child_summaries)
                .field("child_trees", child_trees)
                .finish(),
            Node::Leaf {
                summary,
                items,
                item_summaries,
            } => f
                .debug_struct("Leaf")
                .field("summary", summary)
                .field("items", items)
                .field("item_summaries", item_summaries)
                .finish(),
        }
    }
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

fn sum<'a, T, I>(iter: I, cx: T::Context<'_>) -> T
where
    T: 'a + Summary,
    I: Iterator<Item = &'a T>,
{
    let mut sum = T::zero(cx);
    for value in iter {
        sum.add_summary(value, cx);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distr::StandardUniform, prelude::*};
    use std::cmp;

    #[ctor::ctor]
    fn init_logger() {
        zlog::init_test();
    }

    #[test]
    fn test_extend_and_push_tree() {
        let mut tree1 = SumTree::default();
        tree1.extend(0..20, ());

        let mut tree2 = SumTree::default();
        tree2.extend(50..100, ());

        tree1.append(tree2, ());
        assert_eq!(tree1.items(()), (0..20).chain(50..100).collect::<Vec<u8>>());
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
            let mut tree = SumTree::<u8>::default();
            let count = rng.random_range(0..10);
            if rng.random() {
                tree.extend(rng.sample_iter(StandardUniform).take(count), ());
            } else {
                let items = rng
                    .sample_iter(StandardUniform)
                    .take(count)
                    .collect::<Vec<_>>();
                tree.par_extend(items, ());
            }

            for _ in 0..num_operations {
                let splice_end = rng.random_range(0..tree.extent::<Count>(()).0 + 1);
                let splice_start = rng.random_range(0..splice_end + 1);
                let count = rng.random_range(0..10);
                let tree_end = tree.extent::<Count>(());
                let new_items = rng
                    .sample_iter(StandardUniform)
                    .take(count)
                    .collect::<Vec<u8>>();

                let mut reference_items = tree.items(());
                reference_items.splice(splice_start..splice_end, new_items.clone());

                tree = {
                    let mut cursor = tree.cursor::<Count>(());
                    let mut new_tree = cursor.slice(&Count(splice_start), Bias::Right);
                    if rng.random() {
                        new_tree.extend(new_items, ());
                    } else {
                        new_tree.par_extend(new_items, ());
                    }
                    cursor.seek(&Count(splice_end), Bias::Right);
                    new_tree.append(cursor.slice(&tree_end, Bias::Right), ());
                    new_tree
                };

                assert_eq!(tree.items(()), reference_items);
                assert_eq!(
                    tree.iter().collect::<Vec<_>>(),
                    tree.cursor::<()>(()).collect::<Vec<_>>()
                );

                log::info!("tree items: {:?}", tree.items(()));

                let mut filter_cursor =
                    tree.filter::<_, Count>((), |summary| summary.contains_even);
                let expected_filtered_items = tree
                    .items(())
                    .into_iter()
                    .enumerate()
                    .filter(|(_, item)| (item & 1) == 0)
                    .collect::<Vec<_>>();

                let mut item_ix = if rng.random() {
                    filter_cursor.next();
                    0
                } else {
                    filter_cursor.prev();
                    expected_filtered_items.len().saturating_sub(1)
                };
                while item_ix < expected_filtered_items.len() {
                    log::info!("filter_cursor, item_ix: {}", item_ix);
                    let actual_item = filter_cursor.item().unwrap();
                    let (reference_index, reference_item) = expected_filtered_items[item_ix];
                    assert_eq!(actual_item, &reference_item);
                    assert_eq!(filter_cursor.start().0, reference_index);
                    log::info!("next");
                    filter_cursor.next();
                    item_ix += 1;

                    while item_ix > 0 && rng.random_bool(0.2) {
                        log::info!("prev");
                        filter_cursor.prev();
                        item_ix -= 1;

                        if item_ix == 0 && rng.random_bool(0.2) {
                            filter_cursor.prev();
                            assert_eq!(filter_cursor.item(), None);
                            assert_eq!(filter_cursor.start().0, 0);
                            filter_cursor.next();
                        }
                    }
                }
                assert_eq!(filter_cursor.item(), None);

                let mut before_start = false;
                let mut cursor = tree.cursor::<Count>(());
                let start_pos = rng.random_range(0..=reference_items.len());
                cursor.seek(&Count(start_pos), Bias::Right);
                let mut pos = rng.random_range(start_pos..=reference_items.len());
                cursor.seek_forward(&Count(pos), Bias::Right);

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
                        assert_eq!(cursor.next_item(), reference_items.first());
                    } else if pos + 1 < reference_items.len() {
                        assert_eq!(cursor.next_item().unwrap(), &reference_items[pos + 1]);
                    } else {
                        assert_eq!(cursor.next_item(), None);
                    }

                    if i < 5 {
                        cursor.next();
                        if pos < reference_items.len() {
                            pos += 1;
                            before_start = false;
                        }
                    } else {
                        cursor.prev();
                        if pos == 0 {
                            before_start = true;
                        }
                        pos = pos.saturating_sub(1);
                    }
                }
            }

            for _ in 0..10 {
                let end = rng.random_range(0..tree.extent::<Count>(()).0 + 1);
                let start = rng.random_range(0..end + 1);
                let start_bias = if rng.random() {
                    Bias::Left
                } else {
                    Bias::Right
                };
                let end_bias = if rng.random() {
                    Bias::Left
                } else {
                    Bias::Right
                };

                let mut cursor = tree.cursor::<Count>(());
                cursor.seek(&Count(start), start_bias);
                let slice = cursor.slice(&Count(end), end_bias);

                cursor.seek(&Count(start), start_bias);
                let summary = cursor.summary::<_, Sum>(&Count(end), end_bias);

                assert_eq!(summary.0, slice.summary().sum);
            }
        }
    }

    #[test]
    fn test_cursor() {
        // Empty tree
        let tree = SumTree::<u8>::default();
        let mut cursor = tree.cursor::<IntegersSummary>(());
        assert_eq!(
            cursor.slice(&Count(0), Bias::Right).items(()),
            Vec::<u8>::new()
        );
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 0);
        cursor.prev();
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 0);
        cursor.next();
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 0);

        // Single-element tree
        let mut tree = SumTree::<u8>::default();
        tree.extend(vec![1], ());
        let mut cursor = tree.cursor::<IntegersSummary>(());
        assert_eq!(
            cursor.slice(&Count(0), Bias::Right).items(()),
            Vec::<u8>::new()
        );
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 0);

        cursor.next();
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 1);

        cursor.prev();
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 0);

        let mut cursor = tree.cursor::<IntegersSummary>(());
        assert_eq!(cursor.slice(&Count(1), Bias::Right).items(()), [1]);
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 1);

        cursor.seek(&Count(0), Bias::Right);
        assert_eq!(
            cursor
                .slice(&tree.extent::<Count>(()), Bias::Right)
                .items(()),
            [1]
        );
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 1);

        // Multiple-element tree
        let mut tree = SumTree::default();
        tree.extend(vec![1, 2, 3, 4, 5, 6], ());
        let mut cursor = tree.cursor::<IntegersSummary>(());

        assert_eq!(cursor.slice(&Count(2), Bias::Right).items(()), [1, 2]);
        assert_eq!(cursor.item(), Some(&3));
        assert_eq!(cursor.prev_item(), Some(&2));
        assert_eq!(cursor.next_item(), Some(&4));
        assert_eq!(cursor.start().sum, 3);

        cursor.next();
        assert_eq!(cursor.item(), Some(&4));
        assert_eq!(cursor.prev_item(), Some(&3));
        assert_eq!(cursor.next_item(), Some(&5));
        assert_eq!(cursor.start().sum, 6);

        cursor.next();
        assert_eq!(cursor.item(), Some(&5));
        assert_eq!(cursor.prev_item(), Some(&4));
        assert_eq!(cursor.next_item(), Some(&6));
        assert_eq!(cursor.start().sum, 10);

        cursor.next();
        assert_eq!(cursor.item(), Some(&6));
        assert_eq!(cursor.prev_item(), Some(&5));
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 15);

        cursor.next();
        cursor.next();
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&6));
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 21);

        cursor.prev();
        assert_eq!(cursor.item(), Some(&6));
        assert_eq!(cursor.prev_item(), Some(&5));
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 15);

        cursor.prev();
        assert_eq!(cursor.item(), Some(&5));
        assert_eq!(cursor.prev_item(), Some(&4));
        assert_eq!(cursor.next_item(), Some(&6));
        assert_eq!(cursor.start().sum, 10);

        cursor.prev();
        assert_eq!(cursor.item(), Some(&4));
        assert_eq!(cursor.prev_item(), Some(&3));
        assert_eq!(cursor.next_item(), Some(&5));
        assert_eq!(cursor.start().sum, 6);

        cursor.prev();
        assert_eq!(cursor.item(), Some(&3));
        assert_eq!(cursor.prev_item(), Some(&2));
        assert_eq!(cursor.next_item(), Some(&4));
        assert_eq!(cursor.start().sum, 3);

        cursor.prev();
        assert_eq!(cursor.item(), Some(&2));
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.next_item(), Some(&3));
        assert_eq!(cursor.start().sum, 1);

        cursor.prev();
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.next_item(), Some(&2));
        assert_eq!(cursor.start().sum, 0);

        cursor.prev();
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.next_item(), Some(&1));
        assert_eq!(cursor.start().sum, 0);

        cursor.next();
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.next_item(), Some(&2));
        assert_eq!(cursor.start().sum, 0);

        let mut cursor = tree.cursor::<IntegersSummary>(());
        assert_eq!(
            cursor
                .slice(&tree.extent::<Count>(()), Bias::Right)
                .items(()),
            tree.items(())
        );
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&6));
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 21);

        cursor.seek(&Count(3), Bias::Right);
        assert_eq!(
            cursor
                .slice(&tree.extent::<Count>(()), Bias::Right)
                .items(()),
            [4, 5, 6]
        );
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&6));
        assert_eq!(cursor.next_item(), None);
        assert_eq!(cursor.start().sum, 21);

        // Seeking can bias left or right
        cursor.seek(&Count(1), Bias::Left);
        assert_eq!(cursor.item(), Some(&1));
        cursor.seek(&Count(1), Bias::Right);
        assert_eq!(cursor.item(), Some(&2));

        // Slicing without resetting starts from where the cursor is parked at.
        cursor.seek(&Count(1), Bias::Right);
        assert_eq!(cursor.slice(&Count(3), Bias::Right).items(()), vec![2, 3]);
        assert_eq!(cursor.slice(&Count(6), Bias::Left).items(()), vec![4, 5]);
        assert_eq!(cursor.slice(&Count(6), Bias::Right).items(()), vec![6]);
    }

    #[test]
    fn test_edit() {
        let mut tree = SumTree::<u8>::default();

        let removed = tree.edit(vec![Edit::Insert(1), Edit::Insert(2), Edit::Insert(0)], ());
        assert_eq!(tree.items(()), vec![0, 1, 2]);
        assert_eq!(removed, Vec::<u8>::new());
        assert_eq!(tree.get(&0, ()), Some(&0));
        assert_eq!(tree.get(&1, ()), Some(&1));
        assert_eq!(tree.get(&2, ()), Some(&2));
        assert_eq!(tree.get(&4, ()), None);

        let removed = tree.edit(vec![Edit::Insert(2), Edit::Insert(4), Edit::Remove(0)], ());
        assert_eq!(tree.items(()), vec![1, 2, 4]);
        assert_eq!(removed, vec![0, 2]);
        assert_eq!(tree.get(&0, ()), None);
        assert_eq!(tree.get(&1, ()), Some(&1));
        assert_eq!(tree.get(&2, ()), Some(&2));
        assert_eq!(tree.get(&4, ()), Some(&4));
    }

    #[test]
    fn test_from_iter() {
        assert_eq!(
            SumTree::from_iter(0..100, ()).items(()),
            (0..100).collect::<Vec<_>>()
        );

        // Ensure `from_iter` works correctly when the given iterator restarts
        // after calling `next` if `None` was already returned.
        let mut ix = 0;
        let iterator = std::iter::from_fn(|| {
            ix = (ix + 1) % 2;
            if ix == 1 { Some(1) } else { None }
        });
        assert_eq!(SumTree::from_iter(iterator, ()).items(()), vec![1]);
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

        fn summary(&self, _cx: ()) -> Self::Summary {
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

    impl ContextLessSummary for IntegersSummary {
        fn zero() -> Self {
            Default::default()
        }

        fn add_summary(&mut self, other: &Self) {
            self.count += other.count;
            self.sum += other.sum;
            self.contains_even |= other.contains_even;
            self.max = cmp::max(self.max, other.max);
        }
    }

    impl Dimension<'_, IntegersSummary> for u8 {
        fn zero(_cx: ()) -> Self {
            Default::default()
        }

        fn add_summary(&mut self, summary: &IntegersSummary, _: ()) {
            *self = summary.max;
        }
    }

    impl Dimension<'_, IntegersSummary> for Count {
        fn zero(_cx: ()) -> Self {
            Default::default()
        }

        fn add_summary(&mut self, summary: &IntegersSummary, _: ()) {
            self.0 += summary.count;
        }
    }

    impl SeekTarget<'_, IntegersSummary, IntegersSummary> for Count {
        fn cmp(&self, cursor_location: &IntegersSummary, _: ()) -> Ordering {
            self.0.cmp(&cursor_location.count)
        }
    }

    impl Dimension<'_, IntegersSummary> for Sum {
        fn zero(_cx: ()) -> Self {
            Default::default()
        }

        fn add_summary(&mut self, summary: &IntegersSummary, _: ()) {
            self.0 += summary.sum;
        }
    }
}
