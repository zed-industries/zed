use crate::{
    acp::completion_provider::{ContextPickerCompletionProvider, SlashCommandCompletion},
    context_picker::{ContextPickerAction, fetch_context_picker::fetch_url_content},
};
use acp_thread::{MentionUri, selection_name};
use agent_client_protocol as acp;
use agent_servers::{AgentServer, AgentServerDelegate};
use agent2::HistoryStore;
use anyhow::{Result, anyhow};
use assistant_slash_commands::codeblock_fence_for_path;
use collections::{HashMap, HashSet};
use editor::{
    Addon, Anchor, AnchorRangeExt, ContextMenuOptions, ContextMenuPlacement, Editor, EditorElement,
    EditorEvent, EditorMode, EditorSnapshot, EditorStyle, ExcerptId, FoldPlaceholder, InlayId,
    MultiBuffer, ToOffset,
    actions::Paste,
    display_map::{Crease, CreaseId, FoldId, Inlay},
};
use futures::{
    FutureExt as _,
    future::{Shared, join_all},
};
use gpui::{
    Animation, AnimationExt as _, AppContext, ClipboardEntry, Context, Entity, EntityId,
    EventEmitter, FocusHandle, Focusable, Image, ImageFormat, Img, KeyContext, SharedString,
    Subscription, Task, TextStyle, WeakEntity, pulsating_between,
};
use language::{Buffer, Language, language_settings::InlayHintKind};
use language_model::LanguageModelImage;
use postage::stream::Stream as _;
use project::{
    CompletionIntent, InlayHint, InlayHintLabel, Project, ProjectItem, ProjectPath, Worktree,
};
use prompt_store::{PromptId, PromptStore};
use rope::Point;
use settings::Settings;
use std::{
    cell::{Cell, RefCell},
    ffi::OsStr,
    fmt::Write,
    ops::{Range, RangeInclusive},
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Duration,
};
use text::OffsetRangeExt;
use theme::ThemeSettings;
use ui::{
    ActiveTheme, AnyElement, App, ButtonCommon, ButtonLike, ButtonStyle, Color, Element as _,
    FluentBuilder as _, Icon, IconName, IconSize, InteractiveElement, IntoElement, Label,
    LabelCommon, LabelSize, ParentElement, Render, SelectableButton, Styled, TextSize, TintColor,
    Toggleable, Window, div, h_flex,
};
use util::{ResultExt, debug_panic};
use workspace::{Workspace, notifications::NotifyResultExt as _};
use zed_actions::agent::Chat;

pub struct MessageEditor {
    mention_set: MentionSet,
    editor: Entity<Editor>,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    history_store: Entity<HistoryStore>,
    prompt_store: Option<Entity<PromptStore>>,
    prompt_capabilities: Rc<Cell<acp::PromptCapabilities>>,
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

const COMMAND_HINT_INLAY_ID: usize = 0;

impl MessageEditor {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        history_store: Entity<HistoryStore>,
        prompt_store: Option<Entity<PromptStore>>,
        prompt_capabilities: Rc<Cell<acp::PromptCapabilities>>,
        available_commands: Rc<RefCell<Vec<acp::AvailableCommand>>>,
        agent_name: SharedString,
        placeholder: impl Into<Arc<str>>,
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
        let completion_provider = Rc::new(ContextPickerCompletionProvider::new(
            cx.weak_entity(),
            workspace.clone(),
            history_store.clone(),
            prompt_store.clone(),
            prompt_capabilities.clone(),
            available_commands.clone(),
        ));
        let mention_set = MentionSet::default();
        let editor = cx.new(|cx| {
            let buffer = cx.new(|cx| Buffer::local("", cx).with_language(Arc::new(language), cx));
            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

            let mut editor = Editor::new(mode, buffer, None, window, cx);
            editor.set_placeholder_text(placeholder, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_soft_wrap();
            editor.set_use_modal_editing(true);
            editor.set_completion_provider(Some(completion_provider.clone()));
            editor.set_context_menu_options(ContextMenuOptions {
                min_entries_visible: 12,
                max_entries_visible: 12,
                placement: Some(ContextMenuPlacement::Above),
            });
            editor.register_addon(MessageEditorAddon::new());
            editor
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
                if let EditorEvent::Edited { .. } = event {
                    let snapshot = editor.update(cx, |editor, cx| {
                        let new_hints = this
                            .command_hint(editor.buffer(), cx)
                            .into_iter()
                            .collect::<Vec<_>>();
                        let has_new_hint = !new_hints.is_empty();
                        editor.splice_inlays(
                            if has_hint {
                                &[InlayId::Hint(COMMAND_HINT_INLAY_ID)]
                            } else {
                                &[]
                            },
                            new_hints,
                            cx,
                        );
                        has_hint = has_new_hint;

                        editor.snapshot(window, cx)
                    });
                    this.mention_set.remove_invalid(snapshot);

                    cx.notify();
                }
            }
        }));

