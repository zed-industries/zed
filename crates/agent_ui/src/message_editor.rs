use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Arc;

use crate::agent_diff::AgentDiffThread;
use crate::agent_model_selector::AgentModelSelector;
use crate::tool_compatibility::{IncompatibleToolsState, IncompatibleToolsTooltip};
use crate::ui::{
    MaxModeTooltip,
    preview::{AgentPreview, UsageCallout},
};
use agent::history_store::HistoryStore;
use agent::{
    context::{AgentContextKey, ContextLoadResult, load_context},
    context_store::ContextStoreEvent,
};
use agent_settings::{AgentSettings, CompletionMode};
use ai_onboarding::ApiKeysWithProviders;
use buffer_diff::BufferDiff;
use cloud_llm_client::CompletionIntent;
use collections::{HashMap, HashSet};
use editor::actions::{MoveUp, Paste};
use editor::display_map::CreaseId;
use editor::{
    Addon, AnchorRangeExt, ContextMenuOptions, ContextMenuPlacement, Editor, EditorElement,
    EditorEvent, EditorMode, EditorStyle, MultiBuffer,
};
use file_icons::FileIcons;
use fs::Fs;
use futures::future::Shared;
use futures::{FutureExt as _, future};
use gpui::{
    Animation, AnimationExt, App, Entity, EventEmitter, Focusable, IntoElement, KeyContext,
    Subscription, Task, TextStyle, WeakEntity, linear_color_stop, linear_gradient, point,
    pulsating_between,
};
use language::{Buffer, Language, Point};
use language_model::{
    ConfiguredModel, LanguageModelRegistry, LanguageModelRequestMessage, MessageContent,
    ZED_CLOUD_PROVIDER_ID,
};
use multi_buffer;
use project::Project;
use prompt_store::PromptStore;
use settings::Settings;
use std::time::Duration;
use theme::ThemeSettings;
use ui::{
    Callout, Disclosure, Divider, DividerColor, KeyBinding, PopoverMenuHandle, Tooltip, prelude::*,
};
use util::ResultExt as _;
use workspace::{CollaboratorId, Workspace};
use zed_actions::agent::Chat;
use zed_actions::agent::ToggleModelSelector;

use crate::context_picker::{ContextPicker, ContextPickerCompletionProvider, crease_for_mention};
use crate::context_strip::{ContextStrip, ContextStripEvent, SuggestContextKind};
use crate::profile_selector::ProfileSelector;
use crate::{
    ActiveThread, AgentDiffPane, ChatWithFollow, ExpandMessageEditor, Follow, KeepAll,
    ModelUsageContext, NewThread, OpenAgentDiff, RejectAll, RemoveAllContext, ToggleBurnMode,
    ToggleContextPicker, ToggleProfileSelector, register_agent_preview,
};
use agent::{
    MessageCrease, Thread, TokenUsageRatio,
    context_store::ContextStore,
    thread_store::{TextThreadStore, ThreadStore},
};

pub const MIN_EDITOR_LINES: usize = 4;
pub const MAX_EDITOR_LINES: usize = 8;

#[derive(RegisterComponent)]
pub struct MessageEditor {
    thread: Entity<Thread>,
    incompatible_tools_state: Entity<IncompatibleToolsState>,
    editor: Entity<Editor>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    context_store: Entity<ContextStore>,
    prompt_store: Option<Entity<PromptStore>>,
    history_store: Option<WeakEntity<HistoryStore>>,
    context_strip: Entity<ContextStrip>,
    context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    model_selector: Entity<AgentModelSelector>,
    last_loaded_context: Option<ContextLoadResult>,
    load_context_task: Option<Shared<Task<()>>>,
    profile_selector: Entity<ProfileSelector>,
    edits_expanded: bool,
    editor_is_expanded: bool,
    last_estimated_token_count: Option<u64>,
    update_token_count_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

pub(crate) fn create_editor(
    workspace: WeakEntity<Workspace>,
    context_store: WeakEntity<ContextStore>,
    thread_store: WeakEntity<ThreadStore>,
    text_thread_store: WeakEntity<TextThreadStore>,
    min_lines: usize,
    max_lines: Option<usize>,
    window: &mut Window,
    cx: &mut App,
) -> Entity<Editor> {
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
        let mut editor = Editor::new(
            editor::EditorMode::AutoHeight {
                min_lines,
                max_lines: max_lines,
            },
            buffer,
            None,
            window,
            cx,
        );
        editor.set_placeholder_text("Message the agent – @ to include context", cx);
        editor.set_show_indent_guides(false, cx);
        editor.set_soft_wrap();
        editor.set_use_modal_editing(true);
        editor.set_context_menu_options(ContextMenuOptions {
            min_entries_visible: 12,
            max_entries_visible: 12,
            placement: Some(ContextMenuPlacement::Above),
        });
        editor.register_addon(ContextCreasesAddon::new());
        editor.register_addon(MessageEditorAddon::new());
        editor
    });

    let editor_entity = editor.downgrade();
    editor.update(cx, |editor, _| {
        editor.set_completion_provider(Some(Rc::new(ContextPickerCompletionProvider::new(
            workspace,
            context_store,
            Some(thread_store),
            Some(text_thread_store),
            editor_entity,
            None,
        ))));
    });
    editor
}

