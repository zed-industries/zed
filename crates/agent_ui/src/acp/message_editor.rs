use crate::acp::completion_provider::ContextPickerCompletionProvider;
use crate::acp::{MessageHistory, completion_provider::MentionSet};
use acp_thread::MentionUri;
use agent_client_protocol as acp;
use anyhow::Result;
use collections::HashSet;
use editor::{
    AnchorRangeExt, ContextMenuOptions, ContextMenuPlacement, Editor, EditorElement, EditorMode,
    EditorStyle, MultiBuffer,
};
use file_icons::FileIcons;
use gpui::{
    AppContext, Context, Entity, EventEmitter, FocusHandle, Focusable, Subscription, Task,
    TextStyle, WeakEntity,
};
use language::Language;
use language::{Buffer, BufferSnapshot};
use parking_lot::Mutex;
use project::{CompletionIntent, Project};
use settings::Settings;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::{cell::RefCell, sync::Arc};
use theme::ThemeSettings;
use ui::{
    ActiveTheme, App, IconName, InteractiveElement, IntoElement, ParentElement, Render,
    SharedString, Styled, TextSize, Window, div,
};
use util::ResultExt;
use workspace::Workspace;
use zed_actions::agent::{Chat, NextHistoryMessage, PreviousHistoryMessage};

pub const MIN_EDITOR_LINES: usize = 4;
pub const MAX_EDITOR_LINES: usize = 8;

pub struct MessageEditor {
    editor: Entity<Editor>,
    project: Entity<Project>,
    mention_set: Arc<Mutex<MentionSet>>,
    history: Rc<RefCell<MessageHistory<Vec<acp::ContentBlock>>>>,
    message_set_from_history: Option<BufferSnapshot>,
    _subscription: Subscription,
}

pub enum MessageEditorEvent {
    Chat,
}

impl EventEmitter<MessageEditorEvent> for MessageEditor {}

impl MessageEditor {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        history: Rc<RefCell<MessageHistory<Vec<acp::ContentBlock>>>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let language = Language::new(
            language::LanguageConfig {
                completion_query_characters: HashSet::from_iter(['.', '-', '_', '@']),
                ..Default::default()
            },
            None,
        );

        let mention_set = Arc::new(Mutex::new(MentionSet::default()));
        let editor = cx.new(|cx| {
            let buffer = cx.new(|cx| Buffer::local("", cx).with_language(Arc::new(language), cx));
            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

            let mut editor = Editor::new(
                editor::EditorMode::AutoHeight {
                    min_lines: MIN_EDITOR_LINES,
                    max_lines: Some(MAX_EDITOR_LINES),
                },
                buffer,
                None,
                window,
                cx,
            );
            editor.set_placeholder_text("Message the agent － @ to include files", cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_soft_wrap();
            editor.set_use_modal_editing(true);
            editor.set_completion_provider(Some(Rc::new(ContextPickerCompletionProvider::new(
                mention_set.clone(),
                workspace,
                cx.weak_entity(),
            ))));
            editor.set_context_menu_options(ContextMenuOptions {
                min_entries_visible: 12,
                max_entries_visible: 12,
                placement: Some(ContextMenuPlacement::Above),
            });
            editor
        });
        let message_editor_subscription = cx.subscribe(&editor, |this, editor, event, cx| {
            if let editor::EditorEvent::BufferEdited = &event {
                let buffer = editor
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .as_singleton()
                    .unwrap()
                    .read(cx)
                    .snapshot();
                if let Some(message) = this.message_set_from_history.clone()
                    && message.version() != buffer.version()
                {
                    this.message_set_from_history = None;
                }

                if this.message_set_from_history.is_none() {
                    this.history.borrow_mut().reset_position();
                }
            }
        });

        Self {
            editor,
            project,
            mention_set,
            history,
            message_set_from_history: None,
            _subscription: message_editor_subscription,
        }
    }

    pub fn is_empty(&self, cx: &App) -> bool {
        self.editor.read(cx).is_empty(cx)
    }

