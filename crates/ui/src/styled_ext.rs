use gpui::{hsla, px, Styled, WindowContext};
use settings::Settings;
use theme::ThemeSettings;

use crate::prelude::*;
use crate::{ElevationIndex, UiTextSize};

fn elevated<E: Styled>(this: E, cx: &mut WindowContext, index: ElevationIndex) -> E {
    this.bg(cx.theme().colors().elevated_surface_background)
        .z_index(index.z_index())
        .rounded(px(8.))
        .border()
        .border_color(cx.theme().colors().border_variant)
        .shadow(index.shadow())
}

/// Extends [`gpui::Styled`] with Zed-specific styling methods.
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

    /// Sets the text size using a [`UiTextSize`].
    fn text_ui_size(self, size: UiTextSize) -> Self {
        self.text_size(size.rems())
    }

    /// The large size for UI text.
    ///
    /// `1rem` or `16px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    ///
    /// Use `text_ui` for regular-sized text.
    fn text_ui_lg(self) -> Self {
        self.text_size(UiTextSize::Large.rems())
    }

    /// The default size for UI text.
    ///
    /// `0.825rem` or `14px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    ///
    /// Use `text_ui_sm` for smaller text.
    fn text_ui(self) -> Self {
        self.text_size(UiTextSize::default().rems())
    }

    /// The small size for UI text.
    ///
    /// `0.75rem` or `12px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    ///
    /// Use `text_ui` for regular-sized text.
    fn text_ui_sm(self) -> Self {
        self.text_size(UiTextSize::Small.rems())
    }

    /// The extra small size for UI text.
    ///
    /// `0.625rem` or `10px` at the default scale of `1rem` = `16px`.
    ///
    /// Note: The absolute size of this text will change based on a user's `ui_scale` setting.
    ///
    /// Use `text_ui` for regular-sized text.
    fn text_ui_xs(self) -> Self {
        self.text_size(UiTextSize::XSmall.rems())
    }

    /// The font size for buffer text.
    ///
    /// Retrieves the default font size, or the user's custom font size if set.
    ///
    /// This should only be used for text that is displayed in a buffer,
    /// or other places that text needs to match the user's buffer font size.
    fn text_buffer(self, cx: &mut WindowContext) -> Self {
        let settings = ThemeSettings::get_global(cx);
        self.text_size(settings.buffer_font_size(cx))
    }

    /// The [`Surface`](ElevationIndex::Surface) elevation level, located above the app background, is the standard level for all elements
    ///
    /// Sets `bg()`, `rounded_lg()`, `border()`, `border_color()`, `shadow()`
    ///
    /// Example Elements: Title Bar, Panel, Tab Bar, Editor
    fn elevation_1(self, cx: &mut WindowContext) -> Self {
        elevated(self, cx, ElevationIndex::Surface)
    }

    /// Non-Modal Elevated Surfaces appear above the [`Surface`](ElevationIndex::Surface) layer and is used for things that should appear above most UI elements like an editor or panel, but not elements like popovers, context menus, modals, etc.
    ///
    /// Sets `bg()`, `rounded_lg()`, `border()`, `border_color()`, `shadow()`
    ///
    /// Examples: Notifications, Palettes, Detached/Floating Windows, Detached/Floating Panels
    fn elevation_2(self, cx: &mut WindowContext) -> Self {
        elevated(self, cx, ElevationIndex::ElevatedSurface)
    }

    /// Modal Surfaces are used for elements that should appear above all other UI elements and are located above the wash layer. This is the maximum elevation at which UI elements can be rendered in their default state.
    ///
    /// Elements rendered at this layer should have an enforced behavior: Any interaction outside of the modal will either dismiss the modal or prompt an action (Save your progress, etc) then dismiss the modal.
    ///
    /// If the element does not have this behavior, it should be rendered at the [`Elevated Surface`](ElevationIndex::ElevatedSurface) layer.
    ///
    /// Sets `bg()`, `rounded_lg()`, `border()`, `border_color()`, `shadow()`
    ///
    /// Examples: Settings Modal, Channel Management, Wizards/Setup UI, Dialogs
    fn elevation_3(self, cx: &mut WindowContext) -> Self {
        elevated(self, cx, ElevationIndex::ModalSurface)
    }

    /// The theme's primary border color.
    fn border_primary(self, cx: &mut WindowContext) -> Self {
        self.border_color(cx.theme().colors().border)
    }

    /// The theme's secondary or muted border color.
    fn border_muted(self, cx: &mut WindowContext) -> Self {
        self.border_color(cx.theme().colors().border_variant)
    }

    /// Sets the background color to red for debugging when building UI.
    fn debug_bg_red(self) -> Self {
        self.bg(hsla(0. / 360., 1., 0.5, 1.))
    }

    /// Sets the background color to green for debugging when building UI.
    fn debug_bg_green(self) -> Self {
        self.bg(hsla(120. / 360., 1., 0.5, 1.))
    }

    /// Sets the background color to blue for debugging when building UI.
    fn debug_bg_blue(self) -> Self {
        self.bg(hsla(240. / 360., 1., 0.5, 1.))
    }

    /// Sets the background color to yellow for debugging when building UI.
    fn debug_bg_yellow(self) -> Self {
        self.bg(hsla(60. / 360., 1., 0.5, 1.))
    }

    /// Sets the background color to cyan for debugging when building UI.
    fn debug_bg_cyan(self) -> Self {
        self.bg(hsla(160. / 360., 1., 0.5, 1.))
    }

    /// Sets the background color to magenta for debugging when building UI.
    fn debug_bg_magenta(self) -> Self {
        self.bg(hsla(300. / 360., 1., 0.5, 1.))
    }
}

impl<E: Styled> StyledExt for E {}
