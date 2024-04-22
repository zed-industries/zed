use anyhow::Result;
use assistant_tooling::LanguageModelTool;
use gpui::{prelude::*, AnyElement, AppContext, Model, Task};
use project::Fs;
use schemars::JsonSchema;
use semantic_index::ProjectIndex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ui::{
    div, prelude::*, CollapsibleContainer, Color, Icon, IconName, Label, SharedString,
    WindowContext,
};
use util::ResultExt as _;

#[derive(Serialize, Clone)]
pub struct CodebaseExcerpt {
    path: SharedString,
    text: SharedString,
    score: f32,
}

#[derive(Deserialize, JsonSchema)]
pub struct CodebaseQuery {
    query: String,
}

pub struct ProjectIndexTool {
    pub project_index: Model<ProjectIndex>,
    pub fs: Arc<dyn Fs>,
}

impl LanguageModelTool for ProjectIndexTool {
    type Input = CodebaseQuery;
    type Output = Vec<CodebaseExcerpt>;

    fn name(&self) -> String {
        "query_codebase".to_string()
    }

    fn description(&self) -> String {
        "Executes a query against the codebase, returning excerpts related to the query".to_string()
    }

    fn execute(&self, query: &Self::Input, cx: &AppContext) -> Task<Result<Self::Output>> {
        let project_index = self.project_index.read(cx);
        let results = project_index.search(query.query.as_str(), 10, cx);
        let fs = self.fs.clone();

        cx.spawn(|cx| async move {
            let results = results.await;
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

                    // todo!("Handle out of date ranges");

                    anyhow::Ok(CodebaseExcerpt {
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
            anyhow::Ok(excerpts)
        })
    }

    fn render(
        _tool_call_id: &str,
        input: &Self::Input,
        excerpts: &Self::Output,
        cx: &mut WindowContext,
    ) -> AnyElement {
        let query = input.query.clone();

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
            .children(excerpts.iter().map(|excerpt| {
                // This render doesn't have state/model, so we can't use the listener
                // let expanded = excerpt.expanded;
                // let element_id = excerpt.element_id.clone();
                let element_id = ElementId::Name(nanoid::nanoid!().into());
                let expanded = false;

                CollapsibleContainer::new(element_id.clone(), expanded)
                    .start_slot(
                        h_flex()
                            .gap_1()
                            .child(Icon::new(IconName::File).color(Color::Muted))
                            .child(Label::new(excerpt.path.clone()).color(Color::Muted)),
                    )
                    // .on_click(cx.listener(move |this, _, cx| {
                    //     this.toggle_expanded(element_id.clone(), cx);
                    // }))
                    .child(
                        div()
                            .p_2()
                            .rounded_md()
                            .bg(cx.theme().colors().editor_background)
                            .child(
                                excerpt.text.clone(), // todo!(): Show as an editor block
                            ),
                    )
            }))
            .into_any_element()
    }

    fn format(_input: &Self::Input, excerpts: &Self::Output) -> String {
        let mut body = "Semantic search results:\n".to_string();

        for excerpt in excerpts {
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
}
