use super::*;
use crate::test::{
    assert_text_with_selections, build_editor, editor_lsp_test_context::EditorLspTestContext,
    editor_test_context::EditorTestContext, select_ranges,
};
use drag_and_drop::DragAndDrop;
use futures::StreamExt;
use gpui::{
    executor::Deterministic,
    geometry::{rect::RectF, vector::vec2f},
    platform::{WindowBounds, WindowOptions},
    serde_json,
};
use indoc::indoc;
use language::{BracketPairConfig, FakeLspAdapter, LanguageConfig, LanguageRegistry, Point};
use parking_lot::Mutex;
use project::FakeFs;
use settings::EditorSettings;
use std::{cell::RefCell, rc::Rc, time::Instant};
use unindent::Unindent;
use util::{
    assert_set_eq,
    test::{marked_text_ranges, marked_text_ranges_by, sample_text, TextRangeMarker},
};
use workspace::{
    item::{FollowableItem, ItemHandle},
    NavigationEntry, Pane, ViewId,
};

#[gpui::test]
fn test_edit_events(cx: &mut MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = cx.add_model(|cx| {
        let mut buffer = language::Buffer::new(0, "123456", cx);
        buffer.set_group_interval(Duration::from_secs(1));
        buffer
    });

    let events = Rc::new(RefCell::new(Vec::new()));
    let (_, editor1) = cx.add_window(Default::default(), {
        let events = events.clone();
        |cx| {
            cx.subscribe(&cx.handle(), move |_, _, event, _| {
                if matches!(
                    event,
                    Event::Edited | Event::BufferEdited | Event::DirtyChanged
                ) {
                    events.borrow_mut().push(("editor1", event.clone()));
                }
            })
            .detach();
            Editor::for_buffer(buffer.clone(), None, cx)
        }
    });
    let (_, editor2) = cx.add_window(Default::default(), {
        let events = events.clone();
        |cx| {
            cx.subscribe(&cx.handle(), move |_, _, event, _| {
                if matches!(
                    event,
                    Event::Edited | Event::BufferEdited | Event::DirtyChanged
                ) {
                    events.borrow_mut().push(("editor2", event.clone()));
                }
            })
            .detach();
            Editor::for_buffer(buffer.clone(), None, cx)
        }
    });
    assert_eq!(mem::take(&mut *events.borrow_mut()), []);

    // Mutating editor 1 will emit an `Edited` event only for that editor.
    editor1.update(cx, |editor, cx| editor.insert("X", cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor1", Event::Edited),
            ("editor1", Event::BufferEdited),
            ("editor2", Event::BufferEdited),
            ("editor1", Event::DirtyChanged),
            ("editor2", Event::DirtyChanged)
        ]
    );

    // Mutating editor 2 will emit an `Edited` event only for that editor.
    editor2.update(cx, |editor, cx| editor.delete(&Delete, cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor2", Event::Edited),
            ("editor1", Event::BufferEdited),
            ("editor2", Event::BufferEdited),
        ]
    );

    // Undoing on editor 1 will emit an `Edited` event only for that editor.
    editor1.update(cx, |editor, cx| editor.undo(&Undo, cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor1", Event::Edited),
            ("editor1", Event::BufferEdited),
            ("editor2", Event::BufferEdited),
            ("editor1", Event::DirtyChanged),
            ("editor2", Event::DirtyChanged),
        ]
    );

    // Redoing on editor 1 will emit an `Edited` event only for that editor.
    editor1.update(cx, |editor, cx| editor.redo(&Redo, cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor1", Event::Edited),
            ("editor1", Event::BufferEdited),
            ("editor2", Event::BufferEdited),
            ("editor1", Event::DirtyChanged),
            ("editor2", Event::DirtyChanged),
        ]
    );

    // Undoing on editor 2 will emit an `Edited` event only for that editor.
    editor2.update(cx, |editor, cx| editor.undo(&Undo, cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor2", Event::Edited),
            ("editor1", Event::BufferEdited),
            ("editor2", Event::BufferEdited),
            ("editor1", Event::DirtyChanged),
            ("editor2", Event::DirtyChanged),
        ]
    );

    // Redoing on editor 2 will emit an `Edited` event only for that editor.
    editor2.update(cx, |editor, cx| editor.redo(&Redo, cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor2", Event::Edited),
            ("editor1", Event::BufferEdited),
            ("editor2", Event::BufferEdited),
            ("editor1", Event::DirtyChanged),
            ("editor2", Event::DirtyChanged),
        ]
    );

    // No event is emitted when the mutation is a no-op.
    editor2.update(cx, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([0..0]));

        editor.backspace(&Backspace, cx);
    });
    assert_eq!(mem::take(&mut *events.borrow_mut()), []);
}

#[gpui::test]
fn test_undo_redo_with_selection_restoration(cx: &mut MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let mut now = Instant::now();
    let buffer = cx.add_model(|cx| language::Buffer::new(0, "123456", cx));
    let group_interval = buffer.read(cx).transaction_group_interval();
    let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (_, editor) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));

    editor.update(cx, |editor, cx| {
        editor.start_transaction_at(now, cx);
        editor.change_selections(None, cx, |s| s.select_ranges([2..4]));

        editor.insert("cd", cx);
        editor.end_transaction_at(now, cx);
        assert_eq!(editor.text(cx), "12cd56");
        assert_eq!(editor.selections.ranges(cx), vec![4..4]);

        editor.start_transaction_at(now, cx);
        editor.change_selections(None, cx, |s| s.select_ranges([4..5]));
        editor.insert("e", cx);
        editor.end_transaction_at(now, cx);
        assert_eq!(editor.text(cx), "12cde6");
        assert_eq!(editor.selections.ranges(cx), vec![5..5]);

        now += group_interval + Duration::from_millis(1);
        editor.change_selections(None, cx, |s| s.select_ranges([2..2]));

        // Simulate an edit in another editor
        buffer.update(cx, |buffer, cx| {
            buffer.start_transaction_at(now, cx);
            buffer.edit([(0..1, "a")], None, cx);
            buffer.edit([(1..1, "b")], None, cx);
            buffer.end_transaction_at(now, cx);
        });

        assert_eq!(editor.text(cx), "ab2cde6");
        assert_eq!(editor.selections.ranges(cx), vec![3..3]);

        // Last transaction happened past the group interval in a different editor.
        // Undo it individually and don't restore selections.
        editor.undo(&Undo, cx);
        assert_eq!(editor.text(cx), "12cde6");
        assert_eq!(editor.selections.ranges(cx), vec![2..2]);

        // First two transactions happened within the group interval in this editor.
        // Undo them together and restore selections.
        editor.undo(&Undo, cx);
        editor.undo(&Undo, cx); // Undo stack is empty here, so this is a no-op.
        assert_eq!(editor.text(cx), "123456");
        assert_eq!(editor.selections.ranges(cx), vec![0..0]);

        // Redo the first two transactions together.
        editor.redo(&Redo, cx);
        assert_eq!(editor.text(cx), "12cde6");
        assert_eq!(editor.selections.ranges(cx), vec![5..5]);

        // Redo the last transaction on its own.
        editor.redo(&Redo, cx);
        assert_eq!(editor.text(cx), "ab2cde6");
        assert_eq!(editor.selections.ranges(cx), vec![6..6]);

        // Test empty transactions.
        editor.start_transaction_at(now, cx);
        editor.end_transaction_at(now, cx);
        editor.undo(&Undo, cx);
        assert_eq!(editor.text(cx), "12cde6");
    });
}

#[gpui::test]
fn test_ime_composition(cx: &mut MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = cx.add_model(|cx| {
        let mut buffer = language::Buffer::new(0, "abcde", cx);
        // Ensure automatic grouping doesn't occur.
        buffer.set_group_interval(Duration::ZERO);
        buffer
    });

    let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
    cx.add_window(Default::default(), |cx| {
        let mut editor = build_editor(buffer.clone(), cx);

        // Start a new IME composition.
        editor.replace_and_mark_text_in_range(Some(0..1), "à", None, cx);
        editor.replace_and_mark_text_in_range(Some(0..1), "á", None, cx);
        editor.replace_and_mark_text_in_range(Some(0..1), "ä", None, cx);
        assert_eq!(editor.text(cx), "äbcde");
        assert_eq!(
            editor.marked_text_ranges(cx),
            Some(vec![OffsetUtf16(0)..OffsetUtf16(1)])
        );

        // Finalize IME composition.
        editor.replace_text_in_range(None, "ā", cx);
        assert_eq!(editor.text(cx), "ābcde");
        assert_eq!(editor.marked_text_ranges(cx), None);

        // IME composition edits are grouped and are undone/redone at once.
        editor.undo(&Default::default(), cx);
        assert_eq!(editor.text(cx), "abcde");
        assert_eq!(editor.marked_text_ranges(cx), None);
        editor.redo(&Default::default(), cx);
        assert_eq!(editor.text(cx), "ābcde");
        assert_eq!(editor.marked_text_ranges(cx), None);

        // Start a new IME composition.
        editor.replace_and_mark_text_in_range(Some(0..1), "à", None, cx);
        assert_eq!(
            editor.marked_text_ranges(cx),
            Some(vec![OffsetUtf16(0)..OffsetUtf16(1)])
        );

        // Undoing during an IME composition cancels it.
        editor.undo(&Default::default(), cx);
        assert_eq!(editor.text(cx), "ābcde");
        assert_eq!(editor.marked_text_ranges(cx), None);

        // Start a new IME composition with an invalid marked range, ensuring it gets clipped.
        editor.replace_and_mark_text_in_range(Some(4..999), "è", None, cx);
        assert_eq!(editor.text(cx), "ābcdè");
        assert_eq!(
            editor.marked_text_ranges(cx),
            Some(vec![OffsetUtf16(4)..OffsetUtf16(5)])
        );

        // Finalize IME composition with an invalid replacement range, ensuring it gets clipped.
        editor.replace_text_in_range(Some(4..999), "ę", cx);
        assert_eq!(editor.text(cx), "ābcdę");
        assert_eq!(editor.marked_text_ranges(cx), None);

        // Start a new IME composition with multiple cursors.
        editor.change_selections(None, cx, |s| {
            s.select_ranges([
                OffsetUtf16(1)..OffsetUtf16(1),
                OffsetUtf16(3)..OffsetUtf16(3),
                OffsetUtf16(5)..OffsetUtf16(5),
            ])
        });
        editor.replace_and_mark_text_in_range(Some(4..5), "XYZ", None, cx);
        assert_eq!(editor.text(cx), "XYZbXYZdXYZ");
        assert_eq!(
            editor.marked_text_ranges(cx),
            Some(vec![
                OffsetUtf16(0)..OffsetUtf16(3),
                OffsetUtf16(4)..OffsetUtf16(7),
                OffsetUtf16(8)..OffsetUtf16(11)
            ])
        );

        // Ensure the newly-marked range gets treated as relative to the previously-marked ranges.
        editor.replace_and_mark_text_in_range(Some(1..2), "1", None, cx);
        assert_eq!(editor.text(cx), "X1ZbX1ZdX1Z");
        assert_eq!(
            editor.marked_text_ranges(cx),
            Some(vec![
                OffsetUtf16(1)..OffsetUtf16(2),
                OffsetUtf16(5)..OffsetUtf16(6),
                OffsetUtf16(9)..OffsetUtf16(10)
            ])
        );

        // Finalize IME composition with multiple cursors.
        editor.replace_text_in_range(Some(9..10), "2", cx);
        assert_eq!(editor.text(cx), "X2ZbX2ZdX2Z");
        assert_eq!(editor.marked_text_ranges(cx), None);

        editor
    });
}

#[gpui::test]
fn test_selection_with_mouse(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));

    let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\nddddddd\n", cx);
    let (_, editor) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
    editor.update(cx, |view, cx| {
        view.begin_selection(DisplayPoint::new(2, 2), false, 1, cx);
    });
    assert_eq!(
        editor.update(cx, |view, cx| view.selections.display_ranges(cx)),
        [DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2)]
    );

    editor.update(cx, |view, cx| {
        view.update_selection(DisplayPoint::new(3, 3), 0, Vector2F::zero(), cx);
    });

    assert_eq!(
        editor.update(cx, |view, cx| view.selections.display_ranges(cx)),
        [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
    );

    editor.update(cx, |view, cx| {
        view.update_selection(DisplayPoint::new(1, 1), 0, Vector2F::zero(), cx);
    });

    assert_eq!(
        editor.update(cx, |view, cx| view.selections.display_ranges(cx)),
        [DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1)]
    );

    editor.update(cx, |view, cx| {
        view.end_selection(cx);
        view.update_selection(DisplayPoint::new(3, 3), 0, Vector2F::zero(), cx);
    });

    assert_eq!(
        editor.update(cx, |view, cx| view.selections.display_ranges(cx)),
        [DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1)]
    );

    editor.update(cx, |view, cx| {
        view.begin_selection(DisplayPoint::new(3, 3), true, 1, cx);
        view.update_selection(DisplayPoint::new(0, 0), 0, Vector2F::zero(), cx);
    });

    assert_eq!(
        editor.update(cx, |view, cx| view.selections.display_ranges(cx)),
        [
            DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1),
            DisplayPoint::new(3, 3)..DisplayPoint::new(0, 0)
        ]
    );

    editor.update(cx, |view, cx| {
        view.end_selection(cx);
    });

    assert_eq!(
        editor.update(cx, |view, cx| view.selections.display_ranges(cx)),
        [DisplayPoint::new(3, 3)..DisplayPoint::new(0, 0)]
    );
}

#[gpui::test]
fn test_canceling_pending_selection(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));

    view.update(cx, |view, cx| {
        view.begin_selection(DisplayPoint::new(2, 2), false, 1, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2)]
        );
    });

    view.update(cx, |view, cx| {
        view.update_selection(DisplayPoint::new(3, 3), 0, Vector2F::zero(), cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
        );
    });

    view.update(cx, |view, cx| {
        view.cancel(&Cancel, cx);
        view.update_selection(DisplayPoint::new(1, 1), 0, Vector2F::zero(), cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
        );
    });
}

#[gpui::test]
fn test_clone(cx: &mut gpui::MutableAppContext) {
    let (text, selection_ranges) = marked_text_ranges(
        indoc! {"
            one
            two
            threeˇ
            four
            fiveˇ
        "},
        true,
    );
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple(&text, cx);

    let (_, editor) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));

    editor.update(cx, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges(selection_ranges.clone()));
        editor.fold_ranges(
            [
                Point::new(1, 0)..Point::new(2, 0),
                Point::new(3, 0)..Point::new(4, 0),
            ],
            true,
            cx,
        );
    });

    let (_, cloned_editor) = editor.update(cx, |editor, cx| {
        cx.add_window(Default::default(), |cx| editor.clone(cx))
    });

    let snapshot = editor.update(cx, |e, cx| e.snapshot(cx));
    let cloned_snapshot = cloned_editor.update(cx, |e, cx| e.snapshot(cx));

    assert_eq!(
        cloned_editor.update(cx, |e, cx| e.display_text(cx)),
        editor.update(cx, |e, cx| e.display_text(cx))
    );
    assert_eq!(
        cloned_snapshot
            .folds_in_range(0..text.len())
            .collect::<Vec<_>>(),
        snapshot.folds_in_range(0..text.len()).collect::<Vec<_>>(),
    );
    assert_set_eq!(
        cloned_editor.read(cx).selections.ranges::<Point>(cx),
        editor.read(cx).selections.ranges(cx)
    );
    assert_set_eq!(
        cloned_editor.update(cx, |e, cx| e.selections.display_ranges(cx)),
        editor.update(cx, |e, cx| e.selections.display_ranges(cx))
    );
}

#[gpui::test]
fn test_navigation_history(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    cx.set_global(DragAndDrop::<Workspace>::default());
    use workspace::item::Item;
    let (_, pane) = cx.add_window(Default::default(), |cx| Pane::new(0, None, || &[], cx));
    let buffer = MultiBuffer::build_simple(&sample_text(300, 5, 'a'), cx);

    cx.add_view(&pane, |cx| {
        let mut editor = build_editor(buffer.clone(), cx);
        let handle = cx.handle();
        editor.set_nav_history(Some(pane.read(cx).nav_history_for_item(&handle)));

        fn pop_history(editor: &mut Editor, cx: &mut MutableAppContext) -> Option<NavigationEntry> {
            editor.nav_history.as_mut().unwrap().pop_backward(cx)
        }

        // Move the cursor a small distance.
        // Nothing is added to the navigation history.
        editor.change_selections(None, cx, |s| {
            s.select_display_ranges([DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)])
        });
        editor.change_selections(None, cx, |s| {
            s.select_display_ranges([DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0)])
        });
        assert!(pop_history(&mut editor, cx).is_none());

        // Move the cursor a large distance.
        // The history can jump back to the previous position.
        editor.change_selections(None, cx, |s| {
            s.select_display_ranges([DisplayPoint::new(13, 0)..DisplayPoint::new(13, 3)])
        });
        let nav_entry = pop_history(&mut editor, cx).unwrap();
        editor.navigate(nav_entry.data.unwrap(), cx);
        assert_eq!(nav_entry.item.id(), cx.view_id());
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0)]
        );
        assert!(pop_history(&mut editor, cx).is_none());

        // Move the cursor a small distance via the mouse.
        // Nothing is added to the navigation history.
        editor.begin_selection(DisplayPoint::new(5, 0), false, 1, cx);
        editor.end_selection(cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(5, 0)..DisplayPoint::new(5, 0)]
        );
        assert!(pop_history(&mut editor, cx).is_none());

        // Move the cursor a large distance via the mouse.
        // The history can jump back to the previous position.
        editor.begin_selection(DisplayPoint::new(15, 0), false, 1, cx);
        editor.end_selection(cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(15, 0)..DisplayPoint::new(15, 0)]
        );
        let nav_entry = pop_history(&mut editor, cx).unwrap();
        editor.navigate(nav_entry.data.unwrap(), cx);
        assert_eq!(nav_entry.item.id(), cx.view_id());
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(5, 0)..DisplayPoint::new(5, 0)]
        );
        assert!(pop_history(&mut editor, cx).is_none());

        // Set scroll position to check later
        editor.set_scroll_position(Vector2F::new(5.5, 5.5), cx);
        let original_scroll_position = editor.scroll_manager.anchor();

        // Jump to the end of the document and adjust scroll
        editor.move_to_end(&MoveToEnd, cx);
        editor.set_scroll_position(Vector2F::new(-2.5, -0.5), cx);
        assert_ne!(editor.scroll_manager.anchor(), original_scroll_position);

        let nav_entry = pop_history(&mut editor, cx).unwrap();
        editor.navigate(nav_entry.data.unwrap(), cx);
        assert_eq!(editor.scroll_manager.anchor(), original_scroll_position);

        // Ensure we don't panic when navigation data contains invalid anchors *and* points.
        let mut invalid_anchor = editor.scroll_manager.anchor().top_anchor;
        invalid_anchor.text_anchor.buffer_id = Some(999);
        let invalid_point = Point::new(9999, 0);
        editor.navigate(
            Box::new(NavigationData {
                cursor_anchor: invalid_anchor,
                cursor_position: invalid_point,
                scroll_anchor: ScrollAnchor {
                    top_anchor: invalid_anchor,
                    offset: Default::default(),
                },
                scroll_top_row: invalid_point.row,
            }),
            cx,
        );
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[editor.max_point(cx)..editor.max_point(cx)]
        );
        assert_eq!(
            editor.scroll_position(cx),
            vec2f(0., editor.max_point(cx).row() as f32)
        );

        editor
    });
}

