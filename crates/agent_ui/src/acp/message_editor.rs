use crate::{
    acp::completion_provider::ContextPickerCompletionProvider,
    context_picker::fetch_context_picker::fetch_url_content,
};
use acp_thread::{MentionUri, selection_name};
use agent::{TextThreadStore, ThreadId, ThreadStore};
use agent_client_protocol as acp;
use anyhow::{Context as _, Result, anyhow};
use collections::{HashMap, HashSet};
use editor::{
    Anchor, AnchorRangeExt, ContextMenuOptions, ContextMenuPlacement, Editor, EditorElement,
    EditorMode, EditorStyle, ExcerptId, FoldPlaceholder, MultiBuffer, ToOffset,
    actions::Paste,
    display_map::{Crease, CreaseId, FoldId},
};
use futures::{
    FutureExt as _, TryFutureExt as _,
    future::{Shared, try_join_all},
};
use gpui::{
    AppContext, ClipboardEntry, Context, Entity, EventEmitter, FocusHandle, Focusable, Image,
    ImageFormat, Img, Task, TextStyle, WeakEntity,
};
use language::{Buffer, Language};
use language_model::LanguageModelImage;
use project::{CompletionIntent, Project};
use rope::Point;
use settings::Settings;
use std::{
    ffi::OsStr,
    fmt::Write,
    ops::Range,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use text::OffsetRangeExt;
use theme::ThemeSettings;
use ui::{
    ActiveTheme, AnyElement, App, ButtonCommon, ButtonLike, ButtonStyle, Color, Icon, IconName,
    IconSize, InteractiveElement, IntoElement, Label, LabelCommon, LabelSize, ParentElement,
    Render, SelectableButton, SharedString, Styled, TextSize, TintColor, Toggleable, Window, div,
    h_flex,
};
use url::Url;
use util::ResultExt;
use workspace::{Workspace, notifications::NotifyResultExt as _};
use zed_actions::agent::Chat;

pub struct MessageEditor {
    mention_set: MentionSet,
    editor: Entity<Editor>,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    thread_store: Entity<ThreadStore>,
    text_thread_store: Entity<TextThreadStore>,
}

#[derive(Clone, Copy)]
pub enum MessageEditorEvent {
    Send,
    Cancel,
    Focus,
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
        let completion_provider = ContextPickerCompletionProvider::new(
            workspace.clone(),
            thread_store.downgrade(),
            text_thread_store.downgrade(),
            cx.weak_entity(),
        );
        let mention_set = MentionSet::default();
        let editor = cx.new(|cx| {
            let buffer = cx.new(|cx| Buffer::local("", cx).with_language(Arc::new(language), cx));
            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

            let mut editor = Editor::new(mode, buffer, None, window, cx);
            editor.set_placeholder_text("Message the agent － @ to include files", cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_soft_wrap();
            editor.set_use_modal_editing(true);
            editor.set_completion_provider(Some(Rc::new(completion_provider)));
            editor.set_context_menu_options(ContextMenuOptions {
                min_entries_visible: 12,
                max_entries_visible: 12,
                placement: Some(ContextMenuPlacement::Above),
            });
            editor
        });

        cx.on_focus(&editor.focus_handle(cx), window, |_, _, cx| {
            cx.emit(MessageEditorEvent::Focus)
        })
        .detach();

        Self {
            editor,
            project,
            mention_set,
            thread_store,
            text_thread_store,
            workspace,
        }
    }

    #[cfg(test)]
    pub(crate) fn editor(&self) -> &Entity<Editor> {
        &self.editor
    }

    #[cfg(test)]
    pub(crate) fn mention_set(&mut self) -> &mut MentionSet {
        &mut self.mention_set
    }

    pub fn is_empty(&self, cx: &App) -> bool {
        self.editor.read(cx).is_empty(cx)
    }

    pub fn mentioned_path_and_threads(&self) -> (HashSet<PathBuf>, HashSet<ThreadId>) {
        let mut excluded_paths = HashSet::default();
        let mut excluded_threads = HashSet::default();

        for uri in self.mention_set.uri_by_crease_id.values() {
            match uri {
                MentionUri::File { abs_path, .. } => {
                    excluded_paths.insert(abs_path.clone());
                }
                MentionUri::Thread { id, .. } => {
                    excluded_threads.insert(id.clone());
                }
                _ => {}
            }
        }

        (excluded_paths, excluded_threads)
    }

    pub fn confirm_completion(
        &mut self,
        crease_text: SharedString,
        start: text::Anchor,
        content_len: usize,
        mention_uri: MentionUri,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self
            .editor
            .update(cx, |editor, cx| editor.snapshot(window, cx));
        let Some((excerpt_id, _, _)) = snapshot.buffer_snapshot.as_singleton() else {
            return;
        };
        let Some(anchor) = snapshot
            .buffer_snapshot
            .anchor_in_excerpt(*excerpt_id, start)
        else {
            return;
        };

        let Some(crease_id) = crate::context_picker::insert_crease_for_mention(
            *excerpt_id,
            start,
            content_len,
            crease_text.clone(),
            mention_uri.icon_path(cx),
            self.editor.clone(),
            window,
            cx,
        ) else {
            return;
        };

        match mention_uri {
            MentionUri::Fetch { url } => {
                self.confirm_mention_for_fetch(crease_id, anchor, url, window, cx);
            }
            MentionUri::File {
                abs_path,
                is_directory,
            } => {
                self.confirm_mention_for_file(
                    crease_id,
                    anchor,
                    abs_path,
                    is_directory,
                    window,
                    cx,
                );
            }
            MentionUri::Symbol { .. }
            | MentionUri::Thread { .. }
            | MentionUri::TextThread { .. }
            | MentionUri::Rule { .. }
            | MentionUri::Selection { .. } => {
                self.mention_set.insert_uri(crease_id, mention_uri.clone());
            }
        }
    }

    fn confirm_mention_for_file(
        &mut self,
        crease_id: CreaseId,
        anchor: Anchor,
        abs_path: PathBuf,
        is_directory: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let extension = abs_path
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_default();

        if Img::extensions().contains(&extension) && !extension.contains("svg") {
            let project = self.project.clone();
            let Some(project_path) = project
                .read(cx)
                .project_path_for_absolute_path(&abs_path, cx)
            else {
                return;
            };
            let image = cx.spawn(async move |_, cx| {
                let image = project
                    .update(cx, |project, cx| project.open_image(project_path, cx))?
                    .await?;
                image.read_with(cx, |image, _cx| image.image.clone())
            });
            self.confirm_mention_for_image(crease_id, anchor, Some(abs_path), image, window, cx);
        } else {
            self.mention_set.insert_uri(
                crease_id,
                MentionUri::File {
                    abs_path,
                    is_directory,
                },
            );
        }
    }

    fn confirm_mention_for_fetch(
        &mut self,
        crease_id: CreaseId,
        anchor: Anchor,
        url: url::Url,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(http_client) = self
            .workspace
            .update(cx, |workspace, _cx| workspace.client().http_client())
            .ok()
        else {
            return;
        };

        let url_string = url.to_string();
        let fetch = cx
            .background_executor()
            .spawn(async move {
                fetch_url_content(http_client, url_string)
                    .map_err(|e| e.to_string())
                    .await
            })
            .shared();
        self.mention_set
            .add_fetch_result(url.clone(), fetch.clone());

        cx.spawn_in(window, async move |this, cx| {
            let fetch = fetch.await.notify_async_err(cx);
            this.update(cx, |this, cx| {
                let mention_uri = MentionUri::Fetch { url };
                if fetch.is_some() {
                    this.mention_set.insert_uri(crease_id, mention_uri.clone());
                } else {
                    // Remove crease if we failed to fetch
                    this.editor.update(cx, |editor, cx| {
                        editor.display_map.update(cx, |display_map, cx| {
                            display_map.unfold_intersecting(vec![anchor..anchor], true, cx);
                        });
                        editor.remove_creases([crease_id], cx);
                    });
                }
            })
            .ok();
        })
        .detach();
    }

    pub fn confirm_mention_for_selection(
        &mut self,
        source_range: Range<text::Anchor>,
        selections: Vec<(Entity<Buffer>, Range<text::Anchor>, Range<usize>)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self.editor.read(cx).buffer().read(cx).snapshot(cx);
        let Some((&excerpt_id, _, _)) = snapshot.as_singleton() else {
            return;
        };
        let Some(start) = snapshot.anchor_in_excerpt(excerpt_id, source_range.start) else {
            return;
        };

        let offset = start.to_offset(&snapshot);

        for (buffer, selection_range, range_to_fold) in selections {
            let range = snapshot.anchor_after(offset + range_to_fold.start)
                ..snapshot.anchor_after(offset + range_to_fold.end);

            let path = buffer
                .read(cx)
                .file()
                .map_or(PathBuf::from("untitled"), |file| file.path().to_path_buf());
            let snapshot = buffer.read(cx).snapshot();

            let point_range = selection_range.to_point(&snapshot);
            let line_range = point_range.start.row..point_range.end.row;

            let uri = MentionUri::Selection {
                path: path.clone(),
                line_range: line_range.clone(),
            };
            let crease = crate::context_picker::crease_for_mention(
                selection_name(&path, &line_range).into(),
                uri.icon_path(cx),
                range,
                self.editor.downgrade(),
            );

            let crease_id = self.editor.update(cx, |editor, cx| {
                let crease_ids = editor.insert_creases(vec![crease.clone()], cx);
                editor.fold_creases(vec![crease], false, window, cx);
                crease_ids.first().copied().unwrap()
            });

            self.mention_set
                .insert_uri(crease_id, MentionUri::Selection { path, line_range });
        }
    }

    pub fn contents(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<Vec<acp::ContentBlock>>> {
        let contents = self.mention_set.contents(
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
            editor.remove_creases(self.mention_set.drain(), cx)
        });
    }

    fn send(&mut self, _: &Chat, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(MessageEditorEvent::Send)
    }

    fn cancel(&mut self, _: &editor::actions::Cancel, _: &mut Window, cx: &mut Context<Self>) {
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
            let (excerpt_id, text_anchor, multibuffer_anchor) =
                self.editor.update(cx, |message_editor, cx| {
                    let snapshot = message_editor.snapshot(window, cx);
                    let (excerpt_id, _, buffer_snapshot) =
                        snapshot.buffer_snapshot.as_singleton().unwrap();

                    let text_anchor = buffer_snapshot.anchor_before(buffer_snapshot.len());
                    let multibuffer_anchor = snapshot
                        .buffer_snapshot
                        .anchor_in_excerpt(*excerpt_id, text_anchor);
                    message_editor.edit(
                        [(
                            multi_buffer::Anchor::max()..multi_buffer::Anchor::max(),
                            format!("{replacement_text} "),
                        )],
                        cx,
                    );
                    (*excerpt_id, text_anchor, multibuffer_anchor)
                });

            let content_len = replacement_text.len();
            let Some(anchor) = multibuffer_anchor else {
                return;
            };
            let Some(crease_id) = insert_crease_for_image(
                excerpt_id,
                text_anchor,
                content_len,
                None.clone(),
                self.editor.clone(),
                window,
                cx,
            ) else {
                return;
            };
            self.confirm_mention_for_image(
                crease_id,
                anchor,
                None,
                Task::ready(Ok(Arc::new(image))),
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
                anchor..anchor,
                cx.weak_entity(),
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

    pub fn set_read_only(&mut self, read_only: bool, cx: &mut Context<Self>) {
        self.editor.update(cx, |message_editor, cx| {
            message_editor.set_read_only(read_only);
            cx.notify()
        })
    }

    fn confirm_mention_for_image(
        &mut self,
        crease_id: CreaseId,
        anchor: Anchor,
        abs_path: Option<PathBuf>,
        image: Task<Result<Arc<Image>>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let editor = self.editor.clone();
        let task = cx
            .spawn_in(window, async move |this, cx| {
                let image = image.await.map_err(|e| e.to_string())?;
                let format = image.format;
                let image = cx
                    .update(|_, cx| LanguageModelImage::from_image(image, cx))
                    .map_err(|e| e.to_string())?
                    .await;
                if let Some(image) = image {
                    if let Some(abs_path) = abs_path.clone() {
                        this.update(cx, |this, _cx| {
                            this.mention_set.insert_uri(
                                crease_id,
                                MentionUri::File {
                                    abs_path,
                                    is_directory: false,
                                },
                            );
                        })
                        .map_err(|e| e.to_string())?;
                    }
                    Ok(MentionImage {
                        abs_path,
                        data: image.source,
                        format,
                    })
                } else {
                    editor
                        .update(cx, |editor, cx| {
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

        self.mention_set.insert_image(crease_id, task);
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
        self.clear(window, cx);

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
                self.mention_set.insert_uri(crease_id, mention_uri);
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
                self.mention_set.insert_image(
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

    #[cfg(test)]
    pub fn text(&self, cx: &App) -> String {
        self.editor.read(cx).text(cx)
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
            .on_action(cx.listener(Self::send))
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
    abs_path: Option<Arc<Path>>,
    editor: Entity<Editor>,
    window: &mut Window,
    cx: &mut App,
) -> Option<CreaseId> {
    let crease_label = abs_path
        .as_ref()
        .and_then(|path| path.file_name())
        .map(|name| name.to_string_lossy().to_string().into())
        .unwrap_or(SharedString::from("Image"));

    editor.update(cx, |editor, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);

        let start = snapshot.anchor_in_excerpt(excerpt_id, anchor)?;

        let start = start.bias_right(&snapshot);
        let end = snapshot.anchor_before(start.to_offset(&snapshot) + content_len);

        let placeholder = FoldPlaceholder {
            render: render_image_fold_icon_button(crease_label, cx.weak_entity()),
            merge_adjacent: false,
            ..Default::default()
        };

        let crease = Crease::Inline {
            range: start..end,
            placeholder,
            render_toggle: None,
            render_trailer: None,
            metadata: None,
        };

        let ids = editor.insert_creases(vec![crease.clone()], cx);
        editor.fold_creases(vec![crease], false, window, cx);

        Some(ids[0])
    })
}

fn render_image_fold_icon_button(
    label: SharedString,
    editor: WeakEntity<Editor>,
) -> Arc<dyn Send + Sync + Fn(FoldId, Range<Anchor>, &mut App) -> AnyElement> {
    Arc::new({
        move |fold_id, fold_range, cx| {
            let is_in_text_selection = editor
                .update(cx, |editor, cx| editor.is_range_selected(&fold_range, cx))
                .unwrap_or_default();

            ButtonLike::new(fold_id)
                .style(ButtonStyle::Filled)
                .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                .toggle_state(is_in_text_selection)
                .child(
                    h_flex()
                        .gap_1()
                        .child(
                            Icon::new(IconName::Image)
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        )
                        .child(
                            Label::new(label.clone())
                                .size(LabelSize::Small)
                                .buffer_font(cx)
                                .single_line(),
                        ),
                )
                .into_any_element()
        }
    })
}

#[derive(Debug, Eq, PartialEq)]
pub enum Mention {
    Text { uri: MentionUri, content: String },
    Image(MentionImage),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MentionImage {
    pub abs_path: Option<PathBuf>,
    pub data: SharedString,
    pub format: ImageFormat,
}

#[derive(Default)]
pub struct MentionSet {
    pub(crate) uri_by_crease_id: HashMap<CreaseId, MentionUri>,
    fetch_results: HashMap<Url, Shared<Task<Result<String, String>>>>,
    images: HashMap<CreaseId, Shared<Task<Result<MentionImage, String>>>>,
}

impl MentionSet {
    pub fn insert_uri(&mut self, crease_id: CreaseId, uri: MentionUri) {
        self.uri_by_crease_id.insert(crease_id, uri);
    }

    pub fn add_fetch_result(&mut self, url: Url, content: Shared<Task<Result<String, String>>>) {
        self.fetch_results.insert(url, content);
    }

    pub fn insert_image(
        &mut self,
        crease_id: CreaseId,
        task: Shared<Task<Result<MentionImage, String>>>,
    ) {
        self.images.insert(crease_id, task);
    }

    pub fn drain(&mut self) -> impl Iterator<Item = CreaseId> {
        self.fetch_results.clear();
        self.uri_by_crease_id
            .drain()
            .map(|(id, _)| id)
            .chain(self.images.drain().map(|(id, _)| id))
    }

    pub fn contents(
        &self,
        project: Entity<Project>,
        thread_store: Entity<ThreadStore>,
        text_thread_store: Entity<TextThreadStore>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<HashMap<CreaseId, Mention>>> {
        let mut processed_image_creases = HashSet::default();

        let mut contents = self
            .uri_by_crease_id
            .iter()
            .map(|(&crease_id, uri)| {
                match uri {
                    MentionUri::File { abs_path, .. } => {
                        // TODO directories
                        let uri = uri.clone();
                        let abs_path = abs_path.to_path_buf();

                        if let Some(task) = self.images.get(&crease_id).cloned() {
                            processed_image_creases.insert(crease_id);
                            return cx.spawn(async move |_| {
                                let image = task.await.map_err(|e| anyhow!("{e}"))?;
                                anyhow::Ok((crease_id, Mention::Image(image)))
                            });
                        }

                        let buffer_task = project.update(cx, |project, cx| {
                            let path = project
                                .find_project_path(abs_path, cx)
                                .context("Failed to find project path")?;
                            anyhow::Ok(project.open_buffer(path, cx))
                        });
                        cx.spawn(async move |cx| {
                            let buffer = buffer_task?.await?;
                            let content = buffer.read_with(cx, |buffer, _cx| buffer.text())?;

                            anyhow::Ok((crease_id, Mention::Text { uri, content }))
                        })
                    }
                    MentionUri::Symbol {
                        path, line_range, ..
                    }
                    | MentionUri::Selection {
                        path, line_range, ..
                    } => {
                        let uri = uri.clone();
                        let path_buf = path.clone();
                        let line_range = line_range.clone();

                        let buffer_task = project.update(cx, |project, cx| {
                            let path = project
                                .find_project_path(&path_buf, cx)
                                .context("Failed to find project path")?;
                            anyhow::Ok(project.open_buffer(path, cx))
                        });

                        cx.spawn(async move |cx| {
                            let buffer = buffer_task?.await?;
                            let content = buffer.read_with(cx, |buffer, _cx| {
                                buffer
                                    .text_for_range(
                                        Point::new(line_range.start, 0)
                                            ..Point::new(
                                                line_range.end,
                                                buffer.line_len(line_range.end),
                                            ),
                                    )
                                    .collect()
                            })?;

                            anyhow::Ok((crease_id, Mention::Text { uri, content }))
                        })
                    }
                    MentionUri::Thread { id: thread_id, .. } => {
                        let open_task = thread_store.update(cx, |thread_store, cx| {
                            thread_store.open_thread(&thread_id, window, cx)
                        });

                        let uri = uri.clone();
                        cx.spawn(async move |cx| {
                            let thread = open_task.await?;
                            let content = thread.read_with(cx, |thread, _cx| {
                                thread.latest_detailed_summary_or_text().to_string()
                            })?;

                            anyhow::Ok((crease_id, Mention::Text { uri, content }))
                        })
                    }
                    MentionUri::TextThread { path, .. } => {
                        let context = text_thread_store.update(cx, |text_thread_store, cx| {
                            text_thread_store.open_local_context(path.as_path().into(), cx)
                        });
                        let uri = uri.clone();
                        cx.spawn(async move |cx| {
                            let context = context.await?;
                            let xml = context.update(cx, |context, cx| context.to_xml(cx))?;
                            anyhow::Ok((crease_id, Mention::Text { uri, content: xml }))
                        })
                    }
                    MentionUri::Rule { id: prompt_id, .. } => {
                        let Some(prompt_store) = thread_store.read(cx).prompt_store().clone()
                        else {
                            return Task::ready(Err(anyhow!("missing prompt store")));
                        };
                        let text_task = prompt_store.read(cx).load(*prompt_id, cx);
                        let uri = uri.clone();
                        cx.spawn(async move |_| {
                            // TODO: report load errors instead of just logging
                            let text = text_task.await?;
                            anyhow::Ok((crease_id, Mention::Text { uri, content: text }))
                        })
                    }
                    MentionUri::Fetch { url } => {
                        let Some(content) = self.fetch_results.get(&url).cloned() else {
                            return Task::ready(Err(anyhow!("missing fetch result")));
                        };
                        let uri = uri.clone();
                        cx.spawn(async move |_| {
                            Ok((
                                crease_id,
                                Mention::Text {
                                    uri,
                                    content: content.await.map_err(|e| anyhow::anyhow!("{e}"))?,
                                },
                            ))
                        })
                    }
                }
            })
            .collect::<Vec<_>>();

        // Handle images that didn't have a mention URI (because they were added by the paste handler).
        contents.extend(self.images.iter().filter_map(|(crease_id, image)| {
            if processed_image_creases.contains(crease_id) {
                return None;
            }
            let crease_id = *crease_id;
            let image = image.clone();
            Some(cx.spawn(async move |_| {
                Ok((
                    crease_id,
                    Mention::Image(image.await.map_err(|e| anyhow::anyhow!("{e}"))?),
                ))
            }))
        }));

        cx.spawn(async move |_cx| {
            let contents = try_join_all(contents).await?.into_iter().collect();
            anyhow::Ok(contents)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{ops::Range, path::Path, sync::Arc};

    use agent::{TextThreadStore, ThreadStore};
    use agent_client_protocol as acp;
    use editor::{AnchorRangeExt as _, Editor, EditorMode};
    use fs::FakeFs;
    use futures::StreamExt as _;
    use gpui::{
        AppContext, Entity, EventEmitter, FocusHandle, Focusable, TestAppContext, VisualTestContext,
    };
    use lsp::{CompletionContext, CompletionTriggerKind};
    use project::{CompletionIntent, Project, ProjectPath};
    use serde_json::json;
    use text::Point;
    use ui::{App, Context, IntoElement, Render, SharedString, Window};
    use util::path;
    use workspace::{AppState, Item, Workspace};

    use crate::acp::{
        message_editor::{Mention, MessageEditor},
        thread_view::tests::init_test,
    };

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

    struct MessageEditorItem(Entity<MessageEditor>);

    impl Item for MessageEditorItem {
        type Event = ();

        fn include_in_nav_history() -> bool {
            false
        }

        fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
            "Test".into()
        }
    }

    impl EventEmitter<()> for MessageEditorItem {}

    impl Focusable for MessageEditorItem {
        fn focus_handle(&self, cx: &App) -> FocusHandle {
            self.0.read(cx).focus_handle(cx).clone()
        }
    }

    impl Render for MessageEditorItem {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.0.clone().into_any_element()
        }
    }

    #[gpui::test]
    async fn test_context_completion_provider(cx: &mut TestAppContext) {
        init_test(cx);

        let app_state = cx.update(AppState::test);

        cx.update(|cx| {
            language::init(cx);
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
            Project::init_settings(cx);
        });

        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/dir"),
                json!({
                    "editor": "",
                    "a": {
                        "one.txt": "1",
                        "two.txt": "2",
                        "three.txt": "3",
                        "four.txt": "4"
                    },
                    "b": {
                        "five.txt": "5",
                        "six.txt": "6",
                        "seven.txt": "7",
                        "eight.txt": "8",
                    }
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = window.root(cx).unwrap();

        let worktree = project.update(cx, |project, cx| {
            let mut worktrees = project.worktrees(cx).collect::<Vec<_>>();
            assert_eq!(worktrees.len(), 1);
            worktrees.pop().unwrap()
        });
        let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id());

        let mut cx = VisualTestContext::from_window(*window, cx);

        let paths = vec![
            path!("a/one.txt"),
            path!("a/two.txt"),
            path!("a/three.txt"),
            path!("a/four.txt"),
            path!("b/five.txt"),
            path!("b/six.txt"),
            path!("b/seven.txt"),
            path!("b/eight.txt"),
        ];

        let mut opened_editors = Vec::new();
        for path in paths {
            let buffer = workspace
                .update_in(&mut cx, |workspace, window, cx| {
                    workspace.open_path(
                        ProjectPath {
                            worktree_id,
                            path: Path::new(path).into(),
                        },
                        None,
                        false,
                        window,
                        cx,
                    )
                })
                .await
                .unwrap();
            opened_editors.push(buffer);
        }

        let thread_store = cx.new(|cx| ThreadStore::fake(project.clone(), cx));
        let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));

        let (message_editor, editor) = workspace.update_in(&mut cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            let message_editor = cx.new(|cx| {
                MessageEditor::new(
                    workspace_handle,
                    project.clone(),
                    thread_store.clone(),
                    text_thread_store.clone(),
                    EditorMode::AutoHeight {
                        max_lines: None,
                        min_lines: 1,
                    },
                    window,
                    cx,
                )
            });
            workspace.active_pane().update(cx, |pane, cx| {
                pane.add_item(
                    Box::new(cx.new(|_| MessageEditorItem(message_editor.clone()))),
                    true,
                    true,
                    None,
                    window,
                    cx,
                );
            });
            message_editor.read(cx).focus_handle(cx).focus(window);
            let editor = message_editor.read(cx).editor().clone();
            (message_editor, editor)
        });

        cx.simulate_input("Lorem ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "Lorem ");
            assert!(!editor.has_visible_completions_menu());
        });

        cx.simulate_input("@");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "Lorem @");
            assert!(editor.has_visible_completions_menu());
            assert_eq!(
                current_completion_labels(editor),
                &[
                    "eight.txt dir/b/",
                    "seven.txt dir/b/",
                    "six.txt dir/b/",
                    "five.txt dir/b/",
                    "Files & Directories",
                    "Symbols",
                    "Threads",
                    "Fetch"
                ]
            );
        });

        // Select and confirm "File"
        editor.update_in(&mut cx, |editor, window, cx| {
            assert!(editor.has_visible_completions_menu());
            editor.context_menu_next(&editor::actions::ContextMenuNext, window, cx);
            editor.context_menu_next(&editor::actions::ContextMenuNext, window, cx);
            editor.context_menu_next(&editor::actions::ContextMenuNext, window, cx);
            editor.context_menu_next(&editor::actions::ContextMenuNext, window, cx);
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        cx.run_until_parked();

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "Lorem @file ");
            assert!(editor.has_visible_completions_menu());
        });

        cx.simulate_input("one");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "Lorem @file one");
            assert!(editor.has_visible_completions_menu());
            assert_eq!(current_completion_labels(editor), vec!["one.txt dir/a/"]);
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            assert!(editor.has_visible_completions_menu());
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "Lorem [@one.txt](file:///dir/a/one.txt) ");
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 39)]
            );
        });

        let contents = message_editor
            .update_in(&mut cx, |message_editor, window, cx| {
                message_editor.mention_set().contents(
                    project.clone(),
                    thread_store.clone(),
                    text_thread_store.clone(),
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .into_values()
            .collect::<Vec<_>>();

        pretty_assertions::assert_eq!(
            contents,
            [Mention::Text {
                content: "1".into(),
                uri: "file:///dir/a/one.txt".parse().unwrap()
            }]
        );

        cx.simulate_input(" ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "Lorem [@one.txt](file:///dir/a/one.txt)  ");
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 39)]
            );
        });

        cx.simulate_input("Ipsum ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "Lorem [@one.txt](file:///dir/a/one.txt)  Ipsum ",
            );
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 39)]
            );
        });

        cx.simulate_input("@file ");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                "Lorem [@one.txt](file:///dir/a/one.txt)  Ipsum @file ",
            );
            assert!(editor.has_visible_completions_menu());
            assert_eq!(
                fold_ranges(editor, cx),
                vec![Point::new(0, 6)..Point::new(0, 39)]
            );
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        cx.run_until_parked();

        let contents = message_editor
            .update_in(&mut cx, |message_editor, window, cx| {
                message_editor.mention_set().contents(
                    project.clone(),
                    thread_store.clone(),
                    text_thread_store.clone(),
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .into_values()
            .collect::<Vec<_>>();

        assert_eq!(contents.len(), 2);
        pretty_assertions::assert_eq!(
            contents[1],
            Mention::Text {
                content: "8".to_string(),
                uri: "file:///dir/b/eight.txt".parse().unwrap(),
            }
        );

        editor.update(&mut cx, |editor, cx| {
                assert_eq!(
                    editor.text(cx),
                    "Lorem [@one.txt](file:///dir/a/one.txt)  Ipsum [@eight.txt](file:///dir/b/eight.txt) "
                );
                assert!(!editor.has_visible_completions_menu());
                assert_eq!(
                    fold_ranges(editor, cx),
                    vec![
                        Point::new(0, 6)..Point::new(0, 39),
                        Point::new(0, 47)..Point::new(0, 84)
                    ]
                );
            });

        let plain_text_language = Arc::new(language::Language::new(
            language::LanguageConfig {
                name: "Plain Text".into(),
                matcher: language::LanguageMatcher {
                    path_suffixes: vec!["txt".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        ));

        // Register the language and fake LSP
        let language_registry = project.read_with(&cx, |project, _| project.languages().clone());
        language_registry.add(plain_text_language);

        let mut fake_language_servers = language_registry.register_fake_lsp(
            "Plain Text",
            language::FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    workspace_symbol_provider: Some(lsp::OneOf::Left(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        // Open the buffer to trigger LSP initialization
        let buffer = project
            .update(&mut cx, |project, cx| {
                project.open_local_buffer(path!("/dir/a/one.txt"), cx)
            })
            .await
            .unwrap();

        // Register the buffer with language servers
        let _handle = project.update(&mut cx, |project, cx| {
            project.register_buffer_with_language_servers(&buffer, cx)
        });

        cx.run_until_parked();

        let fake_language_server = fake_language_servers.next().await.unwrap();
        fake_language_server.set_request_handler::<lsp::WorkspaceSymbolRequest, _, _>(
            |_, _| async move {
                Ok(Some(lsp::WorkspaceSymbolResponse::Flat(vec![
                    #[allow(deprecated)]
                    lsp::SymbolInformation {
                        name: "MySymbol".into(),
                        location: lsp::Location {
                            uri: lsp::Url::from_file_path(path!("/dir/a/one.txt")).unwrap(),
                            range: lsp::Range::new(
                                lsp::Position::new(0, 0),
                                lsp::Position::new(0, 1),
                            ),
                        },
                        kind: lsp::SymbolKind::CONSTANT,
                        tags: None,
                        container_name: None,
                        deprecated: None,
                    },
                ])))
            },
        );

        cx.simulate_input("@symbol ");

        editor.update(&mut cx, |editor, cx| {
                assert_eq!(
                    editor.text(cx),
                    "Lorem [@one.txt](file:///dir/a/one.txt)  Ipsum [@eight.txt](file:///dir/b/eight.txt) @symbol "
                );
                assert!(editor.has_visible_completions_menu());
                assert_eq!(
                    current_completion_labels(editor),
                    &[
                        "MySymbol",
                    ]
                );
            });

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        let contents = message_editor
            .update_in(&mut cx, |message_editor, window, cx| {
                message_editor.mention_set().contents(
                    project.clone(),
                    thread_store,
                    text_thread_store,
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .into_values()
            .collect::<Vec<_>>();

        assert_eq!(contents.len(), 3);
        pretty_assertions::assert_eq!(
            contents[2],
            Mention::Text {
                content: "1".into(),
                uri: "file:///dir/a/one.txt?symbol=MySymbol#L1:1"
                    .parse()
                    .unwrap(),
            }
        );

        cx.run_until_parked();

        editor.read_with(&mut cx, |editor, cx| {
                assert_eq!(
                    editor.text(cx),
                    "Lorem [@one.txt](file:///dir/a/one.txt)  Ipsum [@eight.txt](file:///dir/b/eight.txt) [@MySymbol](file:///dir/a/one.txt?symbol=MySymbol#L1:1) "
                );
            });
    }

    fn fold_ranges(editor: &Editor, cx: &mut App) -> Vec<Range<Point>> {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        editor.display_map.update(cx, |display_map, cx| {
            display_map
                .snapshot(cx)
                .folds_in_range(0..snapshot.len())
                .map(|fold| fold.range.to_point(&snapshot))
                .collect()
        })
    }

    fn current_completion_labels(editor: &Editor) -> Vec<String> {
        let completions = editor.current_completions().expect("Missing completions");
        completions
            .into_iter()
            .map(|completion| completion.label.text.to_string())
            .collect::<Vec<_>>()
    }
}
