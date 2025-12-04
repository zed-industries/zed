use crate::{
    ChatWithFollow,
    completion_provider::{
        PromptCompletionProvider, PromptCompletionProviderDelegate, PromptContextAction,
        PromptContextType, SlashCommandCompletion,
    },
    mention_set::{
        Mention, MentionImage, MentionSet, insert_crease_for_mention, paste_images_as_context,
    },
};
use acp_thread::MentionUri;
use agent::HistoryStore;
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use collections::HashSet;
use editor::{
    Addon, AnchorRangeExt, ContextMenuOptions, ContextMenuPlacement, Editor, EditorElement,
    EditorEvent, EditorMode, EditorStyle, Inlay, MultiBuffer, MultiBufferOffset,
    MultiBufferSnapshot, ToOffset, actions::Paste, code_context_menus::CodeContextMenu,
    scroll::Autoscroll,
};
use futures::{FutureExt as _, future::join_all};
use gpui::{
    AppContext, Context, Entity, EventEmitter, FocusHandle, Focusable, ImageFormat, KeyContext,
    SharedString, Subscription, Task, TextStyle, WeakEntity,
};
use language::{Buffer, Language, language_settings::InlayHintKind};
use project::{CompletionIntent, InlayHint, InlayHintLabel, InlayId, Project, Worktree};
use prompt_store::PromptStore;
use rope::Point;
use settings::Settings;
use std::{cell::RefCell, fmt::Write, rc::Rc, sync::Arc};
use theme::ThemeSettings;
use ui::prelude::*;
use util::{ResultExt, debug_panic};
use workspace::{CollaboratorId, Workspace};
use zed_actions::agent::Chat;

pub struct MessageEditor {
    mention_set: Entity<MentionSet>,
    editor: Entity<Editor>,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    prompt_capabilities: Rc<RefCell<acp::PromptCapabilities>>,
    available_commands: Rc<RefCell<Vec<acp::AvailableCommand>>>,
    agent_name: SharedString,
    _subscriptions: Vec<Subscription>,
    _parse_slash_command_task: Task<()>,
}

#[derive(Clone, Copy, Debug)]
pub enum MessageEditorEvent {
    Send,
    Cancel,
    Focus,
    LostFocus,
}

impl EventEmitter<MessageEditorEvent> for MessageEditor {}

const COMMAND_HINT_INLAY_ID: InlayId = InlayId::Hint(0);

impl PromptCompletionProviderDelegate for Entity<MessageEditor> {
    fn supports_images(&self, cx: &App) -> bool {
        self.read(cx).prompt_capabilities.borrow().image
    }

    fn supported_modes(&self, cx: &App) -> Vec<PromptContextType> {
        let mut supported = vec![PromptContextType::File, PromptContextType::Symbol];
        if self.read(cx).prompt_capabilities.borrow().embedded_context {
            supported.extend(&[
                PromptContextType::Thread,
                PromptContextType::Fetch,
                PromptContextType::Rules,
            ]);
        }
        supported
    }

    fn available_commands(&self, cx: &App) -> Vec<crate::completion_provider::AvailableCommand> {
        self.read(cx)
            .available_commands
            .borrow()
            .iter()
            .map(|cmd| crate::completion_provider::AvailableCommand {
                name: cmd.name.clone().into(),
                description: cmd.description.clone().into(),
                requires_argument: cmd.input.is_some(),
            })
            .collect()
    }

    fn confirm_command(&self, cx: &mut App) {
        self.update(cx, |this, cx| this.send(cx));
    }
}

impl MessageEditor {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        history_store: Entity<HistoryStore>,
        prompt_store: Option<Entity<PromptStore>>,
        prompt_capabilities: Rc<RefCell<acp::PromptCapabilities>>,
        available_commands: Rc<RefCell<Vec<acp::AvailableCommand>>>,
        agent_name: SharedString,
        placeholder: &str,
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

        let editor = cx.new(|cx| {
            let buffer = cx.new(|cx| Buffer::local("", cx).with_language(Arc::new(language), cx));
            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

            let mut editor = Editor::new(mode, buffer, None, window, cx);
            editor.set_placeholder_text(placeholder, window, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_soft_wrap();
            editor.set_use_modal_editing(true);
            editor.set_context_menu_options(ContextMenuOptions {
                min_entries_visible: 12,
                max_entries_visible: 12,
                placement: Some(ContextMenuPlacement::Above),
            });
            editor.register_addon(MessageEditorAddon::new());
            editor
        });
        let mention_set = cx.new(|_cx| {
            MentionSet::new(
                project.downgrade(),
                history_store.clone(),
                prompt_store.clone(),
            )
        });
        let completion_provider = Rc::new(PromptCompletionProvider::new(
            cx.entity(),
            editor.downgrade(),
            mention_set.clone(),
            history_store.clone(),
            prompt_store.clone(),
            workspace.clone(),
        ));
        editor.update(cx, |editor, _cx| {
            editor.set_completion_provider(Some(completion_provider.clone()))
        });

        cx.on_focus_in(&editor.focus_handle(cx), window, |_, _, cx| {
            cx.emit(MessageEditorEvent::Focus)
        })
        .detach();
        cx.on_focus_out(&editor.focus_handle(cx), window, |_, _, _, cx| {
            cx.emit(MessageEditorEvent::LostFocus)
        })
        .detach();

        let mut has_hint = false;
        let mut subscriptions = Vec::new();

        subscriptions.push(cx.subscribe_in(&editor, window, {
            move |this, editor, event, window, cx| {
                if let EditorEvent::Edited { .. } = event
                    && !editor.read(cx).read_only(cx)
                {
                    editor.update(cx, |editor, cx| {
                        let snapshot = editor.snapshot(window, cx);
                        this.mention_set
                            .update(cx, |mention_set, _cx| mention_set.remove_invalid(&snapshot));

                        let new_hints = this
                            .command_hint(snapshot.buffer())
                            .into_iter()
                            .collect::<Vec<_>>();
                        let has_new_hint = !new_hints.is_empty();
                        editor.splice_inlays(
                            if has_hint {
                                &[COMMAND_HINT_INLAY_ID]
                            } else {
                                &[]
                            },
                            new_hints,
                            cx,
                        );
                        has_hint = has_new_hint;
                    });
                    cx.notify();
                }
            }
        }));

        Self {
            editor,
            project,
            mention_set,
            workspace,
            prompt_capabilities,
            available_commands,
            agent_name,
            _subscriptions: subscriptions,
            _parse_slash_command_task: Task::ready(()),
        }
    }

