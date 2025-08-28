mod active_toolchain;

pub use active_toolchain::ActiveToolchain;
use convert_case::Casing as _;
use editor::Editor;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    Animation, AnimationExt, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, ParentElement, Render, Styled, Task, WeakEntity, Window, actions, pulsating_between,
};
use language::{Language, LanguageName, Toolchain, ToolchainList, ToolchainLister};
use picker::{Picker, PickerDelegate};
use project::{Project, ProjectPath, WorktreeId};
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use ui::{Divider, HighlightedLabel, KeyBinding, ListItem, ListItemSpacing, prelude::*};
use util::{ResultExt, maybe};
use workspace::{ModalView, Workspace};

actions!(
    toolchain,
    [
        /// Selects a toolchain for the current project.
        Select
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(ToolchainSelector::register).detach();
}

pub struct ToolchainSelector {
    state: State,
    create_search_state: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> SearchState + 'static>,
    language: Option<Arc<Language>>,
}

#[derive(Clone)]
struct SearchState {
    picker: Entity<Picker<ToolchainSelectorDelegate>>,
}

struct AddToolchainState {
    state: AddState,
    project: Entity<Project>,
    language_name: LanguageName,
    weak: WeakEntity<ToolchainSelector>,
}

enum AddState {
    Path {
        editor: Entity<Editor>,
        error: Option<Arc<str>>,
        confirm_task: Option<Task<()>>,
    },
    Name {
        toolchain: Toolchain,
        editor: Entity<Editor>,
    },
}

impl AddToolchainState {
    fn new(
        project: Entity<Project>,
        language_name: LanguageName,
        window: &mut Window,
        cx: &mut Context<ToolchainSelector>,
    ) -> Entity<Self> {
        let weak = cx.weak_entity();
        let path = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Enter path", cx);
            editor
        });
        cx.new(|_| Self {
            state: AddState::Path {
                editor: path,
                error: None,
                confirm_task: None,
            },
            project,
            language_name,
            weak,
        })
    }
    fn confirm_toolchain(
        &mut self,
        _: &menu::Confirm,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match &mut self.state {
            AddState::Path {
                editor,
                confirm_task,
                ..
            } => {
                // Set of a confirmation task with our lister.
                let text = editor.read(cx).text(cx);

                if text.is_empty() {
                    return;
                }
                let task = self.project.read(cx).resolve_toolchain(
                    PathBuf::from(text),
                    self.language_name.clone(),
                    cx,
                );
                *confirm_task = Some(cx.spawn_in(window, async move |this, cx| {
                    let t = task.await;
                    this.update_in(cx, |this, window, cx| {
                        let AddState::Path {
                            error,
                            confirm_task,
                            ..
                        } = &mut this.state
                        else {
                            unreachable!("This closure should not complete concurrently")
                        };

                        match t {
                            Ok(toolchain) => {
                                this.state = AddState::Name {
                                    toolchain,
                                    editor: cx.new(|cx| Editor::single_line(window, cx)),
                                };
                            }
                            Err(err) => {
                                *error = Some(err.to_string().into());
                                confirm_task.take();
                            }
                        }

                        cx.notify();
                    })
                    .ok();
                }));
            }
            AddState::Name { toolchain, editor } => {
                let text = editor.read(cx).text(cx);
                if text.is_empty() {
                    return;
                }
            }
        }
    }
    fn editor(&self) -> &Entity<Editor> {
        match &self.state {
            AddState::Path { editor, .. } | AddState::Name { editor, .. } => editor,
        }
    }
}
impl Focusable for AddToolchainState {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor().focus_handle(cx)
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
        let theme = cx.theme();
        let weak = self.weak.upgrade();
        v_flex()
            .size_full()
            .rounded_md()
            .gap_1()
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
            .bg(theme.colors().background)
            .child(
                h_flex().w_full().child(
                    h_flex()
                        .w_full()
                        .bg(theme.colors().editor_background)
                        .p_2()
                        .rounded_sm()
                        .border_1()
                        .border_color(theme.colors().border)
                        .child(self.editor().clone()),
                ),
            )
            .child(
                h_flex()
                    .rounded_md()
                    .w_full()
                    .bg(theme.colors().background)
                    .p_2()
                    .justify_end()
                    .map(|this| {
                        let is_disabled = match &self.state {
                            AddState::Path {
                                confirm_task,
                                editor,
                                ..
                            } => confirm_task.is_some() || editor.read(cx).is_empty(cx),
                            AddState::Name { editor, .. } => editor.read(cx).is_empty(cx),
                        };
                        let (error, confirm_underway) = match &self.state {
                            AddState::Path {
                                error,
                                confirm_task,
                                ..
                            } => (error.clone(), confirm_task.is_some()),
                            _ => (None, false),
                        };
                        this.when_some(error, |this, error| {
                            this.justify_between()
                                .child(Label::new(error).color(Color::Error).size(LabelSize::Small))
                        })
                        .child(
                            Button::new("add-toolchain", "Confirm")
                                .key_binding(KeyBinding::for_action(&menu::Confirm, window, cx))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.confirm_toolchain(&menu::Confirm, window, cx);
                                }))
                                .disabled(is_disabled)
                                .map(|this| {
                                    if confirm_underway {
                                        this.with_animation(
                                            "inspecting-user-toolchain",
                                            Animation::new(Duration::from_millis(500))
                                                .repeat()
                                                .with_easing(pulsating_between(0.4, 0.8)),
                                            |label, delta| label.alpha(delta),
                                        )
                                        .into_any()
                                    } else {
                                        this.into_any_element()
                                    }
                                }),
                        )
                    }),
            )
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
        let relative_path: Arc<Path> = Arc::from(buffer.read(cx).file()?.path().parent()?);
        let worktree_root_path = project
            .read(cx)
            .worktree_for_id(worktree_id, cx)?
            .read(cx)
            .abs_path();
        let workspace_id = workspace.database_id()?;
        let weak = workspace.weak_handle();
        cx.spawn_in(window, async move |workspace, cx| {
            let as_str = relative_path.to_string_lossy().into_owned();
            let active_toolchain = workspace::WORKSPACE_DB
                .toolchain(workspace_id, worktree_id, as_str, language_name.clone())
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
        relative_path: Arc<Path>,
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
        let create_search_state = Arc::new(move |window: &mut Window, cx: &mut Context<Self>| {
            let toolchain_selector = cx.entity().downgrade();
            let picker = cx.new(|cx| {
                let delegate = ToolchainSelectorDelegate::new(
                    active_toolchain.clone(),
                    toolchain_selector,
                    workspace.clone(),
                    worktree_id,
                    worktree_root.clone(),
                    project.clone(),
                    relative_path.clone(),
                    language_name.clone(),
                    window,
                    cx,
                );
                Picker::uniform_list(delegate, window, cx)
            });
            SearchState { picker }
        });

        Self {
            state: State::Search(create_search_state(window, cx)),
            create_search_state,
            language: None,
        }
    }
}

