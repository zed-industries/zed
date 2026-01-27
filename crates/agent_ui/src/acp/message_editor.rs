use crate::SendImmediately;
use crate::acp::AcpThreadHistory;
use crate::{
    ChatWithFollow,
    completion_provider::{
        PromptCompletionProvider, PromptCompletionProviderDelegate, PromptContextAction,
        PromptContextType, SlashCommandCompletion,
    },
    mention_set::{
        Mention, MentionImage, MentionSet, insert_crease_for_mention, paste_images_as_context,
    },
    user_slash_command::{self, CommandLoadError, UserSlashCommand},
};
use acp_thread::{AgentSessionInfo, MentionUri};
use agent::ThreadStore;
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use collections::HashSet;
use editor::{
    Addon, AnchorRangeExt, ContextMenuOptions, ContextMenuPlacement, Editor, EditorElement,
    EditorEvent, EditorMode, EditorStyle, Inlay, MultiBuffer, MultiBufferOffset,
    MultiBufferSnapshot, ToOffset, actions::Paste, code_context_menus::CodeContextMenu,
    scroll::Autoscroll,
};
use feature_flags::{FeatureFlagAppExt as _, UserSlashCommandsFeatureFlag};
use futures::{FutureExt as _, future::join_all};
use gpui::{
    AppContext, ClipboardEntry, Context, Entity, EventEmitter, FocusHandle, Focusable, ImageFormat,
    KeyContext, SharedString, Subscription, Task, TextStyle, WeakEntity,
};
use language::{Buffer, Language, language_settings::InlayHintKind};
use project::{CompletionIntent, InlayHint, InlayHintLabel, InlayId, Project, Worktree};
use prompt_store::PromptStore;
use rope::Point;
use settings::Settings;
use std::{cell::RefCell, fmt::Write, rc::Rc, sync::Arc};
use theme::ThemeSettings;
use ui::{ButtonLike, ButtonStyle, ContextMenu, Disclosure, ElevationIndex, prelude::*};
use util::{ResultExt, debug_panic};
use workspace::{CollaboratorId, Workspace};
use zed_actions::agent::{Chat, PasteRaw};

enum UserSlashCommands {
    Cached {
        commands: collections::HashMap<String, user_slash_command::UserSlashCommand>,
        errors: Vec<user_slash_command::CommandLoadError>,
    },
    FromFs {
        fs: Arc<dyn fs::Fs>,
        worktree_roots: Vec<std::path::PathBuf>,
    },
}

pub struct MessageEditor {
    mention_set: Entity<MentionSet>,
    editor: Entity<Editor>,
    workspace: WeakEntity<Workspace>,
    prompt_capabilities: Rc<RefCell<acp::PromptCapabilities>>,
    available_commands: Rc<RefCell<Vec<acp::AvailableCommand>>>,
    cached_user_commands: Rc<RefCell<collections::HashMap<String, UserSlashCommand>>>,
    cached_user_command_errors: Rc<RefCell<Vec<CommandLoadError>>>,
    agent_name: SharedString,
    thread_store: Option<Entity<ThreadStore>>,
    _subscriptions: Vec<Subscription>,
    _parse_slash_command_task: Task<()>,
}

#[derive(Clone, Copy, Debug)]
pub enum MessageEditorEvent {
    Send,
    SendImmediately,
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
            if self.read(cx).thread_store.is_some() {
                supported.push(PromptContextType::Thread);
            }
            supported.extend(&[
                PromptContextType::Diagnostics,
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
                source: crate::completion_provider::CommandSource::Server,
            })
            .collect()
    }

    fn confirm_command(&self, cx: &mut App) {
        self.update(cx, |this, cx| this.send(cx));
    }

    fn cached_user_commands(
        &self,
        cx: &App,
    ) -> Option<collections::HashMap<String, UserSlashCommand>> {
        let commands = self.read(cx).cached_user_commands.borrow();
        if commands.is_empty() {
            None
        } else {
            Some(commands.clone())
        }
    }

    fn cached_user_command_errors(&self, cx: &App) -> Option<Vec<CommandLoadError>> {
        let errors = self.read(cx).cached_user_command_errors.borrow();
        if errors.is_empty() {
            None
        } else {
            Some(errors.clone())
        }
    }
}

