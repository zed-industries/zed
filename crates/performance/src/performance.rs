use std::time::Instant;

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

const SHOW_STARTUP_TIME_DURATION: std::time::Duration = std::time::Duration::from_secs(5);

pub fn init(cx: &mut AppContext) {
    PerformanceSettings::register(cx);

    let mut enabled = PerformanceSettings::get_global(cx)
        .show_in_status_bar
        .unwrap_or(false);
    let start_time = Instant::now();
    let mut _observe_workspaces = toggle_status_bar_items(enabled, start_time, cx);

    cx.observe_global::<SettingsStore>(move |cx| {
        let new_value = PerformanceSettings::get_global(cx)
            .show_in_status_bar
            .unwrap_or(false);
        if new_value != enabled {
            enabled = new_value;
            _observe_workspaces = toggle_status_bar_items(enabled, start_time, cx);
        }
    })
    .detach();
}

fn toggle_status_bar_items(
    enabled: bool,
    start_time: Instant,
    cx: &mut AppContext,
) -> Option<Subscription> {
    for window in cx.windows() {
        if let Some(workspace) = window.downcast::<Workspace>() {
            workspace
                .update(cx, |workspace, cx| {
                    toggle_status_bar_item(workspace, enabled, start_time, cx);
                })
                .ok();
        }
    }

    if enabled {
        log::info!("performance metrics display enabled");
        Some(cx.observe_new_views::<Workspace>(move |workspace, cx| {
            toggle_status_bar_item(workspace, true, start_time, cx);
        }))
    } else {
        log::info!("performance metrics display disabled");
        None
    }
}

struct PerformanceStatusBarItem {
    display_mode: DisplayMode,
}

#[derive(Copy, Clone, Debug)]
enum DisplayMode {
    StartupTime,
    Fps,
}

impl PerformanceStatusBarItem {
    fn new(start_time: Instant, cx: &mut ViewContext<Self>) -> Self {
        let now = Instant::now();
        let display_mode = if now < start_time + SHOW_STARTUP_TIME_DURATION {
            DisplayMode::StartupTime
        } else {
            DisplayMode::Fps
        };

        let this = Self { display_mode };

        if let DisplayMode::StartupTime = display_mode {
            cx.spawn(|this, mut cx| async move {
                let now = Instant::now();
                let remaining_duration =
                    (start_time + SHOW_STARTUP_TIME_DURATION).saturating_duration_since(now);
                cx.background_executor().timer(remaining_duration).await;
                this.update(&mut cx, |this, cx| {
                    this.display_mode = DisplayMode::Fps;
                    cx.notify();
                })
                .ok();
            })
            .detach();
        }

        this
    }
}

impl Render for PerformanceStatusBarItem {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        let text = match self.display_mode {
            DisplayMode::StartupTime => cx
                .time_to_first_window_draw()
                .map_or("Pending".to_string(), |duration| {
                    format!("{}ms", duration.as_millis())
                }),
            DisplayMode::Fps => cx.fps().map_or("".to_string(), |fps| {
                format!("{:3} FPS", fps.round() as u32)
            }),
        };

        use gpui::ParentElement;
        let display_mode = self.display_mode;
        div()
            .id("performance status")
            .child(Label::new(text).size(LabelSize::Small))
            .tooltip(move |cx| match display_mode {
                DisplayMode::StartupTime => Tooltip::text("Time to first window draw", cx),
                DisplayMode::Fps => cx
                    .new_view(|cx| {
                        let tooltip = Tooltip::new("Current FPS");
                        if let Some(time_to_first) = cx.time_to_first_window_draw() {
                            tooltip.meta(format!(
                                "Time to first window draw: {}ms",
                                time_to_first.as_millis()
                            ))
                        } else {
                            tooltip
                        }
                    })
                    .into(),
            })
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
    start_time: Instant,
    cx: &mut ViewContext<Workspace>,
) {
    if enabled {
        workspace.status_bar().update(cx, |bar, cx| {
            bar.add_right_item(
                cx.new_view(|cx| PerformanceStatusBarItem::new(start_time, cx)),
                cx,
            )
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
