use crate::prelude::*;
use crate::{v_stack, Buffer, Icon, IconButton, Label};

#[derive(Component)]
pub struct MultiBuffer {
    buffers: Vec<Buffer>,
}

impl MultiBuffer {
    pub fn new(buffers: Vec<Buffer>) -> Self {
        Self { buffers }
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let theme = theme(cx);

        v_stack()
            .w_full()
            .h_full()
            .flex_1()
            .children(self.buffers.clone().into_iter().map(|buffer| {
                v_stack()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .p_4()
                            .bg(theme.editor_subheader)
                            .child(Label::new("main.rs"))
                            .child(IconButton::new("arrow_up_right", Icon::ArrowUpRight)),
                    )
                    .child(buffer)
            }))
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::{hello_world_rust_buffer_example, Story};

    use super::*;

    #[derive(Component)]
    pub struct MultiBufferStory;

    impl MultiBufferStory {
        fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
            let theme = theme(cx);

            Story::container(cx)
                .child(Story::title_for::<_, MultiBuffer>(cx))
                .child(Story::label(cx, "Default"))
                .child(MultiBuffer::new(vec![
                    hello_world_rust_buffer_example(&theme),
                    hello_world_rust_buffer_example(&theme),
                    hello_world_rust_buffer_example(&theme),
                    hello_world_rust_buffer_example(&theme),
                    hello_world_rust_buffer_example(&theme),
                ]))
        }
    }
}
