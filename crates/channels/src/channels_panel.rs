use std::sync::Arc;

use crate::{
    channels_panel_settings::{ChannelsPanelDockPosition, ChannelsPanelSettings},
    Channel, Channels,
};
use anyhow::Result;
use collections::HashMap;
use context_menu::ContextMenu;
use db::kvp::KEY_VALUE_STORE;
use gpui::{
    actions,
    elements::{ChildView, Empty, Flex, Label, MouseEventHandler, ParentElement, Stack},
    serde_json, AnyElement, AppContext, AsyncAppContext, Element, Entity, ModelHandle, Task, View,
    ViewContext, ViewHandle, WeakViewHandle,
};
use project::Fs;
use serde_derive::{Deserialize, Serialize};
use settings::SettingsStore;
use theme::ChannelTreeStyle;
use util::{ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel},
    Workspace,
};

actions!(channels, [ToggleFocus]);

const CHANNELS_PANEL_KEY: &'static str = "ChannelsPanel";

pub fn init(cx: &mut AppContext) {
    settings::register::<ChannelsPanelSettings>(cx);
}

pub struct ChannelsPanel {
    width: Option<f32>,
    fs: Arc<dyn Fs>,
    has_focus: bool,
    pending_serialization: Task<Option<()>>,
    channels: ModelHandle<Channels>,
    context_menu: ViewHandle<ContextMenu>,
    collapsed_channels: HashMap<u64, bool>,
}

#[derive(Serialize, Deserialize)]
struct SerializedChannelsPanel {
    width: Option<f32>,
    collapsed_channels: Option<HashMap<u64, bool>>,
}

#[derive(Debug)]
pub enum Event {
    DockPositionChanged,
    Focus,
}

impl Entity for ChannelsPanel {
    type Event = Event;
}

impl ChannelsPanel {
    pub fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> ViewHandle<Self> {
        cx.add_view(|cx| {
            let view_id = cx.view_id();
            let this = Self {
                width: None,
                has_focus: false,
                fs: workspace.app_state().fs.clone(),
                pending_serialization: Task::ready(None),
                channels: cx.global::<ModelHandle<Channels>>().clone(),
                context_menu: cx.add_view(|cx| ContextMenu::new(view_id, cx)),
                collapsed_channels: HashMap::default(),
            };

            // Update the dock position when the setting changes.
            let mut old_dock_position = this.position(cx);
            cx.observe_global::<SettingsStore, _>(move |this: &mut ChannelsPanel, cx| {
                let new_dock_position = this.position(cx);
                if new_dock_position != old_dock_position {
                    old_dock_position = new_dock_position;
                    cx.emit(Event::DockPositionChanged);
                }
            })
            .detach();

            this
        })
    }

    pub fn load(
        workspace: WeakViewHandle<Workspace>,
        cx: AsyncAppContext,
    ) -> Task<Result<ViewHandle<Self>>> {
        cx.spawn(|mut cx| async move {
            let serialized_panel = if let Some(panel) = cx
                .background()
                .spawn(async move { KEY_VALUE_STORE.read_kvp(CHANNELS_PANEL_KEY) })
                .await
                .log_err()
                .flatten()
            {
                Some(serde_json::from_str::<SerializedChannelsPanel>(&panel)?)
            } else {
                None
            };

            workspace.update(&mut cx, |workspace, cx| {
                let panel = ChannelsPanel::new(workspace, cx);
                if let Some(serialized_panel) = serialized_panel {
                    panel.update(cx, |panel, cx| {
                        panel.width = serialized_panel.width;
                        panel.collapsed_channels =
                            serialized_panel.collapsed_channels.unwrap_or_default();
                        cx.notify();
                    });
                }
                panel
            })
        })
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let width = self.width;
        let collapsed_channels = self.collapsed_channels.clone();
        self.pending_serialization = cx.background().spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        CHANNELS_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedChannelsPanel {
                            width,
                            collapsed_channels: Some(collapsed_channels),
                        })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }

    fn render_channel(
        &mut self,
        depth: usize,
        channel: &Channel,
        style: &ChannelTreeStyle,
        root: bool,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        let has_chilren = !channel.members().is_empty();

        let sub_channel_details = has_chilren.then(|| {
            let mut sub_channels = Flex::column();
            let collapsed = self
                .collapsed_channels
                .get(&channel.id)
                .copied()
                .unwrap_or_default();
            if !collapsed {
                for sub_channel in channel.members() {
                    sub_channels = sub_channels.with_child(self.render_channel(
                        depth + 1,
                        sub_channel,
                        style,
                        false,
                        cx,
                    ));
                }
            }
            (sub_channels, collapsed)
        });

        let channel_id = channel.id;

        enum ChannelCollapser {}
        Flex::row()
            .with_child(
                Empty::new()
                    .constrained()
                    .with_width(depth as f32 * style.channel_indent),
            )
            .with_child(
                Flex::column()
                    .with_child(
                        Flex::row()
                            .with_child(
                                sub_channel_details
                                    .as_ref()
                                    .map(|(_, expanded)| {
                                        MouseEventHandler::<ChannelCollapser, _>::new(
                                            channel.id as usize,
                                            cx,
                                            |state, _cx| {
                                                let icon =
                                                    style.channel_icon.style_for(!*expanded, state);
                                                theme::ui::icon(icon)
                                            },
                                        )
                                        .on_click(
                                            gpui::platform::MouseButton::Left,
                                            move |_, v, cx| {
                                                let entry = v
                                                    .collapsed_channels
                                                    .entry(channel_id)
                                                    .or_default();
                                                *entry = !*entry;
                                                v.serialize(cx);
                                                cx.notify();
                                            },
                                        )
                                        .into_any()
                                    })
                                    .unwrap_or_else(|| {
                                        Empty::new()
                                            .constrained()
                                            .with_width(style.channel_icon.default_style().width())
                                            .into_any()
                                    }),
                            )
                            .with_child(
                                Label::new(
                                    channel.name().to_string(),
                                    if root {
                                        style.root_name.clone()
                                    } else {
                                        style.channel_name.clone()
                                    },
                                )
                                .into_any(),
                            ),
                    )
                    .with_children(sub_channel_details.map(|(elements, _)| elements)),
            )
            .into_any()
    }
}

