use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use crate::agent_model_selector::{AgentModelSelector, ModelType};
use crate::context::{AgentContextKey, ContextCreasesAddon, ContextLoadResult, load_context};
use crate::tool_compatibility::{IncompatibleToolsState, IncompatibleToolsTooltip};
use crate::ui::{
    AnimatedLabel, MaxModeTooltip,
    preview::{AgentPreview, UsageCallout},
};
use crate::context_picker::{ContextPicker, ContextPickerCompletionProvider, crease_for_mention};
use crate::context_store::ContextStore;
use crate::context_strip::{ContextStrip, ContextStripEvent, SuggestContextKind};
use crate::profile_selector::ProfileSelector;
use crate::thread::{MessageCrease, Thread, TokenUsageRatio};
use crate::thread_store::{TextThreadStore, ThreadStore};
use crate::{
    ActiveThread, AgentDiffPane, Chat, ExpandMessageEditor, Follow, NewThread, OpenAgentDiff,
    RemoveAllContext, ToggleContextPicker, ToggleProfileSelector, register_agent_preview,
};

use assistant_settings::{AssistantSettings, CompletionMode};
use buffer_diff::BufferDiff;
use client::UserStore;
use collections::{HashMap, HashSet};
use editor::actions::{MoveUp, Paste};
use editor::{
    AnchorRangeExt, ContextMenuOptions, ContextMenuPlacement, Editor, EditorElement, EditorEvent,
    EditorMode, EditorStyle, MultiBuffer,
};
use file_icons::FileIcons;
use fs::Fs;
use futures::future::Shared;
use futures::{FutureExt as _, future};
use gpui::{
    Animation, AnimationExt, App, ClipboardEntry, Entity, EventEmitter, Focusable, Subscription,
    Task, TextStyle, WeakEntity, linear_color_stop, linear_gradient, point, pulsating_between,
    actions, canvas,
};
use language::{Buffer, Language};
use language_model::{
    ConfiguredModel, LanguageModelRequestMessage, MessageContent, RequestUsage,
    ZED_CLOUD_PROVIDER_ID,
};
use language_model_selector::ToggleModelSelector;
use multi_buffer;
use project::Project;
use prompt_store::PromptStore;
use proto::Plan;
use settings::Settings;
use theme::ThemeSettings;
use ui::{Disclosure, KeyBinding, PopoverMenuHandle, Tooltip, prelude::*};
use util::{ResultExt as _, maybe};
use workspace::{CollaboratorId, Workspace};

// Voice actions
actions!(voice, [ToggleVoiceInput]);

// Voice recording state
#[derive(Clone, Debug)]
pub struct VoiceRecording {
    pub id: String,
    pub duration: Duration,
    pub data: Vec<u8>, // Raw audio data
    pub sample_rate: u32,
    pub channels: u32,
}

#[derive(Clone, Debug)]
pub enum VoiceState {
    Idle,
    Recording { start_time: std::time::Instant },
    Processing,
}

#[derive(Clone, Debug)]
pub struct PlaybackState {
    pub recording_id: String,
    pub start_time: std::time::Instant,
    pub duration: Duration,
    pub original_duration: Duration,
    pub is_playing: bool,
}

#[derive(Clone, Debug)]
pub struct SeekingState {
    pub recording_id: String,
    pub was_playing_before_seek: bool,
    pub seek_position: f32, // 0.0 to 1.0
}

#[derive(RegisterComponent)]
pub struct MessageEditor {
    thread: Entity<Thread>,
    incompatible_tools_state: Entity<IncompatibleToolsState>,
    editor: Entity<Editor>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    user_store: Entity<UserStore>,
    context_store: Entity<ContextStore>,
    prompt_store: Option<Entity<PromptStore>>,
    context_strip: Entity<ContextStrip>,
    context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    model_selector: Entity<AgentModelSelector>,
    last_loaded_context: Option<ContextLoadResult>,
    load_context_task: Option<Shared<Task<()>>>,
    profile_selector: Entity<ProfileSelector>,
    edits_expanded: bool,
    editor_is_expanded: bool,
    last_estimated_token_count: Option<usize>,
    update_token_count_task: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
    // Voice recording state
    voice_state: VoiceState,
    voice_recording_task: Option<Task<()>>,
    current_recording: Option<VoiceRecording>,
    voice_recordings: Vec<VoiceRecording>,
    // Playback state
    playback_state: Option<PlaybackState>,
    playback_update_task: Option<Task<()>>,
    // Seeking state
    seeking_state: Option<SeekingState>,
}

const MAX_EDITOR_LINES: usize = 8;