        Self {
            editor,
            project,
            mention_set,
            workspace,
            history_store,
            prompt_store,
            prompt_capabilities,
            available_commands,
            agent_name,
            _subscriptions: subscriptions,
            _parse_slash_command_task: Task::ready(()),
        }
    }

    fn command_hint(&self, buffer: &Entity<MultiBuffer>, cx: &App) -> Option<Inlay> {
        let available_commands = self.available_commands.borrow();
        if available_commands.is_empty() {
            return None;
        }

        let snapshot = buffer.read(cx).snapshot(cx);
        let parsed_command = SlashCommandCompletion::try_parse(&snapshot.text(), 0)?;
        if parsed_command.argument.is_some() {
            return None;
        }

        let command_name = parsed_command.command?;
        let available_command = available_commands
            .iter()
            .find(|command| command.name == command_name)?;

        let acp::AvailableCommandInput::Unstructured { mut hint } =
            available_command.input.clone()?;

        let mut hint_pos = parsed_command.source_range.end + 1;
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
        thread: agent2::DbThreadMetadata,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let start = self.editor.update(cx, |editor, cx| {
            editor.set_text(format!("{}\n", thread.title), window, cx);
            editor
                .buffer()
                .read(cx)
                .snapshot(cx)
                .anchor_before(Point::zero())
                .text_anchor
        });

        self.confirm_mention_completion(
            thread.title.clone(),
            start,
            thread.title.len(),
            MentionUri::Thread {
                id: thread.id.clone(),
                name: thread.title.to_string(),
            },
            window,
            cx,
        )
        .detach();
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

    pub fn mentions(&self) -> HashSet<MentionUri> {
        self.mention_set
            .mentions
            .values()
            .map(|(uri, _)| uri.clone())
            .collect()
    }

    pub fn confirm_mention_completion(
        &mut self,
        crease_text: SharedString,
        start: text::Anchor,
        content_len: usize,
        mention_uri: MentionUri,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        let snapshot = self
            .editor
            .update(cx, |editor, cx| editor.snapshot(window, cx));
        let Some((excerpt_id, _, _)) = snapshot.buffer_snapshot.as_singleton() else {
            return Task::ready(());
        };
        let Some(start_anchor) = snapshot
            .buffer_snapshot
            .anchor_in_excerpt(*excerpt_id, start)
        else {
            return Task::ready(());
        };
        let end_anchor = snapshot
            .buffer_snapshot
            .anchor_before(start_anchor.to_offset(&snapshot.buffer_snapshot) + content_len + 1);

        let crease = if let MentionUri::File { abs_path } = &mention_uri
            && let Some(extension) = abs_path.extension()
            && let Some(extension) = extension.to_str()
            && Img::extensions().contains(&extension)
            && !extension.contains("svg")
        {
            let Some(project_path) = self
                .project
                .read(cx)
                .project_path_for_absolute_path(&abs_path, cx)
            else {
                log::error!("project path not found");
                return Task::ready(());
            };
            let image = self
                .project
                .update(cx, |project, cx| project.open_image(project_path, cx));
            let image = cx
                .spawn(async move |_, cx| {
                    let image = image.await.map_err(|e| e.to_string())?;
                    let image = image
                        .update(cx, |image, _| image.image.clone())
                        .map_err(|e| e.to_string())?;
                    Ok(image)
                })
                .shared();
            insert_crease_for_mention(
                *excerpt_id,
                start,
                content_len,
                mention_uri.name().into(),
                IconName::Image.path().into(),
                Some(image),
                self.editor.clone(),
                window,
                cx,
            )
        } else {
            insert_crease_for_mention(
                *excerpt_id,
                start,
                content_len,
                crease_text,
                mention_uri.icon_path(cx),
                None,
                self.editor.clone(),
                window,
                cx,
            )
        };
        let Some((crease_id, tx)) = crease else {
            return Task::ready(());
        };

        let task = match mention_uri.clone() {
            MentionUri::Fetch { url } => self.confirm_mention_for_fetch(url, cx),
            MentionUri::Directory { abs_path } => self.confirm_mention_for_directory(abs_path, cx),
            MentionUri::Thread { id, .. } => self.confirm_mention_for_thread(id, cx),
            MentionUri::TextThread { path, .. } => self.confirm_mention_for_text_thread(path, cx),
            MentionUri::File { abs_path } => self.confirm_mention_for_file(abs_path, cx),
            MentionUri::Symbol {
                abs_path,
                line_range,
                ..
            } => self.confirm_mention_for_symbol(abs_path, line_range, cx),
            MentionUri::Rule { id, .. } => self.confirm_mention_for_rule(id, cx),
            MentionUri::PastedImage => {
                debug_panic!("pasted image URI should not be included in completions");
                Task::ready(Err(anyhow!(
                    "pasted imaged URI should not be included in completions"
                )))
            }
            MentionUri::Selection { .. } => {
                // Handled elsewhere
                debug_panic!("unexpected selection URI");
                Task::ready(Err(anyhow!("unexpected selection URI")))
            }
        };
        let task = cx
            .spawn(async move |_, _| task.await.map_err(|e| e.to_string()))
            .shared();
        self.mention_set
            .mentions
            .insert(crease_id, (mention_uri, task.clone()));

        // Notify the user if we failed to load the mentioned context
        cx.spawn_in(window, async move |this, cx| {
            let result = task.await.notify_async_err(cx);
            drop(tx);
            if result.is_none() {
                this.update(cx, |this, cx| {
                    this.editor.update(cx, |editor, cx| {
                        // Remove mention
                        editor.edit([(start_anchor..end_anchor, "")], cx);
                    });
                    this.mention_set.mentions.remove(&crease_id);
                })
                .ok();
            }
        })
    }

    fn confirm_mention_for_file(
        &mut self,
        abs_path: PathBuf,
        cx: &mut Context<Self>,
    ) -> Task<Result<Mention>> {
        let Some(project_path) = self
            .project
            .read(cx)
            .project_path_for_absolute_path(&abs_path, cx)
        else {
            return Task::ready(Err(anyhow!("project path not found")));
        };
        let extension = abs_path
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_default();

        if Img::extensions().contains(&extension) && !extension.contains("svg") {
            if !self.prompt_capabilities.get().image {
                return Task::ready(Err(anyhow!("This model does not support images yet")));
            }
            let task = self
                .project
                .update(cx, |project, cx| project.open_image(project_path, cx));
            return cx.spawn(async move |_, cx| {
                let image = task.await?;
                let image = image.update(cx, |image, _| image.image.clone())?;
                let format = image.format;
                let image = cx
                    .update(|cx| LanguageModelImage::from_image(image, cx))?
                    .await;
                if let Some(image) = image {
                    Ok(Mention::Image(MentionImage {
                        data: image.source,
                        format,
                    }))
                } else {
                    Err(anyhow!("Failed to convert image"))
                }
            });
        }

        let buffer = self
            .project
            .update(cx, |project, cx| project.open_buffer(project_path, cx));
        cx.spawn(async move |_, cx| {
            let buffer = buffer.await?;
            let mention = buffer.update(cx, |buffer, cx| Mention::Text {
                content: buffer.text(),
                tracked_buffers: vec![cx.entity()],
            })?;
            anyhow::Ok(mention)
        })
    }

    fn confirm_mention_for_directory(
        &mut self,
        abs_path: PathBuf,
        cx: &mut Context<Self>,
    ) -> Task<Result<Mention>> {
        fn collect_files_in_path(worktree: &Worktree, path: &Path) -> Vec<(Arc<Path>, PathBuf)> {
            let mut files = Vec::new();

            for entry in worktree.child_entries(path) {
                if entry.is_dir() {
                    files.extend(collect_files_in_path(worktree, &entry.path));
                } else if entry.is_file() {
                    files.push((entry.path.clone(), worktree.full_path(&entry.path)));
                }
            }

            files
        }

        let Some(project_path) = self
            .project
            .read(cx)
            .project_path_for_absolute_path(&abs_path, cx)
        else {
            return Task::ready(Err(anyhow!("project path not found")));
        };
        let Some(entry) = self.project.read(cx).entry_for_path(&project_path, cx) else {
            return Task::ready(Err(anyhow!("project entry not found")));
        };
        let Some(worktree) = self.project.read(cx).worktree_for_entry(entry.id, cx) else {
            return Task::ready(Err(anyhow!("worktree not found")));
        };
        let project = self.project.clone();
        cx.spawn(async move |_, cx| {
            let directory_path = entry.path.clone();

            let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id())?;
            let file_paths = worktree.read_with(cx, |worktree, _cx| {
                collect_files_in_path(worktree, &directory_path)
            })?;
            let descendants_future = cx.update(|cx| {
                join_all(file_paths.into_iter().map(|(worktree_path, full_path)| {
                    let rel_path = worktree_path
                        .strip_prefix(&directory_path)
                        .log_err()
                        .map_or_else(|| worktree_path.clone(), |rel_path| rel_path.into());

                    let open_task = project.update(cx, |project, cx| {
                        project.buffer_store().update(cx, |buffer_store, cx| {
                            let project_path = ProjectPath {
                                worktree_id,
                                path: worktree_path,
                            };
                            buffer_store.open_buffer(project_path, cx)
                        })
                    });

                    // TODO: report load errors instead of just logging
                    let rope_task = cx.spawn(async move |cx| {
                        let buffer = open_task.await.log_err()?;
                        let rope = buffer
                            .read_with(cx, |buffer, _cx| buffer.as_rope().clone())
                            .log_err()?;
                        Some((rope, buffer))
                    });

                    cx.background_spawn(async move {
                        let (rope, buffer) = rope_task.await?;
                        Some((rel_path, full_path, rope.to_string(), buffer))
                    })
                }))
            })?;

            let contents = cx
                .background_spawn(async move {
                    let (contents, tracked_buffers) = descendants_future
                        .await
                        .into_iter()
                        .flatten()
                        .map(|(rel_path, full_path, rope, buffer)| {
                            ((rel_path, full_path, rope), buffer)
                        })
                        .unzip();
                    Mention::Text {
                        content: render_directory_contents(contents),
                        tracked_buffers,
                    }
                })
                .await;
            anyhow::Ok(contents)
        })
    }

    fn confirm_mention_for_fetch(
        &mut self,
        url: url::Url,
        cx: &mut Context<Self>,
    ) -> Task<Result<Mention>> {
        let http_client = match self
            .workspace
            .update(cx, |workspace, _| workspace.client().http_client())
        {
            Ok(http_client) => http_client,
            Err(e) => return Task::ready(Err(e)),
        };
        cx.background_executor().spawn(async move {
            let content = fetch_url_content(http_client, url.to_string()).await?;
            Ok(Mention::Text {
                content,
                tracked_buffers: Vec::new(),
            })
        })
    }

    fn confirm_mention_for_symbol(
        &mut self,
        abs_path: PathBuf,
        line_range: RangeInclusive<u32>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Mention>> {
        let Some(project_path) = self
            .project
            .read(cx)
            .project_path_for_absolute_path(&abs_path, cx)
        else {
            return Task::ready(Err(anyhow!("project path not found")));
        };
        let buffer = self
            .project
            .update(cx, |project, cx| project.open_buffer(project_path, cx));
        cx.spawn(async move |_, cx| {
            let buffer = buffer.await?;
            let mention = buffer.update(cx, |buffer, cx| {
                let start = Point::new(*line_range.start(), 0).min(buffer.max_point());
                let end = Point::new(*line_range.end() + 1, 0).min(buffer.max_point());
                let content = buffer.text_for_range(start..end).collect();
                Mention::Text {
                    content,
                    tracked_buffers: vec![cx.entity()],
                }
            })?;
            anyhow::Ok(mention)
        })
    }

    fn confirm_mention_for_rule(
        &mut self,
        id: PromptId,
        cx: &mut Context<Self>,
    ) -> Task<Result<Mention>> {
        let Some(prompt_store) = self.prompt_store.clone() else {
            return Task::ready(Err(anyhow!("missing prompt store")));
        };
        let prompt = prompt_store.read(cx).load(id, cx);
        cx.spawn(async move |_, _| {
            let prompt = prompt.await?;
            Ok(Mention::Text {
                content: prompt,
                tracked_buffers: Vec::new(),
            })
        })
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

            let abs_path = buffer
                .read(cx)
                .project_path(cx)
                .and_then(|project_path| self.project.read(cx).absolute_path(&project_path, cx));
            let snapshot = buffer.read(cx).snapshot();

            let text = snapshot
                .text_for_range(selection_range.clone())
                .collect::<String>();
            let point_range = selection_range.to_point(&snapshot);
            let line_range = point_range.start.row..=point_range.end.row;

            let uri = MentionUri::Selection {
                abs_path: abs_path.clone(),
                line_range: line_range.clone(),
            };
            let crease = crate::context_picker::crease_for_mention(
                selection_name(abs_path.as_deref(), &line_range).into(),
                uri.icon_path(cx),
                range,
                self.editor.downgrade(),
            );

            let crease_id = self.editor.update(cx, |editor, cx| {
                let crease_ids = editor.insert_creases(vec![crease.clone()], cx);
                editor.fold_creases(vec![crease], false, window, cx);
                crease_ids.first().copied().unwrap()
            });

            self.mention_set.mentions.insert(
                crease_id,
                (
                    uri,
                    Task::ready(Ok(Mention::Text {
                        content: text,
                        tracked_buffers: vec![buffer],
                    }))
                    .shared(),
                ),
            );
        }
    }

    fn confirm_mention_for_thread(
        &mut self,
        id: acp::SessionId,
        cx: &mut Context<Self>,
    ) -> Task<Result<Mention>> {
        let server = Rc::new(agent2::NativeAgentServer::new(
            self.project.read(cx).fs().clone(),
            self.history_store.clone(),
        ));
        let delegate = AgentServerDelegate::new(self.project.clone(), None);
        let connection = server.connect(Path::new(""), delegate, cx);
        cx.spawn(async move |_, cx| {
            let agent = connection.await?;
            let agent = agent.downcast::<agent2::NativeAgentConnection>().unwrap();
            let summary = agent
                .0
                .update(cx, |agent, cx| agent.thread_summary(id, cx))?
                .await?;
            anyhow::Ok(Mention::Text {
                content: summary.to_string(),
                tracked_buffers: Vec::new(),
            })
        })
    }

    fn confirm_mention_for_text_thread(
        &mut self,
        path: PathBuf,
        cx: &mut Context<Self>,
    ) -> Task<Result<Mention>> {
        let context = self.history_store.update(cx, |text_thread_store, cx| {
            text_thread_store.load_text_thread(path.as_path().into(), cx)
        });
        cx.spawn(async move |_, cx| {
            let context = context.await?;
            let xml = context.update(cx, |context, cx| context.to_xml(cx))?;
            Ok(Mention::Text {
                content: xml,
                tracked_buffers: Vec::new(),
            })
        })
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
            .contents(&self.prompt_capabilities.get(), cx);
        let editor = self.editor.clone();

        cx.spawn(async move |_, cx| {
            let contents = contents.await?;
            let mut all_tracked_buffers = Vec::new();

            let result = editor.update(cx, |editor, cx| {
                let mut ix = 0;
                let mut chunks: Vec<acp::ContentBlock> = Vec::new();
                let text = editor.text(cx);
                editor.display_map.update(cx, |map, cx| {
                    let snapshot = map.snapshot(cx);
                    for (crease_id, crease) in snapshot.crease_snapshot.creases() {
                        let Some((uri, mention)) = contents.get(&crease_id) else {
                            continue;
                        };

                        let crease_range = crease.range().to_offset(&snapshot.buffer_snapshot);
                        if crease_range.start > ix {
                            //todo(): Custom slash command ContentBlock?
                            // let chunk = if prevent_slash_commands
                            //     && ix == 0
                            //     && parse_slash_command(&text[ix..]).is_some()
                            // {
                            //     format!(" {}", &text[ix..crease_range.start]).into()
                            // } else {
                            //     text[ix..crease_range.start].into()
                            // };
                            let chunk = text[ix..crease_range.start].into();
                            chunks.push(chunk);
                        }
                        let chunk = match mention {
                            Mention::Text {
                                content,
                                tracked_buffers,
                            } => {
                                all_tracked_buffers.extend(tracked_buffers.iter().cloned());
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
                                let uri = match uri {
                                    MentionUri::File { .. } => Some(uri.to_uri().to_string()),
                                    MentionUri::PastedImage => None,
                                    other => {
                                        debug_panic!(
                                            "unexpected mention uri for image: {:?}",
                                            other
                                        );
                                        None
                                    }
                                };
                                acp::ContentBlock::Image(acp::ImageContent {
                                    annotations: None,
                                    data: mention_image.data.to_string(),
                                    mime_type: mention_image.format.mime_type().into(),
                                    uri,
                                })
                            }
                            Mention::UriOnly => {
                                acp::ContentBlock::ResourceLink(acp::ResourceLink {
                                    name: uri.name(),
                                    uri: uri.to_uri().to_string(),
                                    annotations: None,
                                    description: None,
                                    mime_type: None,
                                    size: None,
                                    title: None,
                                })
                            }
                        };
                        chunks.push(chunk);
                        ix = crease_range.end;
                    }

                    if ix < text.len() {
                        //todo(): Custom slash command ContentBlock?
                        // let last_chunk = if prevent_slash_commands
                        //     && ix == 0
                        //     && parse_slash_command(&text[ix..]).is_some()
                        // {
                        //     format!(" {}", text[ix..].trim_end())
                        // } else {
                        //     text[ix..].trim_end().to_owned()
                        // };
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
                self.mention_set
                    .mentions
                    .drain()
                    .map(|(crease_id, _)| crease_id),
                cx,
            )
        });
    }

    fn send(&mut self, _: &Chat, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_empty(cx) {
            return;
        }
        cx.emit(MessageEditorEvent::Send)
    }

    fn cancel(&mut self, _: &editor::actions::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(MessageEditorEvent::Cancel)
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if !self.prompt_capabilities.get().image {
            return;
        }

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

        let replacement_text = MentionUri::PastedImage.as_link().to_string();
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
            let Some(start_anchor) = multibuffer_anchor else {
                continue;
            };
            let end_anchor = self.editor.update(cx, |editor, cx| {
                let snapshot = editor.buffer().read(cx).snapshot(cx);
                snapshot.anchor_before(start_anchor.to_offset(&snapshot) + content_len)
            });
            let image = Arc::new(image);
            let Some((crease_id, tx)) = insert_crease_for_mention(
                excerpt_id,
                text_anchor,
                content_len,
                MentionUri::PastedImage.name().into(),
                IconName::Image.path().into(),
                Some(Task::ready(Ok(image.clone())).shared()),
                self.editor.clone(),
                window,
                cx,
            ) else {
                continue;
            };
            let task = cx
                .spawn_in(window, {
                    async move |_, cx| {
                        let format = image.format;
                        let image = cx
                            .update(|_, cx| LanguageModelImage::from_image(image, cx))
                            .map_err(|e| e.to_string())?
                            .await;
                        drop(tx);
                        if let Some(image) = image {
                            Ok(Mention::Image(MentionImage {
                                data: image.source,
                                format,
                            }))
                        } else {
                            Err("Failed to convert image".into())
                        }
                    }
                })
                .shared();

            self.mention_set
                .mentions
                .insert(crease_id, (MentionUri::PastedImage, task.clone()));

            cx.spawn_in(window, async move |this, cx| {
                if task.await.notify_async_err(cx).is_none() {
                    this.update(cx, |this, cx| {
                        this.editor.update(cx, |editor, cx| {
                            editor.edit([(start_anchor..end_anchor, "")], cx);
                        });
                        this.mention_set.mentions.remove(&crease_id);
                    })
                    .ok();
                }
            })
            .detach();
        }
    }

    pub fn insert_dragged_files(
        &mut self,
        paths: Vec<project::ProjectPath>,
        added_worktrees: Vec<Entity<Worktree>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let buffer = self.editor.read(cx).buffer().clone();
        let Some(buffer) = buffer.read(cx).as_singleton() else {
            return;
        };
        let mut tasks = Vec::new();
        for path in paths {
            let Some(entry) = self.project.read(cx).entry_for_path(&path, cx) else {
                continue;
            };
            let Some(abs_path) = self.project.read(cx).absolute_path(&path, cx) else {
                continue;
            };
            let path_prefix = abs_path
                .file_name()
                .unwrap_or(path.path.as_os_str())
                .display()
                .to_string();
            let (file_name, _) =
                crate::context_picker::file_context_picker::extract_file_name_and_directory(
                    &path.path,
                    &path_prefix,
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
            tasks.push(self.confirm_mention_completion(
                file_name,
                anchor,
                content_len,
                uri,
                window,
                cx,
            ));
        }
        cx.spawn(async move |_, _| {
            join_all(tasks).await;
            drop(added_worktrees);
        })
        .detach();
    }

    pub fn insert_selections(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let buffer = self.editor.read(cx).buffer().clone();
        let Some(buffer) = buffer.read(cx).as_singleton() else {
            return;
        };
        let anchor = buffer.update(cx, |buffer, _cx| buffer.anchor_before(buffer.len()));
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let Some(completion) = ContextPickerCompletionProvider::completion_for_action(
            ContextPickerAction::AddSelections,
            anchor..anchor,
            cx.weak_entity(),
            &workspace,
            cx,
        ) else {
            return;
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
                    let Some(mention_uri) = MentionUri::parse(&resource.uri).log_err() else {
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
                    if let Some(mention_uri) = MentionUri::parse(&resource.uri).log_err() {
                        let start = text.len();
                        write!(&mut text, "{}", mention_uri.as_link()).ok();
                        let end = text.len();
                        mentions.push((start..end, mention_uri, Mention::UriOnly));
                    }
                }
                acp::ContentBlock::Image(acp::ImageContent {
                    uri,
                    data,
                    mime_type,
                    annotations: _,
                }) => {
                    let mention_uri = if let Some(uri) = uri {
                        MentionUri::parse(&uri)
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
                acp::ContentBlock::Audio(_) | acp::ContentBlock::Resource(_) => {}
            }
        }

        let snapshot = self.editor.update(cx, |editor, cx| {
            editor.set_text(text, window, cx);
            editor.buffer().read(cx).snapshot(cx)
        });

        for (range, mention_uri, mention) in mentions {
            let anchor = snapshot.anchor_before(range.start);
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

            self.mention_set.mentions.insert(
                crease_id,
                (mention_uri.clone(), Task::ready(Ok(mention)).shared()),
            );
        }
        cx.notify();
    }

    pub fn text(&self, cx: &App) -> String {
        self.editor.read(cx).text(cx)
    }

    #[cfg(test)]
    pub fn set_text(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.set_text(text, window, cx);
        });
    }
}

fn render_directory_contents(entries: Vec<(Arc<Path>, PathBuf, String)>) -> String {
    let mut output = String::new();
    for (_relative_path, full_path, content) in entries {
        let fence = codeblock_fence_for_path(Some(&full_path), None);
        write!(output, "\n{fence}\n{content}\n```").unwrap();
    }
    output
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
                        inlay_hints_style: editor::make_inlay_hints_style(cx),
                        ..Default::default()
                    },
                )
            })
    }
}