    pub fn contents(&self, cx: &mut Context<Self>) -> Task<Result<Vec<acp::ContentBlock>>> {
        let contents = self.mention_set.lock().contents(self.project.clone(), cx);
        let editor = self.editor.clone();

        cx.spawn(async move |_, cx| {
            let contents = contents.await?;

            editor.update(cx, |editor, cx| {
                let mut ix = 0;
                let mut chunks: Vec<acp::ContentBlock> = Vec::new();
                let text = editor.text(cx);
                editor.display_map.update(cx, |map, cx| {
                    let snapshot = map.snapshot(cx);
                    for (crease_id, crease) in snapshot.crease_snapshot.creases() {
                        // Skip creases that have been edited out of the message buffer.
                        if !crease.range().start.is_valid(&snapshot.buffer_snapshot) {
                            continue;
                        }

                        if let Some(mention) = contents.get(&crease_id) {
                            let crease_range = crease.range().to_offset(&snapshot.buffer_snapshot);
                            if crease_range.start > ix {
                                chunks.push(text[ix..crease_range.start].into());
                            }
                            chunks.push(acp::ContentBlock::Resource(acp::EmbeddedResource {
                                annotations: None,
                                resource: acp::EmbeddedResourceResource::TextResourceContents(
                                    acp::TextResourceContents {
                                        mime_type: None,
                                        text: mention.content.clone(),
                                        uri: mention.uri.to_uri(),
                                    },
                                ),
                            }));
                            ix = crease_range.end;
                        }
                    }

                    if ix < text.len() {
                        let last_chunk = text[ix..].trim_end();
                        if !last_chunk.is_empty() {
                            chunks.push(last_chunk.into());
                        }
                    }
                });

                chunks
            })
        })
    }

    pub fn clear(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
            editor.remove_creases(self.mention_set.lock().drain(), cx)
        });
    }

    fn chat(&mut self, _: &Chat, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(MessageEditorEvent::Chat)
    }

    pub fn insert_dragged_files(
        &self,
        paths: Vec<project::ProjectPath>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let buffer = self.editor.read(cx).buffer().clone();
        let Some((&excerpt_id, _, _)) = buffer.read(cx).snapshot(cx).as_singleton() else {
            return;
        };
        let Some(buffer) = buffer.read(cx).as_singleton() else {
            return;
        };
        for path in paths {
            let Some(entry) = self.project.read(cx).entry_for_path(&path, cx) else {
                continue;
            };
            let Some(abs_path) = self.project.read(cx).absolute_path(&path, cx) else {
                continue;
            };

            let anchor = buffer.update(cx, |buffer, _cx| buffer.anchor_before(buffer.len()));
            let path_prefix = abs_path
                .file_name()
                .unwrap_or(path.path.as_os_str())
                .display()
                .to_string();
            let completion = ContextPickerCompletionProvider::completion_for_path(
                path,
                &path_prefix,
                false,
                entry.is_dir(),
                excerpt_id,
                anchor..anchor,
                self.editor.clone(),
                self.mention_set.clone(),
                self.project.clone(),
                cx,
            );

            self.editor.update(cx, |message_editor, cx| {
                message_editor.edit(
                    [(
                        multi_buffer::Anchor::max()..multi_buffer::Anchor::max(),
                        completion.new_text,
                    )],
                    cx,
                );
            });
            if let Some(confirm) = completion.confirm.clone() {
                confirm(CompletionIntent::Complete, window, cx);
            }
        }
    }

    pub fn set_expanded(&mut self, expanded: bool, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            if expanded {
                editor.set_mode(EditorMode::Full {
                    scale_ui_elements_with_buffer_font_size: false,
                    show_active_line_background: false,
                    sized_by_content: false,
                })
            } else {
                editor.set_mode(EditorMode::AutoHeight {
                    min_lines: MIN_EDITOR_LINES,
                    max_lines: Some(MAX_EDITOR_LINES),
                })
            }
            cx.notify()
        });
    }

    fn previous_history_message(
        &mut self,
        _: &PreviousHistoryMessage,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.message_set_from_history.is_none() && !self.editor.read(cx).is_empty(cx) {
            self.editor.update(cx, |editor, cx| {
                editor.move_up(&Default::default(), window, cx);
            });
            return;
        }

        self.message_set_from_history = Self::set_draft_message(
            self.editor.clone(),
            self.mention_set.clone(),
            self.project.clone(),
            self.history
                .borrow_mut()
                .prev()
                .map(|blocks| blocks.as_slice()),
            window,
            cx,
        );
    }

    fn next_history_message(
        &mut self,
        _: &NextHistoryMessage,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.message_set_from_history.is_none() {
            self.editor.update(cx, |editor, cx| {
                editor.move_down(&Default::default(), window, cx);
            });
            return;
        }

        let mut history = self.history.borrow_mut();
        let next_history = history.next();

        let set_draft_message = Self::set_draft_message(
            self.editor.clone(),
            self.mention_set.clone(),
            self.project.clone(),
            Some(
                next_history
                    .map(|blocks| blocks.as_slice())
                    .unwrap_or_else(|| &[]),
            ),
            window,
            cx,
        );
        // If we reset the text to an empty string because we ran out of history,
        // we don't want to mark it as coming from the history
        self.message_set_from_history = if next_history.is_some() {
            set_draft_message
        } else {
            None
        };
    }

    fn set_draft_message(
        message_editor: Entity<Editor>,
        mention_set: Arc<Mutex<MentionSet>>,
        project: Entity<Project>,
        message: Option<&[acp::ContentBlock]>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<BufferSnapshot> {
        cx.notify();

        let message = message?;

        let mut text = String::new();
        let mut mentions = Vec::new();

        for chunk in message {
            dbg!(&chunk);
            match chunk {
                acp::ContentBlock::Text(text_content) => {
                    text.push_str(&text_content.text);
                }
                acp::ContentBlock::Resource(acp::EmbeddedResource {
                    resource: acp::EmbeddedResourceResource::TextResourceContents(resource),
                    ..
                }) => {
                    if let Some(ref mention @ MentionUri::File(ref abs_path)) =
                        MentionUri::parse(&resource.uri).log_err()
                    {
                        let project_path = project
                            .read(cx)
                            .project_path_for_absolute_path(&abs_path, cx);
                        let start = text.len();
                        let content = mention.to_uri();
                        text.push_str(&content);
                        let end = text.len();
                        if let Some(project_path) = project_path {
                            let filename: SharedString = project_path
                                .path
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string()
                                .into();
                            mentions.push((start..end, project_path, filename));
                        }
                    }
                }
                acp::ContentBlock::Image(_)
                | acp::ContentBlock::Audio(_)
                | acp::ContentBlock::Resource(_)
                | acp::ContentBlock::ResourceLink(_) => {}
            }
        }
        dbg!(&mentions);

        let snapshot = message_editor.update(cx, |editor, cx| {
            editor.set_text(text, window, cx);
            editor.buffer().read(cx).snapshot(cx)
        });

        for (range, project_path, filename) in mentions {
            let crease_icon_path = if project_path.path.is_dir() {
                FileIcons::get_folder_icon(false, cx)
                    .unwrap_or_else(|| IconName::Folder.path().into())
            } else {
                FileIcons::get_icon(Path::new(project_path.path.as_ref()), cx)
                    .unwrap_or_else(|| IconName::File.path().into())
            };

            let anchor = snapshot.anchor_before(range.start);
            if let Some(project_path) = project.read(cx).absolute_path(&project_path, cx) {
                let crease_id = crate::context_picker::insert_crease_for_mention(
                    anchor.excerpt_id,
                    anchor.text_anchor,
                    range.end - range.start,
                    filename,
                    crease_icon_path,
                    message_editor.clone(),
                    window,
                    cx,
                );

                if let Some(crease_id) = crease_id {
                    mention_set.lock().insert(crease_id, project_path);
                }
            }
        }

        let snapshot = snapshot.as_singleton().unwrap().2.clone();
        Some(snapshot)
    }

    #[cfg(test)]
    pub fn set_text(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.set_text(text, window, cx);
        });
    }
}

