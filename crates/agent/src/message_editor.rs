use std::collections::BTreeMap;
use std::sync::Arc;

use crate::assistant_model_selector::ModelType;
use crate::context::{AssistantContext, format_context_as_string};
use crate::tool_compatibility::{IncompatibleToolsState, IncompatibleToolsTooltip};
use buffer_diff::BufferDiff;
use collections::HashSet;
use editor::actions::MoveUp;
use editor::{
    ContextMenuOptions, ContextMenuPlacement, Editor, EditorElement, EditorEvent, EditorMode,
    EditorStyle, MultiBuffer,
};
use file_icons::FileIcons;
use fs::Fs;
use gpui::{
    Animation, AnimationExt, App, Entity, EventEmitter, Focusable, Subscription, Task, TextStyle,
    WeakEntity, linear_color_stop, linear_gradient, point, pulsating_between,
};
use language::{Buffer, Language};
use language_model::{ConfiguredModel, LanguageModelRegistry, LanguageModelRequestMessage};
use language_model_selector::ToggleModelSelector;
use multi_buffer;
use project::Project;
use settings::Settings;
use std::time::Duration;
use theme::ThemeSettings;
use ui::{Disclosure, KeyBinding, PopoverMenuHandle, Tooltip, prelude::*};
use util::ResultExt as _;
use workspace::Workspace;

use crate::assistant_model_selector::AssistantModelSelector;
use crate::context_picker::{ContextPicker, ContextPickerCompletionProvider};
use crate::context_store::{ContextStore, refresh_context_store_text};
use crate::context_strip::{ContextStrip, ContextStripEvent, SuggestContextKind};
use crate::profile_selector::ProfileSelector;
use crate::thread::{Thread, TokenUsageRatio};
use crate::thread_store::ThreadStore;
use crate::{
    AgentDiff, Chat, ChatMode, ExpandMessageEditor, NewThread, OpenAgentDiff, RemoveAllContext,
    ToggleContextPicker, ToggleProfileSelector,
};

pub struct MessageEditor {
    thread: Entity<Thread>,
    incompatible_tools_state: Entity<IncompatibleToolsState>,
    editor: Entity<Editor>,
    #[allow(dead_code)]
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    context_store: Entity<ContextStore>,
    context_strip: Entity<ContextStrip>,
    context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    model_selector: Entity<AssistantModelSelector>,
    profile_selector: Entity<ProfileSelector>,
    edits_expanded: bool,
    editor_is_expanded: bool,
    waiting_for_summaries_to_send: bool,
    last_estimated_token_count: Option<usize>,
    update_token_count_task: Option<Task<anyhow::Result<()>>>,
    _subscriptions: Vec<Subscription>,
}

const MAX_EDITOR_LINES: usize = 8;

impl MessageEditor {
    pub fn new(
        fs: Arc<dyn Fs>,
        workspace: WeakEntity<Workspace>,
        context_store: Entity<ContextStore>,
        thread_store: WeakEntity<ThreadStore>,
        thread: Entity<Thread>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let context_picker_menu_handle = PopoverMenuHandle::default();
        let model_selector_menu_handle = PopoverMenuHandle::default();

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
            editor.set_placeholder_text("Ask anything, @ to mention, ↑ to select", cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_soft_wrap();
            editor.set_context_menu_options(ContextMenuOptions {
                min_entries_visible: 12,
                max_entries_visible: 12,
                placement: Some(ContextMenuPlacement::Above),
            });
            editor
        });

        let editor_entity = editor.downgrade();
        editor.update(cx, |editor, _| {
            editor.set_completion_provider(Some(Box::new(ContextPickerCompletionProvider::new(
                workspace.clone(),
                context_store.downgrade(),
                Some(thread_store.clone()),
                editor_entity,
            ))));
        });

        let context_strip = cx.new(|cx| {
            ContextStrip::new(
                context_store.clone(),
                workspace.clone(),
                Some(thread_store.clone()),
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
                EditorEvent::BufferEdited => {
                    this.message_or_context_changed(true, cx);
                }
                _ => {}
            }),
            cx.observe(&context_store, |this, _, cx| {
                this.message_or_context_changed(false, cx);
            }),
        ];

