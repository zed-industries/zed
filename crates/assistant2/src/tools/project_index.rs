use anyhow::Result;
use assistant_tooling::LanguageModelTool;
use gpui::{prelude::*, Model, Task};
use project::Fs;
use schemars::JsonSchema;
use semantic_index::{ProjectIndex, Status};
use serde::Deserialize;
use std::{collections::HashSet, sync::Arc};

use ui::{
    div, prelude::*, CollapsibleContainer, Color, Icon, IconName, Label, SharedString,
    WindowContext,
};
use util::ResultExt as _;

const DEFAULT_SEARCH_LIMIT: usize = 20;

#[derive(Clone)]
pub struct CodebaseExcerpt {
    path: SharedString,
    text: SharedString,
    score: f32,
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

        let num_files_searched = output.files_searched.len();

        let header = h_flex()
            .gap_2()
            .child(Icon::new(IconName::File))
            .child(format!(
                "Read {} {}",
                num_files_searched,
                if num_files_searched == 1 {
                    "file"
                } else {
                    "files"
                }
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
                        .child(v_flex().gap_2().children(output.files_searched.iter().map(
                            |path| {
                                h_flex()
                                    .gap_2()
                                    .child(Icon::new(IconName::File))
                                    .child(Label::new(path.clone()).color(Color::Muted))
                            },
                        ))),
                ),
        )
    }
}

pub struct ProjectIndexTool {
    project_index: Model<ProjectIndex>,
    fs: Arc<dyn Fs>,
}

pub struct ProjectIndexOutput {
    excerpts: Vec<CodebaseExcerpt>,
    status: Status,
    files_searched: HashSet<SharedString>,
}

impl ProjectIndexTool {
    pub fn new(project_index: Model<ProjectIndex>, fs: Arc<dyn Fs>) -> Self {
        // Listen for project index status and update the ProjectIndexTool directly

        // TODO: setup a better description based on the user's current codebase.
        Self { project_index, fs }
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
        let results = project_index.search(
            query.query.clone(),
            query.limit.unwrap_or(DEFAULT_SEARCH_LIMIT),
            cx,
        );

        let fs = self.fs.clone();

        cx.spawn(|cx| async move {
            let results = results.await?;

            let excerpts = results.into_iter().map(|result| {
                let abs_path = result
                    .worktree
                    .read_with(&cx, |worktree, _| worktree.abs_path().join(&result.path));
                let fs = fs.clone();

                async move {
                    let path = result.path.clone();
                    let text = fs.load(&abs_path?).await?;

                    let mut start = result.range.start;
                    let mut end = result.range.end.min(text.len());
                    while !text.is_char_boundary(start) {
                        start += 1;
                    }
                    while !text.is_char_boundary(end) {
                        end -= 1;
                    }

                    anyhow::Ok(CodebaseExcerpt {
                        path: path.to_string_lossy().to_string().into(),
                        text: SharedString::from(text[start..end].to_string()),
                        score: result.score,
                    })
                }
            });

            let mut files_searched = HashSet::new();
            let excerpts = futures::future::join_all(excerpts)
                .await
                .into_iter()
                .filter_map(|result| result.log_err())
                .inspect(|excerpt| {
                    files_searched.insert(excerpt.path.clone());
                })
                .collect::<Vec<_>>();

            anyhow::Ok(ProjectIndexOutput {
                excerpts,
                status,
                files_searched,
            })
        })
    }

    fn output_view(
        _tool_call_id: String,
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

    fn format(_input: &Self::Input, output: &Result<Self::Output>) -> String {
        match &output {
            Ok(output) => {
                let mut body = "Semantic search results:\n".to_string();

                if output.status != Status::Idle {
                    body.push_str("Still indexing. Results may be incomplete.\n");
                }

                if output.excerpts.is_empty() {
                    body.push_str("No results found");
                    return body;
                }

                for excerpt in &output.excerpts {
                    body.push_str("Excerpt from ");
                    body.push_str(excerpt.path.as_ref());
                    body.push_str(", score ");
                    body.push_str(&excerpt.score.to_string());
                    body.push_str(":\n");
                    body.push_str("~~~\n");
                    body.push_str(excerpt.text.as_ref());
                    body.push_str("~~~\n");
                }
                body
            }
            Err(err) => format!("Error: {}", err),
        }
    }
}
