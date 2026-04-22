use crate::prelude::*;
use gpui::{ClickEvent, SharedString};

#[derive(IntoElement, RegisterComponent)]
pub struct AgentSetupButton {
    id: ElementId,
    icon: Option<Icon>,
    name: Option<SharedString>,
    state: Option<AnyElement>,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl AgentSetupButton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            icon: None,
            name: None,
            state: None,
            disabled: false,
            on_click: None,
        }
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn state(mut self, element: impl IntoElement) -> Self {
        self.state = Some(element.into_any_element());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Component for AgentSetupButton {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        None
    }
}

impl RenderOnce for AgentSetupButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_clickable = !self.disabled && self.on_click.is_some();

        let has_top_section = self.icon.is_some() || self.name.is_some();
        let top_section = has_top_section.then(|| {
            h_flex()
                .p_1p5()
                .gap_1()
                .justify_center()
                .when_some(self.icon, |this, icon| this.child(icon))
                .when_some(self.name, |this, name| {
                    this.child(Label::new(name).size(LabelSize::Small))
                })
        });

        let bottom_section = self.state.map(|state_element| {
            h_flex()
                .p_0p5()
                .h_full()
                .justify_center()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .bg(cx.theme().colors().element_background.opacity(0.5))
                .child(state_element)
        });

        v_flex()
            .id(self.id)
            .border_1()
            .border_color(cx.theme().colors().border_variant)
            .rounded_sm()
            .when(is_clickable, |this| {
                this.cursor_pointer().hover(|style| {
                    style
                        .bg(cx.theme().colors().element_hover)
                        .border_color(cx.theme().colors().border)
                })
            })
            .when_some(top_section, |this, section| this.child(section))
            .when_some(bottom_section, |this, section| this.child(section))
            .when_some(self.on_click.filter(|_| is_clickable), |this, on_click| {
                this.on_click(on_click)
            })
    }
}
