use gpui::{App, Entity, Window};
use search::{BufferSearchBar, buffer_search};
use workspace::item::{ItemBufferKind, ItemHandle};

use super::{
    QuickActionBarItem, QuickActionButton, QuickActionElement, QuickActionTarget, VisibilityTrigger,
};

pub(super) struct BufferSearchButton {
    buffer_search_bar: Entity<BufferSearchBar>,
}

impl BufferSearchButton {
    pub(super) fn new(buffer_search_bar: Entity<BufferSearchBar>) -> Self {
        Self { buffer_search_bar }
    }
}

impl QuickActionBarItem for BufferSearchButton {
    type Context = ();
    const ID: &'static str = "toggle-buffer-search";
    const TRIGGERS: &'static [VisibilityTrigger] = &[];

    fn context(&self, target: &QuickActionTarget, cx: &mut App) -> Option<()> {
        let editor = target.editor()?;
        (editor.buffer_kind(cx) == ItemBufferKind::Singleton).then_some(())
    }

    fn render(&self, _: &(), _window: &mut Window, cx: &mut App) -> QuickActionElement {
        let buffer_search_bar = self.buffer_search_bar.clone();
        QuickActionElement::Button(
            QuickActionButton::new(search::SEARCH_ICON, "Buffer Search", move |window, cx| {
                buffer_search_bar.update(cx, |search_bar, cx| {
                    search_bar.toggle(&buffer_search::Deploy::find(), window, cx)
                });
            })
            .action(Box::new(buffer_search::Deploy::find()))
            .toggled(!self.buffer_search_bar.read(cx).is_dismissed()),
        )
    }
}
