use crate::component_prelude::*;
use gpui::{AnyElement, AnyView, DefiniteLength};
use ui_macros::RegisterComponent;

use crate::{ButtonCommon, ButtonLike, ButtonSize, ButtonStyle, IconName, IconSize, Label};
use crate::{
    Color, DynamicSpacing, ElevationIndex, IconPosition, KeyBinding, KeybindingPosition, TintColor,
    prelude::*,
};

use super::button_icon::ButtonIcon;

/// An element that creates a button with a label and an optional icon.
///
/// Common buttons:
/// - Label, Icon + Label: [`Button`] (this component)
/// - Icon only: [`IconButton`]
/// - Custom: [`ButtonLike`]
///
/// To create a more complex button than what the [`Button`] or [`IconButton`] components provide, use
/// [`ButtonLike`] directly.
///
/// # Examples
///
/// **A button with a label**, is typically used in scenarios such as a form, where the button's label
/// indicates what action will be performed when the button is clicked.
///
/// ```
/// use ui::prelude::*;
///
/// Button::new("button_id", "Click me!")
///     .on_click(|event, window, cx| {
///         // Handle click event
///     });
/// ```
///
/// **A toggleable button**, is typically used in scenarios such as a toolbar,
/// where the button's state indicates whether a feature is enabled or not, or
/// a trigger for a popover menu, where clicking the button toggles the visibility of the menu.
///
/// ```
/// use ui::prelude::*;
///
/// Button::new("button_id", "Click me!")
///     .icon(IconName::Check)
///     .toggle_state(true)
///     .on_click(|event, window, cx| {
///         // Handle click event
///     });
/// ```
///
/// To change the style of the button when it is selected use the [`selected_style`][Button::selected_style] method.
///
/// ```
/// use ui::prelude::*;
/// use ui::TintColor;
///
/// Button::new("button_id", "Click me!")
///     .toggle_state(true)
///     .selected_style(ButtonStyle::Tinted(TintColor::Accent))
///     .on_click(|event, window, cx| {
///         // Handle click event
///     });
/// ```
/// This will create a button with a blue tinted background when selected.
///
/// **A full-width button**, is typically used in scenarios such as the bottom of a modal or form, where it occupies the entire width of its container.
/// The button's content, including text and icons, is centered by default.
///
/// ```
/// use ui::prelude::*;
///
/// let button = Button::new("button_id", "Click me!")
///     .full_width()
///     .on_click(|event, window, cx| {
///         // Handle click event
///     });
/// ```
///
#[derive(IntoElement, Documented, RegisterComponent)]
pub struct Button {
    base: ButtonLike,
    label: SharedString,
    label_color: Option<Color>,
    label_size: Option<LabelSize>,
    selected_label: Option<SharedString>,
    selected_label_color: Option<Color>,
    icon: Option<IconName>,
    icon_position: Option<IconPosition>,
    icon_size: Option<IconSize>,
    icon_color: Option<Color>,
    selected_icon: Option<IconName>,
    selected_icon_color: Option<Color>,
    key_binding: Option<KeyBinding>,
    key_binding_position: KeybindingPosition,
    alpha: Option<f32>,
    truncate: bool,
}

