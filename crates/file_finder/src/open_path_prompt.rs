use futures::channel::oneshot;
use fuzzy::{StringMatch, StringMatchCandidate};
use picker::{Picker, PickerDelegate};
use project::{DirectoryItem, DirectoryLister};
use std::{
    path::{MAIN_SEPARATOR_STR, Path, PathBuf},
    sync::{
        Arc,
        atomic::{self, AtomicBool},
    },
};
use ui::{Context, ListItem, Window};
use ui::{HighlightedLabel, ListItemSpacing, prelude::*};
use util::{maybe, paths::compare_paths};
use workspace::Workspace;

pub(crate) struct OpenPathPrompt;

pub struct OpenPathDelegate {
    tx: Option<oneshot::Sender<Option<Vec<PathBuf>>>>,
    lister: DirectoryLister,
    selected_index: usize,
    directory_state: Option<DirectoryState>,
    matches: Vec<usize>,
    string_matches: Vec<StringMatch>,
    cancel_flag: Arc<AtomicBool>,
    should_dismiss: bool,
}

impl OpenPathDelegate {
    pub fn new(tx: oneshot::Sender<Option<Vec<PathBuf>>>, lister: DirectoryLister) -> Self {
        Self {
            tx: Some(tx),
            lister,
            selected_index: 0,
            directory_state: None,
            matches: Vec::new(),
            string_matches: Vec::new(),
            cancel_flag: Arc::new(AtomicBool::new(false)),
            should_dismiss: true,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn collect_match_candidates(&self) -> Vec<String> {
        if let Some(state) = self.directory_state.as_ref() {
            self.matches
                .iter()
                .filter_map(|&index| {
                    state
                        .match_candidates
                        .get(index)
                        .map(|candidate| candidate.path.string.clone())
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug)]
struct DirectoryState {
    path: String,
    match_candidates: Vec<CandidateInfo>,
    error: Option<SharedString>,
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
            let delegate = OpenPathDelegate::new(tx, lister.clone());

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
    ) -> gpui::Task<()> {
        let lister = self.lister.clone();
        let query_path = Path::new(&query);
        let last_item = query_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let (mut dir, suffix) = if let Some(dir) = query.strip_suffix(&last_item) {
            (dir.to_string(), last_item)
        } else {
            (query, String::new())
        };

        if dir == "" {
            #[cfg(not(target_os = "windows"))]
            {
                dir = "/".to_string();
            }
            #[cfg(target_os = "windows")]
            {
                dir = "C:\\".to_string();
            }
        }

        let query = if self
            .directory_state
            .as_ref()
            .map_or(false, |s| s.path == dir)
        {
            None
        } else {
            Some(lister.list_directory(dir.clone(), cx))
        };
        self.cancel_flag.store(true, atomic::Ordering::Relaxed);
        self.cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_flag = self.cancel_flag.clone();

        cx.spawn_in(window, async move |this, cx| {
            if let Some(query) = query {
                let paths = query.await;
                if cancel_flag.load(atomic::Ordering::Relaxed) {
                    return;
                }

                this.update(cx, |this, _| {
                    this.delegate.directory_state = Some(match paths {
                        Ok(mut paths) => {
                            if dir == "/" {
                                paths.push(DirectoryItem {
                                    is_dir: true,
                                    path: Default::default(),
                                });
                            }

                            paths.sort_by(|a, b| compare_paths((&a.path, true), (&b.path, true)));
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

                            DirectoryState {
                                match_candidates,
                                path: dir,
                                error: None,
                            }
                        }
                        Err(err) => DirectoryState {
                            match_candidates: vec![],
                            path: dir,
                            error: Some(err.to_string().into()),
                        },
                    });
                })
                .ok();
            }

            let match_candidates = this
                .update(cx, |this, cx| {
                    let directory_state = this.delegate.directory_state.as_ref()?;
                    if directory_state.error.is_some() {
                        this.delegate.matches.clear();
                        this.delegate.selected_index = 0;
                        cx.notify();
                        return None;
                    }

                    Some(directory_state.match_candidates.clone())
                })
                .unwrap_or(None);

            let Some(mut match_candidates) = match_candidates else {
                return;
            };

            if !suffix.starts_with('.') {
                match_candidates.retain(|m| !m.path.string.starts_with('.'));
            }

            if suffix == "" {
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
            if cancel_flag.load(atomic::Ordering::Relaxed) {
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
                        this.delegate.directory_state.as_ref().and_then(|d| {
                            d.match_candidates
                                .get(*m)
                                .map(|c| !c.path.string.starts_with(&suffix))
                        }),
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
        Some(
            maybe!({
                let m = self.matches.get(self.selected_index)?;
                let directory_state = self.directory_state.as_ref()?;
                let candidate = directory_state.match_candidates.get(*m)?;
                Some(format!(
                    "{}{}{}",
                    directory_state.path,
                    candidate.path.string,
                    if candidate.is_dir {
                        MAIN_SEPARATOR_STR
                    } else {
                        ""
                    }
                ))
            })
            .unwrap_or(query),
        )
    }

    fn confirm(&mut self, _: bool, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(m) = self.matches.get(self.selected_index) else {
            return;
        };
        let Some(directory_state) = self.directory_state.as_ref() else {
            return;
        };
        let Some(candidate) = directory_state.match_candidates.get(*m) else {
            return;
        };
        let result = if directory_state.path == "/" && candidate.path.string.is_empty() {
            PathBuf::from("/")
        } else {
            Path::new(
                self.lister
                    .resolve_tilde(&directory_state.path, cx)
                    .as_ref(),
            )
            .join(&candidate.path.string)
        };
        if let Some(tx) = self.tx.take() {
            tx.send(Some(vec![result])).ok();
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
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let m = self.matches.get(ix)?;
        let directory_state = self.directory_state.as_ref()?;
        let candidate = directory_state.match_candidates.get(*m)?;
        let highlight_positions = self
            .string_matches
            .iter()
            .find(|string_match| string_match.candidate_id == *m)
            .map(|string_match| string_match.positions.clone())
            .unwrap_or_default();

        Some(
            ListItem::new(ix)
                .spacing(ListItemSpacing::Sparse)
                .inset(true)
                .toggle_state(selected)
                .child(HighlightedLabel::new(
                    if directory_state.path == "/" {
                        format!("/{}", candidate.path.string)
                    } else {
                        candidate.path.string.clone()
                    },
                    highlight_positions,
                )),
        )
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        let text = if let Some(error) = self.directory_state.as_ref().and_then(|s| s.error.clone())
        {
            error
        } else {
            "No such file or directory".into()
        };
        Some(text)
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        Arc::from(format!("[directory{MAIN_SEPARATOR_STR}]filename.ext"))
    }
}
