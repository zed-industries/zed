use client::{TelemetrySettings, telemetry::Telemetry};

use gpui::{
    Action, App, Context, Entity, EventEmitter, FocusHandle, Focusable, InteractiveElement,
    ParentElement, Render, Styled, Subscription, WeakEntity, Window, svg,
};
use language::language_settings::{EditPredictionProvider, all_language_settings};
use persistence::WALKTHROUGH_DB;
use settings::{Settings, SettingsStore};
use std::sync::Arc;
use ui::{CheckboxWithLabel, ElevationIndex, Tooltip, prelude::*};
use util::ResultExt;
use vim_mode_setting::VimModeSetting;
use workspace::{
    SerializableItem, Workspace, WorkspaceId, delete_unloaded_items,
    item::{Item, ItemEvent},
    register_serializable_item,
};

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _cx| {
        workspace.register_action(|workspace, _: &workspace::Walkthrough, window, cx| {
            let welcome_page = Walkthrough::new(workspace, cx);
            workspace.add_item_to_active_pane(Box::new(welcome_page), None, true, window, cx)
        });
    })
    .detach();

    register_serializable_item::<Walkthrough>(cx);
}

pub struct Walkthrough {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    telemetry: Arc<Telemetry>,
    // steps: ListState,
    _settings_subscription: Subscription,
}

impl Render for Walkthrough {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let edit_prediction_provider_is_zed =
            all_language_settings(None, cx).edit_predictions.provider
                == EditPredictionProvider::Zed;

        let edit_prediction_label = if edit_prediction_provider_is_zed {
            "Edit Prediction Enabled"
        } else {
            "Try Edit Prediction"
        };

        h_flex()
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .key_context("Welcome")
            .track_focus(&self.focus_handle(cx))
            .child(
                v_flex()
                    .gap_8()
                    .mx_auto()
                    .child(
                        v_flex()
                            .w_full()
                            .child(
                                svg()
                                    .path("icons/logo_96.svg")
                                    .text_color(cx.theme().colors().icon_disabled)
                                    .w(px(40.))
                                    .h(px(40.))
                                    .mx_auto()
                                    .mb_4(),
                            )
                            .child(
                                h_flex()
                                    .w_full()
                                    .justify_center()
                                    .child(Headline::new("Welcome to Zed")),
                            )
                            .child(
                                h_flex().w_full().justify_center().child(
                                    Label::new("The editor for what's next")
                                        .color(Color::Muted)
                                        .italic(),
                                ),
                            ),
                    )
                    .child(
                        h_flex()
                            .items_start()
                            .gap_8()
                            .child(
                                v_flex()
                                    .gap_2()
                                    .pr_8()
                                    .border_r_1()
                                    .border_color(cx.theme().colors().border_variant)
                                    .child(
                                        self.section_label( cx).child(
                                            Label::new("Get Started")
                                                .size(LabelSize::XSmall)
                                                .color(Color::Muted),
                                        ),
                                    )
                                    .child(
                                        Button::new("choose-theme", "Choose a Theme")
                                            .icon(IconName::SwatchBook)
                                            .icon_size(IconSize::XSmall)
                                            .icon_color(Color::Muted)
                                            .icon_position(IconPosition::Start)
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                telemetry::event!("Welcome Theme Changed");
                                                this.workspace
                                                    .update(cx, |_workspace, cx| {
                                                        window.dispatch_action(zed_actions::theme_selector::Toggle::default().boxed_clone(), cx);
                                                    })
                                                    .ok();
                                            })),
                                    )