impl View for ChannelsPanel {
    fn ui_name() -> &'static str {
        "ChannelsPanel"
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if !self.has_focus {
            self.has_focus = true;
            cx.emit(Event::Focus);
        }
    }

    fn focus_out(&mut self, _: gpui::AnyViewHandle, _: &mut ViewContext<Self>) {
        self.has_focus = false;
    }

    fn render(&mut self, cx: &mut gpui::ViewContext<'_, '_, Self>) -> gpui::AnyElement<Self> {
        let theme = theme::current(cx).clone();

        let mut channels_column = Flex::column();
        for channel in self.channels.read(cx).channels() {
            channels_column = channels_column.with_child(self.render_channel(
                0,
                &channel,
                &theme.channels_panel.channel_tree,
                true,
                cx,
            ));
        }

        let spacing = theme.channels_panel.spacing;

        enum ChannelsPanelScrollTag {}
        Stack::new()
            .with_child(
                // Full panel column
                Flex::column()
                    .with_spacing(spacing)
                    .with_child(
                        // Channels section column
                        Flex::column()
                            .with_child(
                                Flex::row().with_child(
                                    Label::new(
                                        "Active Channels",
                                        theme.editor.invalid_information_diagnostic.message.clone(),
                                    )
                                    .into_any(),
                                ),
                            )
                            // Channels list column
                            .with_child(channels_column),
                    )
                    // TODO: Replace with spacing implementation
                    .with_child(Empty::new().constrained().with_height(spacing))
                    .with_child(
                        Flex::column().with_child(
                            Flex::row().with_child(
                                Label::new(
                                    "Contacts",
                                    theme.editor.invalid_information_diagnostic.message.clone(),
                                )
                                .into_any(),
                            ),
                        ),
                    )
                    .scrollable::<ChannelsPanelScrollTag>(0, None, cx)
                    .expanded(),
            )
            .with_child(ChildView::new(&self.context_menu, cx))
            .into_any_named("channels panel")
            .into_any()
    }
}

impl Panel for ChannelsPanel {
    fn position(&self, cx: &gpui::WindowContext) -> DockPosition {
        match settings::get::<ChannelsPanelSettings>(cx).dock {
            ChannelsPanelDockPosition::Left => DockPosition::Left,
            ChannelsPanelDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<ChannelsPanelSettings>(
            self.fs.clone(),
            cx,
            move |settings| {
                let dock = match position {
                    DockPosition::Left | DockPosition::Bottom => ChannelsPanelDockPosition::Left,
                    DockPosition::Right => ChannelsPanelDockPosition::Right,
                };
                settings.dock = Some(dock);
            },
        );
    }

    fn size(&self, cx: &gpui::WindowContext) -> f32 {
        self.width
            .unwrap_or_else(|| settings::get::<ChannelsPanelSettings>(cx).default_width)
    }

    fn set_size(&mut self, size: f32, cx: &mut ViewContext<Self>) {
        self.width = Some(size);
        self.serialize(cx);
        cx.notify();
    }

    fn icon_path(&self) -> &'static str {
        "icons/bolt_16.svg"
    }

    fn icon_tooltip(&self) -> (String, Option<Box<dyn gpui::Action>>) {
        ("Channels Panel".to_string(), Some(Box::new(ToggleFocus)))
    }

    fn should_change_position_on_event(event: &Self::Event) -> bool {
        matches!(event, Event::DockPositionChanged)
    }

    fn has_focus(&self, _cx: &gpui::WindowContext) -> bool {
        self.has_focus
    }

    fn is_focus_event(event: &Self::Event) -> bool {
        matches!(event, Event::Focus)
    }
}
