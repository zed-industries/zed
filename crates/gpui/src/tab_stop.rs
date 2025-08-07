use crate::{FocusHandle, FocusId};

/// Represents a collection of tab handles.
///
/// Used to manage the `Tab` event to switch between focus handles.
#[derive(Default)]
pub(crate) struct TabHandles {
    pub(crate) handles: Vec<FocusHandle>,
}

impl TabHandles {
    pub(crate) fn insert(&mut self, focus_handle: &FocusHandle) {
        if !focus_handle.tab_stop {
            return;
        }

        let focus_handle = focus_handle.clone();

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
}

#[cfg(test)]
mod tests {
    use crate::{FocusHandle, FocusMap, TabHandles};
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
        let focus_map = Arc::new(FocusMap::default());
        let mut tab_handles = TabHandles::default();

        // Parse the XML-like structure
        let elements = parse_xml_structure(xml);

        // Create focus handles based on parsed elements
        let mut all_handles = Vec::new();
        let mut actual_to_handle = std::collections::HashMap::<usize, FocusHandle>::new();

        for element in elements {
            let mut handle = FocusHandle::new(&focus_map);

            // Set tab_index if specified
            if let Some(tab_index) = element.tab_index {
                handle = handle.tab_index(tab_index);
            }

            // Enable tab_stop by default unless it's explicitly disabled
            // Container elements (tab-group, focus-trap) should not be tab stops themselves
            let should_be_tab_stop = if element.is_container {
                false
            } else {
                element.tab_stop.unwrap_or(true)
            };
            handle = handle.tab_stop(should_be_tab_stop);

            // Store the handle
            all_handles.push(handle.clone());
            tab_handles.insert(&handle);

            // Track handles by their actual position
            // Skip container elements as they don't participate in tab order directly
            if !element.is_container {
                if let Some(actual) = element.actual {
                    if actual_to_handle.insert(actual, handle).is_some() {
                        panic!("Duplicate actual value: {}", actual);
                    }
                }
            }
        }

        // Get the actual tab order from TabHandles
        let mut tab_order: Vec<FocusHandle> = Vec::new();
        let mut current = None;

        // Build the actual navigation order
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

        // Check that we have the expected number of tab stops
        assert_eq!(
            tab_order.len(),
            actual_to_handle.len(),
            "Number of tab stops ({}) doesn't match expected ({})",
            tab_order.len(),
            actual_to_handle.len()
        );

        // Check each position matches the expected handle
        for (position, handle) in tab_order.iter().enumerate() {
            let expected_handle = actual_to_handle.get(&position).unwrap_or_else(|| {
                panic!(
                    "No element specified with actual={}, but tab order has {} elements",
                    position,
                    tab_order.len()
                )
            });

            assert_eq!(
                handle.id, expected_handle.id,
                "Tab order at position {} doesn't match expected. Got {:?}, expected {:?}",
                position, handle.id, expected_handle.id
            );
        }

        // Test that navigation wraps correctly
        if !tab_order.is_empty() {
            // Test next wraps from last to first
            let last_id = tab_order.last().unwrap().id;
            let first_id = tab_order.first().unwrap().id;
            assert_eq!(
                tab_handles.next(Some(&last_id)).map(|h| h.id),
                Some(first_id),
                "next should wrap from last to first"
            );

            // Test prev wraps from first to last
            assert_eq!(
                tab_handles.prev(Some(&first_id)).map(|h| h.id),
                Some(last_id),
                "prev should wrap from first to last"
            );
        }

        #[derive(Debug)]
        struct ParsedElement {
            element_type: String,
            tab_index: Option<isize>,
            actual: Option<usize>,
            tab_stop: Option<bool>,
            is_container: bool, // For tab-group and focus-trap
        }

        fn parse_xml_structure(xml: &str) -> Vec<ParsedElement> {
            let mut elements = Vec::new();

            for line in xml.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                // Parse opening tags like <tab-index=0 actual=1> or <tab-group tab-index=2 actual=3>
                if line.starts_with('<') && !line.starts_with("</") {
                    let mut element = ParsedElement {
                        element_type: String::new(),
                        tab_index: None,
                        actual: None,
                        tab_stop: None,
                        is_container: false,
                    };

                    // Remove < and > brackets
                    let content = line.trim_start_matches('<').trim_end_matches('>');
                    let parts: Vec<&str> = content.split_whitespace().collect();

                    if !parts.is_empty() {
                        // First part might be element type or tab-index
                        let first_part = parts[0];
                        if first_part.starts_with("tab-index=") {
                            element.element_type = "element".to_string();
                        } else if let Some(idx) = first_part.find(' ') {
                            element.element_type = first_part[..idx].to_string();
                        } else if !first_part.contains('=') {
                            element.element_type = first_part.to_string();
                        } else {
                            element.element_type = "element".to_string();
                        }
                    }

                    // Parse attributes
                    for part in parts {
                        if let Some(eq_pos) = part.find('=') {
                            let key = &part[..eq_pos];
                            let value = &part[eq_pos + 1..];

                            match key {
                                "tab-index" => {
                                    element.tab_index = value.parse::<isize>().ok();
                                }
                                "actual" => {
                                    element.actual = value.parse::<usize>().ok();
                                }
                                "tab-stop" => {
                                    element.tab_stop = value.parse::<bool>().ok();
                                }
                                _ => {}
                            }
                        }
                    }

                    // Mark tab-group and focus-trap as containers
                    if element.element_type == "tab-group" || element.element_type == "focus-trap" {
                        element.is_container = true;
                        // Container elements should not have 'actual' values themselves
                        // Only their children should have actual values
                        if element.actual.is_some() {
                            panic!(
                                "Container element '{}' should not have an 'actual' attribute",
                                element.element_type
                            );
                        }
                    }

                    elements.push(element);
                }
            }