    fn command_hint(&self, snapshot: &MultiBufferSnapshot) -> Option<Inlay> {
        let available_commands = self.available_commands.borrow();
        if available_commands.is_empty() {
            return None;
        }

        let parsed_command = SlashCommandCompletion::try_parse(&snapshot.text(), 0)?;
        if parsed_command.argument.is_some() {
            return None;
        }

        let command_name = parsed_command.command?;
        let available_command = available_commands
            .iter()
            .find(|command| command.name == command_name)?;

        let acp::AvailableCommandInput::Unstructured(acp::UnstructuredCommandInput {
            mut hint,
            ..
        }) = available_command.input.clone()?
        else {
            return None;
        };

        let mut hint_pos = MultiBufferOffset(parsed_command.source_range.end) + 1usize;
        if hint_pos > snapshot.len() {
            hint_pos = snapshot.len();
            hint.insert(0, ' ');
        }

        let hint_pos = snapshot.anchor_after(hint_pos);

        Some(Inlay::hint(
            COMMAND_HINT_INLAY_ID,
            hint_pos,
            &InlayHint {
                position: hint_pos.text_anchor,
                label: InlayHintLabel::String(hint),
                kind: Some(InlayHintKind::Parameter),
                padding_left: false,
                padding_right: false,
                tooltip: None,
                resolve_state: project::ResolveState::Resolved,
            },
        ))
    }

    pub fn insert_thread_summary(
        &mut self,
        thread: agent::DbThreadMetadata,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let uri = MentionUri::Thread {
            id: thread.id.clone(),
            name: thread.title.to_string(),
        };
        let content = format!("{}\n", uri.as_link());

        let content_len = content.len() - 1;

        let start = self.editor.update(cx, |editor, cx| {
            editor.set_text(content, window, cx);
            editor
                .buffer()
                .read(cx)
                .snapshot(cx)
                .anchor_before(Point::zero())
                .text_anchor
        });

        let supports_images = self.prompt_capabilities.borrow().image;

        self.mention_set
            .update(cx, |mention_set, cx| {
                mention_set.confirm_mention_completion(
                    thread.title,
                    start,
                    content_len,
                    uri,
                    supports_images,
                    self.editor.clone(),
                    &workspace,
                    window,
                    cx,
                )
            })
            .detach();
    }

    #[cfg(test)]
    pub(crate) fn editor(&self) -> &Entity<Editor> {
        &self.editor
    }

    pub fn is_empty(&self, cx: &App) -> bool {
        self.editor.read(cx).is_empty(cx)
    }

    pub fn is_completions_menu_visible(&self, cx: &App) -> bool {
        self.editor
            .read(cx)
            .context_menu()
            .borrow()
            .as_ref()
            .is_some_and(|menu| matches!(menu, CodeContextMenu::Completions(_)) && menu.visible())
    }

    #[cfg(test)]
    pub fn mention_set(&self) -> &Entity<MentionSet> {
        &self.mention_set
    }