impl MessageEditor {
    pub fn new(
        fs: Arc<dyn Fs>,
        workspace: WeakEntity<Workspace>,
        context_store: Entity<ContextStore>,
        prompt_store: Option<Entity<PromptStore>>,
        thread_store: WeakEntity<ThreadStore>,
        text_thread_store: WeakEntity<TextThreadStore>,
        history_store: Option<WeakEntity<HistoryStore>>,
        thread: Entity<Thread>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let context_picker_menu_handle = PopoverMenuHandle::default();
        let model_selector_menu_handle = PopoverMenuHandle::default();

        let editor = create_editor(
            workspace.clone(),
            context_store.downgrade(),
            thread_store.clone(),
            text_thread_store.clone(),
            MIN_EDITOR_LINES,
            Some(MAX_EDITOR_LINES),
            window,
            cx,
        );

        let context_strip = cx.new(|cx| {
            ContextStrip::new(
                context_store.clone(),
                workspace.clone(),
                Some(thread_store.clone()),
                Some(text_thread_store.clone()),
                context_picker_menu_handle.clone(),
                SuggestContextKind::File,
                ModelUsageContext::Thread(thread.clone()),
                window,
                cx,
            )
        });

        let incompatible_tools = cx.new(|cx| IncompatibleToolsState::new(thread.clone(), cx));

        let subscriptions = vec![
            cx.subscribe_in(&context_strip, window, Self::handle_context_strip_event),
            cx.subscribe(&editor, |this, _, event, cx| match event {
                EditorEvent::BufferEdited => this.handle_message_changed(cx),
                _ => {}
            }),
            cx.observe(&context_store, |this, _, cx| {
                // When context changes, reload it for token counting.
                let _ = this.reload_context(cx);
            }),
            cx.observe(&thread.read(cx).action_log().clone(), |_, _, cx| {
                cx.notify()
            }),
        ];

        let model_selector = cx.new(|cx| {
            AgentModelSelector::new(
                fs.clone(),
                model_selector_menu_handle,
                editor.focus_handle(cx),
                ModelUsageContext::Thread(thread.clone()),
                window,
                cx,
            )
        });

        let profile_selector =
            cx.new(|cx| ProfileSelector::new(fs, thread.clone(), editor.focus_handle(cx), cx));

        Self {
            editor: editor.clone(),
            project: thread.read(cx).project().clone(),
            thread,
            incompatible_tools_state: incompatible_tools.clone(),
            workspace,
            context_store,
            prompt_store,
            history_store,
            context_strip,
            context_picker_menu_handle,
            load_context_task: None,
            last_loaded_context: None,
            model_selector,
            edits_expanded: false,
            editor_is_expanded: false,
            profile_selector,
            last_estimated_token_count: None,
            update_token_count_task: None,
            _subscriptions: subscriptions,
        }
    }

    pub fn context_store(&self) -> &Entity<ContextStore> {
        &self.context_store
    }

    pub fn get_text(&self, cx: &App) -> String {
        self.editor.read(cx).text(cx)
    }

