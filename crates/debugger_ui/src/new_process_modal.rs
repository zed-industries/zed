use anyhow::{Context as _, bail};
use collections::{FxHashMap, HashMap, HashSet};
use language::{LanguageName, LanguageRegistry};
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::Arc,
    usize,
};
use tasks_ui::{TaskOverrides, TasksModal};

use dap::{
    DapRegistry, DebugRequest, TelemetrySpawnLocation, adapters::DebugAdapterName, send_telemetry,
};
use editor::{Editor, EditorElement, EditorStyle};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    Action, App, AppContext, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    KeyContext, Render, Subscription, Task, TextStyle, WeakEntity,
};
use itertools::Itertools as _;
use picker::{Picker, PickerDelegate, highlighted_match_with_paths::HighlightedMatch};
use project::{DebugScenarioContext, Project, TaskContexts, TaskSourceKind, task_store::TaskStore};
use settings::Settings;
use task::{DebugScenario, RevealTarget, VariableName, ZedDebugConfig};
use theme::ThemeSettings;
use ui::{
    ActiveTheme, Button, ButtonCommon, ButtonSize, CheckboxWithLabel, Clickable, Color, Context,
    ContextMenu, Disableable, DropdownMenu, FluentBuilder, Icon, IconName, IconSize,
    IconWithIndicator, Indicator, InteractiveElement, IntoElement, KeyBinding, Label,
    LabelCommon as _, LabelSize, ListItem, ListItemSpacing, ParentElement, RenderOnce,
    SharedString, Styled, StyledExt, ToggleButton, ToggleState, Toggleable, Tooltip, Window, div,
    h_flex, relative, rems, v_flex,
};
use util::ResultExt;
use workspace::{ModalView, Workspace, notifications::DetachAndPromptErr, pane};

use crate::{attach_modal::AttachModal, debugger_panel::DebugPanel};

pub(super) struct NewProcessModal {
    workspace: WeakEntity<Workspace>,
    debug_panel: WeakEntity<DebugPanel>,
    mode: NewProcessMode,
    debug_picker: Entity<Picker<DebugDelegate>>,
    attach_mode: Entity<AttachMode>,
    configure_mode: Entity<ConfigureMode>,
    task_mode: TaskMode,
    debugger: Option<DebugAdapterName>,
    _subscriptions: [Subscription; 3],
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

impl NewProcessModal {
    pub(super) fn show(
        workspace: &mut Workspace,
        window: &mut Window,
        mode: NewProcessMode,
        reveal_target: Option<RevealTarget>,
        cx: &mut Context<Workspace>,
    ) {
        let Some(debug_panel) = workspace.panel::<DebugPanel>(cx) else {
            return;
        };
        let task_store = workspace.project().read(cx).task_store().clone();
        let languages = workspace.app_state().languages.clone();

        cx.spawn_in(window, async move |workspace, cx| {
            let task_contexts = workspace.update_in(cx, |workspace, window, cx| {
                // todo(debugger): get the buffer here (if the active item is an editor) and store it so we can pass it to start_session later
                tasks_ui::task_contexts(workspace, window, cx)
            })?;
            workspace.update_in(cx, |workspace, window, cx| {
                let workspace_handle = workspace.weak_handle();
                let project = workspace.project().clone();
                workspace.toggle_modal(window, cx, |window, cx| {
                    let attach_mode =
                        AttachMode::new(None, workspace_handle.clone(), project, window, cx);

                    let debug_picker = cx.new(|cx| {
                        let delegate =
                            DebugDelegate::new(debug_panel.downgrade(), task_store.clone());
                        Picker::uniform_list(delegate, window, cx).modal(false)
                    });

                    let configure_mode = ConfigureMode::new(window, cx);

                    let task_overrides = Some(TaskOverrides { reveal_target });

                    let task_mode = TaskMode {
                        task_modal: cx.new(|cx| {
                            TasksModal::new(
                                task_store.clone(),
                                Arc::new(TaskContexts::default()),
                                task_overrides,
                                false,
                                workspace_handle.clone(),
                                window,
                                cx,
                            )
                        }),
                    };

                    let _subscriptions = [
                        cx.subscribe(&debug_picker, |_, _, _, cx| {
                            cx.emit(DismissEvent);
                        }),
                        cx.subscribe(
                            &attach_mode.read(cx).attach_picker.clone(),
                            |_, _, _, cx| {
                                cx.emit(DismissEvent);
                            },
                        ),
                        cx.subscribe(&task_mode.task_modal, |_, _, _: &DismissEvent, cx| {
                            cx.emit(DismissEvent)
                        }),
                    ];

                    cx.spawn_in(window, {
                        let debug_picker = debug_picker.downgrade();
                        let configure_mode = configure_mode.downgrade();
                        let task_modal = task_mode.task_modal.downgrade();
                        let workspace = workspace_handle.clone();

                        async move |this, cx| {
                            let task_contexts = task_contexts.await;
                            let task_contexts = Arc::new(task_contexts);
                            let lsp_task_sources = task_contexts.lsp_task_sources.clone();
                            let task_position = task_contexts.latest_selection;
                            // Get LSP tasks and filter out based on language vs lsp preference
                            let (lsp_tasks, prefer_lsp) =
                                workspace.update(cx, |workspace, cx| {
                                    let lsp_tasks = editor::lsp_tasks(
                                        workspace.project().clone(),
                                        &lsp_task_sources,
                                        task_position,
                                        cx,
                                    );
                                    let prefer_lsp = workspace
                                        .active_item(cx)
                                        .and_then(|item| item.downcast::<Editor>())
                                        .map(|editor| {
                                            editor
                                                .read(cx)
                                                .buffer()
                                                .read(cx)
                                                .language_settings(cx)
                                                .tasks
                                                .prefer_lsp
                                        })
                                        .unwrap_or(false);
                                    (lsp_tasks, prefer_lsp)
                                })?;

                            let lsp_tasks = lsp_tasks.await;
                            let add_current_language_tasks = !prefer_lsp || lsp_tasks.is_empty();

                            let lsp_tasks = lsp_tasks
                                .into_iter()
                                .flat_map(|(kind, tasks_with_locations)| {
                                    tasks_with_locations
                                        .into_iter()
                                        .sorted_by_key(|(location, task)| {
                                            (location.is_none(), task.resolved_label.clone())
                                        })
                                        .map(move |(_, task)| (kind.clone(), task))
                                })
                                .collect::<Vec<_>>();

                            let Some(task_inventory) = task_store
                                .update(cx, |task_store, _| task_store.task_inventory().cloned())?
                            else {
                                return Ok(());
                            };

                            let (used_tasks, current_resolved_tasks) = task_inventory
                                .update(cx, |task_inventory, cx| {
                                    task_inventory
                                        .used_and_current_resolved_tasks(task_contexts.clone(), cx)
                                })?
                                .await;

                            if let Ok(task) = debug_picker.update(cx, |picker, cx| {
                                picker.delegate.tasks_loaded(
                                    task_contexts.clone(),
                                    languages,
                                    lsp_tasks.clone(),
                                    current_resolved_tasks.clone(),
                                    add_current_language_tasks,
                                    cx,
                                )
                            }) {
                                task.await;
                                debug_picker
                                    .update_in(cx, |picker, window, cx| {
                                        picker.refresh(window, cx);
                                        cx.notify();
                                    })
                                    .ok();
                            }

                            if let Some(active_cwd) = task_contexts
                                .active_context()
                                .and_then(|context| context.cwd.clone())
                            {
                                configure_mode
                                    .update_in(cx, |configure_mode, window, cx| {
                                        configure_mode.load(active_cwd, window, cx);
                                    })
                                    .ok();
                            }

                            task_modal
                                .update_in(cx, |task_modal, window, cx| {
                                    task_modal.tasks_loaded(
                                        task_contexts,
                                        lsp_tasks,
                                        used_tasks,
                                        current_resolved_tasks,
                                        add_current_language_tasks,
                                        window,
                                        cx,
                                    );
                                })
                                .ok();

                            this.update(cx, |_, cx| {
                                cx.notify();
                            })
                            .ok();

                            anyhow::Ok(())
                        }
                    })
                    .detach();

                    Self {
                        debug_picker,
                        attach_mode,
                        configure_mode,
                        task_mode,
                        debugger: None,
                        mode,
                        debug_panel: debug_panel.downgrade(),
                        workspace: workspace_handle,
                        _subscriptions,
                    }
                });
            })?;

            anyhow::Ok(())
        })
        .detach();
    }

