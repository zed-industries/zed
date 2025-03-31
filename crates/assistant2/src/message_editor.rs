use std::sync::Arc;

use collections::HashSet;
use editor::actions::MoveUp;
use editor::{ContextMenuOptions, ContextMenuPlacement, Editor, EditorElement, EditorStyle};
use file_icons::FileIcons;
use fs::Fs;
use gpui::{
    Animation, AnimationExt, App, DismissEvent, Entity, Focusable, Subscription, TextStyle,
    WeakEntity, linear_color_stop, linear_gradient, point,
};
use language_model::LanguageModelRegistry;
use language_model_selector::ToggleModelSelector;
use project::Project;
use settings::Settings;
use std::time::Duration;
use theme::ThemeSettings;
use ui::{
    ButtonLike, Disclosure, KeyBinding, PlatformStyle, PopoverMenu, PopoverMenuHandle, Tooltip,
    prelude::*,
};
use util::ResultExt;
use vim_mode_setting::VimModeSetting;
use workspace::Workspace;

use crate::assistant_model_selector::AssistantModelSelector;
use crate::context_picker::{ConfirmBehavior, ContextPicker, ContextPickerCompletionProvider};
use crate::context_store::{ContextStore, refresh_context_store_text};
use crate::context_strip::{ContextStrip, ContextStripEvent, SuggestContextKind};
use crate::profile_selector::ProfileSelector;
use crate::thread::{RequestKind, Thread};
use crate::thread_store::ThreadStore;
use crate::{
    AssistantDiff, Chat, ChatMode, OpenAssistantDiff, RemoveAllContext, ThreadEvent,
    ToggleContextPicker, ToggleProfileSelector,
};

