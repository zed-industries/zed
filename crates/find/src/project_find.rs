use crate::SearchOption;
use editor::{Anchor, Autoscroll, Editor, MultiBuffer};
use gpui::{
    action, elements::*, keymap::Binding, platform::CursorStyle, AppContext, ElementBox, Entity,
    ModelContext, ModelHandle, MutableAppContext, RenderContext, Task, View, ViewContext,
    ViewHandle,
};
use postage::watch;
use project::{search::SearchQuery, Project};
use std::{
    any::{Any, TypeId},
    ops::Range,
    path::PathBuf,
};
use util::ResultExt as _;
use workspace::{Item, ItemHandle, ItemNavHistory, ItemView, Settings, Workspace};

action!(Deploy, bool);
action!(Search);
action!(ToggleSearchOption, SearchOption);
action!(ToggleFocus);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([
        Binding::new("cmd-shift-F", ToggleFocus, Some("ProjectFindView")),
        Binding::new("cmd-shift-F", ToggleFocus, Some("ProjectFindView")),
        Binding::new("cmd-f", ToggleFocus, Some("ProjectFindView")),
        Binding::new("cmd-shift-F", Deploy(true), Some("Workspace")),
        Binding::new("cmd-alt-shift-F", Deploy(false), Some("Workspace")),
        Binding::new("enter", Search, Some("ProjectFindView")),
    ]);
    cx.add_action(ProjectFindView::deploy);
    cx.add_action(ProjectFindView::search);
    cx.add_action(ProjectFindView::toggle_search_option);
    cx.add_action(ProjectFindView::toggle_focus);
}

struct ProjectFind {
    project: ModelHandle<Project>,
    excerpts: ModelHandle<MultiBuffer>,
    pending_search: Option<Task<Option<()>>>,
    highlighted_ranges: Vec<Range<Anchor>>,
}

struct ProjectFindView {
    model: ModelHandle<ProjectFind>,
    query_editor: ViewHandle<Editor>,
    results_editor: ViewHandle<Editor>,
    case_sensitive: bool,
    whole_word: bool,
    regex: bool,
    query_contains_error: bool,
    settings: watch::Receiver<Settings>,
}

impl Entity for ProjectFind {
    type Event = ();
}

impl ProjectFind {
    fn new(project: ModelHandle<Project>, cx: &mut ModelContext<Self>) -> Self {
        let replica_id = project.read(cx).replica_id();
        Self {
            project,
            excerpts: cx.add_model(|_| MultiBuffer::new(replica_id)),
            pending_search: Default::default(),
            highlighted_ranges: Default::default(),
        }
    }

    fn clone(&self, new_cx: &mut ModelContext<Self>) -> Self {
        Self {
            project: self.project.clone(),
            excerpts: self
                .excerpts
                .update(new_cx, |excerpts, cx| cx.add_model(|cx| excerpts.clone(cx))),
            pending_search: Default::default(),
            highlighted_ranges: self.highlighted_ranges.clone(),
        }
    }

    fn search(&mut self, query: SearchQuery, cx: &mut ModelContext<Self>) {
        let search = self
            .project
            .update(cx, |project, cx| project.search(query.clone(), cx));
        self.highlighted_ranges.clear();
        self.pending_search = Some(cx.spawn_weak(|this, mut cx| async move {
            let matches = search.await.log_err()?;
            if let Some(this) = this.upgrade(&cx) {
                this.update(&mut cx, |this, cx| {
                    this.highlighted_ranges.clear();
                    let mut matches = matches.into_iter().collect::<Vec<_>>();
                    matches
                        .sort_by_key(|(buffer, _)| buffer.read(cx).file().map(|file| file.path()));
                    this.excerpts.update(cx, |excerpts, cx| {
                        excerpts.clear(cx);
                        for (buffer, buffer_matches) in matches {
                            let ranges_to_highlight = excerpts.push_excerpts_with_context_lines(
                                buffer,
                                buffer_matches.clone(),
                                1,
                                cx,
                            );
                            this.highlighted_ranges.extend(ranges_to_highlight);
                        }
                    });
                    this.pending_search.take();
                    cx.notify();
                });
            }
            None
        }));
        cx.notify();
    }
}

impl Item for ProjectFind {
    type View = ProjectFindView;

    fn build_view(
        model: ModelHandle<Self>,
        workspace: &Workspace,
        nav_history: ItemNavHistory,
        cx: &mut gpui::ViewContext<Self::View>,
    ) -> Self::View {
        let settings = workspace.settings();
        let excerpts = model.read(cx).excerpts.clone();
        cx.observe(&model, |this, _, cx| this.model_changed(true, cx))
            .detach();
        ProjectFindView {
            model,
            query_editor: cx.add_view(|cx| {
                Editor::single_line(
                    settings.clone(),
                    Some(|theme| theme.find.editor.input.clone()),
                    cx,
                )
            }),
            results_editor: cx.add_view(|cx| {
                let mut editor = Editor::for_buffer(
                    excerpts,
                    Some(workspace.project().clone()),
                    settings.clone(),
                    cx,
                );
                editor.set_searchable(false);
                editor.set_nav_history(Some(nav_history));
                editor
            }),
            case_sensitive: false,
            whole_word: false,
            regex: false,
            query_contains_error: false,
            settings,
        }
    }

