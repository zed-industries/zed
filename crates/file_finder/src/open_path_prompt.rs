use crate::file_finder_settings::FileFinderSettings;
use file_icons::FileIcons;
use futures::channel::oneshot;
use fuzzy::{CharBag, StringMatch, StringMatchCandidate};
use gpui::{HighlightStyle, StyledText, Task};
use picker::{Picker, PickerDelegate};
use project::{DirectoryItem, DirectoryLister};
use settings::Settings;
use std::{
    path::{self, Path, PathBuf},
    sync::{
        Arc,
        atomic::{self, AtomicBool},
    },
};
use ui::{Context, LabelLike, ListItem, Window};
use ui::{HighlightedLabel, ListItemSpacing, prelude::*};
use util::{
    maybe,
    paths::{PathStyle, compare_paths},
};
use workspace::Workspace;

pub(crate) struct OpenPathPrompt;

pub struct OpenPathDelegate {
    tx: Option<oneshot::Sender<Option<Vec<PathBuf>>>>,
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
}

impl OpenPathDelegate {
    pub fn new(
        tx: oneshot::Sender<Option<Vec<PathBuf>>>,
        lister: DirectoryLister,
        creating_path: bool,
        path_style: PathStyle,
    ) -> Self {
        Self {
            tx: Some(tx),
            lister,
            selected_index: 0,
            directory_state: DirectoryState::None {
                create: creating_path,
            },
            string_matches: Vec::new(),
            cancel_flag: Arc::new(AtomicBool::new(false)),
            should_dismiss: true,
            prompt_root: match path_style {
                PathStyle::Posix => "/".to_string(),
                PathStyle::Windows => "C:\\".to_string(),
            },
            path_style,
            replace_prompt: Task::ready(()),
            render_footer: Arc::new(|_, _| None),
            hidden_entries: false,
        }
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

    fn current_dir(&self) -> &'static str {
        match self.path_style {
            PathStyle::Posix => "./",
            PathStyle::Windows => ".\\",
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
    pub(crate) fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.set_prompt_for_open_path(Box::new(|workspace, lister, window, cx| {
            let (tx, rx) = futures::channel::oneshot::channel();
            Self::prompt_for_open_path(workspace, lister, false, tx, window, cx);
            rx
        }));
    }

    pub(crate) fn register_new_path(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.set_prompt_for_new_path(Box::new(|workspace, lister, window, cx| {
            let (tx, rx) = futures::channel::oneshot::channel();
            Self::prompt_for_open_path(workspace, lister, true, tx, window, cx);
            rx
        }));
    }

    fn prompt_for_open_path(
        workspace: &mut Workspace,
        lister: DirectoryLister,
        creating_path: bool,
        tx: oneshot::Sender<Option<Vec<PathBuf>>>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        workspace.toggle_modal(window, cx, |window, cx| {
            let delegate =
                OpenPathDelegate::new(tx, lister.clone(), creating_path, PathStyle::local());
            let picker = Picker::uniform_list(delegate, window, cx).width(rems(34.));
            let query = lister.default_query(cx);
            picker.set_query(query, window, cx);
            picker
        });
    }
}

impl PickerDelegate for OpenPathDelegate {
    type ListItem = ui::ListItem;

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

        let query = match &self.directory_state {
            DirectoryState::List { parent_path, .. } => {
                if parent_path == &dir {
                    None
                } else {
                    Some(lister.list_directory(dir.clone(), cx))
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
                    Some(lister.list_directory(dir.clone(), cx))
                }
            }
            DirectoryState::None { .. } => Some(lister.list_directory(dir.clone(), cx)),
        };
        self.cancel_flag.store(true, atomic::Ordering::Release);
        self.cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_flag = self.cancel_flag.clone();
        let hidden_entries = self.hidden_entries;
        let parent_path_is_root = self.prompt_root == dir;
        let current_dir = self.current_dir();
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
                                    entries: path_candidates(parent_path_is_root, paths),
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
                                    let mut entries = path_candidates(parent_path_is_root, paths);
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