#[gpui::test]
fn test_cancel(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));

    view.update(cx, |view, cx| {
        view.begin_selection(DisplayPoint::new(3, 4), false, 1, cx);
        view.update_selection(DisplayPoint::new(1, 1), 0, Vector2F::zero(), cx);
        view.end_selection(cx);

        view.begin_selection(DisplayPoint::new(0, 1), true, 1, cx);
        view.update_selection(DisplayPoint::new(0, 3), 0, Vector2F::zero(), cx);
        view.end_selection(cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            [
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 3),
                DisplayPoint::new(3, 4)..DisplayPoint::new(1, 1),
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.cancel(&Cancel, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            [DisplayPoint::new(3, 4)..DisplayPoint::new(1, 1)]
        );
    });

    view.update(cx, |view, cx| {
        view.cancel(&Cancel, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            [DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1)]
        );
    });
}

#[gpui::test]
fn test_fold_action(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple(
        &"
            impl Foo {
                // Hello!

                fn a() {
                    1
                }

                fn b() {
                    2
                }

                fn c() {
                    3
                }
            }
        "
        .unindent(),
        cx,
    );
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));

    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([DisplayPoint::new(8, 0)..DisplayPoint::new(12, 0)]);
        });
        view.fold(&Fold, cx);
        assert_eq!(
            view.display_text(cx),
            "
                impl Foo {
                    // Hello!

                    fn a() {
                        1
                    }

                    fn b() {⋯
                    }

                    fn c() {⋯
                    }
                }
            "
            .unindent(),
        );

        view.fold(&Fold, cx);
        assert_eq!(
            view.display_text(cx),
            "
                impl Foo {⋯
                }
            "
            .unindent(),
        );

        view.unfold_lines(&UnfoldLines, cx);
        assert_eq!(
            view.display_text(cx),
            "
                impl Foo {
                    // Hello!

                    fn a() {
                        1
                    }

                    fn b() {⋯
                    }

                    fn c() {⋯
                    }
                }
            "
            .unindent(),
        );

        view.unfold_lines(&UnfoldLines, cx);
        assert_eq!(view.display_text(cx), buffer.read(cx).read(cx).text());
    });
}

#[gpui::test]
fn test_move_cursor(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple(&sample_text(6, 6, 'a'), cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));

    buffer.update(cx, |buffer, cx| {
        buffer.edit(
            vec![
                (Point::new(1, 0)..Point::new(1, 0), "\t"),
                (Point::new(1, 1)..Point::new(1, 1), "\t"),
            ],
            None,
            cx,
        );
    });

    view.update(cx, |view, cx| {
        assert_eq!(
            view.selections.display_ranges(cx),
            &[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)]
        );

        view.move_down(&MoveDown, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)]
        );

        view.move_right(&MoveRight, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4)]
        );

        view.move_left(&MoveLeft, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)]
        );

        view.move_up(&MoveUp, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)]
        );

        view.move_to_end(&MoveToEnd, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[DisplayPoint::new(5, 6)..DisplayPoint::new(5, 6)]
        );

        view.move_to_beginning(&MoveToBeginning, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0)]
        );

        view.change_selections(None, cx, |s| {
            s.select_display_ranges([DisplayPoint::new(0, 1)..DisplayPoint::new(0, 2)]);
        });
        view.select_to_beginning(&SelectToBeginning, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[DisplayPoint::new(0, 1)..DisplayPoint::new(0, 0)]
        );

        view.select_to_end(&SelectToEnd, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[DisplayPoint::new(0, 1)..DisplayPoint::new(5, 6)]
        );
    });
}

#[gpui::test]
fn test_move_cursor_multibyte(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple("ⓐⓑⓒⓓⓔ\nabcde\nαβγδε\n", cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));

    assert_eq!('ⓐ'.len_utf8(), 3);
    assert_eq!('α'.len_utf8(), 2);

    view.update(cx, |view, cx| {
        view.fold_ranges(
            vec![
                Point::new(0, 6)..Point::new(0, 12),
                Point::new(1, 2)..Point::new(1, 4),
                Point::new(2, 4)..Point::new(2, 8),
            ],
            true,
            cx,
        );
        assert_eq!(view.display_text(cx), "ⓐⓑ⋯ⓔ\nab⋯e\nαβ⋯ε\n");

        view.move_right(&MoveRight, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(0, "ⓐ".len())]
        );
        view.move_right(&MoveRight, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(0, "ⓐⓑ".len())]
        );
        view.move_right(&MoveRight, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(0, "ⓐⓑ⋯".len())]
        );

        view.move_down(&MoveDown, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(1, "ab⋯".len())]
        );
        view.move_left(&MoveLeft, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(1, "ab".len())]
        );
        view.move_left(&MoveLeft, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(1, "a".len())]
        );

        view.move_down(&MoveDown, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(2, "α".len())]
        );
        view.move_right(&MoveRight, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(2, "αβ".len())]
        );
        view.move_right(&MoveRight, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(2, "αβ⋯".len())]
        );
        view.move_right(&MoveRight, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(2, "αβ⋯ε".len())]
        );

        view.move_up(&MoveUp, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(1, "ab⋯e".len())]
        );
        view.move_up(&MoveUp, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(0, "ⓐⓑ⋯ⓔ".len())]
        );
        view.move_left(&MoveLeft, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(0, "ⓐⓑ⋯".len())]
        );
        view.move_left(&MoveLeft, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(0, "ⓐⓑ".len())]
        );
        view.move_left(&MoveLeft, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(0, "ⓐ".len())]
        );
    });
}

#[gpui::test]
fn test_move_cursor_different_line_lengths(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple("ⓐⓑⓒⓓⓔ\nabcd\nαβγ\nabcd\nⓐⓑⓒⓓⓔ\n", cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));
    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([empty_range(0, "ⓐⓑⓒⓓⓔ".len())]);
        });
        view.move_down(&MoveDown, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(1, "abcd".len())]
        );

        view.move_down(&MoveDown, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(2, "αβγ".len())]
        );

        view.move_down(&MoveDown, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(3, "abcd".len())]
        );

        view.move_down(&MoveDown, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(4, "ⓐⓑⓒⓓⓔ".len())]
        );

        view.move_up(&MoveUp, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(3, "abcd".len())]
        );

        view.move_up(&MoveUp, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(2, "αβγ".len())]
        );
    });
}

#[gpui::test]
fn test_beginning_end_of_line(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple("abc\n  def", cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4),
            ]);
        });
    });

    view.update(cx, |view, cx| {
        view.move_to_beginning_of_line(&MoveToBeginningOfLine, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.move_to_beginning_of_line(&MoveToBeginningOfLine, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.move_to_beginning_of_line(&MoveToBeginningOfLine, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.move_to_end_of_line(&MoveToEndOfLine, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[
                DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 5)..DisplayPoint::new(1, 5),
            ]
        );
    });

    // Moving to the end of line again is a no-op.
    view.update(cx, |view, cx| {
        view.move_to_end_of_line(&MoveToEndOfLine, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[
                DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 5)..DisplayPoint::new(1, 5),
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.move_left(&MoveLeft, cx);
        view.select_to_beginning_of_line(
            &SelectToBeginningOfLine {
                stop_at_soft_wraps: true,
            },
            cx,
        );
        assert_eq!(
            view.selections.display_ranges(cx),
            &[
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 2),
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.select_to_beginning_of_line(
            &SelectToBeginningOfLine {
                stop_at_soft_wraps: true,
            },
            cx,
        );
        assert_eq!(
            view.selections.display_ranges(cx),
            &[
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 0),
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.select_to_beginning_of_line(
            &SelectToBeginningOfLine {
                stop_at_soft_wraps: true,
            },
            cx,
        );
        assert_eq!(
            view.selections.display_ranges(cx),
            &[
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 0),
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 2),
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.select_to_end_of_line(
            &SelectToEndOfLine {
                stop_at_soft_wraps: true,
            },
            cx,
        );
        assert_eq!(
            view.selections.display_ranges(cx),
            &[
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 5),
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.delete_to_end_of_line(&DeleteToEndOfLine, cx);
        assert_eq!(view.display_text(cx), "ab\n  de");
        assert_eq!(
            view.selections.display_ranges(cx),
            &[
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4),
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.delete_to_beginning_of_line(&DeleteToBeginningOfLine, cx);
        assert_eq!(view.display_text(cx), "\n");
        assert_eq!(
            view.selections.display_ranges(cx),
            &[
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
            ]
        );
    });
}

#[gpui::test]
fn test_prev_next_word_boundary(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple("use std::str::{foo, bar}\n\n  {baz.qux()}", cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(0, 11)..DisplayPoint::new(0, 11),
                DisplayPoint::new(2, 4)..DisplayPoint::new(2, 4),
            ])
        });

        view.move_to_previous_word_start(&MoveToPreviousWordStart, cx);
        assert_selection_ranges("use std::ˇstr::{foo, bar}\n\n  {ˇbaz.qux()}", view, cx);

        view.move_to_previous_word_start(&MoveToPreviousWordStart, cx);
        assert_selection_ranges("use stdˇ::str::{foo, bar}\n\n  ˇ{baz.qux()}", view, cx);

        view.move_to_previous_word_start(&MoveToPreviousWordStart, cx);
        assert_selection_ranges("use ˇstd::str::{foo, bar}\n\nˇ  {baz.qux()}", view, cx);

        view.move_to_previous_word_start(&MoveToPreviousWordStart, cx);
        assert_selection_ranges("ˇuse std::str::{foo, bar}\nˇ\n  {baz.qux()}", view, cx);

        view.move_to_previous_word_start(&MoveToPreviousWordStart, cx);
        assert_selection_ranges("ˇuse std::str::{foo, barˇ}\n\n  {baz.qux()}", view, cx);

        view.move_to_next_word_end(&MoveToNextWordEnd, cx);
        assert_selection_ranges("useˇ std::str::{foo, bar}ˇ\n\n  {baz.qux()}", view, cx);

        view.move_to_next_word_end(&MoveToNextWordEnd, cx);
        assert_selection_ranges("use stdˇ::str::{foo, bar}\nˇ\n  {baz.qux()}", view, cx);

        view.move_to_next_word_end(&MoveToNextWordEnd, cx);
        assert_selection_ranges("use std::ˇstr::{foo, bar}\n\n  {ˇbaz.qux()}", view, cx);

        view.move_right(&MoveRight, cx);
        view.select_to_previous_word_start(&SelectToPreviousWordStart, cx);
        assert_selection_ranges("use std::«ˇs»tr::{foo, bar}\n\n  {«ˇb»az.qux()}", view, cx);

        view.select_to_previous_word_start(&SelectToPreviousWordStart, cx);
        assert_selection_ranges("use std«ˇ::s»tr::{foo, bar}\n\n  «ˇ{b»az.qux()}", view, cx);

        view.select_to_next_word_end(&SelectToNextWordEnd, cx);
        assert_selection_ranges("use std::«ˇs»tr::{foo, bar}\n\n  {«ˇb»az.qux()}", view, cx);
    });
}

#[gpui::test]
fn test_prev_next_word_bounds_with_soft_wrap(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple("use one::{\n    two::three::four::five\n};", cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));

    view.update(cx, |view, cx| {
        view.set_wrap_width(Some(140.), cx);
        assert_eq!(
            view.display_text(cx),
            "use one::{\n    two::three::\n    four::five\n};"
        );

        view.change_selections(None, cx, |s| {
            s.select_display_ranges([DisplayPoint::new(1, 7)..DisplayPoint::new(1, 7)]);
        });

        view.move_to_next_word_end(&MoveToNextWordEnd, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[DisplayPoint::new(1, 9)..DisplayPoint::new(1, 9)]
        );

        view.move_to_next_word_end(&MoveToNextWordEnd, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[DisplayPoint::new(1, 14)..DisplayPoint::new(1, 14)]
        );

        view.move_to_next_word_end(&MoveToNextWordEnd, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[DisplayPoint::new(2, 4)..DisplayPoint::new(2, 4)]
        );

        view.move_to_next_word_end(&MoveToNextWordEnd, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[DisplayPoint::new(2, 8)..DisplayPoint::new(2, 8)]
        );

        view.move_to_previous_word_start(&MoveToPreviousWordStart, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[DisplayPoint::new(2, 4)..DisplayPoint::new(2, 4)]
        );

        view.move_to_previous_word_start(&MoveToPreviousWordStart, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[DisplayPoint::new(1, 14)..DisplayPoint::new(1, 14)]
        );
    });
}

#[gpui::test]
async fn test_move_page_up_page_down(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);

    let line_height = cx.editor(|editor, cx| editor.style(cx).text.line_height(cx.font_cache()));
    cx.simulate_window_resize(cx.window_id, vec2f(100., 4. * line_height));

    cx.set_state(
        &r#"
        ˇone
        two
        threeˇ
        four
        five
        six
        seven
        eight
        nine
        ten
        "#
        .unindent(),
    );

    cx.update_editor(|editor, cx| editor.move_page_down(&MovePageDown::default(), cx));
    cx.assert_editor_state(
        &r#"
        one
        two
        three
        ˇfour
        five
        sixˇ
        seven
        eight
        nine
        ten
        "#
        .unindent(),
    );

    cx.update_editor(|editor, cx| editor.move_page_down(&MovePageDown::default(), cx));
    cx.assert_editor_state(
        &r#"
        one
        two
        three
        four
        five
        six
        ˇseven
        eight
        nineˇ
        ten
        "#
        .unindent(),
    );

    cx.update_editor(|editor, cx| editor.move_page_up(&MovePageUp::default(), cx));
    cx.assert_editor_state(
        &r#"
        one
        two
        three
        ˇfour
        five
        sixˇ
        seven
        eight
        nine
        ten
        "#
        .unindent(),
    );

    cx.update_editor(|editor, cx| editor.move_page_up(&MovePageUp::default(), cx));
    cx.assert_editor_state(
        &r#"
        ˇone
        two
        threeˇ
        four
        five
        six
        seven
        eight
        nine
        ten
        "#
        .unindent(),
    );

    // Test select collapsing
    cx.update_editor(|editor, cx| {
        editor.move_page_down(&MovePageDown::default(), cx);
        editor.move_page_down(&MovePageDown::default(), cx);
        editor.move_page_down(&MovePageDown::default(), cx);
    });
    cx.assert_editor_state(
        &r#"
        one
        two
        three
        four
        five
        six
        seven
        eight
        nine
        ˇten
        ˇ"#
        .unindent(),
    );
}

#[gpui::test]
async fn test_delete_to_beginning_of_line(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);
    cx.set_state("one «two threeˇ» four");
    cx.update_editor(|editor, cx| {
        editor.delete_to_beginning_of_line(&DeleteToBeginningOfLine, cx);
        assert_eq!(editor.text(cx), " four");
    });
}

#[gpui::test]
fn test_delete_to_word_boundary(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple("one two three four", cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));

    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([
                // an empty selection - the preceding word fragment is deleted
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                // characters selected - they are deleted
                DisplayPoint::new(0, 9)..DisplayPoint::new(0, 12),
            ])
        });
        view.delete_to_previous_word_start(&DeleteToPreviousWordStart, cx);
    });

    assert_eq!(buffer.read(cx).read(cx).text(), "e two te four");

    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([
                // an empty selection - the following word fragment is deleted
                DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                // characters selected - they are deleted
                DisplayPoint::new(0, 9)..DisplayPoint::new(0, 10),
            ])
        });
        view.delete_to_next_word_end(&DeleteToNextWordEnd, cx);
    });

    assert_eq!(buffer.read(cx).read(cx).text(), "e t te our");
}

#[gpui::test]
fn test_newline(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple("aaaa\n    bbbb\n", cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));

    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
                DisplayPoint::new(1, 6)..DisplayPoint::new(1, 6),
            ])
        });

        view.newline(&Newline, cx);
        assert_eq!(view.text(cx), "aa\naa\n  \n    bb\n    bb\n");
    });
}

#[gpui::test]
fn test_newline_with_old_selections(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple(
        "
            a
            b(
                X
            )
            c(
                X
            )
        "
        .unindent()
        .as_str(),
        cx,
    );

    let (_, editor) = cx.add_window(Default::default(), |cx| {
        let mut editor = build_editor(buffer.clone(), cx);
        editor.change_selections(None, cx, |s| {
            s.select_ranges([
                Point::new(2, 4)..Point::new(2, 5),
                Point::new(5, 4)..Point::new(5, 5),
            ])
        });
        editor
    });

    // Edit the buffer directly, deleting ranges surrounding the editor's selections
    buffer.update(cx, |buffer, cx| {
        buffer.edit(
            [
                (Point::new(1, 2)..Point::new(3, 0), ""),
                (Point::new(4, 2)..Point::new(6, 0), ""),
            ],
            None,
            cx,
        );
        assert_eq!(
            buffer.read(cx).text(),
            "
                a
                b()
                c()
            "
            .unindent()
        );
    });

    editor.update(cx, |editor, cx| {
        assert_eq!(
            editor.selections.ranges(cx),
            &[
                Point::new(1, 2)..Point::new(1, 2),
                Point::new(2, 2)..Point::new(2, 2),
            ],
        );

        editor.newline(&Newline, cx);
        assert_eq!(
            editor.text(cx),
            "
                a
                b(
                )
                c(
                )
            "
            .unindent()
        );

        // The selections are moved after the inserted newlines
        assert_eq!(
            editor.selections.ranges(cx),
            &[
                Point::new(2, 0)..Point::new(2, 0),
                Point::new(4, 0)..Point::new(4, 0),
            ],
        );
    });
}

