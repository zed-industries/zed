use crate::{Vim, state::Mode};
use collections::HashMap;
use editor::{
    DisplayPoint, Editor, EditorSettings, SelectionEffects, display_map::DisplayRow,
    scroll::ScrollAmount,
};
use gpui::{Context, Window, actions};
use language::{Bias, CursorShape};
use settings::Settings;
use text::{Selection, SelectionGoal};

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
        preserve_cursor_position: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
        by: fn(c: Option<f32>) -> ScrollAmount,
    ) {
        self.with_geometry(
            move |vim, window, cx| {
                let amount = by(Self::peek_count(cx).map(|count| count as f32));
                let cursor_shape = vim
                    .temp_mode
                    .then(|| vim.cursor_shape_for_mode(Mode::Insert, cx));
                let commit = vim.plan_scroll_editor(
                    preserve_cursor_position,
                    amount,
                    cursor_shape,
                    None,
                    window,
                    cx,
                )?;
                Ok(Box::new(move |vim, window, cx| {
                    Vim::take_count(cx);
                    Vim::take_forced_motion(cx);
                    vim.exit_temporary_normal(window, cx);
                    commit(vim, window, cx);
                }))
            },
            window,
            cx,
        );
    }

    fn plan_scroll_editor(
        &mut self,
        preserve_cursor_position: bool,
        original_amount: ScrollAmount,
        cursor_shape: Option<CursorShape>,
        scrolled_from: Option<DisplayRow>,
        window: &mut Window,
        cx: &mut Context<Vim>,
    ) -> Result<crate::GeometryCommit, editor::display_map::PendingRowGeometry> {
        self.update_editor(
            cx,
            |vim,
             editor,
             cx|
             -> Result<crate::GeometryCommit, editor::display_map::PendingRowGeometry> {
                let should_move_cursor = if scrolled_from.is_some() {
                    true
                } else {
                    match cursor_shape {
                        Some(shape) => editor
                            .newest_selection_on_screen_with_cursor_shape(shape, window, cx)?,
                        None => editor.newest_selection_on_screen(window, cx)?,
                    }
                    .is_eq()
                };
                let amount = original_amount;
                let display_snapshot =
                    Self::geometry_snapshot(editor, cursor_shape.map(|_| false), cx);
                let old_top =
                    scrolled_from.unwrap_or_else(|| editor.scroll_top_row(&display_snapshot, cx));

                let full_page_up = amount.is_full_page() && amount.direction().is_upwards();
                let amount = match (amount.is_full_page(), editor.visible_line_count()) {
                    (true, Some(visible_line_count)) => {
                        if amount.direction().is_upwards() {
                            ScrollAmount::Line((amount.lines(visible_line_count) + 1.0) as f32)
                        } else {
                            ScrollAmount::Line((amount.lines(visible_line_count) - 1.0) as f32)
                        }
                    }
                    _ => amount,
                };

                let scroll_only =
                    move |vim: &mut Vim, window: &mut Window, cx: &mut Context<Vim>| {
                        vim.update_editor(cx, |_, editor, cx| {
                            if scrolled_from.is_none()
                                && !editor.scroll_hover(original_amount, window, cx)
                            {
                                editor.scroll_screen(&amount, window, cx);
                            }
                        });
                    };
                if !should_move_cursor {
                    return Ok(Box::new(scroll_only) as crate::GeometryCommit);
                }
                if scrolled_from.is_none() && editor.pending_rename().is_some() {
                    return Ok(Box::new(move |vim, window, cx| {
                        let scrolled = vim
                            .update_editor(cx, |_, editor, cx| {
                                if editor.scroll_hover(original_amount, window, cx) {
                                    return false;
                                }
                                editor.scroll_screen(&amount, window, cx);
                                true
                            })
                            .unwrap_or(false);
                        if scrolled {
                            vim.with_geometry(
                                move |vim, window, cx| {
                                    vim.plan_scroll_editor(
                                        preserve_cursor_position,
                                        original_amount,
                                        None,
                                        Some(old_top),
                                        window,
                                        cx,
                                    )
                                },
                                window,
                                cx,
                            );
                        }
                    }));
                }
                let Some(visible_line_count) = editor.visible_line_count() else {
                    return Ok(Box::new(scroll_only));
                };
                if editor.visible_column_count().is_none() {
                    return Ok(Box::new(scroll_only));
                }
                let destination = scrolled_from
                    .is_none()
                    .then(|| editor.scroll_screen_destination(&amount, &display_snapshot, cx))
                    .flatten();
                let (top, scroll_left) = destination.map_or_else(
                    || {
                        (
                            editor.scroll_top_row(&display_snapshot, cx),
                            editor.scroll_position(cx).x,
                        )
                    },
                    |destination| (destination.top_row, destination.scroll_left),
                );
                let text_layout_details = editor.text_layout_details(window, cx);
                let Some(viewport) = editor.painted_viewport_at(scroll_left, window, cx) else {
                    return Ok(Box::new(scroll_only));
                };
                let viewport = if let Some(shape) = cursor_shape {
                    viewport.with_cursor_shape(shape)
                } else {
                    viewport
                };
                let vertical_scroll_margin = EditorSettings::get_global(cx).vertical_scroll_margin;

                let move_cursor =
                    |map: &editor::display_map::DisplaySnapshot,
                     selection: Selection<DisplayPoint>|
                     -> Result<_, editor::display_map::PendingRowGeometry> {
                        let mut head = selection.head();
                        let goal = selection.goal;
                        let max_point = map.max_point();

                        let vertical_scroll_margin =
                            (vertical_scroll_margin as u32).min(visible_line_count as u32 / 2);

                        let min_row = if top.0 == 0 {
                            DisplayRow(0)
                        } else {
                            DisplayRow(top.0 + vertical_scroll_margin)
                        };

                        let max_visible_row = top.0.saturating_add(
                            (visible_line_count as u32).saturating_sub(1 + vertical_scroll_margin),
                        );
                        // scroll off the end.
                        let max_row = if top.0 + visible_line_count as u32 >= max_point.row().0 {
                            max_point.row()
                        } else {
                            DisplayRow(
                                (top.0 + visible_line_count as u32)
                                    .saturating_sub(1 + vertical_scroll_margin),
                            )
                        };

                        let starting_column = head.column();
                        if preserve_cursor_position {
                            let row = if old_top == top {
                                head.row()
                                    .0
                                    .saturating_add_signed(amount.lines(visible_line_count) as i32)
                            } else {
                                top.0
                                    .saturating_add_signed(head.row().0 as i32 - old_top.0 as i32)
                            };
                            head = map.clip_point(
                                DisplayPoint::new(DisplayRow(row), head.column()),
                                Bias::Left,
                            );
                        }
                        let row = if full_page_up {
                            DisplayRow(max_visible_row)
                        } else if head.row() < min_row {
                            min_row
                        } else if head.row() > max_row {
                            max_row
                        } else {
                            head.row()
                        };
                        let new_head =
                            map.clip_point(DisplayPoint::new(row, starting_column), Bias::Left);
                        let new_head = viewport
                            .clip_selection(
                                map,
                                &selection,
                                new_head,
                                &text_layout_details,
                                window,
                                cx,
                            )?
                            .unwrap_or(new_head);
                        let goal = if matches!(
                            amount,
                            ScrollAmount::Column(_) | ScrollAmount::PageWidth(_)
                        ) {
                            SelectionGoal::None
                        } else {
                            goal
                        };
                        Ok((new_head, goal))
                    };

                let (block_selections, moved_heads) = if vim.mode == Mode::VisualBlock {
                    let selection = editor.selections.newest_display(&display_snapshot);
                    let (new_head, _) = move_cursor(&display_snapshot, selection)?;
                    let selections = vim.plan_visual_block_motion(
                        true,
                        editor,
                        (
                            *editor.selections.newest_anchor(),
                            *editor.selections.oldest_anchor(),
                        ),
                        window,
                        cx,
                        &mut |_, _, goal| Ok(Some((new_head, goal))),
                    )?;
                    (selections, None)
                } else {
                    let moved_heads = editor
                        .selections
                        .all_display(&display_snapshot)
                        .into_iter()
                        .map(|selection| {
                            Ok((selection.id, move_cursor(&display_snapshot, selection)?))
                        })
                        .collect::<Result<HashMap<_, _>, editor::display_map::PendingRowGeometry>>(
                        )?;
                    (None, Some(moved_heads))
                };
                Ok(Box::new(
                    move |vim: &mut Vim, window: &mut Window, cx: &mut Context<Vim>| {
                        vim.update_editor(cx, |vim, editor, cx| {
                            if scrolled_from.is_none() {
                                if editor.scroll_hover(original_amount, &mut *window, cx) {
                                    return;
                                }
                                editor.scroll_screen(&amount, window, cx);
                            }
                            Self::apply_visual_block_motion(editor, block_selections, window, cx);
                            if let Some(moved_heads) = moved_heads {
                                editor.change_selections(
                                    SelectionEffects::no_scroll().nav_history(false),
                                    window,
                                    cx,
                                    |s| {
                                        s.move_with(&mut |_, selection| {
                                            if let Some(&(new_head, goal)) =
                                                moved_heads.get(&selection.id)
                                            {
                                                if selection.is_empty() || !vim.mode.is_visual() {
                                                    selection.collapse_to(new_head, goal)
                                                } else {
                                                    selection.set_head(new_head, goal)
                                                }
                                            }
                                        });
                                    },
                                );
                            }
                        });
                    },
                ) as crate::GeometryCommit)
            },
        )
        .unwrap_or_else(|| Ok(Box::new(|_, _, _| {})))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };
    use editor::{Inlay, ScrollBeyondLastLine};
    use gpui::{AppContext as _, point, px, size};
    use indoc::indoc;
    use language::{Point, language_settings::SoftWrap};
    use settings::SettingsStore;
    use std::num::NonZeroU32;

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

        let (line_height, visible_line_count) = cx.update_editor(|editor, window, cx| {
            (
                editor
                    .style(cx)
                    .text
                    .line_height_in_pixels(window.rem_size()),
                editor.visible_line_count().unwrap(),
            )
        });

        let window = cx.window;
        let margin = cx
            .update_window(window, |_, window, _cx| {
                window.viewport_size().height - line_height * visible_line_count as f32
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
                editor
                    .selections
                    .newest(&editor.display_snapshot(cx))
                    .range(),
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
                editor
                    .selections
                    .newest(&editor.display_snapshot(cx))
                    .range(),
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

        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.all_languages.defaults.soft_wrap = Some(SoftWrap::None);
                settings.project.all_languages.defaults.tab_size = NonZeroU32::new(4);
            });
        });
        cx.set_state(&format!("ˇ\t{}", "x".repeat(2_000)), Mode::Normal);
        cx.simulate_keystrokes("z l 1 0 z l");
        cx.assert_state(
            &format!("\t{}ˇ{}", "x".repeat(7), "x".repeat(1_993)),
            Mode::Normal,
        );

        let content = format!("ˇx\n{}", "x".repeat(2_000));
        cx.set_state(&content, Mode::Normal);
        cx.update_editor(|editor, window, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            editor.splice_inlays(
                &[],
                vec![Inlay::mock_hint(
                    0,
                    snapshot.anchor_after(Point::new(0, 1)),
                    "h".repeat(200),
                )],
                cx,
            );
            editor.set_scroll_position(point(0., 0.), window, cx);
        });
        cx.run_until_parked();
        cx.simulate_keystrokes("2 z l");
        cx.assert_state(&content, Mode::Normal);
    }

    #[gpui::test]
    async fn test_ctrl_d_keeps_column_when_scrolled_horizontally(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.all_languages.defaults.soft_wrap = Some(SoftWrap::None);
            });
        });
        let line = "x".repeat(400);
        let text = format!(
            "\n{}ˇ{}\n{}",
            &line[..200],
            &line[200..],
            format!("{line}\n").repeat(40)
        );
        cx.set_state(&text, Mode::Normal);
        cx.simulate_window_resize(cx.window, size(px(400.), px(400.)));
        cx.update_editor(|editor, window, cx| {
            editor.set_scroll_position(point(180., 0.), window, cx);
        });
        cx.run_until_parked();
        cx.update_editor(|editor, window, cx| {
            assert_eq!(
                editor.snapshot(window, cx).scroll_position(),
                point(180., 0.)
            );
        });

        cx.simulate_keystrokes("ctrl-d");
        cx.update_editor(|editor, _, cx| {
            let head = editor
                .selections
                .newest::<Point>(&editor.display_snapshot(cx))
                .head();
            assert_ne!(head.row, 1);
            assert_eq!(head.column, 200);
        });
    }
}
