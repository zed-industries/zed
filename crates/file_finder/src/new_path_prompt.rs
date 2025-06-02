use futures::channel::oneshot;
use fuzzy::PathMatch;
use gpui::{Entity, HighlightStyle, StyledText};
use picker::{Picker, PickerDelegate};
use project::{DirectoryLister, Entry, PathMatchCandidateSet, Project, ProjectPath, WorktreeId};
use std::{
    path::{MAIN_SEPARATOR_STR, Path, PathBuf},
    sync::{
        Arc,
        atomic::{self, AtomicBool},
    },
};
use ui::{Context, ListItem, Window};
use ui::{LabelLike, ListItemSpacing, highlight_ranges, prelude::*};
use util::{ResultExt, maybe};
use workspace::Workspace;

pub(crate) struct NewPathPrompt;

#[derive(Debug, Clone)]
struct Match {
    path_match: Option<PathMatch>,
    suffix: Option<String>,
}

const DIR_INDICATOR: &str = "[…]";

impl Match {
    fn entry<'a>(&'a self, project: &'a Project, cx: &'a App) -> Option<&'a Entry> {
        if let Some(suffix) = &self.suffix {
            let (worktree, path) = if let Some(path_match) = &self.path_match {
                (
                    project.worktree_for_id(WorktreeId::from_usize(path_match.worktree_id), cx),
                    path_match.path.join(suffix),
                )
            } else {
                (project.worktrees(cx).next(), PathBuf::from(suffix))
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

    fn is_dir(&self, project: &Project, cx: &App) -> bool {
        self.entry(project, cx).is_some_and(|e| e.is_dir())
            || self.suffix.as_ref().is_some_and(|s| s.ends_with('/'))
    }

    fn relative_path(&self, lister: &DirectoryLister) -> String {
        if let Some(path_match) = &self.path_match {
            if let Some(suffix) = &self.suffix {
                format!(
                    "{}/{}",
                    path_match.path.to_string_lossy(),
                    suffix.trim_end_matches('/')
                )
            } else {
                path_match.path.to_string_lossy().to_string()
            }
        } else if let Some(suffix) = &self.suffix {
            suffix.trim_end_matches('/').to_string()
        } else {
            "".to_string()
        }
    }

    fn project_path(
        &self,
        project: &Project,
        lister: &DirectoryLister,
        cx: &App,
    ) -> Option<ProjectPath> {
        let worktree_id = if let Some(path_match) = &self.path_match {
            WorktreeId::from_usize(path_match.worktree_id)
        } else if let Some(worktree) = project.visible_worktrees(cx).find(|worktree| {
            worktree
                .read(cx)
                .root_entry()
                .is_some_and(|entry| entry.is_dir())
        }) {
            worktree.read(cx).id()
        } else {
            // TODO kb
            // todo(): we should find_or_create a workspace.
            return None;
        };

        let path = PathBuf::from(self.relative_path(lister));

        Some(ProjectPath {
            worktree_id,
            path: Arc::from(path),
        })
    }

    fn existing_prefix(&self, project: &Project, cx: &App) -> Option<PathBuf> {
        let worktree = project.worktrees(cx).next()?.read(cx);
        let mut prefix = PathBuf::new();
        let parts = self.suffix.as_ref()?.split('/');
        for part in parts {
            if worktree.entry_for_path(prefix.join(&part)).is_none() {
                return Some(prefix);
            }
            prefix = prefix.join(part);
        }

        None
    }

    fn styled_text(&self, project: &Project, window: &Window, cx: &App) -> StyledText {
        let mut text = "./".to_string();
        let mut highlights = Vec::new();
        let mut offset = text.len();

        let separator = '/';

        if let Some(path_match) = &self.path_match {
            text.push_str(&path_match.path.to_string_lossy());
            let mut whole_path = PathBuf::from(path_match.path_prefix.to_string());
            whole_path = whole_path.join(path_match.path.clone());
            for (range, style) in highlight_ranges(
                &whole_path.to_string_lossy(),
                &path_match.positions,
                gpui::HighlightStyle::color(Color::Accent.color(cx)),
            ) {
                highlights.push((range.start + offset..range.end + offset, style))
            }
            text.push(separator);
            offset = text.len();

            if let Some(suffix) = &self.suffix {
                text.push_str(suffix);
                let entry = self.entry(project, cx);
                let color = if let Some(entry) = entry {
                    if entry.is_dir() {
                        Color::Accent
                    } else {
                        Color::Conflict
                    }
                } else {
                    Color::Created
                };
                highlights.push((
                    offset..offset + suffix.len(),
                    HighlightStyle::color(color.color(cx)),
                ));
                offset += suffix.len();
                if entry.is_some_and(|e| e.is_dir()) {
                    text.push(separator);
                    offset += separator.len_utf8();

                    text.push_str(DIR_INDICATOR);
                    highlights.push((
                        offset..offset + DIR_INDICATOR.len(),
                        HighlightStyle::color(Color::Muted.color(cx)),
                    ));
                }
            } else {
                text.push_str(DIR_INDICATOR);
                highlights.push((
                    offset..offset + DIR_INDICATOR.len(),
                    HighlightStyle::color(Color::Muted.color(cx)),
                ))
            }
        } else if let Some(suffix) = &self.suffix {
            text.push_str(suffix);
            let existing_prefix_len = self
                .existing_prefix(project, cx)
                .map(|prefix| prefix.to_string_lossy().len())
                .unwrap_or(0);

            if existing_prefix_len > 0 {
                highlights.push((
                    offset..offset + existing_prefix_len,
                    HighlightStyle::color(Color::Accent.color(cx)),
                ));
            }
            highlights.push((
                offset + existing_prefix_len..offset + suffix.len(),
                HighlightStyle::color(if self.entry(project, cx).is_some() {
                    Color::Conflict.color(cx)
                } else {
                    Color::Created.color(cx)
                }),
            ));
            offset += suffix.len();
            if suffix.ends_with('/') {
                text.push_str(DIR_INDICATOR);
                highlights.push((
                    offset..offset + DIR_INDICATOR.len(),
                    HighlightStyle::color(Color::Muted.color(cx)),
                ));
            }
        }

        StyledText::new(text).with_default_highlights(&window.text_style().clone(), highlights)
    }
}

pub struct NewPathDelegate {
    tx: Option<oneshot::Sender<Option<PathBuf>>>,
    lister: DirectoryLister,
    selected_index: usize,
    directory_state: Option<DirectoryState>,
    matches: Vec<Match>,
    cancel_flag: Arc<AtomicBool>,
    should_dismiss: bool,
    last_selected_dir: Option<String>,
    project: Entity<Project>,
}

impl NewPathPrompt {
    pub(crate) fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _cx: &mut Context<Workspace>,
    ) {
        workspace.set_prompt_for_new_path(Box::new(|workspace, lister, window, cx| {
            let (tx, rx) = futures::channel::oneshot::channel();
            Self::prompt_for_new_path(workspace, lister, tx, window, cx);
            rx
        }));
    }

    fn prompt_for_new_path(
        workspace: &mut Workspace,
        lister: DirectoryLister,
        tx: oneshot::Sender<Option<PathBuf>>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let project = workspace.project().clone();
        workspace.toggle_modal(window, cx, |window, cx| {
            let delegate = NewPathDelegate {
                project,
                lister: lister.clone(),
                tx: Some(tx),
                selected_index: 0,
                matches: Vec::new(),
                cancel_flag: Arc::new(AtomicBool::new(false)),
                last_selected_dir: None,
                should_dismiss: true,
            };

            let picker = Picker::uniform_list(delegate, window, cx).width(rems(34.));
            let query = lister.default_query(cx);
            picker.set_query(query, window, cx);
            picker
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

    fn set_selected_index(
        &mut self,
        ix: usize,
        _: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) {
        self.selected_index = ix;
        cx.notify();
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<picker::Picker<Self>>,
    ) -> gpui::Task<()> {
        let query = query
            .trim()
            .trim_start_matches("./")
            .trim_start_matches('/');

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
                    candidates: project::Candidates::Directories,
                }
            })
            .collect::<Vec<_>>();

        self.cancel_flag.store(true, atomic::Ordering::Relaxed);
        self.cancel_flag = Arc::new(AtomicBool::new(false));

        let cancel_flag = self.cancel_flag.clone();
        let query = query.to_string();
        let prefix = dir.clone();
        cx.spawn_in(window, async move |picker, cx| {
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
                .update(cx, |picker, cx| {
                    picker
                        .delegate
                        .set_search_matches(query, prefix, suffix, matches, cx)
                })
                .log_err();
        })
    }

    fn confirm_completion(
        &mut self,
        query: String,
        window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<String> {
        Some(
            maybe!({
                let m = self.matches.get(self.selected_index)?;
                let directory_state = self.directory_state.as_ref()?;
                let candidate = directory_state.match_candidates.get(*m)?;
                Some(format!(
                    "{}{}{}{}",
                    directory_state.path,
                    candidate.path.string,
                    if candidate.is_dir {
                        MAIN_SEPARATOR_STR
                    } else {
                        ""
                    },
                    if candidate.is_dir { DIR_INDICATOR } else { "" }
                ))
            })
            .unwrap_or(query),
        )
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<picker::Picker<Self>>) {
        let Some(m) = self.matches.get(self.selected_index) else {
            return;
        };

        let exists = m.entry(self.project.read(cx), cx).is_some();
        if exists {
            self.should_dismiss = false;
            let answer = window.prompt(
                gpui::PromptLevel::Critical,
                &format!("{} already exists. Do you want to replace it?", m.relative_path()),
                Some(
                    "A file or folder with the same name already exists. Replacing it will overwrite its current contents.",
                ),
                &["Replace", "Cancel"],
            cx);
            let m = m.clone();
            cx.spawn_in(window, async move |picker, cx| {
                let answer = answer.await.ok();
                picker
                    .update(cx, |picker, cx| {
                        picker.delegate.should_dismiss = true;
                        if answer != Some(0) {
                            return;
                        }
                        if let Some(path) = m.project_path(picker.delegate.project.read(cx), cx) {
                            if let Some(tx) = picker.delegate.tx.take() {
                                tx.send(Some(path)).ok();
                            }
                        }
                        cx.emit(gpui::DismissEvent);
                    })
                    .ok();
            })
            .detach();
            return;
        }

        if let Some(path) = m.project_path(self.project.read(cx), cx) {
            if let Some(tx) = self.tx.take() {
                tx.send(Some(path)).ok();
            }
        }
        cx.emit(gpui::DismissEvent);
    }

    fn should_dismiss(&self) -> bool {
        self.should_dismiss
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<picker::Picker<Self>>) {
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
        cx: &mut Context<picker::Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let m = self.matches.get(ix)?;

        Some(
            ListItem::new(ix)
                .spacing(ListItemSpacing::Sparse)
                .inset(true)
                .toggle_state(selected)
                .child(LabelLike::new().child(m.styled_text(self.project.read(cx), window, cx))),
        )
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("Type a path...".into())
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        Arc::from(format!("[directory{MAIN_SEPARATOR_STR}]filename.ext"))
    }
}

impl NewPathDelegate {
    fn set_search_matches(
        &mut self,
        query: String,
        prefix: String,
        suffix: Option<String>,
        matches: Vec<PathMatch>,
        cx: &mut Context<Picker<Self>>,
    ) {
        cx.notify();
        if query.is_empty() {
            self.matches = self
                .project
                .read(cx)
                .worktrees(cx)
                .flat_map(|worktree| {
                    let worktree_id = worktree.read(cx).id();
                    worktree
                        .read(cx)
                        .child_entries(Path::new(""))
                        .filter_map(move |entry| {
                            entry.is_dir().then(|| Match {
                                path_match: Some(PathMatch {
                                    score: 1.0,
                                    positions: Default::default(),
                                    worktree_id: worktree_id.to_usize(),
                                    path: entry.path.clone(),
                                    path_prefix: "".into(),
                                    is_dir: entry.is_dir(),
                                    distance_to_relative_ancestor: 0,
                                }),
                                suffix: None,
                            })
                        })
                })
                .collect();

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
            if suffix.is_none()
                || self
                    .last_selected_dir
                    .as_ref()
                    .is_some_and(|d| query.starts_with(d))
            {
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