    fn render_mode(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let dap_menu = self.adapter_drop_down_menu(window, cx);
        match self.mode {
            NewProcessMode::Task => self
                .task_mode
                .task_modal
                .read(cx)
                .picker
                .clone()
                .into_any_element(),
            NewProcessMode::Attach => self.attach_mode.update(cx, |this, cx| {
                this.clone().render(window, cx).into_any_element()
            }),
            NewProcessMode::Launch => self.configure_mode.update(cx, |this, cx| {
                this.clone().render(dap_menu, window, cx).into_any_element()
            }),
            NewProcessMode::Debug => v_flex()
                .w(rems(34.))
                .child(self.debug_picker.clone())
                .into_any_element(),
        }
    }

    fn mode_focus_handle(&self, cx: &App) -> FocusHandle {
        match self.mode {
            NewProcessMode::Task => self.task_mode.task_modal.focus_handle(cx),
            NewProcessMode::Attach => self.attach_mode.read(cx).attach_picker.focus_handle(cx),
            NewProcessMode::Launch => self.configure_mode.read(cx).program.focus_handle(cx),
            NewProcessMode::Debug => self.debug_picker.focus_handle(cx),
        }
    }

    fn debug_scenario(&self, debugger: &str, cx: &App) -> Task<Option<DebugScenario>> {
        let request = match self.mode {
            NewProcessMode::Launch => {
                DebugRequest::Launch(self.configure_mode.read(cx).debug_request(cx))
            }
            NewProcessMode::Attach => {
                DebugRequest::Attach(self.attach_mode.read(cx).debug_request())
            }
            _ => return Task::ready(None),
        };
        let label = suggested_label(&request, debugger);

        let stop_on_entry = if let NewProcessMode::Launch = &self.mode {
            Some(self.configure_mode.read(cx).stop_on_entry.selected())
        } else {
            None
        };

        let session_scenario = ZedDebugConfig {
            adapter: debugger.to_owned().into(),
            label,
            request,
            stop_on_entry,
        };

        let adapter = cx
            .global::<DapRegistry>()
            .adapter(&session_scenario.adapter);

        cx.spawn(async move |_| adapter?.config_from_zed_format(session_scenario).await.ok())
    }

