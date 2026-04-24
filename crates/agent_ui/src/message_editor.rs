use crate::DEFAULT_THREAD_TITLE;
use crate::SendImmediately;
use crate::{
    ChatWithFollow,
    completion_provider::{
        PromptCompletionProvider, PromptCompletionProviderDelegate, PromptContextAction,
        PromptContextType, SlashCommandCompletion,
    },
    mention_set::{Mention, MentionImage, MentionSet, insert_crease_for_mention},
};
use acp_thread::MentionUri;
use agent::ThreadStore;
use agent_client_protocol::schema as acp;
use anyhow::{Result, anyhow};
use editor::{
    Addon, AnchorRangeExt, ContextMenuOptions, Editor, EditorElement, EditorEvent, EditorMode,
    EditorStyle, Inlay, MultiBuffer, MultiBufferOffset, MultiBufferSnapshot, ToOffset,
    actions::{Copy, Paste},
    code_context_menus::CodeContextMenu,
    scroll::Autoscroll,
};
use futures::{FutureExt as _, future::join_all};
use gpui::{
    AppContext, ClipboardEntry, ClipboardItem, Context, Entity, EventEmitter, FocusHandle,
    Focusable, ImageFormat, KeyContext, SharedString, Subscription, Task, TextStyle, WeakEntity,
};
use language::{Buffer, language_settings::InlayHintKind};
use parking_lot::RwLock;
use project::AgentId;
use project::{
    CompletionIntent, InlayHint, InlayHintLabel, InlayId, Project, ProjectPath, Worktree,
};
use prompt_store::PromptStore;
use rope::Point;
use settings::Settings;
use std::{fmt::Write, ops::Range, rc::Rc, sync::Arc};
use theme_settings::ThemeSettings;
use ui::{ContextMenu, Disclosure, ElevationIndex, prelude::*};
use util::paths::PathStyle;
use util::{ResultExt, debug_panic};
use workspace::{CollaboratorId, Workspace};
use zed_actions::agent::{Chat, PasteRaw};

#[derive(Default)]
pub struct SessionCapabilities {
    prompt_capabilities: acp::PromptCapabilities,
    available_commands: Vec<acp::AvailableCommand>,
}

impl SessionCapabilities {
    pub fn new(
        prompt_capabilities: acp::PromptCapabilities,
        available_commands: Vec<acp::AvailableCommand>,
    ) -> Self {
        Self {
            prompt_capabilities,
            available_commands,
        }
    }

    pub fn supports_images(&self) -> bool {
        self.prompt_capabilities.image
    }

    pub fn supports_embedded_context(&self) -> bool {
        self.prompt_capabilities.embedded_context
    }

    pub fn available_commands(&self) -> &[acp::AvailableCommand] {
        &self.available_commands
    }

    fn supported_modes(&self, has_thread_store: bool) -> Vec<PromptContextType> {
        let mut supported = vec![PromptContextType::File, PromptContextType::Symbol];
        if self.prompt_capabilities.embedded_context {
            if has_thread_store {
                supported.push(PromptContextType::Thread);
            }
            supported.extend(&[
                PromptContextType::Diagnostics,
                PromptContextType::Fetch,
                PromptContextType::Rules,
                PromptContextType::BranchDiff,
            ]);
        }
        supported
    }

    pub fn completion_commands(&self) -> Vec<crate::completion_provider::AvailableCommand> {
        self.available_commands
            .iter()
            .map(|cmd| crate::completion_provider::AvailableCommand {
                name: cmd.name.clone().into(),
                description: cmd.description.clone().into(),
                requires_argument: cmd.input.is_some(),
            })
            .collect()
    }

    pub fn set_prompt_capabilities(&mut self, prompt_capabilities: acp::PromptCapabilities) {
        self.prompt_capabilities = prompt_capabilities;
    }

    pub fn set_available_commands(&mut self, available_commands: Vec<acp::AvailableCommand>) {
        self.available_commands = available_commands;
    }
}

pub type SharedSessionCapabilities = Arc<RwLock<SessionCapabilities>>;

struct MessageEditorCompletionDelegate {
    session_capabilities: SharedSessionCapabilities,
    has_thread_store: bool,
    message_editor: WeakEntity<MessageEditor>,
}

impl PromptCompletionProviderDelegate for MessageEditorCompletionDelegate {
    fn supports_images(&self, _cx: &App) -> bool {
        self.session_capabilities.read().supports_images()
    }

    fn supported_modes(&self, _cx: &App) -> Vec<PromptContextType> {
        self.session_capabilities
            .read()
            .supported_modes(self.has_thread_store)
    }

    fn available_commands(&self, _cx: &App) -> Vec<crate::completion_provider::AvailableCommand> {
        self.session_capabilities.read().completion_commands()
    }

    fn confirm_command(&self, cx: &mut App) {
        let _ = self.message_editor.update(cx, |this, cx| this.send(cx));
    }
}

pub struct MessageEditor {
    mention_set: Entity<MentionSet>,
    editor: Entity<Editor>,
    workspace: WeakEntity<Workspace>,
    session_capabilities: SharedSessionCapabilities,
    agent_id: AgentId,
    thread_store: Option<Entity<ThreadStore>>,
    _subscriptions: Vec<Subscription>,
    _parse_slash_command_task: Task<()>,
}

#[derive(Clone, Debug)]
pub enum MessageEditorEvent {
    Send,
    SendImmediately,
    Cancel,
    Focus,
    LostFocus,
    InputAttempted {
        text: Arc<str>,
        cursor_offset: usize,
    },
}

impl EventEmitter<MessageEditorEvent> for MessageEditor {}

const COMMAND_HINT_INLAY_ID: InlayId = InlayId::Hint(0);

enum MentionInsertPosition {
    AtCursor,
    EndOfBuffer,
}

fn insert_mention_for_project_path(
    project_path: &ProjectPath,
    position: MentionInsertPosition,
    editor: &Entity<Editor>,
    mention_set: &Entity<MentionSet>,
    project: &Entity<Project>,
    workspace: &Entity<Workspace>,
    supports_images: bool,
    window: &mut Window,
    cx: &mut App,
) -> Option<Task<()>> {
    let (file_name, mention_uri) = {
        let project = project.read(cx);
        let path_style = project.path_style(cx);
        let entry = project.entry_for_path(project_path, cx)?;
        let worktree = project.worktree_for_id(project_path.worktree_id, cx)?;
        let abs_path = worktree.read(cx).absolutize(&project_path.path);
        let (file_name, _) = crate::completion_provider::extract_file_name_and_directory(
            &project_path.path,
            worktree.read(cx).root_name(),
            path_style,
        );
        let mention_uri = if entry.is_dir() {
            MentionUri::Directory { abs_path }
        } else {
            MentionUri::File { abs_path }
        };
        (file_name, mention_uri)
    };

    let mention_text = mention_uri.as_link().to_string();
    let content_len = mention_text.len();

    let text_anchor = match position {
        MentionInsertPosition::AtCursor => editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx);
            let snapshot = buffer.snapshot(cx);
            let buffer_snapshot = snapshot.as_singleton()?;
            let text_anchor = snapshot
                .anchor_to_buffer_anchor(editor.selections.newest_anchor().start)?
                .0
                .bias_left(&buffer_snapshot);

            editor.insert(&mention_text, window, cx);
            editor.insert(" ", window, cx);

            Some(text_anchor)
        }),
        MentionInsertPosition::EndOfBuffer => {
            let multi_buffer = editor.read(cx).buffer().clone();
            let buffer = multi_buffer.read(cx).as_singleton()?;
            let anchor = buffer.update(cx, |buffer, _cx| buffer.anchor_before(buffer.len()));
            let new_text = format!("{mention_text} ");
            editor.update(cx, |editor, cx| {
                editor.edit(
                    [(
                        multi_buffer::Anchor::Max..multi_buffer::Anchor::Max,
                        new_text,
                    )],
                    cx,
                );
            });
            Some(anchor)
        }
    }?;

    Some(mention_set.update(cx, |mention_set, cx| {
        mention_set.confirm_mention_completion(
            file_name,
            text_anchor,
            content_len,
            mention_uri,
            supports_images,
            editor.clone(),
            workspace,
            window,
            cx,
        )
    }))
}

enum ResolvedPastedContextItem {
    Image(gpui::Image, gpui::SharedString),
    ProjectPath(ProjectPath),
}

async fn resolve_pasted_context_items(
    project: Entity<Project>,
    project_is_local: bool,
    supports_images: bool,
    entries: Vec<ClipboardEntry>,
    cx: &mut gpui::AsyncWindowContext,
) -> (Vec<ResolvedPastedContextItem>, Vec<Entity<Worktree>>) {
    let mut items = Vec::new();
    let mut added_worktrees = Vec::new();
    let default_image_name: SharedString = "Image".into();

    for entry in entries {
        match entry {
            ClipboardEntry::String(_) => {}
            ClipboardEntry::Image(image) => {
                if supports_images {
                    items.push(ResolvedPastedContextItem::Image(
                        image,
                        default_image_name.clone(),
                    ));
                }
            }
            ClipboardEntry::ExternalPaths(paths) => {
                for path in paths.paths().iter() {
                    if let Some((image, name)) = cx
                        .background_spawn({
                            let path = path.clone();
                            let default_image_name = default_image_name.clone();
                            async move {
                                crate::mention_set::load_external_image_from_path(
                                    &path,
                                    &default_image_name,
                                )
                            }
                        })
                        .await
                    {
                        if supports_images {
                            items.push(ResolvedPastedContextItem::Image(image, name));
                        }
                        continue;
                    }

                    if !project_is_local {
                        continue;
                    }

                    let path = path.clone();
                    let Ok(resolve_task) = cx.update({
                        let project = project.clone();
                        move |_, cx| Workspace::project_path_for_path(project, &path, false, cx)
                    }) else {
                        continue;
                    };

                    if let Some((worktree, project_path)) = resolve_task.await.log_err() {
                        added_worktrees.push(worktree);
                        items.push(ResolvedPastedContextItem::ProjectPath(project_path));
                    }
                }
            }
        }
    }

    (items, added_worktrees)
}