                                    .child(
                                        Button::new(
                                            "try-zed-edit-prediction",
                                            edit_prediction_label,
                                        )
                                        .disabled(edit_prediction_provider_is_zed)
                                        .icon(IconName::ZedPredict)
                                        .icon_size(IconSize::XSmall)
                                        .icon_color(Color::Muted)
                                        .icon_position(IconPosition::Start)
                                        .on_click(
                                            cx.listener(|_, _, window, cx| {
                                                telemetry::event!("Welcome Screen Try Edit Prediction clicked");
                                                window.dispatch_action(zed_actions::OpenZedPredictOnboarding.boxed_clone(), cx);
                                            }),
                                        ),
                                    )
                                    .child(
                                        Button::new("edit settings", "Edit Settings")
                                            .icon(IconName::Settings)
                                            .icon_size(IconSize::XSmall)
                                            .icon_color(Color::Muted)
                                            .icon_position(IconPosition::Start)
                                            .on_click(cx.listener(|_, _, window, cx| {
                                                telemetry::event!("Welcome Settings Edited");
                                                window.dispatch_action(Box::new(
                                                    zed_actions::OpenSettings,
                                                ), cx);
                                            })),
                                    )

                            )
                            .child(
                                v_flex()
                                    .gap_2()
                                    .child(
                                        self.section_label(cx).child(
                                            Label::new("Resources")
                                                .size(LabelSize::XSmall)
                                                .color(Color::Muted),
                                        ),
                                    )
                                    .when(cfg!(target_os = "macos"), |el| {
                                        el.child(
                                            Button::new("install-cli", "Install the CLI")
                                                .icon(IconName::Terminal)
                                                .icon_size(IconSize::XSmall)
                                                .icon_color(Color::Muted)
                                                .icon_position(IconPosition::Start)
                                                .on_click(cx.listener(|this, _, window, cx| {
                                                    telemetry::event!("Welcome CLI Installed");
                                                    this.workspace.update(cx, |_, cx|{
                                                        install_cli::install_cli(window, cx);
                                                    }).log_err();
                                                })),
                                        )
                                    })

                                    .child(
                                        Button::new("explore-extensions", "Explore Extensions")
                                            .icon(IconName::Blocks)
                                            .icon_size(IconSize::XSmall)
                                            .icon_color(Color::Muted)
                                            .icon_position(IconPosition::Start)
                                            .on_click(cx.listener(|_, _, window, cx| {
                                                telemetry::event!("Welcome Extensions Page Opened");
                                                window.dispatch_action(Box::new(
                                                    zed_actions::Extensions::default(),
                                                ), cx);
                                            })),
                                    )

                            ),
                    )
                    .child(
                        v_container()
                            .px_2()
                            .gap_2()
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(
                                        CheckboxWithLabel::new(
                                            "enable-vim",
                                            Label::new("Enable Vim Mode"),
                                            if VimModeSetting::get_global(cx).0 {
                                                ui::ToggleState::Selected
                                            } else {
                                                ui::ToggleState::Unselected
                                            },
                                            cx.listener(move |this, selection, _window, cx| {
                                                telemetry::event!("Welcome Vim Mode Toggled");
                                                this.update_settings::<VimModeSetting>(
                                                    selection,
                                                    cx,
                                                    |setting, value| *setting = Some(value),
                                                );
                                            }),
                                        )
                                        .fill()
                                        .elevation(ElevationIndex::ElevatedSurface),
                                    )
                                    .child(
                                        IconButton::new("vim-mode", IconName::Info)
                                            .icon_size(IconSize::XSmall)
                                            .icon_color(Color::Muted)
                                            .tooltip(
                                                Tooltip::text(
                                                    "You can also toggle Vim Mode via the command palette or Editor Controls menu.")
                                            ),
                                    ),
                            )
                            .child(
                                CheckboxWithLabel::new(
                                    "enable-crash",
                                    Label::new("Send Crash Reports"),
                                    if TelemetrySettings::get_global(cx).diagnostics {
                                        ui::ToggleState::Selected
                                    } else {
                                        ui::ToggleState::Unselected
                                    },
                                    cx.listener(move |this, selection, _window, cx| {
                                        telemetry::event!("Welcome Diagnostic Telemetry Toggled");
                                        this.update_settings::<TelemetrySettings>(selection, cx, {
                                            move |settings, value| {
                                                settings.diagnostics = Some(value);
                                                telemetry::event!(
                                                    "Settings Changed",
                                                    setting = "diagnostic telemetry",
                                                    value
                                                );
                                            }
                                        });
                                    }),
                                )
                                .fill()
                                .elevation(ElevationIndex::ElevatedSurface),
                            )
                            .child(
                                CheckboxWithLabel::new(
                                    "enable-telemetry",
                                    Label::new("Send Telemetry"),
                                    if TelemetrySettings::get_global(cx).metrics {
                                        ui::ToggleState::Selected
                                    } else {
                                        ui::ToggleState::Unselected
                                    },
                                    cx.listener(move |this, selection, _window, cx| {
                                        telemetry::event!("Welcome Metric Telemetry Toggled");
                                        this.update_settings::<TelemetrySettings>(selection, cx, {
                                            move |settings, value| {
                                                settings.metrics = Some(value);
                                                telemetry::event!(
                                                    "Settings Changed",
                                                    setting = "metric telemetry",
                                                    value
                                                );
                                            }
                                        });
                                    }),
                                )
                                .fill()
                                .elevation(ElevationIndex::ElevatedSurface),
                            ),
                    ),
            )
    }
}

