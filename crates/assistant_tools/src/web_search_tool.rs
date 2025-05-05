use std::{sync::Arc, time::Duration};

use crate::schema::json_schema_for;
use crate::ui::ToolCallCardHeader;
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolCard, ToolResult, ToolUseStatus};
use futures::{Future, FutureExt, TryFutureExt};
use gpui::{
    AnyWindowHandle, App, AppContext, Context, Entity, IntoElement, Task, WeakEntity, Window,
};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::{IconName, Tooltip, prelude::*};
use web_search::WebSearchRegistry;
use workspace::Workspace;
use zed_llm_client::{WebSearchCitation, WebSearchResponse};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WebSearchToolInput {
    /// The search term or question to query on the web.
    query: String,
}

pub struct WebSearchTool;

impl Tool for WebSearchTool {
    fn name(&self) -> String {
        "web_search".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn description(&self) -> String {
        "Search the web for information using your query. Use this when you need real-time information, facts, or data that might not be in your training. Results will include snippets and links from relevant web pages.".into()
    }

    fn icon(&self) -> IconName {
        IconName::Globe
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<WebSearchToolInput>(format)
    }

    fn ui_text(&self, _input: &serde_json::Value) -> String {
        "Searching the Web".to_string()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        _project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let input = match serde_json::from_value::<WebSearchToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };
        let Some(provider) = WebSearchRegistry::read_global(cx).active_provider() else {
            return Task::ready(Err(anyhow!("Web search is not available."))).into();
        };

        let search_task = provider.search(input.query, cx).map_err(Arc::new).shared();
        let output = cx.background_spawn({
            let search_task = search_task.clone();
            async move {
                let response = search_task.await.map_err(|err| anyhow!(err))?;
                serde_json::to_string(&response).context("Failed to serialize search results")
            }
        });

        ToolResult {
            output,
            card: Some(cx.new(|cx| WebSearchToolCard::new(search_task, cx)).into()),
        }
    }
}

#[derive(RegisterComponent)]
struct WebSearchToolCard {
    response: Option<Result<WebSearchResponse>>,
    _task: Task<()>,
}

impl WebSearchToolCard {
    fn new(
        search_task: impl 'static + Future<Output = Result<WebSearchResponse, Arc<anyhow::Error>>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let _task = cx.spawn(async move |this, cx| {
            let response = search_task.await.map_err(|err| anyhow!(err));
            this.update(cx, |this, cx| {
                this.response = Some(response);
                cx.notify();
            })
            .ok();
        });

        Self {
            response: None,
            _task,
        }
    }
}

impl ToolCard for WebSearchToolCard {
    fn render(
        &mut self,
        _status: &ToolUseStatus,
        _window: &mut Window,
        _workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let header = match self.response.as_ref() {
            Some(Ok(response)) => {
                let text: SharedString = if response.citations.len() == 1 {
                    "1 result".into()
                } else {
                    format!("{} results", response.citations.len()).into()
                };
                ToolCallCardHeader::new(IconName::Globe, "Searched the Web")
                    .with_secondary_text(text)
            }
            Some(Err(error)) => {
                ToolCallCardHeader::new(IconName::Globe, "Web Search").with_error(error.to_string())
            }
            None => ToolCallCardHeader::new(IconName::Globe, "Searching the Web").loading(),
        };

        let content =
            self.response.as_ref().and_then(|response| match response {
                Ok(response) => {
                    Some(
                        v_flex()
                            .overflow_hidden()
                            .ml_1p5()
                            .pl(px(5.))
                            .border_l_1()
                            .border_color(cx.theme().colors().border_variant)
                            .gap_1()
                            .children(response.citations.iter().enumerate().map(
                                |(index, citation)| {
                                    let title = citation.title.clone();
                                    let url = citation.url.clone();

                                    Button::new(("citation", index), title)
                                        .label_size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .icon(IconName::ArrowUpRight)
                                        .icon_size(IconSize::XSmall)
                                        .icon_position(IconPosition::End)
                                        .truncate(true)
                                        .tooltip({
                                            let url = url.clone();
                                            move |window, cx| {
                                                Tooltip::with_meta(
                                                    "Citation Link",
                                                    None,
                                                    url.clone(),
                                                    window,
                                                    cx,
                                                )
                                            }
                                        })
                                        .on_click({
                                            let url = url.clone();
                                            move |_, _, cx| cx.open_url(&url)
                                        })
                                },
                            ))
                            .into_any(),
                    )
                }
                Err(_) => None,
            });

        v_flex().mb_3().gap_1().child(header).children(content)
    }
}