    pub fn set_text(
        &mut self,
        text: impl Into<Arc<str>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.set_text(text, window, cx);
        });
    }

    pub fn expand_message_editor(
        &mut self,
        _: &ExpandMessageEditor,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_editor_is_expanded(!self.editor_is_expanded, cx);
    }

    fn set_editor_is_expanded(&mut self, is_expanded: bool, cx: &mut Context<Self>) {
        self.editor_is_expanded = is_expanded;
        self.editor.update(cx, |editor, _| {
            if self.editor_is_expanded {
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
        });
        cx.notify();
    }

    fn toggle_context_picker(
        &mut self,
        _: &ToggleContextPicker,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.context_picker_menu_handle.toggle(window, cx);
    }

    pub fn remove_all_context(
        &mut self,
        _: &RemoveAllContext,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.context_store.update(cx, |store, cx| store.clear(cx));
        cx.notify();
    }

    fn chat(&mut self, _: &Chat, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_editor_empty(cx) {
            return;
        }

        self.thread.update(cx, |thread, cx| {
            thread.cancel_editing(cx);
        });

        if self.thread.read(cx).is_generating() {
            self.stop_current_and_send_new_message(window, cx);
            return;
        }

        self.set_editor_is_expanded(false, cx);
        self.send_to_model(window, cx);

        cx.emit(MessageEditorEvent::ScrollThreadToBottom);
        cx.notify();
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

        self.chat(&Chat, window, cx);
    }

    fn is_editor_empty(&self, cx: &App) -> bool {
        self.editor.read(cx).text(cx).trim().is_empty()
    }

    pub fn is_editor_fully_empty(&self, cx: &App) -> bool {
        self.editor.read(cx).is_empty(cx)
    }

    fn send_to_model(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(ConfiguredModel { model, provider }) = self
            .thread
            .update(cx, |thread, cx| thread.get_or_init_configured_model(cx))
        else {
            return;
        };

        if provider.must_accept_terms(cx) {
            cx.notify();
            return;
        }

        let (user_message, user_message_creases) = self.editor.update(cx, |editor, cx| {
            let creases = extract_message_creases(editor, cx);
            let text = editor.text(cx);
            editor.clear(window, cx);
            (text, creases)
        });

        self.last_estimated_token_count.take();
        cx.emit(MessageEditorEvent::EstimatedTokenCount);

        let thread = self.thread.clone();
        let git_store = self.project.read(cx).git_store().clone();
        let checkpoint = git_store.update(cx, |git_store, cx| git_store.checkpoint(cx));
        let context_task = self.reload_context(cx);
        let window_handle = window.window_handle();

        cx.spawn(async move |_this, cx| {
            let (checkpoint, loaded_context) = future::join(checkpoint, context_task).await;
            let loaded_context = loaded_context.unwrap_or_default();

            thread
                .update(cx, |thread, cx| {
                    thread.insert_user_message(
                        user_message,
                        loaded_context,
                        checkpoint.ok(),
                        user_message_creases,
                        cx,
                    );
                })
                .log_err();

            thread
                .update(cx, |thread, cx| {
                    thread.advance_prompt_id();
                    thread.send_to_model(
                        model,
                        CompletionIntent::UserPrompt,
                        Some(window_handle),
                        cx,
                    );
                })
                .log_err();
        })
        .detach();
    }

    fn stop_current_and_send_new_message(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.thread.update(cx, |thread, cx| {
            thread.cancel_editing(cx);
        });

        let cancelled = self.thread.update(cx, |thread, cx| {
            thread.cancel_last_completion(Some(window.window_handle()), cx)
        });

        if cancelled {
            self.set_editor_is_expanded(false, cx);
            self.send_to_model(window, cx);
        }
    }

    fn handle_context_strip_event(
        &mut self,
        _context_strip: &Entity<ContextStrip>,
        event: &ContextStripEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            ContextStripEvent::PickerDismissed
            | ContextStripEvent::BlurredEmpty
            | ContextStripEvent::BlurredDown => {
                let editor_focus_handle = self.editor.focus_handle(cx);
                window.focus(&editor_focus_handle);
            }
            ContextStripEvent::BlurredUp => {}
        }
    }

    fn move_up(&mut self, _: &MoveUp, window: &mut Window, cx: &mut Context<Self>) {
        if self.context_picker_menu_handle.is_deployed() {
            cx.propagate();
        } else if self.context_strip.read(cx).has_context_items(cx) {
            self.context_strip.focus_handle(cx).focus(window);
        }
    }

    fn paste(&mut self, _: &Paste, _: &mut Window, cx: &mut Context<Self>) {
        crate::active_thread::attach_pasted_images_as_context(&self.context_store, cx);
    }

    fn handle_review_click(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.edits_expanded = true;
        AgentDiffPane::deploy(self.thread.clone(), self.workspace.clone(), window, cx).log_err();
        cx.notify();
    }

    fn handle_edit_bar_expand(&mut self, cx: &mut Context<Self>) {
        self.edits_expanded = !self.edits_expanded;
        cx.notify();
    }

    fn handle_file_click(
        &self,
        buffer: Entity<Buffer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Ok(diff) = AgentDiffPane::deploy(
            AgentDiffThread::Native(self.thread.clone()),
            self.workspace.clone(),
            window,
            cx,
        ) {
            let path_key = multi_buffer::PathKey::for_buffer(&buffer, cx);
            diff.update(cx, |diff, cx| diff.move_to_path(path_key, window, cx));
        }
    }

    pub fn toggle_burn_mode(
        &mut self,
        _: &ToggleBurnMode,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.thread.update(cx, |thread, _cx| {
            let active_completion_mode = thread.completion_mode();

            thread.set_completion_mode(match active_completion_mode {
                CompletionMode::Burn => CompletionMode::Normal,
                CompletionMode::Normal => CompletionMode::Burn,
            });
        });
    }

    fn handle_accept_all(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if self.thread.read(cx).has_pending_edit_tool_uses() {
            return;
        }

        self.thread.update(cx, |thread, cx| {
            thread.keep_all_edits(cx);
        });
        cx.notify();
    }

    fn handle_reject_all(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if self.thread.read(cx).has_pending_edit_tool_uses() {
            return;
        }

        // Since there's no reject_all_edits method in the thread API,
        // we need to iterate through all buffers and reject their edits
        let action_log = self.thread.read(cx).action_log().clone();
        let changed_buffers = action_log.read(cx).changed_buffers(cx);

        for (buffer, _) in changed_buffers {
            self.thread.update(cx, |thread, cx| {
                let buffer_snapshot = buffer.read(cx);
                let start = buffer_snapshot.anchor_before(Point::new(0, 0));
                let end = buffer_snapshot.anchor_after(buffer_snapshot.max_point());
                thread
                    .reject_edits_in_ranges(buffer, vec![start..end], cx)
                    .detach();
            });
        }
        cx.notify();
    }

    fn handle_reject_file_changes(
        &mut self,
        buffer: Entity<Buffer>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.thread.read(cx).has_pending_edit_tool_uses() {
            return;
        }

        self.thread.update(cx, |thread, cx| {
            let buffer_snapshot = buffer.read(cx);
            let start = buffer_snapshot.anchor_before(Point::new(0, 0));
            let end = buffer_snapshot.anchor_after(buffer_snapshot.max_point());
            thread
                .reject_edits_in_ranges(buffer, vec![start..end], cx)
                .detach();
        });
        cx.notify();
    }

    fn handle_accept_file_changes(
        &mut self,
        buffer: Entity<Buffer>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.thread.read(cx).has_pending_edit_tool_uses() {
            return;
        }

        self.thread.update(cx, |thread, cx| {
            let buffer_snapshot = buffer.read(cx);
            let start = buffer_snapshot.anchor_before(Point::new(0, 0));
            let end = buffer_snapshot.anchor_after(buffer_snapshot.max_point());
            thread.keep_edits_in_range(buffer, start..end, cx);
        });
        cx.notify();
    }

    fn render_burn_mode_toggle(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let thread = self.thread.read(cx);
        let model = thread.configured_model();
        if !model?.model.supports_burn_mode() {
            return None;
        }

        let active_completion_mode = thread.completion_mode();
        let burn_mode_enabled = active_completion_mode == CompletionMode::Burn;
        let icon = if burn_mode_enabled {
            IconName::ZedBurnModeOn
        } else {
            IconName::ZedBurnMode
        };

        Some(
            IconButton::new("burn-mode", icon)
                .icon_size(IconSize::Small)
                .icon_color(Color::Muted)
                .toggle_state(burn_mode_enabled)
                .selected_icon_color(Color::Error)
                .on_click(cx.listener(|this, _event, window, cx| {
                    this.toggle_burn_mode(&ToggleBurnMode, window, cx);
                }))
                .tooltip(move |_window, cx| {
                    cx.new(|_| MaxModeTooltip::new().selected(burn_mode_enabled))
                        .into()
                })
                .into_any_element(),
        )
    }

    fn render_follow_toggle(
        &self,
        is_model_selected: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let following = self
            .workspace
            .read_with(cx, |workspace, _| {
                workspace.is_being_followed(CollaboratorId::Agent)
            })
            .unwrap_or(false);

        IconButton::new("follow-agent", IconName::Crosshair)
            .disabled(!is_model_selected)
            .icon_size(IconSize::Small)
            .icon_color(Color::Muted)
            .toggle_state(following)
            .selected_icon_color(Some(Color::Custom(cx.theme().players().agent().cursor)))
            .tooltip(move |window, cx| {
                if following {
                    Tooltip::for_action("Stop Following Agent", &Follow, window, cx)
                } else {
                    Tooltip::with_meta(
                        "Follow Agent",
                        Some(&Follow),
                        "Track the agent's location as it reads and edits files.",
                        window,
                        cx,
                    )
                }
            })
            .on_click(cx.listener(move |this, _, window, cx| {
                this.workspace
                    .update(cx, |workspace, cx| {
                        if following {
                            workspace.unfollow(CollaboratorId::Agent, window, cx);
                        } else {
                            workspace.follow(CollaboratorId::Agent, window, cx);
                        }
                    })
                    .ok();
            }))
    }

    fn render_editor(&self, window: &mut Window, cx: &mut Context<Self>) -> Div {
        let thread = self.thread.read(cx);
        let model = thread.configured_model();

        let editor_bg_color = cx.theme().colors().editor_background;
        let is_generating = thread.is_generating();
        let focus_handle = self.editor.focus_handle(cx);

        let is_model_selected = model.is_some();
        let is_editor_empty = self.is_editor_empty(cx);

        let incompatible_tools = model
            .as_ref()
            .map(|model| {
                self.incompatible_tools_state.update(cx, |state, cx| {
                    state
                        .incompatible_tools(&model.model, cx)
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>()
                })
            })
            .unwrap_or_default();

        let is_editor_expanded = self.editor_is_expanded;
        let expand_icon = if is_editor_expanded {
            IconName::Minimize
        } else {
            IconName::Maximize
        };

        v_flex()
            .key_context("MessageEditor")
            .on_action(cx.listener(Self::chat))
            .on_action(cx.listener(Self::chat_with_follow))
            .on_action(cx.listener(|this, _: &ToggleProfileSelector, window, cx| {
                this.profile_selector
                    .read(cx)
                    .menu_handle()
                    .toggle(window, cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleModelSelector, window, cx| {
                this.model_selector
                    .update(cx, |model_selector, cx| model_selector.toggle(window, cx));
            }))
            .on_action(cx.listener(Self::toggle_context_picker))
            .on_action(cx.listener(Self::remove_all_context))
            .on_action(cx.listener(Self::move_up))
            .on_action(cx.listener(Self::expand_message_editor))
            .on_action(cx.listener(Self::toggle_burn_mode))
            .on_action(
                cx.listener(|this, _: &KeepAll, window, cx| this.handle_accept_all(window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &RejectAll, window, cx| this.handle_reject_all(window, cx)),
            )
            .capture_action(cx.listener(Self::paste))
            .p_2()
            .gap_2()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .bg(editor_bg_color)
            .child(
                h_flex()
                    .justify_between()
                    .child(self.context_strip.clone())
                    .when(focus_handle.is_focused(window), |this| {
                        this.child(
                            IconButton::new("toggle-height", expand_icon)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Muted)
                                .tooltip({
                                    let focus_handle = focus_handle.clone();
                                    move |window, cx| {
                                        let expand_label = if is_editor_expanded {
                                            "Minimize Message Editor".to_string()
                                        } else {
                                            "Expand Message Editor".to_string()
                                        };

                                        Tooltip::for_action_in(
                                            expand_label,
                                            &ExpandMessageEditor,
                                            &focus_handle,
                                            window,
                                            cx,
                                        )
                                    }
                                })
                                .on_click(cx.listener(|_, _, window, cx| {
                                    window.dispatch_action(Box::new(ExpandMessageEditor), cx);
                                })),
                        )
                    }),
            )
            .child(
                v_flex()
                    .size_full()
                    .gap_1()
                    .when(is_editor_expanded, |this| {
                        this.h(vh(0.8, window)).justify_between()
                    })
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
                                background: editor_bg_color,
                                local_player: cx.theme().players().local(),
                                text: text_style,
                                syntax: cx.theme().syntax().clone(),
                                ..Default::default()
                            },
                        )
                        .into_any()
                    })
                    .child(
                        h_flex()
                            .flex_none()
                            .flex_wrap()
                            .justify_between()
                            .child(
                                h_flex()
                                    .child(self.render_follow_toggle(is_model_selected, cx))
                                    .children(self.render_burn_mode_toggle(cx)),
                            )
                            .child(
                                h_flex()
                                    .gap_1()
                                    .flex_wrap()
                                    .when(!incompatible_tools.is_empty(), |this| {
                                        this.child(
                                            IconButton::new(
                                                "tools-incompatible-warning",
                                                IconName::Warning,
                                            )
                                            .icon_color(Color::Warning)
                                            .icon_size(IconSize::Small)
                                            .tooltip({
                                                move |_, cx| {
                                                    cx.new(|_| IncompatibleToolsTooltip {
                                                        incompatible_tools: incompatible_tools
                                                            .clone(),
                                                    })
                                                    .into()
                                                }
                                            }),
                                        )
                                    })
                                    .child(self.profile_selector.clone())
                                    .child(self.model_selector.clone())
                                    .map({
                                        let focus_handle = focus_handle.clone();
                                        move |parent| {
                                            if is_generating {
                                                parent
                                                    .when(is_editor_empty, |parent| {
                                                        parent.child(
                                                            IconButton::new(
                                                                "stop-generation",
                                                                IconName::StopFilled,
                                                            )
                                                            .icon_color(Color::Error)
                                                            .style(ButtonStyle::Tinted(
                                                                ui::TintColor::Error,
                                                            ))
                                                            .tooltip(move |window, cx| {
                                                                Tooltip::for_action(
                                                                    "Stop Generation",
                                                                    &editor::actions::Cancel,
                                                                    window,
                                                                    cx,
                                                                )
                                                            })
                                                            .on_click({
                                                                let focus_handle =
                                                                    focus_handle.clone();
                                                                move |_event, window, cx| {
                                                                    focus_handle.dispatch_action(
                                                                        &editor::actions::Cancel,
                                                                        window,
                                                                        cx,
                                                                    );
                                                                }
                                                            })
                                                            .with_animation(
                                                                "pulsating-label",
                                                                Animation::new(
                                                                    Duration::from_secs(2),
                                                                )
                                                                .repeat()
                                                                .with_easing(pulsating_between(
                                                                    0.4, 1.0,
                                                                )),
                                                                |icon_button, delta| {
                                                                    icon_button.alpha(delta)
                                                                },
                                                            ),
                                                        )
                                                    })
                                                    .when(!is_editor_empty, |parent| {
                                                        parent.child(
                                                            IconButton::new(
                                                                "send-message",
                                                                IconName::Send,
                                                            )
                                                            .icon_color(Color::Accent)
                                                            .style(ButtonStyle::Filled)
                                                            .disabled(!is_model_selected)
                                                            .on_click({
                                                                let focus_handle =
                                                                    focus_handle.clone();
                                                                move |_event, window, cx| {
                                                                    focus_handle.dispatch_action(
                                                                        &Chat, window, cx,
                                                                    );
                                                                }
                                                            })
                                                            .tooltip(move |window, cx| {
                                                                Tooltip::for_action(
                                                                    "Stop and Send New Message",
                                                                    &Chat,
                                                                    window,
                                                                    cx,
                                                                )
                                                            }),
                                                        )
                                                    })
                                            } else {
                                                parent.child(
                                                    IconButton::new("send-message", IconName::Send)
                                                        .icon_color(Color::Accent)
                                                        .style(ButtonStyle::Filled)
                                                        .disabled(
                                                            is_editor_empty || !is_model_selected,
                                                        )
                                                        .on_click({
                                                            let focus_handle = focus_handle.clone();
                                                            move |_event, window, cx| {
                                                                telemetry::event!(
                                                                    "Agent Message Sent",
                                                                    agent = "zed",
                                                                );
                                                                focus_handle.dispatch_action(
                                                                    &Chat, window, cx,
                                                                );
                                                            }
                                                        })
                                                        .when(
                                                            !is_editor_empty && is_model_selected,
                                                            |button| {
                                                                button.tooltip(move |window, cx| {
                                                                    Tooltip::for_action(
                                                                        "Send", &Chat, window, cx,
                                                                    )
                                                                })
                                                            },
                                                        )
                                                        .when(is_editor_empty, |button| {
                                                            button.tooltip(Tooltip::text(
                                                                "Type a message to submit",
                                                            ))
                                                        })
                                                        .when(!is_model_selected, |button| {
                                                            button.tooltip(Tooltip::text(
                                                                "Select a model to continue",
                                                            ))
                                                        }),
                                                )
                                            }
                                        }
                                    }),
                            ),
                    ),
            )
    }

    fn render_edits_bar(
        &self,
        changed_buffers: &BTreeMap<Entity<Buffer>, Entity<BufferDiff>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Div {
        let focus_handle = self.editor.focus_handle(cx);

        let editor_bg_color = cx.theme().colors().editor_background;
        let border_color = cx.theme().colors().border;
        let active_color = cx.theme().colors().element_selected;
        let bg_edit_files_disclosure = editor_bg_color.blend(active_color.opacity(0.3));

        let is_edit_changes_expanded = self.edits_expanded;
        let thread = self.thread.read(cx);
        let pending_edits = thread.has_pending_edit_tool_uses();

        const EDIT_NOT_READY_TOOLTIP_LABEL: &str = "Wait until file edits are complete.";

        v_flex()
            .mt_1()
            .mx_2()
            .bg(bg_edit_files_disclosure)
            .border_1()
            .border_b_0()
            .border_color(border_color)
            .rounded_t_md()
            .shadow(vec![gpui::BoxShadow {
                color: gpui::black().opacity(0.15),
                offset: point(px(1.), px(-1.)),
                blur_radius: px(3.),
                spread_radius: px(0.),
            }])
            .child(
                h_flex()
                    .p_1()
                    .justify_between()
                    .when(is_edit_changes_expanded, |this| {
                        this.border_b_1().border_color(border_color)
                    })
                    .child(
                        h_flex()
                            .id("edits-container")
                            .cursor_pointer()
                            .w_full()
                            .gap_1()
                            .child(
                                Disclosure::new("edits-disclosure", is_edit_changes_expanded)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.handle_edit_bar_expand(cx)
                                    })),
                            )
                            .map(|this| {
                                if pending_edits {
                                    this.child(
                                        Label::new(format!(
                                            "Editing {} {}…",
                                            changed_buffers.len(),
                                            if changed_buffers.len() == 1 {
                                                "file"
                                            } else {
                                                "files"
                                            }
                                        ))
                                        .color(Color::Muted)
                                        .size(LabelSize::Small)
                                        .with_animation(
                                            "edit-label",
                                            Animation::new(Duration::from_secs(2))
                                                .repeat()
                                                .with_easing(pulsating_between(0.3, 0.7)),
                                            |label, delta| label.alpha(delta),
                                        ),
                                    )
                                } else {
                                    this.child(
                                        Label::new("Edits")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .child(
                                        Label::new("•").size(LabelSize::XSmall).color(Color::Muted),
                                    )
                                    .child(
                                        Label::new(format!(
                                            "{} {}",
                                            changed_buffers.len(),
                                            if changed_buffers.len() == 1 {
                                                "file"
                                            } else {
                                                "files"
                                            }
                                        ))
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                    )
                                }
                            })
                            .on_click(
                                cx.listener(|this, _, _, cx| this.handle_edit_bar_expand(cx)),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                IconButton::new("review-changes", IconName::ListTodo)
                                    .icon_size(IconSize::Small)
                                    .tooltip({
                                        let focus_handle = focus_handle.clone();
                                        move |window, cx| {
                                            Tooltip::for_action_in(
                                                "Review Changes",
                                                &OpenAgentDiff,
                                                &focus_handle,
                                                window,
                                                cx,
                                            )
                                        }
                                    })
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.handle_review_click(window, cx)
                                    })),
                            )
                            .child(Divider::vertical().color(DividerColor::Border))
                            .child(
                                Button::new("reject-all-changes", "Reject All")
                                    .label_size(LabelSize::Small)
                                    .disabled(pending_edits)
                                    .when(pending_edits, |this| {
                                        this.tooltip(Tooltip::text(EDIT_NOT_READY_TOOLTIP_LABEL))
                                    })
                                    .key_binding(
                                        KeyBinding::for_action_in(
                                            &RejectAll,
                                            &focus_handle.clone(),
                                            window,
                                            cx,
                                        )
                                        .map(|kb| kb.size(rems_from_px(10.))),
                                    )
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.handle_reject_all(window, cx)
                                    })),
                            )
                            .child(
                                Button::new("accept-all-changes", "Accept All")
                                    .label_size(LabelSize::Small)
                                    .disabled(pending_edits)
                                    .when(pending_edits, |this| {
                                        this.tooltip(Tooltip::text(EDIT_NOT_READY_TOOLTIP_LABEL))
                                    })
                                    .key_binding(
                                        KeyBinding::for_action_in(
                                            &KeepAll,
                                            &focus_handle,
                                            window,
                                            cx,
                                        )
                                        .map(|kb| kb.size(rems_from_px(10.))),
                                    )
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.handle_accept_all(window, cx)
                                    })),
                            ),
                    ),
            )
            .when(is_edit_changes_expanded, |parent| {
                parent.child(
                    v_flex().children(changed_buffers.into_iter().enumerate().flat_map(
                        |(index, (buffer, _diff))| {
                            let file = buffer.read(cx).file()?;
                            let path = file.path();

                            let file_path = path.parent().and_then(|parent| {
                                let parent_str = parent.to_string_lossy();

                                if parent_str.is_empty() {
                                    None
                                } else {
                                    Some(
                                        Label::new(format!(
                                            "/{}{}",
                                            parent_str,
                                            std::path::MAIN_SEPARATOR_STR
                                        ))
                                        .color(Color::Muted)
                                        .size(LabelSize::XSmall)
                                        .buffer_font(cx),
                                    )
                                }
                            });

                            let file_name = path.file_name().map(|name| {
                                Label::new(name.to_string_lossy().to_string())
                                    .size(LabelSize::XSmall)
                                    .buffer_font(cx)
                            });

                            let file_icon = FileIcons::get_icon(&path, cx)
                                .map(Icon::from_path)
                                .map(|icon| icon.color(Color::Muted).size(IconSize::Small))
                                .unwrap_or_else(|| {
                                    Icon::new(IconName::File)
                                        .color(Color::Muted)
                                        .size(IconSize::Small)
                                });

                            let overlay_gradient = linear_gradient(
                                90.,
                                linear_color_stop(editor_bg_color, 1.),
                                linear_color_stop(editor_bg_color.opacity(0.2), 0.),
                            );

                            let element = h_flex()
                                .group("edited-code")
                                .id(("file-container", index))
                                .relative()
                                .py_1()
                                .pl_2()
                                .pr_1()
                                .gap_2()
                                .justify_between()
                                .bg(editor_bg_color)
                                .when(index < changed_buffers.len() - 1, |parent| {
                                    parent.border_color(border_color).border_b_1()
                                })
                                .child(
                                    h_flex()
                                        .id(("file-name", index))
                                        .pr_8()
                                        .gap_1p5()
                                        .max_w_full()
                                        .overflow_x_scroll()
                                        .child(file_icon)
                                        .child(
                                            h_flex()
                                                .gap_0p5()
                                                .children(file_name)
                                                .children(file_path),
                                        )
                                        .on_click({
                                            let buffer = buffer.clone();
                                            cx.listener(move |this, _, window, cx| {
                                                this.handle_file_click(buffer.clone(), window, cx);
                                            })
                                        }), // TODO: Implement line diff
                                            // .child(Label::new("+").color(Color::Created))
                                            // .child(Label::new("-").color(Color::Deleted)),
                                            //
                                )
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .visible_on_hover("edited-code")
                                        .child(
                                            Button::new("review", "Review")
                                                .label_size(LabelSize::Small)
                                                .on_click({
                                                    let buffer = buffer.clone();
                                                    cx.listener(move |this, _, window, cx| {
                                                        this.handle_file_click(
                                                            buffer.clone(),
                                                            window,
                                                            cx,
                                                        );
                                                    })
                                                }),
                                        )
                                        .child(
                                            Divider::vertical().color(DividerColor::BorderVariant),
                                        )
                                        .child(
                                            Button::new("reject-file", "Reject")
                                                .label_size(LabelSize::Small)
                                                .disabled(pending_edits)
                                                .on_click({
                                                    let buffer = buffer.clone();
                                                    cx.listener(move |this, _, window, cx| {
                                                        this.handle_reject_file_changes(
                                                            buffer.clone(),
                                                            window,
                                                            cx,
                                                        );
                                                    })
                                                }),
                                        )
                                        .child(
                                            Button::new("accept-file", "Accept")
                                                .label_size(LabelSize::Small)
                                                .disabled(pending_edits)
                                                .on_click({
                                                    let buffer = buffer.clone();
                                                    cx.listener(move |this, _, window, cx| {
                                                        this.handle_accept_file_changes(
                                                            buffer.clone(),
                                                            window,
                                                            cx,
                                                        );
                                                    })
                                                }),
                                        ),
                                )
                                .child(
                                    div()
                                        .id("gradient-overlay")
                                        .absolute()
                                        .h_full()
                                        .w_12()
                                        .top_0()
                                        .bottom_0()
                                        .right(px(152.))
                                        .bg(overlay_gradient),
                                );

                            Some(element)
                        },
                    )),
                )
            })
    }

    fn is_using_zed_provider(&self, cx: &App) -> bool {
        self.thread
            .read(cx)
            .configured_model()
            .map_or(false, |model| model.provider.id() == ZED_CLOUD_PROVIDER_ID)
    }

    fn render_usage_callout(&self, line_height: Pixels, cx: &mut Context<Self>) -> Option<Div> {
        if !self.is_using_zed_provider(cx) {
            return None;
        }

        let user_store = self.project.read(cx).user_store().read(cx);
        if user_store.is_usage_based_billing_enabled() {
            return None;
        }

        let plan = user_store.plan().unwrap_or(cloud_llm_client::Plan::ZedFree);

        let usage = user_store.model_request_usage()?;

        Some(
            div()
                .child(UsageCallout::new(plan, usage))
                .line_height(line_height),
        )
    }

    fn render_token_limit_callout(
        &self,
        line_height: Pixels,
        token_usage_ratio: TokenUsageRatio,
        cx: &mut Context<Self>,
    ) -> Option<Div> {
        let icon = if token_usage_ratio == TokenUsageRatio::Exceeded {
            Icon::new(IconName::X)
                .color(Color::Error)
                .size(IconSize::XSmall)
        } else {
            Icon::new(IconName::Warning)
                .color(Color::Warning)
                .size(IconSize::XSmall)
        };

        let title = if token_usage_ratio == TokenUsageRatio::Exceeded {
            "Thread reached the token limit"
        } else {
            "Thread reaching the token limit soon"
        };

        let description = if self.is_using_zed_provider(cx) {
            "To continue, start a new thread from a summary or turn burn mode on."
        } else {
            "To continue, start a new thread from a summary."
        };

        let mut callout = Callout::new()
            .line_height(line_height)
            .icon(icon)
            .title(title)
            .description(description)
            .primary_action(
                Button::new("start-new-thread", "Start New Thread")
                    .label_size(LabelSize::Small)
                    .on_click(cx.listener(|this, _, window, cx| {
                        let from_thread_id = Some(this.thread.read(cx).id().clone());
                        window.dispatch_action(Box::new(NewThread { from_thread_id }), cx);
                    })),
            );

        if self.is_using_zed_provider(cx) {
            callout = callout.secondary_action(
                IconButton::new("burn-mode-callout", IconName::ZedBurnMode)
                    .icon_size(IconSize::XSmall)
                    .on_click(cx.listener(|this, _event, window, cx| {
                        this.toggle_burn_mode(&ToggleBurnMode, window, cx);
                    })),
            );
        }

        Some(
            div()
                .border_t_1()
                .border_color(cx.theme().colors().border)
                .child(callout),
        )
    }

    pub fn last_estimated_token_count(&self) -> Option<u64> {
        self.last_estimated_token_count
    }

    pub fn is_waiting_to_update_token_count(&self) -> bool {
        self.update_token_count_task.is_some()
    }

    fn reload_context(&mut self, cx: &mut Context<Self>) -> Task<Option<ContextLoadResult>> {
        let load_task = cx.spawn(async move |this, cx| {
            let Ok(load_task) = this.update(cx, |this, cx| {
                let new_context = this
                    .context_store
                    .read(cx)
                    .new_context_for_thread(this.thread.read(cx), None);
                load_context(new_context, &this.project, &this.prompt_store, cx)
            }) else {
                return;
            };
            let result = load_task.await;
            this.update(cx, |this, cx| {
                this.last_loaded_context = Some(result);
                this.load_context_task = None;
                this.message_or_context_changed(false, cx);
            })
            .ok();
        });
        // Replace existing load task, if any, causing it to be cancelled.
        let load_task = load_task.shared();
        self.load_context_task = Some(load_task.clone());
        cx.spawn(async move |this, cx| {
            load_task.await;
            this.read_with(cx, |this, _cx| this.last_loaded_context.clone())
                .ok()
                .flatten()
        })
    }

    fn handle_message_changed(&mut self, cx: &mut Context<Self>) {
        self.message_or_context_changed(true, cx);
    }

    fn message_or_context_changed(&mut self, debounce: bool, cx: &mut Context<Self>) {
        cx.emit(MessageEditorEvent::Changed);
        self.update_token_count_task.take();

        let Some(model) = self.thread.read(cx).configured_model() else {
            self.last_estimated_token_count.take();
            return;
        };

        let editor = self.editor.clone();

        self.update_token_count_task = Some(cx.spawn(async move |this, cx| {
            if debounce {
                cx.background_executor()
                    .timer(Duration::from_millis(200))
                    .await;
            }

            let token_count = if let Some(task) = this
                .update(cx, |this, cx| {
                    let loaded_context = this
                        .last_loaded_context
                        .as_ref()
                        .map(|context_load_result| &context_load_result.loaded_context);
                    let message_text = editor.read(cx).text(cx);

                    if message_text.is_empty()
                        && loaded_context.map_or(true, |loaded_context| loaded_context.is_empty())
                    {
                        return None;
                    }

                    let mut request_message = LanguageModelRequestMessage {
                        role: language_model::Role::User,
                        content: Vec::new(),
                        cache: false,
                    };

                    if let Some(loaded_context) = loaded_context {
                        loaded_context.add_to_request_message(&mut request_message);
                    }

                    if !message_text.is_empty() {
                        request_message
                            .content
                            .push(MessageContent::Text(message_text));
                    }

                    let request = language_model::LanguageModelRequest {
                        thread_id: None,
                        prompt_id: None,
                        intent: None,
                        mode: None,
                        messages: vec![request_message],
                        tools: vec![],
                        tool_choice: None,
                        stop: vec![],
                        temperature: AgentSettings::temperature_for_model(&model.model, cx),
                        thinking_allowed: true,
                    };

                    Some(model.model.count_tokens(request, cx))
                })
                .ok()
                .flatten()
            {
                task.await.log_err()
            } else {
                Some(0)
            };

            this.update(cx, |this, cx| {
                if let Some(token_count) = token_count {
                    this.last_estimated_token_count = Some(token_count);
                    cx.emit(MessageEditorEvent::EstimatedTokenCount);
                }
                this.update_token_count_task.take();
            })
            .ok();
        }));
    }
}

