use gpui::{App, Context, Entity, Window};

use crate::item::{Item, ItemHandle};

pub trait CollapsibleItem: Item {
    /// Whether the collapsible item has any collapsed content.
    fn has_any_collapsed(&self, _cx: &App) -> bool;

    /// Collapse all content.
    fn collapse_all(&mut self, window: &mut Window, cx: &mut Context<Self>);

    /// Expand all content.
    fn expand_all(&mut self, window: &mut Window, cx: &mut Context<Self>);
}

pub trait CollapsibleItemHandle: ItemHandle {
    /// Whether the collapsible item has any collapsed content.
    fn has_any_collapsed(&self, cx: &App) -> bool;

    /// Collapse all content.
    fn collapse_all(&mut self, window: &mut Window, cx: &mut App);

    /// Expand all content.
    fn expand_all(&mut self, window: &mut Window, cx: &mut App);
}

impl<T: CollapsibleItem> CollapsibleItemHandle for Entity<T> {
    fn has_any_collapsed(&self, cx: &App) -> bool {
        self.read(cx).has_any_collapsed(cx)
    }

    fn collapse_all(&mut self, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| this.collapse_all(window, cx));
    }

    fn expand_all(&mut self, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| this.expand_all(window, cx));
    }
}
