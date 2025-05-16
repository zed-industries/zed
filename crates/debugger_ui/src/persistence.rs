use collections::HashMap;
use dap::{Capabilities, adapters::DebugAdapterName};
use db::kvp::KEY_VALUE_STORE;
use gpui::{Axis, Context, Entity, EntityId, Focusable, Subscription, WeakEntity, Window};
use project::Project;
use serde::{Deserialize, Serialize};
use ui::{App, SharedString};
use util::ResultExt;
use workspace::{Member, Pane, PaneAxis, Workspace};

use crate::session::running::{
    self, DebugTerminal, RunningState, SubView, breakpoint_list::BreakpointList, console::Console,
    loaded_source_list::LoadedSourceList, module_list::ModuleList,
    stack_frame_list::StackFrameList, variable_list::VariableList,
};

#[derive(Clone, Hash, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) enum DebuggerPaneItem {
    Console,
    Variables,
    BreakpointList,
    Frames,
    Modules,
    LoadedSources,
    Terminal,
}

impl DebuggerPaneItem {
    pub(crate) fn all() -> &'static [DebuggerPaneItem] {
        static VARIANTS: &[DebuggerPaneItem] = &[
            DebuggerPaneItem::Console,
            DebuggerPaneItem::Variables,
            DebuggerPaneItem::BreakpointList,
            DebuggerPaneItem::Frames,
            DebuggerPaneItem::Modules,
            DebuggerPaneItem::LoadedSources,
            DebuggerPaneItem::Terminal,
        ];
        VARIANTS
    }

    pub(crate) fn is_supported(&self, capabilities: &Capabilities) -> bool {
        match self {
            DebuggerPaneItem::Modules => capabilities.supports_modules_request.unwrap_or_default(),
            DebuggerPaneItem::LoadedSources => capabilities
                .supports_loaded_sources_request
                .unwrap_or_default(),
            _ => true,
        }
    }

    pub(crate) fn to_shared_string(self) -> SharedString {
        match self {
            DebuggerPaneItem::Console => SharedString::new_static("Console"),
            DebuggerPaneItem::Variables => SharedString::new_static("Variables"),
            DebuggerPaneItem::BreakpointList => SharedString::new_static("Breakpoints"),
            DebuggerPaneItem::Frames => SharedString::new_static("Frames"),
            DebuggerPaneItem::Modules => SharedString::new_static("Modules"),
            DebuggerPaneItem::LoadedSources => SharedString::new_static("Sources"),
            DebuggerPaneItem::Terminal => SharedString::new_static("Terminal"),
        }
    }
}

