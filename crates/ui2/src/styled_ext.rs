use gpui::{Div, ElementFocus, ElementInteractivity, Styled, UniformList, ViewContext};
use theme2::ActiveTheme;

use crate::{ElevationIndex, UITextSize};

fn elevated<E: Styled, V: 'static>(this: E, cx: &mut ViewContext<V>, index: ElevationIndex) -> E {
    this.bg(cx.theme().colors().elevated_surface_background)
        .rounded_lg()
        .border()
        .border_color(cx.theme().colors().border_variant)
        .shadow(index.shadow())
}

/// Extends [`Styled`](gpui::Styled) with Zed specific styling methods.
pub trait StyledExt: Styled + Sized {
    /// Horizontally stacks elements.
    ///
    /// Sets `flex()`, `flex_row()`, `items_center()`
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }

    /// Vertically stacks elements.
    ///
    /// Sets `flex()`, `flex_col()`
    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }

    fn text_ui_size(self, size: UITextSize) -> Self {
        let size = size.rems();

        self.text_size(size)
    }

    /// The default size for UI text.
    ///
    /// `0.825rem` or `14px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    ///
    /// Use [`text_ui_sm`] for regular-sized text.
    fn text_ui(self) -> Self {
        let size = UITextSize::default().rems();

        self.text_size(size)
    }

    /// The small size for UI text.
    ///
    /// `0.75rem` or `12px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    ///
    /// Use [`text_ui`] for regular-sized text.
    fn text_ui_sm(self) -> Self {
        let size = UITextSize::Small.rems();

        self.text_size(size)
    }

    /// The [`Surface`](ui2::ElevationIndex::Surface) elevation level, located above the app background, is the standard level for all elements
    ///
    /// Sets `bg()`, `rounded_lg()`, `border()`, `border_color()`, `shadow()`
    ///
    /// Example Elements: Title Bar, Panel, Tab Bar, Editor
    fn elevation_1<V: 'static>(self, cx: &mut ViewContext<V>) -> Self {
        elevated(self, cx, ElevationIndex::Surface)
    }

    /// Non-Modal Elevated Surfaces appear above the [`Surface`](ui2::ElevationIndex::Surface) layer and is used for things that should appear above most UI elements like an editor or panel, but not elements like popovers, context menus, modals, etc.
    ///
    /// Sets `bg()`, `rounded_lg()`, `border()`, `border_color()`, `shadow()`
    ///
    /// Examples: Notifications, Palettes, Detached/Floating Windows, Detached/Floating Panels
    fn elevation_2<V: 'static>(self, cx: &mut ViewContext<V>) -> Self {
        elevated(self, cx, ElevationIndex::ElevatedSurface)
    }

    // There is no elevation 3, as the third elevation level is reserved for wash layers. See [`Elevation`](ui2::Elevation).

    /// Modal Surfaces are used for elements that should appear above all other UI elements and are located above the wash layer. This is the maximum elevation at which UI elements can be rendered in their default state.
    ///
    /// Elements rendered at this layer should have an enforced behavior: Any interaction outside of the modal will either dismiss the modal or prompt an action (Save your progress, etc) then dismiss the modal.
    ///
    /// If the element does not have this behavior, it should be rendered at the [`Elevated Surface`](ui2::ElevationIndex::ElevatedSurface) layer.
    ///
    /// Sets `bg()`, `rounded_lg()`, `border()`, `border_color()`, `shadow()`
    ///
    /// Examples: Settings Modal, Channel Management, Wizards/Setup UI, Dialogs
    fn elevation_4<V: 'static>(self, cx: &mut ViewContext<V>) -> Self {
        elevated(self, cx, ElevationIndex::ModalSurface)
    }
}

impl<V, I, F> StyledExt for Div<V, I, F>
where
    I: ElementInteractivity<V>,
    F: ElementFocus<V>,
{
}

impl<V> StyledExt for UniformList<V> {}
