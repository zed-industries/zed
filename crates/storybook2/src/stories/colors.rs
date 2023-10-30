use ui::prelude::*;

use crate::story::Story;

#[derive(Component)]
pub struct ColorsStory;

impl ColorsStory {
    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let color_scales = theme2::default_color_scales();

        Story::container(cx)
            .child(Story::title(cx, "Colors"))
            .child(
                div()
                    .id("colors")
                    .flex()
                    .flex_col()
                    .overflow_y_scroll()
                    .text_color(gpui2::white())
                    .children(color_scales.into_iter().map(|(name, scale)| {
                        div().child(name.to_string()).child(div().flex().children(
                            (1..=12).map(|step| div().flex().size_4().bg(scale.step(cx, step))),
                        ))
                    })),
            )
    }
}
