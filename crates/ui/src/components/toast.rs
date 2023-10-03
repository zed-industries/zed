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
            origin,
            children,
            payload,
        }
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let color = ThemeColor::new(cx);

        let mut div = div();

        if self.origin == ToastOrigin::Bottom {
            div = div.right_1_2();
        } else {
            div = div.right_4();
        }

        div.absolute()
            .bottom_4()
            .flex()
            .py_2()
            .px_1p5()
            .min_w_40()
            .rounded_md()
            .fill(color.elevated_surface)
            .max_w_64()
            .children_any((self.children)(cx, self.payload.as_ref()))
    }
}
