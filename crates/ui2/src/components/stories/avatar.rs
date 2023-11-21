#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::Story;
    use gpui::{Div, Render};

    pub struct AvatarStory;

    impl Render for AvatarStory {
        type Element = Div;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<Avatar>(cx))
                .child(Story::label(cx, "Default"))
                .child(Avatar::new(
                    "https://avatars.githubusercontent.com/u/1714999?v=4",
                ))
                .child(Avatar::new(
                    "https://avatars.githubusercontent.com/u/326587?v=4",
                ))
        }
    }
}
