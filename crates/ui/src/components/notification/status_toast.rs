use std::time::Duration;

use crate::{prelude::*, AnyIcon};
use gpui::{percentage, Animation, AnimationExt, IntoElement, Transformation};

#[derive(IntoElement, IntoComponent)]
#[component(scope = "notification")]
pub struct StatusToast {
    id: ElementId,
    // children: SmallVec<[AnyElement; 2]>,
    icon: Option<AnyIcon>,
    label: SharedString,
    // action
}

impl StatusToast {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            icon: None,
            label: label.into(),
        }
    }
    pub fn with_icon(
        id: impl Into<ElementId>,
        icon: Option<AnyIcon>,
        label: impl Into<SharedString>,
    ) -> Self {
        let icon = icon.into();

        Self {
            id: id.into(),
            icon,
            label: label.into(),
        }
    }
}

impl RenderOnce for StatusToast {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .id(self.id)
            .elevation_3(cx)
            .gap_2()
            .py_2()
            .px_3()
            .rounded_md()
            .bg(cx.theme().colors().surface_background)
            .shadow_lg()
            .items_center()
            .children(self.icon)
            .child(Label::new(self.label).color(Color::Default))
    }
}

impl ComponentPreview for StatusToast {
    fn preview(_window: &mut Window, cx: &mut App) -> AnyElement {
        v_flex()
            .gap_6()
            .p_4()
            .children(vec![
                example_group_with_title(
                    "Basic Toast",
                    vec![single_example(
                        "Text Only",
                        StatusToast::new("simple-toast", "Operation completed").into_any_element(),
                    )],
                ),
                example_group_with_title(
                    "With Icons",
                    vec![
                        single_example(
                            "Success",
                            StatusToast::with_icon(
                                "success-toast",
                                Some(Icon::new(IconName::Check).color(Color::Success).into()),
                                "Successfully saved",
                            )
                            .into_any_element(),
                        ),
                        single_example(
                            "Error",
                            StatusToast::with_icon(
                                "error-toast",
                                Some(Icon::new(IconName::XCircle).color(Color::Error).into()),
                                "Failed to connect",
                            )
                            .into_any_element(),
                        ),
                        single_example(
                            "Warning",
                            StatusToast::with_icon(
                                "warning-toast",
                                Some(Icon::new(IconName::Warning).color(Color::Warning).into()),
                                "Your changes are not saved",
                            )
                            .into_any_element(),
                        ),
                        single_example(
                            "Info",
                            StatusToast::with_icon(
                                "info-toast",
                                Some(Icon::new(IconName::Info).color(Color::Info).into()),
                                "New update available",
                            )
                            .into_any_element(),
                        ),
                    ],
                ),
                example_group_with_title(
                    "Special States",
                    vec![
                        single_example(
                            "Loading",
                            StatusToast::with_icon(
                                "loading-toast",
                                Some(
                                    Icon::new(IconName::ArrowCircle)
                                        .with_animation(
                                            "arrow-circle",
                                            Animation::new(Duration::from_secs(4)).repeat(),
                                            |icon, delta| {
                                                icon.transform(Transformation::rotate(percentage(
                                                    delta,
                                                )))
                                            },
                                        )
                                        .into(),
                                ),
                                "Finding References…",
                            )
                            .into_any_element(),
                        ),
                        single_example(
                            "Download",
                            StatusToast::with_icon(
                                "download-toast",
                                Some(Icon::new(IconName::Download).color(Color::Default).into()),
                                "Download complete",
                            )
                            .into_any_element(),
                        ),
                    ],
                ),
            ])
            .into_any_element()
    }
}
