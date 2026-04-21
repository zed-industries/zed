use accesskit::{Live, Node, NodeId, Role, Toggled, TreeUpdate};

use crate::{Bounds, ScaledPixels, SharedString};

/// Accessibility annotations for a GPUI element.
///
/// Maps 1:1 to WAI-ARIA 1.1 attributes. Attach to any `div()` via the
/// builder methods on `InteractiveElement` (e.g. `.role(Role::Button)`).
#[derive(Default, Clone)]
pub struct Accessibility {
    /// `role` — semantic role (e.g. Button, CheckBox, StaticText)
    pub role: Option<Role>,
    /// `aria-label` — explicit accessible name
    pub label: Option<SharedString>,
    /// `aria-description`
    pub description: Option<SharedString>,
    /// `aria-checked` (maps to accesskit Toggled)
    pub checked: Option<bool>,
    /// `aria-disabled`
    pub disabled: Option<bool>,
    /// `aria-expanded`
    pub expanded: Option<bool>,
    /// `aria-hidden` — exclude from the accessibility tree
    pub hidden: bool,
    /// `aria-pressed` (maps to accesskit Toggled)
    pub pressed: Option<bool>,
    /// `aria-readonly`
    pub readonly: Option<bool>,
    /// `aria-required`
    pub required: Option<bool>,
    /// `aria-selected`
    pub selected: Option<bool>,
    /// `aria-live`
    pub live: Option<Live>,
}

impl Accessibility {
    /// Build an accesskit `Node` from these annotations.
    ///
    /// `child_text` is the concatenated text content of descendant StaticText
    /// nodes, used as the accessible name fallback per ACCNAME 1.2 §4.2 when
    /// no explicit `aria-label` is set.
    pub(crate) fn to_node(
        &self,
        bounds: Bounds<ScaledPixels>,
        is_focused: bool,
        child_text: Option<String>,
    ) -> Node {
        let _ = is_focused; // Focused state is conveyed via TreeUpdate::focus, not on the node.
        let role = self.role.unwrap_or(Role::GenericContainer);
        let mut node = Node::new(role);

        // Accessible name: explicit label takes priority, then child text fallback (ACCNAME 1.2).
        if let Some(label) = &self.label {
            node.set_label(label.as_ref());
        } else if let Some(text) = child_text {
            if !text.is_empty() {
                node.set_label(text);
            }
        }

        if let Some(desc) = &self.description {
            node.set_description(desc.as_ref());
        }

        node.set_bounds(accesskit::Rect {
            x0: bounds.origin.x.0 as f64,
            y0: bounds.origin.y.0 as f64,
            x1: (bounds.origin.x.0 + bounds.size.width.0) as f64,
            y1: (bounds.origin.y.0 + bounds.size.height.0) as f64,
        });

        if let Some(checked) = self.checked {
            node.set_toggled(if checked {
                Toggled::True
            } else {
                Toggled::False
            });
        }
        if self.disabled.unwrap_or(false) {
            node.set_disabled();
        }
        if let Some(expanded) = self.expanded {
            node.set_expanded(expanded);
        }
        if self.hidden {
            node.set_hidden();
        }
        if let Some(pressed) = self.pressed {
            node.set_toggled(if pressed {
                Toggled::True
            } else {
                Toggled::False
            });
        }
        if self.readonly.unwrap_or(false) {
            node.set_read_only();
        }
        if self.required.unwrap_or(false) {
            node.set_required();
        }
        if let Some(selected) = self.selected {
            node.set_selected(selected);
        }
        if let Some(live) = self.live {
            node.set_live(live);
        }

        node
    }
}

/// One accessibility node entry accumulated during `prepaint`.
#[derive(Clone)]
pub(crate) struct AccessibilityEntry {
    pub node_id: NodeId,
    pub node: Node,
    pub parent_id: Option<NodeId>,
}

/// Accumulated accessibility tree for one rendered frame.
pub(crate) struct AccessibilityFrame {
    pub entries: Vec<AccessibilityEntry>,
    pub root_id: Option<NodeId>,
    pub focus: Option<NodeId>,
}

