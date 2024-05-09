use anyhow::{anyhow, Result};
use assistant_tooling::{LanguageModelTool, ToolOutput};
use collections::BTreeMap;
use gpui::{prelude::*, Model, Task};
use project::ProjectPath;
use schemars::JsonSchema;
use semantic_index::{ProjectIndex, Status};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fmt::Write as _, ops::Range, path::Path, sync::Arc};
use ui::{div, prelude::*, CollapsibleContainer, Color, Icon, IconName, Label, WindowContext};

const DEFAULT_SEARCH_LIMIT: usize = 20;

pub struct ProjectIndexTool {
    project_index: Model<ProjectIndex>,
}

// Note: Comments on a `LanguageModelTool::Input` become descriptions on the generated JSON schema as shown to the language model.
// Any changes or deletions to the `CodebaseQuery` comments will change model behavior.

#[derive(Deserialize, JsonSchema)]
pub struct CodebaseQuery {
    /// Semantic search query
    query: String,
    /// Maximum number of results to return, defaults to 20
    limit: Option<usize>,
}

pub struct ProjectIndexView {
    input: CodebaseQuery,
    status: Status,
    excerpts: Result<BTreeMap<ProjectPath, Vec<Range<usize>>>>,
    element_id: ElementId,
    expanded_header: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectIndexOutput {
    status: Status,
    worktrees: BTreeMap<Arc<Path>, WorktreeIndexOutput>,
}

#[derive(Serialize, Deserialize)]
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
        let excerpts = match &self.excerpts {
            Err(err) => {
                return div().child(Label::new(format!("Error: {}", err)).color(Color::Error));
            }
            Ok(excerpts) => excerpts,
        };

        let file_count = excerpts.len();
        let header = h_flex()
            .gap_2()
            .child(Icon::new(IconName::File))
            .child(format!(
                "Read {} {}",
                file_count,
                if file_count == 1 { "file" } else { "files" }
            ));

        v_flex().gap_3().child(
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
                        .child(v_flex().gap_2().children(excerpts.keys().map(|path| {
                            h_flex().gap_2().child(Icon::new(IconName::File)).child(
                                Label::new(path.path.to_string_lossy().to_string())
                                    .color(Color::Muted),
                            )
                        }))),
                ),
        )
    }
}

impl ToolOutput for ProjectIndexView {
    fn generate(
        &self,
        context: &mut assistant_tooling::ProjectContext,
        _: &mut WindowContext,
    ) -> String {
        match &self.excerpts {
            Ok(excerpts) => {
                let mut body = "found results in the following paths:\n".to_string();

                for (project_path, ranges) in excerpts {
                    context.add_excerpts(project_path.clone(), ranges);
                    writeln!(&mut body, "* {}", &project_path.path.display()).unwrap();
                }

                if self.status != Status::Idle {
                    body.push_str("Still indexing. Results may be incomplete.\n");
                }

                body
            }
            Err(err) => format!("Error: {}", err),
        }
    }
}

impl ProjectIndexTool {
    pub fn new(project_index: Model<ProjectIndex>) -> Self {
        Self { project_index }
    }
}

impl LanguageModelTool for ProjectIndexTool {
    type Input = CodebaseQuery;
    type Output = ProjectIndexOutput;
    type View = ProjectIndexView;

    fn name(&self) -> String {
        "query_codebase".to_string()
    }

    fn description(&self) -> String {
        "Semantic search against the user's current codebase, returning excerpts related to the query by computing a dot product against embeddings of code chunks in the code base and an embedding of the query.".to_string()
    }

    fn execute(&self, query: &Self::Input, cx: &mut WindowContext) -> Task<Result<Self::Output>> {
        let project_index = self.project_index.read(cx);
        let status = project_index.status();
        let search = project_index.search(
            query.query.clone(),
            query.limit.unwrap_or(DEFAULT_SEARCH_LIMIT),
            cx,
        );

        cx.spawn(|mut cx| async move {
            let search_results = search.await?;

            cx.update(|cx| {
                let mut output = ProjectIndexOutput {
                    status,
                    worktrees: Default::default(),
                };

                for search_result in search_results {
                    let worktree_path = search_result.worktree.read(cx).abs_path();
                    let excerpts = &mut output
                        .worktrees
                        .entry(worktree_path)
                        .or_insert(WorktreeIndexOutput {
                            excerpts: Default::default(),
                        })
                        .excerpts;

                    let excerpts_for_path = excerpts.entry(search_result.path).or_default();
                    let ix = match excerpts_for_path
                        .binary_search_by_key(&search_result.range.start, |r| r.start)
                    {
                        Ok(ix) | Err(ix) => ix,
                    };
                    excerpts_for_path.insert(ix, search_result.range);
                }

                output
            })
        })
    }

    fn view(
        &self,
        input: Self::Input,
        output: Result<Self::Output>,
        cx: &mut WindowContext,
    ) -> gpui::View<Self::View> {
        cx.new_view(|cx| {
            let status;
            let excerpts;
            match output {
                Ok(output) => {
                    status = output.status;
                    let project_index = self.project_index.read(cx);
                    if let Some(project) = project_index.project().upgrade() {
                        let project = project.read(cx);
                        excerpts = Ok(output
                            .worktrees
                            .into_iter()
                            .filter_map(|(abs_path, output)| {
                                for worktree in project.worktrees() {
                                    let worktree = worktree.read(cx);
                                    if worktree.abs_path() == abs_path {
                                        return Some((worktree.id(), output.excerpts));
                                    }
                                }
                                None
                            })
                            .flat_map(|(worktree_id, excerpts)| {
                                excerpts.into_iter().map(move |(path, ranges)| {
                                    (ProjectPath { worktree_id, path }, ranges)
                                })
                            })
                            .collect::<BTreeMap<_, _>>());
                    } else {
                        excerpts = Err(anyhow!("project was dropped"));
                    }
                }
                Err(err) => {
                    status = Status::Idle;
                    excerpts = Err(err);
                }
            };

            ProjectIndexView {
                input,
                status,
                excerpts,
                element_id: ElementId::Name(nanoid::nanoid!().into()),
                expanded_header: false,
            }
        })
    }

    fn render_running(arguments: &Option<Value>, _: &mut WindowContext) -> impl IntoElement {
        let text: String = arguments
            .as_ref()
            .and_then(|arguments| arguments.get("query"))
            .and_then(|query| query.as_str())
            .map(|query| format!("Searching for: {}", query))
            .unwrap_or_else(|| "Preparing search...".to_string());

        CollapsibleContainer::new(ElementId::Name(nanoid::nanoid!().into()), false).start_slot(text)
    }
}
