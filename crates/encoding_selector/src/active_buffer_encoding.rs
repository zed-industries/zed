use editor::Editor;
use encoding_rs::{Encoding, UTF_8};
use gpui::{
    Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window, div,
};
use ui::{Button, ButtonCommon, Clickable, LabelSize, Tooltip};
use workspace::{
    StatusBarSettings, StatusItemView, Workspace,
    item::{ItemHandle, Settings},
};

pub struct ActiveBufferEncoding {
    active_encoding: Option<&'static Encoding>,
    //workspace: WeakEntity<Workspace>,
    _observe_active_editor: Option<Subscription>,
    has_bom: bool,
}

impl ActiveBufferEncoding {
    pub fn new(_workspace: &Workspace) -> Self {
        Self {
            active_encoding: None,
            //workspace: workspace.weak_handle(),
            _observe_active_editor: None,
            has_bom: false,
        }
    }

    fn update_encoding(&mut self, editor: Entity<Editor>, _: &mut Window, cx: &mut Context<Self>) {
        self.active_encoding = None;

        let editor = editor.read(cx);
        if let Some((_, buffer, _)) = editor.active_excerpt(cx) {
            let buffer = buffer.read(cx);

            self.active_encoding = Some(buffer.encoding());
            self.has_bom = buffer.has_bom();
        }

        cx.notify();
    }
}

impl Render for ActiveBufferEncoding {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(active_encoding) = self.active_encoding else {
            return div().hidden();
        };

        let display_option = StatusBarSettings::get_global(cx).active_encoding_button;
        let is_utf8 = active_encoding == UTF_8;
        if !display_option.should_show(is_utf8, self.has_bom) {
            return div().hidden();
        }

        let mut text = active_encoding.name().to_string();
        if self.has_bom {
            text.push_str(" (BOM)");
        }

        div().child(
            Button::new("change-encoding", text)
                .label_size(LabelSize::Small)
                .on_click(|_, _, _cx| {
                    // No-op
                })
                .tooltip(Tooltip::text("Current Encoding")),
        )
    }
}

impl StatusItemView for ActiveBufferEncoding {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            self._observe_active_editor =
                Some(cx.observe_in(&editor, window, Self::update_encoding));
            self.update_encoding(editor, window, cx);
        } else {
            self.active_encoding = None;
            self.has_bom = false;
            self._observe_active_editor = None;
        }

        cx.notify();
    }
}
