use crate::{
    editor::{self, Editor},
    settings::Settings,
    util,
    workspace::Workspace,
    worktree::{match_paths, PathMatch, Worktree},
};
use gpui::{
    color::ColorF,
    elements::*,
    fonts::{Properties, Weight},
    geometry::vector::vec2f,
    keymap::{self, Binding},
    AppContext, Axis, Border, Entity, MutableAppContext, Task, View, ViewContext, ViewHandle,
    WeakViewHandle,
};
use postage::watch;
use std::{
    cmp,
    path::Path,
    sync::{
        atomic::{self, AtomicBool},
        Arc,
    },
};

pub struct FileFinder {
    handle: WeakViewHandle<Self>,
    settings: watch::Receiver<Settings>,
    workspace: WeakViewHandle<Workspace>,
    query_buffer: ViewHandle<Editor>,
    search_count: usize,
    latest_search_id: usize,
    latest_search_did_cancel: bool,
    latest_search_query: String,
    matches: Vec<PathMatch>,
    selected: Option<(usize, Arc<Path>)>,
    cancel_flag: Arc<AtomicBool>,
    list_state: UniformListState,
}

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action("file_finder:toggle", FileFinder::toggle);
    cx.add_action("file_finder:confirm", FileFinder::confirm);
    cx.add_action("file_finder:select", FileFinder::select);
    cx.add_action("menu:select_prev", FileFinder::select_prev);
    cx.add_action("menu:select_next", FileFinder::select_next);
    cx.add_action("uniform_list:scroll", FileFinder::scroll);

    cx.add_bindings(vec![
        Binding::new("cmd-p", "file_finder:toggle", None),
        Binding::new("escape", "file_finder:toggle", Some("FileFinder")),
        Binding::new("enter", "file_finder:confirm", Some("FileFinder")),
    ]);
}

pub enum Event {
    Selected(usize, Arc<Path>),
    Dismissed,
}

impl Entity for FileFinder {
    type Event = Event;
}

impl View for FileFinder {
    fn ui_name() -> &'static str {
        "FileFinder"
    }

    fn render(&self, _: &AppContext) -> ElementBox {
        let settings = self.settings.borrow();

        Align::new(
            ConstrainedBox::new(
                Container::new(
                    Flex::new(Axis::Vertical)
                        .with_child(ChildView::new(self.query_buffer.id()).boxed())
                        .with_child(Expanded::new(1.0, self.render_matches()).boxed())
                        .boxed(),
                )
                .with_margin_top(12.0)
                .with_uniform_padding(6.0)
                .with_corner_radius(6.0)
                .with_background_color(settings.theme.ui.modal_background)
                .with_shadow(vec2f(0., 4.), 12., ColorF::new(0.0, 0.0, 0.0, 0.5).to_u8())
                .boxed(),
            )
            .with_max_width(600.0)
            .with_max_height(400.0)
            .boxed(),
        )
        .top()
        .named("file finder")
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.query_buffer);
    }

    fn keymap_context(&self, _: &AppContext) -> keymap::Context {
        let mut cx = Self::default_keymap_context();
        cx.set.insert("menu".into());
        cx
    }
}

impl FileFinder {
    fn render_matches(&self) -> ElementBox {
        if self.matches.is_empty() {
            let settings = self.settings.borrow();
            return Container::new(
                Label::new(
                    "No matches".into(),
                    settings.ui_font_family,
                    settings.ui_font_size,
                )
                .with_default_color(settings.theme.editor.default_text.0)
                .boxed(),
            )
            .with_margin_top(6.0)
            .named("empty matches");
        }

        let handle = self.handle.clone();
        let list = UniformList::new(
            self.list_state.clone(),
            self.matches.len(),
            move |mut range, items, cx| {
                let cx = cx.as_ref();
                let finder = handle.upgrade(cx).unwrap();
                let finder = finder.read(cx);
                let start = range.start;
                range.end = cmp::min(range.end, finder.matches.len());
                items.extend(finder.matches[range].iter().enumerate().filter_map(
                    move |(i, path_match)| finder.render_match(path_match, start + i, cx),
                ));
            },
        );

        Container::new(list.boxed())
            .with_margin_top(6.0)
            .named("matches")
    }

