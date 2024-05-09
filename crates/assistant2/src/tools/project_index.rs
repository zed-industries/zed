use anyhow::Result;
use assistant_tooling::{LanguageModelTool, ToolOutput};
use collections::BTreeMap;
use gpui::{prelude::*, Model, Task};
use project::ProjectPath;
use schemars::JsonSchema;
use semantic_index::{ProjectIndex, Status};
use serde::{Deserialize, Serialize};
use std::{fmt::Write as _, ops::Range, path::Path, sync::Arc};
use ui::{prelude::*, CollapsibleContainer, Color, Icon, IconName, Label, WindowContext};

const DEFAULT_SEARCH_LIMIT: usize = 20;

pub struct ProjectIndexTool {
    project_index: Model<ProjectIndex>,
}

pub struct ProjectIndexView {
    error: Option<anyhow::Error>,
    project_index: Model<ProjectIndex>,
    input: CodebaseQuery,
    status: Status,
    excerpts: BTreeMap<ProjectPath, Vec<Range<usize>>>,
    element_id: ElementId,
    expanded_header: bool,
}

#[derive(Default, Deserialize, JsonSchema)]
pub struct CodebaseQuery {
    /// Semantic search query
    query: String,
    /// Maximum number of results to return, defaults to 20
    limit: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectIndexOutput {
    status: Status,
    worktrees: BTreeMap<Arc<Path>, WorktreeIndexOutput>,
}

#[derive(Default, Serialize, Deserialize)]
struct WorktreeIndexOutput {
    excerpts: BTreeMap<Arc<Path>, Vec<Range<usize>>>,
}

impl ProjectIndexView {
    fn toggle_header(&mut self, cx: &mut ViewContext<Self>) {
        self.expanded_header = !self.expanded_header;
        cx.notify();
    }
}

impl Render for ProjectIndexView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if let Some(error) = &self.error {
            return format!("failed to search: {error:?}").into_any_element();
        }

        let query = self.input.query.clone();
        let file_count = self.excerpts.len();
        let header = h_flex()
            .gap_2()
            .child(Icon::new(IconName::File))
            .child(format!(
                "Read {} {}",
                file_count,
                if file_count == 1 { "file" } else { "files" }
            ));

        v_flex()
            .gap_3()
            .child(
                CollapsibleContainer::new(self.element_id.clone(), self.expanded_header)
                    .start_slot(header)
                    .on_click(cx.listener(move |this, _, cx| {
                        this.toggle_header(cx);
                    }))
                    .child(
                        v_flex()
                            .gap_3()
                            .p_3()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(Icon::new(IconName::MagnifyingGlass))
                                    .child(Label::new(format!("`{}`", query)).color(Color::Muted)),
                            )
                            .child(v_flex().gap_2().children(self.excerpts.keys().map(|path| {
                                h_flex().gap_2().child(Icon::new(IconName::File)).child(
                                    Label::new(path.path.to_string_lossy().to_string())
                                        .color(Color::Muted),
                                )
                            }))),
                    ),
            )
            .into_any_element()
    }
}

impl ToolOutput for ProjectIndexView {
    type Input = CodebaseQuery;
    type SerializedState = ProjectIndexOutput;

    fn generate(
        &self,
        context: &mut assistant_tooling::ProjectContext,
        _: &mut ViewContext<Self>,
    ) -> String {
        if let Some(error) = &self.error {
            return format!("failed to search: {error:?}");
        }

        let mut body = "found results in the following paths:\n".to_string();

        for (project_path, ranges) in &self.excerpts {
            context.add_excerpts(project_path.clone(), ranges);
            writeln!(&mut body, "* {}", &project_path.path.display()).unwrap();
        }

        if self.status != Status::Idle {
            body.push_str("Still indexing. Results may be incomplete.\n");
        }

        body
    }

    fn set_input(&mut self, input: Self::Input, cx: &mut ViewContext<Self>) {
        self.input = input;
        cx.notify();
    }

    fn execute(&mut self, cx: &mut ViewContext<Self>) -> Task<Result<()>> {
        let project_index = self.project_index.read(cx);
        let status = project_index.status();
        let search = project_index.search(
            self.input.query.clone(),
            self.input.limit.unwrap_or(DEFAULT_SEARCH_LIMIT),
            cx,
        );

        cx.spawn(|this, mut cx| async move {
            let search_result = search.await;
            this.update(&mut cx, |this, cx| {
                match search_result {
                    Ok(search_results) => {
                        this.status = status;
                        for search_result in search_results {
                            let project_path = ProjectPath {
                                worktree_id: search_result.worktree.read(cx).id(),
                                path: search_result.path,
                            };
                            this.excerpts
                                .entry(project_path)
                                .or_default()
                                .push(search_result.range);
                        }
                    }
                    Err(error) => {
                        this.error = Some(error);
                    }
                }
                cx.notify();
            })
        })
    }

    fn serialize(&self, cx: &mut ViewContext<Self>) -> Self::SerializedState {
        let mut state = ProjectIndexOutput {
            status: self.status,
            worktrees: Default::default(),
        };

        if let Some(project) = self.project_index.read(cx).project().upgrade() {
            let project = project.read(cx);
            for (project_path, excerpts) in &self.excerpts {
                if let Some(worktree) = project.worktree_for_id(project_path.worktree_id, cx) {
                    let worktree_path = worktree.read(cx).abs_path();
                    state
                        .worktrees
                        .entry(worktree_path)
                        .or_default()
                        .excerpts
                        .insert(project_path.path.clone(), excerpts.clone());
                }
            }
        }

        state
    }

    fn deserialize(
        &mut self,
        output: Self::SerializedState,
        cx: &mut ViewContext<Self>,
    ) -> Result<()> {
        self.status = output.status;
        self.excerpts.clear();
        if let Some(project) = self.project_index.read(cx).project().upgrade() {
            let project = project.read(cx);
            for (worktree_path, state) in output.worktrees {
                if let Some(worktree) = project
                    .worktrees()
                    .find(|worktree| worktree.read(cx).abs_path() == worktree_path)
                {
                    let worktree_id = worktree.read(cx).id();
                    for (path, excerpts) in state.excerpts {
                        self.excerpts
                            .insert(ProjectPath { worktree_id, path }, excerpts);
                    }
                }
            }
        }

        Ok(())
    }
}

impl ProjectIndexTool {
    pub fn new(project_index: Model<ProjectIndex>) -> Self {
        Self { project_index }
    }
}

impl LanguageModelTool for ProjectIndexTool {
    type View = ProjectIndexView;

    fn name(&self) -> String {
        "query_codebase".to_string()
    }

    fn description(&self) -> String {
        "Semantic search against the user's current codebase, returning excerpts related to the query by computing a dot product against embeddings of code chunks in the code base and an embedding of the query.".to_string()
    }

    fn view(&self, cx: &mut WindowContext) -> gpui::View<Self::View> {
        cx.new_view(|_| ProjectIndexView {
            error: None,
            input: Default::default(),
            status: Status::Idle,
            excerpts: Default::default(),
            element_id: ElementId::Name(nanoid::nanoid!().into()),
            expanded_header: false,
            project_index: self.project_index.clone(),
        })
    }
}
