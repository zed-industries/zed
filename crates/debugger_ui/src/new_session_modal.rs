use collections::FxHashMap;
use std::{
    borrow::Cow,
    ops::Not,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
    usize,
};

use anyhow::Result;
use dap::{
    DapRegistry, DebugRequest,
    adapters::{DebugAdapterName, DebugTaskDefinition},
};
use editor::{Editor, EditorElement, EditorStyle};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    Animation, AnimationExt as _, App, AppContext, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, Render, Subscription, TextStyle, Transformation, WeakEntity, percentage,
};
use picker::{Picker, PickerDelegate, highlighted_match_with_paths::HighlightedMatch};
use project::{ProjectPath, TaskContexts, TaskSourceKind, task_store::TaskStore};
use settings::Settings;
use task::{DebugScenario, LaunchRequest};
use theme::ThemeSettings;
use ui::{
    ActiveTheme, Button, ButtonCommon, ButtonSize, CheckboxWithLabel, Clickable, Color, Context,
    ContextMenu, Disableable, DropdownMenu, FluentBuilder, Icon, IconButton, IconName, IconSize,
    InteractiveElement, IntoElement, Label, LabelCommon as _, ListItem, ListItemSpacing,
    ParentElement, RenderOnce, SharedString, Styled, StyledExt, ToggleButton, ToggleState,
    Toggleable, Window, div, h_flex, relative, rems, v_flex,
};
use util::ResultExt;
use workspace::{ModalView, Workspace, pane};

use crate::{attach_modal::AttachModal, debugger_panel::DebugPanel};

enum SaveScenarioState {
    Saving,
    Saved(ProjectPath),
    Failed(SharedString),
}

pub(super) struct NewSessionModal {
    workspace: WeakEntity<Workspace>,
    debug_panel: WeakEntity<DebugPanel>,
    mode: NewSessionMode,
    launch_picker: Entity<Picker<DebugScenarioDelegate>>,
    attach_mode: Entity<AttachMode>,
    custom_mode: Entity<CustomMode>,
    debugger: Option<DebugAdapterName>,
    task_contexts: Arc<TaskContexts>,
    save_scenario_state: Option<SaveScenarioState>,
    _subscriptions: [Subscription; 2],
}

fn suggested_label(request: &DebugRequest, debugger: &str) -> SharedString {
    match request {
        DebugRequest::Launch(config) => {
            let last_path_component = Path::new(&config.program)
                .file_name()
                .map(|name| name.to_string_lossy())
                .unwrap_or_else(|| Cow::Borrowed(&config.program));

            format!("{} ({debugger})", last_path_component).into()
        }
        DebugRequest::Attach(config) => format!(
            "pid: {} ({debugger})",
            config.process_id.unwrap_or(u32::MAX)
        )
        .into(),
    }
}

impl NewSessionModal {
    pub(super) fn show(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) else {
            return;
        };
        let task_store = workspace.project().read(cx).task_store().clone();