    fn render_match(
        &self,
        path_match: &PathMatch,
        index: usize,
        cx: &AppContext,
    ) -> Option<ElementBox> {
        let settings = self.settings.borrow();
        let theme = &settings.theme.ui;
        self.labels_for_match(path_match, cx).map(
            |(file_name, file_name_positions, full_path, full_path_positions)| {
                let bold = *Properties::new().weight(Weight::BOLD);
                let selected_index = self.selected_index();
                let mut container = Container::new(
                    Flex::row()
                        .with_child(
                            Container::new(
                                LineBox::new(
                                    settings.ui_font_family,
                                    settings.ui_font_size,
                                    Svg::new("icons/file-16.svg").boxed(),
                                )
                                .boxed(),
                            )
                            .with_padding_right(6.0)
                            .boxed(),
                        )
                        .with_child(
                            Expanded::new(
                                1.0,
                                Flex::column()
                                    .with_child(
                                        Label::new(
                                            file_name.to_string(),
                                            settings.ui_font_family,
                                            settings.ui_font_size,
                                        )
                                        .with_default_color(theme.modal_match_text.0)
                                        .with_highlights(
                                            theme.modal_match_text_highlight.0,
                                            bold,
                                            file_name_positions,
                                        )
                                        .boxed(),
                                    )
                                    .with_child(
                                        Label::new(
                                            full_path,
                                            settings.ui_font_family,
                                            settings.ui_font_size,
                                        )
                                        .with_default_color(theme.modal_match_text.0)
                                        .with_highlights(
                                            theme.modal_match_text_highlight.0,
                                            bold,
                                            full_path_positions,
                                        )
                                        .boxed(),
                                    )
                                    .boxed(),
                            )
                            .boxed(),
                        )
                        .boxed(),
                )
                .with_uniform_padding(6.0)
                .with_background_color(if index == selected_index {
                    theme.modal_match_background_active.0
                } else {
                    theme.modal_match_background.0
                });

                if index == selected_index || index < self.matches.len() - 1 {
                    container =
                        container.with_border(Border::bottom(1.0, theme.modal_match_border));
                }

                let entry = (path_match.tree_id, path_match.path.clone());
                EventHandler::new(container.boxed())
                    .on_mouse_down(move |cx| {
                        cx.dispatch_action("file_finder:select", entry.clone());
                        true
                    })
                    .named("match")
            },
        )
    }

    fn labels_for_match(
        &self,
        path_match: &PathMatch,
        cx: &AppContext,
    ) -> Option<(String, Vec<usize>, String, Vec<usize>)> {
        self.worktree(path_match.tree_id, cx).map(|tree| {
            let prefix = if path_match.include_root_name {
                tree.root_name()
            } else {
                ""
            };

            let path_string = path_match.path.to_string_lossy();
            let full_path = [prefix, path_string.as_ref()].join("");
            let path_positions = path_match.positions.clone();

            let file_name = path_match.path.file_name().map_or_else(
                || prefix.to_string(),
                |file_name| file_name.to_string_lossy().to_string(),
            );
            let file_name_start =
                prefix.chars().count() + path_string.chars().count() - file_name.chars().count();
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
        })
    }

    fn toggle(workspace_view: &mut Workspace, _: &(), cx: &mut ViewContext<Workspace>) {
        workspace_view.toggle_modal(cx, |cx, workspace_view| {
            let workspace = cx.handle();
            let finder =
                cx.add_view(|cx| Self::new(workspace_view.settings.clone(), workspace, cx));
            cx.subscribe_to_view(&finder, Self::on_event);
            finder
        });
    }

