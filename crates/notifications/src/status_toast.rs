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
        self.icon = Some(icon.into());
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
                    Button::new(action.id.clone(), action.label.clone()).when_some(
                        action.on_click.clone(),
                        |el, handler| {
                            el.on_click(move |click_event, window, cx| {
                                handler(click_event, window, cx)
                            })
                        },
                    ),
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
    fn preview(_window: &mut Window, cx: &mut App) -> AnyElement {
        div()
            // let text_example = cx.new(|cx| StatusToast::new("simple-toast", "Operation completed", cx));
            // let action_example =
            //     cx.new(|cx| StatusToast::new("action-toast", "Update ready", cx).action("Restart"));
            // let icon_example = cx.new(|cx| {
            //     StatusToast::with_icon("icon-toast", IconName::Check, "Successfully saved", cx)
            // });
            // let success_example = cx.new(|cx| {
            //     StatusToast::with_icon(
            //         "success-toast",
            //         IconName::Check,
            //         "Pushed 4 changes to `zed/main`",
            //         cx,
            //     )
            // });
            // let error_example = cx.new(|cx| {
            //     StatusToast::with_icon(
            //         "error-toast",
            //         IconName::XCircle,
            //         "git push: Couldn't find remote origin `iamnbutler/zed`",
            //         cx,
            //     )
            //     .action("More Info")
            // });
            // let warning_example = cx.new(|cx| {
            //     StatusToast::with_icon(
            //         "warning-toast",
            //         IconName::Warning,
            //         "Your changes are not saved",
            //         cx,
            //     )
            // });
            // let info_example = cx.new(|cx| {
            //     StatusToast::with_icon("info-toast", IconName::Info, "New update available", cx)
            // });
            // let pr_example = cx.new(|cx| {
            //     StatusToast::with_icon(
            //         "success-toast-pr",
            //         IconName::GitBranchSmall,
            //         "`zed/new-notification-system` created!",
            //         cx,
            //     )
            //     .action("Open Pull Request")
            // });
            // v_flex()
            //     .gap_6()
            //     .p_4()
            //     .children(vec![
            //         example_group_with_title(
            //             "Basic Toast",
            //             vec![
            //                 single_example("Text", div().child(text_example).into_any_element()),
            //                 single_example("Action", div().child(action_example).into_any_element()),
            //                 single_example("Icon", div().child(icon_example).into_any_element()),
            //             ],
            //         ),
            //         example_group_with_title(
            //             "Examples",
            //             vec![
            //                 single_example("Success", div().child(success_example).into_any_element()),
            //                 single_example("Error", div().child(error_example).into_any_element()),
            //                 single_example("Warning", div().child(warning_example).into_any_element()),
            //                 single_example("Info", div().child(info_example).into_any_element()),
            //                 single_example("Create PR", div().child(pr_example).into_any_element()),
            //             ],
            //         )
            //         .vertical(),
            //     ])
            .into_any_element()
    }
}
