use crate::{ListBulletItem, Vector, VectorName, prelude::*};
use component::{Component, ComponentScope, example_group, single_example};
use gpui::{
    AnyElement, ClickEvent, IntoElement, ParentElement, SharedString, linear_color_stop,
    linear_gradient,
};
use smallvec::SmallVec;

#[derive(IntoElement, RegisterComponent)]
pub struct AnnouncementToast {
    illustration: Option<AnyElement>,
    heading: Option<SharedString>,
    description: Option<SharedString>,
    bullet_items: SmallVec<[AnyElement; 6]>,
    primary_action_label: SharedString,
    primary_on_click: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    secondary_action_label: SharedString,
    secondary_on_click: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    dismiss_on_click: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
}

impl AnnouncementToast {
    pub fn new() -> Self {
        Self {
            illustration: None,
            heading: None,
            description: None,
            bullet_items: SmallVec::new(),
            primary_action_label: "Learn More".into(),
            primary_on_click: Box::new(|_, _, _| {}),
            secondary_action_label: "View Release Notes".into(),
            secondary_on_click: Box::new(|_, _, _| {}),
            dismiss_on_click: Box::new(|_, _, _| {}),
        }
    }

    pub fn illustration(mut self, illustration: impl IntoElement) -> Self {
        self.illustration = Some(illustration.into_any_element());
        self
    }

    pub fn heading(mut self, heading: impl Into<SharedString>) -> Self {
        self.heading = Some(heading.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn bullet_item(mut self, item: impl IntoElement) -> Self {
        self.bullet_items.push(item.into_any_element());
        self
    }

    pub fn bullet_items(mut self, items: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.bullet_items
            .extend(items.into_iter().map(IntoElement::into_any_element));
        self
    }

    pub fn primary_action_label(mut self, primary_action_label: impl Into<SharedString>) -> Self {
        self.primary_action_label = primary_action_label.into();
        self
    }

    pub fn primary_on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.primary_on_click = Box::new(handler);
        self
    }

    pub fn secondary_action_label(
        mut self,
        secondary_action_label: impl Into<SharedString>,
    ) -> Self {
        self.secondary_action_label = secondary_action_label.into();
        self
    }

    pub fn secondary_on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.secondary_on_click = Box::new(handler);
        self
    }

    pub fn dismiss_on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.dismiss_on_click = Box::new(handler);
        self
    }
}

impl RenderOnce for AnnouncementToast {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let has_illustration = self.illustration.is_some();
        let illustration = self.illustration;

        v_flex()
            .relative()
            .w_full()
            .elevation_3(cx)
            .when_some(illustration, |this, i| this.child(i))
            .child(
                v_flex()
                    .p_4()
                    .gap_4()
                    .when(has_illustration, |s| {
                        s.border_t_1()
                            .border_color(cx.theme().colors().border_variant)
                    })
                    .child(
                        v_flex()
                            .min_w_0()
                            .when_some(self.heading, |this, heading| {
                                this.child(Headline::new(heading).size(HeadlineSize::Small))
                            })
                            .when_some(self.description, |this, description| {
                                this.child(Label::new(description).color(Color::Muted))
                            }),
                    )
                    .when(!self.bullet_items.is_empty(), |this| {
                        this.child(v_flex().min_w_0().gap_1().children(self.bullet_items))
                    })
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                Button::new("try-now", self.primary_action_label)
                                    .style(ButtonStyle::Outlined)
                                    .full_width()
                                    .on_click(self.primary_on_click),
                            )
                            .child(
                                Button::new("release-notes", self.secondary_action_label)
                                    .full_width()
                                    .on_click(self.secondary_on_click),
                            ),
                    ),
            )
            .child(
                div().absolute().top_1().right_1().child(
                    IconButton::new("dismiss", IconName::Close)
                        .icon_size(IconSize::Small)
                        .on_click(self.dismiss_on_click),
                ),
            )
    }
}

impl Component for AnnouncementToast {
    fn scope() -> ComponentScope {
        ComponentScope::Notification
    }

    fn description() -> Option<&'static str> {
        Some("A special toast for announcing new and exciting features.")
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let illustration = h_flex()
            .relative()
            .h(rems_from_px(126.))
            .bg(cx.theme().colors().editor_background)
            .justify_center()
            .gap_8()
            .rounded_t_md()
            .overflow_hidden()
            .child(
                div().absolute().inset_0().w(px(515.)).h(px(126.)).child(
                    Vector::new(VectorName::AcpGrid, rems_from_px(515.), rems_from_px(126.))
                        .color(Color::Custom(cx.theme().colors().text.opacity(0.02))),
                ),
            )
            .child(div().absolute().inset_0().size_full().bg(linear_gradient(
                0.,
                linear_color_stop(
                    cx.theme().colors().elevated_surface_background.opacity(0.1),
                    0.9,
                ),
                linear_color_stop(
                    cx.theme().colors().elevated_surface_background.opacity(0.),
                    0.,
                ),
            )))
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .size_full()
                    .bg(gpui::black().opacity(0.15)),
            )
            .child(
                Vector::new(
                    VectorName::AcpLogoSerif,
                    rems_from_px(257.),
                    rems_from_px(47.),
                )
                .color(Color::Custom(cx.theme().colors().text.opacity(0.8))),
            );

        let examples = vec![single_example(
            "Basic",
            div().w_80().child(
                AnnouncementToast::new()
                    .illustration(illustration)
                    .heading("What's new in Zed")
                    .description(
                        "This version comes in with some changes to the workspace for a better experience.",
                    )
                    .bullet_item(ListBulletItem::new("Improved agent performance"))
                    .bullet_item(ListBulletItem::new("New agentic features"))
                    .bullet_item(ListBulletItem::new("Better agent capabilities"))

            )
            .into_any_element(),
        )];

        Some(
            v_flex()
                .gap_6()
                .child(example_group(examples).vertical())
                .into_any_element(),
        )
    }
}
