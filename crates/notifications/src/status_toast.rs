use std::sync::Arc;

use gpui::{ClickEvent, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, IntoElement};
use ui::prelude::*;
use workspace::ToastView;

#[derive(Clone)]
pub struct ToastAction {
    id: ElementId,
    label: SharedString,
    on_click: Option<Arc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

#[derive(Clone, Copy)]
pub struct ToastIcon {
    icon: IconName,
    color: Color,
}

impl ToastIcon {
    pub fn new(icon: IconName) -> Self {
        Self {
            icon,
            color: Color::default(),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl From<IconName> for ToastIcon {
    fn from(icon: IconName) -> Self {
        Self {
            icon,
            color: Color::default(),
        }
    }
}

impl ToastAction {
    pub fn new(
        label: SharedString,
        on_click: Option<Arc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    ) -> Self {
        let id = ElementId::Name(label.clone());

        Self {
            id,
            label,
            on_click,
        }
    }
}

#[derive(IntoComponent)]
#[component(scope = "Notification")]
pub struct StatusToast {
    icon: Option<ToastIcon>,
    text: SharedString,
    action: Option<ToastAction>,
    focus_handle: FocusHandle,
}

impl StatusToast {
    pub fn new(
        text: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut App,
        f: impl FnOnce(Self, &mut Window, &mut Context<Self>) -> Self,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let focus_handle = cx.focus_handle();

            window.refresh();
            f(
                Self {
                    text: text.into(),
                    icon: None,
                    action: None,
                    focus_handle,
                },
                window,
                cx,
            )
        })
    }

    pub fn icon(mut self, icon: ToastIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn action(
        mut self,
        label: impl Into<SharedString>,
        f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.action = Some(ToastAction::new(label.into(), Some(Arc::new(f))));
        self
    }
}

impl Render for StatusToast {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .id("status-toast")
            .elevation_3(cx)
            .gap_2()
            .py_1p5()
            .px_2p5()
            .flex_none()
            .bg(cx.theme().colors().surface_background)
            .shadow_lg()
            .items_center()
            .when_some(self.icon.as_ref(), |this, icon| {
                this.child(Icon::new(icon.icon).color(icon.color))
            })
            .child(Label::new(self.text.clone()).color(Color::Default))
            .when_some(self.action.as_ref(), |this, action| {
                this.child(
                    Button::new(action.id.clone(), action.label.clone())
                        .color(Color::Muted)
                        .when_some(action.on_click.clone(), |el, handler| {
                            el.on_click(move |click_event, window, cx| {
                                handler(click_event, window, cx)
                            })
                        }),
                )
            })
    }
}

impl ToastView for StatusToast {}

impl Focusable for StatusToast {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for StatusToast {}

impl ComponentPreview for StatusToast {
    fn preview(window: &mut Window, cx: &mut App) -> AnyElement {
        let text_example = StatusToast::new("Operation completed", window, cx, |this, _, _| this);

        let action_example =
            StatusToast::new("Update ready to install", window, cx, |this, _, cx| {
                this.action("Restart", cx.listener(|_, _, _, _| {}))
            });

        let icon_example = StatusToast::new(
            "Nathan Sobo accepted your contact request",
            window,
            cx,
            |this, _, _| this.icon(ToastIcon::new(IconName::Check).color(Color::Muted)),
        );

        let success_example = StatusToast::new(
            "Pushed 4 changes to `zed/main`",
            window,
            cx,
            |this, _, _| this.icon(ToastIcon::new(IconName::Check).color(Color::Success)),
        );

        let error_example = StatusToast::new(
            "git push: Couldn't find remote origin `iamnbutler/zed`",
            window,
            cx,
            |this, _, cx| {
                this.icon(ToastIcon::new(IconName::XCircle).color(Color::Error))
                    .action("More Info", cx.listener(|_, _, _, _| {}))
            },
        );

        let warning_example =
            StatusToast::new("You have outdated settings", window, cx, |this, _, cx| {
                this.icon(ToastIcon::new(IconName::Warning).color(Color::Warning))
                    .action("More Info", cx.listener(|_, _, _, _| {}))
            });

        let pr_example = StatusToast::new(
            "`zed/new-notification-system` created!",
            window,
            cx,
            |this, _, cx| {
                this.icon(ToastIcon::new(IconName::GitBranchSmall).color(Color::Muted))
                    .action(
                        "Open Pull Request",
                        cx.listener(|_, _, _, cx| cx.open_url("https://github.com/")),
                    )
            },
        );

        v_flex()
            .gap_6()
            .p_4()
            .children(vec![
                example_group_with_title(
                    "Basic Toast",
                    vec![
                        single_example("Text", div().child(text_example).into_any_element()),
                        single_example("Action", div().child(action_example).into_any_element()),
                        single_example("Icon", div().child(icon_example).into_any_element()),
                    ],
                ),
                example_group_with_title(
                    "Examples",
                    vec![
                        single_example("Success", div().child(success_example).into_any_element()),
                        single_example("Error", div().child(error_example).into_any_element()),
                        single_example("Warning", div().child(warning_example).into_any_element()),
                        single_example("Create PR", div().child(pr_example).into_any_element()),
                    ],
                )
                .vertical(),
            ])
            .into_any_element()
    }
}