impl AccessibilityFrame {
    pub(crate) fn new() -> Self {
        Self {
            entries: Vec::new(),
            root_id: None,
            focus: None,
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Convert this frame into an accesskit `TreeUpdate`.
    pub(crate) fn into_tree_update(self) -> TreeUpdate {
        // The window root node owns all top-level annotated nodes.
        let root_id = self.root_id.unwrap_or(NodeId(1));

        // Build parent→children mapping.
        let mut children_of: std::collections::HashMap<NodeId, Vec<NodeId>> =
            std::collections::HashMap::new();
        for entry in &self.entries {
            let parent = entry.parent_id.unwrap_or(root_id);
            children_of.entry(parent).or_default().push(entry.node_id);
        }

        let focus = self
            .focus
            .unwrap_or(root_id);

        let mut nodes: Vec<(NodeId, Node)> = self
            .entries
            .into_iter()
            .map(|mut entry| {
                if let Some(children) = children_of.remove(&entry.node_id) {
                    entry.node.set_children(children);
                }
                (entry.node_id, entry.node)
            })
            .collect();

        // Root node: owns all top-level annotated nodes.
        let mut root_node = Node::new(Role::Window);
        if let Some(top_level) = children_of.remove(&root_id) {
            root_node.set_children(top_level);
        }
        nodes.push((root_id, root_node));

        TreeUpdate {
            nodes,
            tree: Some(accesskit::Tree::new(root_id)),
            tree_id: accesskit::TreeId::ROOT,
            focus,
        }
    }

    /// Format the tree as indented text for the `accessibility_tree()` debug API.
    #[cfg(any(feature = "inspector", debug_assertions))]
    pub(crate) fn format_tree(&self) -> String {
        // Build parent→children index by entry position.
        let mut children_of: std::collections::HashMap<Option<NodeId>, Vec<usize>> =
            std::collections::HashMap::new();
        for (i, entry) in self.entries.iter().enumerate() {
            children_of.entry(entry.parent_id).or_default().push(i);
        }

        let mut out = String::from("[accessibility tree]\n");
        for &idx in children_of.get(&None).into_iter().flatten() {
            write_node(&self.entries, &children_of, idx, 1, &mut out);
        }
        out
    }
}

#[cfg(any(feature = "inspector", debug_assertions))]
fn write_node(
    entries: &[AccessibilityEntry],
    children_of: &std::collections::HashMap<Option<NodeId>, Vec<usize>>,
    idx: usize,
    depth: usize,
    out: &mut String,
) {
    use std::fmt::Write;

    let entry = &entries[idx];
    let indent = "  ".repeat(depth);
    let role = format!("{:?}", entry.node.role());
    let label = entry.node.label().unwrap_or("");
    let _ = writeln!(out, "{}{:<16} {}", indent, role, label);
    let key = Some(entry.node_id);
    for &child_idx in children_of.get(&key).into_iter().flatten() {
        write_node(entries, children_of, child_idx, depth + 1, out);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use accesskit::{Node, NodeId, Role};

    fn entry(id: u64, role: Role, label: &str, parent: Option<u64>) -> AccessibilityEntry {
        let mut node = Node::new(role);
        if !label.is_empty() {
            node.set_label(label);
        }
        AccessibilityEntry {
            node_id: NodeId(id),
            node,
            parent_id: parent.map(NodeId),
        }
    }

    #[test]
    fn test_format_tree_empty() {
        let frame = AccessibilityFrame::new();
        assert_eq!(frame.format_tree(), "[accessibility tree]\n");
    }

    #[test]
    fn test_format_tree_single_node() {
        let mut frame = AccessibilityFrame::new();
        frame.entries.push(entry(1, Role::Button, "Increment", None));
        // Role name left-aligned in 16-char column, then label; depth-1 = 2-space indent.
        assert_eq!(
            frame.format_tree(),
            "[accessibility tree]\n\
             \x20 Button           Increment\n"
        );
    }

    #[test]
    fn test_format_tree_parent_child() {
        let mut frame = AccessibilityFrame::new();
        frame.entries.push(entry(1, Role::Group, "Panel", None));
        frame.entries.push(entry(2, Role::Button, "OK", Some(1)));
        frame.entries.push(entry(3, Role::CheckBox, "Opt", Some(1)));
        // Depth-1 = 2-space indent, depth-2 = 4-space indent.
        assert_eq!(
            frame.format_tree(),
            "[accessibility tree]\n\
             \x20 Group            Panel\n\
             \x20   Button           OK\n\
             \x20   CheckBox         Opt\n"
        );
    }

    #[test]
    fn test_format_tree_no_label() {
        let mut frame = AccessibilityFrame::new();
        frame.entries.push(entry(1, Role::GenericContainer, "", None));
        // "GenericContainer" is exactly 16 chars; trailing space comes from the separator.
        assert_eq!(
            frame.format_tree(),
            "[accessibility tree]\n\
             \x20 GenericContainer \n"
        );
    }
}
