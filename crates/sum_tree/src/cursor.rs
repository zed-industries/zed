use super::*;
use arrayvec::ArrayVec;
use std::{cmp::Ordering, mem, sync::Arc};
use ztracing::instrument;

#[derive(Clone)]
struct StackEntry<'a, T: Item, D> {
    tree: &'a SumTree<T>,
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
    stack: ArrayVec<StackEntry<'a, T, D>, 16>,
    position: D,
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
    stack: ArrayVec<StackEntry<'a, T, ()>, 16>,
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

    fn reset(&mut self) {
        self.did_seek = false;
        self.at_end = self.tree.is_empty();
        self.stack.truncate(0);
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
                self.stack.push(StackEntry {
                    tree: self.tree,
                    index: self.tree.0.child_summaries().len() as u32,
                    position: D::from_summary(self.tree.summary(), self.cx),
                });
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
                        self.stack.push(StackEntry {
                            position: D::zero(self.cx),
                            tree,
                            index: tree.0.child_summaries().len() as u32 - 1,
                        })
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
                self.stack.push(StackEntry {
                    tree: self.tree,
                    index: 0,
                    position: D::zero(self.cx),
                });
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

        if !self.did_seek {
            self.did_seek = true;
            self.stack.push(StackEntry {
                tree: self.tree,
                index: 0,
                position: D::zero(self.cx),
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
                        entry.position = self.position.clone();
                    }

                    for (child_tree, child_summary) in child_trees[entry.index()..]
                        .iter()
                        .zip(&child_summaries[entry.index()..])
                    {
                        let mut child_end = self.position.clone();
                        child_end.add_summary(child_summary, self.cx);

                        let comparison = target.cmp(&child_end, self.cx);
                        if comparison == Ordering::Greater
                            || (comparison == Ordering::Equal && bias == Bias::Right)
                        {
                            self.position = child_end;
                            aggregate.push_tree(child_tree, child_summary, self.cx);
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

                    for (item, item_summary) in items[entry.index()..]
                        .iter()
                        .zip(&item_summaries[entry.index()..])
                    {
                        let mut child_end = self.position.clone();
                        child_end.add_summary(item_summary, self.cx);

                        let comparison = target.cmp(&child_end, self.cx);
                        if comparison == Ordering::Greater
                            || (comparison == Ordering::Equal && bias == Bias::Right)
                        {
                            self.position = child_end;
                            aggregate.push_item(item, item_summary, self.cx);
                            entry.index += 1;
                        } else {
                            aggregate.end_leaf(self.cx);
                            break 'outer;
                        }
                    }

                    aggregate.end_leaf(self.cx);
                }
            }

            self.stack.pop();
            ascending = true;
        }

        self.at_end = self.stack.is_empty();
        debug_assert!(self.stack.is_empty() || self.stack.last().unwrap().tree.0.is_leaf());

        let mut end = self.position.clone();
        if bias == Bias::Left
            && let Some(summary) = self.item_summary()
        {
            end.add_summary(summary, self.cx);
        }

        target.cmp(&end, self.cx) == Ordering::Equal
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

        while !self.stack.is_empty() {
            let new_subtree = {
                let entry = self.stack.last_mut().unwrap();
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
    leaf_items: ArrayVec<T, { 2 * TREE_BASE }>,
    leaf_item_summaries: ArrayVec<T::Summary, { 2 * TREE_BASE }>,
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
        self.leaf_items.push(item.clone());
        self.leaf_item_summaries.push(summary.clone());
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
