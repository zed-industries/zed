mod cursor;

use arrayvec::ArrayVec;
pub use cursor::Cursor;
pub use cursor::FilterCursor;
use std::{fmt, iter::FromIterator, ops::AddAssign, sync::Arc};

#[cfg(test)]
const TREE_BASE: usize = 2;
#[cfg(not(test))]
const TREE_BASE: usize = 6;

pub trait Item: Clone + Eq + fmt::Debug {
    type Summary: for<'a> AddAssign<&'a Self::Summary> + Default + Clone + fmt::Debug;

    fn summary(&self) -> Self::Summary;
}

pub trait KeyedItem: Item {
    type Key: for<'a> Dimension<'a, Self::Summary> + Ord;

    fn key(&self) -> Self::Key;
}

pub trait Dimension<'a, Summary: Default>: 'a + Clone + fmt::Debug + Default {
    fn add_summary(&mut self, summary: &'a Summary);
}

impl<'a, T: Default> Dimension<'a, T> for () {
    fn add_summary(&mut self, _: &'a T) {}
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SeekBias {
    Left,
    Right,
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

    pub fn from_item(item: T) -> Self {
        let mut tree = Self::new();
        tree.push(item);
        tree
    }

    #[allow(unused)]
    pub fn items(&self) -> Vec<T> {
        let mut cursor = self.cursor::<(), ()>();
        cursor.descend_to_first_item(self, |_| true);
        cursor.cloned().collect()
    }

    pub fn cursor<'a, S, U>(&'a self) -> Cursor<T, S, U>
    where
        S: Dimension<'a, T::Summary>,
        U: Dimension<'a, T::Summary>,
    {
        Cursor::new(self)
    }

    pub fn filter<'a, F, U>(&'a self, filter_node: F) -> FilterCursor<F, T, U>
    where
        F: Fn(&T::Summary) -> bool,
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