#[gpui::test]
async fn test_newline_below(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);
    cx.update(|cx| {
        cx.update_global::<Settings, _, _>(|settings, _| {
            settings.editor_overrides.tab_size = Some(NonZeroU32::new(4).unwrap());
        });
    });

    let language = Arc::new(
        Language::new(
            LanguageConfig::default(),
            Some(tree_sitter_rust::language()),
        )
        .with_indents_query(r#"(_ "(" ")" @end) @indent"#)
        .unwrap(),
    );
    cx.update_buffer(|buffer, cx| buffer.set_language(Some(language), cx));

    cx.set_state(indoc! {"
        const a: ˇA = (
            (ˇ
                «const_functionˇ»(ˇ),
                so«mˇ»et«hˇ»ing_ˇelse,ˇ
            )ˇ
        ˇ);ˇ
    "});
    cx.update_editor(|e, cx| e.newline_below(&NewlineBelow, cx));
    cx.assert_editor_state(indoc! {"
        const a: A = (
            ˇ
            (
                ˇ
                const_function(),
                ˇ
                ˇ
                something_else,
                ˇ
                ˇ
                ˇ
                ˇ
            )
            ˇ
        );
        ˇ
        ˇ
    "});
}

#[gpui::test]
fn test_insert_with_old_selections(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple("a( X ), b( Y ), c( Z )", cx);
    let (_, editor) = cx.add_window(Default::default(), |cx| {
        let mut editor = build_editor(buffer.clone(), cx);
        editor.change_selections(None, cx, |s| s.select_ranges([3..4, 11..12, 19..20]));
        editor
    });

    // Edit the buffer directly, deleting ranges surrounding the editor's selections
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(2..5, ""), (10..13, ""), (18..21, "")], None, cx);
        assert_eq!(buffer.read(cx).text(), "a(), b(), c()".unindent());
    });

    editor.update(cx, |editor, cx| {
        assert_eq!(editor.selections.ranges(cx), &[2..2, 7..7, 12..12],);

        editor.insert("Z", cx);
        assert_eq!(editor.text(cx), "a(Z), b(Z), c(Z)");

        // The selections are moved after the inserted characters
        assert_eq!(editor.selections.ranges(cx), &[3..3, 9..9, 15..15],);
    });
}

