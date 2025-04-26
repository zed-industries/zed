use std::rc::Rc;

use gpui::{DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, IntoElement};
use ui::{Tooltip, prelude::*};
use workspace::{ToastAction, ToastView};

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

#[derive(RegisterComponent)]
pub struct StatusToast {
    icon: Option<ToastIcon>,
    text: SharedString,
    action: Option<ToastAction>,
    this_handle: Entity<Self>,
    focus_handle: FocusHandle,
}

impl StatusToast {
    pub fn new(
        text: impl Into<SharedString>,
        cx: &mut App,
        f: impl FnOnce(Self, &mut Context<Self>) -> Self,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let focus_handle = cx.focus_handle();

            f(
                Self {
                    text: text.into(),
                    icon: None,
                    action: None,
                    this_handle: cx.entity(),
                    focus_handle,
                },
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
        f: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        let this_handle = self.this_handle.clone();
        self.action = Some(ToastAction::new(
            label.into(),
            Some(Rc::new(move |window, cx| {
                this_handle.update(cx, |_, cx| {
                    cx.emit(DismissEvent);
                });
                f(window, cx);
            })),
        ));
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
                        .tooltip(Tooltip::for_action_title(
                            action.label.clone(),
                            &workspace::RunAction,
                        ))
                        .color(Color::Muted)
                        .when_some(action.on_click.clone(), |el, handler| {
                            el.on_click(move |_click_event, window, cx| handler(window, cx))
                        }),
                )
            })
    }
}

impl ToastView for StatusToast {
    fn action(&self) -> Option<ToastAction> {
        self.action.clone()
    }
}

impl Focusable for StatusToast {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for StatusToast {}

impl Component for StatusToast {
    type InitialState = ();
    fn scope() -> ComponentScope {
        ComponentScope::Notification
    }

    fn initial_state(_cx: &mut App) -> Self::InitialState {
        ()
    }

    fn preview(_state: &mut (), _window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let text_example = StatusToast::new("Operation completed", cx, |this, _| this);

        let action_example = StatusToast::new("Update ready to install", cx, |this, _cx| {
            this.action("Restart", |_, _| {})
        });

        let icon_example = StatusToast::new(
            "Nathan Sobo accepted your contact request",
            cx,
            |this, _| this.icon(ToastIcon::new(IconName::Check).color(Color::Muted)),
        );

        let success_example = StatusToast::new("Pushed 4 changes to `zed/main`", cx, |this, _| {
            this.icon(ToastIcon::new(IconName::Check).color(Color::Success))
        });

        let error_example = StatusToast::new(
            "git push: Couldn't find remote origin `iamnbutler/zed`",
            cx,
            |this, _cx| {
                this.icon(ToastIcon::new(IconName::XCircle).color(Color::Error))
                    .action("More Info", |_, _| {})
            },
        );

        let warning_example = StatusToast::new("You have outdated settings", cx, |this, _cx| {
            this.icon(ToastIcon::new(IconName::Warning).color(Color::Warning))
                .action("More Info", |_, _| {})
        });

        let pr_example =
            StatusToast::new("`zed/new-notification-system` created!", cx, |this, _cx| {
                this.icon(ToastIcon::new(IconName::GitBranchSmall).color(Color::Muted))
                    .action("Open Pull Request", |_, cx| {
                        cx.open_url("https://github.com/")
                    })
            });

        Some(
            v_flex()
                .gap_6()
                .p_4()
                .children(vec![
                    example_group_with_title(
                        "Basic Toast",
                        vec![
                            single_example("Text", div().child(text_example).into_any_element()),
                            single_example(
                                "Action",
                                div().child(action_example).into_any_element(),
                            ),
                            single_example("Icon", div().child(icon_example).into_any_element()),
                        ],
                    ),
                    example_group_with_title(
                        "Examples",
                        vec![
                            single_example(
                                "Success",
                                div().child(success_example).into_any_element(),
                            ),
                            single_example("Error", div().child(error_example).into_any_element()),
                            single_example(
                                "Warning",
                                div().child(warning_example).into_any_element(),
                            ),
                            single_example("Create PR", div().child(pr_example).into_any_element()),
                        ],
                    )
                    .vertical(),
                ])
                .into_any_element(),
        )
    }
}
