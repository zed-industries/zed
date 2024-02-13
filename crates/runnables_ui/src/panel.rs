use std::{path::PathBuf, sync::Arc};

use crate::runnables_settings::{RunnablesDockPosition, RunnablesSettings};
use crate::status_bar_icon::StatusIconTracker;
use anyhow::{anyhow, Result};
use db::kvp::KEY_VALUE_STORE;
use editor::{Editor, EditorElement, EditorStyle};
use fs::Fs;
use gpui::{
    actions, div, list, px, relative, rems, AsyncWindowContext, EventEmitter, FocusHandle,
    FocusableView, FontStyle, FontWeight, InteractiveElement, IntoElement, ListAlignment,
    ListState, Model, ParentElement as _, Render, SharedString, Styled as _, Task, TextStyle, View,
    ViewContext, VisualContext as _, WeakView, WhiteSpace, WindowContext,
};
use project::Inventory;

use serde::{Deserialize, Serialize};
use settings::Settings as _;
use theme::ThemeSettings;
use ui::{
    prelude::Pixels, v_flex, ActiveTheme, Button, Clickable, Color, FluentBuilder, Icon,
    IconButton, IconName, ListHeader, ListItem, StyledExt,
};
use util::{ResultExt as _, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};

const RUNNABLES_PANEL_KEY: &'static str = "RunnablesPanel";
pub struct RunnablesPanel {
    filter_editor: View<Editor>,
    focus_handle: FocusHandle,
    // todo: po: should this be weak?
    inventory: Model<Inventory>,
    width: Option<Pixels>,
    fs: Arc<dyn Fs>,
    pending_serialization: Task<Option<()>>,
    pub(crate) status_bar_tracker: Option<Model<StatusIconTracker>>,
    workspace: WeakView<Workspace>,
}

#[derive(Serialize, Deserialize)]
struct SerializedRunnablesPanel {
    width: Option<Pixels>,
}

