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

#[derive(Default)]
enum ProjectIndexToolState {
    #[default]
    CollectingQuery,
    Searching,
    Error(anyhow::Error),
    Finished {
        excerpts: BTreeMap<ProjectPath, Vec<Range<usize>>>,
        index_status: Status,
    },
}

pub struct ProjectIndexView {
    project_index: Model<ProjectIndex>,
    input: CodebaseQuery,
    expanded_header: bool,
    state: ProjectIndexToolState,
}

#[derive(Default, Deserialize, JsonSchema)]
pub struct CodebaseQuery {
    /// Semantic search query
    query: String,
    /// Maximum number of results to return, defaults to 20
    limit: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct SerializedState {
    index_status: Status,
    error_message: Option<String>,
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
        let query = self.input.query.clone();

        let (header_text, content) = match &self.state {
            ProjectIndexToolState::Error(error) => {
                return format!("failed to search: {error:?}").into_any_element()
            }
            ProjectIndexToolState::CollectingQuery | ProjectIndexToolState::Searching => {
                ("Searching...".to_string(), div())
            }
            ProjectIndexToolState::Finished { excerpts, .. } => {
                let file_count = excerpts.len();

                let header_text = format!(
                    "Read {} {}",
                    file_count,
                    if file_count == 1 { "file" } else { "files" }
                );

                let el = v_flex().gap_2().children(excerpts.keys().map(|path| {
                    h_flex().gap_2().child(Icon::new(IconName::File)).child(
                        Label::new(path.path.to_string_lossy().to_string()).color(Color::Muted),
                    )
                }));

                (header_text, el)
            }
        };

        let header = h_flex()
            .gap_2()
            .child(Icon::new(IconName::File))
            .child(header_text);

        v_flex()
            .gap_3()
            .child(
                CollapsibleContainer::new("collapsible-container", self.expanded_header)
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
                            .child(content),
                    ),
            )
            .into_any_element()
    }
}

impl ToolOutput for ProjectIndexView {
    type Input = CodebaseQuery;
    type SerializedState = SerializedState;

    fn generate(
        &self,
        context: &mut assistant_tooling::ProjectContext,
        _: &mut ViewContext<Self>,
    ) -> String {
        match &self.state {
            ProjectIndexToolState::CollectingQuery => String::new(),
            ProjectIndexToolState::Searching => String::new(),
            ProjectIndexToolState::Error(error) => format!("failed to search: {error:?}"),
            ProjectIndexToolState::Finished {
                excerpts,
                index_status,
            } => {
                let mut body = "found results in the following paths:\n".to_string();

                for (project_path, ranges) in excerpts {
                    context.add_excerpts(project_path.clone(), ranges);
                    writeln!(&mut body, "* {}", &project_path.path.display()).unwrap();
                }

                if *index_status != Status::Idle {
                    body.push_str("Still indexing. Results may be incomplete.\n");
                }

                body
            }
        }
    }

    fn set_input(&mut self, input: Self::Input, cx: &mut ViewContext<Self>) {
        self.input = input;
        cx.notify();
    }

    fn execute(&mut self, cx: &mut ViewContext<Self>) -> Task<Result<()>> {
        self.state = ProjectIndexToolState::Searching;
        cx.notify();

        let project_index = self.project_index.read(cx);
        let index_status = project_index.status();
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
                        let mut excerpts = BTreeMap::<ProjectPath, Vec<Range<usize>>>::new();
                        for search_result in search_results {
                            let project_path = ProjectPath {
                                worktree_id: search_result.worktree.read(cx).id(),
                                path: search_result.path,
                            };
                            excerpts
                                .entry(project_path)
                                .or_default()
                                .push(search_result.range);
                        }
                        this.state = ProjectIndexToolState::Finished {
                            excerpts,
                            index_status,
                        };
                    }
                    Err(error) => {
                        this.state = ProjectIndexToolState::Error(error);
                    }
                }
                cx.notify();
            })
        })
    }

    fn serialize(&self, cx: &mut ViewContext<Self>) -> Self::SerializedState {
        let mut serialized = SerializedState {
            error_message: None,
            index_status: Status::Idle,
            worktrees: Default::default(),
        };
        match &self.state {
            ProjectIndexToolState::Error(err) => serialized.error_message = Some(err.to_string()),
            ProjectIndexToolState::Finished {
                excerpts,
                index_status,
            } => {
                serialized.index_status = *index_status;
                if let Some(project) = self.project_index.read(cx).project().upgrade() {
                    let project = project.read(cx);
                    for (project_path, excerpts) in excerpts {
                        if let Some(worktree) =
                            project.worktree_for_id(project_path.worktree_id, cx)
                        {
                            let worktree_path = worktree.read(cx).abs_path();
                            serialized
                                .worktrees
                                .entry(worktree_path)
                                .or_default()
                                .excerpts
                                .insert(project_path.path.clone(), excerpts.clone());
                        }
                    }
                }
            }
            _ => {}
        }
        serialized
    }

    fn deserialize(
        &mut self,
        serialized: Self::SerializedState,
        cx: &mut ViewContext<Self>,
    ) -> Result<()> {
        if !serialized.worktrees.is_empty() {
            let mut excerpts = BTreeMap::<ProjectPath, Vec<Range<usize>>>::new();
            if let Some(project) = self.project_index.read(cx).project().upgrade() {
                let project = project.read(cx);
                for (worktree_path, worktree_state) in serialized.worktrees {
                    if let Some(worktree) = project
                        .worktrees()
                        .find(|worktree| worktree.read(cx).abs_path() == worktree_path)
                    {
                        let worktree_id = worktree.read(cx).id();
                        for (path, serialized_excerpts) in worktree_state.excerpts {
                            excerpts.insert(ProjectPath { worktree_id, path }, serialized_excerpts);
                        }
                    }
                }
            }
            self.state = ProjectIndexToolState::Finished {
                excerpts,
                index_status: serialized.index_status,
            };
        }
        cx.notify();
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
            state: ProjectIndexToolState::CollectingQuery,
            input: Default::default(),
            expanded_header: false,
            project_index: self.project_index.clone(),
        })
    }
}