#[derive(Default)]
pub struct ContextCreasesAddon {
    creases: HashMap<AgentContextKey, Vec<(CreaseId, SharedString)>>,
    _subscription: Option<Subscription>,
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

impl Addon for ContextCreasesAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
}

impl ContextCreasesAddon {
    pub fn new() -> Self {
        Self {
            creases: HashMap::default(),
            _subscription: None,
        }
    }

    pub fn add_creases(
        &mut self,
        context_store: &Entity<ContextStore>,
        key: AgentContextKey,
        creases: impl IntoIterator<Item = (CreaseId, SharedString)>,
        cx: &mut Context<Editor>,
    ) {
        self.creases.entry(key).or_default().extend(creases);
        self._subscription = Some(cx.subscribe(
            &context_store,
            |editor, _, event, cx| match event {
                ContextStoreEvent::ContextRemoved(key) => {
                    let Some(this) = editor.addon_mut::<Self>() else {
                        return;
                    };
                    let (crease_ids, replacement_texts): (Vec<_>, Vec<_>) = this
                        .creases
                        .remove(key)
                        .unwrap_or_default()
                        .into_iter()
                        .unzip();
                    let ranges = editor
                        .remove_creases(crease_ids, cx)
                        .into_iter()
                        .map(|(_, range)| range)
                        .collect::<Vec<_>>();
                    editor.unfold_ranges(&ranges, false, false, cx);
                    editor.edit(ranges.into_iter().zip(replacement_texts), cx);
                    cx.notify();
                }
            },
        ))
    }

