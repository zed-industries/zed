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

pub struct OpenPathDelegate {
    tx: Option<oneshot::Sender<Option<Vec<PathBuf>>>>,
    lister: DirectoryLister,
    selected_index: usize,
    directory_state: DirectoryState,
    matches: Vec<usize>,
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
            matches: Vec::new(),
            string_matches: Vec::new(),
            cancel_flag: Arc::new(AtomicBool::new(false)),
            should_dismiss: true,
            replace_prompt: Task::ready(()),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn collect_match_candidates(&self) -> Vec<String> {
        match &self.directory_state {
            DirectoryState::List { current_match, .. }
            | DirectoryState::Create { current_match, .. } => match current_match {
                CurrentMatch::Directory { entries, .. } => self
                    .matches
                    .iter()
                    .filter_map(|&index| {
                        entries
                            .get(index)
                            .map(|candidate| candidate.path.string.clone())
                    })
                    .collect(),
                CurrentMatch::File { file, .. } => vec![file.string.clone()],
            },
            DirectoryState::None { .. } => Vec::new(),
        }
    }
}

#[derive(Debug)]
enum DirectoryState {
    List {
        parent_path: String,
        current_match: CurrentMatch,
        error: Option<SharedString>,
    },
    Create {
        parent_path: String,
        current_match: CurrentMatch,
    },
    None {
        create: bool,
    },
}