pub(crate) fn insert_crease_for_mention(
    excerpt_id: ExcerptId,
    anchor: text::Anchor,
    content_len: usize,
    crease_label: SharedString,
    crease_icon: SharedString,
    // abs_path: Option<Arc<Path>>,
    image: Option<Shared<Task<Result<Arc<Image>, String>>>>,
    editor: Entity<Editor>,
    window: &mut Window,
    cx: &mut App,
) -> Option<(CreaseId, postage::barrier::Sender)> {
    let (tx, rx) = postage::barrier::channel();

    let crease_id = editor.update(cx, |editor, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);

        let start = snapshot.anchor_in_excerpt(excerpt_id, anchor)?;

        let start = start.bias_right(&snapshot);
        let end = snapshot.anchor_before(start.to_offset(&snapshot) + content_len);

        let placeholder = FoldPlaceholder {
            render: render_mention_fold_button(
                crease_label,
                crease_icon,
                start..end,
                rx,
                image,
                cx.weak_entity(),
                cx,
            ),
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
    })?;

    Some((crease_id, tx))
}

fn render_mention_fold_button(
    label: SharedString,
    icon: SharedString,
    range: Range<Anchor>,
    mut loading_finished: postage::barrier::Receiver,
    image_task: Option<Shared<Task<Result<Arc<Image>, String>>>>,
    editor: WeakEntity<Editor>,
    cx: &mut App,
) -> Arc<dyn Send + Sync + Fn(FoldId, Range<Anchor>, &mut App) -> AnyElement> {
    let loading = cx.new(|cx| {
        let loading = cx.spawn(async move |this, cx| {
            loading_finished.recv().await;
            this.update(cx, |this: &mut LoadingContext, cx| {
                this.loading = None;
                cx.notify();
            })
            .ok();
        });
        LoadingContext {
            id: cx.entity_id(),
            label,
            icon,
            range,
            editor,
            loading: Some(loading),
            image: image_task.clone(),
        }
    });
    Arc::new(move |_fold_id, _fold_range, _cx| loading.clone().into_any_element())
}