fn insert_project_path_as_context(
    project_path: ProjectPath,
    editor: Entity<Editor>,
    mention_set: Entity<MentionSet>,
    workspace: WeakEntity<Workspace>,
    supports_images: bool,
    cx: &mut gpui::AsyncWindowContext,
) -> Option<Task<()>> {
    let workspace = workspace.upgrade()?;

    cx.update(move |window, cx| {
        let project = workspace.read(cx).project().clone();
        insert_mention_for_project_path(
            &project_path,
            MentionInsertPosition::AtCursor,
            &editor,
            &mention_set,
            &project,
            &workspace,
            supports_images,
            window,
            cx,
        )
    })
    .ok()
    .flatten()
}

async fn insert_resolved_pasted_context_items(
    items: Vec<ResolvedPastedContextItem>,
    added_worktrees: Vec<Entity<Worktree>>,
    editor: Entity<Editor>,
    mention_set: Entity<MentionSet>,
    workspace: WeakEntity<Workspace>,
    supports_images: bool,
    cx: &mut gpui::AsyncWindowContext,
) {
    let mut path_mention_tasks = Vec::new();

    for item in items {
        match item {
            ResolvedPastedContextItem::Image(image, name) => {
                crate::mention_set::insert_images_as_context(
                    vec![(image, name)],
                    editor.clone(),
                    mention_set.clone(),
                    workspace.clone(),
                    cx,
                )
                .await;
            }
            ResolvedPastedContextItem::ProjectPath(project_path) => {
                if let Some(task) = insert_project_path_as_context(
                    project_path,
                    editor.clone(),
                    mention_set.clone(),
                    workspace.clone(),
                    supports_images,
                    cx,
                ) {
                    path_mention_tasks.push(task);
                }
            }
        }
    }

    join_all(path_mention_tasks).await;
    drop(added_worktrees);
}

impl MessageEditor {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        project: WeakEntity<Project>,
        thread_store: Option<Entity<ThreadStore>>,
        prompt_store: Option<Entity<PromptStore>>,
        session_capabilities: SharedSessionCapabilities,
        agent_id: AgentId,
        placeholder: &str,
        mode: EditorMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let language_registry = project
            .upgrade()
            .map(|project| project.read(cx).languages().clone());

