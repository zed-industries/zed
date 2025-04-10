use documented::Documented;
use gpui::{
    AnyElement, AnyView, ClickEvent, CursorStyle, DefiniteLength, Hsla, MouseButton,
    MouseDownEvent, MouseUpEvent, Rems, relative, transparent_black,
};
use smallvec::SmallVec;

use crate::{DynamicSpacing, ElevationIndex, prelude::*};

/// A trait for buttons that can be Selected. Enables setting the [`ButtonStyle`] of a button when it is selected.
pub trait SelectableButton: Toggleable {
    fn selected_style(self, style: ButtonStyle) -> Self;
}

/// A common set of traits all buttons must implement.
pub trait ButtonCommon: Clickable + Disableable {
    /// A unique element ID to identify the button.
    fn id(&self) -> &ElementId;

    /// The visual style of the button.
    ///
    /// Most commonly will be [`ButtonStyle::Subtle`], or [`ButtonStyle::Filled`]
    /// for an emphasized button.
    fn style(self, style: ButtonStyle) -> Self;

    /// The size of the button.
    ///
    /// Most buttons will use the default size.
    ///
    /// [`ButtonSize`] can also be used to help build non-button elements
    /// that are consistently sized with buttons.
    fn size(self, size: ButtonSize) -> Self;

