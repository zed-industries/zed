use std::{
    borrow::Cow,
    ops::Not,
    path::{Path, PathBuf},
};

use dap::{DapRegistry, DebugRequest, adapters::DebugTaskDefinition};
use editor::{Editor, EditorElement, EditorStyle};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    App, AppContext, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render,
    Subscription, TextStyle, WeakEntity,
};
use picker::{Picker, PickerDelegate, highlighted_match_with_paths::HighlightedMatch};
use project::{TaskSourceKind, task_store::TaskStore};
use session_modes::{AttachMode, DebugScenarioDelegate, LaunchMode};
use settings::Settings;
use task::{DebugScenario, LaunchRequest};
use theme::ThemeSettings;
use ui::{
    ActiveTheme, Button, ButtonCommon, ButtonSize, CheckboxWithLabel, Clickable, Color, Context,
    ContextMenu, Disableable, DropdownMenu, FluentBuilder, Icon, IconName, InteractiveElement,
    IntoElement, Label, LabelCommon as _, ListItem, ListItemSpacing, ParentElement, RenderOnce,
    SharedString, Styled, StyledExt, ToggleButton, ToggleState, Toggleable, Window, div, h_flex,
    relative, rems, v_flex,
};
use util::ResultExt;
use workspace::{ModalView, Workspace};

use crate::{attach_modal::AttachModal, debugger_panel::DebugPanel};

#[derive(Clone)]
pub(super) struct NewSessionModal {
    workspace: WeakEntity<Workspace>,
    debug_panel: WeakEntity<DebugPanel>,
    mode: NewSessionMode,
    stop_on_entry: ToggleState,
    initialize_args: Option<serde_json::Value>,
    debugger: Option<SharedString>,
    last_selected_profile_name: Option<SharedString>,
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
    pub(super) fn new(
        past_debug_definition: Option<DebugTaskDefinition>,
        debug_panel: WeakEntity<DebugPanel>,
        workspace: WeakEntity<Workspace>,
        task_store: Option<Entity<TaskStore>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let debugger = past_debug_definition
            .as_ref()
            .map(|def| def.adapter.clone());

        let stop_on_entry = past_debug_definition
            .as_ref()
            .and_then(|def| def.stop_on_entry);

        let launch_config = match past_debug_definition.map(|def| def.request) {
            Some(DebugRequest::Launch(launch_config)) => Some(launch_config),
            _ => None,
        };

        if let Some(task_store) = task_store {
            cx.defer_in(window, |this, window, cx| {
                this.mode = NewSessionMode::scenario(
                    this.debug_panel.clone(),
                    this.workspace.clone(),
                    task_store,
                    window,
                    cx,
                );
            });
        };

        Self {
            workspace: workspace.clone(),
            debugger,
            debug_panel,
            mode: NewSessionMode::launch(launch_config, window, cx),
            stop_on_entry: stop_on_entry
                .map(Into::into)
                .unwrap_or(ToggleState::Unselected),
            last_selected_profile_name: None,
            initialize_args: None,
        }
    }

    fn debug_config(&self, cx: &App, debugger: &str) -> Option<DebugScenario> {
        let request = self.mode.debug_task(cx)?;
        let label = suggested_label(&request, debugger);
        Some(DebugScenario {
            adapter: debugger.to_owned().into(),
            label,
            request: Some(request),
            initialize_args: self.initialize_args.clone(),
            tcp_connection: None,
            stop_on_entry: match self.stop_on_entry {
                ToggleState::Selected => Some(true),
                _ => None,
            },
            build: None,
        })
    }

