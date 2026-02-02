use crate::{EncodingSelector, Toggle};

use editor::Editor;
use encoding_rs::{Encoding, UTF_8};
use gpui::{
    Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, WeakEntity, Window,
    div,
};
use project::Project;
use ui::{Button, ButtonCommon, Clickable, LabelSize, Tooltip};
use workspace::{
    StatusBarSettings, StatusItemView, Workspace,
    item::{ItemHandle, Settings},
};

pub struct ActiveBufferEncoding {
    active_encoding: Option<&'static Encoding>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    _observe_active_editor: Option<Subscription>,
    has_bom: bool,
    is_dirty: bool,
    is_shared: bool,
    is_via_remote_server: bool,
}

impl ActiveBufferEncoding {
    pub fn new(workspace: &Workspace) -> Self {
        Self {
            active_encoding: None,
            workspace: workspace.weak_handle(),
            project: workspace.project().clone(),
            _observe_active_editor: None,
            has_bom: false,
            is_dirty: false,
            is_shared: false,
            is_via_remote_server: false,
        }
    }

    fn update_encoding(&mut self, editor: Entity<Editor>, _: &mut Window, cx: &mut Context<Self>) {
        self.active_encoding = None;
        self.has_bom = false;
        self.is_dirty = false;

        let project = self.project.read(cx);
        self.is_shared = project.is_shared();
        self.is_via_remote_server = project.is_via_remote_server();

        if let Some((_, buffer, _)) = editor.read(cx).active_excerpt(cx) {
            let buffer = buffer.read(cx);
            self.active_encoding = Some(buffer.encoding());
            self.has_bom = buffer.has_bom();
            self.is_dirty = buffer.is_dirty();
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

        let (disabled, tooltip_text) = if self.is_dirty {
            (true, "Save file to change encoding")
        } else if self.is_shared {
            (true, "Cannot change encoding during collaboration")
        } else if self.is_via_remote_server {
            (true, "Cannot change encoding of remote server file")
        } else {
            (false, "Reopen with Encoding")
        };

        div().child(
            Button::new("change-encoding", text)
                .label_size(LabelSize::Small)
                .on_click(cx.listener(move |this, _, window, cx| {
                    if disabled {
                        return;
                    }
                    if let Some(workspace) = this.workspace.upgrade() {
                        workspace.update(cx, |workspace, cx| {
                            EncodingSelector::toggle(workspace, window, cx)
                        });
                    }
                }))
                .tooltip(move |_window, cx| {
                    if disabled {
                        Tooltip::text(tooltip_text)(_window, cx)
                    } else {
                        Tooltip::for_action(tooltip_text, &Toggle, cx)
                    }
                }),
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
            self.is_dirty = false;
            self.is_shared = false;
            self.is_via_remote_server = false;
            self._observe_active_editor = None;
        }

        cx.notify();
    }
}
