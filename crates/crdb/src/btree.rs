use anyhow::{anyhow, Result};
use arrayvec::ArrayVec;
pub use cursor::{Cursor, FilterCursor, Iter};
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::sync::atomic::Ordering::SeqCst;
use std::{cmp::Ordering, fmt, iter::FromIterator, sync::Arc};

mod cursor;
mod map;

pub use cursor::*;
pub use map::*;

#[cfg(test)]
const TREE_BASE: usize = 2;
#[cfg(not(test))]
const TREE_BASE: usize = 6;

pub trait KvStore {
    fn load<V: for<'de> Deserialize<'de>>(
        &self,
        namespace: &[u8],
        key: &[u8],
    ) -> BoxFuture<Result<V>>;
    fn store<V: Serialize>(&self, namespace: &[u8], key: &[u8], value: &V)
        -> BoxFuture<Result<()>>;
}

pub trait Item: Clone {
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
    fn seek_cmp(&self, cursor_location: &D, cx: &S::Context) -> Ordering;
}

impl<'a, S: Summary, D: Dimension<'a, S> + Ord> SeekTarget<'a, S, D> for D {
    fn seek_cmp(&self, cursor_location: &Self, _: &S::Context) -> Ordering {
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
    fn seek_cmp(&self, cursor_location: &(D1, D2), cx: &S::Context) -> Ordering {
        self.seek_cmp(&cursor_location.0, cx)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SavedId(portable_atomic::AtomicU128);

impl Clone for SavedId {
    fn clone(&self) -> Self {
        Self(portable_atomic::AtomicU128::new(self.0.load(SeqCst)))
    }
}

impl SavedId {
    fn as_bytes(&self) -> [u8; 16] {
        self.0.load(SeqCst).to_be_bytes()
    }

    fn is_saved(&self) -> bool {
        self.0.load(SeqCst) > 0
    }

    fn clear(&self) {
        self.0.store(0, SeqCst)
    }

    fn save(&self) {
        #[cfg(any(test, feature = "test-support"))]
        {
            static NEXT_ID: portable_atomic::AtomicU128 = portable_atomic::AtomicU128::new(1);
            self.0.store(NEXT_ID.fetch_add(1, SeqCst), SeqCst)
        }

        #[cfg(not(any(test, feature = "test-support")))]
        {
            let id = uuid::Uuid::new_v4();
            assert!(id.as_u128() > 0);
            self.0.store(id.as_u128(), SeqCst);
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SavedNode<T: Item> {
    Internal {
        height: u8,
        summary: T::Summary,
        child_summaries: ArrayVec<T::Summary, { 2 * TREE_BASE }>,
        child_trees: ArrayVec<SavedId, { 2 * TREE_BASE }>,
    },
    Leaf {
        summary: T::Summary,
        items: ArrayVec<T, { 2 * TREE_BASE }>,
        item_summaries: ArrayVec<T::Summary, { 2 * TREE_BASE }>,
    },
}

struct End<D>(PhantomData<D>);

impl<D> End<D> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'a, S: Summary, D: Dimension<'a, S>> SeekTarget<'a, S, D> for End<D> {
    fn seek_cmp(&self, _: &D, _: &S::Context) -> Ordering {
        Ordering::Greater
    }
}

impl<D> fmt::Debug for End<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("End").finish()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Bias {
    Left,
    Right,
}

impl Default for Bias {
    fn default() -> Self {
        Bias::Left
    }
}

impl PartialOrd for Bias {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bias {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Left, Self::Left) => Ordering::Equal,
            (Self::Left, Self::Right) => Ordering::Less,
            (Self::Right, Self::Right) => Ordering::Equal,
            (Self::Right, Self::Left) => Ordering::Greater,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sequence<T: Item>(Arc<Node<T>>);

impl<T: Item> Sequence<T> {
    pub fn new() -> Self {
        Sequence(Arc::new(Node::Leaf {
            saved_id: Default::default(),
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
        let mut tree = Self::new();
        tree.extend(iter, cx);
        tree
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
        Some(self.leftmost_leaf()?.0.items().first()?)
    }

    pub fn last(&self) -> Option<&T> {
        Some(self.rightmost_leaf()?.0.items().last()?)
    }

    pub fn update_last(&mut self, f: impl FnOnce(&mut T), cx: &<T::Summary as Summary>::Context) {
        self.update_last_recursive(f, cx);
    }

    fn update_last_recursive(
        &mut self,
        f: impl FnOnce(&mut T),
        cx: &<T::Summary as Summary>::Context,
    ) -> Option<T::Summary> {
        let node = Arc::make_mut(&mut self.0);
        node.saved_id().clear();
        match node {
            Node::Internal {
                summary,
                child_summaries,
                child_trees,
                ..
            } => {
                let last_summary = child_summaries.last_mut().unwrap();
                let last_child = child_trees.iter_mut().rev().find_map(|child_tree| {
                    if let ChildTree::Loaded { tree } = child_tree {
                        Some(tree)
                    } else {
                        None
                    }
                })?;
                *last_summary = last_child.update_last_recursive(f, cx).unwrap();
                *summary = sum(child_summaries.iter(), cx);
                Some(summary.clone())
            }
            Node::Leaf {
                summary,
                items,
                item_summaries,
                ..
            } => {
                let item = items.last_mut()?;
                let item_summary = item_summaries.last_mut()?;
                (f)(item);
                *item_summary = item.summary();
                *summary = sum(item_summaries.iter(), cx);
                Some(summary.clone())
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
        let mut leaf: Option<Node<T>> = None;

        for item in iter {
            if leaf.is_some() && leaf.as_ref().unwrap().items().len() == 2 * TREE_BASE {
                self.append(Sequence(Arc::new(leaf.take().unwrap())), cx);
            }

            if leaf.is_none() {
                leaf = Some(Node::Leaf::<T> {
                    saved_id: Default::default(),
                    summary: T::Summary::default(),
                    items: ArrayVec::new(),
                    item_summaries: ArrayVec::new(),
                });
            }

            if let Some(Node::Leaf {
                summary,
                items,
                item_summaries,
                ..
            }) = leaf.as_mut()
            {
                let item_summary = item.summary();
                <T::Summary as Summary>::add_summary(summary, &item_summary, cx);
                items.push(item);
                item_summaries.push(item_summary);
            } else {
                unreachable!()
            }
        }

        if leaf.is_some() {
            self.append(Sequence(Arc::new(leaf.take().unwrap())), cx);
        }
    }

    pub fn push(&mut self, item: T, cx: &<T::Summary as Summary>::Context) {
        let summary = item.summary();
        self.append(
            Sequence(Arc::new(Node::Leaf {
                saved_id: Default::default(),
                summary: summary.clone(),
                items: ArrayVec::from_iter(Some(item)),
                item_summaries: ArrayVec::from_iter(Some(summary)),
            })),
            cx,
        );
    }

    pub fn append(&mut self, other: Self, cx: &<T::Summary as Summary>::Context) {
        let summary = other.summary().clone();
        self.append_internal(
            ChildTree::Loaded {
                tree: other.clone(),
            },
            summary,
            cx,
        );
    }

    fn append_internal(
        &mut self,
        other_child: ChildTree<T>,
        other_summary: T::Summary,
        cx: &<T::Summary as Summary>::Context,
    ) {
        match &other_child {
            ChildTree::Loaded { tree: other } => {
                if !other.0.is_leaf() || !other.0.items().is_empty() {
                    if self.0.height() < other.0.height() {
                        for (tree, summary) in
                            other.0.child_trees().iter().zip(other.0.child_summaries())
                        {
                            self.append_internal(tree.clone(), summary.clone(), cx);
                        }
                    } else if let Some(split_tree) =
                        self.push_tree_recursive(other_child, other_summary, cx)
                    {
                        *self = Self::from_child_trees(self.clone(), split_tree, cx);
                    }
                }
            }
            ChildTree::Unloaded { saved_id } => {
                if self.0.is_leaf() {
                    if self.0.items().is_empty() {
                        let mut child_summaries = ArrayVec::new();
                        child_summaries.push(other_summary.clone());

                        let mut child_trees = ArrayVec::new();
                        child_trees.push(ChildTree::Unloaded {
                            saved_id: saved_id.clone(),
                        });

                        *self = Self(Arc::new(Node::Internal {
                            saved_id: Default::default(),
                            height: 1,
                            summary: other_summary,
                            child_summaries,
                            child_trees,
                        }));
                    } else {
                        let mut summary = self.0.summary().clone();
                        Summary::add_summary(&mut summary, &other_summary, cx);

                        let mut child_summaries = ArrayVec::new();
                        child_summaries.push(self.0.summary().clone());
                        child_summaries.push(other_summary);

                        let mut child_trees = ArrayVec::new();
                        child_trees.push(ChildTree::Loaded { tree: self.clone() });
                        child_trees.push(ChildTree::Unloaded {
                            saved_id: saved_id.clone(),
                        });

                        *self = Self(Arc::new(Node::Internal {
                            saved_id: Default::default(),
                            height: 1,
                            summary,
                            child_summaries,
                            child_trees,
                        }));
                    }
                } else if let Some(split_tree) =
                    self.push_tree_recursive(other_child, other_summary, cx)
                {
                    *self = Self::from_child_trees(self.clone(), split_tree, cx);
                }
            }
        }
    }

    fn push_tree_recursive(
        &mut self,
        other: ChildTree<T>,
        other_summary: T::Summary,
        cx: &<T::Summary as Summary>::Context,
    ) -> Option<Sequence<T>> {
        let node = Arc::make_mut(&mut self.0);
        node.saved_id().clear();
        match node {
            Node::Internal {
                height,
                summary,
                child_summaries,
                child_trees,
                ..
            } => {
                <T::Summary as Summary>::add_summary(summary, &other_summary, cx);

                let mut summaries_to_append = ArrayVec::<T::Summary, { 2 * TREE_BASE }>::new();
                let mut trees_to_append = ArrayVec::<ChildTree<T>, { 2 * TREE_BASE }>::new();
                match other {
                    ChildTree::Loaded { tree: other } => {
                        let other_node = other.0.clone();
                        let height_delta = *height - other_node.height();
                        if height_delta == 0 {
                            summaries_to_append
                                .extend(other_node.child_summaries().iter().cloned());
                            trees_to_append.extend(other_node.child_trees().iter().cloned());
                        } else if height_delta == 1 && !other_node.is_underflowing() {
                            summaries_to_append.push(other_summary);
                            trees_to_append.push(ChildTree::Loaded { tree: other });
                        } else if let ChildTree::Loaded { tree: last_child } =
                            child_trees.last_mut().unwrap()
                        {
                            let tree_to_append = last_child.push_tree_recursive(
                                ChildTree::Loaded { tree: other },
                                other_summary,
                                cx,
                            );
                            *child_summaries.last_mut().unwrap() = last_child.summary().clone();

                            if let Some(split_tree) = tree_to_append {
                                summaries_to_append.push(split_tree.0.summary().clone());
                                trees_to_append.push(ChildTree::Loaded { tree: split_tree });
                            }
                        } else {
                            summaries_to_append.push(other_summary);
                            trees_to_append.push(ChildTree::Loaded { tree: other });
                        }
                    }
                    ChildTree::Unloaded { saved_id } => {
                        summaries_to_append.push(other_summary);
                        trees_to_append.push(ChildTree::Unloaded { saved_id });
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

                    Some(Sequence(Arc::new(Node::Internal {
                        saved_id: Default::default(),
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
                ..
            } => {
                let other_node = match other {
                    ChildTree::Loaded { tree } => tree.0,
                    ChildTree::Unloaded { .. } => {
                        unreachable!("cannot merge an unloaded leaf node")
                    }
                };

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
                    Some(Sequence(Arc::new(Node::Leaf {
                        saved_id: Default::default(),
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
        left: Sequence<T>,
        right: Sequence<T>,
        cx: &<T::Summary as Summary>::Context,
    ) -> Self {
        let height = left.0.height() + 1;
        let mut child_summaries = ArrayVec::new();
        child_summaries.push(left.0.summary().clone());
        child_summaries.push(right.0.summary().clone());
        let mut child_trees = ArrayVec::new();
        child_trees.push(ChildTree::Loaded { tree: left });
        child_trees.push(ChildTree::Loaded { tree: right });
        Sequence(Arc::new(Node::Internal {
            saved_id: Default::default(),
            height,
            summary: sum(child_summaries.iter(), cx),
            child_summaries,
            child_trees,
        }))
    }

    fn leftmost_leaf(&self) -> Option<&Self> {
        match self.0.as_ref() {
            Node::Leaf { .. } => Some(self),
            Node::Internal { child_trees, .. } => child_trees.iter().find_map(|tree| match tree {
                ChildTree::Loaded { tree } => tree.leftmost_leaf(),
                ChildTree::Unloaded { .. } => None,
            }),
        }
    }

    fn rightmost_leaf(&self) -> Option<&Self> {
        match self.0.as_ref() {
            Node::Leaf { .. } => Some(self),
            Node::Internal { child_trees, .. } => {
                child_trees.iter().rev().find_map(|tree| match tree {
                    ChildTree::Loaded { tree } => tree.rightmost_leaf(),
                    ChildTree::Unloaded { .. } => None,
                })
            }
        }
    }

    #[cfg(debug_assertions)]
    pub fn _debug_entries(&self) -> Vec<&T> {
        self.iter().collect::<Vec<_>>()
    }
}

pub struct Probe<'a, T> {
    start: &'a T,
    summary: &'a T,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Prune {
    Descend,
    Unload,
    Keep,
}

impl<T> Sequence<T>
where
    T: Item + Serialize + for<'a> Deserialize<'a>,
    T::Summary: Serialize + for<'a> Deserialize<'a>,
{
    pub async fn from_root<K: KvStore>(root_id: SavedId, kv: &K) -> Result<Self> {
        let root = kv
            .load::<SavedNode<T>>(b"node", &root_id.as_bytes())
            .await?;
        let node = match root {
            SavedNode::Internal {
                height,
                summary,
                child_summaries,
                child_trees,
            } => Node::Internal {
                saved_id: root_id,
                height,
                summary,
                child_summaries,
                child_trees: child_trees
                    .into_iter()
                    .map(|saved_id| ChildTree::Unloaded { saved_id })
                    .collect(),
            },
            SavedNode::Leaf {
                summary,
                items,
                item_summaries,
            } => Node::Leaf {
                saved_id: root_id,
                summary,
                item_summaries,
                items,
            },
        };
        Ok(Self(Arc::new(node)))
    }

    pub async fn load<F, K>(
        &mut self,
        kv: &K,
        cx: &<T::Summary as Summary>::Context,
        mut f: F,
    ) -> Result<()>
    where
        F: FnMut(Probe<T::Summary>) -> bool,
        K: KvStore,
    {
        struct Frame<'a, T: Item> {
            tree: &'a mut Sequence<T>,
            start_summary: T::Summary,
        }

        let mut stack = Vec::new();
        stack.push(Frame {
            tree: self,
            start_summary: Default::default(),
        });
        while let Some(frame) = stack.pop() {
            let mut summary = frame.start_summary;
            match Arc::make_mut(&mut frame.tree.0) {
                Node::Internal {
                    child_summaries,
                    child_trees,
                    ..
                } => {
                    for (child_tree, child_summary) in child_trees.iter_mut().zip(child_summaries) {
                        let probe = Probe {
                            start: &summary,
                            summary: child_summary,
                        };
                        if f(probe) {
                            match child_tree {
                                ChildTree::Loaded { tree } => stack.push(Frame {
                                    tree,
                                    start_summary: summary.clone(),
                                }),
                                ChildTree::Unloaded { saved_id } => {
                                    let tree = Sequence::from_root(saved_id.clone(), kv).await?;
                                    *child_tree = ChildTree::Loaded { tree };
                                    if let ChildTree::Loaded { tree } = child_tree {
                                        stack.push(Frame {
                                            tree,
                                            start_summary: summary.clone(),
                                        });
                                    }
                                }
                            }
                        }

                        Summary::add_summary(&mut summary, child_summary, cx);
                    }
                }
                Node::Leaf { .. } => {}
            }
        }

        Ok(())
    }

    pub async fn save<K>(&self, kv: &K) -> Result<()>
    where
        K: KvStore,
    {
        struct Frame<'a, T: Item> {
            node: &'a Node<T>,
            children_saved: bool,
        }

        let mut stack = Vec::new();
        stack.push(Frame {
            node: self.0.as_ref(),
            children_saved: false,
        });

        while let Some(frame) = stack.last_mut() {
            if frame.node.saved_id().is_saved() {
                stack.pop();
                continue;
            }

            match frame.node {
                Node::Internal {
                    saved_id,
                    height,
                    summary,
                    child_trees,
                    child_summaries,
                    ..
                } => {
                    if frame.children_saved {
                        saved_id.save();
                        kv.store(
                            b"node",
                            &saved_id.as_bytes(),
                            &SavedNode::<T>::Internal {
                                height: *height,
                                summary: summary.clone(),
                                child_summaries: child_summaries.clone(),
                                child_trees: child_trees
                                    .iter()
                                    .map(|tree| tree.saved_id().clone())
                                    .collect(),
                            },
                        )
                        .await?;
                        stack.pop();
                    } else {
                        // When we return to this frame, the children will be saved
                        // because we pushed them to the stack.
                        frame.children_saved = true;
                        for child_tree in child_trees {
                            match child_tree {
                                ChildTree::Loaded { tree: child_tree } => {
                                    stack.push(Frame {
                                        node: child_tree.0.as_ref(),
                                        children_saved: false,
                                    });
                                }
                                ChildTree::Unloaded { .. } => {
                                    // If this child tree was not loaded, then there's
                                    // no need to save it.
                                }
                            }
                        }
                    }
                }
                Node::Leaf {
                    saved_id,
                    summary,
                    items,
                    item_summaries,
                } => {
                    saved_id.save();
                    kv.store(
                        b"node",
                        &saved_id.as_bytes(),
                        &SavedNode::Leaf {
                            summary: summary.clone(),
                            items: items.clone(),
                            item_summaries: item_summaries.clone(),
                        },
                    )
                    .await?;
                }
            }
        }

        Ok(())
    }

    pub fn prune<F>(&mut self, cx: &<T::Summary as Summary>::Context, mut f: F)
    where
        F: FnMut(Probe<T::Summary>) -> Prune,
    {
        struct Frame<'a, T: Item> {
            tree: &'a mut Sequence<T>,
            start_summary: T::Summary,
        }

        let mut stack = Vec::new();
        stack.push(Frame {
            tree: self,
            start_summary: Default::default(),
        });
        while let Some(frame) = stack.pop() {
            let mut summary = frame.start_summary;
            match Arc::make_mut(&mut frame.tree.0) {
                Node::Internal {
                    child_summaries,
                    child_trees,
                    ..
                } => {
                    for (child_tree, child_summary) in child_trees.iter_mut().zip(child_summaries) {
                        let probe = Probe {
                            start: &summary,
                            summary: child_summary,
                        };
                        match f(probe) {
                            Prune::Descend => {
                                if let ChildTree::Loaded { tree } = child_tree {
                                    stack.push(Frame {
                                        tree,
                                        start_summary: summary.clone(),
                                    });
                                }
                            }
                            Prune::Unload => {
                                if child_tree.saved_id().is_saved() {
                                    *child_tree = ChildTree::Unloaded {
                                        saved_id: child_tree.saved_id().clone(),
                                    };
                                }
                            }
                            Prune::Keep => {}
                        }
                        Summary::add_summary(&mut summary, child_summary, cx);
                    }
                }
                Node::Leaf { .. } => {}
            }
        }
    }
}

impl<T: Item + PartialEq> PartialEq for Sequence<T> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T: Item + Eq> Eq for Sequence<T> {}

impl<T: KeyedItem> Sequence<T> {
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
            let mut new_tree = Sequence::new();
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

impl<T: Item> Default for Sequence<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub enum ChildTree<T: Item> {
    Loaded { tree: Sequence<T> },
    Unloaded { saved_id: SavedId },
}

impl<T: Item> ChildTree<T> {
    fn saved_id(&self) -> &SavedId {
        match self {
            ChildTree::Loaded { tree } => tree.0.saved_id(),
            ChildTree::Unloaded { saved_id } => saved_id,
        }
    }

    fn is_loaded(&self) -> bool {
        matches!(self, ChildTree::Loaded { .. })
    }
}

#[derive(Clone, Debug)]
pub enum Node<T: Item> {
    Internal {
        saved_id: SavedId,
        height: u8,
        summary: T::Summary,
        child_summaries: ArrayVec<T::Summary, { 2 * TREE_BASE }>,
        child_trees: ArrayVec<ChildTree<T>, { 2 * TREE_BASE }>,
    },
    Leaf {
        saved_id: SavedId,
        summary: T::Summary,
        items: ArrayVec<T, { 2 * TREE_BASE }>,
        item_summaries: ArrayVec<T::Summary, { 2 * TREE_BASE }>,
    },
}

impl<T: Item> Node<T> {
    fn saved_id(&self) -> &SavedId {
        match self {
            Node::Internal { saved_id, .. } | Node::Leaf { saved_id, .. } => saved_id,
        }
    }

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

    fn child_trees(&self) -> &ArrayVec<ChildTree<T>, { 2 * TREE_BASE }> {
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
    pub fn key(&self) -> T::Key {
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
    use collections::BTreeMap;
    use futures::FutureExt;
    use parking_lot::Mutex;
    use rand::{distributions, prelude::*};
    use std::cmp;

    #[test]
    fn test_extend_and_push_tree() {
        let mut tree1 = Sequence::new();
        tree1.extend(0..20, &());
        assert_eq!(tree1.items(&()), (0..20).collect::<Vec<u8>>());

        let mut tree2 = Sequence::new();
        tree2.extend(50..100, &());
        assert_eq!(tree2.items(&()), (50..100).collect::<Vec<u8>>());

        tree1.append(tree2, &());
        assert_eq!(
            tree1.items(&()),
            (0..20).chain(50..100).collect::<Vec<u8>>()
        );
    }

    #[test]
    fn test_random_in_memory_sequence() {
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
            let mut tree = Sequence::<u8>::new();
            let count = rng.gen_range(0..10);
            tree.extend(rng.sample_iter(distributions::Standard).take(count), &());

            for _ in 0..num_operations {
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
                    let mut cursor = tree.cursor::<Count>();
                    let mut new_tree = cursor.slice(&Count(splice_start), Bias::Right, &());
                    new_tree.extend(new_items, &());
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

                let mut pos = rng.gen_range(0..tree.extent::<Count>(&()).0 + 1);
                let mut before_start = false;
                let mut cursor = tree.cursor::<Count>();
                cursor.seek(&Count(pos), Bias::Right, &());

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
    fn test_random_saved_sequence() {
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
            let kv = InMemoryKv::default();
            let mut tree = Sequence::<u8>::new();
            let mut reference_items = Vec::new();
            let count = rng.gen_range(0..10);
            let initial_items = rng
                .sample_iter(distributions::Standard)
                .take(count)
                .collect::<Vec<_>>();
            log::info!("tree initial items: {:?}", initial_items);
            tree.extend(initial_items.iter().copied(), &());
            reference_items.extend(initial_items);
            assert_eq!(tree.items(&()), reference_items);
            let mut partial_reference_items = reference_items
                .iter()
                .copied()
                .enumerate()
                .collect::<Vec<_>>();

            for _ in 0..num_operations {
                if rng.gen_bool(0.2) {
                    log::info!("saving");
                    smol::block_on(tree.save(&kv)).unwrap();
                }

                if rng.gen_bool(0.2) {
                    let max = rng.gen::<u8>();
                    log::info!("pruning items > {}", max);
                    tree.prune(&(), |probe| {
                        if probe.summary.min > max {
                            Prune::Unload
                        } else if probe.summary.max < max {
                            Prune::Keep
                        } else {
                            Prune::Descend
                        }
                    });
                }

                let splice_end = rng.gen_range(0..tree.extent::<Count>(&()).0 + 1);
                let splice_start = rng.gen_range(0..splice_end + 1);
                smol::block_on(tree.load(&kv, &(), |probe| {
                    let probe_start = probe.start.count;
                    let probe_end = probe.start.count + probe.summary.count;
                    probe_end >= splice_start && probe_start <= splice_end
                }))
                .unwrap();
                let count = rng.gen_range(0..5);
                let tree_end = tree.extent::<Count>(&());
                assert_eq!(tree_end.0, reference_items.len());
                let new_items = rng
                    .sample_iter(distributions::Standard)
                    .take(count)
                    .collect::<Vec<u8>>();

                log::info!(
                    "splicing {:?}..{:?} with {:?}",
                    splice_start,
                    splice_end,
                    new_items
                );
                reference_items.splice(splice_start..splice_end, new_items.clone());

                tree = {
                    let mut cursor = tree.cursor::<Count>();
                    let mut new_tree = cursor.slice(&Count(splice_start), Bias::Right, &());
                    new_tree.extend(new_items, &());
                    cursor.seek(&Count(splice_end), Bias::Right, &());
                    new_tree.append(cursor.slice(&tree_end, Bias::Right, &()), &());
                    new_tree
                };

                let mut full_tree = tree.clone();
                smol::block_on(full_tree.load(&kv, &(), |_| true)).unwrap();

                assert_eq!(full_tree.items(&()), reference_items);
                assert_eq!(
                    tree.iter().collect::<Vec<_>>(),
                    tree.cursor::<()>().collect::<Vec<_>>()
                );

                log::info!("full tree items: {:?}", full_tree.items(&()));
                log::info!("partial tree items: {:?}", tree.items(&()));

                let mut cursor = tree.cursor::<Count>();
                cursor.next(&());
                partial_reference_items.clear();
                while let Some(item) = cursor.item() {
                    partial_reference_items.push((cursor.start().0, *item));
                    cursor.next(&());
                }

                let mut filter_cursor = tree.filter::<_, Count>(|summary| summary.contains_even);
                let expected_filtered_items = partial_reference_items
                    .iter()
                    .copied()
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
                let start_ix = rng.gen_range(0..=reference_items.len());
                cursor.seek(&Count(start_ix), Bias::Right, &());
                let start_ix = rng.gen_range(cursor.start().0..=reference_items.len());
                cursor.seek_forward(&Count(start_ix), Bias::Right, &());
                let mut partial_ix = partial_reference_items
                    .iter()
                    .position(|(ix, _)| *ix >= start_ix)
                    .unwrap_or(partial_reference_items.len());

                for i in 0..10 {
                    let full_ix = if before_start {
                        0
                    } else {
                        partial_reference_items
                            .get(partial_ix)
                            .map_or(reference_items.len(), |(ix, _)| *ix)
                    };
                    assert_eq!(cursor.start().0, full_ix);

                    if partial_ix > 0 {
                        assert_eq!(
                            cursor.prev_item().unwrap(),
                            &partial_reference_items[partial_ix - 1].1
                        );
                    } else {
                        assert_eq!(cursor.prev_item(), None);
                    }

                    if partial_ix < partial_reference_items.len() && !before_start {
                        assert_eq!(
                            cursor.item().unwrap(),
                            &partial_reference_items[partial_ix].1
                        );
                    } else {
                        assert_eq!(cursor.item(), None);
                    }

                    if i < 5 {
                        cursor.next(&());
                        if partial_ix < partial_reference_items.len() {
                            partial_ix += 1;
                            before_start = false;
                        }
                    } else {
                        cursor.prev(&());
                        if partial_ix == 0 {
                            before_start = true;
                        }
                        partial_ix = partial_ix.saturating_sub(1);
                    }
                }
            }

            for _ in 0..10 {
                let end = rng.gen_range(0..tree.extent::<Count>(&()).0 + 1);
                let start = rng.gen_range(0..end + 1);
                let start_bias = if rng.gen() { Bias::Left } else { Bias::Right };
                let end_bias = if rng.gen() { Bias::Left } else { Bias::Right };

                let reference_start = partial_reference_items
                    .iter()
                    .find(|(ix, _)| match start_bias {
                        Bias::Left => *ix + 1 >= start,
                        Bias::Right => *ix >= start,
                    })
                    .map_or(reference_items.len(), |(ix, _)| *ix);
                let reference_end = if start == end && end_bias == Bias::Left {
                    reference_start
                } else {
                    partial_reference_items
                        .iter()
                        .find(|(ix, _)| match end_bias {
                            Bias::Left => *ix + 1 >= end,
                            Bias::Right => *ix >= end,
                        })
                        .map_or(reference_items.len(), |(ix, _)| *ix)
                };
                let reference_sum = reference_items[reference_start..reference_end]
                    .iter()
                    .map(|item| *item as usize)
                    .sum::<usize>();

                let mut cursor = tree.cursor::<Count>();
                cursor.seek(&Count(start), start_bias, &());
                let seek_end = cmp::max(*cursor.start(), Count(end));
                let slice = cursor.slice(&seek_end, end_bias, &());
                assert_eq!(slice.summary().sum, reference_sum);

                cursor.seek(&Count(start), start_bias, &());
                let summary = cursor.summary::<_, Sum>(&seek_end, end_bias, &());
                assert_eq!(summary.0, reference_sum);
            }
        }
    }

    #[test]
    fn test_cursor() {
        // Empty tree
        let tree = Sequence::<u8>::new();
        let mut cursor = tree.cursor::<IntegersSummary>();
        assert_eq!(
            cursor.slice(&Count(0), Bias::Right, &()).items(&()),
            Vec::<u8>::new()
        );
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.start().sum, 0);
        cursor.prev(&());
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.start().sum, 0);
        cursor.next(&());
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.start().sum, 0);

        // Single-element tree
        let mut tree = Sequence::<u8>::new();
        tree.extend(vec![1], &());
        let mut cursor = tree.cursor::<IntegersSummary>();
        assert_eq!(
            cursor.slice(&Count(0), Bias::Right, &()).items(&()),
            Vec::<u8>::new()
        );
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.start().sum, 0);

        cursor.next(&());
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.start().sum, 1);

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.start().sum, 0);

        let mut cursor = tree.cursor::<IntegersSummary>();
        assert_eq!(cursor.slice(&Count(1), Bias::Right, &()).items(&()), [1]);
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&1));
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
        assert_eq!(cursor.start().sum, 1);

        // Multiple-element tree
        let mut tree = Sequence::new();
        tree.extend(vec![1, 2, 3, 4, 5, 6], &());
        let mut cursor = tree.cursor::<IntegersSummary>();

        assert_eq!(cursor.slice(&Count(2), Bias::Right, &()).items(&()), [1, 2]);
        assert_eq!(cursor.item(), Some(&3));
        assert_eq!(cursor.prev_item(), Some(&2));
        assert_eq!(cursor.start().sum, 3);

        cursor.next(&());
        assert_eq!(cursor.item(), Some(&4));
        assert_eq!(cursor.prev_item(), Some(&3));
        assert_eq!(cursor.start().sum, 6);

        cursor.next(&());
        assert_eq!(cursor.item(), Some(&5));
        assert_eq!(cursor.prev_item(), Some(&4));
        assert_eq!(cursor.start().sum, 10);

        cursor.next(&());
        assert_eq!(cursor.item(), Some(&6));
        assert_eq!(cursor.prev_item(), Some(&5));
        assert_eq!(cursor.start().sum, 15);

        cursor.next(&());
        cursor.next(&());
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), Some(&6));
        assert_eq!(cursor.start().sum, 21);

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&6));
        assert_eq!(cursor.prev_item(), Some(&5));
        assert_eq!(cursor.start().sum, 15);

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&5));
        assert_eq!(cursor.prev_item(), Some(&4));
        assert_eq!(cursor.start().sum, 10);

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&4));
        assert_eq!(cursor.prev_item(), Some(&3));
        assert_eq!(cursor.start().sum, 6);

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&3));
        assert_eq!(cursor.prev_item(), Some(&2));
        assert_eq!(cursor.start().sum, 3);

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&2));
        assert_eq!(cursor.prev_item(), Some(&1));
        assert_eq!(cursor.start().sum, 1);

        cursor.prev(&());
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.start().sum, 0);

        cursor.prev(&());
        assert_eq!(cursor.item(), None);
        assert_eq!(cursor.prev_item(), None);
        assert_eq!(cursor.start().sum, 0);

        cursor.next(&());
        assert_eq!(cursor.item(), Some(&1));
        assert_eq!(cursor.prev_item(), None);
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
        let mut tree = Sequence::<u8>::new();

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

    #[derive(Clone, Default, Debug, Serialize, Deserialize)]
    pub struct IntegersSummary {
        count: usize,
        sum: usize,
        contains_even: bool,
        min: u8,
        max: u8,
    }

    #[derive(Copy, Ord, PartialOrd, Default, Eq, PartialEq, Clone, Debug)]
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
                min: *self,
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
            if self.count == 0 {
                self.min = other.min;
            } else if other.count > 0 {
                self.min = cmp::min(self.min, other.min);
            }
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
        fn seek_cmp(&self, cursor_location: &IntegersSummary, _: &()) -> Ordering {
            std::cmp::Ord::cmp(&self.0, &cursor_location.count)
        }
    }

    impl<'a> Dimension<'a, IntegersSummary> for Sum {
        fn add_summary(&mut self, summary: &IntegersSummary, _: &()) {
            self.0 += summary.sum;
        }
    }

    #[derive(Default)]
    struct InMemoryKv(Arc<Mutex<InMemoryKvState>>);

    #[derive(Default)]
    struct InMemoryKvState {
        namespaces: BTreeMap<Vec<u8>, BTreeMap<Vec<u8>, Vec<u8>>>,
    }

    impl KvStore for InMemoryKv {
        fn load<V: for<'de> Deserialize<'de>>(
            &self,
            namespace: &[u8],
            key: &[u8],
        ) -> BoxFuture<Result<V>> {
            let state = self.0.clone();
            let namespace = namespace.to_vec();
            let key = key.to_vec();
            async move {
                let state = state.lock();
                let namespace = state
                    .namespaces
                    .get(&namespace)
                    .ok_or_else(|| anyhow!("namespace not found"))?;
                let value = namespace
                    .get(&key)
                    .ok_or_else(|| anyhow!("key not found"))?;
                Ok(serde_bare::from_slice(value)?)
            }
            .boxed()
        }

        fn store<V: Serialize>(
            &self,
            namespace: &[u8],
            key: &[u8],
            value: &V,
        ) -> BoxFuture<Result<()>> {
            let state = self.0.clone();
            let namespace = namespace.to_vec();
            let key = key.to_vec();
            let value = serde_bare::to_vec(value);
            async move {
                let mut state = state.lock();
                let namespace = state.namespaces.entry(namespace).or_default();
                namespace.insert(key, value?);
                Ok(())
            }
            .boxed()
        }
    }
}
