use super::*;
use crate::{
    scroll::scroll_amount::ScrollAmount,
    test::{
        assert_text_with_selections, build_editor, editor_lsp_test_context::EditorLspTestContext,
        editor_test_context::EditorTestContext, select_ranges,
    },
    JoinLines,
};

use futures::StreamExt;
use gpui::{div, TestAppContext, VisualTestContext, WindowBounds, WindowOptions};
use indoc::indoc;
use language::{
    language_settings::{AllLanguageSettings, AllLanguageSettingsContent, LanguageSettingsContent},
    BracketPairConfig,
    Capability::ReadWrite,
    FakeLspAdapter, LanguageConfig, LanguageConfigOverride, LanguageMatcher, LanguageRegistry,
    Override, Point,
};
use parking_lot::Mutex;
use project::project_settings::{LspSettings, ProjectSettings};
use project::FakeFs;
use serde_json::{self, json};
use std::sync::atomic;
use std::sync::atomic::AtomicUsize;
use std::{cell::RefCell, future::Future, rc::Rc, time::Instant};
use unindent::Unindent;
use util::{
    assert_set_eq,
    test::{marked_text_ranges, marked_text_ranges_by, sample_text, TextRangeMarker},
};
use workspace::{
    item::{FollowEvent, FollowableItem, Item, ItemHandle},
    NavigationEntry, ViewId,
};

#[gpui::test]
fn test_edit_events(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let buffer = cx.new_model(|cx| {
        let mut buffer =
            language::Buffer::new(0, BufferId::new(cx.entity_id().as_u64()).unwrap(), "123456");
        buffer.set_group_interval(Duration::from_secs(1));
        buffer
    });

    let events = Rc::new(RefCell::new(Vec::new()));
    let editor1 = cx.add_window({
        let events = events.clone();
        |cx| {
            let view = cx.view().clone();
            cx.subscribe(&view, move |_, _, event: &EditorEvent, _| {
                if matches!(event, EditorEvent::Edited | EditorEvent::BufferEdited) {
                    events.borrow_mut().push(("editor1", event.clone()));
                }
            })
            .detach();
            Editor::for_buffer(buffer.clone(), None, cx)
        }
    });

    let editor2 = cx.add_window({
        let events = events.clone();
        |cx| {
            cx.subscribe(&cx.view().clone(), move |_, _, event: &EditorEvent, _| {
                if matches!(event, EditorEvent::Edited | EditorEvent::BufferEdited) {
                    events.borrow_mut().push(("editor2", event.clone()));
                }
            })
            .detach();
            Editor::for_buffer(buffer.clone(), None, cx)
        }
    });

    assert_eq!(mem::take(&mut *events.borrow_mut()), []);

    // Mutating editor 1 will emit an `Edited` event only for that editor.
    _ = editor1.update(cx, |editor, cx| editor.insert("X", cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor1", EditorEvent::Edited),
            ("editor1", EditorEvent::BufferEdited),
            ("editor2", EditorEvent::BufferEdited),
        ]
    );

    // Mutating editor 2 will emit an `Edited` event only for that editor.
    _ = editor2.update(cx, |editor, cx| editor.delete(&Delete, cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor2", EditorEvent::Edited),
            ("editor1", EditorEvent::BufferEdited),
            ("editor2", EditorEvent::BufferEdited),
        ]
    );

    // Undoing on editor 1 will emit an `Edited` event only for that editor.
    _ = editor1.update(cx, |editor, cx| editor.undo(&Undo, cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor1", EditorEvent::Edited),
            ("editor1", EditorEvent::BufferEdited),
            ("editor2", EditorEvent::BufferEdited),
        ]
    );

    // Redoing on editor 1 will emit an `Edited` event only for that editor.
    _ = editor1.update(cx, |editor, cx| editor.redo(&Redo, cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor1", EditorEvent::Edited),
            ("editor1", EditorEvent::BufferEdited),
            ("editor2", EditorEvent::BufferEdited),
        ]
    );

    // Undoing on editor 2 will emit an `Edited` event only for that editor.
    _ = editor2.update(cx, |editor, cx| editor.undo(&Undo, cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor2", EditorEvent::Edited),
            ("editor1", EditorEvent::BufferEdited),
            ("editor2", EditorEvent::BufferEdited),
        ]
    );

    // Redoing on editor 2 will emit an `Edited` event only for that editor.
    _ = editor2.update(cx, |editor, cx| editor.redo(&Redo, cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor2", EditorEvent::Edited),
            ("editor1", EditorEvent::BufferEdited),
            ("editor2", EditorEvent::BufferEdited),
        ]
    );

    // No event is emitted when the mutation is a no-op.
    _ = editor2.update(cx, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([0..0]));

        editor.backspace(&Backspace, cx);
    });
    assert_eq!(mem::take(&mut *events.borrow_mut()), []);
}

#[gpui::test]
fn test_undo_redo_with_selection_restoration(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut now = Instant::now();
    let buffer = cx.new_model(|cx| {
        language::Buffer::new(0, BufferId::new(cx.entity_id().as_u64()).unwrap(), "123456")
    });
    let group_interval = buffer.update(cx, |buffer, _| buffer.transaction_group_interval());
    let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
    let editor = cx.add_window(|cx| build_editor(buffer.clone(), cx));

    _ = editor.update(cx, |editor, cx| {
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
        _ = buffer.update(cx, |buffer, cx| {
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
fn test_ime_composition(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let buffer = cx.new_model(|cx| {
        let mut buffer =
            language::Buffer::new(0, BufferId::new(cx.entity_id().as_u64()).unwrap(), "abcde");
        // Ensure automatic grouping doesn't occur.
        buffer.set_group_interval(Duration::ZERO);
        buffer
    });

    let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
    cx.add_window(|cx| {
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
fn test_selection_with_mouse(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\nddddddd\n", cx);
        build_editor(buffer, cx)
    });

    _ = editor.update(cx, |view, cx| {
        view.begin_selection(DisplayPoint::new(2, 2), false, 1, cx);
    });
    assert_eq!(
        editor
            .update(cx, |view, cx| view.selections.display_ranges(cx))
            .unwrap(),
        [DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2)]
    );

    _ = editor.update(cx, |view, cx| {
        view.update_selection(
            DisplayPoint::new(3, 3),
            0,
            gpui::Point::<f32>::default(),
            cx,
        );
    });

    assert_eq!(
        editor
            .update(cx, |view, cx| view.selections.display_ranges(cx))
            .unwrap(),
        [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
    );

    _ = editor.update(cx, |view, cx| {
        view.update_selection(
            DisplayPoint::new(1, 1),
            0,
            gpui::Point::<f32>::default(),
            cx,
        );
    });

    assert_eq!(
        editor
            .update(cx, |view, cx| view.selections.display_ranges(cx))
            .unwrap(),
        [DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1)]
    );

    _ = editor.update(cx, |view, cx| {
        view.end_selection(cx);
        view.update_selection(
            DisplayPoint::new(3, 3),
            0,
            gpui::Point::<f32>::default(),
            cx,
        );
    });

    assert_eq!(
        editor
            .update(cx, |view, cx| view.selections.display_ranges(cx))
            .unwrap(),
        [DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1)]
    );

    _ = editor.update(cx, |view, cx| {
        view.begin_selection(DisplayPoint::new(3, 3), true, 1, cx);
        view.update_selection(
            DisplayPoint::new(0, 0),
            0,
            gpui::Point::<f32>::default(),
            cx,
        );
    });

    assert_eq!(
        editor
            .update(cx, |view, cx| view.selections.display_ranges(cx))
            .unwrap(),
        [
            DisplayPoint::new(2, 2)..DisplayPoint::new(1, 1),
            DisplayPoint::new(3, 3)..DisplayPoint::new(0, 0)
        ]
    );

    _ = editor.update(cx, |view, cx| {
        view.end_selection(cx);
    });

    assert_eq!(
        editor
            .update(cx, |view, cx| view.selections.display_ranges(cx))
            .unwrap(),
        [DisplayPoint::new(3, 3)..DisplayPoint::new(0, 0)]
    );
}

#[gpui::test]
fn test_canceling_pending_selection(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let view = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx);
        build_editor(buffer, cx)
    });

    _ = view.update(cx, |view, cx| {
        view.begin_selection(DisplayPoint::new(2, 2), false, 1, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(2, 2)]
        );
    });

    _ = view.update(cx, |view, cx| {
        view.update_selection(
            DisplayPoint::new(3, 3),
            0,
            gpui::Point::<f32>::default(),
            cx,
        );
        assert_eq!(
            view.selections.display_ranges(cx),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
        );
    });

    _ = view.update(cx, |view, cx| {
        view.cancel(&Cancel, cx);
        view.update_selection(
            DisplayPoint::new(1, 1),
            0,
            gpui::Point::<f32>::default(),
            cx,
        );
        assert_eq!(
            view.selections.display_ranges(cx),
            [DisplayPoint::new(2, 2)..DisplayPoint::new(3, 3)]
        );
    });
}

#[gpui::test]
fn test_clone(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

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

    let editor = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple(&text, cx);
        build_editor(buffer, cx)
    });

    _ = editor.update(cx, |editor, cx| {
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

    let cloned_editor = editor
        .update(cx, |editor, cx| {
            cx.open_window(Default::default(), |cx| cx.new_view(|cx| editor.clone(cx)))
        })
        .unwrap();

    let snapshot = editor.update(cx, |e, cx| e.snapshot(cx)).unwrap();
    let cloned_snapshot = cloned_editor.update(cx, |e, cx| e.snapshot(cx)).unwrap();

    assert_eq!(
        cloned_editor
            .update(cx, |e, cx| e.display_text(cx))
            .unwrap(),
        editor.update(cx, |e, cx| e.display_text(cx)).unwrap()
    );
    assert_eq!(
        cloned_snapshot
            .folds_in_range(0..text.len())
            .collect::<Vec<_>>(),
        snapshot.folds_in_range(0..text.len()).collect::<Vec<_>>(),
    );
    assert_set_eq!(
        cloned_editor
            .update(cx, |editor, cx| editor.selections.ranges::<Point>(cx))
            .unwrap(),
        editor
            .update(cx, |editor, cx| editor.selections.ranges(cx))
            .unwrap()
    );
    assert_set_eq!(
        cloned_editor
            .update(cx, |e, cx| e.selections.display_ranges(cx))
            .unwrap(),
        editor
            .update(cx, |e, cx| e.selections.display_ranges(cx))
            .unwrap()
    );
}

#[gpui::test]
async fn test_navigation_history(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    use workspace::item::Item;

    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, [], cx).await;
    let workspace = cx.add_window(|cx| Workspace::test_new(project, cx));
    let pane = workspace
        .update(cx, |workspace, _| workspace.active_pane().clone())
        .unwrap();

    _ = workspace.update(cx, |_v, cx| {
        cx.new_view(|cx| {
            let buffer = MultiBuffer::build_simple(&sample_text(300, 5, 'a'), cx);
            let mut editor = build_editor(buffer.clone(), cx);
            let handle = cx.view();
            editor.set_nav_history(Some(pane.read(cx).nav_history_for_item(&handle)));

            fn pop_history(editor: &mut Editor, cx: &mut WindowContext) -> Option<NavigationEntry> {
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
            assert_eq!(nav_entry.item.id(), cx.entity_id());
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
            assert_eq!(nav_entry.item.id(), cx.entity_id());
            assert_eq!(
                editor.selections.display_ranges(cx),
                &[DisplayPoint::new(5, 0)..DisplayPoint::new(5, 0)]
            );
            assert!(pop_history(&mut editor, cx).is_none());

            // Set scroll position to check later
            editor.set_scroll_position(gpui::Point::<f32>::new(5.5, 5.5), cx);
            let original_scroll_position = editor.scroll_manager.anchor();

            // Jump to the end of the document and adjust scroll
            editor.move_to_end(&MoveToEnd, cx);
            editor.set_scroll_position(gpui::Point::<f32>::new(-2.5, -0.5), cx);
            assert_ne!(editor.scroll_manager.anchor(), original_scroll_position);

            let nav_entry = pop_history(&mut editor, cx).unwrap();
            editor.navigate(nav_entry.data.unwrap(), cx);
            assert_eq!(editor.scroll_manager.anchor(), original_scroll_position);

            // Ensure we don't panic when navigation data contains invalid anchors *and* points.
            let mut invalid_anchor = editor.scroll_manager.anchor().anchor;
            invalid_anchor.text_anchor.buffer_id = BufferId::new(999).ok();
            let invalid_point = Point::new(9999, 0);
            editor.navigate(
                Box::new(NavigationData {
                    cursor_anchor: invalid_anchor,
                    cursor_position: invalid_point,
                    scroll_anchor: ScrollAnchor {
                        anchor: invalid_anchor,
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
                gpui::Point::new(0., editor.max_point(cx).row() as f32)
            );

            editor
        })
    });
}

#[gpui::test]
fn test_cancel(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let view = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx);
        build_editor(buffer, cx)
    });

    _ = view.update(cx, |view, cx| {
        view.begin_selection(DisplayPoint::new(3, 4), false, 1, cx);
        view.update_selection(
            DisplayPoint::new(1, 1),
            0,
            gpui::Point::<f32>::default(),
            cx,
        );
        view.end_selection(cx);

        view.begin_selection(DisplayPoint::new(0, 1), true, 1, cx);
        view.update_selection(
            DisplayPoint::new(0, 3),
            0,
            gpui::Point::<f32>::default(),
            cx,
        );
        view.end_selection(cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            [
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 3),
                DisplayPoint::new(3, 4)..DisplayPoint::new(1, 1),
            ]
        );
    });

    _ = view.update(cx, |view, cx| {
        view.cancel(&Cancel, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            [DisplayPoint::new(3, 4)..DisplayPoint::new(1, 1)]
        );
    });

    _ = view.update(cx, |view, cx| {
        view.cancel(&Cancel, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            [DisplayPoint::new(1, 1)..DisplayPoint::new(1, 1)]
        );
    });
}

#[gpui::test]
fn test_fold_action(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let view = cx.add_window(|cx| {
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
        build_editor(buffer.clone(), cx)
    });

    _ = view.update(cx, |view, cx| {
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
        assert_eq!(view.display_text(cx), view.buffer.read(cx).read(cx).text());
    });
}

#[gpui::test]
fn test_move_cursor(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let buffer = cx.update(|cx| MultiBuffer::build_simple(&sample_text(6, 6, 'a'), cx));
    let view = cx.add_window(|cx| build_editor(buffer.clone(), cx));

    _ = buffer.update(cx, |buffer, cx| {
        buffer.edit(
            vec![
                (Point::new(1, 0)..Point::new(1, 0), "\t"),
                (Point::new(1, 1)..Point::new(1, 1), "\t"),
            ],
            None,
            cx,
        );
    });
    _ = view.update(cx, |view, cx| {
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
fn test_move_cursor_multibyte(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let view = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("ⓐⓑⓒⓓⓔ\nabcde\nαβγδε", cx);
        build_editor(buffer.clone(), cx)
    });

    assert_eq!('ⓐ'.len_utf8(), 3);
    assert_eq!('α'.len_utf8(), 2);

    _ = view.update(cx, |view, cx| {
        view.fold_ranges(
            vec![
                Point::new(0, 6)..Point::new(0, 12),
                Point::new(1, 2)..Point::new(1, 4),
                Point::new(2, 4)..Point::new(2, 8),
            ],
            true,
            cx,
        );
        assert_eq!(view.display_text(cx), "ⓐⓑ⋯ⓔ\nab⋯e\nαβ⋯ε");

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
            &[empty_range(1, "ab⋯e".len())]
        );
        view.move_left(&MoveLeft, cx);
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
        view.move_down(&MoveDown, cx);
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
            &[empty_range(0, "ⓐⓑ".len())]
        );
        view.move_left(&MoveLeft, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(0, "ⓐ".len())]
        );
        view.move_left(&MoveLeft, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[empty_range(0, "".len())]
        );
    });
}

