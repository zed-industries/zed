use std::fmt::{self, Write as _};

/// An exact, test-only snapshot of the observable output of one GPUI frame.
///
/// The snapshot preserves ordering within every lane. Floating-point values are
/// encoded using `to_bits`, so `-0.0`, `0.0`, and distinct NaN payloads remain
/// distinguishable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameSnapshot {
    pub(crate) scene: Vec<FrameSnapshotItem>,
    pub(crate) hitboxes: Vec<FrameSnapshotItem>,
    pub(crate) window_control_hitboxes: Vec<FrameSnapshotItem>,
    pub(crate) focus_path: Vec<usize>,
    pub(crate) dispatch_tree: Vec<FrameSnapshotItem>,
    pub(crate) mouse_listeners: Vec<bool>,
    pub(crate) input_handlers: Vec<bool>,
    pub(crate) tooltip_requests: Vec<bool>,
    pub(crate) cursor_styles: Vec<FrameSnapshotItem>,
    pub(crate) tab_stops: Vec<FrameSnapshotItem>,
    pub(crate) deferred_draws: Vec<FrameSnapshotItem>,
    pub(crate) accessibility_update: Option<String>,
}

impl FrameSnapshot {
    /// Produces a labeled report containing at most `limit` divergences.
    pub fn pretty_diff(&self, other: &Self, limit: usize) -> FrameSnapshotDiff {
        let mut divergences = Vec::new();

        compare_lane("scene", &self.scene, &other.scene, limit, &mut divergences);
        compare_lane(
            "hitboxes",
            &self.hitboxes,
            &other.hitboxes,
            limit,
            &mut divergences,
        );
        compare_lane(
            "window_control_hitboxes",
            &self.window_control_hitboxes,
            &other.window_control_hitboxes,
            limit,
            &mut divergences,
        );
        compare_lane(
            "focus_path",
            &self.focus_path,
            &other.focus_path,
            limit,
            &mut divergences,
        );
        compare_lane(
            "dispatch_tree",
            &self.dispatch_tree,
            &other.dispatch_tree,
            limit,
            &mut divergences,
        );
        compare_lane(
            "mouse_listeners",
            &self.mouse_listeners,
            &other.mouse_listeners,
            limit,
            &mut divergences,
        );
        compare_lane(
            "input_handlers",
            &self.input_handlers,
            &other.input_handlers,
            limit,
            &mut divergences,
        );
        compare_lane(
            "tooltip_requests",
            &self.tooltip_requests,
            &other.tooltip_requests,
            limit,
            &mut divergences,
        );
        compare_lane(
            "cursor_styles",
            &self.cursor_styles,
            &other.cursor_styles,
            limit,
            &mut divergences,
        );
        compare_lane(
            "tab_stops",
            &self.tab_stops,
            &other.tab_stops,
            limit,
            &mut divergences,
        );
        compare_lane(
            "deferred_draws",
            &self.deferred_draws,
            &other.deferred_draws,
            limit,
            &mut divergences,
        );

        if divergences.len() < limit && self.accessibility_update != other.accessibility_update {
            divergences.push(FrameSnapshotDivergence {
                lane: "accessibility_update",
                index: 0,
                left: format!("{:?}", self.accessibility_update),
                right: format!("{:?}", other.accessibility_update),
            });
        }

        FrameSnapshotDiff { divergences }
    }

    /// Returns the number of scene paint operations in the snapshot.
    pub fn scene_len(&self) -> usize {
        self.scene.len()
    }
}

/// A pretty, lane-aware difference between two frame snapshots.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameSnapshotDiff {
    divergences: Vec<FrameSnapshotDivergence>,
}

impl FrameSnapshotDiff {
    /// Returns whether the snapshots had no divergences.
    pub fn is_empty(&self) -> bool {
        self.divergences.is_empty()
    }
}

impl fmt::Display for FrameSnapshotDiff {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.divergences.is_empty() {
            return formatter.write_str("frames are identical");
        }

        writeln!(formatter, "{} frame divergence(s):", self.divergences.len())?;
        for divergence in &self.divergences {
            writeln!(
                formatter,
                "- {}[{}]\n    left:  {}\n    right: {}",
                divergence.lane, divergence.index, divergence.left, divergence.right
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FrameSnapshotDivergence {
    lane: &'static str,
    index: usize,
    left: String,
    right: String,
}

#[derive(Clone)]
pub(crate) struct FrameSnapshotItem {
    exact: Vec<u8>,
    label: String,
}

impl PartialEq for FrameSnapshotItem {
    fn eq(&self, other: &Self) -> bool {
        self.exact == other.exact
    }
}

impl Eq for FrameSnapshotItem {}

impl fmt::Debug for FrameSnapshotItem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.label)
    }
}

impl FrameSnapshotItem {
    pub(crate) fn new(label: impl Into<String>) -> Self {
        Self {
            exact: Vec::new(),
            label: label.into(),
        }
    }

    pub(crate) fn push_u64(&mut self, value: u64) {
        self.exact.extend(value.to_le_bytes());
    }

    pub(crate) fn push_usize(&mut self, value: usize) {
        self.push_u64(value as u64);
    }

    pub(crate) fn push_i64(&mut self, value: i64) {
        self.exact.extend(value.to_le_bytes());
    }

    pub(crate) fn push_f32(&mut self, value: f32) {
        self.exact.extend(value.to_bits().to_le_bytes());
    }

    pub(crate) fn push_str(&mut self, value: &str) {
        self.push_usize(value.len());
        self.exact.extend(value.as_bytes());
    }
}

fn compare_lane<T: fmt::Debug + PartialEq>(
    lane: &'static str,
    left: &[T],
    right: &[T],
    limit: usize,
    divergences: &mut Vec<FrameSnapshotDivergence>,
) {
    let common_length = left.len().min(right.len());
    for index in 0..common_length {
        if divergences.len() >= limit {
            return;
        }
        if left[index] != right[index] {
            divergences.push(FrameSnapshotDivergence {
                lane,
                index,
                left: format!("{:?}", left[index]),
                right: format!("{:?}", right[index]),
            });
        }
    }

    if divergences.len() >= limit || left.len() == right.len() {
        return;
    }

    let mut left_label = String::new();
    let mut right_label = String::new();
    write!(&mut left_label, "lane length {}", left.len()).ok();
    write!(&mut right_label, "lane length {}", right.len()).ok();
    divergences.push(FrameSnapshotDivergence {
        lane,
        index: common_length,
        left: left_label,
        right: right_label,
    });
}
