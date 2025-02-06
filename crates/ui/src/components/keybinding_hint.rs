use crate::{h_flex, prelude::*};
use crate::{ElevationIndex, KeyBinding};
use gpui::{point, App, BoxShadow, IntoElement, Window};
use smallvec::smallvec;

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
#[derive(Debug, IntoElement, Clone)]
pub struct KeybindingHint {
    prefix: Option<SharedString>,
    suffix: Option<SharedString>,
    keybinding: KeyBinding,
    size: Option<Pixels>,
    elevation: Option<ElevationIndex>,
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
    /// let hint = KeybindingHint::new(KeyBinding::from_str("Ctrl+C"));
    /// ```
    pub fn new(keybinding: KeyBinding) -> Self {
        Self {
            prefix: None,
            suffix: None,
            keybinding,
            size: None,
            elevation: None,
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
    /// let hint = KeybindingHint::with_prefix("Copy:", KeyBinding::from_str("Ctrl+C"));
    /// ```
    pub fn with_prefix(prefix: impl Into<SharedString>, keybinding: KeyBinding) -> Self {
        Self {
            prefix: Some(prefix.into()),
            suffix: None,
            keybinding,
            size: None,
            elevation: None,
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
    /// let hint = KeybindingHint::with_suffix(KeyBinding::from_str("Ctrl+V"), "Paste");
    /// ```
    pub fn with_suffix(keybinding: KeyBinding, suffix: impl Into<SharedString>) -> Self {
        Self {
            prefix: None,
            suffix: Some(suffix.into()),
            keybinding,
            size: None,
            elevation: None,
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

    /// Sets the elevation of the keybinding hint.
    ///
    /// This method allows specifying the elevation index for the keybinding hint,
    /// which affects its visual appearance in terms of depth or layering.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let hint = KeybindingHint::new(KeyBinding::from_str("Ctrl+A"))
    ///     .elevation(ElevationIndex::new(1));
    /// ```
    pub fn elevation(mut self, elevation: impl Into<Option<ElevationIndex>>) -> Self {
        self.elevation = elevation.into();
        self
    }
}

impl RenderOnce for KeybindingHint {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors().clone();

        let size = self
            .size
            .unwrap_or(TextSize::Small.rems(cx).to_pixels(window.rem_size()));
        let kb_size = size - px(2.0);
        let kb_bg = if let Some(elevation) = self.elevation {
            elevation.on_elevation_bg(cx)
        } else {
            theme::color_alpha(colors.element_background, 0.6)
        };

        h_flex()
            .items_center()
            .gap_0p5()
            .font_buffer(cx)
            .text_size(size)
            .text_color(colors.text_muted)
            .children(self.prefix)
            .child(
                h_flex()
                    .items_center()
                    .rounded_md()
                    .px_0p5()
                    .mr_0p5()
                    .border_1()
                    .border_color(kb_bg)
                    .bg(kb_bg.opacity(0.8))
                    .shadow(smallvec![BoxShadow {
                        color: cx.theme().colors().editor_background.opacity(0.8),
                        offset: point(px(0.), px(1.)),
                        blur_radius: px(0.),
                        spread_radius: px(0.),
                    }])
                    .child(self.keybinding.size(kb_size)),
            )
            .children(self.suffix)
    }
}

impl ComponentPreview for KeybindingHint {
    fn description() -> impl Into<Option<&'static str>> {
        "Used to display hint text for keyboard shortcuts. Can have a prefix and suffix."
    }

    fn examples(window: &mut Window, _cx: &mut App) -> Vec<ComponentExampleGroup<Self>> {
        let home_fallback = gpui::KeyBinding::new("home", menu::SelectFirst, None);
        let home = KeyBinding::for_action(&menu::SelectFirst, window)
            .unwrap_or(KeyBinding::new(home_fallback));

        let end_fallback = gpui::KeyBinding::new("end", menu::SelectLast, None);
        let end = KeyBinding::for_action(&menu::SelectLast, window)
            .unwrap_or(KeyBinding::new(end_fallback));

        let enter_fallback = gpui::KeyBinding::new("enter", menu::Confirm, None);
        let enter = KeyBinding::for_action(&menu::Confirm, window)
            .unwrap_or(KeyBinding::new(enter_fallback));

        let escape_fallback = gpui::KeyBinding::new("escape", menu::Cancel, None);
        let escape = KeyBinding::for_action(&menu::Cancel, window)
            .unwrap_or(KeyBinding::new(escape_fallback));

        vec![
            example_group_with_title(
                "Basic",
                vec![
                    single_example(
                        "With Prefix",
                        KeybindingHint::with_prefix("Go to Start:", home.clone()),
                    ),
                    single_example(
                        "With Suffix",
                        KeybindingHint::with_suffix(end.clone(), "Go to End"),
                    ),
                    single_example(
                        "With Prefix and Suffix",
                        KeybindingHint::new(enter.clone())
                            .prefix("Confirm:")
                            .suffix("Execute selected action"),
                    ),
                ],
            ),
            example_group_with_title(
                "Sizes",
                vec![
                    single_example(
                        "Small",
                        KeybindingHint::new(home.clone())
                            .size(Pixels::from(12.0))
                            .prefix("Small:"),
                    ),
                    single_example(
                        "Medium",
                        KeybindingHint::new(end.clone())
                            .size(Pixels::from(16.0))
                            .suffix("Medium"),
                    ),
                    single_example(
                        "Large",
                        KeybindingHint::new(enter.clone())
                            .size(Pixels::from(20.0))
                            .prefix("Large:")
                            .suffix("Size"),
                    ),
                ],
            ),
            example_group_with_title(
                "Elevations",
                vec![
                    single_example(
                        "Surface",
                        KeybindingHint::new(home.clone())
                            .elevation(ElevationIndex::Surface)
                            .prefix("Surface:"),
                    ),
                    single_example(
                        "Elevated Surface",
                        KeybindingHint::new(end.clone())
                            .elevation(ElevationIndex::ElevatedSurface)
                            .suffix("Elevated"),
                    ),
                    single_example(
                        "Editor Surface",
                        KeybindingHint::new(enter.clone())
                            .elevation(ElevationIndex::EditorSurface)
                            .prefix("Editor:")
                            .suffix("Surface"),
                    ),
                    single_example(
                        "Modal Surface",
                        KeybindingHint::new(escape.clone())
                            .elevation(ElevationIndex::ModalSurface)
                            .prefix("Modal:")
                            .suffix("Escape"),
                    ),
                ],
            ),
        ]
    }
}