    fn start_new_session(&self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(debugger) = self.debugger.as_ref() else {
            // todo(debugger): show in UI.
            log::error!("No debugger selected");
            return;
        };

        if let NewSessionMode::Scenario(picker) = &self.mode {
            picker.update(cx, |picker, cx| {
                picker.delegate.confirm(false, window, cx);
            });
            return;
        }

        let Some(config) = self.debug_config(cx, debugger) else {
            log::error!("debug config not found in mode: {}", self.mode);
            return;
        };

        let debug_panel = self.debug_panel.clone();
        let workspace = self.workspace.clone();

        cx.spawn_in(window, async move |this, cx| {
            let task_contexts = workspace
                .update_in(cx, |workspace, window, cx| {
                    tasks_ui::task_contexts(workspace, window, cx)
                })?
                .await;

            let task_context = task_contexts.active_context().cloned().unwrap_or_default();

            debug_panel.update_in(cx, |debug_panel, window, cx| {
                debug_panel.start_session(config, task_context, None, window, cx)
            })?;
            this.update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
            anyhow::Result::<_, anyhow::Error>::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn update_attach_picker(
        attach: &Entity<AttachMode>,
        selected_debugger: &str,
        window: &mut Window,
        cx: &mut App,
    ) {
        attach.update(cx, |this, cx| {
            if selected_debugger != this.definition.adapter.as_ref() {
                let adapter: SharedString = selected_debugger.to_owned().into();
                this.definition.adapter = adapter.clone();

                this.attach_picker.update(cx, |this, cx| {
                    this.picker.update(cx, |this, cx| {
                        this.delegate.definition.adapter = adapter;
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
    ) -> Option<ui::DropdownMenu> {
        let workspace = self.workspace.clone();
        let language_registry = self
            .workspace
            .update(cx, |this, cx| this.app_state().languages.clone())
            .ok()?;
        let weak = cx.weak_entity();
        let debugger = self.debugger.clone();
        DropdownMenu::new(
            "dap-adapter-picker",
            debugger
                .as_ref()
                .unwrap_or_else(|| &SELECT_DEBUGGER_LABEL)
                .clone(),
            ContextMenu::build(window, cx, move |mut menu, _, cx| {
                let setter_for_name = |name: SharedString| {
                    let weak = weak.clone();
                    move |window: &mut Window, cx: &mut App| {
                        weak.update(cx, |this, cx| {
                            this.debugger = Some(name.clone());
                            cx.notify();
                            if let NewSessionMode::Attach(attach) = &this.mode {
                                Self::update_attach_picker(&attach, &name, window, cx);
                            }
                        })
                        .ok();
                    }
                };

                let available_adapters = workspace
                    .update(cx, |_, cx| DapRegistry::global(cx).enumerate_adapters())
                    .ok()
                    .unwrap_or_default();

                for adapter in available_adapters {
                    menu = menu.entry(adapter.0.clone(), None, setter_for_name(adapter.0.clone()));
                }
                menu
            }),
        )
        .into()
    }

    fn debug_config_drop_down_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ui::DropdownMenu {
        let workspace = self.workspace.clone();
        let weak = cx.weak_entity();
        let last_profile = self.last_selected_profile_name.clone();
        let worktree = workspace
            .update(cx, |this, cx| {
                this.project().read(cx).visible_worktrees(cx).next()
            })
            .unwrap_or_default();
        DropdownMenu::new(
            "debug-config-menu",
            last_profile.unwrap_or_else(|| SELECT_SCENARIO_LABEL.clone()),
            ContextMenu::build(window, cx, move |mut menu, _, cx| {
                let setter_for_name = |task: DebugScenario| {
                    let weak = weak.clone();
                    move |window: &mut Window, cx: &mut App| {
                        weak.update(cx, |this, cx| {
                            this.last_selected_profile_name = Some(SharedString::from(&task.label));
                            this.debugger = Some(task.adapter.clone());
                            this.initialize_args = task.initialize_args.clone();
                            match &task.request {
                                Some(DebugRequest::Launch(launch_config)) => {
                                    this.mode = NewSessionMode::launch(
                                        Some(launch_config.clone()),
                                        window,
                                        cx,
                                    );
                                }
                                Some(DebugRequest::Attach(_)) => {
                                    let Some(workspace) = this.workspace.upgrade() else {
                                        return;
                                    };
                                    this.mode = NewSessionMode::attach(
                                        this.debugger.clone(),
                                        workspace,
                                        window,
                                        cx,
                                    );
                                    this.mode.focus_handle(cx).focus(window);
                                    if let Some((debugger, attach)) =
                                        this.debugger.as_ref().zip(this.mode.as_attach())
                                    {
                                        Self::update_attach_picker(&attach, &debugger, window, cx);
                                    }
                                }
                                _ => log::warn!("Selected debug scenario without either attach or launch request specified"),
                            }
                            cx.notify();
                        })
                        .ok();
                    }
                };

                let available_tasks: Vec<DebugScenario> = workspace
                    .update(cx, |this, cx| {
                        this.project()
                            .read(cx)
                            .task_store()
                            .read(cx)
                            .task_inventory()
                            .iter()
                            .flat_map(|task_inventory| {
                                task_inventory.read(cx).list_debug_scenarios(
                                    worktree
                                        .as_ref()
                                        .map(|worktree| worktree.read(cx).id())
                                        .iter()
                                        .copied(),
                                )
                            })
                            .map(|(_source_kind, scenario)| scenario)
                            .collect()
                    })
                    .ok()
                    .unwrap_or_default();

                for debug_definition in available_tasks {
                    menu = menu.entry(
                        debug_definition.label.clone(),
                        None,
                        setter_for_name(debug_definition),
                    );
                }
                menu
            }),
        )
    }
}

static SELECT_DEBUGGER_LABEL: SharedString = SharedString::new_static("Select Debugger");
static SELECT_SCENARIO_LABEL: SharedString = SharedString::new_static("Select Profile");

#[derive(Clone)]
enum NewSessionMode {
    Launch(Entity<LaunchMode>),
    Scenario(Entity<Picker<DebugScenarioDelegate>>),
    Attach(Entity<AttachMode>),
}

impl NewSessionMode {
    fn debug_task(&self, cx: &App) -> Option<DebugRequest> {
        match self {
            NewSessionMode::Launch(entity) => Some(entity.read(cx).debug_task(cx).into()),
            NewSessionMode::Attach(entity) => Some(entity.read(cx).debug_task().into()),
            NewSessionMode::Scenario(_) => None,
        }
    }
    fn as_attach(&self) -> Option<&Entity<AttachMode>> {
        if let NewSessionMode::Attach(entity) = self {
            Some(entity)
        } else {
            None
        }
    }

    fn scenario(
        debug_panel: WeakEntity<DebugPanel>,
        workspace: WeakEntity<Workspace>,
        task_store: Entity<TaskStore>,
        window: &mut Window,
        cx: &mut Context<NewSessionModal>,
    ) -> NewSessionMode {
        let picker = cx.new(|cx| {
            Picker::uniform_list(
                DebugScenarioDelegate::new(debug_panel, workspace, task_store),
                window,
                cx,
            )
            .modal(false)
        });

        cx.subscribe(&picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        })
        .detach();

        picker.focus_handle(cx).focus(window);
        NewSessionMode::Scenario(picker)
    }

    fn attach(
        debugger: Option<SharedString>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<NewSessionModal>,
    ) -> Self {
        Self::Attach(AttachMode::new(debugger, workspace, window, cx))
    }

    fn launch(
        past_launch_config: Option<LaunchRequest>,
        window: &mut Window,
        cx: &mut Context<NewSessionModal>,
    ) -> Self {
        Self::Launch(LaunchMode::new(past_launch_config, window, cx))
    }

    fn has_match(&self, cx: &App) -> bool {
        match self {
            NewSessionMode::Scenario(picker) => picker.read(cx).delegate.match_count() > 0,
            NewSessionMode::Attach(picker) => {
                picker
                    .read(cx)
                    .attach_picker
                    .read(cx)
                    .picker
                    .read(cx)
                    .delegate
                    .match_count()
                    > 0
            }
            _ => false,
        }
    }
}

impl std::fmt::Display for NewSessionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self {
            NewSessionMode::Launch(_) => "launch".to_owned(),
            NewSessionMode::Attach(_) => "attach".to_owned(),
            NewSessionMode::Scenario(_) => "scenario picker".to_owned(),
        };

        write!(f, "{}", mode)
    }
}

impl Focusable for NewSessionMode {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self {
            NewSessionMode::Launch(entity) => entity.read(cx).program.focus_handle(cx),
            NewSessionMode::Attach(entity) => entity.read(cx).attach_picker.focus_handle(cx),
            NewSessionMode::Scenario(entity) => entity.read(cx).focus_handle(cx),
        }
    }
}

impl RenderOnce for LaunchMode {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
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
                div().child(
                    Label::new("Working Directory")
                        .size(ui::LabelSize::Small)
                        .color(Color::Muted),
                ),
            )
            .child(render_editor(&self.cwd, window, cx))
    }
}

impl RenderOnce for AttachMode {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .w_full()
            .track_focus(&self.attach_picker.focus_handle(cx))
            .child(self.attach_picker.clone())
    }
}

impl RenderOnce for NewSessionMode {
    fn render(self, window: &mut Window, cx: &mut App) -> impl ui::IntoElement {
        match self {
            NewSessionMode::Launch(entity) => entity.update(cx, |this, cx| {
                this.clone().render(window, cx).into_any_element()
            }),
            NewSessionMode::Attach(entity) => entity.update(cx, |this, cx| {
                this.clone().render(window, cx).into_any_element()
            }),
            NewSessionMode::Scenario(entity) => v_flex()
                .w(rems(34.))
                .child(entity.clone())
                .into_any_element(),
        }
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
        v_flex()
            .size_full()
            .w(rems(34.))
            .elevation_3(cx)
            .bg(cx.theme().colors().elevated_surface_background)
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| {
                cx.emit(DismissEvent);
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
                                ToggleButton::new("debugger-session-ui-picker-button", "Scenarios")
                                    .size(ButtonSize::Default)
                                    .style(ui::ButtonStyle::Subtle)
                                    .toggle_state(matches!(self.mode, NewSessionMode::Scenario(_)))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        let Some(task_store) = this
                                            .workspace
                                            .update(cx, |workspace, cx| {
                                                workspace.project().read(cx).task_store().clone()
                                            })
                                            .ok()
                                        else {
                                            return;
                                        };

                                        this.mode = NewSessionMode::scenario(
                                            this.debug_panel.clone(),
                                            this.workspace.clone(),
                                            task_store,
                                            window,
                                            cx,
                                        );

                                        cx.notify();
                                    }))
                                    .first(),
                            )
                            .child(
                                ToggleButton::new(
                                    "debugger-session-ui-launch-button",
                                    "New Session",
                                )
                                .size(ButtonSize::Default)
                                .style(ui::ButtonStyle::Subtle)
                                .toggle_state(matches!(self.mode, NewSessionMode::Launch(_)))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.mode = NewSessionMode::launch(None, window, cx);
                                    this.mode.focus_handle(cx).focus(window);
                                    cx.notify();
                                }))
                                .middle(),
                            )
                            .child(
                                ToggleButton::new(
                                    "debugger-session-ui-attach-button",
                                    "Attach to Process",
                                )
                                .size(ButtonSize::Default)
                                .toggle_state(matches!(self.mode, NewSessionMode::Attach(_)))
                                .style(ui::ButtonStyle::Subtle)
                                .on_click(cx.listener(|this, _, window, cx| {
                                    let Some(workspace) = this.workspace.upgrade() else {
                                        return;
                                    };
                                    this.mode = NewSessionMode::attach(
                                        this.debugger.clone(),
                                        workspace,
                                        window,
                                        cx,
                                    );
                                    this.mode.focus_handle(cx).focus(window);
                                    if let Some((debugger, attach)) =
                                        this.debugger.as_ref().zip(this.mode.as_attach())
                                    {
                                        Self::update_attach_picker(&attach, &debugger, window, cx);
                                    }

                                    cx.notify();
                                }))
                                .last(),
                            ),
                    )
                    .justify_between()
                    .children(self.adapter_drop_down_menu(window, cx))
                    .border_color(cx.theme().colors().border_variant)
                    .border_b_1(),
            )
            .child(v_flex().child(self.mode.clone().render(window, cx)))
            .child(
                h_flex()
                    .justify_between()
                    .gap_2()
                    .p_2()
                    .border_color(cx.theme().colors().border_variant)
                    .border_t_1()
                    .w_full()
                    .child(self.debug_config_drop_down_menu(window, cx))
                    .child(
                        h_flex()
                            .justify_end()
                            .when(matches!(self.mode, NewSessionMode::Launch(_)), |this| {
                                let weak = cx.weak_entity();
                                this.child(
                                    CheckboxWithLabel::new(
                                        "debugger-stop-on-entry",
                                        Label::new("Stop on Entry").size(ui::LabelSize::Small),
                                        self.stop_on_entry,
                                        move |state, _, cx| {
                                            weak.update(cx, |this, _| {
                                                this.stop_on_entry = *state;
                                            })
                                            .ok();
                                        },
                                    )
                                    .checkbox_position(ui::IconPosition::End),
                                )
                            })
                            .child(
                                Button::new("debugger-spawn", "Start")
                                    .on_click(cx.listener(|this, _, window, cx| match &this.mode {
                                        NewSessionMode::Scenario(picker) => {
                                            picker.update(cx, |picker, cx| {
                                                picker.delegate.confirm(true, window, cx)
                                            })
                                        }
                                        _ => this.start_new_session(window, cx),
                                    }))
                                    .disabled(match self.mode {
                                        NewSessionMode::Scenario(_) => !self.mode.has_match(cx),
                                        NewSessionMode::Attach(_) => {
                                            self.debugger.is_none() || !self.mode.has_match(cx)
                                        }
                                        NewSessionMode::Launch(_) => self.debugger.is_none(),
                                    }),
                            ),
                    ),
            )
    }
}