        Self {
            editor: editor.clone(),
            project: thread.read(cx).project().clone(),
            thread,
            incompatible_tools_state: incompatible_tools.clone(),
            workspace,
            context_store,
            context_strip,
            context_picker_menu_handle,
            model_selector: cx.new(|cx| {
                AssistantModelSelector::new(
                    fs.clone(),
                    model_selector_menu_handle,
                    editor.focus_handle(cx),
                    ModelType::Default,
                    window,
                    cx,
                )
            }),
            edits_expanded: false,
            editor_is_expanded: false,
            waiting_for_summaries_to_send: false,
            profile_selector: cx
                .new(|cx| ProfileSelector::new(fs, thread_store, editor.focus_handle(cx), cx)),
            last_estimated_token_count: None,
            update_token_count_task: None,
            _subscriptions: subscriptions,
        }
    }

    fn toggle_chat_mode(&mut self, _: &ChatMode, _window: &mut Window, cx: &mut Context<Self>) {
        cx.notify();
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

    fn is_model_selected(&self, cx: &App) -> bool {
        LanguageModelRegistry::read_global(cx)
            .default_model()
            .is_some()
    }

    fn send_to_model(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let model_registry = LanguageModelRegistry::read_global(cx);
        let Some(ConfiguredModel { model, provider }) = model_registry.default_model() else {
            return;
        };

        if provider.must_accept_terms(cx) {
            cx.notify();
            return;
        }

        let user_message = self.editor.update(cx, |editor, cx| {
            let text = editor.text(cx);
            editor.clear(window, cx);
            text
        });

        self.last_estimated_token_count.take();
        cx.emit(MessageEditorEvent::EstimatedTokenCount);

        let refresh_task =
            refresh_context_store_text(self.context_store.clone(), &HashSet::default(), cx);

        let thread = self.thread.clone();
        let context_store = self.context_store.clone();
        let git_store = self.project.read(cx).git_store().clone();
        let checkpoint = git_store.update(cx, |git_store, cx| git_store.checkpoint(cx));

        cx.spawn(async move |this, cx| {
            let checkpoint = checkpoint.await.ok();
            refresh_task.await;

            thread
                .update(cx, |thread, cx| {
                    let context = context_store.read(cx).context().clone();
                    thread.insert_user_message(user_message, context, checkpoint, cx);
                })
                .log_err();

            context_store
                .update(cx, |context_store, cx| {
                    let excerpt_ids = context_store
                        .context()
                        .iter()
                        .filter(|ctx| matches!(ctx, AssistantContext::Excerpt(_)))
                        .map(|ctx| ctx.id())
                        .collect::<Vec<_>>();

                    for id in excerpt_ids {
                        context_store.remove_context(id, cx);
                    }
                })
                .log_err();

            if let Some(wait_for_summaries) = context_store
                .update(cx, |context_store, cx| context_store.wait_for_summaries(cx))
                .log_err()
            {
                this.update(cx, |this, cx| {
                    this.waiting_for_summaries_to_send = true;
                    cx.notify();
                })
                .log_err();

                wait_for_summaries.await;

                this.update(cx, |this, cx| {
                    this.waiting_for_summaries_to_send = false;
                    cx.notify();
                })
                .log_err();
            }

            // Send to model after summaries are done
            thread
                .update(cx, |thread, cx| {
                    thread.advance_prompt_id();
                    thread.send_to_model(model, cx);
                })
                .log_err();
        })
        .detach();
    }

    fn stop_current_and_send_new_message(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let cancelled = self
            .thread
            .update(cx, |thread, cx| thread.cancel_last_completion(cx));

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

    fn handle_review_click(&self, window: &mut Window, cx: &mut Context<Self>) {
        AgentDiff::deploy(self.thread.clone(), self.workspace.clone(), window, cx).log_err();
    }

    fn handle_file_click(
        &self,
        buffer: Entity<Buffer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Ok(diff) = AgentDiff::deploy(self.thread.clone(), self.workspace.clone(), window, cx)
        {
            let path_key = multi_buffer::PathKey::for_buffer(&buffer, cx);
            diff.update(cx, |diff, cx| diff.move_to_path(path_key, window, cx));
        }
    }

    fn render_editor(
        &self,
        font_size: Rems,
        line_height: Pixels,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Div {
        let thread = self.thread.read(cx);

        let editor_bg_color = cx.theme().colors().editor_background;
        let is_generating = thread.is_generating();
        let focus_handle = self.editor.focus_handle(cx);

        let is_model_selected = self.is_model_selected(cx);
        let is_editor_empty = self.is_editor_empty(cx);

        let model = LanguageModelRegistry::read_global(cx)
            .default_model()
            .map(|default| default.model.clone());

        let incompatible_tools = model
            .as_ref()
            .map(|model| {
                self.incompatible_tools_state.update(cx, |state, cx| {
                    state
                        .incompatible_tools(model, cx)
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
            .on_action(cx.listener(Self::toggle_chat_mode))
            .on_action(cx.listener(Self::expand_message_editor))
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
                    .child(
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
                        div()
                            .min_h_16()
                            .when(is_editor_expanded, |this| this.h_full())
                            .child({
                                let settings = ThemeSettings::get_global(cx);

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
                            .child(h_flex().gap_2().child(self.profile_selector.clone()))
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
                                                    IconButton::new("send-message", IconName::Send)
                                                        .icon_color(Color::Accent)
                                                        .style(ButtonStyle::Filled)
                                                        .disabled(
                                                            !is_model_selected
                                                                || self
                                                                    .waiting_for_summaries_to_send,
                                                        )
                                                        .on_click({
                                                            let focus_handle = focus_handle.clone();
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
                                                            is_editor_empty
                                                                || !is_model_selected
                                                                || self
                                                                    .waiting_for_summaries_to_send,
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

        v_flex()
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
                            .child(
                                Label::new("Edits")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(Label::new("•").size(LabelSize::XSmall).color(Color::Muted))
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
                            ),
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
                                .when(index + 1 < changed_buffers.len(), |parent| {
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
                                        ) // TODO: show lines changed
                                        .child(Label::new("+").color(Color::Created))
                                        .child(Label::new("-").color(Color::Deleted)),
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

    fn render_token_limit_callout(
        &self,
        line_height: Pixels,
        token_usage_ratio: TokenUsageRatio,
        cx: &mut Context<Self>,
    ) -> Div {
        let heading = if token_usage_ratio == TokenUsageRatio::Exceeded {
            "Thread reached the token limit"
        } else {
            "Thread reaching the token limit soon"
        };

        h_flex()
            .p_2()
            .gap_2()
            .flex_wrap()
            .justify_between()
            .bg(
                if token_usage_ratio == TokenUsageRatio::Exceeded {
                    cx.theme().status().error_background.opacity(0.1)
                } else {
                    cx.theme().status().warning_background.opacity(0.1)
                })
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .gap_2()
                    .items_start()
                    .child(
                        h_flex()
                            .h(line_height)
                            .justify_center()
                            .child(
                                if token_usage_ratio == TokenUsageRatio::Exceeded {
                                    Icon::new(IconName::X)
                                        .color(Color::Error)
                                        .size(IconSize::XSmall)
                                } else {
                                    Icon::new(IconName::Warning)
                                        .color(Color::Warning)
                                        .size(IconSize::XSmall)
                                }
                            ),
                    )
                    .child(
                        v_flex()
                            .mr_auto()
                            .child(Label::new(heading).size(LabelSize::Small))
                            .child(
                                Label::new(
                                    "Start a new thread from a summary to continue the conversation.",
                                )
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                            ),
                    ),
            )
            .child(
                Button::new("new-thread", "Start New Thread")
                    .on_click(cx.listener(|this, _, window, cx| {
                        let from_thread_id = Some(this.thread.read(cx).id().clone());

                        window.dispatch_action(Box::new(NewThread {
                            from_thread_id
                        }), cx);
                    }))
                    .icon(IconName::Plus)
                    .icon_position(IconPosition::Start)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                    .label_size(LabelSize::Small),
            )
    }

    pub fn last_estimated_token_count(&self) -> Option<usize> {
        self.last_estimated_token_count
    }

    pub fn is_waiting_to_update_token_count(&self) -> bool {
        self.update_token_count_task.is_some()
    }

    fn message_or_context_changed(&mut self, debounce: bool, cx: &mut Context<Self>) {
        cx.emit(MessageEditorEvent::Changed);
        self.update_token_count_task.take();

        let Some(default_model) = LanguageModelRegistry::read_global(cx).default_model() else {
            self.last_estimated_token_count.take();
            return;
        };

        let context_store = self.context_store.clone();
        let editor = self.editor.clone();
        let thread = self.thread.clone();

        self.update_token_count_task = Some(cx.spawn(async move |this, cx| {
            if debounce {
                cx.background_executor()
                    .timer(Duration::from_millis(200))
                    .await;
            }

            let token_count = if let Some(task) = cx.update(|cx| {
                let context = context_store.read(cx).context().iter();
                let new_context = thread.read(cx).filter_new_context(context);
                let context_text =
                    format_context_as_string(new_context, cx).unwrap_or(String::new());
                let message_text = editor.read(cx).text(cx);

                let content = context_text + &message_text;

                if content.is_empty() {
                    return None;
                }

                let request = language_model::LanguageModelRequest {
                    thread_id: None,
                    prompt_id: None,
                    messages: vec![LanguageModelRequestMessage {
                        role: language_model::Role::User,
                        content: vec![content.into()],
                        cache: false,
                    }],
                    tools: vec![],
                    stop: vec![],
                    temperature: None,
                };

                Some(default_model.model.count_tokens(request, cx))
            })? {
                task.await?
            } else {
                0
            };

            this.update(cx, |this, cx| {
                this.last_estimated_token_count = Some(token_count);
                cx.emit(MessageEditorEvent::EstimatedTokenCount);
                this.update_token_count_task.take();
            })
        }));
    }
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
        let total_token_usage = thread.total_token_usage(cx);
        let token_usage_ratio = total_token_usage.ratio();

        let action_log = self.thread.read(cx).action_log();
        let changed_buffers = action_log.read(cx).changed_buffers(cx);

        let font_size = TextSize::Small.rems(cx);
        let line_height = font_size.to_pixels(window.rem_size()) * 1.5;

        v_flex()
            .size_full()
            .when(self.waiting_for_summaries_to_send, |parent| {
                parent.child(
                    h_flex().py_3().w_full().justify_center().child(
                        h_flex()
                            .flex_none()
                            .px_2()
                            .py_2()
                            .bg(cx.theme().colors().editor_background)
                            .border_1()
                            .border_color(cx.theme().colors().border_variant)
                            .rounded_lg()
                            .shadow_md()
                            .gap_1()
                            .child(
                                Icon::new(IconName::ArrowCircle)
                                    .size(IconSize::XSmall)
                                    .color(Color::Muted)
                                    .with_animation(
                                        "arrow-circle",
                                        Animation::new(Duration::from_secs(2)).repeat(),
                                        |icon, delta| {
                                            icon.transform(gpui::Transformation::rotate(
                                                gpui::percentage(delta),
                                            ))
                                        },
                                    ),
                            )
                            .child(
                                Label::new("Summarizing context…")
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            ),
                    ),
                )
            })
            .when(changed_buffers.len() > 0, |parent| {
                parent.child(self.render_changed_buffers(&changed_buffers, window, cx))
            })
            .child(self.render_editor(font_size, line_height, window, cx))
            .when(token_usage_ratio != TokenUsageRatio::Normal, |parent| {
                parent.child(self.render_token_limit_callout(line_height, token_usage_ratio, cx))
            })
    }
}
