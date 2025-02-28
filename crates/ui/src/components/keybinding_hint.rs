use crate::KeyBinding;
use crate::{h_flex, prelude::*};
use gpui::{point, AnyElement, App, BoxShadow, FontStyle, Hsla, IntoElement, Window};
use smallvec::smallvec;
use theme::Appearance;

/// Represents a hint for a keybinding, optionally with a prefix and suffix.
///
/// This struct allows for the creation and customization of a keybinding hint,
/// which can be used to display keyboard shortcuts or commands in a user interface.
///
/// # Examples
///
/// ```
/// use ui::prelude::*;
///
/// let hint = KeybindingHint::new(KeyBinding::from_str("Ctrl+S"))
///     .prefix("Save:")
///     .size(Pixels::from(14.0));
/// ```
#[derive(Debug, IntoElement, IntoComponent)]
pub struct KeybindingHint {
    prefix: Option<SharedString>,
    suffix: Option<SharedString>,
    keybinding: KeyBinding,
    size: Option<Pixels>,
    background_color: Hsla,
}

impl KeybindingHint {
    /// Creates a new `KeybindingHint` with the specified keybinding.
    ///
    /// This method initializes a new `KeybindingHint` instance with the given keybinding,
    /// setting all other fields to their default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let hint = KeybindingHint::new(KeyBinding::from_str("Ctrl+C"), Hsla::new(0.0, 0.0, 0.0, 1.0));
    /// ```
    pub fn new(keybinding: KeyBinding, background_color: Hsla) -> Self {
        Self {
            prefix: None,
            suffix: None,
            keybinding,
            size: None,
            background_color,
        }
    }

    /// Creates a new `KeybindingHint` with a prefix and keybinding.
    ///
    /// This method initializes a new `KeybindingHint` instance with the given prefix and keybinding,
    /// setting all other fields to their default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let hint = KeybindingHint::with_prefix("Copy:", KeyBinding::from_str("Ctrl+C"), Hsla::new(0.0, 0.0, 0.0, 1.0));
    /// ```
    pub fn with_prefix(
        prefix: impl Into<SharedString>,
        keybinding: KeyBinding,
        background_color: Hsla,
    ) -> Self {
        Self {
            prefix: Some(prefix.into()),
            suffix: None,
            keybinding,
            size: None,
            background_color,
        }
    }

    /// Creates a new `KeybindingHint` with a keybinding and suffix.
    ///
    /// This method initializes a new `KeybindingHint` instance with the given keybinding and suffix,
    /// setting all other fields to their default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let hint = KeybindingHint::with_suffix(KeyBinding::from_str("Ctrl+V"), "Paste", Hsla::new(0.0, 0.0, 0.0, 1.0));
    /// ```
    pub fn with_suffix(
        keybinding: KeyBinding,
        suffix: impl Into<SharedString>,
        background_color: Hsla,
    ) -> Self {
        Self {
            prefix: None,
            suffix: Some(suffix.into()),
            keybinding,
            size: None,
            background_color,
        }
    }

    /// Sets the prefix for the keybinding hint.
    ///
    /// This method allows adding or changing the prefix text that appears before the keybinding.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let hint = KeybindingHint::new(KeyBinding::from_str("Ctrl+X"))
    ///     .prefix("Cut:");
    /// ```
    pub fn prefix(mut self, prefix: impl Into<SharedString>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Sets the suffix for the keybinding hint.
    ///
    /// This method allows adding or changing the suffix text that appears after the keybinding.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let hint = KeybindingHint::new(KeyBinding::from_str("Ctrl+F"))
    ///     .suffix("Find");
    /// ```
    pub fn suffix(mut self, suffix: impl Into<SharedString>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Sets the size of the keybinding hint.
    ///
    /// This method allows specifying the size of the keybinding hint in pixels.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let hint = KeybindingHint::new(KeyBinding::from_str("Ctrl+Z"))
    ///     .size(Pixels::from(16.0));
    /// ```
    pub fn size(mut self, size: impl Into<Option<Pixels>>) -> Self {
        self.size = size.into();
        self
    }
}

impl RenderOnce for KeybindingHint {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors().clone();
        let is_light = cx.theme().appearance() == Appearance::Light;

        let border_color =
            self.background_color
                .blend(colors.text.alpha(if is_light { 0.08 } else { 0.16 }));
        let bg_color =
            self.background_color
                .blend(colors.text.alpha(if is_light { 0.06 } else { 0.12 }));
        let shadow_color = colors.text.alpha(if is_light { 0.04 } else { 0.08 });

        let size = self
            .size
            .unwrap_or(TextSize::Small.rems(cx).to_pixels(window.rem_size()));
        let kb_size = size - px(2.0);

        let mut base = h_flex();

        base.text_style()
            .get_or_insert_with(Default::default)
            .font_style = Some(FontStyle::Italic);

        base.items_center()
            .gap_0p5()
            .font_buffer(cx)
            .text_size(size)
            .text_color(colors.text_disabled)
            .children(self.prefix)
            .child(
                h_flex()
                    .items_center()
                    .rounded_md()
                    .px_0p5()
                    .mr_0p5()
                    .border_1()
                    .border_color(border_color)
                    .bg(bg_color)
                    .shadow(smallvec![BoxShadow {
                        color: shadow_color,
                        offset: point(px(0.), px(1.)),
                        blur_radius: px(0.),
                        spread_radius: px(0.),
                    }])
                    .child(self.keybinding.size(rems_from_px(kb_size.0))),
            )
            .children(self.suffix)
    }
}

// View this component preview using `workspace: open component-preview`
impl ComponentPreview for KeybindingHint {
    fn preview(window: &mut Window, cx: &mut App) -> AnyElement {
        let enter_fallback = gpui::KeyBinding::new("enter", menu::Confirm, None);
        let enter = KeyBinding::for_action(&menu::Confirm, window, cx)
            .unwrap_or(KeyBinding::new(enter_fallback, cx));

        let bg_color = cx.theme().colors().surface_background;

        v_flex()
            .gap_6()
            .children(vec![
                example_group_with_title(
                    "Basic",
                    vec![
                        single_example(
                            "With Prefix",
                            KeybindingHint::with_prefix("Go to Start:", enter.clone(), bg_color)
                                .into_any_element(),
                        ),
                        single_example(
                            "With Suffix",
                            KeybindingHint::with_suffix(enter.clone(), "Go to End", bg_color)
                                .into_any_element(),
                        ),
                        single_example(
                            "With Prefix and Suffix",
                            KeybindingHint::new(enter.clone(), bg_color)
                                .prefix("Confirm:")
                                .suffix("Execute selected action")
                                .into_any_element(),
                        ),
                    ],
                ),
                example_group_with_title(
                    "Sizes",
                    vec![
                        single_example(
                            "Small",
                            KeybindingHint::new(enter.clone(), bg_color)
                                .size(Pixels::from(12.0))
                                .prefix("Small:")
                                .into_any_element(),
                        ),
                        single_example(
                            "Medium",
                            KeybindingHint::new(enter.clone(), bg_color)
                                .size(Pixels::from(16.0))
                                .suffix("Medium")
                                .into_any_element(),
                        ),
                        single_example(
                            "Large",
                            KeybindingHint::new(enter.clone(), bg_color)
                                .size(Pixels::from(20.0))
                                .prefix("Large:")
                                .suffix("Size")
                                .into_any_element(),
                        ),
                    ],
                ),
            ])
            .into_any_element()
    }
}
