use editor::Editor;
use gpui::{
    elements::*,
    platform::{CursorStyle, MouseButton},
    Entity, Subscription, View, ViewContext, ViewHandle, WeakViewHandle,
};
use std::sync::Arc;
use workspace::{item::ItemHandle, StatusItemView, Workspace};

pub struct ActiveBufferLanguage {
    active_language: Option<Option<Arc<str>>>,
    workspace: WeakViewHandle<Workspace>,
    _observe_active_editor: Option<Subscription>,
}

impl ActiveBufferLanguage {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            active_language: None,
            workspace: workspace.weak_handle(),
            _observe_active_editor: None,
        }
    }

    fn update_language(&mut self, editor: ViewHandle<Editor>, cx: &mut ViewContext<Self>) {
        self.active_language = Some(None);

        let editor = editor.read(cx);
        if let Some((_, buffer, _)) = editor.active_excerpt(cx) {
            if let Some(language) = buffer.read(cx).language() {
                self.active_language = Some(Some(language.name()));
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

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        if let Some(active_language) = self.active_language.as_ref() {
            let active_language_text = if let Some(active_language_text) = active_language {
                active_language_text.to_string()
            } else {
                "Unknown".to_string()
            };

            MouseEventHandler::new::<Self, _>(0, cx, |state, cx| {
                let theme = &theme::current(cx).workspace.status_bar;
                let style = theme.active_language.style_for(state);
                Label::new(active_language_text, style.text.clone())
                    .contained()
                    .with_style(style.container)
            })
            .with_cursor_style(CursorStyle::PointingHand)
            .on_click(MouseButton::Left, |_, this, cx| {
                if let Some(workspace) = this.workspace.upgrade(cx) {
                    workspace.update(cx, |workspace, cx| {
                        crate::toggle(workspace, &Default::default(), cx)
                    });
                }
            })
            .into_any()
        } else {
            Empty::new().into_any()
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
