use crate::Vim;
use editor::{
    DisplayPoint, Editor, EditorSettings, SelectionEffects,
    display_map::{DisplayRow, ToDisplayPoint},
    scroll::ScrollAmount,
};
use gpui::{Context, Window, actions};
use language::Bias;
use settings::Settings;
use text::SelectionGoal;

actions!(
    vim,
    [
        /// Scrolls up by one line.
        LineUp,
        /// Scrolls down by one line.
        LineDown,
        /// Scrolls right by one column.
        ColumnRight,
        /// Scrolls left by one column.
        ColumnLeft,
        /// Scrolls up by half a page.
        ScrollUp,
        /// Scrolls down by half a page.
        ScrollDown,
        /// Scrolls up by one page.
        PageUp,
        /// Scrolls down by one page.
        PageDown,
        /// Scrolls right by half a page's width.
        HalfPageRight,
        /// Scrolls left by half a page's width.
        HalfPageLeft,
    ]
);

pub fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, |vim, _: &LineDown, window, cx| {
        vim.scroll(false, window, cx, |c| ScrollAmount::Line(c.unwrap_or(1.)))
    });
    Vim::action(editor, cx, |vim, _: &LineUp, window, cx| {
        vim.scroll(false, window, cx, |c| ScrollAmount::Line(-c.unwrap_or(1.)))
    });
    Vim::action(editor, cx, |vim, _: &ColumnRight, window, cx| {
        vim.scroll(false, window, cx, |c| ScrollAmount::Column(c.unwrap_or(1.)))
    });
    Vim::action(editor, cx, |vim, _: &ColumnLeft, window, cx| {
        vim.scroll(false, window, cx, |c| {
            ScrollAmount::Column(-c.unwrap_or(1.))
        })
    });
    Vim::action(editor, cx, |vim, _: &PageDown, window, cx| {
        vim.scroll(false, window, cx, |c| ScrollAmount::Page(c.unwrap_or(1.)))
    });
    Vim::action(editor, cx, |vim, _: &PageUp, window, cx| {
        vim.scroll(false, window, cx, |c| ScrollAmount::Page(-c.unwrap_or(1.)))
    });
    Vim::action(editor, cx, |vim, _: &HalfPageRight, window, cx| {
        vim.scroll(false, window, cx, |c| {
            ScrollAmount::PageWidth(c.unwrap_or(0.5))
        })
    });
    Vim::action(editor, cx, |vim, _: &HalfPageLeft, window, cx| {
        vim.scroll(false, window, cx, |c| {
            ScrollAmount::PageWidth(-c.unwrap_or(0.5))
        })
    });
    Vim::action(editor, cx, |vim, _: &ScrollDown, window, cx| {
        vim.scroll(true, window, cx, |c| {
            if let Some(c) = c {
                ScrollAmount::Line(c)
            } else {
                ScrollAmount::Page(0.5)
            }
        })
    });
    Vim::action(editor, cx, |vim, _: &ScrollUp, window, cx| {
        vim.scroll(true, window, cx, |c| {
            if let Some(c) = c {
                ScrollAmount::Line(-c)
            } else {
                ScrollAmount::Page(-0.5)
            }
        })
    });
}

impl Vim {
    fn scroll(
        &mut self,
        move_cursor: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
        by: fn(c: Option<f32>) -> ScrollAmount,
    ) {
        let amount = by(Vim::take_count(cx).map(|c| c as f32));
        Vim::take_forced_motion(cx);
        self.exit_temporary_normal(window, cx);
        self.update_editor(cx, |_, editor, cx| {
            scroll_editor(editor, move_cursor, amount, window, cx)
        });
    }
}