#[gpui::test]
async fn test_tab(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);
    cx.update(|cx| {
        cx.update_global::<Settings, _, _>(|settings, _| {
            settings.editor_overrides.tab_size = Some(NonZeroU32::new(3).unwrap());
        });
    });
    cx.set_state(indoc! {"
        ˇabˇc
        ˇ🏀ˇ🏀ˇefg
        dˇ
    "});
    cx.update_editor(|e, cx| e.tab(&Tab, cx));
    cx.assert_editor_state(indoc! {"
           ˇab ˇc
           ˇ🏀  ˇ🏀  ˇefg
        d  ˇ
    "});

    cx.set_state(indoc! {"
        a
        «🏀ˇ»🏀«🏀ˇ»🏀«🏀ˇ»
    "});
    cx.update_editor(|e, cx| e.tab(&Tab, cx));
    cx.assert_editor_state(indoc! {"
        a
           «🏀ˇ»🏀«🏀ˇ»🏀«🏀ˇ»
    "});
}

#[gpui::test]
async fn test_tab_in_leading_whitespace_auto_indents_lines(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);
    let language = Arc::new(
        Language::new(
            LanguageConfig::default(),
            Some(tree_sitter_rust::language()),
        )
        .with_indents_query(r#"(_ "(" ")" @end) @indent"#)
        .unwrap(),
    );
    cx.update_buffer(|buffer, cx| buffer.set_language(Some(language), cx));

    // cursors that are already at the suggested indent level insert
    // a soft tab. cursors that are to the left of the suggested indent
    // auto-indent their line.
    cx.set_state(indoc! {"
        ˇ
        const a: B = (
            c(
                d(
        ˇ
                )
        ˇ
        ˇ    )
        );
    "});
    cx.update_editor(|e, cx| e.tab(&Tab, cx));
    cx.assert_editor_state(indoc! {"
            ˇ
        const a: B = (
            c(
                d(
                    ˇ
                )
                ˇ
            ˇ)
        );
    "});

    // handle auto-indent when there are multiple cursors on the same line
    cx.set_state(indoc! {"
        const a: B = (
            c(
        ˇ    ˇ
        ˇ    )
        );
    "});
    cx.update_editor(|e, cx| e.tab(&Tab, cx));
    cx.assert_editor_state(indoc! {"
        const a: B = (
            c(
                ˇ
            ˇ)
        );
    "});
}

#[gpui::test]
async fn test_tab_with_mixed_whitespace(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);
    let language = Arc::new(
        Language::new(
            LanguageConfig::default(),
            Some(tree_sitter_rust::language()),
        )
        .with_indents_query(r#"(_ "{" "}" @end) @indent"#)
        .unwrap(),
    );
    cx.update_buffer(|buffer, cx| buffer.set_language(Some(language), cx));

    cx.update(|cx| {
        cx.update_global::<Settings, _, _>(|settings, _| {
            settings.editor_overrides.tab_size = Some(4.try_into().unwrap());
        });
    });

    cx.set_state(indoc! {"
        fn a() {
            if b {
        \t ˇc
            }
        }
    "});

    cx.update_editor(|e, cx| e.tab(&Tab, cx));
    cx.assert_editor_state(indoc! {"
        fn a() {
            if b {
                ˇc
            }
        }
    "});
}

#[gpui::test]
async fn test_indent_outdent(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);

    cx.set_state(indoc! {"
          «oneˇ» «twoˇ»
        three
         four
    "});
    cx.update_editor(|e, cx| e.tab(&Tab, cx));
    cx.assert_editor_state(indoc! {"
            «oneˇ» «twoˇ»
        three
         four
    "});

    cx.update_editor(|e, cx| e.tab_prev(&TabPrev, cx));
    cx.assert_editor_state(indoc! {"
        «oneˇ» «twoˇ»
        three
         four
    "});

    // select across line ending
    cx.set_state(indoc! {"
        one two
        t«hree
        ˇ» four
    "});
    cx.update_editor(|e, cx| e.tab(&Tab, cx));
    cx.assert_editor_state(indoc! {"
        one two
            t«hree
        ˇ» four
    "});

    cx.update_editor(|e, cx| e.tab_prev(&TabPrev, cx));
    cx.assert_editor_state(indoc! {"
        one two
        t«hree
        ˇ» four
    "});

    // Ensure that indenting/outdenting works when the cursor is at column 0.
    cx.set_state(indoc! {"
        one two
        ˇthree
            four
    "});
    cx.update_editor(|e, cx| e.tab(&Tab, cx));
    cx.assert_editor_state(indoc! {"
        one two
            ˇthree
            four
    "});

    cx.set_state(indoc! {"
        one two
        ˇ    three
            four
    "});
    cx.update_editor(|e, cx| e.tab_prev(&TabPrev, cx));
    cx.assert_editor_state(indoc! {"
        one two
        ˇthree
            four
    "});
}

#[gpui::test]
async fn test_indent_outdent_with_hard_tabs(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);
    cx.update(|cx| {
        cx.update_global::<Settings, _, _>(|settings, _| {
            settings.editor_overrides.hard_tabs = Some(true);
        });
    });

    // select two ranges on one line
    cx.set_state(indoc! {"
        «oneˇ» «twoˇ»
        three
        four
    "});
    cx.update_editor(|e, cx| e.tab(&Tab, cx));
    cx.assert_editor_state(indoc! {"
        \t«oneˇ» «twoˇ»
        three
        four
    "});
    cx.update_editor(|e, cx| e.tab(&Tab, cx));
    cx.assert_editor_state(indoc! {"
        \t\t«oneˇ» «twoˇ»
        three
        four
    "});
    cx.update_editor(|e, cx| e.tab_prev(&TabPrev, cx));
    cx.assert_editor_state(indoc! {"
        \t«oneˇ» «twoˇ»
        three
        four
    "});
    cx.update_editor(|e, cx| e.tab_prev(&TabPrev, cx));
    cx.assert_editor_state(indoc! {"
        «oneˇ» «twoˇ»
        three
        four
    "});

    // select across a line ending
    cx.set_state(indoc! {"
        one two
        t«hree
        ˇ»four
    "});
    cx.update_editor(|e, cx| e.tab(&Tab, cx));
    cx.assert_editor_state(indoc! {"
        one two
        \tt«hree
        ˇ»four
    "});
    cx.update_editor(|e, cx| e.tab(&Tab, cx));
    cx.assert_editor_state(indoc! {"
        one two
        \t\tt«hree
        ˇ»four
    "});
    cx.update_editor(|e, cx| e.tab_prev(&TabPrev, cx));
    cx.assert_editor_state(indoc! {"
        one two
        \tt«hree
        ˇ»four
    "});
    cx.update_editor(|e, cx| e.tab_prev(&TabPrev, cx));
    cx.assert_editor_state(indoc! {"
        one two
        t«hree
        ˇ»four
    "});

    // Ensure that indenting/outdenting works when the cursor is at column 0.
    cx.set_state(indoc! {"
        one two
        ˇthree
        four
    "});
    cx.update_editor(|e, cx| e.tab_prev(&TabPrev, cx));
    cx.assert_editor_state(indoc! {"
        one two
        ˇthree
        four
    "});
    cx.update_editor(|e, cx| e.tab(&Tab, cx));
    cx.assert_editor_state(indoc! {"
        one two
        \tˇthree
        four
    "});
    cx.update_editor(|e, cx| e.tab_prev(&TabPrev, cx));
    cx.assert_editor_state(indoc! {"
        one two
        ˇthree
        four
    "});
}

#[gpui::test]
fn test_indent_outdent_with_excerpts(cx: &mut gpui::MutableAppContext) {
    cx.set_global(
        Settings::test(cx)
            .with_language_defaults(
                "TOML",
                EditorSettings {
                    tab_size: Some(2.try_into().unwrap()),
                    ..Default::default()
                },
            )
            .with_language_defaults(
                "Rust",
                EditorSettings {
                    tab_size: Some(4.try_into().unwrap()),
                    ..Default::default()
                },
            ),
    );
    let toml_language = Arc::new(Language::new(
        LanguageConfig {
            name: "TOML".into(),
            ..Default::default()
        },
        None,
    ));
    let rust_language = Arc::new(Language::new(
        LanguageConfig {
            name: "Rust".into(),
            ..Default::default()
        },
        None,
    ));

    let toml_buffer =
        cx.add_model(|cx| Buffer::new(0, "a = 1\nb = 2\n", cx).with_language(toml_language, cx));
    let rust_buffer = cx.add_model(|cx| {
        Buffer::new(0, "const c: usize = 3;\n", cx).with_language(rust_language, cx)
    });
    let multibuffer = cx.add_model(|cx| {
        let mut multibuffer = MultiBuffer::new(0);
        multibuffer.push_excerpts(
            toml_buffer.clone(),
            [ExcerptRange {
                context: Point::new(0, 0)..Point::new(2, 0),
                primary: None,
            }],
            cx,
        );
        multibuffer.push_excerpts(
            rust_buffer.clone(),
            [ExcerptRange {
                context: Point::new(0, 0)..Point::new(1, 0),
                primary: None,
            }],
            cx,
        );
        multibuffer
    });

    cx.add_window(Default::default(), |cx| {
        let mut editor = build_editor(multibuffer, cx);

        assert_eq!(
            editor.text(cx),
            indoc! {"
                a = 1
                b = 2

                const c: usize = 3;
            "}
        );

        select_ranges(
            &mut editor,
            indoc! {"
                «aˇ» = 1
                b = 2

                «const c:ˇ» usize = 3;
            "},
            cx,
        );

        editor.tab(&Tab, cx);
        assert_text_with_selections(
            &mut editor,
            indoc! {"
                  «aˇ» = 1
                b = 2

                    «const c:ˇ» usize = 3;
            "},
            cx,
        );
        editor.tab_prev(&TabPrev, cx);
        assert_text_with_selections(
            &mut editor,
            indoc! {"
                «aˇ» = 1
                b = 2

                «const c:ˇ» usize = 3;
            "},
            cx,
        );

        editor
    });
}

#[gpui::test]
async fn test_backspace(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);

    // Basic backspace
    cx.set_state(indoc! {"
        onˇe two three
        fou«rˇ» five six
        seven «ˇeight nine
        »ten
    "});
    cx.update_editor(|e, cx| e.backspace(&Backspace, cx));
    cx.assert_editor_state(indoc! {"
        oˇe two three
        fouˇ five six
        seven ˇten
    "});

    // Test backspace inside and around indents
    cx.set_state(indoc! {"
        zero
            ˇone
                ˇtwo
            ˇ ˇ ˇ  three
        ˇ  ˇ  four
    "});
    cx.update_editor(|e, cx| e.backspace(&Backspace, cx));
    cx.assert_editor_state(indoc! {"
        zero
        ˇone
            ˇtwo
        ˇ  threeˇ  four
    "});

    // Test backspace with line_mode set to true
    cx.update_editor(|e, _| e.selections.line_mode = true);
    cx.set_state(indoc! {"
        The ˇquick ˇbrown
        fox jumps over
        the lazy dog
        ˇThe qu«ick bˇ»rown"});
    cx.update_editor(|e, cx| e.backspace(&Backspace, cx));
    cx.assert_editor_state(indoc! {"
        ˇfox jumps over
        the lazy dogˇ"});
}

#[gpui::test]
async fn test_delete(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);

    cx.set_state(indoc! {"
        onˇe two three
        fou«rˇ» five six
        seven «ˇeight nine
        »ten
    "});
    cx.update_editor(|e, cx| e.delete(&Delete, cx));
    cx.assert_editor_state(indoc! {"
        onˇ two three
        fouˇ five six
        seven ˇten
    "});

    // Test backspace with line_mode set to true
    cx.update_editor(|e, _| e.selections.line_mode = true);
    cx.set_state(indoc! {"
        The ˇquick ˇbrown
        fox «ˇjum»ps over
        the lazy dog
        ˇThe qu«ick bˇ»rown"});
    cx.update_editor(|e, cx| e.backspace(&Backspace, cx));
    cx.assert_editor_state("ˇthe lazy dogˇ");
}

#[gpui::test]
fn test_delete_line(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 1),
                DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
            ])
        });
        view.delete_line(&DeleteLine, cx);
        assert_eq!(view.display_text(cx), "ghi");
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1)
            ]
        );
    });

    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([DisplayPoint::new(2, 0)..DisplayPoint::new(0, 1)])
        });
        view.delete_line(&DeleteLine, cx);
        assert_eq!(view.display_text(cx), "ghi\n");
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1)]
        );
    });
}

#[gpui::test]
fn test_duplicate_line(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
            ])
        });
        view.duplicate_line(&DuplicateLine, cx);
        assert_eq!(view.display_text(cx), "abc\nabc\ndef\ndef\nghi\n\n");
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 1),
                DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
                DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0),
                DisplayPoint::new(6, 0)..DisplayPoint::new(6, 0),
            ]
        );
    });

    let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(0, 1)..DisplayPoint::new(1, 1),
                DisplayPoint::new(1, 2)..DisplayPoint::new(2, 1),
            ])
        });
        view.duplicate_line(&DuplicateLine, cx);
        assert_eq!(view.display_text(cx), "abc\ndef\nghi\nabc\ndef\nghi\n");
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(3, 1)..DisplayPoint::new(4, 1),
                DisplayPoint::new(4, 2)..DisplayPoint::new(5, 1),
            ]
        );
    });
}

#[gpui::test]
fn test_move_line_up_down(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple(&sample_text(10, 5, 'a'), cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
    view.update(cx, |view, cx| {
        view.fold_ranges(
            vec![
                Point::new(0, 2)..Point::new(1, 2),
                Point::new(2, 3)..Point::new(4, 1),
                Point::new(7, 0)..Point::new(8, 4),
            ],
            true,
            cx,
        );
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                DisplayPoint::new(3, 1)..DisplayPoint::new(3, 1),
                DisplayPoint::new(3, 2)..DisplayPoint::new(4, 3),
                DisplayPoint::new(5, 0)..DisplayPoint::new(5, 2),
            ])
        });
        assert_eq!(
            view.display_text(cx),
            "aa⋯bbb\nccc⋯eeee\nfffff\nggggg\n⋯i\njjjjj"
        );

        view.move_line_up(&MoveLineUp, cx);
        assert_eq!(
            view.display_text(cx),
            "aa⋯bbb\nccc⋯eeee\nggggg\n⋯i\njjjjj\nfffff"
        );
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3),
                DisplayPoint::new(4, 0)..DisplayPoint::new(4, 2)
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.move_line_down(&MoveLineDown, cx);
        assert_eq!(
            view.display_text(cx),
            "ccc⋯eeee\naa⋯bbb\nfffff\nggggg\n⋯i\njjjjj"
        );
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1),
                DisplayPoint::new(3, 1)..DisplayPoint::new(3, 1),
                DisplayPoint::new(3, 2)..DisplayPoint::new(4, 3),
                DisplayPoint::new(5, 0)..DisplayPoint::new(5, 2)
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.move_line_down(&MoveLineDown, cx);
        assert_eq!(
            view.display_text(cx),
            "ccc⋯eeee\nfffff\naa⋯bbb\nggggg\n⋯i\njjjjj"
        );
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                DisplayPoint::new(3, 1)..DisplayPoint::new(3, 1),
                DisplayPoint::new(3, 2)..DisplayPoint::new(4, 3),
                DisplayPoint::new(5, 0)..DisplayPoint::new(5, 2)
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.move_line_up(&MoveLineUp, cx);
        assert_eq!(
            view.display_text(cx),
            "ccc⋯eeee\naa⋯bbb\nggggg\n⋯i\njjjjj\nfffff"
        );
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1),
                DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1),
                DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3),
                DisplayPoint::new(4, 0)..DisplayPoint::new(4, 2)
            ]
        );
    });
}

#[gpui::test]
fn test_move_line_up_down_with_blocks(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple(&sample_text(10, 5, 'a'), cx);
    let snapshot = buffer.read(cx).snapshot(cx);
    let (_, editor) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
    editor.update(cx, |editor, cx| {
        editor.insert_blocks(
            [BlockProperties {
                style: BlockStyle::Fixed,
                position: snapshot.anchor_after(Point::new(2, 0)),
                disposition: BlockDisposition::Below,
                height: 1,
                render: Arc::new(|_| Empty::new().boxed()),
            }],
            cx,
        );
        editor.change_selections(None, cx, |s| {
            s.select_ranges([Point::new(2, 0)..Point::new(2, 0)])
        });
        editor.move_line_down(&MoveLineDown, cx);
    });
}

#[gpui::test]
fn test_transpose(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));

    _ = cx
        .add_window(Default::default(), |cx| {
            let mut editor = build_editor(MultiBuffer::build_simple("abc", cx), cx);

            editor.change_selections(None, cx, |s| s.select_ranges([1..1]));
            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "bac");
            assert_eq!(editor.selections.ranges(cx), [2..2]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "bca");
            assert_eq!(editor.selections.ranges(cx), [3..3]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "bac");
            assert_eq!(editor.selections.ranges(cx), [3..3]);

            editor
        })
        .1;

    _ = cx
        .add_window(Default::default(), |cx| {
            let mut editor = build_editor(MultiBuffer::build_simple("abc\nde", cx), cx);

            editor.change_selections(None, cx, |s| s.select_ranges([3..3]));
            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "acb\nde");
            assert_eq!(editor.selections.ranges(cx), [3..3]);

            editor.change_selections(None, cx, |s| s.select_ranges([4..4]));
            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "acbd\ne");
            assert_eq!(editor.selections.ranges(cx), [5..5]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "acbde\n");
            assert_eq!(editor.selections.ranges(cx), [6..6]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "acbd\ne");
            assert_eq!(editor.selections.ranges(cx), [6..6]);

            editor
        })
        .1;

    _ = cx
        .add_window(Default::default(), |cx| {
            let mut editor = build_editor(MultiBuffer::build_simple("abc\nde", cx), cx);

            editor.change_selections(None, cx, |s| s.select_ranges([1..1, 2..2, 4..4]));
            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "bacd\ne");
            assert_eq!(editor.selections.ranges(cx), [2..2, 3..3, 5..5]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "bcade\n");
            assert_eq!(editor.selections.ranges(cx), [3..3, 4..4, 6..6]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "bcda\ne");
            assert_eq!(editor.selections.ranges(cx), [4..4, 6..6]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "bcade\n");
            assert_eq!(editor.selections.ranges(cx), [4..4, 6..6]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "bcaed\n");
            assert_eq!(editor.selections.ranges(cx), [5..5, 6..6]);

            editor
        })
        .1;

    _ = cx
        .add_window(Default::default(), |cx| {
            let mut editor = build_editor(MultiBuffer::build_simple("🍐🏀✋", cx), cx);

            editor.change_selections(None, cx, |s| s.select_ranges([4..4]));
            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "🏀🍐✋");
            assert_eq!(editor.selections.ranges(cx), [8..8]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "🏀✋🍐");
            assert_eq!(editor.selections.ranges(cx), [11..11]);

            editor.transpose(&Default::default(), cx);
            assert_eq!(editor.text(cx), "🏀🍐✋");
            assert_eq!(editor.selections.ranges(cx), [11..11]);

            editor
        })
        .1;
}

#[gpui::test]
async fn test_clipboard(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);

    cx.set_state("«one✅ ˇ»two «three ˇ»four «five ˇ»six ");
    cx.update_editor(|e, cx| e.cut(&Cut, cx));
    cx.assert_editor_state("ˇtwo ˇfour ˇsix ");

    // Paste with three cursors. Each cursor pastes one slice of the clipboard text.
    cx.set_state("two ˇfour ˇsix ˇ");
    cx.update_editor(|e, cx| e.paste(&Paste, cx));
    cx.assert_editor_state("two one✅ ˇfour three ˇsix five ˇ");

    // Paste again but with only two cursors. Since the number of cursors doesn't
    // match the number of slices in the clipboard, the entire clipboard text
    // is pasted at each cursor.
    cx.set_state("ˇtwo one✅ four three six five ˇ");
    cx.update_editor(|e, cx| {
        e.handle_input("( ", cx);
        e.paste(&Paste, cx);
        e.handle_input(") ", cx);
    });
    cx.assert_editor_state(
        &([
            "( one✅ ",
            "three ",
            "five ) ˇtwo one✅ four three six five ( one✅ ",
            "three ",
            "five ) ˇ",
        ]
        .join("\n")),
    );

    // Cut with three selections, one of which is full-line.
    cx.set_state(indoc! {"
        1«2ˇ»3
        4ˇ567
        «8ˇ»9"});
    cx.update_editor(|e, cx| e.cut(&Cut, cx));
    cx.assert_editor_state(indoc! {"
        1ˇ3
        ˇ9"});

    // Paste with three selections, noticing how the copied selection that was full-line
    // gets inserted before the second cursor.
    cx.set_state(indoc! {"
        1ˇ3
        9ˇ
        «oˇ»ne"});
    cx.update_editor(|e, cx| e.paste(&Paste, cx));
    cx.assert_editor_state(indoc! {"
        12ˇ3
        4567
        9ˇ
        8ˇne"});

    // Copy with a single cursor only, which writes the whole line into the clipboard.
    cx.set_state(indoc! {"
        The quick brown
        fox juˇmps over
        the lazy dog"});
    cx.update_editor(|e, cx| e.copy(&Copy, cx));
    cx.cx.assert_clipboard_content(Some("fox jumps over\n"));

    // Paste with three selections, noticing how the copied full-line selection is inserted
    // before the empty selections but replaces the selection that is non-empty.
    cx.set_state(indoc! {"
        Tˇhe quick brown
        «foˇ»x jumps over
        tˇhe lazy dog"});
    cx.update_editor(|e, cx| e.paste(&Paste, cx));
    cx.assert_editor_state(indoc! {"
        fox jumps over
        Tˇhe quick brown
        fox jumps over
        ˇx jumps over
        fox jumps over
        tˇhe lazy dog"});
}

#[gpui::test]
async fn test_paste_multiline(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);
    let language = Arc::new(Language::new(
        LanguageConfig::default(),
        Some(tree_sitter_rust::language()),
    ));
    cx.update_buffer(|buffer, cx| buffer.set_language(Some(language), cx));

    // Cut an indented block, without the leading whitespace.
    cx.set_state(indoc! {"
        const a: B = (
            c(),
            «d(
                e,
                f
            )ˇ»
        );
    "});
    cx.update_editor(|e, cx| e.cut(&Cut, cx));
    cx.assert_editor_state(indoc! {"
        const a: B = (
            c(),
            ˇ
        );
    "});

    // Paste it at the same position.
    cx.update_editor(|e, cx| e.paste(&Paste, cx));
    cx.assert_editor_state(indoc! {"
        const a: B = (
            c(),
            d(
                e,
                f
            )ˇ
        );
    "});

    // Paste it at a line with a lower indent level.
    cx.set_state(indoc! {"
        ˇ
        const a: B = (
            c(),
        );
    "});
    cx.update_editor(|e, cx| e.paste(&Paste, cx));
    cx.assert_editor_state(indoc! {"
        d(
            e,
            f
        )ˇ
        const a: B = (
            c(),
        );
    "});

    // Cut an indented block, with the leading whitespace.
    cx.set_state(indoc! {"
        const a: B = (
            c(),
        «    d(
                e,
                f
            )
        ˇ»);
    "});
    cx.update_editor(|e, cx| e.cut(&Cut, cx));
    cx.assert_editor_state(indoc! {"
        const a: B = (
            c(),
        ˇ);
    "});

    // Paste it at the same position.
    cx.update_editor(|e, cx| e.paste(&Paste, cx));
    cx.assert_editor_state(indoc! {"
        const a: B = (
            c(),
            d(
                e,
                f
            )
        ˇ);
    "});

    // Paste it at a line with a higher indent level.
    cx.set_state(indoc! {"
        const a: B = (
            c(),
            d(
                e,
                fˇ
            )
        );
    "});
    cx.update_editor(|e, cx| e.paste(&Paste, cx));
    cx.assert_editor_state(indoc! {"
        const a: B = (
            c(),
            d(
                e,
                f    d(
                    e,
                    f
                )
        ˇ
            )
        );
    "});
}

#[gpui::test]
fn test_select_all(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple("abc\nde\nfgh", cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
    view.update(cx, |view, cx| {
        view.select_all(&SelectAll, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[DisplayPoint::new(0, 0)..DisplayPoint::new(2, 3)]
        );
    });
}

#[gpui::test]
fn test_select_line(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple(&sample_text(6, 5, 'a'), cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                DisplayPoint::new(4, 2)..DisplayPoint::new(4, 2),
            ])
        });
        view.select_line(&SelectLine, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(0, 0)..DisplayPoint::new(2, 0),
                DisplayPoint::new(4, 0)..DisplayPoint::new(5, 0),
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.select_line(&SelectLine, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(0, 0)..DisplayPoint::new(3, 0),
                DisplayPoint::new(4, 0)..DisplayPoint::new(5, 5),
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.select_line(&SelectLine, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![DisplayPoint::new(0, 0)..DisplayPoint::new(5, 5)]
        );
    });
}

#[gpui::test]
fn test_split_selection_into_lines(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple(&sample_text(9, 5, 'a'), cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));
    view.update(cx, |view, cx| {
        view.fold_ranges(
            vec![
                Point::new(0, 2)..Point::new(1, 2),
                Point::new(2, 3)..Point::new(4, 1),
                Point::new(7, 0)..Point::new(8, 4),
            ],
            true,
            cx,
        );
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
                DisplayPoint::new(4, 4)..DisplayPoint::new(4, 4),
            ])
        });
        assert_eq!(view.display_text(cx), "aa⋯bbb\nccc⋯eeee\nfffff\nggggg\n⋯i");
    });

    view.update(cx, |view, cx| {
        view.split_selection_into_lines(&SplitSelectionIntoLines, cx);
        assert_eq!(
            view.display_text(cx),
            "aaaaa\nbbbbb\nccc⋯eeee\nfffff\nggggg\n⋯i"
        );
        assert_eq!(
            view.selections.display_ranges(cx),
            [
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                DisplayPoint::new(2, 0)..DisplayPoint::new(2, 0),
                DisplayPoint::new(5, 4)..DisplayPoint::new(5, 4)
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([DisplayPoint::new(5, 0)..DisplayPoint::new(0, 1)])
        });
        view.split_selection_into_lines(&SplitSelectionIntoLines, cx);
        assert_eq!(
            view.display_text(cx),
            "aaaaa\nbbbbb\nccccc\nddddd\neeeee\nfffff\nggggg\nhhhhh\niiiii"
        );
        assert_eq!(
            view.selections.display_ranges(cx),
            [
                DisplayPoint::new(0, 5)..DisplayPoint::new(0, 5),
                DisplayPoint::new(1, 5)..DisplayPoint::new(1, 5),
                DisplayPoint::new(2, 5)..DisplayPoint::new(2, 5),
                DisplayPoint::new(3, 5)..DisplayPoint::new(3, 5),
                DisplayPoint::new(4, 5)..DisplayPoint::new(4, 5),
                DisplayPoint::new(5, 5)..DisplayPoint::new(5, 5),
                DisplayPoint::new(6, 5)..DisplayPoint::new(6, 5),
                DisplayPoint::new(7, 0)..DisplayPoint::new(7, 0)
            ]
        );
    });
}

#[gpui::test]
fn test_add_selection_above_below(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = MultiBuffer::build_simple("abc\ndefghi\n\njk\nlmno\n", cx);
    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(buffer, cx));

    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)])
        });
    });
    view.update(cx, |view, cx| {
        view.add_selection_above(&AddSelectionAbove, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.add_selection_above(&AddSelectionAbove, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.add_selection_below(&AddSelectionBelow, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)]
        );

        view.undo_selection(&UndoSelection, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)
            ]
        );

        view.redo_selection(&RedoSelection, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3)]
        );
    });

    view.update(cx, |view, cx| {
        view.add_selection_below(&AddSelectionBelow, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3),
                DisplayPoint::new(4, 3)..DisplayPoint::new(4, 3)
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.add_selection_below(&AddSelectionBelow, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(1, 3)..DisplayPoint::new(1, 3),
                DisplayPoint::new(4, 3)..DisplayPoint::new(4, 3)
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3)])
        });
    });
    view.update(cx, |view, cx| {
        view.add_selection_below(&AddSelectionBelow, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3),
                DisplayPoint::new(4, 4)..DisplayPoint::new(4, 3)
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.add_selection_below(&AddSelectionBelow, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3),
                DisplayPoint::new(4, 4)..DisplayPoint::new(4, 3)
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.add_selection_above(&AddSelectionAbove, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3)]
        );
    });

    view.update(cx, |view, cx| {
        view.add_selection_above(&AddSelectionAbove, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![DisplayPoint::new(1, 4)..DisplayPoint::new(1, 3)]
        );
    });

    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([DisplayPoint::new(0, 1)..DisplayPoint::new(1, 4)])
        });
        view.add_selection_below(&AddSelectionBelow, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 1)..DisplayPoint::new(1, 4),
                DisplayPoint::new(3, 1)..DisplayPoint::new(3, 2),
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.add_selection_below(&AddSelectionBelow, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 1)..DisplayPoint::new(1, 4),
                DisplayPoint::new(3, 1)..DisplayPoint::new(3, 2),
                DisplayPoint::new(4, 1)..DisplayPoint::new(4, 4),
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.add_selection_above(&AddSelectionAbove, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 1)..DisplayPoint::new(1, 4),
                DisplayPoint::new(3, 1)..DisplayPoint::new(3, 2),
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([DisplayPoint::new(4, 3)..DisplayPoint::new(1, 1)])
        });
    });
    view.update(cx, |view, cx| {
        view.add_selection_above(&AddSelectionAbove, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(0, 3)..DisplayPoint::new(0, 1),
                DisplayPoint::new(1, 3)..DisplayPoint::new(1, 1),
                DisplayPoint::new(3, 2)..DisplayPoint::new(3, 1),
                DisplayPoint::new(4, 3)..DisplayPoint::new(4, 1),
            ]
        );
    });

    view.update(cx, |view, cx| {
        view.add_selection_below(&AddSelectionBelow, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(1, 3)..DisplayPoint::new(1, 1),
                DisplayPoint::new(3, 2)..DisplayPoint::new(3, 1),
                DisplayPoint::new(4, 3)..DisplayPoint::new(4, 1),
            ]
        );
    });
}

#[gpui::test]
async fn test_select_next(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);
    cx.set_state("abc\nˇabc abc\ndefabc\nabc");

    cx.update_editor(|e, cx| e.select_next(&SelectNext::default(), cx));
    cx.assert_editor_state("abc\n«abcˇ» abc\ndefabc\nabc");

    cx.update_editor(|e, cx| e.select_next(&SelectNext::default(), cx));
    cx.assert_editor_state("abc\n«abcˇ» «abcˇ»\ndefabc\nabc");

    cx.update_editor(|view, cx| view.undo_selection(&UndoSelection, cx));
    cx.assert_editor_state("abc\n«abcˇ» abc\ndefabc\nabc");

    cx.update_editor(|view, cx| view.redo_selection(&RedoSelection, cx));
    cx.assert_editor_state("abc\n«abcˇ» «abcˇ»\ndefabc\nabc");

    cx.update_editor(|e, cx| e.select_next(&SelectNext::default(), cx));
    cx.assert_editor_state("abc\n«abcˇ» «abcˇ»\ndefabc\n«abcˇ»");

    cx.update_editor(|e, cx| e.select_next(&SelectNext::default(), cx));
    cx.assert_editor_state("«abcˇ»\n«abcˇ» «abcˇ»\ndefabc\n«abcˇ»");
}

#[gpui::test]
async fn test_select_larger_smaller_syntax_node(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| cx.set_global(Settings::test(cx)));
    let language = Arc::new(Language::new(
        LanguageConfig::default(),
        Some(tree_sitter_rust::language()),
    ));

    let text = r#"
        use mod1::mod2::{mod3, mod4};

        fn fn_1(param1: bool, param2: &str) {
            let var1 = "text";
        }
    "#
    .unindent();

    let buffer = cx.add_model(|cx| Buffer::new(0, text, cx).with_language(language, cx));
    let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (_, view) = cx.add_window(|cx| build_editor(buffer, cx));
    view.condition(cx, |view, cx| !view.buffer.read(cx).is_parsing(cx))
        .await;

    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(0, 25)..DisplayPoint::new(0, 25),
                DisplayPoint::new(2, 24)..DisplayPoint::new(2, 12),
                DisplayPoint::new(3, 18)..DisplayPoint::new(3, 18),
            ]);
        });
        view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
    });
    assert_eq!(
        view.update(cx, |view, cx| { view.selections.display_ranges(cx) }),
        &[
            DisplayPoint::new(0, 23)..DisplayPoint::new(0, 27),
            DisplayPoint::new(2, 35)..DisplayPoint::new(2, 7),
            DisplayPoint::new(3, 15)..DisplayPoint::new(3, 21),
        ]
    );

    view.update(cx, |view, cx| {
        view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
    });
    assert_eq!(
        view.update(cx, |view, cx| view.selections.display_ranges(cx)),
        &[
            DisplayPoint::new(0, 16)..DisplayPoint::new(0, 28),
            DisplayPoint::new(4, 1)..DisplayPoint::new(2, 0),
        ]
    );

    view.update(cx, |view, cx| {
        view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
    });
    assert_eq!(
        view.update(cx, |view, cx| view.selections.display_ranges(cx)),
        &[DisplayPoint::new(5, 0)..DisplayPoint::new(0, 0)]
    );

    // Trying to expand the selected syntax node one more time has no effect.
    view.update(cx, |view, cx| {
        view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
    });
    assert_eq!(
        view.update(cx, |view, cx| view.selections.display_ranges(cx)),
        &[DisplayPoint::new(5, 0)..DisplayPoint::new(0, 0)]
    );

    view.update(cx, |view, cx| {
        view.select_smaller_syntax_node(&SelectSmallerSyntaxNode, cx);
    });
    assert_eq!(
        view.update(cx, |view, cx| view.selections.display_ranges(cx)),
        &[
            DisplayPoint::new(0, 16)..DisplayPoint::new(0, 28),
            DisplayPoint::new(4, 1)..DisplayPoint::new(2, 0),
        ]
    );

    view.update(cx, |view, cx| {
        view.select_smaller_syntax_node(&SelectSmallerSyntaxNode, cx);
    });
    assert_eq!(
        view.update(cx, |view, cx| view.selections.display_ranges(cx)),
        &[
            DisplayPoint::new(0, 23)..DisplayPoint::new(0, 27),
            DisplayPoint::new(2, 35)..DisplayPoint::new(2, 7),
            DisplayPoint::new(3, 15)..DisplayPoint::new(3, 21),
        ]
    );

    view.update(cx, |view, cx| {
        view.select_smaller_syntax_node(&SelectSmallerSyntaxNode, cx);
    });
    assert_eq!(
        view.update(cx, |view, cx| view.selections.display_ranges(cx)),
        &[
            DisplayPoint::new(0, 25)..DisplayPoint::new(0, 25),
            DisplayPoint::new(2, 24)..DisplayPoint::new(2, 12),
            DisplayPoint::new(3, 18)..DisplayPoint::new(3, 18),
        ]
    );

    // Trying to shrink the selected syntax node one more time has no effect.
    view.update(cx, |view, cx| {
        view.select_smaller_syntax_node(&SelectSmallerSyntaxNode, cx);
    });
    assert_eq!(
        view.update(cx, |view, cx| view.selections.display_ranges(cx)),
        &[
            DisplayPoint::new(0, 25)..DisplayPoint::new(0, 25),
            DisplayPoint::new(2, 24)..DisplayPoint::new(2, 12),
            DisplayPoint::new(3, 18)..DisplayPoint::new(3, 18),
        ]
    );

    // Ensure that we keep expanding the selection if the larger selection starts or ends within
    // a fold.
    view.update(cx, |view, cx| {
        view.fold_ranges(
            vec![
                Point::new(0, 21)..Point::new(0, 24),
                Point::new(3, 20)..Point::new(3, 22),
            ],
            true,
            cx,
        );
        view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
    });
    assert_eq!(
        view.update(cx, |view, cx| view.selections.display_ranges(cx)),
        &[
            DisplayPoint::new(0, 16)..DisplayPoint::new(0, 28),
            DisplayPoint::new(2, 35)..DisplayPoint::new(2, 7),
            DisplayPoint::new(3, 4)..DisplayPoint::new(3, 23),
        ]
    );
}

#[gpui::test]
async fn test_autoindent_selections(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| cx.set_global(Settings::test(cx)));
    let language = Arc::new(
        Language::new(
            LanguageConfig {
                brackets: BracketPairConfig {
                    pairs: vec![
                        BracketPair {
                            start: "{".to_string(),
                            end: "}".to_string(),
                            close: false,
                            newline: true,
                        },
                        BracketPair {
                            start: "(".to_string(),
                            end: ")".to_string(),
                            close: false,
                            newline: true,
                        },
                    ],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        )
        .with_indents_query(
            r#"
                (_ "(" ")" @end) @indent
                (_ "{" "}" @end) @indent
            "#,
        )
        .unwrap(),
    );

    let text = "fn a() {}";

    let buffer = cx.add_model(|cx| Buffer::new(0, text, cx).with_language(language, cx));
    let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (_, editor) = cx.add_window(|cx| build_editor(buffer, cx));
    editor
        .condition(cx, |editor, cx| !editor.buffer.read(cx).is_parsing(cx))
        .await;

    editor.update(cx, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([5..5, 8..8, 9..9]));
        editor.newline(&Newline, cx);
        assert_eq!(editor.text(cx), "fn a(\n    \n) {\n    \n}\n");
        assert_eq!(
            editor.selections.ranges(cx),
            &[
                Point::new(1, 4)..Point::new(1, 4),
                Point::new(3, 4)..Point::new(3, 4),
                Point::new(5, 0)..Point::new(5, 0)
            ]
        );
    });
}

#[gpui::test]
async fn test_autoclose_pairs(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);

    let language = Arc::new(Language::new(
        LanguageConfig {
            brackets: BracketPairConfig {
                pairs: vec![
                    BracketPair {
                        start: "{".to_string(),
                        end: "}".to_string(),
                        close: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "(".to_string(),
                        end: ")".to_string(),
                        close: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "/*".to_string(),
                        end: " */".to_string(),
                        close: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "[".to_string(),
                        end: "]".to_string(),
                        close: false,
                        newline: true,
                    },
                    BracketPair {
                        start: "\"".to_string(),
                        end: "\"".to_string(),
                        close: true,
                        newline: false,
                    },
                ],
                ..Default::default()
            },
            autoclose_before: "})]".to_string(),
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    ));

    let registry = Arc::new(LanguageRegistry::test());
    registry.add(language.clone());
    cx.update_buffer(|buffer, cx| {
        buffer.set_language_registry(registry);
        buffer.set_language(Some(language), cx);
    });

    cx.set_state(
        &r#"
            🏀ˇ
            εˇ
            ❤️ˇ
        "#
        .unindent(),
    );

    // autoclose multiple nested brackets at multiple cursors
    cx.update_editor(|view, cx| {
        view.handle_input("{", cx);
        view.handle_input("{", cx);
        view.handle_input("{", cx);
    });
    cx.assert_editor_state(
        &"
            🏀{{{ˇ}}}
            ε{{{ˇ}}}
            ❤️{{{ˇ}}}
        "
        .unindent(),
    );

    // insert a different closing bracket
    cx.update_editor(|view, cx| {
        view.handle_input(")", cx);
    });
    cx.assert_editor_state(
        &"
            🏀{{{)ˇ}}}
            ε{{{)ˇ}}}
            ❤️{{{)ˇ}}}
        "
        .unindent(),
    );

    // skip over the auto-closed brackets when typing a closing bracket
    cx.update_editor(|view, cx| {
        view.move_right(&MoveRight, cx);
        view.handle_input("}", cx);
        view.handle_input("}", cx);
        view.handle_input("}", cx);
    });
    cx.assert_editor_state(
        &"
            🏀{{{)}}}}ˇ
            ε{{{)}}}}ˇ
            ❤️{{{)}}}}ˇ
        "
        .unindent(),
    );

    // autoclose multi-character pairs
    cx.set_state(
        &"
            ˇ
            ˇ
        "
        .unindent(),
    );
    cx.update_editor(|view, cx| {
        view.handle_input("/", cx);
        view.handle_input("*", cx);
    });
    cx.assert_editor_state(
        &"
            /*ˇ */
            /*ˇ */
        "
        .unindent(),
    );

    // one cursor autocloses a multi-character pair, one cursor
    // does not autoclose.
    cx.set_state(
        &"
            /ˇ
            ˇ
        "
        .unindent(),
    );
    cx.update_editor(|view, cx| view.handle_input("*", cx));
    cx.assert_editor_state(
        &"
            /*ˇ */
            *ˇ
        "
        .unindent(),
    );

    // Don't autoclose if the next character isn't whitespace and isn't
    // listed in the language's "autoclose_before" section.
    cx.set_state("ˇa b");
    cx.update_editor(|view, cx| view.handle_input("{", cx));
    cx.assert_editor_state("{ˇa b");

    // Don't autoclose if `close` is false for the bracket pair
    cx.set_state("ˇ");
    cx.update_editor(|view, cx| view.handle_input("[", cx));
    cx.assert_editor_state("[ˇ");

    // Surround with brackets if text is selected
    cx.set_state("«aˇ» b");
    cx.update_editor(|view, cx| view.handle_input("{", cx));
    cx.assert_editor_state("{«aˇ»} b");

    // Autclose pair where the start and end characters are the same
    cx.set_state("aˇ");
    cx.update_editor(|view, cx| view.handle_input("\"", cx));
    cx.assert_editor_state("a\"ˇ\"");
    cx.update_editor(|view, cx| view.handle_input("\"", cx));
    cx.assert_editor_state("a\"\"ˇ");
}

#[gpui::test]
async fn test_autoclose_with_embedded_language(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);

    let html_language = Arc::new(
        Language::new(
            LanguageConfig {
                name: "HTML".into(),
                brackets: BracketPairConfig {
                    pairs: vec![
                        BracketPair {
                            start: "<".into(),
                            end: ">".into(),
                            close: true,
                            ..Default::default()
                        },
                        BracketPair {
                            start: "{".into(),
                            end: "}".into(),
                            close: true,
                            ..Default::default()
                        },
                        BracketPair {
                            start: "(".into(),
                            end: ")".into(),
                            close: true,
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                autoclose_before: "})]>".into(),
                ..Default::default()
            },
            Some(tree_sitter_html::language()),
        )
        .with_injection_query(
            r#"
            (script_element
                (raw_text) @content
                (#set! "language" "javascript"))
            "#,
        )
        .unwrap(),
    );

    let javascript_language = Arc::new(Language::new(
        LanguageConfig {
            name: "JavaScript".into(),
            brackets: BracketPairConfig {
                pairs: vec![
                    BracketPair {
                        start: "/*".into(),
                        end: " */".into(),
                        close: true,
                        ..Default::default()
                    },
                    BracketPair {
                        start: "{".into(),
                        end: "}".into(),
                        close: true,
                        ..Default::default()
                    },
                    BracketPair {
                        start: "(".into(),
                        end: ")".into(),
                        close: true,
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            autoclose_before: "})]>".into(),
            ..Default::default()
        },
        Some(tree_sitter_javascript::language()),
    ));

    let registry = Arc::new(LanguageRegistry::test());
    registry.add(html_language.clone());
    registry.add(javascript_language.clone());

    cx.update_buffer(|buffer, cx| {
        buffer.set_language_registry(registry);
        buffer.set_language(Some(html_language), cx);
    });

    cx.set_state(
        &r#"
            <body>ˇ
                <script>
                    var x = 1;ˇ
                </script>
            </body>ˇ
        "#
        .unindent(),
    );

    // Precondition: different languages are active at different locations.
    cx.update_editor(|editor, cx| {
        let snapshot = editor.snapshot(cx);
        let cursors = editor.selections.ranges::<usize>(cx);
        let languages = cursors
            .iter()
            .map(|c| snapshot.language_at(c.start).unwrap().name())
            .collect::<Vec<_>>();
        assert_eq!(
            languages,
            &["HTML".into(), "JavaScript".into(), "HTML".into()]
        );
    });

    // Angle brackets autoclose in HTML, but not JavaScript.
    cx.update_editor(|editor, cx| {
        editor.handle_input("<", cx);
        editor.handle_input("a", cx);
    });
    cx.assert_editor_state(
        &r#"
            <body><aˇ>
                <script>
                    var x = 1;<aˇ
                </script>
            </body><aˇ>
        "#
        .unindent(),
    );

    // Curly braces and parens autoclose in both HTML and JavaScript.
    cx.update_editor(|editor, cx| {
        editor.handle_input(" b=", cx);
        editor.handle_input("{", cx);
        editor.handle_input("c", cx);
        editor.handle_input("(", cx);
    });
    cx.assert_editor_state(
        &r#"
            <body><a b={c(ˇ)}>
                <script>
                    var x = 1;<a b={c(ˇ)}
                </script>
            </body><a b={c(ˇ)}>
        "#
        .unindent(),
    );

    // Brackets that were already autoclosed are skipped.
    cx.update_editor(|editor, cx| {
        editor.handle_input(")", cx);
        editor.handle_input("d", cx);
        editor.handle_input("}", cx);
    });
    cx.assert_editor_state(
        &r#"
            <body><a b={c()d}ˇ>
                <script>
                    var x = 1;<a b={c()d}ˇ
                </script>
            </body><a b={c()d}ˇ>
        "#
        .unindent(),
    );
    cx.update_editor(|editor, cx| {
        editor.handle_input(">", cx);
    });
    cx.assert_editor_state(
        &r#"
            <body><a b={c()d}>ˇ
                <script>
                    var x = 1;<a b={c()d}>ˇ
                </script>
            </body><a b={c()d}>ˇ
        "#
        .unindent(),
    );

    // Reset
    cx.set_state(
        &r#"
            <body>ˇ
                <script>
                    var x = 1;ˇ
                </script>
            </body>ˇ
        "#
        .unindent(),
    );

    cx.update_editor(|editor, cx| {
        editor.handle_input("<", cx);
    });
    cx.assert_editor_state(
        &r#"
            <body><ˇ>
                <script>
                    var x = 1;<ˇ
                </script>
            </body><ˇ>
        "#
        .unindent(),
    );

    // When backspacing, the closing angle brackets are removed.
    cx.update_editor(|editor, cx| {
        editor.backspace(&Backspace, cx);
    });
    cx.assert_editor_state(
        &r#"
            <body>ˇ
                <script>
                    var x = 1;ˇ
                </script>
            </body>ˇ
        "#
        .unindent(),
    );

    // Block comments autoclose in JavaScript, but not HTML.
    cx.update_editor(|editor, cx| {
        editor.handle_input("/", cx);
        editor.handle_input("*", cx);
    });
    cx.assert_editor_state(
        &r#"
            <body>/*ˇ
                <script>
                    var x = 1;/*ˇ */
                </script>
            </body>/*ˇ
        "#
        .unindent(),
    );
}

#[gpui::test]
async fn test_autoclose_with_overrides(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);

    let rust_language = Arc::new(
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                brackets: serde_json::from_value(json!([
                    { "start": "{", "end": "}", "close": true, "newline": true },
                    { "start": "\"", "end": "\"", "close": true, "newline": false, "not_in": ["string"] },
                ]))
                .unwrap(),
                autoclose_before: "})]>".into(),
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        )
        .with_override_query("(string_literal) @string")
        .unwrap(),
    );

    let registry = Arc::new(LanguageRegistry::test());
    registry.add(rust_language.clone());

    cx.update_buffer(|buffer, cx| {
        buffer.set_language_registry(registry);
        buffer.set_language(Some(rust_language), cx);
    });

    cx.set_state(
        &r#"
            let x = ˇ
        "#
        .unindent(),
    );

    // Inserting a quotation mark. A closing quotation mark is automatically inserted.
    cx.update_editor(|editor, cx| {
        editor.handle_input("\"", cx);
    });
    cx.assert_editor_state(
        &r#"
            let x = "ˇ"
        "#
        .unindent(),
    );

    // Inserting another quotation mark. The cursor moves across the existing
    // automatically-inserted quotation mark.
    cx.update_editor(|editor, cx| {
        editor.handle_input("\"", cx);
    });
    cx.assert_editor_state(
        &r#"
            let x = ""ˇ
        "#
        .unindent(),
    );

    // Reset
    cx.set_state(
        &r#"
            let x = ˇ
        "#
        .unindent(),
    );

    // Inserting a quotation mark inside of a string. A second quotation mark is not inserted.
    cx.update_editor(|editor, cx| {
        editor.handle_input("\"", cx);
        editor.handle_input(" ", cx);
        editor.move_left(&Default::default(), cx);
        editor.handle_input("\\", cx);
        editor.handle_input("\"", cx);
    });
    cx.assert_editor_state(
        &r#"
            let x = "\"ˇ "
        "#
        .unindent(),
    );

    // Inserting a closing quotation mark at the position of an automatically-inserted quotation
    // mark. Nothing is inserted.
    cx.update_editor(|editor, cx| {
        editor.move_right(&Default::default(), cx);
        editor.handle_input("\"", cx);
    });
    cx.assert_editor_state(
        &r#"
            let x = "\" "ˇ
        "#
        .unindent(),
    );
}

#[gpui::test]
async fn test_surround_with_pair(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| cx.set_global(Settings::test(cx)));
    let language = Arc::new(Language::new(
        LanguageConfig {
            brackets: BracketPairConfig {
                pairs: vec![
                    BracketPair {
                        start: "{".to_string(),
                        end: "}".to_string(),
                        close: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "/* ".to_string(),
                        end: "*/".to_string(),
                        close: true,
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    ));

    let text = r#"
        a
        b
        c
    "#
    .unindent();

    let buffer = cx.add_model(|cx| Buffer::new(0, text, cx).with_language(language, cx));
    let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (_, view) = cx.add_window(|cx| build_editor(buffer, cx));
    view.condition(cx, |view, cx| !view.buffer.read(cx).is_parsing(cx))
        .await;

    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 1),
                DisplayPoint::new(2, 0)..DisplayPoint::new(2, 1),
            ])
        });

        view.handle_input("{", cx);
        view.handle_input("{", cx);
        view.handle_input("{", cx);
        assert_eq!(
            view.text(cx),
            "
                {{{a}}}
                {{{b}}}
                {{{c}}}
            "
            .unindent()
        );
        assert_eq!(
            view.selections.display_ranges(cx),
            [
                DisplayPoint::new(0, 3)..DisplayPoint::new(0, 4),
                DisplayPoint::new(1, 3)..DisplayPoint::new(1, 4),
                DisplayPoint::new(2, 3)..DisplayPoint::new(2, 4)
            ]
        );

        view.undo(&Undo, cx);
        view.undo(&Undo, cx);
        view.undo(&Undo, cx);
        assert_eq!(
            view.text(cx),
            "
                a
                b
                c
            "
            .unindent()
        );
        assert_eq!(
            view.selections.display_ranges(cx),
            [
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 1),
                DisplayPoint::new(2, 0)..DisplayPoint::new(2, 1)
            ]
        );

        // Ensure inserting the first character of a multi-byte bracket pair
        // doesn't surround the selections with the bracket.
        view.handle_input("/", cx);
        assert_eq!(
            view.text(cx),
            "
                /
                /
                /
            "
            .unindent()
        );
        assert_eq!(
            view.selections.display_ranges(cx),
            [
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1),
                DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1)
            ]
        );

        view.undo(&Undo, cx);
        assert_eq!(
            view.text(cx),
            "
                a
                b
                c
            "
            .unindent()
        );
        assert_eq!(
            view.selections.display_ranges(cx),
            [
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 1),
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 1),
                DisplayPoint::new(2, 0)..DisplayPoint::new(2, 1)
            ]
        );

        // Ensure inserting the last character of a multi-byte bracket pair
        // doesn't surround the selections with the bracket.
        view.handle_input("*", cx);
        assert_eq!(
            view.text(cx),
            "
                *
                *
                *
            "
            .unindent()
        );
        assert_eq!(
            view.selections.display_ranges(cx),
            [
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1),
                DisplayPoint::new(2, 1)..DisplayPoint::new(2, 1)
            ]
        );
    });
}

#[gpui::test]
async fn test_delete_autoclose_pair(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| cx.set_global(Settings::test(cx)));
    let language = Arc::new(Language::new(
        LanguageConfig {
            brackets: BracketPairConfig {
                pairs: vec![BracketPair {
                    start: "{".to_string(),
                    end: "}".to_string(),
                    close: true,
                    newline: true,
                }],
                ..Default::default()
            },
            autoclose_before: "}".to_string(),
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    ));

    let text = r#"
        a
        b
        c
    "#
    .unindent();

    let buffer = cx.add_model(|cx| Buffer::new(0, text, cx).with_language(language, cx));
    let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (_, editor) = cx.add_window(|cx| build_editor(buffer, cx));
    editor
        .condition(cx, |view, cx| !view.buffer.read(cx).is_parsing(cx))
        .await;

    editor.update(cx, |editor, cx| {
        editor.change_selections(None, cx, |s| {
            s.select_ranges([
                Point::new(0, 1)..Point::new(0, 1),
                Point::new(1, 1)..Point::new(1, 1),
                Point::new(2, 1)..Point::new(2, 1),
            ])
        });

        editor.handle_input("{", cx);
        editor.handle_input("{", cx);
        editor.handle_input("_", cx);
        assert_eq!(
            editor.text(cx),
            "
                a{{_}}
                b{{_}}
                c{{_}}
            "
            .unindent()
        );
        assert_eq!(
            editor.selections.ranges::<Point>(cx),
            [
                Point::new(0, 4)..Point::new(0, 4),
                Point::new(1, 4)..Point::new(1, 4),
                Point::new(2, 4)..Point::new(2, 4)
            ]
        );

        editor.backspace(&Default::default(), cx);
        editor.backspace(&Default::default(), cx);
        assert_eq!(
            editor.text(cx),
            "
                a{}
                b{}
                c{}
            "
            .unindent()
        );
        assert_eq!(
            editor.selections.ranges::<Point>(cx),
            [
                Point::new(0, 2)..Point::new(0, 2),
                Point::new(1, 2)..Point::new(1, 2),
                Point::new(2, 2)..Point::new(2, 2)
            ]
        );

        editor.delete_to_previous_word_start(&Default::default(), cx);
        assert_eq!(
            editor.text(cx),
            "
                a
                b
                c
            "
            .unindent()
        );
        assert_eq!(
            editor.selections.ranges::<Point>(cx),
            [
                Point::new(0, 1)..Point::new(0, 1),
                Point::new(1, 1)..Point::new(1, 1),
                Point::new(2, 1)..Point::new(2, 1)
            ]
        );
    });
}

#[gpui::test]
async fn test_snippets(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| cx.set_global(Settings::test(cx)));

    let (text, insertion_ranges) = marked_text_ranges(
        indoc! {"
            a.ˇ b
            a.ˇ b
            a.ˇ b
        "},
        false,
    );

    let buffer = cx.update(|cx| MultiBuffer::build_simple(&text, cx));
    let (_, editor) = cx.add_window(|cx| build_editor(buffer, cx));

    editor.update(cx, |editor, cx| {
        let snippet = Snippet::parse("f(${1:one}, ${2:two}, ${1:three})$0").unwrap();

        editor
            .insert_snippet(&insertion_ranges, snippet, cx)
            .unwrap();

        fn assert(editor: &mut Editor, cx: &mut ViewContext<Editor>, marked_text: &str) {
            let (expected_text, selection_ranges) = marked_text_ranges(marked_text, false);
            assert_eq!(editor.text(cx), expected_text);
            assert_eq!(editor.selections.ranges::<usize>(cx), selection_ranges);
        }

        assert(
            editor,
            cx,
            indoc! {"
                a.f(«one», two, «three») b
                a.f(«one», two, «three») b
                a.f(«one», two, «three») b
            "},
        );

        // Can't move earlier than the first tab stop
        assert!(!editor.move_to_prev_snippet_tabstop(cx));
        assert(
            editor,
            cx,
            indoc! {"
                a.f(«one», two, «three») b
                a.f(«one», two, «three») b
                a.f(«one», two, «three») b
            "},
        );

        assert!(editor.move_to_next_snippet_tabstop(cx));
        assert(
            editor,
            cx,
            indoc! {"
                a.f(one, «two», three) b
                a.f(one, «two», three) b
                a.f(one, «two», three) b
            "},
        );

        editor.move_to_prev_snippet_tabstop(cx);
        assert(
            editor,
            cx,
            indoc! {"
                a.f(«one», two, «three») b
                a.f(«one», two, «three») b
                a.f(«one», two, «three») b
            "},
        );

        assert!(editor.move_to_next_snippet_tabstop(cx));
        assert(
            editor,
            cx,
            indoc! {"
                a.f(one, «two», three) b
                a.f(one, «two», three) b
                a.f(one, «two», three) b
            "},
        );
        assert!(editor.move_to_next_snippet_tabstop(cx));
        assert(
            editor,
            cx,
            indoc! {"
                a.f(one, two, three)ˇ b
                a.f(one, two, three)ˇ b
                a.f(one, two, three)ˇ b
            "},
        );

        // As soon as the last tab stop is reached, snippet state is gone
        editor.move_to_prev_snippet_tabstop(cx);
        assert(
            editor,
            cx,
            indoc! {"
                a.f(one, two, three)ˇ b
                a.f(one, two, three)ˇ b
                a.f(one, two, three)ˇ b
            "},
        );
    });
}

#[gpui::test]
async fn test_document_format_during_save(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_servers = language
        .set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                document_formatting_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        }))
        .await;

    let fs = FakeFs::new(cx.background());
    fs.insert_file("/file.rs", Default::default()).await;

    let project = Project::test(fs, ["/file.rs".as_ref()], cx).await;
    project.update(cx, |project, _| project.languages().add(Arc::new(language)));
    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/file.rs", cx))
        .await
        .unwrap();

    cx.foreground().start_waiting();
    let fake_server = fake_servers.next().await.unwrap();

    let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (_, editor) = cx.add_window(|cx| build_editor(buffer, cx));
    editor.update(cx, |editor, cx| editor.set_text("one\ntwo\nthree\n", cx));
    assert!(cx.read(|cx| editor.is_dirty(cx)));

    let save = cx.update(|cx| editor.save(project.clone(), cx));
    fake_server
        .handle_request::<lsp::request::Formatting, _, _>(move |params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path("/file.rs").unwrap()
            );
            assert_eq!(params.options.tab_size, 4);
            Ok(Some(vec![lsp::TextEdit::new(
                lsp::Range::new(lsp::Position::new(0, 3), lsp::Position::new(1, 0)),
                ", ".to_string(),
            )]))
        })
        .next()
        .await;
    cx.foreground().start_waiting();
    save.await.unwrap();
    assert_eq!(
        editor.read_with(cx, |editor, cx| editor.text(cx)),
        "one, two\nthree\n"
    );
    assert!(!cx.read(|cx| editor.is_dirty(cx)));

    editor.update(cx, |editor, cx| editor.set_text("one\ntwo\nthree\n", cx));
    assert!(cx.read(|cx| editor.is_dirty(cx)));

    // Ensure we can still save even if formatting hangs.
    fake_server.handle_request::<lsp::request::Formatting, _, _>(move |params, _| async move {
        assert_eq!(
            params.text_document.uri,
            lsp::Url::from_file_path("/file.rs").unwrap()
        );
        futures::future::pending::<()>().await;
        unreachable!()
    });
    let save = cx.update(|cx| editor.save(project.clone(), cx));
    cx.foreground().advance_clock(super::FORMAT_TIMEOUT);
    cx.foreground().start_waiting();
    save.await.unwrap();
    assert_eq!(
        editor.read_with(cx, |editor, cx| editor.text(cx)),
        "one\ntwo\nthree\n"
    );
    assert!(!cx.read(|cx| editor.is_dirty(cx)));

    // Set rust language override and assert overriden tabsize is sent to language server
    cx.update(|cx| {
        cx.update_global::<Settings, _, _>(|settings, _| {
            settings.language_overrides.insert(
                "Rust".into(),
                EditorSettings {
                    tab_size: Some(8.try_into().unwrap()),
                    ..Default::default()
                },
            );
        })
    });

    let save = cx.update(|cx| editor.save(project.clone(), cx));
    fake_server
        .handle_request::<lsp::request::Formatting, _, _>(move |params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path("/file.rs").unwrap()
            );
            assert_eq!(params.options.tab_size, 8);
            Ok(Some(vec![]))
        })
        .next()
        .await;
    cx.foreground().start_waiting();
    save.await.unwrap();
}

#[gpui::test]
async fn test_range_format_during_save(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_servers = language
        .set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                document_range_formatting_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        }))
        .await;

    let fs = FakeFs::new(cx.background());
    fs.insert_file("/file.rs", Default::default()).await;

    let project = Project::test(fs, ["/file.rs".as_ref()], cx).await;
    project.update(cx, |project, _| project.languages().add(Arc::new(language)));
    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/file.rs", cx))
        .await
        .unwrap();

    cx.foreground().start_waiting();
    let fake_server = fake_servers.next().await.unwrap();

    let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (_, editor) = cx.add_window(|cx| build_editor(buffer, cx));
    editor.update(cx, |editor, cx| editor.set_text("one\ntwo\nthree\n", cx));
    assert!(cx.read(|cx| editor.is_dirty(cx)));

    let save = cx.update(|cx| editor.save(project.clone(), cx));
    fake_server
        .handle_request::<lsp::request::RangeFormatting, _, _>(move |params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path("/file.rs").unwrap()
            );
            assert_eq!(params.options.tab_size, 4);
            Ok(Some(vec![lsp::TextEdit::new(
                lsp::Range::new(lsp::Position::new(0, 3), lsp::Position::new(1, 0)),
                ", ".to_string(),
            )]))
        })
        .next()
        .await;
    cx.foreground().start_waiting();
    save.await.unwrap();
    assert_eq!(
        editor.read_with(cx, |editor, cx| editor.text(cx)),
        "one, two\nthree\n"
    );
    assert!(!cx.read(|cx| editor.is_dirty(cx)));

    editor.update(cx, |editor, cx| editor.set_text("one\ntwo\nthree\n", cx));
    assert!(cx.read(|cx| editor.is_dirty(cx)));

    // Ensure we can still save even if formatting hangs.
    fake_server.handle_request::<lsp::request::RangeFormatting, _, _>(
        move |params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path("/file.rs").unwrap()
            );
            futures::future::pending::<()>().await;
            unreachable!()
        },
    );
    let save = cx.update(|cx| editor.save(project.clone(), cx));
    cx.foreground().advance_clock(super::FORMAT_TIMEOUT);
    cx.foreground().start_waiting();
    save.await.unwrap();
    assert_eq!(
        editor.read_with(cx, |editor, cx| editor.text(cx)),
        "one\ntwo\nthree\n"
    );
    assert!(!cx.read(|cx| editor.is_dirty(cx)));

    // Set rust language override and assert overriden tabsize is sent to language server
    cx.update(|cx| {
        cx.update_global::<Settings, _, _>(|settings, _| {
            settings.language_overrides.insert(
                "Rust".into(),
                EditorSettings {
                    tab_size: Some(8.try_into().unwrap()),
                    ..Default::default()
                },
            );
        })
    });

    let save = cx.update(|cx| editor.save(project.clone(), cx));
    fake_server
        .handle_request::<lsp::request::RangeFormatting, _, _>(move |params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path("/file.rs").unwrap()
            );
            assert_eq!(params.options.tab_size, 8);
            Ok(Some(vec![]))
        })
        .next()
        .await;
    cx.foreground().start_waiting();
    save.await.unwrap();
}

