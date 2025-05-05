use std::cmp;
use std::mem;
use std::ops::Range;
use std::rc::Rc;
use std::sync::Arc;

use anyhow::{Context as _, Result};
use assistant_settings::AssistantSettings;
use client::telemetry::Telemetry;
use collections::{HashMap, HashSet, VecDeque, hash_map};
use editor::{
    Anchor, AnchorRangeExt, CodeActionProvider, Editor, EditorEvent, ExcerptId, ExcerptRange,
    GutterDimensions, MultiBuffer, MultiBufferSnapshot, ToOffset as _, ToPoint,
    actions::SelectAll,
    display_map::{
        BlockContext, BlockPlacement, BlockProperties, BlockStyle, CustomBlockId, RenderBlock,
        ToDisplayPoint,
    },
};
use feature_flags::{Assistant2FeatureFlag, FeatureFlagViewExt as _};
use fs::Fs;
use gpui::{
    App, Context, Entity, Focusable, Global, HighlightStyle, Subscription, Task, UpdateGlobal,
    WeakEntity, Window, point,
};
use language::{Buffer, Point, Selection, TransactionId};
use language_model::ConfiguredModel;
use language_model::{LanguageModelRegistry, report_assistant_event};
use multi_buffer::MultiBufferRow;
use parking_lot::Mutex;
use project::LspAction;
use project::Project;
use project::{CodeAction, ProjectTransaction};
use prompt_store::PromptBuilder;
use prompt_store::PromptStore;
use settings::{Settings, SettingsStore};
use telemetry_events::{AssistantEventData, AssistantKind, AssistantPhase};
use terminal_view::{TerminalView, terminal_panel::TerminalPanel};
use text::{OffsetRangeExt, ToPoint as _};
use ui::prelude::*;
use util::RangeExt;
use util::ResultExt;
use workspace::{ItemHandle, Toast, Workspace, dock::Panel, notifications::NotificationId};
use zed_actions::agent::OpenConfiguration;

use crate::AssistantPanel;
use crate::buffer_codegen::{BufferCodegen, CodegenAlternative, CodegenEvent};
use crate::context_store::ContextStore;
use crate::inline_prompt_editor::{CodegenStatus, InlineAssistId, PromptEditor, PromptEditorEvent};
use crate::terminal_inline_assistant::TerminalInlineAssistant;
use crate::thread_store::TextThreadStore;
use crate::thread_store::ThreadStore;

pub fn init(
    fs: Arc<dyn Fs>,
    prompt_builder: Arc<PromptBuilder>,
    telemetry: Arc<Telemetry>,
    cx: &mut App,
) {
    cx.set_global(InlineAssistant::new(fs, prompt_builder, telemetry));
    cx.observe_new(|_workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        let workspace = cx.entity().clone();
        InlineAssistant::update_global(cx, |inline_assistant, cx| {
            inline_assistant.register_workspace(&workspace, window, cx)
        });

        cx.observe_flag::<Assistant2FeatureFlag, _>(window, {
            |is_assistant2_enabled, _workspace, _window, cx| {
                InlineAssistant::update_global(cx, |inline_assistant, _cx| {
                    inline_assistant.is_assistant2_enabled = is_assistant2_enabled;
                });
            }
        })
        .detach();
    })
    .detach();
}

const PROMPT_HISTORY_MAX_LEN: usize = 20;

enum InlineAssistTarget {
    Editor(Entity<Editor>),
    Terminal(Entity<TerminalView>),
}

pub struct InlineAssistant {
    next_assist_id: InlineAssistId,
    next_assist_group_id: InlineAssistGroupId,
    assists: HashMap<InlineAssistId, InlineAssist>,
    assists_by_editor: HashMap<WeakEntity<Editor>, EditorInlineAssists>,
    assist_groups: HashMap<InlineAssistGroupId, InlineAssistGroup>,
    confirmed_assists: HashMap<InlineAssistId, Entity<CodegenAlternative>>,
    prompt_history: VecDeque<String>,
    prompt_builder: Arc<PromptBuilder>,
    telemetry: Arc<Telemetry>,
    fs: Arc<dyn Fs>,
    is_assistant2_enabled: bool,
}

impl Global for InlineAssistant {}

impl InlineAssistant {
    pub fn new(
        fs: Arc<dyn Fs>,
        prompt_builder: Arc<PromptBuilder>,
        telemetry: Arc<Telemetry>,
    ) -> Self {
        Self {
            next_assist_id: InlineAssistId::default(),
            next_assist_group_id: InlineAssistGroupId::default(),
            assists: HashMap::default(),
            assists_by_editor: HashMap::default(),
            assist_groups: HashMap::default(),
            confirmed_assists: HashMap::default(),
            prompt_history: VecDeque::default(),
            prompt_builder,
            telemetry,
            fs,
            is_assistant2_enabled: false,
        }
    }

    pub fn register_workspace(
        &mut self,
        workspace: &Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) {
        window
            .subscribe(workspace, cx, |workspace, event, window, cx| {
                Self::update_global(cx, |this, cx| {
                    this.handle_workspace_event(workspace, event, window, cx)
                });
            })
            .detach();

        let workspace = workspace.downgrade();
        cx.observe_global::<SettingsStore>(move |cx| {
            let Some(workspace) = workspace.upgrade() else {
                return;
            };
            let Some(terminal_panel) = workspace.read(cx).panel::<TerminalPanel>(cx) else {
                return;
            };
            let enabled = AssistantSettings::get_global(cx).enabled;
            terminal_panel.update(cx, |terminal_panel, cx| {
                terminal_panel.set_assistant_enabled(enabled, cx)
            });
        })
        .detach();
    }

    fn handle_workspace_event(
        &mut self,
        workspace: Entity<Workspace>,
        event: &workspace::Event,
        window: &mut Window,
        cx: &mut App,
    ) {
        match event {
            workspace::Event::UserSavedItem { item, .. } => {
                // When the user manually saves an editor, automatically accepts all finished transformations.
                if let Some(editor) = item.upgrade().and_then(|item| item.act_as::<Editor>(cx)) {
                    if let Some(editor_assists) = self.assists_by_editor.get(&editor.downgrade()) {
                        for assist_id in editor_assists.assist_ids.clone() {
                            let assist = &self.assists[&assist_id];
                            if let CodegenStatus::Done = assist.codegen.read(cx).status(cx) {
                                self.finish_assist(assist_id, false, window, cx)
                            }
                        }
                    }
                }
            }
            workspace::Event::ItemAdded { item } => {
                self.register_workspace_item(&workspace, item.as_ref(), window, cx);
            }
            _ => (),
        }
    }

    fn register_workspace_item(
        &mut self,
        workspace: &Entity<Workspace>,
        item: &dyn ItemHandle,
        window: &mut Window,
        cx: &mut App,
    ) {
        let is_assistant2_enabled = self.is_assistant2_enabled;

        if let Some(editor) = item.act_as::<Editor>(cx) {
            editor.update(cx, |editor, cx| {
                if is_assistant2_enabled {
                    let panel = workspace.read(cx).panel::<AssistantPanel>(cx);
                    let thread_store = panel
                        .as_ref()
                        .map(|assistant_panel| assistant_panel.read(cx).thread_store().downgrade());
                    let text_thread_store = panel.map(|assistant_panel| {
                        assistant_panel.read(cx).text_thread_store().downgrade()
                    });

                    editor.add_code_action_provider(
                        Rc::new(AssistantCodeActionProvider {
                            editor: cx.entity().downgrade(),
                            workspace: workspace.downgrade(),
                            thread_store,
                            text_thread_store,
                        }),
                        window,
                        cx,
                    );

                    // Remove the Assistant1 code action provider, as it still might be registered.
                    editor.remove_code_action_provider("assistant".into(), window, cx);
                } else {
                    editor.remove_code_action_provider(
                        ASSISTANT_CODE_ACTION_PROVIDER_ID.into(),
                        window,
                        cx,
                    );
                }
            });
        }
    }

