use gpui2::rems;

use crate::{h_stack, prelude::*, Icon};

#[derive(Component)]
pub struct NotificationToast {
    label: SharedString,
    icon: Option<Icon>,
}

impl NotificationToast {
    pub fn new(label: SharedString) -> Self {
        Self { label, icon: None }
    }

    pub fn icon<I>(mut self, icon: I) -> Self
    where
        I: Into<Option<Icon>>,
    {
        self.icon = icon.into();
        self
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
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
