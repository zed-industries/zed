use strum::IntoEnumIterator;
use ui::h_stack;
use ui::prelude::*;
use ui::v_stack;
use ui::Tab;

use crate::story::Story;

#[derive(Element, Default)]
pub struct TabStory {}

impl TabStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let git_statuses = GitStatus::iter();
        let fs_statuses = FileSystemStatus::iter();

        Story::container(cx)
            .child(Story::title_for::<_, Tab>(cx))
            .child(
                h_stack().child(
                    v_stack()
                        .gap_2()
                        .child(Story::label(cx, "Default"))
                        .child(Tab::new()),
                ),
            )
            .child(
                h_stack().child(
                    v_stack().gap_2().child(Story::label(cx, "Current")).child(
                        h_stack()
                            .gap_4()
                            .child(Tab::new().title("Current".to_string()).current(true))
                            .child(Tab::new().title("Not Current".to_string()).current(false)),
                    ),
                ),
            )
            .child(
                h_stack().child(
                    v_stack()
                        .gap_2()
                        .child(Story::label(cx, "Titled"))
                        .child(Tab::new().title("label".to_string())),
                ),
            )
            .child(
                h_stack().child(
                    v_stack()
                        .gap_2()
                        .child(Story::label(cx, "With Icon"))
                        .child(
                            Tab::new()
                                .title("label".to_string())
                                .icon(Some(ui::Icon::Envelope)),
                        ),
                ),
            )
            .child(
                h_stack().child(
                    v_stack()
                        .gap_2()
                        .child(Story::label(cx, "Close Side"))
                        .child(
                            h_stack()
                                .gap_4()
                                .child(
                                    Tab::new()
                                        .title("Left".to_string())
                                        .close_side(IconSide::Left),
                                )
                                .child(Tab::new().title("Right".to_string())),
                        ),
                ),
            )
            .child(
                v_stack()
                    .gap_2()
                    .child(Story::label(cx, "Git Status"))
                    .child(h_stack().gap_4().children(git_statuses.map(|git_status| {
                        Tab::new()
                            .title(git_status.to_string())
                            .git_status(git_status)
                    }))),
            )
            .child(
                v_stack()
                    .gap_2()
                    .child(Story::label(cx, "File System Status"))
                    .child(h_stack().gap_4().children(fs_statuses.map(|fs_status| {
                        Tab::new().title(fs_status.to_string()).fs_status(fs_status)
                    }))),
            )
    }
}