pub struct MessageEditor {
    thread: Entity<Thread>,
    editor: Entity<Editor>,
    #[allow(dead_code)]
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    context_store: Entity<ContextStore>,
    context_strip: Entity<ContextStrip>,
    context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    inline_context_picker: Entity<ContextPicker>,
    inline_context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    model_selector: Entity<AssistantModelSelector>,
    profile_selector: Entity<ProfileSelector>,
    edits_expanded: bool,
    _subscriptions: Vec<Subscription>,
}

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
        let inline_context_picker_menu_handle = PopoverMenuHandle::default();
        let model_selector_menu_handle = PopoverMenuHandle::default();

        let editor = cx.new(|cx| {
            let mut editor = Editor::auto_height(10, window, cx);
            editor.set_placeholder_text("Ask anything, @ to mention, ↑ to select", cx);
            editor.set_show_indent_guides(false, cx);
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

        let inline_context_picker = cx.new(|cx| {
            ContextPicker::new(
                workspace.clone(),
                Some(thread_store.clone()),
                context_store.downgrade(),
                ConfirmBehavior::Close,
                window,
                cx,
            )
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

        let subscriptions = vec![
            cx.subscribe_in(
                &inline_context_picker,
                window,
                Self::handle_inline_context_picker_event,
            ),
            cx.subscribe_in(&context_strip, window, Self::handle_context_strip_event),
        ];

        Self {
            editor: editor.clone(),
            project: thread.read(cx).project().clone(),
            thread,
            workspace,
            context_store,
            context_strip,
            context_picker_menu_handle,
            inline_context_picker,
            inline_context_picker_menu_handle,
            model_selector: cx.new(|cx| {
                AssistantModelSelector::new(
                    fs.clone(),
                    model_selector_menu_handle,
                    editor.focus_handle(cx),
                    window,
                    cx,
                )
            }),
            edits_expanded: false,
            profile_selector: cx
                .new(|cx| ProfileSelector::new(fs, thread_store, editor.focus_handle(cx), cx)),
            _subscriptions: subscriptions,
        }
    }

    fn toggle_chat_mode(&mut self, _: &ChatMode, _window: &mut Window, cx: &mut Context<Self>) {
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
            return;
        }

        self.send_to_model(RequestKind::Chat, window, cx);
    }

    fn is_editor_empty(&self, cx: &App) -> bool {
        self.editor.read(cx).text(cx).is_empty()
    }

    fn is_model_selected(&self, cx: &App) -> bool {
        LanguageModelRegistry::read_global(cx)
            .active_model()
            .is_some()
    }

    fn send_to_model(
        &mut self,
        request_kind: RequestKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let provider = LanguageModelRegistry::read_global(cx).active_provider();
        if provider
            .as_ref()
            .map_or(false, |provider| provider.must_accept_terms(cx))
        {
            cx.notify();
            return;
        }

        let model_registry = LanguageModelRegistry::read_global(cx);
        let Some(model) = model_registry.active_model() else {
            return;
        };

        let user_message = self.editor.update(cx, |editor, cx| {
            let text = editor.text(cx);
            editor.clear(window, cx);
            text
        });

        let refresh_task =
            refresh_context_store_text(self.context_store.clone(), &HashSet::default(), cx);

        let system_prompt_context_task = self.thread.read(cx).load_system_prompt_context(cx);

        let thread = self.thread.clone();
        let context_store = self.context_store.clone();
        let checkpoint = self.project.read(cx).git_store().read(cx).checkpoint(cx);
        cx.spawn(async move |_, cx| {
            let checkpoint = checkpoint.await.ok();
            refresh_task.await;
            let (system_prompt_context, load_error) = system_prompt_context_task.await;
            thread
                .update(cx, |thread, cx| {
                    thread.set_system_prompt_context(system_prompt_context);
                    if let Some(load_error) = load_error {
                        cx.emit(ThreadEvent::ShowError(load_error));
                    }
                })
                .ok();
            thread
                .update(cx, |thread, cx| {
                    let context = context_store.read(cx).snapshot(cx).collect::<Vec<_>>();
                    thread.action_log().update(cx, |action_log, cx| {
                        action_log.clear_reviewed_changes(cx);
                    });
                    thread.insert_user_message(user_message, context, checkpoint, cx);
                    thread.send_to_model(model, request_kind, cx);
                })
                .ok();
        })
        .detach();
    }

    fn handle_inline_context_picker_event(
        &mut self,
        _inline_context_picker: &Entity<ContextPicker>,
        _event: &DismissEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let editor_focus_handle = self.editor.focus_handle(cx);
        window.focus(&editor_focus_handle);
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
        if self.context_picker_menu_handle.is_deployed()
            || self.inline_context_picker_menu_handle.is_deployed()
        {
            cx.propagate();
        } else {
            self.context_strip.focus_handle(cx).focus(window);
        }
    }

    fn handle_review_click(&self, window: &mut Window, cx: &mut Context<Self>) {
        AssistantDiff::deploy(self.thread.clone(), self.workspace.clone(), window, cx).log_err();
    }
}

impl Focusable for MessageEditor {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for MessageEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let font_size = TextSize::Default.rems(cx);
        let line_height = font_size.to_pixels(window.rem_size()) * 1.5;

        let focus_handle = self.editor.focus_handle(cx);
        let inline_context_picker = self.inline_context_picker.clone();

        let is_generating = self.thread.read(cx).is_generating();
        let is_model_selected = self.is_model_selected(cx);
        let is_editor_empty = self.is_editor_empty(cx);
        let submit_label_color = if is_editor_empty {
            Color::Muted
        } else {
            Color::Default
        };

        let vim_mode_enabled = VimModeSetting::get_global(cx).0;
        let platform = PlatformStyle::platform();
        let linux = platform == PlatformStyle::Linux;
        let windows = platform == PlatformStyle::Windows;
        let button_width = if linux || windows || vim_mode_enabled {
            px(82.)
        } else {
            px(64.)
        };

        let action_log = self.thread.read(cx).action_log();
        let changed_buffers = action_log.read(cx).changed_buffers(cx);
        let changed_buffers_count = changed_buffers.len();

        let editor_bg_color = cx.theme().colors().editor_background;
        let border_color = cx.theme().colors().border;
        let active_color = cx.theme().colors().element_selected;
        let bg_edit_files_disclosure = editor_bg_color.blend(active_color.opacity(0.3));

        v_flex()
            .size_full()
            .when(is_generating, |parent| {
                let focus_handle = self.editor.focus_handle(cx).clone();
                parent.child(
                    h_flex().py_3().w_full().justify_center().child(
                        h_flex()
                            .flex_none()
                            .pl_2()
                            .pr_1()
                            .py_1()
                            .bg(editor_bg_color)
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
                                Label::new("Generating…")
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            )
                            .child(ui::Divider::vertical())
                            .child(
                                Button::new("cancel-generation", "Cancel")
                                    .label_size(LabelSize::XSmall)
                                    .key_binding(
                                        KeyBinding::for_action_in(
                                            &editor::actions::Cancel,
                                            &focus_handle,
                                            window,
                                            cx,
                                        )
                                        .map(|kb| kb.size(rems_from_px(10.))),
                                    )
                                    .on_click(move |_event, window, cx| {
                                        focus_handle.dispatch_action(
                                            &editor::actions::Cancel,
                                            window,
                                            cx,
                                        );
                                    }),
                            ),
                    ),
                )
            })
            .when(changed_buffers_count > 0, |parent| {
                parent.child(
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
                                .p_1p5()
                                .justify_between()
                                .when(self.edits_expanded, |this| {
                                    this.border_b_1().border_color(border_color)
                                })
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .child(
                                            Disclosure::new(
                                                "edits-disclosure",
                                                self.edits_expanded,
                                            )
                                            .on_click(
                                                cx.listener(|this, _ev, _window, cx| {
                                                    this.edits_expanded = !this.edits_expanded;
                                                    cx.notify();
                                                }),
                                            ),
                                        )
                                        .child(
                                            Label::new("Edits")
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .child(
                                            Label::new("•")
                                                .size(LabelSize::XSmall)
                                                .color(Color::Muted),
                                        )
                                        .child(
                                            Label::new(format!(
                                                "{} {}",
                                                changed_buffers_count,
                                                if changed_buffers_count == 1 {
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
                                                &OpenAssistantDiff,
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
                        .when(self.edits_expanded, |parent| {
                            parent.child(
                                v_flex().bg(cx.theme().colors().editor_background).children(
                                    changed_buffers.into_iter().enumerate().flat_map(
                                        |(index, (buffer, changed))| {
                                            let file = buffer.read(cx).file()?;
                                            let path = file.path();

                                            let parent_label = path.parent().and_then(|parent| {
                                                let parent_str = parent.to_string_lossy();

                                                if parent_str.is_empty() {
                                                    None
                                                } else {
                                                    Some(
                                                        Label::new(format!(
                                                            "{}{}",
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
                                                .map(|icon| {
                                                    icon.color(Color::Muted).size(IconSize::Small)
                                                })
                                                .unwrap_or_else(|| {
                                                    Icon::new(IconName::File)
                                                        .color(Color::Muted)
                                                        .size(IconSize::Small)
                                                });

                                            let element = div()
                                                .relative()
                                                .py_1()
                                                .px_2()
                                                .when(index + 1 < changed_buffers_count, |parent| {
                                                    parent.border_color(border_color).border_b_1()
                                                })
                                                .child(
                                                    h_flex()
                                                        .gap_2()
                                                        .justify_between()
                                                        .child(
                                                            h_flex()
                                                                .id("file-container")
                                                                .pr_8()
                                                                .gap_1p5()
                                                                .max_w_full()
                                                                .overflow_x_scroll()
                                                                .child(file_icon)
                                                                .child(
                                                                    h_flex()
                                                                        .children(parent_label)
                                                                        .children(name_label),
                                                                ) // TODO: show lines changed
                                                                .child(
                                                                    Label::new("+")
                                                                        .color(Color::Created),
                                                                )
                                                                .child(
                                                                    Label::new("-")
                                                                        .color(Color::Deleted),
                                                                ),
                                                        )
                                                        .when(!changed.needs_review, |parent| {
                                                            parent.child(
                                                                Icon::new(IconName::Check)
                                                                    .color(Color::Success),
                                                            )
                                                        })
                                                        .child(
                                                            div()
                                                                .h_full()
                                                                .absolute()
                                                                .w_8()
                                                                .bottom_0()
                                                                .map(|this| {
                                                                    if !changed.needs_review {
                                                                        this.right_4()
                                                                    } else {
                                                                        this.right_0()
                                                                    }
                                                                })
                                                                .bg(linear_gradient(
                                                                    90.,
                                                                    linear_color_stop(
                                                                        editor_bg_color,
                                                                        1.,
                                                                    ),
                                                                    linear_color_stop(
                                                                        editor_bg_color
                                                                            .opacity(0.2),
                                                                        0.,
                                                                    ),
                                                                )),
                                                        ),
                                                );

                                            Some(element)
                                        },
                                    ),
                                ),
                            )
                        }),
                )
            })
            .child(
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
                    .gap_2()
                    .p_2()
                    .bg(editor_bg_color)
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .child(h_flex().justify_between().child(self.context_strip.clone()))
                    .child(
                        v_flex()
                            .gap_5()
                            .child({
                                let settings = ThemeSettings::get_global(cx);
                                let text_style = TextStyle {
                                    color: cx.theme().colors().text,
                                    font_family: settings.ui_font.family.clone(),
                                    font_fallbacks: settings.ui_font.fallbacks.clone(),
                                    font_features: settings.ui_font.features.clone(),
                                    font_size: font_size.into(),
                                    font_weight: settings.ui_font.weight,
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
                            })
                            .child(
                                PopoverMenu::new("inline-context-picker")
                                    .menu(move |window, cx| {
                                        inline_context_picker.update(cx, |this, cx| {
                                            this.init(window, cx);
                                        });

                                        Some(inline_context_picker.clone())
                                    })
                                    .attach(gpui::Corner::TopLeft)
                                    .anchor(gpui::Corner::BottomLeft)
                                    .offset(gpui::Point {
                                        x: px(0.0),
                                        y: (-ThemeSettings::get_global(cx).ui_font_size(cx) * 2)
                                            - px(4.0),
                                    })
                                    .with_handle(self.inline_context_picker_menu_handle.clone()),
                            )
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(h_flex().gap_2().child(self.profile_selector.clone()))
                                    .child(
                                        h_flex().gap_1().child(self.model_selector.clone()).child(
                                            ButtonLike::new("submit-message")
                                                .width(button_width.into())
                                                .style(ButtonStyle::Filled)
                                                .disabled(
                                                    is_editor_empty
                                                        || !is_model_selected
                                                        || is_generating,
                                                )
                                                .child(
                                                    h_flex()
                                                        .w_full()
                                                        .justify_between()
                                                        .child(
                                                            Label::new("Submit")
                                                                .size(LabelSize::Small)
                                                                .color(submit_label_color),
                                                        )
                                                        .children(
                                                            KeyBinding::for_action_in(
                                                                &Chat,
                                                                &focus_handle,
                                                                window,
                                                                cx,
                                                            )
                                                            .map(|binding| {
                                                                binding
                                                                    .when(vim_mode_enabled, |kb| {
                                                                        kb.size(rems_from_px(12.))
                                                                    })
                                                                    .into_any_element()
                                                            }),
                                                        ),
                                                )
                                                .on_click(move |_event, window, cx| {
                                                    focus_handle.dispatch_action(&Chat, window, cx);
                                                })
                                                .when(is_editor_empty, |button| {
                                                    button.tooltip(Tooltip::text(
                                                        "Type a message to submit",
                                                    ))
                                                })
                                                .when(is_generating, |button| {
                                                    button.tooltip(Tooltip::text(
                                                        "Cancel to submit a new message",
                                                    ))
                                                })
                                                .when(!is_model_selected, |button| {
                                                    button.tooltip(Tooltip::text(
                                                        "Select a model to continue",
                                                    ))
                                                }),
                                        ),
                                    ),
                            ),
                    ),
            )
    }
}
