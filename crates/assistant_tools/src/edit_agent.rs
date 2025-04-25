use anyhow::Result;
use futures::{Stream, StreamExt};
use gpui::{App, AsyncApp, Entity};
use language::Anchor;
use language_model::{
    LanguageModel, LanguageModelRequest, LanguageModelRequestMessage, MessageContent, Role,
};
use project::{Project, ProjectPath};
use serde::Serialize;
use std::{ops::Range, path::Path, sync::Arc};

use crate::{Template, Templates};

#[derive(Serialize)]
pub struct EditAgentTemplate {
    path: Arc<Path>,
    file_content: String,
    instructions: String,
}

impl Template for EditAgentTemplate {
    const TEMPLATE_NAME: &'static str = "edit_agent.hbs";
}

pub struct EditAgent {
    project: Entity<Project>,
    model: Arc<dyn LanguageModel>,
    templates: Arc<Templates>,
}

impl EditAgent {
    pub fn new(
        model: Arc<dyn LanguageModel>,
        project: Entity<Project>,
        templates: Arc<Templates>,
    ) -> Self {
        EditAgent {
            project,
            model,
            templates,
        }
    }

    pub async fn interpret(
        &self,
        path: ProjectPath,
        instructions: String,
        cx: &mut AsyncApp,
    ) -> Result<impl Stream<Item = Result<(Range<Anchor>, String)>>> {
        let buffer = self
            .project
            .update(cx, |project, cx| project.open_buffer(path.clone(), cx))?
            .await?;
        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
        let prompt = EditAgentTemplate {
            path: path.path.clone(),
            file_content: snapshot.text(),
            instructions,
        }
        .render(&self.templates)?;
        let request = LanguageModelRequest {
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text(prompt)],
                cache: false,
            }],
            temperature: Some(0.0),
            ..Default::default()
        };
        let mut stream = self.model.stream_completion_text(request, cx).await?.stream;
        while let Some(chunk) = stream.next().await {
            print!("{}", chunk?);
        }

        Ok(futures::stream::empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use client::{Client, UserStore};
    use fs::FakeFs;
    use gpui::{AppContext, TestAppContext};
    use indoc::indoc;
    use language_model::LanguageModelRegistry;
    use reqwest_client::ReqwestClient;
    use serde_json::json;
    use util::path;

    #[gpui::test]
    async fn test_basic(cx: &mut TestAppContext) {
        let test = agent_test(cx).await;
        let diff = apply_edits(
            "/root/lib.rs",
            indoc! {"
                struct User {
                    id: u32,
                    name: String,
                }

                impl User {
                    pub fn new(id: u32, name: String) -> Self {
                        User { id, name }
                    }

                    pub fn id(&self) -> u32 {
                        self.id
                    }

                    pub fn name(&self) -> &str {
                        &self.name
                    }
                }
            "},
            indoc! {"
                Introduce a new field `age: u8`, add it to the constructor
                and also add a getter method for it.
            "},
            &test,
            cx,
        )
        .await;
    }

    async fn apply_edits(
        path: impl AsRef<Path>,
        content: impl Into<String>,
        instructions: impl Into<String>,
        test: &EditAgentTest,
        cx: &mut TestAppContext,
    ) {
        test.fs
            .insert_file(path.as_ref(), content.into().into_bytes())
            .await;
        let path = test
            .agent
            .project
            .read_with(cx, |project, cx| project.find_project_path(path, cx))
            .unwrap();
        let edits = test
            .agent
            .interpret(path, instructions.into(), &mut cx.to_async())
            .await
            .unwrap();
    }

    struct EditAgentTest {
        fs: Arc<FakeFs>,
        agent: EditAgent,
    }

    async fn agent_test(cx: &mut TestAppContext) -> EditAgentTest {
        cx.executor().allow_parking();
        cx.update(settings::init);
        cx.update(Project::init_settings);
        cx.update(language::init);
        cx.update(gpui_tokio::init);
        cx.update(client::init_settings);

        let fs = FakeFs::new(cx.executor().clone());
        fs.insert_tree("/root", json!({})).await;
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
        let model = cx
            .update(|cx| {
                let http_client = ReqwestClient::user_agent("agent tests").unwrap();
                cx.set_http_client(Arc::new(http_client));

                let client = Client::production(cx);
                let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
                language_model::init(client.clone(), cx);
                language_models::init(user_store.clone(), client.clone(), fs.clone(), cx);

                let models = LanguageModelRegistry::read_global(cx);
                let model = models
                    .available_models(cx)
                    .find(|model| model.id().0 == "gemini-2.5-flash-preview-04-17")
                    .unwrap();

                let provider = models.provider(&model.provider_id()).unwrap();
                let authenticated = provider.authenticate(cx);

                cx.spawn(async move |_| {
                    authenticated.await.unwrap();
                    model
                })
            })
            .await;

        EditAgentTest {
            fs,
            agent: EditAgent::new(model, project, Templates::new()),
        }
    }
}
