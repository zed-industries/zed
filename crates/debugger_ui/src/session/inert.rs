use std::path::PathBuf;

use dap::DebugRequestType;
use editor::{Editor, EditorElement, EditorStyle};
use gpui::{App, AppContext, Entity, EventEmitter, FocusHandle, Focusable, TextStyle, WeakEntity};
use settings::Settings as _;
use task::{DebugTaskDefinition, LaunchConfig, TCPHost};
use theme::ThemeSettings;
use ui::{
    ActiveTheme as _, ButtonCommon, ButtonLike, Clickable, Context, ContextMenu, Disableable,
    DropdownMenu, FluentBuilder, Icon, IconName, IconSize, InteractiveElement, IntoElement, Label,
    LabelCommon, LabelSize, ParentElement, PopoverMenu, PopoverMenuHandle, Render, SharedString,
    SplitButton, Styled, Window, div, h_flex, relative, v_flex,
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
            DebugRequestType::Launch(_) => SpawnMode::Launch,
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
        debug_config: Option<DebugTaskDefinition>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let selected_debugger = debug_config
            .as_ref()
            .map(|config| SharedString::from(config.adapter.clone()));

        let spawn_mode = debug_config
            .as_ref()
            .map(|config| config.request.clone().into())
            .unwrap_or_default();

        let program = debug_config
            .as_ref()
            .and_then(|config| match &config.request {
                DebugRequestType::Attach(_) => None,
                DebugRequestType::Launch(launch_config) => Some(launch_config.program.clone()),
            });

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
            .and_then(|config| match &config.request {
                DebugRequestType::Attach(_) => None,
                DebugRequestType::Launch(launch_config) => launch_config.cwd.clone(),
            })
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
    Spawned { config: DebugTaskDefinition },
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
        let workspace = self.workspace.clone();
        let disable_buttons = self.selected_debugger.is_none();
        let spawn_button = ButtonLike::new_rounded_left("spawn-debug-session")
            .child(Label::new(self.spawn_mode.label()).size(LabelSize::Small))
            .on_click(cx.listener(|this, _, window, cx| {
                if this.spawn_mode == SpawnMode::Launch {
                    let program = this.program_editor.read(cx).text(cx);
                    let cwd = PathBuf::from(this.cwd_editor.read(cx).text(cx));
                    let kind = this
                        .selected_debugger
                        .as_deref()
                        .unwrap_or_else(|| {
                            unimplemented!(
                                "Automatic selection of a debugger based on users project"
                            )
                        })
                        .to_string();

                    cx.emit(InertEvent::Spawned {
                        config: DebugTaskDefinition {
                            label: "hard coded".into(),
                            adapter: kind,
                            request: DebugRequestType::Launch(LaunchConfig {
                                program,
                                cwd: Some(cwd),
                                args: Default::default(),
                            }),
                            tcp_connection: Some(TCPHost::default()),
                            initialize_args: None,
                            locator: None,
                            stop_on_entry: None,
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
                                    ContextMenu::build(window, cx, move |mut this, _, cx| {
                                        let setter_for_name = |name: SharedString| {
                                            let weak = weak.clone();
                                            move |_: &mut Window, cx: &mut App| {
                                                let name = name.clone();
                                                weak.update(cx, move |this, cx| {
                                                    this.selected_debugger = Some(name.clone());
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
        let kind = self
            .selected_debugger
            .as_deref()
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                unimplemented!("Automatic selection of a debugger based on users project")
            });

        let config = DebugTaskDefinition {
            label: "hard coded attach".into(),
            adapter: kind,
            request: DebugRequestType::Attach(task::AttachConfig { process_id: None }),
            initialize_args: None,
            locator: None,
            tcp_connection: Some(TCPHost::default()),
            stop_on_entry: None,
        };

        let _ = self.workspace.update(cx, |workspace, cx| {
            let project = workspace.project().clone();
            workspace.toggle_modal(window, cx, |window, cx| {
                AttachModal::new(project, config, window, cx)
            });
        });
    }
}