#[gpui::test]
async fn test_document_format_manual_trigger(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    let mut language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_servers = language
        .set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                document_formatting_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        }))
        .await;

    let fs = FakeFs::new(cx.background());
    fs.insert_file("/file.rs", Default::default()).await;

    let project = Project::test(fs, ["/file.rs".as_ref()], cx).await;
    project.update(cx, |project, _| project.languages().add(Arc::new(language)));
    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/file.rs", cx))
        .await
        .unwrap();

    cx.foreground().start_waiting();
    let fake_server = fake_servers.next().await.unwrap();

    let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (_, editor) = cx.add_window(|cx| build_editor(buffer, cx));
    editor.update(cx, |editor, cx| editor.set_text("one\ntwo\nthree\n", cx));

    let format = editor.update(cx, |editor, cx| {
        editor.perform_format(project.clone(), FormatTrigger::Manual, cx)
    });
    fake_server
        .handle_request::<lsp::request::Formatting, _, _>(move |params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path("/file.rs").unwrap()
            );
            assert_eq!(params.options.tab_size, 4);
            Ok(Some(vec![lsp::TextEdit::new(
                lsp::Range::new(lsp::Position::new(0, 3), lsp::Position::new(1, 0)),
                ", ".to_string(),
            )]))
        })
        .next()
        .await;
    cx.foreground().start_waiting();
    format.await.unwrap();
    assert_eq!(
        editor.read_with(cx, |editor, cx| editor.text(cx)),
        "one, two\nthree\n"
    );

    editor.update(cx, |editor, cx| editor.set_text("one\ntwo\nthree\n", cx));
    // Ensure we don't lock if formatting hangs.
    fake_server.handle_request::<lsp::request::Formatting, _, _>(move |params, _| async move {
        assert_eq!(
            params.text_document.uri,
            lsp::Url::from_file_path("/file.rs").unwrap()
        );
        futures::future::pending::<()>().await;
        unreachable!()
    });
    let format = editor.update(cx, |editor, cx| {
        editor.perform_format(project, FormatTrigger::Manual, cx)
    });
    cx.foreground().advance_clock(super::FORMAT_TIMEOUT);
    cx.foreground().start_waiting();
    format.await.unwrap();
    assert_eq!(
        editor.read_with(cx, |editor, cx| editor.text(cx)),
        "one\ntwo\nthree\n"
    );
}

