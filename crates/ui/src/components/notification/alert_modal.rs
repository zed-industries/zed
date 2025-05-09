use crate::component_prelude::*;
use crate::prelude::*;
use gpui::IntoElement;
use smallvec::{SmallVec, smallvec};

#[derive(IntoElement, RegisterComponent)]
pub struct AlertModal {
    id: ElementId,
    children: SmallVec<[AnyElement; 2]>,
    title: SharedString,
    primary_action: SharedString,
    dismiss_label: SharedString,
}

impl AlertModal {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            children: smallvec![],
            title: title.into(),
            primary_action: "Ok".into(),
            dismiss_label: "Cancel".into(),
        }
    }

    pub fn primary_action(mut self, primary_action: impl Into<SharedString>) -> Self {
        self.primary_action = primary_action.into();
        self
    }

    pub fn dismiss_label(mut self, dismiss_label: impl Into<SharedString>) -> Self {
        self.dismiss_label = dismiss_label.into();
        self
    }
}

impl RenderOnce for AlertModal {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .id(self.id)
            .elevation_3(cx)
            .w(px(440.))
            .p_5()
            .child(
                v_flex()
                    .text_ui()
                    .text_color(Color::Muted.color(cx))
                    .gap_1()
                    .child(Headline::new(self.title).size(HeadlineSize::Small))
                    .children(self.children),
            )
            .child(
                h_flex()
                    .h(rems(1.75))
                    .items_center()
                    .child(div().flex_1())
                    .child(
                        h_flex()
                            .items_center()
                            .gap_1()
                            .child(
                                Button::new(self.dismiss_label.clone(), self.dismiss_label.clone())
                                    .color(Color::Muted),
                            )
                            .child(Button::new(
                                self.primary_action.clone(),
                                self.primary_action.clone(),
                            )),
                    ),
            )
    }
}

impl ParentElement for AlertModal {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl Component for AlertModal {
    fn scope() -> ComponentScope {
        ComponentScope::Notification
    }

    fn status() -> ComponentStatus {
        ComponentStatus::WorkInProgress
    }

    fn description() -> Option<&'static str> {
        Some("A modal dialog that presents an alert message with primary and dismiss actions.")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .p_4()
                .children(vec![example_group(
                    vec![
                        single_example(
                            "Basic Alert",
                            AlertModal::new("simple-modal", "Do you want to leave the current call?")
                                .child("The current window will be closed, and connections to any shared projects will be terminated."
                                )
                                .primary_action("Leave Call")
                                .into_any_element(),
                        )
                    ],
                )])
                .into_any_element()
        )
    }
}
