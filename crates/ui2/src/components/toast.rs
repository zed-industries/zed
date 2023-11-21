use crate::prelude::*;
use gpui::{prelude::*, AnyElement, RenderOnce};
use gpui::{Div, Element};
use smallvec::SmallVec;

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
#[derive(RenderOnce)]
pub struct Toast {
    origin: ToastOrigin,
    children: SmallVec<[AnyElement; 2]>,
}

impl Component for Toast {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
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
            .bg(cx.theme().colors().elevated_surface_background)
            .children(self.children)
    }
}

impl Toast {
    pub fn new(origin: ToastOrigin) -> Self {
        Self {
            origin,
            children: SmallVec::new(),
        }
    }

    fn render(self, cx: &mut WindowContext) -> impl Element {
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
            .bg(cx.theme().colors().elevated_surface_background)
            .children(self.children)
    }
}

impl ParentElement for Toast {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use gpui::{Div, Render};

    use crate::{Label, Story};

    use super::*;

    pub struct ToastStory;

    impl Render for ToastStory {
        type Element = Div;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<Toast>(cx))
                .child(Story::label(cx, "Default"))
                .child(Toast::new(ToastOrigin::Bottom).child(Label::new("label")))
        }
    }
}