#[derive(Debug, Clone)]
enum CurrentMatch {
    Directory {
        entries_query: Option<String>,
        entries: Vec<CandidateInfo>,
    },
    File {
        exists: bool,
        file: StringMatchCandidate,
    },
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
        self.matches.len()
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
                current_match,
            } => {
                let refresh = match current_match {
                    CurrentMatch::Directory { .. } => parent_path != &dir,
                    CurrentMatch::File { .. } => true,
                };

                if refresh {
                    Some(lister.list_directory(dir.clone(), cx))
                } else {
                    None
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

                let entries_query = if suffix.is_empty() {
                    None
                } else {
                    Some(suffix.clone())
                };

                this.update(cx, |this, _| match &this.delegate.directory_state {
                    DirectoryState::None { create: false } | DirectoryState::List { .. } => {
                        this.delegate.directory_state = match paths {
                            Ok(paths) => DirectoryState::List {
                                current_match: CurrentMatch::Directory {
                                    entries_query,
                                    entries: path_candidates(&dir, paths),
                                },
                                parent_path: dir,
                                error: None,
                            },
                            Err(err) => DirectoryState::List {
                                current_match: CurrentMatch::Directory {
                                    entries_query: None,
                                    entries: Vec::new(),
                                },
                                parent_path: dir,
                                error: Some(err.to_string().into()),
                            },
                        };
                    }
                    DirectoryState::None { create: true } | DirectoryState::Create { .. } => {
                        this.delegate.directory_state = match paths {
                            Ok(paths) => DirectoryState::Create {
                                current_match: CurrentMatch::Directory {
                                    entries_query,
                                    entries: path_candidates(&dir, paths),
                                },
                                parent_path: dir,
                            },
                            Err(_) => DirectoryState::Create {
                                current_match: CurrentMatch::File {
                                    exists: false,
                                    file: StringMatchCandidate::new(0, &suffix),
                                },
                                parent_path: dir,
                            },
                        };
                    }
                })
                .ok();
            }

            let Some(mut current_match) = this
                .update(cx, |this, cx| match &this.delegate.directory_state {
                    DirectoryState::List { error: Some(_), .. } => {
                        this.delegate.matches.clear();
                        this.delegate.selected_index = 0;
                        cx.notify();
                        None
                    }
                    DirectoryState::Create { current_match, .. }
                    | DirectoryState::List {
                        current_match,
                        error: None,
                        ..
                    } => Some(current_match.clone()),
                    DirectoryState::None { .. } => None,
                })
                .unwrap_or(None)
            else {
                return;
            };

            if !suffix.starts_with('.') {
                if let CurrentMatch::Directory { entries, .. } = &mut current_match {
                    entries.retain(|entry| !entry.path.string.starts_with('.'));
                }
            }
            if suffix.is_empty() {
                this.update(cx, |this, cx| {
                    this.delegate.matches.clear();
                    this.delegate.string_matches.clear();
                    if let CurrentMatch::Directory { entries, .. } = current_match {
                        this.delegate
                            .matches
                            .extend(entries.iter().map(|m| m.path.id));
                        // TODO kb is it enough? fill more fields?
                    }

                    cx.notify();
                })
                .ok();
                return;
            }

            let Ok(is_create_state) =
                this.update(cx, |this, _| match &this.delegate.directory_state {
                    DirectoryState::Create { .. } => true,
                    DirectoryState::List { .. } | DirectoryState::None { .. } => false,
                })
            else {
                return;
            };

            let mut replaced_file_create_conflict = false;
            let candidates = match &current_match {
                CurrentMatch::Directory {
                    entries,
                    entries_query,
                } => entries
                    .iter()
                    .filter_map(|entry| {
                        if is_create_state && entries_query.as_ref() == Some(&entry.path.string) {
                            replaced_file_create_conflict = !entry.is_dir;
                            if replaced_file_create_conflict {
                                return None;
                            }
                        }
                        Some(&entry.path)
                    })
                    .collect(),
                CurrentMatch::File { file, .. } => {
                    if is_create_state {
                        Vec::new()
                    } else {
                        vec![file]
                    }
                }
            };
            let mut matches = fuzzy::match_strings(
                candidates.as_slice(),
                &suffix,
                false,
                100,
                &cancel_flag,
                cx.background_executor().clone(),
            )
            .await;

            if replaced_file_create_conflict {
                if let CurrentMatch::Directory {
                    entries_query: Some(new_file_name),
                    ..
                } = &current_match
                {
                    matches.insert(
                        0,
                        StringMatch {
                            // TODO kb where does this point to?
                            candidate_id: 0,
                            score: 1.0,
                            positions: Vec::new(),
                            string: new_file_name.clone(),
                        },
                    );
                }
            }
            if cancel_flag.load(atomic::Ordering::Acquire) {
                return;
            }

            this.update(cx, |this, cx| {
                this.delegate.matches.clear();
                this.delegate.selected_index = 0;
                this.delegate.string_matches = matches.clone();
                this.delegate
                    .matches
                    .extend(matches.into_iter().map(|m| m.candidate_id));

                match &mut this.delegate.directory_state {
                    DirectoryState::List { current_match, .. } => {
                        this.delegate.matches.sort_by_key(|m| {
                            (
                                match current_match {
                                    CurrentMatch::Directory { entries, .. } => {
                                        entries.get_mut(*m).map(|entry| &mut entry.path)
                                    }
                                    CurrentMatch::File { file, .. } => Some(file),
                                }
                                .map(|candidate| !candidate.string.starts_with(&suffix)),
                                *m,
                            )
                        })
                    }
                    DirectoryState::Create { current_match, .. } => {
                        // TODO kb no conflict custom highlights
                        if this.delegate.matches.is_empty() {
                            let id = 0;
                            *current_match = CurrentMatch::File {
                                exists: false,
                                file: StringMatchCandidate::new(id, &suffix),
                            };
                            this.delegate.matches = vec![id];
                            this.delegate.string_matches = vec![StringMatch {
                                candidate_id: id,
                                score: 1.0,
                                positions: Vec::new(),
                                string: suffix,
                            }];
                        } else {
                            this.delegate.matches.sort_by_key(|m| {
                                (
                                    match current_match {
                                        CurrentMatch::Directory { entries, .. } => {
                                            entries.get_mut(*m).map(|entry| &mut entry.path)
                                        }
                                        CurrentMatch::File { file, .. } => Some(file),
                                    }
                                    .map(|c| !c.string.starts_with(&suffix)),
                                    *m,
                                )
                            });
                        }
                    }
                    DirectoryState::None { .. } => {
                        this.delegate.matches.sort_by_key(|m| (None::<()>, *m))
                    }
                }

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
        let m = self.matches.get(self.selected_index)?;
        Some(
            maybe!({
                match &self.directory_state {
                    DirectoryState::Create {
                        parent_path,
                        current_match: CurrentMatch::Directory { entries, .. },
                    } => {
                        let candidate = entries.get(*m)?;
                        Some(format!(
                            "{}{}{}",
                            parent_path,
                            candidate.path.string,
                            if candidate.is_dir {
                                MAIN_SEPARATOR_STR
                            } else {
                                ""
                            }
                        ))
                    }
                    DirectoryState::List {
                        parent_path,
                        current_match,
                        ..
                    } => {
                        let candidate = match current_match {
                            CurrentMatch::Directory { entries, .. } => match entries.get(*m) {
                                Some(candidate) => candidate,
                                None => return None,
                            },
                            CurrentMatch::File { file, .. } => &CandidateInfo {
                                path: file.clone(),
                                is_dir: false,
                            },
                        };
                        Some(format!(
                            "{}{}{}",
                            parent_path,
                            candidate.path.string,
                            if candidate.is_dir {
                                MAIN_SEPARATOR_STR
                            } else {
                                ""
                            }
                        ))
                    }
                    DirectoryState::Create { .. } | DirectoryState::None { .. } => return None,
                }
            })
            .unwrap_or(query),
        )
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(m) = self.matches.get(self.selected_index) else {
            return;
        };

        match &self.directory_state {
            DirectoryState::None { .. } => return,
            DirectoryState::List {
                parent_path,
                current_match,
                ..
            } => {
                let candidate = match current_match {
                    CurrentMatch::Directory { entries, .. } => match entries.get(*m) {
                        Some(candidate) => candidate,
                        None => return,
                    },
                    CurrentMatch::File { file, .. } => &CandidateInfo {
                        path: file.clone(),
                        is_dir: false,
                    },
                };
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
                current_match,
            } => match current_match {
                CurrentMatch::Directory { .. } => return,
                CurrentMatch::File { exists, file } => {
                    let prompted_path = if parent_path == PROMPT_ROOT && file.string.is_empty() {
                        PathBuf::from(PROMPT_ROOT)
                    } else {
                        Path::new(self.lister.resolve_tilde(parent_path, cx).as_ref())
                            .join(&file.string)
                    };
                    if *exists {
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
                        tx.send(Some(vec![PathBuf::from(prompted_path)])).ok();
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
        let m = self.matches.get(ix)?;
        match &self.directory_state {
            DirectoryState::List {
                parent_path,
                current_match,
                ..
            } => {
                let candidate = match current_match {
                    CurrentMatch::Directory { entries, .. } => entries.get(*m)?,
                    CurrentMatch::File { file, .. } => &CandidateInfo {
                        path: file.clone(),
                        is_dir: false,
                    },
                };
                let highlight_positions = self
                    .string_matches
                    .iter()
                    .find(|string_match| string_match.candidate_id == *m)
                    .map(|string_match| string_match.positions.clone())
                    .unwrap_or_default();

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

                Some(
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
                            highlight_positions,
                        )),
                )
            }
            DirectoryState::Create {
                parent_path,
                current_match,
            } => {
                let mut conflicts = false;
                let candidate = match current_match {
                    CurrentMatch::Directory {
                        entries,
                        entries_query,
                    } => {
                        let entry = entries.get(*m)?;
                        if Some(&entry.path.string) == entries_query.as_ref() {
                            conflicts = true;
                        }
                        entry
                    }
                    CurrentMatch::File { file, .. } => &CandidateInfo {
                        path: file.clone(),
                        is_dir: false,
                    },
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

                let (label, delta) = if parent_path == PROMPT_ROOT {
                    (
                        format!("{}{}", PROMPT_ROOT, candidate.path.string),
                        PROMPT_ROOT.len(),
                    )
                } else {
                    (candidate.path.string.clone(), 0)
                };
                let label_len = label.len();

                let label_with_highlights = match current_match {
                    CurrentMatch::Directory { .. } => {
                        if conflicts {
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
                            let highlight_positions = self
                                .string_matches
                                .iter()
                                .find(|string_match| string_match.candidate_id == *m)
                                .map(|string_match| string_match.positions.clone())
                                .map(|mut positions| {
                                    positions.iter_mut().for_each(|position| {
                                        *position += delta;
                                    });
                                    positions
                                })
                                .unwrap_or_default();
                            HighlightedLabel::new(label, highlight_positions).into_any_element()
                        }
                    }
                    CurrentMatch::File { exists, .. } => {
                        let color = if *exists || conflicts {
                            Color::Conflict.color(cx)
                        } else {
                            Color::Created.color(cx)
                        };
                        StyledText::new(label)
                            .with_default_highlights(
                                &window.text_style().clone(),
                                vec![(delta..delta + label_len, HighlightStyle::color(color))],
                            )
                            .into_any_element()
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