    fn project_path(&self) -> Option<project::ProjectPath> {
        None
    }
}

impl Entity for ProjectFindView {
    type Event = ();
}

impl View for ProjectFindView {
    fn ui_name() -> &'static str {
        "ProjectFindView"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let model = &self.model.read(cx);
        let results = if model.highlighted_ranges.is_empty() {
            let theme = &self.settings.borrow().theme;
            let text = if self.query_editor.read(cx).text(cx).is_empty() {
                ""
            } else if model.pending_search.is_some() {
                "Searching..."
            } else {
                "No results"
            };
            Label::new(text.to_string(), theme.find.results_status.clone())
                .aligned()
                .contained()
                .with_background_color(theme.editor.background)
                .flexible(1., true)
                .boxed()
        } else {
            ChildView::new(&self.results_editor)
                .flexible(1., true)
                .boxed()
        };

        Flex::column()
            .with_child(self.render_query_editor(cx))
            .with_child(results)
            .boxed()
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        if self.model.read(cx).highlighted_ranges.is_empty() {
            cx.focus(&self.query_editor);
        } else {
            cx.focus(&self.results_editor);
        }
    }
}

impl ItemView for ProjectFindView {
    fn act_as_type(
        &self,
        type_id: TypeId,
        self_handle: &ViewHandle<Self>,
        _: &gpui::AppContext,
    ) -> Option<gpui::AnyViewHandle> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.into())
        } else if type_id == TypeId::of::<Editor>() {
            Some((&self.results_editor).into())
        } else {
            None
        }
    }

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        self.results_editor
            .update(cx, |editor, cx| editor.deactivated(cx));
    }

    fn item(&self, _: &gpui::AppContext) -> Box<dyn ItemHandle> {
        Box::new(self.model.clone())
    }

    fn tab_content(&self, style: &theme::Tab, _: &gpui::AppContext) -> ElementBox {
        Label::new("Project Find".to_string(), style.label.clone()).boxed()
    }

    fn project_path(&self, _: &gpui::AppContext) -> Option<project::ProjectPath> {
        None
    }

    fn can_save(&self, _: &gpui::AppContext) -> bool {
        true
    }

    fn is_dirty(&self, cx: &AppContext) -> bool {
        self.results_editor.read(cx).is_dirty(cx)
    }

    fn has_conflict(&self, cx: &AppContext) -> bool {
        self.results_editor.read(cx).has_conflict(cx)
    }

    fn save(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.results_editor
            .update(cx, |editor, cx| editor.save(project, cx))
    }

    fn can_save_as(&self, _: &gpui::AppContext) -> bool {
        false
    }

    fn save_as(
        &mut self,
        _: ModelHandle<Project>,
        _: PathBuf,
        _: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        unreachable!("save_as should not have been called")
    }

    fn clone_on_split(
        &self,
        nav_history: ItemNavHistory,
        cx: &mut ViewContext<Self>,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        let query_editor = cx.add_view(|cx| {
            let query = self.query_editor.read(cx).text(cx);
            let editor = Editor::single_line(
                self.settings.clone(),
                Some(|theme| theme.find.editor.input.clone()),
                cx,
            );
            editor
                .buffer()
                .update(cx, |buffer, cx| buffer.edit([0..0], query, cx));
            editor
        });
        let model = self
            .model
            .update(cx, |model, cx| cx.add_model(|cx| model.clone(cx)));

        cx.observe(&model, |this, _, cx| this.model_changed(true, cx))
            .detach();
        let results_editor = cx.add_view(|cx| {
            let model = model.read(cx);
            let excerpts = model.excerpts.clone();
            let project = model.project.clone();
            let scroll_position = self
                .results_editor
                .update(cx, |editor, cx| editor.scroll_position(cx));

            let mut editor = Editor::for_buffer(excerpts, Some(project), self.settings.clone(), cx);
            editor.set_searchable(false);
            editor.set_nav_history(Some(nav_history));
            editor.set_scroll_position(scroll_position, cx);
            editor
        });
        let mut view = Self {
            model,
            query_editor,
            results_editor,
            case_sensitive: self.case_sensitive,
            whole_word: self.whole_word,
            regex: self.regex,
            query_contains_error: self.query_contains_error,
            settings: self.settings.clone(),
        };
        view.model_changed(false, cx);
        Some(view)
    }

    fn navigate(&mut self, data: Box<dyn Any>, cx: &mut ViewContext<Self>) {
        self.results_editor
            .update(cx, |editor, cx| editor.navigate(data, cx));
    }
}

