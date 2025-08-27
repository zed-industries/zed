use crate::{FocusHandle, FocusId};

/// Represents a collection of tab handles.
///
/// Used to manage the `Tab` event to switch between focus handles.
#[derive(Default)]
pub(crate) struct TabHandles {
    pub(crate) handles: Vec<FocusHandle>,
    groups: Vec<GroupDef>,
    group_depth: usize,
}

struct GroupDef {
    index: isize,
    offset: usize,
}

impl TabHandles {
    pub(crate) fn insert(&mut self, focus_handle: &FocusHandle) {
        if !focus_handle.tab_stop {
            return;
        }

        let mut focus_handle = focus_handle.clone();
        for group in self.groups.iter().rev().take(self.group_depth) {
            focus_handle.tab_index += group.index;
        }

        // Insert handle with same tab_index last
        if let Some(ix) = self
            .handles
            .iter()
            .position(|tab| tab.tab_index > focus_handle.tab_index)
        {
            self.handles.insert(ix, focus_handle);
        } else {
            self.handles.push(focus_handle);
        }
    }

    pub(crate) fn clear(&mut self) {
        self.handles.clear();
    }

    fn current_index(&self, focused_id: Option<&FocusId>) -> Option<usize> {
        self.handles.iter().position(|h| Some(&h.id) == focused_id)
    }

    pub(crate) fn next(&self, focused_id: Option<&FocusId>) -> Option<FocusHandle> {
        let next_ix = self
            .current_index(focused_id)
            .and_then(|ix| {
                let next_ix = ix + 1;
                (next_ix < self.handles.len()).then_some(next_ix)
            })
            .unwrap_or_default();

        self.handles.get(next_ix).cloned()
    }

    pub(crate) fn prev(&self, focused_id: Option<&FocusId>) -> Option<FocusHandle> {
        let ix = self.current_index(focused_id).unwrap_or_default();
        let prev_ix = if ix == 0 {
            self.handles.len().saturating_sub(1)
        } else {
            ix.saturating_sub(1)
        };

        self.handles.get(prev_ix).cloned()
    }

    fn begin_group(&mut self, tab_index: isize) {
        self.groups.push(GroupDef {
            index: tab_index,
            offset: 0,
        });
        self.group_depth += 1;
    }

