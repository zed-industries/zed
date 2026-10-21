use super::*;
use heapless::Vec as ArrayVec;
use std::{cmp::Ordering, mem, sync::Arc};
use ztracing::instrument;

#[derive(Clone)]
struct StackEntry<'a, T: Item, D> {
    tree: &'a SumTree<T>,
    /// `index` selects a child subtree for an internal node, or an item for a
    /// leaf node.
    index: u32,
    position: D,
}

impl<'a, T: Item, D> StackEntry<'a, T, D> {
    #[inline]
    fn index(&self) -> usize {
        self.index as usize
    }
}

impl<T: Item + fmt::Debug, D: fmt::Debug> fmt::Debug for StackEntry<'_, T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StackEntry")
            .field("index", &self.index)
            .field("position", &self.position)
            .finish()
    }
}

#[derive(Clone)]
pub struct Cursor<'a, 'b, T: Item, D> {
    tree: &'a SumTree<T>,
    stack: ArrayVec<StackEntry<'a, T, D>, 16, u8>,
    pub position: D,
    did_seek: bool,
    at_end: bool,
    cx: <T::Summary as Summary>::Context<'b>,
}

impl<T: Item + fmt::Debug, D: fmt::Debug> fmt::Debug for Cursor<'_, '_, T, D>
where
    T::Summary: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cursor")
            .field("tree", &self.tree)
            .field("stack", &self.stack)
            .field("position", &self.position)
            .field("did_seek", &self.did_seek)
            .field("at_end", &self.at_end)
            .finish()
    }
}

pub struct Iter<'a, T: Item> {
    tree: &'a SumTree<T>,
    stack: ArrayVec<StackEntry<'a, T, ()>, 16, u8>,
}

