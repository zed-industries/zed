use gpui::{DismissEvent, EventEmitter, FocusHandle, Focusable, IntoElement};
use ui::prelude::*;
use workspace::ToastView;

#[derive(IntoComponent)]
#[component(scope = "Notification")]
pub struct StatusToast {
    id: ElementId,
    // children: SmallVec<[AnyElement; 2]>,
    icon: Option<IconName>,
    label: SharedString,
    action: Option<SharedString>,
    focus_handle: FocusHandle,
}

impl StatusToast {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>, cx: &mut App) -> Self {
        let focus_handle = cx.focus_handle();

        Self {
            id: id.into(),
            icon: None,
            label: label.into(),
            action: None,
            focus_handle,
        }
    }
    pub fn with_icon(
        id: impl Into<ElementId>,
        icon: IconName,
        label: impl Into<SharedString>,
        cx: &mut App,
    ) -> Self {
        Self {
            id: id.into(),
            icon: Some(icon),
            label: label.into(),
            action: None,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn action(mut self, action: impl Into<SharedString>) -> Self {
        self.action = Some(action.into());
        self
    }
}

impl Render for StatusToast {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // let has_icon = &self.icon.is_some();

        h_flex()
            .id(self.id.clone())
            .elevation_3(cx)
            .gap_2()
            .py_1p5()
            .px_2p5()
            .flex_none()
            .bg(cx.theme().colors().surface_background)
            .shadow_lg()
            .items_center()
            .when_some(self.icon, |this, icon| this.child(Icon::new(icon.clone())))
            .child(Label::new(self.label.clone()).color(Color::Default))
            .when_some(self.action.clone(), |this, action| {
                this.child(Button::new(action.clone(), action).color(Color::Muted))
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
        let text_example = cx.new(|cx| StatusToast::new("simple-toast", "Operation completed", cx));
        let action_example =
            cx.new(|cx| StatusToast::new("action-toast", "Update ready", cx).action("Restart"));
        let icon_example = cx.new(|cx| {
            StatusToast::with_icon("icon-toast", IconName::Check, "Successfully saved", cx)
        });
        let success_example = cx.new(|cx| {
            StatusToast::with_icon(
                "success-toast",
                IconName::Check,
                "Pushed 4 changes to `zed/main`",
                cx,
            )
        });
        let error_example = cx.new(|cx| {
            StatusToast::with_icon(
                "error-toast",
                IconName::XCircle,
                "git push: Couldn't find remote origin `iamnbutler/zed`",
                cx,
            )
            .action("More Info")
        });
        let warning_example = cx.new(|cx| {
            StatusToast::with_icon(
                "warning-toast",
                IconName::Warning,
                "Your changes are not saved",
                cx,
            )
        });
        let info_example = cx.new(|cx| {
            StatusToast::with_icon("info-toast", IconName::Info, "New update available", cx)
        });
        let pr_example = cx.new(|cx| {
            StatusToast::with_icon(
                "success-toast-pr",
                IconName::GitBranchSmall,
                "`zed/new-notification-system` created!",
                cx,
            )
            .action("Open Pull Request")
        });

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
                        single_example("Info", div().child(info_example).into_any_element()),
                        single_example("Create PR", div().child(pr_example).into_any_element()),
                    ],
                )
                .vertical(),
            ])
            .into_any_element()
    }
}
