use editor::{Anchor, Autoscroll, Editor, MultiBuffer};
use gpui::{
    action, elements::*, keymap::Binding, platform::CursorStyle, ElementBox, Entity, ModelContext,
    ModelHandle, MutableAppContext, RenderContext, Task, View, ViewContext, ViewHandle,
};
use postage::watch;
use project::{search::SearchQuery, Project};
use std::{any::TypeId, ops::Range};
use workspace::{Settings, Workspace};

use crate::SearchOption;

action!(Deploy);
action!(Search);
action!(ToggleSearchOption, SearchOption);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([
        Binding::new("cmd-shift-F", Deploy, None),
        Binding::new("enter", Search, Some("ProjectFindView")),
    ]);
    cx.add_action(ProjectFindView::deploy);
    cx.add_action(ProjectFindView::search);
    cx.add_action(ProjectFindView::toggle_search_option);
}

struct ProjectFind {
    project: ModelHandle<Project>,
    excerpts: ModelHandle<MultiBuffer>,
    pending_search: Task<Option<()>>,
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
            pending_search: Task::ready(None),
            highlighted_ranges: Default::default(),
        }
    }

    fn search(&mut self, query: SearchQuery, cx: &mut ModelContext<Self>) {
        let search = self
            .project
            .update(cx, |project, cx| project.search(query, cx));
        self.pending_search = cx.spawn_weak(|this, mut cx| async move {
            let matches = search.await;
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
                    cx.notify();
                });
            }
            None
        });
    }
}

impl workspace::Item for ProjectFind {
    type View = ProjectFindView;

    fn build_view(
        model: ModelHandle<Self>,
        workspace: &workspace::Workspace,
        nav_history: workspace::ItemNavHistory,
        cx: &mut gpui::ViewContext<Self::View>,
    ) -> Self::View {
        let settings = workspace.settings();
        let excerpts = model.read(cx).excerpts.clone();
        cx.observe(&model, ProjectFindView::on_model_changed)
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
        Flex::column()
            .with_child(self.render_query_editor(cx))
            .with_child(
                ChildView::new(&self.results_editor)
                    .flexible(1., true)
                    .boxed(),
            )
            .boxed()
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.query_editor);
    }
}

impl workspace::ItemView for ProjectFindView {
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

    fn item_id(&self, _: &gpui::AppContext) -> usize {
        self.model.id()
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
        _: std::path::PathBuf,
        _: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        unreachable!("save_as should not have been called")
    }
}

impl ProjectFindView {
    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        let model = cx.add_model(|cx| ProjectFind::new(workspace.project().clone(), cx));
        workspace.open_item(model, cx);
    }

    fn search(&mut self, _: &Search, cx: &mut ViewContext<Self>) {
        let text = self.query_editor.read(cx).text(cx);
        let query = if self.regex {
            match SearchQuery::regex(text, self.case_sensitive, self.whole_word) {
                Ok(query) => query,
                Err(_) => {
                    self.query_contains_error = true;
                    cx.notify();
                    return;
                }
            }
        } else {
            SearchQuery::text(text, self.case_sensitive, self.whole_word)
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

    fn on_model_changed(&mut self, _: ModelHandle<ProjectFind>, cx: &mut ViewContext<Self>) {
        let theme = &self.settings.borrow().theme.find;
        self.results_editor.update(cx, |editor, cx| {
            let model = self.model.read(cx);
            editor.highlight_ranges::<Self>(
                model.highlighted_ranges.clone(),
                theme.match_background,
                cx,
            );
            editor.select_ranges([0..0], Some(Autoscroll::Fit), cx);
        });
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
