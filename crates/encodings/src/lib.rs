use editor::Editor;
use encoding::Encoding;
use gpui::{ClickEvent, Entity, Subscription, WeakEntity};
use ui::{Button, ButtonCommon, Context, LabelSize, Render, Tooltip, Window, div};
use ui::{Clickable, ParentElement};
use workspace::{ItemHandle, StatusItemView, Workspace};

use crate::selectors::save_or_reopen::{EncodingSaveOrReopenSelector, get_current_encoding};

pub struct EncodingIndicator {
    pub encoding: Option<&'static dyn Encoding>,
    pub workspace: WeakEntity<Workspace>,
    observe: Option<Subscription>,
}

pub mod selectors;

impl Render for EncodingIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let status_element = div();

        status_element.child(
            Button::new("encoding", get_current_encoding())
                .label_size(LabelSize::Small)
                .tooltip(Tooltip::text("Select Encoding"))
                .on_click(cx.listener(|indicator, _: &ClickEvent, window, cx| {
                    if let Some(workspace) = indicator.workspace.upgrade() {
                        workspace.update(cx, |workspace, cx| {
                            EncodingSaveOrReopenSelector::toggle(workspace, window, cx)
                        })
                    } else {
                    }
                })),
        )
    }
}

impl EncodingIndicator {
    pub fn get_current_encoding(&self, cx: &mut Context<Self>, editor: WeakEntity<Editor>) {}

    pub fn new(
        encoding: Option<&'static dyn encoding::Encoding>,
        workspace: WeakEntity<Workspace>,
        observe: Option<Subscription>,
    ) -> EncodingIndicator {
        EncodingIndicator {
            encoding,
            workspace,
            observe,
        }
    }

    pub fn update(
        &mut self,
        editor: Entity<Editor>,
        _: &mut Window,
        cx: &mut Context<EncodingIndicator>,
    ) {
        let editor = editor.read(cx);
        if let Some((_, buffer, _)) = editor.active_excerpt(cx) {
            let encoding = buffer.read(cx).encoding;
            self.encoding = Some(encoding);
        }

        cx.notify();
    }
}

impl StatusItemView for EncodingIndicator {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            Some(editor) => {
                self.observe = Some(cx.observe_in(&editor, window, Self::update));
                self.update(editor, window, cx);
            }
            None => {
                self.encoding = None;
                self.observe = None;
            }
        }
    }
}
