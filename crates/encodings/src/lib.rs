use editor::Editor;
use gpui::{ClickEvent, Entity, WeakEntity};
use ui::{Button, ButtonCommon, Context, LabelSize, Render, Tooltip, Window, div};
use ui::{Clickable, ParentElement};
use workspace::{ItemHandle, StatusItemView, Workspace};

use crate::selectors::save_or_reopen::{EncodingSaveOrReopenSelector, get_current_encoding};

pub enum Encoding {
    Utf8,
    Iso8859_1,
}

impl Encoding {
    pub fn as_str(&self) -> &str {
        match &self {
            Encoding::Utf8 => "UTF-8",
            Encoding::Iso8859_1 => "ISO 8859-1",
        }
    }
}

pub struct EncodingIndicator {
    pub encoding: Encoding,
    pub workspace: WeakEntity<Workspace>,
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
}

impl StatusItemView for EncodingIndicator {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}
