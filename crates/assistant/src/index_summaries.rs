use crate::completion_provider::CompletionProvider;
use crate::{cache_summaries::get_cached_summary, LanguageModelRequest};
use crate::{LanguageModelRequestMessage, Role};
use anyhow::{Context, Result};
use fs::Fs;
use gpui::{AppContext, AsyncAppContext, Task};
use smol::stream::StreamExt;
use std::{path::Path, sync::Arc};

const SUMMARY_PROMPT: &str = "Summarize this file:";

pub fn summarize_file(fs: &impl Fs, path: &Path, cx: &mut AsyncAppContext) -> Task<Result<String>> {
    todo!()
    // cx.background_executor().spawn(async move {
    //     let file_content = fs.load(path).await?;

    //     get_cached_summary(fs, &file_content, || async {
    //         let provider = cx.update(|cx| CompletionProvider::global(cx))?;
    //         generate_summary(&file_content, provider).await
    //     })
    //     .await
    // })
}

async fn generate_summary(file_content: &str, provider: &CompletionProvider) -> Result<String> {
    let request = LanguageModelRequest {
        model: provider.model(),
        messages: vec![
            LanguageModelRequestMessage {
                role: Role::System,
                content: SUMMARY_PROMPT.to_string(),
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: file_content.to_string(),
            },
        ],
        stop: vec![],
        temperature: 0.7,
    };
    let mut messages = provider.complete(request).await?;
    let mut answer = String::new();

    while let Some(chunk) = messages.next().await {
        answer.push_str(&chunk?);
    }

    Ok(answer)
}
