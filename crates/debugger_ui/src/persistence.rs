use collections::HashSet;
use db::kvp::KEY_VALUE_STORE;
use gpui::{Axis, Entity};
use serde::{Deserialize, Serialize};
use ui::{App, SharedString};
use workspace::{Member, Pane, PaneAxis, PaneGroup};

use crate::session::running::SubView;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum DebuggerPaneItem {
    Console,
    Variables,
    BreakpointList,
    Frames,
    Modules,
}

impl DebuggerPaneItem {
    pub(crate) fn from_str(s: impl AsRef<str>) -> Option<Self> {
        match s.as_ref() {
            "Console" => Some(DebuggerPaneItem::Console),
            "Variables" => Some(DebuggerPaneItem::Variables),
            "Breakpoints" => Some(DebuggerPaneItem::BreakpointList),
            "Frames" => Some(DebuggerPaneItem::Frames),
            "Modules" => Some(DebuggerPaneItem::Modules),
            _ => None,
        }
    }

    pub(crate) fn to_shared_string(self) -> SharedString {
        match self {
            DebuggerPaneItem::Console => SharedString::new_static("Console"),
            DebuggerPaneItem::Variables => SharedString::new_static("Variables"),
            DebuggerPaneItem::BreakpointList => SharedString::new_static("Breakpoints"),
            DebuggerPaneItem::Frames => SharedString::new_static("Frames"),
            DebuggerPaneItem::Modules => SharedString::new_static("Modules"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SerializedAxis(pub Axis);

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum SerializedPaneGroup {
    Pane(SerializedPane),
    Group {
        axis: SerializedAxis,
        flexes: Option<Vec<f32>>,
        children: Vec<SerializedPaneGroup>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SerializedPane {
    pub active: bool,
    pub children: Vec<DebuggerPaneItem>,
    pub active_item: Option<DebuggerPaneItem>,
}

pub(crate) async fn serialize_pane_group(
    adapter_name: String,
    pane_group: &PaneGroup,
    active_pane: &Entity<Pane>,
    cx: &mut App,
) -> anyhow::Result<()> {
    let pane_group = build_serialized_pane_group(&pane_group.root, active_pane, cx);

    if let Ok(serialized_pane_group) = serde_json::to_string(&pane_group) {
        KEY_VALUE_STORE
            .write_kvp(adapter_name, serialized_pane_group)
            .await
    } else {
        Err(anyhow::anyhow!("Failed to serialize pane group"))
    }
}

fn build_serialized_pane_group(
    pane_group: &Member,
    active_pane: &Entity<Pane>,
    cx: &mut App,
) -> SerializedPaneGroup {
    match pane_group {
        Member::Axis(PaneAxis {
            axis,
            members,
            flexes,
            bounding_boxes: _,
        }) => SerializedPaneGroup::Group {
            axis: SerializedAxis(*axis),
            children: members
                .iter()
                .map(|member| build_serialized_pane_group(member, active_pane, cx))
                .collect::<Vec<_>>(),
            flexes: Some(flexes.lock().clone()),
        },
        Member::Pane(pane_handle) => {
            SerializedPaneGroup::Pane(serialize_pane(pane_handle, pane_handle == active_pane, cx))
        }
    }
}

fn serialize_pane(pane: &Entity<Pane>, active: bool, cx: &mut App) -> SerializedPane {
    let pane = pane.read(cx);
    let children = pane
        .items()
        .filter_map(|item| {
            item.act_as::<SubView>(cx)
                .and_then(|view| view.read(cx).view_kind())
        })
        .collect::<Vec<_>>();

    let active_item = pane
        .active_item()
        .and_then(|item| item.act_as::<SubView>(cx))
        .and_then(|view| view.read(cx).view_kind());

    SerializedPane {
        active,
        children,
        active_item,
    }
}
