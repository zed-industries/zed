use crate::Vim;
use editor::{
    display_map::ToDisplayPoint,
    scroll::{scroll_amount::ScrollAmount, VERTICAL_SCROLL_MARGIN},
    DisplayPoint, Editor,
};
use gpui::{actions, AppContext, ViewContext};
use language::Bias;
use workspace::Workspace;

actions!(
    vim,
    [LineUp, LineDown, ScrollUp, ScrollDown, PageUp, PageDown,]
);

pub fn init(cx: &mut AppContext) {
    cx.add_action(|_: &mut Workspace, _: &LineDown, cx| {
        scroll(cx, |c| ScrollAmount::Line(c.unwrap_or(1.)))
    });
    cx.add_action(|_: &mut Workspace, _: &LineUp, cx| {
        scroll(cx, |c| ScrollAmount::Line(-c.unwrap_or(1.)))
    });
    cx.add_action(|_: &mut Workspace, _: &PageDown, cx| {
        scroll(cx, |c| ScrollAmount::Page(c.unwrap_or(1.)))
    });
    cx.add_action(|_: &mut Workspace, _: &PageUp, cx| {
        scroll(cx, |c| ScrollAmount::Page(-c.unwrap_or(1.)))
    });
    cx.add_action(|_: &mut Workspace, _: &ScrollDown, cx| {
        scroll(cx, |c| {
            if let Some(c) = c {
                ScrollAmount::Line(c)
            } else {
                ScrollAmount::Page(0.5)
            }
        })
    });
    cx.add_action(|_: &mut Workspace, _: &ScrollUp, cx| {
        scroll(cx, |c| {
            if let Some(c) = c {
                ScrollAmount::Line(-c)
            } else {
                ScrollAmount::Page(-0.5)
            }
        })
    });
}

fn scroll(cx: &mut ViewContext<Workspace>, by: fn(c: Option<f32>) -> ScrollAmount) {
    Vim::update(cx, |vim, cx| {
        let amount = by(vim.pop_number_operator(cx).map(|c| c as f32));
        vim.update_active_editor(cx, |editor, cx| scroll_editor(editor, &amount, cx));
    })
}

fn scroll_editor(editor: &mut Editor, amount: &ScrollAmount, cx: &mut ViewContext<Editor>) {
    let should_move_cursor = editor.newest_selection_on_screen(cx).is_eq();

    editor.scroll_screen(amount, cx);
    if should_move_cursor {
        let visible_rows = if let Some(visible_rows) = editor.visible_line_count() {
            visible_rows as u32
        } else {
            return;
        };

        let top_anchor = editor.scroll_manager.anchor().anchor;

        editor.change_selections(None, cx, |s| {
            s.move_with(|map, selection| {
                let head = selection.head();
                let top = top_anchor.to_display_point(map);
                let min_row = top.row() + VERTICAL_SCROLL_MARGIN as u32;
                let max_row = top.row() + visible_rows - VERTICAL_SCROLL_MARGIN as u32 - 1;

                let new_head = if head.row() < min_row {
                    map.clip_point(DisplayPoint::new(min_row, head.column()), Bias::Left)
                } else if head.row() > max_row {
                    map.clip_point(DisplayPoint::new(max_row, head.column()), Bias::Left)
                } else {
                    head
                };
                if selection.is_empty() {
                    selection.collapse_to(new_head, selection.goal)
                } else {
                    selection.set_head(new_head, selection.goal)
                };
            })
        });
    }
}

#[cfg(test)]
mod test {
    use crate::{state::Mode, test::VimTestContext};
    use gpui::geometry::vector::vec2f;
    use indoc::indoc;
    use language::Point;

    #[gpui::test]
    async fn test_scroll(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        let window = cx.window;
        let line_height =
            cx.editor(|editor, cx| editor.style(cx).text.line_height(cx.font_cache()));
        window.simulate_resize(vec2f(1000., 8.0 * line_height - 1.0), &mut cx);

        cx.set_state(
            indoc!(
                "ˇone
                two
                three
                four
                five
                six
                seven
                eight
                nine
                ten
                eleven
                twelve
            "
            ),
            Mode::Normal,
        );

        cx.update_editor(|editor, cx| {
            assert_eq!(editor.snapshot(cx).scroll_position(), vec2f(0., 0.))
        });
        cx.simulate_keystrokes(["ctrl-e"]);
        cx.update_editor(|editor, cx| {
            assert_eq!(editor.snapshot(cx).scroll_position(), vec2f(0., 1.))
        });
        cx.simulate_keystrokes(["2", "ctrl-e"]);
        cx.update_editor(|editor, cx| {
            assert_eq!(editor.snapshot(cx).scroll_position(), vec2f(0., 3.))
        });
        cx.simulate_keystrokes(["ctrl-y"]);
        cx.update_editor(|editor, cx| {
            assert_eq!(editor.snapshot(cx).scroll_position(), vec2f(0., 2.))
        });

        // does not select in normal mode
        cx.simulate_keystrokes(["g", "g"]);
        cx.update_editor(|editor, cx| {
            assert_eq!(editor.snapshot(cx).scroll_position(), vec2f(0., 0.))
        });
        cx.simulate_keystrokes(["ctrl-d"]);
        cx.update_editor(|editor, cx| {
            assert_eq!(editor.snapshot(cx).scroll_position(), vec2f(0., 2.0));
            assert_eq!(
                editor.selections.newest(cx).range(),
                Point::new(5, 0)..Point::new(5, 0)
            )
        });

        // does select in visual mode
        cx.simulate_keystrokes(["g", "g"]);
        cx.update_editor(|editor, cx| {
            assert_eq!(editor.snapshot(cx).scroll_position(), vec2f(0., 0.))
        });
        cx.simulate_keystrokes(["v", "ctrl-d"]);
        cx.update_editor(|editor, cx| {
            assert_eq!(editor.snapshot(cx).scroll_position(), vec2f(0., 2.0));
            assert_eq!(
                editor.selections.newest(cx).range(),
                Point::new(0, 0)..Point::new(5, 1)
            )
        });
    }
}
