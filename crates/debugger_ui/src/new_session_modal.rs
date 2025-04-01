use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    App, AppContext, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render, TextStyle,
    WeakEntity,
};
use settings::Settings;
use theme::ThemeSettings;
use ui::{
    ActiveTheme, Button, ButtonCommon, ButtonSize, Clickable, ContextMenu, Divider, DropdownMenu,
    FixedWidth, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    StyledExt, ToggleButton, Toggleable, Window, h_flex, relative, v_flex,
};
use workspace::{ModalView, Workspace};

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

struct AttachMode {
    focus_handle: FocusHandle,
}

impl AttachMode {
    fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
        })
    }
}

impl Render for AttachMode {
    fn render(&mut self, window: &mut Window, cx: &mut ui::Context<Self>) -> impl IntoElement {
        v_flex().child("Attach mode contents")
    }
}
static SELECT_DEBUGGER_LABEL: SharedString = SharedString::new_static("Select Debugger");
impl Render for LaunchMode {
    fn render(&mut self, window: &mut Window, cx: &mut ui::Context<Self>) -> impl ui::IntoElement {
        let weak = cx.weak_entity();
        let workspace = self.workspace.clone();
        v_flex()
            .w_full()
            .gap_2()
            .track_focus(&self.program.focus_handle(cx))
            .child(render_editor(&self.program, cx))
            .child(render_editor(&self.cwd, cx))
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(DropdownMenu::new(
                        "dap-adapter-picker",
                        self.debugger
                            .as_ref()
                            .unwrap_or_else(|| &SELECT_DEBUGGER_LABEL)
                            .clone(),
                        ContextMenu::build(window, cx, move |mut this, _, cx| {
                            let setter_for_name = |name: SharedString| {
                                let weak = weak.clone();
                                move |_: &mut Window, cx: &mut App| {
                                    let name = name.clone();
                                    weak.update(cx, move |this, cx| {
                                        this.debugger = Some(name.clone());
                                        cx.notify();
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
                                this = this.entry(
                                    adapter.0.clone(),
                                    None,
                                    setter_for_name(adapter.0.clone()),
                                );
                            }
                            this
                        }),
                    ))
                    .child(Button::new("debugger-launch-spawn", "Launch")),
            )
    }
}
#[derive(Clone)]
enum NewSessionMode {
    Launch(Entity<LaunchMode>),
    Attach(Entity<AttachMode>),
}

impl Focusable for NewSessionMode {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self {
            NewSessionMode::Launch(entity) => entity.read(cx).program.focus_handle(cx),
            NewSessionMode::Attach(entity) => entity.read(cx).focus_handle.clone(),
        }
    }
}

impl RenderOnce for NewSessionMode {
    fn render(self, window: &mut Window, cx: &mut App) -> impl ui::IntoElement {
        match self {
            NewSessionMode::Launch(entity) => {
                entity.update(cx, |this, cx| this.render(window, cx).into_any_element())
            }
            NewSessionMode::Attach(entity) => {
                entity.update(cx, |this, cx| this.render(window, cx).into_any_element())
            }
        }
    }
}

impl NewSessionMode {
    fn attach(_workspace: WeakEntity<Workspace>, _window: &mut Window, cx: &mut App) -> Self {
        Self::Attach(AttachMode::new(cx))
    }
    fn launch(workspace: WeakEntity<Workspace>, window: &mut Window, cx: &mut App) -> Self {
        Self::Launch(LaunchMode::new(workspace, window, cx))
    }
}
fn render_editor(editor: &Entity<Editor>, cx: &App) -> impl IntoElement {
    let settings = ThemeSettings::get_global(cx);
    let text_style = TextStyle {
        color: cx.theme().colors().text,
        font_family: settings.buffer_font.family.clone(),
        font_features: settings.buffer_font.features.clone(),
        font_size: settings.buffer_font_size(cx).into(),
        font_weight: settings.buffer_font.weight,
        line_height: relative(settings.buffer_line_height.value()),
        ..Default::default()
    };

    EditorElement::new(
        editor,
        EditorStyle {
            background: cx.theme().colors().editor_background,
            local_player: cx.theme().players().local(),
            text: text_style,
            ..Default::default()
        },
    )
}
impl Render for NewSessionModal {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        v_flex()
            .min_w_80()
            .size_full()
            .elevation_3(cx)
            .bg(cx.theme().colors().elevated_surface_background)
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| {
                cx.emit(DismissEvent);
            }))
            .child(
                h_flex()
                    .w_full()
                    .justify_around()
                    .child(
                        h_flex()
                            .justify_center()
                            .w_full()
                            .child(
                                ToggleButton::new("debugger-session-ui-launch-button", "Launch")
                                    .full_width()
                                    .size(ButtonSize::Large)
                                    .style(ui::ButtonStyle::Filled)
                                    .toggle_state(matches!(self.mode, NewSessionMode::Launch(_)))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.mode = NewSessionMode::launch(
                                            this.workspace.clone(),
                                            window,
                                            cx,
                                        );
                                        this.mode.focus_handle(cx).focus(window);
                                        cx.notify();
                                    }))
                                    .first(),
                            )
                            .border_r_1()
                            .border_color(cx.theme().colors().border),
                    )
                    .child(
                        ToggleButton::new("debugger-session-ui-attach-button", "Attach")
                            .size(ButtonSize::Large)
                            .width(relative(0.5))
                            .toggle_state(matches!(self.mode, NewSessionMode::Attach(_)))
                            .style(ui::ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.mode =
                                    NewSessionMode::attach(this.workspace.clone(), window, cx);
                                this.mode.focus_handle(cx).focus(window);
                                cx.notify();
                            }))
                            .last(),
                    ),
            )
            .child(Divider::horizontal())
            .child(v_flex().p_2().child(self.mode.clone().render(window, cx)))
    }
}

impl EventEmitter<DismissEvent> for NewSessionModal {}
impl Focusable for NewSessionModal {
    fn focus_handle(&self, cx: &ui::App) -> gpui::FocusHandle {
        self.mode.focus_handle(cx)
    }
}

impl ModalView for NewSessionModal {}
