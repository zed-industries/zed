use std::path::PathBuf;

use dap::{DebugAdapterConfig, DebugAdapterKind, DebugRequestType};
use editor::{Editor, EditorElement, EditorStyle};
use gpui::{App, AppContext, Entity, EventEmitter, FocusHandle, Focusable, TextStyle, WeakEntity};
use settings::Settings as _;
use task::TCPHost;
use theme::ThemeSettings;
use ui::{
    h_flex, relative, v_flex, ActiveTheme as _, Button, ButtonCommon, ButtonStyle, Clickable,
    Context, ContextMenu, Disableable, DropdownMenu, InteractiveElement, IntoElement,
    ParentElement, Render, SharedString, Styled, Window,
};
use workspace::Workspace;

use crate::attach_modal::AttachModal;

pub(crate) struct InertState {
    focus_handle: FocusHandle,
    selected_debugger: Option<SharedString>,
    program_editor: Entity<Editor>,
    cwd_editor: Entity<Editor>,
    workspace: WeakEntity<Workspace>,
}

impl InertState {
    pub(super) fn new(
        workspace: WeakEntity<Workspace>,
        default_cwd: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let program_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Program path", cx);
            editor
        });
        let cwd_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.insert(default_cwd, window, cx);
            editor.set_placeholder_text("Working directory", cx);
            editor
        });
        Self {
            workspace,
            cwd_editor,
            program_editor,
            selected_debugger: None,
            focus_handle: cx.focus_handle(),
        }
    }
}
impl Focusable for InertState {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

pub(crate) enum InertEvent {
    Spawned { config: DebugAdapterConfig },
}

impl EventEmitter<InertEvent> for InertState {}

static SELECT_DEBUGGER_LABEL: SharedString = SharedString::new_static("Select Debugger");

impl Render for InertState {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<'_, Self>,
    ) -> impl ui::IntoElement {
        let weak = cx.weak_entity();
        let disable_buttons = self.selected_debugger.is_none();
        v_flex()
            .track_focus(&self.focus_handle)
            .size_full()
            .gap_1()
            .p_2()

            .child(
                v_flex().gap_1()
                    .child(
                        h_flex()
                            .w_full()
                            .gap_2()

                            .child(Self::render_editor(&self.program_editor, cx))
                            .child(
                                h_flex().child(DropdownMenu::new(
                                    "dap-adapter-picker",
                                    self.selected_debugger
                                        .as_ref()
                                        .unwrap_or_else(|| &SELECT_DEBUGGER_LABEL)
                                        .clone(),
                                    ContextMenu::build(window, cx, move |this, _, _| {
                                        let setter_for_name = |name: &'static str| {
                                            let weak = weak.clone();
                                            move |_: &mut Window, cx: &mut App| {
                                                let name = name;
                                                (&weak)
                                                    .update(cx, move |this, _| {
                                                        this.selected_debugger = Some(name.into());
                                                    })
                                                    .ok();
                                            }
                                        };
                                        this.entry("GDB", None, setter_for_name("GDB"))
                                            .entry("Delve", None, setter_for_name("Delve"))
                                            .entry("LLDB", None, setter_for_name("LLDB"))
                                            .entry("PHP", None, setter_for_name("PHP"))
                                            .entry("JavaScript", None, setter_for_name("JavaScript"))
                                            .entry("Debugpy", None, setter_for_name("Debugpy"))
                                    }),
                                )),
                            )
                    )
                    .child(
                        h_flex().gap_2().child(
                            Self::render_editor(&self.cwd_editor, cx),
                        ).child(h_flex()
                            .gap_4()
                            .pl_2()
                            .child(
                                Button::new("launch-dap", "Launch")
                                    .style(ButtonStyle::Filled)
                                    .disabled(disable_buttons)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        let program = this.program_editor.read(cx).text(cx);
                                        let cwd = PathBuf::from(this.cwd_editor.read(cx).text(cx));
                                        let kind = kind_for_label(this.selected_debugger.as_deref().unwrap_or_else(|| unimplemented!("Automatic selection of a debugger based on users project")));
                                        cx.emit(InertEvent::Spawned {
                                            config: DebugAdapterConfig {
                                                label: "hard coded".into(),
                                                kind,
                                                request: DebugRequestType::Launch,
                                                program: Some(program),
                                                cwd: Some(cwd),
                                                initialize_args: None,
                                                supports_attach: false,
                                            },
                                        });
                                    })),
                            )
                            .child(Button::new("attach-dap", "Attach")
                                .style(ButtonStyle::Filled)
                                .disabled(disable_buttons)
                                .on_click(cx.listener(|this, _, window, cx| this.attach(window, cx)))
                            ))
                    )
            )
    }
}

fn kind_for_label(label: &str) -> DebugAdapterKind {
    match label {
        "LLDB" => DebugAdapterKind::Lldb,
        "Debugpy" => DebugAdapterKind::Python(TCPHost::default()),
        "JavaScript" => DebugAdapterKind::Javascript(TCPHost::default()),
        "PHP" => DebugAdapterKind::Php(TCPHost::default()),
        "Delve" => DebugAdapterKind::Go(TCPHost::default()),
        _ => {
            unimplemented!()
        } // Maybe we should set a toast notification here
    }
}
impl InertState {
    fn render_editor(editor: &Entity<Editor>, cx: &Context<Self>) -> impl IntoElement {
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

    fn attach(&self, window: &mut Window, cx: &mut Context<Self>) {
        let process_id = self.program_editor.read(cx).text(cx).parse::<u32>().ok();
        let cwd = PathBuf::from(self.cwd_editor.read(cx).text(cx));
        let kind = kind_for_label(self.selected_debugger.as_deref().unwrap_or_else(|| {
            unimplemented!("Automatic selection of a debugger based on users project")
        }));

        let config = DebugAdapterConfig {
            label: "hard coded attach".into(),
            kind,
            request: DebugRequestType::Attach(task::AttachConfig { process_id }),
            program: None,
            cwd: Some(cwd),
            initialize_args: None,
            supports_attach: true,
        };

        if process_id.is_some() {
            cx.emit(InertEvent::Spawned { config });
        } else {
            let _ = self.workspace.update(cx, |workspace, cx| {
                let project = workspace.project().clone();
                workspace.toggle_modal(window, cx, |window, cx| {
                    AttachModal::new(project, config, window, cx)
                });
            });
        }
    }
}
