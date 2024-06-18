use crate::{assistant_settings::OpenAiModel, MessageId, MessageMetadata};
use anyhow::{anyhow, Result};
use assistant_slash_command::SlashCommandOutputSection;
use collections::HashMap;
use fs::Fs;
use futures::StreamExt;
use fuzzy::StringMatchCandidate;
use gpui::{AppContext, Model, ModelContext, Task};
use paths::contexts_dir;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{cmp::Reverse, ffi::OsStr, path::PathBuf, sync::Arc, time::Duration};
use ui::Context;
use util::{ResultExt, TryFutureExt};

#[derive(Serialize, Deserialize)]
pub struct SavedMessage {
    pub id: MessageId,
    pub start: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SavedContext {
    pub id: Option<String>,
    pub zed: String,
    pub version: String,
    pub text: String,
    pub messages: Vec<SavedMessage>,
    pub message_metadata: HashMap<MessageId, MessageMetadata>,
    pub summary: String,
    pub slash_command_output_sections: Vec<SlashCommandOutputSection<usize>>,
}

impl SavedContext {
    pub const VERSION: &'static str = "0.3.0";
}

#[derive(Serialize, Deserialize)]
pub struct SavedContextV0_2_0 {
    pub id: Option<String>,
    pub zed: String,
    pub version: String,
    pub text: String,
    pub messages: Vec<SavedMessage>,
    pub message_metadata: HashMap<MessageId, MessageMetadata>,
    pub summary: String,
}

#[derive(Serialize, Deserialize)]
struct SavedContextV0_1_0 {
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
pub struct SavedContextMetadata {
    pub title: String,
    pub path: PathBuf,
    pub mtime: chrono::DateTime<chrono::Local>,
}

pub struct ContextStore {
    contexts_metadata: Vec<SavedContextMetadata>,
    fs: Arc<dyn Fs>,
    _watch_updates: Task<Option<()>>,
}

impl ContextStore {
    pub fn new(fs: Arc<dyn Fs>, cx: &mut AppContext) -> Task<Result<Model<Self>>> {
        cx.spawn(|mut cx| async move {
            const CONTEXT_WATCH_DURATION: Duration = Duration::from_millis(100);
            let (mut events, _) = fs.watch(contexts_dir(), CONTEXT_WATCH_DURATION).await;

            let this = cx.new_model(|cx: &mut ModelContext<Self>| Self {
                contexts_metadata: Vec::new(),
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

    pub fn load(&self, path: PathBuf, cx: &AppContext) -> Task<Result<SavedContext>> {
        let fs = self.fs.clone();
        cx.background_executor().spawn(async move {
            let saved_context = fs.load(&path).await?;
            let saved_context_json = serde_json::from_str::<serde_json::Value>(&saved_context)?;
            match saved_context_json
                .get("version")
                .ok_or_else(|| anyhow!("version not found"))?
            {
                serde_json::Value::String(version) => match version.as_str() {
                    SavedContext::VERSION => {
                        Ok(serde_json::from_value::<SavedContext>(saved_context_json)?)
                    }
                    "0.2.0" => {
                        let saved_context =
                            serde_json::from_value::<SavedContextV0_2_0>(saved_context_json)?;
                        Ok(SavedContext {
                            id: saved_context.id,
                            zed: saved_context.zed,
                            version: saved_context.version,
                            text: saved_context.text,
                            messages: saved_context.messages,
                            message_metadata: saved_context.message_metadata,
                            summary: saved_context.summary,
                            slash_command_output_sections: Vec::new(),
                        })
                    }
                    "0.1.0" => {
                        let saved_context =
                            serde_json::from_value::<SavedContextV0_1_0>(saved_context_json)?;
                        Ok(SavedContext {
                            id: saved_context.id,
                            zed: saved_context.zed,
                            version: saved_context.version,
                            text: saved_context.text,
                            messages: saved_context.messages,
                            message_metadata: saved_context.message_metadata,
                            summary: saved_context.summary,
                            slash_command_output_sections: Vec::new(),
                        })
                    }
                    _ => Err(anyhow!("unrecognized saved context version: {}", version)),
                },
                _ => Err(anyhow!("version not found on saved context")),
            }
        })
    }

    pub fn search(&self, query: String, cx: &AppContext) -> Task<Vec<SavedContextMetadata>> {
        let metadata = self.contexts_metadata.clone();
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
            fs.create_dir(contexts_dir()).await?;

            let mut paths = fs.read_dir(contexts_dir()).await?;
            let mut contexts = Vec::<SavedContextMetadata>::new();
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
                    // This is used to filter out contexts saved by the new assistant.
                    if !re.is_match(file_name) {
                        continue;
                    }

                    if let Some(title) = re.replace(file_name, "").lines().next() {
                        contexts.push(SavedContextMetadata {
                            title: title.to_string(),
                            path,
                            mtime: metadata.mtime.into(),
                        });
                    }
                }
            }
            contexts.sort_unstable_by_key(|context| Reverse(context.mtime));

            this.update(&mut cx, |this, cx| {
                this.contexts_metadata = contexts;
                cx.notify();
            })
        })
    }
}