    pub fn inline_assist(
        workspace: &mut Workspace,
        _action: &zed_actions::assistant::InlineAssist,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let settings = AssistantSettings::get_global(cx);
        if !settings.enabled {
            return;
        }

        let Some(inline_assist_target) = Self::resolve_inline_assist_target(
            workspace,
            workspace.panel::<AssistantPanel>(cx),
            window,
            cx,
        ) else {
            return;
        };

        let is_authenticated = || {
            LanguageModelRegistry::read_global(cx)
                .inline_assistant_model()
                .map_or(false, |model| model.provider.is_authenticated(cx))
        };

        let Some(assistant_panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };
        let assistant_panel = assistant_panel.read(cx);

        let prompt_store = assistant_panel.prompt_store().as_ref().cloned();
        let thread_store = Some(assistant_panel.thread_store().downgrade());
        let text_thread_store = Some(assistant_panel.text_thread_store().downgrade());
        let context_store = assistant_panel.inline_assist_context_store().clone();

        let handle_assist =
            |window: &mut Window, cx: &mut Context<Workspace>| match inline_assist_target {
                InlineAssistTarget::Editor(active_editor) => {
                    InlineAssistant::update_global(cx, |assistant, cx| {
                        assistant.assist(
                            &active_editor,
                            cx.entity().downgrade(),
                            context_store,
                            workspace.project().downgrade(),
                            prompt_store,
                            thread_store,
                            text_thread_store,
                            window,
                            cx,
                        )
                    })
                }
                InlineAssistTarget::Terminal(active_terminal) => {
                    TerminalInlineAssistant::update_global(cx, |assistant, cx| {
                        assistant.assist(
                            &active_terminal,
                            cx.entity().downgrade(),
                            workspace.project().downgrade(),
                            prompt_store,
                            thread_store,
                            text_thread_store,
                            window,
                            cx,
                        )
                    })
                }
            };

        if is_authenticated() {
            handle_assist(window, cx);
        } else {
            cx.spawn_in(window, async move |_workspace, cx| {
                let Some(task) = cx.update(|_, cx| {
                    LanguageModelRegistry::read_global(cx)
                        .inline_assistant_model()
                        .map_or(None, |model| Some(model.provider.authenticate(cx)))
                })?
                else {
                    let answer = cx
                        .prompt(
                            gpui::PromptLevel::Warning,
                            "No language model provider configured",
                            None,
                            &["Configure", "Cancel"],
                        )
                        .await
                        .ok();
                    if let Some(answer) = answer {
                        if answer == 0 {
                            cx.update(|window, cx| {
                                window.dispatch_action(Box::new(OpenConfiguration), cx)
                            })
                            .ok();
                        }
                    }
                    return Ok(());
                };
                task.await?;

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);

            if is_authenticated() {
                handle_assist(window, cx);
            }
        }
    }

    pub fn assist(
        &mut self,
        editor: &Entity<Editor>,
        workspace: WeakEntity<Workspace>,
        context_store: Entity<ContextStore>,
        project: WeakEntity<Project>,
        prompt_store: Option<Entity<PromptStore>>,
        thread_store: Option<WeakEntity<ThreadStore>>,
        text_thread_store: Option<WeakEntity<TextThreadStore>>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let (snapshot, initial_selections) = editor.update(cx, |editor, cx| {
            (
                editor.snapshot(window, cx),
                editor.selections.all::<Point>(cx),
            )
        });

        let mut selections = Vec::<Selection<Point>>::new();
        let mut newest_selection = None;
        for mut selection in initial_selections {
            if selection.end > selection.start {
                selection.start.column = 0;
                // If the selection ends at the start of the line, we don't want to include it.
                if selection.end.column == 0 {
                    selection.end.row -= 1;
                }
                selection.end.column = snapshot
                    .buffer_snapshot
                    .line_len(MultiBufferRow(selection.end.row));
            } else if let Some(fold) =
                snapshot.crease_for_buffer_row(MultiBufferRow(selection.end.row))
            {
                selection.start = fold.range().start;
                selection.end = fold.range().end;
                if MultiBufferRow(selection.end.row) < snapshot.buffer_snapshot.max_row() {
                    let chars = snapshot
                        .buffer_snapshot
                        .chars_at(Point::new(selection.end.row + 1, 0));

                    for c in chars {
                        if c == '\n' {
                            break;
                        }
                        if c.is_whitespace() {
                            continue;
                        }
                        if snapshot
                            .language_at(selection.end)
                            .is_some_and(|language| language.config().brackets.is_closing_brace(c))
                        {
                            selection.end.row += 1;
                            selection.end.column = snapshot
                                .buffer_snapshot
                                .line_len(MultiBufferRow(selection.end.row));
                        }
                    }
                }
            }

            if let Some(prev_selection) = selections.last_mut() {
                if selection.start <= prev_selection.end {
                    prev_selection.end = selection.end;
                    continue;
                }
            }

            let latest_selection = newest_selection.get_or_insert_with(|| selection.clone());
            if selection.id > latest_selection.id {
                *latest_selection = selection.clone();
            }
            selections.push(selection);
        }
        let snapshot = &snapshot.buffer_snapshot;
        let newest_selection = newest_selection.unwrap();

        let mut codegen_ranges = Vec::new();
        for (buffer, buffer_range, excerpt_id) in
            snapshot.ranges_to_buffer_ranges(selections.iter().map(|selection| {
                snapshot.anchor_before(selection.start)..snapshot.anchor_after(selection.end)
            }))
        {
            let anchor_range = Anchor::range_in_buffer(
                excerpt_id,
                buffer.remote_id(),
                buffer.anchor_before(buffer_range.start)..buffer.anchor_after(buffer_range.end),
            );

            codegen_ranges.push(anchor_range);

            if let Some(model) = LanguageModelRegistry::read_global(cx).inline_assistant_model() {
                self.telemetry.report_assistant_event(AssistantEventData {
                    conversation_id: None,
                    kind: AssistantKind::Inline,
                    phase: AssistantPhase::Invoked,
                    message_id: None,
                    model: model.model.telemetry_id(),
                    model_provider: model.provider.id().to_string(),
                    response_latency: None,
                    error_message: None,
                    language_name: buffer.language().map(|language| language.name().to_proto()),
                });
            }
        }

        let assist_group_id = self.next_assist_group_id.post_inc();
        let prompt_buffer =
            cx.new(|cx| MultiBuffer::singleton(cx.new(|cx| Buffer::local(String::new(), cx)), cx));

        let mut assists = Vec::new();
        let mut assist_to_focus = None;
        for range in codegen_ranges {
            let assist_id = self.next_assist_id.post_inc();
            let codegen = cx.new(|cx| {
                BufferCodegen::new(
                    editor.read(cx).buffer().clone(),
                    range.clone(),
                    None,
                    context_store.clone(),
                    project.clone(),
                    prompt_store.clone(),
                    self.telemetry.clone(),
                    self.prompt_builder.clone(),
                    cx,
                )
            });

            let gutter_dimensions = Arc::new(Mutex::new(GutterDimensions::default()));
            let prompt_editor = cx.new(|cx| {
                PromptEditor::new_buffer(
                    assist_id,
                    gutter_dimensions.clone(),
                    self.prompt_history.clone(),
                    prompt_buffer.clone(),
                    codegen.clone(),
                    self.fs.clone(),
                    context_store.clone(),
                    workspace.clone(),
                    thread_store.clone(),
                    text_thread_store.clone(),
                    window,
                    cx,
                )
            });

            if assist_to_focus.is_none() {
                let focus_assist = if newest_selection.reversed {
                    range.start.to_point(&snapshot) == newest_selection.start
                } else {
                    range.end.to_point(&snapshot) == newest_selection.end
                };
                if focus_assist {
                    assist_to_focus = Some(assist_id);
                }
            }

            let [prompt_block_id, end_block_id] =
                self.insert_assist_blocks(editor, &range, &prompt_editor, cx);

            assists.push((
                assist_id,
                range,
                prompt_editor,
                prompt_block_id,
                end_block_id,
            ));
        }

        let editor_assists = self
            .assists_by_editor
            .entry(editor.downgrade())
            .or_insert_with(|| EditorInlineAssists::new(&editor, window, cx));
        let mut assist_group = InlineAssistGroup::new();
        for (assist_id, range, prompt_editor, prompt_block_id, end_block_id) in assists {
            let codegen = prompt_editor.read(cx).codegen().clone();

            self.assists.insert(
                assist_id,
                InlineAssist::new(
                    assist_id,
                    assist_group_id,
                    editor,
                    &prompt_editor,
                    prompt_block_id,
                    end_block_id,
                    range,
                    codegen,
                    workspace.clone(),
                    window,
                    cx,
                ),
            );
            assist_group.assist_ids.push(assist_id);
            editor_assists.assist_ids.push(assist_id);
        }
        self.assist_groups.insert(assist_group_id, assist_group);

        if let Some(assist_id) = assist_to_focus {
            self.focus_assist(assist_id, window, cx);
        }
    }

    pub fn suggest_assist(
        &mut self,
        editor: &Entity<Editor>,
        mut range: Range<Anchor>,
        initial_prompt: String,
        initial_transaction_id: Option<TransactionId>,
        focus: bool,
        workspace: Entity<Workspace>,
        prompt_store: Option<Entity<PromptStore>>,
        thread_store: Option<WeakEntity<ThreadStore>>,
        text_thread_store: Option<WeakEntity<TextThreadStore>>,
        window: &mut Window,
        cx: &mut App,
    ) -> InlineAssistId {
        let assist_group_id = self.next_assist_group_id.post_inc();
        let prompt_buffer = cx.new(|cx| Buffer::local(&initial_prompt, cx));
        let prompt_buffer = cx.new(|cx| MultiBuffer::singleton(prompt_buffer, cx));

        let assist_id = self.next_assist_id.post_inc();

        let buffer = editor.read(cx).buffer().clone();
        {
            let snapshot = buffer.read(cx).read(cx);
            range.start = range.start.bias_left(&snapshot);
            range.end = range.end.bias_right(&snapshot);
        }

        let project = workspace.read(cx).project().downgrade();
        let context_store = cx.new(|_cx| ContextStore::new(project.clone(), thread_store.clone()));

        let codegen = cx.new(|cx| {
            BufferCodegen::new(
                editor.read(cx).buffer().clone(),
                range.clone(),
                initial_transaction_id,
                context_store.clone(),
                project,
                prompt_store,
                self.telemetry.clone(),
                self.prompt_builder.clone(),
                cx,
            )
        });

        let gutter_dimensions = Arc::new(Mutex::new(GutterDimensions::default()));
        let prompt_editor = cx.new(|cx| {
            PromptEditor::new_buffer(
                assist_id,
                gutter_dimensions.clone(),
                self.prompt_history.clone(),
                prompt_buffer.clone(),
                codegen.clone(),
                self.fs.clone(),
                context_store,
                workspace.downgrade(),
                thread_store,
                text_thread_store,
                window,
                cx,
            )
        });

        let [prompt_block_id, end_block_id] =
            self.insert_assist_blocks(editor, &range, &prompt_editor, cx);

        let editor_assists = self
            .assists_by_editor
            .entry(editor.downgrade())
            .or_insert_with(|| EditorInlineAssists::new(&editor, window, cx));

        let mut assist_group = InlineAssistGroup::new();
        self.assists.insert(
            assist_id,
            InlineAssist::new(
                assist_id,
                assist_group_id,
                editor,
                &prompt_editor,
                prompt_block_id,
                end_block_id,
                range,
                codegen.clone(),
                workspace.downgrade(),
                window,
                cx,
            ),
        );
        assist_group.assist_ids.push(assist_id);
        editor_assists.assist_ids.push(assist_id);
        self.assist_groups.insert(assist_group_id, assist_group);

        if focus {
            self.focus_assist(assist_id, window, cx);
        }

        assist_id
    }

    fn insert_assist_blocks(
        &self,
        editor: &Entity<Editor>,
        range: &Range<Anchor>,
        prompt_editor: &Entity<PromptEditor<BufferCodegen>>,
        cx: &mut App,
    ) -> [CustomBlockId; 2] {
        let prompt_editor_height = prompt_editor.update(cx, |prompt_editor, cx| {
            prompt_editor
                .editor
                .update(cx, |editor, cx| editor.max_point(cx).row().0 + 1 + 2)
        });
        let assist_blocks = vec![
            BlockProperties {
                style: BlockStyle::Sticky,
                placement: BlockPlacement::Above(range.start),
                height: Some(prompt_editor_height),
                render: build_assist_editor_renderer(prompt_editor),
                priority: 0,
            },
            BlockProperties {
                style: BlockStyle::Sticky,
                placement: BlockPlacement::Below(range.end),
                height: None,
                render: Arc::new(|cx| {
                    v_flex()
                        .h_full()
                        .w_full()
                        .border_t_1()
                        .border_color(cx.theme().status().info_border)
                        .into_any_element()
                }),
                priority: 0,
            },
        ];

        editor.update(cx, |editor, cx| {
            let block_ids = editor.insert_blocks(assist_blocks, None, cx);
            [block_ids[0], block_ids[1]]
        })
    }

    fn handle_prompt_editor_focus_in(&mut self, assist_id: InlineAssistId, cx: &mut App) {
        let assist = &self.assists[&assist_id];
        let Some(decorations) = assist.decorations.as_ref() else {
            return;
        };
        let assist_group = self.assist_groups.get_mut(&assist.group_id).unwrap();
        let editor_assists = self.assists_by_editor.get_mut(&assist.editor).unwrap();

        assist_group.active_assist_id = Some(assist_id);
        if assist_group.linked {
            for assist_id in &assist_group.assist_ids {
                if let Some(decorations) = self.assists[assist_id].decorations.as_ref() {
                    decorations.prompt_editor.update(cx, |prompt_editor, cx| {
                        prompt_editor.set_show_cursor_when_unfocused(true, cx)
                    });
                }
            }
        }

        assist
            .editor
            .update(cx, |editor, cx| {
                let scroll_top = editor.scroll_position(cx).y;
                let scroll_bottom = scroll_top + editor.visible_line_count().unwrap_or(0.);
                let prompt_row = editor
                    .row_for_block(decorations.prompt_block_id, cx)
                    .unwrap()
                    .0 as f32;

                if (scroll_top..scroll_bottom).contains(&prompt_row) {
                    editor_assists.scroll_lock = Some(InlineAssistScrollLock {
                        assist_id,
                        distance_from_top: prompt_row - scroll_top,
                    });
                } else {
                    editor_assists.scroll_lock = None;
                }
            })
            .ok();
    }

    fn handle_prompt_editor_focus_out(&mut self, assist_id: InlineAssistId, cx: &mut App) {
        let assist = &self.assists[&assist_id];
        let assist_group = self.assist_groups.get_mut(&assist.group_id).unwrap();
        if assist_group.active_assist_id == Some(assist_id) {
            assist_group.active_assist_id = None;
            if assist_group.linked {
                for assist_id in &assist_group.assist_ids {
                    if let Some(decorations) = self.assists[assist_id].decorations.as_ref() {
                        decorations.prompt_editor.update(cx, |prompt_editor, cx| {
                            prompt_editor.set_show_cursor_when_unfocused(false, cx)
                        });
                    }
                }
            }
        }
    }

    fn handle_prompt_editor_event(
        &mut self,
        prompt_editor: Entity<PromptEditor<BufferCodegen>>,
        event: &PromptEditorEvent,
        window: &mut Window,
        cx: &mut App,
    ) {
        let assist_id = prompt_editor.read(cx).id();
        match event {
            PromptEditorEvent::StartRequested => {
                self.start_assist(assist_id, window, cx);
            }
            PromptEditorEvent::StopRequested => {
                self.stop_assist(assist_id, cx);
            }
            PromptEditorEvent::ConfirmRequested { execute: _ } => {
                self.finish_assist(assist_id, false, window, cx);
            }
            PromptEditorEvent::CancelRequested => {
                self.finish_assist(assist_id, true, window, cx);
            }
            PromptEditorEvent::DismissRequested => {
                self.dismiss_assist(assist_id, window, cx);
            }
            PromptEditorEvent::Resized { .. } => {
                // This only matters for the terminal inline assistant
            }
        }
    }

    fn handle_editor_newline(&mut self, editor: Entity<Editor>, window: &mut Window, cx: &mut App) {
        let Some(editor_assists) = self.assists_by_editor.get(&editor.downgrade()) else {
            return;
        };

        if editor.read(cx).selections.count() == 1 {
            let (selection, buffer) = editor.update(cx, |editor, cx| {
                (
                    editor.selections.newest::<usize>(cx),
                    editor.buffer().read(cx).snapshot(cx),
                )
            });
            for assist_id in &editor_assists.assist_ids {
                let assist = &self.assists[assist_id];
                let assist_range = assist.range.to_offset(&buffer);
                if assist_range.contains(&selection.start) && assist_range.contains(&selection.end)
                {
                    if matches!(assist.codegen.read(cx).status(cx), CodegenStatus::Pending) {
                        self.dismiss_assist(*assist_id, window, cx);
                    } else {
                        self.finish_assist(*assist_id, false, window, cx);
                    }

                    return;
                }
            }
        }

        cx.propagate();
    }

    fn handle_editor_cancel(&mut self, editor: Entity<Editor>, window: &mut Window, cx: &mut App) {
        let Some(editor_assists) = self.assists_by_editor.get(&editor.downgrade()) else {
            return;
        };

        if editor.read(cx).selections.count() == 1 {
            let (selection, buffer) = editor.update(cx, |editor, cx| {
                (
                    editor.selections.newest::<usize>(cx),
                    editor.buffer().read(cx).snapshot(cx),
                )
            });
            let mut closest_assist_fallback = None;
            for assist_id in &editor_assists.assist_ids {
                let assist = &self.assists[assist_id];
                let assist_range = assist.range.to_offset(&buffer);
                if assist.decorations.is_some() {
                    if assist_range.contains(&selection.start)
                        && assist_range.contains(&selection.end)
                    {
                        self.focus_assist(*assist_id, window, cx);
                        return;
                    } else {
                        let distance_from_selection = assist_range
                            .start
                            .abs_diff(selection.start)
                            .min(assist_range.start.abs_diff(selection.end))
                            + assist_range
                                .end
                                .abs_diff(selection.start)
                                .min(assist_range.end.abs_diff(selection.end));
                        match closest_assist_fallback {
                            Some((_, old_distance)) => {
                                if distance_from_selection < old_distance {
                                    closest_assist_fallback =
                                        Some((assist_id, distance_from_selection));
                                }
                            }
                            None => {
                                closest_assist_fallback = Some((assist_id, distance_from_selection))
                            }
                        }
                    }
                }
            }

            if let Some((&assist_id, _)) = closest_assist_fallback {
                self.focus_assist(assist_id, window, cx);
            }
        }

        cx.propagate();
    }

    fn handle_editor_release(
        &mut self,
        editor: WeakEntity<Editor>,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(editor_assists) = self.assists_by_editor.get_mut(&editor) {
            for assist_id in editor_assists.assist_ids.clone() {
                self.finish_assist(assist_id, true, window, cx);
            }
        }
    }

    fn handle_editor_change(&mut self, editor: Entity<Editor>, window: &mut Window, cx: &mut App) {
        let Some(editor_assists) = self.assists_by_editor.get(&editor.downgrade()) else {
            return;
        };
        let Some(scroll_lock) = editor_assists.scroll_lock.as_ref() else {
            return;
        };
        let assist = &self.assists[&scroll_lock.assist_id];
        let Some(decorations) = assist.decorations.as_ref() else {
            return;
        };

        editor.update(cx, |editor, cx| {
            let scroll_position = editor.scroll_position(cx);
            let target_scroll_top = editor
                .row_for_block(decorations.prompt_block_id, cx)
                .unwrap()
                .0 as f32
                - scroll_lock.distance_from_top;
            if target_scroll_top != scroll_position.y {
                editor.set_scroll_position(point(scroll_position.x, target_scroll_top), window, cx);
            }
        });
    }

    fn handle_editor_event(
        &mut self,
        editor: Entity<Editor>,
        event: &EditorEvent,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(editor_assists) = self.assists_by_editor.get_mut(&editor.downgrade()) else {
            return;
        };

        match event {
            EditorEvent::Edited { transaction_id } => {
                let buffer = editor.read(cx).buffer().read(cx);
                let edited_ranges =
                    buffer.edited_ranges_for_transaction::<usize>(*transaction_id, cx);
                let snapshot = buffer.snapshot(cx);

                for assist_id in editor_assists.assist_ids.clone() {
                    let assist = &self.assists[&assist_id];
                    if matches!(
                        assist.codegen.read(cx).status(cx),
                        CodegenStatus::Error(_) | CodegenStatus::Done
                    ) {
                        let assist_range = assist.range.to_offset(&snapshot);
                        if edited_ranges
                            .iter()
                            .any(|range| range.overlaps(&assist_range))
                        {
                            self.finish_assist(assist_id, false, window, cx);
                        }
                    }
                }
            }
            EditorEvent::ScrollPositionChanged { .. } => {
                if let Some(scroll_lock) = editor_assists.scroll_lock.as_ref() {
                    let assist = &self.assists[&scroll_lock.assist_id];
                    if let Some(decorations) = assist.decorations.as_ref() {
                        let distance_from_top = editor.update(cx, |editor, cx| {
                            let scroll_top = editor.scroll_position(cx).y;
                            let prompt_row = editor
                                .row_for_block(decorations.prompt_block_id, cx)
                                .unwrap()
                                .0 as f32;
                            prompt_row - scroll_top
                        });

                        if distance_from_top != scroll_lock.distance_from_top {
                            editor_assists.scroll_lock = None;
                        }
                    }
                }
            }
            EditorEvent::SelectionsChanged { .. } => {
                for assist_id in editor_assists.assist_ids.clone() {
                    let assist = &self.assists[&assist_id];
                    if let Some(decorations) = assist.decorations.as_ref() {
                        if decorations
                            .prompt_editor
                            .focus_handle(cx)
                            .is_focused(window)
                        {
                            return;
                        }
                    }
                }

                editor_assists.scroll_lock = None;
            }
            _ => {}
        }
    }

    pub fn finish_assist(
        &mut self,
        assist_id: InlineAssistId,
        undo: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(assist) = self.assists.get(&assist_id) {
            let assist_group_id = assist.group_id;
            if self.assist_groups[&assist_group_id].linked {
                for assist_id in self.unlink_assist_group(assist_group_id, window, cx) {
                    self.finish_assist(assist_id, undo, window, cx);
                }
                return;
            }
        }

        self.dismiss_assist(assist_id, window, cx);

        if let Some(assist) = self.assists.remove(&assist_id) {
            if let hash_map::Entry::Occupied(mut entry) = self.assist_groups.entry(assist.group_id)
            {
                entry.get_mut().assist_ids.retain(|id| *id != assist_id);
                if entry.get().assist_ids.is_empty() {
                    entry.remove();
                }
            }

            if let hash_map::Entry::Occupied(mut entry) =
                self.assists_by_editor.entry(assist.editor.clone())
            {
                entry.get_mut().assist_ids.retain(|id| *id != assist_id);
                if entry.get().assist_ids.is_empty() {
                    entry.remove();
                    if let Some(editor) = assist.editor.upgrade() {
                        self.update_editor_highlights(&editor, cx);
                    }
                } else {
                    entry.get().highlight_updates.send(()).ok();
                }
            }

            let active_alternative = assist.codegen.read(cx).active_alternative().clone();
            let message_id = active_alternative.read(cx).message_id.clone();

            if let Some(model) = LanguageModelRegistry::read_global(cx).inline_assistant_model() {
                let language_name = assist.editor.upgrade().and_then(|editor| {
                    let multibuffer = editor.read(cx).buffer().read(cx);
                    let snapshot = multibuffer.snapshot(cx);
                    let ranges = snapshot.range_to_buffer_ranges(assist.range.clone());
                    ranges
                        .first()
                        .and_then(|(buffer, _, _)| buffer.language())
                        .map(|language| language.name())
                });
                report_assistant_event(
                    AssistantEventData {
                        conversation_id: None,
                        kind: AssistantKind::Inline,
                        message_id,
                        phase: if undo {
                            AssistantPhase::Rejected
                        } else {
                            AssistantPhase::Accepted
                        },
                        model: model.model.telemetry_id(),
                        model_provider: model.model.provider_id().to_string(),
                        response_latency: None,
                        error_message: None,
                        language_name: language_name.map(|name| name.to_proto()),
                    },
                    Some(self.telemetry.clone()),
                    cx.http_client(),
                    model.model.api_key(cx),
                    cx.background_executor(),
                );
            }

            if undo {
                assist.codegen.update(cx, |codegen, cx| codegen.undo(cx));
            } else {
                self.confirmed_assists.insert(assist_id, active_alternative);
            }
        }
    }

    fn dismiss_assist(
        &mut self,
        assist_id: InlineAssistId,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let Some(assist) = self.assists.get_mut(&assist_id) else {
            return false;
        };
        let Some(editor) = assist.editor.upgrade() else {
            return false;
        };
        let Some(decorations) = assist.decorations.take() else {
            return false;
        };

        editor.update(cx, |editor, cx| {
            let mut to_remove = decorations.removed_line_block_ids;
            to_remove.insert(decorations.prompt_block_id);
            to_remove.insert(decorations.end_block_id);
            editor.remove_blocks(to_remove, None, cx);
        });

        if decorations
            .prompt_editor
            .focus_handle(cx)
            .contains_focused(window, cx)
        {
            self.focus_next_assist(assist_id, window, cx);
        }

        if let Some(editor_assists) = self.assists_by_editor.get_mut(&editor.downgrade()) {
            if editor_assists
                .scroll_lock
                .as_ref()
                .map_or(false, |lock| lock.assist_id == assist_id)
            {
                editor_assists.scroll_lock = None;
            }
            editor_assists.highlight_updates.send(()).ok();
        }

        true
    }

    fn focus_next_assist(&mut self, assist_id: InlineAssistId, window: &mut Window, cx: &mut App) {
        let Some(assist) = self.assists.get(&assist_id) else {
            return;
        };

        let assist_group = &self.assist_groups[&assist.group_id];
        let assist_ix = assist_group
            .assist_ids
            .iter()
            .position(|id| *id == assist_id)
            .unwrap();
        let assist_ids = assist_group
            .assist_ids
            .iter()
            .skip(assist_ix + 1)
            .chain(assist_group.assist_ids.iter().take(assist_ix));

        for assist_id in assist_ids {
            let assist = &self.assists[assist_id];
            if assist.decorations.is_some() {
                self.focus_assist(*assist_id, window, cx);
                return;
            }
        }

        assist
            .editor
            .update(cx, |editor, cx| window.focus(&editor.focus_handle(cx)))
            .ok();
    }

    fn focus_assist(&mut self, assist_id: InlineAssistId, window: &mut Window, cx: &mut App) {
        let Some(assist) = self.assists.get(&assist_id) else {
            return;
        };

        if let Some(decorations) = assist.decorations.as_ref() {
            decorations.prompt_editor.update(cx, |prompt_editor, cx| {
                prompt_editor.editor.update(cx, |editor, cx| {
                    window.focus(&editor.focus_handle(cx));
                    editor.select_all(&SelectAll, window, cx);
                })
            });
        }

        self.scroll_to_assist(assist_id, window, cx);
    }

    pub fn scroll_to_assist(
        &mut self,
        assist_id: InlineAssistId,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(assist) = self.assists.get(&assist_id) else {
            return;
        };
        let Some(editor) = assist.editor.upgrade() else {
            return;
        };

        let position = assist.range.start;
        editor.update(cx, |editor, cx| {
            editor.change_selections(None, window, cx, |selections| {
                selections.select_anchor_ranges([position..position])
            });

            let mut scroll_target_top;
            let mut scroll_target_bottom;
            if let Some(decorations) = assist.decorations.as_ref() {
                scroll_target_top = editor
                    .row_for_block(decorations.prompt_block_id, cx)
                    .unwrap()
                    .0 as f32;
                scroll_target_bottom = editor
                    .row_for_block(decorations.end_block_id, cx)
                    .unwrap()
                    .0 as f32;
            } else {
                let snapshot = editor.snapshot(window, cx);
                let start_row = assist
                    .range
                    .start
                    .to_display_point(&snapshot.display_snapshot)
                    .row();
                scroll_target_top = start_row.0 as f32;
                scroll_target_bottom = scroll_target_top + 1.;
            }
            scroll_target_top -= editor.vertical_scroll_margin() as f32;
            scroll_target_bottom += editor.vertical_scroll_margin() as f32;

            let height_in_lines = editor.visible_line_count().unwrap_or(0.);
            let scroll_top = editor.scroll_position(cx).y;
            let scroll_bottom = scroll_top + height_in_lines;

            if scroll_target_top < scroll_top {
                editor.set_scroll_position(point(0., scroll_target_top), window, cx);
            } else if scroll_target_bottom > scroll_bottom {
                if (scroll_target_bottom - scroll_target_top) <= height_in_lines {
                    editor.set_scroll_position(
                        point(0., scroll_target_bottom - height_in_lines),
                        window,
                        cx,
                    );
                } else {
                    editor.set_scroll_position(point(0., scroll_target_top), window, cx);
                }
            }
        });
    }

    fn unlink_assist_group(
        &mut self,
        assist_group_id: InlineAssistGroupId,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<InlineAssistId> {
        let assist_group = self.assist_groups.get_mut(&assist_group_id).unwrap();
        assist_group.linked = false;

        for assist_id in &assist_group.assist_ids {
            let assist = self.assists.get_mut(assist_id).unwrap();
            if let Some(editor_decorations) = assist.decorations.as_ref() {
                editor_decorations
                    .prompt_editor
                    .update(cx, |prompt_editor, cx| prompt_editor.unlink(window, cx));
            }
        }
        assist_group.assist_ids.clone()
    }

    pub fn start_assist(&mut self, assist_id: InlineAssistId, window: &mut Window, cx: &mut App) {
        let assist = if let Some(assist) = self.assists.get_mut(&assist_id) {
            assist
        } else {
            return;
        };

        let assist_group_id = assist.group_id;
        if self.assist_groups[&assist_group_id].linked {
            for assist_id in self.unlink_assist_group(assist_group_id, window, cx) {
                self.start_assist(assist_id, window, cx);
            }
            return;
        }

        let Some(user_prompt) = assist.user_prompt(cx) else {
            return;
        };

        self.prompt_history.retain(|prompt| *prompt != user_prompt);
        self.prompt_history.push_back(user_prompt.clone());
        if self.prompt_history.len() > PROMPT_HISTORY_MAX_LEN {
            self.prompt_history.pop_front();
        }

        let Some(ConfiguredModel { model, .. }) =
            LanguageModelRegistry::read_global(cx).inline_assistant_model()
        else {
            return;
        };

        assist
            .codegen
            .update(cx, |codegen, cx| codegen.start(model, user_prompt, cx))
            .log_err();
    }

    pub fn stop_assist(&mut self, assist_id: InlineAssistId, cx: &mut App) {
        let assist = if let Some(assist) = self.assists.get_mut(&assist_id) {
            assist
        } else {
            return;
        };

        assist.codegen.update(cx, |codegen, cx| codegen.stop(cx));
    }

    fn update_editor_highlights(&self, editor: &Entity<Editor>, cx: &mut App) {
        let mut gutter_pending_ranges = Vec::new();
        let mut gutter_transformed_ranges = Vec::new();
        let mut foreground_ranges = Vec::new();
        let mut inserted_row_ranges = Vec::new();
        let empty_assist_ids = Vec::new();
        let assist_ids = self
            .assists_by_editor
            .get(&editor.downgrade())
            .map_or(&empty_assist_ids, |editor_assists| {
                &editor_assists.assist_ids
            });

        for assist_id in assist_ids {
            if let Some(assist) = self.assists.get(assist_id) {
                let codegen = assist.codegen.read(cx);
                let buffer = codegen.buffer(cx).read(cx).read(cx);
                foreground_ranges.extend(codegen.last_equal_ranges(cx).iter().cloned());

                let pending_range =
                    codegen.edit_position(cx).unwrap_or(assist.range.start)..assist.range.end;
                if pending_range.end.to_offset(&buffer) > pending_range.start.to_offset(&buffer) {
                    gutter_pending_ranges.push(pending_range);
                }

                if let Some(edit_position) = codegen.edit_position(cx) {
                    let edited_range = assist.range.start..edit_position;
                    if edited_range.end.to_offset(&buffer) > edited_range.start.to_offset(&buffer) {
                        gutter_transformed_ranges.push(edited_range);
                    }
                }

                if assist.decorations.is_some() {
                    inserted_row_ranges
                        .extend(codegen.diff(cx).inserted_row_ranges.iter().cloned());
                }
            }
        }

        let snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);
        merge_ranges(&mut foreground_ranges, &snapshot);
        merge_ranges(&mut gutter_pending_ranges, &snapshot);
        merge_ranges(&mut gutter_transformed_ranges, &snapshot);
        editor.update(cx, |editor, cx| {
            enum GutterPendingRange {}
            if gutter_pending_ranges.is_empty() {
                editor.clear_gutter_highlights::<GutterPendingRange>(cx);
            } else {
                editor.highlight_gutter::<GutterPendingRange>(
                    &gutter_pending_ranges,
                    |cx| cx.theme().status().info_background,
                    cx,
                )
            }

            enum GutterTransformedRange {}
            if gutter_transformed_ranges.is_empty() {
                editor.clear_gutter_highlights::<GutterTransformedRange>(cx);
            } else {
                editor.highlight_gutter::<GutterTransformedRange>(
                    &gutter_transformed_ranges,
                    |cx| cx.theme().status().info,
                    cx,
                )
            }

            if foreground_ranges.is_empty() {
                editor.clear_highlights::<InlineAssist>(cx);
            } else {
                editor.highlight_text::<InlineAssist>(
                    foreground_ranges,
                    HighlightStyle {
                        fade_out: Some(0.6),
                        ..Default::default()
                    },
                    cx,
                );
            }

            editor.clear_row_highlights::<InlineAssist>();
            for row_range in inserted_row_ranges {
                editor.highlight_rows::<InlineAssist>(
                    row_range,
                    cx.theme().status().info_background,
                    Default::default(),
                    cx,
                );
            }
        });
    }

    fn update_editor_blocks(
        &mut self,
        editor: &Entity<Editor>,
        assist_id: InlineAssistId,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(assist) = self.assists.get_mut(&assist_id) else {
            return;
        };
        let Some(decorations) = assist.decorations.as_mut() else {
            return;
        };

        let codegen = assist.codegen.read(cx);
        let old_snapshot = codegen.snapshot(cx);
        let old_buffer = codegen.old_buffer(cx);
        let deleted_row_ranges = codegen.diff(cx).deleted_row_ranges.clone();

        editor.update(cx, |editor, cx| {
            let old_blocks = mem::take(&mut decorations.removed_line_block_ids);
            editor.remove_blocks(old_blocks, None, cx);

            let mut new_blocks = Vec::new();
            for (new_row, old_row_range) in deleted_row_ranges {
                let (_, buffer_start) = old_snapshot
                    .point_to_buffer_offset(Point::new(*old_row_range.start(), 0))
                    .unwrap();
                let (_, buffer_end) = old_snapshot
                    .point_to_buffer_offset(Point::new(
                        *old_row_range.end(),
                        old_snapshot.line_len(MultiBufferRow(*old_row_range.end())),
                    ))
                    .unwrap();

                let deleted_lines_editor = cx.new(|cx| {
                    let multi_buffer =
                        cx.new(|_| MultiBuffer::without_headers(language::Capability::ReadOnly));
                    multi_buffer.update(cx, |multi_buffer, cx| {
                        multi_buffer.push_excerpts(
                            old_buffer.clone(),
                            Some(ExcerptRange::new(buffer_start..buffer_end)),
                            cx,
                        );
                    });

                    enum DeletedLines {}
                    let mut editor = Editor::for_multibuffer(multi_buffer, None, window, cx);
                    editor.set_soft_wrap_mode(language::language_settings::SoftWrap::None, cx);
                    editor.set_show_wrap_guides(false, cx);
                    editor.set_show_gutter(false, cx);
                    editor.scroll_manager.set_forbid_vertical_scroll(true);
                    editor.set_show_scrollbars(false, cx);
                    editor.set_read_only(true);
                    editor.set_show_edit_predictions(Some(false), window, cx);
                    editor.highlight_rows::<DeletedLines>(
                        Anchor::min()..Anchor::max(),
                        cx.theme().status().deleted_background,
                        Default::default(),
                        cx,
                    );
                    editor
                });

                let height =
                    deleted_lines_editor.update(cx, |editor, cx| editor.max_point(cx).row().0 + 1);
                new_blocks.push(BlockProperties {
                    placement: BlockPlacement::Above(new_row),
                    height: Some(height),
                    style: BlockStyle::Flex,
                    render: Arc::new(move |cx| {
                        div()
                            .block_mouse_down()
                            .bg(cx.theme().status().deleted_background)
                            .size_full()
                            .h(height as f32 * cx.window.line_height())
                            .pl(cx.gutter_dimensions.full_width())
                            .child(deleted_lines_editor.clone())
                            .into_any_element()
                    }),
                    priority: 0,
                });
            }

            decorations.removed_line_block_ids = editor
                .insert_blocks(new_blocks, None, cx)
                .into_iter()
                .collect();
        })
    }

    fn resolve_inline_assist_target(
        workspace: &mut Workspace,
        assistant_panel: Option<Entity<AssistantPanel>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<InlineAssistTarget> {
        if let Some(terminal_panel) = workspace.panel::<TerminalPanel>(cx) {
            if terminal_panel
                .read(cx)
                .focus_handle(cx)
                .contains_focused(window, cx)
            {
                if let Some(terminal_view) = terminal_panel.read(cx).pane().and_then(|pane| {
                    pane.read(cx)
                        .active_item()
                        .and_then(|t| t.downcast::<TerminalView>())
                }) {
                    return Some(InlineAssistTarget::Terminal(terminal_view));
                }
            }
        }

        let context_editor = assistant_panel
            .and_then(|panel| panel.read(cx).active_context_editor())
            .and_then(|editor| {
                let editor = &editor.read(cx).editor().clone();
                if editor.read(cx).is_focused(window) {
                    Some(editor.clone())
                } else {
                    None
                }
            });

        if let Some(context_editor) = context_editor {
            Some(InlineAssistTarget::Editor(context_editor))
        } else if let Some(workspace_editor) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
        {
            Some(InlineAssistTarget::Editor(workspace_editor))
        } else if let Some(terminal_view) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<TerminalView>(cx))
        {
            Some(InlineAssistTarget::Terminal(terminal_view))
        } else {
            None
        }
    }
}

struct EditorInlineAssists {
    assist_ids: Vec<InlineAssistId>,
    scroll_lock: Option<InlineAssistScrollLock>,
    highlight_updates: async_watch::Sender<()>,
    _update_highlights: Task<Result<()>>,
    _subscriptions: Vec<gpui::Subscription>,
}

struct InlineAssistScrollLock {
    assist_id: InlineAssistId,
    distance_from_top: f32,
}

impl EditorInlineAssists {
    fn new(editor: &Entity<Editor>, window: &mut Window, cx: &mut App) -> Self {
        let (highlight_updates_tx, mut highlight_updates_rx) = async_watch::channel(());
        Self {
            assist_ids: Vec::new(),
            scroll_lock: None,
            highlight_updates: highlight_updates_tx,
            _update_highlights: cx.spawn({
                let editor = editor.downgrade();
                async move |cx| {
                    while let Ok(()) = highlight_updates_rx.changed().await {
                        let editor = editor.upgrade().context("editor was dropped")?;
                        cx.update_global(|assistant: &mut InlineAssistant, cx| {
                            assistant.update_editor_highlights(&editor, cx);
                        })?;
                    }
                    Ok(())
                }
            }),
            _subscriptions: vec![
                cx.observe_release_in(editor, window, {
                    let editor = editor.downgrade();
                    |_, window, cx| {
                        InlineAssistant::update_global(cx, |this, cx| {
                            this.handle_editor_release(editor, window, cx);
                        })
                    }
                }),
                window.observe(editor, cx, move |editor, window, cx| {
                    InlineAssistant::update_global(cx, |this, cx| {
                        this.handle_editor_change(editor, window, cx)
                    })
                }),
                window.subscribe(editor, cx, move |editor, event, window, cx| {
                    InlineAssistant::update_global(cx, |this, cx| {
                        this.handle_editor_event(editor, event, window, cx)
                    })
                }),
                editor.update(cx, |editor, cx| {
                    let editor_handle = cx.entity().downgrade();
                    editor.register_action(move |_: &editor::actions::Newline, window, cx| {
                        InlineAssistant::update_global(cx, |this, cx| {
                            if let Some(editor) = editor_handle.upgrade() {
                                this.handle_editor_newline(editor, window, cx)
                            }
                        })
                    })
                }),
                editor.update(cx, |editor, cx| {
                    let editor_handle = cx.entity().downgrade();
                    editor.register_action(move |_: &editor::actions::Cancel, window, cx| {
                        InlineAssistant::update_global(cx, |this, cx| {
                            if let Some(editor) = editor_handle.upgrade() {
                                this.handle_editor_cancel(editor, window, cx)
                            }
                        })
                    })
                }),
            ],
        }
    }
}

struct InlineAssistGroup {
    assist_ids: Vec<InlineAssistId>,
    linked: bool,
    active_assist_id: Option<InlineAssistId>,
}

impl InlineAssistGroup {
    fn new() -> Self {
        Self {
            assist_ids: Vec::new(),
            linked: true,
            active_assist_id: None,
        }
    }
}

fn build_assist_editor_renderer(editor: &Entity<PromptEditor<BufferCodegen>>) -> RenderBlock {
    let editor = editor.clone();

    Arc::new(move |cx: &mut BlockContext| {
        let gutter_dimensions = editor.read(cx).gutter_dimensions();

        *gutter_dimensions.lock() = *cx.gutter_dimensions;
        editor.clone().into_any_element()
    })
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
struct InlineAssistGroupId(usize);

impl InlineAssistGroupId {
    fn post_inc(&mut self) -> InlineAssistGroupId {
        let id = *self;
        self.0 += 1;
        id
    }
}

pub struct InlineAssist {
    group_id: InlineAssistGroupId,
    range: Range<Anchor>,
    editor: WeakEntity<Editor>,
    decorations: Option<InlineAssistDecorations>,
    codegen: Entity<BufferCodegen>,
    _subscriptions: Vec<Subscription>,
    workspace: WeakEntity<Workspace>,
}

impl InlineAssist {
    fn new(
        assist_id: InlineAssistId,
        group_id: InlineAssistGroupId,
        editor: &Entity<Editor>,
        prompt_editor: &Entity<PromptEditor<BufferCodegen>>,
        prompt_block_id: CustomBlockId,
        end_block_id: CustomBlockId,
        range: Range<Anchor>,
        codegen: Entity<BufferCodegen>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let prompt_editor_focus_handle = prompt_editor.focus_handle(cx);
        InlineAssist {
            group_id,
            editor: editor.downgrade(),
            decorations: Some(InlineAssistDecorations {
                prompt_block_id,
                prompt_editor: prompt_editor.clone(),
                removed_line_block_ids: HashSet::default(),
                end_block_id,
            }),
            range,
            codegen: codegen.clone(),
            workspace: workspace.clone(),
            _subscriptions: vec![
                window.on_focus_in(&prompt_editor_focus_handle, cx, move |_, cx| {
                    InlineAssistant::update_global(cx, |this, cx| {
                        this.handle_prompt_editor_focus_in(assist_id, cx)
                    })
                }),
                window.on_focus_out(&prompt_editor_focus_handle, cx, move |_, _, cx| {
                    InlineAssistant::update_global(cx, |this, cx| {
                        this.handle_prompt_editor_focus_out(assist_id, cx)
                    })
                }),
                window.subscribe(prompt_editor, cx, |prompt_editor, event, window, cx| {
                    InlineAssistant::update_global(cx, |this, cx| {
                        this.handle_prompt_editor_event(prompt_editor, event, window, cx)
                    })
                }),
                window.observe(&codegen, cx, {
                    let editor = editor.downgrade();
                    move |_, window, cx| {
                        if let Some(editor) = editor.upgrade() {
                            InlineAssistant::update_global(cx, |this, cx| {
                                if let Some(editor_assists) =
                                    this.assists_by_editor.get(&editor.downgrade())
                                {
                                    editor_assists.highlight_updates.send(()).ok();
                                }

                                this.update_editor_blocks(&editor, assist_id, window, cx);
                            })
                        }
                    }
                }),
                window.subscribe(&codegen, cx, move |codegen, event, window, cx| {
                    InlineAssistant::update_global(cx, |this, cx| match event {
                        CodegenEvent::Undone => this.finish_assist(assist_id, false, window, cx),
                        CodegenEvent::Finished => {
                            let assist = if let Some(assist) = this.assists.get(&assist_id) {
                                assist
                            } else {
                                return;
                            };

                            if let CodegenStatus::Error(error) = codegen.read(cx).status(cx) {
                                if assist.decorations.is_none() {
                                    if let Some(workspace) = assist.workspace.upgrade() {
                                        let error = format!("Inline assistant error: {}", error);
                                        workspace.update(cx, |workspace, cx| {
                                            struct InlineAssistantError;

                                            let id =
                                                NotificationId::composite::<InlineAssistantError>(
                                                    assist_id.0,
                                                );

                                            workspace.show_toast(Toast::new(id, error), cx);
                                        })
                                    }
                                }
                            }

                            if assist.decorations.is_none() {
                                this.finish_assist(assist_id, false, window, cx);
                            }
                        }
                    })
                }),
            ],
        }
    }

    fn user_prompt(&self, cx: &App) -> Option<String> {
        let decorations = self.decorations.as_ref()?;
        Some(decorations.prompt_editor.read(cx).prompt(cx))
    }
}

struct InlineAssistDecorations {
    prompt_block_id: CustomBlockId,
    prompt_editor: Entity<PromptEditor<BufferCodegen>>,
    removed_line_block_ids: HashSet<CustomBlockId>,
    end_block_id: CustomBlockId,
}

struct AssistantCodeActionProvider {
    editor: WeakEntity<Editor>,
    workspace: WeakEntity<Workspace>,
    thread_store: Option<WeakEntity<ThreadStore>>,
    text_thread_store: Option<WeakEntity<TextThreadStore>>,
}

const ASSISTANT_CODE_ACTION_PROVIDER_ID: &str = "assistant2";

impl CodeActionProvider for AssistantCodeActionProvider {
    fn id(&self) -> Arc<str> {
        ASSISTANT_CODE_ACTION_PROVIDER_ID.into()
    }

    fn code_actions(
        &self,
        buffer: &Entity<Buffer>,
        range: Range<text::Anchor>,
        _: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Vec<CodeAction>>> {
        if !AssistantSettings::get_global(cx).enabled {
            return Task::ready(Ok(Vec::new()));
        }

        let snapshot = buffer.read(cx).snapshot();
        let mut range = range.to_point(&snapshot);

        // Expand the range to line boundaries.
        range.start.column = 0;
        range.end.column = snapshot.line_len(range.end.row);

        let mut has_diagnostics = false;
        for diagnostic in snapshot.diagnostics_in_range::<_, Point>(range.clone(), false) {
            range.start = cmp::min(range.start, diagnostic.range.start);
            range.end = cmp::max(range.end, diagnostic.range.end);
            has_diagnostics = true;
        }
        if has_diagnostics {
            if let Some(symbols_containing_start) = snapshot.symbols_containing(range.start, None) {
                if let Some(symbol) = symbols_containing_start.last() {
                    range.start = cmp::min(range.start, symbol.range.start.to_point(&snapshot));
                    range.end = cmp::max(range.end, symbol.range.end.to_point(&snapshot));
                }
            }

            if let Some(symbols_containing_end) = snapshot.symbols_containing(range.end, None) {
                if let Some(symbol) = symbols_containing_end.last() {
                    range.start = cmp::min(range.start, symbol.range.start.to_point(&snapshot));
                    range.end = cmp::max(range.end, symbol.range.end.to_point(&snapshot));
                }
            }

            Task::ready(Ok(vec![CodeAction {
                server_id: language::LanguageServerId(0),
                range: snapshot.anchor_before(range.start)..snapshot.anchor_after(range.end),
                lsp_action: LspAction::Action(Box::new(lsp::CodeAction {
                    title: "Fix with Assistant".into(),
                    ..Default::default()
                })),
                resolved: true,
            }]))
        } else {
            Task::ready(Ok(Vec::new()))
        }
    }

    fn apply_code_action(
        &self,
        buffer: Entity<Buffer>,
        action: CodeAction,
        excerpt_id: ExcerptId,
        _push_to_history: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<ProjectTransaction>> {
        let editor = self.editor.clone();
        let workspace = self.workspace.clone();
        let thread_store = self.thread_store.clone();
        let text_thread_store = self.text_thread_store.clone();
        let prompt_store = PromptStore::global(cx);
        window.spawn(cx, async move |cx| {
            let workspace = workspace.upgrade().context("workspace was released")?;
            let editor = editor.upgrade().context("editor was released")?;
            let range = editor
                .update(cx, |editor, cx| {
                    editor.buffer().update(cx, |multibuffer, cx| {
                        let buffer = buffer.read(cx);
                        let multibuffer_snapshot = multibuffer.read(cx);

                        let old_context_range =
                            multibuffer_snapshot.context_range_for_excerpt(excerpt_id)?;
                        let mut new_context_range = old_context_range.clone();
                        if action
                            .range
                            .start
                            .cmp(&old_context_range.start, buffer)
                            .is_lt()
                        {
                            new_context_range.start = action.range.start;
                        }
                        if action.range.end.cmp(&old_context_range.end, buffer).is_gt() {
                            new_context_range.end = action.range.end;
                        }
                        drop(multibuffer_snapshot);

                        if new_context_range != old_context_range {
                            multibuffer.resize_excerpt(excerpt_id, new_context_range, cx);
                        }

                        let multibuffer_snapshot = multibuffer.read(cx);
                        Some(
                            multibuffer_snapshot
                                .anchor_in_excerpt(excerpt_id, action.range.start)?
                                ..multibuffer_snapshot
                                    .anchor_in_excerpt(excerpt_id, action.range.end)?,
                        )
                    })
                })?
                .context("invalid range")?;

            let prompt_store = prompt_store.await.ok();
            cx.update_global(|assistant: &mut InlineAssistant, window, cx| {
                let assist_id = assistant.suggest_assist(
                    &editor,
                    range,
                    "Fix Diagnostics".into(),
                    None,
                    true,
                    workspace,
                    prompt_store,
                    thread_store,
                    text_thread_store,
                    window,
                    cx,
                );
                assistant.start_assist(assist_id, window, cx);
            })?;

            Ok(ProjectTransaction::default())
        })
    }
}

fn merge_ranges(ranges: &mut Vec<Range<Anchor>>, buffer: &MultiBufferSnapshot) {
    ranges.sort_unstable_by(|a, b| {
        a.start
            .cmp(&b.start, buffer)
            .then_with(|| b.end.cmp(&a.end, buffer))
    });

    let mut ix = 0;
    while ix + 1 < ranges.len() {
        let b = ranges[ix + 1].clone();
        let a = &mut ranges[ix];
        if a.end.cmp(&b.start, buffer).is_gt() {
            if a.end.cmp(&b.end, buffer).is_lt() {
                a.end = b.end;
            }
            ranges.remove(ix + 1);
        } else {
            ix += 1;
        }
    }
}
