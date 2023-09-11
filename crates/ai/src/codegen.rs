use crate::{
    stream_completion,
    streaming_diff::{Hunk, StreamingDiff},
    OpenAIRequest,
};
use anyhow::Result;
use editor::{multi_buffer, Anchor, MultiBuffer, ToOffset, ToPoint};
use futures::{
    channel::mpsc, future::BoxFuture, stream::BoxStream, FutureExt, SinkExt, Stream, StreamExt,
};
use gpui::{executor::Background, Entity, ModelContext, ModelHandle, Task};
use language::{IndentSize, Point, Rope, TransactionId};
use std::{cmp, future, ops::Range, sync::Arc};

pub trait CompletionProvider {
    fn complete(
        &self,
        prompt: OpenAIRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>>;
}

pub struct OpenAICompletionProvider {
    api_key: String,
    executor: Arc<Background>,
}

impl OpenAICompletionProvider {
    pub fn new(api_key: String, executor: Arc<Background>) -> Self {
        Self { api_key, executor }
    }
}

impl CompletionProvider for OpenAICompletionProvider {
    fn complete(
        &self,
        prompt: OpenAIRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let request = stream_completion(self.api_key.clone(), self.executor.clone(), prompt);
        async move {
            let response = request.await?;
            let stream = response
                .filter_map(|response| async move {
                    match response {
                        Ok(mut response) => Some(Ok(response.choices.pop()?.delta.content?)),
                        Err(error) => Some(Err(error)),
                    }
                })
                .boxed();
            Ok(stream)
        }
        .boxed()
    }
}

pub enum Event {
    Finished,
    Undone,
}

pub struct Codegen {
    provider: Arc<dyn CompletionProvider>,
    buffer: ModelHandle<MultiBuffer>,
    range: Range<Anchor>,
    last_equal_ranges: Vec<Range<Anchor>>,
    transaction_id: Option<TransactionId>,
    error: Option<anyhow::Error>,
    generation: Task<()>,
    idle: bool,
    _subscription: gpui::Subscription,
}

impl Entity for Codegen {
    type Event = Event;
}

impl Codegen {
    pub fn new(
        buffer: ModelHandle<MultiBuffer>,
        range: Range<Anchor>,
        provider: Arc<dyn CompletionProvider>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        Self {
            provider,
            buffer: buffer.clone(),
            range,
            last_equal_ranges: Default::default(),
            transaction_id: Default::default(),
            error: Default::default(),
            idle: true,
            generation: Task::ready(()),
            _subscription: cx.subscribe(&buffer, Self::handle_buffer_event),
        }
    }

    fn handle_buffer_event(
        &mut self,
        _buffer: ModelHandle<MultiBuffer>,
        event: &multi_buffer::Event,
        cx: &mut ModelContext<Self>,
    ) {
        if let multi_buffer::Event::TransactionUndone { transaction_id } = event {
            if self.transaction_id == Some(*transaction_id) {
                self.transaction_id = None;
                self.generation = Task::ready(());
                cx.emit(Event::Undone);
            }
        }
    }

    pub fn range(&self) -> Range<Anchor> {
        self.range.clone()
    }

    pub fn last_equal_ranges(&self) -> &[Range<Anchor>] {
        &self.last_equal_ranges
    }

    pub fn idle(&self) -> bool {
        self.idle
    }

    pub fn error(&self) -> Option<&anyhow::Error> {
        self.error.as_ref()
    }