impl RunnablesPanel {
    fn new(
        inventory: Model<Inventory>,
        workspace: WeakView<Workspace>,
        fs: Arc<dyn Fs>,
        cx: &mut WindowContext<'_>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let filter_editor = cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_placeholder_text("Filter...", cx);
                editor
            });
            Self {
                focus_handle: cx.focus_handle(),
                filter_editor,
                inventory,
                width: None,
                fs,
                pending_serialization: Task::ready(None),
                // We always start off as collapsed - if we were serialized as "open", then
                // there'll be a subsequent call to `set_active`.
                // We need to start out with Some tracker to make color tracking without having user open the panel at least once.
                status_bar_tracker: Some(StatusIconTracker::new(vec![], cx)),
                workspace,
            }
        })
    }

    fn render_filter_input(
        &self,
        editor: &View<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if editor.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features,
            font_size: rems(0.875).into(),
            font_weight: FontWeight::NORMAL,
            font_style: FontStyle::Normal,
            line_height: relative(1.3).into(),
            background_color: None,
            underline: None,
            white_space: WhiteSpace::Normal,
            strikethrough: None,
        };

        EditorElement::new(
            editor,
            EditorStyle {
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    pub async fn load(
        workspace: WeakView<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<View<Self>> {
        let serialized_panel = cx
            .background_executor()
            .spawn(async move { KEY_VALUE_STORE.read_kvp(RUNNABLES_PANEL_KEY) })
            .await
            .map_err(|e| anyhow!("Failed to load project panel: {}", e))
            .log_err()
            .flatten()
            .map(|panel| serde_json::from_str::<SerializedRunnablesPanel>(&panel))
            .transpose()
            .log_err()
            .flatten();
        let workspace_view = workspace.clone();

        workspace.update(&mut cx, |workspace, cx| {
            let inventory = workspace.project().read(cx).runnable_inventory().clone();
            let fs = workspace.app_state().fs.clone();
            let panel = RunnablesPanel::new(inventory, workspace_view, fs, cx);
            if let Some(serialized_panel) = serialized_panel {
                panel.update(cx, |panel, cx| {
                    panel.width = serialized_panel.width;
                    cx.notify();
                });
            }
            panel
        })
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let width = self.width;
        self.pending_serialization = cx.background_executor().spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        RUNNABLES_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedRunnablesPanel { width })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }
}
actions!(runnables_panel, [ToggleFocus]);
impl FocusableView for RunnablesPanel {
    fn focus_handle(&self, _cx: &gpui::AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
impl EventEmitter<PanelEvent> for RunnablesPanel {}

impl Panel for RunnablesPanel {
    fn persistent_name() -> &'static str {
        "RunnablesPanel"
    }

    fn position(&self, cx: &WindowContext) -> DockPosition {
        match RunnablesSettings::get_global(cx).dock {
            RunnablesDockPosition::Left => DockPosition::Left,
            RunnablesDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<RunnablesSettings>(self.fs.clone(), cx, move |settings| {
            let dock = match position {
                DockPosition::Left | DockPosition::Bottom => RunnablesDockPosition::Left,
                DockPosition::Right => RunnablesDockPosition::Right,
            };
            settings.dock = Some(dock);
        });
    }

    fn size(&self, cx: &ui::prelude::WindowContext) -> Pixels {
        self.width
            .unwrap_or_else(|| px(RunnablesSettings::get_global(cx).default_width))
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ui::prelude::ViewContext<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, _cx: &ui::prelude::WindowContext) -> Option<ui::IconName> {
        Some(ui::IconName::Play)
    }

    fn icon_tooltip(&self, _cx: &ui::prelude::WindowContext) -> Option<&'static str> {
        Some("Runnables panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }
    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        if active {
            self.status_bar_tracker.take();
            cx.notify();
        } else {
            let path = PathBuf::new();
            let tasks: Vec<_> = self
                .inventory
                .read(cx)
                .list_runnables(&path, cx)
                .filter_map(|runnable| {
                    runnable
                        .handle(cx)
                        .filter(|handle| handle.result().is_none())
                })
                .collect();

            self.status_bar_tracker = Some(StatusIconTracker::new(tasks, cx));
            cx.notify();
        }
    }
    fn collapsed_icon_color(&self, cx: &WindowContext) -> Option<Color> {
        if let Some(tracker) = &self.status_bar_tracker.as_ref() {
            tracker.read(cx).color()
        } else {
            // We don't care about the color if we're active.
            None
        }
    }
}

impl Render for RunnablesPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let runnables: Vec<_> = self
            .inventory
            .read(cx)
            .list_runnables(&PathBuf::new(), cx)
            .collect();
        //let list = List::new().empty_message("There are no runnables");
        let state = ListState::new(runnables.len(), ListAlignment::Top, px(2.), {
            move |index, cx| {
                let runnable = runnables[index].clone();
                let result = runnable.result(cx);
                let cancelable = runnable.cancel_handle(cx).filter(|_| result.is_none());
                ListItem::new(("Runnables", runnable.id()))
                    .child(
                        Button::new(
                            ("Runnable trigger", runnable.id()),
                            SharedString::from(runnable.metadata().display_name().to_owned()),
                        )
                        .on_click({
                            let runnable = runnable.clone();
                            move |_, _| {
                                // TODO what do we do here?
                                dbg!("Runnable clicked");
                            }
                        }),
                    )
                    .when_some(result, |this, result| {
                        let succeeded = result.is_ok();
                        let icon = if succeeded {
                            IconName::Check
                        } else {
                            IconName::AtSign
                        };
                        this.start_slot(Icon::new(icon))
                    })
                    .when_some(cancelable, |this, cancel_token| {
                        this.end_slot(
                            IconButton::new(
                                ("Runnable cancel button", runnable.id()),
                                IconName::XCircle,
                            )
                            .on_click(move |_, _| {
                                cancel_token.abort();
                            }),
                        )
                    })
                    .into_any_element()
            }
        });
        v_flex()
            .track_focus(&self.focus_handle)
            .p_1()
            .size_full()
            //.child(list(self.list_state.clone()).full())
            .child(ListHeader::new("Active runnables"))
            .child(list(state).full())
            .child(
                v_flex()
                    .child(div().mx_2().border_primary(cx).border_t())
                    .child(
                        v_flex()
                            .p_2()
                            .child(self.render_filter_input(&self.filter_editor, cx)),
                    ),
            )
    }
}
