use std::{
    borrow::Cow,
    ops::Not,
    path::{Path, PathBuf},
};

use anyhow::{Result, anyhow};
use dap::DebugRequestType;
use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    App, AppContext, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render, TextStyle,
    WeakEntity,
};
use settings::Settings;
use task::{DebugTaskDefinition, LaunchConfig};
use theme::ThemeSettings;
use ui::{
    ActiveTheme, Button, ButtonCommon, ButtonSize, CheckboxWithLabel, Clickable, Color, Context,
    ContextMenu, Disableable, DropdownMenu, FluentBuilder, InteractiveElement, IntoElement, Label,
    LabelCommon as _, ParentElement, RenderOnce, SharedString, Styled, StyledExt, ToggleButton,
    ToggleState, Toggleable, Window, div, h_flex, relative, rems, v_flex,
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
    debugger: Option<SharedString>,
    last_selected_profile_name: Option<SharedString>,
}

fn suggested_label(request: &DebugRequestType, debugger: &str) -> String {
    match request {
        DebugRequestType::Launch(config) => {
            let last_path_component = Path::new(&config.program)
                .file_name()
                .map(|name| name.to_string_lossy())
                .unwrap_or_else(|| Cow::Borrowed(&config.program));

            format!("{} ({debugger})", last_path_component)
        }
        DebugRequestType::Attach(config) => format!(
            "pid: {} ({debugger})",
            config.process_id.unwrap_or(u32::MAX)
        ),
    }
}

impl NewSessionModal {
    pub(super) fn new(
        past_debug_definition: Option<DebugTaskDefinition>,
        debug_panel: WeakEntity<DebugPanel>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let debugger = past_debug_definition
            .as_ref()
            .map(|def| def.adapter.clone().into());

        let stop_on_entry = past_debug_definition
            .as_ref()
            .and_then(|def| def.stop_on_entry);

        let launch_config = match past_debug_definition.map(|def| def.request) {
            Some(DebugRequestType::Launch(launch_config)) => Some(launch_config),
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
        }
    }

    fn debug_config(&self, cx: &App) -> Option<DebugTaskDefinition> {
        let request = self.mode.debug_task(cx);

        Some(DebugTaskDefinition {
            adapter: self.debugger.clone()?.to_string(),
            label: suggested_label(&request, self.debugger.as_deref()?),
            request,
            initialize_args: None,
            tcp_connection: None,
            locator: None,
            stop_on_entry: match self.stop_on_entry {
                ToggleState::Selected => Some(true),
                _ => None,
            },
        })
    }
    fn start_new_session(&self, cx: &mut Context<Self>) -> Result<()> {
        let workspace = self.workspace.clone();
        let config = self
            .debug_config(cx)
            .ok_or_else(|| anyhow!("Failed to create a debug config"))?;

        let _ = self.debug_panel.update(cx, |panel, _| {
            panel.past_debug_definition = Some(config.clone());
        });

        cx.spawn(async move |this, cx| {
            let project = workspace.update(cx, |workspace, _| workspace.project().clone())?;
            let task =
                project.update(cx, |this, cx| this.start_debug_session(config.into(), cx))?;
            let spawn_result = task.await;
            if spawn_result.is_ok() {
                this.update(cx, |_, cx| {
                    cx.emit(DismissEvent);
                })
                .ok();
            }
            spawn_result?;
            anyhow::Result::<_, anyhow::Error>::Ok(())
        })
        .detach_and_log_err(cx);
        Ok(())
    }

