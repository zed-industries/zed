use gpui3::{Element, ParentElement, StyleHelpers, ViewContext};

use crate::{
    h_stack, v_stack, Button, Icon, IconButton, IconElement, Label, ThemeColor, Toast, ToastOrigin,
};

/// Notification toasts are used to display a message
/// that requires them to take action.
///
/// To simply convey information, use a Status.
pub struct NotificationToast<S: 'static + Send + Sync + Clone> {
    left_icon: Option<Icon>,
    title: String,
    message: String,
    actions: Vec<Button<S>>,
}

impl<S: 'static + Send + Sync + Clone> NotificationToast<S> {
    pub fn new(
        title: impl Into<String>,
        message: impl Into<String>,
        actions: Vec<Button<S>>,
    ) -> Self {
        Self {
            left_icon: None,
            title: title.into(),
            message: message.into(),
            actions,
        }
    }

    pub fn set_left_icon(mut self, icon: Icon) -> Self {
        self.left_icon = Some(icon);
        self
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new(cx);

        let notification = h_stack()
            .gap_1()
            .items_start()
            .children(self.left_icon.map(|i| IconElement::new(i)))
            .child(
                v_stack()
                    .child(
                        h_stack()
                            .justify_between()
                            .p_1()
                            .child(Label::new(self.title))
                            .child(IconButton::new(Icon::Close)),
                    )
                    .child(
                        h_stack()
                            .p_1()
                            .child(Label::new(self.message))
                            .children(self.actions.iter().map(|action| action)),
                    ),
            );

        Toast::new(ToastOrigin::BottomRight)
    }
}