impl Walkthrough {
    pub fn new(workspace: &Workspace, cx: &mut Context<Workspace>) -> Entity<Self> {
        let this = cx.new(|cx| {
            // cx.on_release(|_: &mut Self, _| {
            //     telemetry::event!("Welcome Page Closed");
            // }
            // .detach();

            Walkthrough {
                focus_handle: cx.focus_handle(),
                workspace: workspace.weak_handle(),
                telemetry: workspace.client().telemetry().clone(),
                _settings_subscription: cx
                    .observe_global::<SettingsStore>(move |_, cx| cx.notify()),
                // steps
                // steps_list: ListState::new(steps.len(), ListAlignment::Top, px(todo!()), || todo!()),
            }
        });

        this
    }

    fn section_label(&self, cx: &mut App) -> Div {
        div()
            .pl_1()
            .font_buffer(cx)
            .text_color(Color::Muted.color(cx))
    }

    fn update_settings<T: Settings>(
        &mut self,
        selection: &ToggleState,
        cx: &mut Context<Self>,
        callback: impl 'static + Send + Fn(&mut T::FileContent, bool),
    ) {
        if let Some(workspace) = self.workspace.upgrade() {
            let fs = workspace.read(cx).app_state().fs.clone();
            let selection = *selection;
            settings::update_settings_file::<T>(fs, cx, move |settings, _| {
                let value = match selection {
                    ToggleState::Unselected => false,
                    ToggleState::Selected => true,
                    _ => return,
                };

                callback(settings, value)
            });
        }
    }
}

impl EventEmitter<ItemEvent> for Walkthrough {}

impl Focusable for Walkthrough {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for Walkthrough {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Walkthrough".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Walkthrough Page Opened")
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>> {
        Some(cx.new(|cx| Walkthrough {
            focus_handle: cx.focus_handle(),
            workspace: self.workspace.clone(),
            telemetry: self.telemetry.clone(),
            _settings_subscription: cx.observe_global::<SettingsStore>(move |_, cx| cx.notify()),
        }))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}

impl SerializableItem for Walkthrough {
    fn serialized_item_kind() -> &'static str {
        "Walkthrough"
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<workspace::ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> gpui::Task<gpui::Result<()>> {
        delete_unloaded_items(
            alive_items,
            workspace_id,
            "walkthroughs",
            &*WALKTHROUGH_DB,
            cx,
        )
    }

    fn deserialize(
        _project: Entity<project::Project>,
        workspace: WeakEntity<Workspace>,
        workspace_id: WorkspaceId,
        item_id: workspace::ItemId,
        _window: &mut Window,
        cx: &mut App,
    ) -> gpui::Task<gpui::Result<Entity<Self>>> {
        let has_walkthrough = WALKTHROUGH_DB.get_walkthrough(item_id, workspace_id);
        cx.spawn(async move |cx| {
            has_walkthrough?;
            workspace.update(cx, |workspace, cx| Walkthrough::new(workspace, cx))
        })
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: workspace::ItemId,
        _closing: bool,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<gpui::Task<gpui::Result<()>>> {
        let workspace_id = workspace.database_id()?;
        Some(cx.background_spawn(async move {
            WALKTHROUGH_DB.save_walkthrough(item_id, workspace_id).await
        }))
    }

    fn should_serialize(&self, _event: &Self::Event) -> bool {
        false
    }
}

mod persistence {
    use db::{define_connection, query, sqlez_macros::sql};
    use workspace::{ItemId, WorkspaceDb};

    define_connection! {
        pub static ref WALKTHROUGH_DB: WalkthroughDb<WorkspaceDb> =
            &[sql!(
                CREATE TABLE walkthroughs (
                    workspace_id INTEGER,
                    item_id INTEGER UNIQUE,
                    PRIMARY KEY(workspace_id, item_id),
                    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
                ) STRICT;
            )];
    }

    impl WalkthroughDb {
        query! {
            pub async fn save_walkthrough(item_id: ItemId, workspace_id: workspace::WorkspaceId) -> Result<()> {
                INSERT INTO walkthroughs(item_id, workspace_id)
                VALUES (?1, ?2)
                ON CONFLICT DO UPDATE SET
                  item_id = ?1,
                  workspace_id = ?2
            }
        }

        query! {
            pub fn get_walkthrough(item_id: ItemId, workspace_id: workspace::WorkspaceId) -> Result<ItemId> {
                SELECT item_id
                FROM walkthroughs
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}
