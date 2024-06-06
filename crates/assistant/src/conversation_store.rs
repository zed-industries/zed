use crate::{assistant_settings::OpenAiModel, MessageId, MessageMetadata};
use anyhow::{anyhow, Result};
use collections::HashMap;
use fs::Fs;
use futures::StreamExt;
use fuzzy::StringMatchCandidate;
use gpui::{AppContext, Model, ModelContext, Task};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{cmp::Reverse, ffi::OsStr, path::PathBuf, sync::Arc, time::Duration};
use ui::Context;
use util::{paths::CONVERSATIONS_DIR, ResultExt, TryFutureExt};

#[derive(Serialize, Deserialize)]
pub struct SavedMessage {
    pub id: MessageId,
    pub start: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SavedConversation {
    pub id: Option<String>,
    pub zed: String,
    pub version: String,
    pub text: String,
    pub messages: Vec<SavedMessage>,
    pub message_metadata: HashMap<MessageId, MessageMetadata>,
    pub summary: String,
}

impl SavedConversation {
    pub const VERSION: &'static str = "0.2.0";
}

#[derive(Serialize, Deserialize)]
struct SavedConversationV0_1_0 {
    id: Option<String>,
    zed: String,
    version: String,
    text: String,
    messages: Vec<SavedMessage>,
    message_metadata: HashMap<MessageId, MessageMetadata>,
    summary: String,
    api_url: Option<String>,
    model: OpenAiModel,
}

#[derive(Clone)]
pub struct SavedConversationMetadata {
    pub title: String,
    pub path: PathBuf,
    pub mtime: chrono::DateTime<chrono::Local>,
}

pub struct ConversationStore {
    conversations_metadata: Vec<SavedConversationMetadata>,
    fs: Arc<dyn Fs>,
    _watch_updates: Task<Option<()>>,
}

impl ConversationStore {
    pub fn new(fs: Arc<dyn Fs>, cx: &mut AppContext) -> Task<Result<Model<Self>>> {
        cx.spawn(|mut cx| async move {
            const CONVERSATION_WATCH_DURATION: Duration = Duration::from_millis(100);
            let (mut events, _) = fs
                .watch(&CONVERSATIONS_DIR, CONVERSATION_WATCH_DURATION)
                .await;

            let this = cx.new_model(|cx: &mut ModelContext<Self>| Self {
                conversations_metadata: Vec::new(),
                fs,
                _watch_updates: cx.spawn(|this, mut cx| {
                    async move {
                        while events.next().await.is_some() {
                            this.update(&mut cx, |this, cx| this.reload(cx))?
                                .await
                                .log_err();
                        }
                        anyhow::Ok(())
                    }
                    .log_err()
                }),
            })?;
            this.update(&mut cx, |this, cx| this.reload(cx))?
                .await
                .log_err();
            Ok(this)
        })
    }

    pub fn load(&self, path: PathBuf, cx: &AppContext) -> Task<Result<SavedConversation>> {
        let fs = self.fs.clone();
        cx.background_executor().spawn(async move {
            let saved_conversation = fs.load(&path).await?;
            let saved_conversation_json =
                serde_json::from_str::<serde_json::Value>(&saved_conversation)?;
            match saved_conversation_json
                .get("version")
                .ok_or_else(|| anyhow!("version not found"))?
            {
                serde_json::Value::String(version) => match version.as_str() {
                    SavedConversation::VERSION => Ok(serde_json::from_value::<SavedConversation>(
                        saved_conversation_json,
                    )?),
                    "0.1.0" => {
                        let saved_conversation = serde_json::from_value::<SavedConversationV0_1_0>(
                            saved_conversation_json,
                        )?;
                        Ok(SavedConversation {
                            id: saved_conversation.id,
                            zed: saved_conversation.zed,
                            version: saved_conversation.version,
                            text: saved_conversation.text,
                            messages: saved_conversation.messages,
                            message_metadata: saved_conversation.message_metadata,
                            summary: saved_conversation.summary,
                        })
                    }
                    _ => Err(anyhow!(
                        "unrecognized saved conversation version: {}",
                        version
                    )),
                },
                _ => Err(anyhow!("version not found on saved conversation")),
            }
        })
    }

    pub fn search(&self, query: String, cx: &AppContext) -> Task<Vec<SavedConversationMetadata>> {
        let metadata = self.conversations_metadata.clone();
        let executor = cx.background_executor().clone();
        cx.background_executor().spawn(async move {
            if query.is_empty() {
                metadata
            } else {
                let candidates = metadata
                    .iter()
                    .enumerate()
                    .map(|(id, metadata)| StringMatchCandidate::new(id, metadata.title.clone()))
                    .collect::<Vec<_>>();
                let matches = fuzzy::match_strings(
                    &candidates,
                    &query,
                    false,
                    100,
                    &Default::default(),
                    executor,
                )
                .await;

                matches
                    .into_iter()
                    .map(|mat| metadata[mat.candidate_id].clone())
                    .collect()
            }
        })
    }

    fn reload(&mut self, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let fs = self.fs.clone();
        cx.spawn(|this, mut cx| async move {
            fs.create_dir(&CONVERSATIONS_DIR).await?;

            let mut paths = fs.read_dir(&CONVERSATIONS_DIR).await?;
            let mut conversations = Vec::<SavedConversationMetadata>::new();
            while let Some(path) = paths.next().await {
                let path = path?;
                if path.extension() != Some(OsStr::new("json")) {
                    continue;
                }

                let pattern = r" - \d+.zed.json$";
                let re = Regex::new(pattern).unwrap();

                let metadata = fs.metadata(&path).await?;
                if let Some((file_name, metadata)) = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .zip(metadata)
                {
                    // This is used to filter out conversations saved by the new assistant.
                    if !re.is_match(file_name) {
                        continue;
                    }

                    if let Some(title) = re.replace(file_name, "").lines().next() {
                        conversations.push(SavedConversationMetadata {
                            title: title.to_string(),
                            path,
                            mtime: metadata.mtime.into(),
                        });
                    }
                }
            }
            conversations.sort_unstable_by_key(|conversation| Reverse(conversation.mtime));

            this.update(&mut cx, |this, cx| {
                this.conversations_metadata = conversations;
                cx.notify();
            })
        })
    }
}
