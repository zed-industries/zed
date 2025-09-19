use std::fmt::Debug;

use ::sum_tree::SumTree;
use collections::FxHashMap;
use sum_tree::Bias;
use util::debug_panic;

use crate::{FocusHandle, FocusId};

/// Represents a collection of focus handles using the tab-index APIs.
#[derive(Debug)]
pub(crate) struct TabIndexMap {
    current_path: TabIndexPath,
    pub(crate) insertion_history: Vec<TabIndexInsertion>,
    by_id: FxHashMap<FocusId, TabIndexNode>,
    order: SumTree<TabIndexNode>,
}

#[derive(Debug)]
pub enum TabIndexInsertion {
    Element(FocusHandle),
    Group(TabIndex),
    GroupEnd,
}

impl TabIndexInsertion {
    fn focus_handle(&self) -> Option<&FocusHandle> {
        match self {
            TabIndexInsertion::Element(focus_handle) => Some(focus_handle),
            _ => None,
        }
    }
}

type TabIndex = isize;

#[derive(Debug, Default, PartialEq, Eq, Clone, Ord, PartialOrd)]
struct TabIndexPath(smallvec::SmallVec<[TabIndex; 6]>);

#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
struct TabIndexNode {
    // Path to access the node in the tree
    // The final node in the list is a leaf node corresponding to an actual focus handle,
    // all other nodes are group nodes
    path: TabIndexPath,
    // index into the backing array of nodes. Corresponds to insertion order
    node_insertion_index: usize,
}

impl Default for TabIndexMap {
    fn default() -> Self {
        Self {
            current_path: TabIndexPath::default(),
            insertion_history: Vec::new(),
            by_id: FxHashMap::default(),
            order: SumTree::new(&()),
        }
    }
}

impl TabIndexMap {
    pub fn insert(&mut self, focus_handle: &FocusHandle) {
        if !focus_handle.tab_stop {
            return;
        }

        self.insertion_history
            .push(TabIndexInsertion::Element(focus_handle.clone()));
        let mut path = self.current_path.clone();
        path.0.push(focus_handle.tab_index);
        let order = TabIndexNode {
            node_insertion_index: self.insertion_history.len() - 1,
            path,
        };
        self.by_id.insert(focus_handle.id, order.clone());
        self.order.insert_or_replace(order, &());
    }

    pub fn begin_group(&mut self, tab_index: isize) {
        self.insertion_history
            .push(TabIndexInsertion::Group(tab_index));
        self.current_path.0.push(tab_index);
    }

    pub fn end_group(&mut self) {
        self.insertion_history.push(TabIndexInsertion::GroupEnd);
        self.current_path.0.pop();
    }

    pub fn clear(&mut self) {
        *self = Self::default();
        self.current_path.0.clear();
        self.insertion_history.clear();
        self.by_id.clear();
        self.order = SumTree::new(&());
    }

    pub fn next(&self, focused_id: Option<&FocusId>) -> Option<FocusHandle> {
        let Some(focused_id) = focused_id else {
            return self
                .order
                .first()
                .and_then(|order| self.focus_handle_for_order(order));
        };

        let path = self.tab_node_for_focus_id(focused_id)?;
        let mut cursor = self.order.cursor::<TabIndexNode>(&());
        cursor.seek(&path, Bias::Left);
        cursor.next();
        cursor
            .item()
            .or_else(|| self.order.first()) // Wrap to the beginning if at the end
            .and_then(|order| self.focus_handle_for_order(order))
    }

    pub fn prev(&self, focused_id: Option<&FocusId>) -> Option<FocusHandle> {
        let Some(focused_id) = focused_id else {
            return self
                .order
                .last()
                .and_then(|order| self.focus_handle_for_order(order));
        };

        let path = self.tab_node_for_focus_id(focused_id)?;
        let mut cursor = self.order.cursor::<TabIndexNode>(&());
        cursor.seek(&path, Bias::Left);
        cursor.prev();
        cursor
            .item()
            .or_else(|| self.order.last()) // Wrap to the end if at the beginning
            .and_then(|order| self.focus_handle_for_order(order))
    }

