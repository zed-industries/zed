use fuzzy::PathMatch;
use gpui::{
    actions, elements::*, AppContext, ModelHandle, MouseState, Task, ViewContext, WeakViewHandle,
};
use picker::{Picker, PickerDelegate};
use project::{PathMatchCandidateSet, Project, ProjectPath, WorktreeId};
use settings::Settings;
use std::{
    path::Path,
    sync::{
        atomic::{self, AtomicBool},
        Arc,
    },
};
use util::{post_inc, ResultExt};
use workspace::Workspace;

pub type FileFinder = Picker<FileFinderDelegate>;

pub struct FileFinderDelegate {
    workspace: WeakViewHandle<Workspace>,
    project: ModelHandle<Project>,
    search_count: usize,
    latest_search_id: usize,
    latest_search_did_cancel: bool,
    latest_search_query: String,
    relative_to: Option<Arc<Path>>,
    matches: Vec<PathMatch>,
    selected: Option<(usize, Arc<Path>)>,
    cancel_flag: Arc<AtomicBool>,
}

actions!(file_finder, [Toggle]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(toggle_file_finder);
    FileFinder::init(cx);
}

fn toggle_file_finder(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
    workspace.toggle_modal(cx, |workspace, cx| {
        let relative_to = workspace
            .active_item(cx)
            .and_then(|item| item.project_path(cx))
            .map(|project_path| project_path.path.clone());
        let project = workspace.project().clone();
        let workspace = cx.handle().downgrade();
        let finder = cx.add_view(|cx| {
            Picker::new(
                FileFinderDelegate::new(workspace, project, relative_to, cx),
                cx,
            )
        });
        finder
    });
}

pub enum Event {
    Selected(ProjectPath),
    Dismissed,
}

impl FileFinderDelegate {
    fn labels_for_match(&self, path_match: &PathMatch) -> (String, Vec<usize>, String, Vec<usize>) {
        let path = &path_match.path;
        let path_string = path.to_string_lossy();
        let full_path = [path_match.path_prefix.as_ref(), path_string.as_ref()].join("");
        let path_positions = path_match.positions.clone();

        let file_name = path.file_name().map_or_else(
            || path_match.path_prefix.to_string(),
            |file_name| file_name.to_string_lossy().to_string(),
        );
        let file_name_start = path_match.path_prefix.chars().count() + path_string.chars().count()
            - file_name.chars().count();
        let file_name_positions = path_positions
            .iter()
            .filter_map(|pos| {
                if pos >= &file_name_start {
                    Some(pos - file_name_start)
                } else {
                    None
                }
            })
            .collect();

        (file_name, file_name_positions, full_path, path_positions)
    }

    pub fn new(
        workspace: WeakViewHandle<Workspace>,
        project: ModelHandle<Project>,
        relative_to: Option<Arc<Path>>,
        cx: &mut ViewContext<FileFinder>,
    ) -> Self {
        cx.observe(&project, |picker, _, cx| {
            picker.update_matches(picker.query(cx), cx);
        })
        .detach();
        Self {
            workspace,
            project,
            search_count: 0,
            latest_search_id: 0,
            latest_search_did_cancel: false,
            latest_search_query: String::new(),
            relative_to,
            matches: Vec::new(),
            selected: None,
            cancel_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    fn spawn_search(&mut self, query: String, cx: &mut ViewContext<FileFinder>) -> Task<()> {
        let relative_to = self.relative_to.clone();
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
                }
            })
            .collect::<Vec<_>>();

        let search_id = util::post_inc(&mut self.search_count);
        self.cancel_flag.store(true, atomic::Ordering::Relaxed);
        self.cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_flag = self.cancel_flag.clone();
        cx.spawn(|picker, mut cx| async move {
            let matches = fuzzy::match_path_sets(
                candidate_sets.as_slice(),
                &query,
                relative_to,
                false,
                100,
                &cancel_flag,
                cx.background(),
            )
            .await;
            let did_cancel = cancel_flag.load(atomic::Ordering::Relaxed);
            picker
                .update(&mut cx, |picker, cx| {
                    picker
                        .delegate_mut()
                        .set_matches(search_id, did_cancel, query, matches, cx)
                })
                .log_err();
        })
    }

    fn set_matches(
        &mut self,
        search_id: usize,
        did_cancel: bool,
        query: String,
        matches: Vec<PathMatch>,
        cx: &mut ViewContext<FileFinder>,
    ) {
        if search_id >= self.latest_search_id {
            self.latest_search_id = search_id;
            if self.latest_search_did_cancel && query == self.latest_search_query {
                util::extend_sorted(&mut self.matches, matches.into_iter(), 100, |a, b| b.cmp(a));
            } else {
                self.matches = matches;
            }
            self.latest_search_query = query;
            self.latest_search_did_cancel = did_cancel;
            cx.notify();
        }
    }
}

