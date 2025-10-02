mod active_toolchain;

pub use active_toolchain::ActiveToolchain;
use convert_case::Casing as _;
use editor::Editor;
use file_finder::OpenPathDelegate;
use futures::channel::oneshot;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    Action, Animation, AnimationExt, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, KeyContext, ParentElement, Render, Styled, Subscription, Task, WeakEntity, Window,
    actions, pulsating_between,
};
use language::{Language, LanguageName, Toolchain, ToolchainScope};
use picker::{Picker, PickerDelegate};
use project::{DirectoryLister, Project, ProjectPath, Toolchains, WorktreeId};
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use ui::{
    Divider, HighlightedLabel, KeyBinding, List, ListItem, ListItemSpacing, Navigable,
    NavigableEntry, prelude::*,
};
use util::{ResultExt, maybe, paths::PathStyle, rel_path::RelPath};
use workspace::{ModalView, Workspace};

actions!(
    toolchain,
    [
        /// Selects a toolchain for the current project.
        Select,
        /// Adds a new toolchain for the current project.
        AddToolchain
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(ToolchainSelector::register).detach();
}

pub struct ToolchainSelector {
    state: State,
    create_search_state: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> SearchState + 'static>,
    language: Option<Arc<Language>>,
    project: Entity<Project>,
    language_name: LanguageName,
    worktree_id: WorktreeId,
    relative_path: Arc<RelPath>,
}

#[derive(Clone)]
struct SearchState {
    picker: Entity<Picker<ToolchainSelectorDelegate>>,
}

struct AddToolchainState {
    state: AddState,
    project: Entity<Project>,
    language_name: LanguageName,
    root_path: ProjectPath,
    weak: WeakEntity<ToolchainSelector>,
}

struct ScopePickerState {
    entries: [NavigableEntry; 3],
    selected_scope: ToolchainScope,
}

#[expect(
    dead_code,
    reason = "These tasks have to be kept alive to run to completion"
)]
enum PathInputState {
    WaitingForPath(Task<()>),
    Resolving(Task<()>),
}

enum AddState {
    Path {
        picker: Entity<Picker<file_finder::OpenPathDelegate>>,
        error: Option<Arc<str>>,
        input_state: PathInputState,
        _subscription: Subscription,
    },
    Name {
        toolchain: Toolchain,
        editor: Entity<Editor>,
        scope_picker: ScopePickerState,
    },
}

impl AddToolchainState {
    fn new(
        project: Entity<Project>,
        language_name: LanguageName,
        root_path: ProjectPath,
        window: &mut Window,
        cx: &mut Context<ToolchainSelector>,
    ) -> Entity<Self> {
        let weak = cx.weak_entity();

        cx.new(|cx| {
            let (lister, rx) = Self::create_path_browser_delegate(project.clone(), cx);
            let picker = cx.new(|cx| Picker::uniform_list(lister, window, cx));
            Self {
                state: AddState::Path {
                    _subscription: cx.subscribe(&picker, |_, _, _: &DismissEvent, cx| {
                        cx.stop_propagation();
                    }),
                    picker,
                    error: None,
                    input_state: Self::wait_for_path(rx, window, cx),
                },
                project,
                language_name,
                root_path,
                weak,
            }
        })
    }

