use crate::{stream_completion, OpenAIRequest, RequestMessage, Role};
use collections::HashMap;
use editor::{Editor, ToOffset};
use futures::StreamExt;
use gpui::{
    actions, elements::*, AnyViewHandle, AppContext, Entity, Task, View, ViewContext, ViewHandle,
    WeakViewHandle,
};
use menu::Confirm;
use similar::{Change, ChangeTag, TextDiff};
use std::{env, iter, ops::Range, sync::Arc};
use util::TryFutureExt;
use workspace::{Modal, Workspace};

actions!(assistant, [Refactor]);

pub fn init(cx: &mut AppContext) {
    cx.set_global(RefactoringAssistant::new());
    cx.add_action(RefactoringModal::deploy);
    cx.add_action(RefactoringModal::confirm);
}

pub struct RefactoringAssistant {
    pending_edits_by_editor: HashMap<usize, Task<Option<()>>>,
}

impl RefactoringAssistant {
    fn new() -> Self {
        Self {
            pending_edits_by_editor: Default::default(),
        }
    }

    fn refactor(&mut self, editor: &ViewHandle<Editor>, prompt: &str, cx: &mut AppContext) {
        let snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);
        let selection = editor.read(cx).selections.newest_anchor().clone();
        let selected_text = snapshot
            .text_for_range(selection.start..selection.end)
            .collect::<String>();
        let language_name = snapshot
            .language_at(selection.start)
            .map(|language| language.name());
        let language_name = language_name.as_deref().unwrap_or("");
        let request = OpenAIRequest {
            model: "gpt-4".into(),
            messages: vec![
                RequestMessage {
                role: Role::User,
                content: format!(
                    "Given the following {language_name} snippet:\n{selected_text}\n{prompt}. Avoid making remarks and reply only with the new code. Preserve indentation."
                ),
            }],
            stream: true,
        };
        let api_key = env::var("OPENAI_API_KEY").unwrap();
        let response = stream_completion(api_key, cx.background().clone(), request);
        let editor = editor.downgrade();
        self.pending_edits_by_editor.insert(
            editor.id(),
            cx.spawn(|mut cx| {
                async move {
                    let selection_start = selection.start.to_offset(&snapshot);

                    let mut new_text = String::new();
                    let mut messages = response.await?;

                    let mut transaction = None;

                    while let Some(message) = messages.next().await {
                        smol::future::yield_now().await;
                        let mut message = message?;
                        if let Some(choice) = message.choices.pop() {
                            if let Some(text) = choice.delta.content {
                                new_text.push_str(&text);

                                println!("-------------------------------------");

                                println!(
                                    "{}",
                                    similar::TextDiff::from_words(&selected_text, &new_text)
                                        .unified_diff()
                                );

                                let mut changes =
                                    similar::TextDiff::from_words(&selected_text, &new_text)
                                        .iter_all_changes()
                                        .collect::<Vec<_>>();

                                let mut ix = 0;
                                while ix < changes.len() {
                                    let deletion_start_ix = ix;
                                    let mut deletion_end_ix = ix;
                                    while changes
                                        .get(ix)
                                        .map_or(false, |change| change.tag() == ChangeTag::Delete)
                                    {
                                        ix += 1;
                                        deletion_end_ix += 1;
                                    }

                                    let insertion_start_ix = ix;
                                    let mut insertion_end_ix = ix;
                                    while changes
                                        .get(ix)
                                        .map_or(false, |change| change.tag() == ChangeTag::Insert)
                                    {
                                        ix += 1;
                                        insertion_end_ix += 1;
                                    }

                                    if deletion_end_ix > deletion_start_ix
                                        && insertion_end_ix > insertion_start_ix
                                    {
                                        for _ in deletion_start_ix..deletion_end_ix {
                                            let deletion = changes.remove(deletion_end_ix);
                                            changes.insert(insertion_end_ix - 1, deletion);
                                        }
                                    }

                                    ix += 1;
                                }

                                while changes
                                    .last()
                                    .map_or(false, |change| change.tag() != ChangeTag::Insert)
                                {
                                    changes.pop();
                                }

                                editor.update(&mut cx, |editor, cx| {
                                    editor.buffer().update(cx, |buffer, cx| {
                                        if let Some(transaction) = transaction.take() {
                                            buffer.undo(cx); // TODO: Undo the transaction instead
                                        }

                                        buffer.start_transaction(cx);
                                        let mut edit_start = selection_start;
                                        dbg!(&changes);
                                        for change in changes {
                                            let value = change.value();
                                            let edit_end = edit_start + value.len();
                                            match change.tag() {
                                                ChangeTag::Equal => {
                                                    edit_start = edit_end;
                                                }
                                                ChangeTag::Delete => {
                                                    let range = snapshot.anchor_after(edit_start)
                                                        ..snapshot.anchor_before(edit_end);
                                                    buffer.edit([(range, "")], None, cx);
                                                    edit_start = edit_end;
                                                }
                                                ChangeTag::Insert => {
                                                    let insertion_start =
                                                        snapshot.anchor_after(edit_start);
                                                    buffer.edit(
                                                        [(insertion_start..insertion_start, value)],
                                                        None,
                                                        cx,
                                                    );
                                                }
                                            }
                                        }
                                        transaction = buffer.end_transaction(cx);
                                    })
                                })?;
                            }
                        }
                    }

                    editor.update(&mut cx, |editor, cx| {
                        editor.buffer().update(cx, |buffer, cx| {
                            if let Some(transaction) = transaction.take() {
                                buffer.undo(cx); // TODO: Undo the transaction instead
                            }

                            buffer.start_transaction(cx);
                            let mut edit_start = selection_start;
                            for change in similar::TextDiff::from_words(&selected_text, &new_text)
                                .iter_all_changes()
                            {
                                let value = change.value();
                                let edit_end = edit_start + value.len();
                                match change.tag() {
                                    ChangeTag::Equal => {
                                        edit_start = edit_end;
                                    }
                                    ChangeTag::Delete => {
                                        let range = snapshot.anchor_after(edit_start)
                                            ..snapshot.anchor_before(edit_end);
                                        buffer.edit([(range, "")], None, cx);
                                        edit_start = edit_end;
                                    }
                                    ChangeTag::Insert => {
                                        let insertion_start = snapshot.anchor_after(edit_start);
                                        buffer.edit(
                                            [(insertion_start..insertion_start, value)],
                                            None,
                                            cx,
                                        );
                                    }
                                }
                            }
                            buffer.end_transaction(cx);
                        })
                    })?;

                    anyhow::Ok(())
                }
                .log_err()
            }),
        );
    }
}

