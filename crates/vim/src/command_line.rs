use editor::{Editor, EditorElement, EditorEvent, EditorStyle, actions::Cancel};
use gpui::{
    App, Context, Entity, FocusHandle, Focusable, IntoElement, KeyContext, ParentElement, Render,
    Styled, Subscription, TextStyle, WeakEntity, Window, actions, div, px, relative, rems,
    transparent_black,
};
use settings::Settings;
use theme::ThemeSettings;
use ui::{h_flex, prelude::*};
use workspace::{StatusItemView, Workspace, item::ItemHandle};

use crate::{Vim, VimEvent, command::command_interceptor};

actions!(vim, [ConfirmCommand]);

pub struct VimCommandLine {
    editor: Entity<Editor>,
    pub(crate) active: bool,
    prefix: String,
    workspace: WeakEntity<Workspace>,
    vim_subscription: Option<Subscription>,
    _editor_subscription: Subscription,
}

impl VimCommandLine {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("", window, cx);
            editor.set_use_autoclose(false);
            editor.set_use_modal_editing(false);
            editor.set_input_enabled(true);
            editor.set_show_gutter(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor
        });

        let window_handle = window.window_handle();
        let self_weak = cx.entity().downgrade();

        let mut this = Self {
            editor: editor.clone(),
            active: false,
            prefix: String::new(),
            workspace,
            vim_subscription: None,
            _editor_subscription: cx.subscribe(&editor, {
                let self_weak = self_weak.clone();
                move |this, editor, _event: &EditorEvent, cx| {
                    // Check if text is empty or prefix was removed after the event is processed
                    if this.active {
                        let prefix = this.prefix.clone();
                        let self_weak = self_weak.clone();
                        cx.defer(move |cx| {
                            cx.update_window(window_handle, |_, window, cx| {
                                let text = editor.read(cx).text(cx);
                                if text.is_empty() || !text.starts_with(&prefix) {
                                    if let Some(command_line) = self_weak.upgrade() {
                                        command_line.update(cx, |this, cx| {
                                            this.dismiss(window, cx);
                                        });
                                    }
                                }
                            })
                            .ok();
                        });
                    }

                    cx.notify();
                }
            }),
        };

        this.subscribe_to_vim(window, cx);

        Vim::update_globals(cx, |globals, _| {
            globals.command_line = Some(self_weak);
        });

        this
    }

    fn subscribe_to_vim(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let handle = cx.entity();
        let window_handle = window.window_handle();

        self.vim_subscription = Some(cx.observe_new::<Vim>(move |_, window, cx| {
            let Some(window) = window else {
                return;
            };
            if window.window_handle() != window_handle {
                return;
            }
            let vim = cx.entity();
            handle.update(cx, |_, cx| {
                cx.subscribe(&vim, |_this, _vim, event, cx| match event {
                    VimEvent::Focused => {
                        cx.notify();
                    }
                })
                .detach()
            })
        }));
    }

    pub fn activate(&mut self, prefix: String, window: &mut Window, cx: &mut Context<Self>) {
        self.active = true;
        self.prefix = prefix.clone();
        self.editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
            editor.set_text(prefix, window, cx);
        });

        window.focus(&self.editor.focus_handle(cx));
        cx.notify();
    }

    pub fn dismiss(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.active = false;
        self.prefix.clear();
        self.editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
        });

        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                if let Some(active_item) = workspace.active_item(cx) {
                    window.focus(&active_item.item_focus_handle(cx));
                }
            });
        }
        cx.notify();
    }

    fn execute_command(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let full_command = self.editor.read(cx).text(cx);

        self.dismiss(window, cx);

        let results = command_interceptor(&full_command, cx);
        if let Some(first_result) = results.first() {
            window.dispatch_action(first_result.action.boxed_clone(), cx);
        }
    }

    fn confirm_command(&mut self, _: &ConfirmCommand, window: &mut Window, cx: &mut Context<Self>) {
        self.execute_command(window, cx);
    }

    fn cancel(&mut self, _: &Cancel, window: &mut Window, cx: &mut Context<Self>) {
        self.dismiss(window, cx);
    }
}

impl Render for VimCommandLine {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.active {
            return div().into_any();
        }

        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("VimCommandLine");
        key_context.add("Editor");

        let theme_colors = cx.theme().colors();
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: theme_colors.text,
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_fallbacks: settings.buffer_font.fallbacks.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.buffer_font.weight,
            line_height: relative(1.3),
            ..TextStyle::default()
        };

        let editor_style = EditorStyle {
            background: transparent_black(),
            local_player: cx.theme().players().local(),
            text: text_style,
            syntax: cx.theme().syntax().clone(),
            ..EditorStyle::default()
        };

        h_flex()
            .items_center()
            .key_context(key_context)
            .child(
                div().flex_1().min_w(px(200.0)).flex().items_center().child(
                    div()
                        .flex_1()
                        .child(EditorElement::new(&self.editor, editor_style)),
                ),
            )
            .on_action(cx.listener(VimCommandLine::confirm_command))
            .on_action(cx.listener(VimCommandLine::cancel))
            .into_any()
    }
}

impl Focusable for VimCommandLine {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl StatusItemView for VimCommandLine {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}
