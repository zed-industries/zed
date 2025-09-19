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

    pub fn reuse_paint(&mut self, nodes: &[TabIndexInsertion]) {
        for node in nodes {
            match node {
                TabIndexInsertion::Element(focus_handle) => self.insert(focus_handle),
                TabIndexInsertion::Group(tab_index) => self.begin_group(*tab_index),
                TabIndexInsertion::GroupEnd => self.end_group(),
            }
        }
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
    use crate::{FocusHandle, FocusMap, TabIndexMap};
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

    /// Helper function to parse XML-like structure and test tab navigation
    ///
    /// The XML structure should define elements with tab-index and actual (expected order) values.
    ///
    /// Currently only supports flat elements:
    /// ```
    /// <tab-index=0 actual=0>
    /// <tab-index=1 actual=1>
    /// <tab-index=2 actual=2>
    /// ```
    ///
    /// Future support (not yet implemented) for nested structures:
    /// ```
    /// <tab-group tab-index=2>
    ///     <tab-index=0 actual=3>  // Would be at position 2.0
    ///     <tab-index=1 actual=4>  // Would be at position 2.1
    /// </tab-group>
    /// ```
    fn check(xml: &str) {
        use std::collections::HashMap;

        // Tree node structure with common fields
        #[derive(Debug, Clone)]
        struct TreeNode {
            xml_tag: String,
            handle: Option<FocusHandle>,
            node_type: NodeType,
        }

        // Node type variants
        #[derive(Debug, Clone)]
        enum NodeType {
            TabStop {
                tab_index: isize,
                actual: usize, // Required for tab stops
            },
            NonTabStop {
                tab_index: Option<isize>,
                // No actual field - these aren't in the tab order
            },
            Group {
                tab_index: isize,
                children: Vec<TreeNode>,
            },
            FocusTrap {
                children: Vec<TreeNode>,
            },
        }

        // Phase 1: Parse - Build tree structure from XML
        fn parse(xml: &str) -> TreeNode {
            let mut root = TreeNode {
                xml_tag: "root".to_string(),
                handle: None,
                node_type: NodeType::Group {
                    tab_index: isize::MIN,
                    children: Vec::new(),
                },
            };

            let mut stack: Vec<TreeNode> = vec![root.clone()];

            for line in xml.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                // Handle closing tags
                if line.starts_with("</") {
                    let tag_name = line.trim_start_matches("</").trim_end_matches('>').trim();

                    if stack.len() > 1 {
                        let completed = stack.pop().unwrap();
                        let parent = stack.last_mut().unwrap();

                        // Verify tag matches
                        if completed.xml_tag != tag_name && !completed.xml_tag.starts_with(tag_name)
                        {
                            panic!(
                                "Mismatched closing tag: expected {}, got {}",
                                completed.xml_tag, tag_name
                            );
                        }

                        match &mut parent.node_type {
                            NodeType::Group { children, .. }
                            | NodeType::FocusTrap { children, .. } => {
                                children.push(completed);
                            }
                            _ => panic!("Tried to add child to non-container node"),
                        }
                    }
                    continue;
                }

                // Handle opening tags
                if line.starts_with('<') {
                    let content = line.trim_start_matches('<').trim_end_matches('>');
                    let parts: Vec<&str> = content.split_whitespace().collect();

                    if parts.is_empty() {
                        continue;
                    }

                    let mut tab_index: Option<isize> = None;
                    let mut actual: Option<usize> = None;
                    let mut tab_stop: Option<bool> = None;

                    // Determine element type
                    let first_part = parts[0];
                    let element_type = if first_part.starts_with("tab-index=") {
                        "element".to_string()
                    } else if !first_part.contains('=') {
                        first_part.to_string()
                    } else {
                        "element".to_string()
                    };

                    // Parse attributes
                    for part in parts {
                        if let Some(eq_pos) = part.find('=') {
                            let key = &part[..eq_pos];
                            let value = &part[eq_pos + 1..];

                            match key {
                                "tab-index" => {
                                    tab_index = value.parse::<isize>().ok();
                                }
                                "actual" => {
                                    actual = value.parse::<usize>().ok();
                                }
                                "tab-stop" => {
                                    tab_stop = value.parse::<bool>().ok();
                                }
                                _ => {}
                            }
                        }
                    }

                    // Create node based on type
                    let node_type = match element_type.as_str() {
                        "tab-group" => {
                            if actual.is_some() {
                                panic!("tab-group elements should not have 'actual' attribute");
                            }
                            let tab_index = tab_index
                                .expect("tab-group elements should have 'tab-index' attribute");
                            NodeType::Group {
                                tab_index,
                                children: Vec::new(),
                            }
                        }
                        "focus-trap" => {
                            if actual.is_some() {
                                panic!("focus-trap elements should not have 'actual' attribute");
                            }
                            NodeType::FocusTrap {
                                children: Vec::new(),
                            }
                        }
                        _ => {
                            // Determine if it's a tab stop based on tab-stop attribute
                            let is_tab_stop = tab_stop.unwrap_or(true);

                            if is_tab_stop {
                                // Tab stops must have an actual value
                                let tab_index =
                                    tab_index.expect("Tab stop must have a 'tab-index' attribute");
                                let actual = actual.expect(&format!(
                                    "Tab stop with tab-index={} must have an 'actual' attribute",
                                    tab_index
                                ));
                                NodeType::TabStop { tab_index, actual }
                            } else {
                                // Non-tab stops should not have an actual value
                                if actual.is_some() {
                                    panic!(
                                        "Non-tab stop (tab-stop=false) should not have an 'actual' attribute"
                                    );
                                }
                                NodeType::NonTabStop { tab_index }
                            }
                        }
                    };

                    let node = TreeNode {
                        xml_tag: element_type.clone(),
                        handle: None,
                        node_type,
                    };

                    // Check if this is a self-closing tag or container
                    let is_container = matches!(element_type.as_str(), "tab-group" | "focus-trap");

                    if is_container {
                        stack.push(node);
                    } else {
                        // Self-closing element, add directly to parent
                        let parent = stack.last_mut().unwrap();
                        match &mut parent.node_type {
                            NodeType::Group { children, .. }
                            | NodeType::FocusTrap { children, .. } => {
                                children.push(node);
                            }
                            _ => panic!("Tried to add child to non-container node"),
                        }
                    }
                }
            }

            // Return the root's children wrapped in the root
            stack.into_iter().next().unwrap()
        }

        // Phase 2: Construct - Build TabHandles from tree
        fn construct(
            node: &mut TreeNode,
            focus_map: &Arc<FocusMap>,
            tab_handles: &mut TabIndexMap,
        ) -> HashMap<usize, FocusHandle> {
            let mut actual_to_handle = HashMap::new();

            fn construct_recursive(
                node: &mut TreeNode,
                focus_map: &Arc<FocusMap>,
                tab_handles: &mut TabIndexMap,
                actual_to_handle: &mut HashMap<usize, FocusHandle>,
            ) {
                match &mut node.node_type {
                    NodeType::TabStop { tab_index, actual } => {
                        let mut handle = FocusHandle::new(focus_map);

                        if *tab_index != isize::MIN {
                            handle = handle.tab_index(*tab_index);
                        }

                        handle = handle.tab_stop(true);
                        tab_handles.insert(&handle);

                        if actual_to_handle.insert(*actual, handle.clone()).is_some() {
                            panic!("Duplicate actual value: {}", actual);
                        }

                        node.handle = Some(handle);
                    }
                    NodeType::NonTabStop { tab_index } => {
                        let mut handle = FocusHandle::new(focus_map);

                        if let Some(idx) = tab_index {
                            handle = handle.tab_index(*idx);
                        }

                        handle = handle.tab_stop(false);
                        tab_handles.insert(&handle);

                        node.handle = Some(handle);
                    }
                    NodeType::Group {
                        children,
                        tab_index,
                    } => {
                        // For now, just process children without special group handling
                        tab_handles.begin_group(*tab_index);
                        for child in children {
                            construct_recursive(child, focus_map, tab_handles, actual_to_handle);
                        }
                        tab_handles.end_group();
                    }
                    NodeType::FocusTrap { children, .. } => {
                        // TODO: Implement focus trap behavior
                        // Focus traps should create a closed navigation loop where:
                        // 1. Tab navigation within the trap cycles only between trap elements
                        // 2. The last element in the trap should navigate to the first element in the trap
                        // 3. The first element in the trap should navigate back to the last element in the trap
                        // 4. Elements outside the trap should not be reachable from within the trap
                        //
                        // This will require modifying TabHandles to support constrained navigation contexts
                        // or implementing a separate mechanism to override next/prev behavior for trapped elements.
                        //
                        // For now, just process children without special trap handling
                        for child in children {
                            construct_recursive(child, focus_map, tab_handles, actual_to_handle);
                        }
                    }
                }
            }

            construct_recursive(node, focus_map, tab_handles, &mut actual_to_handle);
            actual_to_handle
        }

        // Phase 3: Eval - Verify TabHandles matches expected tree traversal
        fn eval(
            tree: &TreeNode,
            tab_handles: &TabIndexMap,
            actual_to_handle: &HashMap<usize, FocusHandle>,
        ) {
            use crate::FocusId;
            // Build an array of handles sorted by their actual values
            // First, find the max actual value to size our array
            let max_actual = actual_to_handle.keys().max().copied().unwrap_or(0);

            // Create an array of Option<FocusHandle> indexed by actual value
            let mut handles_by_actual: Vec<Option<FocusHandle>> = vec![None; max_actual + 1];

            // Insert each handle at its actual position
            for (actual, handle) in actual_to_handle.iter() {
                if *actual > max_actual {
                    panic!("Actual value {} exceeds maximum {}", actual, max_actual);
                }
                handles_by_actual[*actual] = Some(handle.clone());
            }

            // Check for holes (None values) in the array
            for (index, handle_opt) in handles_by_actual.iter().enumerate() {
                if handle_opt.is_none() {
                    panic!(
                        "Missing handle at actual={} position. Expected sequential actual values from 0 to {}",
                        index, max_actual
                    );
                }
            }

            // Convert to Vec<FocusHandle> now that we know there are no holes
            let expected_order: Vec<FocusHandle> = handles_by_actual
                .into_iter()
                .map(|opt| opt.expect("Already checked for None values"))
                .collect();

            // If there are no handles, nothing to test
            if expected_order.is_empty() {
                return;
            }

            // Now verify that tab_handles.next() and .prev() produce the expected navigation order
            for (index, handle) in expected_order.iter().enumerate() {
                let current_id = handle.id;

                // Calculate expected next and prev indices with wrapping
                let next_index = (index + 1) % expected_order.len();
                let prev_index = if index == 0 {
                    expected_order.len() - 1
                } else {
                    index - 1
                };

                let expected_next = &expected_order[next_index];
                let expected_prev = &expected_order[prev_index];

                // Test next navigation
                let actual_next = tab_handles.next(Some(&current_id));
                check_navigation(
                    tree,
                    &format!(
                        "Tab navigation error at position {} (testing next())",
                        index
                    ),
                    current_id,
                    actual_next.as_ref().map(|h| h.id),
                    expected_next.id,
                    tab_handles,
                    actual_to_handle,
                );

                // Test prev navigation
                let actual_prev = tab_handles.prev(Some(&current_id));
                check_navigation(
                    tree,
                    &format!(
                        "Tab navigation error at position {} (testing prev())",
                        index
                    ),
                    current_id,
                    actual_prev.as_ref().map(|h| h.id),
                    expected_prev.id,
                    tab_handles,
                    actual_to_handle,
                );
            }

            // Also test navigation from None (no current focus)
            if !expected_order.is_empty() {
                let first_handle = &expected_order[0];
                let last_handle = expected_order.last().unwrap();

                let actual_next_from_none = tab_handles.next(None);
                check_navigation(
                    tree,
                    "Expected next(None) to return first handle",
                    first_handle.id, // Use first handle ID as "current" for display purposes
                    actual_next_from_none.as_ref().map(|h| h.id),
                    first_handle.id,
                    tab_handles,
                    actual_to_handle,
                );

                let actual_prev_from_none = tab_handles.prev(None);
                check_navigation(
                    tree,
                    "Expected prev(None) to return last handle",
                    last_handle.id, // Use last handle ID as "current" for display purposes
                    actual_prev_from_none.as_ref().map(|h| h.id),
                    last_handle.id,
                    tab_handles,
                    actual_to_handle,
                );
            }

            // Helper function to check navigation and panic with formatted error if it doesn't match
            fn check_navigation(
                tree: &TreeNode,
                error_label: &str,
                current_id: FocusId,
                actual_id: Option<FocusId>,
                expected_id: FocusId,
                _tab_handles: &TabIndexMap,
                actual_to_handle: &HashMap<usize, FocusHandle>,
            ) {
                if actual_id != Some(expected_id) {
                    // Find actual values for the handles
                    let actual_position = actual_id.and_then(|id| {
                        actual_to_handle
                            .iter()
                            .find(|(_, handle)| handle.id == id)
                            .map(|(actual, _)| *actual)
                    });
                    let expected_position = actual_to_handle
                        .iter()
                        .find(|(_, handle)| handle.id == expected_id)
                        .map(|(actual, _)| *actual)
                        .unwrap_or(999);

                    panic!(
                        "Tab navigation error!\n\n{}\n\n{}\n\nExpected: actual={}\nActual: {:?}",
                        error_label,
                        format_tree_with_navigation(
                            tree,
                            current_id,
                            actual_id,
                            expected_id,
                            actual_to_handle
                        ),
                        expected_position,
                        actual_position.map_or("None".to_string(), |p| format!("actual={}", p))
                    );
                }
            }

            // Helper function to format tree with navigation annotations
            fn format_tree_with_navigation(
                node: &TreeNode,
                current_id: FocusId,
                went_to_id: Option<FocusId>,
                expected_id: FocusId,
                actual_to_handle: &HashMap<usize, FocusHandle>,
            ) -> String {
                let mut result = String::new();

                fn format_node_with_nav(
                    node: &TreeNode,
                    indent: usize,
                    current_id: FocusId,
                    went_to_id: Option<FocusId>,
                    expected_id: FocusId,
                    actual_to_handle: &HashMap<usize, FocusHandle>,
                ) -> String {
                    let mut result = String::new();
                    let indent_str = "  ".repeat(indent);

                    match &node.node_type {
                        NodeType::TabStop { tab_index, actual } => {
                            let actual_str = format!(" actual={}", actual);

                            // Add navigation annotations
                            let nav_comment = if let Some(handle) = &node.handle {
                                if handle.id == current_id && current_id != expected_id {
                                    " // <- Current position".to_string()
                                } else if Some(handle.id) == went_to_id {
                                    let actual_val = actual_to_handle
                                        .iter()
                                        .find(|(_, h)| h.id == handle.id)
                                        .map(|(a, _)| *a)
                                        .unwrap_or(999);
                                    format!(" // <- Actually went here (actual={})", actual_val)
                                } else if handle.id == expected_id {
                                    let expected_val = actual_to_handle
                                        .iter()
                                        .find(|(_, h)| h.id == expected_id)
                                        .map(|(a, _)| *a)
                                        .unwrap_or(999);
                                    if went_to_id.is_none() {
                                        format!(
                                            " // <- Expected to go here (actual={}) but got None",
                                            expected_val
                                        )
                                    } else {
                                        format!(
                                            " // <- Expected to go here (actual={})",
                                            expected_val
                                        )
                                    }
                                } else {
                                    "".to_string()
                                }
                            } else {
                                "".to_string()
                            };

                            result.push_str(&format!(
                                "{}<tab-index={}{}>{}\n",
                                indent_str, tab_index, actual_str, nav_comment
                            ));
                        }
                        NodeType::NonTabStop { tab_index } => {
                            result.push_str(&format!(
                                "{}<tab-index={} tab-stop=false>\n",
                                indent_str,
                                tab_index.map_or("None".to_string(), |v| v.to_string()),
                            ));
                        }
                        NodeType::Group {
                            tab_index,
                            children,
                        } => {
                            result.push_str(&format!(
                                "{}<tab-group tab-index={}>\n",
                                indent_str, tab_index
                            ));
                            for child in children {
                                result.push_str(&format_node_with_nav(
                                    child,
                                    indent + 1,
                                    current_id,
                                    went_to_id,
                                    expected_id,
                                    actual_to_handle,
                                ));
                            }
                            result.push_str(&format!("{}</tab-group>\n", indent_str));
                        }
                        NodeType::FocusTrap { children } => {
                            result.push_str(&format!("{}<focus-trap>\n", indent_str));
                            for child in children {
                                result.push_str(&format_node_with_nav(
                                    child,
                                    indent + 1,
                                    current_id,
                                    went_to_id,
                                    expected_id,
                                    actual_to_handle,
                                ));
                            }
                            result.push_str(&format!("{}</focus-trap>\n", indent_str));
                        }
                    }

                    result
                }

                // Skip the root node and format its children
                if let NodeType::Group { children, .. } = &node.node_type {
                    for child in children {
                        result.push_str(&format_node_with_nav(
                            child,
                            0,
                            current_id,
                            went_to_id,
                            expected_id,
                            actual_to_handle,
                        ));
                    }
                }

                result
            }

            // Helper function to format tree structure as XML for error messages
            #[allow(dead_code)]
            fn format_tree_structure(node: &TreeNode, tab_handles: &TabIndexMap) -> String {
                let mut result = String::new();

                fn format_node(
                    node: &TreeNode,
                    tab_handles: &TabIndexMap,
                    indent: usize,
                ) -> String {
                    let mut result = String::new();
                    let indent_str = "  ".repeat(indent);

                    match &node.node_type {
                        NodeType::TabStop {
                            tab_index,
                            actual: _,
                        } => {
                            // let actual = node
                            //     .handle
                            //     .as_ref()
                            //     .and_then(|handle| tab_handles.current_index(&handle.id))
                            //     .unwrap_or(*actual);
                            // todo! get actual index
                            let actual = "?";
                            let actual_str = format!(" actual={}", actual);

                            result.push_str(&format!(
                                "{}<tab-index={}{}>\n",
                                indent_str, tab_index, actual_str
                            ));
                        }
                        NodeType::NonTabStop { tab_index } => {
                            result.push_str(&format!(
                                "{}<tab-index={} tab-stop=false>\n",
                                indent_str,
                                tab_index.map_or("None".to_string(), |v| v.to_string())
                            ));
                        }
                        NodeType::Group {
                            tab_index,
                            children,
                        } => {
                            result.push_str(&format!(
                                "{}<tab-group tab-index={}>\n",
                                indent_str, tab_index
                            ));
                            for child in children {
                                result.push_str(&format_node(child, tab_handles, indent + 1));
                            }
                            result.push_str(&format!("{}</tab-group>\n", indent_str));
                        }
                        NodeType::FocusTrap { children } => {
                            result.push_str(&format!("{}<focus-trap>\n", indent_str));
                            for child in children {
                                result.push_str(&format_node(child, tab_handles, indent + 1));
                            }
                            result.push_str(&format!("{}</focus-trap>\n", indent_str));
                        }
                    }

                    result
                }

                // Skip the root node and format its children
                if let NodeType::Group { children, .. } = &node.node_type {
                    for child in children {
                        result.push_str(&format_node(child, tab_handles, 0));
                    }
                }

                result
            }
        }

        // Main execution
        let focus_map = Arc::new(FocusMap::default());
        let mut tab_handles = TabIndexMap::default();

        // Phase 1: Parse
        let mut tree = parse(xml);

        // Phase 2: Construct
        let actual_to_handle = construct(&mut tree, &focus_map, &mut tab_handles);

        // Phase 3: Eval
        eval(&tree, &tab_handles, &actual_to_handle);
    }

    macro_rules! xml_test {
        ($test_name:ident, $xml:expr) => {
            #[test]
            fn $test_name() {
                let xml = $xml;
                check(xml);
            }
        };
    }

    mod test_helper {
        // use super::*;

        // xml_test!(
        //     test_simple_ordering,
        //     r#"
        //         <tab-index=0 actual=0>
        //         <tab-index=1 actual=1>
        //         <tab-index=2 actual=2>
        //     "#
        // );

        // xml_test!(
        //     test_duplicate_indices_maintain_insertion_order,
        //     r#"
        //         <tab-index=0 actual=0>
        //         <tab-index=0 actual=1>
        //         <tab-index=1 actual=2>
        //         <tab-index=1 actual=3>
        //         <tab-index=2 actual=4>
        //     "#
        // );

        // xml_test!(
        //     test_positive_and_negative_indices,
        //     r#"
        //         <tab-index=1 actual=2>
        //         <tab-index=-1 actual=0>
        //         <tab-index=0 actual=1>
        //         <tab-index=2 actual=3>
        //     "#
        // );

        // #[test]
        // #[should_panic(
        //     expected = "Non-tab stop (tab-stop=false) should not have an 'actual' attribute"
        // )]
        // fn test_non_tab_stop_with_actual_panics() {
        //     let xml = r#"
        //         <tab-index=0 actual=0>
        //         <tab-index=1 tab-stop=false actual=1>
        //         <tab-index=2 actual=2>
        //     "#;
        //     check(xml);
        // }

        // #[test]
        // #[should_panic(expected = "Tab stop with tab-index=1 must have an 'actual' attribute")]
        // fn test_tab_stop_without_actual_panics() {
        //     // Tab stops must have an actual value
        //     let xml = r#"
        //         <tab-index=0 actual=0>
        //         <tab-index=1>
        //         <tab-index=2 actual=2>
        //     "#;
        //     check(xml);
        // }

        // #[test]
        // #[should_panic(expected = "Tab navigation error at position")]
        // fn test_incorrect_tab_order_shows_xml_format() {
        //     // This test intentionally has wrong expected order to demonstrate error reporting
        //     // The actual tab order will be: tab-index=-1, 0, 1, 2 (positions 0, 1, 2, 3)
        //     // But we're expecting them at wrong positions
        //     let xml = r#"
        //         <tab-index=0 actual=0>
        //         <tab-index=-1 actual=1>
        //         <tab-index=2 actual=2>
        //         <tab-index=1 actual=3>
        //     "#;
        //     check(xml);
        // }
    }

    mod basic {
        use super::*;

        xml_test!(
            test_with_disabled_tab_stop,
            r#"
            <tab-index=0 actual=0>
            <tab-index=1 tab-stop=false>
            <tab-index=2 actual=1>
            <tab-index=3 actual=2>
            "#
        );

        xml_test!(
            test_with_disabled_tab_stops,
            r#"
            <tab-index=0 tab-stop=false>
            <tab-index=1 actual=0>
            <tab-index=2 tab-stop=false>
            <tab-index=3 actual=1>
            <tab-index=4 tab-stop=false>
            "#
        );
    }

    mod tab_group {
        use super::*;

        // This test defines the expected behavior for tab-group
        // Tab-group should create a nested tab context where inner elements
        // have tab indices relative to the group
        xml_test!(
            test_tab_group_functionality,
            r#"
                <tab-index=0 actual=0>
                <tab-index=1 actual=1>
                <tab-group tab-index=2>
                    <tab-index=0 actual=2>
                    <tab-index=1 actual=3>
                </tab-group>
                <tab-index=3 actual=4>
                <tab-index=4 actual=5>
            "#
        );

        xml_test!(
            test_sibling_groups,
            r#"
                <tab-index=0 actual=0>
                <tab-index=1 actual=1>
                <tab-group tab-index=2>
                    <tab-index=0 actual=2>
                    <tab-index=1 actual=3>
                </tab-group>
                <tab-index=3 actual=4>
                <tab-index=4 actual=5>
                <tab-group tab-index=6>
                    <tab-index=0 actual=6>
                    <tab-index=1 actual=7>
                </tab-group>
                <tab-index=7 actual=8>
                <tab-index=8 actual=9>
            "#
        );

        xml_test!(
            test_nested_group,
            r#"
                <tab-index=0 actual=0>
                <tab-index=1 actual=1>
                <tab-group tab-index=2>
                    <tab-group tab-index=0>
                        <tab-index=0 actual=2>
                        <tab-index=1 actual=3>
                    </tab-group>
                    <tab-index=1 actual=4>
                </tab-group>
                <tab-index=3 actual=5>
                <tab-index=4 actual=6>
            "#
        );

        #[test]
        // todo! remove
        #[should_panic(expected = "Missing handle at actual=4")]
        fn test_non_sequential_actual_values_should_fail() {
            // This test intentionally uses non-sequential actual values (missing 4 and 5)
            // to verify that the validation catches this error
            let xml = r#"
                <tab-index=0 actual=0>
                <tab-index=1 actual=1>
                <tab-group tab-index=2>
                    <tab-group tab-index=0>
                        <tab-index=0 actual=2>
                        <tab-index=1 actual=3>
                    </tab-group>
                    <tab-index=1 actual=6>
                </tab-group>
                <tab-index=3 actual=7>
                <tab-index=4 actual=8>
            "#;
            check(xml);
        }

        #[test]
        // todo! remove
        #[should_panic(expected = "Tab navigation error at position")]
        fn test_wrong_tab_order_should_fail() {
            // This test has all sequential actual values, but they don't match
            // the expected tab order based on the tree structure
            // The element with actual=4 is in the wrong position in the tree
            let xml = r#"
                <tab-index=0 actual=0>
                <tab-index=1 actual=1>
                <tab-group tab-index=2>
                    <tab-group tab-index=0>
                        <tab-index=0 actual=2>
                        <tab-index=1 actual=3>
                    </tab-group>
                    <tab-index=1 actual=6>
                </tab-group>
                <tab-index=3 actual=4>
                <tab-index=4 actual=5>
            "#;
            check(xml);
        }

        #[test]
        fn test_sibling_nested_groups() {
            let content = r#"
                <tab-index=0 actual=0>
                <tab-index=1 actual=1>
                <tab-group tab-index=2>
                    <tab-index=0 actual=2>
                    <tab-index=2 actual=5>
                    <tab-group tab-index=1>
                        <tab-index=0 actual=3>
                        <tab-index=1 actual=4>
                    </tab-group>
                    <tab-group tab-index=3>
                        <tab-index=0 actual=6>
                        <tab-index=1 actual=7>
                    </tab-group>
                </tab-group>
                <tab-index=3 actual=8>
                <tab-index=4 actual=9>
            "#;
            check(content);
        }
    }
}