pub(crate) fn create_editor(
    workspace: WeakEntity<Workspace>,
    context_store: WeakEntity<ContextStore>,
    thread_store: WeakEntity<ThreadStore>,
    text_thread_store: WeakEntity<TextThreadStore>,
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
                max_lines: MAX_EDITOR_LINES,
            },
            buffer,
            None,
            window,
            cx,
        );
        editor.set_placeholder_text("Message the agent – @ to include context", cx);
        editor.set_show_indent_guides(false, cx);
        editor.set_soft_wrap();
        editor.set_context_menu_options(ContextMenuOptions {
            min_entries_visible: 12,
            max_entries_visible: 12,
            placement: Some(ContextMenuPlacement::Above),
        });
        editor.register_addon(ContextCreasesAddon::new());
        editor
    });

    let editor_entity = editor.downgrade();
    editor.update(cx, |editor, _| {
        editor.set_completion_provider(Some(Box::new(ContextPickerCompletionProvider::new(
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
        user_store: Entity<UserStore>,
        context_store: Entity<ContextStore>,
        prompt_store: Option<Entity<PromptStore>>,
        thread_store: WeakEntity<ThreadStore>,
        text_thread_store: WeakEntity<TextThreadStore>,
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
                window,
                cx,
            )
        });

        let incompatible_tools =
            cx.new(|cx| IncompatibleToolsState::new(thread.read(cx).tools().clone(), cx));

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
                ModelType::Default(thread.clone()),
                window,
                cx,
            )
        });

        let profile_selector = cx.new(|cx| {
            ProfileSelector::new(
                fs,
                thread.clone(),
                thread_store,
                editor.focus_handle(cx),
                cx,
            )
        });

        let instance = Self {
            editor: editor.clone(),
            project: thread.read(cx).project().clone(),
            user_store,
            thread,
            incompatible_tools_state: incompatible_tools.clone(),
            workspace,
            context_store,
            prompt_store,
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
            voice_state: VoiceState::Idle,
            voice_recording_task: None,
            current_recording: None,
            voice_recordings: Vec::new(),
            playback_state: None,
            playback_update_task: None,
            seeking_state: None,
        };
        
        instance
    }

    pub fn context_store(&self) -> &Entity<ContextStore> {
        &self.context_store
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
                    max_lines: MAX_EDITOR_LINES,
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
        self.context_store.update(cx, |store, _cx| store.clear());
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

        cx.notify();
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
                    thread.send_to_model(model, Some(window_handle), cx);
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
        } else {
            self.context_strip.focus_handle(cx).focus(window);
        }
    }

    fn paste(&mut self, _: &Paste, _: &mut Window, cx: &mut Context<Self>) {
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

        self.context_store.update(cx, |store, cx| {
            for image in images {
                store.add_image_instance(Arc::new(image), cx);
            }
        });
    }

    fn handle_review_click(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.edits_expanded = true;
        AgentDiffPane::deploy(self.thread.clone(), self.workspace.clone(), window, cx).log_err();
        cx.notify();
    }

    fn handle_file_click(
        &self,
        buffer: Entity<Buffer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Ok(diff) =
            AgentDiffPane::deploy(self.thread.clone(), self.workspace.clone(), window, cx)
        {
            let path_key = multi_buffer::PathKey::for_buffer(&buffer, cx);
            diff.update(cx, |diff, cx| diff.move_to_path(path_key, window, cx));
        }
    }

    fn render_max_mode_toggle(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let thread = self.thread.read(cx);
        let model = thread.configured_model();
        if !model?.model.supports_max_mode() {
            return None;
        }

        let active_completion_mode = thread.completion_mode();
        let max_mode_enabled = active_completion_mode == CompletionMode::Max;

        Some(
            Button::new("max-mode", "Max Mode")
                .label_size(LabelSize::Small)
                .color(Color::Muted)
                .icon(IconName::ZedMaxMode)
                .icon_size(IconSize::Small)
                .icon_color(Color::Muted)
                .icon_position(IconPosition::Start)
                .toggle_state(max_mode_enabled)
                .on_click(cx.listener(move |this, _event, _window, cx| {
                    this.thread.update(cx, |thread, _cx| {
                        thread.set_completion_mode(match active_completion_mode {
                            CompletionMode::Max => CompletionMode::Normal,
                            CompletionMode::Normal => CompletionMode::Max,
                        });
                    });
                }))
                .tooltip(move |_window, cx| {
                    cx.new(|_| MaxModeTooltip::new().selected(max_mode_enabled))
                        .into()
                })
                .into_any_element(),
        )
    }

    fn render_follow_toggle(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let following = self
            .workspace
            .read_with(cx, |workspace, _| {
                workspace.is_being_followed(CollaboratorId::Agent)
            })
            .unwrap_or(false);

        IconButton::new("follow-agent", IconName::Crosshair)
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
            .on_action(cx.listener(|this, _: &ToggleVoiceInput, _window, cx| {
                this.toggle_voice_input(cx);
            }))
            .capture_action(cx.listener(Self::paste))
            .gap_2()
            .p_2()
            .bg(editor_bg_color)
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .items_start()
                    .justify_between()
                    .child(self.context_strip.clone())
                    .children(self.render_voice_indicator(cx))
                    .child(
                        h_flex()
                            .gap_1()
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
                                            window
                                                .dispatch_action(Box::new(ExpandMessageEditor), cx);
                                        })),
                                )
                            })
                            .children(self.render_voice_recordings(cx)),
                    ),
            )
            .child(
                v_flex()
                    .size_full()
                    .gap_4()
                    .when(is_editor_expanded, |this| {
                        this.h(vh(0.8, window)).justify_between()
                    })
                    .child(
                        v_flex()
                            .min_h_16()
                            .when(is_editor_expanded, |this| this.h_full())
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
                            }),
                    )
                    .child(
                        h_flex()
                            .flex_none()
                            .justify_between()
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(self.render_follow_toggle(cx))
                                    .children(self.render_max_mode_toggle(cx)),
                            )
                            .child(
                                h_flex()
                                    .gap_1()
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
                                    .children(self.render_voice_button(cx))
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

    fn render_changed_buffers(
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
        let is_generating = self.thread.read(cx).is_generating();

        v_flex()
            .mt_1()
            .mx_2()
            .bg(bg_edit_files_disclosure)
            .border_1()
            .border_b_0()
            .border_color(border_color)
            .rounded_t_md()
            .shadow(smallvec::smallvec![gpui::BoxShadow {
                color: gpui::black().opacity(0.15),
                offset: point(px(1.), px(-1.)),
                blur_radius: px(3.),
                spread_radius: px(0.),
            }])
            .child(
                h_flex()
                    .id("edits-container")
                    .cursor_pointer()
                    .p_1p5()
                    .justify_between()
                    .when(is_edit_changes_expanded, |this| {
                        this.border_b_1().border_color(border_color)
                    })
                    .on_click(
                        cx.listener(|this, _, window, cx| this.handle_review_click(window, cx)),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                Disclosure::new("edits-disclosure", is_edit_changes_expanded)
                                    .on_click(cx.listener(|this, _ev, _window, cx| {
                                        this.edits_expanded = !this.edits_expanded;
                                        cx.notify();
                                    })),
                            )
                            .map(|this| {
                                if is_generating {
                                    this.child(
                                        AnimatedLabel::new(format!(
                                            "Editing {} {}",
                                            changed_buffers.len(),
                                            if changed_buffers.len() == 1 {
                                                "file"
                                            } else {
                                                "files"
                                            }
                                        ))
                                        .size(LabelSize::Small),
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
                            }),
                    )
                    .child(
                        Button::new("review", "Review Changes")
                            .label_size(LabelSize::Small)
                            .key_binding(
                                KeyBinding::for_action_in(
                                    &OpenAgentDiff,
                                    &focus_handle,
                                    window,
                                    cx,
                                )
                                .map(|kb| kb.size(rems_from_px(12.))),
                            )
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.handle_review_click(window, cx)
                            })),
                    ),
            )
            .when(is_edit_changes_expanded, |parent| {
                parent.child(
                    v_flex().children(changed_buffers.into_iter().enumerate().flat_map(
                        |(index, (buffer, _diff))| {
                            let file = buffer.read(cx).file()?;
                            let path = file.path();

                            let parent_label = path.parent().and_then(|parent| {
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

                            let name_label = path.file_name().map(|name| {
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

                            let hover_color = cx
                                .theme()
                                .colors()
                                .element_background
                                .blend(cx.theme().colors().editor_foreground.opacity(0.025));

                            let overlay_gradient = linear_gradient(
                                90.,
                                linear_color_stop(editor_bg_color, 1.),
                                linear_color_stop(editor_bg_color.opacity(0.2), 0.),
                            );

                            let overlay_gradient_hover = linear_gradient(
                                90.,
                                linear_color_stop(hover_color, 1.),
                                linear_color_stop(hover_color.opacity(0.2), 0.),
                            );

                            let element = h_flex()
                                .group("edited-code")
                                .id(("file-container", index))
                                .cursor_pointer()
                                .relative()
                                .py_1()
                                .pl_2()
                                .pr_1()
                                .gap_2()
                                .justify_between()
                                .bg(cx.theme().colors().editor_background)
                                .hover(|style| style.bg(hover_color))
                                .when(index < changed_buffers.len() - 1, |parent| {
                                    parent.border_color(border_color).border_b_1()
                                })
                                .child(
                                    h_flex()
                                        .id("file-name")
                                        .pr_8()
                                        .gap_1p5()
                                        .max_w_full()
                                        .overflow_x_scroll()
                                        .child(file_icon)
                                        .child(
                                            h_flex()
                                                .gap_0p5()
                                                .children(name_label)
                                                .children(parent_label),
                                        ), // TODO: Implement line diff
                                           // .child(Label::new("+").color(Color::Created))
                                           // .child(Label::new("-").color(Color::Deleted)),
                                )
                                .child(
                                    div().visible_on_hover("edited-code").child(
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
                                    ),
                                )
                                .child(
                                    div()
                                        .id("gradient-overlay")
                                        .absolute()
                                        .h_5_6()
                                        .w_12()
                                        .bottom_0()
                                        .right(px(52.))
                                        .bg(overlay_gradient)
                                        .group_hover("edited-code", |style| {
                                            style.bg(overlay_gradient_hover)
                                        }),
                                )
                                .on_click({
                                    let buffer = buffer.clone();
                                    cx.listener(move |this, _, window, cx| {
                                        this.handle_file_click(buffer.clone(), window, cx);
                                    })
                                });

                            Some(element)
                        },
                    )),
                )
            })
    }

    fn render_usage_callout(&self, line_height: Pixels, cx: &mut Context<Self>) -> Option<Div> {
        let is_using_zed_provider = self
            .thread
            .read(cx)
            .configured_model()
            .map_or(false, |model| {
                model.provider.id().0 == ZED_CLOUD_PROVIDER_ID
            });
        if !is_using_zed_provider {
            return None;
        }

        let user_store = self.user_store.read(cx);

        let ubb_enable = user_store
            .usage_based_billing_enabled()
            .map_or(false, |enabled| enabled);

        if ubb_enable {
            return None;
        }

        let plan = user_store
            .current_plan()
            .map(|plan| match plan {
                Plan::Free => zed_llm_client::Plan::ZedFree,
                Plan::ZedPro => zed_llm_client::Plan::ZedPro,
                Plan::ZedProTrial => zed_llm_client::Plan::ZedProTrial,
            })
            .unwrap_or(zed_llm_client::Plan::ZedFree);
        let usage = self.thread.read(cx).last_usage().or_else(|| {
            maybe!({
                let amount = user_store.model_request_usage_amount()?;
                let limit = user_store.model_request_usage_limit()?.variant?;

                Some(RequestUsage {
                    amount: amount as i32,
                    limit: match limit {
                        proto::usage_limit::Variant::Limited(limited) => {
                            zed_llm_client::UsageLimit::Limited(limited.limit as i32)
                        }
                        proto::usage_limit::Variant::Unlimited(_) => {
                            zed_llm_client::UsageLimit::Unlimited
                        }
                    },
                })
            })
        })?;

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
        let title = if token_usage_ratio == TokenUsageRatio::Exceeded {
            "Thread reached the token limit"
        } else {
            "Thread reaching the token limit soon"
        };

        let message = "Start a new thread from a summary to continue the conversation.";

        let icon = if token_usage_ratio == TokenUsageRatio::Exceeded {
            Icon::new(IconName::X)
                .color(Color::Error)
                .size(IconSize::XSmall)
        } else {
            Icon::new(IconName::Warning)
                .color(Color::Warning)
                .size(IconSize::XSmall)
        };

        Some(
            div()
                .child(ui::Callout::multi_line(
                    title,
                    message,
                    icon,
                    "Start New Thread",
                    Box::new(cx.listener(|this, _, window, cx| {
                        let from_thread_id = Some(this.thread.read(cx).id().clone());
                        window.dispatch_action(Box::new(NewThread { from_thread_id }), cx);
                    })),
                ))
                .line_height(line_height),
        )
    }

    pub fn last_estimated_token_count(&self) -> Option<usize> {
        self.last_estimated_token_count
    }

    pub fn is_waiting_to_update_token_count(&self) -> bool {
        self.update_token_count_task.is_some()
    }

    fn reload_context(&mut self, cx: &mut Context<Self>) -> Task<Option<ContextLoadResult>> {
        let load_task = cx.spawn(async move |this, cx| {
            let Ok(load_task) = this.update(cx, |this, cx| {
                let new_context = this.context_store.read_with(cx, |context_store, cx| {
                    context_store.new_context_for_thread(this.thread.read(cx), None)
                });
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
                        mode: None,
                        messages: vec![request_message],
                        tools: vec![],
                        tool_choice: None,
                        stop: vec![],
                        temperature: AssistantSettings::temperature_for_model(&model.model, cx),
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

    fn render_voice_indicator(&self, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        if !matches!(self.voice_state, VoiceState::Recording { .. }) {
            return None;
        }

        Some(
            h_flex()
                .gap_2()
                .items_center()
                .p_2()
                .bg(cx.theme().colors().editor_background)
                .border_1()
                .border_color(cx.theme().colors().border)
                .rounded_md()
                .child(
                    h_flex()
                        .gap_1()
                        .items_center()
                        .child(
                            Icon::new(IconName::Mic)
                                .size(IconSize::Small)
                                .color(Color::Success)
                                .with_animation(
                                    "pulsating-mic",
                                    Animation::new(Duration::from_secs(1))
                                        .repeat()
                                        .with_easing(pulsating_between(0.4, 1.0)),
                                    |icon, _delta| icon.color(Color::Success),
                                )
                        )
                        .child(
                            Label::new("Recording...")
                                .size(LabelSize::Small)
                                .color(Color::Success)
                        )
                )
        )
    }

    fn render_voice_recordings(&self, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        if self.voice_recordings.is_empty() {
            return None;
        }

        Some(
            v_flex()
                .gap_2()
                .p_2()
                .bg(cx.theme().colors().editor_background)
                .border_1()
                .border_color(cx.theme().colors().border)
                .rounded_md()
                .child(
                    Label::new("Voice Recordings")
                        .size(LabelSize::Small)
                        .color(Color::Muted)
                )
                .children(
                    self.voice_recordings.iter().map(|recording| {
                        let recording_id = recording.id.clone();
                        let is_playing = self.playback_state.as_ref()
                            .map(|state| state.recording_id == recording_id && state.is_playing)
                            .unwrap_or(false);
                        
                        let is_paused = self.playback_state.as_ref()
                            .map(|state| state.recording_id == recording_id && !state.is_playing)
                            .unwrap_or(false);
                        
                        let (progress, current_time) = if let Some(playback_state) = &self.playback_state {
                            if playback_state.recording_id == recording_id {
                                // Check if we're currently seeking this recording
                                if let Some(seeking_state) = &self.seeking_state {
                                    if seeking_state.recording_id == recording_id {
                                        // Show seeking position
                                        let seek_time = Duration::from_secs_f32(recording.duration.as_secs_f32() * seeking_state.seek_position);
                                        (seeking_state.seek_position, seek_time)
                                    } else {
                                        // Not seeking this recording, show normal playback progress
                                        if playback_state.is_playing {
                                            let elapsed = playback_state.start_time.elapsed();
                                            let played_duration = playback_state.original_duration.saturating_sub(playback_state.duration) + elapsed;
                                            let progress = (played_duration.as_secs_f32() / playback_state.original_duration.as_secs_f32()).min(1.0);
                                            (progress, played_duration)
                                        } else {
                                            // Paused state - calculate progress based on remaining duration
                                            let played_duration = playback_state.original_duration.saturating_sub(playback_state.duration);
                                            let progress = (played_duration.as_secs_f32() / playback_state.original_duration.as_secs_f32()).min(1.0);
                                            (progress, played_duration)
                                        }
                                    }
                                } else {
                                    // Not seeking, show normal playback progress
                                    if playback_state.is_playing {
                                        let elapsed = playback_state.start_time.elapsed();
                                        let played_duration = playback_state.original_duration.saturating_sub(playback_state.duration) + elapsed;
                                        let progress = (played_duration.as_secs_f32() / playback_state.original_duration.as_secs_f32()).min(1.0);
                                        (progress, played_duration)
                                    } else {
                                        // Paused state - calculate progress based on remaining duration
                                        let played_duration = playback_state.original_duration.saturating_sub(playback_state.duration);
                                        let progress = (played_duration.as_secs_f32() / playback_state.original_duration.as_secs_f32()).min(1.0);
                                        (progress, played_duration)
                                    }
                                }
                            } else {
                                (0.0, Duration::ZERO)
                            }
                        } else {
                            (0.0, Duration::ZERO)
                        };
                        
                        let icon_name = if is_playing {
                            IconName::Stop
                        } else if is_paused {
                            IconName::Play
                        } else {
                            IconName::Play
                        };
                        
                        let icon_color = if is_playing {
                            Color::Warning
                        } else if is_paused {
                            Color::Success
                        } else {
                            Color::Accent
                        };
                        
                        v_flex()
                            .gap_1()
                            .p_2()
                            .rounded_sm()
                            .hover(|style| style.bg(cx.theme().colors().element_hover))
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .child(
                                        // Separate play/pause button
                                        div()
                                            .cursor_pointer()
                                            .child(
                                                Icon::new(icon_name)
                                                    .size(IconSize::XSmall)
                                                    .color(icon_color)
                                            )
                                            .on_mouse_down(gpui::MouseButton::Left, {
                                                let recording_id = recording_id.clone();
                                                cx.listener(move |this, _event, _window, cx| {
                                                    this.toggle_voice_playback(recording_id.clone(), cx);
                                                })
                                            })
                                    )
                                    .child(
                                        Label::new(format!("{:.1}s", recording.duration.as_secs_f32()))
                                            .size(LabelSize::XSmall)
                                            .color(Color::Default)
                                    )
                                    .child(
                                        Label::new(&recording.id)
                                            .size(LabelSize::XSmall)
                                            .color(Color::Muted)
                                    )
                            )
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        // Progress bar - separate from play button with exact bounds calculation
                                        canvas(
                                            {
                                                let recording_id = recording_id.clone();
                                                move |bounds, _window, _cx| (bounds, recording_id.clone())
                                            },
                                            {
                                                let recording_id = recording_id.clone();
                                                let progress = progress;
                                                let is_playing = is_playing;
                                                let entity = cx.entity().clone();
                                                move |bounds, (_bounds_data, _recording_id), window, cx| {
                                                    // Draw the progress bar background
                                                    window.paint_quad(gpui::fill(
                                                        bounds,
                                                        cx.theme().colors().element_background,
                                                    ));
                                                    
                                                    // Draw the progress fill
                                                    let progress_width = bounds.size.width * progress;
                                                    let progress_bounds = gpui::Bounds {
                                                        origin: bounds.origin,
                                                        size: gpui::Size {
                                                            width: progress_width,
                                                            height: bounds.size.height,
                                                        },
                                                    };
                                                    
                                                    let fill_color = if is_playing { 
                                                        cx.theme().colors().text_accent 
                                                    } else { 
                                                        cx.theme().colors().element_disabled 
                                                    };
                                                    
                                                    window.paint_quad(gpui::fill(progress_bounds, fill_color));
                                                    
                                                    // Handle mouse events with exact bounds
                                                    window.on_mouse_event({
                                                        let recording_id = recording_id.clone();
                                                        let entity = entity.clone();
                                                        move |event: &gpui::MouseDownEvent, _phase, _window, cx| {
                                                            if bounds.contains(&event.position) {
                                                                // Calculate exact relative position within the progress bar
                                                                let relative_x = event.position.x - bounds.origin.x;
                                                                let relative_position = (relative_x.0 / bounds.size.width.0).clamp(0.0, 1.0);
                                                                
                                                                // Update the MessageEditor entity - pause and seek
                                                                entity.update(cx, |this, cx| {
                                                                    this.start_seek_voice_playback(recording_id.clone(), relative_position, cx);
                                                                });
                                                            }
                                                        }
                                                    });
                                                    
                                                    // Handle mouse move for continuous seeking while dragging
                                                    window.on_mouse_event({
                                                        let recording_id = recording_id.clone();
                                                        let entity = entity.clone();
                                                        move |event: &gpui::MouseMoveEvent, _phase, _window, cx| {
                                                            // Only seek if we're currently seeking this recording and mouse is pressed
                                                            if event.pressed_button == Some(gpui::MouseButton::Left) {
                                                                entity.update(cx, |this, cx| {
                                                                    // Only update if we're actually seeking this recording
                                                                    if let Some(seeking_state) = &this.seeking_state {
                                                                        if seeking_state.recording_id == recording_id {
                                                                            // Calculate relative position, clamping to bounds even if mouse is outside
                                                                            let relative_x = event.position.x - bounds.origin.x;
                                                                            let relative_position = (relative_x.0 / bounds.size.width.0).clamp(0.0, 1.0);
                                                                            
                                                                            this.update_seek_position(recording_id.clone(), relative_position, cx);
                                                                        }
                                                                    }
                                                                });
                                                            }
                                                        }
                                                    });
                                                    
                                                    // Handle mouse up to resume playback after seeking
                                                    window.on_mouse_event({
                                                        let recording_id = recording_id.clone();
                                                        let entity = entity.clone();
                                                        move |_event: &gpui::MouseUpEvent, _phase, _window, cx| {
                                                            // Resume playback after seeking - don't check bounds since user might drag outside and release
                                                            entity.update(cx, |this, cx| {
                                                                // Only end seeking if we're actually seeking this recording
                                                                if let Some(seeking_state) = &this.seeking_state {
                                                                    if seeking_state.recording_id == recording_id {
                                                                        log::info!("🖱️ Mouse up detected for recording: {}", recording_id);
                                                                        this.end_seek_voice_playback(recording_id.clone(), cx);
                                                                    }
                                                                } else {
                                                                    log::debug!("🖱️ Mouse up but no seeking state for recording: {}", recording_id);
                                                                }
                                                            });
                                                        }
                                                    });
                                                }
                                            }
                                        )
                                        .h_1p5()
                                        .w_full()
                                        .rounded_sm()
                                        .cursor_pointer()
                                    )
                            )
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(
                                        Label::new(format!("{:.1}s", current_time.as_secs_f32()))
                                            .size(LabelSize::XSmall)
                                            .color(Color::Muted)
                                    )
                                    .child(
                                        Label::new(format!("{:.1}s", recording.duration.as_secs_f32()))
                                            .size(LabelSize::XSmall)
                                            .color(Color::Muted)
                                    )
                            )
                    })
                )
        )
    }

    fn toggle_voice_playback(&mut self, recording_id: String, cx: &mut Context<Self>) {
        // Check if this recording is currently playing
        if let Some(playback_state) = &self.playback_state {
            if playback_state.recording_id == recording_id {
                if playback_state.is_playing {
                    // Pause current playback
                    self.pause_voice_playback(cx);
                } else {
                    // Resume paused playback
                    self.resume_voice_playback(recording_id, cx);
                }
                return;
            } else {
                // Different recording is playing, stop it and start new one
                self.stop_voice_playback(cx);
            }
        }
        
        // Start playback for new recording
        self.start_voice_playback(recording_id, cx);
    }

    fn start_voice_playback(&mut self, recording_id: String, cx: &mut Context<Self>) {
        if let Some(recording) = self.voice_recordings.iter().find(|r| r.id == recording_id).cloned() {
            log::info!("Starting playback of voice recording: {} ({}s)", recording.id, recording.duration.as_secs_f32());
            
            // Stop any existing playback
            self.stop_voice_playback(cx);
            
            // Clone recording_id for use in async closures
            let recording_id_for_update = recording_id.clone();
            
            // Set up new playback state
            self.playback_state = Some(PlaybackState {
                recording_id: recording_id.clone(),
                start_time: std::time::Instant::now(),
                duration: recording.duration,
                original_duration: recording.duration,
                is_playing: true,
            });
            
            // Start playback update task
            self.playback_update_task = Some(cx.spawn(async move |this, cx| {
                let update_interval = Duration::from_millis(50); // Update every 50ms for smooth progress
                
                loop {
                    smol::Timer::after(update_interval).await;
                    
                    let should_continue = this.update(cx, |this, cx| {
                        if let Some(playback_state) = &this.playback_state {
                            if playback_state.recording_id == recording_id_for_update && playback_state.is_playing {
                                let elapsed = playback_state.start_time.elapsed();
                                
                                if elapsed >= playback_state.duration {
                                    // Playback finished
                                    log::info!("Playback completed for recording: {}", recording_id_for_update);
                                    this.stop_voice_playback(cx);
                                    return false;
                                }
                                
                                // Update UI
                                cx.notify();
                                return true;
                            }
                        }
                        false
                    }).unwrap_or(false);
                    
                    if !should_continue {
                        break;
                    }
                }
            }));
            
            cx.notify();
        }
    }

    fn stop_voice_playback(&mut self, cx: &mut Context<Self>) {
        if let Some(playback_state) = &self.playback_state {
            log::info!("Stopping playback of voice recording: {}", playback_state.recording_id);
        }
        
        self.playback_state = None;
        self.playback_update_task.take();
        cx.notify();
    }

    fn pause_voice_playback(&mut self, cx: &mut Context<Self>) {
        if let Some(playback_state) = &mut self.playback_state {
            if playback_state.is_playing {
                log::info!("Pausing playback of voice recording: {}", playback_state.recording_id);
                playback_state.is_playing = false;
                
                // Calculate how much time has elapsed and adjust the start time
                let elapsed = playback_state.start_time.elapsed();
                playback_state.duration = playback_state.duration.saturating_sub(elapsed);
                
                // Stop the update task
                self.playback_update_task.take();
                
                cx.notify();
            }
        }
    }

    fn resume_voice_playback(&mut self, recording_id: String, cx: &mut Context<Self>) {
        if let Some(playback_state) = &mut self.playback_state {
            if playback_state.recording_id == recording_id && !playback_state.is_playing {
                log::info!("Resuming playback of voice recording: {}", playback_state.recording_id);
                
                // Reset start time for remaining duration
                playback_state.start_time = std::time::Instant::now();
                playback_state.is_playing = true;
                
                // Restart the update task
                let recording_id_for_update = recording_id.clone();
                self.playback_update_task = Some(cx.spawn(async move |this, cx| {
                    let update_interval = Duration::from_millis(50);
                    
                    loop {
                        smol::Timer::after(update_interval).await;
                        
                        let should_continue = this.update(cx, |this, cx| {
                            if let Some(playback_state) = &this.playback_state {
                                if playback_state.recording_id == recording_id_for_update && playback_state.is_playing {
                                    let elapsed = playback_state.start_time.elapsed();
                                    
                                    if elapsed >= playback_state.duration {
                                        // Playback finished
                                        log::info!("Playback completed for recording: {}", recording_id_for_update);
                                        this.stop_voice_playback(cx);
                                        return false;
                                    }
                                    
                                    // Update UI
                                    cx.notify();
                                    return true;
                                }
                            }
                            false
                        }).unwrap_or(false);
                        
                        if !should_continue {
                            break;
                        }
                    }
                }));
                
                cx.notify();
            }
        }
    }

    fn seek_voice_playback_with_position(&mut self, recording_id: String, relative_position: f32, cx: &mut Context<Self>) {
        if let Some(recording) = self.voice_recordings.iter().find(|r| r.id == recording_id).cloned() {
            let target_time = Duration::from_secs_f32(recording.duration.as_secs_f32() * relative_position);
            
            log::info!("Seeking to position {:.1}s in recording: {}", target_time.as_secs_f32(), recording.id);
            
            // Update playback state to new position
            if let Some(playback_state) = &mut self.playback_state {
                if playback_state.recording_id == recording_id {
                    playback_state.duration = recording.duration.saturating_sub(target_time);
                    playback_state.start_time = std::time::Instant::now();
                    
                    // If not currently playing, start playback from seek position
                    if !playback_state.is_playing {
                        self.resume_voice_playback(recording_id, cx);
                    } else {
                        // If playing, restart the update task with new position
                        self.playback_update_task.take();
                        let recording_id_for_update = recording_id.clone();
                        self.playback_update_task = Some(cx.spawn(async move |this, cx| {
                            let update_interval = Duration::from_millis(50);
                            
                            loop {
                                smol::Timer::after(update_interval).await;
                                
                                let should_continue = this.update(cx, |this, cx| {
                                    if let Some(playback_state) = &this.playback_state {
                                        if playback_state.recording_id == recording_id_for_update && playback_state.is_playing {
                                            let elapsed = playback_state.start_time.elapsed();
                                            
                                            if elapsed >= playback_state.duration {
                                                log::info!("Playback completed for recording: {}", recording_id_for_update);
                                                this.stop_voice_playback(cx);
                                                return false;
                                            }
                                            
                                            cx.notify();
                                            return true;
                                        }
                                    }
                                    false
                                }).unwrap_or(false);
                                
                                if !should_continue {
                                    break;
                                }
                            }
                        }));
                    }
                } else {
                    // Different recording, start new playback at seek position
                    self.stop_voice_playback(cx);
                    self.start_voice_playback_at_position(recording_id, target_time, cx);
                }
            } else {
                // No current playback, start at seek position
                self.start_voice_playback_at_position(recording_id, target_time, cx);
            }
            
            cx.notify();
        }
    }

    fn start_seek_voice_playback(&mut self, recording_id: String, relative_position: f32, cx: &mut Context<Self>) {
        if let Some(recording) = self.voice_recordings.iter().find(|r| r.id == recording_id).cloned() {
            // Guard: Don't start seeking if we're already seeking this recording
            if let Some(existing_seeking_state) = &self.seeking_state {
                if existing_seeking_state.recording_id == recording_id {
                    log::debug!("🎯 Already seeking recording: {}, ignoring duplicate start_seek call", recording_id);
                    return;
                }
            }
            
            // Check if this recording is currently playing
            let was_playing = self.playback_state.as_ref()
                .map(|state| state.recording_id == recording_id && state.is_playing)
                .unwrap_or(false);
            
            // Pause playback if it was playing
            if was_playing {
                log::info!("🔇 Pausing playback for seeking: {}", recording_id);
                self.pause_voice_playback(cx);
            }
            
            // Set seeking state
            self.seeking_state = Some(SeekingState {
                recording_id: recording_id.clone(),
                was_playing_before_seek: was_playing,
                seek_position: relative_position,
            });
            
            // Update playback position
            let target_time = Duration::from_secs_f32(recording.duration.as_secs_f32() * relative_position);
            
            if let Some(playback_state) = &mut self.playback_state {
                if playback_state.recording_id == recording_id {
                    // Update existing playback state to new position
                    playback_state.duration = recording.duration.saturating_sub(target_time);
                    playback_state.start_time = std::time::Instant::now();
                    playback_state.is_playing = false; // Paused during seek
                } else {
                    // Different recording, create new playback state
                    self.stop_voice_playback(cx);
                    self.playback_state = Some(PlaybackState {
                        recording_id: recording_id.clone(),
                        start_time: std::time::Instant::now(),
                        duration: recording.duration.saturating_sub(target_time),
                        original_duration: recording.duration,
                        is_playing: false,
                    });
                }
            } else {
                // No current playback, create new state at seek position
                self.playback_state = Some(PlaybackState {
                    recording_id: recording_id.clone(),
                    start_time: std::time::Instant::now(),
                    duration: recording.duration.saturating_sub(target_time),
                    original_duration: recording.duration,
                    is_playing: false,
                });
            }
            
            log::info!("🎯 Started seeking to position {:.1}s in recording: {} (was_playing: {})", 
                target_time.as_secs_f32(), recording_id, was_playing);
            
            cx.notify();
        }
    }

    fn end_seek_voice_playback(&mut self, recording_id: String, cx: &mut Context<Self>) {
        if let Some(seeking_state) = self.seeking_state.take() {
            if seeking_state.recording_id == recording_id {
                log::info!("🎯 Ended seeking for recording: {} (was_playing: {})", 
                    recording_id, seeking_state.was_playing_before_seek);
                
                // Resume playback if it was playing before seeking
                if seeking_state.was_playing_before_seek {
                    log::info!("▶️ Resuming playback after seeking: {}", recording_id);
                    self.resume_voice_playback(recording_id, cx);
                } else {
                    log::info!("⏸️ Staying paused after seeking: {}", recording_id);
                }
                
                cx.notify();
            } else {
                // Put the seeking state back if it's for a different recording
                self.seeking_state = Some(seeking_state);
            }
        }
    }

    fn update_seek_position(&mut self, recording_id: String, relative_position: f32, cx: &mut Context<Self>) {
        // Only update if we're currently seeking this recording
        if let Some(seeking_state) = &mut self.seeking_state {
            if seeking_state.recording_id == recording_id {
                // Update the seek position
                seeking_state.seek_position = relative_position;
                
                // Update the playback state to reflect the new position
                if let Some(recording) = self.voice_recordings.iter().find(|r| r.id == recording_id).cloned() {
                    let target_time = Duration::from_secs_f32(recording.duration.as_secs_f32() * relative_position);
                    
                    if let Some(playback_state) = &mut self.playback_state {
                        if playback_state.recording_id == recording_id {
                            // Update playback position
                            playback_state.duration = recording.duration.saturating_sub(target_time);
                            playback_state.start_time = std::time::Instant::now();
                            // Keep is_playing as false during seeking
                        }
                    }
                    
                    log::debug!("🎯 Continuous seek to {:.1}s ({:.1}%)", target_time.as_secs_f32(), relative_position * 100.0);
                }
                
                // Notify for UI update
                cx.notify();
            }
        }
    }

    fn render_voice_button(&self, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        let is_recording = matches!(self.voice_state, VoiceState::Recording { .. });
        
        Some(
            IconButton::new("voice-toggle", IconName::Mic)
                .icon_size(IconSize::Small)
                .icon_color(if is_recording { Color::Success } else { Color::Muted })
                .style(if is_recording { 
                    ButtonStyle::Tinted(ui::TintColor::Success) 
                } else { 
                    ButtonStyle::Subtle 
                })
                .tooltip(move |_window, _cx| {
                    Tooltip::text(if is_recording { 
                        "Stop Recording" 
                    } else { 
                        "Start Recording" 
                    })(_window, _cx)
                })
                .on_click(cx.listener(move |this, _, _window, cx| {
                    this.toggle_voice_input(cx);
                }))
        )
    }

    fn toggle_voice_input(&mut self, cx: &mut Context<Self>) {
        match self.voice_state {
            VoiceState::Idle => {
                self.start_voice_recording(cx);
            }
            VoiceState::Recording { start_time } => {
                self.stop_voice_recording(start_time, cx);
            }
            VoiceState::Processing => {
                // Do nothing while processing
            }
        }
    }

    fn start_voice_recording(&mut self, cx: &mut Context<Self>) {
        self.voice_state = VoiceState::Recording { 
            start_time: std::time::Instant::now() 
        };
        
        log::info!("Started voice recording");
        
        // Start recording task
        self.voice_recording_task = Some(cx.spawn(async move |this, cx| {
            // Simulate recording for now - in a real implementation this would:
            // 1. Use livekit_client::AudioStack to capture microphone
            // 2. Stream audio data to a buffer
            // 3. Handle audio processing
            
            // For now, just wait and simulate recording
            smol::Timer::after(Duration::from_millis(100)).await;
            
            this.update(cx, |_this, cx| {
                // Update UI to show recording is active
                cx.notify();
            }).ok();
        }));
        
        cx.notify();
    }

    fn stop_voice_recording(&mut self, start_time: std::time::Instant, cx: &mut Context<Self>) {
        let duration = std::time::Instant::now() - start_time;
        self.voice_state = VoiceState::Processing;
        
        log::info!("Stopped voice recording, duration: {:?}", duration);
        
        // Cancel the recording task
        self.voice_recording_task.take();
        
        // Create a voice recording
        let recording = VoiceRecording {
            id: format!("recording_{}", std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()),
            duration,
            data: vec![0u8; (duration.as_secs_f32() * 44100.0 * 2.0) as usize], // Simulate audio data
            sample_rate: 44100,
            channels: 1,
        };
        
        // Add to recordings list
        self.voice_recordings.push(recording.clone());
        self.current_recording = Some(recording.clone());
        
        // Insert voice recording into the chat
        self.insert_voice_recording(recording, cx);
        
        // Reset state
        self.voice_state = VoiceState::Idle;
        self.current_recording = None;
        
        cx.notify();
    }

    fn insert_voice_recording(&mut self, recording: VoiceRecording, cx: &mut Context<Self>) {
        // Insert a voice message representation into the editor
        let voice_text = format!(
            "🎤 Voice Recording ({:.1}s) [{}]\n", 
            recording.duration.as_secs_f32(),
            recording.id
        );
        
        self.editor.update(cx, |editor, cx| {
            let cursor_position = editor.selections.newest::<usize>(cx).head();
            
            editor.buffer().update(cx, |buffer, cx| {
                buffer.edit([(cursor_position..cursor_position, voice_text.as_str())], None, cx);
            });
        });
        
        log::info!("Inserted voice recording: {} ({}s)", recording.id, recording.duration.as_secs_f32());
    }

    fn start_voice_playback_at_position(&mut self, recording_id: String, start_position: Duration, cx: &mut Context<Self>) {
        if let Some(recording) = self.voice_recordings.iter().find(|r| r.id == recording_id).cloned() {
            log::info!("Starting playback of voice recording: {} at position {:.1}s", recording.id, start_position.as_secs_f32());
            
            // Stop any existing playback
            self.stop_voice_playback(cx);
            
            // Calculate remaining duration from start position
            let remaining_duration = recording.duration.saturating_sub(start_position);
            
            // Clone recording_id for use in async closures
            let recording_id_for_update = recording_id.clone();
            
            // Set up new playback state
            self.playback_state = Some(PlaybackState {
                recording_id: recording_id.clone(),
                start_time: std::time::Instant::now(),
                duration: remaining_duration,
                original_duration: recording.duration,
                is_playing: true,
            });
            
            // Start playback update task
            self.playback_update_task = Some(cx.spawn(async move |this, cx| {
                let update_interval = Duration::from_millis(50);
                
                loop {
                    smol::Timer::after(update_interval).await;
                    
                    let should_continue = this.update(cx, |this, cx| {
                        if let Some(playback_state) = &this.playback_state {
                            if playback_state.recording_id == recording_id_for_update && playback_state.is_playing {
                                let elapsed = playback_state.start_time.elapsed();
                                
                                if elapsed >= playback_state.duration {
                                    log::info!("Playback completed for recording: {}", recording_id_for_update);
                                    this.stop_voice_playback(cx);
                                    return false;
                                }
                                
                                cx.notify();
                                return true;
                            }
                        }
                        false
                    }).unwrap_or(false);
                    
                    if !should_continue {
                        break;
                    }
                }
            }));
            
            cx.notify();
        }
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
                    metadata,
                    context,
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

        let action_log = self.thread.read(cx).action_log();
        let changed_buffers = action_log.read(cx).changed_buffers(cx);

        let line_height = TextSize::Small.rems(cx).to_pixels(window.rem_size()) * 1.5;

        v_flex()
            .size_full()
            .when(changed_buffers.len() > 0, |parent| {
                parent.child(self.render_changed_buffers(&changed_buffers, window, cx))
            })
            .child(self.render_editor(window, cx))
            .children({
                let usage_callout = self.render_usage_callout(line_height, cx);

                if usage_callout.is_some() {
                    usage_callout
                } else if token_usage_ratio != TokenUsageRatio::Normal {
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
                crease.metadata.label.clone(),
                crease.metadata.icon_path.clone(),
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
                addon.add_creases(
                    context_store,
                    key,
                    vec![(id, crease.metadata.label.clone())],
                    cx,
                );
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
            let user_store = workspace.read(cx).app_state().user_store.clone();
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
                    user_store,
                    context_store,
                    None,
                    thread_store.downgrade(),
                    text_thread_store.downgrade(),
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
