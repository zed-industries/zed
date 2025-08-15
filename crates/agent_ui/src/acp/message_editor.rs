use crate::acp::completion_provider::ContextPickerCompletionProvider;
use crate::acp::completion_provider::MentionImage;
use crate::acp::completion_provider::MentionSet;
use acp_thread::MentionUri;
use agent::TextThreadStore;
use agent::ThreadStore;
use agent_client_protocol as acp;
use anyhow::Result;
use collections::HashSet;
use editor::ExcerptId;
use editor::actions::Paste;
use editor::display_map::CreaseId;
use editor::{
    AnchorRangeExt, ContextMenuOptions, ContextMenuPlacement, Editor, EditorElement, EditorMode,
    EditorStyle, MultiBuffer,
};
use futures::FutureExt as _;
use gpui::ClipboardEntry;
use gpui::Image;
use gpui::ImageFormat;
use gpui::{
    AppContext, Context, Entity, EventEmitter, FocusHandle, Focusable, Task, TextStyle, WeakEntity,
};
use language::Buffer;
use language::Language;
use language_model::LanguageModelImage;
use parking_lot::Mutex;
use project::{CompletionIntent, Project};
use settings::Settings;
use std::fmt::Write;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use theme::ThemeSettings;
use ui::IconName;
use ui::SharedString;
use ui::{
    ActiveTheme, App, InteractiveElement, IntoElement, ParentElement, Render, Styled, TextSize,
    Window, div,
};
use util::ResultExt;
use workspace::Workspace;
use workspace::notifications::NotifyResultExt as _;
use zed_actions::agent::Chat;

use super::completion_provider::Mention;

pub struct MessageEditor {
    editor: Entity<Editor>,
    project: Entity<Project>,
    thread_store: Entity<ThreadStore>,
    text_thread_store: Entity<TextThreadStore>,
    mention_set: Arc<Mutex<MentionSet>>,
}

pub enum MessageEditorEvent {
    Send,
    Cancel,
}

impl EventEmitter<MessageEditorEvent> for MessageEditor {}

impl MessageEditor {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        thread_store: Entity<ThreadStore>,
        text_thread_store: Entity<TextThreadStore>,
        mode: EditorMode,
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