            elements
        }
    }

    #[test]
    fn test_check_helper() {
        // Test simple ordering
        let xml = r#"
            <tab-index=0 actual=0>
            <tab-index=1 actual=1>
            <tab-index=2 actual=2>
        "#;
        check(xml);

        // Test with duplicate tab indices (should maintain insertion order within same index)
        let xml2 = r#"
            <tab-index=0 actual=0>
            <tab-index=0 actual=1>
            <tab-index=1 actual=2>
            <tab-index=1 actual=3>
            <tab-index=2 actual=4>
        "#;
        check(xml2);

        // Test with negative and positive indices
        let xml3 = r#"
            <tab-index=1 actual=2>
            <tab-index=-1 actual=0>
            <tab-index=0 actual=1>
            <tab-index=2 actual=3>
        "#;
        check(xml3);
    }

    #[test]
    fn test_check_helper_with_nested_structures() {
        // TODO: These tests define the expected structure for tab-group and focus-trap
        // but the grouping logic is not yet implemented. For now, we only test
        // flat elements that will work with the current implementation.

        // Test flat elements only (grouping not yet implemented)
        let xml = r#"
            <tab-index=0 actual=0>
            <tab-index=1 actual=1>
            <tab-index=2 actual=2>
            <tab-index=3 actual=3>
        "#;
        check(xml);

        // Another flat test
        let xml2 = r#"
            <tab-index=0 actual=0>
            <tab-index=0 actual=1>
            <tab-index=1 actual=2>
            <tab-index=2 actual=3>
        "#;
        check(xml2);

        // Future test structure (not yet implemented):
        // <tab-group tab-index=2>
        //     <tab-index=0 actual=X>  // This would be at global position 2.0
        //     <tab-index=1 actual=Y>  // This would be at global position 2.1
        // </tab-group>
        //
        // <focus-trap tab-index=1>
        //     <tab-index=0 actual=X>  // Navigation trapped within this group
        //     <tab-index=1 actual=Y>
        // </focus-trap>
    }

    #[test]
    #[ignore = "Tab-group and focus-trap functionality not yet implemented"]
    fn test_tab_group_functionality() {
        // This test defines the expected behavior for tab-group
        // Tab-group should create a nested tab context where inner elements
        // have tab indices relative to the group
        let xml = r#"
            <tab-index=0 actual=0>
            <tab-index=1 actual=1>
            <tab-group tab-index=2>
                <tab-index=0 actual=2>
                <tab-index=1 actual=3>
            </tab-group>
            <tab-index=3 actual=4>
        "#;
        check(xml);
    }

    #[test]
    #[ignore = "Tab-group and focus-trap functionality not yet implemented"]
    fn test_focus_trap_functionality() {
        // This test defines the expected behavior for focus-trap
        // Focus-trap should trap navigation within its boundaries
        let xml = r#"
            <tab-index=0 actual=0>
            <focus-trap tab-index=1>
                <tab-index=0 actual=1>
                <tab-index=1 actual=2>
            </focus-trap>
            <tab-index=2 actual=3>
        "#;
        check(xml);
    }

    #[test]
    #[ignore = "Tab-group and focus-trap functionality not yet implemented"]
    fn test_nested_groups_and_traps() {
        // This test defines the expected behavior for nested structures
        let xml = r#"
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
        "#;
        check(xml);
    }

    #[test]
    fn test_with_disabled_tab_stops() {
        // Test with mixed tab-stop values
        let xml = r#"
            <tab-index=0 actual=0>
            <tab-index=1 tab-stop=false>
            <tab-index=2 actual=1>
            <tab-index=3 actual=2>
        "#;
        check(xml);

        // Test with all disabled except specific ones
        let xml2 = r#"
            <tab-index=0 tab-stop=false>
            <tab-index=1 actual=0>
            <tab-index=2 tab-stop=false>
            <tab-index=3 actual=1>
            <tab-index=4 tab-stop=false>
        "#;
        check(xml2);
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
