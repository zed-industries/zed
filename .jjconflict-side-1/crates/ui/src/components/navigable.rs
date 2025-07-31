use crate::prelude::*;
use gpui::{AnyElement, FocusHandle, ScrollAnchor, ScrollHandle};

/// An element that can be navigated through via keyboard. Intended for use with scrollable views that want to use
#[derive(IntoElement)]
pub struct Navigable {
    child: AnyElement,
    selectable_children: Vec<NavigableEntry>,
}

/// An entry of [Navigable] that can be navigated to.
#[derive(Clone)]
pub struct NavigableEntry {
    #[allow(missing_docs)]
    pub focus_handle: FocusHandle,
    #[allow(missing_docs)]
    pub scroll_anchor: Option<ScrollAnchor>,
}

impl NavigableEntry {
    /// Creates a new [NavigableEntry] for a given scroll handle.
    pub fn new(scroll_handle: &ScrollHandle, cx: &App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            scroll_anchor: Some(ScrollAnchor::for_handle(scroll_handle.clone())),
        }
    }
    /// Create a new [NavigableEntry] that cannot be scrolled to.
    pub fn focusable(cx: &App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            scroll_anchor: None,
        }
    }
}
impl Navigable {
    /// Creates new empty [Navigable] wrapper.
    pub fn new(child: AnyElement) -> Self {
        Self {
            child,
            selectable_children: vec![],
        }
    }

    /// Add a new entry that can be navigated to via keyboard.
    ///
    /// The order of calls to [Navigable::entry] determines the order of traversal of
    /// elements via successive uses of `menu:::SelectNext/SelectPrevious`
    pub fn entry(mut self, child: NavigableEntry) -> Self {
        self.selectable_children.push(child);
        self
    }

    fn find_focused(
        selectable_children: &[NavigableEntry],
        window: &mut Window,
        cx: &mut App,
    ) -> Option<usize> {
        selectable_children
            .iter()
            .position(|entry| entry.focus_handle.contains_focused(window, cx))
    }
}

impl RenderOnce for Navigable {
    fn render(self, _window: &mut Window, _: &mut App) -> impl crate::IntoElement {
        div()
            .on_action({
                let children = self.selectable_children.clone();

                move |_: &menu::SelectNext, window, cx| {
                    let target = Self::find_focused(&children, window, cx)
                        .and_then(|index| {
                            index.checked_add(1).filter(|index| *index < children.len())
                        })
                        .unwrap_or(0);
                    if let Some(entry) = children.get(target) {
                        entry.focus_handle.focus(window);
                        if let Some(anchor) = &entry.scroll_anchor {
                            anchor.scroll_to(window, cx);
                        }
                    }
                }
            })
            .on_action({
                let children = self.selectable_children;
                move |_: &menu::SelectPrevious, window, cx| {
                    let target = Self::find_focused(&children, window, cx)
                        .and_then(|index| index.checked_sub(1))
                        .or(children.len().checked_sub(1));
                    if let Some(entry) = target.and_then(|target| children.get(target)) {
                        entry.focus_handle.focus(window);
                        if let Some(anchor) = &entry.scroll_anchor {
                            anchor.scroll_to(window, cx);
                        }
                    }
                }
            })
            .size_full()
            .child(self.child)
    }
}