#[gpui::test]
fn test_move_cursor_different_line_lengths(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let view = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("ⓐⓑⓒⓓⓔ\nabcd\nαβγ\nabcd\nⓐⓑⓒⓓⓔ\n", cx);
        build_editor(buffer.clone(), cx)
    });
    _ = view.update(cx, |view, cx| {
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
fn test_beginning_end_of_line(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let view = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("abc\n  def", cx);
        build_editor(buffer, cx)
    });
    _ = view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(0, 1)..DisplayPoint::new(0, 1),
                DisplayPoint::new(1, 4)..DisplayPoint::new(1, 4),
            ]);
        });
    });

    _ = view.update(cx, |view, cx| {
        view.move_to_beginning_of_line(&MoveToBeginningOfLine, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
            ]
        );
    });

    _ = view.update(cx, |view, cx| {
        view.move_to_beginning_of_line(&MoveToBeginningOfLine, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0),
            ]
        );
    });

    _ = view.update(cx, |view, cx| {
        view.move_to_beginning_of_line(&MoveToBeginningOfLine, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[
                DisplayPoint::new(0, 0)..DisplayPoint::new(0, 0),
                DisplayPoint::new(1, 2)..DisplayPoint::new(1, 2),
            ]
        );
    });

    _ = view.update(cx, |view, cx| {
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
    _ = view.update(cx, |view, cx| {
        view.move_to_end_of_line(&MoveToEndOfLine, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[
                DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                DisplayPoint::new(1, 5)..DisplayPoint::new(1, 5),
            ]
        );
    });

    _ = view.update(cx, |view, cx| {
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

    _ = view.update(cx, |view, cx| {
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

    _ = view.update(cx, |view, cx| {
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

    _ = view.update(cx, |view, cx| {
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

    _ = view.update(cx, |view, cx| {
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

    _ = view.update(cx, |view, cx| {
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
fn test_prev_next_word_boundary(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let view = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("use std::str::{foo, bar}\n\n  {baz.qux()}", cx);
        build_editor(buffer, cx)
    });
    _ = view.update(cx, |view, cx| {
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
fn test_prev_next_word_bounds_with_soft_wrap(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let view = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("use one::{\n    two::three::four::five\n};", cx);
        build_editor(buffer, cx)
    });

    _ = view.update(cx, |view, cx| {
        view.set_wrap_width(Some(140.0.into()), cx);
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
async fn test_move_start_of_paragraph_end_of_paragraph(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;

    let line_height = cx.editor(|editor, cx| {
        editor
            .style()
            .unwrap()
            .text
            .line_height_in_pixels(cx.rem_size())
    });
    cx.simulate_window_resize(cx.window, size(px(100.), 4. * line_height));

    cx.set_state(
        &r#"ˇone
        two

        three
        fourˇ
        five

        six"#
            .unindent(),
    );

    cx.update_editor(|editor, cx| editor.move_to_end_of_paragraph(&MoveToEndOfParagraph, cx));
    cx.assert_editor_state(
        &r#"one
        two
        ˇ
        three
        four
        five
        ˇ
        six"#
            .unindent(),
    );

    cx.update_editor(|editor, cx| editor.move_to_end_of_paragraph(&MoveToEndOfParagraph, cx));
    cx.assert_editor_state(
        &r#"one
        two

        three
        four
        five
        ˇ
        sixˇ"#
            .unindent(),
    );

    cx.update_editor(|editor, cx| editor.move_to_end_of_paragraph(&MoveToEndOfParagraph, cx));
    cx.assert_editor_state(
        &r#"one
        two

        three
        four
        five

        sixˇ"#
            .unindent(),
    );

    cx.update_editor(|editor, cx| editor.move_to_start_of_paragraph(&MoveToStartOfParagraph, cx));
    cx.assert_editor_state(
        &r#"one
        two

        three
        four
        five
        ˇ
        six"#
            .unindent(),
    );

    cx.update_editor(|editor, cx| editor.move_to_start_of_paragraph(&MoveToStartOfParagraph, cx));
    cx.assert_editor_state(
        &r#"one
        two
        ˇ
        three
        four
        five

        six"#
            .unindent(),
    );

    cx.update_editor(|editor, cx| editor.move_to_start_of_paragraph(&MoveToStartOfParagraph, cx));
    cx.assert_editor_state(
        &r#"ˇone
        two

        three
        four
        five

        six"#
            .unindent(),
    );
}

#[gpui::test]
async fn test_scroll_page_up_page_down(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;
    let line_height = cx.editor(|editor, cx| {
        editor
            .style()
            .unwrap()
            .text
            .line_height_in_pixels(cx.rem_size())
    });
    let window = cx.window;
    cx.simulate_window_resize(window, size(px(1000.), 4. * line_height + px(0.5)));

    cx.set_state(
        &r#"ˇone
        two
        three
        four
        five
        six
        seven
        eight
        nine
        ten
        "#,
    );

    cx.update_editor(|editor, cx| {
        assert_eq!(
            editor.snapshot(cx).scroll_position(),
            gpui::Point::new(0., 0.)
        );
        editor.scroll_screen(&ScrollAmount::Page(1.), cx);
        assert_eq!(
            editor.snapshot(cx).scroll_position(),
            gpui::Point::new(0., 3.)
        );
        editor.scroll_screen(&ScrollAmount::Page(1.), cx);
        assert_eq!(
            editor.snapshot(cx).scroll_position(),
            gpui::Point::new(0., 6.)
        );
        editor.scroll_screen(&ScrollAmount::Page(-1.), cx);
        assert_eq!(
            editor.snapshot(cx).scroll_position(),
            gpui::Point::new(0., 3.)
        );

        editor.scroll_screen(&ScrollAmount::Page(-0.5), cx);
        assert_eq!(
            editor.snapshot(cx).scroll_position(),
            gpui::Point::new(0., 1.)
        );
        editor.scroll_screen(&ScrollAmount::Page(0.5), cx);
        assert_eq!(
            editor.snapshot(cx).scroll_position(),
            gpui::Point::new(0., 3.)
        );
    });
}

#[gpui::test]
async fn test_autoscroll(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;

    let line_height = cx.update_editor(|editor, cx| {
        editor.set_vertical_scroll_margin(2, cx);
        editor
            .style()
            .unwrap()
            .text
            .line_height_in_pixels(cx.rem_size())
    });
    let window = cx.window;
    cx.simulate_window_resize(window, size(px(1000.), 6. * line_height));

    cx.set_state(
        &r#"ˇone
            two
            three
            four
            five
            six
            seven
            eight
            nine
            ten
        "#,
    );
    cx.update_editor(|editor, cx| {
        assert_eq!(
            editor.snapshot(cx).scroll_position(),
            gpui::Point::new(0., 0.0)
        );
    });

    // Add a cursor below the visible area. Since both cursors cannot fit
    // on screen, the editor autoscrolls to reveal the newest cursor, and
    // allows the vertical scroll margin below that cursor.
    cx.update_editor(|editor, cx| {
        editor.change_selections(Some(Autoscroll::fit()), cx, |selections| {
            selections.select_ranges([
                Point::new(0, 0)..Point::new(0, 0),
                Point::new(6, 0)..Point::new(6, 0),
            ]);
        })
    });
    cx.update_editor(|editor, cx| {
        assert_eq!(
            editor.snapshot(cx).scroll_position(),
            gpui::Point::new(0., 3.0)
        );
    });

    // Move down. The editor cursor scrolls down to track the newest cursor.
    cx.update_editor(|editor, cx| {
        editor.move_down(&Default::default(), cx);
    });
    cx.update_editor(|editor, cx| {
        assert_eq!(
            editor.snapshot(cx).scroll_position(),
            gpui::Point::new(0., 4.0)
        );
    });

    // Add a cursor above the visible area. Since both cursors fit on screen,
    // the editor scrolls to show both.
    cx.update_editor(|editor, cx| {
        editor.change_selections(Some(Autoscroll::fit()), cx, |selections| {
            selections.select_ranges([
                Point::new(1, 0)..Point::new(1, 0),
                Point::new(6, 0)..Point::new(6, 0),
            ]);
        })
    });
    cx.update_editor(|editor, cx| {
        assert_eq!(
            editor.snapshot(cx).scroll_position(),
            gpui::Point::new(0., 1.0)
        );
    });
}

#[gpui::test]
async fn test_move_page_up_page_down(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;

    let line_height = cx.editor(|editor, cx| {
        editor
            .style()
            .unwrap()
            .text
            .line_height_in_pixels(cx.rem_size())
    });
    let window = cx.window;
    cx.simulate_window_resize(window, size(px(100.), 4. * line_height));
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
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state("one «two threeˇ» four");
    cx.update_editor(|editor, cx| {
        editor.delete_to_beginning_of_line(&DeleteToBeginningOfLine, cx);
        assert_eq!(editor.text(cx), " four");
    });
}

#[gpui::test]
fn test_delete_to_word_boundary(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let view = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("one two three four", cx);
        build_editor(buffer.clone(), cx)
    });

    _ = view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([
                // an empty selection - the preceding word fragment is deleted
                DisplayPoint::new(0, 2)..DisplayPoint::new(0, 2),
                // characters selected - they are deleted
                DisplayPoint::new(0, 9)..DisplayPoint::new(0, 12),
            ])
        });
        view.delete_to_previous_word_start(&DeleteToPreviousWordStart, cx);
        assert_eq!(view.buffer.read(cx).read(cx).text(), "e two te four");
    });

    _ = view.update(cx, |view, cx| {
        view.change_selections(None, cx, |s| {
            s.select_display_ranges([
                // an empty selection - the following word fragment is deleted
                DisplayPoint::new(0, 3)..DisplayPoint::new(0, 3),
                // characters selected - they are deleted
                DisplayPoint::new(0, 9)..DisplayPoint::new(0, 10),
            ])
        });
        view.delete_to_next_word_end(&DeleteToNextWordEnd, cx);
        assert_eq!(view.buffer.read(cx).read(cx).text(), "e t te our");
    });
}

#[gpui::test]
fn test_newline(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let view = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("aaaa\n    bbbb\n", cx);
        build_editor(buffer.clone(), cx)
    });

    _ = view.update(cx, |view, cx| {
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
fn test_newline_with_old_selections(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|cx| {
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
        let mut editor = build_editor(buffer.clone(), cx);
        editor.change_selections(None, cx, |s| {
            s.select_ranges([
                Point::new(2, 4)..Point::new(2, 5),
                Point::new(5, 4)..Point::new(5, 5),
            ])
        });
        editor
    });

    _ = editor.update(cx, |editor, cx| {
        // Edit the buffer directly, deleting ranges surrounding the editor's selections
        editor.buffer.update(cx, |buffer, cx| {
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
async fn test_newline_above(cx: &mut gpui::TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.tab_size = NonZeroU32::new(4)
    });

    let language = Arc::new(
        Language::new(
            LanguageConfig::default(),
            Some(tree_sitter_rust::language()),
        )
        .with_indents_query(r#"(_ "(" ")" @end) @indent"#)
        .unwrap(),
    );

    let mut cx = EditorTestContext::new(cx).await;
    cx.update_buffer(|buffer, cx| buffer.set_language(Some(language), cx));
    cx.set_state(indoc! {"
        const a: ˇA = (
            (ˇ
                «const_functionˇ»(ˇ),
                so«mˇ»et«hˇ»ing_ˇelse,ˇ
            )ˇ
        ˇ);ˇ
    "});

    cx.update_editor(|e, cx| e.newline_above(&NewlineAbove, cx));
    cx.assert_editor_state(indoc! {"
        ˇ
        const a: A = (
            ˇ
            (
                ˇ
                ˇ
                const_function(),
                ˇ
                ˇ
                ˇ
                ˇ
                something_else,
                ˇ
            )
            ˇ
            ˇ
        );
    "});
}

#[gpui::test]
async fn test_newline_below(cx: &mut gpui::TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.tab_size = NonZeroU32::new(4)
    });

    let language = Arc::new(
        Language::new(
            LanguageConfig::default(),
            Some(tree_sitter_rust::language()),
        )
        .with_indents_query(r#"(_ "(" ")" @end) @indent"#)
        .unwrap(),
    );

    let mut cx = EditorTestContext::new(cx).await;
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
async fn test_newline_comments(cx: &mut gpui::TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.tab_size = NonZeroU32::new(4)
    });

    let language = Arc::new(Language::new(
        LanguageConfig {
            line_comments: vec!["//".into()],
            ..LanguageConfig::default()
        },
        None,
    ));
    {
        let mut cx = EditorTestContext::new(cx).await;
        cx.update_buffer(|buffer, cx| buffer.set_language(Some(language), cx));
        cx.set_state(indoc! {"
        // Fooˇ
    "});

        cx.update_editor(|e, cx| e.newline(&Newline, cx));
        cx.assert_editor_state(indoc! {"
        // Foo
        //ˇ
    "});
        // Ensure that if cursor is before the comment start, we do not actually insert a comment prefix.
        cx.set_state(indoc! {"
        ˇ// Foo
    "});
        cx.update_editor(|e, cx| e.newline(&Newline, cx));
        cx.assert_editor_state(indoc! {"

        ˇ// Foo
    "});
    }
    // Ensure that comment continuations can be disabled.
    update_test_language_settings(cx, |settings| {
        settings.defaults.extend_comment_on_newline = Some(false);
    });
    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state(indoc! {"
        // Fooˇ
    "});
    cx.update_editor(|e, cx| e.newline(&Newline, cx));
    cx.assert_editor_state(indoc! {"
        // Foo
        ˇ
    "});
}

#[gpui::test]
fn test_insert_with_old_selections(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("a( X ), b( Y ), c( Z )", cx);
        let mut editor = build_editor(buffer.clone(), cx);
        editor.change_selections(None, cx, |s| s.select_ranges([3..4, 11..12, 19..20]));
        editor
    });

    _ = editor.update(cx, |editor, cx| {
        // Edit the buffer directly, deleting ranges surrounding the editor's selections
        editor.buffer.update(cx, |buffer, cx| {
            buffer.edit([(2..5, ""), (10..13, ""), (18..21, "")], None, cx);
            assert_eq!(buffer.read(cx).text(), "a(), b(), c()".unindent());
        });
        assert_eq!(editor.selections.ranges(cx), &[2..2, 7..7, 12..12],);

        editor.insert("Z", cx);
        assert_eq!(editor.text(cx), "a(Z), b(Z), c(Z)");

        // The selections are moved after the inserted characters
        assert_eq!(editor.selections.ranges(cx), &[3..3, 9..9, 15..15],);
    });
}

#[gpui::test]
async fn test_tab(cx: &mut gpui::TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.tab_size = NonZeroU32::new(3)
    });

    let mut cx = EditorTestContext::new(cx).await;
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
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
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
    init_test(cx, |settings| {
        settings.defaults.tab_size = NonZeroU32::new(4)
    });

    let language = Arc::new(
        Language::new(
            LanguageConfig::default(),
            Some(tree_sitter_rust::language()),
        )
        .with_indents_query(r#"(_ "{" "}" @end) @indent"#)
        .unwrap(),
    );

    let mut cx = EditorTestContext::new(cx).await;
    cx.update_buffer(|buffer, cx| buffer.set_language(Some(language), cx));
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
    init_test(cx, |settings| {
        settings.defaults.tab_size = NonZeroU32::new(4);
    });

    let mut cx = EditorTestContext::new(cx).await;

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
    init_test(cx, |settings| {
        settings.defaults.hard_tabs = Some(true);
    });

    let mut cx = EditorTestContext::new(cx).await;

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
fn test_indent_outdent_with_excerpts(cx: &mut TestAppContext) {
    init_test(cx, |settings| {
        settings.languages.extend([
            (
                "TOML".into(),
                LanguageSettingsContent {
                    tab_size: NonZeroU32::new(2),
                    ..Default::default()
                },
            ),
            (
                "Rust".into(),
                LanguageSettingsContent {
                    tab_size: NonZeroU32::new(4),
                    ..Default::default()
                },
            ),
        ]);
    });

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

    let toml_buffer = cx.new_model(|cx| {
        Buffer::new(
            0,
            BufferId::new(cx.entity_id().as_u64()).unwrap(),
            "a = 1\nb = 2\n",
        )
        .with_language(toml_language, cx)
    });
    let rust_buffer = cx.new_model(|cx| {
        Buffer::new(
            0,
            BufferId::new(cx.entity_id().as_u64()).unwrap(),
            "const c: usize = 3;\n",
        )
        .with_language(rust_language, cx)
    });
    let multibuffer = cx.new_model(|cx| {
        let mut multibuffer = MultiBuffer::new(0, ReadWrite);
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

    cx.add_window(|cx| {
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
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

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
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
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
fn test_delete_line(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let view = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        build_editor(buffer, cx)
    });
    _ = view.update(cx, |view, cx| {
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

    let view = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        build_editor(buffer, cx)
    });
    _ = view.update(cx, |view, cx| {
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
fn test_join_lines_with_single_selection(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("aaa\nbbb\nccc\nddd\n\n", cx);
        let mut editor = build_editor(buffer.clone(), cx);
        let buffer = buffer.read(cx).as_singleton().unwrap();

        assert_eq!(
            editor.selections.ranges::<Point>(cx),
            &[Point::new(0, 0)..Point::new(0, 0)]
        );

        // When on single line, replace newline at end by space
        editor.join_lines(&JoinLines, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb\nccc\nddd\n\n");
        assert_eq!(
            editor.selections.ranges::<Point>(cx),
            &[Point::new(0, 3)..Point::new(0, 3)]
        );

        // When multiple lines are selected, remove newlines that are spanned by the selection
        editor.change_selections(None, cx, |s| {
            s.select_ranges([Point::new(0, 5)..Point::new(2, 2)])
        });
        editor.join_lines(&JoinLines, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb ccc ddd\n\n");
        assert_eq!(
            editor.selections.ranges::<Point>(cx),
            &[Point::new(0, 11)..Point::new(0, 11)]
        );

        // Undo should be transactional
        editor.undo(&Undo, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb\nccc\nddd\n\n");
        assert_eq!(
            editor.selections.ranges::<Point>(cx),
            &[Point::new(0, 5)..Point::new(2, 2)]
        );

        // When joining an empty line don't insert a space
        editor.change_selections(None, cx, |s| {
            s.select_ranges([Point::new(2, 1)..Point::new(2, 2)])
        });
        editor.join_lines(&JoinLines, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb\nccc\nddd\n");
        assert_eq!(
            editor.selections.ranges::<Point>(cx),
            [Point::new(2, 3)..Point::new(2, 3)]
        );

        // We can remove trailing newlines
        editor.join_lines(&JoinLines, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb\nccc\nddd");
        assert_eq!(
            editor.selections.ranges::<Point>(cx),
            [Point::new(2, 3)..Point::new(2, 3)]
        );

        // We don't blow up on the last line
        editor.join_lines(&JoinLines, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb\nccc\nddd");
        assert_eq!(
            editor.selections.ranges::<Point>(cx),
            [Point::new(2, 3)..Point::new(2, 3)]
        );

        // reset to test indentation
        editor.buffer.update(cx, |buffer, cx| {
            buffer.edit(
                [
                    (Point::new(1, 0)..Point::new(1, 2), "  "),
                    (Point::new(2, 0)..Point::new(2, 3), "  \n\td"),
                ],
                None,
                cx,
            )
        });

        // We remove any leading spaces
        assert_eq!(buffer.read(cx).text(), "aaa bbb\n  c\n  \n\td");
        editor.change_selections(None, cx, |s| {
            s.select_ranges([Point::new(0, 1)..Point::new(0, 1)])
        });
        editor.join_lines(&JoinLines, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb c\n  \n\td");

        // We don't insert a space for a line containing only spaces
        editor.join_lines(&JoinLines, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb c\n\td");

        // We ignore any leading tabs
        editor.join_lines(&JoinLines, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb c d");

        editor
    });
}

#[gpui::test]
fn test_join_lines_with_multi_selection(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("aaa\nbbb\nccc\nddd\n\n", cx);
        let mut editor = build_editor(buffer.clone(), cx);
        let buffer = buffer.read(cx).as_singleton().unwrap();

        editor.change_selections(None, cx, |s| {
            s.select_ranges([
                Point::new(0, 2)..Point::new(1, 1),
                Point::new(1, 2)..Point::new(1, 2),
                Point::new(3, 1)..Point::new(3, 2),
            ])
        });

        editor.join_lines(&JoinLines, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb ccc\nddd\n");

        assert_eq!(
            editor.selections.ranges::<Point>(cx),
            [
                Point::new(0, 7)..Point::new(0, 7),
                Point::new(1, 3)..Point::new(1, 3)
            ]
        );
        editor
    });
}

#[gpui::test]
async fn test_manipulate_lines_with_single_selection(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    // Test sort_lines_case_insensitive()
    cx.set_state(indoc! {"
        «z
        y
        x
        Z
        Y
        Xˇ»
    "});
    cx.update_editor(|e, cx| e.sort_lines_case_insensitive(&SortLinesCaseInsensitive, cx));
    cx.assert_editor_state(indoc! {"
        «x
        X
        y
        Y
        z
        Zˇ»
    "});

    // Test reverse_lines()
    cx.set_state(indoc! {"
        «5
        4
        3
        2
        1ˇ»
    "});
    cx.update_editor(|e, cx| e.reverse_lines(&ReverseLines, cx));
    cx.assert_editor_state(indoc! {"
        «1
        2
        3
        4
        5ˇ»
    "});

    // Skip testing shuffle_line()

    // From here on out, test more complex cases of manipulate_lines() with a single driver method: sort_lines_case_sensitive()
    // Since all methods calling manipulate_lines() are doing the exact same general thing (reordering lines)

    // Don't manipulate when cursor is on single line, but expand the selection
    cx.set_state(indoc! {"
        ddˇdd
        ccc
        bb
        a
    "});
    cx.update_editor(|e, cx| e.sort_lines_case_sensitive(&SortLinesCaseSensitive, cx));
    cx.assert_editor_state(indoc! {"
        «ddddˇ»
        ccc
        bb
        a
    "});

    // Basic manipulate case
    // Start selection moves to column 0
    // End of selection shrinks to fit shorter line
    cx.set_state(indoc! {"
        dd«d
        ccc
        bb
        aaaaaˇ»
    "});
    cx.update_editor(|e, cx| e.sort_lines_case_sensitive(&SortLinesCaseSensitive, cx));
    cx.assert_editor_state(indoc! {"
        «aaaaa
        bb
        ccc
        dddˇ»
    "});

    // Manipulate case with newlines
    cx.set_state(indoc! {"
        dd«d
        ccc

        bb
        aaaaa

        ˇ»
    "});
    cx.update_editor(|e, cx| e.sort_lines_case_sensitive(&SortLinesCaseSensitive, cx));
    cx.assert_editor_state(indoc! {"
        «

        aaaaa
        bb
        ccc
        dddˇ»

    "});

    // Adding new line
    cx.set_state(indoc! {"
        aa«a
        bbˇ»b
    "});
    cx.update_editor(|e, cx| e.manipulate_lines(cx, |lines| lines.push("added_line")));
    cx.assert_editor_state(indoc! {"
        «aaa
        bbb
        added_lineˇ»
    "});

    // Removing line
    cx.set_state(indoc! {"
        aa«a
        bbbˇ»
    "});
    cx.update_editor(|e, cx| {
        e.manipulate_lines(cx, |lines| {
            lines.pop();
        })
    });
    cx.assert_editor_state(indoc! {"
        «aaaˇ»
    "});

    // Removing all lines
    cx.set_state(indoc! {"
        aa«a
        bbbˇ»
    "});
    cx.update_editor(|e, cx| {
        e.manipulate_lines(cx, |lines| {
            lines.drain(..);
        })
    });
    cx.assert_editor_state(indoc! {"
        ˇ
    "});
}

#[gpui::test]
async fn test_unique_lines_multi_selection(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    // Consider continuous selection as single selection
    cx.set_state(indoc! {"
        Aaa«aa
        cˇ»c«c
        bb
        aaaˇ»aa
    "});
    cx.update_editor(|e, cx| e.unique_lines_case_sensitive(&UniqueLinesCaseSensitive, cx));
    cx.assert_editor_state(indoc! {"
        «Aaaaa
        ccc
        bb
        aaaaaˇ»
    "});

    cx.set_state(indoc! {"
        Aaa«aa
        cˇ»c«c
        bb
        aaaˇ»aa
    "});
    cx.update_editor(|e, cx| e.unique_lines_case_insensitive(&UniqueLinesCaseInsensitive, cx));
    cx.assert_editor_state(indoc! {"
        «Aaaaa
        ccc
        bbˇ»
    "});

    // Consider non continuous selection as distinct dedup operations
    cx.set_state(indoc! {"
        «aaaaa
        bb
        aaaaa
        aaaaaˇ»

        aaa«aaˇ»
    "});
    cx.update_editor(|e, cx| e.unique_lines_case_sensitive(&UniqueLinesCaseSensitive, cx));
    cx.assert_editor_state(indoc! {"
        «aaaaa
        bbˇ»

        «aaaaaˇ»
    "});
}

#[gpui::test]
async fn test_unique_lines_single_selection(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    cx.set_state(indoc! {"
        «Aaa
        aAa
        Aaaˇ»
    "});
    cx.update_editor(|e, cx| e.unique_lines_case_sensitive(&UniqueLinesCaseSensitive, cx));
    cx.assert_editor_state(indoc! {"
        «Aaa
        aAaˇ»
    "});

    cx.set_state(indoc! {"
        «Aaa
        aAa
        aaAˇ»
    "});
    cx.update_editor(|e, cx| e.unique_lines_case_insensitive(&UniqueLinesCaseInsensitive, cx));
    cx.assert_editor_state(indoc! {"
        «Aaaˇ»
    "});
}

#[gpui::test]
async fn test_manipulate_lines_with_multi_selection(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    // Manipulate with multiple selections on a single line
    cx.set_state(indoc! {"
        dd«dd
        cˇ»c«c
        bb
        aaaˇ»aa
    "});
    cx.update_editor(|e, cx| e.sort_lines_case_sensitive(&SortLinesCaseSensitive, cx));
    cx.assert_editor_state(indoc! {"
        «aaaaa
        bb
        ccc
        ddddˇ»
    "});

    // Manipulate with multiple disjoin selections
    cx.set_state(indoc! {"
        5«
        4
        3
        2
        1ˇ»

        dd«dd
        ccc
        bb
        aaaˇ»aa
    "});
    cx.update_editor(|e, cx| e.sort_lines_case_sensitive(&SortLinesCaseSensitive, cx));
    cx.assert_editor_state(indoc! {"
        «1
        2
        3
        4
        5ˇ»

        «aaaaa
        bb
        ccc
        ddddˇ»
    "});

    // Adding lines on each selection
    cx.set_state(indoc! {"
        2«
        1ˇ»

        bb«bb
        aaaˇ»aa
    "});
    cx.update_editor(|e, cx| e.manipulate_lines(cx, |lines| lines.push("added line")));
    cx.assert_editor_state(indoc! {"
        «2
        1
        added lineˇ»

        «bbbb
        aaaaa
        added lineˇ»
    "});

    // Removing lines on each selection
    cx.set_state(indoc! {"
        2«
        1ˇ»

        bb«bb
        aaaˇ»aa
    "});
    cx.update_editor(|e, cx| {
        e.manipulate_lines(cx, |lines| {
            lines.pop();
        })
    });
    cx.assert_editor_state(indoc! {"
        «2ˇ»

        «bbbbˇ»
    "});
}

#[gpui::test]
async fn test_manipulate_text(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    // Test convert_to_upper_case()
    cx.set_state(indoc! {"
        «hello worldˇ»
    "});
    cx.update_editor(|e, cx| e.convert_to_upper_case(&ConvertToUpperCase, cx));
    cx.assert_editor_state(indoc! {"
        «HELLO WORLDˇ»
    "});

    // Test convert_to_lower_case()
    cx.set_state(indoc! {"
        «HELLO WORLDˇ»
    "});
    cx.update_editor(|e, cx| e.convert_to_lower_case(&ConvertToLowerCase, cx));
    cx.assert_editor_state(indoc! {"
        «hello worldˇ»
    "});

    // Test multiple line, single selection case
    // Test code hack that covers the fact that to_case crate doesn't support '\n' as a word boundary
    cx.set_state(indoc! {"
        «The quick brown
        fox jumps over
        the lazy dogˇ»
    "});
    cx.update_editor(|e, cx| e.convert_to_title_case(&ConvertToTitleCase, cx));
    cx.assert_editor_state(indoc! {"
        «The Quick Brown
        Fox Jumps Over
        The Lazy Dogˇ»
    "});

    // Test multiple line, single selection case
    // Test code hack that covers the fact that to_case crate doesn't support '\n' as a word boundary
    cx.set_state(indoc! {"
        «The quick brown
        fox jumps over
        the lazy dogˇ»
    "});
    cx.update_editor(|e, cx| e.convert_to_upper_camel_case(&ConvertToUpperCamelCase, cx));
    cx.assert_editor_state(indoc! {"
        «TheQuickBrown
        FoxJumpsOver
        TheLazyDogˇ»
    "});

    // From here on out, test more complex cases of manipulate_text()

    // Test no selection case - should affect words cursors are in
    // Cursor at beginning, middle, and end of word
    cx.set_state(indoc! {"
        ˇhello big beauˇtiful worldˇ
    "});
    cx.update_editor(|e, cx| e.convert_to_upper_case(&ConvertToUpperCase, cx));
    cx.assert_editor_state(indoc! {"
        «HELLOˇ» big «BEAUTIFULˇ» «WORLDˇ»
    "});

    // Test multiple selections on a single line and across multiple lines
    cx.set_state(indoc! {"
        «Theˇ» quick «brown
        foxˇ» jumps «overˇ»
        the «lazyˇ» dog
    "});
    cx.update_editor(|e, cx| e.convert_to_upper_case(&ConvertToUpperCase, cx));
    cx.assert_editor_state(indoc! {"
        «THEˇ» quick «BROWN
        FOXˇ» jumps «OVERˇ»
        the «LAZYˇ» dog
    "});

    // Test case where text length grows
    cx.set_state(indoc! {"
        «tschüßˇ»
    "});
    cx.update_editor(|e, cx| e.convert_to_upper_case(&ConvertToUpperCase, cx));
    cx.assert_editor_state(indoc! {"
        «TSCHÜSSˇ»
    "});

    // Test to make sure we don't crash when text shrinks
    cx.set_state(indoc! {"
        aaa_bbbˇ
    "});
    cx.update_editor(|e, cx| e.convert_to_lower_camel_case(&ConvertToLowerCamelCase, cx));
    cx.assert_editor_state(indoc! {"
        «aaaBbbˇ»
    "});

    // Test to make sure we all aware of the fact that each word can grow and shrink
    // Final selections should be aware of this fact
    cx.set_state(indoc! {"
        aaa_bˇbb bbˇb_ccc ˇccc_ddd
    "});
    cx.update_editor(|e, cx| e.convert_to_lower_camel_case(&ConvertToLowerCamelCase, cx));
    cx.assert_editor_state(indoc! {"
        «aaaBbbˇ» «bbbCccˇ» «cccDddˇ»
    "});
}

#[gpui::test]
fn test_duplicate_line(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let view = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        build_editor(buffer, cx)
    });
    _ = view.update(cx, |view, cx| {
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

    let view = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        build_editor(buffer, cx)
    });
    _ = view.update(cx, |view, cx| {
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
fn test_move_line_up_down(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let view = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple(&sample_text(10, 5, 'a'), cx);
        build_editor(buffer, cx)
    });
    _ = view.update(cx, |view, cx| {
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

    _ = view.update(cx, |view, cx| {
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

    _ = view.update(cx, |view, cx| {
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

    _ = view.update(cx, |view, cx| {
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
fn test_move_line_up_down_with_blocks(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple(&sample_text(10, 5, 'a'), cx);
        build_editor(buffer, cx)
    });
    _ = editor.update(cx, |editor, cx| {
        let snapshot = editor.buffer.read(cx).snapshot(cx);
        editor.insert_blocks(
            [BlockProperties {
                style: BlockStyle::Fixed,
                position: snapshot.anchor_after(Point::new(2, 0)),
                disposition: BlockDisposition::Below,
                height: 1,
                render: Arc::new(|_| div().into_any()),
            }],
            Some(Autoscroll::fit()),
            cx,
        );
        editor.change_selections(None, cx, |s| {
            s.select_ranges([Point::new(2, 0)..Point::new(2, 0)])
        });
        editor.move_line_down(&MoveLineDown, cx);
    });
}

#[gpui::test]
fn test_transpose(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    _ = cx.add_window(|cx| {
        let mut editor = build_editor(MultiBuffer::build_simple("abc", cx), cx);
        editor.set_style(EditorStyle::default(), cx);
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
    });

    _ = cx.add_window(|cx| {
        let mut editor = build_editor(MultiBuffer::build_simple("abc\nde", cx), cx);
        editor.set_style(EditorStyle::default(), cx);
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
    });

    _ = cx.add_window(|cx| {
        let mut editor = build_editor(MultiBuffer::build_simple("abc\nde", cx), cx);
        editor.set_style(EditorStyle::default(), cx);
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
    });

    _ = cx.add_window(|cx| {
        let mut editor = build_editor(MultiBuffer::build_simple("🍐🏀✋", cx), cx);
        editor.set_style(EditorStyle::default(), cx);
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
    });
}

#[gpui::test]
async fn test_clipboard(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

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
    assert_eq!(
        cx.read_from_clipboard().map(|item| item.text().to_owned()),
        Some("fox jumps over\n".to_owned())
    );

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
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
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
fn test_select_all(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let view = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple("abc\nde\nfgh", cx);
        build_editor(buffer, cx)
    });
    _ = view.update(cx, |view, cx| {
        view.select_all(&SelectAll, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            &[DisplayPoint::new(0, 0)..DisplayPoint::new(2, 3)]
        );
    });
}

#[gpui::test]
fn test_select_line(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let view = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple(&sample_text(6, 5, 'a'), cx);
        build_editor(buffer, cx)
    });
    _ = view.update(cx, |view, cx| {
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

    _ = view.update(cx, |view, cx| {
        view.select_line(&SelectLine, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(0, 0)..DisplayPoint::new(3, 0),
                DisplayPoint::new(4, 0)..DisplayPoint::new(5, 5),
            ]
        );
    });

    _ = view.update(cx, |view, cx| {
        view.select_line(&SelectLine, cx);
        assert_eq!(
            view.selections.display_ranges(cx),
            vec![DisplayPoint::new(0, 0)..DisplayPoint::new(5, 5)]
        );
    });
}

#[gpui::test]
fn test_split_selection_into_lines(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let view = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple(&sample_text(9, 5, 'a'), cx);
        build_editor(buffer, cx)
    });
    _ = view.update(cx, |view, cx| {
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

    _ = view.update(cx, |view, cx| {
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

    _ = view.update(cx, |view, cx| {
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
async fn test_add_selection_above_below(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    // let buffer = MultiBuffer::build_simple("abc\ndefghi\n\njk\nlmno\n", cx);
    cx.set_state(indoc!(
        r#"abc
           defˇghi

           jk
           nlmo
           "#
    ));

    cx.update_editor(|editor, cx| {
        editor.add_selection_above(&Default::default(), cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abcˇ
           defˇghi

           jk
           nlmo
           "#
    ));

    cx.update_editor(|editor, cx| {
        editor.add_selection_above(&Default::default(), cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abcˇ
            defˇghi

            jk
            nlmo
            "#
    ));

    cx.update_editor(|view, cx| {
        view.add_selection_below(&Default::default(), cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abc
           defˇghi

           jk
           nlmo
           "#
    ));

    cx.update_editor(|view, cx| {
        view.undo_selection(&Default::default(), cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abcˇ
           defˇghi

           jk
           nlmo
           "#
    ));

    cx.update_editor(|view, cx| {
        view.redo_selection(&Default::default(), cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abc
           defˇghi

           jk
           nlmo
           "#
    ));

    cx.update_editor(|view, cx| {
        view.add_selection_below(&Default::default(), cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abc
           defˇghi

           jk
           nlmˇo
           "#
    ));

    cx.update_editor(|view, cx| {
        view.add_selection_below(&Default::default(), cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abc
           defˇghi

           jk
           nlmˇo
           "#
    ));

    // change selections
    cx.set_state(indoc!(
        r#"abc
           def«ˇg»hi

           jk
           nlmo
           "#
    ));

    cx.update_editor(|view, cx| {
        view.add_selection_below(&Default::default(), cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abc
           def«ˇg»hi

           jk
           nlm«ˇo»
           "#
    ));

    cx.update_editor(|view, cx| {
        view.add_selection_below(&Default::default(), cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abc
           def«ˇg»hi

           jk
           nlm«ˇo»
           "#
    ));

    cx.update_editor(|view, cx| {
        view.add_selection_above(&Default::default(), cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abc
           def«ˇg»hi

           jk
           nlmo
           "#
    ));

    cx.update_editor(|view, cx| {
        view.add_selection_above(&Default::default(), cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abc
           def«ˇg»hi

           jk
           nlmo
           "#
    ));

    // Change selections again
    cx.set_state(indoc!(
        r#"a«bc
           defgˇ»hi

           jk
           nlmo
           "#
    ));

    cx.update_editor(|view, cx| {
        view.add_selection_below(&Default::default(), cx);
    });

    cx.assert_editor_state(indoc!(
        r#"a«bcˇ»
           d«efgˇ»hi

           j«kˇ»
           nlmo
           "#
    ));

    cx.update_editor(|view, cx| {
        view.add_selection_below(&Default::default(), cx);
    });
    cx.assert_editor_state(indoc!(
        r#"a«bcˇ»
           d«efgˇ»hi

           j«kˇ»
           n«lmoˇ»
           "#
    ));
    cx.update_editor(|view, cx| {
        view.add_selection_above(&Default::default(), cx);
    });

    cx.assert_editor_state(indoc!(
        r#"a«bcˇ»
           d«efgˇ»hi

           j«kˇ»
           nlmo
           "#
    ));

    // Change selections again
    cx.set_state(indoc!(
        r#"abc
           d«ˇefghi

           jk
           nlm»o
           "#
    ));

    cx.update_editor(|view, cx| {
        view.add_selection_above(&Default::default(), cx);
    });

    cx.assert_editor_state(indoc!(
        r#"a«ˇbc»
           d«ˇef»ghi

           j«ˇk»
           n«ˇlm»o
           "#
    ));

    cx.update_editor(|view, cx| {
        view.add_selection_below(&Default::default(), cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abc
           d«ˇef»ghi

           j«ˇk»
           n«ˇlm»o
           "#
    ));
}

#[gpui::test]
async fn test_select_next(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state("abc\nˇabc abc\ndefabc\nabc");

    cx.update_editor(|e, cx| e.select_next(&SelectNext::default(), cx))
        .unwrap();
    cx.assert_editor_state("abc\n«abcˇ» abc\ndefabc\nabc");

    cx.update_editor(|e, cx| e.select_next(&SelectNext::default(), cx))
        .unwrap();
    cx.assert_editor_state("abc\n«abcˇ» «abcˇ»\ndefabc\nabc");

    cx.update_editor(|view, cx| view.undo_selection(&UndoSelection, cx));
    cx.assert_editor_state("abc\n«abcˇ» abc\ndefabc\nabc");

    cx.update_editor(|view, cx| view.redo_selection(&RedoSelection, cx));
    cx.assert_editor_state("abc\n«abcˇ» «abcˇ»\ndefabc\nabc");

    cx.update_editor(|e, cx| e.select_next(&SelectNext::default(), cx))
        .unwrap();
    cx.assert_editor_state("abc\n«abcˇ» «abcˇ»\ndefabc\n«abcˇ»");

    cx.update_editor(|e, cx| e.select_next(&SelectNext::default(), cx))
        .unwrap();
    cx.assert_editor_state("«abcˇ»\n«abcˇ» «abcˇ»\ndefabc\n«abcˇ»");
}

#[gpui::test]
async fn test_select_all_matches(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state("abc\nˇabc abc\ndefabc\nabc");

    cx.update_editor(|e, cx| e.select_all_matches(&SelectAllMatches, cx))
        .unwrap();
    cx.assert_editor_state("«abcˇ»\n«abcˇ» «abcˇ»\ndefabc\n«abcˇ»");
}

#[gpui::test]
async fn test_select_next_with_multiple_carets(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state(
        r#"let foo = 2;
lˇet foo = 2;
let fooˇ = 2;
let foo = 2;
let foo = ˇ2;"#,
    );

    cx.update_editor(|e, cx| e.select_next(&SelectNext::default(), cx))
        .unwrap();
    cx.assert_editor_state(
        r#"let foo = 2;
«letˇ» foo = 2;
let «fooˇ» = 2;
let foo = 2;
let foo = «2ˇ»;"#,
    );

    // noop for multiple selections with different contents
    cx.update_editor(|e, cx| e.select_next(&SelectNext::default(), cx))
        .unwrap();
    cx.assert_editor_state(
        r#"let foo = 2;
«letˇ» foo = 2;
let «fooˇ» = 2;
let foo = 2;
let foo = «2ˇ»;"#,
    );
}

#[gpui::test]
async fn test_select_previous_with_single_caret(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state("abc\nˇabc abc\ndefabc\nabc");

    cx.update_editor(|e, cx| e.select_previous(&SelectPrevious::default(), cx))
        .unwrap();
    cx.assert_editor_state("abc\n«abcˇ» abc\ndefabc\nabc");

    cx.update_editor(|e, cx| e.select_previous(&SelectPrevious::default(), cx))
        .unwrap();
    cx.assert_editor_state("«abcˇ»\n«abcˇ» abc\ndefabc\nabc");

    cx.update_editor(|view, cx| view.undo_selection(&UndoSelection, cx));
    cx.assert_editor_state("abc\n«abcˇ» abc\ndefabc\nabc");

    cx.update_editor(|view, cx| view.redo_selection(&RedoSelection, cx));
    cx.assert_editor_state("«abcˇ»\n«abcˇ» abc\ndefabc\nabc");

    cx.update_editor(|e, cx| e.select_previous(&SelectPrevious::default(), cx))
        .unwrap();
    cx.assert_editor_state("«abcˇ»\n«abcˇ» abc\ndefabc\n«abcˇ»");

    cx.update_editor(|e, cx| e.select_previous(&SelectPrevious::default(), cx))
        .unwrap();
    cx.assert_editor_state("«abcˇ»\n«abcˇ» abc\ndef«abcˇ»\n«abcˇ»");

    cx.update_editor(|e, cx| e.select_previous(&SelectPrevious::default(), cx))
        .unwrap();
    cx.assert_editor_state("«abcˇ»\n«abcˇ» «abcˇ»\ndef«abcˇ»\n«abcˇ»");
}

#[gpui::test]
async fn test_select_previous_with_multiple_carets(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state(
        r#"let foo = 2;
lˇet foo = 2;
let fooˇ = 2;
let foo = 2;
let foo = ˇ2;"#,
    );

    cx.update_editor(|e, cx| e.select_previous(&SelectPrevious::default(), cx))
        .unwrap();
    cx.assert_editor_state(
        r#"let foo = 2;
«letˇ» foo = 2;
let «fooˇ» = 2;
let foo = 2;
let foo = «2ˇ»;"#,
    );

    // noop for multiple selections with different contents
    cx.update_editor(|e, cx| e.select_previous(&SelectPrevious::default(), cx))
        .unwrap();
    cx.assert_editor_state(
        r#"let foo = 2;
«letˇ» foo = 2;
let «fooˇ» = 2;
let foo = 2;
let foo = «2ˇ»;"#,
    );
}

#[gpui::test]
async fn test_select_previous_with_single_selection(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state("abc\n«ˇabc» abc\ndefabc\nabc");

    cx.update_editor(|e, cx| e.select_previous(&SelectPrevious::default(), cx))
        .unwrap();
    cx.assert_editor_state("«abcˇ»\n«ˇabc» abc\ndefabc\nabc");

    cx.update_editor(|e, cx| e.select_previous(&SelectPrevious::default(), cx))
        .unwrap();
    cx.assert_editor_state("«abcˇ»\n«ˇabc» abc\ndefabc\n«abcˇ»");

    cx.update_editor(|view, cx| view.undo_selection(&UndoSelection, cx));
    cx.assert_editor_state("«abcˇ»\n«ˇabc» abc\ndefabc\nabc");

    cx.update_editor(|view, cx| view.redo_selection(&RedoSelection, cx));
    cx.assert_editor_state("«abcˇ»\n«ˇabc» abc\ndefabc\n«abcˇ»");

    cx.update_editor(|e, cx| e.select_previous(&SelectPrevious::default(), cx))
        .unwrap();
    cx.assert_editor_state("«abcˇ»\n«ˇabc» abc\ndef«abcˇ»\n«abcˇ»");

    cx.update_editor(|e, cx| e.select_previous(&SelectPrevious::default(), cx))
        .unwrap();
    cx.assert_editor_state("«abcˇ»\n«ˇabc» «abcˇ»\ndef«abcˇ»\n«abcˇ»");
}

#[gpui::test]
async fn test_select_larger_smaller_syntax_node(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

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

    let buffer = cx.new_model(|cx| {
        Buffer::new(0, BufferId::new(cx.entity_id().as_u64()).unwrap(), text)
            .with_language(language, cx)
    });
    let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (view, cx) = cx.add_window_view(|cx| build_editor(buffer, cx));

    view.condition::<crate::EditorEvent>(&cx, |view, cx| !view.buffer.read(cx).is_parsing(cx))
        .await;

    _ = view.update(cx, |view, cx| {
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

    _ = view.update(cx, |view, cx| {
        view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
    });
    assert_eq!(
        view.update(cx, |view, cx| view.selections.display_ranges(cx)),
        &[
            DisplayPoint::new(0, 16)..DisplayPoint::new(0, 28),
            DisplayPoint::new(4, 1)..DisplayPoint::new(2, 0),
        ]
    );

    _ = view.update(cx, |view, cx| {
        view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
    });
    assert_eq!(
        view.update(cx, |view, cx| view.selections.display_ranges(cx)),
        &[DisplayPoint::new(5, 0)..DisplayPoint::new(0, 0)]
    );

    // Trying to expand the selected syntax node one more time has no effect.
    _ = view.update(cx, |view, cx| {
        view.select_larger_syntax_node(&SelectLargerSyntaxNode, cx);
    });
    assert_eq!(
        view.update(cx, |view, cx| view.selections.display_ranges(cx)),
        &[DisplayPoint::new(5, 0)..DisplayPoint::new(0, 0)]
    );

    _ = view.update(cx, |view, cx| {
        view.select_smaller_syntax_node(&SelectSmallerSyntaxNode, cx);
    });
    assert_eq!(
        view.update(cx, |view, cx| view.selections.display_ranges(cx)),
        &[
            DisplayPoint::new(0, 16)..DisplayPoint::new(0, 28),
            DisplayPoint::new(4, 1)..DisplayPoint::new(2, 0),
        ]
    );

    _ = view.update(cx, |view, cx| {
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

    _ = view.update(cx, |view, cx| {
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
    _ = view.update(cx, |view, cx| {
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
    _ = view.update(cx, |view, cx| {
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
    init_test(cx, |_| {});

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

    let buffer = cx.new_model(|cx| {
        Buffer::new(0, BufferId::new(cx.entity_id().as_u64()).unwrap(), text)
            .with_language(language, cx)
    });
    let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|cx| build_editor(buffer, cx));
    editor
        .condition::<crate::EditorEvent>(cx, |editor, cx| !editor.buffer.read(cx).is_parsing(cx))
        .await;

    _ = editor.update(cx, |editor, cx| {
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
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

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
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

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
        Some(tree_sitter_typescript::language_tsx()),
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
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

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
    init_test(cx, |_| {});

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

    let buffer = cx.new_model(|cx| {
        Buffer::new(0, BufferId::new(cx.entity_id().as_u64()).unwrap(), text)
            .with_language(language, cx)
    });
    let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (view, cx) = cx.add_window_view(|cx| build_editor(buffer, cx));
    view.condition::<crate::EditorEvent>(cx, |view, cx| !view.buffer.read(cx).is_parsing(cx))
        .await;

    _ = view.update(cx, |view, cx| {
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
    init_test(cx, |_| {});

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

    let buffer = cx.new_model(|cx| {
        Buffer::new(0, BufferId::new(cx.entity_id().as_u64()).unwrap(), text)
            .with_language(language, cx)
    });
    let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|cx| build_editor(buffer, cx));
    editor
        .condition::<crate::EditorEvent>(cx, |view, cx| !view.buffer.read(cx).is_parsing(cx))
        .await;

    _ = editor.update(cx, |editor, cx| {
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
    init_test(cx, |_| {});

    let (text, insertion_ranges) = marked_text_ranges(
        indoc! {"
            a.ˇ b
            a.ˇ b
            a.ˇ b
        "},
        false,
    );

    let buffer = cx.update(|cx| MultiBuffer::build_simple(&text, cx));
    let (editor, cx) = cx.add_window_view(|cx| build_editor(buffer, cx));

    _ = editor.update(cx, |editor, cx| {
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
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    fs.insert_file("/file.rs", Default::default()).await;

    let project = Project::test(fs, ["/file.rs".as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                document_formatting_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/file.rs", cx))
        .await
        .unwrap();

    cx.executor().start_waiting();
    let fake_server = fake_servers.next().await.unwrap();

    let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|cx| build_editor(buffer, cx));
    _ = editor.update(cx, |editor, cx| editor.set_text("one\ntwo\nthree\n", cx));
    assert!(cx.read(|cx| editor.is_dirty(cx)));

    let save = editor
        .update(cx, |editor, cx| editor.save(true, project.clone(), cx))
        .unwrap();
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
    cx.executor().start_waiting();
    let _x = save.await;

    assert_eq!(
        editor.update(cx, |editor, cx| editor.text(cx)),
        "one, two\nthree\n"
    );
    assert!(!cx.read(|cx| editor.is_dirty(cx)));

    _ = editor.update(cx, |editor, cx| editor.set_text("one\ntwo\nthree\n", cx));
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
    let save = editor
        .update(cx, |editor, cx| editor.save(true, project.clone(), cx))
        .unwrap();
    cx.executor().advance_clock(super::FORMAT_TIMEOUT);
    cx.executor().start_waiting();
    save.await;
    assert_eq!(
        editor.update(cx, |editor, cx| editor.text(cx)),
        "one\ntwo\nthree\n"
    );
    assert!(!cx.read(|cx| editor.is_dirty(cx)));

    // Set rust language override and assert overridden tabsize is sent to language server
    update_test_language_settings(cx, |settings| {
        settings.languages.insert(
            "Rust".into(),
            LanguageSettingsContent {
                tab_size: NonZeroU32::new(8),
                ..Default::default()
            },
        );
    });

    let save = editor
        .update(cx, |editor, cx| editor.save(true, project.clone(), cx))
        .unwrap();
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
    cx.executor().start_waiting();
    save.await;
}

#[gpui::test]
async fn test_range_format_during_save(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    fs.insert_file("/file.rs", Default::default()).await;

    let project = Project::test(fs, ["/file.rs".as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                document_range_formatting_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/file.rs", cx))
        .await
        .unwrap();

    cx.executor().start_waiting();
    let fake_server = fake_servers.next().await.unwrap();

    let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|cx| build_editor(buffer, cx));
    _ = editor.update(cx, |editor, cx| editor.set_text("one\ntwo\nthree\n", cx));
    assert!(cx.read(|cx| editor.is_dirty(cx)));

    let save = editor
        .update(cx, |editor, cx| editor.save(true, project.clone(), cx))
        .unwrap();
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
    cx.executor().start_waiting();
    save.await;
    assert_eq!(
        editor.update(cx, |editor, cx| editor.text(cx)),
        "one, two\nthree\n"
    );
    assert!(!cx.read(|cx| editor.is_dirty(cx)));

    _ = editor.update(cx, |editor, cx| editor.set_text("one\ntwo\nthree\n", cx));
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
    let save = editor
        .update(cx, |editor, cx| editor.save(true, project.clone(), cx))
        .unwrap();
    cx.executor().advance_clock(super::FORMAT_TIMEOUT);
    cx.executor().start_waiting();
    save.await;
    assert_eq!(
        editor.update(cx, |editor, cx| editor.text(cx)),
        "one\ntwo\nthree\n"
    );
    assert!(!cx.read(|cx| editor.is_dirty(cx)));

    // Set rust language override and assert overridden tabsize is sent to language server
    update_test_language_settings(cx, |settings| {
        settings.languages.insert(
            "Rust".into(),
            LanguageSettingsContent {
                tab_size: NonZeroU32::new(8),
                ..Default::default()
            },
        );
    });

    let save = editor
        .update(cx, |editor, cx| editor.save(true, project.clone(), cx))
        .unwrap();
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
    cx.executor().start_waiting();
    save.await;
}

#[gpui::test]
async fn test_document_format_manual_trigger(cx: &mut gpui::TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.formatter = Some(language_settings::Formatter::LanguageServer)
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_file("/file.rs", Default::default()).await;

    let project = Project::test(fs, ["/file.rs".as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(Arc::new(Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            // Enable Prettier formatting for the same buffer, and ensure
            // LSP is called instead of Prettier.
            prettier_parser_name: Some("test_parser".to_string()),
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    )));
    let mut fake_servers = language_registry.register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                document_formatting_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/file.rs", cx))
        .await
        .unwrap();

    cx.executor().start_waiting();
    let fake_server = fake_servers.next().await.unwrap();

    let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|cx| build_editor(buffer, cx));
    _ = editor.update(cx, |editor, cx| editor.set_text("one\ntwo\nthree\n", cx));

    let format = editor
        .update(cx, |editor, cx| {
            editor.perform_format(project.clone(), FormatTrigger::Manual, cx)
        })
        .unwrap();
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
    cx.executor().start_waiting();
    format.await;
    assert_eq!(
        editor.update(cx, |editor, cx| editor.text(cx)),
        "one, two\nthree\n"
    );

    _ = editor.update(cx, |editor, cx| editor.set_text("one\ntwo\nthree\n", cx));
    // Ensure we don't lock if formatting hangs.
    fake_server.handle_request::<lsp::request::Formatting, _, _>(move |params, _| async move {
        assert_eq!(
            params.text_document.uri,
            lsp::Url::from_file_path("/file.rs").unwrap()
        );
        futures::future::pending::<()>().await;
        unreachable!()
    });
    let format = editor
        .update(cx, |editor, cx| {
            editor.perform_format(project, FormatTrigger::Manual, cx)
        })
        .unwrap();
    cx.executor().advance_clock(super::FORMAT_TIMEOUT);
    cx.executor().start_waiting();
    format.await;
    assert_eq!(
        editor.update(cx, |editor, cx| editor.text(cx)),
        "one\ntwo\nthree\n"
    );
}

#[gpui::test]
async fn test_concurrent_format_requests(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

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
            let executor = cx.background_executor().clone();
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
    cx.executor().run_until_parked();

    // Submit a second format request.
    let format_2 = cx
        .update_editor(|editor, cx| editor.format(&Format, cx))
        .unwrap();
    cx.executor().run_until_parked();

    // Wait for both format requests to complete
    cx.executor().advance_clock(Duration::from_millis(200));
    cx.executor().start_waiting();
    format_1.await.unwrap();
    cx.executor().start_waiting();
    format_2.await.unwrap();

    // The formatting edits only happens once.
    cx.assert_editor_state(indoc! {"
        one
            .twoˇ
    "});
}

#[gpui::test]
async fn test_strip_whitespace_and_format_via_lsp(cx: &mut gpui::TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.formatter = Some(language_settings::Formatter::Auto)
    });

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
            "twoˇ",   //
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
            "twoˇ",  //
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
            "twoˇ",   //
            "three ", //
            "four",   //
        ]
        .join("\n"),
    );
}

#[gpui::test]
async fn test_completion(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            completion_provider: Some(lsp::CompletionOptions {
                trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                resolve_provider: Some(true),
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
        editor.context_menu_next(&Default::default(), cx);
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
                "overlapping additional edit",
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
    assert!(cx.editor(|e, _| e.context_menu.read().is_none()));
    cx.simulate_keystroke("s");
    assert!(cx.editor(|e, _| e.context_menu.read().is_none()));

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

    _ = cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|settings, cx| {
            settings.update_user_settings::<EditorSettings>(cx, |settings| {
                settings.show_completions_on_input = Some(false);
            });
        })
    });
    cx.set_state("editorˇ");
    cx.simulate_keystroke(".");
    assert!(cx.editor(|e, _| e.context_menu.read().is_none()));
    cx.simulate_keystroke("c");
    cx.simulate_keystroke("l");
    cx.simulate_keystroke("o");
    cx.assert_editor_state("editor.cloˇ");
    assert!(cx.editor(|e, _| e.context_menu.read().is_none()));
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
}

#[gpui::test]
async fn test_toggle_comment(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;
    let language = Arc::new(Language::new(
        LanguageConfig {
            line_comments: vec!["// ".into()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    ));
    cx.update_buffer(|buffer, cx| buffer.set_language(Some(language), cx));

    // If multiple selections intersect a line, the line is only toggled once.
    cx.set_state(indoc! {"
        fn a() {
            «//b();
            ˇ»// «c();
            //ˇ»  d();
        }
    "});

    cx.update_editor(|e, cx| e.toggle_comments(&ToggleComments::default(), cx));

    cx.assert_editor_state(indoc! {"
        fn a() {
            «b();
            c();
            ˇ» d();
        }
    "});

    // The comment prefix is inserted at the same column for every line in a
    // selection.
    cx.update_editor(|e, cx| e.toggle_comments(&ToggleComments::default(), cx));

    cx.assert_editor_state(indoc! {"
        fn a() {
            // «b();
            // c();
            ˇ»//  d();
        }
    "});

    // If a selection ends at the beginning of a line, that line is not toggled.
    cx.set_selections_state(indoc! {"
        fn a() {
            // b();
            «// c();
        ˇ»    //  d();
        }
    "});

    cx.update_editor(|e, cx| e.toggle_comments(&ToggleComments::default(), cx));

    cx.assert_editor_state(indoc! {"
        fn a() {
            // b();
            «c();
        ˇ»    //  d();
        }
    "});

    // If a selection span a single line and is empty, the line is toggled.
    cx.set_state(indoc! {"
        fn a() {
            a();
            b();
        ˇ
        }
    "});

    cx.update_editor(|e, cx| e.toggle_comments(&ToggleComments::default(), cx));

    cx.assert_editor_state(indoc! {"
        fn a() {
            a();
            b();
        //•ˇ
        }
    "});

    // If a selection span multiple lines, empty lines are not toggled.
    cx.set_state(indoc! {"
        fn a() {
            «a();

            c();ˇ»
        }
    "});

    cx.update_editor(|e, cx| e.toggle_comments(&ToggleComments::default(), cx));

    cx.assert_editor_state(indoc! {"
        fn a() {
            // «a();

            // c();ˇ»
        }
    "});
}

#[gpui::test]
async fn test_advance_downward_on_toggle_comment(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let language = Arc::new(Language::new(
        LanguageConfig {
            line_comments: vec!["// ".into()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    ));

    let registry = Arc::new(LanguageRegistry::test());
    registry.add(language.clone());

    let mut cx = EditorTestContext::new(cx).await;
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
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

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
            line_comments: vec!["// ".into()],
            ..Default::default()
        },
        Some(tree_sitter_typescript::language_tsx()),
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
    cx.executor().run_until_parked();
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
fn test_editing_disjoint_excerpts(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let buffer = cx.new_model(|cx| {
        Buffer::new(
            0,
            BufferId::new(cx.entity_id().as_u64()).unwrap(),
            sample_text(3, 4, 'a'),
        )
    });
    let multibuffer = cx.new_model(|cx| {
        let mut multibuffer = MultiBuffer::new(0, ReadWrite);
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
        assert_eq!(multibuffer.read(cx).text(), "aaaa\nbbbb");
        multibuffer
    });

    let (view, cx) = cx.add_window_view(|cx| build_editor(multibuffer, cx));
    _ = view.update(cx, |view, cx| {
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
        );

        // Ensure the cursor's head is respected when deleting across an excerpt boundary.
        view.change_selections(None, cx, |s| {
            s.select_ranges([Point::new(0, 2)..Point::new(1, 2)])
        });
        view.backspace(&Default::default(), cx);
        assert_eq!(view.text(cx), "Xa\nbbb");
        assert_eq!(
            view.selections.ranges(cx),
            [Point::new(1, 0)..Point::new(1, 0)]
        );

        view.change_selections(None, cx, |s| {
            s.select_ranges([Point::new(1, 1)..Point::new(0, 1)])
        });
        view.backspace(&Default::default(), cx);
        assert_eq!(view.text(cx), "X\nbb");
        assert_eq!(
            view.selections.ranges(cx),
            [Point::new(0, 1)..Point::new(0, 1)]
        );
    });
}

#[gpui::test]
fn test_editing_overlapping_excerpts(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

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
    let buffer = cx.new_model(|cx| {
        Buffer::new(
            0,
            BufferId::new(cx.entity_id().as_u64()).unwrap(),
            initial_text,
        )
    });
    let multibuffer = cx.new_model(|cx| {
        let mut multibuffer = MultiBuffer::new(0, ReadWrite);
        multibuffer.push_excerpts(buffer, excerpt_ranges, cx);
        multibuffer
    });

    let (view, cx) = cx.add_window_view(|cx| build_editor(multibuffer, cx));
    _ = view.update(cx, |view, cx| {
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
fn test_refresh_selections(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let buffer = cx.new_model(|cx| {
        Buffer::new(
            0,
            BufferId::new(cx.entity_id().as_u64()).unwrap(),
            sample_text(3, 4, 'a'),
        )
    });
    let mut excerpt1_id = None;
    let multibuffer = cx.new_model(|cx| {
        let mut multibuffer = MultiBuffer::new(0, ReadWrite);
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
        assert_eq!(multibuffer.read(cx).text(), "aaaa\nbbbb\nbbbb\ncccc");
        multibuffer
    });

    let editor = cx.add_window(|cx| {
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
    _ = editor.update(cx, |editor, cx| {
        editor.change_selections(None, cx, |s| s.refresh());
        assert_eq!(
            editor.selections.ranges(cx),
            [
                Point::new(1, 3)..Point::new(1, 3),
                Point::new(2, 1)..Point::new(2, 1),
            ]
        );
    });

    _ = multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.remove_excerpts([excerpt1_id.unwrap()], cx);
    });
    _ = editor.update(cx, |editor, cx| {
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
fn test_refresh_selections_while_selecting_with_mouse(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let buffer = cx.new_model(|cx| {
        Buffer::new(
            0,
            BufferId::new(cx.entity_id().as_u64()).unwrap(),
            sample_text(3, 4, 'a'),
        )
    });
    let mut excerpt1_id = None;
    let multibuffer = cx.new_model(|cx| {
        let mut multibuffer = MultiBuffer::new(0, ReadWrite);
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
        assert_eq!(multibuffer.read(cx).text(), "aaaa\nbbbb\nbbbb\ncccc");
        multibuffer
    });

    let editor = cx.add_window(|cx| {
        let mut editor = build_editor(multibuffer.clone(), cx);
        let snapshot = editor.snapshot(cx);
        editor.begin_selection(Point::new(1, 3).to_display_point(&snapshot), false, 1, cx);
        assert_eq!(
            editor.selections.ranges(cx),
            [Point::new(1, 3)..Point::new(1, 3)]
        );
        editor
    });

    _ = multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.remove_excerpts([excerpt1_id.unwrap()], cx);
    });
    _ = editor.update(cx, |editor, cx| {
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
    init_test(cx, |_| {});

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

    let buffer = cx.new_model(|cx| {
        Buffer::new(0, BufferId::new(cx.entity_id().as_u64()).unwrap(), text)
            .with_language(language, cx)
    });
    let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (view, cx) = cx.add_window_view(|cx| build_editor(buffer, cx));
    view.condition::<crate::EditorEvent>(cx, |view, cx| !view.buffer.read(cx).is_parsing(cx))
        .await;

    _ = view.update(cx, |view, cx| {
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
fn test_highlighted_ranges(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|cx| {
        let buffer = MultiBuffer::build_simple(&sample_text(16, 8, 'a'), cx);
        build_editor(buffer.clone(), cx)
    });

    _ = editor.update(cx, |editor, cx| {
        struct Type1;
        struct Type2;

        let buffer = editor.buffer.read(cx).snapshot(cx);

        let anchor_range =
            |range: Range<Point>| buffer.anchor_after(range.start)..buffer.anchor_after(range.end);

        editor.highlight_background::<Type1>(
            vec![
                anchor_range(Point::new(2, 1)..Point::new(2, 3)),
                anchor_range(Point::new(4, 2)..Point::new(4, 4)),
                anchor_range(Point::new(6, 3)..Point::new(6, 5)),
                anchor_range(Point::new(8, 4)..Point::new(8, 6)),
            ],
            |_| Hsla::red(),
            cx,
        );
        editor.highlight_background::<Type2>(
            vec![
                anchor_range(Point::new(3, 2)..Point::new(3, 5)),
                anchor_range(Point::new(5, 3)..Point::new(5, 6)),
                anchor_range(Point::new(7, 4)..Point::new(7, 7)),
                anchor_range(Point::new(9, 5)..Point::new(9, 8)),
            ],
            |_| Hsla::green(),
            cx,
        );

        let snapshot = editor.snapshot(cx);
        let mut highlighted_ranges = editor.background_highlights_in_range(
            anchor_range(Point::new(3, 4)..Point::new(7, 4)),
            &snapshot,
            cx.theme().colors(),
        );
        // Enforce a consistent ordering based on color without relying on the ordering of the
        // highlight's `TypeId` which is non-executor.
        highlighted_ranges.sort_unstable_by_key(|(_, color)| *color);
        assert_eq!(
            highlighted_ranges,
            &[
                (
                    DisplayPoint::new(4, 2)..DisplayPoint::new(4, 4),
                    Hsla::red(),
                ),
                (
                    DisplayPoint::new(6, 3)..DisplayPoint::new(6, 5),
                    Hsla::red(),
                ),
                (
                    DisplayPoint::new(3, 2)..DisplayPoint::new(3, 5),
                    Hsla::green(),
                ),
                (
                    DisplayPoint::new(5, 3)..DisplayPoint::new(5, 6),
                    Hsla::green(),
                ),
            ]
        );
        assert_eq!(
            editor.background_highlights_in_range(
                anchor_range(Point::new(5, 6)..Point::new(6, 4)),
                &snapshot,
                cx.theme().colors(),
            ),
            &[(
                DisplayPoint::new(6, 3)..DisplayPoint::new(6, 5),
                Hsla::red(),
            )]
        );
    });
}

#[gpui::test]
async fn test_following(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, ["/file.rs".as_ref()], cx).await;

    let buffer = project.update(cx, |project, cx| {
        let buffer = project
            .create_buffer(&sample_text(16, 8, 'a'), None, cx)
            .unwrap();
        cx.new_model(|cx| MultiBuffer::singleton(buffer, cx))
    });
    let leader = cx.add_window(|cx| build_editor(buffer.clone(), cx));
    let follower = cx.update(|cx| {
        cx.open_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(Bounds::from_corners(
                    gpui::Point::new(0_f64.into(), 0_f64.into()),
                    gpui::Point::new(10_f64.into(), 80_f64.into()),
                )),
                ..Default::default()
            },
            |cx| cx.new_view(|cx| build_editor(buffer.clone(), cx)),
        )
    });

    let is_still_following = Rc::new(RefCell::new(true));
    let follower_edit_event_count = Rc::new(RefCell::new(0));
    let pending_update = Rc::new(RefCell::new(None));
    _ = follower.update(cx, {
        let update = pending_update.clone();
        let is_still_following = is_still_following.clone();
        let follower_edit_event_count = follower_edit_event_count.clone();
        |_, cx| {
            cx.subscribe(
                &leader.root_view(cx).unwrap(),
                move |_, leader, event, cx| {
                    leader
                        .read(cx)
                        .add_event_to_update_proto(event, &mut update.borrow_mut(), cx);
                },
            )
            .detach();

            cx.subscribe(
                &follower.root_view(cx).unwrap(),
                move |_, _, event: &EditorEvent, _cx| {
                    if matches!(Editor::to_follow_event(event), Some(FollowEvent::Unfollow)) {
                        *is_still_following.borrow_mut() = false;
                    }

                    if let EditorEvent::BufferEdited = event {
                        *follower_edit_event_count.borrow_mut() += 1;
                    }
                },
            )
            .detach();
        }
    });

    // Update the selections only
    _ = leader.update(cx, |leader, cx| {
        leader.change_selections(None, cx, |s| s.select_ranges([1..1]));
    });
    follower
        .update(cx, |follower, cx| {
            follower.apply_update_proto(&project, pending_update.borrow_mut().take().unwrap(), cx)
        })
        .unwrap()
        .await
        .unwrap();
    _ = follower.update(cx, |follower, cx| {
        assert_eq!(follower.selections.ranges(cx), vec![1..1]);
    });
    assert_eq!(*is_still_following.borrow(), true);
    assert_eq!(*follower_edit_event_count.borrow(), 0);

    // Update the scroll position only
    _ = leader.update(cx, |leader, cx| {
        leader.set_scroll_position(gpui::Point::new(1.5, 3.5), cx);
    });
    follower
        .update(cx, |follower, cx| {
            follower.apply_update_proto(&project, pending_update.borrow_mut().take().unwrap(), cx)
        })
        .unwrap()
        .await
        .unwrap();
    assert_eq!(
        follower
            .update(cx, |follower, cx| follower.scroll_position(cx))
            .unwrap(),
        gpui::Point::new(1.5, 3.5)
    );
    assert_eq!(*is_still_following.borrow(), true);
    assert_eq!(*follower_edit_event_count.borrow(), 0);

    // Update the selections and scroll position. The follower's scroll position is updated
    // via autoscroll, not via the leader's exact scroll position.
    _ = leader.update(cx, |leader, cx| {
        leader.change_selections(None, cx, |s| s.select_ranges([0..0]));
        leader.request_autoscroll(Autoscroll::newest(), cx);
        leader.set_scroll_position(gpui::Point::new(1.5, 3.5), cx);
    });
    follower
        .update(cx, |follower, cx| {
            follower.apply_update_proto(&project, pending_update.borrow_mut().take().unwrap(), cx)
        })
        .unwrap()
        .await
        .unwrap();
    _ = follower.update(cx, |follower, cx| {
        assert_eq!(follower.scroll_position(cx), gpui::Point::new(1.5, 0.0));
        assert_eq!(follower.selections.ranges(cx), vec![0..0]);
    });
    assert_eq!(*is_still_following.borrow(), true);

    // Creating a pending selection that precedes another selection
    _ = leader.update(cx, |leader, cx| {
        leader.change_selections(None, cx, |s| s.select_ranges([1..1]));
        leader.begin_selection(DisplayPoint::new(0, 0), true, 1, cx);
    });
    follower
        .update(cx, |follower, cx| {
            follower.apply_update_proto(&project, pending_update.borrow_mut().take().unwrap(), cx)
        })
        .unwrap()
        .await
        .unwrap();
    _ = follower.update(cx, |follower, cx| {
        assert_eq!(follower.selections.ranges(cx), vec![0..0, 1..1]);
    });
    assert_eq!(*is_still_following.borrow(), true);

    // Extend the pending selection so that it surrounds another selection
    _ = leader.update(cx, |leader, cx| {
        leader.extend_selection(DisplayPoint::new(0, 2), 1, cx);
    });
    follower
        .update(cx, |follower, cx| {
            follower.apply_update_proto(&project, pending_update.borrow_mut().take().unwrap(), cx)
        })
        .unwrap()
        .await
        .unwrap();
    _ = follower.update(cx, |follower, cx| {
        assert_eq!(follower.selections.ranges(cx), vec![0..2]);
    });

    // Scrolling locally breaks the follow
    _ = follower.update(cx, |follower, cx| {
        let top_anchor = follower.buffer().read(cx).read(cx).anchor_after(0);
        follower.set_scroll_anchor(
            ScrollAnchor {
                anchor: top_anchor,
                offset: gpui::Point::new(0.0, 0.5),
            },
            cx,
        );
    });
    assert_eq!(*is_still_following.borrow(), false);
}

#[gpui::test]
async fn test_following_with_multiple_excerpts(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, ["/file.rs".as_ref()], cx).await;
    let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
    let pane = workspace
        .update(cx, |workspace, _| workspace.active_pane().clone())
        .unwrap();

    let cx = &mut VisualTestContext::from_window(*workspace.deref(), cx);

    let leader = pane.update(cx, |_, cx| {
        let multibuffer = cx.new_model(|_| MultiBuffer::new(0, ReadWrite));
        cx.new_view(|cx| build_editor(multibuffer.clone(), cx))
    });

    // Start following the editor when it has no excerpts.
    let mut state_message = leader.update(cx, |leader, cx| leader.to_state_proto(cx));
    let follower_1 = cx
        .update_window(*workspace.deref(), |_, cx| {
            Editor::from_state_proto(
                pane.clone(),
                workspace.root_view(cx).unwrap(),
                ViewId {
                    creator: Default::default(),
                    id: 0,
                },
                &mut state_message,
                cx,
            )
        })
        .unwrap()
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
                    .add_event_to_update_proto(event, &mut update.borrow_mut(), cx);
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
    _ = leader.update(cx, |leader, cx| {
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
        follower_1.update(cx, |editor, cx| editor.text(cx)),
        leader.update(cx, |editor, cx| editor.text(cx))
    );
    update_message.borrow_mut().take();

    // Start following separately after it already has excerpts.
    let mut state_message = leader.update(cx, |leader, cx| leader.to_state_proto(cx));
    let follower_2 = cx
        .update_window(*workspace.deref(), |_, cx| {
            Editor::from_state_proto(
                pane.clone(),
                workspace.root_view(cx).unwrap().clone(),
                ViewId {
                    creator: Default::default(),
                    id: 0,
                },
                &mut state_message,
                cx,
            )
        })
        .unwrap()
        .unwrap()
        .await
        .unwrap();
    assert_eq!(
        follower_2.update(cx, |editor, cx| editor.text(cx)),
        leader.update(cx, |editor, cx| editor.text(cx))
    );

    // Remove some excerpts.
    _ = leader.update(cx, |leader, cx| {
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
        follower_1.update(cx, |editor, cx| editor.text(cx)),
        leader.update(cx, |editor, cx| editor.text(cx))
    );
}

#[gpui::test]
async fn go_to_prev_overlapping_diagnostic(
    executor: BackgroundExecutor,
    cx: &mut gpui::TestAppContext,
) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let project = cx.update_editor(|editor, _| editor.project.clone().unwrap());

    cx.set_state(indoc! {"
        ˇfn func(abc def: i32) -> u32 {
        }
    "});

    _ = cx.update(|cx| {
        _ = project.update(cx, |project, cx| {
            project
                .update_diagnostics(
                    LanguageServerId(0),
                    lsp::PublishDiagnosticsParams {
                        uri: lsp::Url::from_file_path("/root/file").unwrap(),
                        version: None,
                        diagnostics: vec![
                            lsp::Diagnostic {
                                range: lsp::Range::new(
                                    lsp::Position::new(0, 11),
                                    lsp::Position::new(0, 12),
                                ),
                                severity: Some(lsp::DiagnosticSeverity::ERROR),
                                ..Default::default()
                            },
                            lsp::Diagnostic {
                                range: lsp::Range::new(
                                    lsp::Position::new(0, 12),
                                    lsp::Position::new(0, 15),
                                ),
                                severity: Some(lsp::DiagnosticSeverity::ERROR),
                                ..Default::default()
                            },
                            lsp::Diagnostic {
                                range: lsp::Range::new(
                                    lsp::Position::new(0, 25),
                                    lsp::Position::new(0, 28),
                                ),
                                severity: Some(lsp::DiagnosticSeverity::ERROR),
                                ..Default::default()
                            },
                        ],
                    },
                    &[],
                    cx,
                )
                .unwrap()
        });
    });

    executor.run_until_parked();

    cx.update_editor(|editor, cx| {
        editor.go_to_prev_diagnostic(&GoToPrevDiagnostic, cx);
    });

    cx.assert_editor_state(indoc! {"
        fn func(abc def: i32) -> ˇu32 {
        }
    "});

    cx.update_editor(|editor, cx| {
        editor.go_to_prev_diagnostic(&GoToPrevDiagnostic, cx);
    });

    cx.assert_editor_state(indoc! {"
        fn func(abc ˇdef: i32) -> u32 {
        }
    "});

    cx.update_editor(|editor, cx| {
        editor.go_to_prev_diagnostic(&GoToPrevDiagnostic, cx);
    });

    cx.assert_editor_state(indoc! {"
        fn func(abcˇ def: i32) -> u32 {
        }
    "});

    cx.update_editor(|editor, cx| {
        editor.go_to_prev_diagnostic(&GoToPrevDiagnostic, cx);
    });

    cx.assert_editor_state(indoc! {"
        fn func(abc def: i32) -> ˇu32 {
        }
    "});
}

#[gpui::test]
async fn go_to_hunk(executor: BackgroundExecutor, cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

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
    executor.run_until_parked();

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
        editor.go_to_prev_hunk(&GoToPrevHunk, cx);
    });

    cx.assert_editor_state(
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

    cx.update_editor(|editor, cx| {
        for _ in 0..3 {
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
    fn split(text: &str) -> Vec<&str> {
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
    init_test(cx, |_| {});

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

#[gpui::test(iterations = 10)]
async fn test_copilot(executor: BackgroundExecutor, cx: &mut gpui::TestAppContext) {
    // flaky
    init_test(cx, |_| {});

    let (copilot, copilot_lsp) = Copilot::fake(cx);
    _ = cx.update(|cx| Copilot::set_global(copilot, cx));
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

    // When inserting, ensure autocompletion is favored over Copilot suggestions.
    cx.set_state(indoc! {"
        oneˇ
        two
        three
    "});
    cx.simulate_keystroke(".");
    let _ = handle_completion_request(
        &mut cx,
        indoc! {"
            one.|<>
            two
            three
        "},
        vec!["completion_a", "completion_b"],
    );
    handle_copilot_completion_request(
        &copilot_lsp,
        vec![copilot::request::Completion {
            text: "one.copilot1".into(),
            range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 4)),
            ..Default::default()
        }],
        vec![],
    );
    executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
    cx.update_editor(|editor, cx| {
        assert!(editor.context_menu_visible());
        assert!(!editor.has_active_copilot_suggestion(cx));

        // Confirming a completion inserts it and hides the context menu, without showing
        // the copilot suggestion afterwards.
        editor
            .confirm_completion(&Default::default(), cx)
            .unwrap()
            .detach();
        assert!(!editor.context_menu_visible());
        assert!(!editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.text(cx), "one.completion_a\ntwo\nthree\n");
        assert_eq!(editor.display_text(cx), "one.completion_a\ntwo\nthree\n");
    });

    // Ensure Copilot suggestions are shown right away if no autocompletion is available.
    cx.set_state(indoc! {"
        oneˇ
        two
        three
    "});
    cx.simulate_keystroke(".");
    let _ = handle_completion_request(
        &mut cx,
        indoc! {"
            one.|<>
            two
            three
        "},
        vec![],
    );
    handle_copilot_completion_request(
        &copilot_lsp,
        vec![copilot::request::Completion {
            text: "one.copilot1".into(),
            range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 4)),
            ..Default::default()
        }],
        vec![],
    );
    executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
    cx.update_editor(|editor, cx| {
        assert!(!editor.context_menu_visible());
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "one.copilot1\ntwo\nthree\n");
        assert_eq!(editor.text(cx), "one.\ntwo\nthree\n");
    });

    // Reset editor, and ensure autocompletion is still favored over Copilot suggestions.
    cx.set_state(indoc! {"
        oneˇ
        two
        three
    "});
    cx.simulate_keystroke(".");
    let _ = handle_completion_request(
        &mut cx,
        indoc! {"
            one.|<>
            two
            three
        "},
        vec!["completion_a", "completion_b"],
    );
    handle_copilot_completion_request(
        &copilot_lsp,
        vec![copilot::request::Completion {
            text: "one.copilot1".into(),
            range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 4)),
            ..Default::default()
        }],
        vec![],
    );
    executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
    cx.update_editor(|editor, cx| {
        assert!(editor.context_menu_visible());
        assert!(!editor.has_active_copilot_suggestion(cx));

        // When hiding the context menu, the Copilot suggestion becomes visible.
        editor.hide_context_menu(cx);
        assert!(!editor.context_menu_visible());
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "one.copilot1\ntwo\nthree\n");
        assert_eq!(editor.text(cx), "one.\ntwo\nthree\n");
    });

    // Ensure existing completion is interpolated when inserting again.
    cx.simulate_keystroke("c");
    executor.run_until_parked();
    cx.update_editor(|editor, cx| {
        assert!(!editor.context_menu_visible());
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "one.copilot1\ntwo\nthree\n");
        assert_eq!(editor.text(cx), "one.c\ntwo\nthree\n");
    });

    // After debouncing, new Copilot completions should be requested.
    handle_copilot_completion_request(
        &copilot_lsp,
        vec![copilot::request::Completion {
            text: "one.copilot2".into(),
            range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 5)),
            ..Default::default()
        }],
        vec![],
    );
    executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
    cx.update_editor(|editor, cx| {
        assert!(!editor.context_menu_visible());
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "one.copilot2\ntwo\nthree\n");
        assert_eq!(editor.text(cx), "one.c\ntwo\nthree\n");

        // Canceling should remove the active Copilot suggestion.
        editor.cancel(&Default::default(), cx);
        assert!(!editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "one.c\ntwo\nthree\n");
        assert_eq!(editor.text(cx), "one.c\ntwo\nthree\n");

        // After canceling, tabbing shouldn't insert the previously shown suggestion.
        editor.tab(&Default::default(), cx);
        assert!(!editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "one.c   \ntwo\nthree\n");
        assert_eq!(editor.text(cx), "one.c   \ntwo\nthree\n");

        // When undoing the previously active suggestion is shown again.
        editor.undo(&Default::default(), cx);
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "one.copilot2\ntwo\nthree\n");
        assert_eq!(editor.text(cx), "one.c\ntwo\nthree\n");
    });

    // If an edit occurs outside of this editor, the suggestion is still correctly interpolated.
    cx.update_buffer(|buffer, cx| buffer.edit([(5..5, "o")], None, cx));
    cx.update_editor(|editor, cx| {
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "one.copilot2\ntwo\nthree\n");
        assert_eq!(editor.text(cx), "one.co\ntwo\nthree\n");

        // Tabbing when there is an active suggestion inserts it.
        editor.tab(&Default::default(), cx);
        assert!(!editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "one.copilot2\ntwo\nthree\n");
        assert_eq!(editor.text(cx), "one.copilot2\ntwo\nthree\n");

        // When undoing the previously active suggestion is shown again.
        editor.undo(&Default::default(), cx);
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "one.copilot2\ntwo\nthree\n");
        assert_eq!(editor.text(cx), "one.co\ntwo\nthree\n");

        // Hide suggestion.
        editor.cancel(&Default::default(), cx);
        assert!(!editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "one.co\ntwo\nthree\n");
        assert_eq!(editor.text(cx), "one.co\ntwo\nthree\n");
    });

    // If an edit occurs outside of this editor but no suggestion is being shown,
    // we won't make it visible.
    cx.update_buffer(|buffer, cx| buffer.edit([(6..6, "p")], None, cx));
    cx.update_editor(|editor, cx| {
        assert!(!editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "one.cop\ntwo\nthree\n");
        assert_eq!(editor.text(cx), "one.cop\ntwo\nthree\n");
    });

    // Reset the editor to verify how suggestions behave when tabbing on leading indentation.
    cx.update_editor(|editor, cx| {
        editor.set_text("fn foo() {\n  \n}", cx);
        editor.change_selections(None, cx, |s| {
            s.select_ranges([Point::new(1, 2)..Point::new(1, 2)])
        });
    });
    handle_copilot_completion_request(
        &copilot_lsp,
        vec![copilot::request::Completion {
            text: "    let x = 4;".into(),
            range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 2)),
            ..Default::default()
        }],
        vec![],
    );

    cx.update_editor(|editor, cx| editor.next_copilot_suggestion(&Default::default(), cx));
    executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
    cx.update_editor(|editor, cx| {
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "fn foo() {\n    let x = 4;\n}");
        assert_eq!(editor.text(cx), "fn foo() {\n  \n}");

        // Tabbing inside of leading whitespace inserts indentation without accepting the suggestion.
        editor.tab(&Default::default(), cx);
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.text(cx), "fn foo() {\n    \n}");
        assert_eq!(editor.display_text(cx), "fn foo() {\n    let x = 4;\n}");

        // Tabbing again accepts the suggestion.
        editor.tab(&Default::default(), cx);
        assert!(!editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.text(cx), "fn foo() {\n    let x = 4;\n}");
        assert_eq!(editor.display_text(cx), "fn foo() {\n    let x = 4;\n}");
    });
}

#[gpui::test(iterations = 10)]
async fn test_accept_partial_copilot_suggestion(
    executor: BackgroundExecutor,
    cx: &mut gpui::TestAppContext,
) {
    // flaky
    init_test(cx, |_| {});

    let (copilot, copilot_lsp) = Copilot::fake(cx);
    _ = cx.update(|cx| Copilot::set_global(copilot, cx));
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

    // Setup the editor with a completion request.
    cx.set_state(indoc! {"
        oneˇ
        two
        three
    "});
    cx.simulate_keystroke(".");
    let _ = handle_completion_request(
        &mut cx,
        indoc! {"
            one.|<>
            two
            three
        "},
        vec![],
    );
    handle_copilot_completion_request(
        &copilot_lsp,
        vec![copilot::request::Completion {
            text: "one.copilot1".into(),
            range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 4)),
            ..Default::default()
        }],
        vec![],
    );
    executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
    cx.update_editor(|editor, cx| {
        assert!(editor.has_active_copilot_suggestion(cx));

        // Accepting the first word of the suggestion should only accept the first word and still show the rest.
        editor.accept_partial_copilot_suggestion(&Default::default(), cx);
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.text(cx), "one.copilot\ntwo\nthree\n");
        assert_eq!(editor.display_text(cx), "one.copilot1\ntwo\nthree\n");

        // Accepting next word should accept the non-word and copilot suggestion should be gone
        editor.accept_partial_copilot_suggestion(&Default::default(), cx);
        assert!(!editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.text(cx), "one.copilot1\ntwo\nthree\n");
        assert_eq!(editor.display_text(cx), "one.copilot1\ntwo\nthree\n");
    });

    // Reset the editor and check non-word and whitespace completion
    cx.set_state(indoc! {"
        oneˇ
        two
        three
    "});
    cx.simulate_keystroke(".");
    let _ = handle_completion_request(
        &mut cx,
        indoc! {"
            one.|<>
            two
            three
        "},
        vec![],
    );
    handle_copilot_completion_request(
        &copilot_lsp,
        vec![copilot::request::Completion {
            text: "one.123. copilot\n 456".into(),
            range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 4)),
            ..Default::default()
        }],
        vec![],
    );
    executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
    cx.update_editor(|editor, cx| {
        assert!(editor.has_active_copilot_suggestion(cx));

        // Accepting the first word (non-word) of the suggestion should only accept the first word and still show the rest.
        editor.accept_partial_copilot_suggestion(&Default::default(), cx);
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.text(cx), "one.123. \ntwo\nthree\n");
        assert_eq!(
            editor.display_text(cx),
            "one.123. copilot\n 456\ntwo\nthree\n"
        );

        // Accepting next word should accept the next word and copilot suggestion should still exist
        editor.accept_partial_copilot_suggestion(&Default::default(), cx);
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.text(cx), "one.123. copilot\ntwo\nthree\n");
        assert_eq!(
            editor.display_text(cx),
            "one.123. copilot\n 456\ntwo\nthree\n"
        );

        // Accepting the whitespace should accept the non-word/whitespaces with newline and copilot suggestion should be gone
        editor.accept_partial_copilot_suggestion(&Default::default(), cx);
        assert!(!editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.text(cx), "one.123. copilot\n 456\ntwo\nthree\n");
        assert_eq!(
            editor.display_text(cx),
            "one.123. copilot\n 456\ntwo\nthree\n"
        );
    });
}

#[gpui::test]
async fn test_copilot_completion_invalidation(
    executor: BackgroundExecutor,
    cx: &mut gpui::TestAppContext,
) {
    init_test(cx, |_| {});

    let (copilot, copilot_lsp) = Copilot::fake(cx);
    _ = cx.update(|cx| Copilot::set_global(copilot, cx));
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
        one
        twˇ
        three
    "});

    handle_copilot_completion_request(
        &copilot_lsp,
        vec![copilot::request::Completion {
            text: "two.foo()".into(),
            range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 2)),
            ..Default::default()
        }],
        vec![],
    );
    cx.update_editor(|editor, cx| editor.next_copilot_suggestion(&Default::default(), cx));
    executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
    cx.update_editor(|editor, cx| {
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "one\ntwo.foo()\nthree\n");
        assert_eq!(editor.text(cx), "one\ntw\nthree\n");

        editor.backspace(&Default::default(), cx);
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "one\ntwo.foo()\nthree\n");
        assert_eq!(editor.text(cx), "one\nt\nthree\n");

        editor.backspace(&Default::default(), cx);
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "one\ntwo.foo()\nthree\n");
        assert_eq!(editor.text(cx), "one\n\nthree\n");

        // Deleting across the original suggestion range invalidates it.
        editor.backspace(&Default::default(), cx);
        assert!(!editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "one\nthree\n");
        assert_eq!(editor.text(cx), "one\nthree\n");

        // Undoing the deletion restores the suggestion.
        editor.undo(&Default::default(), cx);
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(editor.display_text(cx), "one\ntwo.foo()\nthree\n");
        assert_eq!(editor.text(cx), "one\n\nthree\n");
    });
}

#[gpui::test]
async fn test_copilot_multibuffer(executor: BackgroundExecutor, cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let (copilot, copilot_lsp) = Copilot::fake(cx);
    _ = cx.update(|cx| Copilot::set_global(copilot, cx));

    let buffer_1 = cx.new_model(|cx| {
        Buffer::new(
            0,
            BufferId::new(cx.entity_id().as_u64()).unwrap(),
            "a = 1\nb = 2\n",
        )
    });
    let buffer_2 = cx.new_model(|cx| {
        Buffer::new(
            0,
            BufferId::new(cx.entity_id().as_u64()).unwrap(),
            "c = 3\nd = 4\n",
        )
    });
    let multibuffer = cx.new_model(|cx| {
        let mut multibuffer = MultiBuffer::new(0, ReadWrite);
        multibuffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange {
                context: Point::new(0, 0)..Point::new(2, 0),
                primary: None,
            }],
            cx,
        );
        multibuffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange {
                context: Point::new(0, 0)..Point::new(2, 0),
                primary: None,
            }],
            cx,
        );
        multibuffer
    });
    let editor = cx.add_window(|cx| build_editor(multibuffer, cx));

    handle_copilot_completion_request(
        &copilot_lsp,
        vec![copilot::request::Completion {
            text: "b = 2 + a".into(),
            range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 5)),
            ..Default::default()
        }],
        vec![],
    );
    _ = editor.update(cx, |editor, cx| {
        // Ensure copilot suggestions are shown for the first excerpt.
        editor.change_selections(None, cx, |s| {
            s.select_ranges([Point::new(1, 5)..Point::new(1, 5)])
        });
        editor.next_copilot_suggestion(&Default::default(), cx);
    });
    executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
    _ = editor.update(cx, |editor, cx| {
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(
            editor.display_text(cx),
            "\n\na = 1\nb = 2 + a\n\n\n\nc = 3\nd = 4\n"
        );
        assert_eq!(editor.text(cx), "a = 1\nb = 2\n\nc = 3\nd = 4\n");
    });

    handle_copilot_completion_request(
        &copilot_lsp,
        vec![copilot::request::Completion {
            text: "d = 4 + c".into(),
            range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 6)),
            ..Default::default()
        }],
        vec![],
    );
    _ = editor.update(cx, |editor, cx| {
        // Move to another excerpt, ensuring the suggestion gets cleared.
        editor.change_selections(None, cx, |s| {
            s.select_ranges([Point::new(4, 5)..Point::new(4, 5)])
        });
        assert!(!editor.has_active_copilot_suggestion(cx));
        assert_eq!(
            editor.display_text(cx),
            "\n\na = 1\nb = 2\n\n\n\nc = 3\nd = 4\n"
        );
        assert_eq!(editor.text(cx), "a = 1\nb = 2\n\nc = 3\nd = 4\n");

        // Type a character, ensuring we don't even try to interpolate the previous suggestion.
        editor.handle_input(" ", cx);
        assert!(!editor.has_active_copilot_suggestion(cx));
        assert_eq!(
            editor.display_text(cx),
            "\n\na = 1\nb = 2\n\n\n\nc = 3\nd = 4 \n"
        );
        assert_eq!(editor.text(cx), "a = 1\nb = 2\n\nc = 3\nd = 4 \n");
    });

    // Ensure the new suggestion is displayed when the debounce timeout expires.
    executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
    _ = editor.update(cx, |editor, cx| {
        assert!(editor.has_active_copilot_suggestion(cx));
        assert_eq!(
            editor.display_text(cx),
            "\n\na = 1\nb = 2\n\n\n\nc = 3\nd = 4 + c\n"
        );
        assert_eq!(editor.text(cx), "a = 1\nb = 2\n\nc = 3\nd = 4 \n");
    });
}

#[gpui::test]
async fn test_copilot_disabled_globs(executor: BackgroundExecutor, cx: &mut gpui::TestAppContext) {
    init_test(cx, |settings| {
        settings
            .copilot
            .get_or_insert(Default::default())
            .disabled_globs = Some(vec![".env*".to_string()]);
    });

    let (copilot, copilot_lsp) = Copilot::fake(cx);
    _ = cx.update(|cx| Copilot::set_global(copilot, cx));

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/test",
        json!({
            ".env": "SECRET=something\n",
            "README.md": "hello\n"
        }),
    )
    .await;
    let project = Project::test(fs, ["/test".as_ref()], cx).await;

    let private_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/test/.env", cx)
        })
        .await
        .unwrap();
    let public_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/test/README.md", cx)
        })
        .await
        .unwrap();

    let multibuffer = cx.new_model(|cx| {
        let mut multibuffer = MultiBuffer::new(0, ReadWrite);
        multibuffer.push_excerpts(
            private_buffer.clone(),
            [ExcerptRange {
                context: Point::new(0, 0)..Point::new(1, 0),
                primary: None,
            }],
            cx,
        );
        multibuffer.push_excerpts(
            public_buffer.clone(),
            [ExcerptRange {
                context: Point::new(0, 0)..Point::new(1, 0),
                primary: None,
            }],
            cx,
        );
        multibuffer
    });
    let editor = cx.add_window(|cx| build_editor(multibuffer, cx));

    let mut copilot_requests = copilot_lsp
        .handle_request::<copilot::request::GetCompletions, _, _>(move |_params, _cx| async move {
            Ok(copilot::request::GetCompletionsResult {
                completions: vec![copilot::request::Completion {
                    text: "next line".into(),
                    range: lsp::Range::new(lsp::Position::new(1, 0), lsp::Position::new(1, 0)),
                    ..Default::default()
                }],
            })
        });

    _ = editor.update(cx, |editor, cx| {
        editor.change_selections(None, cx, |selections| {
            selections.select_ranges([Point::new(0, 0)..Point::new(0, 0)])
        });
        editor.next_copilot_suggestion(&Default::default(), cx);
    });

    executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
    assert!(copilot_requests.try_next().is_err());

    _ = editor.update(cx, |editor, cx| {
        editor.change_selections(None, cx, |s| {
            s.select_ranges([Point::new(2, 0)..Point::new(2, 0)])
        });
        editor.next_copilot_suggestion(&Default::default(), cx);
    });

    executor.advance_clock(COPILOT_DEBOUNCE_TIMEOUT);
    assert!(copilot_requests.try_next().is_ok());
}

#[gpui::test]
async fn test_on_type_formatting_not_triggered(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/a",
        json!({
            "main.rs": "fn main() { let a = 5; }",
            "other.rs": "// Test file",
        }),
    )
    .await;
    let project = Project::test(fs, ["/a".as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(Arc::new(Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            brackets: BracketPairConfig {
                pairs: vec![BracketPair {
                    start: "{".to_string(),
                    end: "}".to_string(),
                    close: true,
                    newline: true,
                }],
                disabled_scopes_by_bracket_ix: Vec::new(),
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    )));
    let mut fake_servers = language_registry.register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                document_on_type_formatting_provider: Some(lsp::DocumentOnTypeFormattingOptions {
                    first_trigger_character: "{".to_string(),
                    more_trigger_character: None,
                }),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let workspace = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));

    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let worktree_id = workspace
        .update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees().next().unwrap().read(cx).id()
            })
        })
        .unwrap();

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/a/main.rs", cx)
        })
        .await
        .unwrap();
    cx.executor().run_until_parked();
    cx.executor().start_waiting();
    let fake_server = fake_servers.next().await.unwrap();
    let editor_handle = workspace
        .update(cx, |workspace, cx| {
            workspace.open_path((worktree_id, "main.rs"), None, true, cx)
        })
        .unwrap()
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    fake_server.handle_request::<lsp::request::OnTypeFormatting, _, _>(|params, _| async move {
        assert_eq!(
            params.text_document_position.text_document.uri,
            lsp::Url::from_file_path("/a/main.rs").unwrap(),
        );
        assert_eq!(
            params.text_document_position.position,
            lsp::Position::new(0, 21),
        );

        Ok(Some(vec![lsp::TextEdit {
            new_text: "]".to_string(),
            range: lsp::Range::new(lsp::Position::new(0, 22), lsp::Position::new(0, 22)),
        }]))
    });

    editor_handle.update(cx, |editor, cx| {
        editor.focus(cx);
        editor.change_selections(None, cx, |s| {
            s.select_ranges([Point::new(0, 21)..Point::new(0, 20)])
        });
        editor.handle_input("{", cx);
    });

    cx.executor().run_until_parked();

    _ = buffer.update(cx, |buffer, _| {
        assert_eq!(
            buffer.text(),
            "fn main() { let a = {5}; }",
            "No extra braces from on type formatting should appear in the buffer"
        )
    });
}

#[gpui::test]
async fn test_language_server_restart_due_to_settings_change(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/a",
        json!({
            "main.rs": "fn main() { let a = 5; }",
            "other.rs": "// Test file",
        }),
    )
    .await;

    let project = Project::test(fs, ["/a".as_ref()], cx).await;

    let server_restarts = Arc::new(AtomicUsize::new(0));
    let closure_restarts = Arc::clone(&server_restarts);
    let language_server_name = "test language server";
    let language_name: Arc<str> = "Rust".into();

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(Arc::new(Language::new(
        LanguageConfig {
            name: Arc::clone(&language_name),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    )));
    let mut fake_servers = language_registry.register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            name: language_server_name,
            initialization_options: Some(json!({
                "testOptionValue": true
            })),
            initializer: Some(Box::new(move |fake_server| {
                let task_restarts = Arc::clone(&closure_restarts);
                fake_server.handle_request::<lsp::request::Shutdown, _, _>(move |_, _| {
                    task_restarts.fetch_add(1, atomic::Ordering::Release);
                    futures::future::ready(Ok(()))
                });
            })),
            ..Default::default()
        },
    );

    let _window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
    let _buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/a/main.rs", cx)
        })
        .await
        .unwrap();
    let _fake_server = fake_servers.next().await.unwrap();
    update_test_language_settings(cx, |language_settings| {
        language_settings.languages.insert(
            Arc::clone(&language_name),
            LanguageSettingsContent {
                tab_size: NonZeroU32::new(8),
                ..Default::default()
            },
        );
    });
    cx.executor().run_until_parked();
    assert_eq!(
        server_restarts.load(atomic::Ordering::Acquire),
        0,
        "Should not restart LSP server on an unrelated change"
    );

    update_test_project_settings(cx, |project_settings| {
        project_settings.lsp.insert(
            "Some other server name".into(),
            LspSettings {
                settings: None,
                initialization_options: Some(json!({
                    "some other init value": false
                })),
            },
        );
    });
    cx.executor().run_until_parked();
    assert_eq!(
        server_restarts.load(atomic::Ordering::Acquire),
        0,
        "Should not restart LSP server on an unrelated LSP settings change"
    );

    update_test_project_settings(cx, |project_settings| {
        project_settings.lsp.insert(
            language_server_name.into(),
            LspSettings {
                settings: None,
                initialization_options: Some(json!({
                    "anotherInitValue": false
                })),
            },
        );
    });
    cx.executor().run_until_parked();
    assert_eq!(
        server_restarts.load(atomic::Ordering::Acquire),
        1,
        "Should restart LSP server on a related LSP settings change"
    );

    update_test_project_settings(cx, |project_settings| {
        project_settings.lsp.insert(
            language_server_name.into(),
            LspSettings {
                settings: None,
                initialization_options: Some(json!({
                    "anotherInitValue": false
                })),
            },
        );
    });
    cx.executor().run_until_parked();
    assert_eq!(
        server_restarts.load(atomic::Ordering::Acquire),
        1,
        "Should not restart LSP server on a related LSP settings change that is the same"
    );

    update_test_project_settings(cx, |project_settings| {
        project_settings.lsp.insert(
            language_server_name.into(),
            LspSettings {
                settings: None,
                initialization_options: None,
            },
        );
    });
    cx.executor().run_until_parked();
    assert_eq!(
        server_restarts.load(atomic::Ordering::Acquire),
        2,
        "Should restart LSP server on another related LSP settings change"
    );
}

#[gpui::test]
async fn test_completions_with_additional_edits(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            completion_provider: Some(lsp::CompletionOptions {
                trigger_characters: Some(vec![".".to_string()]),
                resolve_provider: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        },
        cx,
    )
    .await;

    cx.set_state(indoc! {"fn main() { let a = 2ˇ; }"});
    cx.simulate_keystroke(".");
    let completion_item = lsp::CompletionItem {
        label: "some".into(),
        kind: Some(lsp::CompletionItemKind::SNIPPET),
        detail: Some("Wrap the expression in an `Option::Some`".to_string()),
        documentation: Some(lsp::Documentation::MarkupContent(lsp::MarkupContent {
            kind: lsp::MarkupKind::Markdown,
            value: "```rust\nSome(2)\n```".to_string(),
        })),
        deprecated: Some(false),
        sort_text: Some("fffffff2".to_string()),
        filter_text: Some("some".to_string()),
        insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
            range: lsp::Range {
                start: lsp::Position {
                    line: 0,
                    character: 22,
                },
                end: lsp::Position {
                    line: 0,
                    character: 22,
                },
            },
            new_text: "Some(2)".to_string(),
        })),
        additional_text_edits: Some(vec![lsp::TextEdit {
            range: lsp::Range {
                start: lsp::Position {
                    line: 0,
                    character: 20,
                },
                end: lsp::Position {
                    line: 0,
                    character: 22,
                },
            },
            new_text: "".to_string(),
        }]),
        ..Default::default()
    };

    let closure_completion_item = completion_item.clone();
    let mut request = cx.handle_request::<lsp::request::Completion, _, _>(move |_, _, _| {
        let task_completion_item = closure_completion_item.clone();
        async move {
            Ok(Some(lsp::CompletionResponse::Array(vec![
                task_completion_item,
            ])))
        }
    });

    request.next().await;

    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    let apply_additional_edits = cx.update_editor(|editor, cx| {
        editor
            .confirm_completion(&ConfirmCompletion::default(), cx)
            .unwrap()
    });
    cx.assert_editor_state(indoc! {"fn main() { let a = 2.Some(2)ˇ; }"});

    cx.handle_request::<lsp::request::ResolveCompletionItem, _, _>(move |_, _, _| {
        let task_completion_item = completion_item.clone();
        async move { Ok(task_completion_item) }
    })
    .next()
    .await
    .unwrap();
    apply_additional_edits.await.unwrap();
    cx.assert_editor_state(indoc! {"fn main() { let a = Some(2)ˇ; }"});
}

#[gpui::test]
async fn test_completions_in_languages_with_extra_word_characters(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorLspTestContext::new(
        Language::new(
            LanguageConfig {
                matcher: LanguageMatcher {
                    path_suffixes: vec!["jsx".into()],
                    ..Default::default()
                },
                overrides: [(
                    "element".into(),
                    LanguageConfigOverride {
                        word_characters: Override::Set(['-'].into_iter().collect()),
                        ..Default::default()
                    },
                )]
                .into_iter()
                .collect(),
                ..Default::default()
            },
            Some(tree_sitter_typescript::language_tsx()),
        )
        .with_override_query("(jsx_self_closing_element) @element")
        .unwrap(),
        lsp::ServerCapabilities {
            completion_provider: Some(lsp::CompletionOptions {
                trigger_characters: Some(vec![":".to_string()]),
                ..Default::default()
            }),
            ..Default::default()
        },
        cx,
    )
    .await;

    cx.lsp
        .handle_request::<lsp::request::Completion, _, _>(move |_, _| async move {
            Ok(Some(lsp::CompletionResponse::Array(vec![
                lsp::CompletionItem {
                    label: "bg-blue".into(),
                    ..Default::default()
                },
                lsp::CompletionItem {
                    label: "bg-red".into(),
                    ..Default::default()
                },
                lsp::CompletionItem {
                    label: "bg-yellow".into(),
                    ..Default::default()
                },
            ])))
        });

    cx.set_state(r#"<p class="bgˇ" />"#);

    // Trigger completion when typing a dash, because the dash is an extra
    // word character in the 'element' scope, which contains the cursor.
    cx.simulate_keystroke("-");
    cx.executor().run_until_parked();
    cx.update_editor(|editor, _| {
        if let Some(ContextMenu::Completions(menu)) = editor.context_menu.read().as_ref() {
            assert_eq!(
                menu.matches.iter().map(|m| &m.string).collect::<Vec<_>>(),
                &["bg-red", "bg-blue", "bg-yellow"]
            );
        } else {
            panic!("expected completion menu to be open");
        }
    });

    cx.simulate_keystroke("l");
    cx.executor().run_until_parked();
    cx.update_editor(|editor, _| {
        if let Some(ContextMenu::Completions(menu)) = editor.context_menu.read().as_ref() {
            assert_eq!(
                menu.matches.iter().map(|m| &m.string).collect::<Vec<_>>(),
                &["bg-blue", "bg-yellow"]
            );
        } else {
            panic!("expected completion menu to be open");
        }
    });

    // When filtering completions, consider the character after the '-' to
    // be the start of a subword.
    cx.set_state(r#"<p class="yelˇ" />"#);
    cx.simulate_keystroke("l");
    cx.executor().run_until_parked();
    cx.update_editor(|editor, _| {
        if let Some(ContextMenu::Completions(menu)) = editor.context_menu.read().as_ref() {
            assert_eq!(
                menu.matches.iter().map(|m| &m.string).collect::<Vec<_>>(),
                &["bg-yellow"]
            );
        } else {
            panic!("expected completion menu to be open");
        }
    });
}

#[gpui::test]
async fn test_document_format_with_prettier(cx: &mut gpui::TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.formatter = Some(language_settings::Formatter::Prettier)
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_file("/file.rs", Default::default()).await;

    let project = Project::test(fs, ["/file.rs".as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());

    language_registry.add(Arc::new(Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            prettier_parser_name: Some("test_parser".to_string()),
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    )));

    let test_plugin = "test_plugin";
    let _ = language_registry.register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            prettier_plugins: vec![test_plugin],
            ..Default::default()
        },
    );

    let prettier_format_suffix = project::TEST_PRETTIER_FORMAT_SUFFIX;
    let buffer = project
        .update(cx, |project, cx| project.open_local_buffer("/file.rs", cx))
        .await
        .unwrap();

    let buffer_text = "one\ntwo\nthree\n";
    let buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|cx| build_editor(buffer, cx));
    _ = editor.update(cx, |editor, cx| editor.set_text(buffer_text, cx));

    editor
        .update(cx, |editor, cx| {
            editor.perform_format(project.clone(), FormatTrigger::Manual, cx)
        })
        .unwrap()
        .await;
    assert_eq!(
        editor.update(cx, |editor, cx| editor.text(cx)),
        buffer_text.to_string() + prettier_format_suffix,
        "Test prettier formatting was not applied to the original buffer text",
    );

    update_test_language_settings(cx, |settings| {
        settings.defaults.formatter = Some(language_settings::Formatter::Auto)
    });
    let format = editor.update(cx, |editor, cx| {
        editor.perform_format(project.clone(), FormatTrigger::Manual, cx)
    });
    format.await.unwrap();
    assert_eq!(
        editor.update(cx, |editor, cx| editor.text(cx)),
        buffer_text.to_string() + prettier_format_suffix + "\n" + prettier_format_suffix,
        "Autoformatting (via test prettier) was not applied to the original buffer text",
    );
}

#[gpui::test]
async fn test_find_all_references(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            document_formatting_provider: Some(lsp::OneOf::Left(true)),
            ..Default::default()
        },
        cx,
    )
    .await;

    cx.set_state(indoc! {"
        fn foo(«paramˇ»: i64) {
            println!(param);
        }
    "});

    cx.lsp
        .handle_request::<lsp::request::References, _, _>(move |_, _| async move {
            Ok(Some(vec![
                lsp::Location {
                    uri: lsp::Url::from_file_path("/root/dir/file.rs").unwrap(),
                    range: lsp::Range::new(lsp::Position::new(0, 7), lsp::Position::new(0, 12)),
                },
                lsp::Location {
                    uri: lsp::Url::from_file_path("/root/dir/file.rs").unwrap(),
                    range: lsp::Range::new(lsp::Position::new(1, 13), lsp::Position::new(1, 18)),
                },
            ]))
        });

    let references = cx
        .update_editor(|editor, cx| editor.find_all_references(&FindAllReferences, cx))
        .unwrap();

    cx.executor().run_until_parked();

    cx.executor().start_waiting();
    references.await.unwrap();

    cx.assert_editor_state(indoc! {"
        fn foo(param: i64) {
            println!(«paramˇ»);
        }
    "});

    let references = cx
        .update_editor(|editor, cx| editor.find_all_references(&FindAllReferences, cx))
        .unwrap();

    cx.executor().run_until_parked();

    cx.executor().start_waiting();
    references.await.unwrap();

    cx.assert_editor_state(indoc! {"
        fn foo(«paramˇ»: i64) {
            println!(param);
        }
    "});

    cx.set_state(indoc! {"
        fn foo(param: i64) {
            let a = param;
            let aˇ = param;
            let a = param;
            println!(param);
        }
    "});

    cx.lsp
        .handle_request::<lsp::request::References, _, _>(move |_, _| async move {
            Ok(Some(vec![lsp::Location {
                uri: lsp::Url::from_file_path("/root/dir/file.rs").unwrap(),
                range: lsp::Range::new(lsp::Position::new(2, 8), lsp::Position::new(2, 9)),
            }]))
        });

    let references = cx
        .update_editor(|editor, cx| editor.find_all_references(&FindAllReferences, cx))
        .unwrap();

    cx.executor().run_until_parked();

    cx.executor().start_waiting();
    references.await.unwrap();

    cx.assert_editor_state(indoc! {"
        fn foo(param: i64) {
            let a = param;
            let «aˇ» = param;
            let a = param;
            println!(param);
        }
    "});
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

/// Handle completion request passing a marked string specifying where the completion
/// should be triggered from using '|' character, what range should be replaced, and what completions
/// should be returned using '<' and '>' to delimit the range
pub fn handle_completion_request(
    cx: &mut EditorLspTestContext,
    marked_string: &str,
    completions: Vec<&'static str>,
) -> impl Future<Output = ()> {
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

    let mut request = cx.handle_request::<lsp::request::Completion, _, _>(move |url, params, _| {
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
    });

    async move {
        request.next().await;
    }
}

fn handle_resolve_completion_request(
    cx: &mut EditorLspTestContext,
    edits: Option<Vec<(&'static str, &'static str)>>,
) -> impl Future<Output = ()> {
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

    let mut request =
        cx.handle_request::<lsp::request::ResolveCompletionItem, _, _>(move |_, _, _| {
            let edits = edits.clone();
            async move {
                Ok(lsp::CompletionItem {
                    additional_text_edits: edits,
                    ..Default::default()
                })
            }
        });

    async move {
        request.next().await;
    }
}

fn handle_copilot_completion_request(
    lsp: &lsp::FakeLanguageServer,
    completions: Vec<copilot::request::Completion>,
    completions_cycling: Vec<copilot::request::Completion>,
) {
    lsp.handle_request::<copilot::request::GetCompletions, _, _>(move |_params, _cx| {
        let completions = completions.clone();
        async move {
            Ok(copilot::request::GetCompletionsResult {
                completions: completions.clone(),
            })
        }
    });
    lsp.handle_request::<copilot::request::GetCompletionsCycling, _, _>(move |_params, _cx| {
        let completions_cycling = completions_cycling.clone();
        async move {
            Ok(copilot::request::GetCompletionsResult {
                completions: completions_cycling.clone(),
            })
        }
    });
}

pub(crate) fn update_test_language_settings(
    cx: &mut TestAppContext,
    f: impl Fn(&mut AllLanguageSettingsContent),
) {
    _ = cx.update(|cx| {
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings::<AllLanguageSettings>(cx, f);
        });
    });
}

pub(crate) fn update_test_project_settings(
    cx: &mut TestAppContext,
    f: impl Fn(&mut ProjectSettings),
) {
    _ = cx.update(|cx| {
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings::<ProjectSettings>(cx, f);
        });
    });
}

pub(crate) fn init_test(cx: &mut TestAppContext, f: fn(&mut AllLanguageSettingsContent)) {
    _ = cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        theme::init(theme::LoadThemes::JustBase, cx);
        release_channel::init("0.0.0", cx);
        client::init_settings(cx);
        language::init(cx);
        Project::init_settings(cx);
        workspace::init_settings(cx);
        crate::init(cx);
    });

    update_test_language_settings(cx, f);
}

pub(crate) fn rust_lang() -> Arc<Language> {
    Arc::new(Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    ))
}