    /// The tooltip that shows when a user hovers over the button.
    ///
    /// Nearly all interactable elements should have a tooltip. Some example
    /// exceptions might a scroll bar, or a slider.
    fn tooltip(self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self;

    fn layer(self, elevation: ElevationIndex) -> Self;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum IconPosition {
    #[default]
    Start,
    End,
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum KeybindingPosition {
    Start,
    #[default]
    End,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum TintColor {
    #[default]
    Accent,
    Error,
    Warning,
    Success,
}

impl TintColor {
    fn button_like_style(self, cx: &mut App) -> ButtonLikeStyles {
        match self {
            TintColor::Accent => ButtonLikeStyles {
                background: cx.theme().status().info_background,
                border_color: cx.theme().status().info_border,
                label_color: cx.theme().colors().text,
                icon_color: cx.theme().colors().text,
            },
            TintColor::Error => ButtonLikeStyles {
                background: cx.theme().status().error_background,
                border_color: cx.theme().status().error_border,
                label_color: cx.theme().colors().text,
                icon_color: cx.theme().colors().text,
            },
            TintColor::Warning => ButtonLikeStyles {
                background: cx.theme().status().warning_background,
                border_color: cx.theme().status().warning_border,
                label_color: cx.theme().colors().text,
                icon_color: cx.theme().colors().text,
            },
            TintColor::Success => ButtonLikeStyles {
                background: cx.theme().status().success_background,
                border_color: cx.theme().status().success_border,
                label_color: cx.theme().colors().text,
                icon_color: cx.theme().colors().text,
            },
        }
    }
}

impl From<TintColor> for Color {
    fn from(tint: TintColor) -> Self {
        match tint {
            TintColor::Accent => Color::Accent,
            TintColor::Error => Color::Error,
            TintColor::Warning => Color::Warning,
            TintColor::Success => Color::Success,
        }
    }
}

// Used to go from ButtonStyle -> Color through tint colors.
impl From<ButtonStyle> for Color {
    fn from(style: ButtonStyle) -> Self {
        match style {
            ButtonStyle::Tinted(tint) => tint.into(),
            _ => Color::Default,
        }
    }
}

/// The visual appearance of a button.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum ButtonStyle {
    /// A filled button with a solid background color. Provides emphasis versus
    /// the more common subtle button.
    Filled,

    /// Used to emphasize a button in some way, like a selected state, or a semantic
    /// coloring like an error or success button.
    Tinted(TintColor),

    /// The default button style, used for most buttons. Has a transparent background,
    /// but has a background color to indicate states like hover and active.
    #[default]
    Subtle,

    /// Used for buttons that only change foreground color on hover and active states.
    ///
    /// TODO: Better docs for this.
    Transparent,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub(crate) enum ButtonLikeRounding {
    All,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub(crate) struct ButtonLikeStyles {
    pub background: Hsla,
    #[allow(unused)]
    pub border_color: Hsla,
    #[allow(unused)]
    pub label_color: Hsla,
    #[allow(unused)]
    pub icon_color: Hsla,
}

fn element_bg_from_elevation(elevation: Option<ElevationIndex>, cx: &mut App) -> Hsla {
    match elevation {
        Some(ElevationIndex::Background) => cx.theme().colors().element_background,
        Some(ElevationIndex::ElevatedSurface) => cx.theme().colors().elevated_surface_background,
        Some(ElevationIndex::Surface) => cx.theme().colors().surface_background,
        Some(ElevationIndex::ModalSurface) => cx.theme().colors().background,
        _ => cx.theme().colors().element_background,
    }
}

impl ButtonStyle {
    pub(crate) fn enabled(
        self,
        elevation: Option<ElevationIndex>,

        cx: &mut App,
    ) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => ButtonLikeStyles {
                background: element_bg_from_elevation(elevation, cx),
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Tinted(tint) => tint.button_like_style(cx),
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: cx.theme().colors().ghost_element_background,
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
        }
    }

    pub(crate) fn hovered(
        self,
        elevation: Option<ElevationIndex>,

        cx: &mut App,
    ) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => {
                let mut filled_background = element_bg_from_elevation(elevation, cx);
                filled_background.fade_out(0.92);

                ButtonLikeStyles {
                    background: filled_background,
                    border_color: transparent_black(),
                    label_color: Color::Default.color(cx),
                    icon_color: Color::Default.color(cx),
                }
            }
            ButtonStyle::Tinted(tint) => {
                let mut styles = tint.button_like_style(cx);
                let theme = cx.theme();
                styles.background = theme.darken(styles.background, 0.05, 0.2);
                styles
            }
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: cx.theme().colors().ghost_element_hover,
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: transparent_black(),
                // TODO: These are not great
                label_color: Color::Muted.color(cx),
                // TODO: These are not great
                icon_color: Color::Muted.color(cx),
            },
        }
    }

    pub(crate) fn active(self, cx: &mut App) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => ButtonLikeStyles {
                background: cx.theme().colors().element_active,
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Tinted(tint) => tint.button_like_style(cx),
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: cx.theme().colors().ghost_element_active,
                border_color: transparent_black(),
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: transparent_black(),
                // TODO: These are not great
                label_color: Color::Muted.color(cx),
                // TODO: These are not great
                icon_color: Color::Muted.color(cx),
            },
        }
    }

    #[allow(unused)]
    pub(crate) fn focused(self, window: &mut Window, cx: &mut App) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => ButtonLikeStyles {
                background: cx.theme().colors().element_background,
                border_color: cx.theme().colors().border_focused,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Tinted(tint) => tint.button_like_style(cx),
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: cx.theme().colors().ghost_element_background,
                border_color: cx.theme().colors().border_focused,
                label_color: Color::Default.color(cx),
                icon_color: Color::Default.color(cx),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: cx.theme().colors().border_focused,
                label_color: Color::Accent.color(cx),
                icon_color: Color::Accent.color(cx),
            },
        }
    }

    #[allow(unused)]
    pub(crate) fn disabled(
        self,
        elevation: Option<ElevationIndex>,
        window: &mut Window,
        cx: &mut App,
    ) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => ButtonLikeStyles {
                background: cx.theme().colors().element_disabled,
                border_color: cx.theme().colors().border_disabled,
                label_color: Color::Disabled.color(cx),
                icon_color: Color::Disabled.color(cx),
            },
            ButtonStyle::Tinted(tint) => tint.button_like_style(cx),
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: cx.theme().colors().ghost_element_disabled,
                border_color: cx.theme().colors().border_disabled,
                label_color: Color::Disabled.color(cx),
                icon_color: Color::Disabled.color(cx),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: transparent_black(),
                label_color: Color::Disabled.color(cx),
                icon_color: Color::Disabled.color(cx),
            },
        }
    }
}

/// The height of a button.
///
/// Can also be used to size non-button elements to align with [`Button`]s.
#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonSize {
    Large,
    #[default]
    Default,
    Compact,
    None,
}

impl ButtonSize {
    pub fn rems(self) -> Rems {
        match self {
            ButtonSize::Large => rems_from_px(32.),
            ButtonSize::Default => rems_from_px(22.),
            ButtonSize::Compact => rems_from_px(18.),
            ButtonSize::None => rems_from_px(16.),
        }
    }
}