    pub fn into_inner(self) -> HashMap<AgentContextKey, Vec<(CreaseId, SharedString)>> {
        self.creases
    }
}

pub fn extract_message_creases(
    editor: &mut Editor,
    cx: &mut Context<'_, Editor>,
) -> Vec<MessageCrease> {
    let buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
    let mut contexts_by_crease_id = editor
        .addon_mut::<ContextCreasesAddon>()
        .map(std::mem::take)
        .unwrap_or_default()
        .into_inner()
        .into_iter()
        .flat_map(|(key, creases)| {
            let context = key.0;
            creases
                .into_iter()
                .map(move |(id, _)| (id, context.clone()))
        })
        .collect::<HashMap<_, _>>();
    // Filter the addon's list of creases based on what the editor reports,
    // since the addon might have removed creases in it.
    let creases = editor.display_map.update(cx, |display_map, cx| {
        display_map
            .snapshot(cx)
            .crease_snapshot
            .creases()
            .filter_map(|(id, crease)| {
                Some((
                    id,
                    (
                        crease.range().to_offset(&buffer_snapshot),
                        crease.metadata()?.clone(),
                    ),
                ))
            })
            .map(|(id, (range, metadata))| {
                let context = contexts_by_crease_id.remove(&id);
                MessageCrease {
                    range,
                    context,
                    label: metadata.label,
                    icon_path: metadata.icon_path,
                }
            })
            .collect()
    });
    creases
}