struct RefactoringModal {
    editor: WeakViewHandle<Editor>,
    prompt_editor: ViewHandle<Editor>,
    has_focus: bool,
}

impl Entity for RefactoringModal {
    type Event = ();
}

impl View for RefactoringModal {
    fn ui_name() -> &'static str {
        "RefactoringModal"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        ChildView::new(&self.prompt_editor, cx).into_any()
    }

    fn focus_in(&mut self, _: AnyViewHandle, _: &mut ViewContext<Self>) {
        self.has_focus = true;
    }

    fn focus_out(&mut self, _: AnyViewHandle, _: &mut ViewContext<Self>) {
        self.has_focus = false;
    }
}

impl Modal for RefactoringModal {
    fn has_focus(&self) -> bool {
        self.has_focus
    }

    fn dismiss_on_event(event: &Self::Event) -> bool {
        // TODO
        false
    }
}

impl RefactoringModal {
    fn deploy(workspace: &mut Workspace, _: &Refactor, cx: &mut ViewContext<Workspace>) {
        if let Some(editor) = workspace
            .active_item(cx)
            .and_then(|item| Some(item.downcast::<Editor>()?.downgrade()))
        {
            workspace.toggle_modal(cx, |_, cx| {
                let prompt_editor = cx.add_view(|cx| {
                    let mut editor = Editor::auto_height(
                        4,
                        Some(Arc::new(|theme| theme.search.editor.input.clone())),
                        cx,
                    );
                    editor.set_text("Replace with match statement.", cx);
                    editor
                });
                cx.add_view(|_| RefactoringModal {
                    editor,
                    prompt_editor,
                    has_focus: false,
                })
            });
        }
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        if let Some(editor) = self.editor.upgrade(cx) {
            let prompt = self.prompt_editor.read(cx).text(cx);
            cx.update_global(|assistant: &mut RefactoringAssistant, cx| {
                assistant.refactor(&editor, &prompt, cx);
            });
        }
    }
}
fn words(text: &str) -> impl Iterator<Item = (Range<usize>, &str)> {
    let mut word_start_ix = None;
    let mut chars = text.char_indices();
    iter::from_fn(move || {
        while let Some((ix, ch)) = chars.next() {
            if let Some(start_ix) = word_start_ix {
                if !ch.is_alphanumeric() {
                    let word = &text[start_ix..ix];
                    word_start_ix.take();
                    return Some((start_ix..ix, word));
                }
            } else {
                if ch.is_alphanumeric() {
                    word_start_ix = Some(ix);
                }
            }
        }
        None
    })
}