fn scroll_editor(
    editor: &mut Editor,
    preserve_cursor_position: bool,
    amount: ScrollAmount,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let should_move_cursor = editor.newest_selection_on_screen(cx).is_eq();
    let old_top_anchor = editor.scroll_manager.anchor().anchor;

    if editor.scroll_hover(amount, window, cx) {
        return;
    }

    let full_page_up = amount.is_full_page() && amount.direction().is_upwards();
    let amount = match (amount.is_full_page(), editor.visible_line_count()) {
        (true, Some(visible_line_count)) => {
            if amount.direction().is_upwards() {
                ScrollAmount::Line(amount.lines(visible_line_count) + 1.0)
            } else {
                ScrollAmount::Line(amount.lines(visible_line_count) - 1.0)
            }
        }
        _ => amount,
    };

    editor.scroll_screen(&amount, window, cx);
    if !should_move_cursor {
        return;
    }

    let Some(visible_line_count) = editor.visible_line_count() else {
        return;
    };

    let Some(visible_column_count) = editor.visible_column_count() else {
        return;
    };

    let top_anchor = editor.scroll_manager.anchor().anchor;
    let vertical_scroll_margin = EditorSettings::get_global(cx).vertical_scroll_margin;

    editor.change_selections(
        SelectionEffects::no_scroll().nav_history(false),
        window,
        cx,
        |s| {
            s.move_with(|map, selection| {
                // TODO: Improve the logic and function calls below to be dependent on
                // the `amount`. If the amount is vertical, we don't care about
                // columns, while if it's horizontal, we don't care about rows,
                // so we don't need to calculate both and deal with logic for
                // both.
                let mut head = selection.head();
                let top = top_anchor.to_display_point(map);
                let max_point = map.max_point();
                let starting_column = head.column();

                let vertical_scroll_margin =
                    (vertical_scroll_margin as u32).min(visible_line_count as u32 / 2);

                if preserve_cursor_position {
                    let old_top = old_top_anchor.to_display_point(map);
                    let new_row = if old_top.row() == top.row() {
                        DisplayRow(
                            head.row()
                                .0
                                .saturating_add_signed(amount.lines(visible_line_count) as i32),
                        )
                    } else {
                        DisplayRow(top.row().0 + selection.head().row().0 - old_top.row().0)
                    };
                    head = map.clip_point(DisplayPoint::new(new_row, head.column()), Bias::Left)
                }

                let min_row = if top.row().0 == 0 {
                    DisplayRow(0)
                } else {
                    DisplayRow(top.row().0 + vertical_scroll_margin)
                };

                let max_visible_row = top.row().0.saturating_add(
                    (visible_line_count as u32).saturating_sub(1 + vertical_scroll_margin),
                );
                // scroll off the end.
                let max_row = if top.row().0 + visible_line_count as u32 >= max_point.row().0 {
                    max_point.row()
                } else {
                    DisplayRow(
                        (top.row().0 + visible_line_count as u32)
                            .saturating_sub(1 + vertical_scroll_margin),
                    )
                };

                let new_row = if full_page_up {
                    // Special-casing ctrl-b/page-up, which is special-cased by Vim, it seems
                    // to always put the cursor on the last line of the page, even if the cursor
                    // was before that.
                    DisplayRow(max_visible_row)
                } else if head.row() < min_row {
                    min_row
                } else if head.row() > max_row {
                    max_row
                } else {
                    head.row()
                };

                // The minimum column position that the cursor position can be
                // at is either the scroll manager's anchor column, which is the
                // left-most column in the visible area, or the scroll manager's
                // old anchor column, in case the cursor position is being
                // preserved. This is necessary for motions like `ctrl-d` in
                // case there's not enough content to scroll half page down, in
                // which case the scroll manager's anchor column will be the
                // maximum column for the current line, so the minimum column
                // would end up being the same as the maximum column.
                let min_column = match preserve_cursor_position {
                    true => old_top_anchor.to_display_point(map).column(),
                    false => top.column(),
                };

                // As for the maximum column position, that should be either the
                // right-most column in the visible area, which we can easily
                // calculate by adding the visible column count to the minimum
                // column position, or the right-most column in the current
                // line, seeing as the cursor might be in a short line, in which
                // case we don't want to go past its last column.
                let max_row_column = if new_row <= map.max_point().row() {
                    map.line_len(new_row)
                } else {
                    0
                };
                let max_column = match min_column + visible_column_count as u32 {
                    max_column if max_column >= max_row_column => max_row_column,
                    max_column => max_column,
                };

                // Ensure that the cursor's column stays within the visible
                // area, otherwise clip it at either the left or right edge of
                // the visible area.
                let new_column = match (min_column, max_column) {
                    (min_column, _) if starting_column < min_column => min_column,
                    (_, max_column) if starting_column > max_column => max_column,
                    _ => starting_column,
                };

                let new_head = map.clip_point(DisplayPoint::new(new_row, new_column), Bias::Left);
                let goal = match amount {
                    ScrollAmount::Column(_) | ScrollAmount::PageWidth(_) => SelectionGoal::None,
                    _ => selection.goal,
                };

                if selection.is_empty() {
                    selection.collapse_to(new_head, goal)
                } else {
                    selection.set_head(new_head, goal)
                };
            })
        },
    );
}

