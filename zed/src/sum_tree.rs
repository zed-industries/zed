mod cursor;

use crate::util::Bias;
use arrayvec::ArrayVec;
pub use cursor::Cursor;
pub use cursor::FilterCursor;
use std::{cmp::Ordering, fmt, iter::FromIterator, sync::Arc};

#[cfg(test)]
const TREE_BASE: usize = 2;
#[cfg(not(test))]
const TREE_BASE: usize = 6;

pub trait Item: Clone + fmt::Debug {
    type Summary: Summary;

    fn summary(&self) -> Self::Summary;
}

pub trait KeyedItem: Item {
    type Key: for<'a> Dimension<'a, Self::Summary> + Ord;

    fn key(&self) -> Self::Key;
}

pub trait Summary: Default + Clone + fmt::Debug {
    type Context;

    fn add_summary(&mut self, summary: &Self, cx: &Self::Context);
}

pub trait Dimension<'a, S: Summary>: Clone + fmt::Debug + Default {
    fn add_summary(&mut self, _summary: &'a S, _: &S::Context);
}

impl<'a, T: Summary> Dimension<'a, T> for () {
    fn add_summary(&mut self, _: &'a T, _: &T::Context) {}
}

pub trait SeekDimension<'a, T: Summary>: Dimension<'a, T> {
    fn cmp(&self, other: &Self, cx: &T::Context) -> Ordering;
}

impl<'a, S: Summary, T: Dimension<'a, S> + Ord> SeekDimension<'a, S> for T {
    fn cmp(&self, other: &Self, _ctx: &S::Context) -> Ordering {
        Ord::cmp(self, other)
    }
}

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

    #[allow(unused)]
    pub fn items(&self, cx: &<T::Summary as Summary>::Context) -> Vec<T> {
        let mut items = Vec::new();
        let mut cursor = self.cursor::<(), ()>();
        cursor.next(cx);
        while let Some(item) = cursor.item() {
            items.push(item.clone());
            cursor.next(cx);
        }
        items
    }

    pub fn cursor<'a, S, U>(&'a self) -> Cursor<T, S, U>
    where
        S: Dimension<'a, T::Summary>,
        U: Dimension<'a, T::Summary>,
    {
        Cursor::new(self)
    }

    pub fn filter<'a, F, U>(
        &'a self,
        filter_node: F,
        cx: &<T::Summary as Summary>::Context,
    ) -> FilterCursor<F, T, U>
    where
        F: Fn(&T::Summary) -> bool,
        U: Dimension<'a, T::Summary>,
    {
        FilterCursor::new(self, filter_node, cx)
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

    pub fn summary(&self) -> T::Summary {
        match self.0.as_ref() {
            Node::Internal { summary, .. } => summary.clone(),
            Node::Leaf { summary, .. } => summary.clone(),
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
        let mut leaf: Option<Node<T>> = None;

        for item in iter {
            if leaf.is_some() && leaf.as_ref().unwrap().items().len() == 2 * TREE_BASE {
                self.push_tree(SumTree(Arc::new(leaf.take().unwrap())), cx);
            }

            if leaf.is_none() {
                leaf = Some(Node::Leaf::<T> {
                    summary: T::Summary::default(),
                    items: ArrayVec::new(),
                    item_summaries: ArrayVec::new(),
                });
            }

            if let Some(Node::Leaf {
                summary,
                items,
                item_summaries,
            }) = leaf.as_mut()
            {
                let item_summary = item.summary();
                summary.add_summary(&item_summary, cx);
                items.push(item);
                item_summaries.push(item_summary);
            } else {
                unreachable!()
            }
        }

        if leaf.is_some() {
            self.push_tree(SumTree(Arc::new(leaf.take().unwrap())), cx);
        }
    }

    pub fn push(&mut self, item: T, cx: &<T::Summary as Summary>::Context) {
        let summary = item.summary();
        self.push_tree(
            SumTree(Arc::new(Node::Leaf {
                summary: summary.clone(),
                items: ArrayVec::from_iter(Some(item)),
                item_summaries: ArrayVec::from_iter(Some(summary)),
            })),
            cx,
        );
    }

    pub fn push_tree(&mut self, other: Self, cx: &<T::Summary as Summary>::Context) {
        if !other.0.is_leaf() || other.0.items().len() > 0 {
            if self.0.height() < other.0.height() {
                for tree in other.0.child_trees() {
                    self.push_tree(tree.clone(), cx);
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
                summary.add_summary(other_node.summary(), cx);

                let height_delta = *height - other_node.height();
                let mut summaries_to_append = ArrayVec::<[T::Summary; 2 * TREE_BASE]>::new();
                let mut trees_to_append = ArrayVec::<[SumTree<T>; 2 * TREE_BASE]>::new();
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
                    let left_summaries: ArrayVec<_>;
                    let right_summaries: ArrayVec<_>;
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
                    let right_summaries: ArrayVec<[T::Summary; 2 * TREE_BASE]>;

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
                    summary.add_summary(other_node.summary(), cx);
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
}

impl<T: KeyedItem> SumTree<T> {
    pub fn insert_or_replace(&mut self, item: T, cx: &<T::Summary as Summary>::Context) -> bool {
        let mut replaced = false;
        *self = {
            let mut cursor = self.cursor::<T::Key, ()>();
            let mut new_tree = cursor.slice(&item.key(), Bias::Left, cx);
            if cursor
                .item()
                .map_or(false, |cursor_item| cursor_item.key() == item.key())
            {
                cursor.next(cx);
                replaced = true;
            }
            new_tree.push(item, cx);
            new_tree.push_tree(cursor.suffix(cx), cx);
            new_tree
        };
        replaced
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
            let mut cursor = self.cursor::<T::Key, ()>();
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
                    new_tree.push_tree(slice, cx);
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
            new_tree.push_tree(cursor.suffix(cx), cx);
            new_tree
        };

        removed
    }

    pub fn get(&self, key: &T::Key, cx: &<T::Summary as Summary>::Context) -> Option<&T> {
        let mut cursor = self.cursor::<T::Key, ()>();
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
        child_summaries: ArrayVec<[T::Summary; 2 * TREE_BASE]>,
        child_trees: ArrayVec<[SumTree<T>; 2 * TREE_BASE]>,
    },
    Leaf {
        summary: T::Summary,
        items: ArrayVec<[T; 2 * TREE_BASE]>,
        item_summaries: ArrayVec<[T::Summary; 2 * TREE_BASE]>,
    },
}

impl<T: Item> Node<T> {
    fn is_leaf(&self) -> bool {
        match self {
            Node::Leaf { .. } => true,
            _ => false,
        }
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

    fn child_trees(&self) -> &ArrayVec<[SumTree<T>; 2 * TREE_BASE]> {
        match self {
            Node::Internal { child_trees, .. } => child_trees,
            Node::Leaf { .. } => panic!("Leaf nodes have no child trees"),
        }
    }

    fn items(&self) -> &ArrayVec<[T; 2 * TREE_BASE]> {
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
    use std::ops::Add;

    #[test]
    fn test_extend_and_push_tree() {
        let mut tree1 = SumTree::new();
        tree1.extend(0..20, &());

        let mut tree2 = SumTree::new();
        tree2.extend(50..100, &());

        tree1.push_tree(tree2, &());
        assert_eq!(
            tree1.items(&()),
            (0..20).chain(50..100).collect::<Vec<u8>>()
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_random(mut rng: StdRng) {
        let rng = &mut rng;
        let mut tree = SumTree::<u8>::new();
        let count = rng.gen_range(0..10);
        tree.extend(rng.sample_iter(distributions::Standard).take(count), &());

        for _ in 0..5 {
            let splice_end = rng.gen_range(0..tree.extent::<Count>(&()).0 + 1);
            let splice_start = rng.gen_range(0..splice_end + 1);
            let count = rng.gen_range(0..3);
            let tree_end = tree.extent::<Count>(&());
            let new_items = rng
                .sample_iter(distributions::Standard)
                .take(count)
                .collect::<Vec<u8>>();

            let mut reference_items = tree.items(&());
            reference_items.splice(splice_start..splice_end, new_items.clone());

            tree = {
                let mut cursor = tree.cursor::<Count, ()>();
                let mut new_tree = cursor.slice(&Count(splice_start), Bias::Right, &());
                new_tree.extend(new_items, &());
                cursor.seek(&Count(splice_end), Bias::Right, &());
                new_tree.push_tree(cursor.slice(&tree_end, Bias::Right, &()), &());
                new_tree
            };

            assert_eq!(tree.items(&()), reference_items);

            let mut filter_cursor = tree.filter::<_, Count>(|summary| summary.contains_even, &());
            let mut reference_filter = tree
                .items(&())
                .into_iter()
                .enumerate()
                .filter(|(_, item)| (item & 1) == 0);
            while let Some(actual_item) = filter_cursor.item() {
                let (reference_index, reference_item) = reference_filter.next().unwrap();
                assert_eq!(actual_item, &reference_item);
                assert_eq!(filter_cursor.start().0, reference_index);
                filter_cursor.next(&());
            }
            assert!(reference_filter.next().is_none());

            let mut pos = rng.gen_range(0..tree.extent::<Count>(&()).0 + 1);
            let mut before_start = false;
            let mut cursor = tree.cursor::<Count, Count>();
            cursor.seek(&Count(pos), Bias::Right, &());

            for i in 0..10 {
                assert_eq!(cursor.sum_start().0, pos);

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

            let mut cursor = tree.cursor::<Count, ()>();
            cursor.seek(&Count(start), start_bias, &());
            let slice = cursor.slice(&Count(end), end_bias, &());

            cursor.seek(&Count(start), start_bias, &());
            let summary = cursor.summary::<Sum>(&Count(end), end_bias, &());

            assert_eq!(summary, slice.summary().sum);
        }
    }

    #[test]
    fn test_cursor() {
        // Empty tree
        let tree = SumTree::<u8>::new();
        let mut cursor = tree.cursor::<Count, Sum>();
        assert_eq!(
            cursor.slice(&Count(0), Bias::Right, &()).items(&()),
            Vec::<u8>::new()
        );
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.sum_start(), &Sum(0));

        // Single-element tree
        let mut tree = SumTree::<u8>::new();
        tree.extend(vec![1], &());
        let mut cursor = tree.cursor::<Count, Sum>();
        assert_eq!(
            cursor.slice(&Count(0), Bias::Right, &()).items(&()),
            Vec::<u8>::new()
        );
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.sum_start(), &Sum(0));

        cursor.next(&());
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.sum_start(), &Sum(1));

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.sum_start(), &Sum(0));

        let mut cursor = tree.cursor::<Count, Sum>();
        assert_eq!(cursor.slice(&Count(1), Bias::Right, &()).items(&()), [1]);
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.sum_start(), &Sum(1));

        cursor.seek(&Count(0), Bias::Right, &());
        assert_eq!(
            cursor
                .slice(&tree.extent::<Count>(&()), Bias::Right, &())
                .items(&()),
            [1]
        );
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.sum_start(), &Sum(1));

        // Multiple-element tree
        let mut tree = SumTree::new();
        tree.extend(vec![1, 2, 3, 4, 5, 6], &());
        let mut cursor = tree.cursor::<Count, Sum>();

        assert_eq!(cursor.slice(&Count(2), Bias::Right, &()).items(&()), [1, 2]);
        assert_eq!(cursor.item(), Some(&3));
        assert_eq!(cursor.prev_item(), Some(&2));
        assert_eq!(cursor.sum_start(), &Sum(3));

        cursor.next(&());
        assert_eq!(cursor.item(), Some(&4));
        assert_eq!(cursor.prev_item(), Some(&3));
        assert_eq!(cursor.sum_start(), &Sum(6));

        cursor.next(&());
        assert_eq!(cursor.item(), Some(&5));
        assert_eq!(cursor.prev_item(), Some(&4));
        assert_eq!(cursor.sum_start(), &Sum(10));

        cursor.next(&());
        assert_eq!(cursor.item(), Some(&6));
        assert_eq!(cursor.prev_item(), Some(&5));
        assert_eq!(cursor.sum_start(), &Sum(15));

        cursor.next(&());
        cursor.next(&());
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&6));
        assert_eq!(cursor.sum_start(), &Sum(21));

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&6));
        assert_eq!(cursor.prev_item(), Some(&5));
        assert_eq!(cursor.sum_start(), &Sum(15));

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&5));
        assert_eq!(cursor.prev_item(), Some(&4));
        assert_eq!(cursor.sum_start(), &Sum(10));

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&4));
        assert_eq!(cursor.prev_item(), Some(&3));
        assert_eq!(cursor.sum_start(), &Sum(6));

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&3));
        assert_eq!(cursor.prev_item(), Some(&2));
        assert_eq!(cursor.sum_start(), &Sum(3));

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&2));
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.sum_start(), &Sum(1));

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.sum_start(), &Sum(0));

        cursor.prev(&());
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.sum_start(), &Sum(0));

        cursor.next(&());
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.sum_start(), &Sum(0));

        let mut cursor = tree.cursor::<Count, Sum>();
        assert_eq!(
            cursor
                .slice(&tree.extent::<Count>(&()), Bias::Right, &())
                .items(&()),
            tree.items(&())
        );
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&6));
        assert_eq!(cursor.sum_start(), &Sum(21));

        cursor.seek(&Count(3), Bias::Right, &());
        assert_eq!(
            cursor
                .slice(&tree.extent::<Count>(&()), Bias::Right, &())
                .items(&()),
            [4, 5, 6]
        );
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&6));
        assert_eq!(cursor.sum_start(), &Sum(21));

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
        count: Count,
        sum: Sum,
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
                count: Count(1),
                sum: Sum(*self as usize),
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
            self.count.0 += &other.count.0;
            self.sum.0 += &other.sum.0;
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
            self.0 += summary.count.0;
        }
    }

    impl<'a> Dimension<'a, IntegersSummary> for Sum {
        fn add_summary(&mut self, summary: &IntegersSummary, _: &()) {
            self.0 += summary.sum.0;
        }
    }

    impl<'a> Add<&'a Self> for Sum {
        type Output = Self;

        fn add(mut self, other: &Self) -> Self {
            self.0 += other.0;
            self
        }
    }
}
