use crate::{prelude::*, v_stack, ButtonGroup};

#[derive(Component)]
pub struct Details<V: 'static> {
    text: &'static str,
    meta: Option<&'static str>,
    actions: Option<ButtonGroup<V>>,
}

impl<V: 'static> Details<V> {
    pub fn new(text: &'static str) -> Self {
        Self {
            text,
            meta: None,
            actions: None,
        }
    }

    pub fn meta_text(mut self, meta: &'static str) -> Self {
        self.meta = Some(meta);
        self
    }

    pub fn actions(mut self, actions: ButtonGroup<V>) -> Self {
        self.actions = Some(actions);
        self
    }

    fn render(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let theme = old_theme(cx);

        v_stack()
            .p_1()
            .gap_0p5()
            .text_xs()
            .text_color(theme.text)
            .size_full()
            .child(self.text)
            .children(self.meta.map(|m| m))
            .children(self.actions.map(|a| a))
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::{Button, Story};
    use gpui2::{Div, Render};

    pub struct DetailsStory;

    impl Render for DetailsStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, Details<Self>>(cx))
                .child(Story::label(cx, "Default"))
                .child(Details::new("The quick brown fox jumps over the lazy dog"))
                .child(Story::label(cx, "With meta"))
                .child(
                    Details::new("The quick brown fox jumps over the lazy dog")
                        .meta_text("Sphinx of black quartz, judge my vow."),
                )
                .child(Story::label(cx, "With meta and actions"))
                .child(
                    Details::new("The quick brown fox jumps over the lazy dog")
                        .meta_text("Sphinx of black quartz, judge my vow.")
                        .actions(ButtonGroup::new(vec![
                            Button::new("Decline"),
                            Button::new("Accept").variant(crate::ButtonVariant::Filled),
                        ])),
                )
        }
    }
}
