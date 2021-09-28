use super::*;
use arrayvec::ArrayVec;
use std::{cmp::Ordering, sync::Arc};

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
            at_end: false,
        }
    }

    fn reset(&mut self) {
        self.did_seek = false;
        self.at_end = false;
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
            self.at_end = false;
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

    fn next_internal<F>(&mut self, filter_node: F, cx: &<T::Summary as Summary>::Context)
    where
        F: Fn(&T::Summary) -> bool,
    {
        let mut descend = false;

        if self.stack.is_empty() && !self.at_end {
            self.stack.push(StackEntry {
                tree: self.tree,
                index: 0,
                position: D::default(),
            });
            descend = true;
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
                    let last_index = item_summaries.len().saturating_sub(1);
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
        self.seek_internal::<_, ()>(pos, bias, &mut SeekAggregate::None, cx)
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
        self.seek_internal::<_, ()>(pos, bias, &mut SeekAggregate::None, cx)
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
        let mut slice = SeekAggregate::Slice(SumTree::new());
        self.seek_internal::<_, ()>(end, bias, &mut slice, cx);
        if let SeekAggregate::Slice(slice) = slice {
            slice
        } else {
            unreachable!()
        }
    }

    pub fn suffix(&mut self, cx: &<T::Summary as Summary>::Context) -> SumTree<T> {
        let mut slice = SeekAggregate::Slice(SumTree::new());
        self.seek_internal::<_, ()>(&End::new(), Bias::Right, &mut slice, cx);
        if let SeekAggregate::Slice(slice) = slice {
            slice
        } else {
            unreachable!()
        }
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
        let mut summary = SeekAggregate::Summary(Output::default());
        self.seek_internal(end, bias, &mut summary, cx);
        if let SeekAggregate::Summary(summary) = summary {
            summary
        } else {
            unreachable!()
        }
    }

    fn seek_internal<Target, Output>(
        &mut self,
        target: &Target,
        bias: Bias,
        aggregate: &mut SeekAggregate<T, Output>,
        cx: &<T::Summary as Summary>::Context,
    ) -> bool
    where
        Target: SeekTarget<'a, T::Summary, D>,
        Output: Dimension<'a, T::Summary>,
    {
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
                            match aggregate {
                                SeekAggregate::None => {}
                                SeekAggregate::Slice(slice) => {
                                    slice.push_tree(child_tree.clone(), cx);
                                }
                                SeekAggregate::Summary(summary) => {
                                    summary.add_summary(child_summary, cx);
                                }
                            }
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
                    let mut slice_items = ArrayVec::<T, { 2 * TREE_BASE }>::new();
                    let mut slice_item_summaries = ArrayVec::<T::Summary, { 2 * TREE_BASE }>::new();
                    let mut slice_items_summary = match aggregate {
                        SeekAggregate::Slice(_) => Some(T::Summary::default()),
                        _ => None,
                    };

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
                            match aggregate {
                                SeekAggregate::None => {}
                                SeekAggregate::Slice(_) => {
                                    slice_items.push(item.clone());
                                    slice_item_summaries.push(item_summary.clone());
                                    <T::Summary as Summary>::add_summary(
                                        slice_items_summary.as_mut().unwrap(),
                                        item_summary,
                                        cx,
                                    );
                                }
                                SeekAggregate::Summary(summary) => {
                                    summary.add_summary(item_summary, cx);
                                }
                            }
                            entry.index += 1;
                        } else {
                            if let SeekAggregate::Slice(slice) = aggregate {
                                slice.push_tree(
                                    SumTree(Arc::new(Node::Leaf {
                                        summary: slice_items_summary.unwrap(),
                                        items: slice_items,
                                        item_summaries: slice_item_summaries,
                                    })),
                                    cx,
                                );
                            }
                            break 'outer;
                        }
                    }

                    if let SeekAggregate::Slice(slice) = aggregate {
                        if !slice_items.is_empty() {
                            slice.push_tree(
                                SumTree(Arc::new(Node::Leaf {
                                    summary: slice_items_summary.unwrap(),
                                    items: slice_items,
                                    item_summaries: slice_item_summaries,
                                })),
                                cx,
                            );
                        }
                    }
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

pub struct FilterCursor<'a, F: Fn(&T::Summary) -> bool, T: Item, D> {
    cursor: Cursor<'a, T, D>,
    filter_node: F,
}

impl<'a, F, T, D> FilterCursor<'a, F, T, D>
where
    F: Fn(&T::Summary) -> bool,
    T: Item,
    D: Dimension<'a, T::Summary>,
{
    pub fn new(
        tree: &'a SumTree<T>,
        filter_node: F,
        cx: &<T::Summary as Summary>::Context,
    ) -> Self {
        let mut cursor = tree.cursor::<D>();
        cursor.next_internal(&filter_node, cx);
        Self {
            cursor,
            filter_node,
        }
    }

    pub fn start(&self) -> &D {
        self.cursor.start()
    }

    pub fn item(&self) -> Option<&'a T> {
        self.cursor.item()
    }

    pub fn next(&mut self, cx: &<T::Summary as Summary>::Context) {
        self.cursor.next_internal(&self.filter_node, cx);
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

enum SeekAggregate<T: Item, D> {
    None,
    Slice(SumTree<T>),
    Summary(D),
}