impl<'a, 'b, T, D> Cursor<'a, 'b, T, D>
where
    T: Item,
    D: Dimension<'a, T::Summary>,
{
    pub fn new(tree: &'a SumTree<T>, cx: <T::Summary as Summary>::Context<'b>) -> Self {
        Self {
            tree,
            stack: ArrayVec::new(),
            position: D::zero(cx),
            did_seek: false,
            at_end: tree.is_empty(),
            cx,
        }
    }

    pub fn reset(&mut self) {
        self.did_seek = false;
        self.at_end = self.tree.is_empty();
        self.stack.clear();
        self.position = D::zero(self.cx);
    }

    pub fn start(&self) -> &D {
        &self.position
    }

    #[track_caller]
    pub fn end(&self) -> D {
        if let Some(item_summary) = self.item_summary() {
            let mut end = self.start().clone();
            end.add_summary(item_summary, self.cx);
            end
        } else {
            self.start().clone()
        }
    }

    /// Item is None, when the list is empty, or this cursor is at the end of the list.
    #[track_caller]
    pub fn item(&self) -> Option<&'a T> {
        self.assert_did_seek();
        if let Some(entry) = self.stack.last() {
            match *entry.tree.0 {
                Node::Leaf { ref items, .. } => {
                    if entry.index() == items.len() {
                        None
                    } else {
                        Some(&items[entry.index()])
                    }
                }
                _ => unreachable!(),
            }
        } else {
            None
        }
    }

    #[track_caller]
    pub fn item_summary(&self) -> Option<&'a T::Summary> {
        self.assert_did_seek();
        if let Some(entry) = self.stack.last() {
            match *entry.tree.0 {
                Node::Leaf {
                    ref item_summaries, ..
                } => {
                    if entry.index() == item_summaries.len() {
                        None
                    } else {
                        Some(&item_summaries[entry.index()])
                    }
                }
                _ => unreachable!(),
            }
        } else {
            None
        }
    }

    #[track_caller]
    pub fn next_item(&self) -> Option<&'a T> {
        self.assert_did_seek();
        if let Some(entry) = self.stack.last() {
            if entry.index() == entry.tree.0.items().len() - 1 {
                if let Some(next_leaf) = self.next_leaf() {
                    Some(next_leaf.0.items().first().unwrap())
                } else {
                    None
                }
            } else {
                match *entry.tree.0 {
                    Node::Leaf { ref items, .. } => Some(&items[entry.index() + 1]),
                    _ => unreachable!(),
                }
            }
        } else if self.at_end {
            None
        } else {
            self.tree.first()
        }
    }

    #[track_caller]
    fn next_leaf(&self) -> Option<&'a SumTree<T>> {
        for entry in self.stack.iter().rev().skip(1) {
            if entry.index() < entry.tree.0.child_trees().len() - 1 {
                match *entry.tree.0 {
                    Node::Internal {
                        ref child_trees, ..
                    } => return Some(child_trees[entry.index() + 1].leftmost_leaf()),
                    Node::Leaf { .. } => unreachable!(),
                };
            }
        }
        None
    }

    #[track_caller]
    pub fn prev_item(&self) -> Option<&'a T> {
        self.assert_did_seek();
        if let Some(entry) = self.stack.last() {
            if entry.index() == 0 {
                if let Some(prev_leaf) = self.prev_leaf() {
                    Some(prev_leaf.0.items().last().unwrap())
                } else {
                    None
                }
            } else {
                match *entry.tree.0 {
                    Node::Leaf { ref items, .. } => Some(&items[entry.index() - 1]),
                    _ => unreachable!(),
                }
            }
        } else if self.at_end {
            self.tree.last()
        } else {
            None
        }
    }

    #[track_caller]
    fn prev_leaf(&self) -> Option<&'a SumTree<T>> {
        for entry in self.stack.iter().rev().skip(1) {
            if entry.index() != 0 {
                match *entry.tree.0 {
                    Node::Internal {
                        ref child_trees, ..
                    } => return Some(child_trees[entry.index() - 1].rightmost_leaf()),
                    Node::Leaf { .. } => unreachable!(),
                };
            }
        }
        None
    }

    #[track_caller]
    #[instrument(skip_all)]
    pub fn prev(&mut self) {
        self.search_backward(|_| true)
    }

    #[track_caller]
    pub fn search_backward<F>(&mut self, mut filter_node: F)
    where
        F: FnMut(&T::Summary) -> bool,
    {
        if !self.did_seek {
            self.did_seek = true;
            self.at_end = true;
        }

        if self.at_end {
            self.position = D::zero(self.cx);
            self.at_end = self.tree.is_empty();
            if !self.tree.is_empty() {
                self.stack
                    .push(StackEntry {
                        tree: self.tree,
                        index: self.tree.0.child_summaries().len() as u32,
                        position: D::from_summary(self.tree.summary(), self.cx),
                    })
                    .unwrap_oob();
            }
        }

        let mut descending = false;
        while !self.stack.is_empty() {
            if let Some(StackEntry { position, .. }) = self.stack.iter().rev().nth(1) {
                self.position = position.clone();
            } else {
                self.position = D::zero(self.cx);
            }

            let entry = self.stack.last_mut().unwrap();
            if !descending {
                if entry.index() == 0 {
                    self.stack.pop();
                    continue;
                } else {
                    entry.index -= 1;
                }
            }

            for summary in &entry.tree.0.child_summaries()[..entry.index()] {
                self.position.add_summary(summary, self.cx);
            }
            entry.position = self.position.clone();

            descending = filter_node(&entry.tree.0.child_summaries()[entry.index()]);
            match entry.tree.0.as_ref() {
                Node::Internal { child_trees, .. } => {
                    if descending {
                        let tree = &child_trees[entry.index()];
                        self.stack
                            .push(StackEntry {
                                position: D::zero(self.cx),
                                tree,
                                index: tree.0.child_summaries().len() as u32 - 1,
                            })
                            .unwrap_oob();
                    }
                }
                Node::Leaf { .. } => {
                    if descending {
                        break;
                    }
                }
            }
        }
    }

    #[track_caller]
    pub fn next(&mut self) {
        self.search_forward(|_| true)
    }

    #[track_caller]
    pub fn search_forward<F>(&mut self, mut filter_node: F)
    where
        F: FnMut(&T::Summary) -> bool,
    {
        let mut descend = false;

        if self.stack.is_empty() {
            if !self.at_end {
                self.stack
                    .push(StackEntry {
                        tree: self.tree,
                        index: 0,
                        position: D::zero(self.cx),
                    })
                    .unwrap_oob();
                descend = true;
            }
            self.did_seek = true;
        }

        while !self.stack.is_empty() {
            let new_subtree = {
                let entry = self.stack.last_mut().unwrap();
                match entry.tree.0.as_ref() {
                    Node::Internal {
                        child_trees,
                        child_summaries,
                        ..
                    } => {
                        if !descend {
                            entry.index += 1;
                            entry.position = self.position.clone();
                        }

                        while entry.index() < child_summaries.len() {
                            let next_summary = &child_summaries[entry.index()];
                            if filter_node(next_summary) {
                                break;
                            } else {
                                entry.index += 1;
                                entry.position.add_summary(next_summary, self.cx);
                                self.position.add_summary(next_summary, self.cx);
                            }
                        }

                        child_trees.get(entry.index())
                    }
                    Node::Leaf { item_summaries, .. } => {
                        if !descend {
                            let item_summary = &item_summaries[entry.index()];
                            entry.index += 1;
                            entry.position.add_summary(item_summary, self.cx);
                            self.position.add_summary(item_summary, self.cx);
                        }

                        loop {
                            if let Some(next_item_summary) = item_summaries.get(entry.index()) {
                                if filter_node(next_item_summary) {
                                    return;
                                } else {
                                    entry.index += 1;
                                    entry.position.add_summary(next_item_summary, self.cx);
                                    self.position.add_summary(next_item_summary, self.cx);
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
                self.stack
                    .push(StackEntry {
                        tree: subtree,
                        index: 0,
                        position: self.position.clone(),
                    })
                    .unwrap_oob();
            } else {
                descend = false;
                self.stack.pop();
            }
        }

        self.at_end = self.stack.is_empty();
        debug_assert!(self.stack.is_empty() || self.stack.last().unwrap().tree.0.is_leaf());
    }

    #[track_caller]
    fn assert_did_seek(&self) {
        assert!(
            self.did_seek,
            "Must call `seek`, `next` or `prev` before calling this method"
        );
    }

    pub fn did_seek(&self) -> bool {
        self.did_seek
    }

    pub fn at_end(&self) -> bool {
        self.at_end
    }
}

impl<'a, 'b, T, D> Cursor<'a, 'b, T, D>
where
    T: Item,
    D: Dimension<'a, T::Summary>,
{
    /// Returns whether we found the item you were seeking for.
    #[track_caller]
    #[instrument(skip_all)]
    pub fn seek<Target>(&mut self, pos: &Target, bias: Bias) -> bool
    where
        Target: SeekTarget<'a, T::Summary, D>,
    {
        self.reset();
        self.seek_internal(pos, bias, &mut ())
    }

    /// Returns whether we found the item you were seeking for.
    ///
    /// # Panics
    ///
    /// If we did not seek before, use seek instead in that case.
    #[track_caller]
    #[instrument(skip_all)]
    pub fn seek_forward<Target>(&mut self, pos: &Target, bias: Bias) -> bool
    where
        Target: SeekTarget<'a, T::Summary, D>,
    {
        self.seek_internal(pos, bias, &mut ())
    }

    /// Advances the cursor and returns traversed items as a tree.
    #[track_caller]
    pub fn slice<Target>(&mut self, end: &Target, bias: Bias) -> SumTree<T>
    where
        Target: SeekTarget<'a, T::Summary, D>,
    {
        let mut slice = SliceSeekAggregate {
            tree: SumTree::new(self.cx),
            leaf_items: ArrayVec::new(),
            leaf_item_summaries: ArrayVec::new(),
            leaf_summary: <T::Summary as Summary>::zero(self.cx),
        };
        self.seek_internal(end, bias, &mut slice);
        slice.tree
    }

    #[track_caller]
    pub fn suffix(&mut self) -> SumTree<T> {
        self.slice(&End::new(), Bias::Right)
    }

    #[track_caller]
    pub fn summary<Target, Output>(&mut self, end: &Target, bias: Bias) -> Output
    where
        Target: SeekTarget<'a, T::Summary, D>,
        Output: Dimension<'a, T::Summary>,
    {
        let mut summary = SummarySeekAggregate(Output::zero(self.cx));
        self.seek_internal(end, bias, &mut summary);
        summary.0
    }

    // TODO: Rewrite this comment: see "test_seek_return_values".
    /// Returns whether we found the item you were seeking for.
    #[track_caller]
    #[instrument(skip_all)]
    fn seek_internal(
        &mut self,
        target: &dyn SeekTarget<'a, T::Summary, D>,
        bias: Bias,
        aggregate: &mut dyn SeekAggregate<'a, T>,
    ) -> bool {
        assert!(
            target.cmp(&self.position, self.cx).is_ge(),
            "cannot seek backward",
        );
        self.did_seek = true;

        // An empty stack means that there's no tree traversal to resume. This
        // is normally because the cursor has been newly created (or because it
        // has been reset). Another possibility is that the cursor has traversed
        // back to a "before first item" state, and in that case we must restart
        // the search from the root as well.
        //
        // Otherwise, the empty stack might mean the cursor is past the end of
        // the tree, and in that case no further seeking is necessary and the
        // loop below is skipped.
        let should_start_from_root = self.stack.is_empty() && !self.at_end();
        if should_start_from_root {
            // For implementation details, note that we start with the zero
            // position, but also already with a zero index, meaning that this
            // entry already refers to the first item or subtree in the tree, if
            // any.
            self.stack
                .push(StackEntry {
                    tree: self.tree,
                    index: 0,
                    position: D::zero(self.cx),
                })
                .unwrap_oob();
        }

        // The last entry we have in the stack will be our initial entry to
        // consider. In the case of an initial seek (or after a cursor has been
        // reset) it will be the root of the tree as pushed above. Otherwise,
        // we're starting from an existing position, and the entry itself
        // carries aggregate and current information to continue the search.

        // We always start descending through the tree. If the position we seek
        // is not in any subtree here, the end of the 'outer loop makes us
        // ascend (to try again in another subtree).
        let mut ascending = false;
        'outer: while let Some(entry) = self.stack.last_mut() {
            match *entry.tree.0 {
                // For an internal node, we need to either assess that it's
                // completely covered by the seek (and thus move to its next
                // sibling node) or identify the relevant subtree for the seek
                // (and move there).
                Node::Internal {
                    ref child_summaries,
                    ref child_trees,
                    ..
                } => {
                    // If we're ascending the tree, it's because we already
                    // descended it and didn't find the target. The current
                    // entry's index is the one we descended on, so to continue
                    // the search we must look at the next index (which will
                    // represent the next unexplored child node).
                    if ascending {
                        entry.index += 1;
                        entry.position = self.position.clone();
                    }

                    let current_child = entry.index();
                    for (child_tree, child_summary) in child_trees[current_child..]
                        .iter()
                        .zip(&child_summaries[entry.index()..])
                    {
                        let dimension_including_child = self
                            .position
                            .clone()
                            .with_added_summary(child_summary, self.cx);

                        let must_continue = {
                            let comparison = target.cmp(&dimension_including_child, self.cx);
                            let target_is_greater = comparison == Ordering::Greater;
                            let seek_is_inclusive =
                                comparison == Ordering::Equal && bias == Bias::Right;
                            target_is_greater || seek_is_inclusive
                        };

                        // This is the crucial characteristic of a SumTree: the
                        // information contained in the summaries allows us to
                        // skip entire subtrees -- provided we know our target
                        // is past them.
                        if must_continue {
                            self.position = dimension_including_child;
                            aggregate.push_tree(child_tree, child_summary, self.cx);
                            // Note that if this is the last child tree to be
                            // considered in this internal node, the next index
                            // will be out of bounds. This is fine because we
                            // reach the end of the 'outer loop's code that will
                            // pop this stack entry entirely (and ascend the
                            // tree, since we didn't find the target in any
                            // subtree here).
                            entry.index += 1;
                            entry.position = self.position.clone();
                        } else {
                            // We have found the relevant subtree that contains
                            // our target, and shall go there next.
                            self.stack
                                .push(StackEntry {
                                    tree: child_tree,
                                    index: 0,
                                    position: self.position.clone(),
                                })
                                .unwrap_oob();
                            ascending = false;
                            continue 'outer;
                        }
                    }
                }
                // For a leaf node, we need to identify the item in the position
                // we're seeking.
                Node::Leaf {
                    ref items,
                    ref item_summaries,
                    ..
                } => {
                    aggregate.begin_leaf();

                    let current_item = entry.index();
                    for (item, item_summary) in items[current_item..]
                        .iter()
                        .zip(&item_summaries[entry.index()..])
                    {
                        let dimension_including_item = self
                            .position
                            .clone()
                            .with_added_summary(item_summary, self.cx);

                        let must_continue = {
                            let comparison = target.cmp(&dimension_including_item, self.cx);
                            let target_is_greater = comparison == Ordering::Greater;
                            let seek_is_inclusive =
                                comparison == Ordering::Equal && bias == Bias::Right;
                            target_is_greater || seek_is_inclusive
                        };

                        if must_continue {
                            self.position = dimension_including_item;
                            aggregate.push_item(item, item_summary, self.cx);
                            entry.index += 1;
                            // TODO: The fact that we do not update the entry's
                            // position here means that it gets out of sync with
                            // the cursor's. This is fine for correctness
                            // because nothing relies on a leaf node entry's
                            // position, but would be surprising while
                            // debugging. We might want to refactor "stack
                            // entry" to differentiate between internal and leaf
                            // nodes, and then to not carry a position for leaf
                            // nodes.
                        } else {
                            aggregate.end_leaf(self.cx);
                            break 'outer;
                        }
                    }

                    aggregate.end_leaf(self.cx);
                }
            }

            // We only reach this point if we have not stopped early in the
            // loop. That means we did not find a relevant child subtree to
            // search (because that would mean `continue`ing the outer loop),
            // and that we did not find the item in a leaf node (because that
            // would mean `break`ing the outer loop).
            //
            // This means that our target must lie somewhere else in the tree
            // (if it exists), and we must backtrack (ascend) the tree to
            // continue looking.
            self.stack.pop();
            ascending = true;
        }

        self.at_end = self.stack.is_empty();
        debug_assert!(self.stack.is_empty() || self.stack.last().unwrap().tree.0.is_leaf());

        // Note that this is only for reporting purposes. It does not change the
        // cursor's position.
        let mut dimension_after_traversal = self.position.clone();
        if bias == Bias::Left
            && let Some(summary) = self.item_summary()
        {
            // In the case of a left-biased seek, the attached item's
            // "dimension-width" is also considered for the return value.
            dimension_after_traversal.add_summary(summary, self.cx);
        }

        // See `test_seek_return_values()` for a survey of the possible return
        // values here.

        target.cmp(&dimension_after_traversal, self.cx) == Ordering::Equal
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
            self.stack
                .push(StackEntry {
                    tree: self.tree,
                    index: 0,
                    position: (),
                })
                .unwrap_oob();
            descend = true;
        }

        while let Some(entry) = self.stack.last_mut() {
            let new_subtree = {
                match entry.tree.0.as_ref() {
                    Node::Internal { child_trees, .. } => {
                        if !descend {
                            entry.index += 1;
                        }
                        child_trees.get(entry.index())
                    }
                    Node::Leaf { items, .. } => {
                        if !descend {
                            entry.index += 1;
                        }

                        if let Some(next_item) = items.get(entry.index()) {
                            return Some(next_item);
                        } else {
                            None
                        }
                    }
                }
            };

            if let Some(subtree) = new_subtree {
                descend = true;
                self.stack
                    .push(StackEntry {
                        tree: subtree,
                        index: 0,
                        position: (),
                    })
                    .unwrap_oob();
            } else {
                descend = false;
                self.stack.pop();
            }
        }

        None
    }

    fn last(mut self) -> Option<Self::Item> {
        self.stack.clear();
        self.tree.rightmost_leaf().last()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let lower_bound = match self.stack.last() {
            Some(top) => top.tree.0.child_summaries().len() - top.index as usize,
            None => self.tree.0.child_summaries().len(),
        };

        (lower_bound, None)
    }
}

impl<'a, 'b, T: Item, D> Iterator for Cursor<'a, 'b, T, D>
where
    D: Dimension<'a, T::Summary>,
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

pub struct FilterCursor<'a, 'b, F, T: Item, D> {
    cursor: Cursor<'a, 'b, T, D>,
    filter_node: F,
}

impl<'a, 'b, F, T: Item, D> FilterCursor<'a, 'b, F, T, D>
where
    F: FnMut(&T::Summary) -> bool,
    T: Item,
    D: Dimension<'a, T::Summary>,
{
    pub fn new(
        tree: &'a SumTree<T>,
        cx: <T::Summary as Summary>::Context<'b>,
        filter_node: F,
    ) -> Self {
        let cursor = tree.cursor::<D>(cx);
        Self {
            cursor,
            filter_node,
        }
    }

    pub fn start(&self) -> &D {
        self.cursor.start()
    }

    pub fn end(&self) -> D {
        self.cursor.end()
    }

    pub fn item(&self) -> Option<&'a T> {
        self.cursor.item()
    }

    pub fn item_summary(&self) -> Option<&'a T::Summary> {
        self.cursor.item_summary()
    }

    pub fn next(&mut self) {
        self.cursor.search_forward(&mut self.filter_node);
    }

    pub fn prev(&mut self) {
        self.cursor.search_backward(&mut self.filter_node);
    }
}