fn streaming_diff<'a>(old_text: &'a str, new_text: &'a str) -> Vec<Change<'a, str>> {
    let changes = TextDiff::configure()
        .algorithm(similar::Algorithm::Patience)
        .diff_words(old_text, new_text);
    let mut changes = changes.iter_all_changes().peekable();

    let mut result = vec![];

    loop {
        let mut deletions = vec![];
        let mut insertions = vec![];

        while changes
            .peek()
            .map_or(false, |change| change.tag() == ChangeTag::Delete)
        {
            deletions.push(changes.next().unwrap());
        }

        while changes
            .peek()
            .map_or(false, |change| change.tag() == ChangeTag::Insert)
        {
            insertions.push(changes.next().unwrap());
        }

        if !deletions.is_empty() && !insertions.is_empty() {
            result.append(&mut insertions);
            result.append(&mut deletions);
        } else {
            result.append(&mut deletions);
            result.append(&mut insertions);
        }

        if let Some(change) = changes.next() {
            result.push(change);
        } else {
            break;
        }
    }

    // Remove all non-inserts at the end.
    while result
        .last()
        .map_or(false, |change| change.tag() != ChangeTag::Insert)
    {
        result.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_streaming_diff() {
        let old_text = indoc! {"
            match (self.format, src_format) {
                (Format::A8, Format::A8)
                | (Format::Rgb24, Format::Rgb24)
                | (Format::Rgba32, Format::Rgba32) => {
                    return self
                        .blit_from_with::<BlitMemcpy>(dst_rect, src_bytes, src_stride, src_format);
                }
                (Format::A8, Format::Rgb24) => {
                    return self
                        .blit_from_with::<BlitRgb24ToA8>(dst_rect, src_bytes, src_stride, src_format);
                }
                (Format::Rgb24, Format::A8) => {
                    return self
                        .blit_from_with::<BlitA8ToRgb24>(dst_rect, src_bytes, src_stride, src_format);
                }
                (Format::Rgb24, Format::Rgba32) => {
                    return self.blit_from_with::<BlitRgba32ToRgb24>(
                        dst_rect, src_bytes, src_stride, src_format,
                    );
                }
                (Format::Rgba32, Format::Rgb24)
                | (Format::Rgba32, Format::A8)
                | (Format::A8, Format::Rgba32) => {
                    unimplemented!()
                }
                _ => {}
            }
        "};
        let new_text = indoc! {"
            if self.format == src_format
        "};
        dbg!(streaming_diff(old_text, new_text));
    }
}
