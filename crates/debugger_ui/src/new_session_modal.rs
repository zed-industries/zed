use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    App, AppContext, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render, TextStyle,
    WeakEntity,
};
use settings::Settings;
use task::DebugTaskDefinition;
use theme::ThemeSettings;
use ui::{
    ActiveTheme, Button, ButtonCommon, ButtonSize, CheckboxWithLabel, Clickable, Color,
    ContextMenu, DropdownMenu, FluentBuilder, InteractiveElement, IntoElement, Label,
    LabelCommon as _, ParentElement, RenderOnce, SharedString, Styled, StyledExt, ToggleButton,
    ToggleState, Toggleable, Window, div, h_flex, relative, rems, v_flex,
};
use workspace::{ModalView, Workspace};

use crate::attach_modal::AttachModal;

#[derive(Clone)]
pub(super) struct NewSessionModal {
    workspace: WeakEntity<Workspace>,
    mode: NewSessionMode,
}

impl NewSessionModal {
    pub(super) fn new(workspace: WeakEntity<Workspace>, window: &mut Window, cx: &mut App) -> Self {
        Self {
            workspace: workspace.clone(),
            mode: NewSessionMode::launch(workspace, window, cx),
        }
    }
}

#[derive(Clone)]
struct LaunchMode {
    program: Entity<Editor>,
    cwd: Entity<Editor>,
    debugger: Option<SharedString>,
    workspace: WeakEntity<Workspace>,
}

impl LaunchMode {
    fn new(workspace: WeakEntity<Workspace>, window: &mut Window, cx: &mut App) -> Entity<Self> {
        let program = cx.new(|cx| Editor::single_line(window, cx));
        program.update(cx, |this, cx| {
            this.set_placeholder_text("Program path", cx);
        });
        let cwd = cx.new(|cx| Editor::single_line(window, cx));
        cwd.update(cx, |this, cx| {
            this.set_placeholder_text("Working Directory", cx);
        });
        cx.new(|_| Self {
            program,
            cwd,
            debugger: None,
            workspace,
        })
    }
}

#[derive(Clone)]
struct AttachMode {
    workspace: WeakEntity<Workspace>,
    debugger: Option<SharedString>,
    debug_definition: DebugTaskDefinition,
    attach_picker: Option<Entity<AttachModal>>,
    focus_handle: FocusHandle,
}

impl AttachMode {
    fn new(workspace: WeakEntity<Workspace>, cx: &mut App) -> Entity<Self> {
        let debug_definition = DebugTaskDefinition {
            label: "Attach New Session Setup".into(),
            request: dap::DebugRequestType::Attach(task::AttachConfig { process_id: None }),
            tcp_connection: None,
            args: Vec::default(),
            adapter: "".into(),
            locator: None,
            initialize_args: None,
        };

        cx.new(|cx| Self {
            workspace,
            debugger: None,
            debug_definition,
            attach_picker: None,
            focus_handle: cx.focus_handle(),
        })
    }
}

static SELECT_DEBUGGER_LABEL: SharedString = SharedString::new_static("Select Debugger");

#[derive(Clone)]
enum NewSessionMode {
    Launch(Entity<LaunchMode>),
    Attach(Entity<AttachMode>),
}

impl NewSessionMode {
    fn adapter_drop_down_menu(&self, window: &mut Window, cx: &mut App) -> ui::DropdownMenu {
        #[derive(Clone)]
        enum Either {
            Launch(WeakEntity<LaunchMode>),
            Attach(WeakEntity<AttachMode>),
        }

        let (debugger, either, workspace) = match self {
            NewSessionMode::Launch(launch) => {
                let entity = launch.read(cx);
                (
                    entity.debugger.clone(),
                    Either::Launch(launch.downgrade()),
                    entity.workspace.clone(),
                )
            }
            NewSessionMode::Attach(attach) => {
                let entity = attach.read(cx);
                (
                    entity.debugger.clone(),
                    Either::Attach(attach.downgrade()),
                    entity.workspace.clone(),
                )
            }
        };

        DropdownMenu::new(
            "dap-adapter-picker",
            debugger
                .as_ref()
                .unwrap_or_else(|| &SELECT_DEBUGGER_LABEL)
                .clone(),
            ContextMenu::build(window, cx, move |mut menu, _, cx| {
                let setter_for_name = |name: SharedString| {
                    let either = either.clone();
                    move |window: &mut Window, cx: &mut App| match &either {
                        Either::Launch(launch) => {
                            let name = name.clone();
                            launch
                                .update(cx, move |this, cx| {
                                    this.debugger = Some(name);
                                    cx.notify();
                                })
                                .ok();
                        }
                        Either::Attach(attach) => {
                            let name = name.clone();
                            attach
                                .update(cx, |this, cx| {
                                    if name != this.debug_definition.adapter {
                                        this.debug_definition.adapter = name.clone().into();
                                        if let Some(project) = this
                                            .workspace
                                            .read_with(cx, |workspace, _| {
                                                workspace.project().clone()
                                            })
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

                                    this.debugger = Some(name);
                                    cx.notify();
                                })
                                .ok();
                        }
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
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .p_2()
            .w_full()
            .gap_2()
            .track_focus(&self.program.focus_handle(cx))
            .child(
                div().px_1().child(
                    Label::new("Program")
                        .size(ui::LabelSize::Small)
                        .color(Color::Muted),
                ),
            )
            .child(render_editor(&self.program, cx))
            .child(
                div().child(
                    Label::new("Working Directory")
                        .size(ui::LabelSize::Small)
                        .color(Color::Muted),
                ),
            )
            .child(render_editor(&self.cwd, cx))
    }
}

impl RenderOnce for AttachMode {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_2()
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(Button::new("debugger-launch-spawn", "Attach")),
            )
            .when_some(self.attach_picker.clone(), |this, picker| {
                this.child(picker)
            })
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
    fn attach(workspace: WeakEntity<Workspace>, _window: &mut Window, cx: &mut App) -> Self {
        Self::Attach(AttachMode::new(workspace, cx))
    }
    fn launch(workspace: WeakEntity<Workspace>, window: &mut Window, cx: &mut App) -> Self {
        Self::Launch(LaunchMode::new(workspace, window, cx))
    }
}
fn render_editor(editor: &Entity<Editor>, cx: &App) -> impl IntoElement {
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
        .border_color(theme.colors().border_variant)
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
                                    this.mode =
                                        NewSessionMode::launch(this.workspace.clone(), window, cx);
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
                                    this.mode =
                                        NewSessionMode::attach(this.workspace.clone(), window, cx);
                                    this.mode.focus_handle(cx).focus(window);
                                    cx.notify();
                                }))
                                .last(),
                            ),
                    )
                    .justify_between()
                    .child(self.mode.clone().adapter_drop_down_menu(window, cx))
                    .border_color(cx.theme().colors().border_variant)
                    .border_b_1(),
            )
            .child(v_flex().child(self.mode.clone().render(window, cx)))
            .child(
                h_flex()
                    .border_color(cx.theme().colors().border_variant)
                    .border_b_1()
                    .w_full()
                    .justify_end()
                    .p_2()
                    .when(matches!(self.mode, NewSessionMode::Launch(_)), |this| {
                        this.child(CheckboxWithLabel::new(
                            "debugger-stop-on-entry",
                            Label::new("Stop on Entry"),
                            ToggleState::Unselected,
                            |_, _, _| {},
                        ))
                    })
                    .child(Button::new("debugger-spawn", "Start")),
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