    pub fn start(&mut self, prompt: OpenAIRequest, cx: &mut ModelContext<Self>) {
        let range = self.range.clone();
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let selected_text = snapshot
            .text_for_range(range.start..range.end)
            .collect::<Rope>();

        let selection_start = range.start.to_point(&snapshot);
        let selection_end = range.end.to_point(&snapshot);

        let mut base_indent: Option<IndentSize> = None;
        let mut start_row = selection_start.row;
        if snapshot.is_line_blank(start_row) {
            if let Some(prev_non_blank_row) = snapshot.prev_non_blank_row(start_row) {
                start_row = prev_non_blank_row;
            }
        }
        for row in start_row..=selection_end.row {
            if snapshot.is_line_blank(row) {
                continue;
            }

            let line_indent = snapshot.indent_size_for_line(row);
            if let Some(base_indent) = base_indent.as_mut() {
                if line_indent.len < base_indent.len {
                    *base_indent = line_indent;
                }
            } else {
                base_indent = Some(line_indent);
            }
        }

        let mut normalized_selected_text = selected_text.clone();
        if let Some(base_indent) = base_indent {
            for row in selection_start.row..=selection_end.row {
                let selection_row = row - selection_start.row;
                let line_start =
                    normalized_selected_text.point_to_offset(Point::new(selection_row, 0));
                let indent_len = if row == selection_start.row {
                    base_indent.len.saturating_sub(selection_start.column)
                } else {
                    let line_len = normalized_selected_text.line_len(selection_row);
                    cmp::min(line_len, base_indent.len)
                };
                let indent_end = cmp::min(
                    line_start + indent_len as usize,
                    normalized_selected_text.len(),
                );
                normalized_selected_text.replace(line_start..indent_end, "");
            }
        }

        let response = self.provider.complete(prompt);
        self.generation = cx.spawn_weak(|this, mut cx| {
            async move {
                let generate = async {
                    let mut edit_start = range.start.to_offset(&snapshot);

                    let (mut hunks_tx, mut hunks_rx) = mpsc::channel(1);
                    let diff = cx.background().spawn(async move {
                        let chunks = strip_markdown_codeblock(response.await?);
                        futures::pin_mut!(chunks);
                        let mut diff = StreamingDiff::new(selected_text.to_string());

                        let mut indent_len;
                        let indent_text;
                        if let Some(base_indent) = base_indent {
                            indent_len = base_indent.len;
                            indent_text = match base_indent.kind {
                                language::IndentKind::Space => " ",
                                language::IndentKind::Tab => "\t",
                            };
                        } else {
                            indent_len = 0;
                            indent_text = "";
                        };

                        let mut first_line_len = 0;
                        let mut first_line_non_whitespace_char_ix = None;
                        let mut first_line = true;
                        let mut new_text = String::new();

                        while let Some(chunk) = chunks.next().await {
                            let chunk = chunk?;

                            let mut lines = chunk.split('\n');
                            if let Some(mut line) = lines.next() {
                                if first_line {
                                    if first_line_non_whitespace_char_ix.is_none() {
                                        if let Some(mut char_ix) =
                                            line.find(|ch: char| !ch.is_whitespace())
                                        {
                                            line = &line[char_ix..];
                                            char_ix += first_line_len;
                                            first_line_non_whitespace_char_ix = Some(char_ix);
                                            let first_line_indent = char_ix
                                                .saturating_sub(selection_start.column as usize)
                                                as usize;
                                            new_text
                                                .push_str(&indent_text.repeat(first_line_indent));
                                            indent_len = indent_len.saturating_sub(char_ix as u32);
                                        }
                                    }
                                    first_line_len += line.len();
                                }

                                if first_line_non_whitespace_char_ix.is_some() {
                                    new_text.push_str(line);
                                }
                            }

                            for line in lines {
                                first_line = false;
                                new_text.push('\n');
                                if !line.is_empty() {
                                    new_text.push_str(&indent_text.repeat(indent_len as usize));
                                }
                                new_text.push_str(line);
                            }

                            let hunks = diff.push_new(&new_text);
                            hunks_tx.send(hunks).await?;
                            new_text.clear();
                        }
                        hunks_tx.send(diff.finish()).await?;

                        anyhow::Ok(())
                    });

                    while let Some(hunks) = hunks_rx.next().await {
                        let this = if let Some(this) = this.upgrade(&cx) {
                            this
                        } else {
                            break;
                        };

                        this.update(&mut cx, |this, cx| {
                            this.last_equal_ranges.clear();

                            let transaction = this.buffer.update(cx, |buffer, cx| {
                                // Avoid grouping assistant edits with user edits.
                                buffer.finalize_last_transaction(cx);

                                buffer.start_transaction(cx);
                                buffer.edit(
                                    hunks.into_iter().filter_map(|hunk| match hunk {
                                        Hunk::Insert { text } => {
                                            let edit_start = snapshot.anchor_after(edit_start);
                                            Some((edit_start..edit_start, text))
                                        }
                                        Hunk::Remove { len } => {
                                            let edit_end = edit_start + len;
                                            let edit_range = snapshot.anchor_after(edit_start)
                                                ..snapshot.anchor_before(edit_end);
                                            edit_start = edit_end;
                                            Some((edit_range, String::new()))
                                        }
                                        Hunk::Keep { len } => {
                                            let edit_end = edit_start + len;
                                            let edit_range = snapshot.anchor_after(edit_start)
                                                ..snapshot.anchor_before(edit_end);
                                            edit_start += len;
                                            this.last_equal_ranges.push(edit_range);
                                            None
                                        }
                                    }),
                                    None,
                                    cx,
                                );

                                buffer.end_transaction(cx)
                            });

                            if let Some(transaction) = transaction {
                                if let Some(first_transaction) = this.transaction_id {
                                    // Group all assistant edits into the first transaction.
                                    this.buffer.update(cx, |buffer, cx| {
                                        buffer.merge_transactions(
                                            transaction,
                                            first_transaction,
                                            cx,
                                        )
                                    });
                                } else {
                                    this.transaction_id = Some(transaction);
                                    this.buffer.update(cx, |buffer, cx| {
                                        buffer.finalize_last_transaction(cx)
                                    });
                                }
                            }

                            cx.notify();
                        });
                    }

                    diff.await?;
                    anyhow::Ok(())
                };

                let result = generate.await;
                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, cx| {
                        this.last_equal_ranges.clear();
                        this.idle = true;
                        if let Err(error) = result {
                            this.error = Some(error);
                        }
                        cx.emit(Event::Finished);
                        cx.notify();
                    });
                }
            }
        });
        self.error.take();
        self.idle = false;
        cx.notify();
    }

    pub fn undo(&mut self, cx: &mut ModelContext<Self>) {
        if let Some(transaction_id) = self.transaction_id {
            self.buffer
                .update(cx, |buffer, cx| buffer.undo_transaction(transaction_id, cx));
        }
    }
}