    pub fn replay(&mut self, nodes: &[TabIndexOperation]) {
        for node in nodes {
            match node {
                TabIndexInsertion::Element(focus_handle) => self.insert(focus_handle),
                TabIndexInsertion::Group(tab_index) => self.begin_group(*tab_index),
                TabIndexInsertion::GroupEnd => self.end_group(),
            }
        }
    }

    pub fn paint_index(&self) -> usize {
        self.insertion_history.len()
    }

    fn focus_handle_for_order(&self, order: &TabIndexNode) -> Option<FocusHandle> {
        let handle = self.insertion_history[order.node_insertion_index].focus_handle();
        debug_assert!(
            handle.is_some(),
            "The order node did not correspond to an element, this is a GPUI bug"
        );
        handle.cloned()
    }

    fn tab_node_for_focus_id(&self, focused_id: &FocusId) -> Option<&TabIndexNode> {
        let Some(order) = self.by_id.get(focused_id) else {
            debug_panic!("The focused ID was not stored in the ID map, this is a GPUI bug");
            return None;
        };
        Some(order)
    }
}

mod sum_tree_impl {
    use sum_tree::SeekTarget;

    use crate::tab_stop::{TabIndexNode, TabIndexPath};

    #[derive(Clone, Debug)]
    pub struct TabOrderNodeSummary {
        max_index: usize,
        max_path: TabIndexPath,
    }

    impl sum_tree::Summary for TabOrderNodeSummary {
        type Context = ();

        fn zero(_cx: &Self::Context) -> Self {
            TabOrderNodeSummary {
                max_index: 0,
                max_path: TabIndexPath::default(),
            }
        }

        fn add_summary(&mut self, summary: &Self, _cx: &Self::Context) {
            *self = summary.clone();
        }
    }

    impl sum_tree::KeyedItem for TabIndexNode {
        type Key = Self;

        fn key(&self) -> Self::Key {
            self.clone()
        }
    }

    impl sum_tree::Item for TabIndexNode {
        type Summary = TabOrderNodeSummary;

        fn summary(&self, _cx: &<Self::Summary as sum_tree::Summary>::Context) -> Self::Summary {
            TabOrderNodeSummary {
                max_index: self.node_insertion_index,
                max_path: self.path.clone(),
            }
        }
    }

    impl<'a> sum_tree::Dimension<'a, TabOrderNodeSummary> for TabIndexNode {
        fn zero(_: &<TabOrderNodeSummary as sum_tree::Summary>::Context) -> Self {
            TabIndexNode::default()
        }

        fn add_summary(
            &mut self,
            summary: &'a TabOrderNodeSummary,
            _: &<TabOrderNodeSummary as sum_tree::Summary>::Context,
        ) {
            self.node_insertion_index = summary.max_index;
            self.path = summary.max_path.clone();
        }
    }

