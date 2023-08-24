use collections::HashMap;
use editor::{Editor, ToOffset, ToPoint};
use futures::{channel::mpsc, SinkExt, StreamExt};
use gpui::{AppContext, Task, ViewHandle};
use language::{Point, Rope};
use std::{cmp, env, fmt::Write};
use util::TryFutureExt;

use crate::{
    stream_completion,
    streaming_diff::{Hunk, StreamingDiff},
    OpenAIRequest, RequestMessage, Role,
};

pub struct RefactoringAssistant {
    pending_edits_by_editor: HashMap<usize, Task<Option<()>>>,
}

impl RefactoringAssistant {
    fn new() -> Self {
        Self {
            pending_edits_by_editor: Default::default(),
        }
    }

    pub fn update<F, T>(cx: &mut AppContext, f: F) -> T
    where
        F: FnOnce(&mut Self, &mut AppContext) -> T,
    {
        if !cx.has_global::<Self>() {
            cx.set_global(Self::new());
        }

        cx.update_global(f)
    }

    pub fn refactor(
        &mut self,
        editor: &ViewHandle<Editor>,
        user_prompt: &str,
        cx: &mut AppContext,
    ) {
        let api_key = if let Ok(api_key) = env::var("OPENAI_API_KEY") {
            api_key
        } else {
            // TODO: ensure the API key is present by going through the assistant panel's flow.
            return;
        };

        let snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);
        let selection = editor.read(cx).selections.newest_anchor().clone();
        let selected_text = snapshot
            .text_for_range(selection.start..selection.end)
            .collect::<Rope>();

        let mut normalized_selected_text = selected_text.clone();
        let mut base_indentation: Option<language::IndentSize> = None;
        let selection_start = selection.start.to_point(&snapshot);
        let selection_end = selection.end.to_point(&snapshot);
        if selection_start.row < selection_end.row {
            for row in selection_start.row..=selection_end.row {
                if snapshot.is_line_blank(row) {
                    continue;
                }

                let line_indentation = snapshot.indent_size_for_line(row);
                if let Some(base_indentation) = base_indentation.as_mut() {
                    if line_indentation.len < base_indentation.len {
                        *base_indentation = line_indentation;
                    }
                } else {
                    base_indentation = Some(line_indentation);
                }
            }
        }

        if let Some(base_indentation) = base_indentation {
            for row in selection_start.row..=selection_end.row {
                let selection_row = row - selection_start.row;
                let line_start =
                    normalized_selected_text.point_to_offset(Point::new(selection_row, 0));
                let indentation_len = if row == selection_start.row {
                    base_indentation.len.saturating_sub(selection_start.column)
                } else {
                    let line_len = normalized_selected_text.line_len(selection_row);
                    cmp::min(line_len, base_indentation.len)
                };
                let indentation_end = cmp::min(
                    line_start + indentation_len as usize,
                    normalized_selected_text.len(),
                );
                normalized_selected_text.replace(line_start..indentation_end, "");
            }
        }

        let language_name = snapshot
            .language_at(selection.start)
            .map(|language| language.name());
        let language_name = language_name.as_deref().unwrap_or("");

        let mut prompt = String::new();
        writeln!(prompt, "Given the following {language_name} snippet:").unwrap();
        writeln!(prompt, "{normalized_selected_text}").unwrap();
        writeln!(prompt, "{user_prompt}.").unwrap();
        writeln!(prompt, "Never make remarks, reply only with the new code.").unwrap();
        let request = OpenAIRequest {
            model: "gpt-4".into(),
            messages: vec![RequestMessage {
                role: Role::User,
                content: prompt,
            }],
            stream: true,
        };
        let response = stream_completion(api_key, cx.background().clone(), request);
        let editor = editor.downgrade();
        self.pending_edits_by_editor.insert(
            editor.id(),
            cx.spawn(|mut cx| {
                async move {
                    let _clear_highlights = util::defer({
                        let mut cx = cx.clone();
                        let editor = editor.clone();
                        move || {
                            let _ = editor.update(&mut cx, |editor, cx| {
                                editor.clear_text_highlights::<Self>(cx);
                            });
                        }
                    });

                    let mut edit_start = selection.start.to_offset(&snapshot);

                    let (mut hunks_tx, mut hunks_rx) = mpsc::channel(1);
                    let diff = cx.background().spawn(async move {
                        let mut messages = response.await?.ready_chunks(4);
                        let mut diff = StreamingDiff::new(selected_text.to_string());

                        let indentation_len;
                        let indentation_text;
                        if let Some(base_indentation) = base_indentation {
                            indentation_len = base_indentation.len;
                            indentation_text = match base_indentation.kind {
                                language::IndentKind::Space => " ",
                                language::IndentKind::Tab => "\t",
                            };
                        } else {
                            indentation_len = 0;
                            indentation_text = "";
                        };

                        let mut new_text =
                            indentation_text.repeat(
                                indentation_len.saturating_sub(selection_start.column) as usize,
                            );
                        while let Some(messages) = messages.next().await {
                            for message in messages {
                                let mut message = message?;
                                if let Some(choice) = message.choices.pop() {
                                    if let Some(text) = choice.delta.content {
                                        let mut lines = text.split('\n');
                                        if let Some(first_line) = lines.next() {
                                            new_text.push_str(&first_line);
                                        }

                                        for line in lines {
                                            new_text.push('\n');
                                            new_text.push_str(
                                                &indentation_text.repeat(indentation_len as usize),
                                            );
                                            new_text.push_str(line);
                                        }
                                    }
                                }
                            }

                            let hunks = diff.push_new(&new_text);
                            hunks_tx.send(hunks).await?;
                            new_text.clear();
                        }
                        hunks_tx.send(diff.finish()).await?;

                        anyhow::Ok(())
                    });

                    let mut first_transaction = None;
                    while let Some(hunks) = hunks_rx.next().await {
                        editor.update(&mut cx, |editor, cx| {
                            let mut highlights = Vec::new();

                            editor.buffer().update(cx, |buffer, cx| {
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
                                            highlights.push(edit_range);
                                            None
                                        }
                                    }),
                                    None,
                                    cx,
                                );
                                if let Some(transaction) = buffer.end_transaction(cx) {
                                    if let Some(first_transaction) = first_transaction {
                                        // Group all assistant edits into the first transaction.
                                        buffer.merge_transactions(
                                            transaction,
                                            first_transaction,
                                            cx,
                                        );
                                    } else {
                                        first_transaction = Some(transaction);
                                        buffer.finalize_last_transaction(cx);
                                    }
                                }
                            });

                            editor.highlight_text::<Self>(
                                highlights,
                                gpui::fonts::HighlightStyle {
                                    fade_out: Some(0.6),
                                    ..Default::default()
                                },
                                cx,
                            );
                        })?;
                    }
                    diff.await?;

                    anyhow::Ok(())
                }
                .log_err()
            }),
        );
    }
}