        cx.spawn_in(window, async move |workspace, cx| {
            let task_contexts = Arc::from(
                workspace
                    .update_in(cx, |workspace, window, cx| {
                        tasks_ui::task_contexts(workspace, window, cx)
                    })?
                    .await,
            );

            workspace.update_in(cx, |workspace, window, cx| {
                let workspace_handle = workspace.weak_handle();
                workspace.toggle_modal(window, cx, |window, cx| {
                    let attach_mode = AttachMode::new(None, workspace_handle.clone(), window, cx);

                    let launch_picker = cx.new(|cx| {
                        Picker::uniform_list(
                            DebugScenarioDelegate::new(
                                debug_panel.downgrade(),
                                workspace_handle.clone(),
                                task_store,
                                task_contexts.clone(),
                            ),
                            window,
                            cx,
                        )
                        .modal(false)
                    });

                    let _subscriptions = [
                        cx.subscribe(&launch_picker, |_, _, _, cx| {
                            cx.emit(DismissEvent);
                        }),
                        cx.subscribe(
                            &attach_mode.read(cx).attach_picker.clone(),
                            |_, _, _, cx| {
                                cx.emit(DismissEvent);
                            },
                        ),
                    ];

                    let custom_mode = CustomMode::new(None, window, cx);

                    Self {
                        launch_picker,
                        attach_mode,
                        custom_mode,
                        debugger: None,
                        mode: NewSessionMode::Launch,
                        debug_panel: debug_panel.downgrade(),
                        workspace: workspace_handle,
                        task_contexts,
                        save_scenario_state: None,
                        _subscriptions,
                    }
                });
            })?;

            anyhow::Ok(())
        })
        .detach();
    }

    fn render_mode(&self, window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let dap_menu = self.adapter_drop_down_menu(window, cx);
        match self.mode {
            NewSessionMode::Attach => self.attach_mode.update(cx, |this, cx| {
                this.clone().render(window, cx).into_any_element()
            }),
            NewSessionMode::Custom => self.custom_mode.update(cx, |this, cx| {
                this.clone().render(dap_menu, window, cx).into_any_element()
            }),
            NewSessionMode::Launch => v_flex()
                .w(rems(34.))
                .child(self.launch_picker.clone())
                .into_any_element(),
        }
    }

    fn mode_focus_handle(&self, cx: &App) -> FocusHandle {
        match self.mode {
            NewSessionMode::Attach => self.attach_mode.read(cx).attach_picker.focus_handle(cx),
            NewSessionMode::Custom => self.custom_mode.read(cx).program.focus_handle(cx),
            NewSessionMode::Launch => self.launch_picker.focus_handle(cx),
        }
    }

    fn debug_scenario(&self, debugger: &str, cx: &App) -> Option<DebugScenario> {
        let request = match self.mode {
            NewSessionMode::Custom => Some(DebugRequest::Launch(
                self.custom_mode.read(cx).debug_request(cx),
            )),
            NewSessionMode::Attach => Some(DebugRequest::Attach(
                self.attach_mode.read(cx).debug_request(),
            )),
            _ => None,
        }?;
        let label = suggested_label(&request, debugger);

        let stop_on_entry = if let NewSessionMode::Custom = &self.mode {
            Some(self.custom_mode.read(cx).stop_on_entry.selected())
        } else {
            None
        };

        Some(DebugScenario {
            adapter: debugger.to_owned().into(),
            label,
            request: Some(request),
            initialize_args: None,
            tcp_connection: None,
            stop_on_entry,
            build: None,
        })
    }

    fn start_new_session(&self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(debugger) = self.debugger.as_ref() else {
            // todo(debugger): show in UI.
            log::error!("No debugger selected");
            return;
        };

        if let NewSessionMode::Launch = &self.mode {
            self.launch_picker.update(cx, |picker, cx| {
                picker.delegate.confirm(false, window, cx);
            });
            return;
        }

        let Some(config) = self.debug_scenario(debugger, cx) else {
            log::error!("debug config not found in mode: {}", self.mode);
            return;
        };

        let debug_panel = self.debug_panel.clone();
        let task_contexts = self.task_contexts.clone();
        cx.spawn_in(window, async move |this, cx| {
            let task_context = task_contexts.active_context().cloned().unwrap_or_default();
            let worktree_id = task_contexts.worktree();
            debug_panel.update_in(cx, |debug_panel, window, cx| {
                debug_panel.start_session(config, task_context, None, worktree_id, window, cx)
            })?;
            this.update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
            Result::<_, anyhow::Error>::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn update_attach_picker(
        attach: &Entity<AttachMode>,
        adapter: &DebugAdapterName,
        window: &mut Window,
        cx: &mut App,
    ) {
        attach.update(cx, |this, cx| {
            if adapter != &this.definition.adapter {
                this.definition.adapter = adapter.clone();

                this.attach_picker.update(cx, |this, cx| {
                    this.picker.update(cx, |this, cx| {
                        this.delegate.definition.adapter = adapter.clone();
                        this.focus(window, cx);
                    })
                });
            }

            cx.notify();
        })
    }
    fn adapter_drop_down_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ui::DropdownMenu {
        let workspace = self.workspace.clone();
        let weak = cx.weak_entity();
        let label = self
            .debugger
            .as_ref()
            .map(|d| d.0.clone())
            .unwrap_or_else(|| SELECT_DEBUGGER_LABEL.clone());
        let active_buffer_language = self
            .task_contexts
            .active_item_context
            .as_ref()
            .and_then(|item| {
                item.1
                    .as_ref()
                    .and_then(|location| location.buffer.read(cx).language())
            })
            .cloned();

        DropdownMenu::new(
            "dap-adapter-picker",
            label,
            ContextMenu::build(window, cx, move |mut menu, _, cx| {
                let setter_for_name = |name: DebugAdapterName| {
                    let weak = weak.clone();
                    move |window: &mut Window, cx: &mut App| {
                        weak.update(cx, |this, cx| {
                            this.debugger = Some(name.clone());
                            cx.notify();
                            if let NewSessionMode::Attach = &this.mode {
                                Self::update_attach_picker(&this.attach_mode, &name, window, cx);
                            }
                        })
                        .ok();
                    }
                };

                let mut available_adapters = workspace
                    .update(cx, |_, cx| DapRegistry::global(cx).enumerate_adapters())
                    .unwrap_or_default();
                if let Some(language) = active_buffer_language {
                    available_adapters.sort_by_key(|adapter| {
                        language
                            .config()
                            .debuggers
                            .get_index_of(adapter.0.as_ref())
                            .unwrap_or(usize::MAX)
                    });
                }

                for adapter in available_adapters.into_iter() {
                    menu = menu.entry(adapter.0.clone(), None, setter_for_name(adapter.clone()));
                }
                menu
            }),
        )
    }
}

static SELECT_DEBUGGER_LABEL: SharedString = SharedString::new_static("Select Debugger");

#[derive(Clone)]
enum NewSessionMode {
    Custom,
    Attach,
    Launch,
}

impl std::fmt::Display for NewSessionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self {
            NewSessionMode::Launch => "Launch".to_owned(),
            NewSessionMode::Attach => "Attach".to_owned(),
            NewSessionMode::Custom => "Custom".to_owned(),
        };

        write!(f, "{}", mode)
    }
}

