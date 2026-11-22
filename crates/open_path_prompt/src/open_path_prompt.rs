pub mod file_finder_settings;

#[cfg(test)]
mod open_path_prompt_tests;

use file_finder_settings::FileFinderSettings;
use file_icons::FileIcons;
use futures::channel::oneshot;
use fuzzy::{CharBag, StringMatch, StringMatchCandidate};
use gpui::{Action, Entity, HighlightStyle, StyledText, Subscription, Task};
use picker::{Picker, PickerDelegate};
use project::{DirectoryItem, DirectoryLister, Worktree};
use settings::SettingsStore;
use settings::{ProjectPanelSortMode, Settings};
use std::{
    path::{self, Path, PathBuf},
    sync::{
        Arc,
        atomic::{self, AtomicBool},
    },
};
use ui::{Context, KeyBinding, LabelLike, ListItem, Window};
use ui::{HighlightedLabel, ListItemSpacing, prelude::*};
use util::{
    maybe,
    paths::{PathStyle, compare_paths},
};
use workspace::{OpenPathPromptMode, OpenPathPromptResult, Workspace};

pub struct OpenPathPrompt;

pub struct OpenPathDelegate {
    on_confirm: Option<Box<dyn FnOnce(Option<OpenPathPromptResult>)>>,
    mode: OpenPathPromptMode,
    lister: DirectoryLister,
    selected_index: usize,
    directory_state: DirectoryState,
    string_matches: Vec<StringMatch>,
    cancel_flag: Arc<AtomicBool>,
    should_dismiss: bool,
    prompt_root: String,
    path_style: PathStyle,
    replace_prompt: Task<()>,
    render_footer:
        Arc<dyn Fn(&mut Window, &mut Context<Picker<Self>>) -> Option<AnyElement> + 'static>,
    hidden_entries: bool,
    sort_mode: ProjectPanelSortMode,
    relative_to: PathBuf,
    /// Watches a project root that `relative_to` was derived from before it was known whether
    /// the root is a directory or a single file.
    unresolved_root: Option<Subscription>,
}

impl OpenPathDelegate {
    pub fn new(
        tx: oneshot::Sender<Option<Vec<PathBuf>>>,
        lister: DirectoryLister,
        creating_path: bool,
        relative_to: PathBuf,
        cx: &App,
    ) -> Self {
        Self::with_confirmation(
            Box::new(move |result| {
                tx.send(result.map(|result| result.paths)).ok();
            }),
            lister,
            creating_path,
            OpenPathPromptMode::SelectPath,
            relative_to,
            cx,
        )
    }

    pub fn with_footer(
        mut self,
        footer: Arc<
            dyn Fn(&mut Window, &mut Context<Picker<Self>>) -> Option<AnyElement> + 'static,
        >,
    ) -> Self {
        self.render_footer = footer;
        self
    }

    pub fn show_hidden(mut self) -> Self {
        self.hidden_entries = true;
        self
    }
    fn get_entry(&self, selected_match_index: usize) -> Option<CandidateInfo> {
        match &self.directory_state {
            DirectoryState::List { entries, .. } => {
                let id = self.string_matches.get(selected_match_index)?.candidate_id;
                entries.iter().find(|entry| entry.path.id == id).cloned()
            }
            DirectoryState::Create {
                user_input,
                entries,
                ..
            } => {
                let mut i = selected_match_index;
                if let Some(user_input) = user_input
                    && (!user_input.exists || !user_input.is_dir)
                {
                    if i == 0 {
                        return Some(CandidateInfo {
                            path: user_input.file.clone(),
                            is_dir: false,
                        });
                    } else {
                        i -= 1;
                    }
                }
                let id = self.string_matches.get(i)?.candidate_id;
                entries.iter().find(|entry| entry.path.id == id).cloned()
            }
            DirectoryState::None { .. } => None,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn collect_match_candidates(&self) -> Vec<String> {
        match &self.directory_state {
            DirectoryState::List { entries, .. } => self
                .string_matches
                .iter()
                .filter_map(|string_match| {
                    entries
                        .iter()
                        .find(|entry| entry.path.id == string_match.candidate_id)
                        .map(|candidate| candidate.path.string.clone())
                })
                .collect(),
            DirectoryState::Create {
                user_input,
                entries,
                ..
            } => user_input
                .iter()
                .filter(|user_input| !user_input.exists || !user_input.is_dir)
                .map(|user_input| user_input.file.string.clone())
                .chain(self.string_matches.iter().filter_map(|string_match| {
                    entries
                        .iter()
                        .find(|entry| entry.path.id == string_match.candidate_id)
                        .map(|candidate| candidate.path.string.clone())
                }))
                .collect(),
            DirectoryState::None { .. } => Vec::new(),
        }
    }

    fn for_open(
        tx: oneshot::Sender<Option<OpenPathPromptResult>>,
        lister: DirectoryLister,
        mode: OpenPathPromptMode,
        relative_to: PathBuf,
        cx: &App,
    ) -> Self {
        let delegate = Self::with_confirmation(
            Box::new(move |result| {
                tx.send(result).ok();
            }),
            lister,
            false,
            mode,
            relative_to,
            cx,
        );
        if mode == OpenPathPromptMode::SelectPath {
            return delegate;
        }
        delegate.with_footer(Arc::new(move |_, cx| {
            let (primary_label, secondary_label) = if mode == OpenPathPromptMode::NewWindow {
                ("New Window", "This Window")
            } else {
                ("This Window", "New Window")
            };
            Some(
                h_flex()
                    .flex_1()
                    .p_1p5()
                    .gap_1()
                    .justify_end()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        Button::new("open_secondary", secondary_label)
                            .key_binding(KeyBinding::for_action(&menu::SecondaryConfirm, cx))
                            .on_click(|_, window, cx| {
                                window.dispatch_action(menu::SecondaryConfirm.boxed_clone(), cx)
                            }),
                    )
                    .child(
                        Button::new("open_primary", primary_label)
                            .key_binding(KeyBinding::for_action(&menu::Confirm, cx))
                            .on_click(|_, window, cx| {
                                window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                            }),
                    )
                    .into_any_element(),
            )
        }))
    }

    fn with_confirmation(
        on_confirm: Box<dyn FnOnce(Option<OpenPathPromptResult>)>,
        lister: DirectoryLister,
        creating_path: bool,
        mode: OpenPathPromptMode,
        relative_to: PathBuf,
        cx: &App,
    ) -> Self {
        let path_style = lister.path_style(cx);
        let sort_mode = cx
            .try_global::<SettingsStore>()
            .and_then(|store| store.merged_settings().project_panel.as_ref()?.sort_mode)
            .unwrap_or_default();
        Self {
            on_confirm: Some(on_confirm),
            mode,
            lister,
            selected_index: 0,
            directory_state: DirectoryState::None {
                create: creating_path,
            },
            string_matches: Vec::new(),
            cancel_flag: Arc::new(AtomicBool::new(false)),
            should_dismiss: true,
            prompt_root: match path_style {
                PathStyle::Unix => "/".to_string(),
                PathStyle::Windows => "C:\\".to_string(),
            },
            path_style,
            replace_prompt: Task::ready(()),
            render_footer: Arc::new(|_, _| None),
            hidden_entries: false,
            sort_mode,
            relative_to,
            unresolved_root: None,
        }
    }

    /// Resolves `./` and `../` against the directory of `root` once it is known whether `root`
    /// is a single file, refreshing the listing when that changes the base directory.
    fn track_unresolved_root(
        picker: &mut Picker<Self>,
        root: Entity<Worktree>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let subscription = cx.observe_in(&root, window, |picker, root, window, cx| {
            let delegate = &mut picker.delegate;
            let Some(directory) =
                OpenPathPrompt::root_directory(root.read(cx), delegate.path_style)
            else {
                return;
            };
            delegate.unresolved_root = None;
            if directory == delegate.relative_to {
                return;
            }
            delegate.relative_to = directory;
            // The listing is cached by the query, which no longer resolves to the same directory.
            let create = matches!(
                delegate.directory_state,
                DirectoryState::Create { .. } | DirectoryState::None { create: true }
            );
            delegate.directory_state = DirectoryState::None { create };
            picker.refresh(window, cx);
        });
        picker.delegate.unresolved_root = Some(subscription);
    }

    fn resolve_path(&self, path: &str) -> String {
        resolve_relative_path(path, &self.relative_to, self.path_style)
    }

    fn current_dir(&self) -> &'static str {
        match self.path_style {
            PathStyle::Unix => "./",
            PathStyle::Windows => ".\\",
        }
    }
}

