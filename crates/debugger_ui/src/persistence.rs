use db::kvp::KEY_VALUE_STORE;
use gpui::{Axis, Context, Entity, Focusable, WeakEntity, Window};
use project::{Project, debugger::session::Session};
use serde::{Deserialize, Serialize};
use ui::{App, SharedString};
use util::ResultExt;
use workspace::{Member, Pane, PaneAxis, PaneGroup, Workspace};

use crate::session::running::{
    self, RunningState, SubView, breakpoint_list::BreakpointList, console::Console,
    module_list::ModuleList, stack_frame_list::StackFrameList, variable_list::VariableList,
};

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
            .write_kvp(
                format!("{}-{adapter_name}", db::kvp::DEBUGGER_PANEL_PREFIX),
                serialized_pane_group,
            )
            .await
    } else {
        Err(anyhow::anyhow!(
            "Failed to serialize pane group with serde_json as a string"
        ))
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

pub(crate) async fn get_serialized_pane(
    adapter_name: impl AsRef<str>,
) -> Option<SerializedPaneGroup> {
    let key = format!(
        "{}-{}",
        db::kvp::DEBUGGER_PANEL_PREFIX,
        adapter_name.as_ref()
    );

    KEY_VALUE_STORE
        .read_kvp(&key)
        .log_err()
        .flatten()
        .and_then(|value| serde_json::from_str::<SerializedPaneGroup>(&value).ok())
}

pub(crate) fn deserialize_pane_group(
    serialized: &SerializedPaneGroup,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    session: Entity<Session>,
    stack_frame_list: Entity<StackFrameList>,
    variable_list: Entity<VariableList>,
    module_list: Entity<ModuleList>,
    console: Entity<Console>,
    breakpoint_list: Entity<BreakpointList>,
    window: &mut Window,
    cx: &mut Context<RunningState>,
) -> Option<(Member, Option<Entity<Pane>>)> {
    match serialized {
        SerializedPaneGroup::Group {
            axis,
            flexes,
            children,
        } => {
            let mut current_active_pane = None;
            let mut members = Vec::new();
            for child in children {
                if let Some((new_member, active_pane)) = deserialize_pane_group(
                    child,
                    workspace.clone(),
                    project.clone(),
                    session.clone(),
                    stack_frame_list.clone(),
                    variable_list.clone(),
                    module_list.clone(),
                    console.clone(),
                    breakpoint_list.clone(),
                    window,
                    cx,
                ) {
                    members.push(new_member);
                    current_active_pane = current_active_pane.or(active_pane);
                }
            }

            if members.is_empty() {
                return None;
            }

            if members.len() == 1 {
                return Some((members.remove(0), current_active_pane));
            }

            Some((
                Member::Axis(PaneAxis::load(axis.0, members, flexes.clone())),
                current_active_pane,
            ))
        }
        SerializedPaneGroup::Pane(serialized_pane) => {
            let active = serialized_pane.active;
            let pane = running::new_debugger_pane(workspace.clone(), project.clone(), window, cx);

            let sub_views: Vec<_> = serialized_pane
                .children
                .iter()
                .map(|child| match child {
                    DebuggerPaneItem::Frames => Box::new(SubView::new(
                        pane.focus_handle(cx),
                        stack_frame_list.clone().into(),
                        DebuggerPaneItem::Frames.to_shared_string(),
                        None,
                        cx,
                    )),
                    DebuggerPaneItem::Variables => Box::new(SubView::new(
                        variable_list.focus_handle(cx),
                        variable_list.clone().into(),
                        DebuggerPaneItem::Variables.to_shared_string(),
                        None,
                        cx,
                    )),
                    DebuggerPaneItem::BreakpointList => Box::new(SubView::new(
                        breakpoint_list.focus_handle(cx),
                        breakpoint_list.clone().into(),
                        DebuggerPaneItem::BreakpointList.to_shared_string(),
                        None,
                        cx,
                    )),
                    DebuggerPaneItem::Modules => Box::new(SubView::new(
                        pane.focus_handle(cx),
                        module_list.clone().into(),
                        DebuggerPaneItem::Modules.to_shared_string(),
                        None,
                        cx,
                    )),

                    DebuggerPaneItem::Console => Box::new(SubView::new(
                        pane.focus_handle(cx),
                        console.clone().into(),
                        DebuggerPaneItem::Console.to_shared_string(),
                        Some(Box::new({
                            let console = console.clone().downgrade();
                            move |cx| {
                                console
                                    .read_with(cx, |console, cx| console.show_indicator(cx))
                                    .unwrap_or_default()
                            }
                        })),
                        cx,
                    )),
                })
                .collect();

            pane.update(cx, |pane, cx| {
                for sub_view in sub_views.into_iter() {
                    pane.add_item(sub_view, false, false, None, window, cx);
                }
            });

            Some((Member::Pane(pane.clone()), active.then_some(pane)))
        }
    }
}