impl Button {
    /// Creates a new [`Button`] with a specified identifier and label.
    ///
    /// This is the primary constructor for a [`Button`] component. It initializes
    /// the button with the provided identifier and label text, setting all other
    /// properties to their default values, which can be customized using the
    /// builder pattern methods provided by this struct.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            base: ButtonLike::new(id),
            label: label.into(),
            label_color: None,
            label_size: None,
            selected_label: None,
            selected_label_color: None,
            icon: None,
            icon_position: None,
            icon_size: None,
            icon_color: None,
            selected_icon: None,
            selected_icon_color: None,
            key_binding: None,
            key_binding_position: KeybindingPosition::default(),
            alpha: None,
            truncate: false,
        }
    }

    /// Sets the color of the button's label.
    pub fn color(mut self, label_color: impl Into<Option<Color>>) -> Self {
        self.label_color = label_color.into();
        self
    }

    /// Defines the size of the button's label.
    pub fn label_size(mut self, label_size: impl Into<Option<LabelSize>>) -> Self {
        self.label_size = label_size.into();
        self
    }

    /// Sets the label used when the button is in a selected state.
    pub fn selected_label<L: Into<SharedString>>(mut self, label: impl Into<Option<L>>) -> Self {
        self.selected_label = label.into().map(Into::into);
        self
    }

    /// Sets the label color used when the button is in a selected state.
    pub fn selected_label_color(mut self, color: impl Into<Option<Color>>) -> Self {
        self.selected_label_color = color.into();
        self
    }

    /// Assigns an icon to the button.
    pub fn icon(mut self, icon: impl Into<Option<IconName>>) -> Self {
        self.icon = icon.into();
        self
    }

    /// Sets the position of the icon relative to the label.
    pub fn icon_position(mut self, icon_position: impl Into<Option<IconPosition>>) -> Self {
        self.icon_position = icon_position.into();
        self
    }

    /// Specifies the size of the button's icon.
    pub fn icon_size(mut self, icon_size: impl Into<Option<IconSize>>) -> Self {
        self.icon_size = icon_size.into();
        self
    }

    /// Sets the color of the button's icon.
    pub fn icon_color(mut self, icon_color: impl Into<Option<Color>>) -> Self {
        self.icon_color = icon_color.into();
        self
    }

    /// Chooses an icon to display when the button is in a selected state.
    pub fn selected_icon(mut self, icon: impl Into<Option<IconName>>) -> Self {
        self.selected_icon = icon.into();
        self
    }

    /// Sets the icon color used when the button is in a selected state.
    pub fn selected_icon_color(mut self, color: impl Into<Option<Color>>) -> Self {
        self.selected_icon_color = color.into();
        self
    }

    /// Display the keybinding that triggers the button action.
    pub fn key_binding(mut self, key_binding: impl Into<Option<KeyBinding>>) -> Self {
        self.key_binding = key_binding.into();
        self
    }

    /// Sets the position of the keybinding relative to the button label.
    ///
    /// This method allows you to specify where the keybinding should be displayed
    /// in relation to the button's label.
    pub fn key_binding_position(mut self, position: KeybindingPosition) -> Self {
        self.key_binding_position = position;
        self
    }

    /// Sets the alpha property of the color of label.
    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = Some(alpha);
        self
    }

    /// Truncates overflowing labels with an ellipsis (`…`) if needed.
    ///
    /// Buttons with static labels should _never_ be truncated, ensure
    /// this is only used when the label is dynamic and may overflow.
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }
}

impl Toggleable for Button {
    /// Sets the selected state of the button.
    ///
    /// This method allows the selection state of the button to be specified.
    /// It modifies the button's appearance to reflect its selected state.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// Button::new("button_id", "Click me!")
    ///     .toggle_state(true)
    ///     .on_click(|event, window, cx| {
    ///         // Handle click event
    ///     });
    /// ```
    ///
    /// Use [`selected_style`](Button::selected_style) to change the style of the button when it is selected.
    fn toggle_state(mut self, selected: bool) -> Self {
        self.base = self.base.toggle_state(selected);
        self
    }
}

impl SelectableButton for Button {
    /// Sets the style for the button when selected.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    /// use ui::TintColor;
    ///
    /// Button::new("button_id", "Click me!")
    ///     .toggle_state(true)
    ///     .selected_style(ButtonStyle::Tinted(TintColor::Accent))
    ///     .on_click(|event, window, cx| {
    ///         // Handle click event
    ///     });
    /// ```
    /// This results in a button with a blue tinted background when selected.
    fn selected_style(mut self, style: ButtonStyle) -> Self {
        self.base = self.base.selected_style(style);
        self
    }
}

