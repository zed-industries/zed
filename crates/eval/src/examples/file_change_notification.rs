use agent_settings::AgentProfileId;
use anyhow::Result;
use async_trait::async_trait;

use crate::example::{Example, ExampleContext, ExampleMetadata, JudgeAssertion};

pub struct FileChangeNotificationExample;

#[async_trait(?Send)]
impl Example for FileChangeNotificationExample {
    fn meta(&self) -> ExampleMetadata {
        ExampleMetadata {
            name: "file_change_notification".to_string(),
            url: "https://github.com/octocat/hello-world".to_string(),
            revision: "7fd1a60b01f91b314f59955a4e4d4e80d8edf11d".to_string(),
            language_server: None,
            max_assertions: Some(3),
            profile_id: AgentProfileId::default(),
            existing_thread_json: None,
            max_turns: Some(3),
        }
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        // Track README in action log, so that the model gets notified of its change
        let project_path = cx.agent_thread().read_with(cx, |thread, cx| {
            let project = thread.project().clone();
            let project_path = project
                .read(cx)
                .find_project_path("README", cx)
                .expect("README file should exist");
            project_path
        })?;
        let buffer = cx
            .agent_thread()
            .update(cx, |thread, cx| {
                let project = thread.project().clone();
                project.update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })
            })?
            .await?;

        cx.agent_thread().update(cx, |thread, cx| {
            thread.action_log().update(cx, |action_log, cx| {
                action_log.buffer_read(buffer.clone(), cx);
            });
        })?;

        // Start conversation, the specific message is not important
        cx.push_user_message("Find all files in this repo");
        cx.run_turn().await?;

        // Edit README -- the model should get a notification on next turn
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..buffer.len(), "Surprise!")], None, cx);
        })?;

        // Run for some more turns.
        // The model shouldn't thank for letting it know about the file change.
        cx.run_turns(3).await?;

        Ok(())
    }

    fn thread_assertions(&self) -> Vec<JudgeAssertion> {
        let assert = JudgeAssertion {
            id: "change-file-notification".into(),
            description:
                "Agent should not acknowledge or mention anything about files that have been changed"
                    .into(),
        };
        vec![assert]
    }
}