fn strip_markdown_codeblock(
    stream: impl Stream<Item = Result<String>>,
) -> impl Stream<Item = Result<String>> {
    let mut first_line = true;
    let mut buffer = String::new();
    let mut starts_with_fenced_code_block = false;
    stream.filter_map(move |chunk| {
        let chunk = match chunk {
            Ok(chunk) => chunk,
            Err(err) => return future::ready(Some(Err(err))),
        };
        buffer.push_str(&chunk);

        if first_line {
            if buffer == "" || buffer == "`" || buffer == "``" {
                return future::ready(None);
            } else if buffer.starts_with("```") {
                starts_with_fenced_code_block = true;
                if let Some(newline_ix) = buffer.find('\n') {
                    buffer.replace_range(..newline_ix + 1, "");
                    first_line = false;
                } else {
                    return future::ready(None);
                }
            }
        }

        let text = if starts_with_fenced_code_block {
            buffer
                .strip_suffix("\n```\n")
                .or_else(|| buffer.strip_suffix("\n```"))
                .or_else(|| buffer.strip_suffix("\n``"))
                .or_else(|| buffer.strip_suffix("\n`"))
                .or_else(|| buffer.strip_suffix('\n'))
                .unwrap_or(&buffer)
        } else {
            &buffer
        };

        if text.contains('\n') {
            first_line = false;
        }

        let remainder = buffer.split_off(text.len());
        let result = if buffer.is_empty() {
            None
        } else {
            Some(Ok(buffer.clone()))
        };
        buffer = remainder;
        future::ready(result)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream;
    use gpui::{executor::Deterministic, TestAppContext};
    use indoc::indoc;
    use language::{tree_sitter_rust, Buffer, Language, LanguageConfig};
    use parking_lot::Mutex;
    use rand::prelude::*;

    #[gpui::test(iterations = 10)]
    async fn test_autoindent(
        cx: &mut TestAppContext,
        mut rng: StdRng,
        deterministic: Arc<Deterministic>,
    ) {
        let text = indoc! {"
            fn main() {
                let x = 0;
                for _ in 0..10 {
                    x += 1;
                }
            }
        "};
        let buffer =
            cx.add_model(|cx| Buffer::new(0, 0, text).with_language(Arc::new(rust_lang()), cx));
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 4))..snapshot.anchor_after(Point::new(4, 4))
        });
        let provider = Arc::new(TestCompletionProvider::new());
        let codegen = cx.add_model(|cx| Codegen::new(buffer.clone(), range, provider.clone(), cx));
        codegen.update(cx, |codegen, cx| codegen.start(Default::default(), cx));

        let mut new_text = indoc! {"
                   let mut x = 0;
            while x < 10 {
                           x += 1;
               }
        "};
        while !new_text.is_empty() {
            let max_len = cmp::min(new_text.len(), 10);
            let len = rng.gen_range(1..=max_len);
            let (chunk, suffix) = new_text.split_at(len);
            provider.send_completion(chunk);
            new_text = suffix;
            deterministic.run_until_parked();
        }
        provider.finish_completion();
        deterministic.run_until_parked();

        assert_eq!(
            buffer.read_with(cx, |buffer, cx| buffer.snapshot(cx).text()),
            indoc! {"
                fn main() {
                    let mut x = 0;
                    while x < 10 {
                        x += 1;
                    }
                }
            "}
        );
    }

    #[gpui::test]
    async fn test_strip_markdown_codeblock() {
        assert_eq!(
            strip_markdown_codeblock(chunks("Lorem ipsum dolor", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum dolor"
        );
        assert_eq!(
            strip_markdown_codeblock(chunks("```\nLorem ipsum dolor", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum dolor"
        );
        assert_eq!(
            strip_markdown_codeblock(chunks("```\nLorem ipsum dolor\n```", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum dolor"
        );
        assert_eq!(
            strip_markdown_codeblock(chunks("```\nLorem ipsum dolor\n```\n", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum dolor"
        );
        assert_eq!(
            strip_markdown_codeblock(chunks("```html\n```js\nLorem ipsum dolor\n```\n```", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "```js\nLorem ipsum dolor\n```"
        );
        assert_eq!(
            strip_markdown_codeblock(chunks("``\nLorem ipsum dolor\n```", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "``\nLorem ipsum dolor\n```"
        );

        fn chunks(text: &str, size: usize) -> impl Stream<Item = Result<String>> {
            stream::iter(
                text.chars()
                    .collect::<Vec<_>>()
                    .chunks(size)
                    .map(|chunk| Ok(chunk.iter().collect::<String>()))
                    .collect::<Vec<_>>(),
            )
        }
    }

    struct TestCompletionProvider {
        last_completion_tx: Mutex<Option<mpsc::Sender<String>>>,
    }

    impl TestCompletionProvider {
        fn new() -> Self {
            Self {
                last_completion_tx: Mutex::new(None),
            }
        }

        fn send_completion(&self, completion: impl Into<String>) {
            let mut tx = self.last_completion_tx.lock();
            tx.as_mut().unwrap().try_send(completion.into()).unwrap();
        }

        fn finish_completion(&self) {
            self.last_completion_tx.lock().take().unwrap();
        }
    }

    impl CompletionProvider for TestCompletionProvider {
        fn complete(
            &self,
            _prompt: OpenAIRequest,
        ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
            let (tx, rx) = mpsc::channel(1);
            *self.last_completion_tx.lock() = Some(tx);
            async move { Ok(rx.map(|rx| Ok(rx)).boxed()) }.boxed()
        }
    }

    fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        )
        .with_indents_query(
            r#"
            (call_expression) @indent
            (field_expression) @indent
            (_ "(" ")" @end) @indent
            (_ "{" "}" @end) @indent
            "#,
        )
        .unwrap()
    }
}