    impl<'a, 'b> SeekTarget<'a, TabOrderNodeSummary, TabIndexNode> for &'b TabIndexNode {
        fn cmp(&self, cursor_location: &TabIndexNode, _: &()) -> std::cmp::Ordering {
            Iterator::cmp(self.path.0.iter(), cursor_location.path.0.iter()).then(
                self.node_insertion_index
                    .cmp(&cursor_location.node_insertion_index),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools as _;

    use crate::{FocusHandle, FocusId, FocusMap, TabIndexMap};
    use std::sync::Arc;

    #[test]
    fn test_tab_handles() {
        let focus_map = Arc::new(FocusMap::default());
        let mut tab_index_map = TabIndexMap::default();

        let focus_handles = vec![
            FocusHandle::new(&focus_map).tab_stop(true).tab_index(0),
            FocusHandle::new(&focus_map).tab_stop(true).tab_index(1),
            FocusHandle::new(&focus_map).tab_stop(true).tab_index(1),
            FocusHandle::new(&focus_map),
            FocusHandle::new(&focus_map).tab_index(2),
            FocusHandle::new(&focus_map).tab_stop(true).tab_index(0),
            FocusHandle::new(&focus_map).tab_stop(true).tab_index(2),
        ];

        for handle in focus_handles.iter() {
            tab_index_map.insert(handle);
        }
        let expected = [
            focus_handles[0].clone(),
            focus_handles[5].clone(),
            focus_handles[1].clone(),
            focus_handles[2].clone(),
            focus_handles[6].clone(),
        ];

        let mut prev = None;
        let mut found = vec![];
        for _ in 0..expected.len() {
            let handle = tab_index_map.next(prev.as_ref()).unwrap();
            prev = Some(handle.id.clone());
            found.push(handle.id);
        }

        assert_eq!(
            found,
            expected
                .iter()
                .map(|handle| handle.id.clone())
                .collect::<Vec<_>>()
        );

        // Select first tab index if no handle is currently focused.
        assert_eq!(tab_index_map.next(None), Some(expected[0].clone()));
        // Select last tab index if no handle is currently focused.
        assert_eq!(tab_index_map.prev(None), expected.last().cloned(),);

        assert_eq!(
            tab_index_map.next(Some(&expected[0].id)),
            Some(expected[1].clone())
        );
        assert_eq!(
            tab_index_map.next(Some(&expected[1].id)),
            Some(expected[2].clone())
        );
        assert_eq!(
            tab_index_map.next(Some(&expected[2].id)),
            Some(expected[3].clone())
        );
        assert_eq!(
            tab_index_map.next(Some(&expected[3].id)),
            Some(expected[4].clone())
        );
        assert_eq!(
            tab_index_map.next(Some(&expected[4].id)),
            Some(expected[0].clone())
        );

        // prev
        assert_eq!(tab_index_map.prev(None), Some(expected[4].clone()));
        assert_eq!(
            tab_index_map.prev(Some(&expected[0].id)),
            Some(expected[4].clone())
        );
        assert_eq!(
            tab_index_map.prev(Some(&expected[1].id)),
            Some(expected[0].clone())
        );
        assert_eq!(
            tab_index_map.prev(Some(&expected[2].id)),
            Some(expected[1].clone())
        );
        assert_eq!(
            tab_index_map.prev(Some(&expected[3].id)),
            Some(expected[2].clone())
        );
        assert_eq!(
            tab_index_map.prev(Some(&expected[4].id)),
            Some(expected[3].clone())
        );
    }

    #[must_use]
    struct TabIndexMapTest {
        tab_map: TabIndexMap,
        focus_map: Arc<FocusMap>,
        expected: Vec<(usize, FocusId)>,
    }

    impl TabIndexMapTest {
        #[must_use]
        fn new() -> Self {
            Self {
                tab_map: TabIndexMap::default(),
                focus_map: Arc::new(FocusMap::default()),
                expected: Vec::default(),
            }
        }

        #[must_use]
        fn tab_non_stop(mut self, index: isize) -> Self {
            let handle = FocusHandle::new(&self.focus_map)
                .tab_stop(false)
                .tab_index(index);
            self.tab_map.insert(&handle);
            self
        }

        #[must_use]
        fn tab_stop(mut self, index: isize, expected: usize) -> Self {
            let handle = FocusHandle::new(&self.focus_map)
                .tab_stop(true)
                .tab_index(index);
            self.tab_map.insert(&handle);
            self.expected.push((expected, handle.id));
            self.expected.sort_by_key(|(expected, _)| *expected);
            self
        }

        #[must_use]
        fn tab_group(mut self, tab_index: isize, children: impl FnOnce(Self) -> Self) -> Self {
            self.tab_map.begin_group(tab_index);
            self = children(self);
            self.tab_map.end_group();
            self
        }

        fn traverse_tab_map(
            &self,
            traverse: impl Fn(&TabIndexMap, Option<&FocusId>) -> Option<FocusHandle>,
        ) -> Vec<FocusId> {
            let mut last_focus_id = None;
            let mut found = vec![];
            for _ in 0..self.expected.len() {
                let handle = traverse(&self.tab_map, last_focus_id.as_ref()).unwrap();
                last_focus_id = Some(handle.id.clone());
                found.push(handle.id);
            }
            found
        }

        fn assert(self) {
            let mut expected = self.expected.iter().map(|(_, id)| id.clone()).collect_vec();

            dbg!(&self.tab_map, &expected);

            // Check next order
            let forward_found = self.traverse_tab_map(|tab_map, prev| tab_map.next(prev).clone());
            assert_eq!(forward_found, expected);

            // Test overflow. Last to first
            assert_eq!(
                self.tab_map
                    .next(forward_found.last())
                    .map(|handle| handle.id),
                expected.first().cloned()
            );

            // Check previous order
            let reversed_found = self.traverse_tab_map(|tab_map, prev| tab_map.prev(prev).clone());
            expected.reverse();
            assert_eq!(reversed_found, expected);

            // Test overflow. First to last
            assert_eq!(
                self.tab_map
                    .prev(reversed_found.last())
                    .map(|handle| handle.id),
                expected.first().cloned(),
            );
        }
    }

    #[test]
    fn test_with_disabled_tab_stop() {
        TabIndexMapTest::new()
            .tab_stop(0, 0)
            .tab_non_stop(1)
            .tab_stop(2, 1)
            .tab_stop(3, 2)
            .assert();
    }

    #[test]
    fn test_with_disabled_tab_stops() {
        TabIndexMapTest::new()
            .tab_non_stop(0)
            .tab_stop(1, 0)
            .tab_non_stop(3)
            .tab_stop(3, 1)
            .tab_non_stop(4)
            .assert();
    }

    #[test]
    fn test_tab_group_functionality() {
        TabIndexMapTest::new()
            .tab_stop(0, 0)
            .tab_stop(0, 1)
            .tab_group(2, |t| t.tab_stop(0, 2).tab_stop(1, 3))
            .tab_stop(3, 4)
            .tab_stop(4, 5)
            .assert()
    }

    #[test]
    fn test_sibling_groups() {
        TabIndexMapTest::new()
            .tab_stop(0, 0)
            .tab_stop(1, 1)
            .tab_group(2, |test| test.tab_stop(0, 2).tab_stop(1, 3))
            .tab_stop(3, 4)
            .tab_stop(4, 5)
            .tab_group(6, |test| test.tab_stop(0, 6).tab_stop(1, 7))
            .tab_stop(7, 8)
            .tab_stop(8, 9)
            .assert();
    }

    #[test]
    fn test_nested_group() {
        TabIndexMapTest::new()
            .tab_stop(0, 0)
            .tab_stop(1, 1)
            .tab_group(2, |t| {
                t.tab_group(0, |t| t.tab_stop(0, 2).tab_stop(1, 3))
                    .tab_stop(1, 4)
            })
            .tab_stop(3, 5)
            .tab_stop(4, 6)
            .assert();
    }

    #[test]
    fn test_sibling_nested_groups() {
        TabIndexMapTest::new()
            .tab_stop(0, 0)
            .tab_stop(1, 1)
            .tab_group(2, |builder| {
                builder
                    .tab_stop(0, 2)
                    .tab_stop(2, 5)
                    .tab_group(1, |builder| builder.tab_stop(0, 3).tab_stop(1, 4))
                    .tab_group(3, |builder| builder.tab_stop(0, 6).tab_stop(1, 7))
            })
            .tab_stop(3, 8)
            .tab_stop(4, 9)
            .assert();
    }

    #[test]
    fn test_sibling_nested_groups_out_of_order() {
        TabIndexMapTest::new()
            .tab_stop(9, 9)
            .tab_stop(8, 8)
            .tab_group(7, |builder| {
                builder
                    .tab_stop(0, 2)
                    .tab_stop(2, 5)
                    .tab_group(3, |builder| builder.tab_stop(1, 7).tab_stop(0, 6))
                    .tab_group(1, |builder| builder.tab_stop(0, 3).tab_stop(1, 4))
            })
            .tab_stop(3, 0)
            .tab_stop(4, 1)
            .assert();
    }
}