impl EventEmitter<DismissEvent> for NewSessionModal {}
impl Focusable for NewSessionModal {
    fn focus_handle(&self, cx: &ui::App) -> gpui::FocusHandle {
        self.mode.focus_handle(cx)
    }
}

impl ModalView for NewSessionModal {}

// This module makes sure that the modes setup the correct subscriptions whenever they're created
mod session_modes {
    use std::rc::Rc;

    use super::*;

    #[derive(Clone)]
    #[non_exhaustive]
    pub(super) struct LaunchMode {
        pub(super) program: Entity<Editor>,
        pub(super) cwd: Entity<Editor>,
    }

    impl LaunchMode {
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
                this.set_placeholder_text("Program path", cx);

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
            cx.new(|_| Self { program, cwd })
        }

        pub(super) fn debug_task(&self, cx: &App) -> task::LaunchRequest {
            let path = self.cwd.read(cx).text(cx);
            task::LaunchRequest {
                program: self.program.read(cx).text(cx),
                cwd: path.is_empty().not().then(|| PathBuf::from(path)),
                args: Default::default(),
                env: Default::default(),
            }
        }
    }

    #[derive(Clone)]
    pub(super) struct AttachMode {
        pub(super) definition: DebugTaskDefinition,
        pub(super) attach_picker: Entity<AttachModal>,
        _subscription: Rc<Subscription>,
    }

    impl AttachMode {
        pub(super) fn new(
            debugger: Option<SharedString>,
            workspace: Entity<Workspace>,
            window: &mut Window,
            cx: &mut Context<NewSessionModal>,
        ) -> Entity<Self> {
            let definition = DebugTaskDefinition {
                adapter: debugger.clone().unwrap_or_default(),
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

            let subscription = cx.subscribe(&attach_picker, |_, _, _, cx| {
                cx.emit(DismissEvent);
            });

            cx.new(|_| Self {
                definition,
                attach_picker,
                _subscription: Rc::new(subscription),
            })
        }
        pub(super) fn debug_task(&self) -> task::AttachRequest {
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
    }

    impl DebugScenarioDelegate {
        pub(super) fn new(
            debug_panel: WeakEntity<DebugPanel>,
            workspace: WeakEntity<Workspace>,
            task_store: Entity<TaskStore>,
        ) -> Self {
            Self {
                task_store,
                candidates: None,
                selected_index: 0,
                matches: Vec::new(),
                prompt: String::new(),
                debug_panel,
                workspace,
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
            let candidates: Vec<_> = match &self.candidates {
                Some(candidates) => candidates
                    .into_iter()
                    .enumerate()
                    .map(|(index, (_, candidate))| {
                        StringMatchCandidate::new(index, candidate.label.as_ref())
                    })
                    .collect(),
                None => {
                    let worktree_ids: Vec<_> = self
                        .workspace
                        .update(cx, |this, cx| {
                            this.visible_worktrees(cx)
                                .map(|tree| tree.read(cx).id())
                                .collect()
                        })
                        .ok()
                        .unwrap_or_default();

                    let scenarios: Vec<_> = self
                        .task_store
                        .read(cx)
                        .task_inventory()
                        .map(|item| item.read(cx).list_debug_scenarios(worktree_ids.into_iter()))
                        .unwrap_or_default();

                    self.candidates = Some(scenarios.clone());

                    scenarios
                        .into_iter()
                        .enumerate()
                        .map(|(index, (_, candidate))| {
                            StringMatchCandidate::new(index, candidate.label.as_ref())
                        })
                        .collect()
                }
            };

            cx.spawn_in(window, async move |picker, cx| {
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

        fn confirm(
            &mut self,
            _: bool,
            window: &mut Window,
            cx: &mut Context<picker::Picker<Self>>,
        ) {
            let debug_scenario =
                self.matches
                    .get(self.selected_index())
                    .and_then(|match_candidate| {
                        self.candidates
                            .as_ref()
                            .map(|candidates| candidates[match_candidate.candidate_id].clone())
                    });

            let Some((task_source_kind, debug_scenario)) = debug_scenario else {
                return;
            };

            let task_context = if let TaskSourceKind::Worktree {
                id: worktree_id,
                directory_in_worktree: _,
                id_base: _,
            } = task_source_kind
            {
                let workspace = self.workspace.clone();

                cx.spawn_in(window, async move |_, cx| {
                    workspace
                        .update_in(cx, |workspace, window, cx| {
                            tasks_ui::task_contexts(workspace, window, cx)
                        })
                        .ok()?
                        .await
                        .task_context_for_worktree_id(worktree_id)
                        .cloned()
                })
            } else {
                gpui::Task::ready(None)
            };

            cx.spawn_in(window, async move |this, cx| {
                let task_context = task_context.await.unwrap_or_default();

                this.update_in(cx, |this, window, cx| {
                    this.delegate
                        .debug_panel
                        .update(cx, |panel, cx| {
                            panel.start_session(debug_scenario, task_context, None, window, cx);
                        })
                        .ok();

                    cx.emit(DismissEvent);
                })
                .ok();
            })
            .detach();
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
}
