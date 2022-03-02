use super::*;
use arrayvec::ArrayVec;
use std::{cmp::Ordering, mem, sync::Arc};

#[derive(Clone)]
struct StackEntry<'a, T: Item, D> {
    tree: &'a SumTree<T>,
    index: usize,
    position: D,
}

#[derive(Clone)]
pub struct Cursor<'a, T: Item, D> {
    tree: &'a SumTree<T>,
    stack: ArrayVec<StackEntry<'a, T, D>, 16>,
    position: D,
    did_seek: bool,
    at_end: bool,
}

pub struct Iter<'a, T: Item> {
    tree: &'a SumTree<T>,
    stack: ArrayVec<StackEntry<'a, T, ()>, 16>,
}

impl<'a, T, D> Cursor<'a, T, D>
where
    T: Item,
    D: Dimension<'a, T::Summary>,
{
    pub fn new(tree: &'a SumTree<T>) -> Self {
        Self {
            tree,
            stack: ArrayVec::new(),
            position: D::default(),
            did_seek: false,
            at_end: tree.is_empty(),
        }
    }

    fn reset(&mut self) {
        self.did_seek = false;
        self.at_end = self.tree.is_empty();
        self.stack.truncate(0);
        self.position = D::default();
    }

    pub fn start(&self) -> &D {
        &self.position
    }

    pub fn end(&self, cx: &<T::Summary as Summary>::Context) -> D {
        if let Some(item_summary) = self.item_summary() {
            let mut end = self.start().clone();
            end.add_summary(item_summary, cx);
            end
        } else {
            self.start().clone()
        }
    }

    pub fn item(&self) -> Option<&'a T> {
        assert!(self.did_seek, "Must seek before calling this method");
        if let Some(entry) = self.stack.last() {
            match *entry.tree.0 {
                Node::Leaf { ref items, .. } => {
                    if entry.index == items.len() {
                        None
                    } else {
                        Some(&items[entry.index])
                    }
                }
                _ => unreachable!(),
            }
        } else {
            None
        }
    }

    pub fn item_summary(&self) -> Option<&'a T::Summary> {
        assert!(self.did_seek, "Must seek before calling this method");
        if let Some(entry) = self.stack.last() {
            match *entry.tree.0 {
                Node::Leaf {
                    ref item_summaries, ..
                } => {
                    if entry.index == item_summaries.len() {
                        None
                    } else {
                        Some(&item_summaries[entry.index])
                    }
                }
                _ => unreachable!(),
            }
        } else {
            None
        }
    }

    pub fn prev_item(&self) -> Option<&'a T> {
        assert!(self.did_seek, "Must seek before calling this method");
        if let Some(entry) = self.stack.last() {
            if entry.index == 0 {
                if let Some(prev_leaf) = self.prev_leaf() {
                    Some(prev_leaf.0.items().last().unwrap())
                } else {
                    None
                }
            } else {
                match *entry.tree.0 {
                    Node::Leaf { ref items, .. } => Some(&items[entry.index - 1]),
                    _ => unreachable!(),
                }
            }
        } else if self.at_end {
            self.tree.last()
        } else {
            None
        }
    }

    fn prev_leaf(&self) -> Option<&'a SumTree<T>> {
        for entry in self.stack.iter().rev().skip(1) {
            if entry.index != 0 {
                match *entry.tree.0 {
                    Node::Internal {
                        ref child_trees, ..
                    } => return Some(child_trees[entry.index - 1].rightmost_leaf()),
                    Node::Leaf { .. } => unreachable!(),
                };
            }
        }
        None
    }

    pub fn prev(&mut self, cx: &<T::Summary as Summary>::Context) {
        assert!(self.did_seek, "Must seek before calling this method");

        if self.at_end {
            self.position = D::default();
            self.descend_to_last_item(self.tree, cx);
            self.at_end = self.tree.is_empty();
        } else {
            while let Some(entry) = self.stack.pop() {
                if entry.index > 0 {
                    let new_index = entry.index - 1;

                    if let Some(StackEntry { position, .. }) = self.stack.last() {
                        self.position = position.clone();
                    } else {
                        self.position = D::default();
                    }

                    match entry.tree.0.as_ref() {
                        Node::Internal {
                            child_trees,
                            child_summaries,
                            ..
                        } => {
                            for summary in &child_summaries[0..new_index] {
                                self.position.add_summary(summary, cx);
                            }
                            self.stack.push(StackEntry {
                                tree: entry.tree,
                                index: new_index,
                                position: self.position.clone(),
                            });
                            self.descend_to_last_item(&child_trees[new_index], cx);
                        }
                        Node::Leaf { item_summaries, .. } => {
                            for item_summary in &item_summaries[0..new_index] {
                                self.position.add_summary(item_summary, cx);
                            }
                            self.stack.push(StackEntry {
                                tree: entry.tree,
                                index: new_index,
                                position: self.position.clone(),
                            });
                        }
                    }

                    break;
                }
            }
        }
    }

    pub fn next(&mut self, cx: &<T::Summary as Summary>::Context) {
        self.next_internal(|_| true, cx)
    }

    fn next_internal<F>(&mut self, mut filter_node: F, cx: &<T::Summary as Summary>::Context)
    where
        F: FnMut(&T::Summary) -> bool,
    {
        let mut descend = false;

        if self.stack.is_empty() {
            if !self.at_end {
                self.stack.push(StackEntry {
                    tree: self.tree,
                    index: 0,
                    position: D::default(),
                });
                descend = true;
            }
            self.did_seek = true;
        }

        while self.stack.len() > 0 {
            let new_subtree = {
                let entry = self.stack.last_mut().unwrap();
                match entry.tree.0.as_ref() {
                    Node::Internal {
                        child_trees,
                        child_summaries,
                        ..
                    } => {
                        if !descend {
                            entry.position = self.position.clone();
                            entry.index += 1;
                        }

                        while entry.index < child_summaries.len() {
                            let next_summary = &child_summaries[entry.index];
                            if filter_node(next_summary) {
                                break;
                            } else {
                                self.position.add_summary(next_summary, cx);
                            }
                            entry.index += 1;
                        }

                        child_trees.get(entry.index)
                    }
                    Node::Leaf { item_summaries, .. } => {
                        if !descend {
                            let item_summary = &item_summaries[entry.index];
                            self.position.add_summary(item_summary, cx);
                            entry.position.add_summary(item_summary, cx);
                            entry.index += 1;
                        }

                        loop {
                            if let Some(next_item_summary) = item_summaries.get(entry.index) {
                                if filter_node(next_item_summary) {
                                    return;
                                } else {
                                    self.position.add_summary(next_item_summary, cx);
                                    entry.position.add_summary(next_item_summary, cx);
                                    entry.index += 1;
                                }
                            } else {
                                break None;
                            }
                        }
                    }
                }
            };

            if let Some(subtree) = new_subtree {
                descend = true;
                self.stack.push(StackEntry {
                    tree: subtree,
                    index: 0,
                    position: self.position.clone(),
                });
            } else {
                descend = false;
                self.stack.pop();
            }
        }

        self.at_end = self.stack.is_empty();
        debug_assert!(self.stack.is_empty() || self.stack.last().unwrap().tree.0.is_leaf());
    }

    fn descend_to_last_item(
        &mut self,
        mut subtree: &'a SumTree<T>,
        cx: &<T::Summary as Summary>::Context,
    ) {
        self.did_seek = true;
        if subtree.is_empty() {
            return;
        }

        loop {
            match subtree.0.as_ref() {
                Node::Internal {
                    child_trees,
                    child_summaries,
                    ..
                } => {
                    for summary in &child_summaries[0..child_summaries.len() - 1] {
                        self.position.add_summary(summary, cx);
                    }

                    self.stack.push(StackEntry {
                        tree: subtree,
                        index: child_trees.len() - 1,
                        position: self.position.clone(),
                    });
                    subtree = child_trees.last().unwrap();
                }
                Node::Leaf { item_summaries, .. } => {
                    let last_index = item_summaries.len() - 1;
                    for item_summary in &item_summaries[0..last_index] {
                        self.position.add_summary(item_summary, cx);
                    }
                    self.stack.push(StackEntry {
                        tree: subtree,
                        index: last_index,
                        position: self.position.clone(),
                    });
                    break;
                }
            }
        }
    }
}