    fn validate_slash_commands(
        text: &str,
        available_commands: &[acp::AvailableCommand],
        agent_name: &str,
    ) -> Result<()> {
        if let Some(parsed_command) = SlashCommandCompletion::try_parse(text, 0) {
            if let Some(command_name) = parsed_command.command {
                // Check if this command is in the list of available commands from the server
                let is_supported = available_commands
                    .iter()
                    .any(|cmd| cmd.name == command_name);

                if !is_supported {
                    return Err(anyhow!(
                        "The /{} command is not supported by {}.\n\nAvailable commands: {}",
                        command_name,
                        agent_name,
                        if available_commands.is_empty() {
                            "none".to_string()
                        } else {
                            available_commands
                                .iter()
                                .map(|cmd| format!("/{}", cmd.name))
                                .collect::<Vec<_>>()
                                .join(", ")
                        }
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn contents(
        &self,
        full_mention_content: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<(Vec<acp::ContentBlock>, Vec<Entity<Buffer>>)>> {
        // Check for unsupported slash commands before spawning async task
        let text = self.editor.read(cx).text(cx);
        let available_commands = self.available_commands.borrow().clone();
        if let Err(err) =
            Self::validate_slash_commands(&text, &available_commands, &self.agent_name)
        {
            return Task::ready(Err(err));
        }

        let contents = self
            .mention_set
            .update(cx, |store, cx| store.contents(full_mention_content, cx));
        let editor = self.editor.clone();
        let supports_embedded_context = self.prompt_capabilities.borrow().embedded_context;

        cx.spawn(async move |_, cx| {
            let contents = contents.await?;
            let mut all_tracked_buffers = Vec::new();

            let result = editor.update(cx, |editor, cx| {
                let (mut ix, _) = text
                    .char_indices()
                    .find(|(_, c)| !c.is_whitespace())
                    .unwrap_or((0, '\0'));
                let mut chunks: Vec<acp::ContentBlock> = Vec::new();
                let text = editor.text(cx);
                editor.display_map.update(cx, |map, cx| {
                    let snapshot = map.snapshot(cx);
                    for (crease_id, crease) in snapshot.crease_snapshot.creases() {
                        let Some((uri, mention)) = contents.get(&crease_id) else {
                            continue;
                        };

                        let crease_range = crease.range().to_offset(&snapshot.buffer_snapshot());
                        if crease_range.start.0 > ix {
                            let chunk = text[ix..crease_range.start.0].into();
                            chunks.push(chunk);
                        }
                        let chunk = match mention {
                            Mention::Text {
                                content,
                                tracked_buffers,
                            } => {
                                all_tracked_buffers.extend(tracked_buffers.iter().cloned());
                                if supports_embedded_context {
                                    acp::ContentBlock::Resource(acp::EmbeddedResource::new(
                                        acp::EmbeddedResourceResource::TextResourceContents(
                                            acp::TextResourceContents::new(
                                                content.clone(),
                                                uri.to_uri().to_string(),
                                            ),
                                        ),
                                    ))
                                } else {
                                    acp::ContentBlock::ResourceLink(acp::ResourceLink::new(
                                        uri.name(),
                                        uri.to_uri().to_string(),
                                    ))
                                }
                            }
                            Mention::Image(mention_image) => {
                                let mut image = acp::ImageContent::new(
                                    mention_image.data.clone(),
                                    mention_image.format.mime_type(),
                                );

                                if let Some(uri) = match uri {
                                    MentionUri::File { .. } => Some(uri.to_uri().to_string()),
                                    MentionUri::PastedImage => None,
                                    other => {
                                        debug_panic!(
                                            "unexpected mention uri for image: {:?}",
                                            other
                                        );
                                        None
                                    }
                                } {
                                    image = image.uri(uri)
                                };
                                acp::ContentBlock::Image(image)
                            }
                            Mention::Link => acp::ContentBlock::ResourceLink(
                                acp::ResourceLink::new(uri.name(), uri.to_uri().to_string()),
                            ),
                        };
                        chunks.push(chunk);
                        ix = crease_range.end.0;
                    }

                    if ix < text.len() {
                        let last_chunk = text[ix..].trim_end().to_owned();
                        if !last_chunk.is_empty() {
                            chunks.push(last_chunk.into());
                        }
                    }
                });
                Ok((chunks, all_tracked_buffers))
            })?;
            result
        })
    }

    pub fn clear(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
            editor.remove_creases(
                self.mention_set.update(cx, |mention_set, _cx| {
                    mention_set
                        .clear()
                        .map(|(crease_id, _)| crease_id)
                        .collect::<Vec<_>>()
                }),
                cx,
            )
        });
    }

    pub fn send(&mut self, cx: &mut Context<Self>) {
        if self.is_empty(cx) {
            return;
        }
        self.editor.update(cx, |editor, cx| {
            editor.clear_inlay_hints(cx);
        });
        cx.emit(MessageEditorEvent::Send)
    }

    pub fn trigger_completion_menu(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let editor = self.editor.clone();

        cx.spawn_in(window, async move |_, cx| {
            editor
                .update_in(cx, |editor, window, cx| {
                    let menu_is_open =
                        editor.context_menu().borrow().as_ref().is_some_and(|menu| {
                            matches!(menu, CodeContextMenu::Completions(_)) && menu.visible()
                        });

                    let has_at_sign = {
                        let snapshot = editor.display_snapshot(cx);
                        let cursor = editor.selections.newest::<text::Point>(&snapshot).head();
                        let offset = cursor.to_offset(&snapshot);
                        if offset.0 > 0 {
                            snapshot
                                .buffer_snapshot()
                                .reversed_chars_at(offset)
                                .next()
                                .map(|sign| sign == '@')
                                .unwrap_or(false)
                        } else {
                            false
                        }
                    };

                    if menu_is_open && has_at_sign {
                        return;
                    }

                    editor.insert("@", window, cx);
                    editor.show_completions(&editor::actions::ShowCompletions, window, cx);
                })
                .log_err();
        })
        .detach();
    }

    fn chat(&mut self, _: &Chat, _: &mut Window, cx: &mut Context<Self>) {
        self.send(cx);
    }

    fn chat_with_follow(
        &mut self,
        _: &ChatWithFollow,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.workspace
            .update(cx, |this, cx| {
                this.follow(CollaboratorId::Agent, window, cx)
            })
            .log_err();

        self.send(cx);
    }

    fn cancel(&mut self, _: &editor::actions::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(MessageEditorEvent::Cancel)
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if self.prompt_capabilities.borrow().image
            && let Some(task) =
                paste_images_as_context(self.editor.clone(), self.mention_set.clone(), window, cx)
        {
            task.detach();
        }
    }

    pub fn insert_dragged_files(
        &mut self,
        paths: Vec<project::ProjectPath>,
        added_worktrees: Vec<Entity<Worktree>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let path_style = self.project.read(cx).path_style(cx);
        let buffer = self.editor.read(cx).buffer().clone();
        let Some(buffer) = buffer.read(cx).as_singleton() else {
            return;
        };
        let mut tasks = Vec::new();
        for path in paths {
            let Some(entry) = self.project.read(cx).entry_for_path(&path, cx) else {
                continue;
            };
            let Some(worktree) = self.project.read(cx).worktree_for_id(path.worktree_id, cx) else {
                continue;
            };
            let abs_path = worktree.read(cx).absolutize(&path.path);
            let (file_name, _) = crate::completion_provider::extract_file_name_and_directory(
                &path.path,
                worktree.read(cx).root_name(),
                path_style,
            );

            let uri = if entry.is_dir() {
                MentionUri::Directory { abs_path }
            } else {
                MentionUri::File { abs_path }
            };

            let new_text = format!("{} ", uri.as_link());
            let content_len = new_text.len() - 1;

            let anchor = buffer.update(cx, |buffer, _cx| buffer.anchor_before(buffer.len()));

            self.editor.update(cx, |message_editor, cx| {
                message_editor.edit(
                    [(
                        multi_buffer::Anchor::max()..multi_buffer::Anchor::max(),
                        new_text,
                    )],
                    cx,
                );
            });
            let supports_images = self.prompt_capabilities.borrow().image;
            tasks.push(self.mention_set.update(cx, |mention_set, cx| {
                mention_set.confirm_mention_completion(
                    file_name,
                    anchor,
                    content_len,
                    uri,
                    supports_images,
                    self.editor.clone(),
                    &workspace,
                    window,
                    cx,
                )
            }));
        }
        cx.spawn(async move |_, _| {
            join_all(tasks).await;
            drop(added_worktrees);
        })
        .detach();
    }

    pub fn insert_selections(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let editor = self.editor.read(cx);
        let editor_buffer = editor.buffer().read(cx);
        let Some(buffer) = editor_buffer.as_singleton() else {
            return;
        };
        let cursor_anchor = editor.selections.newest_anchor().head();
        let cursor_offset = cursor_anchor.to_offset(&editor_buffer.snapshot(cx));
        let anchor = buffer.update(cx, |buffer, _cx| {
            buffer.anchor_before(cursor_offset.0.min(buffer.len()))
        });
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let Some(completion) =
            PromptCompletionProvider::<Entity<MessageEditor>>::completion_for_action(
                PromptContextAction::AddSelections,
                anchor..anchor,
                self.editor.downgrade(),
                self.mention_set.downgrade(),
                &workspace,
                cx,
            )
        else {
            return;
        };

        self.editor.update(cx, |message_editor, cx| {
            message_editor.edit([(cursor_anchor..cursor_anchor, completion.new_text)], cx);
            message_editor.request_autoscroll(Autoscroll::fit(), cx);
        });
        if let Some(confirm) = completion.confirm {
            confirm(CompletionIntent::Complete, window, cx);
        }
    }

    pub fn set_read_only(&mut self, read_only: bool, cx: &mut Context<Self>) {
        self.editor.update(cx, |message_editor, cx| {
            message_editor.set_read_only(read_only);
            cx.notify()
        })
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

        let path_style = self.project.read(cx).path_style(cx);
        let mut text = String::new();
        let mut mentions = Vec::new();

        for chunk in message {
            match chunk {
                acp::ContentBlock::Text(text_content) => {
                    text.push_str(&text_content.text);
                }
                acp::ContentBlock::Resource(acp::EmbeddedResource {
                    resource: acp::EmbeddedResourceResource::TextResourceContents(resource),
                    ..
                }) => {
                    let Some(mention_uri) = MentionUri::parse(&resource.uri, path_style).log_err()
                    else {
                        continue;
                    };
                    let start = text.len();
                    write!(&mut text, "{}", mention_uri.as_link()).ok();
                    let end = text.len();
                    mentions.push((
                        start..end,
                        mention_uri,
                        Mention::Text {
                            content: resource.text,
                            tracked_buffers: Vec::new(),
                        },
                    ));
                }
                acp::ContentBlock::ResourceLink(resource) => {
                    if let Some(mention_uri) =
                        MentionUri::parse(&resource.uri, path_style).log_err()
                    {
                        let start = text.len();
                        write!(&mut text, "{}", mention_uri.as_link()).ok();
                        let end = text.len();
                        mentions.push((start..end, mention_uri, Mention::Link));
                    }
                }
                acp::ContentBlock::Image(acp::ImageContent {
                    uri,
                    data,
                    mime_type,
                    ..
                }) => {
                    let mention_uri = if let Some(uri) = uri {
                        MentionUri::parse(&uri, path_style)
                    } else {
                        Ok(MentionUri::PastedImage)
                    };
                    let Some(mention_uri) = mention_uri.log_err() else {
                        continue;
                    };
                    let Some(format) = ImageFormat::from_mime_type(&mime_type) else {
                        log::error!("failed to parse MIME type for image: {mime_type:?}");
                        continue;
                    };
                    let start = text.len();
                    write!(&mut text, "{}", mention_uri.as_link()).ok();
                    let end = text.len();
                    mentions.push((
                        start..end,
                        mention_uri,
                        Mention::Image(MentionImage {
                            data: data.into(),
                            format,
                        }),
                    ));
                }
                _ => {}
            }
        }

        let snapshot = self.editor.update(cx, |editor, cx| {
            editor.set_text(text, window, cx);
            editor.buffer().read(cx).snapshot(cx)
        });

        for (range, mention_uri, mention) in mentions {
            let anchor = snapshot.anchor_before(MultiBufferOffset(range.start));
            let Some((crease_id, tx)) = insert_crease_for_mention(
                anchor.excerpt_id,
                anchor.text_anchor,
                range.end - range.start,
                mention_uri.name().into(),
                mention_uri.icon_path(cx),
                None,
                self.editor.clone(),
                window,
                cx,
            ) else {
                continue;
            };
            drop(tx);

            self.mention_set.update(cx, |mention_set, _cx| {
                mention_set.insert_mention(
                    crease_id,
                    mention_uri.clone(),
                    Task::ready(Ok(mention)).shared(),
                )
            });
        }
        cx.notify();
    }

    pub fn text(&self, cx: &App) -> String {
        self.editor.read(cx).text(cx)
    }

    pub fn set_placeholder_text(
        &mut self,
        placeholder: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.set_placeholder_text(placeholder, window, cx);
        });
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
            .on_action(cx.listener(Self::chat_with_follow))
            .on_action(cx.listener(Self::cancel))
            .capture_action(cx.listener(Self::paste))
            .flex_1()
            .child({
                let settings = ThemeSettings::get_global(cx);

                let text_style = TextStyle {
                    color: cx.theme().colors().text,
                    font_family: settings.buffer_font.family.clone(),
                    font_fallbacks: settings.buffer_font.fallbacks.clone(),
                    font_features: settings.buffer_font.features.clone(),
                    font_size: settings.agent_buffer_font_size(cx).into(),
                    line_height: relative(settings.buffer_line_height.value()),
                    ..Default::default()
                };

                EditorElement::new(
                    &self.editor,
                    EditorStyle {
                        background: cx.theme().colors().editor_background,
                        local_player: cx.theme().players().local(),
                        text: text_style,
                        syntax: cx.theme().syntax().clone(),
                        inlay_hints_style: editor::make_inlay_hints_style(cx),
                        ..Default::default()
                    },
                )
            })
    }
}

pub struct MessageEditorAddon {}

impl MessageEditorAddon {
    pub fn new() -> Self {
        Self {}
    }
}

impl Addon for MessageEditorAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }

    fn extend_key_context(&self, key_context: &mut KeyContext, cx: &App) {
        let settings = agent_settings::AgentSettings::get_global(cx);
        if settings.use_modifier_to_send {
            key_context.add("use_modifier_to_send");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, ops::Range, path::Path, rc::Rc, sync::Arc};

    use acp_thread::MentionUri;
    use agent::{HistoryStore, outline};
    use agent_client_protocol as acp;
    use assistant_text_thread::TextThreadStore;
    use editor::{AnchorRangeExt as _, Editor, EditorMode, MultiBufferOffset};
    use fs::FakeFs;
    use futures::StreamExt as _;
    use gpui::{
        AppContext, Entity, EventEmitter, FocusHandle, Focusable, TestAppContext, VisualTestContext,
    };
    use language_model::LanguageModelRegistry;
    use lsp::{CompletionContext, CompletionTriggerKind};
    use project::{CompletionIntent, Project, ProjectPath};
    use serde_json::json;
    use text::Point;
    use ui::{App, Context, IntoElement, Render, SharedString, Window};
    use util::{path, paths::PathStyle, rel_path::rel_path};
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

        let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(text_thread_store, cx));

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                MessageEditor::new(
                    workspace.downgrade(),
                    project.clone(),
                    history_store.clone(),
                    None,
                    Default::default(),
                    Default::default(),
                    "Test Agent".into(),
                    "Test",
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
            let range = snapshot
                .anchor_range_in_excerpt(excerpt_id, completion.replace_range)
                .unwrap();
            editor.edit([(range, completion.new_text)], cx);
            (completion.confirm.unwrap())(CompletionIntent::Complete, window, cx);
        });