        let editor = cx.new(|cx| {
            let buffer = cx.new(|cx| {
                let buffer = Buffer::local("", cx);
                if let Some(language_registry) = language_registry.as_ref() {
                    buffer.set_language_registry(language_registry.clone());
                }
                buffer
            });
            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

            let mut editor = Editor::new(mode, buffer, None, window, cx);
            editor.set_placeholder_text(placeholder, window, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_show_completions_on_input(Some(true));
            editor.set_soft_wrap();
            editor.disable_mouse_wheel_zoom();
            editor.set_use_modal_editing(true);
            editor.set_context_menu_options(ContextMenuOptions {
                min_entries_visible: 12,
                max_entries_visible: 12,
                placement: None,
            });
            editor.register_addon(MessageEditorAddon::new());

            editor.set_custom_context_menu(|editor, _point, window, cx| {
                let has_selection = editor.has_non_empty_selection(&editor.display_snapshot(cx));

                Some(ContextMenu::build(window, cx, |menu, _, _| {
                    menu.action("Cut", Box::new(editor::actions::Cut))
                        .action_disabled_when(
                            !has_selection,
                            "Copy",
                            Box::new(editor::actions::Copy),
                        )
                        .action("Paste", Box::new(editor::actions::Paste))
                        .action("Paste as Plain Text", Box::new(PasteRaw))
                }))
            });

            editor
        });
        let mention_set =
            cx.new(|_cx| MentionSet::new(project, thread_store.clone(), prompt_store.clone()));
        let completion_provider = Rc::new(PromptCompletionProvider::new(
            MessageEditorCompletionDelegate {
                session_capabilities: session_capabilities.clone(),
                has_thread_store: thread_store.is_some(),
                message_editor: cx.weak_entity(),
            },
            editor.downgrade(),
            mention_set.clone(),
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
                let input_attempted_text = match event {
                    EditorEvent::InputHandled { text, .. } => Some(text),
                    EditorEvent::InputIgnored { text } => Some(text),
                    _ => None,
                };
                if let Some(text) = input_attempted_text
                    && editor.read(cx).read_only(cx)
                    && !text.is_empty()
                {
                    let editor = editor.read(cx);
                    let cursor_anchor = editor.selections.newest_anchor().head();
                    let cursor_offset = cursor_anchor
                        .to_offset(&editor.buffer().read(cx).snapshot(cx))
                        .0;
                    cx.emit(MessageEditorEvent::InputAttempted {
                        text: text.clone(),
                        cursor_offset,
                    });
                }

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

        if let Some(language_registry) = language_registry {
            let editor = editor.clone();
            cx.spawn(async move |_, cx| {
                let markdown = language_registry.language_for_name("Markdown").await?;
                editor.update(cx, |editor, cx| {
                    if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
                        buffer.update(cx, |buffer, cx| {
                            buffer.set_language(Some(markdown), cx);
                        });
                    }
                });
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }

        Self {
            editor,
            mention_set,
            workspace,
            session_capabilities,
            agent_id,
            thread_store,
            _subscriptions: subscriptions,
            _parse_slash_command_task: Task::ready(()),
        }
    }

    pub fn set_session_capabilities(
        &mut self,
        session_capabilities: SharedSessionCapabilities,
        _cx: &mut Context<Self>,
    ) {
        self.session_capabilities = session_capabilities;
    }

    fn command_hint(&self, snapshot: &MultiBufferSnapshot) -> Option<Inlay> {
        let session_capabilities = self.session_capabilities.read();
        let available_commands = session_capabilities.available_commands();
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
                position: snapshot.anchor_to_buffer_anchor(hint_pos)?.0,
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
        session_id: acp::SessionId,
        title: Option<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.thread_store.is_none() {
            return;
        }
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let thread_title = title
            .filter(|title| !title.is_empty())
            .unwrap_or_else(|| SharedString::new_static(DEFAULT_THREAD_TITLE));
        let uri = MentionUri::Thread {
            id: session_id,
            name: thread_title.to_string(),
        };
        let content = format!("{}\n", uri.as_link());

        let content_len = content.len() - 1;

        let start = self.editor.update(cx, |editor, cx| {
            editor.set_text(content, window, cx);
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            snapshot
                .anchor_to_buffer_anchor(snapshot.anchor_before(Point::zero()))
                .unwrap()
                .0
        });

        let supports_images = self.session_capabilities.read().supports_images();

        self.mention_set
            .update(cx, |mention_set, cx| {
                mention_set.confirm_mention_completion(
                    thread_title,
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
        agent_id: &AgentId,
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
                        agent_id,
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
        let text = self.editor.read(cx).text(cx);
        let available_commands = self
            .session_capabilities
            .read()
            .available_commands()
            .to_vec();
        let agent_id = self.agent_id.clone();
        let build_task = self.build_content_blocks(full_mention_content, cx);

        cx.spawn(async move |_, _cx| {
            Self::validate_slash_commands(&text, &available_commands, &agent_id)?;
            build_task.await
        })
    }

    pub fn draft_contents(&self, cx: &mut Context<Self>) -> Task<Result<Vec<acp::ContentBlock>>> {
        let build_task = self.build_content_blocks(false, cx);
        cx.spawn(async move |_, _cx| {
            let (blocks, _tracked_buffers) = build_task.await?;
            Ok(blocks)
        })
    }

    fn build_content_blocks(
        &self,
        full_mention_content: bool,
        cx: &mut Context<Self>,
    ) -> Task<Result<(Vec<acp::ContentBlock>, Vec<Entity<Buffer>>)>> {
        let contents = self
            .mention_set
            .update(cx, |store, cx| store.contents(full_mention_content, cx));
        let editor = self.editor.clone();
        let supports_embedded_context =
            self.session_capabilities.read().supports_embedded_context();

        cx.spawn(async move |_, cx| {
            let contents = contents.await?;
            let mut all_tracked_buffers = Vec::new();

            let result = editor.update(cx, |editor, cx| {
                let text = editor.text(cx);
                let (mut ix, _) = text
                    .char_indices()
                    .find(|(_, c)| !c.is_whitespace())
                    .unwrap_or((0, '\0'));
                let mut chunks: Vec<acp::ContentBlock> = Vec::new();
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
                            Mention::Image(mention_image) => acp::ContentBlock::Image(
                                acp::ImageContent::new(
                                    mention_image.data.clone(),
                                    mention_image.format.mime_type(),
                                )
                                .uri(match uri {
                                    MentionUri::File { .. } => Some(uri.to_uri().to_string()),
                                    MentionUri::PastedImage { .. } => {
                                        Some(uri.to_uri().to_string())
                                    }
                                    other => {
                                        debug_panic!(
                                            "unexpected mention uri for image: {:?}",
                                            other
                                        );
                                        None
                                    }
                                }),
                            ),
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
                anyhow::Ok((chunks, all_tracked_buffers))
            })?;
            Ok(result)
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
        if !self.is_empty(cx) {
            self.editor.update(cx, |editor, cx| {
                editor.clear_inlay_hints(cx);
            });
        }
        cx.emit(MessageEditorEvent::Send)
    }

    pub fn trigger_completion_menu(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.insert_context_prefix("@", window, cx);
    }

    pub fn insert_context_type(
        &mut self,
        context_keyword: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let prefix = format!("@{}", context_keyword);
        self.insert_context_prefix(&prefix, window, cx);
    }

    fn insert_context_prefix(&mut self, prefix: &str, window: &mut Window, cx: &mut Context<Self>) {
        let editor = self.editor.clone();
        let prefix = prefix.to_string();

        cx.spawn_in(window, async move |_, cx| {
            editor
                .update_in(cx, |editor, window, cx| {
                    let menu_is_open =
                        editor.context_menu().borrow().as_ref().is_some_and(|menu| {
                            matches!(menu, CodeContextMenu::Completions(_)) && menu.visible()
                        });

                    let has_prefix = {
                        let snapshot = editor.display_snapshot(cx);
                        let cursor = editor.selections.newest::<text::Point>(&snapshot).head();
                        let offset = cursor.to_offset(&snapshot);
                        let buffer_snapshot = snapshot.buffer_snapshot();
                        let prefix_char_count = prefix.chars().count();
                        buffer_snapshot
                            .reversed_chars_at(offset)
                            .take(prefix_char_count)
                            .eq(prefix.chars().rev())
                    };

                    if menu_is_open && has_prefix {
                        return;
                    }

                    editor.insert(&prefix, window, cx);
                    editor.show_completions(&editor::actions::ShowCompletions, window, cx);
                })
                .log_err();
        })
        .detach();
    }

    fn chat(&mut self, _: &Chat, _: &mut Window, cx: &mut Context<Self>) {
        self.send(cx);
    }

    fn send_immediately(&mut self, _: &SendImmediately, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_empty(cx) {
            return;
        }

        self.editor.update(cx, |editor, cx| {
            editor.clear_inlay_hints(cx);
        });

        cx.emit(MessageEditorEvent::SendImmediately)
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
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let editor_clipboard_selections = cx.read_from_clipboard().and_then(|item| {
            item.entries().iter().find_map(|entry| match entry {
                ClipboardEntry::String(text) => {
                    text.metadata_json::<Vec<editor::ClipboardSelection>>()
                }
                _ => None,
            })
        });

        // Insert creases for pasted clipboard selections that:
        // 1. Contain exactly one selection
        // 2. Have an associated file path
        // 3. Span multiple lines (not single-line selections)
        // 4. Belong to a file that exists in the current project
        let should_insert_creases = util::maybe!({
            let selections = editor_clipboard_selections.as_ref()?;
            if selections.len() > 1 {
                return Some(false);
            }
            let selection = selections.first()?;
            let file_path = selection.file_path.as_ref()?;
            let line_range = selection.line_range.as_ref()?;

            if line_range.start() == line_range.end() {
                return Some(false);
            }

            Some(
                workspace
                    .read(cx)
                    .project()
                    .read(cx)
                    .project_path_for_absolute_path(file_path, cx)
                    .is_some(),
            )
        })
        .unwrap_or(false);

        if should_insert_creases && let Some(selections) = editor_clipboard_selections {
            cx.stop_propagation();
            let snapshot = self.editor.read(cx).buffer().read(cx).snapshot(cx);
            let (insertion_target, _) = snapshot
                .anchor_to_buffer_anchor(self.editor.read(cx).selections.newest_anchor().start)
                .unwrap();

            let project = workspace.read(cx).project().clone();
            for selection in selections {
                if let (Some(file_path), Some(line_range)) =
                    (selection.file_path, selection.line_range)
                {
                    let crease_text =
                        acp_thread::selection_name(Some(file_path.as_ref()), &line_range);

                    let mention_uri = MentionUri::Selection {
                        abs_path: Some(file_path.clone()),
                        line_range: line_range.clone(),
                    };

                    let mention_text = mention_uri.as_link().to_string();
                    let (text_anchor, content_len) = self.editor.update(cx, |editor, cx| {
                        let buffer = editor.buffer().read(cx);
                        let snapshot = buffer.snapshot(cx);
                        let buffer_snapshot = snapshot.as_singleton().unwrap();
                        let text_anchor = insertion_target.bias_left(&buffer_snapshot);

                        editor.insert(&mention_text, window, cx);
                        editor.insert(" ", window, cx);

                        (text_anchor, mention_text.len())
                    });

                    let Some((crease_id, tx)) = insert_crease_for_mention(
                        text_anchor,
                        content_len,
                        crease_text.into(),
                        mention_uri.icon_path(cx),
                        mention_uri.tooltip_text(),
                        Some(mention_uri.clone()),
                        Some(self.workspace.clone()),
                        None,
                        self.editor.clone(),
                        window,
                        cx,
                    ) else {
                        continue;
                    };
                    drop(tx);

                    let mention_task = cx
                        .spawn({
                            let project = project.clone();
                            async move |_, cx| {
                                let project_path = project
                                    .update(cx, |project, cx| {
                                        project.project_path_for_absolute_path(&file_path, cx)
                                    })
                                    .ok_or_else(|| "project path not found".to_string())?;

                                let buffer = project
                                    .update(cx, |project, cx| project.open_buffer(project_path, cx))
                                    .await
                                    .map_err(|e| e.to_string())?;

                                Ok(buffer.update(cx, |buffer, cx| {
                                    let start =
                                        Point::new(*line_range.start(), 0).min(buffer.max_point());
                                    let end = Point::new(*line_range.end() + 1, 0)
                                        .min(buffer.max_point());
                                    let content = buffer.text_for_range(start..end).collect();
                                    Mention::Text {
                                        content,
                                        tracked_buffers: vec![cx.entity()],
                                    }
                                }))
                            }
                        })
                        .shared();

                    self.mention_set.update(cx, |mention_set, _cx| {
                        mention_set.insert_mention(crease_id, mention_uri.clone(), mention_task)
                    });
                }
            }
            return;
        }
        // Handle text paste with potential markdown mention links before
        // clipboard context entries so markdown text still pastes as text.
        if let Some(clipboard_text) = cx.read_from_clipboard().and_then(|item| {
            item.entries().iter().find_map(|entry| match entry {
                ClipboardEntry::String(text) => Some(text.text().to_string()),
                _ => None,
            })
        }) {
            if clipboard_text.contains("[@") {
                cx.stop_propagation();
                let selections_before = self.editor.update(cx, |editor, cx| {
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    editor
                        .selections
                        .disjoint_anchors()
                        .iter()
                        .map(|selection| {
                            (
                                selection.start.bias_left(&snapshot),
                                selection.end.bias_right(&snapshot),
                            )
                        })
                        .collect::<Vec<_>>()
                });

                self.editor.update(cx, |editor, cx| {
                    editor.insert(&clipboard_text, window, cx);
                });

                let snapshot = self.editor.read(cx).buffer().read(cx).snapshot(cx);
                let path_style = workspace.read(cx).project().read(cx).path_style(cx);

                let mut all_mentions = Vec::new();
                for (start_anchor, end_anchor) in selections_before {
                    let start_offset = start_anchor.to_offset(&snapshot);
                    let end_offset = end_anchor.to_offset(&snapshot);

                    // Get the actual inserted text from the buffer (may differ due to auto-indent)
                    let inserted_text: String =
                        snapshot.text_for_range(start_offset..end_offset).collect();

                    let parsed_mentions = parse_mention_links(&inserted_text, path_style);
                    for (range, mention_uri) in parsed_mentions {
                        let mention_start_offset = MultiBufferOffset(start_offset.0 + range.start);
                        let anchor = snapshot.anchor_before(mention_start_offset);
                        let content_len = range.end - range.start;
                        all_mentions.push((anchor, content_len, mention_uri));
                    }
                }

                if !all_mentions.is_empty() {
                    let supports_images = self.session_capabilities.read().supports_images();
                    let http_client = workspace.read(cx).client().http_client();

                    for (anchor, content_len, mention_uri) in all_mentions {
                        let Some((crease_id, tx)) = insert_crease_for_mention(
                            snapshot.anchor_to_buffer_anchor(anchor).unwrap().0,
                            content_len,
                            mention_uri.name().into(),
                            mention_uri.icon_path(cx),
                            mention_uri.tooltip_text(),
                            Some(mention_uri.clone()),
                            Some(self.workspace.clone()),
                            None,
                            self.editor.clone(),
                            window,
                            cx,
                        ) else {
                            continue;
                        };

                        // Create the confirmation task based on the mention URI type.
                        // This properly loads file content, fetches URLs, etc.
                        let task = self.mention_set.update(cx, |mention_set, cx| {
                            mention_set.confirm_mention_for_uri(
                                mention_uri.clone(),
                                supports_images,
                                http_client.clone(),
                                cx,
                            )
                        });
                        let task = cx
                            .spawn(async move |_, _| task.await.map_err(|e| e.to_string()))
                            .shared();

                        self.mention_set.update(cx, |mention_set, _cx| {
                            mention_set.insert_mention(crease_id, mention_uri.clone(), task.clone())
                        });

                        // Drop the tx after inserting to signal the crease is ready
                        drop(tx);
                    }
                    return;
                }
            }
        }

        if self.handle_pasted_context(window, cx) {
            return;
        }

        // Fall through to default editor paste
        cx.propagate();
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        let Some(text) = self.serialized_copy_text(cx) else {
            cx.propagate();
            return;
        };

        cx.stop_propagation();
        cx.write_to_clipboard(ClipboardItem::new_string(text));
    }

    fn paste_raw(&mut self, _: &PasteRaw, window: &mut Window, cx: &mut Context<Self>) {
        let editor = self.editor.clone();
        window.defer(cx, move |window, cx| {
            editor.update(cx, |editor, cx| editor.paste(&Paste, window, cx));
        });
    }

    fn handle_pasted_context(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        let Some(clipboard) = cx.read_from_clipboard() else {
            return false;
        };

        if matches!(
            clipboard.entries().first(),
            Some(ClipboardEntry::String(_)) | None
        ) {
            return false;
        }

        let Some(workspace) = self.workspace.upgrade() else {
            return false;
        };
        let project = workspace.read(cx).project().clone();
        let project_is_local = project.read(cx).is_local();
        let supports_images = self.session_capabilities.read().supports_images();
        if !project_is_local && !supports_images {
            return false;
        }
        let editor = self.editor.clone();
        let mention_set = self.mention_set.clone();
        let workspace = self.workspace.clone();
        let entries = clipboard.into_entries().collect::<Vec<_>>();

        cx.stop_propagation();

        window
            .spawn(cx, async move |mut cx| {
                let (items, added_worktrees) = resolve_pasted_context_items(
                    project,
                    project_is_local,
                    supports_images,
                    entries,
                    &mut cx,
                )
                .await;
                insert_resolved_pasted_context_items(
                    items,
                    added_worktrees,
                    editor,
                    mention_set,
                    workspace,
                    supports_images,
                    &mut cx,
                )
                .await;
                Ok::<(), anyhow::Error>(())
            })
            .detach_and_log_err(cx);

        true
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
        let project = workspace.read(cx).project().clone();
        let supports_images = self.session_capabilities.read().supports_images();
        let mut tasks = Vec::new();
        for path in paths {
            if let Some(task) = insert_mention_for_project_path(
                &path,
                MentionInsertPosition::EndOfBuffer,
                &self.editor,
                &self.mention_set,
                &project,
                &workspace,
                supports_images,
                window,
                cx,
            ) {
                tasks.push(task);
            }
        }
        cx.spawn(async move |_, _| {
            join_all(tasks).await;
            drop(added_worktrees);
        })
        .detach();
    }

    /// Inserts code snippets as creases into the editor.
    /// Each tuple contains (code_text, crease_title).
    pub fn insert_code_creases(
        &mut self,
        creases: Vec<(String, String)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.insert("\n", window, cx);
        });
        for (text, crease_title) in creases {
            self.insert_crease_impl(text, crease_title, IconName::TextSnippet, true, window, cx);
        }
    }

    pub fn insert_branch_diff_crease(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let project = workspace.read(cx).project().clone();

        let Some(repo) = project.read(cx).active_repository(cx) else {
            return;
        };

        let default_branch_receiver = repo.update(cx, |repo, _| repo.default_branch(false));
        let editor = self.editor.clone();
        let mention_set = self.mention_set.clone();
        let weak_workspace = self.workspace.clone();

        window
            .spawn(cx, async move |cx| {
                let base_ref: SharedString = default_branch_receiver
                    .await
                    .ok()
                    .and_then(|r| r.ok())
                    .flatten()
                    .ok_or_else(|| anyhow!("Could not determine default branch"))?;

                cx.update(|window, cx| {
                    let mention_uri = MentionUri::GitDiff {
                        base_ref: base_ref.to_string(),
                    };
                    let mention_text = mention_uri.as_link().to_string();

                    let (text_anchor, content_len) = editor.update(cx, |editor, cx| {
                        let buffer = editor.buffer().read(cx);
                        let snapshot = buffer.snapshot(cx);
                        let buffer_snapshot = snapshot.as_singleton().unwrap();
                        let text_anchor = snapshot
                            .anchor_to_buffer_anchor(editor.selections.newest_anchor().start)
                            .unwrap()
                            .0
                            .bias_left(&buffer_snapshot);

                        editor.insert(&mention_text, window, cx);
                        editor.insert(" ", window, cx);

                        (text_anchor, mention_text.len())
                    });

                    let Some((crease_id, tx)) = insert_crease_for_mention(
                        text_anchor,
                        content_len,
                        mention_uri.name().into(),
                        mention_uri.icon_path(cx),
                        mention_uri.tooltip_text(),
                        Some(mention_uri.clone()),
                        Some(weak_workspace),
                        None,
                        editor,
                        window,
                        cx,
                    ) else {
                        return;
                    };
                    drop(tx);

                    let confirm_task = mention_set.update(cx, |mention_set, cx| {
                        mention_set.confirm_mention_for_git_diff(base_ref, cx)
                    });

                    let mention_task = cx
                        .spawn(async move |_cx| confirm_task.await.map_err(|e| e.to_string()))
                        .shared();

                    mention_set.update(cx, |mention_set, _| {
                        mention_set.insert_mention(crease_id, mention_uri, mention_task);
                    });
                })
            })
            .detach_and_log_err(cx);
    }

    fn insert_crease_impl(
        &mut self,
        text: String,
        title: String,
        icon: IconName,
        add_trailing_newline: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        use editor::display_map::{Crease, FoldPlaceholder};
        use multi_buffer::MultiBufferRow;
        use rope::Point;

        self.editor.update(cx, |editor, cx| {
            let point = editor
                .selections
                .newest::<Point>(&editor.display_snapshot(cx))
                .head();
            let start_row = MultiBufferRow(point.row);

            editor.insert(&text, window, cx);

            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let anchor_before = snapshot.anchor_after(point);
            let anchor_after = editor
                .selections
                .newest_anchor()
                .head()
                .bias_left(&snapshot);

            if add_trailing_newline {
                editor.insert("\n", window, cx);
            }

            let fold_placeholder = FoldPlaceholder {
                render: Arc::new({
                    let title = title.clone();
                    move |_fold_id, _fold_range, _cx| {
                        Button::new("crease", title.clone())
                            .layer(ElevationIndex::ElevatedSurface)
                            .start_icon(Icon::new(icon))
                            .into_any_element()
                    }
                }),
                merge_adjacent: false,
                ..Default::default()
            };

            let crease = Crease::inline(
                anchor_before..anchor_after,
                fold_placeholder,
                |row, is_folded, fold, _window, _cx| {
                    Disclosure::new(("crease-toggle", row.0 as u64), !is_folded)
                        .toggle_state(is_folded)
                        .on_click(move |_e, window, cx| fold(!is_folded, window, cx))
                        .into_any_element()
                },
                |_, _, _, _| gpui::Empty.into_any(),
            );
            editor.insert_creases(vec![crease], cx);
            editor.fold_at(start_row, window, cx);
        });
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
            PromptCompletionProvider::<MessageEditorCompletionDelegate>::completion_for_action(
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

    pub fn add_images_from_picker(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.session_capabilities.read().supports_images() {
            return;
        }

        let editor = self.editor.clone();
        let mention_set = self.mention_set.clone();
        let workspace = self.workspace.clone();

        let paths_receiver = cx.prompt_for_paths(gpui::PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: Some("Select Images".into()),
        });

        window
            .spawn(cx, async move |cx| {
                let paths = match paths_receiver.await {
                    Ok(Ok(Some(paths))) => paths,
                    _ => return Ok::<(), anyhow::Error>(()),
                };

                let default_image_name: SharedString = "Image".into();
                let images = cx
                    .background_spawn(async move {
                        paths
                            .into_iter()
                            .filter_map(|path| {
                                crate::mention_set::load_external_image_from_path(
                                    &path,
                                    &default_image_name,
                                )
                            })
                            .collect::<Vec<_>>()
                    })
                    .await;

                crate::mention_set::insert_images_as_context(
                    images,
                    editor,
                    mention_set,
                    workspace,
                    cx,
                )
                .await;
                Ok(())
            })
            .detach_and_log_err(cx);
    }

    pub fn set_read_only(&mut self, read_only: bool, cx: &mut Context<Self>) {
        self.editor.update(cx, |message_editor, cx| {
            message_editor.set_read_only(read_only);
            cx.notify()
        })
    }

    pub fn set_mode(&mut self, mode: EditorMode, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            if *editor.mode() != mode {
                editor.set_mode(mode);
                cx.notify()
            }
        });
    }

    pub fn set_message(
        &mut self,
        message: Vec<acp::ContentBlock>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.clear(window, cx);
        self.insert_message_blocks(message, false, window, cx);
    }

    pub fn append_message(
        &mut self,
        message: Vec<acp::ContentBlock>,
        separator: Option<&str>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if message.is_empty() {
            return;
        }

        if let Some(separator) = separator
            && !separator.is_empty()
            && !self.is_empty(cx)
        {
            self.editor.update(cx, |editor, cx| {
                editor.insert(separator, window, cx);
            });
        }

        self.insert_message_blocks(message, true, window, cx);
    }

    fn insert_message_blocks(
        &mut self,
        message: Vec<acp::ContentBlock>,
        append_to_existing: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let path_style = workspace.read(cx).project().read(cx).path_style(cx);
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
                        Ok(MentionUri::PastedImage {
                            name: "Image".to_string(),
                        })
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

        if text.is_empty() && mentions.is_empty() {
            return;
        }

        let insertion_start = if append_to_existing {
            self.editor.read(cx).text(cx).len()
        } else {
            0
        };

        let snapshot = if append_to_existing {
            self.editor.update(cx, |editor, cx| {
                editor.insert(&text, window, cx);
                editor.buffer().read(cx).snapshot(cx)
            })
        } else {
            self.editor.update(cx, |editor, cx| {
                editor.set_text(text, window, cx);
                editor.buffer().read(cx).snapshot(cx)
            })
        };

        for (range, mention_uri, mention) in mentions {
            let adjusted_start = insertion_start + range.start;
            let anchor = snapshot.anchor_before(MultiBufferOffset(adjusted_start));
            let Some((crease_id, tx)) = insert_crease_for_mention(
                snapshot.anchor_to_buffer_anchor(anchor).unwrap().0,
                range.end - range.start,
                mention_uri.name().into(),
                mention_uri.icon_path(cx),
                mention_uri.tooltip_text(),
                Some(mention_uri.clone()),
                Some(self.workspace.clone()),
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

    pub fn set_cursor_offset(
        &mut self,
        offset: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let offset = snapshot.clip_offset(MultiBufferOffset(offset), text::Bias::Left);
            editor.change_selections(Default::default(), window, cx, |selections| {
                selections.select_ranges([offset..offset]);
            });
        });
    }

    pub fn insert_text(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        if text.is_empty() {
            return;
        }

        self.editor.update(cx, |editor, cx| {
            editor.insert(text, window, cx);
        });
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

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_text(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.set_text(text, window, cx);
        });
    }

    fn serialized_copy_text(&self, cx: &mut App) -> Option<String> {
        let display_snapshot = self
            .editor
            .update(cx, |editor, cx| editor.display_snapshot(cx));
        let editor = self.editor.read(cx);
        if !editor.has_non_empty_selection(&display_snapshot) {
            return None;
        }

        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let mention_set = self.mention_set.read(cx);
        let mention_ranges = display_snapshot
            .crease_snapshot
            .crease_items_with_offsets(&snapshot)
            .into_iter()
            .filter_map(|(crease_id, range)| {
                mention_set.mention_uri_for_crease(&crease_id).map(|uri| {
                    (
                        range.start.to_offset(&snapshot),
                        range.end.to_offset(&snapshot),
                        uri,
                    )
                })
            })
            .collect::<Vec<_>>();

        let mut text = String::new();
        let mut has_mentions = false;
        let mut is_first = true;

        for selection in editor
            .selections
            .all::<MultiBufferOffset>(&display_snapshot)
        {
            if is_first {
                is_first = false;
            } else {
                text.push('\n');
            }

            let mut overlapping_mentions = mention_ranges
                .iter()
                .filter(|(start, end, _)| *start < selection.end && selection.start < *end)
                .peekable();

            if overlapping_mentions.peek().is_none() {
                text.extend(snapshot.text_for_range(selection.start..selection.end));
                continue;
            }

            has_mentions = true;

            let mut cursor = selection.start;
            for (start, end, uri) in overlapping_mentions {
                if cursor < *start {
                    text.extend(snapshot.text_for_range(cursor..*start));
                }

                write!(text, "{}", uri.as_link()).unwrap();
                cursor = *end;
            }

            if cursor < selection.end {
                text.extend(snapshot.text_for_range(cursor..selection.end));
            }
        }

        has_mentions.then_some(text)
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
            .on_action(cx.listener(Self::send_immediately))
            .on_action(cx.listener(Self::chat_with_follow))
            .on_action(cx.listener(Self::cancel))
            .capture_action(cx.listener(Self::copy))
            .on_action(cx.listener(Self::paste_raw))
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
                    font_weight: settings.buffer_font.weight,
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

/// Parses markdown mention links in the format `[@name](uri)` from text.
/// Returns a vector of (range, MentionUri) pairs where range is the byte range in the text.
fn parse_mention_links(text: &str, path_style: PathStyle) -> Vec<(Range<usize>, MentionUri)> {
    let mut mentions = Vec::new();
    let mut search_start = 0;

    while let Some(link_start) = text[search_start..].find("[@") {
        let absolute_start = search_start + link_start;

        // Find the matching closing bracket for the name, handling nested brackets.
        // Start at the '[' character so find_matching_bracket can track depth correctly.
        let Some(name_end) = find_matching_bracket(&text[absolute_start..], '[', ']') else {
            search_start = absolute_start + 2;
            continue;
        };
        let name_end = absolute_start + name_end;

        // Check for opening parenthesis immediately after
        if text.get(name_end + 1..name_end + 2) != Some("(") {
            search_start = name_end + 1;
            continue;
        }

        // Find the matching closing parenthesis for the URI, handling nested parens
        let uri_start = name_end + 2;
        let Some(uri_end_relative) = find_matching_bracket(&text[name_end + 1..], '(', ')') else {
            search_start = uri_start;
            continue;
        };
        let uri_end = name_end + 1 + uri_end_relative;
        let link_end = uri_end + 1;

        let uri_str = &text[uri_start..uri_end];

        // Try to parse the URI as a MentionUri
        if let Ok(mention_uri) = MentionUri::parse(uri_str, path_style) {
            mentions.push((absolute_start..link_end, mention_uri));
        }

        search_start = link_end;
    }

    mentions
}

/// Finds the position of the matching closing bracket, handling nested brackets.
/// The input `text` should start with the opening bracket.
/// Returns the index of the matching closing bracket relative to `text`.
fn find_matching_bracket(text: &str, open: char, close: char) -> Option<usize> {
    let mut depth = 0;
    for (index, character) in text.char_indices() {
        if character == open {
            depth += 1;
        } else if character == close {
            depth -= 1;
            if depth == 0 {
                return Some(index);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::{ops::Range, path::Path, path::PathBuf, sync::Arc};

    use acp_thread::MentionUri;
    use agent::{ThreadStore, outline};
    use agent_client_protocol::schema as acp;
    use base64::Engine as _;
    use editor::{
        AnchorRangeExt as _, Editor, EditorMode, MultiBufferOffset, SelectionEffects,
        actions::Paste,
    };

    use fs::FakeFs;
    use futures::{FutureExt as _, StreamExt as _};
    use gpui::{
        AppContext, ClipboardEntry, ClipboardItem, Entity, EventEmitter, ExternalPaths,
        FocusHandle, Focusable, Task, TestAppContext, VisualTestContext,
    };
    use language_model::LanguageModelRegistry;
    use lsp::{CompletionContext, CompletionTriggerKind};
    use parking_lot::RwLock;
    use project::{CompletionIntent, Project, ProjectPath};
    use serde_json::{Value, json};

    use text::Point;
    use ui::{App, Context, IntoElement, Render, SharedString, Window};
    use util::{path, paths::PathStyle, rel_path::rel_path};
    use workspace::{AppState, Item, MultiWorkspace};

    use crate::completion_provider::PromptContextType;
    use crate::{
        conversation_view::tests::init_test,
        mention_set::insert_crease_for_mention,
        message_editor::{Mention, MessageEditor, SessionCapabilities, parse_mention_links},
    };

    #[test]
    fn test_parse_mention_links() {
        // Single file mention
        let text = "[@bundle-mac](file:///Users/test/zed/script/bundle-mac)";
        let mentions = parse_mention_links(text, PathStyle::local());
        assert_eq!(mentions.len(), 1);
        assert_eq!(mentions[0].0, 0..text.len());
        assert!(matches!(mentions[0].1, MentionUri::File { .. }));

        // Multiple mentions
        let text = "Check [@file1](file:///path/to/file1) and [@file2](file:///path/to/file2)!";
        let mentions = parse_mention_links(text, PathStyle::local());
        assert_eq!(mentions.len(), 2);

        // Text without mentions
        let text = "Just some regular text without mentions";
        let mentions = parse_mention_links(text, PathStyle::local());
        assert_eq!(mentions.len(), 0);

        // Malformed mentions (should be skipped)
        let text = "[@incomplete](invalid://uri) and [@missing](";
        let mentions = parse_mention_links(text, PathStyle::local());
        assert_eq!(mentions.len(), 0);

        // Mixed content with valid mention
        let text = "Before [@valid](file:///path/to/file) after";
        let mentions = parse_mention_links(text, PathStyle::local());
        assert_eq!(mentions.len(), 1);
        assert_eq!(mentions[0].0.start, 7);

        // HTTP URL mention (Fetch)
        let text = "Check out [@docs](https://example.com/docs) for more info";
        let mentions = parse_mention_links(text, PathStyle::local());
        assert_eq!(mentions.len(), 1);
        assert!(matches!(mentions[0].1, MentionUri::Fetch { .. }));

        // Directory mention (trailing slash)
        let text = "[@src](file:///path/to/src/)";
        let mentions = parse_mention_links(text, PathStyle::local());
        assert_eq!(mentions.len(), 1);
        assert!(matches!(mentions[0].1, MentionUri::Directory { .. }));

        // Multiple different mention types
        let text = "File [@f](file:///a) and URL [@u](https://b.com) and dir [@d](file:///c/)";
        let mentions = parse_mention_links(text, PathStyle::local());
        assert_eq!(mentions.len(), 3);
        assert!(matches!(mentions[0].1, MentionUri::File { .. }));
        assert!(matches!(mentions[1].1, MentionUri::Fetch { .. }));
        assert!(matches!(mentions[2].1, MentionUri::Directory { .. }));

        // Adjacent mentions without separator
        let text = "[@a](file:///a)[@b](file:///b)";
        let mentions = parse_mention_links(text, PathStyle::local());
        assert_eq!(mentions.len(), 2);

        // Regular markdown link (not a mention) should be ignored
        let text = "[regular link](https://example.com)";
        let mentions = parse_mention_links(text, PathStyle::local());
        assert_eq!(mentions.len(), 0);

        // Incomplete mention link patterns
        let text = "[@name] without url and [@name( malformed";
        let mentions = parse_mention_links(text, PathStyle::local());
        assert_eq!(mentions.len(), 0);

        // Nested brackets in name portion
        let text = "[@name [with brackets]](file:///path/to/file)";
        let mentions = parse_mention_links(text, PathStyle::local());
        assert_eq!(mentions.len(), 1);
        assert_eq!(mentions[0].0, 0..text.len());

        // Deeply nested brackets
        let text = "[@outer [inner [deep]]](file:///path)";
        let mentions = parse_mention_links(text, PathStyle::local());
        assert_eq!(mentions.len(), 1);

        // Unbalanced brackets should fail gracefully
        let text = "[@unbalanced [bracket](file:///path)";
        let mentions = parse_mention_links(text, PathStyle::local());
        assert_eq!(mentions.len(), 0);

        // Nested parentheses in URI (common in URLs with query params)
        let text = "[@wiki](https://en.wikipedia.org/wiki/Rust_(programming_language))";
        let mentions = parse_mention_links(text, PathStyle::local());
        assert_eq!(mentions.len(), 1);
        if let MentionUri::Fetch { url } = &mentions[0].1 {
            assert!(url.as_str().contains("Rust_(programming_language)"));
        } else {
            panic!("Expected Fetch URI");
        }
    }

    #[gpui::test]
    async fn test_at_mention_removal(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({"file": ""})).await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let thread_store = None;

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                MessageEditor::new(
                    workspace.downgrade(),
                    project.downgrade(),
                    thread_store.clone(),
                    None,
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

        let completions = editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello @file ", window, cx);
            let buffer = editor.buffer().read(cx).as_singleton().unwrap();
            let completion_provider = editor.completion_provider().unwrap();
            completion_provider.completions(
                &buffer,
                text::Anchor::max_for_buffer(buffer.read(cx).remote_id()),
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
                .buffer_anchor_range_to_anchor_range(completion.replace_range)
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
        let thread_store = None;
        let session_capabilities = Arc::new(RwLock::new(SessionCapabilities::new(
            acp::PromptCapabilities::default(),
            vec![],
        )));

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let workspace_handle = workspace.downgrade();
        let message_editor = workspace.update_in(cx, |_, window, cx| {
            cx.new(|cx| {
                MessageEditor::new(
                    workspace_handle.clone(),
                    project.downgrade(),
                    thread_store.clone(),
                    None,
                    session_capabilities.clone(),
                    "Claude Agent".into(),
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
        assert!(error_message.contains("not supported by Claude Agent"));
        assert!(error_message.contains("Available commands: none"));

        // Now simulate Claude providing its list of available commands (which doesn't include file)
        session_capabilities
            .write()
            .set_available_commands(vec![acp::AvailableCommand::new("help", "Get help")]);

        // Test that unsupported slash commands trigger an error when we have a list of available commands
        editor.update_in(cx, |editor, window, cx| {
            editor.set_text("/file test.txt", window, cx);
        });

        let contents_result = message_editor
            .update(cx, |message_editor, cx| message_editor.contents(false, cx))
            .await;

        assert!(contents_result.is_err());
        let error_message = contents_result.unwrap_err().to_string();
        assert!(error_message.contains("not supported by Claude Agent"));
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
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();

        let mut cx = VisualTestContext::from_window(window.into(), cx);

        let thread_store = None;
        let session_capabilities = Arc::new(RwLock::new(SessionCapabilities::new(
            acp::PromptCapabilities::default(),
            vec![
                acp::AvailableCommand::new("quick-math", "2 + 2 = 4 - 1 = 3"),
                acp::AvailableCommand::new("say-hello", "Say hello to whoever you want").input(
                    acp::AvailableCommandInput::Unstructured(acp::UnstructuredCommandInput::new(
                        "<name>",
                    )),
                ),
            ],
        )));

        let editor = workspace.update_in(&mut cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            let message_editor = cx.new(|cx| {
                MessageEditor::new(
                    workspace_handle,
                    project.downgrade(),
                    thread_store.clone(),
                    None,
                    session_capabilities.clone(),
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
            message_editor.read(cx).focus_handle(cx).focus(window, cx);
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
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();

        let worktree = project.update(cx, |project, cx| {
            let mut worktrees = project.worktrees(cx).collect::<Vec<_>>();
            assert_eq!(worktrees.len(), 1);
            worktrees.pop().unwrap()
        });
        let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id());

        let mut cx = VisualTestContext::from_window(window.into(), cx);

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

        let thread_store = cx.new(|cx| ThreadStore::new(cx));
        let session_capabilities = Arc::new(RwLock::new(SessionCapabilities::new(
            acp::PromptCapabilities::default(),
            vec![],
        )));

        let (message_editor, editor) = workspace.update_in(&mut cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            let message_editor = cx.new(|cx| {
                MessageEditor::new(
                    workspace_handle,
                    project.downgrade(),
                    Some(thread_store),
                    None,
                    session_capabilities.clone(),
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
            message_editor.read(cx).focus_handle(cx).focus(window, cx);
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

        message_editor.update(&mut cx, |editor, _cx| {
            editor.session_capabilities.write().set_prompt_capabilities(
                acp::PromptCapabilities::new()
                    .image(true)
                    .audio(true)
                    .embedded_context(true),
            );
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

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let thread_store = Some(cx.new(|cx| ThreadStore::new(cx)));

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                let editor = MessageEditor::new(
                    workspace.downgrade(),
                    project.downgrade(),
                    thread_store.clone(),
                    None,
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
                    .session_capabilities
                    .write()
                    .set_prompt_capabilities(acp::PromptCapabilities::new().embedded_context(true));
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

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let thread_store = Some(cx.new(|cx| ThreadStore::new(cx)));

        let session_id = acp::SessionId::new("thread-123");
        let title = Some("Previous Conversation".into());

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                let mut editor = MessageEditor::new(
                    workspace.downgrade(),
                    project.downgrade(),
                    thread_store.clone(),
                    None,
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
                editor.insert_thread_summary(session_id.clone(), title.clone(), window, cx);
                editor
            })
        });

        // Construct expected values for verification
        let expected_uri = MentionUri::Thread {
            id: session_id.clone(),
            name: title.as_ref().unwrap().to_string(),
        };
        let expected_title = title.as_ref().unwrap();
        let expected_link = format!("[@{}]({})", expected_title, expected_uri.to_uri());

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
    async fn test_insert_thread_summary_skipped_for_external_agents(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(LanguageModelRegistry::test);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({"file": ""})).await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let thread_store = None;

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                let mut editor = MessageEditor::new(
                    workspace.downgrade(),
                    project.downgrade(),
                    thread_store.clone(),
                    None,
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
                editor.insert_thread_summary(
                    acp::SessionId::new("thread-123"),
                    Some("Previous Conversation".into()),
                    window,
                    cx,
                );
                editor
            })
        });

        message_editor.read_with(cx, |editor, cx| {
            assert!(
                editor.text(cx).is_empty(),
                "Expected thread summary to be skipped for external agents"
            );
            assert!(
                editor.mention_set().read(cx).mentions().is_empty(),
                "Expected no mentions when thread summary is skipped"
            );
        });
    }

    #[gpui::test]
    async fn test_thread_mode_hidden_when_disabled(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({"file": ""})).await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let thread_store = None;

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                MessageEditor::new(
                    workspace.downgrade(),
                    project.downgrade(),
                    thread_store.clone(),
                    None,
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

        message_editor.update(cx, |editor, _cx| {
            editor
                .session_capabilities
                .write()
                .set_prompt_capabilities(acp::PromptCapabilities::new().embedded_context(true));
        });

        let supported_modes = {
            let app = cx.app.borrow();
            let _ = &app;
            message_editor
                .read(&app)
                .session_capabilities
                .read()
                .supported_modes(false)
        };

        assert!(
            !supported_modes.contains(&PromptContextType::Thread),
            "Expected thread mode to be hidden when thread mentions are disabled"
        );
    }

    #[gpui::test]
    async fn test_thread_mode_visible_when_enabled(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({"file": ""})).await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let thread_store = Some(cx.new(|cx| ThreadStore::new(cx)));

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                MessageEditor::new(
                    workspace.downgrade(),
                    project.downgrade(),
                    thread_store.clone(),
                    None,
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

        message_editor.update(cx, |editor, _cx| {
            editor
                .session_capabilities
                .write()
                .set_prompt_capabilities(acp::PromptCapabilities::new().embedded_context(true));
        });

        let supported_modes = {
            let app = cx.app.borrow();
            let _ = &app;
            message_editor
                .read(&app)
                .session_capabilities
                .read()
                .supported_modes(true)
        };

        assert!(
            supported_modes.contains(&PromptContextType::Thread),
            "Expected thread mode to be visible when enabled"
        );
    }

    #[gpui::test]
    async fn test_whitespace_trimming(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({"file.rs": "fn main() {}"}))
            .await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let thread_store = Some(cx.new(|cx| ThreadStore::new(cx)));

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                MessageEditor::new(
                    workspace.downgrade(),
                    project.downgrade(),
                    thread_store.clone(),
                    None,
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

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let thread_store = Some(cx.new(|cx| ThreadStore::new(cx)));

        let (message_editor, editor) = workspace.update_in(cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            let message_editor = cx.new(|cx| {
                MessageEditor::new(
                    workspace_handle,
                    project.downgrade(),
                    thread_store.clone(),
                    None,
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
            message_editor.read(cx).focus_handle(cx).focus(window, cx);
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
                .session_capabilities
                .write()
                .set_prompt_capabilities(acp::PromptCapabilities::new().embedded_context(true))
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
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();

        let worktree = project.update(cx, |project, cx| {
            let mut worktrees = project.worktrees(cx).collect::<Vec<_>>();
            assert_eq!(worktrees.len(), 1);
            worktrees.pop().unwrap()
        });
        let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id());

        let mut cx = VisualTestContext::from_window(window.into(), cx);

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

        let thread_store = Some(cx.new(|cx| ThreadStore::new(cx)));

        // Create a new `MessageEditor`. The `EditorMode::full()` has to be used
        // to ensure we have a fixed viewport, so we can eventually actually
        // place the cursor outside of the visible area.
        let message_editor = workspace.update_in(&mut cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            let message_editor = cx.new(|cx| {
                MessageEditor::new(
                    workspace_handle,
                    project.downgrade(),
                    thread_store.clone(),
                    None,
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

    #[gpui::test]
    async fn test_insert_context_with_multibyte_characters(cx: &mut TestAppContext) {
        init_test(cx);

        let app_state = cx.update(AppState::test);

        cx.update(|cx| {
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
        });

        app_state
            .fs
            .as_fake()
            .insert_tree(path!("/dir"), json!({}))
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();

        let mut cx = VisualTestContext::from_window(window.into(), cx);

        let thread_store = cx.new(|cx| ThreadStore::new(cx));

        let (message_editor, editor) = workspace.update_in(&mut cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            let message_editor = cx.new(|cx| {
                MessageEditor::new(
                    workspace_handle,
                    project.downgrade(),
                    Some(thread_store.clone()),
                    None,
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
            message_editor.read(cx).focus_handle(cx).focus(window, cx);
            let editor = message_editor.read(cx).editor().clone();
            (message_editor, editor)
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.set_text("😄😄", window, cx);
        });

        cx.run_until_parked();

        message_editor.update_in(&mut cx, |message_editor, window, cx| {
            message_editor.insert_context_type("file", window, cx);
        });

        cx.run_until_parked();

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "😄😄@file");
        });
    }

    #[gpui::test]
    async fn test_paste_mention_link_with_multiple_selections(cx: &mut TestAppContext) {
        init_test(cx);

        let app_state = cx.update(AppState::test);

        cx.update(|cx| {
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
        });

        app_state
            .fs
            .as_fake()
            .insert_tree(path!("/project"), json!({"file.txt": "content"}))
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/project").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();

        let mut cx = VisualTestContext::from_window(window.into(), cx);

        let thread_store = cx.new(|cx| ThreadStore::new(cx));

        let (message_editor, editor) = workspace.update_in(&mut cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            let message_editor = cx.new(|cx| {
                MessageEditor::new(
                    workspace_handle,
                    project.downgrade(),
                    Some(thread_store),
                    None,
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
            message_editor.read(cx).focus_handle(cx).focus(window, cx);
            let editor = message_editor.read(cx).editor().clone();
            (message_editor, editor)
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.set_text(
                "AAAAAAAAAAAAAAAAAAAAAAAAA     AAAAAAAAAAAAAAAAAAAAAAAAA",
                window,
                cx,
            );
        });

        cx.run_until_parked();

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_ranges([
                    MultiBufferOffset(0)..MultiBufferOffset(25), // First selection (large)
                    MultiBufferOffset(30)..MultiBufferOffset(55), // Second selection (newest)
                ]);
            });
        });

        let mention_link = "[@f](file:///test.txt)";
        cx.write_to_clipboard(ClipboardItem::new_string(mention_link.into()));

        message_editor.update_in(&mut cx, |message_editor, window, cx| {
            message_editor.paste(&Paste, window, cx);
        });

        let text = editor.update(&mut cx, |editor, cx| editor.text(cx));
        assert!(
            text.contains("[@f](file:///test.txt)"),
            "Expected mention link to be pasted, got: {}",
            text
        );
    }

    #[gpui::test]
    async fn test_copy_with_selection_mentions_serializes_links(cx: &mut TestAppContext) {
        init_test(cx);

        let (source_message_editor, _source_editor, mut cx) = setup_paste_test_message_editor(
            json!({"file.rs": "line 1\nline 2\nline 3\nline 4\n"}),
            cx,
        )
        .await;

        let workspace = source_message_editor.read_with(&cx, |message_editor, _| {
            message_editor.workspace.upgrade().expect("workspace")
        });
        let project = workspace.read_with(&cx, |workspace, _| workspace.project().clone());

        let source_text = "selection needs work\nselection looks fine";
        let first_range = 0..9;
        let second_start = "selection needs work\n".len();
        let second_range = second_start..(second_start + "selection".len());
        let first_uri = MentionUri::Selection {
            abs_path: Some(path!("/project/file.rs").into()),
            line_range: 0..=1,
        };
        let second_uri = MentionUri::Selection {
            abs_path: Some(path!("/project/file.rs").into()),
            line_range: 2..=3,
        };

        source_message_editor.update_in(&mut cx, |message_editor, window, cx| {
            message_editor.set_text(source_text, window, cx);

            let snapshot = message_editor
                .editor
                .read(cx)
                .buffer()
                .read(cx)
                .snapshot(cx);
            for (range, uri, content) in [
                (
                    first_range.clone(),
                    first_uri.clone(),
                    "line 1\nline 2\n".to_string(),
                ),
                (
                    second_range.clone(),
                    second_uri.clone(),
                    "line 3\nline 4\n".to_string(),
                ),
            ] {
                let Some((crease_id, tx)) = insert_crease_for_mention(
                    snapshot
                        .anchor_to_buffer_anchor(
                            snapshot.anchor_before(MultiBufferOffset(range.start)),
                        )
                        .expect("selection mention anchor should map to a buffer")
                        .0,
                    range.len(),
                    uri.name().into(),
                    uri.icon_path(cx),
                    uri.tooltip_text(),
                    Some(uri.clone()),
                    Some(message_editor.workspace.clone()),
                    None,
                    message_editor.editor.clone(),
                    window,
                    cx,
                ) else {
                    panic!("expected mention crease insertion");
                };
                drop(tx);

                message_editor.mention_set.update(cx, |mention_set, _cx| {
                    mention_set.insert_mention(
                        crease_id,
                        uri,
                        Task::ready(Ok(Mention::Text {
                            content,
                            tracked_buffers: Vec::new(),
                        }))
                        .shared(),
                    );
                });
            }

            let buffer_len = snapshot.len();
            message_editor.editor.update(cx, |editor, cx| {
                editor.change_selections(Default::default(), window, cx, |selections| {
                    selections.select_ranges([MultiBufferOffset(0)..buffer_len]);
                });
            });
        });

        let copied_text = source_message_editor.update(&mut cx, |message_editor, cx| {
            message_editor
                .serialized_copy_text(cx)
                .expect("selection mentions should serialize")
        });
        let expected_text = format!(
            "{} needs work\n{} looks fine",
            first_uri.as_link(),
            second_uri.as_link()
        );
        assert_eq!(copied_text, expected_text);

        let target_message_editor = workspace.update_in(&mut cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            let thread_store = cx.new(|cx| ThreadStore::new(cx));
            let message_editor = cx.new(|cx| {
                MessageEditor::new(
                    workspace_handle,
                    project.downgrade(),
                    Some(thread_store),
                    None,
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
            message_editor.read(cx).focus_handle(cx).focus(window, cx);
            message_editor
        });

        cx.write_to_clipboard(ClipboardItem::new_string(copied_text));
        target_message_editor.update_in(&mut cx, |message_editor, window, cx| {
            message_editor.paste(&Paste, window, cx);
        });
        cx.run_until_parked();

        let target_text = target_message_editor.read_with(&cx, |message_editor, cx| {
            message_editor.editor.read(cx).text(cx)
        });
        assert_eq!(target_text, expected_text);

        let contents = mention_contents(&target_message_editor, &mut cx).await;
        assert_eq!(contents.len(), 2);
        assert!(contents.iter().any(|(uri, _)| uri == &first_uri));
        assert!(contents.iter().any(|(uri, _)| uri == &second_uri));
    }

    struct SelectionMentionFixture {
        message_editor: Entity<MessageEditor>,
        first_uri: MentionUri,
        first_range: Range<usize>,
        second_range: Range<usize>,
    }

    async fn setup_selection_mention_fixture(
        cx: &mut TestAppContext,
    ) -> (SelectionMentionFixture, VisualTestContext) {
        let (message_editor, _source_editor, mut cx) = setup_paste_test_message_editor(
            json!({"file.rs": "line 1\nline 2\nline 3\nline 4\n"}),
            cx,
        )
        .await;

        let source_text = "selection needs work\nselection looks fine";
        let first_range = 0..9;
        let second_start = "selection needs work\n".len();
        let second_range = second_start..(second_start + "selection".len());
        let first_uri = MentionUri::Selection {
            abs_path: Some(path!("/project/file.rs").into()),
            line_range: 0..=1,
        };
        let second_uri = MentionUri::Selection {
            abs_path: Some(path!("/project/file.rs").into()),
            line_range: 2..=3,
        };

        message_editor.update_in(&mut cx, |message_editor, window, cx| {
            message_editor.set_text(source_text, window, cx);

            let snapshot = message_editor
                .editor
                .read(cx)
                .buffer()
                .read(cx)
                .snapshot(cx);
            for (range, uri, content) in [
                (
                    first_range.clone(),
                    first_uri.clone(),
                    "line 1\nline 2\n".to_string(),
                ),
                (
                    second_range.clone(),
                    second_uri.clone(),
                    "line 3\nline 4\n".to_string(),
                ),
            ] {
                let Some((crease_id, tx)) = insert_crease_for_mention(
                    snapshot
                        .anchor_to_buffer_anchor(
                            snapshot.anchor_before(MultiBufferOffset(range.start)),
                        )
                        .expect("selection mention anchor should map to a buffer")
                        .0,
                    range.len(),
                    uri.name().into(),
                    uri.icon_path(cx),
                    uri.tooltip_text(),
                    Some(uri.clone()),
                    Some(message_editor.workspace.clone()),
                    None,
                    message_editor.editor.clone(),
                    window,
                    cx,
                ) else {
                    panic!("expected mention crease insertion");
                };
                drop(tx);

                message_editor.mention_set.update(cx, |mention_set, _cx| {
                    mention_set.insert_mention(
                        crease_id,
                        uri,
                        Task::ready(Ok(Mention::Text {
                            content,
                            tracked_buffers: Vec::new(),
                        }))
                        .shared(),
                    );
                });
            }
        });

        (
            SelectionMentionFixture {
                message_editor,
                first_uri,
                first_range,
                second_range,
            },
            cx,
        )
    }

    #[gpui::test]
    async fn test_serialized_copy_text_selection_covers_only_mention(cx: &mut TestAppContext) {
        init_test(cx);

        let (fixture, mut cx) = setup_selection_mention_fixture(cx).await;

        fixture
            .message_editor
            .update_in(&mut cx, |message_editor, window, cx| {
                let range = fixture.first_range.clone();
                message_editor.editor.update(cx, |editor, cx| {
                    editor.change_selections(Default::default(), window, cx, |selections| {
                        selections.select_ranges([
                            MultiBufferOffset(range.start)..MultiBufferOffset(range.end)
                        ]);
                    });
                });
            });

        let copied = fixture
            .message_editor
            .update(&mut cx, |message_editor, cx| {
                message_editor.serialized_copy_text(cx)
            });

        assert_eq!(copied, Some(fixture.first_uri.as_link().to_string()));
    }

    #[gpui::test]
    async fn test_serialized_copy_text_returns_none_when_mentions_outside_selection(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let (fixture, mut cx) = setup_selection_mention_fixture(cx).await;

        let between_start = fixture.first_range.end;
        let between_end = fixture.second_range.start - 1;

        fixture
            .message_editor
            .update_in(&mut cx, |message_editor, window, cx| {
                message_editor.editor.update(cx, |editor, cx| {
                    editor.change_selections(Default::default(), window, cx, |selections| {
                        selections.select_ranges([
                            MultiBufferOffset(between_start)..MultiBufferOffset(between_end)
                        ]);
                    });
                });
            });

        let copied = fixture
            .message_editor
            .update(&mut cx, |message_editor, cx| {
                message_editor.serialized_copy_text(cx)
            });

        assert_eq!(copied, None);
    }

    #[gpui::test]
    async fn test_paste_mention_link_with_completion_trigger_does_not_panic(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let app_state = cx.update(AppState::test);

        cx.update(|cx| {
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
        });

        app_state
            .fs
            .as_fake()
            .insert_tree(path!("/project"), json!({"file.txt": "content"}))
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/project").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();

        let mut cx = VisualTestContext::from_window(window.into(), cx);

        let thread_store = cx.new(|cx| ThreadStore::new(cx));

        let (_message_editor, editor) = workspace.update_in(&mut cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            let message_editor = cx.new(|cx| {
                MessageEditor::new(
                    workspace_handle,
                    project.downgrade(),
                    Some(thread_store),
                    None,
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
            message_editor.read(cx).focus_handle(cx).focus(window, cx);
            let editor = message_editor.read(cx).editor().clone();
            (message_editor, editor)
        });

        cx.simulate_input("@");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), "@");
            assert!(editor.has_visible_completions_menu());
        });

        cx.write_to_clipboard(ClipboardItem::new_string("[@f](file:///test.txt) @".into()));
        cx.dispatch_action(Paste);

        editor.update(&mut cx, |editor, cx| {
            assert!(editor.text(cx).contains("[@f](file:///test.txt)"));
        });
    }

    #[gpui::test]
    async fn test_paste_external_file_path_inserts_file_mention(cx: &mut TestAppContext) {
        init_test(cx);
        let (message_editor, editor, mut cx) =
            setup_paste_test_message_editor(json!({"file.txt": "content"}), cx).await;
        paste_external_paths(
            &message_editor,
            vec![PathBuf::from(path!("/project/file.txt"))],
            &mut cx,
        );

        let expected_uri = MentionUri::File {
            abs_path: path!("/project/file.txt").into(),
        }
        .to_uri()
        .to_string();

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), format!("[@file.txt]({expected_uri}) "));
        });

        let contents = mention_contents(&message_editor, &mut cx).await;

        let [(uri, Mention::Text { content, .. })] = contents.as_slice() else {
            panic!("Unexpected mentions");
        };
        assert_eq!(content, "content");
        assert_eq!(
            uri,
            &MentionUri::File {
                abs_path: path!("/project/file.txt").into(),
            }
        );
    }

    #[gpui::test]
    async fn test_paste_external_directory_path_inserts_directory_mention(cx: &mut TestAppContext) {
        init_test(cx);
        let (message_editor, editor, mut cx) = setup_paste_test_message_editor(
            json!({
                "src": {
                    "main.rs": "fn main() {}\n",
                }
            }),
            cx,
        )
        .await;
        paste_external_paths(
            &message_editor,
            vec![PathBuf::from(path!("/project/src"))],
            &mut cx,
        );

        let expected_uri = MentionUri::Directory {
            abs_path: path!("/project/src").into(),
        }
        .to_uri()
        .to_string();

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(editor.text(cx), format!("[@src]({expected_uri}) "));
        });

        let contents = mention_contents(&message_editor, &mut cx).await;

        let [(uri, Mention::Link)] = contents.as_slice() else {
            panic!("Unexpected mentions");
        };
        assert_eq!(
            uri,
            &MentionUri::Directory {
                abs_path: path!("/project/src").into(),
            }
        );
    }

    #[gpui::test]
    async fn test_paste_external_file_path_inserts_at_cursor(cx: &mut TestAppContext) {
        init_test(cx);
        let (message_editor, editor, mut cx) =
            setup_paste_test_message_editor(json!({"file.txt": "content"}), cx).await;

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.set_text("Hello world", window, cx);
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([MultiBufferOffset(6)..MultiBufferOffset(6)]);
            });
        });

        paste_external_paths(
            &message_editor,
            vec![PathBuf::from(path!("/project/file.txt"))],
            &mut cx,
        );

        let expected_uri = MentionUri::File {
            abs_path: path!("/project/file.txt").into(),
        }
        .to_uri()
        .to_string();

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                format!("Hello [@file.txt]({expected_uri}) world")
            );
        });
    }

    #[gpui::test]
    async fn test_paste_mixed_external_image_without_extension_and_file_path(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let (message_editor, editor, mut cx) =
            setup_paste_test_message_editor(json!({"file.txt": "content"}), cx).await;

        message_editor.update(&mut cx, |message_editor, _cx| {
            message_editor
                .session_capabilities
                .write()
                .set_prompt_capabilities(acp::PromptCapabilities::new().image(true));
        });

        let temporary_image_path = write_test_png_file(None);
        paste_external_paths(
            &message_editor,
            vec![
                temporary_image_path.clone(),
                PathBuf::from(path!("/project/file.txt")),
            ],
            &mut cx,
        );

        let image_name = temporary_image_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Image")
            .to_string();
        std::fs::remove_file(&temporary_image_path).expect("remove temp png");

        let expected_file_uri = MentionUri::File {
            abs_path: path!("/project/file.txt").into(),
        }
        .to_uri()
        .to_string();
        let expected_image_uri = MentionUri::PastedImage {
            name: image_name.clone(),
        }
        .to_uri()
        .to_string();

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                format!("[@{image_name}]({expected_image_uri}) [@file.txt]({expected_file_uri}) ")
            );
        });

        let contents = mention_contents(&message_editor, &mut cx).await;

        assert_eq!(contents.len(), 2);
        assert!(contents.iter().any(|(uri, mention)| {
            matches!(uri, MentionUri::PastedImage { .. }) && matches!(mention, Mention::Image(_))
        }));
        assert!(contents.iter().any(|(uri, mention)| {
            *uri == MentionUri::File {
                abs_path: path!("/project/file.txt").into(),
            } && matches!(
                mention,
                Mention::Text {
                    content,
                    tracked_buffers: _,
                } if content == "content"
            )
        }));
    }

    async fn setup_paste_test_message_editor(
        project_tree: Value,
        cx: &mut TestAppContext,
    ) -> (Entity<MessageEditor>, Entity<Editor>, VisualTestContext) {
        let app_state = cx.update(AppState::test);

        cx.update(|cx| {
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
        });

        app_state
            .fs
            .as_fake()
            .insert_tree(path!("/project"), project_tree)
            .await;

        let project = Project::test(app_state.fs.clone(), [path!("/project").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();

        let mut cx = VisualTestContext::from_window(window.into(), cx);

        let thread_store = cx.new(|cx| ThreadStore::new(cx));

        let (message_editor, editor) = workspace.update_in(&mut cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            let message_editor = cx.new(|cx| {
                MessageEditor::new(
                    workspace_handle,
                    project.downgrade(),
                    Some(thread_store),
                    None,
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
            message_editor.read(cx).focus_handle(cx).focus(window, cx);
            let editor = message_editor.read(cx).editor().clone();
            (message_editor, editor)
        });

        (message_editor, editor, cx)
    }

    fn paste_external_paths(
        message_editor: &Entity<MessageEditor>,
        paths: Vec<PathBuf>,
        cx: &mut VisualTestContext,
    ) {
        cx.write_to_clipboard(ClipboardItem {
            entries: vec![ClipboardEntry::ExternalPaths(ExternalPaths(paths.into()))],
        });

        message_editor.update_in(cx, |message_editor, window, cx| {
            message_editor.paste(&Paste, window, cx);
        });
        cx.run_until_parked();
    }

    async fn mention_contents(
        message_editor: &Entity<MessageEditor>,
        cx: &mut VisualTestContext,
    ) -> Vec<(MentionUri, Mention)> {
        message_editor
            .update(cx, |message_editor, cx| {
                message_editor
                    .mention_set()
                    .update(cx, |mention_set, cx| mention_set.contents(false, cx))
            })
            .await
            .unwrap()
            .into_values()
            .collect::<Vec<_>>()
    }

    fn write_test_png_file(extension: Option<&str>) -> PathBuf {
        let bytes = base64::prelude::BASE64_STANDARD
            .decode("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==")
            .expect("decode png");
        let file_name = match extension {
            Some(extension) => format!("zed-agent-ui-test-{}.{}", uuid::Uuid::new_v4(), extension),
            None => format!("zed-agent-ui-test-{}", uuid::Uuid::new_v4()),
        };
        let path = std::env::temp_dir().join(file_name);
        std::fs::write(&path, bytes).expect("write temp png");
        path
    }

    // Helper that creates a minimal MessageEditor inside a window, returning both
    // the entity and the underlying VisualTestContext so callers can drive updates.
    async fn setup_message_editor(
        cx: &mut TestAppContext,
    ) -> (Entity<MessageEditor>, &mut VisualTestContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({"file.txt": ""})).await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                MessageEditor::new(
                    workspace.downgrade(),
                    project.downgrade(),
                    None,
                    None,
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

        cx.run_until_parked();
        (message_editor, cx)
    }

    #[gpui::test]
    async fn test_set_message_plain_text(cx: &mut TestAppContext) {
        init_test(cx);
        let (message_editor, cx) = setup_message_editor(cx).await;

        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_message(
                vec![acp::ContentBlock::Text(acp::TextContent::new(
                    "hello world".to_string(),
                ))],
                window,
                cx,
            );
        });

        let text = message_editor.update(cx, |editor, cx| editor.text(cx));
        assert_eq!(text, "hello world");
        assert!(!message_editor.update(cx, |editor, cx| editor.is_empty(cx)));
    }

    #[gpui::test]
    async fn test_set_message_replaces_existing_content(cx: &mut TestAppContext) {
        init_test(cx);
        let (message_editor, cx) = setup_message_editor(cx).await;

        // Set initial content.
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_message(
                vec![acp::ContentBlock::Text(acp::TextContent::new(
                    "old content".to_string(),
                ))],
                window,
                cx,
            );
        });

        // Replace with new content.
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_message(
                vec![acp::ContentBlock::Text(acp::TextContent::new(
                    "new content".to_string(),
                ))],
                window,
                cx,
            );
        });

        let text = message_editor.update(cx, |editor, cx| editor.text(cx));
        assert_eq!(
            text, "new content",
            "set_message should replace old content"
        );
    }

    #[gpui::test]
    async fn test_append_message_to_empty_editor(cx: &mut TestAppContext) {
        init_test(cx);
        let (message_editor, cx) = setup_message_editor(cx).await;

        message_editor.update_in(cx, |editor, window, cx| {
            editor.append_message(
                vec![acp::ContentBlock::Text(acp::TextContent::new(
                    "appended".to_string(),
                ))],
                Some("\n\n"),
                window,
                cx,
            );
        });

        let text = message_editor.update(cx, |editor, cx| editor.text(cx));
        assert_eq!(
            text, "appended",
            "No separator should be inserted when the editor is empty"
        );
    }

    #[gpui::test]
    async fn test_append_message_to_non_empty_editor(cx: &mut TestAppContext) {
        init_test(cx);
        let (message_editor, cx) = setup_message_editor(cx).await;

        // Seed initial content.
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_message(
                vec![acp::ContentBlock::Text(acp::TextContent::new(
                    "initial".to_string(),
                ))],
                window,
                cx,
            );
        });

        // Append with separator.
        message_editor.update_in(cx, |editor, window, cx| {
            editor.append_message(
                vec![acp::ContentBlock::Text(acp::TextContent::new(
                    "appended".to_string(),
                ))],
                Some("\n\n"),
                window,
                cx,
            );
        });

        let text = message_editor.update(cx, |editor, cx| editor.text(cx));
        assert_eq!(
            text, "initial\n\nappended",
            "Separator should appear between existing and appended content"
        );
    }

    #[gpui::test]
    async fn test_append_message_preserves_mention_offset(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({"file.txt": "content"}))
            .await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                MessageEditor::new(
                    workspace.downgrade(),
                    project.downgrade(),
                    None,
                    None,
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

        cx.run_until_parked();

        // Seed plain-text prefix so the editor is non-empty before appending.
        message_editor.update_in(cx, |editor, window, cx| {
            editor.set_message(
                vec![acp::ContentBlock::Text(acp::TextContent::new(
                    "prefix text".to_string(),
                ))],
                window,
                cx,
            );
        });

        // Append a message that contains a ResourceLink mention.
        message_editor.update_in(cx, |editor, window, cx| {
            editor.append_message(
                vec![acp::ContentBlock::ResourceLink(acp::ResourceLink::new(
                    "file.txt",
                    "file:///project/file.txt",
                ))],
                Some("\n\n"),
                window,
                cx,
            );
        });

        cx.run_until_parked();

        // The mention should be registered in the mention_set so that contents()
        // will emit it as a structured block rather than plain text.
        let mention_uris =
            message_editor.update(cx, |editor, cx| editor.mention_set.read(cx).mentions());
        assert_eq!(
            mention_uris.len(),
            1,
            "Expected exactly one mention in the mention_set after append, got: {mention_uris:?}"
        );

        // The editor text should start with the prefix, then the separator, then
        // the mention placeholder — confirming the offset was computed correctly.
        let text = message_editor.update(cx, |editor, cx| editor.text(cx));
        assert!(
            text.starts_with("prefix text\n\n"),
            "Expected text to start with 'prefix text\\n\\n', got: {text:?}"
        );
    }
}