impl EventEmitter<MessageEditorEvent> for MessageEditor {}

pub enum MessageEditorEvent {
    EstimatedTokenCount,
    Changed,
    ScrollThreadToBottom,
}

impl Focusable for MessageEditor {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for MessageEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let thread = self.thread.read(cx);
        let token_usage_ratio = thread
            .total_token_usage()
            .map_or(TokenUsageRatio::Normal, |total_token_usage| {
                total_token_usage.ratio()
            });

        let burn_mode_enabled = thread.completion_mode() == CompletionMode::Burn;

        let action_log = self.thread.read(cx).action_log();
        let changed_buffers = action_log.read(cx).changed_buffers(cx);

        let line_height = TextSize::Small.rems(cx).to_pixels(window.rem_size()) * 1.5;

        let has_configured_providers = LanguageModelRegistry::read_global(cx)
            .providers()
            .iter()
            .filter(|provider| {
                provider.is_authenticated(cx) && provider.id() != ZED_CLOUD_PROVIDER_ID
            })
            .count()
            > 0;

        let is_signed_out = self
            .workspace
            .read_with(cx, |workspace, _| {
                workspace.client().status().borrow().is_signed_out()
            })
            .unwrap_or(true);

        let has_history = self
            .history_store
            .as_ref()
            .and_then(|hs| hs.update(cx, |hs, cx| hs.entries(cx).len() > 0).ok())
            .unwrap_or(false)
            || self
                .thread
                .read_with(cx, |thread, _| thread.messages().len() > 0);

