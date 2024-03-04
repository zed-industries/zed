use crate::streaming_diff::{Hunk, StreamingDiff};
use ai::completion::{CompletionProvider, CompletionRequest};
use anyhow::Result;
use editor::{Anchor, MultiBuffer, MultiBufferSnapshot, ToOffset, ToPoint};
use futures::{channel::mpsc, SinkExt, Stream, StreamExt};
use gpui::{EventEmitter, Model, ModelContext, Task};
use language::{Rope, TransactionId};
use multi_buffer;
use std::{cmp, future, ops::Range, sync::Arc};

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
    buffer: Model<MultiBuffer>,
    snapshot: MultiBufferSnapshot,
    kind: CodegenKind,
    last_equal_ranges: Vec<Range<Anchor>>,
    transaction_id: Option<TransactionId>,
    error: Option<anyhow::Error>,
    generation: Task<()>,
    idle: bool,
    _subscription: gpui::Subscription,
}

impl EventEmitter<Event> for Codegen {}

impl Codegen {
    pub fn new(
        buffer: Model<MultiBuffer>,
        kind: CodegenKind,
        provider: Arc<dyn CompletionProvider>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let snapshot = buffer.read(cx).snapshot(cx);
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
        _buffer: Model<MultiBuffer>,
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

    pub fn start(&mut self, prompt: Box<dyn CompletionRequest>, cx: &mut ModelContext<Self>) {
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
        self.generation = cx.spawn(|this, mut cx| {
            async move {
                let generate = async {
                    let mut edit_start = range.start.to_offset(&snapshot);

                    let (mut hunks_tx, mut hunks_rx) = mpsc::channel(1);
                    let diff = cx.background_executor().spawn(async move {
                        let chunks = strip_invalid_spans_from_codeblock(response.await?);
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
                        })?;
                    }

                    diff.await?;
                    anyhow::Ok(())
                };

                let result = generate.await;
                this.update(&mut cx, |this, cx| {
                    this.last_equal_ranges.clear();
                    this.idle = true;
                    if let Err(error) = result {
                        this.error = Some(error);
                    }
                    cx.emit(Event::Finished);
                    cx.notify();
                })
                .ok();
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

fn strip_invalid_spans_from_codeblock(
    stream: impl Stream<Item = Result<String>>,
) -> impl Stream<Item = Result<String>> {
    let mut first_line = true;
    let mut buffer = String::new();
    let mut starts_with_markdown_codeblock = false;
    let mut includes_start_or_end_span = false;
    stream.filter_map(move |chunk| {
        let chunk = match chunk {
            Ok(chunk) => chunk,
            Err(err) => return future::ready(Some(Err(err))),
        };
        buffer.push_str(&chunk);

        if buffer.len() > "<|S|".len() && buffer.starts_with("<|S|") {
            includes_start_or_end_span = true;

            buffer = buffer
                .strip_prefix("<|S|>")
                .or_else(|| buffer.strip_prefix("<|S|"))
                .unwrap_or(&buffer)
                .to_string();
        } else if buffer.ends_with("|E|>") {
            includes_start_or_end_span = true;
        } else if buffer.starts_with("<|")
            || buffer.starts_with("<|S")
            || buffer.starts_with("<|S|")
            || buffer.ends_with('|')
            || buffer.ends_with("|E")
            || buffer.ends_with("|E|")
        {
            return future::ready(None);
        }

        if first_line {
            if buffer == "" || buffer == "`" || buffer == "``" {
                return future::ready(None);
            } else if buffer.starts_with("```") {
                starts_with_markdown_codeblock = true;
                if let Some(newline_ix) = buffer.find('\n') {
                    buffer.replace_range(..newline_ix + 1, "");
                    first_line = false;
                } else {
                    return future::ready(None);
                }
            }
        }

        let mut text = buffer.to_string();
        if starts_with_markdown_codeblock {
            text = text
                .strip_suffix("\n```\n")
                .or_else(|| text.strip_suffix("\n```"))
                .or_else(|| text.strip_suffix("\n``"))
                .or_else(|| text.strip_suffix("\n`"))
                .or_else(|| text.strip_suffix('\n'))
                .unwrap_or(&text)
                .to_string();
        }

        if includes_start_or_end_span {
            text = text
                .strip_suffix("|E|>")
                .or_else(|| text.strip_suffix("E|>"))
                .or_else(|| text.strip_prefix("|>"))
                .or_else(|| text.strip_prefix('>'))
                .unwrap_or(&text)
                .to_string();
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
    use std::sync::Arc;

    use super::*;
    use ai::test::FakeCompletionProvider;
    use futures::stream::{self};
    use gpui::{Context, TestAppContext};
    use indoc::indoc;
    use language::{
        language_settings, tree_sitter_rust, Buffer, BufferId, Language, LanguageConfig,
        LanguageMatcher, Point,
    };
    use rand::prelude::*;
    use serde::Serialize;
    use settings::SettingsStore;

    #[derive(Serialize)]
    pub struct DummyCompletionRequest {
        pub name: String,
    }

    impl CompletionRequest for DummyCompletionRequest {
        fn data(&self) -> serde_json::Result<String> {
            serde_json::to_string(self)
        }
    }

    #[gpui::test(iterations = 10)]
    async fn test_transform_autoindent(cx: &mut TestAppContext, mut rng: StdRng) {
        cx.set_global(cx.update(SettingsStore::test));
        cx.update(language_settings::init);

        let text = indoc! {"
            fn main() {
                let x = 0;
                for _ in 0..10 {
                    x += 1;
                }
            }
        "};
        let buffer = cx.new_model(|cx| {
            Buffer::new(0, BufferId::new(1).unwrap(), text).with_language(Arc::new(rust_lang()), cx)
        });
        let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
        let range = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 0))..snapshot.anchor_after(Point::new(4, 5))
        });
        let provider = Arc::new(FakeCompletionProvider::new());
        let codegen = cx.new_model(|cx| {
            Codegen::new(
                buffer.clone(),
                CodegenKind::Transform { range },
                provider.clone(),
                cx,
            )
        });

        let request = Box::new(DummyCompletionRequest {
            name: "test".to_string(),
        });
        codegen.update(cx, |codegen, cx| codegen.start(request, cx));

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
            println!("CHUNK: {:?}", &chunk);
            provider.send_completion(chunk);
            new_text = suffix;
            cx.background_executor.run_until_parked();
        }
        provider.finish_completion();
        cx.background_executor.run_until_parked();

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
    ) {
        cx.set_global(cx.update(SettingsStore::test));
        cx.update(language_settings::init);

        let text = indoc! {"
            fn main() {
                le
            }
        "};
        let buffer = cx.new_model(|cx| {
            Buffer::new(0, BufferId::new(1).unwrap(), text).with_language(Arc::new(rust_lang()), cx)
        });
        let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
        let position = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 6))
        });
        let provider = Arc::new(FakeCompletionProvider::new());
        let codegen = cx.new_model(|cx| {
            Codegen::new(
                buffer.clone(),
                CodegenKind::Generate { position },
                provider.clone(),
                cx,
            )
        });

        let request = Box::new(DummyCompletionRequest {
            name: "test".to_string(),
        });
        codegen.update(cx, |codegen, cx| codegen.start(request, cx));

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
            cx.background_executor.run_until_parked();
        }
        provider.finish_completion();
        cx.background_executor.run_until_parked();

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
    ) {
        cx.set_global(cx.update(SettingsStore::test));
        cx.update(language_settings::init);

        let text = concat!(
            "fn main() {\n",
            "  \n",
            "}\n" //
        );
        let buffer = cx.new_model(|cx| {
            Buffer::new(0, BufferId::new(1).unwrap(), text).with_language(Arc::new(rust_lang()), cx)
        });
        let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
        let position = buffer.read_with(cx, |buffer, cx| {
            let snapshot = buffer.snapshot(cx);
            snapshot.anchor_before(Point::new(1, 2))
        });
        let provider = Arc::new(FakeCompletionProvider::new());
        let codegen = cx.new_model(|cx| {
            Codegen::new(
                buffer.clone(),
                CodegenKind::Generate { position },
                provider.clone(),
                cx,
            )
        });

        let request = Box::new(DummyCompletionRequest {
            name: "test".to_string(),
        });
        codegen.update(cx, |codegen, cx| codegen.start(request, cx));

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
            println!("{:?}", &chunk);
            provider.send_completion(chunk);
            new_text = suffix;
            cx.background_executor.run_until_parked();
        }
        provider.finish_completion();
        cx.background_executor.run_until_parked();

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
    async fn test_strip_invalid_spans_from_codeblock() {
        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("Lorem ipsum dolor", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum dolor"
        );
        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("```\nLorem ipsum dolor", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum dolor"
        );
        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("```\nLorem ipsum dolor\n```", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum dolor"
        );
        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("```\nLorem ipsum dolor\n```\n", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum dolor"
        );
        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks(
                "```html\n```js\nLorem ipsum dolor\n```\n```",
                2
            ))
            .map(|chunk| chunk.unwrap())
            .collect::<String>()
            .await,
            "```js\nLorem ipsum dolor\n```"
        );
        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("``\nLorem ipsum dolor\n```", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "``\nLorem ipsum dolor\n```"
        );
        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("<|S|Lorem ipsum|E|>", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum"
        );

        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("<|S|>Lorem ipsum", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum"
        );

        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("```\n<|S|>Lorem ipsum\n```", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum"
        );
        assert_eq!(
            strip_invalid_spans_from_codeblock(chunks("```\n<|S|Lorem ipsum|E|>\n```", 2))
                .map(|chunk| chunk.unwrap())
                .collect::<String>()
                .await,
            "Lorem ipsum"
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

    fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
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
