use std::marker::PhantomData;

use gpui3::{Element, ParentElement, StyleHelpers, ViewContext};

use crate::{
    h_stack, v_stack, Button, Icon, IconButton, IconElement, Label, ThemeColor, Toast, ToastOrigin,
};

/// Notification toasts are used to display a message
/// that requires them to take action.
///
/// You must provide a primary action for the user to take.
///
/// To simply convey information, use a `StatusToast`.
#[derive(Element)]
pub struct NotificationToast<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    left_icon: Option<Icon>,
    title: String,
    message: String,
    primary_action: Option<Button<S>>,
    secondary_action: Option<Button<S>>,
}

impl<S: 'static + Send + Sync + Clone> NotificationToast<S> {
    pub fn new(
        title: impl Into<String>,
        message: impl Into<String>,
        primary_action: Button<S>,
    ) -> Self {
        Self {
            state_type: PhantomData,
            left_icon: None,
            title: title.into(),
            message: message.into(),
            primary_action: Some(primary_action),
            secondary_action: None,
        }
    }

    pub fn left_icon(mut self, icon: Icon) -> Self {
        self.left_icon = Some(icon);
        self
    }

    pub fn secondary_action(mut self, action: Button<S>) -> Self {
        self.secondary_action = Some(action);
        self
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new(cx);

        let notification = h_stack()
            .min_w_64()
            .max_w_96()
            .gap_1()
            .items_start()
            .p_1()
            .children(self.left_icon.map(|i| IconElement::new(i)))
            .child(
                v_stack()
                    .flex_1()
                    .w_full()
                    .gap_1()
                    .child(
                        h_stack()
                            .justify_between()
                            .child(Label::new(self.title.clone()))
                            .child(IconButton::new(Icon::Close).color(crate::IconColor::Muted)),
                    )
                    .child(
                        v_stack()
                            .overflow_hidden_x()
                            .gap_1()
                            .child(Label::new(self.message.clone()))
                            .child(
                                h_stack()
                                    .gap_1()
                                    .justify_end()
                                    .children(self.secondary_action.take())
                                    .children(self.primary_action.take()),
                            ),
                    ),
            );

        Toast::new(ToastOrigin::BottomRight).child(notification)
    }
}