        v_flex()
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .when(
                !has_history && is_signed_out && has_configured_providers,
                |this| this.child(cx.new(ApiKeysWithProviders::new)),
            )
            .when(changed_buffers.len() > 0, |parent| {
                parent.child(self.render_edits_bar(&changed_buffers, window, cx))
            })
            .child(self.render_editor(window, cx))
            .children({
                let usage_callout = self.render_usage_callout(line_height, cx);

                if usage_callout.is_some() {
                    usage_callout
                } else if token_usage_ratio != TokenUsageRatio::Normal && !burn_mode_enabled {
                    self.render_token_limit_callout(line_height, token_usage_ratio, cx)
                } else {
                    None
                }
            })
    }
}

pub fn insert_message_creases(
    editor: &mut Editor,
    message_creases: &[MessageCrease],
    context_store: &Entity<ContextStore>,
    window: &mut Window,
    cx: &mut Context<'_, Editor>,
) {
    let buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
    let creases = message_creases
        .iter()
        .map(|crease| {
            let start = buffer_snapshot.anchor_after(crease.range.start);
            let end = buffer_snapshot.anchor_before(crease.range.end);
            crease_for_mention(
                crease.label.clone(),
                crease.icon_path.clone(),
                start..end,
                cx.weak_entity(),
            )
        })
        .collect::<Vec<_>>();
    let ids = editor.insert_creases(creases.clone(), cx);
    editor.fold_creases(creases, false, window, cx);
    if let Some(addon) = editor.addon_mut::<ContextCreasesAddon>() {
        for (crease, id) in message_creases.iter().zip(ids) {
            if let Some(context) = crease.context.as_ref() {
                let key = AgentContextKey(context.clone());
                addon.add_creases(context_store, key, vec![(id, crease.label.clone())], cx);
            }
        }
    }
}
impl Component for MessageEditor {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn description() -> Option<&'static str> {
        Some(
            "The composer experience of the Agent Panel. This interface handles context, composing messages, switching profiles, models and more.",
        )
    }
}

