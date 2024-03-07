use crate::Vim;
use editor::{
    display_map::ToDisplayPoint, scroll::ScrollAmount, DisplayPoint, Editor, EditorSettings,
};
use gpui::{actions, ViewContext};
use language::Bias;
use settings::Settings;
use workspace::Workspace;

actions!(
    vim,
    [LineUp, LineDown, ScrollUp, ScrollDown, PageUp, PageDown]
);

pub fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
    workspace.register_action(|_: &mut Workspace, _: &LineDown, cx| {
        scroll(cx, false, |c| ScrollAmount::Line(c.unwrap_or(1.)))
    });
    workspace.register_action(|_: &mut Workspace, _: &LineUp, cx| {
        scroll(cx, false, |c| ScrollAmount::Line(-c.unwrap_or(1.)))
    });
    workspace.register_action(|_: &mut Workspace, _: &PageDown, cx| {
        scroll(cx, false, |c| ScrollAmount::Page(c.unwrap_or(1.)))
    });
    workspace.register_action(|_: &mut Workspace, _: &PageUp, cx| {
        scroll(cx, false, |c| ScrollAmount::Page(-c.unwrap_or(1.)))
    });
    workspace.register_action(|_: &mut Workspace, _: &ScrollDown, cx| {
        scroll(cx, true, |c| {
            if let Some(c) = c {
                ScrollAmount::Line(c)
            } else {
                ScrollAmount::Page(0.5)
            }
        })
    });
    workspace.register_action(|_: &mut Workspace, _: &ScrollUp, cx| {
        scroll(cx, true, |c| {
            if let Some(c) = c {
                ScrollAmount::Line(-c)
            } else {
                ScrollAmount::Page(-0.5)
            }
        })
    });
}

fn scroll(
    cx: &mut ViewContext<Workspace>,
    move_cursor: bool,
    by: fn(c: Option<f32>) -> ScrollAmount,
) {
    Vim::update(cx, |vim, cx| {
        let amount = by(vim.take_count(cx).map(|c| c as f32));
        vim.update_active_editor(cx, |_, editor, cx| {
            scroll_editor(editor, move_cursor, &amount, cx)
        });
    })
}

fn scroll_editor(
    editor: &mut Editor,
    preserve_cursor_position: bool,
    amount: &ScrollAmount,
    cx: &mut ViewContext<Editor>,
) {
    let should_move_cursor = editor.newest_selection_on_screen(cx).is_eq();
    let old_top_anchor = editor.scroll_manager.anchor().anchor;

    editor.scroll_screen(amount, cx);
    if should_move_cursor {
        let visible_rows = if let Some(visible_rows) = editor.visible_line_count() {
            visible_rows as u32
        } else {
            return;
        };

        let top_anchor = editor.scroll_manager.anchor().anchor;
        let vertical_scroll_margin = EditorSettings::get_global(cx).vertical_scroll_margin;

        editor.change_selections(None, cx, |s| {
            s.move_with(|map, selection| {
                let mut head = selection.head();
                let top = top_anchor.to_display_point(map);

                if preserve_cursor_position {
                    let old_top = old_top_anchor.to_display_point(map);
                    let new_row = top.row() + selection.head().row() - old_top.row();
                    head = map.clip_point(DisplayPoint::new(new_row, head.column()), Bias::Left)
                }
                let min_row = top.row() + vertical_scroll_margin as u32;
                let max_row = top.row() + visible_rows - vertical_scroll_margin as u32 - 1;

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
    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };
    use gpui::{point, px, size, Context};
    use indoc::indoc;
    use language::Point;

    #[gpui::test]
    async fn test_scroll(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        let (line_height, visible_line_count) = cx.editor(|editor, cx| {
            (
                editor
                    .style()
                    .unwrap()
                    .text
                    .line_height_in_pixels(cx.rem_size()),
                editor.visible_line_count().unwrap(),
            )
        });

        let window = cx.window;
        let margin = cx
            .update_window(window, |_, cx| {
                cx.viewport_size().height - line_height * visible_line_count
            })
            .unwrap();
        cx.simulate_window_resize(
            cx.window,
            size(px(1000.), margin + 8. * line_height - px(1.0)),
        );

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
            assert_eq!(editor.snapshot(cx).scroll_position(), point(0., 0.))
        });
        cx.simulate_keystrokes(["ctrl-e"]);
        cx.update_editor(|editor, cx| {
            assert_eq!(editor.snapshot(cx).scroll_position(), point(0., 1.))
        });
        cx.simulate_keystrokes(["2", "ctrl-e"]);
        cx.update_editor(|editor, cx| {
            assert_eq!(editor.snapshot(cx).scroll_position(), point(0., 3.))
        });
        cx.simulate_keystrokes(["ctrl-y"]);
        cx.update_editor(|editor, cx| {
            assert_eq!(editor.snapshot(cx).scroll_position(), point(0., 2.))
        });

        // does not select in normal mode
        cx.simulate_keystrokes(["g", "g"]);
        cx.update_editor(|editor, cx| {
            assert_eq!(editor.snapshot(cx).scroll_position(), point(0., 0.))
        });
        cx.simulate_keystrokes(["ctrl-d"]);
        cx.update_editor(|editor, cx| {
            assert_eq!(editor.snapshot(cx).scroll_position(), point(0., 3.0));
            assert_eq!(
                editor.selections.newest(cx).range(),
                Point::new(6, 0)..Point::new(6, 0)
            )
        });

        // does select in visual mode
        cx.simulate_keystrokes(["g", "g"]);
        cx.update_editor(|editor, cx| {
            assert_eq!(editor.snapshot(cx).scroll_position(), point(0., 0.))
        });
        cx.simulate_keystrokes(["v", "ctrl-d"]);
        cx.update_editor(|editor, cx| {
            assert_eq!(editor.snapshot(cx).scroll_position(), point(0., 3.0));
            assert_eq!(
                editor.selections.newest(cx).range(),
                Point::new(0, 0)..Point::new(6, 1)
            )
        });
    }
    #[gpui::test]
    async fn test_ctrl_d_u(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_scroll_height(10).await;

        pub fn sample_text(rows: usize, cols: usize, start_char: char) -> String {
            let mut text = String::new();
            for row in 0..rows {
                let c: char = (start_char as u32 + row as u32) as u8 as char;
                let mut line = c.to_string().repeat(cols);
                if row < rows - 1 {
                    line.push('\n');
                }
                text += &line;
            }
            text
        }
        let content = "ˇ".to_owned() + &sample_text(26, 2, 'a');
        cx.set_shared_state(&content).await;

        // skip over the scrolloff at the top
        // test ctrl-d
        cx.simulate_shared_keystrokes(["4", "j", "ctrl-d"]).await;
        cx.assert_state_matches().await;
        cx.simulate_shared_keystrokes(["ctrl-d"]).await;
        cx.assert_state_matches().await;
        cx.simulate_shared_keystrokes(["g", "g", "ctrl-d"]).await;
        cx.assert_state_matches().await;

        // test ctrl-u
        cx.simulate_shared_keystrokes(["ctrl-u"]).await;
        cx.assert_state_matches().await;
        cx.simulate_shared_keystrokes(["ctrl-d", "ctrl-d", "4", "j", "ctrl-u", "ctrl-u"])
            .await;
        cx.assert_state_matches().await;
    }
}
