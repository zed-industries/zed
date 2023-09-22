use gpui2::elements::div;
use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use ui::{avatar, theme};

#[derive(Element, Default)]
pub struct AvatarStory {}

impl AvatarStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        div().size_full().flex().child(avatar(
            "https://avatars.githubusercontent.com/u/1714999?v=4",
        ))
    }
}