impl Focusable for NewSessionMode {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        cx.focus_handle()
    }
}

fn render_editor(editor: &Entity<Editor>, window: &mut Window, cx: &App) -> impl IntoElement {
    let settings = ThemeSettings::get_global(cx);
    let theme = cx.theme();

    let text_style = TextStyle {
        color: cx.theme().colors().text,
        font_family: settings.buffer_font.family.clone(),
        font_features: settings.buffer_font.features.clone(),
        font_size: settings.buffer_font_size(cx).into(),
        font_weight: settings.buffer_font.weight,
        line_height: relative(settings.buffer_line_height.value()),
        background_color: Some(theme.colors().editor_background),
        ..Default::default()
    };

    let element = EditorElement::new(
        editor,
        EditorStyle {
            background: theme.colors().editor_background,
            local_player: theme.players().local(),
            text: text_style,
            ..Default::default()
        },
    );

    div()
        .rounded_md()
        .p_1()
        .border_1()
        .border_color(theme.colors().border_variant)
        .when(
            editor.focus_handle(cx).contains_focused(window, cx),
            |this| this.border_color(theme.colors().border_focused),
        )
        .child(element)
        .bg(theme.colors().editor_background)
}

impl Render for NewSessionModal {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        let this = cx.weak_entity().clone();