            let mut editor = Editor::new(mode, buffer, None, window, cx);
            editor.set_placeholder_text("Message the agent － @ to include files", cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_soft_wrap();
            editor.set_use_modal_editing(true);
            editor.set_completion_provider(Some(Rc::new(ContextPickerCompletionProvider::new(
                mention_set.clone(),
                workspace,
                thread_store.downgrade(),
                text_thread_store.downgrade(),
                cx.weak_entity(),
            ))));
            editor.set_context_menu_options(ContextMenuOptions {
                min_entries_visible: 12,
                max_entries_visible: 12,
                placement: Some(ContextMenuPlacement::Above),
            });
            editor
        });

        Self {
            editor,
            project,
            mention_set,
            thread_store,
            text_thread_store,
        }
    }

    pub fn is_empty(&self, cx: &App) -> bool {
        self.editor.read(cx).is_empty(cx)
    }

    pub fn contents(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<acp::ContentBlock>>> {
        let contents = self.mention_set.lock().contents(
            self.project.clone(),
            self.thread_store.clone(),
            self.text_thread_store.clone(),
            window,
            cx,
        );
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

                        let Some(mention) = contents.get(&crease_id) else {
                            continue;
                        };

                        let crease_range = crease.range().to_offset(&snapshot.buffer_snapshot);
                        if crease_range.start > ix {
                            chunks.push(text[ix..crease_range.start].into());
                        }
                        let chunk = match mention {
                            Mention::Text { uri, content } => {
                                acp::ContentBlock::Resource(acp::EmbeddedResource {
                                    annotations: None,
                                    resource: acp::EmbeddedResourceResource::TextResourceContents(
                                        acp::TextResourceContents {
                                            mime_type: None,
                                            text: content.clone(),
                                            uri: uri.to_uri().to_string(),
                                        },
                                    ),
                                })
                            }
                            Mention::Image(mention_image) => {
                                acp::ContentBlock::Image(acp::ImageContent {
                                    annotations: None,
                                    data: mention_image.data.to_string(),
                                    mime_type: mention_image.format.mime_type().into(),
                                    uri: mention_image
                                        .abs_path
                                        .as_ref()
                                        .map(|path| format!("file://{}", path.display())),
                                })
                            }
                        };
                        chunks.push(chunk);
                        ix = crease_range.end;
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
        cx.emit(MessageEditorEvent::Send)
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(MessageEditorEvent::Cancel)
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        let images = cx
            .read_from_clipboard()
            .map(|item| {
                item.into_entries()
                    .filter_map(|entry| {
                        if let ClipboardEntry::Image(image) = entry {
                            Some(image)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        if images.is_empty() {
            return;
        }
        cx.stop_propagation();

        let replacement_text = "image";
        for image in images {
            let (excerpt_id, anchor) = self.editor.update(cx, |message_editor, cx| {
                let snapshot = message_editor.snapshot(window, cx);
                let (excerpt_id, _, snapshot) = snapshot.buffer_snapshot.as_singleton().unwrap();

                let anchor = snapshot.anchor_before(snapshot.len());
                message_editor.edit(
                    [(
                        multi_buffer::Anchor::max()..multi_buffer::Anchor::max(),
                        format!("{replacement_text} "),
                    )],
                    cx,
                );
                (*excerpt_id, anchor)
            });

            self.insert_image(
                excerpt_id,
                anchor,
                replacement_text.len(),
                Arc::new(image),
                None,
                window,
                cx,
            );
        }
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
            let Some(completion) = ContextPickerCompletionProvider::completion_for_path(
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
            ) else {
                continue;
            };

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

    fn insert_image(
        &mut self,
        excerpt_id: ExcerptId,
        crease_start: text::Anchor,
        content_len: usize,
        image: Arc<Image>,
        abs_path: Option<Arc<Path>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(crease_id) = insert_crease_for_image(
            excerpt_id,
            crease_start,
            content_len,
            self.editor.clone(),
            window,
            cx,
        ) else {
            return;
        };
        self.editor.update(cx, |_editor, cx| {
            let format = image.format;
            let convert = LanguageModelImage::from_image(image, cx);

            let task = cx
                .spawn_in(window, async move |editor, cx| {
                    if let Some(image) = convert.await {
                        Ok(MentionImage {
                            abs_path,
                            data: image.source,
                            format,
                        })
                    } else {
                        editor
                            .update(cx, |editor, cx| {
                                let snapshot = editor.buffer().read(cx).snapshot(cx);
                                let Some(anchor) =
                                    snapshot.anchor_in_excerpt(excerpt_id, crease_start)
                                else {
                                    return;
                                };
                                editor.display_map.update(cx, |display_map, cx| {
                                    display_map.unfold_intersecting(vec![anchor..anchor], true, cx);
                                });
                                editor.remove_creases([crease_id], cx);
                            })
                            .ok();
                        Err("Failed to convert image".to_string())
                    }
                })
                .shared();

            cx.spawn_in(window, {
                let task = task.clone();
                async move |_, cx| task.clone().await.notify_async_err(cx)
            })
            .detach();

            self.mention_set.lock().insert_image(crease_id, task);
        });
    }

    pub fn set_mode(&mut self, mode: EditorMode, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.set_mode(mode);
            cx.notify()
        });
    }

    pub fn set_message(
        &mut self,
        message: Vec<acp::ContentBlock>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut text = String::new();
        let mut mentions = Vec::new();
        let mut images = Vec::new();

        for chunk in message {
            match chunk {
                acp::ContentBlock::Text(text_content) => {
                    text.push_str(&text_content.text);
                }
                acp::ContentBlock::Resource(acp::EmbeddedResource {
                    resource: acp::EmbeddedResourceResource::TextResourceContents(resource),
                    ..
                }) => {
                    if let Some(mention_uri) = MentionUri::parse(&resource.uri).log_err() {
                        let start = text.len();
                        write!(&mut text, "{}", mention_uri.as_link()).ok();
                        let end = text.len();
                        mentions.push((start..end, mention_uri));
                    }
                }
                acp::ContentBlock::Image(content) => {
                    let start = text.len();
                    text.push_str("image");
                    let end = text.len();
                    images.push((start..end, content));
                }
                acp::ContentBlock::Audio(_)
                | acp::ContentBlock::Resource(_)
                | acp::ContentBlock::ResourceLink(_) => {}
            }
        }

        let snapshot = self.editor.update(cx, |editor, cx| {
            editor.set_text(text, window, cx);
            editor.buffer().read(cx).snapshot(cx)
        });

        self.mention_set.lock().clear();
        for (range, mention_uri) in mentions {
            let anchor = snapshot.anchor_before(range.start);
            let crease_id = crate::context_picker::insert_crease_for_mention(
                anchor.excerpt_id,
                anchor.text_anchor,
                range.end - range.start,
                mention_uri.name().into(),
                mention_uri.icon_path(cx),
                self.editor.clone(),
                window,
                cx,
            );

            if let Some(crease_id) = crease_id {
                self.mention_set.lock().insert_uri(crease_id, mention_uri);
            }
        }
        for (range, content) in images {
            let Some(format) = ImageFormat::from_mime_type(&content.mime_type) else {
                continue;
            };
            let anchor = snapshot.anchor_before(range.start);
            let abs_path = content
                .uri
                .as_ref()
                .and_then(|uri| uri.strip_prefix("file://").map(|s| Path::new(s).into()));

            let name = content
                .uri
                .as_ref()
                .and_then(|uri| {
                    uri.strip_prefix("file://")
                        .and_then(|path| Path::new(path).file_name())
                })
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or("Image".to_owned());
            let crease_id = crate::context_picker::insert_crease_for_mention(
                anchor.excerpt_id,
                anchor.text_anchor,
                range.end - range.start,
                name.into(),
                IconName::Image.path().into(),
                self.editor.clone(),
                window,
                cx,
            );
            let data: SharedString = content.data.to_string().into();

            if let Some(crease_id) = crease_id {
                self.mention_set.lock().insert_image(
                    crease_id,
                    Task::ready(Ok(MentionImage {
                        abs_path,
                        data,
                        format,
                    }))
                    .shared(),
                );
            }
        }
        cx.notify();
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
            .on_action(cx.listener(Self::chat))
            .on_action(cx.listener(Self::cancel))
            .capture_action(cx.listener(Self::paste))
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

pub(crate) fn insert_crease_for_image(
    excerpt_id: ExcerptId,
    anchor: text::Anchor,
    content_len: usize,
    editor: Entity<Editor>,
    window: &mut Window,
    cx: &mut App,
) -> Option<CreaseId> {
    crate::context_picker::insert_crease_for_mention(
        excerpt_id,
        anchor,
        content_len,
        "Image".into(),
        IconName::Image.path().into(),
        editor,
        window,
        cx,
    )
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use agent::{TextThreadStore, ThreadStore};
    use agent_client_protocol as acp;
    use editor::EditorMode;
    use fs::FakeFs;
    use gpui::{AppContext, TestAppContext};
    use lsp::{CompletionContext, CompletionTriggerKind};
    use project::{CompletionIntent, Project};
    use serde_json::json;
    use util::path;
    use workspace::Workspace;

    use crate::acp::{message_editor::MessageEditor, thread_view::tests::init_test};

    #[gpui::test]
    async fn test_at_mention_removal(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({"file": ""})).await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let thread_store = cx.new(|cx| ThreadStore::fake(project.clone(), cx));
        let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                MessageEditor::new(
                    workspace.downgrade(),
                    project.clone(),
                    thread_store.clone(),
                    text_thread_store.clone(),
                    EditorMode::AutoHeight {
                        min_lines: 1,
                        max_lines: None,
                    },
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
            editor.set_text("Hello @file ", window, cx);
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
            .update_in(cx, |message_editor, window, cx| {
                message_editor.contents(window, cx)
            })
            .await
            .unwrap();

        // We don't send a resource link for the deleted crease.
        pretty_assertions::assert_matches!(content.as_slice(), [acp::ContentBlock::Text { .. }]);
    }
}
