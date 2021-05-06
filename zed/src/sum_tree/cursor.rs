use super::*;
use arrayvec::ArrayVec;
use std::{cmp::Ordering, sync::Arc};

#[derive(Clone)]
struct StackEntry<'a, T: Item, S, U> {
    tree: &'a SumTree<T>,
    index: usize,
    seek_dimension: S,
    sum_dimension: U,
}

#[derive(Clone)]
pub struct Cursor<'a, T: Item, S, U> {
    tree: &'a SumTree<T>,
    stack: ArrayVec<[StackEntry<'a, T, S, U>; 16]>,
    seek_dimension: S,
    sum_dimension: U,
    did_seek: bool,
    at_end: bool,
}

impl<'a, T, S, U> Cursor<'a, T, S, U>
where
    T: Item,
    S: Dimension<'a, T::Summary>,
    U: Dimension<'a, T::Summary>,
{
    pub fn new(tree: &'a SumTree<T>) -> Self {
        Self {
            tree,
            stack: ArrayVec::new(),
            seek_dimension: S::default(),
            sum_dimension: U::default(),
            did_seek: false,
            at_end: false,
        }
    }

    fn reset(&mut self) {
        self.did_seek = false;
        self.at_end = false;
        self.stack.truncate(0);
        self.seek_dimension = S::default();
        self.sum_dimension = U::default();
    }

    pub fn start(&self) -> &U {
        &self.sum_dimension
    }

    pub fn end(&self) -> U {
        if let Some(item_summary) = self.item_summary() {
            let mut end = self.start().clone();
            end.add_summary(item_summary);
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

    #[allow(unused)]
    pub fn prev(&mut self) {
        assert!(self.did_seek, "Must seek before calling this method");

        if self.at_end {
            self.seek_dimension = S::default();
            self.sum_dimension = U::default();
            self.descend_to_last_item(self.tree);
            self.at_end = false;
        } else {
            while let Some(entry) = self.stack.pop() {
                if entry.index > 0 {
                    let new_index = entry.index - 1;

                    if let Some(StackEntry {
                        seek_dimension,
                        sum_dimension,
                        ..
                    }) = self.stack.last()
                    {
                        self.seek_dimension = seek_dimension.clone();
                        self.sum_dimension = sum_dimension.clone();
                    } else {
                        self.seek_dimension = S::default();
                        self.sum_dimension = U::default();
                    }

                    match entry.tree.0.as_ref() {
                        Node::Internal {
                            child_trees,
                            child_summaries,
                            ..
                        } => {
                            for summary in &child_summaries[0..new_index] {
                                self.seek_dimension.add_summary(summary);
                                self.sum_dimension.add_summary(summary);
                            }
                            self.stack.push(StackEntry {
                                tree: entry.tree,
                                index: new_index,
                                seek_dimension: self.seek_dimension.clone(),
                                sum_dimension: self.sum_dimension.clone(),
                            });
                            self.descend_to_last_item(&child_trees[new_index]);
                        }
                        Node::Leaf { item_summaries, .. } => {
                            for item_summary in &item_summaries[0..new_index] {
                                self.seek_dimension.add_summary(item_summary);
                                self.sum_dimension.add_summary(item_summary);
                            }
                            self.stack.push(StackEntry {
                                tree: entry.tree,
                                index: new_index,
                                seek_dimension: self.seek_dimension.clone(),
                                sum_dimension: self.sum_dimension.clone(),
                            });
                        }
                    }

                    break;
                }
            }
        }
    }

    pub fn next(&mut self) {
        self.next_internal(|_| true)
    }

    fn next_internal<F>(&mut self, filter_node: F)
    where
        F: Fn(&T::Summary) -> bool,
    {
        let mut descend = false;

        if self.stack.is_empty() && !self.at_end {
            self.stack.push(StackEntry {
                tree: self.tree,
                index: 0,
                seek_dimension: S::default(),
                sum_dimension: U::default(),
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
                            let summary = &child_summaries[entry.index];
                            entry.seek_dimension.add_summary(summary);
                            entry.sum_dimension.add_summary(summary);
                            entry.index += 1;
                        }

                        while entry.index < child_summaries.len() {
                            let next_summary = &child_summaries[entry.index];
                            if filter_node(next_summary) {
                                break;
                            } else {
                                self.seek_dimension.add_summary(next_summary);
                                self.sum_dimension.add_summary(next_summary);
                            }
                            entry.index += 1;
                        }

                        child_trees.get(entry.index)
                    }
                    Node::Leaf { item_summaries, .. } => {
                        if !descend {
                            let item_summary = &item_summaries[entry.index];
                            self.seek_dimension.add_summary(item_summary);
                            entry.seek_dimension.add_summary(item_summary);
                            self.sum_dimension.add_summary(item_summary);
                            entry.sum_dimension.add_summary(item_summary);
                            entry.index += 1;
                        }

                        loop {
                            if let Some(next_item_summary) = item_summaries.get(entry.index) {
                                if filter_node(next_item_summary) {
                                    return;
                                } else {
                                    self.seek_dimension.add_summary(next_item_summary);
                                    entry.seek_dimension.add_summary(next_item_summary);
                                    self.sum_dimension.add_summary(next_item_summary);
                                    entry.sum_dimension.add_summary(next_item_summary);
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
                    seek_dimension: self.seek_dimension.clone(),
                    sum_dimension: self.sum_dimension.clone(),
                });
            } else {
                descend = false;
                self.stack.pop();
            }
        }

        self.at_end = self.stack.is_empty();
        debug_assert!(self.stack.is_empty() || self.stack.last().unwrap().tree.0.is_leaf());
    }

    fn descend_to_last_item(&mut self, mut subtree: &'a SumTree<T>) {
        self.did_seek = true;
        loop {
            match subtree.0.as_ref() {
                Node::Internal {
                    child_trees,
                    child_summaries,
                    ..
                } => {
                    for summary in &child_summaries[0..child_summaries.len() - 1] {
                        self.seek_dimension.add_summary(summary);
                        self.sum_dimension.add_summary(summary);
                    }

                    self.stack.push(StackEntry {
                        tree: subtree,
                        index: child_trees.len() - 1,
                        seek_dimension: self.seek_dimension.clone(),
                        sum_dimension: self.sum_dimension.clone(),
                    });
                    subtree = child_trees.last().unwrap();
                }
                Node::Leaf { item_summaries, .. } => {
                    let last_index = item_summaries.len().saturating_sub(1);
                    for item_summary in &item_summaries[0..last_index] {
                        self.seek_dimension.add_summary(item_summary);
                        self.sum_dimension.add_summary(item_summary);
                    }
                    self.stack.push(StackEntry {
                        tree: subtree,
                        index: last_index,
                        seek_dimension: self.seek_dimension.clone(),
                        sum_dimension: self.sum_dimension.clone(),
                    });
                    break;
                }
            }
        }
    }
}

impl<'a, T, S, U> Cursor<'a, T, S, U>
where
    T: Item,
    S: SeekDimension<'a, T::Summary>,
    U: Dimension<'a, T::Summary>,
{
    pub fn seek(
        &mut self,
        pos: &S,
        bias: SeekBias,
        ctx: &<T::Summary as Summary>::Context,
    ) -> bool {
        self.reset();
        self.seek_internal::<()>(pos, bias, &mut SeekAggregate::None, ctx)
    }

    pub fn seek_forward(
        &mut self,
        pos: &S,
        bias: SeekBias,
        ctx: &<T::Summary as Summary>::Context,
    ) -> bool {
        self.seek_internal::<()>(pos, bias, &mut SeekAggregate::None, ctx)
    }

    pub fn slice(
        &mut self,
        end: &S,
        bias: SeekBias,
        ctx: &<T::Summary as Summary>::Context,
    ) -> SumTree<T> {
        let mut slice = SeekAggregate::Slice(SumTree::new());
        self.seek_internal::<()>(end, bias, &mut slice, ctx);
        if let SeekAggregate::Slice(slice) = slice {
            slice
        } else {
            unreachable!()
        }
    }

    pub fn suffix(&mut self, ctx: &<T::Summary as Summary>::Context) -> SumTree<T> {
        let extent = self.tree.extent::<S>();
        let mut slice = SeekAggregate::Slice(SumTree::new());
        self.seek_internal::<()>(&extent, SeekBias::Right, &mut slice, ctx);
        if let SeekAggregate::Slice(slice) = slice {
            slice
        } else {
            unreachable!()
        }
    }

    pub fn summary<D>(
        &mut self,
        end: &S,
        bias: SeekBias,
        ctx: &<T::Summary as Summary>::Context,
    ) -> D
    where
        D: Dimension<'a, T::Summary>,
    {
        let mut summary = SeekAggregate::Summary(D::default());
        self.seek_internal(end, bias, &mut summary, ctx);
        if let SeekAggregate::Summary(summary) = summary {
            summary
        } else {
            unreachable!()
        }
    }

    fn seek_internal<D>(
        &mut self,
        target: &S,
        bias: SeekBias,
        aggregate: &mut SeekAggregate<T, D>,
        ctx: &<T::Summary as Summary>::Context,
    ) -> bool
    where
        D: Dimension<'a, T::Summary>,
    {
        debug_assert!(target.cmp(&self.seek_dimension, ctx) >= Ordering::Equal);
        let mut containing_subtree = None;

        if self.did_seek {
            'outer: while let Some(entry) = self.stack.last_mut() {
                {
                    match *entry.tree.0 {
                        Node::Internal {
                            ref child_summaries,
                            ref child_trees,
                            ..
                        } => {
                            entry.index += 1;
                            for (child_tree, child_summary) in child_trees[entry.index..]
                                .iter()
                                .zip(&child_summaries[entry.index..])
                            {
                                let mut child_end = self.seek_dimension.clone();
                                child_end.add_summary(&child_summary);

                                let comparison = target.cmp(&child_end, ctx);
                                if comparison == Ordering::Greater
                                    || (comparison == Ordering::Equal && bias == SeekBias::Right)
                                {
                                    self.seek_dimension.add_summary(child_summary);
                                    self.sum_dimension.add_summary(child_summary);
                                    match aggregate {
                                        SeekAggregate::None => {}
                                        SeekAggregate::Slice(slice) => {
                                            slice.push_tree(child_tree.clone(), ctx);
                                        }
                                        SeekAggregate::Summary(summary) => {
                                            summary.add_summary(child_summary);
                                        }
                                    }
                                    entry.index += 1;
                                } else {
                                    containing_subtree = Some(child_tree);
                                    break 'outer;
                                }
                            }
                        }
                        Node::Leaf {
                            ref items,
                            ref item_summaries,
                            ..
                        } => {
                            let mut slice_items = ArrayVec::<[T; 2 * TREE_BASE]>::new();
                            let mut slice_item_summaries =
                                ArrayVec::<[T::Summary; 2 * TREE_BASE]>::new();
                            let mut slice_items_summary = match aggregate {
                                SeekAggregate::Slice(_) => Some(T::Summary::default()),
                                _ => None,
                            };

                            for (item, item_summary) in items[entry.index..]
                                .iter()
                                .zip(&item_summaries[entry.index..])
                            {
                                let mut item_end = self.seek_dimension.clone();
                                item_end.add_summary(item_summary);

                                let comparison = target.cmp(&item_end, ctx);
                                if comparison == Ordering::Greater
                                    || (comparison == Ordering::Equal && bias == SeekBias::Right)
                                {
                                    self.seek_dimension.add_summary(item_summary);
                                    self.sum_dimension.add_summary(item_summary);
                                    match aggregate {
                                        SeekAggregate::None => {}
                                        SeekAggregate::Slice(_) => {
                                            slice_items.push(item.clone());
                                            slice_item_summaries.push(item_summary.clone());
                                            slice_items_summary
                                                .as_mut()
                                                .unwrap()
                                                .add_summary(item_summary, ctx);
                                        }
                                        SeekAggregate::Summary(summary) => {
                                            summary.add_summary(item_summary);
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
                                            ctx,
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
                                        ctx,
                                    );
                                }
                            }
                        }
                    }
                }

                self.stack.pop();
            }
        } else {
            self.did_seek = true;
            containing_subtree = Some(self.tree);
        }

        if let Some(mut subtree) = containing_subtree {
            loop {
                let mut next_subtree = None;
                match *subtree.0 {
                    Node::Internal {
                        ref child_summaries,
                        ref child_trees,
                        ..
                    } => {
                        for (index, (child_tree, child_summary)) in
                            child_trees.iter().zip(child_summaries).enumerate()
                        {
                            let mut child_end = self.seek_dimension.clone();
                            child_end.add_summary(child_summary);

                            let comparison = target.cmp(&child_end, ctx);
                            if comparison == Ordering::Greater
                                || (comparison == Ordering::Equal && bias == SeekBias::Right)
                            {
                                self.seek_dimension.add_summary(child_summary);
                                self.sum_dimension.add_summary(child_summary);
                                match aggregate {
                                    SeekAggregate::None => {}
                                    SeekAggregate::Slice(slice) => {
                                        slice.push_tree(child_trees[index].clone(), ctx);
                                    }
                                    SeekAggregate::Summary(summary) => {
                                        summary.add_summary(child_summary);
                                    }
                                }
                            } else {
                                self.stack.push(StackEntry {
                                    tree: subtree,
                                    index,
                                    seek_dimension: self.seek_dimension.clone(),
                                    sum_dimension: self.sum_dimension.clone(),
                                });
                                next_subtree = Some(child_tree);
                                break;
                            }
                        }
                    }
                    Node::Leaf {
                        ref items,
                        ref item_summaries,
                        ..
                    } => {
                        let mut slice_items = ArrayVec::<[T; 2 * TREE_BASE]>::new();
                        let mut slice_item_summaries =
                            ArrayVec::<[T::Summary; 2 * TREE_BASE]>::new();
                        let mut slice_items_summary = match aggregate {
                            SeekAggregate::Slice(_) => Some(T::Summary::default()),
                            _ => None,
                        };

                        for (index, (item, item_summary)) in
                            items.iter().zip(item_summaries).enumerate()
                        {
                            let mut child_end = self.seek_dimension.clone();
                            child_end.add_summary(item_summary);

                            let comparison = target.cmp(&child_end, ctx);
                            if comparison == Ordering::Greater
                                || (comparison == Ordering::Equal && bias == SeekBias::Right)
                            {
                                self.seek_dimension.add_summary(item_summary);
                                self.sum_dimension.add_summary(item_summary);
                                match aggregate {
                                    SeekAggregate::None => {}
                                    SeekAggregate::Slice(_) => {
                                        slice_items.push(item.clone());
                                        slice_items_summary
                                            .as_mut()
                                            .unwrap()
                                            .add_summary(item_summary, ctx);
                                        slice_item_summaries.push(item_summary.clone());
                                    }
                                    SeekAggregate::Summary(summary) => {
                                        summary.add_summary(item_summary);
                                    }
                                }
                            } else {
                                self.stack.push(StackEntry {
                                    tree: subtree,
                                    index,
                                    seek_dimension: self.seek_dimension.clone(),
                                    sum_dimension: self.sum_dimension.clone(),
                                });
                                break;
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
                                    ctx,
                                );
                            }
                        }
                    }
                };

                if let Some(next_subtree) = next_subtree {
                    subtree = next_subtree;
                } else {
                    break;
                }
            }
        }

        self.at_end = self.stack.is_empty();
        debug_assert!(self.stack.is_empty() || self.stack.last().unwrap().tree.0.is_leaf());
        if bias == SeekBias::Left {
            let mut end = self.seek_dimension.clone();
            if let Some(summary) = self.item_summary() {
                end.add_summary(summary);
            }
            target.cmp(&end, ctx) == Ordering::Equal
        } else {
            target.cmp(&self.seek_dimension, ctx) == Ordering::Equal
        }
    }
}

impl<'a, T, S, U> Iterator for Cursor<'a, T, S, U>
where
    T: Item,
    S: Dimension<'a, T::Summary>,
    U: Dimension<'a, T::Summary>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.did_seek {
            self.next();
        }

        if let Some(item) = self.item() {
            self.next();
            Some(item)
        } else {
            None
        }
    }
}

pub struct FilterCursor<'a, F: Fn(&T::Summary) -> bool, T: Item, U> {
    cursor: Cursor<'a, T, (), U>,
    filter_node: F,
}

impl<'a, F, T, U> FilterCursor<'a, F, T, U>
where
    F: Fn(&T::Summary) -> bool,
    T: Item,
    U: Dimension<'a, T::Summary>,
{
    pub fn new(tree: &'a SumTree<T>, filter_node: F) -> Self {
        let mut cursor = tree.cursor::<(), U>();
        cursor.next_internal(&filter_node);
        Self {
            cursor,
            filter_node,
        }
    }

    pub fn start(&self) -> &U {
        self.cursor.start()
    }

    pub fn item(&self) -> Option<&'a T> {
        self.cursor.item()
    }

    pub fn next(&mut self) {
        self.cursor.next_internal(&self.filter_node);
    }
}

impl<'a, F, T, U> Iterator for FilterCursor<'a, F, T, U>
where
    F: Fn(&T::Summary) -> bool,
    T: Item,
    U: Dimension<'a, T::Summary>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.item() {
            self.cursor.next_internal(&self.filter_node);
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