    fn on_event(
        workspace_view: &mut Workspace,
        _: ViewHandle<FileFinder>,
        event: &Event,
        cx: &mut ViewContext<Workspace>,
    ) {
        match event {
            Event::Selected(tree_id, path) => {
                workspace_view
                    .open_entry((*tree_id, path.clone()), cx)
                    .map(|d| d.detach());
                workspace_view.dismiss_modal(cx);
            }
            Event::Dismissed => {
                workspace_view.dismiss_modal(cx);
            }
        }
    }

    pub fn new(
        settings: watch::Receiver<Settings>,
        workspace: ViewHandle<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.observe_view(&workspace, Self::workspace_updated);

        let query_buffer = cx.add_view(|cx| Editor::single_line(settings.clone(), cx));
        cx.subscribe_to_view(&query_buffer, Self::on_query_editor_event);

        Self {
            handle: cx.handle().downgrade(),
            settings,
            workspace: workspace.downgrade(),
            query_buffer,
            search_count: 0,
            latest_search_id: 0,
            latest_search_did_cancel: false,
            latest_search_query: String::new(),
            matches: Vec::new(),
            selected: None,
            cancel_flag: Arc::new(AtomicBool::new(false)),
            list_state: UniformListState::new(),
        }
    }

    fn workspace_updated(&mut self, _: ViewHandle<Workspace>, cx: &mut ViewContext<Self>) {
        let query = self.query_buffer.update(cx, |buffer, cx| buffer.text(cx));
        if let Some(task) = self.spawn_search(query, cx) {
            task.detach();
        }
    }

