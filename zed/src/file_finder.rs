use crate::{
    editor::{buffer_view, BufferView},
    settings::Settings,
    util, watch,
    workspace::{Workspace, WorkspaceView},
    worktree::{match_paths, PathMatch, Worktree},
};
use gpui::{
    color::{ColorF, ColorU},
    elements::*,
    fonts::{Properties, Weight},
    geometry::vector::vec2f,
    keymap::{self, Binding},
    AppContext, Axis, Border, Entity, ModelHandle, MutableAppContext, View, ViewContext,
    ViewHandle, WeakViewHandle,
};
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
    workspace: ModelHandle<Workspace>,
    query_buffer: ViewHandle<BufferView>,
    search_count: usize,
    latest_search_id: usize,
    latest_search_did_cancel: bool,
    latest_search_query: String,
    matches: Vec<PathMatch>,
    selected: Option<(usize, Arc<Path>)>,
    cancel_flag: Arc<AtomicBool>,
    list_state: UniformListState,
}

pub fn init(app: &mut MutableAppContext) {
    app.add_action("file_finder:toggle", FileFinder::toggle);
    app.add_action("file_finder:confirm", FileFinder::confirm);
    app.add_action("file_finder:select", FileFinder::select);
    app.add_action("menu:select_prev", FileFinder::select_prev);
    app.add_action("menu:select_next", FileFinder::select_next);
    app.add_action("uniform_list:scroll", FileFinder::scroll);

    app.add_bindings(vec![
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
                .with_background_color(ColorU::from_u32(0xf2f2f2ff))
                .with_shadow(vec2f(0., 4.), 12., ColorF::new(0.0, 0.0, 0.0, 0.25).to_u8())
                .boxed(),
            )
            .with_max_width(600.0)
            .with_max_height(400.0)
            .boxed(),
        )
        .top()
        .named("file finder")
    }

    fn on_focus(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.focus(&self.query_buffer);
    }

    fn keymap_context(&self, _: &AppContext) -> keymap::Context {
        let mut ctx = Self::default_keymap_context();
        ctx.set.insert("menu".into());
        ctx
    }
}

impl FileFinder {
    fn render_matches(&self) -> ElementBox {
        if self.matches.is_empty() {
            let settings = smol::block_on(self.settings.read());
            return Container::new(
                Label::new(
                    "No matches".into(),
                    settings.ui_font_family,
                    settings.ui_font_size,
                )
                .boxed(),
            )
            .with_margin_top(6.0)
            .named("empty matches");
        }

        let handle = self.handle.clone();
        let list = UniformList::new(
            self.list_state.clone(),
            self.matches.len(),
            move |mut range, items, app| {
                let finder = handle.upgrade(app).unwrap();
                let finder = finder.read(app);
                let start = range.start;
                range.end = cmp::min(range.end, finder.matches.len());
                items.extend(finder.matches[range].iter().enumerate().filter_map(
                    move |(i, path_match)| finder.render_match(path_match, start + i, app),
                ));
            },
        );

        Container::new(list.boxed())
            .with_background_color(ColorU::from_u32(0xf7f7f7ff))
            .with_border(Border::all(1.0, ColorU::from_u32(0xdbdbdcff)))
            .with_margin_top(6.0)
            .named("matches")
    }

