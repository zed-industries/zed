use anyhow::Result;
use assistant_tooling::{
    // assistant_tool_button::{AssistantToolButton, ToolStatus},
    LanguageModelTool,
};
use gpui::{prelude::*, Model, Task};
use project::Fs;
use schemars::JsonSchema;
use semantic_index::{ProjectIndex, Status};
use serde::Deserialize;
use std::sync::Arc;
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
    element_id: ElementId,
    expanded: bool,
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
}

impl ProjectIndexView {
    fn toggle_expanded(&mut self, element_id: ElementId, cx: &mut ViewContext<Self>) {
        if let Ok(output) = &mut self.output {
            if let Some(excerpt) = output
                .excerpts
                .iter_mut()
                .find(|excerpt| excerpt.element_id == element_id)
            {
                excerpt.expanded = !excerpt.expanded;
                cx.notify();
            }
        }
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

        div()
            .v_flex()
            .gap_2()
            .child(
                div()
                    .p_2()
                    .rounded_md()
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        h_flex()
                            .child(Label::new("Query: ").color(Color::Modified))
                            .child(Label::new(query).color(Color::Muted)),
                    ),
            )
            .children(output.excerpts.iter().map(|excerpt| {
                let element_id = excerpt.element_id.clone();
                let expanded = excerpt.expanded;

                CollapsibleContainer::new(element_id.clone(), expanded)
                    .start_slot(
                        h_flex()
                            .gap_1()
                            .child(Icon::new(IconName::File).color(Color::Muted))
                            .child(Label::new(excerpt.path.clone()).color(Color::Muted)),
                    )
                    .on_click(cx.listener(move |this, _, cx| {
                        this.toggle_expanded(element_id.clone(), cx);
                    }))
                    .child(
                        div()
                            .p_2()
                            .rounded_md()
                            .bg(cx.theme().colors().editor_background)
                            .child(excerpt.text.clone()),
                    )
            }))
    }
}

pub struct ProjectIndexTool {
    project_index: Model<ProjectIndex>,
    fs: Arc<dyn Fs>,
}

pub struct ProjectIndexOutput {
    excerpts: Vec<CodebaseExcerpt>,
    status: Status,
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
        "Semantic search against the user's current codebase, returning excerpts related to the query by computing a dot product against embeddings of chunks and an embedding of the query".to_string()
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
                        element_id: ElementId::Name(nanoid::nanoid!().into()),
                        expanded: false,
                        path: path.to_string_lossy().to_string().into(),
                        text: SharedString::from(text[start..end].to_string()),
                        score: result.score,
                    })
                }
            });

            let excerpts = futures::future::join_all(excerpts)
                .await
                .into_iter()
                .filter_map(|result| result.log_err())
                .collect();
            anyhow::Ok(ProjectIndexOutput { excerpts, status })
        })
    }

    fn output_view(
        _tool_call_id: String,
        input: Self::Input,
        output: Result<Self::Output>,
        cx: &mut WindowContext,
    ) -> gpui::View<Self::View> {
        cx.new_view(|_cx| ProjectIndexView { input, output })
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