    fn create_path_browser_delegate(
        project: Entity<Project>,
        cx: &mut Context<Self>,
    ) -> (OpenPathDelegate, oneshot::Receiver<Option<Vec<PathBuf>>>) {
        let (tx, rx) = oneshot::channel();
        let weak = cx.weak_entity();
        let lister = OpenPathDelegate::new(
            tx,
            DirectoryLister::Project(project),
            false,
            PathStyle::local(),
        )
        .show_hidden()
        .with_footer(Arc::new(move |_, cx| {
            let error = weak
                .read_with(cx, |this, _| {
                    if let AddState::Path { error, .. } = &this.state {
                        error.clone()
                    } else {
                        None
                    }
                })
                .ok()
                .flatten();
            let is_loading = weak
                .read_with(cx, |this, _| {
                    matches!(
                        this.state,
                        AddState::Path {
                            input_state: PathInputState::Resolving(_),
                            ..
                        }
                    )
                })
                .unwrap_or_default();
            Some(
                v_flex()
                    .child(Divider::horizontal())
                    .child(
                        h_flex()
                            .p_1()
                            .justify_between()
                            .gap_2()
                            .child(Label::new("Select Toolchain Path").color(Color::Muted).map(
                                |this| {
                                    if is_loading {
                                        this.with_animation(
                                            "select-toolchain-label",
                                            Animation::new(Duration::from_secs(2))
                                                .repeat()
                                                .with_easing(pulsating_between(0.4, 0.8)),
                                            |label, delta| label.alpha(delta),
                                        )
                                        .into_any()
                                    } else {
                                        this.into_any_element()
                                    }
                                },
                            ))
                            .when_some(error, |this, error| {
                                this.child(Label::new(error).color(Color::Error))
                            }),
                    )
                    .into_any(),
            )
        }));

        (lister, rx)
    }
    fn resolve_path(
        path: PathBuf,
        root_path: ProjectPath,
        language_name: LanguageName,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> PathInputState {
        PathInputState::Resolving(cx.spawn_in(window, async move |this, cx| {
            _ = maybe!(async move {
                let toolchain = project
                    .update(cx, |this, cx| {
                        this.resolve_toolchain(path.clone(), language_name, cx)
                    })?
                    .await;
                let Ok(toolchain) = toolchain else {
                    // Go back to the path input state
                    _ = this.update_in(cx, |this, window, cx| {
                        if let AddState::Path {
                            input_state,
                            picker,
                            error,
                            ..
                        } = &mut this.state
                            && matches!(input_state, PathInputState::Resolving(_))
                        {
                            let Err(e) = toolchain else { unreachable!() };
                            *error = Some(Arc::from(e.to_string()));
                            let (delegate, rx) =
                                Self::create_path_browser_delegate(this.project.clone(), cx);
                            picker.update(cx, |picker, cx| {
                                *picker = Picker::uniform_list(delegate, window, cx);
                                picker.set_query(
                                    Arc::from(path.to_string_lossy().as_ref()),
                                    window,
                                    cx,
                                );
                            });
                            *input_state = Self::wait_for_path(rx, window, cx);
                            this.focus_handle(cx).focus(window);
                        }
                    });
                    return Err(anyhow::anyhow!("Failed to resolve toolchain"));
                };
                let resolved_toolchain_path = project.read_with(cx, |this, cx| {
                    this.find_project_path(&toolchain.path.as_ref(), cx)
                })?;

                // Suggest a default scope based on the applicability.
                let scope = if let Some(project_path) = resolved_toolchain_path {
                    if !root_path.path.as_ref().is_empty() && project_path.starts_with(&root_path) {
                        ToolchainScope::Subproject(root_path.worktree_id, root_path.path)
                    } else {
                        ToolchainScope::Project
                    }
                } else {
                    // This path lies outside of the project.
                    ToolchainScope::Global
                };

                _ = this.update_in(cx, |this, window, cx| {
                    let scope_picker = ScopePickerState {
                        entries: std::array::from_fn(|_| NavigableEntry::focusable(cx)),
                        selected_scope: scope,
                    };
                    this.state = AddState::Name {
                        editor: cx.new(|cx| {
                            let mut editor = Editor::single_line(window, cx);
                            editor.set_text(toolchain.name.as_ref(), window, cx);
                            editor
                        }),
                        toolchain,
                        scope_picker,
                    };
                    this.focus_handle(cx).focus(window);
                });

                Result::<_, anyhow::Error>::Ok(())
            })
            .await;
        }))
    }

    fn wait_for_path(
        rx: oneshot::Receiver<Option<Vec<PathBuf>>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> PathInputState {
        let task = cx.spawn_in(window, async move |this, cx| {
            maybe!(async move {
                let result = rx.await.log_err()?;

                let path = result
                    .into_iter()
                    .flat_map(|paths| paths.into_iter())
                    .next()?;
                this.update_in(cx, |this, window, cx| {
                    if let AddState::Path {
                        input_state, error, ..
                    } = &mut this.state
                        && matches!(input_state, PathInputState::WaitingForPath(_))
                    {
                        error.take();
                        *input_state = Self::resolve_path(
                            path,
                            this.root_path.clone(),
                            this.language_name.clone(),
                            this.project.clone(),
                            window,
                            cx,
                        );
                    }
                })
                .ok()?;
                Some(())
            })
            .await;
        });
        PathInputState::WaitingForPath(task)
    }

    fn confirm_toolchain(
        &mut self,
        _: &menu::Confirm,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let AddState::Name {
            toolchain,
            editor,
            scope_picker,
        } = &mut self.state
        else {
            return;
        };

        let text = editor.read(cx).text(cx);
        if text.is_empty() {
            return;
        }

        toolchain.name = SharedString::from(text);
        self.project.update(cx, |this, cx| {
            this.add_toolchain(toolchain.clone(), scope_picker.selected_scope.clone(), cx);
        });
        _ = self.weak.update(cx, |this, cx| {
            this.state = State::Search((this.create_search_state)(window, cx));
            this.focus_handle(cx).focus(window);
            cx.notify();
        });
    }
}
impl Focusable for AddToolchainState {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.state {
            AddState::Path { picker, .. } => picker.focus_handle(cx),
            AddState::Name { editor, .. } => editor.focus_handle(cx),
        }
    }
}

impl AddToolchainState {
    fn select_scope(&mut self, scope: ToolchainScope, cx: &mut Context<Self>) {
        if let AddState::Name { scope_picker, .. } = &mut self.state {
            scope_picker.selected_scope = scope;
            cx.notify();
        }
    }
}

impl Focusable for State {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match self {
            State::Search(state) => state.picker.focus_handle(cx),
            State::AddToolchain(state) => state.focus_handle(cx),
        }
    }
}
impl Render for AddToolchainState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let weak = self.weak.upgrade();
        let label = SharedString::new_static("Add");

        v_flex()
            .size_full()
            // todo: These modal styles shouldn't be needed as the modal picker already has `elevation_3`
            // They get duplicated in the middle state of adding a virtual env, but then are needed for this last state
            .bg(cx.theme().colors().elevated_surface_background)
            .border_1()
            .border_color(cx.theme().colors().border_variant)
            .rounded_lg()
            .when_some(weak, |this, weak| {
                this.on_action(window.listener_for(
                    &weak,
                    |this: &mut ToolchainSelector, _: &menu::Cancel, window, cx| {
                        this.state = State::Search((this.create_search_state)(window, cx));
                        this.state.focus_handle(cx).focus(window);
                        cx.notify();
                    },
                ))
            })
            .on_action(cx.listener(Self::confirm_toolchain))
            .map(|this| match &self.state {
                AddState::Path { picker, .. } => this.child(picker.clone()),
                AddState::Name {
                    editor,
                    scope_picker,
                    ..
                } => {
                    let scope_options = [
                        ToolchainScope::Global,
                        ToolchainScope::Project,
                        ToolchainScope::Subproject(
                            self.root_path.worktree_id,
                            self.root_path.path.clone(),
                        ),
                    ];

                    let mut navigable_scope_picker = Navigable::new(
                        v_flex()
                            .child(
                                h_flex()
                                    .w_full()
                                    .p_2()
                                    .border_b_1()
                                    .border_color(theme.colors().border)
                                    .child(editor.clone()),
                            )
                            .child(
                                v_flex()
                                    .child(
                                        Label::new("Scope")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                            .mt_1()
                                            .ml_2(),
                                    )
                                    .child(List::new().children(
                                        scope_options.iter().enumerate().map(|(i, scope)| {
                                            let is_selected = *scope == scope_picker.selected_scope;
                                            let label = scope.label();
                                            let description = scope.description();
                                            let scope_clone_for_action = scope.clone();
                                            let scope_clone_for_click = scope.clone();

                                            div()
                                                .id(SharedString::from(format!("scope-option-{i}")))
                                                .track_focus(&scope_picker.entries[i].focus_handle)
                                                .on_action(cx.listener(
                                                    move |this, _: &menu::Confirm, _, cx| {
                                                        this.select_scope(
                                                            scope_clone_for_action.clone(),
                                                            cx,
                                                        );
                                                    },
                                                ))
                                                .child(
                                                    ListItem::new(SharedString::from(format!(
                                                        "scope-{i}"
                                                    )))
                                                    .toggle_state(
                                                        is_selected
                                                            || scope_picker.entries[i]
                                                                .focus_handle
                                                                .contains_focused(window, cx),
                                                    )
                                                    .inset(true)
                                                    .spacing(ListItemSpacing::Sparse)
                                                    .child(
                                                        h_flex()
                                                            .gap_2()
                                                            .child(Label::new(label))
                                                            .child(
                                                                Label::new(description)
                                                                    .size(LabelSize::Small)
                                                                    .color(Color::Muted),
                                                            ),
                                                    )
                                                    .on_click(cx.listener(move |this, _, _, cx| {
                                                        this.select_scope(
                                                            scope_clone_for_click.clone(),
                                                            cx,
                                                        );
                                                    })),
                                                )
                                        }),
                                    ))
                                    .child(Divider::horizontal())
                                    .child(h_flex().p_1p5().justify_end().map(|this| {
                                        let is_disabled = editor.read(cx).is_empty(cx);
                                        let handle = self.focus_handle(cx);
                                        this.child(
                                            Button::new("add-toolchain", label)
                                                .disabled(is_disabled)
                                                .key_binding(KeyBinding::for_action_in(
                                                    &menu::Confirm,
                                                    &handle,
                                                    window,
                                                    cx,
                                                ))
                                                .on_click(cx.listener(|this, _, window, cx| {
                                                    this.confirm_toolchain(
                                                        &menu::Confirm,
                                                        window,
                                                        cx,
                                                    );
                                                }))
                                                .map(|this| {
                                                    if false {
                                                        this.with_animation(
                                                            "inspecting-user-toolchain",
                                                            Animation::new(Duration::from_millis(
                                                                500,
                                                            ))
                                                            .repeat()
                                                            .with_easing(pulsating_between(
                                                                0.4, 0.8,
                                                            )),
                                                            |label, delta| label.alpha(delta),
                                                        )
                                                        .into_any()
                                                    } else {
                                                        this.into_any_element()
                                                    }
                                                }),
                                        )
                                    })),
                            )
                            .into_any_element(),
                    );

                    for entry in &scope_picker.entries {
                        navigable_scope_picker = navigable_scope_picker.entry(entry.clone());
                    }

                    this.child(navigable_scope_picker.render(window, cx))
                }
            })
    }
}

