use crate::prelude::*;
use crate::v_flex;
use gpui::{
    AnyElement, App, Element, IntoElement, ParentElement, Pixels, RenderOnce, Styled, Window, div,
};
use smallvec::SmallVec;

/// Y height added beyond the size of the contents.
pub const POPOVER_Y_PADDING: Pixels = px(8.);

/// A popover is used to display a menu or show some options.
///
/// Clicking the element that launches the popover should not change the current view,
/// and the popover should be statically positioned relative to that element (not the
/// user's mouse.)
///
/// Example: A "new" menu with options like "new file", "new folder", etc,
/// Linear's "Display" menu, a profile menu that appears when you click your avatar.
///
/// Related elements:
///
/// [`ContextMenu`](crate::ContextMenu):
///
/// Used to display a popover menu that only contains a list of items. Context menus are always
/// launched by secondary clicking on an element. The menu is positioned relative to the user's cursor.
///
/// Example: Right clicking a file in the file tree to get a list of actions, right clicking
/// a tab to in the tab bar to get a list of actions.
///
/// `Dropdown`:
///
/// Used to display a list of options when the user clicks an element. The menu is
/// positioned relative the element that was clicked, and clicking an item in the
/// dropdown should change the value of the element that was clicked.
///
/// Example: A theme select control. Displays "One Dark", clicking it opens a list of themes.
/// When one is selected, the theme select control displays the selected theme.
#[derive(IntoElement)]
pub struct Popover {
    children: SmallVec<[AnyElement; 2]>,
    aside: Option<AnyElement>,
}

impl RenderOnce for Popover {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .gap_1()
            .child(
                v_flex()
                    .elevation_2(cx)
                    .py(POPOVER_Y_PADDING / 2.)
                    .children(self.children),
            )
            .when_some(self.aside, |this, aside| {
                this.child(
                    v_flex()
                        .elevation_2(cx)
                        .bg(cx.theme().colors().surface_background)
                        .px_1()
                        .child(aside),
                )
            })
    }
}

impl Default for Popover {
    fn default() -> Self {
        Self::new()
    }
}

impl Popover {
    pub fn new() -> Self {
        Self {
            children: SmallVec::new(),
            aside: None,
        }
    }

    pub fn aside(mut self, aside: impl IntoElement) -> Self
    where
        Self: Sized,
    {
        self.aside = Some(aside.into_element().into_any());
        self
    }
}

impl ParentElement for Popover {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