        v_flex()
            .size_full()
            .w(rems(34.))
            .key_context("Pane")
            .elevation_3(cx)
            .bg(cx.theme().colors().elevated_surface_background)
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| {
                cx.emit(DismissEvent);
            }))
            .on_action(
                cx.listener(|this, _: &pane::ActivatePreviousItem, window, cx| {
                    this.mode = match this.mode {
                        NewSessionMode::Attach => NewSessionMode::Launch,
                        NewSessionMode::Launch => NewSessionMode::Attach,
                        _ => {
                            return;
                        }
                    };

                    this.mode_focus_handle(cx).focus(window);
                }),
            )
            .on_action(cx.listener(|this, _: &pane::ActivateNextItem, window, cx| {
                this.mode = match this.mode {
                    NewSessionMode::Attach => NewSessionMode::Launch,
                    NewSessionMode::Launch => NewSessionMode::Attach,
                    _ => {
                        return;
                    }
                };

                this.mode_focus_handle(cx).focus(window);
            }))
            .child(
                h_flex()
                    .w_full()
                    .justify_around()
                    .p_2()
                    .child(
                        h_flex()
                            .justify_start()
                            .w_full()
                            .child(
                                ToggleButton::new("debugger-session-ui-picker-button", "Launch")
                                    .size(ButtonSize::Default)
                                    .style(ui::ButtonStyle::Subtle)
                                    .toggle_state(matches!(self.mode, NewSessionMode::Launch))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.mode = NewSessionMode::Launch;
                                        this.mode_focus_handle(cx).focus(window);
                                        cx.notify();
                                    }))
                                    .first(),
                            )
                            .child(
                                ToggleButton::new("debugger-session-ui-attach-button", "Attach")
                                    .size(ButtonSize::Default)
                                    .toggle_state(matches!(self.mode, NewSessionMode::Attach))
                                    .style(ui::ButtonStyle::Subtle)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.mode = NewSessionMode::Attach;

                                        if let Some(debugger) = this.debugger.as_ref() {
                                            Self::update_attach_picker(
                                                &this.attach_mode,
                                                &debugger,
                                                window,
                                                cx,
                                            );
                                        }
                                        this.mode_focus_handle(cx).focus(window);
                                        cx.notify();
                                    }))
                                    .last(),
                            ),
                    )
                    .justify_between()
                    .border_color(cx.theme().colors().border_variant)
                    .border_b_1(),
            )
            .child(v_flex().child(self.render_mode(window, cx)))
            .child(
                h_flex()
                    .justify_between()
                    .gap_2()
                    .p_2()
                    .border_color(cx.theme().colors().border_variant)
                    .border_t_1()
                    .w_full()
                    .child(match self.mode {
                        NewSessionMode::Attach => {
                            div().child(self.adapter_drop_down_menu(window, cx))
                        }
                        NewSessionMode::Launch => div().child(
                            Button::new("new-session-modal-custom", "Custom").on_click({
                                let this = cx.weak_entity();
                                move |_, window, cx| {
                                    this.update(cx, |this, cx| {
                                        this.mode = NewSessionMode::Custom;
                                        this.mode_focus_handle(cx).focus(window);
                                    })
                                    .ok();
                                }
                            }),
                        ),
                        NewSessionMode::Custom => h_flex()
                            .child(
                                Button::new("new-session-modal-back", "Save to .zed/debug.json...")
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        let Some(save_scenario) = this
                                            .debugger
                                            .as_ref()
                                            .and_then(|debugger| this.debug_scenario(&debugger, cx))
                                            .zip(this.task_contexts.worktree())
                                            .and_then(|(scenario, worktree_id)| {
                                                this.debug_panel
                                                    .update(cx, |panel, cx| {
                                                        panel.save_scenario(
                                                            &scenario,
                                                            worktree_id,
                                                            window,
                                                            cx,
                                                        )
                                                    })
                                                    .ok()
                                            })
                                        else {
                                            return;
                                        };

                                        this.save_scenario_state = Some(SaveScenarioState::Saving);

                                        cx.spawn(async move |this, cx| {
                                            let res = save_scenario.await;

                                            this.update(cx, |this, _| match res {
                                                Ok(saved_file) => {
                                                    this.save_scenario_state =
                                                        Some(SaveScenarioState::Saved(saved_file))
                                                }
                                                Err(error) => {
                                                    this.save_scenario_state =
                                                        Some(SaveScenarioState::Failed(
                                                            error.to_string().into(),
                                                        ))
                                                }
                                            })
                                            .ok();

                                            cx.background_executor()
                                                .timer(Duration::from_secs(2))
                                                .await;
                                            this.update(cx, |this, _| {
                                                this.save_scenario_state.take()
                                            })
                                            .ok();
                                        })
                                        .detach();
                                    }))
                                    .disabled(
                                        self.debugger.is_none()
                                            || self
                                                .custom_mode
                                                .read(cx)
                                                .program
                                                .read(cx)
                                                .is_empty(cx)
                                            || self.save_scenario_state.is_some(),
                                    ),
                            )
                            .when_some(self.save_scenario_state.as_ref(), {
                                let this_entity = this.clone();

                                move |this, save_state| match save_state {
                                    SaveScenarioState::Saved(saved_path) => this.child(
                                        IconButton::new(
                                            "new-session-modal-go-to-file",
                                            IconName::ArrowUpRight,
                                        )
                                        .icon_size(IconSize::Small)
                                        .icon_color(Color::Muted)
                                        .on_click({
                                            let this_entity = this_entity.clone();
                                            let saved_path = saved_path.clone();
                                            move |_, window, cx| {
                                                window
                                                    .spawn(cx, {
                                                        let this_entity = this_entity.clone();
                                                        let saved_path = saved_path.clone();

                                                        async move |cx| {
                                                            this_entity
                                                                .update_in(
                                                                    cx,
                                                                    |this, window, cx| {
                                                                        this.workspace.update(
                                                                            cx,
                                                                            |workspace, cx| {
                                                                                workspace.open_path(
                                                                                    saved_path
                                                                                        .clone(),
                                                                                    None,
                                                                                    true,
                                                                                    window,
                                                                                    cx,
                                                                                )
                                                                            },
                                                                        )
                                                                    },
                                                                )??
                                                                .await?;

                                                            this_entity
                                                                .update(cx, |_, cx| {
                                                                    cx.emit(DismissEvent)
                                                                })
                                                                .ok();

                                                            anyhow::Ok(())
                                                        }
                                                    })
                                                    .detach();
                                            }
                                        }),
                                    ),
                                    SaveScenarioState::Saving => this.child(
                                        Icon::new(IconName::Spinner)
                                            .size(IconSize::Small)
                                            .color(Color::Muted)
                                            .with_animation(
                                                "Spinner",
                                                Animation::new(Duration::from_secs(3)).repeat(),
                                                |icon, delta| {
                                                    icon.transform(Transformation::rotate(
                                                        percentage(delta),
                                                    ))
                                                },
                                            ),
                                    ),
                                    SaveScenarioState::Failed(error_msg) => this.child(
                                        IconButton::new("Failed Scenario Saved", IconName::X)
                                            .icon_size(IconSize::Small)
                                            .icon_color(Color::Error)
                                            .tooltip(ui::Tooltip::text(error_msg.clone())),
                                    ),
                                }
                            }),
                    })
                    .child(
                        Button::new("debugger-spawn", "Start")
                            .on_click(cx.listener(|this, _, window, cx| match &this.mode {
                                NewSessionMode::Launch => {
                                    this.launch_picker.update(cx, |picker, cx| {
                                        picker.delegate.confirm(true, window, cx)
                                    })
                                }
                                _ => this.start_new_session(window, cx),
                            }))
                            .disabled(match self.mode {
                                NewSessionMode::Launch => {
                                    !self.launch_picker.read(cx).delegate.matches.is_empty()
                                }
                                NewSessionMode::Attach => {
                                    self.debugger.is_none()
                                        || self
                                            .attach_mode
                                            .read(cx)
                                            .attach_picker
                                            .read(cx)
                                            .picker
                                            .read(cx)
                                            .delegate
                                            .match_count()
                                            == 0
                                }
                                NewSessionMode::Custom => {
                                    self.debugger.is_none()
                                        || self.custom_mode.read(cx).program.read(cx).is_empty(cx)
                                }
                            }),
                    ),
            )
    }
}

