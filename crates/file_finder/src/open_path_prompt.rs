use crate::file_finder_settings::FileFinderSettings;
use file_icons::FileIcons;
use futures::channel::oneshot;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{HighlightStyle, StyledText, Task};
use picker::{Picker, PickerDelegate};
use project::{DirectoryItem, DirectoryLister};
use settings::Settings;
use std::{
    path::{self, MAIN_SEPARATOR_STR, Path, PathBuf},
    sync::{
        Arc,
        atomic::{self, AtomicBool},
    },
};
use ui::{Context, LabelLike, ListItem, Window};
use ui::{HighlightedLabel, ListItemSpacing, prelude::*};
use util::{maybe, paths::compare_paths};
use workspace::Workspace;

pub(crate) struct OpenPathPrompt;

#[cfg(target_os = "windows")]
const PROMPT_ROOT: &str = "C:\\";
#[cfg(not(target_os = "windows"))]
const PROMPT_ROOT: &str = "/";

#[derive(Debug)]
pub struct OpenPathDelegate {
    tx: Option<oneshot::Sender<Option<Vec<PathBuf>>>>,
    lister: DirectoryLister,
    selected_index: usize,
    directory_state: DirectoryState,
    string_matches: Vec<StringMatch>,
    cancel_flag: Arc<AtomicBool>,
    should_dismiss: bool,
    replace_prompt: Task<()>,
}

impl OpenPathDelegate {
    pub fn new(
        tx: oneshot::Sender<Option<Vec<PathBuf>>>,
        lister: DirectoryLister,
        creating_path: bool,
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
            replace_prompt: Task::ready(()),
        }
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
                if let Some(user_input) = user_input {
                    if !user_input.exists || !user_input.is_dir {
                        if i == 0 {
                            return Some(CandidateInfo {
                                path: user_input.file.clone(),
                                is_dir: false,
                            });
                        } else {
                            i -= 1;
                        }
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
                .into_iter()
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
            let delegate = OpenPathDelegate::new(tx, lister.clone(), creating_path);
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
        let last_item = Path::new(&query)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();
        let (mut dir, suffix) = if let Some(dir) = query.strip_suffix(last_item.as_ref()) {
            (dir.to_string(), last_item.into_owned())
        } else {
            (query, String::new())
        };
        if dir == "" {
            dir = PROMPT_ROOT.to_string();
        }

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
                                    entries: path_candidates(&dir, paths),
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
                                    let mut entries = path_candidates(&dir, paths);
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

            if !suffix.starts_with('.') {
                new_entries.retain(|entry| !entry.path.string.starts_with('.'));
            }
            if suffix.is_empty() {
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
        Some(
            maybe!({
                match &self.directory_state {
                    DirectoryState::Create { parent_path, .. } => Some(format!(
                        "{}{}{}",
                        parent_path,
                        candidate.path.string,
                        if candidate.is_dir {
                            MAIN_SEPARATOR_STR
                        } else {
                            ""
                        }
                    )),
                    DirectoryState::List { parent_path, .. } => Some(format!(
                        "{}{}{}",
                        parent_path,
                        candidate.path.string,
                        if candidate.is_dir {
                            MAIN_SEPARATOR_STR
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
                    if parent_path == PROMPT_ROOT && candidate.path.string.is_empty() {
                        PathBuf::from(PROMPT_ROOT)
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
                        if parent_path == PROMPT_ROOT && user_input.file.string.is_empty() {
                            PathBuf::from(PROMPT_ROOT)
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
        let match_positions = match &self.directory_state {
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

        let file_icon = maybe!({
            if !settings.file_icons {
                return None;
            }
            let icon = if candidate.is_dir {
                FileIcons::get_folder_icon(false, cx)?
            } else {
                let path = path::Path::new(&candidate.path.string);
                FileIcons::get_icon(&path, cx)?
            };
            Some(Icon::from_path(icon).color(Color::Muted))
        });

        match &self.directory_state {
            DirectoryState::List { parent_path, .. } => Some(
                ListItem::new(ix)
                    .spacing(ListItemSpacing::Sparse)
                    .start_slot::<Icon>(file_icon)
                    .inset(true)
                    .toggle_state(selected)
                    .child(HighlightedLabel::new(
                        if parent_path == PROMPT_ROOT {
                            format!("{}{}", PROMPT_ROOT, candidate.path.string)
                        } else {
                            candidate.path.string.clone()
                        },
                        match_positions,
                    )),
            ),
            DirectoryState::Create {
                parent_path,
                user_input,
                ..
            } => {
                let (label, delta) = if parent_path == PROMPT_ROOT {
                    (
                        format!("{}{}", PROMPT_ROOT, candidate.path.string),
                        PROMPT_ROOT.len(),
                    )
                } else {
                    (candidate.path.string.clone(), 0)
                };
                let label_len = label.len();

                let label_with_highlights = match user_input {
                    Some(user_input) => {
                        if user_input.file.string == candidate.path.string {
                            if user_input.exists {
                                let label = if user_input.is_dir {
                                    label
                                } else {
                                    format!("{label} (replace)")
                                };
                                StyledText::new(label)
                                    .with_default_highlights(
                                        &window.text_style().clone(),
                                        vec![(
                                            delta..delta + label_len,
                                            HighlightStyle::color(Color::Conflict.color(cx)),
                                        )],
                                    )
                                    .into_any_element()
                            } else {
                                StyledText::new(format!("{label} (create)"))
                                    .with_default_highlights(
                                        &window.text_style().clone(),
                                        vec![(
                                            delta..delta + label_len,
                                            HighlightStyle::color(Color::Created.color(cx)),
                                        )],
                                    )
                                    .into_any_element()
                            }
                        } else {
                            let mut highlight_positions = match_positions;
                            highlight_positions.iter_mut().for_each(|position| {
                                *position += delta;
                            });
                            HighlightedLabel::new(label, highlight_positions).into_any_element()
                        }
                    }
                    None => {
                        let mut highlight_positions = match_positions;
                        highlight_positions.iter_mut().for_each(|position| {
                            *position += delta;
                        });
                        HighlightedLabel::new(label, highlight_positions).into_any_element()
                    }
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
            DirectoryState::None { .. } => return None,
        }
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
        Arc::from(format!("[directory{MAIN_SEPARATOR_STR}]filename.ext"))
    }
}

fn path_candidates(parent_path: &String, mut children: Vec<DirectoryItem>) -> Vec<CandidateInfo> {
    if *parent_path == PROMPT_ROOT {
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