impl AgentPreview for MessageEditor {
    fn agent_preview(
        workspace: WeakEntity<Workspace>,
        active_thread: Entity<ActiveThread>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        if let Some(workspace) = workspace.upgrade() {
            let fs = workspace.read(cx).app_state().fs.clone();
            let project = workspace.read(cx).project().clone();
            let weak_project = project.downgrade();
            let context_store = cx.new(|_cx| ContextStore::new(weak_project, None));
            let active_thread = active_thread.read(cx);
            let thread = active_thread.thread().clone();
            let thread_store = active_thread.thread_store().clone();
            let text_thread_store = active_thread.text_thread_store().clone();

            let default_message_editor = cx.new(|cx| {
                MessageEditor::new(
                    fs,
                    workspace.downgrade(),
                    context_store,
                    None,
                    thread_store.downgrade(),
                    text_thread_store.downgrade(),
                    None,
                    thread,
                    window,
                    cx,
                )
            });

            Some(
                v_flex()
                    .gap_4()
                    .children(vec![single_example(
                        "Default Message Editor",
                        div()
                            .w(px(540.))
                            .pt_12()
                            .bg(cx.theme().colors().panel_background)
                            .border_1()
                            .border_color(cx.theme().colors().border)
                            .child(default_message_editor.clone())
                            .into_any_element(),
                    )])
                    .into_any_element(),
            )
        } else {
            None
        }
    }
}

register_agent_preview!(MessageEditor);
