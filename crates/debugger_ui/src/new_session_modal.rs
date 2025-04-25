use std::{
    borrow::Cow,
    ops::Not,
    path::{Path, PathBuf},
};

use dap::{DapRegistry, DebugRequest, adapters::DebugTaskDefinition};
use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    App, AppContext, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render, TextStyle,
    WeakEntity,
};
use settings::Settings;
use task::{DebugScenario, LaunchRequest, TaskContext};
use theme::ThemeSettings;
use ui::{
    ActiveTheme, Button, ButtonCommon, ButtonSize, CheckboxWithLabel, Clickable, Color, Context,
    ContextMenu, Disableable, DropdownMenu, FluentBuilder, InteractiveElement, IntoElement, Label,
    LabelCommon as _, ParentElement, RenderOnce, SharedString, Styled, StyledExt, ToggleButton,
    ToggleState, Toggleable, Window, div, h_flex, relative, rems, v_flex,
};
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

    fn debug_config(&self, cx: &App, debugger: &str) -> DebugScenario {
        let request = self.mode.debug_task(cx);
        let label = suggested_label(&request, debugger);
        DebugScenario {
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
        }
    }

    fn start_new_session(&self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(debugger) = self.debugger.as_ref() else {
            // todo: show in UI.
            log::error!("No debugger selected");
            return;
        };
        let config = self.debug_config(cx, debugger);
        let debug_panel = self.debug_panel.clone();

        cx.spawn_in(window, async move |this, cx| {
            debug_panel.update_in(cx, |debug_panel, window, cx| {
                debug_panel.start_session(config, TaskContext::default(), None, window, cx)
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
    ) -> ui::DropdownMenu {
        let workspace = self.workspace.clone();
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
    }

    fn debug_config_drop_down_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ui::DropdownMenu {
        let workspace = self.workspace.clone();
        let weak = cx.weak_entity();
        let last_profile = self.last_selected_profile_name.clone();
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
                                task_inventory.read(cx).list_debug_scenarios(None)
                            })
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

#[derive(Clone)]
struct LaunchMode {
    program: Entity<Editor>,
    cwd: Entity<Editor>,
}

impl LaunchMode {
    fn new(
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

    fn debug_task(&self, cx: &App) -> task::LaunchRequest {
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
struct AttachMode {
    definition: DebugTaskDefinition,
    attach_picker: Entity<AttachModal>,
}

impl AttachMode {
    fn new(
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
        cx.new(|_| Self {
            definition,
            attach_picker,
        })
    }
    fn debug_task(&self) -> task::AttachRequest {
        task::AttachRequest { process_id: None }
    }
}

static SELECT_DEBUGGER_LABEL: SharedString = SharedString::new_static("Select Debugger");
static SELECT_SCENARIO_LABEL: SharedString = SharedString::new_static("Select Profile");

#[derive(Clone)]
enum NewSessionMode {
    Launch(Entity<LaunchMode>),
    Attach(Entity<AttachMode>),
}

impl NewSessionMode {
    fn debug_task(&self, cx: &App) -> DebugRequest {
        match self {
            NewSessionMode::Launch(entity) => entity.read(cx).debug_task(cx).into(),
            NewSessionMode::Attach(entity) => entity.read(cx).debug_task().into(),
        }
    }
    fn as_attach(&self) -> Option<&Entity<AttachMode>> {
        if let NewSessionMode::Attach(entity) = self {
            Some(entity)
        } else {
            None
        }
    }
}

impl Focusable for NewSessionMode {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self {
            NewSessionMode::Launch(entity) => entity.read(cx).program.focus_handle(cx),
            NewSessionMode::Attach(entity) => entity.read(cx).attach_picker.focus_handle(cx),
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
        }
    }
}

impl NewSessionMode {
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
                                .first(),
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
                    .child(self.adapter_drop_down_menu(window, cx))
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
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.start_new_session(window, cx);
                                    }))
                                    .disabled(self.debugger.is_none()),
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