impl ProjectFindView {
    fn deploy(
        workspace: &mut Workspace,
        &Deploy(activate_existing): &Deploy,
        cx: &mut ViewContext<Workspace>,
    ) {
        if activate_existing {
            if let Some(existing) = workspace.item_of_type::<ProjectFind>(cx) {
                workspace.activate_item(&existing, cx);
                return;
            }
        }
        let model = cx.add_model(|cx| ProjectFind::new(workspace.project().clone(), cx));
        workspace.open_item(model, cx);
    }

    fn search(&mut self, _: &Search, cx: &mut ViewContext<Self>) {
        let text = self.query_editor.read(cx).text(cx);
        let query = if self.regex {
            match SearchQuery::regex(text, self.whole_word, self.case_sensitive) {
                Ok(query) => query,
                Err(_) => {
                    self.query_contains_error = true;
                    cx.notify();
                    return;
                }
            }
        } else {
            SearchQuery::text(text, self.whole_word, self.case_sensitive)
        };

        self.model.update(cx, |model, cx| model.search(query, cx));
    }

    fn toggle_search_option(
        &mut self,
        ToggleSearchOption(option): &ToggleSearchOption,
        cx: &mut ViewContext<Self>,
    ) {
        let value = match option {
            SearchOption::WholeWord => &mut self.whole_word,
            SearchOption::CaseSensitive => &mut self.case_sensitive,
            SearchOption::Regex => &mut self.regex,
        };
        *value = !*value;
        self.search(&Search, cx);
        cx.notify();
    }

    fn toggle_focus(&mut self, _: &ToggleFocus, cx: &mut ViewContext<Self>) {
        if self.query_editor.is_focused(cx) {
            cx.focus(&self.results_editor);
        } else {
            cx.focus(&self.query_editor);
        }
    }

    fn model_changed(&mut self, reset_selections: bool, cx: &mut ViewContext<Self>) {
        let highlighted_ranges = self.model.read(cx).highlighted_ranges.clone();
        if !highlighted_ranges.is_empty() {
            let theme = &self.settings.borrow().theme.find;
            self.results_editor.update(cx, |editor, cx| {
                editor.highlight_ranges::<Self>(highlighted_ranges, theme.match_background, cx);
                if reset_selections {
                    editor.select_ranges([0..0], Some(Autoscroll::Fit), cx);
                }
            });
            if self.query_editor.is_focused(cx) {
                cx.focus(&self.results_editor);
            }
        }

        cx.notify();
    }

    fn render_query_editor(&self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &self.settings.borrow().theme;
        let editor_container = if self.query_contains_error {
            theme.find.invalid_editor
        } else {
            theme.find.editor.input.container
        };
        Flex::row()
            .with_child(
                ChildView::new(&self.query_editor)
                    .contained()
                    .with_style(editor_container)
                    .aligned()
                    .constrained()
                    .with_max_width(theme.find.editor.max_width)
                    .boxed(),
            )
            .with_child(
                Flex::row()
                    .with_child(self.render_option_button("Case", SearchOption::CaseSensitive, cx))
                    .with_child(self.render_option_button("Word", SearchOption::WholeWord, cx))
                    .with_child(self.render_option_button("Regex", SearchOption::Regex, cx))
                    .contained()
                    .with_style(theme.find.option_button_group)
                    .aligned()
                    .boxed(),
            )
            .contained()
            .with_style(theme.find.container)
            .constrained()
            .with_height(theme.workspace.toolbar.height)
            .named("find bar")
    }

    fn render_option_button(
        &self,
        icon: &str,
        option: SearchOption,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let theme = &self.settings.borrow().theme.find;
        let is_active = self.is_option_enabled(option);
        MouseEventHandler::new::<Self, _, _>(option as usize, cx, |state, _| {
            let style = match (is_active, state.hovered) {
                (false, false) => &theme.option_button,
                (false, true) => &theme.hovered_option_button,
                (true, false) => &theme.active_option_button,
                (true, true) => &theme.active_hovered_option_button,
            };
            Label::new(icon.to_string(), style.text.clone())
                .contained()
                .with_style(style.container)
                .boxed()
        })
        .on_click(move |cx| cx.dispatch_action(ToggleSearchOption(option)))
        .with_cursor_style(CursorStyle::PointingHand)
        .boxed()
    }

    fn is_option_enabled(&self, option: SearchOption) -> bool {
        match option {
            SearchOption::WholeWord => self.whole_word,
            SearchOption::CaseSensitive => self.case_sensitive,
            SearchOption::Regex => self.regex,
        }
    }
}
