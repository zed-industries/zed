use anyhow::Result;
use assistant_tooling::{LanguageModelTool, ToolOutput};
use collections::BTreeMap;
use gpui::{prelude::*, Model, Task};
use project::ProjectPath;
use schemars::JsonSchema;
use semantic_index::{ProjectIndex, Status};
use serde::Deserialize;
use std::{fmt::Write as _, ops::Range};
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
    output: Result<ProjectIndexOutput>,
    element_id: ElementId,
    expanded_header: bool,
}

pub struct ProjectIndexOutput {
    status: Status,
    excerpts: BTreeMap<ProjectPath, Vec<Range<usize>>>,
}

impl ProjectIndexView {
    fn new(input: CodebaseQuery, output: Result<ProjectIndexOutput>) -> Self {
        let element_id = ElementId::Name(nanoid::nanoid!().into());

        Self {
            input,
            output,
            element_id,
            expanded_header: false,
        }
    }

    fn toggle_header(&mut self, cx: &mut ViewContext<Self>) {
        self.expanded_header = !self.expanded_header;
        cx.notify();
    }
}

impl Render for ProjectIndexView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let query = self.input.query.clone();

        let result = &self.output;

        let output = match result {
            Err(err) => {
                return div().child(Label::new(format!("Error: {}", err)).color(Color::Error));
            }
            Ok(output) => output,
        };

        let file_count = output.excerpts.len();

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
                        .child(
                            v_flex()
                                .gap_2()
                                .children(output.excerpts.keys().map(|path| {
                                    h_flex().gap_2().child(Icon::new(IconName::File)).child(
                                        Label::new(path.path.to_string_lossy().to_string())
                                            .color(Color::Muted),
                                    )
                                })),
                        ),
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
        match &self.output {
            Ok(output) => {
                let mut body = "found results in the following paths:\n".to_string();

                for (project_path, ranges) in &output.excerpts {
                    context.add_excerpts(project_path.clone(), ranges);
                    writeln!(&mut body, "* {}", &project_path.path.display()).unwrap();
                }

                if output.status != Status::Idle {
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
                    excerpts: Default::default(),
                };

                for search_result in search_results {
                    let path = ProjectPath {
                        worktree_id: search_result.worktree.read(cx).id(),
                        path: search_result.path.clone(),
                    };

                    let excerpts_for_path = output.excerpts.entry(path).or_default();
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

    fn output_view(
        input: Self::Input,
        output: Result<Self::Output>,
        cx: &mut WindowContext,
    ) -> gpui::View<Self::View> {
        cx.new_view(|_cx| ProjectIndexView::new(input, output))
    }

    fn render_running(_: &mut WindowContext) -> impl IntoElement {
        CollapsibleContainer::new(ElementId::Name(nanoid::nanoid!().into()), false)
            .start_slot("Searching code base")
    }
}
