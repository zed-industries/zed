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

#[cfg(not(target_os = "windows"))]
const PROMPT_ROOT: &str = "C:\\";
#[cfg(target_os = "windows")]
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
            DirectoryState::List {
                match_candidates, ..
            }
            | DirectoryState::Create {
                match_candidates, ..
            } => self
                .matches
                .iter()
                .filter_map(|&index| {
                    match_candidates
                        .get(index)
                        .map(|candidate| candidate.path.string.clone())
                })
                .collect(),
            DirectoryState::None { .. } => Vec::new(),
        }
    }
}

#[derive(Debug)]
enum DirectoryState {
    List {
        path: String,
        match_candidates: Vec<CandidateInfo>,
        error: Option<SharedString>,
    },
    Create {
        path: String,
        match_candidates: Vec<CandidateInfo>,
        is_dir: Option<bool>,
    },
    None {
        create: bool,
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
            Self::prompt_for_open_path(workspace, lister, tx, window, cx);
            rx
        }));
    }

    fn prompt_for_open_path(
        workspace: &mut Workspace,
        lister: DirectoryLister,
        tx: oneshot::Sender<Option<Vec<PathBuf>>>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        workspace.toggle_modal(window, cx, |window, cx| {
            let delegate = OpenPathDelegate::new(tx, lister.clone(), false);
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
            DirectoryState::List { path, .. } => {
                if path == &dir {
                    None
                } else {
                    Some(lister.list_directory(dir.clone(), cx))
                }
            }
            DirectoryState::Create { path, is_dir, .. } => {
                if is_dir.unwrap_or(false) && path != &dir {
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

                this.update(cx, |this, _| match &this.delegate.directory_state {
                    DirectoryState::None { create: false } | DirectoryState::List { .. } => {
                        this.delegate.directory_state = match paths {
                            Ok(mut paths) => {
                                if dir == PROMPT_ROOT {
                                    paths.push(DirectoryItem {
                                        is_dir: true,
                                        path: PathBuf::default(),
                                    });
                                }

                                paths.sort_by(|a, b| {
                                    compare_paths((&a.path, true), (&b.path, true))
                                });
                                let match_candidates = paths
                                    .iter()
                                    .enumerate()
                                    .map(|(ix, item)| CandidateInfo {
                                        path: StringMatchCandidate::new(
                                            ix,
                                            &item.path.to_string_lossy(),
                                        ),
                                        is_dir: item.is_dir,
                                    })
                                    .collect::<Vec<_>>();

                                DirectoryState::List {
                                    match_candidates,
                                    path: dir,
                                    error: None,
                                }
                            }
                            Err(err) => DirectoryState::List {
                                match_candidates: Vec::new(),
                                path: dir,
                                error: Some(err.to_string().into()),
                            },
                        };
                    }
                    DirectoryState::None { create: true } | DirectoryState::Create { .. } => {
                        this.delegate.directory_state = match paths {
                            Ok(mut paths) => {
                                if dir == PROMPT_ROOT {
                                    paths.push(DirectoryItem {
                                        is_dir: true,
                                        path: PathBuf::default(),
                                    });
                                }

                                paths.sort_by(|a, b| {
                                    compare_paths((&a.path, true), (&b.path, true))
                                });
                                let match_candidates = paths
                                    .iter()
                                    .enumerate()
                                    .map(|(ix, item)| CandidateInfo {
                                        path: StringMatchCandidate::new(
                                            ix,
                                            &item.path.to_string_lossy(),
                                        ),
                                        is_dir: item.is_dir,
                                    })
                                    .collect::<Vec<_>>();

                                DirectoryState::Create {
                                    match_candidates,
                                    path: dir,
                                    is_dir: Some(true),
                                }
                            }
                            Err(_) => DirectoryState::Create {
                                match_candidates: vec![CandidateInfo {
                                    path: StringMatchCandidate::new(0, &suffix),
                                    is_dir: false,
                                }],
                                path: dir,
                                is_dir: None,
                            },
                        };
                    }
                })
                .ok();
            }

            let Some(mut match_candidates) = this
                .update(cx, |this, cx| match &this.delegate.directory_state {
                    DirectoryState::List { error: Some(_), .. } => {
                        this.delegate.matches.clear();
                        this.delegate.selected_index = 0;
                        cx.notify();
                        None
                    }
                    DirectoryState::List {
                        match_candidates,
                        error: None,
                        ..
                    } => Some(match_candidates.clone()),
                    DirectoryState::Create {
                        match_candidates, ..
                    } => Some(match_candidates.clone()),
                    DirectoryState::None { .. } => None,
                })
                .unwrap_or(None)
            else {
                return;
            };

            if !suffix.starts_with('.') {
                match_candidates.retain(|m| !m.path.string.starts_with('.'));
            }
            if suffix.is_empty() {
                this.update(cx, |this, cx| {
                    this.delegate.matches.clear();
                    this.delegate.string_matches.clear();
                    this.delegate
                        .matches
                        .extend(match_candidates.iter().map(|m| m.path.id));

                    cx.notify();
                })
                .ok();
                return;
            }

            let candidates = match_candidates.iter().map(|m| &m.path).collect::<Vec<_>>();
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
                this.delegate.matches.clear();
                this.delegate.string_matches = matches.clone();
                this.delegate
                    .matches
                    .extend(matches.into_iter().map(|m| m.candidate_id));
                this.delegate.matches.sort_by_key(|m| {
                    (
                        match &this.delegate.directory_state {
                            DirectoryState::List {
                                match_candidates, ..
                            } => match_candidates
                                .get(*m)
                                .map(|c| !c.path.string.starts_with(&suffix)),
                            DirectoryState::Create {
                                match_candidates, ..
                            } => match_candidates
                                .get(*m)
                                .map(|c| !c.path.string.starts_with(&suffix)),
                            DirectoryState::None { .. } => None,
                        },
                        *m,
                    )
                });
                this.delegate.selected_index = 0;
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
                    DirectoryState::List {
                        path,
                        match_candidates,
                        ..
                    }
                    | DirectoryState::Create {
                        path,
                        match_candidates,
                        is_dir: Some(true),
                    } => {
                        let candidate = match_candidates.get(*m)?;
                        Some(format!(
                            "{}{}{}",
                            path,
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
                path,
                match_candidates,
                ..
            } => {
                let Some(candidate) = match_candidates.get(*m) else {
                    return;
                };
                let confirmed_path = if path == PROMPT_ROOT && candidate.path.string.is_empty() {
                    PathBuf::from(PROMPT_ROOT)
                } else {
                    Path::new(self.lister.resolve_tilde(path, cx).as_ref())
                        .join(&candidate.path.string)
                };
                if let Some(tx) = self.tx.take() {
                    tx.send(Some(vec![confirmed_path])).ok();
                }
            }
            DirectoryState::Create {
                path,
                is_dir,
                match_candidates,
            } => match is_dir {
                Some(true) => {
                    return;
                }
                Some(false) => {
                    let Some(candidate) = match_candidates.get(*m) else {
                        return;
                    };
                    self.should_dismiss = false;
                    let prompted_path = if path == PROMPT_ROOT && candidate.path.string.is_empty() {
                        PathBuf::from(PROMPT_ROOT)
                    } else {
                        Path::new(self.lister.resolve_tilde(path, cx).as_ref())
                            .join(&candidate.path.string)
                    };
                    let answer = window.prompt(
                            gpui::PromptLevel::Critical,
                            &format!("{prompted_path:?} already exists. Do you want to replace it?"),
                            Some(
                                "A file or folder with the same name already exists. Replacing it will overwrite its current contents.",
                            ),
                            &["Replace", "Cancel"],
                        cx);
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
                }
                None => {
                    if let Some(tx) = self.tx.take() {
                        tx.send(Some(vec![PathBuf::from(path)])).ok();
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
                path,
                match_candidates,
                ..
            } => {
                let candidate = match_candidates.get(*m)?;
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
                            if path == PROMPT_ROOT {
                                format!("{}{}", PROMPT_ROOT, candidate.path.string)
                            } else {
                                candidate.path.string.clone()
                            },
                            highlight_positions,
                        )),
                )
            }
            DirectoryState::Create {
                path,
                is_dir,
                match_candidates,
            } => {
                let candidate = match_candidates.get(*m)?;

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

                let (label, delta) = if path == PROMPT_ROOT {
                    (
                        format!("{}{}", PROMPT_ROOT, candidate.path.string),
                        PROMPT_ROOT.len(),
                    )
                } else {
                    (candidate.path.string.clone(), 0)
                };
                let label_len = label.len();

                let label_with_highlights = match is_dir {
                    Some(true) => {
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
                    Some(false) => StyledText::new(label)
                        .with_default_highlights(
                            &window.text_style().clone(),
                            vec![(
                                delta..delta + label_len,
                                HighlightStyle::color(Color::Conflict.color(cx)),
                            )],
                        )
                        .into_any_element(),
                    None => StyledText::new(label)
                        .with_default_highlights(
                            &window.text_style().clone(),
                            vec![(
                                delta..delta + label_len,
                                HighlightStyle::color(Color::Created.color(cx)),
                            )],
                        )
                        .into_any_element(),
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
