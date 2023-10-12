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
pub struct Toast<S: 'static + Send + Sync> {
    origin: ToastOrigin,
    children: HackyChildren<S>,
    payload: HackyChildrenPayload,
}

impl<S: 'static + Send + Sync> Toast<S> {
    pub fn new(
        origin: ToastOrigin,
        children: HackyChildren<S>,
        payload: HackyChildrenPayload,
    ) -> Self {
        Self {
            origin,
            children,
            payload,
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new(cx);

        let mut div = div();

        if self.origin == ToastOrigin::Bottom {
            div = div.right_1_2();
        } else {
            div = div.right_4();
        }

        div.z_index(5)
            .absolute()
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

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use std::marker::PhantomData;

    use crate::{Label, Story};

    use super::*;

    #[derive(Element)]
    pub struct ToastStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> ToastStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
            Story::container(cx)
                .child(Story::title_for::<_, Toast<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(Toast::new(
                    ToastOrigin::Bottom,
                    |_, _| vec![Label::new("label").into_any()],
                    Box::new(()),
                ))
        }
    }
}
