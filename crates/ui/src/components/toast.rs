use crate::prelude::*;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ToastOrigin {
    #[default]
    Bottom,
    BottomRight,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ToastVariant {
    #[default]
    Toast,
    Status,
}

/// A toast is a small, temporary window that appears to show a message to the user
/// or indicate a required action.
///
/// Toasts should not persist on the screen for more than a few seconds unless
/// they are actively showing the a process in progress.
///
/// Only one toast may be visible at a time.
#[derive(Element)]
pub struct Toast<V: 'static> {
    origin: ToastOrigin,
    children: HackyChildren<V>,
    payload: HackyChildrenPayload,
}

impl<V: 'static> Toast<V> {
    pub fn new(
        origin: ToastOrigin,
        children: HackyChildren<V>,
        payload: HackyChildrenPayload,
    ) -> Self {
        Self {
            origin: ToastOrigin::BottomRight,
            children,
            payload,
        }
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let color = ThemeColor::new(cx);
        let system_color = SystemColor::new();

        let mut div = div();

        if self.origin == ToastOrigin::Bottom {
            div = div.right_1_2().bottom_0();
        } else {
            div = div.right_0().bottom_0();
        }

        div.absolute()
            .p_2()
            .rounded_md()
            .fill(system_color.mac_os_traffic_light_red)
            .max_w_64()
            .children_any((self.children)(cx, self.payload.as_ref()))
    }
}
