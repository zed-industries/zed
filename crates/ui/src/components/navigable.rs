use crate::prelude::*;
use gpui::{AnyElement, FocusHandle, Role, ScrollAnchor, ScrollHandle};

/// An element that can be navigated through via keyboard. Intended for use with scrollable views that want to use
#[derive(IntoElement)]
pub struct Navigable {
    id: Option<ElementId>,
    a11y_label: Option<SharedString>,
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
            id: None,
            a11y_label: None,
            child,
            selectable_children: vec![],
        }
    }

    /// Set the element ID for this navigable, enabling accessibility role.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the accessibility label for this navigable.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.a11y_label = Some(label.into());
        self
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
        let select_next_children = self.selectable_children.clone();
        let select_prev_children = self.selectable_children;

        let base = div()
            .on_action(move |_: &menu::SelectNext, window, cx| {
                let target = Self::find_focused(&select_next_children, window, cx)
                    .and_then(|index| {
                        index
                            .checked_add(1)
                            .filter(|index| *index < select_next_children.len())
                    })
                    .unwrap_or(0);
                if let Some(entry) = select_next_children.get(target) {
                    entry.focus_handle.focus(window, cx);
                    if let Some(anchor) = &entry.scroll_anchor {
                        anchor.scroll_to(window, cx);
                    }
                }
            })
            .on_action(move |_: &menu::SelectPrevious, window, cx| {
                let target = Self::find_focused(&select_prev_children, window, cx)
                    .and_then(|index| index.checked_sub(1))
                    .or(select_prev_children.len().checked_sub(1));
                if let Some(entry) = target.and_then(|target| select_prev_children.get(target)) {
                    entry.focus_handle.focus(window, cx);
                    if let Some(anchor) = &entry.scroll_anchor {
                        anchor.scroll_to(window, cx);
                    }
                }
            })
            .size_full()
            .child(self.child);

        if let Some(id) = self.id {
            let mut element = base.id(id).role(Role::Navigation);
            if let Some(label) = self.a11y_label {
                element = element.aria_label(label);
            }
            element.into_any_element()
        } else {
            base.into_any_element()
        }
    }
}