impl From<DebuggerPaneItem> for SharedString {
    fn from(item: DebuggerPaneItem) -> Self {
        item.to_shared_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SerializedLayout {
    pub(crate) panes: SerializedPaneLayout,
    pub(crate) dock_axis: Axis,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) enum SerializedPaneLayout {
    Pane(SerializedPane),
    Group {
        axis: Axis,
        flexes: Option<Vec<f32>>,
        children: Vec<SerializedPaneLayout>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct SerializedPane {
    pub children: Vec<DebuggerPaneItem>,
    pub active_item: Option<DebuggerPaneItem>,
}

const DEBUGGER_PANEL_PREFIX: &str = "debugger_panel_";

pub(crate) async fn serialize_pane_layout(
    adapter_name: DebugAdapterName,
    pane_group: SerializedLayout,
) -> anyhow::Result<()> {
    if let Ok(serialized_pane_group) = serde_json::to_string(&pane_group) {
        KEY_VALUE_STORE
            .write_kvp(
                format!("{DEBUGGER_PANEL_PREFIX}-{adapter_name}"),
                serialized_pane_group,
            )
            .await
    } else {
        Err(anyhow::anyhow!(
            "Failed to serialize pane group with serde_json as a string"
        ))
    }
}

pub(crate) fn build_serialized_layout(
    pane_group: &Member,
    dock_axis: Axis,
    cx: &App,
) -> SerializedLayout {
    SerializedLayout {
        dock_axis,
        panes: build_serialized_pane_layout(pane_group, cx),
    }
}

pub(crate) fn build_serialized_pane_layout(pane_group: &Member, cx: &App) -> SerializedPaneLayout {
    match pane_group {
        Member::Axis(PaneAxis {
            axis,
            members,
            flexes,
            bounding_boxes: _,
        }) => SerializedPaneLayout::Group {
            axis: *axis,
            children: members
                .iter()
                .map(|member| build_serialized_pane_layout(member, cx))
                .collect::<Vec<_>>(),
            flexes: Some(flexes.lock().clone()),
        },
        Member::Pane(pane_handle) => SerializedPaneLayout::Pane(serialize_pane(pane_handle, cx)),
    }
}

fn serialize_pane(pane: &Entity<Pane>, cx: &App) -> SerializedPane {
    let pane = pane.read(cx);
    let children = pane
        .items()
        .filter_map(|item| {
            item.act_as::<SubView>(cx)
                .map(|view| view.read(cx).view_kind())
        })
        .collect::<Vec<_>>();

    let active_item = pane
        .active_item()
        .and_then(|item| item.act_as::<SubView>(cx))
        .map(|view| view.read(cx).view_kind());

    SerializedPane {
        children,
        active_item,
    }
}

pub(crate) async fn get_serialized_layout(
    adapter_name: impl AsRef<str>,
) -> Option<SerializedLayout> {
    let key = format!("{DEBUGGER_PANEL_PREFIX}-{}", adapter_name.as_ref());

    KEY_VALUE_STORE
        .read_kvp(&key)
        .log_err()
        .flatten()
        .and_then(|value| serde_json::from_str::<SerializedLayout>(&value).ok())
}

pub(crate) fn deserialize_pane_layout(
    serialized: SerializedPaneLayout,
    should_invert: bool,
    workspace: &WeakEntity<Workspace>,
    project: &Entity<Project>,
    stack_frame_list: &Entity<StackFrameList>,
    variable_list: &Entity<VariableList>,
    module_list: &Entity<ModuleList>,
    console: &Entity<Console>,
    breakpoint_list: &Entity<BreakpointList>,
    loaded_sources: &Entity<LoadedSourceList>,
    terminal: &Entity<DebugTerminal>,
    subscriptions: &mut HashMap<EntityId, Subscription>,
    window: &mut Window,
    cx: &mut Context<RunningState>,
) -> Option<Member> {
    match serialized {
        SerializedPaneLayout::Group {
            axis,
            flexes,
            children,
        } => {
            let mut members = Vec::new();
            for child in children {
                if let Some(new_member) = deserialize_pane_layout(
                    child,
                    should_invert,
                    workspace,
                    project,
                    stack_frame_list,
                    variable_list,
                    module_list,
                    console,
                    breakpoint_list,
                    loaded_sources,
                    terminal,
                    subscriptions,
                    window,
                    cx,
                ) {
                    members.push(new_member);
                }
            }

            if members.is_empty() {
                return None;
            }

            if members.len() == 1 {
                return Some(members.remove(0));
            }

            Some(Member::Axis(PaneAxis::load(
                if should_invert { axis.invert() } else { axis },
                members,
                flexes.clone(),
            )))
        }
        SerializedPaneLayout::Pane(serialized_pane) => {
            let pane = running::new_debugger_pane(workspace.clone(), project.clone(), window, cx);
            subscriptions.insert(
                pane.entity_id(),
                cx.subscribe_in(&pane, window, RunningState::handle_pane_event),
            );

            let sub_views: Vec<_> = serialized_pane
                .children
                .iter()
                .map(|child| match child {
                    DebuggerPaneItem::Frames => Box::new(SubView::new(
                        stack_frame_list.focus_handle(cx),
                        stack_frame_list.clone().into(),
                        DebuggerPaneItem::Frames,
                        None,
                        cx,
                    )),
                    DebuggerPaneItem::Variables => Box::new(SubView::new(
                        variable_list.focus_handle(cx),
                        variable_list.clone().into(),
                        DebuggerPaneItem::Variables,
                        None,
                        cx,
                    )),
                    DebuggerPaneItem::BreakpointList => Box::new(SubView::new(
                        breakpoint_list.focus_handle(cx),
                        breakpoint_list.clone().into(),
                        DebuggerPaneItem::BreakpointList,
                        None,
                        cx,
                    )),
                    DebuggerPaneItem::Modules => Box::new(SubView::new(
                        module_list.focus_handle(cx),
                        module_list.clone().into(),
                        DebuggerPaneItem::Modules,
                        None,
                        cx,
                    )),
                    DebuggerPaneItem::LoadedSources => Box::new(SubView::new(
                        loaded_sources.focus_handle(cx),
                        loaded_sources.clone().into(),
                        DebuggerPaneItem::LoadedSources,
                        None,
                        cx,
                    )),
                    DebuggerPaneItem::Console => Box::new(SubView::new(
                        console.focus_handle(cx),
                        console.clone().into(),
                        DebuggerPaneItem::Console,
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
                    DebuggerPaneItem::Terminal => Box::new(SubView::new(
                        terminal.focus_handle(cx),
                        terminal.clone().into(),
                        DebuggerPaneItem::Terminal,
                        None,
                        cx,
                    )),
                })
                .collect();

            pane.update(cx, |pane, cx| {
                let mut active_idx = 0;
                for (idx, sub_view) in sub_views.into_iter().enumerate() {
                    if serialized_pane
                        .active_item
                        .is_some_and(|active| active == sub_view.read(cx).view_kind())
                    {
                        active_idx = idx;
                    }
                    pane.add_item(sub_view, false, false, None, window, cx);
                }

                pane.activate_item(active_idx, false, false, window, cx);
            });

            Some(Member::Pane(pane.clone()))
        }
    }
}

#[cfg(test)]
impl SerializedPaneLayout {
    pub(crate) fn in_order(&self) -> Vec<SerializedPaneLayout> {
        let mut panes = vec![];

        Self::inner_in_order(&self, &mut panes);
        panes
    }

    fn inner_in_order(&self, panes: &mut Vec<SerializedPaneLayout>) {
        match self {
            SerializedPaneLayout::Pane(_) => panes.push((*self).clone()),
            SerializedPaneLayout::Group {
                axis: _,
                flexes: _,
                children,
            } => {
                for child in children {
                    child.inner_in_order(panes);
                }
            }
        }
    }
}
