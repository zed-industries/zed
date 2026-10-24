use core::fmt::Debug;

use proptest::{prelude::*, sample::SizeRange};

use crate::{Bias, Item, SumTree, Summary};

impl<T> Arbitrary for SumTree<T>
where
    T: Debug + Arbitrary + Item + 'static,
    T::Summary: Debug + Summary<Context<'static> = ()>,
{
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        any::<Vec<T>>()
            .prop_map(|vec| SumTree::from_iter(vec, ()))
            .boxed()
    }
}

impl Arbitrary for Bias {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        proptest::sample::select(vec![Self::Left, Self::Right]).boxed()
    }
}

/// A strategy for producing a [`SumTree`] with a given size.
///
/// Equivalent to [`proptest::collection::vec`].
pub fn sum_tree<S, T>(values: S, size: impl Into<SizeRange>) -> impl Strategy<Value = SumTree<T>>
where
    T: Debug + Arbitrary + Item + 'static,
    T::Summary: Debug + Summary<Context<'static> = ()>,
    S: Strategy<Value = T>,
{
    proptest::collection::vec(values, size).prop_map(|vec| SumTree::from_iter(vec, ()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cursor, tests::Sum};
    use proptest::sample::Index;

    #[proptest::property_test]
    fn test_cursor_sum_pbt(
        #[strategy = initial_items()] initial_items: Vec<u8>,
        #[strategy = operations()] operations: Vec<Operation>,
    ) {
        let tree = SumTree::from_iter(initial_items.iter().copied(), ());
        let cursor_sum_tree = tree.cursor::<Sum>(());

        let cursor_reference = CursorReference::from_tree_items(initial_items);
        let mut model = CursorModel {
            cursor_sum_tree,
            cursor_reference,
        };
        model.check_with_reference();
        for operation in operations {
            model.apply(operation);
            model.check_with_reference();
        }
    }

    // A reference model implemented backed by a `Vec`, and using a `Sum`
    // dimension.
    //
    // sum           0       2           5   6                   11
    // items         |---2---|-----3-----|-1-|---------5---------|
    //
    // Using "sum" (rather than "count") as a dimension allows items to have
    // variable dimension-width, which makes for more interesting cases.
    //
    // Although it makes the testing a little more complicated, the shrinking of
    // a failing test case will bias towards generating items with value of one,
    // making sum behave exactly as count if possible.
    #[derive(Default, Debug, Clone)]
    struct CursorReference {
        // All the items in the tree, regardless of where this cursor is
        // positioned.
        items: Vec<u8>,
        start_sum: usize,
        end_sum: usize,
        attached_item: Option<AttachedItem>,
        // Mimics the current implementation of cursor that requires a form of
        // seeking to be called before some methods are allowed.
        did_seek: bool,
        at_end: bool,
    }

    #[derive(Debug, Clone, Copy)]
    struct AttachedItem {
        value: u8,
        index: usize,
    }

    impl CursorReference {
        fn from_tree_items(items: Vec<u8>) -> Self {
            Self {
                at_end: items.is_empty(),
                items,
                ..Default::default()
            }
        }

        fn reset(&mut self) {
            self.did_seek = false;
            self.at_end = self.is_empty();
            self.attached_item = None;
            self.start_sum = 0;
            self.end_sum = 0;
        }

        fn is_empty(&self) -> bool {
            self.items.is_empty()
        }

        fn len(&self) -> usize {
            self.items.len()
        }

        fn item_at(&self, index: usize) -> Option<AttachedItem> {
            if index < self.len() {
                Some(AttachedItem {
                    value: self.items[index],
                    index,
                })
            } else {
                None
            }
        }
    }

    impl CursorReference {
        fn seek(&mut self, target_sum: usize, bias: Bias) -> bool {
            self.reset();

            self.did_seek = true;

            // If the tree is empty, the return value depends on whether we've
            // seeked exactly to zero (because the return value of seek is
            // related to the dimension, and not with items).
            if self.is_empty() {
                return target_sum == 0;
            }

            // Move through all items and stop when appropriate.
            let mut cumulative_sum = 0;
            for (index, &item) in self.items.iter().enumerate() {
                let with_item = cumulative_sum + item as usize;
                let should_stop = match bias {
                    Bias::Left => with_item >= target_sum,
                    Bias::Right => with_item > target_sum,
                };
                if should_stop {
                    self.start_sum = cumulative_sum;
                    self.end_sum = with_item;
                    self.attached_item = self.item_at(index);

                    return match bias {
                        Bias::Left => self.end_sum == target_sum,
                        Bias::Right => self.start_sum == target_sum,
                    };
                }

                cumulative_sum = with_item;
            }

            // Otherwise, we've exhausted all items (but might still return
            // true, to comply with `seek()`).
            self.start_sum = cumulative_sum;
            self.end_sum = cumulative_sum;
            self.at_end = true;

            target_sum == cumulative_sum
        }

        fn next(&mut self) {
            self.did_seek = true;

            if self.at_end {
                return;
            }

            // Because we've checked if we're at the end, we know that a `None`
            // means a "before first item" state, and the next index to be
            // considered should be the first item's.
            let next_index = match self.attached_item {
                Some(item) => item.index + 1,
                None => 0,
            };

            // Now we move to the next item.
            self.start_sum = self.end_sum;
            self.attached_item = self.item_at(next_index);

            // And handle the case where we're at the end now.
            match self.attached_item {
                Some(item) => self.end_sum = self.start_sum + usize::from(item.value),
                None => {
                    self.end_sum = self.start_sum;
                    self.at_end = true;
                }
            }
        }

        fn prev(&mut self) {
            // prev() in an unpositioned cursor moves it to the last item of the
            // tree, so here we must identify that case and act accordingly.
            let previous_index = if !self.did_seek || self.at_end {
                let total_sum = self.items.iter().map(|&item| usize::from(item)).sum();
                self.start_sum = total_sum;
                self.len().checked_sub(1)
            } else {
                self.attached_item
                    .and_then(|item| item.index.checked_sub(1))
            };
            self.did_seek = true;

            self.end_sum = self.start_sum;
            self.attached_item = previous_index.and_then(|index| self.item_at(index));

            match self.attached_item {
                Some(item) => {
                    self.start_sum = self.end_sum - usize::from(item.value);
                    self.at_end = false;
                }
                None => {
                    self.start_sum = 0;
                    self.end_sum = 0;
                    // If we're "before the first item" but the tree
                    // is also empty, we're at the end as well.
                    self.at_end = self.is_empty();
                }
            }
        }

        fn item(&self) -> Option<u8> {
            self.attached_item.map(|item| item.value)
        }

        fn prev_item(&self) -> Option<u8> {
            let mut other = self.clone();
            other.prev();
            other.item()
        }

        fn next_item(&self) -> Option<u8> {
            let mut other = self.clone();
            other.next();
            other.item()
        }

        fn slice(&mut self, count: usize, bias: Bias) -> Vec<u8> {
            self.did_seek = true;

            let mut collected = vec![];

            // In this special state, the `None` is simply because we're before
            // the start of the tree.
            let before_first_item =
                self.start_sum == 0 && self.end_sum == 0 && self.attached_item.is_none();
            if before_first_item {
                // So any slice should start by considering the first item.
                self.next();
            }

            loop {
                let Some(current_item) = self.attached_item else {
                    return collected;
                };

                let with_item = self.start_sum + usize::from(current_item.value);
                if with_item < count || (with_item == count && bias == Bias::Right) {
                    collected.push(current_item.value);
                    self.next();
                } else {
                    break;
                }
            }

            collected
        }
    }

    struct CursorModel<'a> {
        cursor_sum_tree: Cursor<'a, 'a, u8, Sum>,
        cursor_reference: CursorReference,
    }

    // Targets past the extent of the tree are not super relevant because they
    // should produce the same cursor state and return value.
    #[derive(Debug, Clone, Copy, proptest_derive::Arbitrary)]
    enum Target {
        #[proptest(weight = 4)]
        WithinTree(Index),
        #[proptest(weight = 2)]
        ExactlyAtExtent,
        #[proptest(weight = 1)]
        PastExtent(Index),
    }

    impl Target {
        fn resolve(self, minimum: usize, total_sum: usize) -> usize {
            match self {
                Self::WithinTree(index) => minimum + index.index(total_sum - minimum + 1),
                Self::ExactlyAtExtent => total_sum,
                Self::PastExtent(index) => total_sum + 1 + index.index(usize::MAX - total_sum),
            }
        }
    }

    #[derive(Debug, Clone, Copy, proptest_derive::Arbitrary)]
    enum Operation {
        Seek { target: Target, bias: Bias },
        Slice { target: Target, bias: Bias },
        Next,
        Prev,
    }

    impl CursorModel<'_> {
        fn apply(&mut self, operation: Operation) {
            let total_sum = self
                .cursor_reference
                .items
                .iter()
                .map(|&item| usize::from(item))
                .sum();

            match operation {
                Operation::Seek { target, bias } => {
                    let sum = target.resolve(0, total_sum);
                    let found = self.cursor_sum_tree.seek(&Sum(sum), bias);
                    let reference_found = self.cursor_reference.seek(sum, bias);
                    assert_eq!(
                        found, reference_found,
                        "return value from seek did not match reference"
                    );
                }
                Operation::Slice { target, bias } => {
                    // Backwards slicing is not allowed, so we resolve the
                    // target considering the known minimum valid position.
                    let sum = target.resolve(self.cursor_reference.start_sum, total_sum);
                    let returned_items = self.cursor_sum_tree.slice(&Sum(sum), bias).items(());
                    let reference_items = self.cursor_reference.slice(sum, bias);
                    assert_eq!(
                        returned_items, reference_items,
                        "returned items by slice did not match reference"
                    );
                }
                Operation::Next => {
                    self.cursor_sum_tree.next();
                    self.cursor_reference.next();
                }
                Operation::Prev => {
                    self.cursor_sum_tree.prev();
                    self.cursor_reference.prev();
                }
            }
        }

        fn check_with_reference(&self) {
            assert_eq!(
                self.cursor_sum_tree.did_seek(),
                self.cursor_reference.did_seek,
                "did seek state should match"
            );
            assert_eq!(
                self.cursor_sum_tree.at_end(),
                self.cursor_reference.at_end,
                "cursor at end state should match"
            );

            // Some methods are only allowed after seek-related calls.
            if !self.cursor_reference.did_seek {
                return;
            }

            assert_eq!(
                self.cursor_sum_tree.start().0,
                self.cursor_reference.start_sum,
                "cursor start should match"
            );
            assert_eq!(
                self.cursor_sum_tree.end().0,
                self.cursor_reference.end_sum,
                "cursor end should match"
            );

            assert_eq!(
                self.cursor_sum_tree.item(),
                self.cursor_reference.item().as_ref(),
                "cursor attached item should match"
            );
            assert_eq!(
                self.cursor_sum_tree.prev_item(),
                self.cursor_reference.prev_item().as_ref(),
                "cursor prev item should match"
            );
            assert_eq!(
                self.cursor_sum_tree.next_item(),
                self.cursor_reference.next_item().as_ref(),
                "cursor next item should match"
            );
        }
    }

    fn operations() -> impl Strategy<Value = Vec<Operation>> {
        proptest::collection::vec(any::<Operation>(), 0..=128)
    }

    fn initial_items() -> impl Strategy<Value = Vec<u8>> {
        // Ones are interesting because they're simple. Each value will have an
        // a dimension-width of one, and "sum" will behave as the simpler
        // "count", making debugging easier.
        //
        // Zeros are interesting because items with zero dimension-width can
        // trigger edge cases. Note that zero-width items are actually used, in
        // cases such as for text that has been deleted and should not be
        // visible.
        proptest::collection::vec(
            prop_oneof![2 => Just(1u8), 2 => Just(0u8), 1 => any::<u8>()],
            0..=1024,
        )
    }
}
