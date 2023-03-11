use editor::Editor;
use gpui::{
    elements::*, CursorStyle, Entity, MouseButton, RenderContext, Subscription, View, ViewContext,
    ViewHandle,
};
use settings::Settings;
use std::sync::Arc;
use workspace::{item::ItemHandle, StatusItemView};

pub struct ActiveBufferLanguage {
    active_language: Option<Arc<str>>,
    _observe_active_editor: Option<Subscription>,
}

impl Default for ActiveBufferLanguage {
    fn default() -> Self {
        Self::new()
    }
}

impl ActiveBufferLanguage {
    pub fn new() -> Self {
        Self {
            active_language: None,
            _observe_active_editor: None,
        }
    }

    fn update_language(&mut self, editor: ViewHandle<Editor>, cx: &mut ViewContext<Self>) {
        self.active_language.take();

        let editor = editor.read(cx);
        if let Some((_, buffer, _)) = editor.active_excerpt(cx) {
            if let Some(language) = buffer.read(cx).language() {
                self.active_language = Some(language.name());
            }
        }

        cx.notify();
    }
}

impl Entity for ActiveBufferLanguage {
    type Event = ();
}

impl View for ActiveBufferLanguage {
    fn ui_name() -> &'static str {
        "ActiveBufferLanguage"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        if let Some(active_language) = self.active_language.as_ref() {
            MouseEventHandler::<Self>::new(0, cx, |state, cx| {
                let theme = &cx.global::<Settings>().theme.workspace.status_bar;
                let style = theme.active_language.style_for(state, false);
                Label::new(active_language.to_string(), style.text.clone())
                    .contained()
                    .with_style(style.container)
                    .boxed()
            })
            .with_cursor_style(CursorStyle::PointingHand)
            .on_click(MouseButton::Left, |_, cx| cx.dispatch_action(crate::Toggle))
            .boxed()
        } else {
            Empty::new().boxed()
        }
    }
}

impl StatusItemView for ActiveBufferLanguage {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.act_as::<Editor>(cx)) {
            self._observe_active_editor = Some(cx.observe(&editor, Self::update_language));
            self.update_language(editor, cx);
        } else {
            self.active_language = None;
            self._observe_active_editor = None;
        }

        cx.notify();
    }
}