impl<'a, T, D> Cursor<'a, T, D>
where
    T: Item,
    D: Dimension<'a, T::Summary>,
{
    pub fn seek<Target>(
        &mut self,
        pos: &Target,
        bias: Bias,
        cx: &<T::Summary as Summary>::Context,
    ) -> bool
    where
        Target: SeekTarget<'a, T::Summary, D>,
    {
        self.reset();
        self.seek_internal(pos, bias, &mut (), cx)
    }

    pub fn seek_forward<Target>(
        &mut self,
        pos: &Target,
        bias: Bias,
        cx: &<T::Summary as Summary>::Context,
    ) -> bool
    where
        Target: SeekTarget<'a, T::Summary, D>,
    {
        self.seek_internal(pos, bias, &mut (), cx)
    }

    pub fn slice<Target>(
        &mut self,
        end: &Target,
        bias: Bias,
        cx: &<T::Summary as Summary>::Context,
    ) -> SumTree<T>
    where
        Target: SeekTarget<'a, T::Summary, D>,
    {
        let mut slice = SliceSeekAggregate {
            tree: SumTree::new(),
            leaf_items: ArrayVec::new(),
            leaf_item_summaries: ArrayVec::new(),
            leaf_summary: T::Summary::default(),
        };
        self.seek_internal(end, bias, &mut slice, cx);
        slice.tree
    }

    pub fn suffix(&mut self, cx: &<T::Summary as Summary>::Context) -> SumTree<T> {
        self.slice(&End::new(), Bias::Right, cx)
    }

    pub fn summary<Target, Output>(
        &mut self,
        end: &Target,
        bias: Bias,
        cx: &<T::Summary as Summary>::Context,
    ) -> Output
    where
        Target: SeekTarget<'a, T::Summary, D>,
        Output: Dimension<'a, T::Summary>,
    {
        let mut summary = SummarySeekAggregate(Output::default());
        self.seek_internal(end, bias, &mut summary, cx);
        summary.0
    }

    fn seek_internal(
        &mut self,
        target: &dyn SeekTarget<'a, T::Summary, D>,
        bias: Bias,
        aggregate: &mut dyn SeekAggregate<'a, T>,
        cx: &<T::Summary as Summary>::Context,
    ) -> bool {
        debug_assert!(
            target.cmp(&self.position, cx) >= Ordering::Equal,
            "cannot seek backward from {:?} to {:?}",
            self.position,
            target
        );

        if !self.did_seek {
            self.did_seek = true;
            self.stack.push(StackEntry {
                tree: self.tree,
                index: 0,
                position: Default::default(),
            });
        }

        let mut ascending = false;
        'outer: while let Some(entry) = self.stack.last_mut() {
            match *entry.tree.0 {
                Node::Internal {
                    ref child_summaries,
                    ref child_trees,
                    ..
                } => {
                    if ascending {
                        entry.index += 1;
                    }

                    for (child_tree, child_summary) in child_trees[entry.index..]
                        .iter()
                        .zip(&child_summaries[entry.index..])
                    {
                        let mut child_end = self.position.clone();
                        child_end.add_summary(&child_summary, cx);

                        let comparison = target.cmp(&child_end, cx);
                        if comparison == Ordering::Greater
                            || (comparison == Ordering::Equal && bias == Bias::Right)
                        {
                            self.position = child_end;
                            aggregate.push_tree(child_tree, child_summary, cx);
                            entry.index += 1;
                            entry.position = self.position.clone();
                        } else {
                            self.stack.push(StackEntry {
                                tree: child_tree,
                                index: 0,
                                position: self.position.clone(),
                            });
                            ascending = false;
                            continue 'outer;
                        }
                    }
                }
                Node::Leaf {
                    ref items,
                    ref item_summaries,
                    ..
                } => {
                    aggregate.begin_leaf();

                    for (item, item_summary) in items[entry.index..]
                        .iter()
                        .zip(&item_summaries[entry.index..])
                    {
                        let mut child_end = self.position.clone();
                        child_end.add_summary(item_summary, cx);

                        let comparison = target.cmp(&child_end, cx);
                        if comparison == Ordering::Greater
                            || (comparison == Ordering::Equal && bias == Bias::Right)
                        {
                            self.position = child_end;
                            aggregate.push_item(item, item_summary, cx);
                            entry.index += 1;
                        } else {
                            aggregate.end_leaf(cx);
                            break 'outer;
                        }
                    }

                    aggregate.end_leaf(cx);
                }
            }

            self.stack.pop();
            ascending = true;
        }

        self.at_end = self.stack.is_empty();
        debug_assert!(self.stack.is_empty() || self.stack.last().unwrap().tree.0.is_leaf());

        let mut end = self.position.clone();
        if bias == Bias::Left {
            if let Some(summary) = self.item_summary() {
                end.add_summary(summary, cx);
            }
        }

        target.cmp(&end, cx) == Ordering::Equal
    }
}