impl Disableable for Button {
    /// Disables the button.
    ///
    /// This method allows the button to be disabled. When a button is disabled,
    /// it doesn't react to user interactions and its appearance is updated to reflect this.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// Button::new("button_id", "Click me!")
    ///     .disabled(true)
    ///     .on_click(|event, window, cx| {
    ///         // Handle click event
    ///     });
    /// ```
    ///
    /// This results in a button that is disabled and does not respond to click events.
    fn disabled(mut self, disabled: bool) -> Self {
        self.base = self.base.disabled(disabled);
        self
    }
}

impl Clickable for Button {
    /// Sets the click event handler for the button.
    fn on_click(
        mut self,
        handler: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.base = self.base.on_click(handler);
        self
    }

    fn cursor_style(mut self, cursor_style: gpui::CursorStyle) -> Self {
        self.base = self.base.cursor_style(cursor_style);
        self
    }
}

impl FixedWidth for Button {
    /// Sets a fixed width for the button.
    ///
    /// This function allows a button to have a fixed width instead of automatically growing or shrinking.
    /// Sets a fixed width for the button.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// Button::new("button_id", "Click me!")
    ///     .width(px(100.))
    ///     .on_click(|event, window, cx| {
    ///         // Handle click event
    ///     });
    /// ```
    ///
    /// This sets the button's width to be exactly 100 pixels.
    fn width(mut self, width: impl Into<DefiniteLength>) -> Self {
        self.base = self.base.width(width);
        self
    }

    /// Sets the button to occupy the full width of its container.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// Button::new("button_id", "Click me!")
    ///     .full_width()
    ///     .on_click(|event, window, cx| {
    ///         // Handle click event
    ///     });
    /// ```
    ///
    /// This stretches the button to the full width of its container.
    fn full_width(mut self) -> Self {
        self.base = self.base.full_width();
        self
    }
}

impl ButtonCommon for Button {
    /// Sets the button's id.
    fn id(&self) -> &ElementId {
        self.base.id()
    }

    /// Sets the visual style of the button using a [`ButtonStyle`].
    fn style(mut self, style: ButtonStyle) -> Self {
        self.base = self.base.style(style);
        self
    }

    /// Sets the button's size using a [`ButtonSize`].
    fn size(mut self, size: ButtonSize) -> Self {
        self.base = self.base.size(size);
        self
    }

    /// Sets a tooltip for the button.
    ///
    /// This method allows a tooltip to be set for the button. The tooltip is a function that
    /// takes a mutable references to [`Window`] and [`App`], and returns an [`AnyView`]. The
    /// tooltip is displayed when the user hovers over the button.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    /// use ui::Tooltip;
    ///
    /// Button::new("button_id", "Click me!")
    ///     .tooltip(Tooltip::text("This is a tooltip"))
    ///     .on_click(|event, window, cx| {
    ///         // Handle click event
    ///     });
    /// ```
    ///
    /// This will create a button with a tooltip that displays "This is a tooltip" when hovered over.
    fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.base = self.base.tooltip(tooltip);
        self
    }

    fn tab_index(mut self, tab_index: impl Into<isize>) -> Self {
        self.base = self.base.tab_index(tab_index);
        self
    }

    fn layer(mut self, elevation: ElevationIndex) -> Self {
        self.base = self.base.layer(elevation);
        self
    }
}

