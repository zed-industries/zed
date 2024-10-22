use futures::channel::oneshot;
use fuzzy::StringMatchCandidate;
use picker::{Picker, PickerDelegate};
use project::DirectoryLister;
use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{self, AtomicBool},
        Arc,
    },
};
use ui::{prelude::*, LabelLike, ListItemSpacing};
use ui::{ListItem, ViewContext};
use util::{maybe, paths::compare_paths};
use workspace::Workspace;

pub(crate) struct OpenPathPrompt;

pub struct OpenPathDelegate {
    tx: Option<oneshot::Sender<Option<Vec<PathBuf>>>>,
    lister: DirectoryLister,
    selected_index: usize,
    directory_state: Option<DirectoryState>,
    matches: Vec<usize>,
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
            cancel_flag: Arc::new(AtomicBool::new(false)),
            should_dismiss: true,
        }
    }
}

struct DirectoryState {
    path: String,
    match_candidates: Vec<StringMatchCandidate>,
    error: Option<SharedString>,
}

impl OpenPathPrompt {
    pub(crate) fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        workspace.set_prompt_for_open_path(Box::new(|workspace, lister, cx| {
            let (tx, rx) = futures::channel::oneshot::channel();
            Self::prompt_for_open_path(workspace, lister, tx, cx);
            rx
        }));
    }

    fn prompt_for_open_path(
        workspace: &mut Workspace,
        lister: DirectoryLister,
        tx: oneshot::Sender<Option<Vec<PathBuf>>>,
        cx: &mut ViewContext<Workspace>,
    ) {
        workspace.toggle_modal(cx, |cx| {
            let delegate = OpenPathDelegate::new(tx, lister.clone());

            let picker = Picker::uniform_list(delegate, cx).width(rems(34.));
            let query = lister.default_query(cx);
            picker.set_query(query, cx);
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

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
        cx.notify();
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> gpui::Task<()> {
        let lister = self.lister.clone();
        let (mut dir, suffix) = if let Some(index) = query.rfind('/') {
            (query[..index].to_string(), query[index + 1..].to_string())
        } else {
            (query, String::new())
        };
        if dir == "" {
            dir = "/".to_string();
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

        cx.spawn(|this, mut cx| async move {
            if let Some(query) = query {
                let paths = query.await;
                if cancel_flag.load(atomic::Ordering::Relaxed) {
                    return;
                }

                this.update(&mut cx, |this, _| {
                    this.delegate.directory_state = Some(match paths {
                        Ok(mut paths) => {
                            paths.sort_by(|a, b| compare_paths((a, true), (b, true)));
                            let match_candidates = paths
                                .iter()
                                .enumerate()
                                .map(|(ix, path)| {
                                    StringMatchCandidate::new(ix, path.to_string_lossy().into())
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
                .update(&mut cx, |this, cx| {
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
                match_candidates.retain(|m| !m.string.starts_with('.'));
            }

            if suffix == "" {
                this.update(&mut cx, |this, cx| {
                    this.delegate.matches.clear();
                    this.delegate
                        .matches
                        .extend(match_candidates.iter().map(|m| m.id));

                    cx.notify();
                })
                .ok();
                return;
            }

            let matches = fuzzy::match_strings(
                match_candidates.as_slice(),
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

            this.update(&mut cx, |this, cx| {
                this.delegate.matches.clear();
                this.delegate
                    .matches
                    .extend(matches.into_iter().map(|m| m.candidate_id));
                this.delegate.matches.sort_by_key(|m| {
                    (
                        this.delegate.directory_state.as_ref().and_then(|d| {
                            d.match_candidates
                                .get(*m)
                                .map(|c| !c.string.starts_with(&suffix))
                        }),
                        *m,
                    )
                });
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm_completion(&self, query: String) -> Option<String> {
        Some(
            maybe!({
                let m = self.matches.get(self.selected_index)?;
                let directory_state = self.directory_state.as_ref()?;
                let candidate = directory_state.match_candidates.get(*m)?;
                Some(format!("{}/{}", directory_state.path, candidate.string))
            })
            .unwrap_or(query),
        )
    }

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<Self>>) {
        let Some(m) = self.matches.get(self.selected_index) else {
            return;
        };
        let Some(directory_state) = self.directory_state.as_ref() else {
            return;
        };
        let Some(candidate) = directory_state.match_candidates.get(*m) else {
            return;
        };
        let result = Path::new(
            self.lister
                .resolve_tilde(&directory_state.path, cx)
                .as_ref(),
        )
        .join(&candidate.string);
        if let Some(tx) = self.tx.take() {
            tx.send(Some(vec![result])).ok();
        }
        cx.emit(gpui::DismissEvent);
    }

    fn should_dismiss(&self) -> bool {
        self.should_dismiss
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(tx) = self.tx.take() {
            tx.send(None).ok();
        }
        cx.emit(gpui::DismissEvent)
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let m = self.matches.get(ix)?;
        let directory_state = self.directory_state.as_ref()?;
        let candidate = directory_state.match_candidates.get(*m)?;

        Some(
            ListItem::new(ix)
                .spacing(ListItemSpacing::Sparse)
                .inset(true)
                .selected(selected)
                .child(LabelLike::new().child(candidate.string.clone())),
        )
    }

    fn no_matches_text(&self, _cx: &mut WindowContext) -> SharedString {
        if let Some(error) = self.directory_state.as_ref().and_then(|s| s.error.clone()) {
            error
        } else {
            "No such file or directory".into()
        }
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        Arc::from("[directory/]filename.ext")
    }
}
