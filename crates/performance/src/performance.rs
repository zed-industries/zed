use anyhow::Result;
use gpui::{
    div, AppContext, InteractiveElement as _, Render, StatefulInteractiveElement as _,
    Subscription, ViewContext, VisualContext,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources, SettingsStore};
use workspace::{
    ui::{Label, LabelCommon, LabelSize, Tooltip},
    ItemHandle, StatusItemView, Workspace,
};

pub fn init(cx: &mut AppContext) {
    PerformanceSettings::register(cx);

    let mut enabled = PerformanceSettings::get_global(cx)
        .show_in_status_bar
        .unwrap_or(false);
    let mut _observe_workspaces = toggle_status_bar_items(enabled, cx);

    cx.observe_global::<SettingsStore>(move |cx| {
        let new_value = PerformanceSettings::get_global(cx)
            .show_in_status_bar
            .unwrap_or(false);
        if new_value != enabled {
            enabled = new_value;
            _observe_workspaces = toggle_status_bar_items(enabled, cx);
        }
    })
    .detach();
}

fn toggle_status_bar_items(enabled: bool, cx: &mut AppContext) -> Option<Subscription> {
    for window in cx.windows() {
        if let Some(workspace) = window.downcast::<Workspace>() {
            workspace
                .update(cx, |workspace, cx| {
                    toggle_status_bar_item(workspace, enabled, cx);
                })
                .ok();
        }
    }

    if enabled {
        log::info!("performance metrics display enabled");
        Some(cx.observe_new_views::<Workspace>(|workspace, cx| {
            toggle_status_bar_item(workspace, true, cx);
        }))
    } else {
        log::info!("Performance metrics display disabled");
        None
    }
}

struct PerformanceStatusBarItem;

impl Render for PerformanceStatusBarItem {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        let text = cx
            .time_to_first_window_draw()
            .map_or("Pending".to_string(), |duration| {
                format!("{}ms", duration.as_millis())
            });

        use gpui::ParentElement;
        div()
            .id("performance status")
            .child(Label::new(text).size(LabelSize::Small))
            .tooltip(|cx| Tooltip::text("Time to first window draw", cx))
    }
}

impl StatusItemView for PerformanceStatusBarItem {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _cx: &mut gpui::ViewContext<Self>,
    ) {
        // This is not currently used.
    }
}

fn toggle_status_bar_item(
    workspace: &mut Workspace,
    enabled: bool,
    cx: &mut ViewContext<Workspace>,
) {
    if enabled {
        workspace.status_bar().update(cx, |bar, cx| {
            bar.add_right_item(cx.new_view(|_cx| PerformanceStatusBarItem), cx)
        });
    } else {
        workspace.status_bar().update(cx, |bar, cx| {
            bar.remove_items_of_type::<PerformanceStatusBarItem>(cx);
        });
    }
}

/// Configuration of the display of performance details.
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct PerformanceSettings {
    /// Display the time to first window draw and frame rate in the status bar.
    ///
    /// Default: false
    pub show_in_status_bar: Option<bool>,
}

impl Settings for PerformanceSettings {
    const KEY: Option<&'static str> = Some("performance");

    type FileContent = Self;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}