impl RenderOnce for Button {
    #[allow(refining_impl_trait)]
    fn render(self, _window: &mut Window, cx: &mut App) -> ButtonLike {
        let is_disabled = self.base.disabled;
        let is_selected = self.base.selected;

        let label = self
            .selected_label
            .filter(|_| is_selected)
            .unwrap_or(self.label);

        let label_color = if is_disabled {
            Color::Disabled
        } else if is_selected {
            self.selected_label_color.unwrap_or(Color::Selected)
        } else {
            self.label_color.unwrap_or_default()
        };

        self.base.child(
            h_flex()
                .gap(DynamicSpacing::Base04.rems(cx))
                .when(self.icon_position == Some(IconPosition::Start), |this| {
                    this.children(self.icon.map(|icon| {
                        ButtonIcon::new(icon)
                            .disabled(is_disabled)
                            .toggle_state(is_selected)
                            .selected_icon(self.selected_icon)
                            .selected_icon_color(self.selected_icon_color)
                            .size(self.icon_size)
                            .color(self.icon_color)
                    }))
                })
                .child(
                    h_flex()
                        .when(
                            self.key_binding_position == KeybindingPosition::Start,
                            |this| this.flex_row_reverse(),
                        )
                        .gap(DynamicSpacing::Base06.rems(cx))
                        .justify_between()
                        .child(
                            Label::new(label)
                                .color(label_color)
                                .size(self.label_size.unwrap_or_default())
                                .when_some(self.alpha, |this, alpha| this.alpha(alpha))
                                .when(self.truncate, |this| this.truncate()),
                        )
                        .children(self.key_binding),
                )
                .when(self.icon_position != Some(IconPosition::Start), |this| {
                    this.children(self.icon.map(|icon| {
                        ButtonIcon::new(icon)
                            .disabled(is_disabled)
                            .toggle_state(is_selected)
                            .selected_icon(self.selected_icon)
                            .selected_icon_color(self.selected_icon_color)
                            .size(self.icon_size)
                            .color(self.icon_color)
                    }))
                }),
        )
    }
}

// View this component preview using `workspace: open component-preview`
impl Component for Button {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn sort_name() -> &'static str {
        "ButtonA"
    }

    fn description() -> Option<&'static str> {
        Some("A button triggers an event or action.")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Button Styles",
                        vec![
                            single_example(
                                "Default",
                                Button::new("default", "Default").into_any_element(),
                            ),
                            single_example(
                                "Filled",
                                Button::new("filled", "Filled")
                                    .style(ButtonStyle::Filled)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Subtle",
                                Button::new("outline", "Subtle")
                                    .style(ButtonStyle::Subtle)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Tinted",
                                Button::new("tinted_accent_style", "Accent")
                                    .style(ButtonStyle::Tinted(TintColor::Accent))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Transparent",
                                Button::new("transparent", "Transparent")
                                    .style(ButtonStyle::Transparent)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Tint Styles",
                        vec![
                            single_example(
                                "Accent",
                                Button::new("tinted_accent", "Accent")
                                    .style(ButtonStyle::Tinted(TintColor::Accent))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Error",
                                Button::new("tinted_negative", "Error")
                                    .style(ButtonStyle::Tinted(TintColor::Error))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Warning",
                                Button::new("tinted_warning", "Warning")
                                    .style(ButtonStyle::Tinted(TintColor::Warning))
                                    .into_any_element(),
                            ),
                            single_example(
                                "Success",
                                Button::new("tinted_positive", "Success")
                                    .style(ButtonStyle::Tinted(TintColor::Success))
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Special States",
                        vec![
                            single_example(
                                "Default",
                                Button::new("default_state", "Default").into_any_element(),
                            ),
                            single_example(
                                "Disabled",
                                Button::new("disabled", "Disabled")
                                    .disabled(true)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Selected",
                                Button::new("selected", "Selected")
                                    .toggle_state(true)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Buttons with Icons",
                        vec![
                            single_example(
                                "Icon Start",
                                Button::new("icon_start", "Icon Start")
                                    .icon(IconName::Check)
                                    .icon_position(IconPosition::Start)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Icon End",
                                Button::new("icon_end", "Icon End")
                                    .icon(IconName::Check)
                                    .icon_position(IconPosition::End)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Icon Color",
                                Button::new("icon_color", "Icon Color")
                                    .icon(IconName::Check)
                                    .icon_color(Color::Accent)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                ])
                .into_any_element(),
        )
    }
}