#[cfg(test)]
mod test {
    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };
    use editor::ScrollBeyondLastLine;
    use gpui::{AppContext as _, point, px, size};
    use indoc::indoc;
    use language::Point;
    use settings::SettingsStore;

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

    #[gpui::test]
    async fn test_scroll(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        let (line_height, visible_line_count) = cx.editor(|editor, window, _cx| {
            (
                editor
                    .style()
                    .unwrap()
                    .text
                    .line_height_in_pixels(window.rem_size()),
                editor.visible_line_count().unwrap(),
            )
        });

        let window = cx.window;
        let margin = cx
            .update_window(window, |_, window, _cx| {
                window.viewport_size().height - line_height * visible_line_count
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

        cx.update_editor(|editor, window, cx| {
            assert_eq!(editor.snapshot(window, cx).scroll_position(), point(0., 0.))
        });
        cx.simulate_keystrokes("ctrl-e");
        cx.update_editor(|editor, window, cx| {
            assert_eq!(editor.snapshot(window, cx).scroll_position(), point(0., 1.))
        });
        cx.simulate_keystrokes("2 ctrl-e");
        cx.update_editor(|editor, window, cx| {
            assert_eq!(editor.snapshot(window, cx).scroll_position(), point(0., 3.))
        });
        cx.simulate_keystrokes("ctrl-y");
        cx.update_editor(|editor, window, cx| {
            assert_eq!(editor.snapshot(window, cx).scroll_position(), point(0., 2.))
        });

        // does not select in normal mode
        cx.simulate_keystrokes("g g");
        cx.update_editor(|editor, window, cx| {
            assert_eq!(editor.snapshot(window, cx).scroll_position(), point(0., 0.))
        });
        cx.simulate_keystrokes("ctrl-d");
        cx.update_editor(|editor, window, cx| {
            assert_eq!(
                editor.snapshot(window, cx).scroll_position(),
                point(0., 3.0)
            );
            assert_eq!(
                editor.selections.newest(cx).range(),
                Point::new(6, 0)..Point::new(6, 0)
            )
        });

        // does select in visual mode
        cx.simulate_keystrokes("g g");
        cx.update_editor(|editor, window, cx| {
            assert_eq!(editor.snapshot(window, cx).scroll_position(), point(0., 0.))
        });
        cx.simulate_keystrokes("v ctrl-d");
        cx.update_editor(|editor, window, cx| {
            assert_eq!(
                editor.snapshot(window, cx).scroll_position(),
                point(0., 3.0)
            );
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

        let content = "ˇ".to_owned() + &sample_text(26, 2, 'a');
        cx.set_shared_state(&content).await;

        // skip over the scrolloff at the top
        // test ctrl-d
        cx.simulate_shared_keystrokes("4 j ctrl-d").await;
        cx.shared_state().await.assert_matches();
        cx.simulate_shared_keystrokes("ctrl-d").await;
        cx.shared_state().await.assert_matches();
        cx.simulate_shared_keystrokes("g g ctrl-d").await;
        cx.shared_state().await.assert_matches();

        // test ctrl-u
        cx.simulate_shared_keystrokes("ctrl-u").await;
        cx.shared_state().await.assert_matches();
        cx.simulate_shared_keystrokes("ctrl-d ctrl-d 4 j ctrl-u ctrl-u")
            .await;
        cx.shared_state().await.assert_matches();

        // test returning to top
        cx.simulate_shared_keystrokes("g g ctrl-d ctrl-u ctrl-u")
            .await;
        cx.shared_state().await.assert_matches();
    }

    #[gpui::test]
    async fn test_ctrl_f_b(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        let visible_lines = 10;
        cx.set_scroll_height(visible_lines).await;

        // First test without vertical scroll margin
        cx.neovim.set_option(&format!("scrolloff={}", 0)).await;
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings(cx, |s| s.editor.vertical_scroll_margin = Some(0.0));
        });

        let content = "ˇ".to_owned() + &sample_text(26, 2, 'a');
        cx.set_shared_state(&content).await;

        // scroll down: ctrl-f
        cx.simulate_shared_keystrokes("ctrl-f").await;
        cx.shared_state().await.assert_matches();

        cx.simulate_shared_keystrokes("ctrl-f").await;
        cx.shared_state().await.assert_matches();

        // scroll up: ctrl-b
        cx.simulate_shared_keystrokes("ctrl-b").await;
        cx.shared_state().await.assert_matches();

        cx.simulate_shared_keystrokes("ctrl-b").await;
        cx.shared_state().await.assert_matches();

        // Now go back to start of file, and test with vertical scroll margin
        cx.simulate_shared_keystrokes("g g").await;
        cx.shared_state().await.assert_matches();

        cx.neovim.set_option(&format!("scrolloff={}", 3)).await;
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings(cx, |s| s.editor.vertical_scroll_margin = Some(3.0));
        });

        // scroll down: ctrl-f
        cx.simulate_shared_keystrokes("ctrl-f").await;
        cx.shared_state().await.assert_matches();

        cx.simulate_shared_keystrokes("ctrl-f").await;
        cx.shared_state().await.assert_matches();

        // scroll up: ctrl-b
        cx.simulate_shared_keystrokes("ctrl-b").await;
        cx.shared_state().await.assert_matches();

        cx.simulate_shared_keystrokes("ctrl-b").await;
        cx.shared_state().await.assert_matches();
    }

    #[gpui::test]
    async fn test_scroll_beyond_last_line(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_scroll_height(10).await;

        let content = "ˇ".to_owned() + &sample_text(26, 2, 'a');
        cx.set_shared_state(&content).await;

        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings(cx, |s| {
                s.editor.scroll_beyond_last_line = Some(ScrollBeyondLastLine::Off);
            });
        });

        // ctrl-d can reach the end and the cursor stays in the first column
        cx.simulate_shared_keystrokes("shift-g k").await;
        cx.shared_state().await.assert_matches();
        cx.simulate_shared_keystrokes("ctrl-d").await;
        cx.shared_state().await.assert_matches();

        // ctrl-u from the last line
        cx.simulate_shared_keystrokes("shift-g").await;
        cx.shared_state().await.assert_matches();
        cx.simulate_shared_keystrokes("ctrl-u").await;
        cx.shared_state().await.assert_matches();
    }

    #[gpui::test]
    async fn test_ctrl_y_e(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_scroll_height(10).await;

        let content = "ˇ".to_owned() + &sample_text(26, 2, 'a');
        cx.set_shared_state(&content).await;

        for _ in 0..8 {
            cx.simulate_shared_keystrokes("ctrl-e").await;
            cx.shared_state().await.assert_matches();
        }

        for _ in 0..8 {
            cx.simulate_shared_keystrokes("ctrl-y").await;
            cx.shared_state().await.assert_matches();
        }
    }

    #[gpui::test]
    async fn test_scroll_jumps(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_scroll_height(20).await;

        let content = "ˇ".to_owned() + &sample_text(52, 2, 'a');
        cx.set_shared_state(&content).await;

        cx.simulate_shared_keystrokes("shift-g g g").await;
        cx.simulate_shared_keystrokes("ctrl-d ctrl-d ctrl-o").await;
        cx.shared_state().await.assert_matches();
        cx.simulate_shared_keystrokes("ctrl-o").await;
        cx.shared_state().await.assert_matches();
    }

    #[gpui::test]
    async fn test_horizontal_scroll(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_scroll_height(20).await;
        cx.set_shared_wrap(12).await;
        cx.set_neovim_option("nowrap").await;

        let content = "ˇ01234567890123456789";
        cx.set_shared_state(content).await;

        cx.simulate_shared_keystrokes("z shift-l").await;
        cx.shared_state().await.assert_eq("012345ˇ67890123456789");

        // At this point, `z h` should not move the cursor as it should still be
        // visible within the 12 column width.
        cx.simulate_shared_keystrokes("z h").await;
        cx.shared_state().await.assert_eq("012345ˇ67890123456789");

        let content = "ˇ01234567890123456789";
        cx.set_shared_state(content).await;

        cx.simulate_shared_keystrokes("z l").await;
        cx.shared_state().await.assert_eq("0ˇ1234567890123456789");
    }
}
