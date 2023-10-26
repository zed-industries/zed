use gpui2::AnyElement;
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum ToastOrigin {
    #[default]
    Bottom,
    BottomRight,
}

/// Don't use toast directly:
///
/// - For messages with a required action, use a `NotificationToast`.
/// - For messages that convey information, use a `StatusToast`.
///
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
    children: SmallVec<[AnyElement<S>; 2]>,
}

impl<S: 'static + Send + Sync> Toast<S> {
    pub fn new(origin: ToastOrigin) -> Self {
        Self {
            origin,
            children: SmallVec::new(),
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl IntoAnyElement<S> {
        let theme = theme(cx);

        let mut div = div();

        if self.origin == ToastOrigin::Bottom {
            div = div.right_1_2();
        } else {
            div = div.right_2();
        }

        div.z_index(5)
            .absolute()
            .bottom_9()
            .flex()
            .py_1()
            .px_1p5()
            .rounded_lg()
            .shadow_md()
            .overflow_hidden()
            .bg(theme.elevated_surface)
            .children(self.children.drain(..))
    }
}

impl<S: 'static + Send + Sync> ParentElement<S> for Toast<S> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<S>; 2]> {
        &mut self.children
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
    pub struct ToastStory<S: 'static + Send + Sync> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync> ToastStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl IntoAnyElement<S> {
            Story::container(cx)
                .child(Story::title_for::<_, Toast<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(Toast::new(ToastOrigin::Bottom).child(Label::new("label")))
        }
    }
}
