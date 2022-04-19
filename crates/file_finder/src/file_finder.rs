use fuzzy::PathMatch;
use gpui::{
    actions, elements::*, AppContext, Entity, ModelHandle, MutableAppContext, RenderContext, Task,
    View, ViewContext, ViewHandle,
};
use picker::{Picker, PickerDelegate};
use project::{Project, ProjectPath, WorktreeId};
use settings::Settings;
use std::{
    path::Path,
    sync::{
        atomic::{self, AtomicBool},
        Arc,
    },
};
use util::post_inc;
use workspace::Workspace;

pub struct FileFinder {
    project: ModelHandle<Project>,
    picker: ViewHandle<Picker<Self>>,
    search_count: usize,
    latest_search_id: usize,
    latest_search_did_cancel: bool,
    latest_search_query: String,
    matches: Vec<PathMatch>,
    selected: Option<(usize, Arc<Path>)>,
    cancel_flag: Arc<AtomicBool>,
}

actions!(file_finder, [Toggle]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(FileFinder::toggle);
    Picker::<FileFinder>::init(cx);
}

pub enum Event {
    Selected(ProjectPath),
    Dismissed,
}

impl Entity for FileFinder {
    type Event = Event;
}

impl View for FileFinder {
    fn ui_name() -> &'static str {
        "FileFinder"
    }

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        ChildView::new(self.picker.clone()).boxed()
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.picker);
    }
}

impl FileFinder {
    fn labels_for_match(&self, path_match: &PathMatch) -> (String, Vec<usize>, String, Vec<usize>) {
        let path_string = path_match.path.to_string_lossy();
        let full_path = [path_match.path_prefix.as_ref(), path_string.as_ref()].join("");
        let path_positions = path_match.positions.clone();

        let file_name = path_match.path.file_name().map_or_else(
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

    fn toggle(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
        workspace.toggle_modal(cx, |cx, workspace| {
            let project = workspace.project().clone();
            let finder = cx.add_view(|cx| Self::new(project, cx));
            cx.subscribe(&finder, Self::on_event).detach();
            finder
        });
    }

    fn on_event(
        workspace: &mut Workspace,
        _: ViewHandle<FileFinder>,
        event: &Event,
        cx: &mut ViewContext<Workspace>,
    ) {
        match event {
            Event::Selected(project_path) => {
                workspace
                    .open_path(project_path.clone(), cx)
                    .detach_and_log_err(cx);
                workspace.dismiss_modal(cx);
            }
            Event::Dismissed => {
                workspace.dismiss_modal(cx);
            }
        }
    }

    pub fn new(project: ModelHandle<Project>, cx: &mut ViewContext<Self>) -> Self {
        let handle = cx.weak_handle();
        cx.observe(&project, Self::project_updated).detach();
        Self {
            project,
            picker: cx.add_view(|cx| Picker::new(handle, cx)),
            search_count: 0,
            latest_search_id: 0,
            latest_search_did_cancel: false,
            latest_search_query: String::new(),
            matches: Vec::new(),
            selected: None,
            cancel_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    fn project_updated(&mut self, _: ModelHandle<Project>, cx: &mut ViewContext<Self>) {
        self.spawn_search(self.latest_search_query.clone(), cx)
            .detach();
    }

    fn spawn_search(&mut self, query: String, cx: &mut ViewContext<Self>) -> Task<()> {
        let search_id = util::post_inc(&mut self.search_count);
        self.cancel_flag.store(true, atomic::Ordering::Relaxed);
        self.cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_flag = self.cancel_flag.clone();
        let project = self.project.clone();
        cx.spawn(|this, mut cx| async move {
            let matches = project
                .read_with(&cx, |project, cx| {
                    project.match_paths(&query, false, false, 100, cancel_flag.as_ref(), cx)
                })
                .await;
            let did_cancel = cancel_flag.load(atomic::Ordering::Relaxed);
            this.update(&mut cx, |this, cx| {
                this.set_matches(search_id, did_cancel, query, matches, cx)
            });
        })
    }

    fn set_matches(
        &mut self,
        search_id: usize,
        did_cancel: bool,
        query: String,
        matches: Vec<PathMatch>,
        cx: &mut ViewContext<Self>,
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
            self.picker.update(cx, |_, cx| cx.notify());
        }
    }
}

impl PickerDelegate for FileFinder {
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

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Self>) {
        let mat = &self.matches[ix];
        self.selected = Some((mat.worktree_id, mat.path.clone()));
        cx.notify();
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Self>) -> Task<()> {
        if query.is_empty() {
            self.latest_search_id = post_inc(&mut self.search_count);
            self.matches.clear();
            cx.notify();
            Task::ready(())
        } else {
            self.spawn_search(query, cx)
        }
    }

    fn confirm(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(m) = self.matches.get(self.selected_index()) {
            cx.emit(Event::Selected(ProjectPath {
                worktree_id: WorktreeId::from_usize(m.worktree_id),
                path: m.path.clone(),
            }));
        }
    }

    fn dismiss(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismissed);
    }

    fn render_match(&self, ix: usize, selected: bool, cx: &AppContext) -> ElementBox {
        let path_match = &self.matches[ix];
        let settings = cx.global::<Settings>();
        let style = if selected {
            &settings.theme.selector.active_item
        } else {
            &settings.theme.selector.item
        };
        let (file_name, file_name_positions, full_path, full_path_positions) =
            self.labels_for_match(path_match);
        Flex::column()
            .with_child(
                Label::new(file_name.to_string(), style.label.clone())
                    .with_highlights(file_name_positions)
                    .boxed(),
            )
            .with_child(
                Label::new(full_path, style.label.clone())
                    .with_highlights(full_path_positions)
                    .boxed(),
            )
            .flex(1., false)
            .contained()
            .with_style(style.container)
            .named("match")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::{Editor, Input};
    use serde_json::json;
    use std::path::PathBuf;
    use workspace::menu::{Confirm, SelectNext};
    use workspace::{Workspace, WorkspaceParams};

    #[ctor::ctor]
    fn init_logger() {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::init();
        }
    }

    #[gpui::test]
    async fn test_matching_paths(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            super::init(cx);
            editor::init(cx);
        });