#[gpui::test]
async fn test_concurrent_format_requests(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            document_formatting_provider: Some(lsp::OneOf::Left(true)),
            ..Default::default()
        },
        cx,
    )
    .await;

    cx.set_state(indoc! {"
        one.twoˇ
    "});

    // The format request takes a long time. When it completes, it inserts
    // a newline and an indent before the `.`
    cx.lsp
        .handle_request::<lsp::request::Formatting, _, _>(move |_, cx| {
            let executor = cx.background();
            async move {
                executor.timer(Duration::from_millis(100)).await;
                Ok(Some(vec![lsp::TextEdit {
                    range: lsp::Range::new(lsp::Position::new(0, 3), lsp::Position::new(0, 3)),
                    new_text: "\n    ".into(),
                }]))
            }
        });

    // Submit a format request.
    let format_1 = cx
        .update_editor(|editor, cx| editor.format(&Format, cx))
        .unwrap();
    cx.foreground().run_until_parked();

    // Submit a second format request.
    let format_2 = cx
        .update_editor(|editor, cx| editor.format(&Format, cx))
        .unwrap();
    cx.foreground().run_until_parked();

    // Wait for both format requests to complete
    cx.foreground().advance_clock(Duration::from_millis(200));
    cx.foreground().start_waiting();
    format_1.await.unwrap();
    cx.foreground().start_waiting();
    format_2.await.unwrap();

    // The formatting edits only happens once.
    cx.assert_editor_state(indoc! {"
        one
            .twoˇ
    "});
}

#[gpui::test]
async fn test_strip_whitespace_and_format_via_lsp(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            document_formatting_provider: Some(lsp::OneOf::Left(true)),
            ..Default::default()
        },
        cx,
    )
    .await;

    // Set up a buffer white some trailing whitespace and no trailing newline.
    cx.set_state(
        &[
            "one ",   //
            "twoˇ",  //
            "three ", //
            "four",   //
        ]
        .join("\n"),
    );

    // Submit a format request.
    let format = cx
        .update_editor(|editor, cx| editor.format(&Format, cx))
        .unwrap();

    // Record which buffer changes have been sent to the language server
    let buffer_changes = Arc::new(Mutex::new(Vec::new()));
    cx.lsp
        .handle_notification::<lsp::notification::DidChangeTextDocument, _>({
            let buffer_changes = buffer_changes.clone();
            move |params, _| {
                buffer_changes.lock().extend(
                    params
                        .content_changes
                        .into_iter()
                        .map(|e| (e.range.unwrap(), e.text)),
                );
            }
        });

    // Handle formatting requests to the language server.
    cx.lsp.handle_request::<lsp::request::Formatting, _, _>({
        let buffer_changes = buffer_changes.clone();
        move |_, _| {
            // When formatting is requested, trailing whitespace has already been stripped,
            // and the trailing newline has already been added.
            assert_eq!(
                &buffer_changes.lock()[1..],
                &[
                    (
                        lsp::Range::new(lsp::Position::new(0, 3), lsp::Position::new(0, 4)),
                        "".into()
                    ),
                    (
                        lsp::Range::new(lsp::Position::new(2, 5), lsp::Position::new(2, 6)),
                        "".into()
                    ),
                    (
                        lsp::Range::new(lsp::Position::new(3, 4), lsp::Position::new(3, 4)),
                        "\n".into()
                    ),
                ]
            );

            // Insert blank lines between each line of the buffer.
            async move {
                Ok(Some(vec![
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 0)),
                        new_text: "\n".into(),
                    },
                    lsp::TextEdit {
                        range: lsp::Range::new(lsp::Position::new(2, 0), lsp::Position::new(2, 0)),
                        new_text: "\n".into(),
                    },
                ]))
            }
        }
    });

    // After formatting the buffer, the trailing whitespace is stripped,
    // a newline is appended, and the edits provided by the language server
    // have been applied.
    format.await.unwrap();
    cx.assert_editor_state(
        &[
            "one",   //
            "",      //
            "twoˇ", //
            "",      //
            "three", //
            "four",  //
            "",      //
        ]
        .join("\n"),
    );

    // Undoing the formatting undoes the trailing whitespace removal, the
    // trailing newline, and the LSP edits.
    cx.update_buffer(|buffer, cx| buffer.undo(cx));
    cx.assert_editor_state(
        &[
            "one ",   //
            "twoˇ",  //
            "three ", //
            "four",   //
        ]
        .join("\n"),
    );
}

