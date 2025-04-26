use std::sync::Arc;

use gpui::{ClickEvent, CursorStyle};

use crate::{Color, IconButton, IconButtonShape, IconName, IconSize, prelude::*};

#[derive(IntoElement, RegisterComponent)]
pub struct Disclosure {
    id: ElementId,
    is_open: bool,
    selected: bool,
    disabled: bool,
    on_toggle: Option<Arc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    cursor_style: CursorStyle,
    opened_icon: IconName,
    closed_icon: IconName,
}

impl Disclosure {
    pub fn new(id: impl Into<ElementId>, is_open: bool) -> Self {
        Self {
            id: id.into(),
            is_open,
            selected: false,
            disabled: false,
            on_toggle: None,
            cursor_style: CursorStyle::PointingHand,
            opened_icon: IconName::ChevronDown,
            closed_icon: IconName::ChevronRight,
        }
    }

    pub fn on_toggle(
        mut self,
        handler: impl Into<Option<Arc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>>,
    ) -> Self {
        self.on_toggle = handler.into();
        self
    }

    pub fn opened_icon(mut self, icon: IconName) -> Self {
        self.opened_icon = icon;
        self
    }

    pub fn closed_icon(mut self, icon: IconName) -> Self {
        self.closed_icon = icon;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Toggleable for Disclosure {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Clickable for Disclosure {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Arc::new(handler));
        self
    }

    fn cursor_style(mut self, cursor_style: gpui::CursorStyle) -> Self {
        self.cursor_style = cursor_style;
        self
    }
}

impl RenderOnce for Disclosure {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        IconButton::new(
            self.id,
            match self.is_open {
                true => self.opened_icon,
                false => self.closed_icon,
            },
        )
        .shape(IconButtonShape::Square)
        .icon_color(Color::Muted)
        .icon_size(IconSize::Small)
        .disabled(self.disabled)
        .toggle_state(self.selected)
        .when_some(self.on_toggle, move |this, on_toggle| {
            this.on_click(move |event, window, cx| on_toggle(event, window, cx))
        })
    }
}

impl Component for Disclosure {
    type InitialState = ();
    fn scope() -> ComponentScope {
        ComponentScope::Navigation
    }

    fn description() -> Option<&'static str> {
        Some(
            "An interactive element used to show or hide content, typically used in expandable sections or tree-like structures.",
        )
    }

    fn initial_state(_cx: &mut App) -> Self::InitialState {
        ()
    }

    fn preview(_state: &mut (), _window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Disclosure States",
                        vec![
                            single_example(
                                "Closed",
                                Disclosure::new("closed", false).into_any_element(),
                            ),
                            single_example(
                                "Open",
                                Disclosure::new("open", true).into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "Interactive Example",
                        vec![single_example(
                            "Toggleable",
                            v_flex()
                                .gap_2()
                                .child(Disclosure::new("interactive", false).into_any_element())
                                .child(Label::new("Click to toggle"))
                                .into_any_element(),
                        )],
                    ),
                ])
                .into_any_element(),
        )
    }
}
