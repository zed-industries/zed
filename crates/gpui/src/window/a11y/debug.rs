//! Developer tooling for inspecting the accessibility tree.
//!
//! [`A11yDebug`] retains the last [`TreeUpdate`] sent to the platform adapter so
//! it can be serialized on demand (see
//! [`crate::Window::debug_a11y_tree_json`]). In `cfg(debug_assertions)` builds,
//! we capture extra info.

use accesskit::{Action, NodeId, TreeUpdate};
use collections::FxHashMap;

use crate::{Pixels, SharedString, Size};

#[derive(Default)]
pub(crate) struct FrameDebugInfo {
    pub viewport_size: Size<Pixels>,
    pub scale_factor: f32,
    pub tab_stop_count: usize,
}

struct CapturedFrame {
    rendered_at: String,
    frame_number: u64,
    window_title: Option<SharedString>,
    node_count: usize,
    tab_stop_count: usize,
    viewport_size: Size<Pixels>,
    scale_factor: f32,
}

#[cfg(debug_assertions)]
#[derive(Clone, Default)]
pub(crate) struct NodeDebugInfo {
    /// Whether the node was synthesized via
    /// [`crate::Element::a11y_synthetic_children`] rather than created from a
    /// real element with a role and ID.
    pub synthetic: bool,
    /// The type name of the `Render` view that was rendering when the node was
    /// created.
    pub view: Option<&'static str>,
    /// The [`ElementId`](crate::ElementId) of the creating element (the leaf of
    /// its `GlobalElementId`, not the full path). For a synthetic node, this is
    /// the real element whose `a11y_synthetic_children` produced it.
    pub element_id: Option<String>,
    /// Source location where the creating element was constructed.
    pub source_location: Option<&'static core::panic::Location<'static>>,
}

#[cfg(debug_assertions)]
#[derive(Clone, Default)]
pub(crate) struct NodeCreator {
    pub view: Option<&'static str>,
    pub element_id: Option<String>,
    pub source_location: Option<&'static core::panic::Location<'static>>,
}

#[derive(Default)]
pub(crate) struct A11yDebug {
    last_tree_update: Option<TreeUpdate>,
    last_gpui_focus: Option<NodeId>,
    last_active_descendant: Option<NodeId>,
    /// Monotonic counter incremented on each captured frame, so a re-dump makes
    /// it obvious whether the tree actually refreshed.
    frame_number: u64,
    /// Metadata about the most recently captured frame.
    last_frame: Option<CapturedFrame>,
    #[cfg(debug_assertions)]
    last_node_info: FxHashMap<NodeId, NodeDebugInfo>,
}

impl A11yDebug {
    pub(crate) fn capture(
        &mut self,
        update: &TreeUpdate,
        gpui_focus: Option<NodeId>,
        active_descendant: Option<NodeId>,
        window_title: Option<&SharedString>,
        frame: FrameDebugInfo,
    ) {
        self.last_tree_update = Some(update.clone());
        self.last_gpui_focus = gpui_focus;
        self.last_active_descendant = active_descendant;
        self.frame_number += 1;
        self.last_frame = Some(CapturedFrame {
            rendered_at: chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, false),
            frame_number: self.frame_number,
            window_title: window_title.cloned(),
            node_count: update.nodes.len(),
            tab_stop_count: frame.tab_stop_count,
            viewport_size: frame.viewport_size,
            scale_factor: frame.scale_factor,
        });
    }

    #[cfg(debug_assertions)]
    pub(crate) fn capture_node_info(&mut self, node_info: &FxHashMap<NodeId, NodeDebugInfo>) {
        self.last_node_info = node_info.clone();
    }

    /// Serialize the last tree update to a readable JSON string. Node ids are
    /// replaced with short ephemeral ids (`a`, `b`, ..., `z`, `aa`, ...).
    pub(crate) fn to_json(&self) -> Option<String> {
        let update = self.last_tree_update.as_ref()?;

        let mut ephemeral: FxHashMap<NodeId, String> = FxHashMap::default();
        for (index, (id, _)) in update.nodes.iter().enumerate() {
            ephemeral.insert(*id, ephemeral_id(index));
        }

        let mut nodes = serde_json::Map::new();
        for (id, node) in &update.nodes {
            let key = ephemeral
                .get(id)
                .cloned()
                .unwrap_or_else(|| id.0.to_string());
            #[cfg(debug_assertions)]
            let provenance = self
                .last_node_info
                .get(id)
                .map(|info| NodeProvenance {
                    element_id: info.element_id.as_deref(),
                    view: info.view,
                    source_location: info.source_location.map(|loc| loc.to_string()),
                    // Only surface synthetic nodes; `false` is the default and
                    // would just be noise on every real node.
                    synthetic: info.synthetic.then_some(true),
                })
                .unwrap_or_default();
            #[cfg(not(debug_assertions))]
            let provenance = NodeProvenance::default();
            let value = node_to_json(*id, node, &ephemeral, &provenance);
            nodes.insert(key, value);
        }

        let frame = self.last_frame.as_ref().map(|frame| {
            serde_json::json!({
                "rendered_at": frame.rendered_at,
                "frame_number": frame.frame_number,
                "window_title": frame.window_title.as_ref().map(|title| title.to_string()),
                "node_count": frame.node_count,
                "tab_stop_count": frame.tab_stop_count,
                "viewport_size": {
                    "width": frame.viewport_size.width.0,
                    "height": frame.viewport_size.height.0,
                },
                "scale_factor": frame.scale_factor,
            })
        });

        let root = update
            .tree
            .as_ref()
            .map(|tree| tree.root)
            .and_then(|id| ephemeral.get(&id).cloned());

        let value = serde_json::json!({
            "root": root,
            "gpui_focus": self.last_gpui_focus.and_then(|id| ephemeral.get(&id).cloned()),
            "active_descendant_focus": self.last_active_descendant.and_then(|id| ephemeral.get(&id).cloned()),
            "frame": frame,
            "nodes": nodes,
        });
        Some(serde_json::to_string_pretty(&value).unwrap_or_else(|_| "{}".to_string()))
    }
}