    fn start_new_session(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.debugger.as_ref().is_none() {
            return;
        }

        if let NewProcessMode::Debug = &self.mode {
            self.debug_picker.update(cx, |picker, cx| {
                picker.delegate.confirm(false, window, cx);
            });
            return;
        }

        if let NewProcessMode::Launch = &self.mode
            && self.configure_mode.read(cx).save_to_debug_json.selected()
        {
            self.save_debug_scenario(window, cx);
        }

        let Some(debugger) = self.debugger.clone() else {
            return;
        };

        let debug_panel = self.debug_panel.clone();
        let Some(task_contexts) = self.task_contexts(cx) else {
            return;
        };

        let task_context = task_contexts.active_context().cloned().unwrap_or_default();
        let worktree_id = task_contexts.worktree();
        let mode = self.mode;
        cx.spawn_in(window, async move |this, cx| {
            let Some(config) = this
                .update(cx, |this, cx| this.debug_scenario(&debugger, cx))?
                .await
            else {
                bail!("debug config not found in mode: {mode}");
            };

            debug_panel.update_in(cx, |debug_panel, window, cx| {
                send_telemetry(&config, TelemetrySpawnLocation::Custom, cx);
                debug_panel.start_session(config, task_context, None, worktree_id, window, cx)
            })?;
            this.update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
            anyhow::Ok(())
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
            if adapter.0 != this.definition.adapter {
                this.definition.adapter = adapter.0.clone();

                this.attach_picker.update(cx, |this, cx| {
                    this.picker.update(cx, |this, cx| {
                        this.delegate.definition.adapter = adapter.0.clone();
                        this.focus(window, cx);
                    })
                });
            }

            cx.notify();
        })
    }

    fn task_contexts(&self, cx: &App) -> Option<Arc<TaskContexts>> {
        self.debug_picker.read(cx).delegate.task_contexts.clone()
    }

    pub fn save_debug_scenario(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let task_contexts = self.task_contexts(cx);
        let Some(adapter) = self.debugger.as_ref() else {
            return;
        };
        let scenario = self.debug_scenario(adapter, cx);
        cx.spawn_in(window, async move |this, cx| {
            let scenario = scenario.await.context("no scenario to save")?;
            let worktree_id = task_contexts
                .context("no task contexts")?
                .worktree()
                .context("no active worktree")?;
            this.update_in(cx, |this, window, cx| {
                this.debug_panel.update(cx, |panel, cx| {
                    panel.save_scenario(scenario, worktree_id, window, cx)
                })
            })??
            .await?;
            this.update_in(cx, |_, _, cx| {
                cx.emit(DismissEvent);
            })
        })
        .detach_and_prompt_err("Failed to edit debug.json", window, cx, |_, _, _| None);
    }

    fn adapter_drop_down_menu(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ui::DropdownMenu {
        let workspace = self.workspace.clone();
        let weak = cx.weak_entity();
        let active_buffer = self.task_contexts(cx).and_then(|tc| {
            tc.active_item_context
                .as_ref()
                .and_then(|aic| aic.1.as_ref().map(|l| l.buffer.clone()))
        });

        let active_buffer_language = active_buffer
            .and_then(|buffer| buffer.read(cx).language())
            .cloned();

        let mut available_adapters: Vec<_> = workspace
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
            if self.debugger.is_none() {
                self.debugger = available_adapters.first().cloned();
            }
        }

        let label = self
            .debugger
            .as_ref()
            .map(|d| d.0.clone())
            .unwrap_or_else(|| SELECT_DEBUGGER_LABEL.clone());

        DropdownMenu::new(
            "dap-adapter-picker",
            label,
            ContextMenu::build(window, cx, move |mut menu, _, _| {
                let setter_for_name = |name: DebugAdapterName| {
                    let weak = weak.clone();
                    move |window: &mut Window, cx: &mut App| {
                        weak.update(cx, |this, cx| {
                            this.debugger = Some(name.clone());
                            cx.notify();
                            if let NewProcessMode::Attach = &this.mode {
                                Self::update_attach_picker(&this.attach_mode, &name, window, cx);
                            }
                        })
                        .ok();
                    }
                };

                for adapter in available_adapters.into_iter() {
                    menu = menu.entry(adapter.0.clone(), None, setter_for_name(adapter.clone()));
                }

                menu
            }),
        )
    }
}

static SELECT_DEBUGGER_LABEL: SharedString = SharedString::new_static("Select Debugger");

#[derive(Clone, Copy)]
pub(crate) enum NewProcessMode {
    Task,
    Launch,
    Attach,
    Debug,
}

impl std::fmt::Display for NewProcessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self {
            NewProcessMode::Task => "Run",
            NewProcessMode::Debug => "Debug",
            NewProcessMode::Attach => "Attach",
            NewProcessMode::Launch => "Launch",
        };

        write!(f, "{}", mode)
    }
}