impl MessageEditor {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        project: WeakEntity<Project>,
        thread_store: Option<Entity<ThreadStore>>,
        history: WeakEntity<AcpThreadHistory>,
        prompt_store: Option<Entity<PromptStore>>,
        prompt_capabilities: Rc<RefCell<acp::PromptCapabilities>>,
        available_commands: Rc<RefCell<Vec<acp::AvailableCommand>>>,
        agent_name: SharedString,
        placeholder: &str,
        mode: EditorMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let cached_user_commands = Rc::new(RefCell::new(collections::HashMap::default()));
        let cached_user_command_errors = Rc::new(RefCell::new(Vec::new()));
        Self::new_with_cache(
            workspace,
            project,
            thread_store,
            history,
            prompt_store,
            prompt_capabilities,
            available_commands,
            cached_user_commands,
            cached_user_command_errors,
            agent_name,
            placeholder,
            mode,
            window,
            cx,
        )
    }

    pub fn new_with_cache(
        workspace: WeakEntity<Workspace>,
        project: WeakEntity<Project>,
        thread_store: Option<Entity<ThreadStore>>,
        history: WeakEntity<AcpThreadHistory>,
        prompt_store: Option<Entity<PromptStore>>,
        prompt_capabilities: Rc<RefCell<acp::PromptCapabilities>>,
        available_commands: Rc<RefCell<Vec<acp::AvailableCommand>>>,
        cached_user_commands: Rc<RefCell<collections::HashMap<String, UserSlashCommand>>>,
        cached_user_command_errors: Rc<RefCell<Vec<CommandLoadError>>>,
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
            editor.set_show_completions_on_input(Some(true));
            editor.set_soft_wrap();
            editor.set_use_modal_editing(true);
            editor.set_context_menu_options(ContextMenuOptions {
                min_entries_visible: 12,
                max_entries_visible: 12,
                placement: Some(ContextMenuPlacement::Above),
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
                }))
            });

            editor
        });
        let mention_set =
            cx.new(|_cx| MentionSet::new(project, thread_store.clone(), prompt_store.clone()));
        let completion_provider = Rc::new(PromptCompletionProvider::new(
            cx.entity(),
            editor.downgrade(),
            mention_set.clone(),
            history,
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
            mention_set,
            workspace,
            prompt_capabilities,
            available_commands,
            cached_user_commands,
            cached_user_command_errors,
            agent_name,
            thread_store,
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
        thread: AgentSessionInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.thread_store.is_none() {
            return;
        }
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let thread_title = thread
            .title
            .clone()
            .filter(|title| !title.is_empty())
            .unwrap_or_else(|| SharedString::new_static("New Thread"));
        let uri = MentionUri::Thread {
            id: thread.session_id,
            name: thread_title.to_string(),
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
        self.contents_with_cache(full_mention_content, None, None, cx)
    }

    pub fn contents_with_cache(
        &self,
        full_mention_content: bool,
        cached_user_commands: Option<
            collections::HashMap<String, user_slash_command::UserSlashCommand>,
        >,
        cached_user_command_errors: Option<Vec<user_slash_command::CommandLoadError>>,
        cx: &mut Context<Self>,
    ) -> Task<Result<(Vec<acp::ContentBlock>, Vec<Entity<Buffer>>)>> {
        let text = self.editor.read(cx).text(cx);
        let available_commands = self.available_commands.borrow().clone();
        let agent_name = self.agent_name.clone();

        let user_slash_commands = if !cx.has_flag::<UserSlashCommandsFeatureFlag>() {
            UserSlashCommands::Cached {
                commands: collections::HashMap::default(),
                errors: Vec::new(),
            }
        } else if let Some(cached) = cached_user_commands {
            UserSlashCommands::Cached {
                commands: cached,
                errors: cached_user_command_errors.unwrap_or_default(),
            }
        } else if let Some(workspace) = self.workspace.upgrade() {
            let fs = workspace.read(cx).project().read(cx).fs().clone();
            let worktree_roots: Vec<std::path::PathBuf> = workspace
                .read(cx)
                .visible_worktrees(cx)
                .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
                .collect();
            UserSlashCommands::FromFs { fs, worktree_roots }
        } else {
            UserSlashCommands::Cached {
                commands: collections::HashMap::default(),
                errors: Vec::new(),
            }
        };

        let contents = self
            .mention_set
            .update(cx, |store, cx| store.contents(full_mention_content, cx));
        let editor = self.editor.clone();
        let supports_embedded_context = self.prompt_capabilities.borrow().embedded_context;

        cx.spawn(async move |_, cx| {
            let (mut user_commands, mut user_command_errors) = match user_slash_commands {
                UserSlashCommands::Cached { commands, errors } => (commands, errors),
                UserSlashCommands::FromFs { fs, worktree_roots } => {
                    let load_result =
                        user_slash_command::load_all_commands_async(&fs, &worktree_roots).await;

                    (
                        user_slash_command::commands_to_map(&load_result.commands),
                        load_result.errors,
                    )
                }
            };

            let server_command_names = available_commands
                .iter()
                .map(|command| command.name.clone())
                .collect::<HashSet<_>>();
            user_slash_command::apply_server_command_conflicts_to_map(
                &mut user_commands,
                &mut user_command_errors,
                &server_command_names,
            );

            // Check if the user is trying to use an errored slash command.
            // If so, report the error to the user.
            if let Some(parsed) = user_slash_command::try_parse_user_command(&text) {
                for error in &user_command_errors {
                    if let Some(error_cmd_name) = error.command_name() {
                        if error_cmd_name == parsed.name {
                            return Err(anyhow::anyhow!(
                                "Failed to load /{}: {}",
                                parsed.name,
                                error.message
                            ));
                        }
                    }
                }
            }
            // Errors for commands that don't match the user's input are silently ignored here,
            // since the user will see them via the error callout in the thread view.

            // Check if this is a user-defined slash command and expand it
            match user_slash_command::try_expand_from_commands(&text, &user_commands) {
                Ok(Some(expanded)) => return Ok((vec![expanded.into()], Vec::new())),
                Err(err) => return Err(err),
                Ok(None) => {} // Not a user command, continue with normal processing
            }

            if let Err(err) = Self::validate_slash_commands(&text, &available_commands, &agent_name)
            {
                return Err(err);
            }

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
                            Mention::Image(mention_image) => acp::ContentBlock::Image(
                                acp::ImageContent::new(
                                    mention_image.data.clone(),
                                    mention_image.format.mime_type(),
                                )
                                .uri(match uri {
                                    MentionUri::File { .. } => Some(uri.to_uri().to_string()),
                                    MentionUri::PastedImage => None,
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
                        if offset.0 >= prefix.len() {
                            let start_offset = MultiBufferOffset(offset.0 - prefix.len());
                            let buffer_snapshot = snapshot.buffer_snapshot();
                            let text = buffer_snapshot
                                .text_for_range(start_offset..offset)
                                .collect::<String>();
                            text == prefix
                        } else {
                            false
                        }
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
        let editor_clipboard_selections = cx
            .read_from_clipboard()
            .and_then(|item| item.entries().first().cloned())
            .and_then(|entry| match entry {
                ClipboardEntry::String(text) => {
                    text.metadata_json::<Vec<editor::ClipboardSelection>>()
                }
                _ => None,
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
            let insertion_target = self
                .editor
                .read(cx)
                .selections
                .newest_anchor()
                .start
                .text_anchor;

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
                    let (excerpt_id, text_anchor, content_len) =
                        self.editor.update(cx, |editor, cx| {
                            let buffer = editor.buffer().read(cx);
                            let snapshot = buffer.snapshot(cx);
                            let (excerpt_id, _, buffer_snapshot) = snapshot.as_singleton().unwrap();
                            let text_anchor = insertion_target.bias_left(&buffer_snapshot);

                            editor.insert(&mention_text, window, cx);
                            editor.insert(" ", window, cx);

                            (*excerpt_id, text_anchor, mention_text.len())
                        });

                    let Some((crease_id, tx)) = insert_crease_for_mention(
                        excerpt_id,
                        text_anchor,
                        content_len,
                        crease_text.into(),
                        mention_uri.icon_path(cx),
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

        if self.prompt_capabilities.borrow().image
            && let Some(task) =
                paste_images_as_context(self.editor.clone(), self.mention_set.clone(), window, cx)
        {
            task.detach();
        }
    }

    fn paste_raw(&mut self, _: &PasteRaw, window: &mut Window, cx: &mut Context<Self>) {
        let editor = self.editor.clone();
        window.defer(cx, move |window, cx| {
            editor.update(cx, |editor, cx| editor.paste(&Paste, window, cx));
        });
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
        let path_style = project.read(cx).path_style(cx);
        let buffer = self.editor.read(cx).buffer().clone();
        let Some(buffer) = buffer.read(cx).as_singleton() else {
            return;
        };
        let mut tasks = Vec::new();
        for path in paths {
            let Some(entry) = project.read(cx).entry_for_path(&path, cx) else {
                continue;
            };
            let Some(worktree) = project.read(cx).worktree_for_id(path.worktree_id, cx) else {
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

    /// Inserts code snippets as creases into the editor.
    /// Each tuple contains (code_text, crease_title).
    pub fn insert_code_creases(
        &mut self,
        creases: Vec<(String, String)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        use editor::display_map::{Crease, FoldPlaceholder};
        use multi_buffer::MultiBufferRow;
        use rope::Point;

        self.editor.update(cx, |editor, cx| {
            editor.insert("\n", window, cx);
            for (text, crease_title) in creases {
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

                editor.insert("\n", window, cx);

                let fold_placeholder = FoldPlaceholder {
                    render: Arc::new({
                        let title = crease_title.clone();
                        move |_fold_id, _fold_range, _cx| {
                            ButtonLike::new("code-crease")
                                .style(ButtonStyle::Filled)
                                .layer(ElevationIndex::ElevatedSurface)
                                .child(Icon::new(IconName::TextSnippet))
                                .child(Label::new(title.clone()).single_line())
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
                        Disclosure::new(("code-crease-toggle", row.0 as u64), !is_folded)
                            .toggle_state(is_folded)
                            .on_click(move |_e, window, cx| fold(!is_folded, window, cx))
                            .into_any_element()
                    },
                    |_, _, _, _| gpui::Empty.into_any(),
                );
                editor.insert_creases(vec![crease], cx);
                editor.fold_at(start_row, window, cx);
            }
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

    pub fn add_images_from_picker(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.prompt_capabilities.borrow().image {
            return;
        }

        let editor = self.editor.clone();
        let mention_set = self.mention_set.clone();

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

                let supported_formats = [
                    ("png", gpui::ImageFormat::Png),
                    ("jpg", gpui::ImageFormat::Jpeg),
                    ("jpeg", gpui::ImageFormat::Jpeg),
                    ("webp", gpui::ImageFormat::Webp),
                    ("gif", gpui::ImageFormat::Gif),
                    ("bmp", gpui::ImageFormat::Bmp),
                    ("tiff", gpui::ImageFormat::Tiff),
                    ("tif", gpui::ImageFormat::Tiff),
                    ("ico", gpui::ImageFormat::Ico),
                ];

                let mut images = Vec::new();
                for path in paths {
                    let extension = path
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|s| s.to_lowercase());

                    let Some(format) = extension.and_then(|ext| {
                        supported_formats
                            .iter()
                            .find(|(e, _)| *e == ext)
                            .map(|(_, f)| *f)
                    }) else {
                        continue;
                    };

                    let Ok(content) = async_fs::read(&path).await else {
                        continue;
                    };

                    images.push(gpui::Image::from_bytes(format, content));
                }

                crate::mention_set::insert_images_as_context(images, editor, mention_set, cx).await;
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
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        self.clear(window, cx);

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
            .on_action(cx.listener(Self::send_immediately))
            .on_action(cx.listener(Self::chat_with_follow))
            .on_action(cx.listener(Self::cancel))
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

    use acp_thread::{AgentSessionInfo, MentionUri};
    use agent::{ThreadStore, outline};
    use agent_client_protocol as acp;
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
    use crate::completion_provider::{PromptCompletionProviderDelegate, PromptContextType};

    #[gpui::test]
    async fn test_at_mention_removal(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/project", json!({"file": ""})).await;
        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let thread_store = None;
        let history = cx
            .update(|window, cx| cx.new(|cx| crate::acp::AcpThreadHistory::new(None, window, cx)));

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                MessageEditor::new_with_cache(
                    workspace.downgrade(),
                    project.downgrade(),
                    thread_store.clone(),
                    history.downgrade(),
                    None,
                    Default::default(),
                    Default::default(),
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
            .update(cx, |message_editor, cx| {
                message_editor.contents_with_cache(false, None, None, cx)
            })
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
        let prompt_capabilities = Rc::new(RefCell::new(acp::PromptCapabilities::default()));
        // Start with no available commands - simulating Claude which doesn't support slash commands
        let available_commands = Rc::new(RefCell::new(vec![]));

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let history = cx
            .update(|window, cx| cx.new(|cx| crate::acp::AcpThreadHistory::new(None, window, cx)));
        let workspace_handle = workspace.downgrade();
        let message_editor = workspace.update_in(cx, |_, window, cx| {
            cx.new(|cx| {
                MessageEditor::new_with_cache(
                    workspace_handle.clone(),
                    project.downgrade(),
                    thread_store.clone(),
                    history.downgrade(),
                    None,
                    prompt_capabilities.clone(),
                    available_commands.clone(),
                    Default::default(),
                    Default::default(),
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
            .update(cx, |message_editor, cx| {
                message_editor.contents_with_cache(false, None, None, cx)
            })
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
            .update(cx, |message_editor, cx| {
                message_editor.contents_with_cache(false, None, None, cx)
            })
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
            .update(cx, |message_editor, cx| {
                message_editor.contents_with_cache(false, None, None, cx)
            })
            .await;

        // Should succeed because /help is in available_commands
        assert!(contents_result.is_ok());

        // Test that regular text works fine
        editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello Claude!", window, cx);
        });

        let (content, _) = message_editor
            .update(cx, |message_editor, cx| {
                message_editor.contents_with_cache(false, None, None, cx)
            })
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
            .update(cx, |message_editor, cx| {
                message_editor.contents_with_cache(false, None, None, cx)
            })
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

        let thread_store = None;
        let history = cx
            .update(|window, cx| cx.new(|cx| crate::acp::AcpThreadHistory::new(None, window, cx)));
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
                MessageEditor::new_with_cache(
                    workspace_handle,
                    project.downgrade(),
                    thread_store.clone(),
                    history.downgrade(),
                    None,
                    prompt_capabilities.clone(),
                    available_commands.clone(),
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

        let thread_store = cx.new(|cx| ThreadStore::new(cx));
        let history = cx
            .update(|window, cx| cx.new(|cx| crate::acp::AcpThreadHistory::new(None, window, cx)));
        let prompt_capabilities = Rc::new(RefCell::new(acp::PromptCapabilities::default()));

        let (message_editor, editor) = workspace.update_in(&mut cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            let message_editor = cx.new(|cx| {
                MessageEditor::new_with_cache(
                    workspace_handle,
                    project.downgrade(),
                    Some(thread_store),
                    history.downgrade(),
                    None,
                    prompt_capabilities.clone(),
                    Default::default(),
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

        let thread_store = Some(cx.new(|cx| ThreadStore::new(cx)));
        let history = cx
            .update(|window, cx| cx.new(|cx| crate::acp::AcpThreadHistory::new(None, window, cx)));

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                let editor = MessageEditor::new_with_cache(
                    workspace.downgrade(),
                    project.downgrade(),
                    thread_store.clone(),
                    history.downgrade(),
                    None,
                    Default::default(),
                    Default::default(),
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

        let thread_store = Some(cx.new(|cx| ThreadStore::new(cx)));
        let history = cx
            .update(|window, cx| cx.new(|cx| crate::acp::AcpThreadHistory::new(None, window, cx)));

        // Create a thread metadata to insert as summary
        let thread_metadata = AgentSessionInfo {
            session_id: acp::SessionId::new("thread-123"),
            cwd: None,
            title: Some("Previous Conversation".into()),
            updated_at: Some(chrono::Utc::now()),
            meta: None,
        };

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                let mut editor = MessageEditor::new_with_cache(
                    workspace.downgrade(),
                    project.downgrade(),
                    thread_store.clone(),
                    history.downgrade(),
                    None,
                    Default::default(),
                    Default::default(),
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
            id: thread_metadata.session_id.clone(),
            name: thread_metadata.title.as_ref().unwrap().to_string(),
        };
        let expected_title = thread_metadata.title.as_ref().unwrap();
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

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let thread_store = None;
        let history = cx
            .update(|window, cx| cx.new(|cx| crate::acp::AcpThreadHistory::new(None, window, cx)));

        let thread_metadata = AgentSessionInfo {
            session_id: acp::SessionId::new("thread-123"),
            cwd: None,
            title: Some("Previous Conversation".into()),
            updated_at: Some(chrono::Utc::now()),
            meta: None,
        };

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                let mut editor = MessageEditor::new_with_cache(
                    workspace.downgrade(),
                    project.downgrade(),
                    thread_store.clone(),
                    history.downgrade(),
                    None,
                    Default::default(),
                    Default::default(),
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
                editor.insert_thread_summary(thread_metadata, window, cx);
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

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let thread_store = None;
        let history = cx
            .update(|window, cx| cx.new(|cx| crate::acp::AcpThreadHistory::new(None, window, cx)));

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                MessageEditor::new_with_cache(
                    workspace.downgrade(),
                    project.downgrade(),
                    thread_store.clone(),
                    history.downgrade(),
                    None,
                    Default::default(),
                    Default::default(),
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

        message_editor.update(cx, |editor, _cx| {
            editor
                .prompt_capabilities
                .replace(acp::PromptCapabilities::new().embedded_context(true));
        });

        let supported_modes = {
            let app = cx.app.borrow();
            message_editor.supported_modes(&app)
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

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let thread_store = Some(cx.new(|cx| ThreadStore::new(cx)));
        let history = cx
            .update(|window, cx| cx.new(|cx| crate::acp::AcpThreadHistory::new(None, window, cx)));

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                MessageEditor::new_with_cache(
                    workspace.downgrade(),
                    project.downgrade(),
                    thread_store.clone(),
                    history.downgrade(),
                    None,
                    Default::default(),
                    Default::default(),
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

        message_editor.update(cx, |editor, _cx| {
            editor
                .prompt_capabilities
                .replace(acp::PromptCapabilities::new().embedded_context(true));
        });

        let supported_modes = {
            let app = cx.app.borrow();
            message_editor.supported_modes(&app)
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

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let thread_store = Some(cx.new(|cx| ThreadStore::new(cx)));
        let history = cx
            .update(|window, cx| cx.new(|cx| crate::acp::AcpThreadHistory::new(None, window, cx)));

        let message_editor = cx.update(|window, cx| {
            cx.new(|cx| {
                MessageEditor::new_with_cache(
                    workspace.downgrade(),
                    project.downgrade(),
                    thread_store.clone(),
                    history.downgrade(),
                    None,
                    Default::default(),
                    Default::default(),
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
            .update(cx, |message_editor, cx| {
                message_editor.contents_with_cache(false, None, None, cx)
            })
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

        let thread_store = Some(cx.new(|cx| ThreadStore::new(cx)));
        let history = cx
            .update(|window, cx| cx.new(|cx| crate::acp::AcpThreadHistory::new(None, window, cx)));

        let (message_editor, editor) = workspace.update_in(cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            let message_editor = cx.new(|cx| {
                MessageEditor::new_with_cache(
                    workspace_handle,
                    project.downgrade(),
                    thread_store.clone(),
                    history.downgrade(),
                    None,
                    Default::default(),
                    Default::default(),
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
            .update(cx, |editor, cx| {
                editor.contents_with_cache(false, None, None, cx)
            })
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
            .update(cx, |editor, cx| {
                editor.contents_with_cache(false, None, None, cx)
            })
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

        let thread_store = Some(cx.new(|cx| ThreadStore::new(cx)));
        let history = cx
            .update(|window, cx| cx.new(|cx| crate::acp::AcpThreadHistory::new(None, window, cx)));

        // Create a new `MessageEditor`. The `EditorMode::full()` has to be used
        // to ensure we have a fixed viewport, so we can eventually actually
        // place the cursor outside of the visible area.
        let message_editor = workspace.update_in(&mut cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            let message_editor = cx.new(|cx| {
                MessageEditor::new_with_cache(
                    workspace_handle,
                    project.downgrade(),
                    thread_store.clone(),
                    history.downgrade(),
                    None,
                    Default::default(),
                    Default::default(),
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