#[derive(Default)]
struct NodeProvenance<'a> {
    element_id: Option<&'a str>,
    view: Option<&'a str>,
    source_location: Option<String>,
    synthetic: Option<bool>,
}

fn node_to_json(
    id: NodeId,
    node: &accesskit::Node,
    ephemeral: &FxHashMap<NodeId, String>,
    provenance: &NodeProvenance,
) -> serde_json::Value {
    use serde_json::json;

    let mut map = serde_json::Map::new();
    map.insert("accesskit_id".into(), json!(id.0.to_string()));

    let children: Vec<String> = node
        .children()
        .iter()
        .map(|child| {
            ephemeral
                .get(child)
                .cloned()
                .unwrap_or_else(|| child.0.to_string())
        })
        .collect();
    if !children.is_empty() {
        map.insert("children".into(), json!(children));
    }

    // Provenance (debug builds only), ordered before the accessibility section.
    if let Some(element_id) = provenance.element_id {
        map.insert("element_id".into(), json!(element_id));
    }
    if let Some(view) = provenance.view {
        map.insert("view".into(), json!(view));
    }
    if let Some(source_location) = &provenance.source_location {
        map.insert("source_location".into(), json!(source_location));
    }
    if let Some(synthetic) = provenance.synthetic {
        map.insert("synthetic".into(), json!(synthetic));
    }

    // Accessibility semantics for this node, grouped together.
    let mut aria = serde_json::Map::new();
    aria.insert("role".into(), json!(format!("{:?}", node.role())));

    // Which action types the node supports. AccessKit keeps these in a private
    // bitset with no getter or iterator, so we probe each variant. `Action::n`
    // (from AccessKit's `enumn` feature) maps a discriminant to its variant,
    // returning `None` past the last one - so this can't drift out of sync with
    // AccessKit's `Action` enum the way a hand-maintained list would.
    let mut next_action = 0u8;
    let on_action: Vec<String> = std::iter::from_fn(move || {
        let action = Action::n(next_action)?;
        next_action += 1;
        Some(action)
    })
    .filter(|action| node.supports_action(*action))
    .map(|action| format!("{action:?}"))
    .collect();
    if !on_action.is_empty() {
        aria.insert("on_action".into(), json!(on_action));
    }

    // String properties.
    if let Some(v) = node.label() {
        aria.insert("label".into(), json!(v));
    }
    if let Some(v) = node.description() {
        aria.insert("description".into(), json!(v));
    }
    if let Some(v) = node.value() {
        aria.insert("value".into(), json!(v));
    }
    if let Some(v) = node.keyboard_shortcut() {
        aria.insert("keyboard_shortcut".into(), json!(v));
    }
    if let Some(v) = node.access_key() {
        aria.insert("access_key".into(), json!(v));
    }
    if let Some(v) = node.placeholder() {
        aria.insert("placeholder".into(), json!(v));
    }
    if let Some(v) = node.tooltip() {
        aria.insert("tooltip".into(), json!(v));
    }
    if let Some(v) = node.role_description() {
        aria.insert("role_description".into(), json!(v));
    }

    // Boolean / enum states.
    if let Some(v) = node.is_selected() {
        aria.insert("selected".into(), json!(v));
    }
    if let Some(v) = node.is_expanded() {
        aria.insert("expanded".into(), json!(v));
    }
    if let Some(v) = node.toggled() {
        aria.insert("toggled".into(), json!(format!("{v:?}")));
    }
    if let Some(v) = node.orientation() {
        aria.insert("orientation".into(), json!(format!("{v:?}")));
    }

    // Numeric properties.
    if let Some(v) = node.numeric_value() {
        aria.insert("numeric_value".into(), json!(v));
    }
    if let Some(v) = node.min_numeric_value() {
        aria.insert("min_numeric_value".into(), json!(v));
    }
    if let Some(v) = node.max_numeric_value() {
        aria.insert("max_numeric_value".into(), json!(v));
    }
    if let Some(v) = node.numeric_value_step() {
        aria.insert("numeric_value_step".into(), json!(v));
    }

    // Set / table properties.
    if let Some(v) = node.level() {
        aria.insert("level".into(), json!(v));
    }
    if let Some(v) = node.position_in_set() {
        aria.insert("position_in_set".into(), json!(v));
    }
    if let Some(v) = node.size_of_set() {
        aria.insert("size_of_set".into(), json!(v));
    }
    if let Some(v) = node.row_index() {
        aria.insert("row_index".into(), json!(v));
    }
    if let Some(v) = node.column_index() {
        aria.insert("column_index".into(), json!(v));
    }
    if let Some(v) = node.row_count() {
        aria.insert("row_count".into(), json!(v));
    }
    if let Some(v) = node.column_count() {
        aria.insert("column_count".into(), json!(v));
    }

    map.insert("aria".into(), serde_json::Value::Object(aria));

    serde_json::Value::Object(map)
}

/// Maps a 0-based index to a short id in the sequence `a, b, ..., z, aa, ab,
/// ...` (bijective base-26).
fn ephemeral_id(mut index: usize) -> String {
    let mut bytes = Vec::new();
    loop {
        bytes.push(b'a' + (index % 26) as u8);
        if index < 26 {
            break;
        }
        index = index / 26 - 1;
    }
    bytes.reverse();
    String::from_utf8(bytes).unwrap_or_default()
}
