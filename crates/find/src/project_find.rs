use anyhow::Result;
use editor::{Editor, MultiBuffer};
use gpui::{
    action, elements::*, keymap::Binding, ElementBox, Entity, ModelContext, ModelHandle,
    MutableAppContext, Task, View, ViewContext, ViewHandle,
};
use project::Project;
use workspace::Workspace;

action!(Deploy);
action!(Search);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings([
        Binding::new("cmd-shift-f", Deploy, None),
        Binding::new("enter", Search, Some("ProjectFindView")),
    ]);
    cx.add_action(ProjectFindView::deploy);
    cx.add_async_action(ProjectFindView::search);
}

struct ProjectFind {
    last_search: SearchParams,
    project: ModelHandle<Project>,
    excerpts: ModelHandle<MultiBuffer>,
    pending_search: Task<Option<()>>,
}

#[derive(Default)]
struct SearchParams {
    query: String,
    regex: bool,
    whole_word: bool,
    case_sensitive: bool,
}

struct ProjectFindView {
    model: ModelHandle<ProjectFind>,
    query_editor: ViewHandle<Editor>,
    results_editor: ViewHandle<Editor>,
}

impl Entity for ProjectFind {
    type Event = ();
}

impl ProjectFind {
    fn new(project: ModelHandle<Project>, cx: &mut ModelContext<Self>) -> Self {
        let replica_id = project.read(cx).replica_id();
        Self {
            project,
            last_search: Default::default(),
            excerpts: cx.add_model(|_| MultiBuffer::new(replica_id)),
            pending_search: Task::ready(None),
        }
    }

    fn search(&mut self, params: SearchParams, cx: &mut ModelContext<Self>) {
        self.pending_search = cx.spawn_weak(|this, cx| async move {
            //
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
                Editor::for_buffer(
                    excerpts,
                    Some(workspace.project().clone()),
                    settings.clone(),
                    cx,
                )
            }),
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

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        Flex::column()
            .with_child(ChildView::new(&self.query_editor).boxed())
            .with_child(ChildView::new(&self.results_editor).boxed())
            .boxed()
    }
}

impl workspace::ItemView for ProjectFindView {
    fn item_id(&self, cx: &gpui::AppContext) -> usize {
        self.model.id()
    }

    fn tab_content(&self, style: &theme::Tab, cx: &gpui::AppContext) -> ElementBox {
        Label::new("Project Find".to_string(), style.label.clone()).boxed()
    }

    fn project_path(&self, cx: &gpui::AppContext) -> Option<project::ProjectPath> {
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

    fn can_save_as(&self, cx: &gpui::AppContext) -> bool {
        false
    }

    fn save_as(
        &mut self,
        project: ModelHandle<Project>,
        abs_path: std::path::PathBuf,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        unreachable!("save_as should not have been called")
    }
}

impl ProjectFindView {
    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        let model = cx.add_model(|cx| ProjectFind::new(workspace.project().clone(), cx));
        workspace.open_item(model, cx);
    }

    fn search(&mut self, _: &Search, cx: &mut ViewContext<Self>) -> Option<Task<Result<()>>> {
        todo!()
    }
}
