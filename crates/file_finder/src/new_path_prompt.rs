use futures::channel::oneshot;
use fuzzy::PathMatch;
use gpui::Model;
use picker::{Picker, PickerDelegate};
use project::{Entry, PathMatchCandidateSet, Project, WorktreeId};
use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{self, AtomicBool},
        Arc,
    },
};
use ui::{prelude::*, HighlightedLabel};
use ui::{ListItem, ViewContext};
use util::ResultExt;
use workspace::Workspace;

pub(crate) struct NewPathPrompt;

#[derive(Debug)]
struct Match {
    path_match: Option<PathMatch>,
    suffix: Option<String>,
}

impl Match {
    fn entry<'a>(&'a self, project: &'a Project, cx: &'a WindowContext) -> Option<&'a Entry> {
        if let Some(suffix) = &self.suffix {
            let (worktree, path) = if let Some(path_match) = &self.path_match {
                (
                    project.worktree_for_id(WorktreeId::from_usize(path_match.worktree_id), cx),
                    path_match.path.join(suffix),
                )
            } else {
                (project.worktrees().next(), PathBuf::from(suffix))
            };

            worktree.and_then(|worktree| worktree.read(cx).entry_for_path(path))
        } else if let Some(path_match) = &self.path_match {
            let worktree =
                project.worktree_for_id(WorktreeId::from_usize(path_match.worktree_id), cx)?;
            worktree.read(cx).entry_for_path(path_match.path.as_ref())
        } else {
            None
        }
    }
}

pub struct NewPathDelegate {
    project: Model<Project>,
    tx: Option<oneshot::Sender<Option<PathBuf>>>,
    selected_index: usize,
    matches: Vec<Match>,
    cancel_flag: Arc<AtomicBool>,
}

impl NewPathPrompt {
    pub(crate) fn register(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        workspace.set_prompt_for_new_path(Box::new(|workspace, cx| {
            let (tx, rx) = futures::channel::oneshot::channel();
            Self::prompt_for_new_path(workspace, tx, cx);
            rx
        }));
    }

    fn prompt_for_new_path(
        workspace: &mut Workspace,
        tx: oneshot::Sender<Option<PathBuf>>,
        cx: &mut ViewContext<Workspace>,
    ) {
        let project = workspace.project().clone();
        workspace.toggle_modal(cx, |cx| {
            let delegate = NewPathDelegate {
                project,
                tx: Some(tx),
                selected_index: 0,
                matches: vec![],
                cancel_flag: Arc::new(AtomicBool::new(false)),
            };

            Picker::uniform_list(delegate, cx).width(rems(34.))
        });
    }
}

