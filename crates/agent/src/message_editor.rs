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
use media_player::{VoicePlayer, VoiceRecorder, VoiceRecording, VoicePlayerEvent};
use multi_buffer;
use project::Project;
use prompt_store::PromptStore;
use proto::Plan;
use settings::Settings;
use theme::ThemeSettings;
use ui::{Disclosure, KeyBinding, PopoverMenuHandle, PopoverMenu, Tooltip, prelude::*};
use ui::ContextMenu;
use util::{ResultExt as _, maybe};
use workspace::{CollaboratorId, Workspace};

// Voice actions
actions!(voice, [ToggleVoiceInput]);

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
    // Voice recording and playback using media_player
    voice_player: VoicePlayer,
    voice_recorder: VoiceRecorder,
    playback_update_task: Option<Task<()>>,
    playback_speed_menu_handle: PopoverMenuHandle<ContextMenu>,
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
            voice_player: VoicePlayer::with_executor(cx.background_executor().clone()),
            voice_recorder: VoiceRecorder::with_executor(cx.background_executor().clone()),
            playback_update_task: None,
            playback_speed_menu_handle: PopoverMenuHandle::default(),
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
            let message_text = editor.text(cx);
            editor.clear(window, cx);
            (message_text, creases)
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
                            }),
                    ),
            )
            .children(self.render_voice_recordings(cx))
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
            }).ok();
        }));
    }

    fn render_voice_indicator(&self, _cx: &mut Context<Self>) -> Option<impl IntoElement> {
        if !self.voice_recorder.is_recording() {
            return None;
        }

        let duration = self.voice_recorder.get_recording_duration().unwrap_or(Duration::ZERO);
        
        Some(
            div()
                .flex()
                .items_center()
                .gap_2()
                .p_2()
                .bg(gpui::red().alpha(0.1))
                .border_1()
                .border_color(gpui::red())
                .rounded_md()
                .child(
                    div()
                        .size_2()
                        .bg(gpui::red())
                        .rounded_full()
                        .with_animation(
                            "pulse",
                            Animation::new(Duration::from_millis(1000))
                                .repeat()
                                .with_easing(pulsating_between(0.4, 1.0)),
                            |div, delta| div.opacity(delta),
                        )
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(gpui::red())
                        .child(format!("Recording... {:.1}s", duration.as_secs_f32()))
                )
        )
    }

    fn render_voice_recordings(&self, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        let recordings = self.voice_player.get_recordings();
        if recordings.is_empty() {
            return None;
        }

        Some(
            div()
                .child(
                    v_flex()
                        .gap_2()
                        .w_full()
                        .min_w_0()
                        .p_3()
                        .bg(cx.theme().colors().editor_background)
                        .border_1()
                        .border_color(cx.theme().colors().border)
                        .rounded_md()
                        .child(
                            h_flex()
                                .justify_between()
                                .items_center()
                                .child(
                                    Label::new("Voice Recordings")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                )
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .items_center()
                                        .child(
                                            // Playback speed control
                                            Button::new("playback_speed", format!("{:.1}x", self.voice_player.get_playback_speed()))
                                                .label_size(LabelSize::XSmall)
                                                .style(ButtonStyle::Subtle)
                                                .tooltip(move |window, cx| {
                                                    Tooltip::text("Click to change playback speed")(window, cx)
                                                })
                                                .on_click(cx.listener(|this, _event, window, cx| {
                                                    this.playback_speed_menu_handle.toggle(window, cx);
                                                }))
                                        )
                                        .child(
                                            Label::new(format!("{} recording{}", recordings.len(), if recordings.len() == 1 { "" } else { "s" }))
                                                .size(LabelSize::XSmall)
                                                .color(Color::Disabled)
                                        )
                                )
                        )
                        .children(
                            recordings.iter().enumerate().map(|(index, (_recording_id, recording))| {
                                let recording_id = recording.id.clone();
                                let is_playing = self.voice_player.is_playing(&recording_id);
                                let is_paused = self.voice_player.is_paused(&recording_id);
                                
                                let (progress, current_time) = self.voice_player.get_progress(&recording_id)
                                    .unwrap_or((0.0, Duration::ZERO));
                                
                                // Debug: Log progress for each recording
                                log::debug!("📊 Recording {} progress: {:.1}% ({:.1}s/{:.1}s) - playing: {}, paused: {}, seeking: {}", 
                                    recording_id, 
                                    progress * 100.0, 
                                    current_time.as_secs_f32(), 
                                    recording.duration.as_secs_f32(),
                                    is_playing,
                                    is_paused,
                                    self.voice_player.is_seeking(&recording_id)
                                );
                                
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
                                    .gap_2()
                                    .w_full() // Make each recording flexible
                                    .min_w_0() // Allow shrinking
                                    .p_3()
                                    .rounded_md()
                                    .bg(cx.theme().colors().element_background)
                                    .hover(|style| style.bg(cx.theme().colors().element_hover))
                                    .child(
                                        // Header with controls
                                        h_flex()
                                            .gap_2()
                                            .items_center()
                                            .justify_between()
                                            .w_full()
                                            .child(
                                                h_flex()
                                                    .gap_2()
                                                    .items_center()
                                                    .flex_1() // Take available space
                                                    .min_w_0() // Allow shrinking
                                                    .child(
                                                        // Play/pause button
                                                        IconButton::new(("play", index), icon_name)
                                                            .size(ButtonSize::Compact)
                                                            .style(ButtonStyle::Subtle)
                                                            .icon_color(icon_color)
                                                            .tooltip(move |window, cx| {
                                                                Tooltip::text(
                                                                    if is_playing { "Pause" } else { "Play" }
                                                                )(window, cx)
                                                            })
                                                            .on_click({
                                                                let recording_id = recording_id.clone();
                                                                cx.listener(move |this, _event, _window, cx| {
                                                                    this.toggle_voice_playback(recording_id.clone(), cx);
                                                                })
                                                            })
                                                    )
                                                    .child(
                                                        v_flex()
                                                            .gap_1()
                                                            .flex_1()
                                                            .min_w_0()
                                                            .child(
                                                                // Recording ID (truncated if needed)
                                                                Label::new(recording.id.clone())
                                                                    .size(LabelSize::XSmall)
                                                                    .color(Color::Default)
                                                                    .single_line()
                                                            )
                                                            .child(
                                                                // Time info
                                                                h_flex()
                                                                    .gap_1()
                                                                    .items_center()
                                                                    .child(
                                                                        Label::new(format!("{:.1}s", current_time.as_secs_f32()))
                                                                            .size(LabelSize::XSmall)
                                                                            .color(Color::Muted)
                                                                    )
                                                                    .child(
                                                                        Label::new("/")
                                                                            .size(LabelSize::XSmall)
                                                                            .color(Color::Disabled)
                                                                    )
                                                                    .child(
                                                                        Label::new(format!("{:.1}s", recording.duration.as_secs_f32()))
                                                                            .size(LabelSize::XSmall)
                                                                            .color(Color::Muted)
                                                                    )
                                                            )
                                                    )
                                            )
                                            .child(
                                                // Remove button
                                                IconButton::new(("remove", index), IconName::Trash)
                                                    .size(ButtonSize::Compact)
                                                    .style(ButtonStyle::Subtle)
                                                    .icon_color(Color::Error)
                                                    .tooltip(move |window, cx| {
                                                        Tooltip::text("Remove recording")(window, cx)
                                                    })
                                                    .on_click({
                                                        let recording_id = recording_id.clone();
                                                        cx.listener(move |this, _event, _window, cx| {
                                                            this.remove_voice_recording(recording_id.clone(), cx);
                                                        })
                                                    })
                                            )
                                    )
                                    .child(
                                        // Progress bar
                                        canvas(
                                            {
                                                let recording_id = recording_id.clone();
                                                move |bounds, _window, _cx| (bounds, recording_id.clone())
                                            },
                                            {
                                                let recording_id = recording_id.clone();
                                                let entity = cx.entity().clone();
                                                move |bounds, (_bounds_data, _recording_id), window, cx| {
                                                    // Draw simple progress bar background
                                                    window.paint_quad(gpui::fill(
                                                        bounds,
                                                        cx.theme().colors().element_background,
                                                    ));
                                                    
                                                    // Draw simple progress fill
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
                                                    
                                                    // Handle mouse events
                                                    window.on_mouse_event({
                                                        let recording_id = recording_id.clone();
                                                        let entity = entity.clone();
                                                        move |event: &gpui::MouseDownEvent, _phase, _window, cx| {
                                                            if bounds.contains(&event.position) {
                                                                let relative_x = event.position.x - bounds.origin.x;
                                                                let relative_position = (relative_x.0 / bounds.size.width.0).clamp(0.0, 1.0);
                                                                
                                                                entity.update(cx, |this, cx| {
                                                                    this.start_seek_voice_playback(recording_id.clone(), relative_position, cx);
                                                                });
                                                            }
                                                        }
                                                    });
                                                    
                                                    window.on_mouse_event({
                                                        let recording_id = recording_id.clone();
                                                        let entity = entity.clone();
                                                        move |event: &gpui::MouseMoveEvent, _phase, _window, cx| {
                                                            if event.pressed_button == Some(gpui::MouseButton::Left) {
                                                                let relative_x = event.position.x - bounds.origin.x;
                                                                let relative_position = (relative_x.0 / bounds.size.width.0).clamp(0.0, 1.0);
                                                                
                                                                entity.update(cx, |this, cx| {
                                                                    if this.voice_player.is_seeking(&recording_id) {
                                                                        this.update_seek_position(recording_id.clone(), relative_position, cx);
                                                                    }
                                                                });
                                                            }
                                                        }
                                                    });
                                                    
                                                    window.on_mouse_event({
                                                        let recording_id = recording_id.clone();
                                                        let entity = entity.clone();
                                                        move |_event: &gpui::MouseUpEvent, _phase, _window, cx| {
                                                            entity.update(cx, |this, cx| {
                                                                if this.voice_player.is_seeking(&recording_id) {
                                                                    this.end_seek_voice_playback(recording_id.clone(), cx);
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
                            })
                        )
                )
                .child(
                    PopoverMenu::new("playback-speed-menu")
                        .trigger(Button::new("playback_speed_trigger", ""))
                        .with_handle(self.playback_speed_menu_handle.clone())
                        .menu({
                            let entity = cx.entity().clone();
                            let current_speed = self.voice_player.get_playback_speed();
                            move |window, cx| {
                                let speeds = [0.5, 0.75, 1.0, 1.25, 1.5, 2.0];
                                
                                Some(ContextMenu::build(window, cx, |mut menu, _window, _cx| {
                                    for &speed in &speeds {
                                        let is_current = (speed - current_speed).abs() < 0.01;
                                        menu = menu.toggleable_entry(
                                            format!("{:.1}x", speed),
                                            is_current,
                                            IconPosition::Start,
                                            None,
                                            {
                                                let entity = entity.clone();
                                                move |_window, cx| {
                                                    entity.update(cx, |this, cx| {
                                                        this.set_playback_speed(speed, cx);
                                                    });
                                                }
                                            }
                                        );
                                    }
                                    menu
                                }))
                            }
                        })
                )
        )
    }

    fn toggle_voice_playback(&mut self, recording_id: String, cx: &mut Context<Self>) {
        log::info!("🎵 Toggling playback for recording: {}", recording_id);
        
        if let Err(e) = self.voice_player.toggle_playback(recording_id.clone()) {
            log::error!("Failed to toggle voice playback: {}", e);
            return;
        }
        
        let has_any_active_recording = self.voice_player.get_recordings()
            .iter()
            .any(|(_, recording)| {
                self.voice_player.is_playing(&recording.id) || 
                self.voice_player.is_paused(&recording.id) ||
                self.voice_player.is_seeking(&recording.id)
            });
        
        if has_any_active_recording {
            if self.playback_update_task.is_none() {
                log::info!("🔄 Starting playback update task for active recordings");
                self.start_playback_update_task(cx);
            }
        } else {
            log::info!("⏹️ No active recordings, stopping update task");
            self.playback_update_task.take();
        }
        
        // Immediately update UI for responsive playback control
        cx.notify();
    }

    fn end_seek_voice_playback(&mut self, recording_id: String, cx: &mut Context<Self>) {
        log::info!("🎯 Ending seek for recording: {}", recording_id);
        if let Err(e) = self.voice_player.end_seeking(recording_id.clone()) {
            log::error!("Failed to end seeking: {}", e);
        } else {
            log::info!("✅ Successfully ended seeking for recording: {}", recording_id);
            
            let has_any_active_recording = self.voice_player.get_recordings()
                .iter()
                .any(|(_, recording)| {
                    self.voice_player.is_playing(&recording.id) || 
                    self.voice_player.is_paused(&recording.id) ||
                    self.voice_player.is_seeking(&recording.id)
                });
            
            if has_any_active_recording && self.playback_update_task.is_none() {
                log::info!("🔄 Restarting update task after seeking ended");
                self.start_playback_update_task(cx);
            }
        }
        // Immediately update UI when seeking ends
        cx.notify();
    }

    fn update_seek_position(&mut self, recording_id: String, relative_position: f32, cx: &mut Context<Self>) {
        log::debug!("🎯 Updating seek position for recording: {} to {:.1}%", recording_id, relative_position * 100.0);
        if let Err(e) = self.voice_player.update_seek_position(recording_id.clone(), relative_position) {
            log::error!("Failed to update seek position: {}", e);
        }
        // Immediately update UI for smooth seeking
        cx.notify();
    }

    fn start_seek_voice_playback(&mut self, recording_id: String, relative_position: f32, cx: &mut Context<Self>) {
        log::info!("🎯 Starting seek for recording: {} at position {:.1}%", recording_id, relative_position * 100.0);
        if let Err(e) = self.voice_player.start_seeking(recording_id.clone(), relative_position) {
            log::error!("Failed to start seeking: {}", e);
        } else {
            log::info!("✅ Successfully started seeking for recording: {}", recording_id);
        }
        // Immediately update UI for responsive seeking
        cx.notify();
    }

    fn render_voice_button(&self, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        let is_recording = self.voice_recorder.is_recording();
        
        Some(
            IconButton::new("voice_input", IconName::Mic)
                .size(ButtonSize::Large)
                .style(if is_recording { 
                    ButtonStyle::Filled 
                } else { 
                    ButtonStyle::Subtle 
                })
                .icon_color(if is_recording { 
                    Color::Error 
                } else { 
                    Color::Muted 
                })
                .tooltip(move |window, cx| {
                    Tooltip::text(
                        if is_recording { "Stop Recording" } else { "Start Voice Recording" }
                    )(window, cx)
                })
                .on_click(cx.listener(|this, _event, _window, cx| {
                    this.toggle_voice_input(cx);
                }))
        )
    }

    fn toggle_voice_input(&mut self, cx: &mut Context<Self>) {
        let weak_entity = cx.entity().downgrade();
        self.voice_recorder.set_event_callback(move |event| {
            if let Some(_entity) = weak_entity.upgrade() {
                log::info!("Voice recorder event: {:?}", event);
            }
        });

        if let Ok(recording) = self.voice_recorder.toggle_recording() {
            if let Some(recording) = recording {
                self.voice_player.add_recording(recording.clone());
                self.insert_voice_recording(recording, cx);
            }
        }
        cx.notify();
    }

    fn insert_voice_recording(&mut self, recording: VoiceRecording, cx: &mut Context<Self>) {
        let text = format!("[Voice Recording: {} ({:.1}s)]", recording.id, recording.duration.as_secs_f32());
        
        self.editor.update(cx, |editor, cx| {
            let cursor_position = editor.selections.newest::<usize>(cx).head();
            
            editor.buffer().update(cx, |buffer, cx| {
                buffer.edit([(cursor_position..cursor_position, text.as_str())], None, cx);
            });
        });
        
        cx.notify();
    }

    fn start_playback_update_task(&mut self, cx: &mut Context<Self>) {
        self.playback_update_task.take();
        
        log::info!("🔄 Starting event-driven playback update task");
        
        self.playback_update_task = Some(cx.spawn(async move |this, cx| {
            // Use a shorter interval but focus on immediate updates when state changes
            let check_interval = Duration::from_millis(8); // Very fast check for state changes
            
            loop {
                smol::Timer::after(check_interval).await;
                
                let should_continue = this.update(cx, |this, cx| {
                    // Check for any voice player events first
                    if let Some(event) = this.voice_player.update_playback() {
                        match event {
                            VoicePlayerEvent::PlaybackCompleted { recording_id } => {
                                log::info!("✅ Playback completed for recording: {}", recording_id);
                                // Immediately update UI when playback completes
                                cx.notify();
                            }
                            _ => {
                                // Any other event should trigger immediate UI update
                                cx.notify();
                            }
                        }
                    }
                    
                    // Check if any recording is still active
                    let has_active_recordings = this.voice_player.get_recordings()
                        .iter()
                        .any(|(_, recording)| {
                            this.voice_player.is_playing(&recording.id) || 
                            this.voice_player.is_paused(&recording.id) ||
                            this.voice_player.is_seeking(&recording.id)
                        });
                    
                    if !has_active_recordings {
                        log::info!("⏹️ No more active recordings, stopping update task");
                        return false;
                    }
                    
                    // Always update UI for smooth progress during playback
                    cx.notify();
                    true
                }).unwrap_or(false);
                
                if !should_continue {
                    break;
                }
            }
            
            this.update(cx, |this, _cx| {
                this.playback_update_task.take();
                log::info!("🧹 Event-driven update task cleaned up");
            }).ok();
        }));
    }

    fn remove_voice_recording(&mut self, recording_id: String, cx: &mut Context<Self>) {
        log::info!("🗑️ Removing recording: {}", recording_id);
        self.voice_player.remove_recording(&recording_id);
        log::info!("✅ Successfully removed recording: {}", recording_id);
        cx.notify();
    }

    fn set_playback_speed(&mut self, speed: f32, cx: &mut Context<Self>) {
        log::info!("Setting playback speed to {:.2}x", speed);
        self.voice_player.set_playback_speed(speed);
        cx.notify();
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