/// A button-like element that can be used to create a custom button when
/// prebuilt buttons are not sufficient. Use this sparingly, as it is
/// unconstrained and may make the UI feel less consistent.
///
/// This is also used to build the prebuilt buttons.
#[derive(IntoElement, Documented, RegisterComponent)]
pub struct ButtonLike {
    pub(super) base: Div,
    id: ElementId,
    pub(super) style: ButtonStyle,
    pub(super) disabled: bool,
    pub(super) selected: bool,
    pub(super) selected_style: Option<ButtonStyle>,
    pub(super) width: Option<DefiniteLength>,
    pub(super) height: Option<DefiniteLength>,
    pub(super) layer: Option<ElevationIndex>,
    size: ButtonSize,
    rounding: Option<ButtonLikeRounding>,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView>>,
    cursor_style: CursorStyle,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_right_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    children: SmallVec<[AnyElement; 2]>,
}

impl ButtonLike {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div(),
            id: id.into(),
            style: ButtonStyle::default(),
            disabled: false,
            selected: false,
            selected_style: None,
            width: None,
            height: None,
            size: ButtonSize::Default,
            rounding: Some(ButtonLikeRounding::All),
            tooltip: None,
            children: SmallVec::new(),
            cursor_style: CursorStyle::PointingHand,
            on_click: None,
            on_right_click: None,
            layer: None,
        }
    }

    pub fn new_rounded_left(id: impl Into<ElementId>) -> Self {
        Self::new(id).rounding(ButtonLikeRounding::Left)
    }

    pub fn new_rounded_right(id: impl Into<ElementId>) -> Self {
        Self::new(id).rounding(ButtonLikeRounding::Right)
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.base = self.base.opacity(opacity);
        self
    }

    pub fn height(mut self, height: DefiniteLength) -> Self {
        self.height = Some(height);
        self
    }

    pub(crate) fn rounding(mut self, rounding: impl Into<Option<ButtonLikeRounding>>) -> Self {
        self.rounding = rounding.into();
        self
    }

    pub fn on_right_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_right_click = Some(Box::new(handler));
        self
    }
}

impl Disableable for ButtonLike {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Toggleable for ButtonLike {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl SelectableButton for ButtonLike {
    fn selected_style(mut self, style: ButtonStyle) -> Self {
        self.selected_style = Some(style);
        self
    }
}

impl Clickable for ButtonLike {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    fn cursor_style(mut self, cursor_style: CursorStyle) -> Self {
        self.cursor_style = cursor_style;
        self
    }
}

impl FixedWidth for ButtonLike {
    fn width(mut self, width: DefiniteLength) -> Self {
        self.width = Some(width);
        self
    }

    fn full_width(mut self) -> Self {
        self.width = Some(relative(1.));
        self
    }
}

impl ButtonCommon for ButtonLike {
    fn id(&self) -> &ElementId {
        &self.id
    }

    fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }

    fn layer(mut self, elevation: ElevationIndex) -> Self {
        self.layer = Some(elevation);
        self
    }
}

impl VisibleOnHover for ButtonLike {
    fn visible_on_hover(mut self, group_name: impl Into<SharedString>) -> Self {
        self.base = self.base.visible_on_hover(group_name);
        self
    }
}

impl ParentElement for ButtonLike {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for ButtonLike {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = self
            .selected_style
            .filter(|_| self.selected)
            .unwrap_or(self.style);

