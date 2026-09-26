use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;
use std::time::Duration;

use anyhow::Result;
use editor::{CompletionContext, CompletionProvider, Editor};
use gpui::{Context, Entity, Task, WeakEntity, Window};
use language::{self, CodeLabel, ToOffset, ToPoint};
use project::{
    Completion, CompletionDisplayOptions, CompletionResponse, CompletionSource, Project,
    lsp_store::CompletionDocumentation,
};
use runtimelib::media::MediaType;

use crate::repl_editor::{CompletionChunk, completion_chunk};
use crate::session::Session;

pub(crate) struct JupyterCompletionProvider {
    project: Entity<Project>,
    session: WeakEntity<Session>,
}

impl JupyterCompletionProvider {
    pub fn new(project: Entity<Project>, session: WeakEntity<Session>) -> Self {
        Self { project, session }
    }
}

impl CompletionProvider for JupyterCompletionProvider {
    fn completions(
        &self,
        buffer: &Entity<language::Buffer>,
        buffer_position: language::Anchor,
        trigger: CompletionContext,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Vec<CompletionResponse>>> {
        let lsp_completions =
            self.project
                .completions(buffer, buffer_position, trigger, window, cx);

        let snapshot = buffer.read(cx).snapshot();
        let cursor_point = buffer_position.to_point(&snapshot);
        let chunk = completion_chunk(&snapshot, cursor_point, cx);

        let jupyter_receiver = chunk.as_ref().and_then(|chunk| {
            let session = self.session.upgrade()?;
            session.update(cx, |session, cx| {
                session.request_completions(chunk.code.clone(), chunk.cursor_pos, cx)
            })
        });

        let timer = cx.background_executor().timer(Duration::from_secs(5));
        cx.spawn(async move |_, _cx| {
            let mut responses = lsp_completions.await.unwrap_or_default();

            if let (Some(rx), Some(chunk)) = (jupyter_receiver, chunk) {
                let reply = match futures::future::select(std::pin::pin!(rx), std::pin::pin!(timer))
                    .await
                {
                    futures::future::Either::Left((Ok(reply), _)) => Some(reply),
                    _ => None,
                };

                if let Some(reply) = reply
                    && reply.status == runtimelib::ReplyStatus::Ok
                {
                    if let Some(response) = jupyter_reply_to_completion_response(
                        &reply,
                        &chunk,
                        &snapshot,
                        buffer_position,
                    ) {
                        responses.push(response);
                    }
                }
            }

            Ok(responses)
        })
    }

    fn resolve_completions(
        &self,
        buffer: Entity<language::Buffer>,
        completion_indices: Vec<usize>,
        completions: Rc<RefCell<Box<[Completion]>>>,
        cx: &mut Context<Editor>,
    ) -> Task<Result<bool>> {
        let mut lsp_indices = Vec::new();
        let mut jupyter_indices = Vec::new();

        for &index in &completion_indices {
            match &completions.borrow()[index].source {
                CompletionSource::Custom => jupyter_indices.push(index),
                _ => lsp_indices.push(index),
            }
        }

        let lsp_task = if lsp_indices.is_empty() {
            Task::ready(Ok(false))
        } else {
            self.project
                .resolve_completions(buffer, lsp_indices, completions.clone(), cx)
        };

        let jupyter_receivers: Vec<_> = jupyter_indices
            .iter()
            .filter_map(|&index| {
                let session = self.session.upgrade()?;
                let new_text = completions.borrow()[index].new_text.clone();
                let cursor_pos = new_text.chars().count();
                let rx = session.update(cx, |session, cx| {
                    session.request_inspect(new_text, cursor_pos, cx)
                })?;
                Some((index, rx))
            })
            .collect();

        let executor = cx.background_executor().clone();
        cx.spawn(async move |_, _cx| {
            let lsp_resolved = lsp_task.await.unwrap_or(false);

            let mut jupyter_resolved = false;
            for (index, rx) in jupyter_receivers {
                let timeout = executor.timer(Duration::from_secs(5));
                let reply = match futures::future::select(
                    std::pin::pin!(rx),
                    std::pin::pin!(timeout),
                )
                .await
                {
                    futures::future::Either::Left((Ok(reply), _)) => Some(reply),
                    _ => None,
                };

                if let Some(reply) = reply
                    && reply.found
                    && reply.status == runtimelib::ReplyStatus::Ok
                {
                    if let Some(documentation) = inspect_reply_to_documentation(&reply) {
                        completions.borrow_mut()[index].documentation = Some(documentation);
                        jupyter_resolved = true;
                    }
                }
            }

            Ok(lsp_resolved || jupyter_resolved)
        })
    }

    fn apply_additional_edits_for_completion(
        &self,
        buffer: Entity<language::Buffer>,
        completions: Rc<RefCell<Box<[Completion]>>>,
        completion_index: usize,
        push_to_history: bool,
        all_commit_ranges: Vec<Range<language::Anchor>>,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Option<language::Transaction>>> {
        self.project.apply_additional_edits_for_completion(
            buffer,
            completions,
            completion_index,
            push_to_history,
            all_commit_ranges,
            cx,
        )
    }

    fn is_completion_trigger(
        &self,
        buffer: &Entity<language::Buffer>,
        position: language::Anchor,
        text: &str,
        trigger_in_words: bool,
        cx: &mut Context<Editor>,
    ) -> bool {
        self.project
            .is_completion_trigger(buffer, position, text, trigger_in_words, cx)
    }

    fn sort_completions(&self) -> bool {
        true
    }

    fn filter_completions(&self) -> bool {
        true
    }

    fn show_snippets(&self) -> bool {
        true
    }
}

fn char_offset_to_byte_offset(text: &str, char_offset: usize) -> usize {
    text.char_indices()
        .nth(char_offset)
        .map(|(byte_idx, _)| byte_idx)
        .unwrap_or(text.len())
}

fn jupyter_reply_to_completion_response(
    reply: &runtimelib::CompleteReply,
    chunk: &CompletionChunk,
    snapshot: &language::BufferSnapshot,
    buffer_position: language::Anchor,
) -> Option<CompletionResponse> {
    if reply.matches.is_empty() {
        return None;
    }

    let start_byte = chunk.start_byte + char_offset_to_byte_offset(&chunk.code, reply.cursor_start);
    let end_byte = chunk.start_byte + char_offset_to_byte_offset(&chunk.code, reply.cursor_end);

    let replace_start = snapshot.anchor_after(start_byte);
    let replace_end = snapshot.anchor_before(end_byte);
    let replace_range = replace_start..replace_end;

    let byte_offset = buffer_position.to_offset(snapshot);
    let match_start_byte = start_byte.min(byte_offset);
    let match_start = snapshot.anchor_after(match_start_byte);

    let completions = reply
        .matches
        .iter()
        .map(|match_text| Completion {
            replace_range: replace_range.clone(),
            new_text: match_text.clone(),
            label: CodeLabel::filtered(
                format!("{match_text} Jupyter"),
                match_text.len(),
                None,
                vec![],
            ),
            documentation: None,
            source: CompletionSource::Custom,
            icon_path: None,
            match_start: Some(match_start),
            snippet_deduplication_key: None,
            insert_text_mode: None,
            confirm: None,
            group: None,
        })
        .collect();

    Some(CompletionResponse {
        completions,
        display_options: CompletionDisplayOptions::default(),
        is_incomplete: false,
    })
}

fn inspect_reply_to_documentation(
    reply: &runtimelib::InspectReply,
) -> Option<CompletionDocumentation> {
    for media_type in &reply.data.content {
        if let MediaType::Markdown(text) = media_type {
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                return Some(CompletionDocumentation::MultiLineMarkdown(
                    trimmed.to_string().into(),
                ));
            }
        }
    }

    for media_type in &reply.data.content {
        if let MediaType::Plain(text) = media_type {
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                return Some(CompletionDocumentation::MultiLinePlainText(
                    trimmed.to_string().into(),
                ));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{App, AppContext as _};
    use language::Buffer;

    fn make_chunk(code: &str, start_byte: usize) -> CompletionChunk {
        CompletionChunk {
            code: code.into(),
            cursor_pos: code.chars().count(),
            start_byte,
        }
    }

    #[gpui::test]
    fn test_jupyter_reply_empty_matches(cx: &mut App) {
        let buffer = cx.new(|cx| Buffer::local("foo bar", cx));
        let snapshot = buffer.read(cx).snapshot();
        let buffer_position = snapshot.anchor_after(4);
        let chunk = make_chunk("foo bar", 0);
        let reply = runtimelib::CompleteReply::default();

        assert!(
            jupyter_reply_to_completion_response(&reply, &chunk, &snapshot, buffer_position)
                .is_none()
        );
    }

    #[gpui::test]
    fn test_jupyter_reply_translates_offsets(cx: &mut App) {
        // Buffer matches the chunk one-to-one; chunk starts at byte 0.
        let text = "import numpy as np\nnp.ar";
        let buffer = cx.new(|cx| Buffer::local(text, cx));
        let snapshot = buffer.read(cx).snapshot();
        let buffer_position = snapshot.anchor_after(text.len());
        let chunk = make_chunk(text, 0);
        let reply = runtimelib::CompleteReply {
            matches: vec!["np.array".into(), "np.arange".into()],
            cursor_start: 19, // start of "np.ar" (after "import numpy as np\n")
            cursor_end: 24,   // end of "np.ar"
            ..Default::default()
        };

        let response =
            jupyter_reply_to_completion_response(&reply, &chunk, &snapshot, buffer_position)
                .unwrap();
        assert_eq!(response.completions.len(), 2);
        let completion = &response.completions[0];
        assert_eq!(completion.replace_range.start.to_offset(&snapshot), 19);
        assert_eq!(completion.replace_range.end.to_offset(&snapshot), 24);
        assert_eq!(completion.new_text, "np.array");
        // In the common case, `match_start` equals the replace-range start.
        assert_eq!(completion.match_start.unwrap().to_offset(&snapshot), 19);
    }

    #[gpui::test]
    fn test_jupyter_reply_chunk_offset_into_buffer(cx: &mut App) {
        // The chunk lives at byte 12 of the buffer; replace_range must be
        // shifted by chunk.start_byte to land in the right buffer span.
        let prelude = "x = 1\ny = 2\n"; // 12 bytes
        let chunk_text = "z.fo";
        let mut text = String::from(prelude);
        text.push_str(chunk_text);
        let buffer = cx.new({
            let text = text.clone();
            |cx| Buffer::local(text, cx)
        });
        let snapshot = buffer.read(cx).snapshot();
        let buffer_position = snapshot.anchor_after(text.len());
        let chunk = make_chunk(chunk_text, 12);
        let reply = runtimelib::CompleteReply {
            matches: vec!["z.foo".into()],
            cursor_start: 0,
            cursor_end: 4,
            ..Default::default()
        };

        let response =
            jupyter_reply_to_completion_response(&reply, &chunk, &snapshot, buffer_position)
                .unwrap();
        let completion = &response.completions[0];
        assert_eq!(completion.replace_range.start.to_offset(&snapshot), 12);
        assert_eq!(completion.replace_range.end.to_offset(&snapshot), 16);
    }

    #[gpui::test]
    fn test_jupyter_reply_multibyte_offsets(cx: &mut App) {
        // "π" is 2 bytes / 1 char, so char-offset and byte-offset diverge.
        let text = "x = π.";
        let buffer = cx.new(|cx| Buffer::local(text, cx));
        let snapshot = buffer.read(cx).snapshot();
        let buffer_position = snapshot.anchor_after(text.len());
        let chunk = make_chunk(text, 0);
        let reply = runtimelib::CompleteReply {
            matches: vec!["π.bit_length".into()],
            cursor_start: 4, // char offset of "π"
            cursor_end: 6,   // char offset after "."
            ..Default::default()
        };

        let response =
            jupyter_reply_to_completion_response(&reply, &chunk, &snapshot, buffer_position)
                .unwrap();
        let completion = &response.completions[0];
        // char offset 4 → byte 4 ("x = " = 4 bytes)
        // char offset 6 → byte 7 ("x = π." = 4 + 2 + 1 = 7 bytes)
        assert_eq!(completion.replace_range.start.to_offset(&snapshot), 4);
        assert_eq!(completion.replace_range.end.to_offset(&snapshot), 7);
    }
}
