use std::sync::Arc;

use gpui::{AnyView, ClickEvent, SharedString};

use crate::IconButtonShape;
use crate::prelude::*;

#[derive(IntoElement, RegisterComponent)]
pub struct Disclosure {
    id: ElementId,
    is_open: bool,
    selected: bool,
    disabled: bool,
    closed_icon: IconName,
    shape: Option<IconButtonShape>,
    visible_on_hover: Option<SharedString>,
    tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>>,
    on_toggle_expanded: Option<Arc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    opened_icon: IconName,
}

impl Disclosure {
    pub fn new(id: impl Into<ElementId>, is_open: bool) -> Self {
        Self {
            id: id.into(),
            is_open,
            selected: false,
            disabled: false,
            on_toggle_expanded: None,
            opened_icon: IconName::ChevronDown,
            closed_icon: IconName::ChevronRight,
            shape: None,
            visible_on_hover: None,
            tooltip: None,
        }
    }

    pub fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }

    pub fn on_toggle_expanded(
        mut self,
        handler: impl Into<Option<Arc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>>,
    ) -> Self {
        self.on_toggle_expanded = handler.into();
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

    /// Sets the shape of the underlying [`IconButton`].
    pub fn shape(mut self, shape: IconButtonShape) -> Self {
        self.shape = Some(shape);
        self
    }

    /// Alias for [`Self::on_toggle_expanded`].
    pub fn on_click(self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_toggle_expanded(Arc::new(handler) as Arc<_>)
    }
}

impl Toggleable for Disclosure {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl VisibleOnHover for Disclosure {
    fn visible_on_hover(mut self, group_name: impl Into<SharedString>) -> Self {
        self.visible_on_hover = Some(group_name.into());
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
        .icon_color(Color::Muted)
        .icon_size(IconSize::Small)
        .aria_label(if self.is_open { "Collapse" } else { "Expand" })
        .aria_expanded(self.is_open)
        .disabled(self.disabled)
        .when_some(self.shape, |this, shape| this.shape(shape))
        .toggle_state(self.selected)
        .when_some(self.visible_on_hover.clone(), |this, group_name| {
            this.visible_on_hover(group_name)
        })
        .when_some(self.tooltip, |this, tooltip| this.tooltip(tooltip))
        .when_some(self.on_toggle_expanded, move |this, on_toggle| {
            this.on_click(move |event, window, cx| on_toggle(event, window, cx))
        })
    }
}

impl Component for Disclosure {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn description() -> &'static str {
        "An interactive element used to show or hide content, \
            typically used in expandable sections or tree-like structures."
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> AnyElement {
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
                        single_example("Open", Disclosure::new("open", true).into_any_element()),
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
            .into_any_element()
    }
}