impl Render for ToolchainSelector {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w(rems(34.))
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
    candidates: ToolchainList,
    matches: Vec<StringMatch>,
    selected_index: usize,
    workspace: WeakEntity<Workspace>,
    worktree_id: WorktreeId,
    worktree_abs_path_root: Arc<Path>,
    relative_path: Arc<Path>,
    placeholder_text: Arc<str>,
    add_toolchain_text: Arc<str>,
    project: Entity<Project>,
    language_name: LanguageName,
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
        relative_path: Arc<Path>,
        language_name: LanguageName,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Self {
        let language = language_name.clone();
        let _project = project.clone();

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

                let (available_toolchains, relative_path) = _project
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
                    let path = relative_path.to_string_lossy();
                    if path.is_empty() {
                        Cow::Borrowed("worktree root")
                    } else {
                        Cow::Owned(format!("`{}`", path))
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
                    this.delegate.candidates = available_toolchains;

                    if let Some(active_toolchain) = active_toolchain
                        && let Some(position) = this
                            .delegate
                            .candidates
                            .toolchains
                            .iter()
                            .position(|toolchain| *toolchain == active_toolchain)
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
            language_name: language,
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
            let toolchain = self.candidates.toolchains[string_match.candidate_id].clone();
            if let Some(workspace_id) = self
                .workspace
                .read_with(cx, |this, _| this.database_id())
                .ok()
                .flatten()
            {
                let workspace = self.workspace.clone();
                let worktree_id = self.worktree_id;
                let path = self.relative_path.clone();
                let relative_path = self.relative_path.to_string_lossy().into_owned();
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
                    .toolchains
                    .into_iter()
                    .enumerate()
                    .map(|(index, candidate)| {
                        let path = Self::relativize_path(candidate.path, &worktree_root_path);
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
                    .toolchains
                    .into_iter()
                    .enumerate()
                    .map(|(candidate_id, toolchain)| {
                        let path = Self::relativize_path(toolchain.path, &worktree_root_path);
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
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches[ix];
        let toolchain = &self.candidates.toolchains[mat.candidate_id];

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
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(HighlightedLabel::new(label, name_highlights))
                .child(
                    HighlightedLabel::new(path, path_highlights)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
    }
    fn render_footer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        Some(
            v_flex()
                // .bg(cx.theme().colors().background.clone())
                .rounded_b_md()
                .child(Divider::horizontal())
                .child(
                    h_flex().justify_end().p_1().child(
                        Button::new("xd", self.add_toolchain_text.clone())
                            .icon(IconName::Plus)
                            .style(ButtonStyle::Filled)
                            .icon_position(IconPosition::Start)
                            .on_click(cx.listener({
                                let project = self.project.clone();
                                let language = self.language_name.clone();
                                move |picker, _, window, cx| {
                                    maybe!({
                                        picker
                                            .delegate
                                            .toolchain_selector
                                            .update(cx, |this, cx| {
                                                this.state =
                                                    State::AddToolchain(AddToolchainState::new(
                                                        project.clone(),
                                                        language.clone(),
                                                        window,
                                                        cx,
                                                    ));
                                                this.state.focus_handle(cx).focus(window);

                                                cx.notify();
                                            })
                                            .ok()
                                    });
                                }
                            })),
                    ),
                )
                .into_any_element(),
        )
    }
}