impl<'a, 'b, F, T: Item, U> Iterator for FilterCursor<'a, 'b, F, T, U>
where
    F: FnMut(&T::Summary) -> bool,
    U: Dimension<'a, T::Summary>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.cursor.did_seek {
            self.next();
        }

        if let Some(item) = self.item() {
            self.cursor.search_forward(&mut self.filter_node);
            Some(item)
        } else {
            None
        }
    }
}

trait SeekAggregate<'a, T: Item> {
    fn begin_leaf(&mut self);
    fn end_leaf(&mut self, cx: <T::Summary as Summary>::Context<'_>);
    fn push_item(
        &mut self,
        item: &'a T,
        summary: &'a T::Summary,
        cx: <T::Summary as Summary>::Context<'_>,
    );
    fn push_tree(
        &mut self,
        tree: &'a SumTree<T>,
        summary: &'a T::Summary,
        cx: <T::Summary as Summary>::Context<'_>,
    );
}

struct SliceSeekAggregate<T: Item> {
    tree: SumTree<T>,
    leaf_items: ArrayVec<T, { 2 * TREE_BASE }, u8>,
    leaf_item_summaries: ArrayVec<T::Summary, { 2 * TREE_BASE }, u8>,
    leaf_summary: T::Summary,
}

struct SummarySeekAggregate<D>(D);