    fn render_match(
        &self,
        path_match: &PathMatch,
        index: usize,
        app: &AppContext,
    ) -> Option<ElementBox> {
        self.labels_for_match(path_match, app).map(
            |(file_name, file_name_positions, full_path, full_path_positions)| {
                let settings = smol::block_on(self.settings.read());
                let highlight_color = ColorU::from_u32(0x304ee2ff);
                let bold = *Properties::new().weight(Weight::BOLD);
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
                                        .with_highlights(highlight_color, bold, file_name_positions)
                                        .boxed(),
                                    )
                                    .with_child(
                                        Label::new(
                                            full_path,
                                            settings.ui_font_family,
                                            settings.ui_font_size,
                                        )
                                        .with_highlights(highlight_color, bold, full_path_positions)
                                        .boxed(),
                                    )
                                    .boxed(),
                            )
                            .boxed(),
                        )
                        .boxed(),
                )
                .with_uniform_padding(6.0);

                let selected_index = self.selected_index();
                if index == selected_index || index < self.matches.len() - 1 {
                    container =
                        container.with_border(Border::bottom(1.0, ColorU::from_u32(0xdbdbdcff)));
                }

                if index == selected_index {
                    container = container.with_background_color(ColorU::from_u32(0xdbdbdcff));
                }

                let entry = (path_match.tree_id, path_match.path.clone());
                EventHandler::new(container.boxed())
                    .on_mouse_down(move |ctx| {
                        ctx.dispatch_action("file_finder:select", entry.clone());
                        true
                    })
                    .named("match")
            },
        )
    }

    fn labels_for_match(
        &self,
        path_match: &PathMatch,
        app: &AppContext,
    ) -> Option<(String, Vec<usize>, String, Vec<usize>)> {
        self.worktree(path_match.tree_id, app).map(|tree| {
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

    fn toggle(workspace_view: &mut WorkspaceView, _: &(), ctx: &mut ViewContext<WorkspaceView>) {
        workspace_view.toggle_modal(ctx, |ctx, workspace_view| {
            let handle = ctx.add_view(|ctx| {
                Self::new(
                    workspace_view.settings.clone(),
                    workspace_view.workspace.clone(),
                    ctx,
                )
            });
            ctx.subscribe_to_view(&handle, Self::on_event);
            handle
        });
    }

    fn on_event(
        workspace_view: &mut WorkspaceView,
        _: ViewHandle<FileFinder>,
        event: &Event,
        ctx: &mut ViewContext<WorkspaceView>,
    ) {
        match event {
            Event::Selected(tree_id, path) => {
                workspace_view.open_entry((*tree_id, path.clone()), ctx);
                workspace_view.dismiss_modal(ctx);
            }
            Event::Dismissed => {
                workspace_view.dismiss_modal(ctx);
            }
        }
    }

    pub fn new(
        settings: watch::Receiver<Settings>,
        workspace: ModelHandle<Workspace>,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        ctx.observe(&workspace, Self::workspace_updated);

        let query_buffer = ctx.add_view(|ctx| BufferView::single_line(settings.clone(), ctx));
        ctx.subscribe_to_view(&query_buffer, Self::on_query_buffer_event);

        settings.notify_view_on_change(ctx);

        Self {
            handle: ctx.handle().downgrade(),
            settings,
            workspace,
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

    fn workspace_updated(&mut self, _: ModelHandle<Workspace>, ctx: &mut ViewContext<Self>) {
        self.spawn_search(self.query_buffer.read(ctx).text(ctx.as_ref()), ctx);
    }

    fn on_query_buffer_event(
        &mut self,
        _: ViewHandle<BufferView>,
        event: &buffer_view::Event,
        ctx: &mut ViewContext<Self>,
    ) {
        use buffer_view::Event::*;
        match event {
            Edited => {
                let query = self.query_buffer.read(ctx).text(ctx.as_ref());
                if query.is_empty() {
                    self.latest_search_id = util::post_inc(&mut self.search_count);
                    self.matches.clear();
                    ctx.notify();
                } else {
                    self.spawn_search(query, ctx);
                }
            }
            Blurred => ctx.emit(Event::Dismissed),
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

    fn select_prev(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let mut selected_index = self.selected_index();
        if selected_index > 0 {
            selected_index -= 1;
            let mat = &self.matches[selected_index];
            self.selected = Some((mat.tree_id, mat.path.clone()));
        }
        self.list_state.scroll_to(selected_index);
        ctx.notify();
    }

    fn select_next(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        let mut selected_index = self.selected_index();
        if selected_index + 1 < self.matches.len() {
            selected_index += 1;
            let mat = &self.matches[selected_index];
            self.selected = Some((mat.tree_id, mat.path.clone()));
        }
        self.list_state.scroll_to(selected_index);
        ctx.notify();
    }

    fn scroll(&mut self, _: &f32, ctx: &mut ViewContext<Self>) {
        ctx.notify();
    }

    fn confirm(&mut self, _: &(), ctx: &mut ViewContext<Self>) {
        if let Some(m) = self.matches.get(self.selected_index()) {
            ctx.emit(Event::Selected(m.tree_id, m.path.clone()));
        }
    }

    fn select(&mut self, (tree_id, path): &(usize, Arc<Path>), ctx: &mut ViewContext<Self>) {
        ctx.emit(Event::Selected(*tree_id, path.clone()));
    }

    fn spawn_search(&mut self, query: String, ctx: &mut ViewContext<Self>) {
        let snapshots = self
            .workspace
            .read(ctx)
            .worktrees()
            .iter()
            .map(|tree| tree.read(ctx).snapshot())
            .collect::<Vec<_>>();
        let search_id = util::post_inc(&mut self.search_count);
        let pool = ctx.as_ref().thread_pool().clone();
        self.cancel_flag.store(true, atomic::Ordering::Relaxed);
        self.cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_flag = self.cancel_flag.clone();
        let task = ctx.background_executor().spawn(async move {
            let include_root_name = snapshots.len() > 1;
            let matches = match_paths(
                snapshots.iter(),
                &query,
                include_root_name,
                false,
                false,
                100,
                cancel_flag.clone(),
                pool,
            );
            let did_cancel = cancel_flag.load(atomic::Ordering::Relaxed);
            (search_id, did_cancel, query, matches)
        });

        ctx.spawn(task, Self::update_matches).detach();
    }

    fn update_matches(
        &mut self,
        (search_id, did_cancel, query, matches): (usize, bool, String, Vec<PathMatch>),
        ctx: &mut ViewContext<Self>,
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
            ctx.notify();
        }
    }

    fn worktree<'a>(&'a self, tree_id: usize, app: &'a AppContext) -> Option<&'a Worktree> {
        self.workspace
            .read(app)
            .worktrees()
            .get(&tree_id)
            .map(|worktree| worktree.read(app))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        editor, settings,
        test::temp_tree,
        workspace::{Workspace, WorkspaceView},
    };
    use gpui::App;
    use serde_json::json;
    use std::fs;
    use tempdir::TempDir;

    #[test]
    fn test_matching_paths() {
        App::test_async((), |mut app| async move {
            let tmp_dir = TempDir::new("example").unwrap();
            fs::create_dir(tmp_dir.path().join("a")).unwrap();
            fs::write(tmp_dir.path().join("a/banana"), "banana").unwrap();
            fs::write(tmp_dir.path().join("a/bandana"), "bandana").unwrap();
            app.update(|ctx| {
                super::init(ctx);
                editor::init(ctx);
            });

            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let workspace = app.add_model(|ctx| Workspace::new(vec![tmp_dir.path().into()], ctx));
            let (window_id, workspace_view) =
                app.add_window(|ctx| WorkspaceView::new(workspace.clone(), settings, ctx));
            app.read(|ctx| workspace.read(ctx).worktree_scans_complete(ctx))
                .await;
            app.dispatch_action(
                window_id,
                vec![workspace_view.id()],
                "file_finder:toggle".into(),
                (),
            );

            let finder = app.read(|ctx| {
                workspace_view
                    .read(ctx)
                    .modal()
                    .cloned()
                    .unwrap()
                    .downcast::<FileFinder>()
                    .unwrap()
            });
            let query_buffer = app.read(|ctx| finder.read(ctx).query_buffer.clone());

            let chain = vec![finder.id(), query_buffer.id()];
            app.dispatch_action(window_id, chain.clone(), "buffer:insert", "b".to_string());
            app.dispatch_action(window_id, chain.clone(), "buffer:insert", "n".to_string());
            app.dispatch_action(window_id, chain.clone(), "buffer:insert", "a".to_string());
            finder
                .condition(&app, |finder, _| finder.matches.len() == 2)
                .await;

            let active_pane = app.read(|ctx| workspace_view.read(ctx).active_pane().clone());
            app.dispatch_action(
                window_id,
                vec![workspace_view.id(), finder.id()],
                "menu:select_next",
                (),
            );
            app.dispatch_action(
                window_id,
                vec![workspace_view.id(), finder.id()],
                "file_finder:confirm",
                (),
            );
            active_pane
                .condition(&app, |pane, _| pane.active_item().is_some())
                .await;
            app.read(|ctx| {
                let active_item = active_pane.read(ctx).active_item().unwrap();
                assert_eq!(active_item.title(ctx), "bandana");
            });
        });
    }

    #[test]
    fn test_matching_cancellation() {
        App::test_async((), |mut app| async move {
            let tmp_dir = temp_tree(json!({
                "hello": "",
                "goodbye": "",
                "halogen-light": "",
                "happiness": "",
                "height": "",
                "hi": "",
                "hiccup": "",
            }));
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let workspace = app.add_model(|ctx| Workspace::new(vec![tmp_dir.path().into()], ctx));
            app.read(|ctx| workspace.read(ctx).worktree_scans_complete(ctx))
                .await;
            let (_, finder) =
                app.add_window(|ctx| FileFinder::new(settings, workspace.clone(), ctx));

            let query = "hi".to_string();
            finder.update(&mut app, |f, ctx| f.spawn_search(query.clone(), ctx));
            finder.condition(&app, |f, _| f.matches.len() == 5).await;

            finder.update(&mut app, |finder, ctx| {
                let matches = finder.matches.clone();

                // Simulate a search being cancelled after the time limit,
                // returning only a subset of the matches that would have been found.
                finder.spawn_search(query.clone(), ctx);
                finder.update_matches(
                    (
                        finder.latest_search_id,
                        true, // did-cancel
                        query.clone(),
                        vec![matches[1].clone(), matches[3].clone()],
                    ),
                    ctx,
                );

                // Simulate another cancellation.
                finder.spawn_search(query.clone(), ctx);
                finder.update_matches(
                    (
                        finder.latest_search_id,
                        true, // did-cancel
                        query.clone(),
                        vec![matches[0].clone(), matches[2].clone(), matches[3].clone()],
                    ),
                    ctx,
                );

                assert_eq!(finder.matches, matches[0..4])
            });
        });
    }

    #[test]
    fn test_single_file_worktrees() {
        App::test_async((), |mut app| async move {
            let temp_dir = TempDir::new("test-single-file-worktrees").unwrap();
            let dir_path = temp_dir.path().join("the-parent-dir");
            let file_path = dir_path.join("the-file");
            fs::create_dir(&dir_path).unwrap();
            fs::write(&file_path, "").unwrap();

            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let workspace = app.add_model(|ctx| Workspace::new(vec![file_path], ctx));
            app.read(|ctx| workspace.read(ctx).worktree_scans_complete(ctx))
                .await;
            let (_, finder) =
                app.add_window(|ctx| FileFinder::new(settings, workspace.clone(), ctx));

            // Even though there is only one worktree, that worktree's filename
            // is included in the matching, because the worktree is a single file.
            finder.update(&mut app, |f, ctx| f.spawn_search("thf".into(), ctx));
            finder.condition(&app, |f, _| f.matches.len() == 1).await;

            app.read(|ctx| {
                let finder = finder.read(ctx);
                let (file_name, file_name_positions, full_path, full_path_positions) =
                    finder.labels_for_match(&finder.matches[0], ctx).unwrap();

                assert_eq!(file_name, "the-file");
                assert_eq!(file_name_positions, &[0, 1, 4]);
                assert_eq!(full_path, "the-file");
                assert_eq!(full_path_positions, &[0, 1, 4]);
            });

            // Since the worktree root is a file, searching for its name followed by a slash does
            // not match anything.
            finder.update(&mut app, |f, ctx| f.spawn_search("thf/".into(), ctx));
            finder.condition(&app, |f, _| f.matches.len() == 0).await;
        });
    }

    #[test]
    fn test_multiple_matches_with_same_relative_path() {
        App::test_async((), |mut app| async move {
            let tmp_dir = temp_tree(json!({
                "dir1": { "a.txt": "" },
                "dir2": { "a.txt": "" }
            }));
            let settings = settings::channel(&app.font_cache()).unwrap().1;
            let workspace = app.add_model(|ctx| {
                Workspace::new(
                    vec![tmp_dir.path().join("dir1"), tmp_dir.path().join("dir2")],
                    ctx,
                )
            });
            app.read(|ctx| workspace.read(ctx).worktree_scans_complete(ctx))
                .await;
            let (_, finder) =
                app.add_window(|ctx| FileFinder::new(settings, workspace.clone(), ctx));

            // Run a search that matches two files with the same relative path.
            finder.update(&mut app, |f, ctx| f.spawn_search("a.t".into(), ctx));
            finder.condition(&app, |f, _| f.matches.len() == 2).await;

            // Can switch between different matches with the same relative path.
            finder.update(&mut app, |f, ctx| {
                assert_eq!(f.selected_index(), 0);
                f.select_next(&(), ctx);
                assert_eq!(f.selected_index(), 1);
                f.select_prev(&(), ctx);
                assert_eq!(f.selected_index(), 0);
            });
        });
    }
}
