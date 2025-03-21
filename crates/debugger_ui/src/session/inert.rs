use std::path::PathBuf;

use dap::{DebugAdapterConfig, DebugAdapterKind, DebugRequestType};
use editor::{Editor, EditorElement, EditorStyle};
use gpui::{App, AppContext, Entity, EventEmitter, FocusHandle, Focusable, TextStyle, WeakEntity};
use settings::Settings as _;
use task::TCPHost;
use theme::ThemeSettings;
use ui::{
    div, h_flex, relative, v_flex, ActiveTheme as _, ButtonCommon, ButtonLike, Clickable, Context,
    ContextMenu, Disableable, DropdownMenu, FluentBuilder, Icon, IconName, IconSize,
    InteractiveElement, IntoElement, Label, LabelCommon, LabelSize, ParentElement, PopoverMenu,
    PopoverMenuHandle, Render, SharedString, SplitButton, Styled, Window,
};
use workspace::Workspace;

use crate::attach_modal::AttachModal;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
enum SpawnMode {
    #[default]
    Launch,
    Attach,
}

impl SpawnMode {
    fn label(&self) -> &'static str {
        match self {
            SpawnMode::Launch => "Launch",
            SpawnMode::Attach => "Attach",
        }
    }
}

impl From<DebugRequestType> for SpawnMode {
    fn from(request: DebugRequestType) -> Self {
        match request {
            DebugRequestType::Launch => SpawnMode::Launch,
            DebugRequestType::Attach(_) => SpawnMode::Attach,
        }
    }
}

pub(crate) struct InertState {
    focus_handle: FocusHandle,
    selected_debugger: Option<SharedString>,
    program_editor: Entity<Editor>,
    cwd_editor: Entity<Editor>,
    workspace: WeakEntity<Workspace>,
    spawn_mode: SpawnMode,
    popover_handle: PopoverMenuHandle<ContextMenu>,
}

impl InertState {
    pub(super) fn new(
        workspace: WeakEntity<Workspace>,
        default_cwd: &str,
        debug_config: Option<DebugAdapterConfig>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let selected_debugger = debug_config.as_ref().and_then(|config| match config.kind {
            DebugAdapterKind::Lldb => Some("LLDB".into()),
            DebugAdapterKind::Go(_) => Some("Delve".into()),
            DebugAdapterKind::Php(_) => Some("PHP".into()),
            DebugAdapterKind::Javascript(_) => Some("JavaScript".into()),
            DebugAdapterKind::Python(_) => Some("Debugpy".into()),
            _ => None,
        });

        let spawn_mode = debug_config
            .as_ref()
            .map(|config| config.request.clone().into())
            .unwrap_or_default();

        let program = debug_config
            .as_ref()
            .and_then(|config| config.program.to_owned());

        let program_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            if let Some(program) = program {
                editor.insert(&program, window, cx);
            } else {
                editor.set_placeholder_text("Program path", cx);
            }
            editor
        });

        let cwd = debug_config
            .and_then(|config| config.cwd.map(|cwd| cwd.to_owned()))
            .unwrap_or_else(|| PathBuf::from(default_cwd));

        let cwd_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.insert(cwd.to_str().unwrap_or_else(|| default_cwd), window, cx);
            editor.set_placeholder_text("Working directory", cx);
            editor
        });

        Self {
            workspace,
            cwd_editor,
            program_editor,
            selected_debugger,
            spawn_mode,
            focus_handle: cx.focus_handle(),
            popover_handle: Default::default(),
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
        let spawn_button = ButtonLike::new_rounded_left("spawn-debug-session")
            .child(Label::new(self.spawn_mode.label()).size(LabelSize::Small))
            .on_click(cx.listener(|this, _, window, cx| {
                if this.spawn_mode == SpawnMode::Launch {
                    let program = this.program_editor.read(cx).text(cx);
                    let cwd = PathBuf::from(this.cwd_editor.read(cx).text(cx));
                    let kind =
                        kind_for_label(this.selected_debugger.as_deref().unwrap_or_else(|| {
                            unimplemented!(
                                "Automatic selection of a debugger based on users project"
                            )
                        }));
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
                } else {
                    this.attach(window, cx)
                }
            }))
            .disabled(disable_buttons);
        v_flex()
            .track_focus(&self.focus_handle)
            .size_full()
            .gap_1()
            .p_2()
            .child(
                v_flex()
                    .gap_1()
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
                                            .entry(
                                                "JavaScript",
                                                None,
                                                setter_for_name("JavaScript"),
                                            )
                                            .entry("Debugpy", None, setter_for_name("Debugpy"))
                                    }),
                                )),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Self::render_editor(&self.cwd_editor, cx))
                            .map(|this| {
                                let entity = cx.weak_entity();
                                this.child(SplitButton {
                                    left: spawn_button,
                                    right: PopoverMenu::new("debugger-select-spawn-mode")
                                        .trigger(
                                            ButtonLike::new_rounded_right(
                                                "debugger-spawn-button-mode",
                                            )
                                            .layer(ui::ElevationIndex::ModalSurface)
                                            .size(ui::ButtonSize::None)
                                            .child(
                                                div().px_1().child(
                                                    Icon::new(IconName::ChevronDownSmall)
                                                        .size(IconSize::XSmall),
                                                ),
                                            ),
                                        )
                                        .menu(move |window, cx| {
                                            Some(ContextMenu::build(window, cx, {
                                                let entity = entity.clone();
                                                move |this, _, _| {
                                                    this.entry("Launch", None, {
                                                        let entity = entity.clone();
                                                        move |_, cx| {
                                                            let _ =
                                                                entity.update(cx, |this, cx| {
                                                                    this.spawn_mode =
                                                                        SpawnMode::Launch;
                                                                    cx.notify();
                                                                });
                                                        }
                                                    })
                                                    .entry("Attach", None, {
                                                        let entity = entity.clone();
                                                        move |_, cx| {
                                                            let _ =
                                                                entity.update(cx, |this, cx| {
                                                                    this.spawn_mode =
                                                                        SpawnMode::Attach;
                                                                    cx.notify();
                                                                });
                                                        }
                                                    })
                                                }
                                            }))
                                        })
                                        .with_handle(self.popover_handle.clone())
                                        .into_any_element(),
                                })
                            }),
                    ),
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
        let cwd = PathBuf::from(self.cwd_editor.read(cx).text(cx));
        let kind = kind_for_label(self.selected_debugger.as_deref().unwrap_or_else(|| {
            unimplemented!("Automatic selection of a debugger based on users project")
        }));

        let config = DebugAdapterConfig {
            label: "hard coded attach".into(),
            kind,
            request: DebugRequestType::Attach(task::AttachConfig { process_id: None }),
            program: None,
            cwd: Some(cwd),
            initialize_args: None,
            supports_attach: true,
        };

        let _ = self.workspace.update(cx, |workspace, cx| {
            let project = workspace.project().clone();
            workspace.toggle_modal(window, cx, |window, cx| {
                AttachModal::new(project, config, window, cx)
            });
        });
    }
}