/// Directory that `./` and `../` queries resolve against, see [`OpenPathPrompt::relative_to`].
struct RelativeBase {
    directory: PathBuf,
    /// The project root `directory` was derived from, when it was not known yet whether that
    /// root is a single file.
    unresolved_root: Option<Entity<Worktree>>,
}

impl RelativeBase {
    fn resolved(directory: PathBuf) -> Self {
        Self {
            directory,
            unresolved_root: None,
        }
    }
}

#[derive(Debug)]
enum DirectoryState {
    List {
        parent_path: String,
        entries: Vec<CandidateInfo>,
        error: Option<SharedString>,
    },
    Create {
        parent_path: String,
        user_input: Option<UserInput>,
        entries: Vec<CandidateInfo>,
    },
    None {
        create: bool,
    },
}

#[derive(Debug, Clone)]
struct UserInput {
    file: StringMatchCandidate,
    exists: bool,
    is_dir: bool,
}

#[derive(Debug, Clone)]
struct CandidateInfo {
    path: StringMatchCandidate,
    is_dir: bool,
}

impl OpenPathPrompt {
    pub fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.set_prompt_for_open_path(Box::new(|workspace, lister, mode, window, cx| {
            let (tx, rx) = oneshot::channel();
            let relative_to = Self::relative_to(workspace, &lister, false, cx);
            let delegate = OpenPathDelegate::for_open(tx, lister, mode, relative_to.directory, cx);
            Self::prompt_for_path(
                workspace,
                delegate,
                relative_to.unresolved_root,
                None,
                window,
                cx,
            );
            rx
        }));
    }

    /// Directory that `./` and `../` queries resolve against: the active file's directory
    /// when opening, otherwise the first project root (or its parent directory when the root
    /// is a single file, even one not saved yet), falling back to the process cwd, or to the
    /// remote home directory for a remote project, whose lister must never be asked for a client
    /// path.
    ///
    /// Workspace paths are only used when `lister` browses the project's own file system: a
    /// [`DirectoryLister::Local`] lister for a remote project lists the local machine, where the
    /// remote paths may be absent or point to unrelated files.
    fn relative_to(
        workspace: &Workspace,
        lister: &DirectoryLister,
        creating_path: bool,
        cx: &App,
    ) -> RelativeBase {
        let current_dir = || std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let project = workspace.project().read(cx);
        if matches!(lister, DirectoryLister::Local(..)) && !project.is_local() {
            return RelativeBase::resolved(current_dir());
        }
        let path_style = project.path_style(cx);
        let active_file_directory = (!creating_path)
            .then(|| workspace.active_item(cx))
            .flatten()
            .and_then(|item| item.project_path(cx))
            .and_then(|project_path| {
                workspace
                    .project()
                    .read(cx)
                    .absolute_path(&project_path, cx)
            })
            .and_then(|path| path_style.parent(&path).map(Path::to_path_buf));
        if let Some(directory) = active_file_directory {
            return RelativeBase::resolved(directory);
        }
        if let Some(root) = workspace.visible_worktrees(cx).next() {
            return match Self::root_directory(root.read(cx), path_style) {
                Some(directory) => RelativeBase::resolved(directory),
                // Assume a directory, as most roots are, until the root's entries arrive.
                None => RelativeBase {
                    directory: root.read(cx).abs_path().to_path_buf(),
                    unresolved_root: Some(root),
                },
            };
        }
        RelativeBase::resolved(if project.is_local() {
            current_dir()
        } else {
            PathBuf::from(REMOTE_HOME)
        })
    }

    /// Directory of a project root: the root itself, or its parent directory when the root is
    /// a single file, even one not saved yet. Returns `None` while that is not known yet.
    fn root_directory(worktree: &Worktree, path_style: PathStyle) -> Option<PathBuf> {
        let abs_path = worktree.abs_path();
        let is_file = match worktree.root_entry() {
            Some(entry) => entry.is_file(),
            // A local worktree gets its root entry on creation and a remote one with its first
            // complete update, so a root still missing after that is a file not saved yet.
            None if worktree.is_local() || worktree.completed_scan_id() > 0 => true,
            None => return None,
        };
        Some(if is_file {
            path_style
                .parent(&abs_path)
                .unwrap_or(&abs_path)
                .to_path_buf()
        } else {
            abs_path.to_path_buf()
        })
    }

    pub fn register_new_path(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.set_prompt_for_new_path(Box::new(
            |workspace, lister, suggested_name, window, cx| {
                let (tx, rx) = oneshot::channel();
                let relative_to = Self::relative_to(workspace, &lister, true, cx);
                let delegate = OpenPathDelegate::new(tx, lister, true, relative_to.directory, cx);
                Self::prompt_for_path(
                    workspace,
                    delegate,
                    relative_to.unresolved_root,
                    suggested_name,
                    window,
                    cx,
                );
                rx
            },
        ));
    }

    fn prompt_for_path(
        workspace: &mut Workspace,
        delegate: OpenPathDelegate,
        unresolved_root: Option<Entity<Worktree>>,
        suggested_name: Option<String>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        workspace.toggle_modal(window, cx, |window, cx| {
            let mut query = delegate.lister.default_query(cx);
            let mut picker = Picker::uniform_list(delegate.show_hidden(), window, cx);
            if let Some(root) = unresolved_root {
                OpenPathDelegate::track_unresolved_root(&mut picker, root, window, cx);
            }
            if let Some(suggested_name) = suggested_name {
                query.push_str(&suggested_name);
            }
            picker.set_query(&query, window, cx);
            picker
        });
    }
}