#[derive(Clone)]
enum State {
    Search(SearchState),
    AddToolchain(Entity<AddToolchainState>),
}

impl RenderOnce for State {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        match self {
            State::Search(state) => state.picker.into_any_element(),
            State::AddToolchain(state) => state.into_any_element(),
        }
    }
}
impl ToolchainSelector {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(move |workspace, _: &Select, window, cx| {
            Self::toggle(workspace, window, cx);
        });
        workspace.register_action(move |workspace, _: &AddToolchain, window, cx| {
            let Some(toolchain_selector) = workspace.active_modal::<Self>(cx) else {
                Self::toggle(workspace, window, cx);
                return;
            };

            toolchain_selector.update(cx, |toolchain_selector, cx| {
                toolchain_selector.handle_add_toolchain(&AddToolchain, window, cx);
            });
        });
    }

    fn toggle(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<()> {
        let (_, buffer, _) = workspace
            .active_item(cx)?
            .act_as::<Editor>(cx)?
            .read(cx)
            .active_excerpt(cx)?;
        let project = workspace.project().clone();

        let language_name = buffer.read(cx).language()?.name();
        let worktree_id = buffer.read(cx).file()?.worktree_id(cx);
        let relative_path: Arc<RelPath> = buffer.read(cx).file()?.path().parent()?.into();
        let worktree_root_path = project
            .read(cx)
            .worktree_for_id(worktree_id, cx)?
            .read(cx)
            .abs_path();
        let workspace_id = workspace.database_id()?;
        let weak = workspace.weak_handle();
        cx.spawn_in(window, async move |workspace, cx| {
            let active_toolchain = workspace::WORKSPACE_DB
                .toolchain(
                    workspace_id,
                    worktree_id,
                    relative_path.clone(),
                    language_name.clone(),
                )
                .await
                .ok()
                .flatten();
            workspace
                .update_in(cx, |this, window, cx| {
                    this.toggle_modal(window, cx, move |window, cx| {
                        ToolchainSelector::new(
                            weak,
                            project,
                            active_toolchain,
                            worktree_id,
                            worktree_root_path,
                            relative_path,
                            language_name,
                            window,
                            cx,
                        )
                    });
                })
                .ok();
        })
        .detach();

        Some(())
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        active_toolchain: Option<Toolchain>,
        worktree_id: WorktreeId,
        worktree_root: Arc<Path>,
        relative_path: Arc<RelPath>,
        language_name: LanguageName,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let language_registry = project.read(cx).languages().clone();
        cx.spawn({
            let language_name = language_name.clone();
            async move |this, cx| {
                let language = language_registry
                    .language_for_name(&language_name.0)
                    .await
                    .ok();
                this.update(cx, |this, cx| {
                    this.language = language;
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
        let project_clone = project.clone();
        let language_name_clone = language_name.clone();
        let relative_path_clone = relative_path.clone();

        let create_search_state = Arc::new(move |window: &mut Window, cx: &mut Context<Self>| {
            let toolchain_selector = cx.entity().downgrade();
            let picker = cx.new(|cx| {
                let delegate = ToolchainSelectorDelegate::new(
                    active_toolchain.clone(),
                    toolchain_selector,
                    workspace.clone(),
                    worktree_id,
                    worktree_root.clone(),
                    project_clone.clone(),
                    relative_path_clone.clone(),
                    language_name_clone.clone(),
                    window,
                    cx,
                );
                Picker::uniform_list(delegate, window, cx)
            });
            let picker_focus_handle = picker.focus_handle(cx);
            picker.update(cx, |picker, _| {
                picker.delegate.focus_handle = picker_focus_handle.clone();
            });
            SearchState { picker }
        });

        Self {
            state: State::Search(create_search_state(window, cx)),
            create_search_state,
            language: None,
            project,
            language_name,
            worktree_id,
            relative_path,
        }
    }

    fn handle_add_toolchain(
        &mut self,
        _: &AddToolchain,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(self.state, State::Search(_)) {
            self.state = State::AddToolchain(AddToolchainState::new(
                self.project.clone(),
                self.language_name.clone(),
                ProjectPath {
                    worktree_id: self.worktree_id,
                    path: self.relative_path.clone(),
                },
                window,
                cx,
            ));
            self.state.focus_handle(cx).focus(window);
            cx.notify();
        }
    }
}

impl Render for ToolchainSelector {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("ToolchainSelector");

        v_flex()
            .key_context(key_context)
            .w(rems(34.))
            .on_action(cx.listener(Self::handle_add_toolchain))
            .child(self.state.clone().render(window, cx))
    }
}

impl Focusable for ToolchainSelector {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.state.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for ToolchainSelector {}
impl ModalView for ToolchainSelector {}

pub struct ToolchainSelectorDelegate {
    toolchain_selector: WeakEntity<ToolchainSelector>,
    candidates: Arc<[(Toolchain, Option<ToolchainScope>)]>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    workspace: WeakEntity<Workspace>,
    worktree_id: WorktreeId,
    worktree_abs_path_root: Arc<Path>,
    relative_path: Arc<RelPath>,
    placeholder_text: Arc<str>,
    add_toolchain_text: Arc<str>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    _fetch_candidates_task: Task<Option<()>>,
}

impl ToolchainSelectorDelegate {
    fn new(
        active_toolchain: Option<Toolchain>,
        toolchain_selector: WeakEntity<ToolchainSelector>,
        workspace: WeakEntity<Workspace>,
        worktree_id: WorktreeId,
        worktree_abs_path_root: Arc<Path>,
        project: Entity<Project>,
        relative_path: Arc<RelPath>,
        language_name: LanguageName,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Self {
        let _project = project.clone();
        let path_style = project.read(cx).path_style(cx);

        let _fetch_candidates_task = cx.spawn_in(window, {
            async move |this, cx| {
                let meta = _project
                    .read_with(cx, |this, _| {
                        Project::toolchain_metadata(this.languages().clone(), language_name.clone())
                    })
                    .ok()?
                    .await?;
                let relative_path = this
                    .update(cx, |this, cx| {
                        this.delegate.add_toolchain_text = format!(
                            "Add {}",
                            meta.term.as_ref().to_case(convert_case::Case::Title)
                        )
                        .into();
                        cx.notify();
                        this.delegate.relative_path.clone()
                    })
                    .ok()?;

                let Toolchains {
                    toolchains: available_toolchains,
                    root_path: relative_path,
                    user_toolchains,
                } = _project
                    .update(cx, |this, cx| {
                        this.available_toolchains(
                            ProjectPath {
                                worktree_id,
                                path: relative_path.clone(),
                            },
                            language_name,
                            cx,
                        )
                    })
                    .ok()?
                    .await?;
                let pretty_path = {
                    if relative_path.is_empty() {
                        Cow::Borrowed("worktree root")
                    } else {
                        Cow::Owned(format!("`{}`", relative_path.display(path_style)))
                    }
                };
                let placeholder_text =
                    format!("Select a {} for {pretty_path}…", meta.term.to_lowercase(),).into();
                let _ = this.update_in(cx, move |this, window, cx| {
                    this.delegate.relative_path = relative_path;
                    this.delegate.placeholder_text = placeholder_text;
                    this.refresh_placeholder(window, cx);
                });

                let _ = this.update_in(cx, move |this, window, cx| {
                    this.delegate.candidates = user_toolchains
                        .into_iter()
                        .flat_map(|(scope, toolchains)| {
                            toolchains
                                .into_iter()
                                .map(move |toolchain| (toolchain, Some(scope.clone())))
                        })
                        .chain(
                            available_toolchains
                                .toolchains
                                .into_iter()
                                .map(|toolchain| (toolchain, None)),
                        )
                        .collect();

                    if let Some(active_toolchain) = active_toolchain
                        && let Some(position) = this
                            .delegate
                            .candidates
                            .iter()
                            .position(|(toolchain, _)| *toolchain == active_toolchain)
                    {
                        this.delegate.set_selected_index(position, window, cx);
                    }
                    this.update_matches(this.query(cx), window, cx);
                });

                Some(())
            }
        });
        let placeholder_text = "Select a toolchain…".to_string().into();
        Self {
            toolchain_selector,
            candidates: Default::default(),
            matches: vec![],
            selected_index: 0,
            workspace,
            worktree_id,
            worktree_abs_path_root,
            placeholder_text,
            relative_path,
            _fetch_candidates_task,
            project,
            focus_handle: cx.focus_handle(),
            add_toolchain_text: Arc::from("Add Toolchain"),
        }
    }
    fn relativize_path(path: SharedString, worktree_root: &Path) -> SharedString {
        Path::new(&path.as_ref())
            .strip_prefix(&worktree_root)
            .ok()
            .map(|suffix| Path::new(".").join(suffix))
            .and_then(|path| path.to_str().map(String::from).map(SharedString::from))
            .unwrap_or(path)
    }
}

impl PickerDelegate for ToolchainSelectorDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        self.placeholder_text.clone()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(string_match) = self.matches.get(self.selected_index) {
            let (toolchain, _) = self.candidates[string_match.candidate_id].clone();
            if let Some(workspace_id) = self
                .workspace
                .read_with(cx, |this, _| this.database_id())
                .ok()
                .flatten()
            {
                let workspace = self.workspace.clone();
                let worktree_id = self.worktree_id;
                let path = self.relative_path.clone();
                let relative_path = self.relative_path.clone();
                cx.spawn_in(window, async move |_, cx| {
                    workspace::WORKSPACE_DB
                        .set_toolchain(workspace_id, worktree_id, relative_path, toolchain.clone())
                        .await
                        .log_err();
                    workspace
                        .update(cx, |this, cx| {
                            this.project().update(cx, |this, cx| {
                                this.activate_toolchain(
                                    ProjectPath { worktree_id, path },
                                    toolchain,
                                    cx,
                                )
                            })
                        })
                        .ok()?
                        .await;
                    Some(())
                })
                .detach();
            }
        }
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.toolchain_selector
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self.candidates.clone();
        let worktree_root_path = self.worktree_abs_path_root.clone();
        cx.spawn_in(window, async move |this, cx| {
            let matches = if query.is_empty() {
                candidates
                    .into_iter()
                    .enumerate()
                    .map(|(index, (candidate, _))| {
                        let path =
                            Self::relativize_path(candidate.path.clone(), &worktree_root_path);
                        let string = format!("{}{}", candidate.name, path);
                        StringMatch {
                            candidate_id: index,
                            string,
                            positions: Vec::new(),
                            score: 0.0,
                        }
                    })
                    .collect()
            } else {
                let candidates = candidates
                    .into_iter()
                    .enumerate()
                    .map(|(candidate_id, (toolchain, _))| {
                        let path =
                            Self::relativize_path(toolchain.path.clone(), &worktree_root_path);
                        let string = format!("{}{}", toolchain.name, path);
                        StringMatchCandidate::new(candidate_id, &string)
                    })
                    .collect::<Vec<_>>();
                match_strings(
                    &candidates,
                    &query,
                    false,
                    true,
                    100,
                    &Default::default(),
                    background,
                )
                .await
            };

            this.update(cx, |this, cx| {
                let delegate = &mut this.delegate;
                delegate.matches = matches;
                delegate.selected_index = delegate
                    .selected_index
                    .min(delegate.matches.len().saturating_sub(1));
                cx.notify();
            })
            .log_err();
        })
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches.get(ix)?;
        let (toolchain, scope) = &self.candidates.get(mat.candidate_id)?;

        let label = toolchain.name.clone();
        let path = Self::relativize_path(toolchain.path.clone(), &self.worktree_abs_path_root);
        let (name_highlights, mut path_highlights) = mat
            .positions
            .iter()
            .cloned()
            .partition::<Vec<_>, _>(|index| *index < label.len());
        path_highlights.iter_mut().for_each(|index| {
            *index -= label.len();
        });
        let id: SharedString = format!("toolchain-{ix}",).into();
        Some(
            ListItem::new(id)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(HighlightedLabel::new(label, name_highlights))
                .child(
                    HighlightedLabel::new(path, path_highlights)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .when_some(scope.as_ref(), |this, scope| {
                    let id: SharedString = format!(
                        "delete-custom-toolchain-{}-{}",
                        toolchain.name, toolchain.path
                    )
                    .into();
                    let toolchain = toolchain.clone();
                    let scope = scope.clone();

                    this.end_slot(IconButton::new(id, IconName::Trash).on_click(cx.listener(
                        move |this, _, _, cx| {
                            this.delegate.project.update(cx, |this, cx| {
                                this.remove_toolchain(toolchain.clone(), scope.clone(), cx)
                            });

                            this.delegate.matches.retain_mut(|m| {
                                if m.candidate_id == ix {
                                    return false;
                                } else if m.candidate_id > ix {
                                    m.candidate_id -= 1;
                                }
                                true
                            });

                            this.delegate.candidates = this
                                .delegate
                                .candidates
                                .iter()
                                .enumerate()
                                .filter_map(|(i, toolchain)| (ix != i).then_some(toolchain.clone()))
                                .collect();

                            if this.delegate.selected_index >= ix {
                                this.delegate.selected_index =
                                    this.delegate.selected_index.saturating_sub(1);
                            }
                            cx.stop_propagation();
                            cx.notify();
                        },
                    )))
                }),
        )
    }
    fn render_footer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        Some(
            v_flex()
                .rounded_b_md()
                .child(Divider::horizontal())
                .child(
                    h_flex()
                        .p_1p5()
                        .gap_0p5()
                        .justify_end()
                        .child(
                            Button::new("xd", self.add_toolchain_text.clone())
                                .key_binding(KeyBinding::for_action_in(
                                    &AddToolchain,
                                    &self.focus_handle,
                                    _window,
                                    cx,
                                ))
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(AddToolchain), cx)
                                }),
                        )
                        .child(
                            Button::new("select", "Select")
                                .key_binding(KeyBinding::for_action_in(
                                    &menu::Confirm,
                                    &self.focus_handle,
                                    _window,
                                    cx,
                                ))
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                                }),
                        ),
                )
                .into_any_element(),
        )
    }
}