    fn end_group(&mut self) {
        self.group_depth -= 1;
        self.groups.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::{FocusHandle, FocusId, FocusMap, TabHandles};
    use std::sync::Arc;

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
            tab_handles: &mut TabHandles,
        ) -> HashMap<usize, FocusHandle> {
            let mut actual_to_handle = HashMap::new();

            fn construct_recursive(
                node: &mut TreeNode,
                focus_map: &Arc<FocusMap>,
                tab_handles: &mut TabHandles,
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
        // This tests that focus traps create proper closed loops and that
        // navigation respects trap boundaries.
        fn eval(
            tree: &TreeNode,
            tab_handles: &TabHandles,
            actual_to_handle: &HashMap<usize, FocusHandle>,
        ) {
            // First, collect information about which handles are in focus traps
            #[derive(Debug, Clone)]
            struct HandleContext {
                handle: FocusHandle,
                actual: usize,
                focus_trap_members: Option<Vec<FocusHandle>>,
            }

            fn collect_handle_contexts(
                node: &TreeNode,
                contexts: &mut Vec<HandleContext>,
                current_trap: Option<Vec<FocusHandle>>,
            ) {
                match &node.node_type {
                    NodeType::TabStop { actual, .. } => {
                        if let Some(handle) = &node.handle {
                            contexts.push(HandleContext {
                                handle: handle.clone(),
                                actual: *actual,
                                focus_trap_members: current_trap.clone(),
                            });
                        }
                    }
                    NodeType::NonTabStop { .. } => {
                        // Non-tab stops don't participate in navigation
                    }
                    NodeType::Group { children, .. } => {
                        // Groups are transparent - just recurse with same trap context
                        for child in children {
                            collect_handle_contexts(child, contexts, current_trap.clone());
                        }
                    }
                    NodeType::FocusTrap { children } => {
                        // Start collecting handles for this focus trap
                        let mut trap_handles = Vec::new();

                        // First pass: collect all handles in this trap
                        fn collect_trap_handles(node: &TreeNode, handles: &mut Vec<FocusHandle>) {
                            match &node.node_type {
                                NodeType::TabStop { .. } => {
                                    if let Some(handle) = &node.handle {
                                        handles.push(handle.clone());
                                    }
                                }
                                NodeType::NonTabStop { .. } => {
                                    // Non-tab stops don't participate in tab navigation
                                }
                                NodeType::Group { children, .. } => {
                                    for child in children {
                                        collect_trap_handles(child, handles);
                                    }
                                }
                                NodeType::FocusTrap { children } => {
                                    // Nested traps create their own context
                                    for child in children {
                                        collect_trap_handles(child, handles);
                                    }
                                }
                            }
                        }

                        for child in children {
                            collect_trap_handles(child, &mut trap_handles);
                        }

                        // Second pass: add contexts with trap information
                        for child in children {
                            collect_handle_contexts(child, contexts, Some(trap_handles.clone()));
                        }
                    }
                }
            }

            let mut handle_contexts = Vec::new();
            // Skip the root node
            if let NodeType::Group { children, .. } = &tree.node_type {
                for child in children {
                    collect_handle_contexts(child, &mut handle_contexts, None);
                }
            }

            // Sort by actual position to get expected order
            handle_contexts.sort_by_key(|c| c.actual);

            // Helper function to format tree structure as XML for error messages
            fn format_tree_structure(node: &TreeNode, tab_handles: &TabHandles) -> String {
                let mut result = String::new();

                fn format_node(node: &TreeNode, tab_handles: &TabHandles, indent: usize) -> String {
                    let mut result = String::new();
                    let indent_str = "  ".repeat(indent);

                    match &node.node_type {
                        NodeType::TabStop { tab_index, actual } => {
                            let actual = node
                                .handle
                                .as_ref()
                                .and_then(|handle| tab_handles.current_index(Some(&handle.id)))
                                .unwrap_or(*actual);
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

            // Helper function to check navigation and panic with formatted error if it doesn't match
            fn check_navigation(
                tree: &TreeNode,
                error_label: &str,
                current_id: FocusId,
                actual_id: Option<FocusId>,
                expected_id: FocusId,
                tab_handles: &TabHandles,
            ) {
                if actual_id != Some(expected_id) {
                    panic!(
                        "Tab navigation error!\n\n{}\n\n{}\n\n{}",
                        error_label,
                        format_tree_with_navigation(tree, current_id, actual_id, expected_id),
                        pretty_assertions::StrComparison::new(
                            &format_tree_structure(tree, tab_handles),
                            &format_tree_structure(tree, &TabHandles::default()),
                        ),
                    );
                }
            }

            // Helper function to format tree with navigation annotations
            fn format_tree_with_navigation(
                node: &TreeNode,
                current_id: FocusId,
                went_to_id: Option<FocusId>,
                expected_id: FocusId,
            ) -> String {
                let mut result = String::new();

                fn format_node_with_nav(
                    node: &TreeNode,
                    indent: usize,
                    current_id: FocusId,
                    went_to_id: Option<FocusId>,
                    expected_id: FocusId,
                ) -> String {
                    let mut result = String::new();
                    let indent_str = "  ".repeat(indent);

                    match &node.node_type {
                        NodeType::TabStop { tab_index, actual } => {
                            let actual_str = format!(" actual={}", actual);

                            // Add navigation annotations
                            let nav_comment = if let Some(handle) = &node.handle {
                                if handle.id == current_id && current_id != expected_id {
                                    " // <- Started here"
                                } else if Some(handle.id) == went_to_id {
                                    " // <- Actually went here"
                                } else if handle.id == expected_id {
                                    if went_to_id.is_none() {
                                        " // <- Expected to go here (but got None)"
                                    } else {
                                        " // <- Expected to go here"
                                    }
                                } else {
                                    ""
                                }
                            } else {
                                ""
                            };

                            result.push_str(&format!(
                                "{}<tab-index={}{}>{}\n",
                                indent_str, tab_index, actual_str, nav_comment
                            ));
                        }
                        NodeType::NonTabStop { tab_index } => {
                            // Format non-tab stops without actual value
                            let nav_comment = String::new(); // Non-tab stops don't participate in navigation

                            result.push_str(&format!(
                                "{}<tab-index={} tab-stop=false>{}\n",
                                indent_str,
                                tab_index.map_or("None".to_string(), |v| v.to_string()),
                                nav_comment
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
                        ));
                    }
                }

                result
            }

            // Check that we have the expected number of tab stops
            if handle_contexts.len() != actual_to_handle.len() {
                // Build maps for error display
                let mut actual_map = HashMap::new();
                let mut current = None;
                for i in 0..tab_handles.handles.len() {
                    if let Some(next_handle) =
                        tab_handles.next(current.as_ref().map(|h: &FocusHandle| &h.id))
                    {
                        if i > 0
                            && current.as_ref().map(|h: &FocusHandle| h.id) == Some(next_handle.id)
                        {
                            break;
                        }
                        actual_map.insert(next_handle.id, i);
                        current = Some(next_handle);
                    } else {
                        break;
                    }
                }

                let mut expected_map = HashMap::new();
                for (pos, handle) in actual_to_handle.iter() {
                    expected_map.insert(handle.id, *pos);
                }

                panic!(
                    "Number of tab stops doesn't match! Expected {} but found {}\n\n{}",
                    actual_to_handle.len(),
                    handle_contexts.len(),
                    pretty_assertions::StrComparison::new(
                        &format_tree_structure(tree, tab_handles),
                        &format_tree_structure(tree, &TabHandles::default()),
                    ),
                );
            }

            // Check if there are any focus traps at all
            let has_focus_traps = handle_contexts
                .iter()
                .any(|c| c.focus_trap_members.is_some());

            // If there are no focus traps, use the simpler validation
            if !has_focus_traps {
                // Build the actual navigation order from TabHandles
                let mut tab_order: Vec<FocusHandle> = Vec::new();
                let mut current = None;

                for _ in 0..tab_handles.handles.len() {
                    if let Some(next_handle) =
                        tab_handles.next(current.as_ref().map(|h: &FocusHandle| &h.id))
                    {
                        // Check if we've cycled back to the beginning
                        if !tab_order.is_empty() && tab_order[0].id == next_handle.id {
                            break;
                        }
                        current = Some(next_handle.clone());
                        tab_order.push(next_handle);
                    } else {
                        break;
                    }
                }

                // Check each position matches the expected handle
                for (position, handle) in tab_order.iter().enumerate() {
                    let expected_handle = actual_to_handle.get(&position).unwrap_or_else(|| {
                        panic!(
                            "No element specified with actual={}, but tab order has {} elements",
                            position,
                            tab_order.len()
                        )
                    });

                    if handle.id != expected_handle.id {
                        // Use navigation-style error formatting
                        // For position 0, we're testing what should come first
                        // For other positions, we show the previous position as "started"
                        let (started_id, went_to_id) = if position > 0 {
                            // Normal case: show where we started (previous position) and where we went
                            (tab_order[position - 1].id, Some(handle.id))
                        } else {
                            // Position 0: we're testing what should be first
                            // Don't show "Started here" since we haven't started anywhere yet
                            // Show the actual first element as "Actually went here"
                            (expected_handle.id, Some(handle.id))
                        };

                        check_navigation(
                            tree,
                            &format!("Tab order mismatch at position {}", position),
                            started_id,
                            went_to_id,
                            expected_handle.id,
                            tab_handles,
                        );
                    }
                }

                // Test that navigation wraps correctly
                if !tab_order.is_empty() {
                    // Test next wraps from last to first
                    let last_id = tab_order.last().unwrap().id;
                    let first_id = tab_order.first().unwrap().id;
                    // Test next wraps from last to first
                    let actual_next_from_last = tab_handles.next(Some(&last_id));
                    let actual_next_id = actual_next_from_last.as_ref().map(|h| h.id);
                    check_navigation(
                        tree,
                        "Expected wrap from last to first (testing next() wrap-around):",
                        last_id,
                        actual_next_id,
                        first_id,
                        tab_handles,
                    );

                    // Test prev wraps from first to last
                    let actual_prev_from_first = tab_handles.prev(Some(&first_id));
                    let actual_prev_id = actual_prev_from_first.as_ref().map(|h| h.id);
                    check_navigation(
                        tree,
                        "Expected wrap from first to last (testing prev() wrap-around):",
                        first_id,
                        actual_prev_id,
                        last_id,
                        tab_handles,
                    );
                }

                return; // Early return for non-focus-trap case
            }

            // Now test navigation for each handle (focus-trap aware)
            for context in &handle_contexts {
                let current_id = context.handle.id;

                // Determine expected next and prev based on context
                let (expected_next, expected_prev) =
                    if let Some(trap_members) = &context.focus_trap_members {
                        // We're in a focus trap - navigation should stay within the trap
                        let trap_position = trap_members
                            .iter()
                            .position(|h| h.id == current_id)
                            .expect("Handle should be in its own trap");

                        let next_idx = (trap_position + 1) % trap_members.len();
                        let prev_idx = if trap_position == 0 {
                            trap_members.len() - 1
                        } else {
                            trap_position - 1
                        };

                        (trap_members[next_idx].id, trap_members[prev_idx].id)
                    } else {
                        // Not in a focus trap - normal navigation through all non-trapped elements
                        let non_trapped: Vec<&HandleContext> = handle_contexts
                            .iter()
                            .filter(|c| c.focus_trap_members.is_none())
                            .collect();

                        let non_trapped_position = non_trapped
                            .iter()
                            .position(|c| c.handle.id == current_id)
                            .expect("Non-trapped handle should be in non-trapped list");

                        let next_idx = (non_trapped_position + 1) % non_trapped.len();
                        let prev_idx = if non_trapped_position == 0 {
                            non_trapped.len() - 1
                        } else {
                            non_trapped_position - 1
                        };

                        (
                            non_trapped[next_idx].handle.id,
                            non_trapped[prev_idx].handle.id,
                        )
                    };

                // Test next navigation
                let actual_next = tab_handles.next(Some(&current_id));
                let actual_next_id = actual_next.as_ref().map(|h| h.id);
                check_navigation(
                    tree,
                    "Expected (testing next()):",
                    current_id,
                    actual_next_id,
                    expected_next,
                    tab_handles,
                );

                // Test prev navigation
                let actual_prev = tab_handles.prev(Some(&current_id));
                let actual_prev_id = actual_prev.as_ref().map(|h| h.id);
                check_navigation(
                    tree,
                    "Expected (testing prev()):",
                    current_id,
                    actual_prev_id,
                    expected_prev,
                    tab_handles,
                );
            }
        }

        // Main execution
        let focus_map = Arc::new(FocusMap::default());
        let mut tab_handles = TabHandles::default();

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
        use super::*;

        xml_test!(
            test_simple_ordering,
            r#"
                <tab-index=0 actual=0>
                <tab-index=1 actual=1>
                <tab-index=2 actual=2>
            "#
        );

        xml_test!(
            test_duplicate_indices_maintain_insertion_order,
            r#"
                <tab-index=0 actual=0>
                <tab-index=0 actual=1>
                <tab-index=1 actual=2>
                <tab-index=1 actual=3>
                <tab-index=2 actual=4>
            "#
        );

        xml_test!(
            test_positive_and_negative_indices,
            r#"
                <tab-index=1 actual=2>
                <tab-index=-1 actual=0>
                <tab-index=0 actual=1>
                <tab-index=2 actual=3>
            "#
        );

        #[test]
        #[should_panic(
            expected = "Non-tab stop (tab-stop=false) should not have an 'actual' attribute"
        )]
        fn test_non_tab_stop_with_actual_panics() {
            let xml = r#"
                <tab-index=0 actual=0>
                <tab-index=1 tab-stop=false actual=1>
                <tab-index=2 actual=2>
            "#;
            check(xml);
        }

        #[test]
        #[should_panic(expected = "Tab stop with tab-index=1 must have an 'actual' attribute")]
        fn test_tab_stop_without_actual_panics() {
            // Tab stops must have an actual value
            let xml = r#"
                <tab-index=0 actual=0>
                <tab-index=1>
                <tab-index=2 actual=2>
            "#;
            check(xml);
        }

        #[test]
        #[should_panic(expected = "Tab order mismatch at position")]
        fn test_incorrect_tab_order_shows_xml_format() {
            // This test intentionally has wrong expected order to demonstrate error reporting
            // The actual tab order will be: tab-index=-1, 0, 1, 2 (positions 0, 1, 2, 3)
            // But we're expecting them at wrong positions
            let xml = r#"
                <tab-index=0 actual=0>
                <tab-index=-1 actual=1>
                <tab-index=2 actual=2>
                <tab-index=1 actual=3>
            "#;
            check(xml);
        }
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
                </tab-group>
                <tab-index=3 actual=4>
                <tab-index=4 actual=5>
            "#
        );

        xml_test!(
            test_sibling_nested_groups,
            r#"
                <tab-index=0 actual=0>
                <tab-index=1 actual=1>
                <tab-group tab-index=2>
                    <tab-index=0 actual=2>
                    <tab-group tab-index=1>
                        <tab-index=0 actual=3>
                        <tab-index=1 actual=4>
                    </tab-group>
                    <tab-index=2 actual=5>
                    <tab-group tab-index=3>
                        <tab-index=0 actual=6>
                        <tab-index=1 actual=7>
                    </tab-group>
                </tab-group>
                <tab-index=3 actual=8>
                <tab-index=4 actual=9>
            "#
        );
    }

    mod focus_trap {
        use super::*;

        xml_test!(
            test_focus_trap_in_group,
            r#"
                <tab-index=0 actual=0>
                <tab-group tab-index=1>
                    <tab-index=0 actual=1>
                    <focus-trap>
                        <tab-index=0 actual=2>
                        <tab-index=1 actual=3>
                    </focus-trap>
                    <tab-index=1 actual=4>
                </tab-group>
                <tab-index=2 actual=5>
            "#
        );

        // This test defines the expected behavior for focus-trap
        // Focus-trap should trap navigation within its boundaries
        xml_test!(
            test_focus_trap_functionality,
            r#"
                <tab-index=0 actual=0>
                <focus-trap tab-index=1>
                    <tab-index=0 actual=1>
                    <tab-index=1 actual=2>
                </focus-trap>
                <tab-index=2 actual=3>
            "#
        );

        xml_test!(
            test_nested_groups_and_traps,
            r#"
                <tab-index=0 actual=0>
                <tab-group tab-index=1>
                    <tab-index=0 actual=1>
                    <focus-trap tab-index=1>
                        <tab-index=0 actual=2>
                        <tab-index=1 actual=3>
                    </focus-trap>
                    <tab-index=2 actual=4>
                </tab-group>
                <tab-index=2 actual=5>
            "#
        );
    }

    #[test]
    fn test_tab_handles() {
        let focus_map = Arc::new(FocusMap::default());
        let mut tab = TabHandles::default();

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
            tab.insert(handle);
        }
        assert_eq!(
            tab.handles
                .iter()
                .map(|handle| handle.id)
                .collect::<Vec<_>>(),
            vec![
                focus_handles[0].id,
                focus_handles[5].id,
                focus_handles[1].id,
                focus_handles[2].id,
                focus_handles[6].id,
            ]
        );

        // Select first tab index if no handle is currently focused.
        assert_eq!(tab.next(None), Some(tab.handles[0].clone()));
        // Select last tab index if no handle is currently focused.
        assert_eq!(
            tab.prev(None),
            Some(tab.handles[tab.handles.len() - 1].clone())
        );

        assert_eq!(
            tab.next(Some(&tab.handles[0].id)),
            Some(tab.handles[1].clone())
        );
        assert_eq!(
            tab.next(Some(&tab.handles[1].id)),
            Some(tab.handles[2].clone())
        );
        assert_eq!(
            tab.next(Some(&tab.handles[2].id)),
            Some(tab.handles[3].clone())
        );
        assert_eq!(
            tab.next(Some(&tab.handles[3].id)),
            Some(tab.handles[4].clone())
        );
        assert_eq!(
            tab.next(Some(&tab.handles[4].id)),
            Some(tab.handles[0].clone())
        );

        // prev
        assert_eq!(tab.prev(None), Some(tab.handles[4].clone()));
        assert_eq!(
            tab.prev(Some(&tab.handles[0].id)),
            Some(tab.handles[4].clone())
        );
        assert_eq!(
            tab.prev(Some(&tab.handles[1].id)),
            Some(tab.handles[0].clone())
        );
        assert_eq!(
            tab.prev(Some(&tab.handles[2].id)),
            Some(tab.handles[1].clone())
        );
        assert_eq!(
            tab.prev(Some(&tab.handles[3].id)),
            Some(tab.handles[2].clone())
        );
        assert_eq!(
            tab.prev(Some(&tab.handles[4].id)),
            Some(tab.handles[3].clone())
        );
    }
}
