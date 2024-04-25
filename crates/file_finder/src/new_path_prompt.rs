use futures::channel::oneshot;
use fuzzy::PathMatch;
use gpui::Model;
use picker::{highlighted_match_with_paths::HighlightedText, Picker, PickerDelegate};
use project::{PathMatchCandidateSet, Project};
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

struct Match {
    path: Arc<Path>,
    path_positions: Vec<usize>,
    suffix: Option<String>,
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
        let (dir, suffix) = if let Some(index) = query.find('/') {
            (
                query[0..index].to_string(),
                Some(query[index + 1..].to_string()),
            )
        } else {
            (query, None)
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
                    picker.delegate.set_search_matches(suffix, matches, cx)
                })
                .log_err();
        })
    }

    fn confirm(&mut self, secondary: bool, cx: &mut ViewContext<picker::Picker<Self>>) {
        todo!()
    }

    fn dismissed(&mut self, cx: &mut ViewContext<picker::Picker<Self>>) {
        if let Some(tx) = self.tx.take() {
            tx.send(None).ok();
        }
        cx.emit(gpui::DismissEvent)
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<picker::Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let m = self.matches.get(ix)?;

        Some(
            ListItem::new(ix).child(
                h_flex()
                    .child(HighlightedLabel::new(
                        m.path.to_string_lossy().to_string(),
                        m.path_positions.clone(),
                    ))
                    .child(div().child(Label::new("/")).mr_1())
                    .child(if let Some(suffix) = &m.suffix {
                        Label::new(suffix.clone()).color(Color::Created)
                    } else {
                        Label::new("[…]").color(Color::Muted)
                    }),
            ),
        )
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        Arc::from("boop")
    }
}

impl NewPathDelegate {
    fn set_search_matches(
        &mut self,
        suffix: Option<String>,
        matches: Vec<PathMatch>,
        cx: &mut ViewContext<Picker<Self>>,
    ) {
        self.matches = matches
            .into_iter()
            .map(|m| Match {
                path: m.path.clone(),
                path_positions: m.positions,
                suffix: suffix.clone(),
            })
            .collect();
    }
}
