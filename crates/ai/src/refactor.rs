use crate::{stream_completion, OpenAIRequest, RequestMessage, Role};
use collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use editor::{Anchor, Editor, MultiBuffer, MultiBufferSnapshot, ToOffset};
use futures::{io::BufWriter, AsyncReadExt, AsyncWriteExt, StreamExt};
use gpui::{
    actions, elements::*, AnyViewHandle, AppContext, Entity, Task, View, ViewContext, ViewHandle,
    WeakViewHandle,
};
use menu::Confirm;
use serde::Deserialize;
use similar::ChangeTag;
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
        let buffer = editor.read(cx).buffer().read(cx).snapshot(cx);
        let selection = editor.read(cx).selections.newest_anchor().clone();
        let selected_text = buffer
            .text_for_range(selection.start..selection.end)
            .collect::<String>();
        let language_name = buffer
            .language_at(selection.start)
            .map(|language| language.name());
        let language_name = language_name.as_deref().unwrap_or("");
        let request = OpenAIRequest {
            model: "gpt-4".into(),
            messages: vec![
                RequestMessage {
                role: Role::User,
                content: format!(
                    "Given the following {language_name} snippet:\n{selected_text}\n{prompt}. Avoid making remarks and reply only with the new code."
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
                    let selection_start = selection.start.to_offset(&buffer);

                    // Find unique words in the selected text to use as diff boundaries.
                    let mut duplicate_words = HashSet::default();
                    let mut unique_old_words = HashMap::default();
                    for (range, word) in words(&selected_text) {
                        if !duplicate_words.contains(word) {
                            if unique_old_words.insert(word, range.end).is_some() {
                                unique_old_words.remove(word);
                                duplicate_words.insert(word);
                            }
                        }
                    }

                    let mut new_text = String::new();
                    let mut messages = response.await?;
                    let mut new_word_search_start_ix = 0;
                    let mut last_old_word_end_ix = 0;

                    'outer: loop {
                        const MIN_DIFF_LEN: usize = 50;

                        let start = new_word_search_start_ix;
                        let mut words = words(&new_text[start..]);
                        while let Some((range, new_word)) = words.next() {
                            // We found a word in the new text that was unique in the old text. We can use
                            // it as a diff boundary, and start applying edits.
                            if let Some(old_word_end_ix) = unique_old_words.get(new_word).copied() {
                                if old_word_end_ix.saturating_sub(last_old_word_end_ix)
                                    > MIN_DIFF_LEN
                                {
                                    drop(words);

                                    let remainder = new_text.split_off(start + range.end);
                                    let edits = diff(
                                        selection_start + last_old_word_end_ix,
                                        &selected_text[last_old_word_end_ix..old_word_end_ix],
                                        &new_text,
                                        &buffer,
                                    );
                                    editor.update(&mut cx, |editor, cx| {
                                        editor
                                            .buffer()
                                            .update(cx, |buffer, cx| buffer.edit(edits, None, cx))
                                    })?;

                                    new_text = remainder;
                                    new_word_search_start_ix = 0;
                                    last_old_word_end_ix = old_word_end_ix;
                                    continue 'outer;
                                }
                            }

                            new_word_search_start_ix = start + range.end;
                        }
                        drop(words);

                        // Buffer incoming text, stopping if the stream was exhausted.
                        if let Some(message) = messages.next().await {
                            let mut message = message?;
                            if let Some(choice) = message.choices.pop() {
                                if let Some(text) = choice.delta.content {
                                    new_text.push_str(&text);
                                }
                            }
                        } else {
                            break;
                        }
                    }

                    let edits = diff(
                        selection_start + last_old_word_end_ix,
                        &selected_text[last_old_word_end_ix..],
                        &new_text,
                        &buffer,
                    );
                    editor.update(&mut cx, |editor, cx| {
                        editor
                            .buffer()
                            .update(cx, |buffer, cx| buffer.edit(edits, None, cx))
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
                    Editor::auto_height(
                        4,
                        Some(Arc::new(|theme| theme.search.editor.input.clone())),
                        cx,
                    )
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

fn diff<'a>(
    start_ix: usize,
    old_text: &'a str,
    new_text: &'a str,
    old_buffer_snapshot: &MultiBufferSnapshot,
) -> Vec<(Range<Anchor>, &'a str)> {
    let mut edit_start = start_ix;
    let mut edits = Vec::new();
    let diff = similar::TextDiff::from_words(old_text, &new_text);
    for change in diff.iter_all_changes() {
        let value = change.value();
        let edit_end = edit_start + value.len();
        match change.tag() {
            ChangeTag::Equal => {
                edit_start = edit_end;
            }
            ChangeTag::Delete => {
                edits.push((
                    old_buffer_snapshot.anchor_after(edit_start)
                        ..old_buffer_snapshot.anchor_before(edit_end),
                    "",
                ));
                edit_start = edit_end;
            }
            ChangeTag::Insert => {
                edits.push((
                    old_buffer_snapshot.anchor_after(edit_start)
                        ..old_buffer_snapshot.anchor_after(edit_start),
                    value,
                ));
            }
        }
    }
    edits
}