        cx.run_until_parked();

        // Backspace over the inserted crease (and the following space).
        editor.update_in(cx, |editor, window, cx| {
            editor.backspace(&Default::default(), window, cx);
            editor.backspace(&Default::default(), window, cx);
        });

        let (content, _) = message_editor
            .update(cx, |message_editor, cx| message_editor.contents(false, cx))
            .await
            .unwrap();

        // We don't send a resource link for the deleted crease.
        pretty_assertions::assert_matches!(content.as_slice(), [acp::ContentBlock::Text { .. }]);
    }

    #[gpui::test]
    async fn test_slash_command_validation(cx: &mut gpui::TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/test",
            json!({
                ".zed": {
                    "tasks.json": r#"[{"label": "test", "command": "echo"}]"#
                },
                "src": {
                    "main.rs": "fn main() {}",
                },
            }),
        )
        .await;

        let project = Project::test(fs.clone(), ["/test".as_ref()], cx).await;
        let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(text_thread_store, cx));
        let prompt_capabilities = Rc::new(RefCell::new(acp::PromptCapabilities::default()));
        // Start with no available commands - simulating Claude which doesn't support slash commands
        let available_commands = Rc::new(RefCell::new(vec![]));

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace_handle = workspace.downgrade();
        let message_editor = workspace.update_in(cx, |_, window, cx| {
            cx.new(|cx| {
                MessageEditor::new(
                    workspace_handle.clone(),
                    project.clone(),
                    history_store.clone(),
                    None,
                    prompt_capabilities.clone(),
                    available_commands.clone(),
                    "Claude Code".into(),
                    "Test",
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

        // Test that slash commands fail when no available_commands are set (empty list means no commands supported)
        editor.update_in(cx, |editor, window, cx| {
            editor.set_text("/file test.txt", window, cx);
        });

        let contents_result = message_editor
            .update(cx, |message_editor, cx| message_editor.contents(false, cx))
            .await;

        // Should fail because available_commands is empty (no commands supported)
        assert!(contents_result.is_err());
        let error_message = contents_result.unwrap_err().to_string();
        assert!(error_message.contains("not supported by Claude Code"));
        assert!(error_message.contains("Available commands: none"));

        // Now simulate Claude providing its list of available commands (which doesn't include file)
        available_commands.replace(vec![acp::AvailableCommand::new("help", "Get help")]);

        // Test that unsupported slash commands trigger an error when we have a list of available commands
        editor.update_in(cx, |editor, window, cx| {
            editor.set_text("/file test.txt", window, cx);
        });

        let contents_result = message_editor
            .update(cx, |message_editor, cx| message_editor.contents(false, cx))
            .await;

        assert!(contents_result.is_err());
        let error_message = contents_result.unwrap_err().to_string();
        assert!(error_message.contains("not supported by Claude Code"));
        assert!(error_message.contains("/file"));
        assert!(error_message.contains("Available commands: /help"));

        // Test that supported commands work fine
        editor.update_in(cx, |editor, window, cx| {
            editor.set_text("/help", window, cx);
        });

        let contents_result = message_editor
            .update(cx, |message_editor, cx| message_editor.contents(false, cx))
            .await;

        // Should succeed because /help is in available_commands
        assert!(contents_result.is_ok());

        // Test that regular text works fine
        editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello Claude!", window, cx);
        });

        let (content, _) = message_editor
            .update(cx, |message_editor, cx| message_editor.contents(false, cx))
            .await
            .unwrap();

        assert_eq!(content.len(), 1);
        if let acp::ContentBlock::Text(text) = &content[0] {
            assert_eq!(text.text, "Hello Claude!");
        } else {
            panic!("Expected ContentBlock::Text");
        }

        // Test that @ mentions still work
        editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Check this @", window, cx);
        });

        // The @ mention functionality should not be affected
        let (content, _) = message_editor
            .update(cx, |message_editor, cx| message_editor.contents(false, cx))
            .await
            .unwrap();

        assert_eq!(content.len(), 1);
        if let acp::ContentBlock::Text(text) = &content[0] {
            assert_eq!(text.text, "Check this @");
        } else {
            panic!("Expected ContentBlock::Text");
        }
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
            self.0.read(cx).focus_handle(cx)
        }
    }

    impl Render for MessageEditorItem {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.0.clone().into_any_element()
        }
    }

    #[gpui::test]
    async fn test_completion_provider_commands(cx: &mut TestAppContext) {
        init_test(cx);

        let app_state = cx.update(AppState::test);

        cx.update(|cx| {
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
        });

        let project = Project::test(app_state.fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = window.root(cx).unwrap();

        let mut cx = VisualTestContext::from_window(*window, cx);

        let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(text_thread_store, cx));
        let prompt_capabilities = Rc::new(RefCell::new(acp::PromptCapabilities::default()));
        let available_commands = Rc::new(RefCell::new(vec![
            acp::AvailableCommand::new("quick-math", "2 + 2 = 4 - 1 = 3"),
            acp::AvailableCommand::new("say-hello", "Say hello to whoever you want").input(
                acp::AvailableCommandInput::Unstructured(acp::UnstructuredCommandInput::new(
                    "<name>",
                )),
            ),
        ]));

        let editor = workspace.update_in(&mut cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            let message_editor = cx.new(|cx| {
                MessageEditor::new(
                    workspace_handle,
                    project.clone(),
                    history_store.clone(),
                    None,
                    prompt_capabilities.clone(),
                    available_commands.clone(),
                    "Test Agent".into(),
                    "Test",
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
            message_editor.read(cx).editor().clone()
        });

        cx.simulate_input("/");

        editor.update_in(&mut cx, |editor, window, cx| {
            assert_eq!(editor.text(cx), "/");
            assert!(editor.has_visible_completions_menu());

            assert_eq!(
                current_completion_labels_with_documentation(editor),
                &[
                    ("quick-math".into(), "2 + 2 = 4 - 1 = 3".into()),
                    ("say-hello".into(), "Say hello to whoever you want".into())
                ]
            );
            editor.set_text("", window, cx);
        });

        cx.simulate_input("/qui");

        editor.update_in(&mut cx, |editor, window, cx| {
            assert_eq!(editor.text(cx), "/qui");
            assert!(editor.has_visible_completions_menu());

            assert_eq!(
                current_completion_labels_with_documentation(editor),
                &[("quick-math".into(), "2 + 2 = 4 - 1 = 3".into())]
            );
            editor.set_text("", window, cx);
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            assert!(editor.has_visible_completions_menu());
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        cx.run_until_parked();

        editor.update_in(&mut cx, |editor, window, cx| {
            assert_eq!(editor.display_text(cx), "/quick-math ");
            assert!(!editor.has_visible_completions_menu());
            editor.set_text("", window, cx);
        });

        cx.simulate_input("/say");

        editor.update_in(&mut cx, |editor, _window, cx| {
            assert_eq!(editor.display_text(cx), "/say");
            assert!(editor.has_visible_completions_menu());

            assert_eq!(
                current_completion_labels_with_documentation(editor),
                &[("say-hello".into(), "Say hello to whoever you want".into())]
            );
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            assert!(editor.has_visible_completions_menu());
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        cx.run_until_parked();

        editor.update_in(&mut cx, |editor, _window, cx| {
            assert_eq!(editor.text(cx), "/say-hello ");
            assert_eq!(editor.display_text(cx), "/say-hello <name>");
            assert!(!editor.has_visible_completions_menu());
        });

        cx.simulate_input("GPT5");

        cx.run_until_parked();

        editor.update_in(&mut cx, |editor, window, cx| {
            assert_eq!(editor.text(cx), "/say-hello GPT5");
            assert_eq!(editor.display_text(cx), "/say-hello GPT5");
            assert!(!editor.has_visible_completions_menu());

            // Delete argument
            for _ in 0..5 {
                editor.backspace(&editor::actions::Backspace, window, cx);
            }
        });

        cx.run_until_parked();

        editor.update_in(&mut cx, |editor, window, cx| {
            assert_eq!(editor.text(cx), "/say-hello");
            // Hint is visible because argument was deleted
            assert_eq!(editor.display_text(cx), "/say-hello <name>");

            // Delete last command letter
            editor.backspace(&editor::actions::Backspace, window, cx);
        });

        cx.run_until_parked();

        editor.update_in(&mut cx, |editor, _window, cx| {
            // Hint goes away once command no longer matches an available one
            assert_eq!(editor.text(cx), "/say-hell");
            assert_eq!(editor.display_text(cx), "/say-hell");
            assert!(!editor.has_visible_completions_menu());
        });
    }

    #[gpui::test]
    async fn test_context_completion_provider_mentions(cx: &mut TestAppContext) {
        init_test(cx);

        let app_state = cx.update(AppState::test);

        cx.update(|cx| {
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
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
                    },
                    "x.png": "",
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
            rel_path("a/one.txt"),
            rel_path("a/two.txt"),
            rel_path("a/three.txt"),
            rel_path("a/four.txt"),
            rel_path("b/five.txt"),
            rel_path("b/six.txt"),
            rel_path("b/seven.txt"),
            rel_path("b/eight.txt"),
        ];

        let slash = PathStyle::local().primary_separator();

        let mut opened_editors = Vec::new();
        for path in paths {
            let buffer = workspace
                .update_in(&mut cx, |workspace, window, cx| {
                    workspace.open_path(
                        ProjectPath {
                            worktree_id,
                            path: path.into(),
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

        let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(text_thread_store, cx));
        let prompt_capabilities = Rc::new(RefCell::new(acp::PromptCapabilities::default()));

        let (message_editor, editor) = workspace.update_in(&mut cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            let message_editor = cx.new(|cx| {
                MessageEditor::new(
                    workspace_handle,
                    project.clone(),
                    history_store.clone(),
                    None,
                    prompt_capabilities.clone(),
                    Default::default(),
                    "Test Agent".into(),
                    "Test",
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

        cx.simulate_input("Lorem @");

        editor.update_in(&mut cx, |editor, window, cx| {
            assert_eq!(editor.text(cx), "Lorem @");
            assert!(editor.has_visible_completions_menu());

            assert_eq!(
                current_completion_labels(editor),
                &[
                    format!("eight.txt b{slash}"),
                    format!("seven.txt b{slash}"),
                    format!("six.txt b{slash}"),
                    format!("five.txt b{slash}"),
                    "Files & Directories".into(),
                    "Symbols".into()
                ]
            );
            editor.set_text("", window, cx);
        });

        prompt_capabilities.replace(
            acp::PromptCapabilities::new()
                .image(true)
                .audio(true)
                .embedded_context(true),
        );

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
                    format!("eight.txt b{slash}"),
                    format!("seven.txt b{slash}"),
                    format!("six.txt b{slash}"),
                    format!("five.txt b{slash}"),
                    "Files & Directories".into(),
                    "Symbols".into(),
                    "Threads".into(),
                    "Fetch".into()
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
            assert_eq!(
                current_completion_labels(editor),
                vec![format!("one.txt a{slash}")]
            );
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            assert!(editor.has_visible_completions_menu());
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        let url_one = MentionUri::File {
            abs_path: path!("/dir/a/one.txt").into(),
        }
        .to_uri()
        .to_string();
        editor.update(&mut cx, |editor, cx| {
            let text = editor.text(cx);
            assert_eq!(text, format!("Lorem [@one.txt]({url_one}) "));
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(fold_ranges(editor, cx).len(), 1);
        });

        let contents = message_editor
            .update(&mut cx, |message_editor, cx| {
                message_editor
                    .mention_set()
                    .update(cx, |mention_set, cx| mention_set.contents(false, cx))
            })
            .await
            .unwrap()
            .into_values()
            .collect::<Vec<_>>();

        {
            let [(uri, Mention::Text { content, .. })] = contents.as_slice() else {
                panic!("Unexpected mentions");
            };
            pretty_assertions::assert_eq!(content, "1");
            pretty_assertions::assert_eq!(
                uri,
                &MentionUri::parse(&url_one, PathStyle::local()).unwrap()
            );
        }

        cx.simulate_input(" ");

        editor.update(&mut cx, |editor, cx| {
            let text = editor.text(cx);
            assert_eq!(text, format!("Lorem [@one.txt]({url_one})  "));
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(fold_ranges(editor, cx).len(), 1);
        });

        cx.simulate_input("Ipsum ");

        editor.update(&mut cx, |editor, cx| {
            let text = editor.text(cx);
            assert_eq!(text, format!("Lorem [@one.txt]({url_one})  Ipsum "),);
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(fold_ranges(editor, cx).len(), 1);
        });

        cx.simulate_input("@file ");

        editor.update(&mut cx, |editor, cx| {
            let text = editor.text(cx);
            assert_eq!(text, format!("Lorem [@one.txt]({url_one})  Ipsum @file "),);
            assert!(editor.has_visible_completions_menu());
            assert_eq!(fold_ranges(editor, cx).len(), 1);
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        cx.run_until_parked();

        let contents = message_editor
            .update(&mut cx, |message_editor, cx| {
                message_editor
                    .mention_set()
                    .update(cx, |mention_set, cx| mention_set.contents(false, cx))
            })
            .await
            .unwrap()
            .into_values()
            .collect::<Vec<_>>();

        let url_eight = MentionUri::File {
            abs_path: path!("/dir/b/eight.txt").into(),
        }
        .to_uri()
        .to_string();

        {
            let [_, (uri, Mention::Text { content, .. })] = contents.as_slice() else {
                panic!("Unexpected mentions");
            };
            pretty_assertions::assert_eq!(content, "8");
            pretty_assertions::assert_eq!(
                uri,
                &MentionUri::parse(&url_eight, PathStyle::local()).unwrap()
            );
        }

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                format!("Lorem [@one.txt]({url_one})  Ipsum [@eight.txt]({url_eight}) ")
            );
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(fold_ranges(editor, cx).len(), 2);
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
            move |_, _| async move {
                Ok(Some(lsp::WorkspaceSymbolResponse::Flat(vec![
                    #[allow(deprecated)]
                    lsp::SymbolInformation {
                        name: "MySymbol".into(),
                        location: lsp::Location {
                            uri: lsp::Uri::from_file_path(path!("/dir/a/one.txt")).unwrap(),
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
                format!("Lorem [@one.txt]({url_one})  Ipsum [@eight.txt]({url_eight}) @symbol ")
            );
            assert!(editor.has_visible_completions_menu());
            assert_eq!(current_completion_labels(editor), &["MySymbol one.txt L1"]);
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        let symbol = MentionUri::Symbol {
            abs_path: path!("/dir/a/one.txt").into(),
            name: "MySymbol".into(),
            line_range: 0..=0,
        };

        let contents = message_editor
            .update(&mut cx, |message_editor, cx| {
                message_editor
                    .mention_set()
                    .update(cx, |mention_set, cx| mention_set.contents(false, cx))
            })
            .await
            .unwrap()
            .into_values()
            .collect::<Vec<_>>();

        {
            let [_, _, (uri, Mention::Text { content, .. })] = contents.as_slice() else {
                panic!("Unexpected mentions");
            };
            pretty_assertions::assert_eq!(content, "1");
            pretty_assertions::assert_eq!(uri, &symbol);
        }

        cx.run_until_parked();

        editor.read_with(&cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                format!(
                    "Lorem [@one.txt]({url_one})  Ipsum [@eight.txt]({url_eight}) [@MySymbol]({}) ",
                    symbol.to_uri(),
                )
            );
        });

        // Try to mention an "image" file that will fail to load
        cx.simulate_input("@file x.png");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                format!("Lorem [@one.txt]({url_one})  Ipsum [@eight.txt]({url_eight}) [@MySymbol]({}) @file x.png", symbol.to_uri())
            );
            assert!(editor.has_visible_completions_menu());
            assert_eq!(current_completion_labels(editor), &["x.png "]);
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        // Getting the message contents fails
        message_editor
            .update(&mut cx, |message_editor, cx| {
                message_editor
                    .mention_set()
                    .update(cx, |mention_set, cx| mention_set.contents(false, cx))
            })
            .await
            .expect_err("Should fail to load x.png");

        cx.run_until_parked();

        // Mention was removed
        editor.read_with(&cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                format!(
                    "Lorem [@one.txt]({url_one})  Ipsum [@eight.txt]({url_eight}) [@MySymbol]({}) ",
                    symbol.to_uri()
                )
            );
        });

        // Once more
        cx.simulate_input("@file x.png");

        editor.update(&mut cx, |editor, cx| {
                    assert_eq!(
                        editor.text(cx),
                        format!("Lorem [@one.txt]({url_one})  Ipsum [@eight.txt]({url_eight}) [@MySymbol]({}) @file x.png", symbol.to_uri())
                    );
                    assert!(editor.has_visible_completions_menu());
                    assert_eq!(current_completion_labels(editor), &["x.png "]);
                });

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        // This time don't immediately get the contents, just let the confirmed completion settle
        cx.run_until_parked();

        // Mention was removed
        editor.read_with(&cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                format!(
                    "Lorem [@one.txt]({url_one})  Ipsum [@eight.txt]({url_eight}) [@MySymbol]({}) ",
                    symbol.to_uri()
                )
            );
        });

        // Now getting the contents succeeds, because the invalid mention was removed
        let contents = message_editor
            .update(&mut cx, |message_editor, cx| {
                message_editor
                    .mention_set()
                    .update(cx, |mention_set, cx| mention_set.contents(false, cx))
            })
            .await
            .unwrap();
        assert_eq!(contents.len(), 3);
    }

    fn fold_ranges(editor: &Editor, cx: &mut App) -> Vec<Range<Point>> {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        editor.display_map.update(cx, |display_map, cx| {
            display_map
                .snapshot(cx)
                .folds_in_range(MultiBufferOffset(0)..snapshot.len())
                .map(|fold| fold.range.to_point(&snapshot))
                .collect()
        })
    }

    fn current_completion_labels(editor: &Editor) -> Vec<String> {
        let completions = editor.current_completions().expect("Missing completions");
        completions
            .into_iter()
            .map(|completion| completion.label.text)
            .collect::<Vec<_>>()
    }

    fn current_completion_labels_with_documentation(editor: &Editor) -> Vec<(String, String)> {
        let completions = editor.current_completions().expect("Missing completions");
        completions
            .into_iter()
            .map(|completion| {
                (
                    completion.label.text,
                    completion
                        .documentation
                        .map(|d| d.text().to_string())
                        .unwrap_or_default(),
                )
            })
            .collect::<Vec<_>>()
    }

    #[gpui::test]
    async fn test_large_file_mention_fallback(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());

        // Create a large file that exceeds AUTO_OUTLINE_SIZE
        // Using plain text without a configured language, so no outline is available
        const LINE: &str = "This is a line of text in the file\n";
        let large_content = LINE.repeat(2 * (outline::AUTO_OUTLINE_SIZE / LINE.len()));
        assert!(large_content.len() > outline::AUTO_OUTLINE_SIZE);

        // Create a small file that doesn't exceed AUTO_OUTLINE_SIZE
        let small_content = "fn small_function() { /* small */ }\n";
        assert!(small_content.len() < outline::AUTO_OUTLINE_SIZE);

        fs.insert_tree(
            "/project",
            json!({
                "large_file.txt": large_content.clone(),
                "small_file.txt": small_content,
            }),
        )
        .await;

        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(text_thread_store, cx));

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                let editor = MessageEditor::new(
                    workspace.downgrade(),
                    project.clone(),
                    history_store.clone(),
                    None,
                    Default::default(),
                    Default::default(),
                    "Test Agent".into(),
                    "Test",
                    EditorMode::AutoHeight {
                        min_lines: 1,
                        max_lines: None,
                    },
                    window,
                    cx,
                );
                // Enable embedded context so files are actually included
                editor
                    .prompt_capabilities
                    .replace(acp::PromptCapabilities::new().embedded_context(true));
                editor
            })
        });

        // Test large file mention
        // Get the absolute path using the project's worktree
        let large_file_abs_path = project.read_with(cx, |project, cx| {
            let worktree = project.worktrees(cx).next().unwrap();
            let worktree_root = worktree.read(cx).abs_path();
            worktree_root.join("large_file.txt")
        });
        let large_file_task = message_editor.update(cx, |editor, cx| {
            editor.mention_set().update(cx, |set, cx| {
                set.confirm_mention_for_file(large_file_abs_path, true, cx)
            })
        });

        let large_file_mention = large_file_task.await.unwrap();
        match large_file_mention {
            Mention::Text { content, .. } => {
                // Should contain some of the content but not all of it
                assert!(
                    content.contains(LINE),
                    "Should contain some of the file content"
                );
                assert!(
                    !content.contains(&LINE.repeat(100)),
                    "Should not contain the full file"
                );
                // Should be much smaller than original
                assert!(
                    content.len() < large_content.len() / 10,
                    "Should be significantly truncated"
                );
            }
            _ => panic!("Expected Text mention for large file"),
        }

        // Test small file mention
        // Get the absolute path using the project's worktree
        let small_file_abs_path = project.read_with(cx, |project, cx| {
            let worktree = project.worktrees(cx).next().unwrap();
            let worktree_root = worktree.read(cx).abs_path();
            worktree_root.join("small_file.txt")
        });
        let small_file_task = message_editor.update(cx, |editor, cx| {
            editor.mention_set().update(cx, |set, cx| {
                set.confirm_mention_for_file(small_file_abs_path, true, cx)
            })
        });

        let small_file_mention = small_file_task.await.unwrap();
        match small_file_mention {
            Mention::Text { content, .. } => {
                // Should contain the full actual content
                assert_eq!(content, small_content);
            }
            _ => panic!("Expected Text mention for small file"),
        }
    }

    #[gpui::test]
    async fn test_insert_thread_summary(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(LanguageModelRegistry::test);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({"file": ""})).await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(text_thread_store, cx));

        // Create a thread metadata to insert as summary
        let thread_metadata = agent::DbThreadMetadata {
            id: acp::SessionId::new("thread-123"),
            title: "Previous Conversation".into(),
            updated_at: chrono::Utc::now(),
        };

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                let mut editor = MessageEditor::new(
                    workspace.downgrade(),
                    project.clone(),
                    history_store.clone(),
                    None,
                    Default::default(),
                    Default::default(),
                    "Test Agent".into(),
                    "Test",
                    EditorMode::AutoHeight {
                        min_lines: 1,
                        max_lines: None,
                    },
                    window,
                    cx,
                );
                editor.insert_thread_summary(thread_metadata.clone(), window, cx);
                editor
            })
        });

        // Construct expected values for verification
        let expected_uri = MentionUri::Thread {
            id: thread_metadata.id.clone(),
            name: thread_metadata.title.to_string(),
        };
        let expected_link = format!("[@{}]({})", thread_metadata.title, expected_uri.to_uri());

        message_editor.read_with(cx, |editor, cx| {
            let text = editor.text(cx);

            assert!(
                text.contains(&expected_link),
                "Expected editor text to contain thread mention link.\nExpected substring: {}\nActual text: {}",
                expected_link,
                text
            );

            let mentions = editor.mention_set().read(cx).mentions();
            assert_eq!(
                mentions.len(),
                1,
                "Expected exactly one mention after inserting thread summary"
            );

            assert!(
                mentions.contains(&expected_uri),
                "Expected mentions to contain the thread URI"
            );
        });
    }

    #[gpui::test]
    async fn test_whitespace_trimming(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({"file.rs": "fn main() {}"}))
            .await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(text_thread_store, cx));

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                MessageEditor::new(
                    workspace.downgrade(),
                    project.clone(),
                    history_store.clone(),
                    None,
                    Default::default(),
                    Default::default(),
                    "Test Agent".into(),
                    "Test",
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

        editor.update_in(cx, |editor, window, cx| {
            editor.set_text("  \u{A0}してhello world  ", window, cx);
        });

        let (content, _) = message_editor
            .update(cx, |message_editor, cx| message_editor.contents(false, cx))
            .await
            .unwrap();

        assert_eq!(content, vec!["してhello world".into()]);
    }

    #[gpui::test]
    async fn test_editor_respects_embedded_context_capability(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());

        let file_content = "fn main() { println!(\"Hello, world!\"); }\n";

        fs.insert_tree(
            "/project",
            json!({
                "src": {
                    "main.rs": file_content,
                }
            }),
        )
        .await;

        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(text_thread_store, cx));

        let (message_editor, editor) = workspace.update_in(cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            let message_editor = cx.new(|cx| {
                MessageEditor::new(
                    workspace_handle,
                    project.clone(),
                    history_store.clone(),
                    None,
                    Default::default(),
                    Default::default(),
                    "Test Agent".into(),
                    "Test",
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

        cx.simulate_input("What is in @file main");

        editor.update_in(cx, |editor, window, cx| {
            assert!(editor.has_visible_completions_menu());
            assert_eq!(editor.text(cx), "What is in @file main");
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        let content = message_editor
            .update(cx, |editor, cx| editor.contents(false, cx))
            .await
            .unwrap()
            .0;

        let main_rs_uri = if cfg!(windows) {
            "file:///C:/project/src/main.rs"
        } else {
            "file:///project/src/main.rs"
        };

        // When embedded context is `false` we should get a resource link
        pretty_assertions::assert_eq!(
            content,
            vec![
                "What is in ".into(),
                acp::ContentBlock::ResourceLink(acp::ResourceLink::new("main.rs", main_rs_uri))
            ]
        );

        message_editor.update(cx, |editor, _cx| {
            editor
                .prompt_capabilities
                .replace(acp::PromptCapabilities::new().embedded_context(true))
        });

        let content = message_editor
            .update(cx, |editor, cx| editor.contents(false, cx))
            .await
            .unwrap()
            .0;

        // When embedded context is `true` we should get a resource
        pretty_assertions::assert_eq!(
            content,
            vec![
                "What is in ".into(),
                acp::ContentBlock::Resource(acp::EmbeddedResource::new(
                    acp::EmbeddedResourceResource::TextResourceContents(
                        acp::TextResourceContents::new(file_content, main_rs_uri)
                    )
                ))
            ]
        );
    }

    #[gpui::test]
    async fn test_autoscroll_after_insert_selections(cx: &mut TestAppContext) {
        init_test(cx);

        let app_state = cx.update(AppState::test);

        cx.update(|cx| {
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
        });

        app_state
            .fs
            .as_fake()
            .insert_tree(
                path!("/dir"),
                json!({
                    "test.txt": "line1\nline2\nline3\nline4\nline5\n",
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

        // Open a regular editor with the created file, and select a portion of
        // the text that will be used for the selections that are meant to be
        // inserted in the agent panel.
        let editor = workspace
            .update_in(&mut cx, |workspace, window, cx| {
                workspace.open_path(
                    ProjectPath {
                        worktree_id,
                        path: rel_path("test.txt").into(),
                    },
                    None,
                    false,
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.change_selections(Default::default(), window, cx, |selections| {
                selections.select_ranges([Point::new(0, 0)..Point::new(0, 5)]);
            });
        });

        let text_thread_store = cx.new(|cx| TextThreadStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(text_thread_store, cx));

        // Create a new `MessageEditor`. The `EditorMode::full()` has to be used
        // to ensure we have a fixed viewport, so we can eventually actually
        // place the cursor outside of the visible area.
        let message_editor = workspace.update_in(&mut cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            let message_editor = cx.new(|cx| {
                MessageEditor::new(
                    workspace_handle,
                    project.clone(),
                    history_store.clone(),
                    None,
                    Default::default(),
                    Default::default(),
                    "Test Agent".into(),
                    "Test",
                    EditorMode::full(),
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

            message_editor
        });

        message_editor.update_in(&mut cx, |message_editor, window, cx| {
            message_editor.editor.update(cx, |editor, cx| {
                // Update the Agent Panel's Message Editor text to have 100
                // lines, ensuring that the cursor is set at line 90 and that we
                // then scroll all the way to the top, so the cursor's position
                // remains off screen.
                let mut lines = String::new();
                for _ in 1..=100 {
                    lines.push_str(&"Another line in the agent panel's message editor\n");
                }
                editor.set_text(lines.as_str(), window, cx);
                editor.change_selections(Default::default(), window, cx, |selections| {
                    selections.select_ranges([Point::new(90, 0)..Point::new(90, 0)]);
                });
                editor.set_scroll_position(gpui::Point::new(0., 0.), window, cx);
            });
        });

        cx.run_until_parked();

        // Before proceeding, let's assert that the cursor is indeed off screen,
        // otherwise the rest of the test doesn't make sense.
        message_editor.update_in(&mut cx, |message_editor, window, cx| {
            message_editor.editor.update(cx, |editor, cx| {
                let snapshot = editor.snapshot(window, cx);
                let cursor_row = editor.selections.newest::<Point>(&snapshot).head().row;
                let scroll_top = snapshot.scroll_position().y as u32;
                let visible_lines = editor.visible_line_count().unwrap() as u32;
                let visible_range = scroll_top..(scroll_top + visible_lines);

                assert!(!visible_range.contains(&cursor_row));
            })
        });

        // Now let's insert the selection in the Agent Panel's editor and
        // confirm that, after the insertion, the cursor is now in the visible
        // range.
        message_editor.update_in(&mut cx, |message_editor, window, cx| {
            message_editor.insert_selections(window, cx);
        });

        cx.run_until_parked();

        message_editor.update_in(&mut cx, |message_editor, window, cx| {
            message_editor.editor.update(cx, |editor, cx| {
                let snapshot = editor.snapshot(window, cx);
                let cursor_row = editor.selections.newest::<Point>(&snapshot).head().row;
                let scroll_top = snapshot.scroll_position().y as u32;
                let visible_lines = editor.visible_line_count().unwrap() as u32;
                let visible_range = scroll_top..(scroll_top + visible_lines);

                assert!(visible_range.contains(&cursor_row));
            })
        });
    }
}