struct LoadingContext {
    id: EntityId,
    label: SharedString,
    icon: SharedString,
    range: Range<Anchor>,
    editor: WeakEntity<Editor>,
    loading: Option<Task<()>>,
    image: Option<Shared<Task<Result<Arc<Image>, String>>>>,
}

impl Render for LoadingContext {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_in_text_selection = self
            .editor
            .update(cx, |editor, cx| editor.is_range_selected(&self.range, cx))
            .unwrap_or_default();
        ButtonLike::new(("loading-context", self.id))
            .style(ButtonStyle::Filled)
            .selected_style(ButtonStyle::Tinted(TintColor::Accent))
            .toggle_state(is_in_text_selection)
            .when_some(self.image.clone(), |el, image_task| {
                el.hoverable_tooltip(move |_, cx| {
                    let image = image_task.peek().cloned().transpose().ok().flatten();
                    let image_task = image_task.clone();
                    cx.new::<ImageHover>(|cx| ImageHover {
                        image,
                        _task: cx.spawn(async move |this, cx| {
                            if let Ok(image) = image_task.clone().await {
                                this.update(cx, |this, cx| {
                                    if this.image.replace(image).is_none() {
                                        cx.notify();
                                    }
                                })
                                .ok();
                            }
                        }),
                    })
                    .into()
                })
            })
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Icon::from_path(self.icon.clone())
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    )
                    .child(
                        Label::new(self.label.clone())
                            .size(LabelSize::Small)
                            .buffer_font(cx)
                            .single_line(),
                    )
                    .map(|el| {
                        if self.loading.is_some() {
                            el.with_animation(
                                "loading-context-crease",
                                Animation::new(Duration::from_secs(2))
                                    .repeat()
                                    .with_easing(pulsating_between(0.4, 0.8)),
                                |label, delta| label.opacity(delta),
                            )
                            .into_any()
                        } else {
                            el.into_any()
                        }
                    }),
            )
    }
}