impl Focusable for MessageEditor {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for MessageEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("MessageEditor")
            .on_action(cx.listener(Self::previous_history_message))
            .on_action(cx.listener(Self::next_history_message))
            .on_action(cx.listener(Self::chat))
            .flex_1()
            .child({
                let settings = ThemeSettings::get_global(cx);
                let font_size = TextSize::Small
                    .rems(cx)
                    .to_pixels(settings.agent_font_size(cx));
                let line_height = settings.buffer_line_height.value() * font_size;

                let text_style = TextStyle {
                    color: cx.theme().colors().text,
                    font_family: settings.buffer_font.family.clone(),
                    font_fallbacks: settings.buffer_font.fallbacks.clone(),
                    font_features: settings.buffer_font.features.clone(),
                    font_size: font_size.into(),
                    line_height: line_height.into(),
                    ..Default::default()
                };

                EditorElement::new(
                    &self.editor,
                    EditorStyle {
                        background: cx.theme().colors().editor_background,
                        local_player: cx.theme().players().local(),
                        text: text_style,
                        syntax: cx.theme().syntax().clone(),
                        ..Default::default()
                    },
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, path::Path, rc::Rc};

    use agent_client_protocol as acp;
    use fs::FakeFs;
    use gpui::{AppContext, TestAppContext};
    use lsp::{CompletionContext, CompletionTriggerKind};
    use pretty_assertions::assert_matches;
    use project::{CompletionIntent, Project};
    use serde_json::json;
    use util::path;
    use workspace::Workspace;

    use crate::acp::{
        MessageHistory, message_editor::MessageEditor, thread_view::tests::init_test,
    };

    #[gpui::test]
    async fn test_at_mention_history(cx: &mut TestAppContext) {
        init_test(cx);

        let history = Rc::new(RefCell::new(MessageHistory::default()));
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({"file": ""})).await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                MessageEditor::new(
                    workspace.downgrade(),
                    project.clone(),
                    history.clone(),
                    window,
                    cx,
                )
            })
        });
        let editor = message_editor.update(cx, |message_editor, _| message_editor.editor.clone());

        cx.run_until_parked();

        let excerpt_id = editor.update(cx, |editor, cx| {
            editor
                .buffer()
                .read(cx)
                .excerpt_ids()
                .into_iter()
                .next()
                .unwrap()
        });
        let completions = editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello @", window, cx);
            let buffer = editor.buffer().read(cx).as_singleton().unwrap();
            let completion_provider = editor.completion_provider().unwrap();
            completion_provider.completions(
                excerpt_id,
                &buffer,
                text::Anchor::MAX,
                CompletionContext {
                    trigger_kind: CompletionTriggerKind::TRIGGER_CHARACTER,
                    trigger_character: Some("@".into()),
                },
                window,
                cx,
            )
        });
        let [_, completion]: [_; 2] = completions
            .await
            .unwrap()
            .into_iter()
            .flat_map(|response| response.completions)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        editor.update_in(cx, |editor, window, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let start = snapshot
                .anchor_in_excerpt(excerpt_id, completion.replace_range.start)
                .unwrap();
            let end = snapshot
                .anchor_in_excerpt(excerpt_id, completion.replace_range.end)
                .unwrap();
            editor.edit([(start..end, completion.new_text)], cx);
            (completion.confirm.unwrap())(CompletionIntent::Complete, window, cx);
        });

        cx.run_until_parked();

        let content = message_editor
            .update(cx, |message_editor, cx| message_editor.contents(cx))
            .await
            .unwrap();
        assert_eq!(content.len(), 2);
        assert_matches!(&content[0], &acp::ContentBlock::Text(_));
        assert_matches!(&content[1], &acp::ContentBlock::Resource(_));

        history.borrow_mut().push(content);
        message_editor.update_in(cx, |message_editor, window, cx| {
            message_editor.clear(window, cx);
            message_editor.previous_history_message(&Default::default(), window, cx);
        });

        let content = message_editor
            .update(cx, |message_editor, cx| message_editor.contents(cx))
            .await
            .unwrap();
        assert_eq!(content.len(), 2);
        assert_matches!(&content[0], &acp::ContentBlock::Text(_));
        assert_matches!(&content[1], &acp::ContentBlock::Resource(_));
    }

    #[gpui::test]
    async fn test_at_mention_removal(cx: &mut TestAppContext) {
        init_test(cx);

        let history = Rc::new(RefCell::new(MessageHistory::default()));
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({"file": ""})).await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                MessageEditor::new(
                    workspace.downgrade(),
                    project.clone(),
                    history.clone(),
                    window,
                    cx,
                )
            })
        });
        let editor = message_editor.update(cx, |message_editor, _| message_editor.editor.clone());

        cx.run_until_parked();

        let excerpt_id = editor.update(cx, |editor, cx| {
            editor
                .buffer()
                .read(cx)
                .excerpt_ids()
                .into_iter()
                .next()
                .unwrap()
        });
        let completions = editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello @", window, cx);
            let buffer = editor.buffer().read(cx).as_singleton().unwrap();
            let completion_provider = editor.completion_provider().unwrap();
            completion_provider.completions(
                excerpt_id,
                &buffer,
                text::Anchor::MAX,
                CompletionContext {
                    trigger_kind: CompletionTriggerKind::TRIGGER_CHARACTER,
                    trigger_character: Some("@".into()),
                },
                window,
                cx,
            )
        });
        let [_, completion]: [_; 2] = completions
            .await
            .unwrap()
            .into_iter()
            .flat_map(|response| response.completions)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        editor.update_in(cx, |editor, window, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let start = snapshot
                .anchor_in_excerpt(excerpt_id, completion.replace_range.start)
                .unwrap();
            let end = snapshot
                .anchor_in_excerpt(excerpt_id, completion.replace_range.end)
                .unwrap();
            editor.edit([(start..end, completion.new_text)], cx);
            (completion.confirm.unwrap())(CompletionIntent::Complete, window, cx);
        });

        cx.run_until_parked();

        // Backspace over the inserted crease (and the following space).
        editor.update_in(cx, |editor, window, cx| {
            editor.backspace(&Default::default(), window, cx);
            editor.backspace(&Default::default(), window, cx);
        });

        let content = message_editor
            .update_in(cx, |message_editor, window, cx| message_editor.contents(cx))
            .await
            .unwrap();

        // We don't send a resource link for the deleted crease.
        pretty_assertions::assert_matches!(content.as_slice(), [acp::ContentBlock::Text { .. }]);
    }
}