        self.base
            .h_flex()
            .id(self.id.clone())
            .font_ui(cx)
            .group("")
            .flex_none()
            .h(self.height.unwrap_or(self.size.rems().into()))
            .when_some(self.width, |this, width| {
                this.w(width).justify_center().text_center()
            })
            .when_some(self.rounding, |this, rounding| match rounding {
                ButtonLikeRounding::All => this.rounded_sm(),
                ButtonLikeRounding::Left => this.rounded_l_sm(),
                ButtonLikeRounding::Right => this.rounded_r_sm(),
            })
            .gap(DynamicSpacing::Base04.rems(cx))
            .map(|this| match self.size {
                ButtonSize::Large => this.px(DynamicSpacing::Base06.rems(cx)),
                ButtonSize::Default | ButtonSize::Compact => {
                    this.px(DynamicSpacing::Base04.rems(cx))
                }
                ButtonSize::None => this,
            })
            .bg(style.enabled(self.layer, cx).background)
            .when(self.disabled, |this| this.cursor_not_allowed())
            .when(!self.disabled, |this| {
                this.cursor_pointer()
                    .hover(|hover| hover.bg(style.hovered(self.layer, cx).background))
                    .active(|active| active.bg(style.active(cx).background))
            })
            .when_some(
                self.on_right_click.filter(|_| !self.disabled),
                |this, on_right_click| {
                    this.on_mouse_down(MouseButton::Right, |_event, window, cx| {
                        window.prevent_default();
                        cx.stop_propagation();
                    })
                    .on_mouse_up(
                        MouseButton::Right,
                        move |event, window, cx| {
                            cx.stop_propagation();
                            let click_event = ClickEvent {
                                down: MouseDownEvent {
                                    button: MouseButton::Right,
                                    position: event.position,
                                    modifiers: event.modifiers,
                                    click_count: 1,
                                    first_mouse: false,
                                },
                                up: MouseUpEvent {
                                    button: MouseButton::Right,
                                    position: event.position,
                                    modifiers: event.modifiers,
                                    click_count: 1,
                                },
                            };
                            (on_right_click)(&click_event, window, cx)
                        },
                    )
                },
            )
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_mouse_down(MouseButton::Left, |_, window, _| window.prevent_default())
                        .on_click(move |event, window, cx| {
                            cx.stop_propagation();
                            (on_click)(event, window, cx)
                        })
                },
            )
            .when_some(self.tooltip, |this, tooltip| {
                this.tooltip(move |window, cx| tooltip(window, cx))
            })
            .children(self.children)
    }
}

impl Component for ButtonLike {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn sort_name() -> &'static str {
        // ButtonLike should be at the bottom of the button list
        "ButtonZ"
    }

    fn description() -> Option<&'static str> {
        Some(ButtonLike::DOCS)
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group(vec![
                        single_example(
                            "Default",
                            ButtonLike::new("default")
                                .child(Label::new("Default"))
                                .into_any_element(),
                        ),
                        single_example(
                            "Filled",
                            ButtonLike::new("filled")
                                .style(ButtonStyle::Filled)
                                .child(Label::new("Filled"))
                                .into_any_element(),
                        ),
                        single_example(
                            "Subtle",
                            ButtonLike::new("outline")
                                .style(ButtonStyle::Subtle)
                                .child(Label::new("Subtle"))
                                .into_any_element(),
                        ),
                        single_example(
                            "Tinted",
                            ButtonLike::new("tinted_accent_style")
                                .style(ButtonStyle::Tinted(TintColor::Accent))
                                .child(Label::new("Accent"))
                                .into_any_element(),
                        ),
                        single_example(
                            "Transparent",
                            ButtonLike::new("transparent")
                                .style(ButtonStyle::Transparent)
                                .child(Label::new("Transparent"))
                                .into_any_element(),
                        ),
                    ]),
                    example_group_with_title(
                        "Button Group Constructors",
                        vec![
                            single_example(
                                "Left Rounded",
                                ButtonLike::new_rounded_left("left_rounded")
                                    .child(Label::new("Left Rounded"))
                                    .style(ButtonStyle::Filled)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Right Rounded",
                                ButtonLike::new_rounded_right("right_rounded")
                                    .child(Label::new("Right Rounded"))
                                    .style(ButtonStyle::Filled)
                                    .into_any_element(),
                            ),
                            single_example(
                                "Button Group",
                                h_flex()
                                    .gap_px()
                                    .child(
                                        ButtonLike::new_rounded_left("bg_left")
                                            .child(Label::new("Left"))
                                            .style(ButtonStyle::Filled),
                                    )
                                    .child(
                                        ButtonLike::new_rounded_right("bg_right")
                                            .child(Label::new("Right"))
                                            .style(ButtonStyle::Filled),
                                    )
                                    .into_any_element(),
                            ),
                        ],
                    ),
                ])
                .into_any_element(),
        )
    }
}