impl Focusable for NewProcessMode {
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

impl Render for NewProcessModal {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        v_flex()
            .key_context({
                let mut key_context = KeyContext::new_with_defaults();
                key_context.add("Pane");
                key_context.add("RunModal");
                key_context
            })
            .size_full()
            .w(rems(34.))
            .elevation_3(cx)
            .overflow_hidden()
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| {
                cx.emit(DismissEvent);
            }))
            .on_action(cx.listener(|this, _: &pane::ActivateNextItem, window, cx| {
                this.mode = match this.mode {
                    NewProcessMode::Task => NewProcessMode::Debug,
                    NewProcessMode::Debug => NewProcessMode::Attach,
                    NewProcessMode::Attach => NewProcessMode::Launch,
                    NewProcessMode::Launch => NewProcessMode::Task,
                };

                this.mode_focus_handle(cx).focus(window);
            }))
            .on_action(
                cx.listener(|this, _: &pane::ActivatePreviousItem, window, cx| {
                    this.mode = match this.mode {
                        NewProcessMode::Task => NewProcessMode::Launch,
                        NewProcessMode::Debug => NewProcessMode::Task,
                        NewProcessMode::Attach => NewProcessMode::Debug,
                        NewProcessMode::Launch => NewProcessMode::Attach,
                    };

                    this.mode_focus_handle(cx).focus(window);
                }),
            )
            .child(
                h_flex()
                    .p_2()
                    .w_full()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        ToggleButton::new(
                            "debugger-session-ui-tasks-button",
                            NewProcessMode::Task.to_string(),
                        )
                        .size(ButtonSize::Default)
                        .toggle_state(matches!(self.mode, NewProcessMode::Task))
                        .style(ui::ButtonStyle::Subtle)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.mode = NewProcessMode::Task;
                            this.mode_focus_handle(cx).focus(window);
                            cx.notify();
                        }))
                        .tooltip(Tooltip::text("Run predefined task"))
                        .first(),
                    )
                    .child(
                        ToggleButton::new(
                            "debugger-session-ui-launch-button",
                            NewProcessMode::Debug.to_string(),
                        )
                        .size(ButtonSize::Default)
                        .style(ui::ButtonStyle::Subtle)
                        .toggle_state(matches!(self.mode, NewProcessMode::Debug))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.mode = NewProcessMode::Debug;
                            this.mode_focus_handle(cx).focus(window);
                            cx.notify();
                        }))
                        .tooltip(Tooltip::text("Start a predefined debug scenario"))
                        .middle(),
                    )
                    .child(
                        ToggleButton::new(
                            "debugger-session-ui-attach-button",
                            NewProcessMode::Attach.to_string(),
                        )
                        .size(ButtonSize::Default)
                        .toggle_state(matches!(self.mode, NewProcessMode::Attach))
                        .style(ui::ButtonStyle::Subtle)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.mode = NewProcessMode::Attach;

                            if let Some(debugger) = this.debugger.as_ref() {
                                Self::update_attach_picker(&this.attach_mode, debugger, window, cx);
                            }
                            this.mode_focus_handle(cx).focus(window);
                            cx.notify();
                        }))
                        .tooltip(Tooltip::text("Attach the debugger to a running process"))
                        .middle(),
                    )
                    .child(
                        ToggleButton::new(
                            "debugger-session-ui-custom-button",
                            NewProcessMode::Launch.to_string(),
                        )
                        .size(ButtonSize::Default)
                        .toggle_state(matches!(self.mode, NewProcessMode::Launch))
                        .style(ui::ButtonStyle::Subtle)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.mode = NewProcessMode::Launch;
                            this.mode_focus_handle(cx).focus(window);
                            cx.notify();
                        }))
                        .tooltip(Tooltip::text("Launch a new process with a debugger"))
                        .last(),
                    ),
            )
            .child(v_flex().child(self.render_mode(window, cx)))
            .map(|el| {
                let container = h_flex()
                    .w_full()
                    .p_1p5()
                    .gap_2()
                    .justify_between()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant);
                match self.mode {
                    NewProcessMode::Launch => el.child(
                        container
                            .child(
                                h_flex().child(
                                    Button::new("edit-custom-debug", "Edit in debug.json")
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.save_debug_scenario(window, cx);
                                        }))
                                        .disabled(
                                            self.debugger.is_none()
                                                || self
                                                    .configure_mode
                                                    .read(cx)
                                                    .program
                                                    .read(cx)
                                                    .is_empty(cx),
                                        ),
                                ),
                            )
                            .child(
                                Button::new("debugger-spawn", "Start")
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.start_new_session(window, cx)
                                    }))
                                    .disabled(
                                        self.debugger.is_none()
                                            || self
                                                .configure_mode
                                                .read(cx)
                                                .program
                                                .read(cx)
                                                .is_empty(cx),
                                    ),
                            ),
                    ),
                    NewProcessMode::Attach => el.child({
                        let disabled = self.debugger.is_none()
                            || self
                                .attach_mode
                                .read(cx)
                                .attach_picker
                                .read(cx)
                                .picker
                                .read(cx)
                                .delegate
                                .match_count()
                                == 0;
                        let secondary_action = menu::SecondaryConfirm.boxed_clone();
                        container
                            .child(div().children(
                                KeyBinding::for_action(&*secondary_action, window, cx).map(
                                    |keybind| {
                                        Button::new("edit-attach-task", "Edit in debug.json")
                                            .label_size(LabelSize::Small)
                                            .key_binding(keybind)
                                            .on_click(move |_, window, cx| {
                                                window.dispatch_action(
                                                    secondary_action.boxed_clone(),
                                                    cx,
                                                )
                                            })
                                            .disabled(disabled)
                                    },
                                ),
                            ))
                            .child(
                                h_flex()
                                    .child(div().child(self.adapter_drop_down_menu(window, cx))),
                            )
                    }),
                    NewProcessMode::Debug => el,
                    NewProcessMode::Task => el,
                }
            })
    }
}

