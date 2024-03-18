use crate::{assistant_settings::OpenAiModel, MessageId, MessageMetadata};
use anyhow::{anyhow, Result};
use collections::HashMap;
use fs::Fs;
use futures::StreamExt;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Reverse,
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::paths::CONVERSATIONS_DIR;

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

    pub async fn load(path: &Path, fs: &dyn Fs) -> Result<Self> {
        let saved_conversation = fs.load(path).await?;
        let saved_conversation_json =
            serde_json::from_str::<serde_json::Value>(&saved_conversation)?;
        match saved_conversation_json
            .get("version")
            .ok_or_else(|| anyhow!("version not found"))?
        {
            serde_json::Value::String(version) => match version.as_str() {
                Self::VERSION => Ok(serde_json::from_value::<Self>(saved_conversation_json)?),
                "0.1.0" => {
                    let saved_conversation =
                        serde_json::from_value::<SavedConversationV0_1_0>(saved_conversation_json)?;
                    Ok(Self {
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
    }
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

pub struct SavedConversationMetadata {
    pub title: String,
    pub path: PathBuf,
    pub mtime: chrono::DateTime<chrono::Local>,
}

impl SavedConversationMetadata {
    pub async fn list(fs: Arc<dyn Fs>) -> Result<Vec<Self>> {
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
                let title = re.replace(file_name, "");
                conversations.push(Self {
                    title: title.into_owned(),
                    path,
                    mtime: metadata.mtime.into(),
                });
            }
        }
        conversations.sort_unstable_by_key(|conversation| Reverse(conversation.mtime));

        Ok(conversations)
    }
}
