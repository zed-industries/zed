use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The tab bar placement in a pane.
///
/// Default: top
#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TabBarPlacement {
    /// Don't show tab bar.
    No,
    /// Place tab bar on top of the pane.
    Top,
    /// Place tab bar at the bottom of the pane.
    Bottom,
}