impl<T: Item> SeekAggregate<'_, T> for () {
    fn begin_leaf(&mut self) {}
    fn end_leaf(&mut self, _: <T::Summary as Summary>::Context<'_>) {}
    fn push_item(&mut self, _: &T, _: &T::Summary, _: <T::Summary as Summary>::Context<'_>) {}
    fn push_tree(
        &mut self,
        _: &SumTree<T>,
        _: &T::Summary,
        _: <T::Summary as Summary>::Context<'_>,
    ) {
    }
}

impl<T: Item> SeekAggregate<'_, T> for SliceSeekAggregate<T> {
    fn begin_leaf(&mut self) {}
    fn end_leaf(&mut self, cx: <T::Summary as Summary>::Context<'_>) {
        self.tree.append(
            SumTree(Arc::new(Node::Leaf {
                summary: mem::replace(&mut self.leaf_summary, <T::Summary as Summary>::zero(cx)),
                items: mem::take(&mut self.leaf_items),
                item_summaries: mem::take(&mut self.leaf_item_summaries),
            })),
            cx,
        );
    }
    fn push_item(
        &mut self,
        item: &T,
        summary: &T::Summary,
        cx: <T::Summary as Summary>::Context<'_>,
    ) {
        self.leaf_items.push(item.clone()).unwrap_oob();
        self.leaf_item_summaries.push(summary.clone()).unwrap_oob();
        Summary::add_summary(&mut self.leaf_summary, summary, cx);
    }
    fn push_tree(
        &mut self,
        tree: &SumTree<T>,
        _: &T::Summary,
        cx: <T::Summary as Summary>::Context<'_>,
    ) {
        self.tree.append(tree.clone(), cx);
    }
}

impl<'a, T: Item, D> SeekAggregate<'a, T> for SummarySeekAggregate<D>
where
    D: Dimension<'a, T::Summary>,
{
    fn begin_leaf(&mut self) {}
    fn end_leaf(&mut self, _: <T::Summary as Summary>::Context<'_>) {}
    fn push_item(
        &mut self,
        _: &T,
        summary: &'a T::Summary,
        cx: <T::Summary as Summary>::Context<'_>,
    ) {
        self.0.add_summary(summary, cx);
    }
    fn push_tree(
        &mut self,
        _: &SumTree<T>,
        summary: &'a T::Summary,
        cx: <T::Summary as Summary>::Context<'_>,
    ) {
        self.0.add_summary(summary, cx);
    }
}

struct End<D>(PhantomData<D>);

impl<D> End<D> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'a, S: Summary, D: Dimension<'a, S>> SeekTarget<'a, S, D> for End<D> {
    fn cmp(&self, _: &D, _: S::Context<'_>) -> Ordering {
        Ordering::Greater
    }
}

impl<D> fmt::Debug for End<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("End").finish()
    }
}