impl EventEmitter<DismissEvent> for NewSessionModal {}
impl Focusable for NewSessionModal {
    fn focus_handle(&self, cx: &ui::App) -> gpui::FocusHandle {
        self.mode_focus_handle(cx)
    }
}

impl ModalView for NewSessionModal {}

impl RenderOnce for AttachMode {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .w_full()
            .track_focus(&self.attach_picker.focus_handle(cx))
            .child(self.attach_picker.clone())
    }
}

#[derive(Clone)]
pub(super) struct CustomMode {
    program: Entity<Editor>,
    cwd: Entity<Editor>,
    stop_on_entry: ToggleState,
}

impl CustomMode {
    pub(super) fn new(
        past_launch_config: Option<LaunchRequest>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let (past_program, past_cwd) = past_launch_config
            .map(|config| (Some(config.program), config.cwd))
            .unwrap_or_else(|| (None, None));

        let program = cx.new(|cx| Editor::single_line(window, cx));
        program.update(cx, |this, cx| {
            this.set_placeholder_text("Run", cx);

            if let Some(past_program) = past_program {
                this.set_text(past_program, window, cx);
            };
        });
        let cwd = cx.new(|cx| Editor::single_line(window, cx));
        cwd.update(cx, |this, cx| {
            this.set_placeholder_text("Working Directory", cx);
            if let Some(past_cwd) = past_cwd {
                this.set_text(past_cwd.to_string_lossy(), window, cx);
            };
        });
        cx.new(|_| Self {
            program,
            cwd,
            stop_on_entry: ToggleState::Unselected,
        })
    }

