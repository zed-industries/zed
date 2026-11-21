//! Validation of [`LayoutId`](super::LayoutId)s, enabled by the `layout-id-debug` feature.
//!
//! Each ID is tagged with the layout engine (one per window) and frame that
//! issued it. Using an ID outside that engine and frame, or one whose node can't
//! be a slotmap key, panics with a message saying which kind of invalid ID it is
//! and where it was used, rather than Taffy's uninformative slotmap panic.

use std::sync::atomic::{AtomicU32, Ordering};
use taffy::tree::NodeId;

/// Issues engine identities. Zero is never issued, so a zeroed tag can't match
/// any engine.
static NEXT_ENGINE_ID: AtomicU32 = AtomicU32::new(1);

/// The engine and frame a [`LayoutId`](super::LayoutId) was issued in.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(super) struct LayoutIdTag {
    engine_id: u32,
    /// Starts at 1 so that a zeroed tag never matches.
    frame: u32,
}

impl LayoutIdTag {
    /// Returns the tag for a new engine's first frame.
    pub(super) fn new_engine() -> Self {
        Self {
            engine_id: NEXT_ENGINE_ID.fetch_add(1, Ordering::Relaxed),
            frame: 1,
        }
    }

    /// Moves to the engine's next frame, invalidating IDs from this one.
    pub(super) fn advance_frame(&mut self) {
        self.frame = self.frame.wrapping_add(1).max(1);
    }

    /// Panics if an ID with tag `id` and node `node` isn't valid for the engine
    /// whose current tag is `self`.
    #[inline]
    pub(super) fn check(self, id: Self, node: NodeId, describe_use: impl FnOnce() -> String) {
        if id != self || malformed_node_reason(node).is_some() {
            self.invalid(id, node, &describe_use());
        }
    }

    #[cold]
    #[inline(never)]
    fn invalid(self, id: Self, node: NodeId, use_description: &str) -> ! {
        panic!(
            "invalid LayoutId ({}): {use_description}; id node {:#018x} engine {} frame {}, current engine {} frame {}",
            self.classify(id, node),
            u64::from(node),
            id.engine_id,
            id.frame,
            self.engine_id,
            self.frame,
        )
    }

    fn classify(self, id: Self, node: NodeId) -> String {
        if u64::from(node) == 0 && id.engine_id == 0 && id.frame == 0 {
            "zeroed".into()
        } else if id.engine_id == 0
            || id.frame == 0
            || id.engine_id >= NEXT_ENGINE_ID.load(Ordering::Relaxed)
        {
            "corrupt: never issued".into()
        } else if id.engine_id != self.engine_id {
            "foreign: issued by another window's layout engine".into()
        } else if id.frame < self.frame {
            format!("stale: issued {} frames ago", self.frame - id.frame)
        } else if id.frame > self.frame {
            "corrupt: issued in a future frame".into()
        } else if let Some(reason) = malformed_node_reason(node) {
            format!("malformed node: {reason}")
        } else {
            unreachable!("classified a valid LayoutId")
        }
    }
}

/// Returns why `node` can't be a key into Taffy's slotmaps, if it can't.
///
/// A [`NodeId`] stores the slotmap version in its high 32 bits and the slot
/// index in its low 32 bits. Slotmap reserves index 0 as a sentinel and gives
/// occupied slots odd versions, so other patterns can only come from corrupted
/// memory.
fn malformed_node_reason(node: NodeId) -> Option<&'static str> {
    let bits = u64::from(node);
    if bits as u32 == 0 {
        Some("slot index 0")
    } else if (bits >> 32) % 2 == 0 {
        Some("even slot version")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Pixels, Style,
        taffy::{LayoutId, TaffyLayoutEngine},
    };

    #[test]
    #[should_panic(
        expected = "invalid LayoutId (stale: issued 1 frames ago): passed to layout_bounds"
    )]
    fn layout_id_from_previous_frame_is_reported_as_stale() {
        let mut engine = TaffyLayoutEngine::new();
        let id = request_leaf(&mut engine);
        engine.clear();
        engine.layout_bounds(id, 1.0);
    }

    #[test]
    #[should_panic(
        expected = "invalid LayoutId (foreign: issued by another window's layout engine): child 0 of 1 passed to request_layout"
    )]
    fn layout_id_from_another_engine_is_reported_as_foreign() {
        let mut other_engine = TaffyLayoutEngine::new();
        let foreign_id = request_leaf(&mut other_engine);
        let mut engine = TaffyLayoutEngine::new();
        engine.request_layout(Style::default(), Pixels(16.0), 1.0, &[foreign_id]);
    }

    #[test]
    #[should_panic(
        expected = "invalid LayoutId (malformed node: slot index 0): passed to layout_bounds"
    )]
    fn layout_id_with_zeroed_node_is_reported_as_malformed() {
        let mut engine = TaffyLayoutEngine::new();
        let mut id = request_leaf(&mut engine);
        id.node = NodeId::from(0_u64);
        engine.layout_bounds(id, 1.0);
    }

    #[test]
    #[should_panic(
        expected = "invalid LayoutId (malformed node: even slot version): passed to layout_bounds"
    )]
    fn layout_id_with_even_node_version_is_reported_as_malformed() {
        let mut engine = TaffyLayoutEngine::new();
        let mut id = request_leaf(&mut engine);
        id.node = NodeId::from(u64::from(id.node) + (1 << 32));
        engine.layout_bounds(id, 1.0);
    }

    #[test]
    #[should_panic(expected = "invalid LayoutId (zeroed): child 1 of 2 passed to request_layout")]
    fn zeroed_layout_id_is_reported_as_zeroed() {
        let mut engine = TaffyLayoutEngine::new();
        let valid_id = request_leaf(&mut engine);
        let zeroed_id = LayoutId {
            node: NodeId::from(0_u64),
            tag: LayoutIdTag {
                engine_id: 0,
                frame: 0,
            },
        };
        engine.request_layout(Style::default(), Pixels(16.0), 1.0, &[valid_id, zeroed_id]);
    }

    fn request_leaf(engine: &mut TaffyLayoutEngine) -> LayoutId {
        engine.request_layout(Style::default(), Pixels(16.0), 1.0, &[])
    }
}
