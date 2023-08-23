use crate::{stream_completion, OpenAIRequest, RequestMessage, Role};
use collections::HashMap;
use editor::{Editor, ToOffset};
use futures::{channel::mpsc, SinkExt, StreamExt};
use gpui::{
    actions, elements::*, AnyViewHandle, AppContext, Entity, Task, View, ViewContext, ViewHandle,
    WeakViewHandle,
};
use menu::Confirm;
use std::{env, sync::Arc};
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
                    "Given the following {language_name} snippet:\n{selected_text}\n{prompt}. Never make remarks and reply only with the new code. Never change the leading whitespace on each line."
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
                    let mut edit_start = selection.start.to_offset(&snapshot);

                    let (mut hunks_tx, mut hunks_rx) = mpsc::channel(1);
                    let diff = cx.background().spawn(async move {
                        let mut messages = response.await?.ready_chunks(4);
                        let mut diff = crate::diff::Diff::new(selected_text);

                        while let Some(messages) = messages.next().await {
                            let mut new_text = String::new();
                            for message in messages {
                                let mut message = message?;
                                if let Some(choice) = message.choices.pop() {
                                    if let Some(text) = choice.delta.content {
                                        new_text.push_str(&text);
                                    }
                                }
                            }

                            let hunks = diff.push_new(&new_text);
                            hunks_tx.send(hunks).await?;
                        }
                        hunks_tx.send(diff.finish()).await?;

                        anyhow::Ok(())
                    });

                    while let Some(hunks) = hunks_rx.next().await {
                        editor.update(&mut cx, |editor, cx| {
                            let mut highlights = Vec::new();

                            editor.buffer().update(cx, |buffer, cx| {
                                buffer.start_transaction(cx);
                                for hunk in hunks {
                                    match hunk {
                                        crate::diff::Hunk::Insert { text } => {
                                            let edit_start = snapshot.anchor_after(edit_start);
                                            buffer.edit([(edit_start..edit_start, text)], None, cx);
                                        }
                                        crate::diff::Hunk::Remove { len } => {
                                            let edit_end = edit_start + len;
                                            let edit_range = snapshot.anchor_after(edit_start)
                                                ..snapshot.anchor_before(edit_end);
                                            buffer.edit([(edit_range, "")], None, cx);
                                            edit_start = edit_end;
                                        }
                                        crate::diff::Hunk::Keep { len } => {
                                            let edit_end = edit_start + len;
                                            let edit_range = snapshot.anchor_after(edit_start)
                                                ..snapshot.anchor_before(edit_end);
                                            highlights.push(edit_range);
                                            edit_start += len;
                                        }
                                    }
                                }
                                buffer.end_transaction(cx);
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
                    editor.update(&mut cx, |editor, cx| {
                        editor.clear_text_highlights::<Self>(cx);
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
                    editor.set_text("Replace with if statement.", cx);
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