struct ImageHover {
    image: Option<Arc<Image>>,
    _task: Task<()>,
}

impl Render for ImageHover {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(image) = self.image.clone() {
            gpui::img(image).max_w_96().max_h_96().into_any_element()
        } else {
            gpui::Empty.into_any_element()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Mention {
    Text {
        content: String,
        tracked_buffers: Vec<Entity<Buffer>>,
    },
    Image(MentionImage),
    UriOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MentionImage {
    pub data: SharedString,
    pub format: ImageFormat,
}

#[derive(Default)]
pub struct MentionSet {
    mentions: HashMap<CreaseId, (MentionUri, Shared<Task<Result<Mention, String>>>)>,
}

impl MentionSet {
    fn contents(
        &self,
        prompt_capabilities: &acp::PromptCapabilities,
        cx: &mut App,
    ) -> Task<Result<HashMap<CreaseId, (MentionUri, Mention)>>> {
        if !prompt_capabilities.embedded_context {
            let mentions = self
                .mentions
                .iter()
                .map(|(crease_id, (uri, _))| (*crease_id, (uri.clone(), Mention::UriOnly)))
                .collect();

            return Task::ready(Ok(mentions));
        }

        let mentions = self.mentions.clone();
        cx.spawn(async move |_cx| {
            let mut contents = HashMap::default();
            for (crease_id, (mention_uri, task)) in mentions {
                contents.insert(
                    crease_id,
                    (mention_uri, task.await.map_err(|e| anyhow!("{e}"))?),
                );
            }
            Ok(contents)
        })
    }

    fn remove_invalid(&mut self, snapshot: EditorSnapshot) {
        for (crease_id, crease) in snapshot.crease_snapshot.creases() {
            if !crease.range().start.is_valid(&snapshot.buffer_snapshot) {
                self.mentions.remove(&crease_id);
            }
        }
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
    use std::{
        cell::{Cell, RefCell},
        ops::Range,
        path::Path,
        rc::Rc,
        sync::Arc,
    };

    use acp_thread::MentionUri;
    use agent_client_protocol as acp;
    use agent2::HistoryStore;
    use assistant_context::ContextStore;
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
    use util::{path, uri};
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

        let context_store = cx.new(|cx| ContextStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(context_store, cx));

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

        let (content, _) = message_editor
            .update(cx, |message_editor, cx| message_editor.contents(cx))
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
        let context_store = cx.new(|cx| ContextStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(context_store, cx));
        let prompt_capabilities = Rc::new(Cell::new(acp::PromptCapabilities::default()));
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
            .update(cx, |message_editor, cx| message_editor.contents(cx))
            .await;

        // Should fail because available_commands is empty (no commands supported)
        assert!(contents_result.is_err());
        let error_message = contents_result.unwrap_err().to_string();
        assert!(error_message.contains("not supported by Claude Code"));
        assert!(error_message.contains("Available commands: none"));

        // Now simulate Claude providing its list of available commands (which doesn't include file)
        available_commands.replace(vec![acp::AvailableCommand {
            name: "help".to_string(),
            description: "Get help".to_string(),
            input: None,
        }]);

        // Test that unsupported slash commands trigger an error when we have a list of available commands
        editor.update_in(cx, |editor, window, cx| {
            editor.set_text("/file test.txt", window, cx);
        });

        let contents_result = message_editor
            .update(cx, |message_editor, cx| message_editor.contents(cx))
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
            .update(cx, |message_editor, cx| message_editor.contents(cx))
            .await;

        // Should succeed because /help is in available_commands
        assert!(contents_result.is_ok());

        // Test that regular text works fine
        editor.update_in(cx, |editor, window, cx| {
            editor.set_text("Hello Claude!", window, cx);
        });

        let (content, _) = message_editor
            .update(cx, |message_editor, cx| message_editor.contents(cx))
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
            .update(cx, |message_editor, cx| message_editor.contents(cx))
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
            language::init(cx);
            editor::init(cx);
            workspace::init(app_state.clone(), cx);
            Project::init_settings(cx);
        });

        let project = Project::test(app_state.fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let workspace = window.root(cx).unwrap();

        let mut cx = VisualTestContext::from_window(*window, cx);

        let context_store = cx.new(|cx| ContextStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(context_store, cx));
        let prompt_capabilities = Rc::new(Cell::new(acp::PromptCapabilities::default()));
        let available_commands = Rc::new(RefCell::new(vec![
            acp::AvailableCommand {
                name: "quick-math".to_string(),
                description: "2 + 2 = 4 - 1 = 3".to_string(),
                input: None,
            },
            acp::AvailableCommand {
                name: "say-hello".to_string(),
                description: "Say hello to whoever you want".to_string(),
                input: Some(acp::AvailableCommandInput::Unstructured {
                    hint: "<name>".to_string(),
                }),
            },
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
            assert!(editor.has_visible_completions_menu());

            assert_eq!(
                current_completion_labels_with_documentation(editor),
                &[("say-hello".into(), "Say hello to whoever you want".into())]
            );
        });

        cx.simulate_input("GPT5");

        editor.update_in(&mut cx, |editor, window, cx| {
            assert!(editor.has_visible_completions_menu());
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        cx.run_until_parked();

        editor.update_in(&mut cx, |editor, window, cx| {
            assert_eq!(editor.text(cx), "/say-hello GPT5");
            assert_eq!(editor.display_text(cx), "/say-hello GPT5");
            assert!(!editor.has_visible_completions_menu());

            // Delete argument
            for _ in 0..4 {
                editor.backspace(&editor::actions::Backspace, window, cx);
            }
        });

        cx.run_until_parked();

        editor.update_in(&mut cx, |editor, window, cx| {
            assert_eq!(editor.text(cx), "/say-hello ");
            // Hint is visible because argument was deleted
            assert_eq!(editor.display_text(cx), "/say-hello <name>");

            // Delete last command letter
            editor.backspace(&editor::actions::Backspace, window, cx);
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

        let context_store = cx.new(|cx| ContextStore::fake(project.clone(), cx));
        let history_store = cx.new(|cx| HistoryStore::new(context_store, cx));
        let prompt_capabilities = Rc::new(Cell::new(acp::PromptCapabilities::default()));

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
                    "eight.txt dir/b/",
                    "seven.txt dir/b/",
                    "six.txt dir/b/",
                    "five.txt dir/b/",
                ]
            );
            editor.set_text("", window, cx);
        });

        prompt_capabilities.set(acp::PromptCapabilities {
            image: true,
            audio: true,
            embedded_context: true,
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

        let url_one = uri!("file:///dir/a/one.txt");
        editor.update(&mut cx, |editor, cx| {
            let text = editor.text(cx);
            assert_eq!(text, format!("Lorem [@one.txt]({url_one}) "));
            assert!(!editor.has_visible_completions_menu());
            assert_eq!(fold_ranges(editor, cx).len(), 1);
        });

        let all_prompt_capabilities = acp::PromptCapabilities {
            image: true,
            audio: true,
            embedded_context: true,
        };

        let contents = message_editor
            .update(&mut cx, |message_editor, cx| {
                message_editor
                    .mention_set()
                    .contents(&all_prompt_capabilities, cx)
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
            pretty_assertions::assert_eq!(uri, &url_one.parse::<MentionUri>().unwrap());
        }

        let contents = message_editor
            .update(&mut cx, |message_editor, cx| {
                message_editor
                    .mention_set()
                    .contents(&acp::PromptCapabilities::default(), cx)
            })
            .await
            .unwrap()
            .into_values()
            .collect::<Vec<_>>();

        {
            let [(uri, Mention::UriOnly)] = contents.as_slice() else {
                panic!("Unexpected mentions");
            };
            pretty_assertions::assert_eq!(uri, &url_one.parse::<MentionUri>().unwrap());
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
                    .contents(&all_prompt_capabilities, cx)
            })
            .await
            .unwrap()
            .into_values()
            .collect::<Vec<_>>();

        let url_eight = uri!("file:///dir/b/eight.txt");

        {
            let [_, (uri, Mention::Text { content, .. })] = contents.as_slice() else {
                panic!("Unexpected mentions");
            };
            pretty_assertions::assert_eq!(content, "8");
            pretty_assertions::assert_eq!(uri, &url_eight.parse::<MentionUri>().unwrap());
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
            assert_eq!(current_completion_labels(editor), &["MySymbol"]);
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        let contents = message_editor
            .update(&mut cx, |message_editor, cx| {
                message_editor
                    .mention_set()
                    .contents(&all_prompt_capabilities, cx)
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
            pretty_assertions::assert_eq!(
                uri,
                &format!("{url_one}?symbol=MySymbol#L1:1")
                    .parse::<MentionUri>()
                    .unwrap()
            );
        }

        cx.run_until_parked();

        editor.read_with(&cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                format!("Lorem [@one.txt]({url_one})  Ipsum [@eight.txt]({url_eight}) [@MySymbol]({url_one}?symbol=MySymbol#L1:1) ")
            );
        });

        // Try to mention an "image" file that will fail to load
        cx.simulate_input("@file x.png");

        editor.update(&mut cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                format!("Lorem [@one.txt]({url_one})  Ipsum [@eight.txt]({url_eight}) [@MySymbol]({url_one}?symbol=MySymbol#L1:1) @file x.png")
            );
            assert!(editor.has_visible_completions_menu());
            assert_eq!(current_completion_labels(editor), &["x.png dir/"]);
        });

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.confirm_completion(&editor::actions::ConfirmCompletion::default(), window, cx);
        });

        // Getting the message contents fails
        message_editor
            .update(&mut cx, |message_editor, cx| {
                message_editor
                    .mention_set()
                    .contents(&all_prompt_capabilities, cx)
            })
            .await
            .expect_err("Should fail to load x.png");

        cx.run_until_parked();

        // Mention was removed
        editor.read_with(&cx, |editor, cx| {
            assert_eq!(
                editor.text(cx),
                format!("Lorem [@one.txt]({url_one})  Ipsum [@eight.txt]({url_eight}) [@MySymbol]({url_one}?symbol=MySymbol#L1:1) ")
            );
        });

        // Once more
        cx.simulate_input("@file x.png");

        editor.update(&mut cx, |editor, cx| {
                    assert_eq!(
                        editor.text(cx),
                        format!("Lorem [@one.txt]({url_one})  Ipsum [@eight.txt]({url_eight}) [@MySymbol]({url_one}?symbol=MySymbol#L1:1) @file x.png")
                    );
                    assert!(editor.has_visible_completions_menu());
                    assert_eq!(current_completion_labels(editor), &["x.png dir/"]);
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
                        format!("Lorem [@one.txt]({url_one})  Ipsum [@eight.txt]({url_eight}) [@MySymbol]({url_one}?symbol=MySymbol#L1:1) ")
                    );
                });

        // Now getting the contents succeeds, because the invalid mention was removed
        let contents = message_editor
            .update(&mut cx, |message_editor, cx| {
                message_editor
                    .mention_set()
                    .contents(&all_prompt_capabilities, cx)
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
                .folds_in_range(0..snapshot.len())
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
}