impl Component for WebSearchToolCard {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn preview(window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let in_progress_search = cx.new(|cx| WebSearchToolCard {
            response: None,
            _task: cx.spawn(async move |_this, cx| {
                loop {
                    cx.background_executor()
                        .timer(Duration::from_secs(60))
                        .await
                }
            }),
        });

        let successful_search = cx.new(|_cx| WebSearchToolCard {
            response: Some(Ok(example_search_response())),
            _task: Task::ready(()),
        });

        let error_search = cx.new(|_cx| WebSearchToolCard {
            response: Some(Err(anyhow!("Failed to resolve https://google.com"))),
            _task: Task::ready(()),
        });

        Some(
            v_flex()
                .gap_6()
                .children(vec![example_group(vec![
                    single_example(
                        "In Progress",
                        div()
                            .size_full()
                            .child(in_progress_search.update(cx, |tool, cx| {
                                tool.render(
                                    &ToolUseStatus::Pending,
                                    window,
                                    WeakEntity::new_invalid(),
                                    cx,
                                )
                                .into_any_element()
                            }))
                            .into_any_element(),
                    ),
                    single_example(
                        "Successful",
                        div()
                            .size_full()
                            .child(successful_search.update(cx, |tool, cx| {
                                tool.render(
                                    &ToolUseStatus::Finished("".into()),
                                    window,
                                    WeakEntity::new_invalid(),
                                    cx,
                                )
                                .into_any_element()
                            }))
                            .into_any_element(),
                    ),
                    single_example(
                        "Error",
                        div()
                            .size_full()
                            .child(error_search.update(cx, |tool, cx| {
                                tool.render(
                                    &ToolUseStatus::Error("".into()),
                                    window,
                                    WeakEntity::new_invalid(),
                                    cx,
                                )
                                .into_any_element()
                            }))
                            .into_any_element(),
                    ),
                ])])
                .into_any_element(),
        )
    }
}

fn example_search_response() -> WebSearchResponse {
    WebSearchResponse {
        summary: r#"Toronto boasts a vibrant culinary scene with a diverse array of..."#
            .to_string(),
        citations: vec![
            WebSearchCitation {
                title: "Alo".to_string(),
                url: "https://www.google.com/maps/search/Alo%2C+Toronto%2C+Canada".to_string(),
                range: Some(147..213),
            },
            WebSearchCitation {
                title: "Edulis".to_string(),
                url: "https://www.google.com/maps/search/Edulis%2C+Toronto%2C+Canada".to_string(),
                range: Some(447..519),
            },
            WebSearchCitation {
                title: "Sushi Masaki Saito".to_string(),
                url: "https://www.google.com/maps/search/Sushi+Masaki+Saito%2C+Toronto%2C+Canada"
                    .to_string(),
                range: Some(776..872),
            },
            WebSearchCitation {
                title: "Shoushin".to_string(),
                url: "https://www.google.com/maps/search/Shoushin%2C+Toronto%2C+Canada".to_string(),
                range: Some(1072..1148),
            },
            WebSearchCitation {
                title: "Restaurant 20 Victoria".to_string(),
                url:
                    "https://www.google.com/maps/search/Restaurant+20+Victoria%2C+Toronto%2C+Canada"
                        .to_string(),
                range: Some(1291..1395),
            },
        ],
    }
}