        let params = cx.update(WorkspaceParams::test);
        params
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

        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));
        params
            .project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root", true, cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;
        cx.dispatch_action(window_id, Toggle);

        let finder = cx.read(|cx| {
            workspace
                .read(cx)
                .modal()
                .cloned()
                .unwrap()
                .downcast::<FileFinder>()
                .unwrap()
        });
        cx.dispatch_action(window_id, Input("b".into()));
        cx.dispatch_action(window_id, Input("n".into()));
        cx.dispatch_action(window_id, Input("a".into()));
        finder
            .condition(&cx, |finder, _| finder.matches.len() == 2)
            .await;

        let active_pane = cx.read(|cx| workspace.read(cx).active_pane().clone());
        cx.dispatch_action(window_id, SelectNext);
        cx.dispatch_action(window_id, Confirm);
        active_pane
            .condition(&cx, |pane, _| pane.active_item().is_some())
            .await;
        cx.read(|cx| {
            let active_item = active_pane.read(cx).active_item().unwrap();
            assert_eq!(
                active_item
                    .to_any()
                    .downcast::<Editor>()
                    .unwrap()
                    .read(cx)
                    .title(cx),
                "bandana"
            );
        });
    }

    #[gpui::test]
    async fn test_matching_cancellation(cx: &mut gpui::TestAppContext) {
        let params = cx.update(WorkspaceParams::test);
        let fs = params.fs.as_fake();
        fs.insert_tree(
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

        let (_, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));
        params
            .project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/dir", true, cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;
        let (_, finder) =
            cx.add_window(|cx| FileFinder::new(workspace.read(cx).project().clone(), cx));

        let query = "hi".to_string();
        finder
            .update(cx, |f, cx| f.spawn_search(query.clone(), cx))
            .await;
        finder.read_with(cx, |f, _| assert_eq!(f.matches.len(), 5));

        finder.update(cx, |finder, cx| {
            let matches = finder.matches.clone();

            // Simulate a search being cancelled after the time limit,
            // returning only a subset of the matches that would have been found.
            finder.spawn_search(query.clone(), cx).detach();
            finder.set_matches(
                finder.latest_search_id,
                true, // did-cancel
                query.clone(),
                vec![matches[1].clone(), matches[3].clone()],
                cx,
            );

            // Simulate another cancellation.
            finder.spawn_search(query.clone(), cx).detach();
            finder.set_matches(
                finder.latest_search_id,
                true, // did-cancel
                query.clone(),
                vec![matches[0].clone(), matches[2].clone(), matches[3].clone()],
                cx,
            );

            assert_eq!(finder.matches, matches[0..4])
        });
    }

    #[gpui::test]
    async fn test_single_file_worktrees(cx: &mut gpui::TestAppContext) {
        let params = cx.update(WorkspaceParams::test);
        params
            .fs
            .as_fake()
            .insert_tree("/root", json!({ "the-parent-dir": { "the-file": "" } }))
            .await;

        let (_, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));
        params
            .project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root/the-parent-dir/the-file", true, cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;
        let (_, finder) =
            cx.add_window(|cx| FileFinder::new(workspace.read(cx).project().clone(), cx));

        // Even though there is only one worktree, that worktree's filename
        // is included in the matching, because the worktree is a single file.
        finder
            .update(cx, |f, cx| f.spawn_search("thf".into(), cx))
            .await;
        cx.read(|cx| {
            let finder = finder.read(cx);
            assert_eq!(finder.matches.len(), 1);

            let (file_name, file_name_positions, full_path, full_path_positions) =
                finder.labels_for_match(&finder.matches[0]);
            assert_eq!(file_name, "the-file");
            assert_eq!(file_name_positions, &[0, 1, 4]);
            assert_eq!(full_path, "the-file");
            assert_eq!(full_path_positions, &[0, 1, 4]);
        });

        // Since the worktree root is a file, searching for its name followed by a slash does
        // not match anything.
        finder
            .update(cx, |f, cx| f.spawn_search("thf/".into(), cx))
            .await;
        finder.read_with(cx, |f, _| assert_eq!(f.matches.len(), 0));
    }

    #[gpui::test(retries = 5)]
    async fn test_multiple_matches_with_same_relative_path(cx: &mut gpui::TestAppContext) {
        let params = cx.update(WorkspaceParams::test);
        params
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

        let (_, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));

        workspace
            .update(cx, |workspace, cx| {
                workspace.open_paths(
                    &[PathBuf::from("/root/dir1"), PathBuf::from("/root/dir2")],
                    cx,
                )
            })
            .await;
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;

        let (_, finder) =
            cx.add_window(|cx| FileFinder::new(workspace.read(cx).project().clone(), cx));

        // Run a search that matches two files with the same relative path.
        finder
            .update(cx, |f, cx| f.spawn_search("a.t".into(), cx))
            .await;

        // Can switch between different matches with the same relative path.
        finder.update(cx, |f, cx| {
            assert_eq!(f.matches.len(), 2);
            assert_eq!(f.selected_index(), 0);
            f.set_selected_index(1, cx);
            assert_eq!(f.selected_index(), 1);
            f.set_selected_index(0, cx);
            assert_eq!(f.selected_index(), 0);
        });
    }
}
