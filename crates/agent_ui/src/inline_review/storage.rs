use acp_thread::AcpThread;
use agent_servers::AcpConnection;
use anyhow::{Context as _, Result, ensure};
use editor::SavedInlineReview;
use gpui::{App, Entity, Task};
use serde_json::{Value, json};
use std::{path::Path, rc::Rc};

#[derive(Clone)]
pub(super) struct ReviewStorage {
    pub(super) request: Rc<dyn Fn(Value, &mut App) -> Task<Result<Value>>>,
}

impl ReviewStorage {
    pub(super) fn for_thread(thread: &Entity<AcpThread>, cx: &App) -> Result<Self> {
        let session_id = thread.read(cx).session_id().to_string();
        let connection = thread
            .read(cx)
            .connection()
            .clone()
            .downcast::<AcpConnection>()
            .context("Review storage requires the pi-live ACP v1 bridge")?;
        Ok(Self {
            request: Rc::new(move |mut params, cx| {
                params["sessionId"] = json!(session_id);
                params["version"] = json!(1);
                let response =
                    connection.extension_request("_zed-pi-live/review-store", params, cx);
                cx.spawn(async move |_| {
                let value = response.await.map_err(|error| anyhow::anyhow!("Remote review storage failed: {error:#}. If unsupported, update and reconnect pi-live."))?;
                ensure!(value["version"] == 1, "Unsupported review-store response version");
                Ok(value)
            })
            }),
        })
    }

    pub(super) fn create(&self, review: SavedInlineReview, cx: &mut App) -> Task<Result<()>> {
        let review = match serde_json::to_value(review) {
            Ok(review) => review,
            Err(error) => return Task::ready(Err(error.into())),
        };
        let request = (self.request)(json!({"operation": "create", "review": review}), cx);
        cx.spawn(async move |_| {
            request.await?;
            Ok(())
        })
    }

    pub(super) fn finish(
        &self,
        submission_id: &str,
        outcome: &str,
        reply: Option<String>,
        error: Option<String>,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let mut params =
            json!({"operation": "finish", "submissionId": submission_id, "outcome": outcome});
        if let Some(reply) = reply {
            params["reply"] = json!(reply);
        }
        if let Some(error) = error {
            params["error"] = json!(error);
        }
        let request = (self.request)(params, cx);
        cx.spawn(async move |_| {
            request.await?;
            Ok(())
        })
    }

    pub(super) fn load(&self, path: &Path, cx: &mut App) -> Task<Result<Vec<SavedInlineReview>>> {
        let Some(path) = path.to_str() else {
            return Task::ready(Err(anyhow::anyhow!(
                "Review storage requires a UTF-8 file path"
            )));
        };
        let request = (self.request)(json!({"operation": "list", "path": path}), cx);
        cx.spawn(async move |_| {
            serde_json::from_value(request.await?["reviews"].clone())
                .context("Invalid saved reviews")
        })
    }
}