    pub fn extent<'a, D: Dimension<'a, T::Summary>>(&'a self) -> D {
        let mut extent = D::default();
        match self.0.as_ref() {
            Node::Internal { summary, .. } | Node::Leaf { summary, .. } => {
                extent.add_summary(summary)
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

    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        let mut leaf: Option<Node<T>> = None;

        for item in iter {
            if leaf.is_some() && leaf.as_ref().unwrap().items().len() == 2 * TREE_BASE {
                self.push_tree(SumTree(Arc::new(leaf.take().unwrap())));
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
                *summary += &item_summary;
                items.push(item);
                item_summaries.push(item_summary);
            } else {
                unreachable!()
            }
        }

        if leaf.is_some() {
            self.push_tree(SumTree(Arc::new(leaf.take().unwrap())));
        }
    }

    pub fn push(&mut self, item: T) {
        let summary = item.summary();
        self.push_tree(SumTree::from_child_trees(vec![SumTree(Arc::new(
            Node::Leaf {
                summary: summary.clone(),
                items: ArrayVec::from_iter(Some(item)),
                item_summaries: ArrayVec::from_iter(Some(summary)),
            },
        ))]))
    }

    pub fn push_tree(&mut self, other: Self) {
        let other_node = other.0.clone();
        if !other_node.is_leaf() || other_node.items().len() > 0 {
            if self.0.height() < other_node.height() {
                for tree in other_node.child_trees() {
                    self.push_tree(tree.clone());
                }
            } else if let Some(split_tree) = self.push_tree_recursive(other) {
                *self = Self::from_child_trees(vec![self.clone(), split_tree]);
            }
        }
    }

    fn push_tree_recursive(&mut self, other: SumTree<T>) -> Option<SumTree<T>> {
        match Arc::make_mut(&mut self.0) {
            Node::Internal {
                height,
                summary,
                child_summaries,
                child_trees,
                ..
            } => {
                let other_node = other.0.clone();
                *summary += other_node.summary();

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
                    let tree_to_append = child_trees.last_mut().unwrap().push_tree_recursive(other);
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
                    *summary = sum(left_summaries.iter());
                    *child_summaries = left_summaries;
                    *child_trees = left_trees;

                    Some(SumTree(Arc::new(Node::Internal {
                        height: *height,
                        summary: sum(right_summaries.iter()),
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
                    *summary = sum(item_summaries.iter());
                    Some(SumTree(Arc::new(Node::Leaf {
                        items: right_items,
                        summary: sum(right_summaries.iter()),
                        item_summaries: right_summaries,
                    })))
                } else {
                    *summary += other_node.summary();
                    items.extend(other_node.items().iter().cloned());
                    item_summaries.extend(other_node.child_summaries().iter().cloned());
                    None
                }
            }
        }
    }

    fn from_child_trees(child_trees: Vec<SumTree<T>>) -> Self {
        let height = child_trees[0].0.height() + 1;
        let mut child_summaries = ArrayVec::new();
        for child in &child_trees {
            child_summaries.push(child.0.summary().clone());
        }
        let summary = sum(child_summaries.iter());
        SumTree(Arc::new(Node::Internal {
            height,
            summary,
            child_summaries,
            child_trees: ArrayVec::from_iter(child_trees),
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
    #[allow(unused)]
    pub fn insert(&mut self, item: T) {
        *self = {
            let mut cursor = self.cursor::<T::Key, ()>();
            let mut new_tree = cursor.slice(&item.key(), SeekBias::Left);
            new_tree.push(item);
            new_tree.push_tree(cursor.suffix());
            new_tree
        };
    }

    pub fn edit(&mut self, edits: &mut [Edit<T>]) {
        if edits.is_empty() {
            return;
        }

        edits.sort_unstable_by_key(|item| item.key());

        *self = {
            let mut cursor = self.cursor::<T::Key, ()>();
            let mut new_tree = SumTree::new();
            let mut buffered_items = Vec::new();

            cursor.seek(&T::Key::default(), SeekBias::Left);
            for edit in edits {
                let new_key = edit.key();
                let mut old_item = cursor.item();

                if old_item
                    .as_ref()
                    .map_or(false, |old_item| old_item.key() < new_key)
                {
                    new_tree.extend(buffered_items.drain(..));
                    let slice = cursor.slice(&new_key, SeekBias::Left);
                    new_tree.push_tree(slice);
                    old_item = cursor.item();
                }
                if old_item.map_or(false, |old_item| old_item.key() == new_key) {
                    cursor.next();
                }
                match edit {
                    Edit::Insert(item) => {
                        buffered_items.push(item.clone());
                    }
                }
            }

            new_tree.extend(buffered_items);
            new_tree.push_tree(cursor.suffix());
            new_tree
        };
    }

    pub fn get(&self, key: &T::Key) -> Option<&T> {
        let mut cursor = self.cursor::<T::Key, ()>();
        if cursor.seek(key, SeekBias::Left) {
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
}

impl<T: KeyedItem> Edit<T> {
    fn key(&self) -> T::Key {
        match self {
            Edit::Insert(item) => item.key(),
        }
    }
}

fn sum<'a, T, I>(iter: I) -> T
where
    T: 'a + Default + AddAssign<&'a T>,
    I: Iterator<Item = &'a T>,
{
    let mut sum = T::default();
    for value in iter {
        sum += value;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Add;

    #[test]
    fn test_extend_and_push_tree() {
        let mut tree1 = SumTree::new();
        tree1.extend(0..20);

        let mut tree2 = SumTree::new();
        tree2.extend(50..100);

        tree1.push_tree(tree2);
        assert_eq!(tree1.items(), (0..20).chain(50..100).collect::<Vec<u8>>());
    }

    #[test]
    fn test_random() {
        for seed in 0..100 {
            use rand::{distributions, prelude::*};

            let rng = &mut StdRng::seed_from_u64(seed);

            let mut tree = SumTree::<u8>::new();
            let count = rng.gen_range(0..10);
            tree.extend(rng.sample_iter(distributions::Standard).take(count));

            for _ in 0..5 {
                let splice_end = rng.gen_range(0..tree.extent::<Count>().0 + 1);
                let splice_start = rng.gen_range(0..splice_end + 1);
                let count = rng.gen_range(0..3);
                let tree_end = tree.extent::<Count>();
                let new_items = rng
                    .sample_iter(distributions::Standard)
                    .take(count)
                    .collect::<Vec<u8>>();

                let mut reference_items = tree.items();
                reference_items.splice(splice_start..splice_end, new_items.clone());

                tree = {
                    let mut cursor = tree.cursor::<Count, ()>();
                    let mut new_tree = cursor.slice(&Count(splice_start), SeekBias::Right);
                    new_tree.extend(new_items);
                    cursor.seek(&Count(splice_end), SeekBias::Right);
                    new_tree.push_tree(cursor.slice(&tree_end, SeekBias::Right));
                    new_tree
                };

                assert_eq!(tree.items(), reference_items);

                let mut filter_cursor = tree.filter::<_, Count>(|summary| summary.contains_even);
                let mut reference_filter = tree
                    .items()
                    .into_iter()
                    .enumerate()
                    .filter(|(_, item)| (item & 1) == 0);
                while let Some(actual_item) = filter_cursor.item() {
                    let (reference_index, reference_item) = reference_filter.next().unwrap();
                    assert_eq!(actual_item, &reference_item);
                    assert_eq!(filter_cursor.start().0, reference_index);
                    filter_cursor.next();
                }
                assert!(reference_filter.next().is_none());

                let mut pos = rng.gen_range(0..tree.extent::<Count>().0 + 1);
                let mut before_start = false;
                let mut cursor = tree.cursor::<Count, Count>();
                cursor.seek(&Count(pos), SeekBias::Right);

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
                let end = rng.gen_range(0..tree.extent::<Count>().0 + 1);
                let start = rng.gen_range(0..end + 1);
                let start_bias = if rng.gen() {
                    SeekBias::Left
                } else {
                    SeekBias::Right
                };
                let end_bias = if rng.gen() {
                    SeekBias::Left
                } else {
                    SeekBias::Right
                };

                let mut cursor = tree.cursor::<Count, ()>();
                cursor.seek(&Count(start), start_bias);
                let slice = cursor.slice(&Count(end), end_bias);

                cursor.seek(&Count(start), start_bias);
                let summary = cursor.summary::<Sum>(&Count(end), end_bias);

                assert_eq!(summary, slice.summary().sum);
            }
        }
    }

    #[test]
    fn test_cursor() {
        // Empty tree
        let tree = SumTree::<u8>::new();
        let mut cursor = tree.cursor::<Count, Sum>();
        assert_eq!(
            cursor.slice(&Count(0), SeekBias::Right).items(),
            Vec::<u8>::new()
        );
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.start(), &Sum(0));

        // Single-element tree
        let mut tree = SumTree::<u8>::new();
        tree.extend(vec![1]);
        let mut cursor = tree.cursor::<Count, Sum>();
        assert_eq!(
            cursor.slice(&Count(0), SeekBias::Right).items(),
            Vec::<u8>::new()
        );
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.start(), &Sum(0));

        cursor.next();
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.start(), &Sum(1));

        cursor.prev();
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.start(), &Sum(0));

        let mut cursor = tree.cursor::<Count, Sum>();
        assert_eq!(cursor.slice(&Count(1), SeekBias::Right).items(), [1]);
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.start(), &Sum(1));

        cursor.seek(&Count(0), SeekBias::Right);
        assert_eq!(
            cursor
                .slice(&tree.extent::<Count>(), SeekBias::Right)
                .items(),
            [1]
        );
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.start(), &Sum(1));

        // Multiple-element tree
        let mut tree = SumTree::new();
        tree.extend(vec![1, 2, 3, 4, 5, 6]);
        let mut cursor = tree.cursor::<Count, Sum>();

        assert_eq!(cursor.slice(&Count(2), SeekBias::Right).items(), [1, 2]);
        assert_eq!(cursor.item(), Some(&3));
        assert_eq!(cursor.prev_item(), Some(&2));
        assert_eq!(cursor.start(), &Sum(3));

        cursor.next();
        assert_eq!(cursor.item(), Some(&4));
        assert_eq!(cursor.prev_item(), Some(&3));
        assert_eq!(cursor.start(), &Sum(6));

        cursor.next();
        assert_eq!(cursor.item(), Some(&5));
        assert_eq!(cursor.prev_item(), Some(&4));
        assert_eq!(cursor.start(), &Sum(10));

        cursor.next();
        assert_eq!(cursor.item(), Some(&6));
        assert_eq!(cursor.prev_item(), Some(&5));
        assert_eq!(cursor.start(), &Sum(15));

        cursor.next();
        cursor.next();
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&6));
        assert_eq!(cursor.start(), &Sum(21));

        cursor.prev();
        assert_eq!(cursor.item(), Some(&6));
        assert_eq!(cursor.prev_item(), Some(&5));
        assert_eq!(cursor.start(), &Sum(15));

        cursor.prev();
        assert_eq!(cursor.item(), Some(&5));
        assert_eq!(cursor.prev_item(), Some(&4));
        assert_eq!(cursor.start(), &Sum(10));

        cursor.prev();
        assert_eq!(cursor.item(), Some(&4));
        assert_eq!(cursor.prev_item(), Some(&3));
        assert_eq!(cursor.start(), &Sum(6));

        cursor.prev();
        assert_eq!(cursor.item(), Some(&3));
        assert_eq!(cursor.prev_item(), Some(&2));
        assert_eq!(cursor.start(), &Sum(3));

        cursor.prev();
        assert_eq!(cursor.item(), Some(&2));
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.start(), &Sum(1));

        cursor.prev();
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.start(), &Sum(0));

        cursor.prev();
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.start(), &Sum(0));

        cursor.next();
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.start(), &Sum(0));

        let mut cursor = tree.cursor::<Count, Sum>();
        assert_eq!(
            cursor
                .slice(&tree.extent::<Count>(), SeekBias::Right)
                .items(),
            tree.items()
        );
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&6));
        assert_eq!(cursor.start(), &Sum(21));