impl EventEmitter<DismissEvent> for NewProcessModal {}
impl Focusable for NewProcessModal {
    fn focus_handle(&self, cx: &ui::App) -> gpui::FocusHandle {
        self.mode_focus_handle(cx)
    }
}

impl ModalView for NewProcessModal {}

impl RenderOnce for AttachMode {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .w_full()
            .track_focus(&self.attach_picker.focus_handle(cx))
            .child(self.attach_picker)
    }
}

#[derive(Clone)]
pub(super) struct ConfigureMode {
    program: Entity<Editor>,
    cwd: Entity<Editor>,
    stop_on_entry: ToggleState,
    save_to_debug_json: ToggleState,
}

impl ConfigureMode {
    pub(super) fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let program = cx.new(|cx| Editor::single_line(window, cx));
        program.update(cx, |this, cx| {
            this.set_placeholder_text("ENV=Zed ~/bin/program --option", window, cx);
        });

        let cwd = cx.new(|cx| Editor::single_line(window, cx));
        cwd.update(cx, |this, cx| {
            this.set_placeholder_text("Ex: $ZED_WORKTREE_ROOT", window, cx);
        });

        cx.new(|_| Self {
            program,
            cwd,
            stop_on_entry: ToggleState::Unselected,
            save_to_debug_json: ToggleState::Unselected,
        })
    }

    fn load(&mut self, cwd: PathBuf, window: &mut Window, cx: &mut App) {
        self.cwd.update(cx, |editor, cx| {
            if editor.is_empty(cx) {
                editor.set_text(cwd.to_string_lossy(), window, cx);
            }
        });
    }

    pub(super) fn debug_request(&self, cx: &App) -> task::LaunchRequest {
        let cwd_text = self.cwd.read(cx).text(cx);
        let cwd = if cwd_text.is_empty() {
            None
        } else {
            Some(PathBuf::from(cwd_text))
        };

        if cfg!(windows) {
            return task::LaunchRequest {
                program: self.program.read(cx).text(cx),
                cwd,
                args: Default::default(),
                env: Default::default(),
            };
        }
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
            cwd,
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
            .gap_2()
            .track_focus(&self.program.focus_handle(cx))
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Label::new("Debugger")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(adapter_menu),
            )
            .child(
                v_flex()
                    .gap_0p5()
                    .child(
                        Label::new("Program")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(render_editor(&self.program, window, cx)),
            )
            .child(
                v_flex()
                    .gap_0p5()
                    .child(
                        Label::new("Working Directory")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(render_editor(&self.cwd, window, cx)),
            )
            .child(
                CheckboxWithLabel::new(
                    "debugger-stop-on-entry",
                    Label::new("Stop on Entry")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
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
    pub(super) definition: ZedDebugConfig,
    pub(super) attach_picker: Entity<AttachModal>,
}

impl AttachMode {
    pub(super) fn new(
        debugger: Option<DebugAdapterName>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<NewProcessModal>,
    ) -> Entity<Self> {
        let definition = ZedDebugConfig {
            adapter: debugger.unwrap_or(DebugAdapterName("".into())).0,
            label: "Attach New Session Setup".into(),
            request: dap::DebugRequest::Attach(task::AttachRequest { process_id: None }),
            stop_on_entry: Some(false),
        };
        let attach_picker = cx.new(|cx| {
            let modal = AttachModal::new(definition.clone(), workspace, project, false, window, cx);
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

#[derive(Clone)]
pub(super) struct TaskMode {
    pub(super) task_modal: Entity<TasksModal>,
}

pub(super) struct DebugDelegate {
    task_store: Entity<TaskStore>,
    candidates: Vec<(
        Option<TaskSourceKind>,
        Option<LanguageName>,
        DebugScenario,
        Option<DebugScenarioContext>,
    )>,
    selected_index: usize,
    matches: Vec<StringMatch>,
    prompt: String,
    debug_panel: WeakEntity<DebugPanel>,
    task_contexts: Option<Arc<TaskContexts>>,
    divider_index: Option<usize>,
    last_used_candidate_index: Option<usize>,
}

impl DebugDelegate {
    pub(super) fn new(debug_panel: WeakEntity<DebugPanel>, task_store: Entity<TaskStore>) -> Self {
        Self {
            task_store,
            candidates: Vec::default(),
            selected_index: 0,
            matches: Vec::new(),
            prompt: String::new(),
            debug_panel,
            task_contexts: None,
            divider_index: None,
            last_used_candidate_index: None,
        }
    }

    fn get_task_subtitle(
        &self,
        task_kind: &Option<TaskSourceKind>,
        context: &Option<DebugScenarioContext>,
        cx: &mut App,
    ) -> Option<String> {
        match task_kind {
            Some(TaskSourceKind::Worktree {
                id: worktree_id,
                directory_in_worktree,
                ..
            }) => self
                .debug_panel
                .update(cx, |debug_panel, cx| {
                    let project = debug_panel.project().read(cx);
                    let worktrees: Vec<_> = project.visible_worktrees(cx).collect();

                    let mut path = if worktrees.len() > 1
                        && let Some(worktree) = project.worktree_for_id(*worktree_id, cx)
                    {
                        let worktree_path = worktree.read(cx).abs_path();
                        let full_path = worktree_path.join(directory_in_worktree);
                        full_path
                    } else {
                        directory_in_worktree.clone()
                    };

                    match path
                        .components()
                        .next_back()
                        .and_then(|component| component.as_os_str().to_str())
                    {
                        Some(".zed") => {
                            path.push("debug.json");
                        }
                        Some(".vscode") => {
                            path.push("launch.json");
                        }
                        _ => {}
                    }
                    Some(path.display().to_string())
                })
                .unwrap_or_else(|_| Some(directory_in_worktree.display().to_string())),
            Some(TaskSourceKind::AbsPath { abs_path, .. }) => {
                Some(abs_path.to_string_lossy().into_owned())
            }
            Some(TaskSourceKind::Lsp { language_name, .. }) => {
                Some(format!("LSP: {language_name}"))
            }
            Some(TaskSourceKind::Language { .. }) => None,
            _ => context.clone().and_then(|ctx| {
                ctx.task_context
                    .task_variables
                    .get(&VariableName::RelativeFile)
                    .map(|f| format!("in {f}"))
                    .or_else(|| {
                        ctx.task_context
                            .task_variables
                            .get(&VariableName::Dirname)
                            .map(|d| format!("in {d}/"))
                    })
            }),
        }
    }

    fn get_scenario_language(
        languages: &Arc<LanguageRegistry>,
        dap_registry: &DapRegistry,
        scenario: DebugScenario,
    ) -> (Option<LanguageName>, DebugScenario) {
        let language_names = languages.language_names();
        let language_name = dap_registry.adapter_language(&scenario.adapter);

        let language_name = language_name.or_else(|| {
            scenario.label.split_whitespace().find_map(|word| {
                language_names
                    .iter()
                    .find(|name| name.as_ref().eq_ignore_ascii_case(word))
                    .cloned()
            })
        });

        (language_name, scenario)
    }

    pub fn tasks_loaded(
        &mut self,
        task_contexts: Arc<TaskContexts>,
        languages: Arc<LanguageRegistry>,
        lsp_tasks: Vec<(TaskSourceKind, task::ResolvedTask)>,
        current_resolved_tasks: Vec<(TaskSourceKind, task::ResolvedTask)>,
        add_current_language_tasks: bool,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.task_contexts = Some(task_contexts.clone());
        let task = self.task_store.update(cx, |task_store, cx| {
            task_store.task_inventory().map(|inventory| {
                inventory.update(cx, |inventory, cx| {
                    inventory.list_debug_scenarios(
                        &task_contexts,
                        lsp_tasks,
                        current_resolved_tasks,
                        add_current_language_tasks,
                        cx,
                    )
                })
            })
        });

        let valid_adapters: HashSet<_> = cx.global::<DapRegistry>().enumerate_adapters();

        cx.spawn(async move |this, cx| {
            let (recent, scenarios) = if let Some(task) = task {
                task.await
            } else {
                (Vec::new(), Vec::new())
            };

            this.update(cx, |this, cx| {
                if !recent.is_empty() {
                    this.delegate.last_used_candidate_index = Some(recent.len() - 1);
                }

                let dap_registry = cx.global::<DapRegistry>();
                let hide_vscode = scenarios.iter().any(|(kind, _)| match kind {
                    TaskSourceKind::Worktree {
                        id: _,
                        directory_in_worktree: dir,
                        id_base: _,
                    } => dir.ends_with(".zed"),
                    _ => false,
                });

                this.delegate.candidates = recent
                    .into_iter()
                    .map(|(scenario, context)| {
                        let (language_name, scenario) =
                            Self::get_scenario_language(&languages, dap_registry, scenario);
                        (None, language_name, scenario, Some(context))
                    })
                    .chain(
                        scenarios
                            .into_iter()
                            .filter(|(kind, _)| match kind {
                                TaskSourceKind::Worktree {
                                    id: _,
                                    directory_in_worktree: dir,
                                    id_base: _,
                                } => !(hide_vscode && dir.ends_with(".vscode")),
                                _ => true,
                            })
                            .filter(|(_, scenario)| valid_adapters.contains(&scenario.adapter))
                            .map(|(kind, scenario)| {
                                let (language_name, scenario) =
                                    Self::get_scenario_language(&languages, dap_registry, scenario);
                                (Some(kind), language_name, scenario, None)
                            }),
                    )
                    .collect();
            })
            .ok();
        })
    }
}

impl PickerDelegate for DebugDelegate {
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
        "Find a debug task, or debug a command.".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> gpui::Task<()> {
        let candidates = self.candidates.clone();

        cx.spawn_in(window, async move |picker, cx| {
            let candidates: Vec<_> = candidates
                .into_iter()
                .enumerate()
                .map(|(index, (_, _, candidate, _))| {
                    StringMatchCandidate::new(index, candidate.label.as_ref())
                })
                .collect();

            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                true,
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

                    delegate.divider_index = delegate.last_used_candidate_index.and_then(|index| {
                        let index = delegate
                            .matches
                            .partition_point(|matching_task| matching_task.candidate_id <= index);
                        Some(index).and_then(|index| (index != 0).then(|| index - 1))
                    });

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

    fn separators_after_indices(&self) -> Vec<usize> {
        if let Some(i) = self.divider_index {
            vec![i]
        } else {
            Vec::new()
        }
    }

    fn confirm_input(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let text = self.prompt.clone();
        let (task_context, worktree_id) = self
            .task_contexts
            .as_ref()
            .and_then(|task_contexts| {
                Some((
                    task_contexts.active_context().cloned()?,
                    task_contexts.worktree(),
                ))
            })
            .unwrap_or_default();

        let mut args = shlex::split(&text).into_iter().flatten().peekable();
        let mut env = HashMap::default();
        while args.peek().is_some_and(|arg| arg.contains('=')) {
            let arg = args.next().unwrap();
            let (lhs, rhs) = arg.split_once('=').unwrap();
            env.insert(lhs.to_string(), rhs.to_string());
        }

        let program = if let Some(program) = args.next() {
            program
        } else {
            env = HashMap::default();
            text
        };

        let args = args.collect::<Vec<_>>();
        let task = task::TaskTemplate {
            label: "one-off".to_owned(), // TODO: rename using command as label
            env,
            command: program,
            args,
            ..Default::default()
        };

        let Some(location) = self
            .task_contexts
            .as_ref()
            .and_then(|cx| cx.location().cloned())
        else {
            return;
        };
        let file = location.buffer.read(cx).file();
        let language = location.buffer.read(cx).language();
        let language_name = language.as_ref().map(|l| l.name());
        let Some(adapter): Option<DebugAdapterName> =
            language::language_settings::language_settings(language_name, file, cx)
                .debuggers
                .first()
                .map(SharedString::from)
                .map(Into::into)
                .or_else(|| {
                    language.and_then(|l| {
                        l.config()
                            .debuggers
                            .first()
                            .map(SharedString::from)
                            .map(Into::into)
                    })
                })
        else {
            return;
        };
        let locators = cx.global::<DapRegistry>().locators();
        cx.spawn_in(window, async move |this, cx| {
            let Some(debug_scenario) = cx
                .background_spawn(async move {
                    for locator in locators {
                        if let Some(scenario) =
                            // TODO: use a more informative label than "one-off"
                            locator
                                .1
                                .create_scenario(&task, &task.label, &adapter)
                                .await
                        {
                            return Some(scenario);
                        }
                    }
                    None
                })
                .await
            else {
                return;
            };

            this.update_in(cx, |this, window, cx| {
                send_telemetry(&debug_scenario, TelemetrySpawnLocation::ScenarioList, cx);
                this.delegate
                    .debug_panel
                    .update(cx, |panel, cx| {
                        panel.start_session(
                            debug_scenario,
                            task_context,
                            None,
                            worktree_id,
                            window,
                            cx,
                        );
                    })
                    .ok();
                cx.emit(DismissEvent);
            })
            .ok();
        })
        .detach();
    }

    fn confirm(
        &mut self,
        secondary: bool,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) {
        let debug_scenario = self
            .matches
            .get(self.selected_index())
            .and_then(|match_candidate| self.candidates.get(match_candidate.candidate_id).cloned());

        let Some((kind, _, debug_scenario, context)) = debug_scenario else {
            return;
        };

        let context = context.unwrap_or_else(|| {
            self.task_contexts
                .as_ref()
                .and_then(|task_contexts| {
                    Some(DebugScenarioContext {
                        task_context: task_contexts.active_context().cloned()?,
                        active_buffer: None,
                        worktree_id: task_contexts.worktree(),
                    })
                })
                .unwrap_or_default()
        });
        let DebugScenarioContext {
            task_context,
            active_buffer: _,
            worktree_id,
        } = context;

        if secondary {
            let Some(kind) = kind else { return };
            let Some(id) = worktree_id else { return };
            let debug_panel = self.debug_panel.clone();
            cx.spawn_in(window, async move |_, cx| {
                debug_panel
                    .update_in(cx, |debug_panel, window, cx| {
                        debug_panel.go_to_scenario_definition(kind, debug_scenario, id, window, cx)
                    })?
                    .await?;
                anyhow::Ok(())
            })
            .detach();
        } else {
            send_telemetry(&debug_scenario, TelemetrySpawnLocation::ScenarioList, cx);
            self.debug_panel
                .update(cx, |panel, cx| {
                    panel.start_session(
                        debug_scenario,
                        task_context,
                        None,
                        worktree_id,
                        window,
                        cx,
                    );
                })
                .ok();
        }

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<picker::Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_footer(
        &self,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<ui::AnyElement> {
        let current_modifiers = window.modifiers();
        let footer = h_flex()
            .w_full()
            .p_1p5()
            .justify_between()
            .border_t_1()
            .border_color(cx.theme().colors().border_variant)
            .children({
                let action = menu::SecondaryConfirm.boxed_clone();
                if self.matches.is_empty() {
                    Some(
                        Button::new("edit-debug-json", "Edit debug.json")
                            .label_size(LabelSize::Small)
                            .on_click(cx.listener(|_picker, _, window, cx| {
                                window.dispatch_action(
                                    zed_actions::OpenProjectDebugTasks.boxed_clone(),
                                    cx,
                                );
                                cx.emit(DismissEvent);
                            })),
                    )
                } else {
                    KeyBinding::for_action(&*action, window, cx).map(|keybind| {
                        Button::new("edit-debug-task", "Edit in debug.json")
                            .label_size(LabelSize::Small)
                            .key_binding(keybind)
                            .on_click(move |_, window, cx| {
                                window.dispatch_action(action.boxed_clone(), cx)
                            })
                    })
                }
            })
            .map(|this| {
                if (current_modifiers.alt || self.matches.is_empty()) && !self.prompt.is_empty() {
                    let action = picker::ConfirmInput { secondary: false }.boxed_clone();
                    this.children(KeyBinding::for_action(&*action, window, cx).map(|keybind| {
                        Button::new("launch-custom", "Launch Custom")
                            .key_binding(keybind)
                            .on_click(move |_, window, cx| {
                                window.dispatch_action(action.boxed_clone(), cx)
                            })
                    }))
                } else {
                    this.children(KeyBinding::for_action(&menu::Confirm, window, cx).map(
                        |keybind| {
                            let is_recent_selected =
                                self.divider_index >= Some(self.selected_index);
                            let run_entry_label =
                                if is_recent_selected { "Rerun" } else { "Spawn" };

                            Button::new("spawn", run_entry_label)
                                .key_binding(keybind)
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(menu::Confirm.boxed_clone(), cx);
                                })
                        },
                    ))
                }
            });
        Some(footer.into_any_element())
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let hit = &self.matches.get(ix)?;
        let (task_kind, language_name, _scenario, context) = &self.candidates[hit.candidate_id];

        let highlighted_location = HighlightedMatch {
            text: hit.string.clone(),
            highlight_positions: hit.positions.clone(),
            char_count: hit.string.chars().count(),
            color: Color::Default,
        };

        let subtitle = self.get_task_subtitle(task_kind, context, cx);

        let language_icon = language_name.as_ref().and_then(|lang| {
            file_icons::FileIcons::get(cx)
                .get_icon_for_type(&lang.0.to_lowercase(), cx)
                .map(Icon::from_path)
        });

        let (icon, indicator) = match task_kind {
            Some(TaskSourceKind::UserInput) => (Some(Icon::new(IconName::Terminal)), None),
            Some(TaskSourceKind::AbsPath { .. }) => (Some(Icon::new(IconName::Settings)), None),
            Some(TaskSourceKind::Worktree { .. }) => (Some(Icon::new(IconName::FileTree)), None),
            Some(TaskSourceKind::Lsp { language_name, .. }) => (
                file_icons::FileIcons::get(cx)
                    .get_icon_for_type(&language_name.to_lowercase(), cx)
                    .map(Icon::from_path),
                Some(Indicator::icon(
                    Icon::new(IconName::BoltFilled)
                        .color(Color::Muted)
                        .size(IconSize::Small),
                )),
            ),
            Some(TaskSourceKind::Language { name }) => (
                file_icons::FileIcons::get(cx)
                    .get_icon_for_type(&name.to_lowercase(), cx)
                    .map(Icon::from_path),
                None,
            ),
            None => (Some(Icon::new(IconName::HistoryRerun)), None),
        };

        let icon = language_icon.or(icon).map(|icon| {
            IconWithIndicator::new(icon.color(Color::Muted).size(IconSize::Small), indicator)
                .indicator_border_color(Some(cx.theme().colors().border_transparent))
        });

        Some(
            ListItem::new(SharedString::from(format!("debug-scenario-selection-{ix}")))
                .inset(true)
                .start_slot::<IconWithIndicator>(icon)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    v_flex()
                        .items_start()
                        .child(highlighted_location.render(window, cx))
                        .when_some(subtitle, |this, subtitle_text| {
                            this.child(
                                Label::new(subtitle_text)
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                        }),
                ),
        )
    }
}

pub(crate) fn resolve_path(path: &mut String) {
    if path.starts_with('~') {
        let home = paths::home_dir().to_string_lossy().to_string();
        let trimmed_path = path.trim().to_owned();
        *path = trimmed_path.replacen('~', &home, 1);
    } else if let Some(strip_path) = path.strip_prefix(&format!(".{}", std::path::MAIN_SEPARATOR)) {
        *path = format!(
            "$ZED_WORKTREE_ROOT{}{}",
            std::path::MAIN_SEPARATOR,
            &strip_path
        );
    };
}

#[cfg(test)]
impl NewProcessModal {
    pub(crate) fn set_configure(
        &mut self,
        program: impl AsRef<str>,
        cwd: impl AsRef<str>,
        stop_on_entry: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.mode = NewProcessMode::Launch;
        self.debugger = Some(dap::adapters::DebugAdapterName("fake-adapter".into()));

        self.configure_mode.update(cx, |configure, cx| {
            configure.program.update(cx, |editor, cx| {
                editor.clear(window, cx);
                editor.set_text(program.as_ref(), window, cx);
            });

            configure.cwd.update(cx, |editor, cx| {
                editor.clear(window, cx);
                editor.set_text(cwd.as_ref(), window, cx);
            });

            configure.stop_on_entry = match stop_on_entry {
                true => ToggleState::Selected,
                _ => ToggleState::Unselected,
            }
        })
    }

    pub(crate) fn debug_picker_candidate_subtitles(&self, cx: &mut App) -> Vec<String> {
        self.debug_picker.update(cx, |picker, cx| {
            picker
                .delegate
                .candidates
                .iter()
                .filter_map(|(task_kind, _, _, context)| {
                    picker.delegate.get_task_subtitle(task_kind, context, cx)
                })
                .collect()
        })
    }
}
