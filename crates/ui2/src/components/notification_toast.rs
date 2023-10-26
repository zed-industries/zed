use std::marker::PhantomData;

use gpui2::rems;

use crate::{h_stack, prelude::*, Icon};

#[derive(IntoAnyElement)]
pub struct NotificationToast<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    label: SharedString,
    icon: Option<Icon>,
}

impl<S: 'static + Send + Sync + Clone> NotificationToast<S> {
    pub fn new(label: SharedString) -> Self {
        Self {
            state_type: PhantomData,
            label,
            icon: None,
        }
    }

    pub fn icon<I>(mut self, icon: I) -> Self
    where
        I: Into<Option<Icon>>,
    {
        self.icon = icon.into();
        self
    }

    fn render(self, _view: &mut S, cx: &mut ViewContext<S>) -> impl IntoAnyElement<S> {
        let theme = theme(cx);

        h_stack()
            .z_index(5)
            .absolute()
            .top_1()
            .right_1()
            .w(rems(9999.))
            .max_w_56()
            .py_1()
            .px_1p5()
            .rounded_lg()
            .shadow_md()
            .bg(theme.elevated_surface)
            .child(div().size_full().child(self.label.clone()))
    }
}