    fn update_attach_picker(
        attach: &Entity<AttachMode>,
        selected_debugger: &str,
        window: &mut Window,
        cx: &mut App,
    ) {
        attach.update(cx, |this, cx| {
            if selected_debugger != this.debug_definition.adapter {
                this.debug_definition.adapter = selected_debugger.into();
                if let Some(project) = this
                    .workspace
                    .read_with(cx, |workspace, _| workspace.project().clone())
                    .ok()
                {
                    this.attach_picker = Some(cx.new(|cx| {
                        let modal = AttachModal::new(
                            project,
                            this.debug_definition.clone(),
                            false,
                            window,
                            cx,
                        );

                        window.focus(&modal.focus_handle(cx));

                        modal
                    }));
                }
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
                    .update(cx, |this, cx| {
                        this.project()
                            .read(cx)
                            .debug_adapters()
                            .enumerate_adapters()
                    })
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
                let setter_for_name = |task: DebugTaskDefinition| {
                    let weak = weak.clone();
                    let workspace = workspace.clone();
                    move |window: &mut Window, cx: &mut App| {
                        weak.update(cx, |this, cx| {
                            this.last_selected_profile_name = Some(SharedString::from(&task.label));
                            this.debugger = Some(task.adapter.clone().into());

                            match &task.request {
                                DebugRequestType::Launch(launch_config) => {
                                    this.mode = NewSessionMode::launch(
                                        Some(launch_config.clone()),
                                        window,
                                        cx,
                                    );
                                }
                                DebugRequestType::Attach(_) => {
                                    this.mode = NewSessionMode::attach(
                                        this.debugger.clone(),
                                        workspace.clone(),
                                        window,
                                        cx,
                                    );
                                    if let Some((debugger, attach)) =
                                        this.debugger.as_ref().zip(this.mode.as_attach())
                                    {
                                        Self::update_attach_picker(&attach, &debugger, window, cx);
                                    }
                                }
                            }
                            cx.notify();
                        })
                        .ok();
                    }
                };

                let available_adapters: Vec<DebugTaskDefinition> = workspace
                    .update(cx, |this, cx| {
                        this.project()
                            .read(cx)
                            .task_store()
                            .read(cx)
                            .task_inventory()
                            .iter()
                            .flat_map(|task_inventory| task_inventory.read(cx).list_debug_tasks())
                            .cloned()
                            .filter_map(|task| task.try_into().ok())
                            .collect()
                    })
                    .ok()
                    .unwrap_or_default();

                for debug_definition in available_adapters {
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
        past_launch_config: Option<LaunchConfig>,
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

    fn debug_task(&self, cx: &App) -> task::LaunchConfig {
        let path = self.cwd.read(cx).text(cx);
        task::LaunchConfig {
            program: self.program.read(cx).text(cx),
            cwd: path.is_empty().not().then(|| PathBuf::from(path)),
            args: Default::default(),
        }
    }
}

#[derive(Clone)]
struct AttachMode {
    workspace: WeakEntity<Workspace>,
    debug_definition: DebugTaskDefinition,
    attach_picker: Option<Entity<AttachModal>>,
    focus_handle: FocusHandle,
}

impl AttachMode {
    fn new(
        debugger: Option<SharedString>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let debug_definition = DebugTaskDefinition {
            label: "Attach New Session Setup".into(),
            request: dap::DebugRequestType::Attach(task::AttachConfig { process_id: None }),
            tcp_connection: None,
            adapter: debugger.clone().unwrap_or_default().into(),
            locator: None,
            initialize_args: None,
            stop_on_entry: Some(false),
        };

        let attach_picker = if let Some(project) = debugger.and(
            workspace
                .read_with(cx, |workspace, _| workspace.project().clone())
                .ok(),
        ) {
            Some(cx.new(|cx| {
                let modal = AttachModal::new(project, debug_definition.clone(), false, window, cx);
                window.focus(&modal.focus_handle(cx));

                modal
            }))
        } else {
            None
        };

        cx.new(|cx| Self {
            workspace,
            debug_definition,
            attach_picker,
            focus_handle: cx.focus_handle(),
        })
    }
    fn debug_task(&self) -> task::AttachConfig {
        task::AttachConfig { process_id: None }
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
    fn debug_task(&self, cx: &App) -> DebugRequestType {
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
            NewSessionMode::Attach(entity) => entity.read(cx).focus_handle.clone(),
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
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        v_flex().w_full().children(self.attach_picker.clone())
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
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        Self::Attach(AttachMode::new(debugger, workspace, window, cx))
    }
    fn launch(past_launch_config: Option<LaunchConfig>, window: &mut Window, cx: &mut App) -> Self {
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
                                    this.mode = NewSessionMode::attach(
                                        this.debugger.clone(),
                                        this.workspace.clone(),
                                        window,
                                        cx,
                                    );
                                    if let Some((debugger, attach)) =
                                        this.debugger.as_ref().zip(this.mode.as_attach())
                                    {
                                        Self::update_attach_picker(&attach, &debugger, window, cx);
                                    }
                                    this.mode.focus_handle(cx).focus(window);
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
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.start_new_session(cx).log_err();
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