            let mut max_id = 0;
            if !suffix.starts_with('.') && !hidden_entries {
                new_entries.retain(|entry| {
                    max_id = max_id.max(entry.path.id);
                    !entry.path.string.starts_with('.')
                });
            }

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
                            path_style.separator()
                        } else {
                            ""
                        }
                    )),
                    DirectoryState::List { parent_path, .. } => Some(format!(
                        "{}{}{}",
                        parent_path,
                        candidate.path.string,
                        if candidate.is_dir {
                            path_style.separator()
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

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(candidate) = self.get_entry(self.selected_index) else {
            return;
        };

        match &self.directory_state {
            DirectoryState::None { .. } => return,
            DirectoryState::List { parent_path, .. } => {
                let confirmed_path =
                    if parent_path == &self.prompt_root && candidate.path.string.is_empty() {
                        PathBuf::from(&self.prompt_root)
                    } else {
                        Path::new(self.lister.resolve_tilde(parent_path, cx).as_ref())
                            .join(&candidate.path.string)
                    };
                if let Some(tx) = self.tx.take() {
                    tx.send(Some(vec![confirmed_path])).ok();
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
                    let prompted_path =
                        if parent_path == &self.prompt_root && user_input.file.string.is_empty() {
                            PathBuf::from(&self.prompt_root)
                        } else {
                            Path::new(self.lister.resolve_tilde(parent_path, cx).as_ref())
                                .join(&user_input.file.string)
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
                                    if let Some(tx) = picker.delegate.tx.take() {
                                        tx.send(Some(vec![prompted_path])).ok();
                                    }
                                    cx.emit(gpui::DismissEvent);
                                })
                                .ok();
                        });
                        return;
                    } else if let Some(tx) = self.tx.take() {
                        tx.send(Some(vec![prompted_path])).ok();
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
        if let Some(tx) = self.tx.take() {
            tx.send(None).ok();
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
        let mut match_positions = match &self.directory_state {
            DirectoryState::List { .. } => self.string_matches.get(ix)?.positions.clone(),
            DirectoryState::Create { user_input, .. } => {
                if let Some(user_input) = user_input {
                    if !user_input.exists || !user_input.is_dir {
                        if ix == 0 {
                            Vec::new()
                        } else {
                            self.string_matches.get(ix - 1)?.positions.clone()
                        }
                    } else {
                        self.string_matches.get(ix)?.positions.clone()
                    }
                } else {
                    self.string_matches.get(ix)?.positions.clone()
                }
            }
            DirectoryState::None { .. } => Vec::new(),
        };

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
        Arc::from(format!("[directory{}]filename.ext", self.path_style.separator()).as_str())
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
) -> Vec<CandidateInfo> {
    if parent_path_is_root {
        children.push(DirectoryItem {
            is_dir: true,
            path: PathBuf::default(),
        });
    }

    children.sort_by(|a, b| compare_paths((&a.path, true), (&b.path, true)));
    children
        .iter()
        .enumerate()
        .map(|(ix, item)| CandidateInfo {
            path: StringMatchCandidate::new(ix, &item.path.to_string_lossy()),
            is_dir: item.is_dir,
        })
        .collect()
}

#[cfg(target_os = "windows")]
fn get_dir_and_suffix(query: String, path_style: PathStyle) -> (String, String) {
    let last_item = Path::new(&query)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    let (mut dir, suffix) = if let Some(dir) = query.strip_suffix(last_item.as_ref()) {
        (dir.to_string(), last_item.into_owned())
    } else {
        (query.to_string(), String::new())
    };
    match path_style {
        PathStyle::Posix => {
            if dir.is_empty() {
                dir = "/".to_string();
            }
        }
        PathStyle::Windows => {
            if dir.len() < 3 {
                dir = "C:\\".to_string();
            }
        }
    }
    (dir, suffix)
}

#[cfg(not(target_os = "windows"))]
fn get_dir_and_suffix(query: String, path_style: PathStyle) -> (String, String) {
    match path_style {
        PathStyle::Posix => {
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
            let (mut dir, suffix) = if let Some(index) = query.rfind('\\') {
                (query[..index].to_string(), query[index + 1..].to_string())
            } else {
                (query, String::new())
            };
            if dir.len() < 3 {
                dir = "C:\\".to_string();
            }
            if !dir.ends_with('\\') {
                dir.push('\\');
            }
            (dir, suffix)
        }
    }
}

#[cfg(test)]
mod tests {
    use util::paths::PathStyle;

    use crate::open_path_prompt::get_dir_and_suffix;

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
    }

    #[test]
    fn test_get_dir_and_suffix_with_posix_style() {
        let (dir, suffix) = get_dir_and_suffix("".into(), PathStyle::Posix);
        assert_eq!(dir, "/");
        assert_eq!(suffix, "");

        let (dir, suffix) = get_dir_and_suffix("/".into(), PathStyle::Posix);
        assert_eq!(dir, "/");
        assert_eq!(suffix, "");

        let (dir, suffix) = get_dir_and_suffix("/Use".into(), PathStyle::Posix);
        assert_eq!(dir, "/");
        assert_eq!(suffix, "Use");

        let (dir, suffix) = get_dir_and_suffix("/Users/Junkui/Docum".into(), PathStyle::Posix);
        assert_eq!(dir, "/Users/Junkui/");
        assert_eq!(suffix, "Docum");

        let (dir, suffix) = get_dir_and_suffix("/Users/Junkui/Documents".into(), PathStyle::Posix);
        assert_eq!(dir, "/Users/Junkui/");
        assert_eq!(suffix, "Documents");

        let (dir, suffix) = get_dir_and_suffix("/Users/Junkui/Documents/".into(), PathStyle::Posix);
        assert_eq!(dir, "/Users/Junkui/Documents/");
        assert_eq!(suffix, "");
    }
}