impl<'a, T: Item> Iter<'a, T> {
    pub(crate) fn new(tree: &'a SumTree<T>) -> Self {
        Self {
            tree,
            stack: Default::default(),
        }
    }
}

impl<'a, T: Item> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut descend = false;

        if self.stack.is_empty() {
            self.stack.push(StackEntry {
                tree: self.tree,
                index: 0,
                position: (),
            });
            descend = true;
        }

        while self.stack.len() > 0 {
            let new_subtree = {
                let entry = self.stack.last_mut().unwrap();
                match entry.tree.0.as_ref() {
                    Node::Internal { child_trees, .. } => {
                        if !descend {
                            entry.index += 1;
                        }
                        child_trees.get(entry.index)
                    }
                    Node::Leaf { items, .. } => {
                        if !descend {
                            entry.index += 1;
                        }

                        if let Some(next_item) = items.get(entry.index) {
                            return Some(next_item);
                        } else {
                            None
                        }
                    }
                }
            };

            if let Some(subtree) = new_subtree {
                descend = true;
                self.stack.push(StackEntry {
                    tree: subtree,
                    index: 0,
                    position: (),
                });
            } else {
                descend = false;
                self.stack.pop();
            }
        }

        None
    }
}

impl<'a, T, S, D> Iterator for Cursor<'a, T, D>
where
    T: Item<Summary = S>,
    S: Summary<Context = ()>,
    D: Dimension<'a, T::Summary>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.did_seek {
            self.next(&());
        }

        if let Some(item) = self.item() {
            self.next(&());
            Some(item)
        } else {
            None
        }
    }
}