    pub(super) fn debug_request(&self, cx: &App) -> task::LaunchRequest {
        let path = self.cwd.read(cx).text(cx);
        let command = self.program.read(cx).text(cx);
        let mut args = shlex::split(&command).into_iter().flatten().peekable();
        let mut env = FxHashMap::default();
        while args.peek().is_some_and(|arg| arg.contains('=')) {
            let arg = args.next().unwrap();
            let (lhs, rhs) = arg.split_once('=').unwrap();
            env.insert(lhs.to_string(), rhs.to_string());
        }

        let program = if let Some(program) = args.next() {
            program
        } else {
            env = FxHashMap::default();
            command
        };

        let args = args.collect::<Vec<_>>();

        task::LaunchRequest {
            program,
            cwd: path.is_empty().not().then(|| PathBuf::from(path)),
            args,
            env,
        }
    }

    fn render(
        &mut self,
        adapter_menu: DropdownMenu,
        window: &mut Window,
        cx: &mut ui::Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .p_2()
            .w_full()
            .gap_3()
            .track_focus(&self.program.focus_handle(cx))
            .child(
                div().child(
                    Label::new("Program")
                        .size(ui::LabelSize::Small)
                        .color(Color::Muted),
                ),
            )
            .child(render_editor(&self.program, window, cx))
            .child(
                h_flex()
                    .child(
                        Label::new("Debugger")
                            .size(ui::LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .gap(ui::DynamicSpacing::Base08.rems(cx))
                    .child(adapter_menu),
            )
            .child(
                CheckboxWithLabel::new(
                    "debugger-stop-on-entry",
                    Label::new("Stop on Entry").size(ui::LabelSize::Small),
                    self.stop_on_entry,
                    {
                        let this = cx.weak_entity();
                        move |state, _, cx| {
                            this.update(cx, |this, _| {
                                this.stop_on_entry = *state;
                            })
                            .ok();
                        }
                    },
                )
                .checkbox_position(ui::IconPosition::End),
            )
    }
}

#[derive(Clone)]
pub(super) struct AttachMode {
    pub(super) definition: DebugTaskDefinition,
    pub(super) attach_picker: Entity<AttachModal>,
}

impl AttachMode {
    pub(super) fn new(
        debugger: Option<DebugAdapterName>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<NewSessionModal>,
    ) -> Entity<Self> {
        let definition = DebugTaskDefinition {
            adapter: debugger.unwrap_or(DebugAdapterName("".into())),
            label: "Attach New Session Setup".into(),
            request: dap::DebugRequest::Attach(task::AttachRequest { process_id: None }),
            initialize_args: None,
            tcp_connection: None,
            stop_on_entry: Some(false),
        };
        let attach_picker = cx.new(|cx| {
            let modal = AttachModal::new(definition.clone(), workspace, false, window, cx);
            window.focus(&modal.focus_handle(cx));

            modal
        });

        cx.new(|_| Self {
            definition,
            attach_picker,
        })
    }
    pub(super) fn debug_request(&self) -> task::AttachRequest {
        task::AttachRequest { process_id: None }
    }
}

pub(super) struct DebugScenarioDelegate {
    task_store: Entity<TaskStore>,
    candidates: Option<Vec<(TaskSourceKind, DebugScenario)>>,
    selected_index: usize,
    matches: Vec<StringMatch>,
    prompt: String,
    debug_panel: WeakEntity<DebugPanel>,
    workspace: WeakEntity<Workspace>,
    task_contexts: Arc<TaskContexts>,
}

impl DebugScenarioDelegate {
    pub(super) fn new(
        debug_panel: WeakEntity<DebugPanel>,
        workspace: WeakEntity<Workspace>,
        task_store: Entity<TaskStore>,
        task_contexts: Arc<TaskContexts>,
    ) -> Self {
        Self {
            task_store,
            candidates: None,
            selected_index: 0,
            matches: Vec::new(),
            prompt: String::new(),
            debug_panel,
            workspace,
            task_contexts,
        }
    }
}

impl PickerDelegate for DebugScenarioDelegate {
    type ListItem = ui::ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<picker::Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> std::sync::Arc<str> {
        "".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> gpui::Task<()> {
        let candidates = self.candidates.clone();
        let workspace = self.workspace.clone();
        let task_store = self.task_store.clone();

        cx.spawn_in(window, async move |picker, cx| {
            let candidates: Vec<_> = match &candidates {
                Some(candidates) => candidates
                    .into_iter()
                    .enumerate()
                    .map(|(index, (_, candidate))| {
                        StringMatchCandidate::new(index, candidate.label.as_ref())
                    })
                    .collect(),
                None => {
                    let worktree_ids: Vec<_> = workspace
                        .update(cx, |this, cx| {
                            this.visible_worktrees(cx)
                                .map(|tree| tree.read(cx).id())
                                .collect()
                        })
                        .ok()
                        .unwrap_or_default();

                    let scenarios: Vec<_> = task_store
                        .update(cx, |task_store, cx| {
                            task_store.task_inventory().map(|item| {
                                item.read(cx).list_debug_scenarios(worktree_ids.into_iter())
                            })
                        })
                        .ok()
                        .flatten()
                        .unwrap_or_default();

                    picker
                        .update(cx, |picker, _| {
                            picker.delegate.candidates = Some(scenarios.clone());
                        })
                        .ok();

                    scenarios
                        .into_iter()
                        .enumerate()
                        .map(|(index, (_, candidate))| {
                            StringMatchCandidate::new(index, candidate.label.as_ref())
                        })
                        .collect()
                }
            };

            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                true,
                1000,
                &Default::default(),
                cx.background_executor().clone(),
            )
            .await;

            picker
                .update(cx, |picker, _| {
                    let delegate = &mut picker.delegate;

                    delegate.matches = matches;
                    delegate.prompt = query;

                    if delegate.matches.is_empty() {
                        delegate.selected_index = 0;
                    } else {
                        delegate.selected_index =
                            delegate.selected_index.min(delegate.matches.len() - 1);
                    }
                })
                .log_err();
        })
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<picker::Picker<Self>>) {
        let debug_scenario = self
            .matches
            .get(self.selected_index())
            .and_then(|match_candidate| {
                self.candidates
                    .as_ref()
                    .map(|candidates| candidates[match_candidate.candidate_id].clone())
            });

        let Some((task_source_kind, debug_scenario)) = debug_scenario else {
            return;
        };

        let (task_context, worktree_id) = if let TaskSourceKind::Worktree {
            id: worktree_id,
            directory_in_worktree: _,
            id_base: _,
        } = task_source_kind
        {
            self.task_contexts
                .task_context_for_worktree_id(worktree_id)
                .cloned()
                .map(|context| (context, Some(worktree_id)))
        } else {
            None
        }
        .unwrap_or_default();

        self.debug_panel
            .update(cx, |panel, cx| {
                panel.start_session(debug_scenario, task_context, None, worktree_id, window, cx);
            })
            .ok();

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<picker::Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let hit = &self.matches[ix];

        let highlighted_location = HighlightedMatch {
            text: hit.string.clone(),
            highlight_positions: hit.positions.clone(),
            char_count: hit.string.chars().count(),
            color: Color::Default,
        };

        let icon = Icon::new(IconName::FileTree)
            .color(Color::Muted)
            .size(ui::IconSize::Small);

        Some(
            ListItem::new(SharedString::from(format!("debug-scenario-selection-{ix}")))
                .inset(true)
                .start_slot::<Icon>(icon)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(highlighted_location.render(window, cx)),
        )
    }
}