#[gpui::test]
async fn test_completion(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            completion_provider: Some(lsp::CompletionOptions {
                trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                ..Default::default()
            }),
            ..Default::default()
        },
        cx,
    )
    .await;

    cx.set_state(indoc! {"
        oneˇ
        two
        three
    "});
    cx.simulate_keystroke(".");
    handle_completion_request(
        &mut cx,
        indoc! {"
            one.|<>
            two
            three
        "},
        vec!["first_completion", "second_completion"],
    )
    .await;
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    let apply_additional_edits = cx.update_editor(|editor, cx| {
        editor.move_down(&MoveDown, cx);
        editor
            .confirm_completion(&ConfirmCompletion::default(), cx)
            .unwrap()
    });
    cx.assert_editor_state(indoc! {"
        one.second_completionˇ
        two
        three
    "});

    handle_resolve_completion_request(
        &mut cx,
        Some(vec![
            (
                //This overlaps with the primary completion edit which is
                //misbehavior from the LSP spec, test that we filter it out
                indoc! {"
                    one.second_ˇcompletion
                    two
                    threeˇ
                "},
                "overlapping aditional edit",
            ),
            (
                indoc! {"
                    one.second_completion
                    two
                    threeˇ
                "},
                "\nadditional edit",
            ),
        ]),
    )
    .await;
    apply_additional_edits.await.unwrap();
    cx.assert_editor_state(indoc! {"
        one.second_completionˇ
        two
        three
        additional edit
    "});

    cx.set_state(indoc! {"
        one.second_completion
        twoˇ
        threeˇ
        additional edit
    "});
    cx.simulate_keystroke(" ");
    assert!(cx.editor(|e, _| e.context_menu.is_none()));
    cx.simulate_keystroke("s");
    assert!(cx.editor(|e, _| e.context_menu.is_none()));

    cx.assert_editor_state(indoc! {"
        one.second_completion
        two sˇ
        three sˇ
        additional edit
    "});
    handle_completion_request(
        &mut cx,
        indoc! {"
            one.second_completion
            two s
            three <s|>
            additional edit
        "},
        vec!["fourth_completion", "fifth_completion", "sixth_completion"],
    )
    .await;
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;

    cx.simulate_keystroke("i");

    handle_completion_request(
        &mut cx,
        indoc! {"
            one.second_completion
            two si
            three <si|>
            additional edit
        "},
        vec!["fourth_completion", "fifth_completion", "sixth_completion"],
    )
    .await;
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;

    let apply_additional_edits = cx.update_editor(|editor, cx| {
        editor
            .confirm_completion(&ConfirmCompletion::default(), cx)
            .unwrap()
    });
    cx.assert_editor_state(indoc! {"
        one.second_completion
        two sixth_completionˇ
        three sixth_completionˇ
        additional edit
    "});

    handle_resolve_completion_request(&mut cx, None).await;
    apply_additional_edits.await.unwrap();

    cx.update(|cx| {
        cx.update_global::<Settings, _, _>(|settings, _| {
            settings.show_completions_on_input = false;
        })
    });
    cx.set_state("editorˇ");
    cx.simulate_keystroke(".");
    assert!(cx.editor(|e, _| e.context_menu.is_none()));
    cx.simulate_keystroke("c");
    cx.simulate_keystroke("l");
    cx.simulate_keystroke("o");
    cx.assert_editor_state("editor.cloˇ");
    assert!(cx.editor(|e, _| e.context_menu.is_none()));
    cx.update_editor(|editor, cx| {
        editor.show_completions(&ShowCompletions, cx);
    });
    handle_completion_request(&mut cx, "editor.<clo|>", vec!["close", "clobber"]).await;
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    let apply_additional_edits = cx.update_editor(|editor, cx| {
        editor
            .confirm_completion(&ConfirmCompletion::default(), cx)
            .unwrap()
    });
    cx.assert_editor_state("editor.closeˇ");
    handle_resolve_completion_request(&mut cx, None).await;
    apply_additional_edits.await.unwrap();

    // Handle completion request passing a marked string specifying where the completion
    // should be triggered from using '|' character, what range should be replaced, and what completions
    // should be returned using '<' and '>' to delimit the range
    async fn handle_completion_request<'a>(
        cx: &mut EditorLspTestContext<'a>,
        marked_string: &str,
        completions: Vec<&'static str>,
    ) {
        let complete_from_marker: TextRangeMarker = '|'.into();
        let replace_range_marker: TextRangeMarker = ('<', '>').into();
        let (_, mut marked_ranges) = marked_text_ranges_by(
            marked_string,
            vec![complete_from_marker.clone(), replace_range_marker.clone()],
        );

        let complete_from_position =
            cx.to_lsp(marked_ranges.remove(&complete_from_marker).unwrap()[0].start);
        let replace_range =
            cx.to_lsp_range(marked_ranges.remove(&replace_range_marker).unwrap()[0].clone());

        cx.handle_request::<lsp::request::Completion, _, _>(move |url, params, _| {
            let completions = completions.clone();
            async move {
                assert_eq!(params.text_document_position.text_document.uri, url.clone());
                assert_eq!(
                    params.text_document_position.position,
                    complete_from_position
                );
                Ok(Some(lsp::CompletionResponse::Array(
                    completions
                        .iter()
                        .map(|completion_text| lsp::CompletionItem {
                            label: completion_text.to_string(),
                            text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                                range: replace_range,
                                new_text: completion_text.to_string(),
                            })),
                            ..Default::default()
                        })
                        .collect(),
                )))
            }
        })
        .next()
        .await;
    }

    async fn handle_resolve_completion_request<'a>(
        cx: &mut EditorLspTestContext<'a>,
        edits: Option<Vec<(&'static str, &'static str)>>,
    ) {
        let edits = edits.map(|edits| {
            edits
                .iter()
                .map(|(marked_string, new_text)| {
                    let (_, marked_ranges) = marked_text_ranges(marked_string, false);
                    let replace_range = cx.to_lsp_range(marked_ranges[0].clone());
                    lsp::TextEdit::new(replace_range, new_text.to_string())
                })
                .collect::<Vec<_>>()
        });

        cx.handle_request::<lsp::request::ResolveCompletionItem, _, _>(move |_, _, _| {
            let edits = edits.clone();
            async move {
                Ok(lsp::CompletionItem {
                    additional_text_edits: edits,
                    ..Default::default()
                })
            }
        })
        .next()
        .await;
    }
}

#[gpui::test]
async fn test_toggle_comment(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| cx.set_global(Settings::test(cx)));
    let language = Arc::new(Language::new(
        LanguageConfig {
            line_comment: Some("// ".into()),
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    ));

    let text = "
        fn a() {
            //b();
            // c();
            //  d();
        }
    "
    .unindent();

    let buffer = cx.add_model(|cx| Buffer::new(0, text, cx).with_language(language, cx));
    let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (_, view) = cx.add_window(|cx| build_editor(buffer, cx));

    view.update(cx, |editor, cx| {
        // If multiple selections intersect a line, the line is only
        // toggled once.
        editor.change_selections(None, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(1, 3)..DisplayPoint::new(2, 3),
                DisplayPoint::new(3, 5)..DisplayPoint::new(3, 6),
            ])
        });
        editor.toggle_comments(&ToggleComments::default(), cx);
        assert_eq!(
            editor.text(cx),
            "
                fn a() {
                    b();
                    c();
                     d();
                }
            "
            .unindent()
        );

        // The comment prefix is inserted at the same column for every line
        // in a selection.
        editor.change_selections(None, cx, |s| {
            s.select_display_ranges([DisplayPoint::new(1, 3)..DisplayPoint::new(3, 6)])
        });
        editor.toggle_comments(&ToggleComments::default(), cx);
        assert_eq!(
            editor.text(cx),
            "
                fn a() {
                    // b();
                    // c();
                    //  d();
                }
            "
            .unindent()
        );

        // If a selection ends at the beginning of a line, that line is not toggled.
        editor.change_selections(None, cx, |s| {
            s.select_display_ranges([DisplayPoint::new(2, 0)..DisplayPoint::new(3, 0)])
        });
        editor.toggle_comments(&ToggleComments::default(), cx);
        assert_eq!(
            editor.text(cx),
            "
                fn a() {
                    // b();
                    c();
                    //  d();
                }
            "
            .unindent()
        );
    });
}

#[gpui::test]
async fn test_advance_downward_on_toggle_comment(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);
    cx.update(|cx| cx.set_global(Settings::test(cx)));

    let language = Arc::new(Language::new(
        LanguageConfig {
            line_comment: Some("// ".into()),
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    ));

    let registry = Arc::new(LanguageRegistry::test());
    registry.add(language.clone());

    cx.update_buffer(|buffer, cx| {
        buffer.set_language_registry(registry);
        buffer.set_language(Some(language), cx);
    });

    let toggle_comments = &ToggleComments {
        advance_downwards: true,
    };

    // Single cursor on one line -> advance
    // Cursor moves horizontally 3 characters as well on non-blank line
    cx.set_state(indoc!(
        "fn a() {
             ˇdog();
             cat();
        }"
    ));
    cx.update_editor(|editor, cx| {
        editor.toggle_comments(toggle_comments, cx);
    });
    cx.assert_editor_state(indoc!(
        "fn a() {
             // dog();
             catˇ();
        }"
    ));

    // Single selection on one line -> don't advance
    cx.set_state(indoc!(
        "fn a() {
             «dog()ˇ»;
             cat();
        }"
    ));
    cx.update_editor(|editor, cx| {
        editor.toggle_comments(toggle_comments, cx);
    });
    cx.assert_editor_state(indoc!(
        "fn a() {
             // «dog()ˇ»;
             cat();
        }"
    ));

    // Multiple cursors on one line -> advance
    cx.set_state(indoc!(
        "fn a() {
             ˇdˇog();
             cat();
        }"
    ));
    cx.update_editor(|editor, cx| {
        editor.toggle_comments(toggle_comments, cx);
    });
    cx.assert_editor_state(indoc!(
        "fn a() {
             // dog();
             catˇ(ˇ);
        }"
    ));

    // Multiple cursors on one line, with selection -> don't advance
    cx.set_state(indoc!(
        "fn a() {
             ˇdˇog«()ˇ»;
             cat();
        }"
    ));
    cx.update_editor(|editor, cx| {
        editor.toggle_comments(toggle_comments, cx);
    });
    cx.assert_editor_state(indoc!(
        "fn a() {
             // ˇdˇog«()ˇ»;
             cat();
        }"
    ));

    // Single cursor on one line -> advance
    // Cursor moves to column 0 on blank line
    cx.set_state(indoc!(
        "fn a() {
             ˇdog();

             cat();
        }"
    ));
    cx.update_editor(|editor, cx| {
        editor.toggle_comments(toggle_comments, cx);
    });
    cx.assert_editor_state(indoc!(
        "fn a() {
             // dog();
        ˇ
             cat();
        }"
    ));

    // Single cursor on one line -> advance
    // Cursor starts and ends at column 0
    cx.set_state(indoc!(
        "fn a() {
         ˇ    dog();
             cat();
        }"
    ));
    cx.update_editor(|editor, cx| {
        editor.toggle_comments(toggle_comments, cx);
    });
    cx.assert_editor_state(indoc!(
        "fn a() {
             // dog();
         ˇ    cat();
        }"
    ));
}

#[gpui::test]
async fn test_toggle_block_comment(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);

    let html_language = Arc::new(
        Language::new(
            LanguageConfig {
                name: "HTML".into(),
                block_comment: Some(("<!-- ".into(), " -->".into())),
                ..Default::default()
            },
            Some(tree_sitter_html::language()),
        )
        .with_injection_query(
            r#"
            (script_element
                (raw_text) @content
                (#set! "language" "javascript"))
            "#,
        )
        .unwrap(),
    );

    let javascript_language = Arc::new(Language::new(
        LanguageConfig {
            name: "JavaScript".into(),
            line_comment: Some("// ".into()),
            ..Default::default()
        },
        Some(tree_sitter_javascript::language()),
    ));

    let registry = Arc::new(LanguageRegistry::test());
    registry.add(html_language.clone());
    registry.add(javascript_language.clone());

    cx.update_buffer(|buffer, cx| {
        buffer.set_language_registry(registry);
        buffer.set_language(Some(html_language), cx);
    });

    // Toggle comments for empty selections
    cx.set_state(
        &r#"
            <p>A</p>ˇ
            <p>B</p>ˇ
            <p>C</p>ˇ
        "#
        .unindent(),
    );
    cx.update_editor(|editor, cx| editor.toggle_comments(&ToggleComments::default(), cx));
    cx.assert_editor_state(
        &r#"
            <!-- <p>A</p>ˇ -->
            <!-- <p>B</p>ˇ -->
            <!-- <p>C</p>ˇ -->
        "#
        .unindent(),
    );
    cx.update_editor(|editor, cx| editor.toggle_comments(&ToggleComments::default(), cx));
    cx.assert_editor_state(
        &r#"
            <p>A</p>ˇ
            <p>B</p>ˇ
            <p>C</p>ˇ
        "#
        .unindent(),
    );

    // Toggle comments for mixture of empty and non-empty selections, where
    // multiple selections occupy a given line.
    cx.set_state(
        &r#"
            <p>A«</p>
            <p>ˇ»B</p>ˇ
            <p>C«</p>
            <p>ˇ»D</p>ˇ
        "#
        .unindent(),
    );

    cx.update_editor(|editor, cx| editor.toggle_comments(&ToggleComments::default(), cx));
    cx.assert_editor_state(
        &r#"
            <!-- <p>A«</p>
            <p>ˇ»B</p>ˇ -->
            <!-- <p>C«</p>
            <p>ˇ»D</p>ˇ -->
        "#
        .unindent(),
    );
    cx.update_editor(|editor, cx| editor.toggle_comments(&ToggleComments::default(), cx));
    cx.assert_editor_state(
        &r#"
            <p>A«</p>
            <p>ˇ»B</p>ˇ
            <p>C«</p>
            <p>ˇ»D</p>ˇ
        "#
        .unindent(),
    );

    // Toggle comments when different languages are active for different
    // selections.
    cx.set_state(
        &r#"
            ˇ<script>
                ˇvar x = new Y();
            ˇ</script>
        "#
        .unindent(),
    );
    cx.foreground().run_until_parked();
    cx.update_editor(|editor, cx| editor.toggle_comments(&ToggleComments::default(), cx));
    cx.assert_editor_state(
        &r#"
            <!-- ˇ<script> -->
                // ˇvar x = new Y();
            <!-- ˇ</script> -->
        "#
        .unindent(),
    );
}

#[gpui::test]
fn test_editing_disjoint_excerpts(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(3, 4, 'a'), cx));
    let multibuffer = cx.add_model(|cx| {
        let mut multibuffer = MultiBuffer::new(0);
        multibuffer.push_excerpts(
            buffer.clone(),
            [
                ExcerptRange {
                    context: Point::new(0, 0)..Point::new(0, 4),
                    primary: None,
                },
                ExcerptRange {
                    context: Point::new(1, 0)..Point::new(1, 4),
                    primary: None,
                },
            ],
            cx,
        );
        multibuffer
    });

    assert_eq!(multibuffer.read(cx).read(cx).text(), "aaaa\nbbbb");

    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(multibuffer, cx));
    view.update(cx, |view, cx| {
        assert_eq!(view.text(cx), "aaaa\nbbbb");
        view.change_selections(None, cx, |s| {
            s.select_ranges([
                Point::new(0, 0)..Point::new(0, 0),
                Point::new(1, 0)..Point::new(1, 0),
            ])
        });

        view.handle_input("X", cx);
        assert_eq!(view.text(cx), "Xaaaa\nXbbbb");
        assert_eq!(
            view.selections.ranges(cx),
            [
                Point::new(0, 1)..Point::new(0, 1),
                Point::new(1, 1)..Point::new(1, 1),
            ]
        )
    });
}

#[gpui::test]
fn test_editing_overlapping_excerpts(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let markers = vec![('[', ']').into(), ('(', ')').into()];
    let (initial_text, mut excerpt_ranges) = marked_text_ranges_by(
        indoc! {"
            [aaaa
            (bbbb]
            cccc)",
        },
        markers.clone(),
    );
    let excerpt_ranges = markers.into_iter().map(|marker| {
        let context = excerpt_ranges.remove(&marker).unwrap()[0].clone();
        ExcerptRange {
            context,
            primary: None,
        }
    });
    let buffer = cx.add_model(|cx| Buffer::new(0, initial_text, cx));
    let multibuffer = cx.add_model(|cx| {
        let mut multibuffer = MultiBuffer::new(0);
        multibuffer.push_excerpts(buffer, excerpt_ranges, cx);
        multibuffer
    });

    let (_, view) = cx.add_window(Default::default(), |cx| build_editor(multibuffer, cx));
    view.update(cx, |view, cx| {
        let (expected_text, selection_ranges) = marked_text_ranges(
            indoc! {"
                aaaa
                bˇbbb
                bˇbbˇb
                cccc"
            },
            true,
        );
        assert_eq!(view.text(cx), expected_text);
        view.change_selections(None, cx, |s| s.select_ranges(selection_ranges));

        view.handle_input("X", cx);

        let (expected_text, expected_selections) = marked_text_ranges(
            indoc! {"
                aaaa
                bXˇbbXb
                bXˇbbXˇb
                cccc"
            },
            false,
        );
        assert_eq!(view.text(cx), expected_text);
        assert_eq!(view.selections.ranges(cx), expected_selections);

        view.newline(&Newline, cx);
        let (expected_text, expected_selections) = marked_text_ranges(
            indoc! {"
                aaaa
                bX
                ˇbbX
                b
                bX
                ˇbbX
                ˇb
                cccc"
            },
            false,
        );
        assert_eq!(view.text(cx), expected_text);
        assert_eq!(view.selections.ranges(cx), expected_selections);
    });
}

#[gpui::test]
fn test_refresh_selections(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(3, 4, 'a'), cx));
    let mut excerpt1_id = None;
    let multibuffer = cx.add_model(|cx| {
        let mut multibuffer = MultiBuffer::new(0);
        excerpt1_id = multibuffer
            .push_excerpts(
                buffer.clone(),
                [
                    ExcerptRange {
                        context: Point::new(0, 0)..Point::new(1, 4),
                        primary: None,
                    },
                    ExcerptRange {
                        context: Point::new(1, 0)..Point::new(2, 4),
                        primary: None,
                    },
                ],
                cx,
            )
            .into_iter()
            .next();
        multibuffer
    });
    assert_eq!(
        multibuffer.read(cx).read(cx).text(),
        "aaaa\nbbbb\nbbbb\ncccc"
    );
    let (_, editor) = cx.add_window(Default::default(), |cx| {
        let mut editor = build_editor(multibuffer.clone(), cx);
        let snapshot = editor.snapshot(cx);
        editor.change_selections(None, cx, |s| {
            s.select_ranges([Point::new(1, 3)..Point::new(1, 3)])
        });
        editor.begin_selection(Point::new(2, 1).to_display_point(&snapshot), true, 1, cx);
        assert_eq!(
            editor.selections.ranges(cx),
            [
                Point::new(1, 3)..Point::new(1, 3),
                Point::new(2, 1)..Point::new(2, 1),
            ]
        );
        editor
    });

    // Refreshing selections is a no-op when excerpts haven't changed.
    editor.update(cx, |editor, cx| {
        editor.change_selections(None, cx, |s| s.refresh());
        assert_eq!(
            editor.selections.ranges(cx),
            [
                Point::new(1, 3)..Point::new(1, 3),
                Point::new(2, 1)..Point::new(2, 1),
            ]
        );
    });

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.remove_excerpts([excerpt1_id.unwrap()], cx);
    });
    editor.update(cx, |editor, cx| {
        // Removing an excerpt causes the first selection to become degenerate.
        assert_eq!(
            editor.selections.ranges(cx),
            [
                Point::new(0, 0)..Point::new(0, 0),
                Point::new(0, 1)..Point::new(0, 1)
            ]
        );

        // Refreshing selections will relocate the first selection to the original buffer
        // location.
        editor.change_selections(None, cx, |s| s.refresh());
        assert_eq!(
            editor.selections.ranges(cx),
            [
                Point::new(0, 1)..Point::new(0, 1),
                Point::new(0, 3)..Point::new(0, 3)
            ]
        );
        assert!(editor.selections.pending_anchor().is_some());
    });
}

#[gpui::test]
fn test_refresh_selections_while_selecting_with_mouse(cx: &mut gpui::MutableAppContext) {
    cx.set_global(Settings::test(cx));
    let buffer = cx.add_model(|cx| Buffer::new(0, sample_text(3, 4, 'a'), cx));
    let mut excerpt1_id = None;
    let multibuffer = cx.add_model(|cx| {
        let mut multibuffer = MultiBuffer::new(0);
        excerpt1_id = multibuffer
            .push_excerpts(
                buffer.clone(),
                [
                    ExcerptRange {
                        context: Point::new(0, 0)..Point::new(1, 4),
                        primary: None,
                    },
                    ExcerptRange {
                        context: Point::new(1, 0)..Point::new(2, 4),
                        primary: None,
                    },
                ],
                cx,
            )
            .into_iter()
            .next();
        multibuffer
    });
    assert_eq!(
        multibuffer.read(cx).read(cx).text(),
        "aaaa\nbbbb\nbbbb\ncccc"
    );
    let (_, editor) = cx.add_window(Default::default(), |cx| {
        let mut editor = build_editor(multibuffer.clone(), cx);
        let snapshot = editor.snapshot(cx);
        editor.begin_selection(Point::new(1, 3).to_display_point(&snapshot), false, 1, cx);
        assert_eq!(
            editor.selections.ranges(cx),
            [Point::new(1, 3)..Point::new(1, 3)]
        );
        editor
    });

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.remove_excerpts([excerpt1_id.unwrap()], cx);
    });
    editor.update(cx, |editor, cx| {
        assert_eq!(
            editor.selections.ranges(cx),
            [Point::new(0, 0)..Point::new(0, 0)]
        );

        // Ensure we don't panic when selections are refreshed and that the pending selection is finalized.
        editor.change_selections(None, cx, |s| s.refresh());
        assert_eq!(
            editor.selections.ranges(cx),
            [Point::new(0, 3)..Point::new(0, 3)]
        );
        assert!(editor.selections.pending_anchor().is_some());
    });
}