pub struct FilterCursor<'a, F, T: Item, D> {
    cursor: Cursor<'a, T, D>,
    filter_node: F,
}

impl<'a, F, T, D> FilterCursor<'a, F, T, D>
where
    F: FnMut(&T::Summary) -> bool,
    T: Item,
    D: Dimension<'a, T::Summary>,
{
    pub fn new(
        tree: &'a SumTree<T>,
        mut filter_node: F,
        cx: &<T::Summary as Summary>::Context,
    ) -> Self {
        let mut cursor = tree.cursor::<D>();
        cursor.next_internal(&mut filter_node, cx);
        Self {
            cursor,
            filter_node,
        }
    }

    pub fn start(&self) -> &D {
        self.cursor.start()
    }

    pub fn end(&self, cx: &<T::Summary as Summary>::Context) -> D {
        self.cursor.end(cx)
    }

    pub fn item(&self) -> Option<&'a T> {
        self.cursor.item()
    }

    pub fn next(&mut self, cx: &<T::Summary as Summary>::Context) {
        self.cursor.next_internal(&mut self.filter_node, cx);
    }
}

impl<'a, F, T, S, U> Iterator for FilterCursor<'a, F, T, U>
where
    F: Fn(&T::Summary) -> bool,
    T: Item<Summary = S>,
    S: Summary<Context = ()>,
    U: Dimension<'a, T::Summary>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.item() {
            self.cursor.next_internal(&self.filter_node, &());
            Some(item)
        } else {
            None
        }
    }
}