impl PickerDelegate for FileFinderDelegate {
    fn placeholder_text(&self) -> Arc<str> {
        "Search project files...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        if let Some(selected) = self.selected.as_ref() {
            for (ix, path_match) in self.matches.iter().enumerate() {
                if (path_match.worktree_id, path_match.path.as_ref())
                    == (selected.0, selected.1.as_ref())
                {
                    return ix;
                }
            }
        }
        0
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<FileFinder>) {
        let mat = &self.matches[ix];
        self.selected = Some((mat.worktree_id, mat.path.clone()));
        cx.notify();
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<FileFinder>) -> Task<()> {
        if query.is_empty() {
            self.latest_search_id = post_inc(&mut self.search_count);
            self.matches.clear();
            cx.notify();
            Task::ready(())
        } else {
            self.spawn_search(query, cx)
        }
    }

    fn confirm(&mut self, cx: &mut ViewContext<FileFinder>) {
        if let Some(m) = self.matches.get(self.selected_index()) {
            if let Some(workspace) = self.workspace.upgrade(cx) {
                let project_path = ProjectPath {
                    worktree_id: WorktreeId::from_usize(m.worktree_id),
                    path: m.path.clone(),
                };

                workspace.update(cx, |workspace, cx| {
                    workspace
                        .open_path(project_path.clone(), None, true, cx)
                        .detach_and_log_err(cx);
                    workspace.dismiss_modal(cx);
                })
            }
        }
    }

    fn dismissed(&mut self, _: &mut ViewContext<FileFinder>) {}

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &mut MouseState,
        selected: bool,
        cx: &AppContext,
    ) -> AnyElement<Picker<Self>> {
        let path_match = &self.matches[ix];
        let settings = cx.global::<Settings>();
        let style = settings.theme.picker.item.style_for(mouse_state, selected);
        let (file_name, file_name_positions, full_path, full_path_positions) =
            self.labels_for_match(path_match);
        Flex::column()
            .with_child(
                Label::new(file_name, style.label.clone()).with_highlights(file_name_positions),
            )
            .with_child(
                Label::new(full_path, style.label.clone()).with_highlights(full_path_positions),
            )
            .flex(1., false)
            .contained()
            .with_style(style.container)
            .into_any_named("match")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::Editor;
    use menu::{Confirm, SelectNext};
    use serde_json::json;
    use workspace::{AppState, Workspace};

    #[ctor::ctor]
    fn init_logger() {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::init();
        }
    }

    #[gpui::test]
    async fn test_matching_paths(cx: &mut gpui::TestAppContext) {
        let app_state = cx.update(|cx| {
            super::init(cx);
            editor::init(cx);
            AppState::test(cx)
        });

        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/root",
                json!({
                    "a": {
                        "banana": "",
                        "bandana": "",
                    }
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
        cx.dispatch_action(window_id, Toggle);

        let finder = cx.read(|cx| workspace.read(cx).modal::<FileFinder>().unwrap());
        finder
            .update(cx, |finder, cx| {
                finder.delegate_mut().update_matches("bna".to_string(), cx)
            })
            .await;
        finder.read_with(cx, |finder, _| {
            assert_eq!(finder.delegate().matches.len(), 2);
        });

        let active_pane = cx.read(|cx| workspace.read(cx).active_pane().clone());
        cx.dispatch_action(window_id, SelectNext);
        cx.dispatch_action(window_id, Confirm);
        active_pane
            .condition(cx, |pane, _| pane.active_item().is_some())
            .await;
        cx.read(|cx| {
            let active_item = active_pane.read(cx).active_item().unwrap();
            assert_eq!(
                active_item
                    .as_any()
                    .downcast_ref::<Editor>()
                    .unwrap()
                    .read(cx)
                    .title(cx),
                "bandana"
            );
        });
    }

    #[gpui::test]
    async fn test_matching_cancellation(cx: &mut gpui::TestAppContext) {
        let app_state = cx.update(AppState::test);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/dir",
                json!({
                    "hello": "",
                    "goodbye": "",
                    "halogen-light": "",
                    "happiness": "",
                    "height": "",
                    "hi": "",
                    "hiccup": "",
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/dir".as_ref()], cx).await;
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
        let (_, finder) = cx.add_window(|cx| {
            Picker::new(
                FileFinderDelegate::new(
                    workspace.downgrade(),
                    workspace.read(cx).project().clone(),
                    None,
                    cx,
                ),
                cx,
            )
        });

        let query = "hi".to_string();
        finder
            .update(cx, |f, cx| f.delegate_mut().spawn_search(query.clone(), cx))
            .await;
        finder.read_with(cx, |f, _| assert_eq!(f.delegate().matches.len(), 5));

        finder.update(cx, |finder, cx| {
            let delegate = finder.delegate_mut();
            let matches = delegate.matches.clone();

            // Simulate a search being cancelled after the time limit,
            // returning only a subset of the matches that would have been found.
            drop(delegate.spawn_search(query.clone(), cx));
            delegate.set_matches(
                delegate.latest_search_id,
                true, // did-cancel
                query.clone(),
                vec![matches[1].clone(), matches[3].clone()],
                cx,
            );

            // Simulate another cancellation.
            drop(delegate.spawn_search(query.clone(), cx));
            delegate.set_matches(
                delegate.latest_search_id,
                true, // did-cancel
                query.clone(),
                vec![matches[0].clone(), matches[2].clone(), matches[3].clone()],
                cx,
            );

            assert_eq!(delegate.matches, matches[0..4])
        });
    }

    #[gpui::test]
    async fn test_ignored_files(cx: &mut gpui::TestAppContext) {
        let app_state = cx.update(AppState::test);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/ancestor",
                json!({
                    ".gitignore": "ignored-root",
                    "ignored-root": {
                        "happiness": "",
                        "height": "",
                        "hi": "",
                        "hiccup": "",
                    },
                    "tracked-root": {
                        ".gitignore": "height",
                        "happiness": "",
                        "height": "",
                        "hi": "",
                        "hiccup": "",
                    },
                }),
            )
            .await;

        let project = Project::test(
            app_state.fs.clone(),
            [
                "/ancestor/tracked-root".as_ref(),
                "/ancestor/ignored-root".as_ref(),
            ],
            cx,
        )
        .await;
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
        let (_, finder) = cx.add_window(|cx| {
            Picker::new(
                FileFinderDelegate::new(
                    workspace.downgrade(),
                    workspace.read(cx).project().clone(),
                    None,
                    cx,
                ),
                cx,
            )
        });
        finder
            .update(cx, |f, cx| f.delegate_mut().spawn_search("hi".into(), cx))
            .await;
        finder.read_with(cx, |f, _| assert_eq!(f.delegate().matches.len(), 7));
    }

    #[gpui::test]
    async fn test_single_file_worktrees(cx: &mut gpui::TestAppContext) {
        let app_state = cx.update(AppState::test);
        app_state
            .fs
            .as_fake()
            .insert_tree("/root", json!({ "the-parent-dir": { "the-file": "" } }))
            .await;

        let project = Project::test(
            app_state.fs.clone(),
            ["/root/the-parent-dir/the-file".as_ref()],
            cx,
        )
        .await;
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
        let (_, finder) = cx.add_window(|cx| {
            Picker::new(
                FileFinderDelegate::new(
                    workspace.downgrade(),
                    workspace.read(cx).project().clone(),
                    None,
                    cx,
                ),
                cx,
            )
        });

        // Even though there is only one worktree, that worktree's filename
        // is included in the matching, because the worktree is a single file.
        finder
            .update(cx, |f, cx| f.delegate_mut().spawn_search("thf".into(), cx))
            .await;
        cx.read(|cx| {
            let finder = finder.read(cx);
            let delegate = finder.delegate();
            assert_eq!(delegate.matches.len(), 1);

            let (file_name, file_name_positions, full_path, full_path_positions) =
                delegate.labels_for_match(&delegate.matches[0]);
            assert_eq!(file_name, "the-file");
            assert_eq!(file_name_positions, &[0, 1, 4]);
            assert_eq!(full_path, "the-file");
            assert_eq!(full_path_positions, &[0, 1, 4]);
        });

        // Since the worktree root is a file, searching for its name followed by a slash does
        // not match anything.
        finder
            .update(cx, |f, cx| f.delegate_mut().spawn_search("thf/".into(), cx))
            .await;
        finder.read_with(cx, |f, _| assert_eq!(f.delegate().matches.len(), 0));
    }

    #[gpui::test]
    async fn test_multiple_matches_with_same_relative_path(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();

        let app_state = cx.update(AppState::test);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/root",
                json!({
                    "dir1": { "a.txt": "" },
                    "dir2": { "a.txt": "" }
                }),
            )
            .await;

        let project = Project::test(
            app_state.fs.clone(),
            ["/root/dir1".as_ref(), "/root/dir2".as_ref()],
            cx,
        )
        .await;
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));

        let (_, finder) = cx.add_window(|cx| {
            Picker::new(
                FileFinderDelegate::new(
                    workspace.downgrade(),
                    workspace.read(cx).project().clone(),
                    None,
                    cx,
                ),
                cx,
            )
        });

        // Run a search that matches two files with the same relative path.
        finder
            .update(cx, |f, cx| f.delegate_mut().spawn_search("a.t".into(), cx))
            .await;

        // Can switch between different matches with the same relative path.
        finder.update(cx, |finder, cx| {
            let delegate = finder.delegate_mut();
            assert_eq!(delegate.matches.len(), 2);
            assert_eq!(delegate.selected_index(), 0);
            delegate.set_selected_index(1, cx);
            assert_eq!(delegate.selected_index(), 1);
            delegate.set_selected_index(0, cx);
            assert_eq!(delegate.selected_index(), 0);
        });
    }

    #[gpui::test]
    async fn test_path_distance_ordering(cx: &mut gpui::TestAppContext) {
        cx.foreground().forbid_parking();

        let app_state = cx.update(AppState::test);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/root",
                json!({
                    "dir1": { "a.txt": "" },
                    "dir2": {
                        "a.txt": "",
                        "b.txt": ""
                    }
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));

        // When workspace has an active item, sort items which are closer to that item
        // first when they have the same name. In this case, b.txt is closer to dir2's a.txt
        // so that one should be sorted earlier
        let b_path = Some(Arc::from(Path::new("/root/dir2/b.txt")));
        let (_, finder) = cx.add_window(|cx| {
            Picker::new(
                FileFinderDelegate::new(
                    workspace.downgrade(),
                    workspace.read(cx).project().clone(),
                    b_path,
                    cx,
                ),
                cx,
            )
        });

        finder
            .update(cx, |f, cx| {
                f.delegate_mut().spawn_search("a.txt".into(), cx)
            })
            .await;

        finder.read_with(cx, |f, _| {
            let delegate = f.delegate();
            assert_eq!(delegate.matches[0].path.as_ref(), Path::new("dir2/a.txt"));
            assert_eq!(delegate.matches[1].path.as_ref(), Path::new("dir1/a.txt"));
        });
    }

    #[gpui::test]
    async fn test_search_worktree_without_files(cx: &mut gpui::TestAppContext) {
        let app_state = cx.update(AppState::test);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/root",
                json!({
                    "dir1": {},
                    "dir2": {
                        "dir3": {}
                    }
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
        let (_, finder) = cx.add_window(|cx| {
            Picker::new(
                FileFinderDelegate::new(
                    workspace.downgrade(),
                    workspace.read(cx).project().clone(),
                    None,
                    cx,
                ),
                cx,
            )
        });
        finder
            .update(cx, |f, cx| f.delegate_mut().spawn_search("dir".into(), cx))
            .await;
        cx.read(|cx| {
            let finder = finder.read(cx);
            assert_eq!(finder.delegate().matches.len(), 0);
        });
    }
}