impl PickerDelegate for NewPathDelegate {
    type ListItem = ui::ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<picker::Picker<Self>>) {
        self.selected_index = ix;
        cx.notify();
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<picker::Picker<Self>>,
    ) -> gpui::Task<()> {
        let query = query.trim().trim_start_matches('/');
        let (dir, suffix) = if let Some(index) = query.rfind('/') {
            let suffix = if index + 1 < query.len() {
                Some(query[index + 1..].to_string())
            } else {
                None
            };
            (query[0..index].to_string(), suffix)
        } else {
            (query.to_string(), None)
        };

        let worktrees = self
            .project
            .read(cx)
            .visible_worktrees(cx)
            .collect::<Vec<_>>();
        let include_root_name = worktrees.len() > 1;
        let candidate_sets = worktrees
            .into_iter()
            .map(|worktree| {
                let worktree = worktree.read(cx);
                PathMatchCandidateSet {
                    snapshot: worktree.snapshot(),
                    include_ignored: worktree
                        .root_entry()
                        .map_or(false, |entry| entry.is_ignored),
                    include_root_name,
                    directories_only: true,
                }
            })
            .collect::<Vec<_>>();

        self.cancel_flag.store(true, atomic::Ordering::Relaxed);
        self.cancel_flag = Arc::new(AtomicBool::new(false));

        let cancel_flag = self.cancel_flag.clone();
        let query = query.to_string();
        let prefix = dir.clone();
        cx.spawn(|picker, mut cx| async move {
            let matches = fuzzy::match_path_sets(
                candidate_sets.as_slice(),
                &dir,
                None,
                false,
                100,
                &cancel_flag,
                cx.background_executor().clone(),
            )
            .await;
            let did_cancel = cancel_flag.load(atomic::Ordering::Relaxed);
            if did_cancel {
                return;
            }
            picker
                .update(&mut cx, |picker, cx| {
                    picker
                        .delegate
                        .set_search_matches(query, prefix, suffix, matches, cx)
                })
                .log_err();
        })
    }

    fn confirm_update_query(&mut self, cx: &mut ViewContext<Picker<Self>>) -> Option<String> {
        self.matches.get(self.selected_index).and_then(|m| {
            if m.suffix.is_none() && m.path_match.is_some() {
                Some(format!(
                    "{}/",
                    m.path_match.as_ref().unwrap().path.to_string_lossy()
                ))
            } else {
                None
            }
        })
    }

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<picker::Picker<Self>>) {
        let Some(m) = self.matches.get(self.selected_index) else {
            return;
        };

        let Some(suffix) = &m.suffix else { return };

        let abs_path = if let Some(path_match) = &m.path_match {
            let Some(worktree) = self
                .project
                .read(cx)
                .worktree_for_id(WorktreeId::from_usize(path_match.worktree_id), cx)
            else {
                cx.emit(gpui::DismissEvent);
                return;
            };

            worktree
                .read(cx)
                .abs_path()
                .join(path_match.path.as_ref())
                .join(suffix)
        } else {
            let Some(worktree) = self.project.read(cx).worktrees().next() else {
                cx.emit(gpui::DismissEvent);
                return;
            };

            worktree.read(cx).abs_path().join(suffix)
        };

        if let Some(tx) = self.tx.take() {
            tx.send(Some(abs_path)).ok();
        }
        cx.emit(gpui::DismissEvent)
    }

    fn dismissed(&mut self, cx: &mut ViewContext<picker::Picker<Self>>) {
        if let Some(tx) = self.tx.take() {
            tx.send(None).ok();
        }
        cx.emit(gpui::DismissEvent)
    }

    /// Cases to consider for each match:
    /// - if there's no path_match, then we are in the "root" directory
    /// - if there's a path_match, then we are in that directory.
    /// - if there's no suffix, then we only prompt to select the directory
    /// - if there's a suffix, and a directory exists with that name, we prompt to select it
    /// - if there's a suffix, and a file exists, we show it as a conflict.
    /// - otherwise, we prompt you to create the file called "suffix" in the directory.
    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<picker::Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let m = self.matches.get(ix)?;

        let mut suffix = m.suffix.as_ref();
        let entry = m.entry(self.project.read(cx), cx);
        let match_label = m.path_match.as_ref().map(|path_match| {
            HighlightedLabel::new(
                path_match.path.to_string_lossy().to_string(),
                path_match.positions.clone(),
            )
        });
        let suffix_dir_label = if let Some(entry) = entry {
            if entry.is_dir() {
                suffix
                    .take()
                    .map(|suffix| Label::new(suffix.to_string()).color(Color::Accent))
            } else {
                None
            }
        } else if suffix.is_some_and(|s| s.ends_with("/")) {
            // TODO: in the case that you are creating a new directory, we should highlight the existing part of the path in white.
            suffix.take().map(|suffix| {
                Label::new(suffix.trim_end_matches('/').to_string()).color(Color::Created)
            })
        } else {
            None
        };

        // TODO: used StyledText for this so the kerning doesn't change as you type
        Some(
            ListItem::new(ix).selected(selected).child(
                h_flex()
                    .child(".")
                    .when_some(match_label, |el, match_label| {
                        el.child(Label::new("/")).child(match_label)
                    })
                    .when_some(suffix_dir_label, |el, suffix_dir_label| {
                        el.child(Label::new("/")).child(suffix_dir_label)
                    })
                    .child(Label::new("/"))
                    .child(if let Some(suffix) = suffix {
                        Label::new(suffix.clone()).color(if entry.is_some() {
                            Color::Conflict
                        } else {
                            Color::Created
                        })
                    } else {
                        Label::new("[…]").color(Color::Muted)
                    }),
            ),
        )
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        Arc::from("[directory/]filename.ext")
    }
}

impl NewPathDelegate {
    fn set_search_matches(
        &mut self,
        query: String,
        prefix: String,
        suffix: Option<String>,
        matches: Vec<PathMatch>,
        cx: &mut ViewContext<Picker<Self>>,
    ) {
        cx.notify();
        if query.is_empty() {
            self.matches = vec![];
            return;
        }

        let mut directory_exists = false;

        self.matches = matches
            .into_iter()
            .map(|m| {
                if m.path.as_ref().to_string_lossy() == prefix {
                    directory_exists = true
                }
                Match {
                    path_match: Some(m),
                    suffix: suffix.clone(),
                }
            })
            .collect();

        if !directory_exists {
            if suffix.is_none() {
                self.matches.insert(
                    0,
                    Match {
                        path_match: None,
                        suffix: Some(query.clone()),
                    },
                )
            } else {
                self.matches.push(Match {
                    path_match: None,
                    suffix: Some(query.clone()),
                })
            }
        }
    }
}
