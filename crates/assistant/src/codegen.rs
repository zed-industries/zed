use crate::{
    stream_completion,
    streaming_diff::{Hunk, StreamingDiff},
    OpenAIRequest,
};
use anyhow::Result;
use editor::{
    multi_buffer, Anchor, AnchorRangeExt, MultiBuffer, MultiBufferSnapshot, ToOffset, ToPoint,
};
use futures::{
    channel::mpsc, future::BoxFuture, stream::BoxStream, FutureExt, SinkExt, Stream, StreamExt,
};
use gpui::{executor::Background, Entity, ModelContext, ModelHandle, Task};
use language::{Rope, TransactionId};
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

#[derive(Clone)]
pub enum CodegenKind {
    Transform { range: Range<Anchor> },
    Generate { position: Anchor },
}

pub struct Codegen {
    provider: Arc<dyn CompletionProvider>,
    buffer: ModelHandle<MultiBuffer>,
    snapshot: MultiBufferSnapshot,
    kind: CodegenKind,
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
        mut kind: CodegenKind,
        provider: Arc<dyn CompletionProvider>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let snapshot = buffer.read(cx).snapshot(cx);
        match &mut kind {
            CodegenKind::Transform { range } => {
                let mut point_range = range.to_point(&snapshot);
                point_range.start.column = 0;
                if point_range.end.column > 0 || point_range.start.row == point_range.end.row {
                    point_range.end.column = snapshot.line_len(point_range.end.row);
                }
                range.start = snapshot.anchor_before(point_range.start);
                range.end = snapshot.anchor_after(point_range.end);
            }
            CodegenKind::Generate { position } => {
                *position = position.bias_right(&snapshot);
            }
        }

        Self {
            provider,
            buffer: buffer.clone(),
            snapshot,
            kind,
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
        match &self.kind {
            CodegenKind::Transform { range } => range.clone(),
            CodegenKind::Generate { position } => position.bias_left(&self.snapshot)..*position,
        }
    }

    pub fn kind(&self) -> &CodegenKind {
        &self.kind
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
        let range = self.range();
        let snapshot = self.snapshot.clone();
        let selected_text = snapshot
            .text_for_range(range.start..range.end)
            .collect::<Rope>();

        let selection_start = range.start.to_point(&snapshot);
        let suggested_line_indent = snapshot
            .suggested_indents(selection_start.row..selection_start.row + 1, cx)
            .into_values()
            .next()
            .unwrap_or_else(|| snapshot.indent_size_for_line(selection_start.row));

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

                        let mut new_text = String::new();
                        let mut base_indent = None;
                        let mut line_indent = None;
                        let mut first_line = true;

                        while let Some(chunk) = chunks.next().await {
                            let chunk = chunk?;

                            let mut lines = chunk.split('\n').peekable();
                            while let Some(line) = lines.next() {
                                new_text.push_str(line);
                                if line_indent.is_none() {
                                    if let Some(non_whitespace_ch_ix) =
                                        new_text.find(|ch: char| !ch.is_whitespace())
                                    {
                                        line_indent = Some(non_whitespace_ch_ix);
                                        base_indent = base_indent.or(line_indent);

                                        let line_indent = line_indent.unwrap();
                                        let base_indent = base_indent.unwrap();
                                        let indent_delta = line_indent as i32 - base_indent as i32;
                                        let mut corrected_indent_len = cmp::max(
                                            0,
                                            suggested_line_indent.len as i32 + indent_delta,
                                        )
                                            as usize;
                                        if first_line {
                                            corrected_indent_len = corrected_indent_len
                                                .saturating_sub(selection_start.column as usize);
                                        }

                                        let indent_char = suggested_line_indent.char();
                                        let mut indent_buffer = [0; 4];
                                        let indent_str =
                                            indent_char.encode_utf8(&mut indent_buffer);
                                        new_text.replace_range(
                                            ..line_indent,
                                            &indent_str.repeat(corrected_indent_len),
                                        );
                                    }
                                }

                                if line_indent.is_some() {
                                    hunks_tx.send(diff.push_new(&new_text)).await?;
                                    new_text.clear();
                                }

                                if lines.peek().is_some() {
                                    hunks_tx.send(diff.push_new("\n")).await?;
                                    line_indent = None;
                                    first_line = false;
                                }
                            }
                        }
                        hunks_tx.send(diff.push_new(&new_text)).await?;
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
                                            edit_start = edit_end;
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
    use language::{language_settings, tree_sitter_rust, Buffer, Language, LanguageConfig, Point};
    use parking_lot::Mutex;
    use rand::prelude::*;
    use settings::SettingsStore;

    #[gpui::test(iterations = 10)]
    async fn test_transform_autoindent(
        cx: &mut TestAppContext,
        mut rng: StdRng,
        deterministic: Arc<Deterministic>,
    ) {
        cx.set_global(cx.read(SettingsStore::test));
        cx.update(language_settings::init);

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
        let codegen = cx.add_model(|cx| {
            Codegen::new(
                buffer.clone(),
                CodegenKind::Transform { range },
                provider.clone(),
                cx,
            )
        });
        codegen.update(cx, |codegen, cx| codegen.start(Default::default(), cx));

        let mut new_text = concat!(
            "       let mut x = 0;\n",
            "       while x < 10 {\n",
            "           x += 1;\n",
            "       }",
        );
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

    #[gpui::test(iterations = 10)]
    async fn test_autoindent_when_generating_past_indentation(
        cx: &mut TestAppContext,
        mut rng: StdRng,
        deterministic: Arc<Deterministic>,
    ) {
        cx.set_global(cx.read(SettingsStore::test));
        cx.update(language_settings::init);

        let text = indoc! {"
            fn main() {
                le
            }
        "};
        let buffer =
            cx.add_model(|cx| Buffer::new(0, 0, text).with_language(Arc::new(rust_lang()), cx));
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        let position = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 6))
        });
        let provider = Arc::new(TestCompletionProvider::new());
        let codegen = cx.add_model(|cx| {
            Codegen::new(
                buffer.clone(),
                CodegenKind::Generate { position },
                provider.clone(),
                cx,
            )
        });
        codegen.update(cx, |codegen, cx| codegen.start(Default::default(), cx));

        let mut new_text = concat!(
            "t mut x = 0;\n",
            "while x < 10 {\n",
            "    x += 1;\n",
            "}", //
        );
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

    #[gpui::test(iterations = 10)]
    async fn test_autoindent_when_generating_before_indentation(
        cx: &mut TestAppContext,
        mut rng: StdRng,
        deterministic: Arc<Deterministic>,
    ) {
        cx.set_global(cx.read(SettingsStore::test));
        cx.update(language_settings::init);

        let text = concat!(
            "fn main() {\n",
            "  \n",
            "}\n" //
        );
        let buffer =
            cx.add_model(|cx| Buffer::new(0, 0, text).with_language(Arc::new(rust_lang()), cx));
        let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
        let position = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 2))
        });
        let provider = Arc::new(TestCompletionProvider::new());
        let codegen = cx.add_model(|cx| {
            Codegen::new(
                buffer.clone(),
                CodegenKind::Generate { position },
                provider.clone(),
                cx,
            )
        });
        codegen.update(cx, |codegen, cx| codegen.start(Default::default(), cx));

        let mut new_text = concat!(
            "let mut x = 0;\n",
            "while x < 10 {\n",
            "    x += 1;\n",
            "}", //
        );
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
