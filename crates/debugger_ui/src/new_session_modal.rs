use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    App, AppContext, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, ManagedView,
    Render, TextStyle, div,
};
use settings::Settings;
use theme::ThemeSettings;
use ui::{
    ActiveTheme, Button, ButtonCommon, ButtonSize, Clickable, Divider, FixedWidth,
    InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString, Styled, StyledExt,
    ToggleButton, Toggleable, Window, h_flex, relative, v_flex,
};
use workspace::ModalView;

#[derive(Clone)]
pub(super) struct NewSessionModal {
    mode: NewSessionMode,
    focus_handle: FocusHandle,
}

impl NewSessionModal {
    pub(super) fn new(window: &mut Window, cx: &mut App) -> Self {
        Self {
            mode: NewSessionMode::launch(window, cx),
            focus_handle: cx.focus_handle(),
        }
    }
}

#[derive(Clone)]
struct LaunchMode {
    program: Entity<Editor>,
    cwd: Entity<Editor>,
    debugger: Option<SharedString>,
}

impl LaunchMode {
    fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let program = cx.new(|cx| Editor::single_line(window, cx));
        program.update(cx, |this, cx| {
            this.set_placeholder_text("Program path", cx);
        });
        let cwd = cx.new(|cx| Editor::single_line(window, cx));
        cwd.update(cx, |this, cx| {
            this.set_placeholder_text("Working Directory", cx);
        });
        cx.new(move |_| Self {
            program,
            cwd,
            debugger: None,
        })
    }
}

impl Render for LaunchMode {
    fn render(&mut self, window: &mut Window, cx: &mut ui::Context<Self>) -> impl ui::IntoElement {
        v_flex()
            .w_full()
            .gap_2()
            .child(render_editor(&self.program, cx))
            .child(render_editor(&self.cwd, cx))
    }
}
#[derive(Clone)]
enum NewSessionMode {
    Launch(Entity<LaunchMode>),
    Attach(),
}

impl RenderOnce for NewSessionMode {
    fn render(self, window: &mut Window, cx: &mut App) -> impl ui::IntoElement {
        match self {
            NewSessionMode::Launch(entity) => div()
                .child(entity.update(cx, |this, cx| this.render(window, cx).into_any_element())),
            NewSessionMode::Attach() => div(),
        }
    }
}

impl NewSessionMode {
    fn launch(window: &mut Window, cx: &mut App) -> Self {
        Self::Launch(LaunchMode::new(window, cx))
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
            .track_focus(&self.focus_handle)
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
                                        this.mode = NewSessionMode::launch(window, cx);
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
                            .toggle_state(matches!(self.mode, NewSessionMode::Attach()))
                            .style(ui::ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.mode = NewSessionMode::Attach();
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
    fn focus_handle(&self, _: &ui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for NewSessionModal {}