trait SeekAggregate<'a, T: Item> {
    fn begin_leaf(&mut self);
    fn end_leaf(&mut self, cx: &<T::Summary as Summary>::Context);
    fn push_item(
        &mut self,
        item: &'a T,
        summary: &'a T::Summary,
        cx: &<T::Summary as Summary>::Context,
    );
    fn push_tree(
        &mut self,
        tree: &'a SumTree<T>,
        summary: &'a T::Summary,
        cx: &<T::Summary as Summary>::Context,
    );
}

struct SliceSeekAggregate<T: Item> {
    tree: SumTree<T>,
    leaf_items: ArrayVec<T, { 2 * TREE_BASE }>,
    leaf_item_summaries: ArrayVec<T::Summary, { 2 * TREE_BASE }>,
    leaf_summary: T::Summary,
}

struct SummarySeekAggregate<D>(D);

impl<'a, T: Item> SeekAggregate<'a, T> for () {
    fn begin_leaf(&mut self) {}
    fn end_leaf(&mut self, _: &<T::Summary as Summary>::Context) {}
    fn push_item(&mut self, _: &T, _: &T::Summary, _: &<T::Summary as Summary>::Context) {}
    fn push_tree(&mut self, _: &SumTree<T>, _: &T::Summary, _: &<T::Summary as Summary>::Context) {}
}

impl<'a, T: Item> SeekAggregate<'a, T> for SliceSeekAggregate<T> {
    fn begin_leaf(&mut self) {}
    fn end_leaf(&mut self, cx: &<T::Summary as Summary>::Context) {
        self.tree.push_tree(
            SumTree(Arc::new(Node::Leaf {
                summary: mem::take(&mut self.leaf_summary),
                items: mem::take(&mut self.leaf_items),
                item_summaries: mem::take(&mut self.leaf_item_summaries),
            })),
            cx,
        );
    }
    fn push_item(&mut self, item: &T, summary: &T::Summary, cx: &<T::Summary as Summary>::Context) {
        self.leaf_items.push(item.clone());
        self.leaf_item_summaries.push(summary.clone());
        Summary::add_summary(&mut self.leaf_summary, summary, cx);
    }
    fn push_tree(
        &mut self,
        tree: &SumTree<T>,
        _: &T::Summary,
        cx: &<T::Summary as Summary>::Context,
    ) {
        self.tree.push_tree(tree.clone(), cx);
    }
}

impl<'a, T: Item, D> SeekAggregate<'a, T> for SummarySeekAggregate<D>
where
    D: Dimension<'a, T::Summary>,
{
    fn begin_leaf(&mut self) {}
    fn end_leaf(&mut self, _: &<T::Summary as Summary>::Context) {}
    fn push_item(&mut self, _: &T, summary: &'a T::Summary, cx: &<T::Summary as Summary>::Context) {
        self.0.add_summary(summary, cx);
    }
    fn push_tree(
        &mut self,
        _: &SumTree<T>,
        summary: &'a T::Summary,
        cx: &<T::Summary as Summary>::Context,
    ) {
        self.0.add_summary(summary, cx);
    }
}