        cursor.seek(&Count(3), SeekBias::Right);
        assert_eq!(
            cursor
                .slice(&tree.extent::<Count>(), SeekBias::Right)
                .items(),
            [4, 5, 6]
        );
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&6));
        assert_eq!(cursor.start(), &Sum(21));

        // Seeking can bias left or right
        cursor.seek(&Count(1), SeekBias::Left);
        assert_eq!(cursor.item(), Some(&1));
        cursor.seek(&Count(1), SeekBias::Right);
        assert_eq!(cursor.item(), Some(&2));

        // Slicing without resetting starts from where the cursor is parked at.
        cursor.seek(&Count(1), SeekBias::Right);
        assert_eq!(cursor.slice(&Count(3), SeekBias::Right).items(), vec![2, 3]);
        assert_eq!(cursor.slice(&Count(6), SeekBias::Left).items(), vec![4, 5]);
        assert_eq!(cursor.slice(&Count(6), SeekBias::Right).items(), vec![6]);
    }

    #[derive(Clone, Default, Debug)]
    pub struct IntegersSummary {
        count: Count,
        sum: Sum,
        contains_even: bool,
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
            }
        }
    }

    impl<'a> AddAssign<&'a Self> for IntegersSummary {
        fn add_assign(&mut self, other: &Self) {
            self.count.0 += &other.count.0;
            self.sum.0 += &other.sum.0;
            self.contains_even |= other.contains_even;
        }
    }

    impl<'a> Dimension<'a, IntegersSummary> for Count {
        fn add_summary(&mut self, summary: &IntegersSummary) {
            self.0 += summary.count.0;
        }
    }

    // impl<'a> Add<&'a Self> for Count {
    //     type Output = Self;
    //
    //     fn add(mut self, other: &Self) -> Self {
    //         self.0 += other.0;
    //         self
    //     }
    // }

    impl<'a> Dimension<'a, IntegersSummary> for Sum {
        fn add_summary(&mut self, summary: &IntegersSummary) {
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
