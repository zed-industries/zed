use crate::KeyBinding;
use crate::prelude::*;
use gpui::{AnyElement, App, BoxShadow, FontStyle, Hsla, IntoElement, Window, point};
use theme::Appearance;

/// Represents a hint for a keybinding, optionally with a prefix and suffix.
///
/// This struct allows for the creation and customization of a keybinding hint,
/// which can be used to display keyboard shortcuts or commands in a user interface.
///
/// # Examples
///
/// ```no_run
/// use gpui::{App, Hsla, KeybindingKeystroke, Keystroke};
/// use ui::prelude::*;
/// use ui::{KeyBinding, KeybindingHint};
/// use settings::KeybindSource;
///
/// # fn example(cx: &App) {
/// let hint = KeybindingHint::new(
///     KeyBinding::from_keystrokes(vec![KeybindingKeystroke::from_keystroke(Keystroke::parse("ctrl-s").unwrap())].into(), KeybindSource::Base),
///     Hsla::black()
/// )
///     .prefix("Save:")
///     .size(Pixels::from(14.0));
/// # }
/// ```
#[derive(Debug, IntoElement, RegisterComponent)]
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
    /// ```no_run
    /// use gpui::{App, Hsla, KeybindingKeystroke, Keystroke};
    /// use ui::prelude::*;
    /// use ui::{KeyBinding, KeybindingHint};
    /// use settings::KeybindSource;
    ///
    /// # fn example(cx: &App) {
    /// let hint = KeybindingHint::new(
    ///     KeyBinding::from_keystrokes(vec![KeybindingKeystroke::from_keystroke(Keystroke::parse("ctrl-c").unwrap())].into(), KeybindSource::Base),
    ///     Hsla::black()
    /// );
    /// # }
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
    /// ```no_run
    /// use gpui::{App, Hsla, KeybindingKeystroke, Keystroke};
    /// use ui::prelude::*;
    /// use ui::{KeyBinding, KeybindingHint};
    /// use settings::KeybindSource;
    ///
    /// # fn example(cx: &App) {
    /// let hint = KeybindingHint::with_prefix(
    ///     "Copy:",
    ///     KeyBinding::from_keystrokes(vec![KeybindingKeystroke::from_keystroke(Keystroke::parse("ctrl-c").unwrap())].into(), KeybindSource::Base),
    ///     Hsla::black()
    /// );
    /// # }
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
    /// ```no_run
    /// use gpui::{App, Hsla, KeybindingKeystroke, Keystroke};
    /// use ui::prelude::*;
    /// use ui::{KeyBinding, KeybindingHint};
    /// use settings::KeybindSource;
    ///
    /// # fn example(cx: &App) {
    /// let hint = KeybindingHint::with_suffix(
    ///     KeyBinding::from_keystrokes(vec![KeybindingKeystroke::from_keystroke(Keystroke::parse("ctrl-v").unwrap())].into(), KeybindSource::Base),
    ///     "Paste",
    ///     Hsla::black()
    /// );
    /// # }
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
    /// ```no_run
    /// use gpui::{App, Hsla, KeybindingKeystroke, Keystroke};
    /// use ui::prelude::*;
    /// use ui::{KeyBinding, KeybindingHint};
    /// use settings::KeybindSource;
    ///
    /// # fn example(cx: &App) {
    /// let hint = KeybindingHint::new(
    ///     KeyBinding::from_keystrokes(vec![KeybindingKeystroke::from_keystroke(Keystroke::parse("ctrl-x").unwrap())].into(), KeybindSource::Base),
    ///     Hsla::black()
    /// )
    ///     .prefix("Cut:");
    /// # }
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
    /// ```no_run
    /// use gpui::{App, Hsla, KeybindingKeystroke, Keystroke};
    /// use ui::prelude::*;
    /// use ui::{KeyBinding, KeybindingHint};
    /// use settings::KeybindSource;
    ///
    /// # fn example(cx: &App) {
    /// let hint = KeybindingHint::new(
    ///     KeyBinding::from_keystrokes(vec![KeybindingKeystroke::from_keystroke(Keystroke::parse("ctrl-f").unwrap())].into(), KeybindSource::Base),
    ///     Hsla::black()
    /// )
    ///     .suffix("Find");
    /// # }
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
    /// ```no_run
    /// use gpui::{App, Hsla, KeybindingKeystroke, Keystroke};
    /// use ui::prelude::*;
    /// use ui::{KeyBinding, KeybindingHint};
    /// use settings::KeybindSource;
    ///
    /// # fn example(cx: &App) {
    /// let hint = KeybindingHint::new(
    ///     KeyBinding::from_keystrokes(vec![KeybindingKeystroke::from_keystroke(Keystroke::parse("ctrl-z").unwrap())].into(), KeybindSource::Base),
    ///     Hsla::black()
    /// )
    ///     .size(Pixels::from(16.0));
    /// # }
    /// ```
    pub fn size(mut self, size: impl Into<Option<Pixels>>) -> Self {
        self.size = size.into();
        self
    }
}

impl RenderOnce for KeybindingHint {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();
        let is_light = cx.theme().appearance() == Appearance::Light;

        let border_color =
            self.background_color
                .blend(colors.text.alpha(if is_light { 0.08 } else { 0.16 }));

        let bg_color = self
            .background_color
            .blend(colors.text_accent.alpha(if is_light { 0.05 } else { 0.1 }));

        let shadow_color = colors.text.alpha(if is_light { 0.04 } else { 0.08 });

        let size = self
            .size
            .unwrap_or(TextSize::Small.rems(cx).to_pixels(window.rem_size()));

        let kb_size = size - px(2.0);

        let mut base = h_flex();

        base.text_style()
            .get_or_insert_with(Default::default)
            .font_style = Some(FontStyle::Italic);

        base.gap_1()
            .font_buffer(cx)
            .text_size(size)
            .text_color(colors.text_disabled)
            .children(self.prefix)
            .child(
                h_flex()
                    .rounded_sm()
                    .px_0p5()
                    .mr_0p5()
                    .border_1()
                    .border_color(border_color)
                    .bg(bg_color)
                    .shadow(vec![BoxShadow {
                        color: shadow_color,
                        offset: point(px(0.), px(1.)),
                        blur_radius: px(0.),
                        spread_radius: px(0.),
                    }])
                    .child(self.keybinding.size(rems_from_px(kb_size))),
            )
            .children(self.suffix)
    }
}

impl Component for KeybindingHint {
    fn scope() -> ComponentScope {
        ComponentScope::DataDisplay
    }

    fn description() -> Option<&'static str> {
        Some("Displays a keyboard shortcut hint with optional prefix and suffix text")
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let enter = KeyBinding::for_action(&menu::Confirm, cx);

        let bg_color = cx.theme().colors().surface_background;

        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Basic",
                        vec![
                            single_example(
                                "With Prefix",
                                KeybindingHint::with_prefix(
                                    "Go to Start:",
                                    enter.clone(),
                                    bg_color,
                                )
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
                                KeybindingHint::new(enter, bg_color)
                                    .size(Pixels::from(20.0))
                                    .prefix("Large:")
                                    .suffix("Size")
                                    .into_any_element(),
                            ),
                        ],
                    ),
                ])
                .into_any_element(),
        )
    }
}