impl PickerDelegate for OpenPathDelegate {
    type ListItem = ui::ListItem;

    fn name() -> &'static str {
        "open path prompt"
    }

    fn match_count(&self) -> usize {
        let user_input = if let DirectoryState::Create { user_input, .. } = &self.directory_state {
            user_input
                .as_ref()
                .filter(|input| !input.exists || !input.is_dir)
                .into_iter()
                .count()
        } else {
            0
        };
        self.string_matches.len() + user_input
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = ix;
        cx.notify();
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let lister = &self.lister;
        let input_is_empty = query.is_empty();
        let (dir, suffix) = get_dir_and_suffix(query, self.path_style);
        let resolved_dir = self.resolve_path(&dir);
        let query = match &self.directory_state {
            DirectoryState::List { parent_path, .. } => {
                if parent_path == &dir {
                    None
                } else {
                    Some(lister.list_directory(resolved_dir, cx))
                }
            }
            DirectoryState::Create {
                parent_path,
                user_input,
                ..
            } => {
                if parent_path == &dir
                    && user_input.as_ref().map(|input| &input.file.string) == Some(&suffix)
                {
                    None
                } else {
                    Some(lister.list_directory(resolved_dir, cx))
                }
            }
            DirectoryState::None { .. } => Some(lister.list_directory(resolved_dir, cx)),
        };
        self.cancel_flag.store(true, atomic::Ordering::Release);
        self.cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_flag = self.cancel_flag.clone();
        let hidden_entries = self.hidden_entries;
        let parent_path_is_root = self.prompt_root == dir;
        let current_dir = self.current_dir();
        let sort_mode = self.sort_mode;
        cx.spawn_in(window, async move |this, cx| {
            if let Some(query) = query {
                let paths = query.await;
                if cancel_flag.load(atomic::Ordering::Acquire) {
                    return;
                }

                if this
                    .update(cx, |this, _| {
                        let new_state = match &this.delegate.directory_state {
                            DirectoryState::None { create: false }
                            | DirectoryState::List { .. } => match paths {
                                Ok(paths) => DirectoryState::List {
                                    entries: path_candidates(parent_path_is_root, paths, sort_mode),
                                    parent_path: dir.clone(),
                                    error: None,
                                },
                                Err(e) => DirectoryState::List {
                                    entries: Vec::new(),
                                    parent_path: dir.clone(),
                                    error: Some(SharedString::from(e.to_string())),
                                },
                            },
                            DirectoryState::None { create: true }
                            | DirectoryState::Create { .. } => match paths {
                                Ok(paths) => {
                                    let mut entries =
                                        path_candidates(parent_path_is_root, paths, sort_mode);
                                    let mut exists = false;
                                    let mut is_dir = false;
                                    let mut new_id = None;
                                    entries.retain(|entry| {
                                        new_id = new_id.max(Some(entry.path.id));
                                        if entry.path.string == suffix {
                                            exists = true;
                                            is_dir = entry.is_dir;
                                        }
                                        !exists || is_dir
                                    });

                                    let new_id = new_id.map(|id| id + 1).unwrap_or(0);
                                    let user_input = if suffix.is_empty() {
                                        None
                                    } else {
                                        Some(UserInput {
                                            file: StringMatchCandidate::new(new_id, &suffix),
                                            exists,
                                            is_dir,
                                        })
                                    };
                                    DirectoryState::Create {
                                        entries,
                                        parent_path: dir.clone(),
                                        user_input,
                                    }
                                }
                                Err(_) => DirectoryState::Create {
                                    entries: Vec::new(),
                                    parent_path: dir.clone(),
                                    user_input: Some(UserInput {
                                        exists: false,
                                        is_dir: false,
                                        file: StringMatchCandidate::new(0, &suffix),
                                    }),
                                },
                            },
                        };
                        this.delegate.directory_state = new_state;
                    })
                    .is_err()
                {
                    return;
                }
            }

            let Ok(mut new_entries) =
                this.update(cx, |this, _| match &this.delegate.directory_state {
                    DirectoryState::List {
                        entries,
                        error: None,
                        ..
                    }
                    | DirectoryState::Create { entries, .. } => entries.clone(),
                    DirectoryState::List { error: Some(_), .. } | DirectoryState::None { .. } => {
                        Vec::new()
                    }
                })
            else {
                return;
            };

            if !hidden_entries {
                new_entries.retain(|entry| !entry.path.string.starts_with('.'));
            }

            let max_id = new_entries
                .iter()
                .map(|entry| entry.path.id)
                .max()
                .unwrap_or(0);

            if suffix.is_empty() {
                let should_prepend_with_current_dir = this
                    .read_with(cx, |picker, _| {
                        !input_is_empty
                            && match &picker.delegate.directory_state {
                                DirectoryState::List { error, .. } => error.is_none(),
                                DirectoryState::Create { .. } => false,
                                DirectoryState::None { .. } => false,
                            }
                    })
                    .unwrap_or(false);

                let current_dir_in_new_entries = new_entries
                    .iter()
                    .any(|entry| &entry.path.string == current_dir);

                if should_prepend_with_current_dir && !current_dir_in_new_entries {
                    new_entries.insert(
                        0,
                        CandidateInfo {
                            path: StringMatchCandidate {
                                id: max_id + 1,
                                string: current_dir.to_string(),
                                char_bag: CharBag::from(current_dir),
                            },
                            is_dir: true,
                        },
                    );
                }

                this.update(cx, |this, cx| {
                    this.delegate.selected_index = 0;
                    this.delegate.string_matches = new_entries
                        .iter()
                        .map(|m| StringMatch {
                            candidate_id: m.path.id,
                            score: 0.0,
                            positions: Vec::new(),
                            string: m.path.string.clone(),
                        })
                        .collect();
                    this.delegate.directory_state =
                        match &this.delegate.directory_state {
                            DirectoryState::None { create: false }
                            | DirectoryState::List { .. } => DirectoryState::List {
                                parent_path: dir.clone(),
                                entries: new_entries,
                                error: None,
                            },
                            DirectoryState::None { create: true }
                            | DirectoryState::Create { .. } => DirectoryState::Create {
                                parent_path: dir.clone(),
                                user_input: None,
                                entries: new_entries,
                            },
                        };
                    cx.notify();
                })
                .ok();
                return;
            }

            let Ok(is_create_state) =
                this.update(cx, |this, _| match &this.delegate.directory_state {
                    DirectoryState::Create { .. } => true,
                    DirectoryState::List { .. } => false,
                    DirectoryState::None { create } => *create,
                })
            else {
                return;
            };

            let candidates = new_entries
                .iter()
                .filter_map(|entry| {
                    if is_create_state && !entry.is_dir && Some(&suffix) == Some(&entry.path.string)
                    {
                        None
                    } else if !suffix.is_empty() && entry.path.string == current_dir {
                        None
                    } else {
                        Some(&entry.path)
                    }
                })
                .collect::<Vec<_>>();

            let matches = fuzzy::match_strings(
                candidates.as_slice(),
                &suffix,
                false,
                true,
                100,
                &cancel_flag,
                cx.background_executor().clone(),
            )
            .await;
            if cancel_flag.load(atomic::Ordering::Acquire) {
                return;
            }

            this.update(cx, |this, cx| {
                this.delegate.selected_index = 0;
                this.delegate.string_matches = matches.clone();
                this.delegate.string_matches.sort_by_key(|m| {
                    (
                        new_entries
                            .iter()
                            .find(|entry| entry.path.id == m.candidate_id)
                            .map(|entry| &entry.path)
                            .map(|candidate| !candidate.string.starts_with(&suffix)),
                        m.candidate_id,
                    )
                });
                this.delegate.directory_state = match &this.delegate.directory_state {
                    DirectoryState::None { create: false } | DirectoryState::List { .. } => {
                        DirectoryState::List {
                            entries: new_entries,
                            parent_path: dir.clone(),
                            error: None,
                        }
                    }
                    DirectoryState::None { create: true } => DirectoryState::Create {
                        entries: new_entries,
                        parent_path: dir.clone(),
                        user_input: Some(UserInput {
                            file: StringMatchCandidate::new(0, &suffix),
                            exists: false,
                            is_dir: false,
                        }),
                    },
                    DirectoryState::Create { user_input, .. } => {
                        let (new_id, exists, is_dir) = user_input
                            .as_ref()
                            .map(|input| (input.file.id, input.exists, input.is_dir))
                            .unwrap_or_else(|| (0, false, false));
                        DirectoryState::Create {
                            entries: new_entries,
                            parent_path: dir.clone(),
                            user_input: Some(UserInput {
                                file: StringMatchCandidate::new(new_id, &suffix),
                                exists,
                                is_dir,
                            }),
                        }
                    }
                };

                cx.notify();
            })
            .ok();
        })
    }
    fn select_child(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<String> {
        let candidate = self.get_entry(self.selected_index)?;
        if candidate.path.string.is_empty() || !candidate.is_dir {
            return None;
        }
        let path_style = self.path_style;
        match &self.directory_state {
            DirectoryState::List { parent_path, .. }
            | DirectoryState::Create { parent_path, .. } => Some(format!(
                "{}{}{}",
                parent_path,
                candidate.path.string,
                path_style.primary_separator()
            )),
            DirectoryState::None { .. } => None,
        }
    }

    fn select_parent(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<String> {
        let parent_path = match &self.directory_state {
            DirectoryState::List { parent_path, .. }
            | DirectoryState::Create { parent_path, .. } => parent_path,
            DirectoryState::None { .. } => return None,
        };
        if parent_path == &self.prompt_root {
            return None;
        }
        Some(parent_directory_query(
            parent_path,
            &self.relative_to,
            self.path_style,
        ))
    }
    fn confirm_completion(
        &mut self,
        query: String,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<String> {
        let candidate = self.get_entry(self.selected_index)?;
        if candidate.path.string.is_empty() || candidate.path.string == self.current_dir() {
            return None;
        }

        let path_style = self.path_style;
        Some(
            maybe!({
                match &self.directory_state {
                    DirectoryState::Create { parent_path, .. } => Some(format!(
                        "{}{}{}",
                        parent_path,
                        candidate.path.string,
                        if candidate.is_dir {
                            path_style.primary_separator()
                        } else {
                            ""
                        }
                    )),
                    DirectoryState::List { parent_path, .. } => Some(format!(
                        "{}{}{}",
                        parent_path,
                        candidate.path.string,
                        if candidate.is_dir {
                            path_style.primary_separator()
                        } else {
                            ""
                        }
                    )),
                    DirectoryState::None { .. } => return None,
                }
            })
            .unwrap_or(query),
        )
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(candidate) = self.get_entry(self.selected_index) else {
            return;
        };
        let open_in_new_window = match self.mode {
            OpenPathPromptMode::SelectPath => false,
            OpenPathPromptMode::CurrentWindow => secondary,
            OpenPathPromptMode::NewWindow => !secondary,
        };

        match &self.directory_state {
            DirectoryState::None { .. } => return,
            DirectoryState::List { parent_path, .. } => {
                let resolved_parent = self.resolve_path(parent_path);
                let confirmed_path =
                    if parent_path == &self.prompt_root && candidate.path.string.is_empty() {
                        PathBuf::from(&self.prompt_root)
                    } else {
                        self.path_style.normalize_separators(self.path_style.join(
                            Path::new(self.lister.resolve_tilde(&resolved_parent, cx).as_ref()),
                            &candidate.path.string,
                        ))
                    };
                if let Some(on_confirm) = self.on_confirm.take() {
                    on_confirm(Some(OpenPathPromptResult {
                        paths: vec![confirmed_path],
                        open_in_new_window,
                    }));
                }
            }
            DirectoryState::Create {
                parent_path,
                user_input,
                ..
            } => match user_input {
                None => return,
                Some(user_input) => {
                    if user_input.is_dir {
                        return;
                    }
                    let resolved_parent = self.resolve_path(parent_path);
                    let prompted_path =
                        if parent_path == &self.prompt_root && user_input.file.string.is_empty() {
                            PathBuf::from(&self.prompt_root)
                        } else {
                            let parent_path = self.lister.resolve_tilde(&resolved_parent, cx);
                            PathBuf::from(format!("{parent_path}{}", user_input.file.string))
                        };
                    if user_input.exists {
                        self.should_dismiss = false;
                        let answer = window.prompt(
                            gpui::PromptLevel::Critical,
                            &format!("{prompted_path:?} already exists. Do you want to replace it?"),
                            Some(
                                "A file or folder with the same name already exists. Replacing it will overwrite its current contents.",
                            ),
                            &["Replace", "Cancel"],
                            cx
                        );
                        self.replace_prompt = cx.spawn_in(window, async move |picker, cx| {
                            let answer = answer.await.ok();
                            picker
                                .update(cx, |picker, cx| {
                                    picker.delegate.should_dismiss = true;
                                    if answer != Some(0) {
                                        return;
                                    }
                                    if let Some(on_confirm) = picker.delegate.on_confirm.take() {
                                        on_confirm(Some(OpenPathPromptResult {
                                            paths: vec![prompted_path],
                                            open_in_new_window,
                                        }));
                                    }
                                    cx.emit(gpui::DismissEvent);
                                })
                                .ok();
                        });
                        return;
                    } else if let Some(on_confirm) = self.on_confirm.take() {
                        on_confirm(Some(OpenPathPromptResult {
                            paths: vec![prompted_path],
                            open_in_new_window,
                        }));
                    }
                }
            },
        }

        cx.emit(gpui::DismissEvent);
    }

    fn should_dismiss(&self) -> bool {
        self.should_dismiss
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.cancel_flag.store(true, atomic::Ordering::Release);
        if let Some(on_confirm) = self.on_confirm.take() {
            on_confirm(None);
        }
        cx.emit(gpui::DismissEvent)
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let settings = FileFinderSettings::get_global(cx);
        let candidate = self.get_entry(ix)?;
        let string_match = match &self.directory_state {
            DirectoryState::List { .. } => self.string_matches.get(ix),
            DirectoryState::Create { user_input, .. } => {
                if let Some(user_input) = user_input {
                    if !user_input.exists || !user_input.is_dir {
                        if ix == 0 {
                            None
                        } else {
                            self.string_matches.get(ix - 1)
                        }
                    } else {
                        self.string_matches.get(ix)
                    }
                } else {
                    self.string_matches.get(ix)
                }
            }
            DirectoryState::None { .. } => None,
        };

        // Directory entries and string matches can briefly go out of sync during
        // async updates. When that happens, render the row without highlights.
        let mut match_positions = string_match
            .filter(|string_match| {
                string_match.candidate_id == candidate.path.id
                    && string_match.string == candidate.path.string
            })
            .map(|string_match| string_match.positions.clone())
            .unwrap_or_default();

        let is_current_dir_candidate = candidate.path.string == self.current_dir();

        let file_icon = maybe!({
            if !settings.file_icons {
                return None;
            }

            let path = path::Path::new(&candidate.path.string);
            let icon = if candidate.is_dir {
                if is_current_dir_candidate {
                    return Some(Icon::new(IconName::ReplyArrowRight).color(Color::Muted));
                } else {
                    FileIcons::get_folder_icon(false, path, cx)?
                }
            } else {
                FileIcons::get_icon(path, cx)?
            };
            Some(Icon::from_path(icon).color(Color::Muted))
        });

        match &self.directory_state {
            DirectoryState::List { parent_path, .. } => {
                let (label, indices) = if is_current_dir_candidate {
                    ("open this directory".to_string(), vec![])
                } else if *parent_path == self.prompt_root {
                    match_positions.iter_mut().for_each(|position| {
                        *position += self.prompt_root.len();
                    });
                    (
                        format!("{}{}", self.prompt_root, candidate.path.string),
                        match_positions,
                    )
                } else {
                    (candidate.path.string, match_positions)
                };
                Some(
                    ListItem::new(ix)
                        .spacing(ListItemSpacing::Sparse)
                        .start_slot::<Icon>(file_icon)
                        .inset(true)
                        .toggle_state(selected)
                        .child(HighlightedLabel::new(label, indices)),
                )
            }
            DirectoryState::Create {
                parent_path,
                user_input,
                ..
            } => {
                let (label, delta) = if *parent_path == self.prompt_root {
                    match_positions.iter_mut().for_each(|position| {
                        *position += self.prompt_root.len();
                    });
                    (
                        format!("{}{}", self.prompt_root, candidate.path.string),
                        self.prompt_root.len(),
                    )
                } else {
                    (candidate.path.string.clone(), 0)
                };

                let label_with_highlights = match user_input {
                    Some(user_input) => {
                        let label_len = label.len();
                        if user_input.file.string == candidate.path.string {
                            if user_input.exists {
                                let label = if user_input.is_dir {
                                    label
                                } else {
                                    format!("{label} (replace)")
                                };
                                StyledText::new(label)
                                    .with_default_highlights(
                                        &window.text_style(),
                                        vec![(
                                            delta..label_len,
                                            HighlightStyle::color(Color::Conflict.color(cx)),
                                        )],
                                    )
                                    .into_any_element()
                            } else {
                                StyledText::new(format!("{label} (create)"))
                                    .with_default_highlights(
                                        &window.text_style(),
                                        vec![(
                                            delta..label_len,
                                            HighlightStyle::color(Color::Created.color(cx)),
                                        )],
                                    )
                                    .into_any_element()
                            }
                        } else {
                            HighlightedLabel::new(label, match_positions).into_any_element()
                        }
                    }
                    None => HighlightedLabel::new(label, match_positions).into_any_element(),
                };

                Some(
                    ListItem::new(ix)
                        .spacing(ListItemSpacing::Sparse)
                        .start_slot::<Icon>(file_icon)
                        .inset(true)
                        .toggle_state(selected)
                        .child(LabelLike::new().child(label_with_highlights)),
                )
            }
            DirectoryState::None { .. } => None,
        }
    }

    fn render_footer(
        &self,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        (self.render_footer)(window, cx)
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some(match &self.directory_state {
            DirectoryState::Create { .. } => SharedString::from("Type a path…"),
            DirectoryState::List {
                error: Some(error), ..
            } => error.clone(),
            DirectoryState::List { .. } | DirectoryState::None { .. } => {
                SharedString::from("No such file or directory")
            }
        })
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        Arc::from(
            format!(
                "[directory{}]filename.ext",
                self.path_style.primary_separator()
            )
            .as_str(),
        )
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        let Some(m) = self.string_matches.first() else {
            return Vec::new();
        };
        if m.string == self.current_dir() {
            vec![0]
        } else {
            Vec::new()
        }
    }
}

fn path_candidates(
    parent_path_is_root: bool,
    mut children: Vec<DirectoryItem>,
    sort_mode: ProjectPanelSortMode,
) -> Vec<CandidateInfo> {
    if parent_path_is_root {
        children.push(DirectoryItem {
            is_dir: true,
            path: PathBuf::default(),
        });
    }

    children.sort_by(|a, b| {
        let (a_is_file, b_is_file) = match sort_mode {
            ProjectPanelSortMode::DirectoriesFirst => (!a.is_dir, !b.is_dir),
            ProjectPanelSortMode::FilesFirst => (a.is_dir, b.is_dir),
            ProjectPanelSortMode::Mixed => (true, true),
        };
        compare_paths((&a.path, a_is_file), (&b.path, b_is_file))
    });
    children
        .iter()
        .enumerate()
        .map(|(ix, item)| CandidateInfo {
            path: StringMatchCandidate::new(ix, &item.path.to_string_lossy()),
            is_dir: item.is_dir,
        })
        .collect()
}

/// Resolves `./`, `../`, `.` and `..` queries against `relative_to`, using `path_style`
/// so that remote paths are interpreted according to the remote OS rather than the local one.
fn resolve_relative_path(path: &str, relative_to: &Path, path_style: PathStyle) -> String {
    let parent = || match path_style.parent(relative_to) {
        // A single-component base like `~` has no lexical parent, so let the host resolve `..`.
        Some(parent) if parent.as_os_str().is_empty() => path_style.join(relative_to, ".."),
        Some(parent) => parent.to_path_buf(),
        // There is no parent above a root.
        None => relative_to.to_path_buf(),
    };
    // Only the path style's own separators delimit `.` and `..`: on Unix, `.\nested` is a
    // valid file name rather than a relative path.
    let strip_dots = |dots: &str| {
        path.strip_prefix(dots)
            .and_then(|rest| rest.strip_prefix(path_style.separators_ch()))
    };
    // Repeated separators (`.//nested`) must not turn the operand into an absolute path,
    // which `join` would use in place of the base.
    let join = |base: &Path, rest: &str| {
        let mut joined = path_style
            .join(base, rest.trim_start_matches(path_style.separators_ch()))
            .to_string_lossy()
            .into_owned();
        // Joining onto a verbatim (`\\?\`) base drops the trailing separator, which callers
        // rely on to tell a directory query from a directory and file name pair.
        if rest.ends_with(path_style.separators_ch())
            && !joined.ends_with(path_style.separators_ch())
        {
            joined.push_str(path_style.primary_separator());
        }
        joined
    };
    if let Some(rest) = strip_dots(".") {
        join(relative_to, rest)
    } else if let Some(rest) = strip_dots("..") {
        join(&parent(), rest)
    } else if path == "." {
        relative_to.to_string_lossy().into_owned()
    } else if path == ".." {
        parent().to_string_lossy().into_owned()
    } else {
        path.to_string()
    }
}

/// Base for `./` and `../` in a remote project with no root or active file to resolve against.
/// The remote server expands `~` to its home directory, so this never names a client path.
const REMOTE_HOME: &str = "~";

/// Query for the directory above `parent_path`, resolving `./` and `../` first so that a
/// relative query moves up from the directory it points at.
fn parent_directory_query(parent_path: &str, relative_to: &Path, path_style: PathStyle) -> String {
    let resolved = resolve_relative_path(parent_path, relative_to, path_style);
    let trimmed = resolved.trim_end_matches(['/', '\\']);
    let (dir, _) = get_dir_and_suffix(trimmed.to_string(), path_style);
    dir
}

fn get_dir_and_suffix(query: String, path_style: PathStyle) -> (String, String) {
    match path_style {
        PathStyle::Unix => {
            let (mut dir, suffix) = if let Some(index) = query.rfind('/') {
                (query[..index].to_string(), query[index + 1..].to_string())
            } else {
                (query, String::new())
            };
            if !dir.ends_with('/') {
                dir.push('/');
            }
            (dir, suffix)
        }
        PathStyle::Windows => {
            let last_sep = query.rfind('\\').into_iter().chain(query.rfind('/')).max();
            let (mut dir, suffix) = if let Some(index) = last_sep {
                (
                    query[..index + 1].to_string(),
                    query[index + 1..].to_string(),
                )
            } else {
                (query, String::new())
            };
            let is_relative = dir == "./" || dir == ".\\";
            if dir.len() < 3 && !is_relative {
                dir = "C:\\".to_string();
            }
            (dir, suffix)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use util::paths::PathStyle;

    use super::{REMOTE_HOME, get_dir_and_suffix, parent_directory_query, resolve_relative_path};

    #[test]
    fn test_resolve_relative_path_with_windows_style() {
        let relative_to = Path::new("C:\\root\\subdir");
        let style = PathStyle::Windows;

        assert_eq!(
            resolve_relative_path("./", relative_to, style),
            "C:\\root\\subdir\\"
        );
        assert_eq!(
            resolve_relative_path(".\\foo", relative_to, style),
            "C:\\root\\subdir\\foo"
        );
        assert_eq!(
            resolve_relative_path("../", relative_to, style),
            "C:\\root\\"
        );
        assert_eq!(
            resolve_relative_path("..\\foo", relative_to, style),
            "C:\\root\\foo"
        );
        assert_eq!(
            resolve_relative_path(".", relative_to, style),
            "C:\\root\\subdir"
        );
        assert_eq!(resolve_relative_path("..", relative_to, style), "C:\\root");
        assert_eq!(
            resolve_relative_path("C:\\other", relative_to, style),
            "C:\\other"
        );
    }

    #[test]
    fn test_resolve_relative_path_with_unix_style() {
        let relative_to = Path::new("/root/subdir");
        let style = PathStyle::Unix;

        assert_eq!(
            resolve_relative_path("./", relative_to, style),
            "/root/subdir/"
        );
        assert_eq!(
            resolve_relative_path("./foo", relative_to, style),
            "/root/subdir/foo"
        );
        assert_eq!(resolve_relative_path("../", relative_to, style), "/root/");
        assert_eq!(
            resolve_relative_path("../foo", relative_to, style),
            "/root/foo"
        );
        assert_eq!(
            resolve_relative_path(".", relative_to, style),
            "/root/subdir"
        );
        assert_eq!(resolve_relative_path("..", relative_to, style), "/root");
        assert_eq!(resolve_relative_path("..", Path::new("/"), style), "/");
    }

    #[test]
    fn test_resolve_relative_path_with_repeated_separators() {
        let unix = PathStyle::Unix;
        let relative_to = Path::new("/root/subdir");
        assert_eq!(
            resolve_relative_path(".//nested/", relative_to, unix),
            "/root/subdir/nested/"
        );
        assert_eq!(
            resolve_relative_path("..//nested/", relative_to, unix),
            "/root/nested/"
        );
        assert_eq!(
            resolve_relative_path(".//", relative_to, unix),
            "/root/subdir/"
        );

        let windows = PathStyle::Windows;
        let relative_to = Path::new("C:\\root\\subdir");
        assert_eq!(
            resolve_relative_path(".\\\\nested\\", relative_to, windows),
            "C:\\root\\subdir\\nested\\"
        );
        assert_eq!(
            resolve_relative_path("..\\/nested\\", relative_to, windows),
            "C:\\root\\nested\\"
        );
    }

    #[test]
    fn test_resolve_relative_path_ignores_backslash_prefixes_with_unix_style() {
        let unix = PathStyle::Unix;
        let relative_to = Path::new("/tmp/repro");
        // `\` is an ordinary file name character on Unix, so these name entries literally.
        for query in [".\\nested/", "..\\nested/", ".\\", "..\\"] {
            assert_eq!(resolve_relative_path(query, relative_to, unix), query);
        }
    }

    #[test]
    fn test_resolve_relative_path_against_remote_home() {
        let unix = PathStyle::Unix;
        let home = Path::new(REMOTE_HOME);
        assert_eq!(resolve_relative_path("./", home, unix), "~/");
        assert_eq!(resolve_relative_path("./sub/", home, unix), "~/sub/");
        assert_eq!(resolve_relative_path("../", home, unix), "~/../");
        assert_eq!(resolve_relative_path("../sub/", home, unix), "~/../sub/");
        assert_eq!(resolve_relative_path(".", home, unix), "~");
        assert_eq!(resolve_relative_path("..", home, unix), "~/..");

        let windows = PathStyle::Windows;
        assert_eq!(resolve_relative_path(".\\sub\\", home, windows), "~\\sub\\");
        assert_eq!(resolve_relative_path("..\\", home, windows), "~\\..\\");
    }

    #[test]
    fn test_resolve_relative_path_keeps_trailing_separator_for_verbatim_base() {
        let windows = PathStyle::Windows;
        assert_eq!(
            resolve_relative_path(".\\nested\\", Path::new("\\\\?\\C:\\root"), windows),
            "\\\\?\\C:\\root\\nested\\"
        );
        assert_eq!(
            resolve_relative_path(
                "..\\nested\\",
                Path::new("\\\\?\\C:\\root\\subdir"),
                windows
            ),
            "\\\\?\\C:\\root\\nested\\"
        );
    }

    #[test]
    fn test_parent_directory_query_resolves_relative_queries() {
        let unix = PathStyle::Unix;
        let relative_to = Path::new("/root/subdir");
        assert_eq!(parent_directory_query("./", relative_to, unix), "/root/");
        assert_eq!(parent_directory_query("../", relative_to, unix), "/");
        assert_eq!(
            parent_directory_query("./nested/", relative_to, unix),
            "/root/subdir/"
        );
        assert_eq!(parent_directory_query("/a/b/", relative_to, unix), "/a/");
        assert_eq!(parent_directory_query("/", relative_to, unix), "/");

        let windows = PathStyle::Windows;
        let relative_to = Path::new("D:\\root\\subdir");
        assert_eq!(
            parent_directory_query(".\\", relative_to, windows),
            "D:\\root\\"
        );
        assert_eq!(parent_directory_query("..\\", relative_to, windows), "D:\\");
        assert_eq!(
            parent_directory_query("C:\\a\\b\\", relative_to, windows),
            "C:\\a\\"
        );
    }

    #[test]
    fn test_get_dir_and_suffix_with_windows_style_relative_prefix() {
        let (dir, suffix) = get_dir_and_suffix("./".into(), PathStyle::Windows);
        assert_eq!(dir, "./");
        assert_eq!(suffix, "");

        let (dir, suffix) = get_dir_and_suffix(".\\".into(), PathStyle::Windows);
        assert_eq!(dir, ".\\");
        assert_eq!(suffix, "");

        let (dir, suffix) = get_dir_and_suffix("./foo".into(), PathStyle::Windows);
        assert_eq!(dir, "./");
        assert_eq!(suffix, "foo");

        let (dir, suffix) = get_dir_and_suffix("../".into(), PathStyle::Windows);
        assert_eq!(dir, "../");
        assert_eq!(suffix, "");
    }

    #[test]
    fn test_get_dir_and_suffix_with_windows_style() {
        let (dir, suffix) = get_dir_and_suffix("".into(), PathStyle::Windows);
        assert_eq!(dir, "C:\\");
        assert_eq!(suffix, "");

        let (dir, suffix) = get_dir_and_suffix("C:".into(), PathStyle::Windows);
        assert_eq!(dir, "C:\\");
        assert_eq!(suffix, "");

        let (dir, suffix) = get_dir_and_suffix("C:\\".into(), PathStyle::Windows);
        assert_eq!(dir, "C:\\");
        assert_eq!(suffix, "");

        let (dir, suffix) = get_dir_and_suffix("C:\\Use".into(), PathStyle::Windows);
        assert_eq!(dir, "C:\\");
        assert_eq!(suffix, "Use");

        let (dir, suffix) =
            get_dir_and_suffix("C:\\Users\\Junkui\\Docum".into(), PathStyle::Windows);
        assert_eq!(dir, "C:\\Users\\Junkui\\");
        assert_eq!(suffix, "Docum");

        let (dir, suffix) =
            get_dir_and_suffix("C:\\Users\\Junkui\\Documents".into(), PathStyle::Windows);
        assert_eq!(dir, "C:\\Users\\Junkui\\");
        assert_eq!(suffix, "Documents");

        let (dir, suffix) =
            get_dir_and_suffix("C:\\Users\\Junkui\\Documents\\".into(), PathStyle::Windows);
        assert_eq!(dir, "C:\\Users\\Junkui\\Documents\\");
        assert_eq!(suffix, "");

        let (dir, suffix) = get_dir_and_suffix("C:\\root\\.".into(), PathStyle::Windows);
        assert_eq!(dir, "C:\\root\\");
        assert_eq!(suffix, ".");

        let (dir, suffix) = get_dir_and_suffix("C:\\root\\..".into(), PathStyle::Windows);
        assert_eq!(dir, "C:\\root\\");
        assert_eq!(suffix, "..");

        let (dir, suffix) = get_dir_and_suffix("C:\\root\\.hidden".into(), PathStyle::Windows);
        assert_eq!(dir, "C:\\root\\");
        assert_eq!(suffix, ".hidden");

        let (dir, suffix) = get_dir_and_suffix("C:/root/".into(), PathStyle::Windows);
        assert_eq!(dir, "C:/root/");
        assert_eq!(suffix, "");

        let (dir, suffix) = get_dir_and_suffix("C:/root/Use".into(), PathStyle::Windows);
        assert_eq!(dir, "C:/root/");
        assert_eq!(suffix, "Use");

        let (dir, suffix) = get_dir_and_suffix("C:\\root/Use".into(), PathStyle::Windows);
        assert_eq!(dir, "C:\\root/");
        assert_eq!(suffix, "Use");

        let (dir, suffix) = get_dir_and_suffix("C:/root\\.hidden".into(), PathStyle::Windows);
        assert_eq!(dir, "C:/root\\");
        assert_eq!(suffix, ".hidden");
    }

    #[test]
    fn test_get_dir_and_suffix_with_posix_style() {
        let (dir, suffix) = get_dir_and_suffix("".into(), PathStyle::Unix);
        assert_eq!(dir, "/");
        assert_eq!(suffix, "");

        let (dir, suffix) = get_dir_and_suffix("/".into(), PathStyle::Unix);
        assert_eq!(dir, "/");
        assert_eq!(suffix, "");

        let (dir, suffix) = get_dir_and_suffix("/Use".into(), PathStyle::Unix);
        assert_eq!(dir, "/");
        assert_eq!(suffix, "Use");

        let (dir, suffix) = get_dir_and_suffix("/Users/Junkui/Docum".into(), PathStyle::Unix);
        assert_eq!(dir, "/Users/Junkui/");
        assert_eq!(suffix, "Docum");

        let (dir, suffix) = get_dir_and_suffix("/Users/Junkui/Documents".into(), PathStyle::Unix);
        assert_eq!(dir, "/Users/Junkui/");
        assert_eq!(suffix, "Documents");

        let (dir, suffix) = get_dir_and_suffix("/Users/Junkui/Documents/".into(), PathStyle::Unix);
        assert_eq!(dir, "/Users/Junkui/Documents/");
        assert_eq!(suffix, "");

        let (dir, suffix) = get_dir_and_suffix("/root/.".into(), PathStyle::Unix);
        assert_eq!(dir, "/root/");
        assert_eq!(suffix, ".");

        let (dir, suffix) = get_dir_and_suffix("/root/..".into(), PathStyle::Unix);
        assert_eq!(dir, "/root/");
        assert_eq!(suffix, "..");

        let (dir, suffix) = get_dir_and_suffix("/root/.hidden".into(), PathStyle::Unix);
        assert_eq!(dir, "/root/");
        assert_eq!(suffix, ".hidden");
    }
}