#[gpui::test]
async fn test_extra_newline_insertion(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| cx.set_global(Settings::test(cx)));
    let language = Arc::new(
        Language::new(
            LanguageConfig {
                brackets: BracketPairConfig {
                    pairs: vec![
                        BracketPair {
                            start: "{".to_string(),
                            end: "}".to_string(),
                            close: true,
                            newline: true,
                        },
                        BracketPair {
                            start: "/* ".to_string(),
                            end: " */".to_string(),
                            close: true,
                            newline: true,
                        },
                    ],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        )
        .with_indents_query("")
        .unwrap(),
    );

    let text = concat!(
        "{   }\n",     //
        "  x\n",       //
        "  /*   */\n", //
        "x\n",         //
        "{{} }\n",     //
    );

    let buffer = cx.add_model(|cx| Buffer::new(0, text, cx).with_language(language, cx));
    let buffer = cx.add_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (_, view) = cx.add_window(|cx| build_editor(buffer, cx));
    view.condition(cx, |view, cx| !view.buffer.read(cx).is_parsing(cx))
        .await;

    view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 3),
                DisplayPoint::new(2, 5)..DisplayPoint::new(2, 5),
                DisplayPoint::new(4, 4)..DisplayPoint::new(4, 4),
            ])
        });
        view.newline(&Newline, cx);

        assert_eq!(
            view.buffer().read(cx).read(cx).text(),
            concat!(
                "{ \n",    // Suppress rustfmt
                "\n",      //
                "}\n",     //
                "  x\n",   //
                "  /* \n", //
                "  \n",    //
                "  */\n",  //
                "x\n",     //
                "{{} \n",  //
                "}\n",     //
            )
        );
    });
}

#[gpui::test]
fn test_highlighted_ranges(cx: &mut gpui::MutableAppContext) {
    let buffer = MultiBuffer::build_simple(&sample_text(16, 8, 'a'), cx);

    cx.set_global(Settings::test(cx));
    let (_, editor) = cx.add_window(Default::default(), |cx| build_editor(buffer.clone(), cx));

    editor.update(cx, |editor, cx| {
        struct Type1;
        struct Type2;

        let buffer = buffer.read(cx).snapshot(cx);

        let anchor_range =
            |range: Range<Point>| buffer.anchor_after(range.start)..buffer.anchor_after(range.end);

        editor.highlight_background::<Type1>(
            vec![
                anchor_range(Point::new(2, 1)..Point::new(2, 3)),
                anchor_range(Point::new(4, 2)..Point::new(4, 4)),
                anchor_range(Point::new(6, 3)..Point::new(6, 5)),
                anchor_range(Point::new(8, 4)..Point::new(8, 6)),
            ],
            |_| Color::red(),
            cx,
        );
        editor.highlight_background::<Type2>(
            vec![
                anchor_range(Point::new(3, 2)..Point::new(3, 5)),
                anchor_range(Point::new(5, 3)..Point::new(5, 6)),
                anchor_range(Point::new(7, 4)..Point::new(7, 7)),
                anchor_range(Point::new(9, 5)..Point::new(9, 8)),
            ],
            |_| Color::green(),
            cx,
        );

        let snapshot = editor.snapshot(cx);
        let mut highlighted_ranges = editor.background_highlights_in_range(
            anchor_range(Point::new(3, 4)..Point::new(7, 4)),
            &snapshot,
            cx.global::<Settings>().theme.as_ref(),
        );
        // Enforce a consistent ordering based on color without relying on the ordering of the
        // highlight's `TypeId` which is non-deterministic.
        highlighted_ranges.sort_unstable_by_key(|(_, color)| *color);
        assert_eq!(
            highlighted_ranges,
            &[
                (
                    DisplayPoint::new(3, 2)..DisplayPoint::new(3, 5),
                    Color::green(),
                ),
                (
                    DisplayPoint::new(5, 3)..DisplayPoint::new(5, 6),
                    Color::green(),
                ),
                (
                    DisplayPoint::new(4, 2)..DisplayPoint::new(4, 4),
                    Color::red(),
                ),
                (
                    DisplayPoint::new(6, 3)..DisplayPoint::new(6, 5),
                    Color::red(),
                ),
            ]
        );
        assert_eq!(
            editor.background_highlights_in_range(
                anchor_range(Point::new(5, 6)..Point::new(6, 4)),
                &snapshot,
                cx.global::<Settings>().theme.as_ref(),
            ),
            &[(
                DisplayPoint::new(6, 3)..DisplayPoint::new(6, 5),
                Color::red(),
            )]
        );
    });
}

#[gpui::test]
async fn test_following(cx: &mut gpui::TestAppContext) {
    Settings::test_async(cx);
    let fs = FakeFs::new(cx.background());
    let project = Project::test(fs, ["/file.rs".as_ref()], cx).await;

    let buffer = project.update(cx, |project, cx| {
        let buffer = project
            .create_buffer(&sample_text(16, 8, 'a'), None, cx)
            .unwrap();
        cx.add_model(|cx| MultiBuffer::singleton(buffer, cx))
    });
    let (_, leader) = cx.add_window(|cx| build_editor(buffer.clone(), cx));
    let (_, follower) = cx.update(|cx| {
        cx.add_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(RectF::from_points(vec2f(0., 0.), vec2f(10., 80.))),
                ..Default::default()
            },
            |cx| build_editor(buffer.clone(), cx),
        )
    });

    let is_still_following = Rc::new(RefCell::new(true));
    let pending_update = Rc::new(RefCell::new(None));
    follower.update(cx, {
        let update = pending_update.clone();
        let is_still_following = is_still_following.clone();
        |_, cx| {
            cx.subscribe(&leader, move |_, leader, event, cx| {
                leader
                    .read(cx)
                    .add_event_to_update_proto(event, &mut *update.borrow_mut(), cx);
            })
            .detach();

            cx.subscribe(&follower, move |_, _, event, cx| {
                if Editor::should_unfollow_on_event(event, cx) {
                    *is_still_following.borrow_mut() = false;
                }
            })
            .detach();
        }
    });

    // Update the selections only
    leader.update(cx, |leader, cx| {
        leader.change_selections(None, cx, |s| s.select_ranges([1..1]));
    });
    follower
        .update(cx, |follower, cx| {
            follower.apply_update_proto(&project, pending_update.borrow_mut().take().unwrap(), cx)
        })
        .await
        .unwrap();
    follower.read_with(cx, |follower, cx| {
        assert_eq!(follower.selections.ranges(cx), vec![1..1]);
    });
    assert_eq!(*is_still_following.borrow(), true);

    // Update the scroll position only
    leader.update(cx, |leader, cx| {
        leader.set_scroll_position(vec2f(1.5, 3.5), cx);
    });
    follower
        .update(cx, |follower, cx| {
            follower.apply_update_proto(&project, pending_update.borrow_mut().take().unwrap(), cx)
        })
        .await
        .unwrap();
    assert_eq!(
        follower.update(cx, |follower, cx| follower.scroll_position(cx)),
        vec2f(1.5, 3.5)
    );
    assert_eq!(*is_still_following.borrow(), true);

    // Update the selections and scroll position. The follower's scroll position is updated
    // via autoscroll, not via the leader's exact scroll position.
    leader.update(cx, |leader, cx| {
        leader.change_selections(None, cx, |s| s.select_ranges([0..0]));
        leader.request_autoscroll(Autoscroll::newest(), cx);
        leader.set_scroll_position(vec2f(1.5, 3.5), cx);
    });
    follower
        .update(cx, |follower, cx| {
            follower.apply_update_proto(&project, pending_update.borrow_mut().take().unwrap(), cx)
        })
        .await
        .unwrap();
    follower.update(cx, |follower, cx| {
        assert_eq!(follower.scroll_position(cx), vec2f(1.5, 0.0));
        assert_eq!(follower.selections.ranges(cx), vec![0..0]);
    });
    assert_eq!(*is_still_following.borrow(), true);

    // Creating a pending selection that precedes another selection
    leader.update(cx, |leader, cx| {
        leader.change_selections(None, cx, |s| s.select_ranges([1..1]));
        leader.begin_selection(DisplayPoint::new(0, 0), true, 1, cx);
    });
    follower
        .update(cx, |follower, cx| {
            follower.apply_update_proto(&project, pending_update.borrow_mut().take().unwrap(), cx)
        })
        .await
        .unwrap();
    follower.read_with(cx, |follower, cx| {
        assert_eq!(follower.selections.ranges(cx), vec![0..0, 1..1]);
    });
    assert_eq!(*is_still_following.borrow(), true);

    // Extend the pending selection so that it surrounds another selection
    leader.update(cx, |leader, cx| {
        leader.extend_selection(DisplayPoint::new(0, 2), 1, cx);
    });
    follower
        .update(cx, |follower, cx| {
            follower.apply_update_proto(&project, pending_update.borrow_mut().take().unwrap(), cx)
        })
        .await
        .unwrap();
    follower.read_with(cx, |follower, cx| {
        assert_eq!(follower.selections.ranges(cx), vec![0..2]);
    });

    // Scrolling locally breaks the follow
    follower.update(cx, |follower, cx| {
        let top_anchor = follower.buffer().read(cx).read(cx).anchor_after(0);
        follower.set_scroll_anchor(
            ScrollAnchor {
                top_anchor,
                offset: vec2f(0.0, 0.5),
            },
            cx,
        );
    });
    assert_eq!(*is_still_following.borrow(), false);
}

#[gpui::test]
async fn test_following_with_multiple_excerpts(cx: &mut gpui::TestAppContext) {
    Settings::test_async(cx);
    let fs = FakeFs::new(cx.background());
    let project = Project::test(fs, ["/file.rs".as_ref()], cx).await;
    let (_, pane) = cx.add_window(|cx| Pane::new(0, None, || &[], cx));

    let leader = pane.update(cx, |_, cx| {
        let multibuffer = cx.add_model(|_| MultiBuffer::new(0));
        cx.add_view(|cx| build_editor(multibuffer.clone(), cx))
    });

    // Start following the editor when it has no excerpts.
    let mut state_message = leader.update(cx, |leader, cx| leader.to_state_proto(cx));
    let follower_1 = cx
        .update(|cx| {
            Editor::from_state_proto(
                pane.clone(),
                project.clone(),
                ViewId {
                    creator: Default::default(),
                    id: 0,
                },
                &mut state_message,
                cx,
            )
        })
        .unwrap()
        .await
        .unwrap();

    let update_message = Rc::new(RefCell::new(None));
    follower_1.update(cx, {
        let update = update_message.clone();
        |_, cx| {
            cx.subscribe(&leader, move |_, leader, event, cx| {
                leader
                    .read(cx)
                    .add_event_to_update_proto(event, &mut *update.borrow_mut(), cx);
            })
            .detach();
        }
    });

    let (buffer_1, buffer_2) = project.update(cx, |project, cx| {
        (
            project
                .create_buffer("abc\ndef\nghi\njkl\n", None, cx)
                .unwrap(),
            project
                .create_buffer("mno\npqr\nstu\nvwx\n", None, cx)
                .unwrap(),
        )
    });

    // Insert some excerpts.
    leader.update(cx, |leader, cx| {
        leader.buffer.update(cx, |multibuffer, cx| {
            let excerpt_ids = multibuffer.push_excerpts(
                buffer_1.clone(),
                [
                    ExcerptRange {
                        context: 1..6,
                        primary: None,
                    },
                    ExcerptRange {
                        context: 12..15,
                        primary: None,
                    },
                    ExcerptRange {
                        context: 0..3,
                        primary: None,
                    },
                ],
                cx,
            );
            multibuffer.insert_excerpts_after(
                excerpt_ids[0],
                buffer_2.clone(),
                [
                    ExcerptRange {
                        context: 8..12,
                        primary: None,
                    },
                    ExcerptRange {
                        context: 0..6,
                        primary: None,
                    },
                ],
                cx,
            );
        });
    });

    // Apply the update of adding the excerpts.
    follower_1
        .update(cx, |follower, cx| {
            follower.apply_update_proto(&project, update_message.borrow().clone().unwrap(), cx)
        })
        .await
        .unwrap();
    assert_eq!(
        follower_1.read_with(cx, Editor::text),
        leader.read_with(cx, Editor::text)
    );
    update_message.borrow_mut().take();

    // Start following separately after it already has excerpts.
    let mut state_message = leader.update(cx, |leader, cx| leader.to_state_proto(cx));
    let follower_2 = cx
        .update(|cx| {
            Editor::from_state_proto(
                pane.clone(),
                project.clone(),
                ViewId {
                    creator: Default::default(),
                    id: 0,
                },
                &mut state_message,
                cx,
            )
        })
        .unwrap()
        .await
        .unwrap();
    assert_eq!(
        follower_2.read_with(cx, Editor::text),
        leader.read_with(cx, Editor::text)
    );

    // Remove some excerpts.
    leader.update(cx, |leader, cx| {
        leader.buffer.update(cx, |multibuffer, cx| {
            let excerpt_ids = multibuffer.excerpt_ids();
            multibuffer.remove_excerpts([excerpt_ids[1], excerpt_ids[2]], cx);
            multibuffer.remove_excerpts([excerpt_ids[0]], cx);
        });
    });

    // Apply the update of removing the excerpts.
    follower_1
        .update(cx, |follower, cx| {
            follower.apply_update_proto(&project, update_message.borrow().clone().unwrap(), cx)
        })
        .await
        .unwrap();
    follower_2
        .update(cx, |follower, cx| {
            follower.apply_update_proto(&project, update_message.borrow().clone().unwrap(), cx)
        })
        .await
        .unwrap();
    update_message.borrow_mut().take();
    assert_eq!(
        follower_1.read_with(cx, Editor::text),
        leader.read_with(cx, Editor::text)
    );
}

#[test]
fn test_combine_syntax_and_fuzzy_match_highlights() {
    let string = "abcdefghijklmnop";
    let syntax_ranges = [
        (
            0..3,
            HighlightStyle {
                color: Some(Color::red()),
                ..Default::default()
            },
        ),
        (
            4..8,
            HighlightStyle {
                color: Some(Color::green()),
                ..Default::default()
            },
        ),
    ];
    let match_indices = [4, 6, 7, 8];
    assert_eq!(
        combine_syntax_and_fuzzy_match_highlights(
            string,
            Default::default(),
            syntax_ranges.into_iter(),
            &match_indices,
        ),
        &[
            (
                0..3,
                HighlightStyle {
                    color: Some(Color::red()),
                    ..Default::default()
                },
            ),
            (
                4..5,
                HighlightStyle {
                    color: Some(Color::green()),
                    weight: Some(fonts::Weight::BOLD),
                    ..Default::default()
                },
            ),
            (
                5..6,
                HighlightStyle {
                    color: Some(Color::green()),
                    ..Default::default()
                },
            ),
            (
                6..8,
                HighlightStyle {
                    color: Some(Color::green()),
                    weight: Some(fonts::Weight::BOLD),
                    ..Default::default()
                },
            ),
            (
                8..9,
                HighlightStyle {
                    weight: Some(fonts::Weight::BOLD),
                    ..Default::default()
                },
            ),
        ]
    );
}

#[gpui::test]
async fn go_to_hunk(deterministic: Arc<Deterministic>, cx: &mut gpui::TestAppContext) {
    let mut cx = EditorTestContext::new(cx);

    let diff_base = r#"
        use some::mod;

        const A: u32 = 42;

        fn main() {
            println!("hello");

            println!("world");
        }
        "#
    .unindent();

    // Edits are modified, removed, modified, added
    cx.set_state(
        &r#"
        use some::modified;

        ˇ
        fn main() {
            println!("hello there");

            println!("around the");
            println!("world");
        }
        "#
        .unindent(),
    );

    cx.set_diff_base(Some(&diff_base));
    deterministic.run_until_parked();

    cx.update_editor(|editor, cx| {
        //Wrap around the bottom of the buffer
        for _ in 0..3 {
            editor.go_to_hunk(&GoToHunk, cx);
        }
    });

    cx.assert_editor_state(
        &r#"
        ˇuse some::modified;


        fn main() {
            println!("hello there");

            println!("around the");
            println!("world");
        }
        "#
        .unindent(),
    );

    cx.update_editor(|editor, cx| {
        //Wrap around the top of the buffer
        for _ in 0..2 {
            editor.go_to_prev_hunk(&GoToPrevHunk, cx);
        }
    });

    cx.assert_editor_state(
        &r#"
        use some::modified;


        fn main() {
        ˇ    println!("hello there");

            println!("around the");
            println!("world");
        }
        "#
        .unindent(),
    );

    cx.update_editor(|editor, cx| {
        editor.fold(&Fold, cx);

        //Make sure that the fold only gets one hunk
        for _ in 0..4 {
            editor.go_to_hunk(&GoToHunk, cx);
        }
    });

    cx.assert_editor_state(
        &r#"
        ˇuse some::modified;


        fn main() {
            println!("hello there");

            println!("around the");
            println!("world");
        }
        "#
        .unindent(),
    );
}

#[test]
fn test_split_words() {
    fn split<'a>(text: &'a str) -> Vec<&'a str> {
        split_words(text).collect()
    }

    assert_eq!(split("HelloWorld"), &["Hello", "World"]);
    assert_eq!(split("hello_world"), &["hello_", "world"]);
    assert_eq!(split("_hello_world_"), &["_", "hello_", "world_"]);
    assert_eq!(split("Hello_World"), &["Hello_", "World"]);
    assert_eq!(split("helloWOrld"), &["hello", "WOrld"]);
    assert_eq!(split("helloworld"), &["helloworld"]);
}

#[gpui::test]
async fn test_move_to_enclosing_bracket(cx: &mut gpui::TestAppContext) {
    let mut cx = EditorLspTestContext::new_typescript(Default::default(), cx).await;
    let mut assert = |before, after| {
        let _state_context = cx.set_state(before);
        cx.update_editor(|editor, cx| {
            editor.move_to_enclosing_bracket(&MoveToEnclosingBracket, cx)
        });
        cx.assert_editor_state(after);
    };

    // Outside bracket jumps to outside of matching bracket
    assert("console.logˇ(var);", "console.log(var)ˇ;");
    assert("console.log(var)ˇ;", "console.logˇ(var);");

    // Inside bracket jumps to inside of matching bracket
    assert("console.log(ˇvar);", "console.log(varˇ);");
    assert("console.log(varˇ);", "console.log(ˇvar);");

    // When outside a bracket and inside, favor jumping to the inside bracket
    assert(
        "console.log('foo', [1, 2, 3]ˇ);",
        "console.log(ˇ'foo', [1, 2, 3]);",
    );
    assert(
        "console.log(ˇ'foo', [1, 2, 3]);",
        "console.log('foo', [1, 2, 3]ˇ);",
    );

    // Bias forward if two options are equally likely
    assert(
        "let result = curried_fun()ˇ();",
        "let result = curried_fun()()ˇ;",
    );

    // If directly adjacent to a smaller pair but inside a larger (not adjacent), pick the smaller
    assert(
        indoc! {"
            function test() {
                console.log('test')ˇ
            }"},
        indoc! {"
            function test() {
                console.logˇ('test')
            }"},
    );
}

fn empty_range(row: usize, column: usize) -> Range<DisplayPoint> {
    let point = DisplayPoint::new(row as u32, column as u32);
    point..point
}

fn assert_selection_ranges(marked_text: &str, view: &mut Editor, cx: &mut ViewContext<Editor>) {
    let (text, ranges) = marked_text_ranges(marked_text, true);
    assert_eq!(view.text(cx), text);
    assert_eq!(
        view.selections.ranges(cx),
        ranges,
        "Assert selections are {}",
        marked_text
    );
}
