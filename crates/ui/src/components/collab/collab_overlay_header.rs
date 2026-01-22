use crate::{Tooltip, prelude::*};
use gpui::{AnyElement, ClickEvent, IntoElement, ParentElement, Styled};

#[derive(IntoElement, RegisterComponent)]
pub struct CollabOverlayHeader {
    channel_name: SharedString,
    is_open: bool,
    on_toggle: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_channel_notes: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl CollabOverlayHeader {
    pub fn new(channel_name: impl Into<SharedString>) -> Self {
        Self {
            channel_name: channel_name.into(),
            is_open: false,
            on_toggle: None,
            on_channel_notes: None,
        }
    }

    pub fn is_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn on_toggle(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }

    pub fn on_channel_notes(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_channel_notes = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for CollabOverlayHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let chevron = if self.is_open {
            IconName::ChevronDown
        } else {
            IconName::ChevronUp
        };

        h_flex()
            .id("collab-overlay-header")
            .py_1()
            .px_2()
            .w_full()
            .gap_2()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .bg(cx.theme().colors().surface_background)
            .cursor_pointer()
            .hover(|style| style.bg(cx.theme().colors().element_hover))
            .tooltip(Tooltip::text("Open Channel Notes"))
            .when_some(self.on_channel_notes, |this, handler| {
                this.on_click(handler)
            })
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Icon::new(IconName::FileDoc)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(Label::new(self.channel_name)),
            )
            .child(
                IconButton::new("collapse-toggle", chevron)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Muted)
                    .tooltip(Tooltip::text(if self.is_open {
                        "Collapse"
                    } else {
                        "Expand"
                    }))
                    .when_some(self.on_toggle, |this, handler| {
                        this.on_click(move |event, window, cx| {
                            cx.stop_propagation();
                            handler(event, window, cx);
                        })
                    }),
            )
    }
}

impl Component for CollabOverlayHeader {
    fn scope() -> ComponentScope {
        ComponentScope::Collaboration
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let ex_container = h_flex()
            .w_80()
            .border_1()
            .border_color(cx.theme().colors().border);

        let examples = vec![single_example(
            "Default",
            ex_container
                .child(CollabOverlayHeader::new("Admin Dashboard v2").is_open(true))
                .into_any_element(),
        )];

        Some(example_group(examples).vertical().into_any_element())
    }
}