    fn on_query_editor_event(
        &mut self,
        _: ViewHandle<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::Event::Edited => {
                let query = self.query_buffer.update(cx, |buffer, cx| buffer.text(cx));
                if query.is_empty() {
                    self.latest_search_id = util::post_inc(&mut self.search_count);
                    self.matches.clear();
                    cx.notify();
                } else {
                    if let Some(task) = self.spawn_search(query, cx) {
                        task.detach();
                    }
                }
            }
            editor::Event::Blurred => cx.emit(Event::Dismissed),
            _ => {}
        }
    }

    fn selected_index(&self) -> usize {
        if let Some(selected) = self.selected.as_ref() {
            for (ix, path_match) in self.matches.iter().enumerate() {
                if (path_match.tree_id, path_match.path.as_ref())
                    == (selected.0, selected.1.as_ref())
                {
                    return ix;
                }
            }
        }
        0
    }

    fn select_prev(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let mut selected_index = self.selected_index();
        if selected_index > 0 {
            selected_index -= 1;
            let mat = &self.matches[selected_index];
            self.selected = Some((mat.tree_id, mat.path.clone()));
        }
        self.list_state.scroll_to(selected_index);
        cx.notify();
    }

    fn select_next(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        let mut selected_index = self.selected_index();
        if selected_index + 1 < self.matches.len() {
            selected_index += 1;
            let mat = &self.matches[selected_index];
            self.selected = Some((mat.tree_id, mat.path.clone()));
        }
        self.list_state.scroll_to(selected_index);
        cx.notify();
    }

    fn scroll(&mut self, _: &f32, cx: &mut ViewContext<Self>) {
        cx.notify();
    }

    fn confirm(&mut self, _: &(), cx: &mut ViewContext<Self>) {
        if let Some(m) = self.matches.get(self.selected_index()) {
            cx.emit(Event::Selected(m.tree_id, m.path.clone()));
        }
    }

    fn select(&mut self, (tree_id, path): &(usize, Arc<Path>), cx: &mut ViewContext<Self>) {
        cx.emit(Event::Selected(*tree_id, path.clone()));
    }

    #[must_use]
    fn spawn_search(&mut self, query: String, cx: &mut ViewContext<Self>) -> Option<Task<()>> {
        let snapshots = self
            .workspace
            .upgrade(&cx)?
            .read(cx)
            .worktrees()
            .iter()
            .map(|tree| tree.read(cx).snapshot())
            .collect::<Vec<_>>();
        let search_id = util::post_inc(&mut self.search_count);
        let background = cx.as_ref().background().clone();
        self.cancel_flag.store(true, atomic::Ordering::Relaxed);
        self.cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_flag = self.cancel_flag.clone();
        Some(cx.spawn(|this, mut cx| async move {
            let include_root_name = snapshots.len() > 1;
            let matches = match_paths(
                snapshots.iter(),
                &query,
                include_root_name,
                false,
                false,
                100,
                cancel_flag.clone(),
                background,
            )
            .await;
            let did_cancel = cancel_flag.load(atomic::Ordering::Relaxed);
            this.update(&mut cx, |this, cx| {
                this.update_matches((search_id, did_cancel, query, matches), cx)
            });
        }))
    }

    fn update_matches(
        &mut self,
        (search_id, did_cancel, query, matches): (usize, bool, String, Vec<PathMatch>),
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
            self.list_state.scroll_to(self.selected_index());
            cx.notify();
        }
    }

    fn worktree<'a>(&'a self, tree_id: usize, cx: &'a AppContext) -> Option<&'a Worktree> {
        self.workspace
            .upgrade(cx)?
            .read(cx)
            .worktrees()
            .get(&tree_id)
            .map(|worktree| worktree.read(cx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        editor,
        fs::FakeFs,
        test::{build_app_state, temp_tree},
        workspace::Workspace,
    };
    use serde_json::json;
    use std::fs;
    use tempdir::TempDir;

    #[gpui::test]
    async fn test_matching_paths(mut cx: gpui::TestAppContext) {
        let tmp_dir = TempDir::new("example").unwrap();
        fs::create_dir(tmp_dir.path().join("a")).unwrap();
        fs::write(tmp_dir.path().join("a/banana"), "banana").unwrap();
        fs::write(tmp_dir.path().join("a/bandana"), "bandana").unwrap();
        cx.update(|cx| {
            super::init(cx);
            editor::init(cx);
        });

        let app_state = cx.read(build_app_state);
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(&app_state, cx));
        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.add_worktree(tmp_dir.path(), cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;
        cx.dispatch_action(
            window_id,
            vec![workspace.id()],
            "file_finder:toggle".into(),
            (),
        );

        let finder = cx.read(|cx| {
            workspace
                .read(cx)
                .modal()
                .cloned()
                .unwrap()
                .downcast::<FileFinder>()
                .unwrap()
        });
        let query_buffer = cx.read(|cx| finder.read(cx).query_buffer.clone());

        let chain = vec![finder.id(), query_buffer.id()];
        cx.dispatch_action(window_id, chain.clone(), "buffer:insert", "b".to_string());
        cx.dispatch_action(window_id, chain.clone(), "buffer:insert", "n".to_string());
        cx.dispatch_action(window_id, chain.clone(), "buffer:insert", "a".to_string());
        finder
            .condition(&cx, |finder, _| finder.matches.len() == 2)
            .await;

        let active_pane = cx.read(|cx| workspace.read(cx).active_pane().clone());
        cx.dispatch_action(
            window_id,
            vec![workspace.id(), finder.id()],
            "menu:select_next",
            (),
        );
        cx.dispatch_action(
            window_id,
            vec![workspace.id(), finder.id()],
            "file_finder:confirm",
            (),
        );
        active_pane
            .condition(&cx, |pane, _| pane.active_item().is_some())
            .await;
        cx.read(|cx| {
            let active_item = active_pane.read(cx).active_item().unwrap();
            assert_eq!(active_item.title(cx), "bandana");
        });
    }

    #[gpui::test]
    async fn test_matching_cancellation(mut cx: gpui::TestAppContext) {
        let fs = Arc::new(FakeFs::new());
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

        let mut app_state = cx.read(build_app_state);
        Arc::get_mut(&mut app_state).unwrap().fs = fs;

        let (_, workspace) = cx.add_window(|cx| Workspace::new(&app_state, cx));
        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.add_worktree("/dir".as_ref(), cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;
        let (_, finder) =
            cx.add_window(|cx| FileFinder::new(app_state.settings.clone(), workspace.clone(), cx));

        let query = "hi".to_string();
        finder
            .update(&mut cx, |f, cx| f.spawn_search(query.clone(), cx))
            .unwrap()
            .await;
        finder.read_with(&cx, |f, _| assert_eq!(f.matches.len(), 5));

        finder.update(&mut cx, |finder, cx| {
            let matches = finder.matches.clone();

            // Simulate a search being cancelled after the time limit,
            // returning only a subset of the matches that would have been found.
            finder.spawn_search(query.clone(), cx).unwrap().detach();
            finder.update_matches(
                (
                    finder.latest_search_id,
                    true, // did-cancel
                    query.clone(),
                    vec![matches[1].clone(), matches[3].clone()],
                ),
                cx,
            );

            // Simulate another cancellation.
            finder.spawn_search(query.clone(), cx).unwrap().detach();
            finder.update_matches(
                (
                    finder.latest_search_id,
                    true, // did-cancel
                    query.clone(),
                    vec![matches[0].clone(), matches[2].clone(), matches[3].clone()],
                ),
                cx,
            );

            assert_eq!(finder.matches, matches[0..4])
        });
    }

    #[gpui::test]
    async fn test_single_file_worktrees(mut cx: gpui::TestAppContext) {
        let temp_dir = TempDir::new("test-single-file-worktrees").unwrap();
        let dir_path = temp_dir.path().join("the-parent-dir");
        let file_path = dir_path.join("the-file");
        fs::create_dir(&dir_path).unwrap();
        fs::write(&file_path, "").unwrap();

        let app_state = cx.read(build_app_state);
        let (_, workspace) = cx.add_window(|cx| Workspace::new(&app_state, cx));
        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.add_worktree(&file_path, cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;
        let (_, finder) =
            cx.add_window(|cx| FileFinder::new(app_state.settings.clone(), workspace.clone(), cx));

        // Even though there is only one worktree, that worktree's filename
        // is included in the matching, because the worktree is a single file.
        finder
            .update(&mut cx, |f, cx| f.spawn_search("thf".into(), cx))
            .unwrap()
            .await;
        cx.read(|cx| {
            let finder = finder.read(cx);
            assert_eq!(finder.matches.len(), 1);

            let (file_name, file_name_positions, full_path, full_path_positions) =
                finder.labels_for_match(&finder.matches[0], cx).unwrap();
            assert_eq!(file_name, "the-file");
            assert_eq!(file_name_positions, &[0, 1, 4]);
            assert_eq!(full_path, "the-file");
            assert_eq!(full_path_positions, &[0, 1, 4]);
        });

        // Since the worktree root is a file, searching for its name followed by a slash does
        // not match anything.
        finder
            .update(&mut cx, |f, cx| f.spawn_search("thf/".into(), cx))
            .unwrap()
            .await;
        finder.read_with(&cx, |f, _| assert_eq!(f.matches.len(), 0));
    }

    #[gpui::test(retries = 5)]
    async fn test_multiple_matches_with_same_relative_path(mut cx: gpui::TestAppContext) {
        let tmp_dir = temp_tree(json!({
            "dir1": { "a.txt": "" },
            "dir2": { "a.txt": "" }
        }));

        let app_state = cx.read(build_app_state);
        let (_, workspace) = cx.add_window(|cx| Workspace::new(&app_state, cx));

        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.open_paths(
                    &[tmp_dir.path().join("dir1"), tmp_dir.path().join("dir2")],
                    cx,
                )
            })
            .await;
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;

        let (_, finder) =
            cx.add_window(|cx| FileFinder::new(app_state.settings.clone(), workspace.clone(), cx));

        // Run a search that matches two files with the same relative path.
        finder
            .update(&mut cx, |f, cx| f.spawn_search("a.t".into(), cx))
            .unwrap()
            .await;

        // Can switch between different matches with the same relative path.
        finder.update(&mut cx, |f, cx| {
            assert_eq!(f.matches.len(), 2);
            assert_eq!(f.selected_index(), 0);
            f.select_next(&(), cx);
            assert_eq!(f.selected_index(), 1);
            f.select_prev(&(), cx);
            assert_eq!(f.selected_index(), 0);
        });
    }
}
