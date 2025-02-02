use super::*;
use crate::{
    JoinLines,
    linked_editing_ranges::LinkedEditingRanges,
    scroll::scroll_amount::ScrollAmount,
    test::{
        assert_text_with_selections, build_editor,
        editor_lsp_test_context::{EditorLspTestContext, git_commit_lang},
        editor_test_context::EditorTestContext,
        select_ranges,
    },
};
use buffer_diff::{BufferDiff, DiffHunkSecondaryStatus, DiffHunkStatus, DiffHunkStatusKind};
use futures::StreamExt;
use gpui::{
    BackgroundExecutor, DismissEvent, SemanticVersion, TestAppContext, UpdateGlobal,
    VisualTestContext, WindowBounds, WindowOptions, div,
};
use indoc::indoc;
use language::{
    BracketPairConfig,
    Capability::ReadWrite,
    FakeLspAdapter, LanguageConfig, LanguageConfigOverride, LanguageMatcher, LanguageName,
    Override, Point,
    language_settings::{
        AllLanguageSettings, AllLanguageSettingsContent, CompletionSettings,
        LanguageSettingsContent, LspInsertMode, PrettierSettings,
    },
};
use language_settings::{Formatter, FormatterList, IndentGuideSettings};
use lsp::CompletionParams;
use multi_buffer::{IndentGuide, PathKey};
use parking_lot::Mutex;
use pretty_assertions::{assert_eq, assert_ne};
use project::{
    FakeFs,
    debugger::breakpoint_store::{BreakpointState, SourceBreakpoint},
    project_settings::{LspSettings, ProjectSettings},
};
use serde_json::{self, json};
use std::{cell::RefCell, future::Future, rc::Rc, sync::atomic::AtomicBool, time::Instant};
use std::{
    iter,
    sync::atomic::{self, AtomicUsize},
};
use test::{build_editor_with_project, editor_lsp_test_context::rust_lang};
use text::ToPoint as _;
use unindent::Unindent;
use util::{
    assert_set_eq, path,
    test::{TextRangeMarker, marked_text_ranges, marked_text_ranges_by, sample_text},
    uri,
};
use workspace::{
    CloseActiveItem, CloseAllItems, CloseInactiveItems, NavigationEntry, OpenOptions, ViewId,
    item::{FollowEvent, FollowableItem, Item, ItemHandle},
};

#[gpui::test]
fn test_edit_events(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let buffer = cx.new(|cx| {
        let mut buffer = language::Buffer::local("123456", cx);
        buffer.set_group_interval(Duration::from_secs(1));
        buffer
    });

    let events = Rc::new(RefCell::new(Vec::new()));
    let editor1 = cx.add_window({
        let events = events.clone();
        |window, cx| {
            let entity = cx.entity().clone();
            cx.subscribe_in(
                &entity,
                window,
                move |_, _, event: &EditorEvent, _, _| match event {
                    EditorEvent::Edited { .. } => events.borrow_mut().push(("editor1", "edited")),
                    EditorEvent::BufferEdited => {
                        events.borrow_mut().push(("editor1", "buffer edited"))
                    }
                    _ => {}
                },
            )
            .detach();
            Editor::for_buffer(buffer.clone(), None, window, cx)
        }
    });

    let editor2 = cx.add_window({
        let events = events.clone();
        |window, cx| {
            cx.subscribe_in(
                &cx.entity().clone(),
                window,
                move |_, _, event: &EditorEvent, _, _| match event {
                    EditorEvent::Edited { .. } => events.borrow_mut().push(("editor2", "edited")),
                    EditorEvent::BufferEdited => {
                        events.borrow_mut().push(("editor2", "buffer edited"))
                    }
                    _ => {}
                },
            )
            .detach();
            Editor::for_buffer(buffer.clone(), None, window, cx)
        }
    });

    assert_eq!(mem::take(&mut *events.borrow_mut()), []);

    // Mutating editor 1 will emit an `Edited` event only for that editor.
    _ = editor1.update(cx, |editor, window, cx| editor.insert("X", window, cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor1", "edited"),
            ("editor1", "buffer edited"),
            ("editor2", "buffer edited"),
        ]
    );

    // Mutating editor 2 will emit an `Edited` event only for that editor.
    _ = editor2.update(cx, |editor, window, cx| editor.delete(&Delete, window, cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor2", "edited"),
            ("editor1", "buffer edited"),
            ("editor2", "buffer edited"),
        ]
    );

    // Undoing on editor 1 will emit an `Edited` event only for that editor.
    _ = editor1.update(cx, |editor, window, cx| editor.undo(&Undo, window, cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor1", "edited"),
            ("editor1", "buffer edited"),
            ("editor2", "buffer edited"),
        ]
    );

    // Redoing on editor 1 will emit an `Edited` event only for that editor.
    _ = editor1.update(cx, |editor, window, cx| editor.redo(&Redo, window, cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor1", "edited"),
            ("editor1", "buffer edited"),
            ("editor2", "buffer edited"),
        ]
    );

    // Undoing on editor 2 will emit an `Edited` event only for that editor.
    _ = editor2.update(cx, |editor, window, cx| editor.undo(&Undo, window, cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor2", "edited"),
            ("editor1", "buffer edited"),
            ("editor2", "buffer edited"),
        ]
    );

    // Redoing on editor 2 will emit an `Edited` event only for that editor.
    _ = editor2.update(cx, |editor, window, cx| editor.redo(&Redo, window, cx));
    assert_eq!(
        mem::take(&mut *events.borrow_mut()),
        [
            ("editor2", "edited"),
            ("editor1", "buffer edited"),
            ("editor2", "buffer edited"),
        ]
    );

    // No event is emitted when the mutation is a no-op.
    _ = editor2.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| s.select_ranges([0..0]));

        editor.backspace(&Backspace, window, cx);
    });
    assert_eq!(mem::take(&mut *events.borrow_mut()), []);
}

#[gpui::test]
fn test_undo_redo_with_selection_restoration(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut now = Instant::now();
    let group_interval = Duration::from_millis(1);
    let buffer = cx.new(|cx| {
        let mut buf = language::Buffer::local("123456", cx);
        buf.set_group_interval(group_interval);
        buf
    });
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let editor = cx.add_window(|window, cx| build_editor(buffer.clone(), window, cx));

    _ = editor.update(cx, |editor, window, cx| {
        editor.start_transaction_at(now, window, cx);
        editor.change_selections(None, window, cx, |s| s.select_ranges([2..4]));

        editor.insert("cd", window, cx);
        editor.end_transaction_at(now, cx);
        assert_eq!(editor.text(cx), "12cd56");
        assert_eq!(editor.selections.ranges(cx), vec![4..4]);

        editor.start_transaction_at(now, window, cx);
        editor.change_selections(None, window, cx, |s| s.select_ranges([4..5]));
        editor.insert("e", window, cx);
        editor.end_transaction_at(now, cx);
        assert_eq!(editor.text(cx), "12cde6");
        assert_eq!(editor.selections.ranges(cx), vec![5..5]);

        now += group_interval + Duration::from_millis(1);
        editor.change_selections(None, window, cx, |s| s.select_ranges([2..2]));

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
        editor.undo(&Undo, window, cx);
        assert_eq!(editor.text(cx), "12cde6");
        assert_eq!(editor.selections.ranges(cx), vec![2..2]);

        // First two transactions happened within the group interval in this editor.
        // Undo them together and restore selections.
        editor.undo(&Undo, window, cx);
        editor.undo(&Undo, window, cx); // Undo stack is empty here, so this is a no-op.
        assert_eq!(editor.text(cx), "123456");
        assert_eq!(editor.selections.ranges(cx), vec![0..0]);

        // Redo the first two transactions together.
        editor.redo(&Redo, window, cx);
        assert_eq!(editor.text(cx), "12cde6");
        assert_eq!(editor.selections.ranges(cx), vec![5..5]);

        // Redo the last transaction on its own.
        editor.redo(&Redo, window, cx);
        assert_eq!(editor.text(cx), "ab2cde6");
        assert_eq!(editor.selections.ranges(cx), vec![6..6]);

        // Test empty transactions.
        editor.start_transaction_at(now, window, cx);
        editor.end_transaction_at(now, cx);
        editor.undo(&Undo, window, cx);
        assert_eq!(editor.text(cx), "12cde6");
    });
}

#[gpui::test]
fn test_ime_composition(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let buffer = cx.new(|cx| {
        let mut buffer = language::Buffer::local("abcde", cx);
        // Ensure automatic grouping doesn't occur.
        buffer.set_group_interval(Duration::ZERO);
        buffer
    });

    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    cx.add_window(|window, cx| {
        let mut editor = build_editor(buffer.clone(), window, cx);

        // Start a new IME composition.
        editor.replace_and_mark_text_in_range(Some(0..1), "à", None, window, cx);
        editor.replace_and_mark_text_in_range(Some(0..1), "á", None, window, cx);
        editor.replace_and_mark_text_in_range(Some(0..1), "ä", None, window, cx);
        assert_eq!(editor.text(cx), "äbcde");
        assert_eq!(
            editor.marked_text_ranges(cx),
            Some(vec![OffsetUtf16(0)..OffsetUtf16(1)])
        );

        // Finalize IME composition.
        editor.replace_text_in_range(None, "ā", window, cx);
        assert_eq!(editor.text(cx), "ābcde");
        assert_eq!(editor.marked_text_ranges(cx), None);

        // IME composition edits are grouped and are undone/redone at once.
        editor.undo(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "abcde");
        assert_eq!(editor.marked_text_ranges(cx), None);
        editor.redo(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "ābcde");
        assert_eq!(editor.marked_text_ranges(cx), None);

        // Start a new IME composition.
        editor.replace_and_mark_text_in_range(Some(0..1), "à", None, window, cx);
        assert_eq!(
            editor.marked_text_ranges(cx),
            Some(vec![OffsetUtf16(0)..OffsetUtf16(1)])
        );

        // Undoing during an IME composition cancels it.
        editor.undo(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "ābcde");
        assert_eq!(editor.marked_text_ranges(cx), None);

        // Start a new IME composition with an invalid marked range, ensuring it gets clipped.
        editor.replace_and_mark_text_in_range(Some(4..999), "è", None, window, cx);
        assert_eq!(editor.text(cx), "ābcdè");
        assert_eq!(
            editor.marked_text_ranges(cx),
            Some(vec![OffsetUtf16(4)..OffsetUtf16(5)])
        );

        // Finalize IME composition with an invalid replacement range, ensuring it gets clipped.
        editor.replace_text_in_range(Some(4..999), "ę", window, cx);
        assert_eq!(editor.text(cx), "ābcdę");
        assert_eq!(editor.marked_text_ranges(cx), None);

        // Start a new IME composition with multiple cursors.
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([
                OffsetUtf16(1)..OffsetUtf16(1),
                OffsetUtf16(3)..OffsetUtf16(3),
                OffsetUtf16(5)..OffsetUtf16(5),
            ])
        });
        editor.replace_and_mark_text_in_range(Some(4..5), "XYZ", None, window, cx);
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
        editor.replace_and_mark_text_in_range(Some(1..2), "1", None, window, cx);
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
        editor.replace_text_in_range(Some(9..10), "2", window, cx);
        assert_eq!(editor.text(cx), "X2ZbX2ZdX2Z");
        assert_eq!(editor.marked_text_ranges(cx), None);

        editor
    });
}

#[gpui::test]
fn test_selection_with_mouse(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\nddddddd\n", cx);
        build_editor(buffer, window, cx)
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.begin_selection(DisplayPoint::new(DisplayRow(2), 2), false, 1, window, cx);
    });
    assert_eq!(
        editor
            .update(cx, |editor, _, cx| editor.selections.display_ranges(cx))
            .unwrap(),
        [DisplayPoint::new(DisplayRow(2), 2)..DisplayPoint::new(DisplayRow(2), 2)]
    );

    _ = editor.update(cx, |editor, window, cx| {
        editor.update_selection(
            DisplayPoint::new(DisplayRow(3), 3),
            0,
            gpui::Point::<f32>::default(),
            window,
            cx,
        );
    });

    assert_eq!(
        editor
            .update(cx, |editor, _, cx| editor.selections.display_ranges(cx))
            .unwrap(),
        [DisplayPoint::new(DisplayRow(2), 2)..DisplayPoint::new(DisplayRow(3), 3)]
    );

    _ = editor.update(cx, |editor, window, cx| {
        editor.update_selection(
            DisplayPoint::new(DisplayRow(1), 1),
            0,
            gpui::Point::<f32>::default(),
            window,
            cx,
        );
    });

    assert_eq!(
        editor
            .update(cx, |editor, _, cx| editor.selections.display_ranges(cx))
            .unwrap(),
        [DisplayPoint::new(DisplayRow(2), 2)..DisplayPoint::new(DisplayRow(1), 1)]
    );

    _ = editor.update(cx, |editor, window, cx| {
        editor.end_selection(window, cx);
        editor.update_selection(
            DisplayPoint::new(DisplayRow(3), 3),
            0,
            gpui::Point::<f32>::default(),
            window,
            cx,
        );
    });

    assert_eq!(
        editor
            .update(cx, |editor, _, cx| editor.selections.display_ranges(cx))
            .unwrap(),
        [DisplayPoint::new(DisplayRow(2), 2)..DisplayPoint::new(DisplayRow(1), 1)]
    );

    _ = editor.update(cx, |editor, window, cx| {
        editor.begin_selection(DisplayPoint::new(DisplayRow(3), 3), true, 1, window, cx);
        editor.update_selection(
            DisplayPoint::new(DisplayRow(0), 0),
            0,
            gpui::Point::<f32>::default(),
            window,
            cx,
        );
    });

    assert_eq!(
        editor
            .update(cx, |editor, _, cx| editor.selections.display_ranges(cx))
            .unwrap(),
        [
            DisplayPoint::new(DisplayRow(2), 2)..DisplayPoint::new(DisplayRow(1), 1),
            DisplayPoint::new(DisplayRow(3), 3)..DisplayPoint::new(DisplayRow(0), 0)
        ]
    );

    _ = editor.update(cx, |editor, window, cx| {
        editor.end_selection(window, cx);
    });

    assert_eq!(
        editor
            .update(cx, |editor, _, cx| editor.selections.display_ranges(cx))
            .unwrap(),
        [DisplayPoint::new(DisplayRow(3), 3)..DisplayPoint::new(DisplayRow(0), 0)]
    );
}

#[gpui::test]
fn test_multiple_cursor_removal(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\nddddddd\n", cx);
        build_editor(buffer, window, cx)
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.begin_selection(DisplayPoint::new(DisplayRow(2), 1), false, 1, window, cx);
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.end_selection(window, cx);
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.begin_selection(DisplayPoint::new(DisplayRow(3), 2), true, 1, window, cx);
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.end_selection(window, cx);
    });

    assert_eq!(
        editor
            .update(cx, |editor, _, cx| editor.selections.display_ranges(cx))
            .unwrap(),
        [
            DisplayPoint::new(DisplayRow(2), 1)..DisplayPoint::new(DisplayRow(2), 1),
            DisplayPoint::new(DisplayRow(3), 2)..DisplayPoint::new(DisplayRow(3), 2)
        ]
    );

    _ = editor.update(cx, |editor, window, cx| {
        editor.begin_selection(DisplayPoint::new(DisplayRow(2), 1), true, 1, window, cx);
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.end_selection(window, cx);
    });

    assert_eq!(
        editor
            .update(cx, |editor, _, cx| editor.selections.display_ranges(cx))
            .unwrap(),
        [DisplayPoint::new(DisplayRow(3), 2)..DisplayPoint::new(DisplayRow(3), 2)]
    );
}

#[gpui::test]
fn test_canceling_pending_selection(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx);
        build_editor(buffer, window, cx)
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.begin_selection(DisplayPoint::new(DisplayRow(2), 2), false, 1, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            [DisplayPoint::new(DisplayRow(2), 2)..DisplayPoint::new(DisplayRow(2), 2)]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.update_selection(
            DisplayPoint::new(DisplayRow(3), 3),
            0,
            gpui::Point::<f32>::default(),
            window,
            cx,
        );
        assert_eq!(
            editor.selections.display_ranges(cx),
            [DisplayPoint::new(DisplayRow(2), 2)..DisplayPoint::new(DisplayRow(3), 3)]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.cancel(&Cancel, window, cx);
        editor.update_selection(
            DisplayPoint::new(DisplayRow(1), 1),
            0,
            gpui::Point::<f32>::default(),
            window,
            cx,
        );
        assert_eq!(
            editor.selections.display_ranges(cx),
            [DisplayPoint::new(DisplayRow(2), 2)..DisplayPoint::new(DisplayRow(3), 3)]
        );
    });
}

#[gpui::test]
fn test_movement_actions_with_pending_selection(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx);
        build_editor(buffer, window, cx)
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.begin_selection(DisplayPoint::new(DisplayRow(2), 2), false, 1, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            [DisplayPoint::new(DisplayRow(2), 2)..DisplayPoint::new(DisplayRow(2), 2)]
        );

        editor.move_down(&Default::default(), window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            [DisplayPoint::new(DisplayRow(3), 2)..DisplayPoint::new(DisplayRow(3), 2)]
        );

        editor.begin_selection(DisplayPoint::new(DisplayRow(2), 2), false, 1, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            [DisplayPoint::new(DisplayRow(2), 2)..DisplayPoint::new(DisplayRow(2), 2)]
        );

        editor.move_up(&Default::default(), window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            [DisplayPoint::new(DisplayRow(1), 2)..DisplayPoint::new(DisplayRow(1), 2)]
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

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple(&text, cx);
        build_editor(buffer, window, cx)
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges(selection_ranges.clone())
        });
        editor.fold_creases(
            vec![
                Crease::simple(Point::new(1, 0)..Point::new(2, 0), FoldPlaceholder::test()),
                Crease::simple(Point::new(3, 0)..Point::new(4, 0), FoldPlaceholder::test()),
            ],
            true,
            window,
            cx,
        );
    });

    let cloned_editor = editor
        .update(cx, |editor, _, cx| {
            cx.open_window(Default::default(), |window, cx| {
                cx.new(|cx| editor.clone(window, cx))
            })
        })
        .unwrap()
        .unwrap();

    let snapshot = editor
        .update(cx, |e, window, cx| e.snapshot(window, cx))
        .unwrap();
    let cloned_snapshot = cloned_editor
        .update(cx, |e, window, cx| e.snapshot(window, cx))
        .unwrap();

    assert_eq!(
        cloned_editor
            .update(cx, |e, _, cx| e.display_text(cx))
            .unwrap(),
        editor.update(cx, |e, _, cx| e.display_text(cx)).unwrap()
    );
    assert_eq!(
        cloned_snapshot
            .folds_in_range(0..text.len())
            .collect::<Vec<_>>(),
        snapshot.folds_in_range(0..text.len()).collect::<Vec<_>>(),
    );
    assert_set_eq!(
        cloned_editor
            .update(cx, |editor, _, cx| editor.selections.ranges::<Point>(cx))
            .unwrap(),
        editor
            .update(cx, |editor, _, cx| editor.selections.ranges(cx))
            .unwrap()
    );
    assert_set_eq!(
        cloned_editor
            .update(cx, |e, _window, cx| e.selections.display_ranges(cx))
            .unwrap(),
        editor
            .update(cx, |e, _, cx| e.selections.display_ranges(cx))
            .unwrap()
    );
}

#[gpui::test]
async fn test_navigation_history(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    use workspace::item::Item;

    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, [], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project, window, cx));
    let pane = workspace
        .update(cx, |workspace, _, _| workspace.active_pane().clone())
        .unwrap();

    _ = workspace.update(cx, |_v, window, cx| {
        cx.new(|cx| {
            let buffer = MultiBuffer::build_simple(&sample_text(300, 5, 'a'), cx);
            let mut editor = build_editor(buffer.clone(), window, cx);
            let handle = cx.entity();
            editor.set_nav_history(Some(pane.read(cx).nav_history_for_item(&handle)));

            fn pop_history(editor: &mut Editor, cx: &mut App) -> Option<NavigationEntry> {
                editor.nav_history.as_mut().unwrap().pop_backward(cx)
            }

            // Move the cursor a small distance.
            // Nothing is added to the navigation history.
            editor.change_selections(None, window, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 0)
                ])
            });
            editor.change_selections(None, window, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(DisplayRow(3), 0)..DisplayPoint::new(DisplayRow(3), 0)
                ])
            });
            assert!(pop_history(&mut editor, cx).is_none());

            // Move the cursor a large distance.
            // The history can jump back to the previous position.
            editor.change_selections(None, window, cx, |s| {
                s.select_display_ranges([
                    DisplayPoint::new(DisplayRow(13), 0)..DisplayPoint::new(DisplayRow(13), 3)
                ])
            });
            let nav_entry = pop_history(&mut editor, cx).unwrap();
            editor.navigate(nav_entry.data.unwrap(), window, cx);
            assert_eq!(nav_entry.item.id(), cx.entity_id());
            assert_eq!(
                editor.selections.display_ranges(cx),
                &[DisplayPoint::new(DisplayRow(3), 0)..DisplayPoint::new(DisplayRow(3), 0)]
            );
            assert!(pop_history(&mut editor, cx).is_none());

            // Move the cursor a small distance via the mouse.
            // Nothing is added to the navigation history.
            editor.begin_selection(DisplayPoint::new(DisplayRow(5), 0), false, 1, window, cx);
            editor.end_selection(window, cx);
            assert_eq!(
                editor.selections.display_ranges(cx),
                &[DisplayPoint::new(DisplayRow(5), 0)..DisplayPoint::new(DisplayRow(5), 0)]
            );
            assert!(pop_history(&mut editor, cx).is_none());

            // Move the cursor a large distance via the mouse.
            // The history can jump back to the previous position.
            editor.begin_selection(DisplayPoint::new(DisplayRow(15), 0), false, 1, window, cx);
            editor.end_selection(window, cx);
            assert_eq!(
                editor.selections.display_ranges(cx),
                &[DisplayPoint::new(DisplayRow(15), 0)..DisplayPoint::new(DisplayRow(15), 0)]
            );
            let nav_entry = pop_history(&mut editor, cx).unwrap();
            editor.navigate(nav_entry.data.unwrap(), window, cx);
            assert_eq!(nav_entry.item.id(), cx.entity_id());
            assert_eq!(
                editor.selections.display_ranges(cx),
                &[DisplayPoint::new(DisplayRow(5), 0)..DisplayPoint::new(DisplayRow(5), 0)]
            );
            assert!(pop_history(&mut editor, cx).is_none());

            // Set scroll position to check later
            editor.set_scroll_position(gpui::Point::<f32>::new(5.5, 5.5), window, cx);
            let original_scroll_position = editor.scroll_manager.anchor();

            // Jump to the end of the document and adjust scroll
            editor.move_to_end(&MoveToEnd, window, cx);
            editor.set_scroll_position(gpui::Point::<f32>::new(-2.5, -0.5), window, cx);
            assert_ne!(editor.scroll_manager.anchor(), original_scroll_position);

            let nav_entry = pop_history(&mut editor, cx).unwrap();
            editor.navigate(nav_entry.data.unwrap(), window, cx);
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
                window,
                cx,
            );
            assert_eq!(
                editor.selections.display_ranges(cx),
                &[editor.max_point(cx)..editor.max_point(cx)]
            );
            assert_eq!(
                editor.scroll_position(cx),
                gpui::Point::new(0., editor.max_point(cx).row().as_f32())
            );

            editor
        })
    });
}

#[gpui::test]
fn test_cancel(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\ndddddd\n", cx);
        build_editor(buffer, window, cx)
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.begin_selection(DisplayPoint::new(DisplayRow(3), 4), false, 1, window, cx);
        editor.update_selection(
            DisplayPoint::new(DisplayRow(1), 1),
            0,
            gpui::Point::<f32>::default(),
            window,
            cx,
        );
        editor.end_selection(window, cx);

        editor.begin_selection(DisplayPoint::new(DisplayRow(0), 1), true, 1, window, cx);
        editor.update_selection(
            DisplayPoint::new(DisplayRow(0), 3),
            0,
            gpui::Point::<f32>::default(),
            window,
            cx,
        );
        editor.end_selection(window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            [
                DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(0), 3),
                DisplayPoint::new(DisplayRow(3), 4)..DisplayPoint::new(DisplayRow(1), 1),
            ]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.cancel(&Cancel, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            [DisplayPoint::new(DisplayRow(3), 4)..DisplayPoint::new(DisplayRow(1), 1)]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.cancel(&Cancel, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            [DisplayPoint::new(DisplayRow(1), 1)..DisplayPoint::new(DisplayRow(1), 1)]
        );
    });
}

#[gpui::test]
fn test_fold_action(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
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
        build_editor(buffer.clone(), window, cx)
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(7), 0)..DisplayPoint::new(DisplayRow(12), 0)
            ]);
        });
        editor.fold(&Fold, window, cx);
        assert_eq!(
            editor.display_text(cx),
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

        editor.fold(&Fold, window, cx);
        assert_eq!(
            editor.display_text(cx),
            "
                impl Foo {⋯
                }
            "
            .unindent(),
        );

        editor.unfold_lines(&UnfoldLines, window, cx);
        assert_eq!(
            editor.display_text(cx),
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

        editor.unfold_lines(&UnfoldLines, window, cx);
        assert_eq!(
            editor.display_text(cx),
            editor.buffer.read(cx).read(cx).text()
        );
    });
}

#[gpui::test]
fn test_fold_action_whitespace_sensitive_language(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple(
            &"
                class Foo:
                    # Hello!

                    def a():
                        print(1)

                    def b():
                        print(2)

                    def c():
                        print(3)
            "
            .unindent(),
            cx,
        );
        build_editor(buffer.clone(), window, cx)
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(6), 0)..DisplayPoint::new(DisplayRow(10), 0)
            ]);
        });
        editor.fold(&Fold, window, cx);
        assert_eq!(
            editor.display_text(cx),
            "
                class Foo:
                    # Hello!

                    def a():
                        print(1)

                    def b():⋯

                    def c():⋯
            "
            .unindent(),
        );

        editor.fold(&Fold, window, cx);
        assert_eq!(
            editor.display_text(cx),
            "
                class Foo:⋯
            "
            .unindent(),
        );

        editor.unfold_lines(&UnfoldLines, window, cx);
        assert_eq!(
            editor.display_text(cx),
            "
                class Foo:
                    # Hello!

                    def a():
                        print(1)

                    def b():⋯

                    def c():⋯
            "
            .unindent(),
        );

        editor.unfold_lines(&UnfoldLines, window, cx);
        assert_eq!(
            editor.display_text(cx),
            editor.buffer.read(cx).read(cx).text()
        );
    });
}

#[gpui::test]
fn test_fold_action_multiple_line_breaks(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple(
            &"
                class Foo:
                    # Hello!

                    def a():
                        print(1)

                    def b():
                        print(2)


                    def c():
                        print(3)


            "
            .unindent(),
            cx,
        );
        build_editor(buffer.clone(), window, cx)
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(6), 0)..DisplayPoint::new(DisplayRow(11), 0)
            ]);
        });
        editor.fold(&Fold, window, cx);
        assert_eq!(
            editor.display_text(cx),
            "
                class Foo:
                    # Hello!

                    def a():
                        print(1)

                    def b():⋯


                    def c():⋯


            "
            .unindent(),
        );

        editor.fold(&Fold, window, cx);
        assert_eq!(
            editor.display_text(cx),
            "
                class Foo:⋯


            "
            .unindent(),
        );

        editor.unfold_lines(&UnfoldLines, window, cx);
        assert_eq!(
            editor.display_text(cx),
            "
                class Foo:
                    # Hello!

                    def a():
                        print(1)

                    def b():⋯


                    def c():⋯


            "
            .unindent(),
        );

        editor.unfold_lines(&UnfoldLines, window, cx);
        assert_eq!(
            editor.display_text(cx),
            editor.buffer.read(cx).read(cx).text()
        );
    });
}

#[gpui::test]
fn test_fold_at_level(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple(
            &"
                class Foo:
                    # Hello!

                    def a():
                        print(1)

                    def b():
                        print(2)


                class Bar:
                    # World!

                    def a():
                        print(1)

                    def b():
                        print(2)


            "
            .unindent(),
            cx,
        );
        build_editor(buffer.clone(), window, cx)
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.fold_at_level(&FoldAtLevel(2), window, cx);
        assert_eq!(
            editor.display_text(cx),
            "
                class Foo:
                    # Hello!

                    def a():⋯

                    def b():⋯


                class Bar:
                    # World!

                    def a():⋯

                    def b():⋯


            "
            .unindent(),
        );

        editor.fold_at_level(&FoldAtLevel(1), window, cx);
        assert_eq!(
            editor.display_text(cx),
            "
                class Foo:⋯


                class Bar:⋯


            "
            .unindent(),
        );

        editor.unfold_all(&UnfoldAll, window, cx);
        editor.fold_at_level(&FoldAtLevel(0), window, cx);
        assert_eq!(
            editor.display_text(cx),
            "
                class Foo:
                    # Hello!

                    def a():
                        print(1)

                    def b():
                        print(2)


                class Bar:
                    # World!

                    def a():
                        print(1)

                    def b():
                        print(2)


            "
            .unindent(),
        );

        assert_eq!(
            editor.display_text(cx),
            editor.buffer.read(cx).read(cx).text()
        );
    });
}

#[gpui::test]
fn test_move_cursor(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let buffer = cx.update(|cx| MultiBuffer::build_simple(&sample_text(6, 6, 'a'), cx));
    let editor = cx.add_window(|window, cx| build_editor(buffer.clone(), window, cx));

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
    _ = editor.update(cx, |editor, window, cx| {
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 0)]
        );

        editor.move_down(&MoveDown, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 0)]
        );

        editor.move_right(&MoveRight, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(DisplayRow(1), 4)..DisplayPoint::new(DisplayRow(1), 4)]
        );

        editor.move_left(&MoveLeft, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 0)]
        );

        editor.move_up(&MoveUp, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 0)]
        );

        editor.move_to_end(&MoveToEnd, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(DisplayRow(5), 6)..DisplayPoint::new(DisplayRow(5), 6)]
        );

        editor.move_to_beginning(&MoveToBeginning, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 0)]
        );

        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(0), 2)
            ]);
        });
        editor.select_to_beginning(&SelectToBeginning, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(0), 0)]
        );

        editor.select_to_end(&SelectToEnd, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(5), 6)]
        );
    });
}

#[gpui::test]
fn test_move_cursor_multibyte(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("🟥🟧🟨🟩🟦🟪\nabcde\nαβγδε", cx);
        build_editor(buffer.clone(), window, cx)
    });

    assert_eq!('🟥'.len_utf8(), 4);
    assert_eq!('α'.len_utf8(), 2);

    _ = editor.update(cx, |editor, window, cx| {
        editor.fold_creases(
            vec![
                Crease::simple(Point::new(0, 8)..Point::new(0, 16), FoldPlaceholder::test()),
                Crease::simple(Point::new(1, 2)..Point::new(1, 4), FoldPlaceholder::test()),
                Crease::simple(Point::new(2, 4)..Point::new(2, 8), FoldPlaceholder::test()),
            ],
            true,
            window,
            cx,
        );
        assert_eq!(editor.display_text(cx), "🟥🟧⋯🟦🟪\nab⋯e\nαβ⋯ε");

        editor.move_right(&MoveRight, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(0, "🟥".len())]
        );
        editor.move_right(&MoveRight, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(0, "🟥🟧".len())]
        );
        editor.move_right(&MoveRight, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(0, "🟥🟧⋯".len())]
        );

        editor.move_down(&MoveDown, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(1, "ab⋯e".len())]
        );
        editor.move_left(&MoveLeft, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(1, "ab⋯".len())]
        );
        editor.move_left(&MoveLeft, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(1, "ab".len())]
        );
        editor.move_left(&MoveLeft, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(1, "a".len())]
        );

        editor.move_down(&MoveDown, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(2, "α".len())]
        );
        editor.move_right(&MoveRight, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(2, "αβ".len())]
        );
        editor.move_right(&MoveRight, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(2, "αβ⋯".len())]
        );
        editor.move_right(&MoveRight, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(2, "αβ⋯ε".len())]
        );

        editor.move_up(&MoveUp, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(1, "ab⋯e".len())]
        );
        editor.move_down(&MoveDown, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(2, "αβ⋯ε".len())]
        );
        editor.move_up(&MoveUp, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(1, "ab⋯e".len())]
        );

        editor.move_up(&MoveUp, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(0, "🟥🟧".len())]
        );
        editor.move_left(&MoveLeft, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(0, "🟥".len())]
        );
        editor.move_left(&MoveLeft, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(0, "".len())]
        );
    });
}

#[gpui::test]
fn test_move_cursor_different_line_lengths(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("ⓐⓑⓒⓓⓔ\nabcd\nαβγ\nabcd\nⓐⓑⓒⓓⓔ\n", cx);
        build_editor(buffer.clone(), window, cx)
    });
    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([empty_range(0, "ⓐⓑⓒⓓⓔ".len())]);
        });

        // moving above start of document should move selection to start of document,
        // but the next move down should still be at the original goal_x
        editor.move_up(&MoveUp, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(0, "".len())]
        );

        editor.move_down(&MoveDown, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(1, "abcd".len())]
        );

        editor.move_down(&MoveDown, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(2, "αβγ".len())]
        );

        editor.move_down(&MoveDown, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(3, "abcd".len())]
        );

        editor.move_down(&MoveDown, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(4, "ⓐⓑⓒⓓⓔ".len())]
        );

        // moving past end of document should not change goal_x
        editor.move_down(&MoveDown, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(5, "".len())]
        );

        editor.move_down(&MoveDown, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(5, "".len())]
        );

        editor.move_up(&MoveUp, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(4, "ⓐⓑⓒⓓⓔ".len())]
        );

        editor.move_up(&MoveUp, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(3, "abcd".len())]
        );

        editor.move_up(&MoveUp, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[empty_range(2, "αβγ".len())]
        );
    });
}

#[gpui::test]
fn test_beginning_end_of_line(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let move_to_beg = MoveToBeginningOfLine {
        stop_at_soft_wraps: true,
        stop_at_indent: true,
    };

    let delete_to_beg = DeleteToBeginningOfLine {
        stop_at_indent: false,
    };

    let move_to_end = MoveToEndOfLine {
        stop_at_soft_wraps: true,
    };

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("abc\n  def", cx);
        build_editor(buffer, window, cx)
    });
    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(0), 1),
                DisplayPoint::new(DisplayRow(1), 4)..DisplayPoint::new(DisplayRow(1), 4),
            ]);
        });
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.move_to_beginning_of_line(&move_to_beg, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 0),
                DisplayPoint::new(DisplayRow(1), 2)..DisplayPoint::new(DisplayRow(1), 2),
            ]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.move_to_beginning_of_line(&move_to_beg, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 0),
                DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 0),
            ]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.move_to_beginning_of_line(&move_to_beg, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 0),
                DisplayPoint::new(DisplayRow(1), 2)..DisplayPoint::new(DisplayRow(1), 2),
            ]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.move_to_end_of_line(&move_to_end, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[
                DisplayPoint::new(DisplayRow(0), 3)..DisplayPoint::new(DisplayRow(0), 3),
                DisplayPoint::new(DisplayRow(1), 5)..DisplayPoint::new(DisplayRow(1), 5),
            ]
        );
    });

    // Moving to the end of line again is a no-op.
    _ = editor.update(cx, |editor, window, cx| {
        editor.move_to_end_of_line(&move_to_end, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[
                DisplayPoint::new(DisplayRow(0), 3)..DisplayPoint::new(DisplayRow(0), 3),
                DisplayPoint::new(DisplayRow(1), 5)..DisplayPoint::new(DisplayRow(1), 5),
            ]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.move_left(&MoveLeft, window, cx);
        editor.select_to_beginning_of_line(
            &SelectToBeginningOfLine {
                stop_at_soft_wraps: true,
                stop_at_indent: true,
            },
            window,
            cx,
        );
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[
                DisplayPoint::new(DisplayRow(0), 2)..DisplayPoint::new(DisplayRow(0), 0),
                DisplayPoint::new(DisplayRow(1), 4)..DisplayPoint::new(DisplayRow(1), 2),
            ]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.select_to_beginning_of_line(
            &SelectToBeginningOfLine {
                stop_at_soft_wraps: true,
                stop_at_indent: true,
            },
            window,
            cx,
        );
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[
                DisplayPoint::new(DisplayRow(0), 2)..DisplayPoint::new(DisplayRow(0), 0),
                DisplayPoint::new(DisplayRow(1), 4)..DisplayPoint::new(DisplayRow(1), 0),
            ]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.select_to_beginning_of_line(
            &SelectToBeginningOfLine {
                stop_at_soft_wraps: true,
                stop_at_indent: true,
            },
            window,
            cx,
        );
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[
                DisplayPoint::new(DisplayRow(0), 2)..DisplayPoint::new(DisplayRow(0), 0),
                DisplayPoint::new(DisplayRow(1), 4)..DisplayPoint::new(DisplayRow(1), 2),
            ]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.select_to_end_of_line(
            &SelectToEndOfLine {
                stop_at_soft_wraps: true,
            },
            window,
            cx,
        );
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[
                DisplayPoint::new(DisplayRow(0), 2)..DisplayPoint::new(DisplayRow(0), 3),
                DisplayPoint::new(DisplayRow(1), 4)..DisplayPoint::new(DisplayRow(1), 5),
            ]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.delete_to_end_of_line(&DeleteToEndOfLine, window, cx);
        assert_eq!(editor.display_text(cx), "ab\n  de");
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[
                DisplayPoint::new(DisplayRow(0), 2)..DisplayPoint::new(DisplayRow(0), 2),
                DisplayPoint::new(DisplayRow(1), 4)..DisplayPoint::new(DisplayRow(1), 4),
            ]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.delete_to_beginning_of_line(&delete_to_beg, window, cx);
        assert_eq!(editor.display_text(cx), "\n");
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 0),
                DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 0),
            ]
        );
    });
}

#[gpui::test]
fn test_beginning_end_of_line_ignore_soft_wrap(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let move_to_beg = MoveToBeginningOfLine {
        stop_at_soft_wraps: false,
        stop_at_indent: false,
    };

    let move_to_end = MoveToEndOfLine {
        stop_at_soft_wraps: false,
    };

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("thequickbrownfox\njumpedoverthelazydogs", cx);
        build_editor(buffer, window, cx)
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.set_wrap_width(Some(140.0.into()), cx);

        // We expect the following lines after wrapping
        // ```
        // thequickbrownfox
        // jumpedoverthelazydo
        // gs
        // ```
        // The final `gs` was soft-wrapped onto a new line.
        assert_eq!(
            "thequickbrownfox\njumpedoverthelaz\nydogs",
            editor.display_text(cx),
        );

        // First, let's assert behavior on the first line, that was not soft-wrapped.
        // Start the cursor at the `k` on the first line
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 7)..DisplayPoint::new(DisplayRow(0), 7)
            ]);
        });

        // Moving to the beginning of the line should put us at the beginning of the line.
        editor.move_to_beginning_of_line(&move_to_beg, window, cx);
        assert_eq!(
            vec![DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 0),],
            editor.selections.display_ranges(cx)
        );

        // Moving to the end of the line should put us at the end of the line.
        editor.move_to_end_of_line(&move_to_end, window, cx);
        assert_eq!(
            vec![DisplayPoint::new(DisplayRow(0), 16)..DisplayPoint::new(DisplayRow(0), 16),],
            editor.selections.display_ranges(cx)
        );

        // Now, let's assert behavior on the second line, that ended up being soft-wrapped.
        // Start the cursor at the last line (`y` that was wrapped to a new line)
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(2), 0)..DisplayPoint::new(DisplayRow(2), 0)
            ]);
        });

        // Moving to the beginning of the line should put us at the start of the second line of
        // display text, i.e., the `j`.
        editor.move_to_beginning_of_line(&move_to_beg, window, cx);
        assert_eq!(
            vec![DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 0),],
            editor.selections.display_ranges(cx)
        );

        // Moving to the beginning of the line again should be a no-op.
        editor.move_to_beginning_of_line(&move_to_beg, window, cx);
        assert_eq!(
            vec![DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 0),],
            editor.selections.display_ranges(cx)
        );

        // Moving to the end of the line should put us right after the `s` that was soft-wrapped to the
        // next display line.
        editor.move_to_end_of_line(&move_to_end, window, cx);
        assert_eq!(
            vec![DisplayPoint::new(DisplayRow(2), 5)..DisplayPoint::new(DisplayRow(2), 5),],
            editor.selections.display_ranges(cx)
        );

        // Moving to the end of the line again should be a no-op.
        editor.move_to_end_of_line(&move_to_end, window, cx);
        assert_eq!(
            vec![DisplayPoint::new(DisplayRow(2), 5)..DisplayPoint::new(DisplayRow(2), 5),],
            editor.selections.display_ranges(cx)
        );
    });
}

#[gpui::test]
fn test_beginning_of_line_stop_at_indent(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let move_to_beg = MoveToBeginningOfLine {
        stop_at_soft_wraps: true,
        stop_at_indent: true,
    };

    let select_to_beg = SelectToBeginningOfLine {
        stop_at_soft_wraps: true,
        stop_at_indent: true,
    };

    let delete_to_beg = DeleteToBeginningOfLine {
        stop_at_indent: true,
    };

    let move_to_end = MoveToEndOfLine {
        stop_at_soft_wraps: false,
    };

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("abc\n  def", cx);
        build_editor(buffer, window, cx)
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(0), 1),
                DisplayPoint::new(DisplayRow(1), 4)..DisplayPoint::new(DisplayRow(1), 4),
            ]);
        });

        // Moving to the beginning of the line should put the first cursor at the beginning of the line,
        // and the second cursor at the first non-whitespace character in the line.
        editor.move_to_beginning_of_line(&move_to_beg, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 0),
                DisplayPoint::new(DisplayRow(1), 2)..DisplayPoint::new(DisplayRow(1), 2),
            ]
        );

        // Moving to the beginning of the line again should be a no-op for the first cursor,
        // and should move the second cursor to the beginning of the line.
        editor.move_to_beginning_of_line(&move_to_beg, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 0),
                DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 0),
            ]
        );

        // Moving to the beginning of the line again should still be a no-op for the first cursor,
        // and should move the second cursor back to the first non-whitespace character in the line.
        editor.move_to_beginning_of_line(&move_to_beg, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 0),
                DisplayPoint::new(DisplayRow(1), 2)..DisplayPoint::new(DisplayRow(1), 2),
            ]
        );

        // Selecting to the beginning of the line should select to the beginning of the line for the first cursor,
        // and to the first non-whitespace character in the line for the second cursor.
        editor.move_to_end_of_line(&move_to_end, window, cx);
        editor.move_left(&MoveLeft, window, cx);
        editor.select_to_beginning_of_line(&select_to_beg, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[
                DisplayPoint::new(DisplayRow(0), 2)..DisplayPoint::new(DisplayRow(0), 0),
                DisplayPoint::new(DisplayRow(1), 4)..DisplayPoint::new(DisplayRow(1), 2),
            ]
        );

        // Selecting to the beginning of the line again should be a no-op for the first cursor,
        // and should select to the beginning of the line for the second cursor.
        editor.select_to_beginning_of_line(&select_to_beg, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[
                DisplayPoint::new(DisplayRow(0), 2)..DisplayPoint::new(DisplayRow(0), 0),
                DisplayPoint::new(DisplayRow(1), 4)..DisplayPoint::new(DisplayRow(1), 0),
            ]
        );

        // Deleting to the beginning of the line should delete to the beginning of the line for the first cursor,
        // and should delete to the first non-whitespace character in the line for the second cursor.
        editor.move_to_end_of_line(&move_to_end, window, cx);
        editor.move_left(&MoveLeft, window, cx);
        editor.delete_to_beginning_of_line(&delete_to_beg, window, cx);
        assert_eq!(editor.text(cx), "c\n  f");
    });
}

#[gpui::test]
fn test_prev_next_word_boundary(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("use std::str::{foo, bar}\n\n  {baz.qux()}", cx);
        build_editor(buffer, window, cx)
    });
    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 11)..DisplayPoint::new(DisplayRow(0), 11),
                DisplayPoint::new(DisplayRow(2), 4)..DisplayPoint::new(DisplayRow(2), 4),
            ])
        });

        editor.move_to_previous_word_start(&MoveToPreviousWordStart, window, cx);
        assert_selection_ranges("use std::ˇstr::{foo, bar}\n\n  {ˇbaz.qux()}", editor, cx);

        editor.move_to_previous_word_start(&MoveToPreviousWordStart, window, cx);
        assert_selection_ranges("use stdˇ::str::{foo, bar}\n\n  ˇ{baz.qux()}", editor, cx);

        editor.move_to_previous_word_start(&MoveToPreviousWordStart, window, cx);
        assert_selection_ranges("use ˇstd::str::{foo, bar}\n\nˇ  {baz.qux()}", editor, cx);

        editor.move_to_previous_word_start(&MoveToPreviousWordStart, window, cx);
        assert_selection_ranges("ˇuse std::str::{foo, bar}\nˇ\n  {baz.qux()}", editor, cx);

        editor.move_to_previous_word_start(&MoveToPreviousWordStart, window, cx);
        assert_selection_ranges("ˇuse std::str::{foo, barˇ}\n\n  {baz.qux()}", editor, cx);

        editor.move_to_next_word_end(&MoveToNextWordEnd, window, cx);
        assert_selection_ranges("useˇ std::str::{foo, bar}ˇ\n\n  {baz.qux()}", editor, cx);

        editor.move_to_next_word_end(&MoveToNextWordEnd, window, cx);
        assert_selection_ranges("use stdˇ::str::{foo, bar}\nˇ\n  {baz.qux()}", editor, cx);

        editor.move_to_next_word_end(&MoveToNextWordEnd, window, cx);
        assert_selection_ranges("use std::ˇstr::{foo, bar}\n\n  {ˇbaz.qux()}", editor, cx);

        editor.move_right(&MoveRight, window, cx);
        editor.select_to_previous_word_start(&SelectToPreviousWordStart, window, cx);
        assert_selection_ranges(
            "use std::«ˇs»tr::{foo, bar}\n\n  {«ˇb»az.qux()}",
            editor,
            cx,
        );

        editor.select_to_previous_word_start(&SelectToPreviousWordStart, window, cx);
        assert_selection_ranges(
            "use std«ˇ::s»tr::{foo, bar}\n\n  «ˇ{b»az.qux()}",
            editor,
            cx,
        );

        editor.select_to_next_word_end(&SelectToNextWordEnd, window, cx);
        assert_selection_ranges(
            "use std::«ˇs»tr::{foo, bar}\n\n  {«ˇb»az.qux()}",
            editor,
            cx,
        );
    });
}

#[gpui::test]
fn test_prev_next_word_bounds_with_soft_wrap(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("use one::{\n    two::three::four::five\n};", cx);
        build_editor(buffer, window, cx)
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.set_wrap_width(Some(140.0.into()), cx);
        assert_eq!(
            editor.display_text(cx),
            "use one::{\n    two::three::\n    four::five\n};"
        );

        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(1), 7)..DisplayPoint::new(DisplayRow(1), 7)
            ]);
        });

        editor.move_to_next_word_end(&MoveToNextWordEnd, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(DisplayRow(1), 9)..DisplayPoint::new(DisplayRow(1), 9)]
        );

        editor.move_to_next_word_end(&MoveToNextWordEnd, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(DisplayRow(1), 14)..DisplayPoint::new(DisplayRow(1), 14)]
        );

        editor.move_to_next_word_end(&MoveToNextWordEnd, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(DisplayRow(2), 4)..DisplayPoint::new(DisplayRow(2), 4)]
        );

        editor.move_to_next_word_end(&MoveToNextWordEnd, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(DisplayRow(2), 8)..DisplayPoint::new(DisplayRow(2), 8)]
        );

        editor.move_to_previous_word_start(&MoveToPreviousWordStart, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(DisplayRow(2), 4)..DisplayPoint::new(DisplayRow(2), 4)]
        );

        editor.move_to_previous_word_start(&MoveToPreviousWordStart, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(DisplayRow(1), 14)..DisplayPoint::new(DisplayRow(1), 14)]
        );
    });
}

#[gpui::test]
async fn test_move_start_of_paragraph_end_of_paragraph(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;

    let line_height = cx.editor(|editor, window, _| {
        editor
            .style()
            .unwrap()
            .text
            .line_height_in_pixels(window.rem_size())
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

    cx.update_editor(|editor, window, cx| {
        editor.move_to_end_of_paragraph(&MoveToEndOfParagraph, window, cx)
    });
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

    cx.update_editor(|editor, window, cx| {
        editor.move_to_end_of_paragraph(&MoveToEndOfParagraph, window, cx)
    });
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

    cx.update_editor(|editor, window, cx| {
        editor.move_to_end_of_paragraph(&MoveToEndOfParagraph, window, cx)
    });
    cx.assert_editor_state(
        &r#"one
        two

        three
        four
        five

        sixˇ"#
            .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.move_to_start_of_paragraph(&MoveToStartOfParagraph, window, cx)
    });
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

    cx.update_editor(|editor, window, cx| {
        editor.move_to_start_of_paragraph(&MoveToStartOfParagraph, window, cx)
    });
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

    cx.update_editor(|editor, window, cx| {
        editor.move_to_start_of_paragraph(&MoveToStartOfParagraph, window, cx)
    });
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
async fn test_scroll_page_up_page_down(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;
    let line_height = cx.editor(|editor, window, _| {
        editor
            .style()
            .unwrap()
            .text
            .line_height_in_pixels(window.rem_size())
    });
    let window = cx.window;
    cx.simulate_window_resize(window, size(px(1000.), 4. * line_height + px(0.5)));

    cx.set_state(
        r#"ˇone
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

    cx.update_editor(|editor, window, cx| {
        assert_eq!(
            editor.snapshot(window, cx).scroll_position(),
            gpui::Point::new(0., 0.)
        );
        editor.scroll_screen(&ScrollAmount::Page(1.), window, cx);
        assert_eq!(
            editor.snapshot(window, cx).scroll_position(),
            gpui::Point::new(0., 3.)
        );
        editor.scroll_screen(&ScrollAmount::Page(1.), window, cx);
        assert_eq!(
            editor.snapshot(window, cx).scroll_position(),
            gpui::Point::new(0., 6.)
        );
        editor.scroll_screen(&ScrollAmount::Page(-1.), window, cx);
        assert_eq!(
            editor.snapshot(window, cx).scroll_position(),
            gpui::Point::new(0., 3.)
        );

        editor.scroll_screen(&ScrollAmount::Page(-0.5), window, cx);
        assert_eq!(
            editor.snapshot(window, cx).scroll_position(),
            gpui::Point::new(0., 1.)
        );
        editor.scroll_screen(&ScrollAmount::Page(0.5), window, cx);
        assert_eq!(
            editor.snapshot(window, cx).scroll_position(),
            gpui::Point::new(0., 3.)
        );
    });
}

#[gpui::test]
async fn test_autoscroll(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;

    let line_height = cx.update_editor(|editor, window, cx| {
        editor.set_vertical_scroll_margin(2, cx);
        editor
            .style()
            .unwrap()
            .text
            .line_height_in_pixels(window.rem_size())
    });
    let window = cx.window;
    cx.simulate_window_resize(window, size(px(1000.), 6. * line_height));

    cx.set_state(
        r#"ˇone
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
    cx.update_editor(|editor, window, cx| {
        assert_eq!(
            editor.snapshot(window, cx).scroll_position(),
            gpui::Point::new(0., 0.0)
        );
    });

    // Add a cursor below the visible area. Since both cursors cannot fit
    // on screen, the editor autoscrolls to reveal the newest cursor, and
    // allows the vertical scroll margin below that cursor.
    cx.update_editor(|editor, window, cx| {
        editor.change_selections(Some(Autoscroll::fit()), window, cx, |selections| {
            selections.select_ranges([
                Point::new(0, 0)..Point::new(0, 0),
                Point::new(6, 0)..Point::new(6, 0),
            ]);
        })
    });
    cx.update_editor(|editor, window, cx| {
        assert_eq!(
            editor.snapshot(window, cx).scroll_position(),
            gpui::Point::new(0., 3.0)
        );
    });

    // Move down. The editor cursor scrolls down to track the newest cursor.
    cx.update_editor(|editor, window, cx| {
        editor.move_down(&Default::default(), window, cx);
    });
    cx.update_editor(|editor, window, cx| {
        assert_eq!(
            editor.snapshot(window, cx).scroll_position(),
            gpui::Point::new(0., 4.0)
        );
    });

    // Add a cursor above the visible area. Since both cursors fit on screen,
    // the editor scrolls to show both.
    cx.update_editor(|editor, window, cx| {
        editor.change_selections(Some(Autoscroll::fit()), window, cx, |selections| {
            selections.select_ranges([
                Point::new(1, 0)..Point::new(1, 0),
                Point::new(6, 0)..Point::new(6, 0),
            ]);
        })
    });
    cx.update_editor(|editor, window, cx| {
        assert_eq!(
            editor.snapshot(window, cx).scroll_position(),
            gpui::Point::new(0., 1.0)
        );
    });
}

#[gpui::test]
async fn test_move_page_up_page_down(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;

    let line_height = cx.editor(|editor, window, _cx| {
        editor
            .style()
            .unwrap()
            .text
            .line_height_in_pixels(window.rem_size())
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

    cx.update_editor(|editor, window, cx| {
        editor.move_page_down(&MovePageDown::default(), window, cx)
    });
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

    cx.update_editor(|editor, window, cx| {
        editor.move_page_down(&MovePageDown::default(), window, cx)
    });
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

    cx.update_editor(|editor, window, cx| editor.move_page_up(&MovePageUp::default(), window, cx));
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

    cx.update_editor(|editor, window, cx| editor.move_page_up(&MovePageUp::default(), window, cx));
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
    cx.update_editor(|editor, window, cx| {
        editor.move_page_down(&MovePageDown::default(), window, cx);
        editor.move_page_down(&MovePageDown::default(), window, cx);
        editor.move_page_down(&MovePageDown::default(), window, cx);
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
async fn test_delete_to_beginning_of_line(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state("one «two threeˇ» four");
    cx.update_editor(|editor, window, cx| {
        editor.delete_to_beginning_of_line(
            &DeleteToBeginningOfLine {
                stop_at_indent: false,
            },
            window,
            cx,
        );
        assert_eq!(editor.text(cx), " four");
    });
}

#[gpui::test]
fn test_delete_to_word_boundary(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("one two three four", cx);
        build_editor(buffer.clone(), window, cx)
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                // an empty selection - the preceding word fragment is deleted
                DisplayPoint::new(DisplayRow(0), 2)..DisplayPoint::new(DisplayRow(0), 2),
                // characters selected - they are deleted
                DisplayPoint::new(DisplayRow(0), 9)..DisplayPoint::new(DisplayRow(0), 12),
            ])
        });
        editor.delete_to_previous_word_start(
            &DeleteToPreviousWordStart {
                ignore_newlines: false,
            },
            window,
            cx,
        );
        assert_eq!(editor.buffer.read(cx).read(cx).text(), "e two te four");
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                // an empty selection - the following word fragment is deleted
                DisplayPoint::new(DisplayRow(0), 3)..DisplayPoint::new(DisplayRow(0), 3),
                // characters selected - they are deleted
                DisplayPoint::new(DisplayRow(0), 9)..DisplayPoint::new(DisplayRow(0), 10),
            ])
        });
        editor.delete_to_next_word_end(
            &DeleteToNextWordEnd {
                ignore_newlines: false,
            },
            window,
            cx,
        );
        assert_eq!(editor.buffer.read(cx).read(cx).text(), "e t te our");
    });
}

#[gpui::test]
fn test_delete_to_previous_word_start_or_newline(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("one\n2\nthree\n4", cx);
        build_editor(buffer.clone(), window, cx)
    });
    let del_to_prev_word_start = DeleteToPreviousWordStart {
        ignore_newlines: false,
    };
    let del_to_prev_word_start_ignore_newlines = DeleteToPreviousWordStart {
        ignore_newlines: true,
    };

    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(3), 1)..DisplayPoint::new(DisplayRow(3), 1)
            ])
        });
        editor.delete_to_previous_word_start(&del_to_prev_word_start, window, cx);
        assert_eq!(editor.buffer.read(cx).read(cx).text(), "one\n2\nthree\n");
        editor.delete_to_previous_word_start(&del_to_prev_word_start, window, cx);
        assert_eq!(editor.buffer.read(cx).read(cx).text(), "one\n2\nthree");
        editor.delete_to_previous_word_start(&del_to_prev_word_start, window, cx);
        assert_eq!(editor.buffer.read(cx).read(cx).text(), "one\n2\n");
        editor.delete_to_previous_word_start(&del_to_prev_word_start, window, cx);
        assert_eq!(editor.buffer.read(cx).read(cx).text(), "one\n2");
        editor.delete_to_previous_word_start(&del_to_prev_word_start_ignore_newlines, window, cx);
        assert_eq!(editor.buffer.read(cx).read(cx).text(), "one\n");
        editor.delete_to_previous_word_start(&del_to_prev_word_start_ignore_newlines, window, cx);
        assert_eq!(editor.buffer.read(cx).read(cx).text(), "");
    });
}

#[gpui::test]
fn test_delete_to_next_word_end_or_newline(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("\none\n   two\nthree\n   four", cx);
        build_editor(buffer.clone(), window, cx)
    });
    let del_to_next_word_end = DeleteToNextWordEnd {
        ignore_newlines: false,
    };
    let del_to_next_word_end_ignore_newlines = DeleteToNextWordEnd {
        ignore_newlines: true,
    };

    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 0)
            ])
        });
        editor.delete_to_next_word_end(&del_to_next_word_end, window, cx);
        assert_eq!(
            editor.buffer.read(cx).read(cx).text(),
            "one\n   two\nthree\n   four"
        );
        editor.delete_to_next_word_end(&del_to_next_word_end, window, cx);
        assert_eq!(
            editor.buffer.read(cx).read(cx).text(),
            "\n   two\nthree\n   four"
        );
        editor.delete_to_next_word_end(&del_to_next_word_end, window, cx);
        assert_eq!(
            editor.buffer.read(cx).read(cx).text(),
            "two\nthree\n   four"
        );
        editor.delete_to_next_word_end(&del_to_next_word_end, window, cx);
        assert_eq!(editor.buffer.read(cx).read(cx).text(), "\nthree\n   four");
        editor.delete_to_next_word_end(&del_to_next_word_end_ignore_newlines, window, cx);
        assert_eq!(editor.buffer.read(cx).read(cx).text(), "\n   four");
        editor.delete_to_next_word_end(&del_to_next_word_end_ignore_newlines, window, cx);
        assert_eq!(editor.buffer.read(cx).read(cx).text(), "");
    });
}

#[gpui::test]
fn test_newline(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("aaaa\n    bbbb\n", cx);
        build_editor(buffer.clone(), window, cx)
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 2)..DisplayPoint::new(DisplayRow(0), 2),
                DisplayPoint::new(DisplayRow(1), 2)..DisplayPoint::new(DisplayRow(1), 2),
                DisplayPoint::new(DisplayRow(1), 6)..DisplayPoint::new(DisplayRow(1), 6),
            ])
        });

        editor.newline(&Newline, window, cx);
        assert_eq!(editor.text(cx), "aa\naa\n  \n    bb\n    bb\n");
    });
}

#[gpui::test]
fn test_newline_with_old_selections(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
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
        let mut editor = build_editor(buffer.clone(), window, cx);
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([
                Point::new(2, 4)..Point::new(2, 5),
                Point::new(5, 4)..Point::new(5, 5),
            ])
        });
        editor
    });

    _ = editor.update(cx, |editor, window, cx| {
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

        editor.newline(&Newline, window, cx);
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
async fn test_newline_above(cx: &mut TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.tab_size = NonZeroU32::new(4)
    });

    let language = Arc::new(
        Language::new(
            LanguageConfig::default(),
            Some(tree_sitter_rust::LANGUAGE.into()),
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

    cx.update_editor(|e, window, cx| e.newline_above(&NewlineAbove, window, cx));
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
async fn test_newline_below(cx: &mut TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.tab_size = NonZeroU32::new(4)
    });

    let language = Arc::new(
        Language::new(
            LanguageConfig::default(),
            Some(tree_sitter_rust::LANGUAGE.into()),
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

    cx.update_editor(|e, window, cx| e.newline_below(&NewlineBelow, window, cx));
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
async fn test_newline_comments(cx: &mut TestAppContext) {
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

        cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
        cx.assert_editor_state(indoc! {"
        // Foo
        //ˇ
    "});
        // Ensure that if cursor is before the comment start, we do not actually insert a comment prefix.
        cx.set_state(indoc! {"
        ˇ// Foo
    "});
        cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
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
    cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
    cx.assert_editor_state(indoc! {"
        // Foo
        ˇ
    "});
}

#[gpui::test]
async fn test_newline_documentation_comments(cx: &mut TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.tab_size = NonZeroU32::new(4)
    });

    let language = Arc::new(Language::new(
        LanguageConfig {
            documentation: Some(language::DocumentationConfig {
                start: "/**".into(),
                end: "*/".into(),
                prefix: "* ".into(),
                tab_size: NonZeroU32::new(1).unwrap(),
            }),
            ..LanguageConfig::default()
        },
        None,
    ));
    {
        let mut cx = EditorTestContext::new(cx).await;
        cx.update_buffer(|buffer, cx| buffer.set_language(Some(language), cx));
        cx.set_state(indoc! {"
        /**ˇ
    "});

        cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
        cx.assert_editor_state(indoc! {"
        /**
         * ˇ
    "});
        // Ensure that if cursor is before the comment start,
        // we do not actually insert a comment prefix.
        cx.set_state(indoc! {"
        ˇ/**
    "});
        cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
        cx.assert_editor_state(indoc! {"

        ˇ/**
    "});
        // Ensure that if cursor is between it doesn't add comment prefix.
        cx.set_state(indoc! {"
        /*ˇ*
    "});
        cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
        cx.assert_editor_state(indoc! {"
        /*
        ˇ*
    "});
        // Ensure that if suffix exists on same line after cursor it adds new line.
        cx.set_state(indoc! {"
        /**ˇ*/
    "});
        cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
        cx.assert_editor_state(indoc! {"
        /**
         * ˇ
         */
    "});
        // Ensure that if suffix exists on same line after cursor with space it adds new line.
        cx.set_state(indoc! {"
        /**ˇ */
    "});
        cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
        cx.assert_editor_state(indoc! {"
        /**
         * ˇ
         */
    "});
        // Ensure that if suffix exists on same line after cursor with space it adds new line.
        cx.set_state(indoc! {"
        /** ˇ*/
    "});
        cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
        cx.assert_editor_state(
            indoc! {"
        /**s
         * ˇ
         */
    "}
            .replace("s", " ") // s is used as space placeholder to prevent format on save
            .as_str(),
        );
        // Ensure that delimiter space is preserved when newline on already
        // spaced delimiter.
        cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
        cx.assert_editor_state(
            indoc! {"
        /**s
         *s
         * ˇ
         */
    "}
            .replace("s", " ") // s is used as space placeholder to prevent format on save
            .as_str(),
        );
        // Ensure that delimiter space is preserved when space is not
        // on existing delimiter.
        cx.set_state(indoc! {"
        /**
         *ˇ
         */
    "});
        cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
        cx.assert_editor_state(indoc! {"
        /**
         *
         * ˇ
         */
    "});
        // Ensure that if suffix exists on same line after cursor it
        // doesn't add extra new line if prefix is not on same line.
        cx.set_state(indoc! {"
        /**
        ˇ*/
    "});
        cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
        cx.assert_editor_state(indoc! {"
        /**

        ˇ*/
    "});
        // Ensure that it detects suffix after existing prefix.
        cx.set_state(indoc! {"
        /**ˇ/
    "});
        cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
        cx.assert_editor_state(indoc! {"
        /**
        ˇ/
    "});
        // Ensure that if suffix exists on same line before
        // cursor it does not add comment prefix.
        cx.set_state(indoc! {"
        /** */ˇ
    "});
        cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
        cx.assert_editor_state(indoc! {"
        /** */
        ˇ
    "});
        // Ensure that if suffix exists on same line before
        // cursor it does not add comment prefix.
        cx.set_state(indoc! {"
        /**
         *
         */ˇ
    "});
        cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
        cx.assert_editor_state(indoc! {"
        /**
         *
         */
         ˇ
    "});
    }
    // Ensure that comment continuations can be disabled.
    update_test_language_settings(cx, |settings| {
        settings.defaults.extend_comment_on_newline = Some(false);
    });
    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state(indoc! {"
        /**ˇ
    "});
    cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
    cx.assert_editor_state(indoc! {"
        /**
        ˇ
    "});
}

#[gpui::test]
fn test_insert_with_old_selections(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("a( X ), b( Y ), c( Z )", cx);
        let mut editor = build_editor(buffer.clone(), window, cx);
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([3..4, 11..12, 19..20])
        });
        editor
    });

    _ = editor.update(cx, |editor, window, cx| {
        // Edit the buffer directly, deleting ranges surrounding the editor's selections
        editor.buffer.update(cx, |buffer, cx| {
            buffer.edit([(2..5, ""), (10..13, ""), (18..21, "")], None, cx);
            assert_eq!(buffer.read(cx).text(), "a(), b(), c()".unindent());
        });
        assert_eq!(editor.selections.ranges(cx), &[2..2, 7..7, 12..12],);

        editor.insert("Z", window, cx);
        assert_eq!(editor.text(cx), "a(Z), b(Z), c(Z)");

        // The selections are moved after the inserted characters
        assert_eq!(editor.selections.ranges(cx), &[3..3, 9..9, 15..15],);
    });
}

#[gpui::test]
async fn test_tab(cx: &mut TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.tab_size = NonZeroU32::new(3)
    });

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state(indoc! {"
        ˇabˇc
        ˇ🏀ˇ🏀ˇefg
        dˇ
    "});
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
           ˇab ˇc
           ˇ🏀  ˇ🏀  ˇefg
        d  ˇ
    "});

    cx.set_state(indoc! {"
        a
        «🏀ˇ»🏀«🏀ˇ»🏀«🏀ˇ»
    "});
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
        a
           «🏀ˇ»🏀«🏀ˇ»🏀«🏀ˇ»
    "});
}

#[gpui::test]
async fn test_tab_in_leading_whitespace_auto_indents_lines(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let language = Arc::new(
        Language::new(
            LanguageConfig::default(),
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
        .with_indents_query(r#"(_ "(" ")" @end) @indent"#)
        .unwrap(),
    );
    cx.update_buffer(|buffer, cx| buffer.set_language(Some(language), cx));

    // test when all cursors are not at suggested indent
    // then simply move to their suggested indent location
    cx.set_state(indoc! {"
        const a: B = (
            c(
        ˇ
        ˇ    )
        );
    "});
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
        const a: B = (
            c(
                ˇ
            ˇ)
        );
    "});

    // test cursor already at suggested indent not moving when
    // other cursors are yet to reach their suggested indents
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
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
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
    // test when all cursors are at suggested indent then tab is inserted
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
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

    // test when current indent is less than suggested indent,
    // we adjust line to match suggested indent and move cursor to it
    //
    // when no other cursor is at word boundary, all of them should move
    cx.set_state(indoc! {"
        const a: B = (
            c(
                d(
        ˇ
        ˇ   )
        ˇ   )
        );
    "});
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
        const a: B = (
            c(
                d(
                    ˇ
                ˇ)
            ˇ)
        );
    "});

    // test when current indent is less than suggested indent,
    // we adjust line to match suggested indent and move cursor to it
    //
    // when some other cursor is at word boundary, it should not move
    cx.set_state(indoc! {"
        const a: B = (
            c(
                d(
        ˇ
        ˇ   )
           ˇ)
        );
    "});
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
        const a: B = (
            c(
                d(
                    ˇ
                ˇ)
            ˇ)
        );
    "});

    // test when current indent is more than suggested indent,
    // we just move cursor to current indent instead of suggested indent
    //
    // when no other cursor is at word boundary, all of them should move
    cx.set_state(indoc! {"
        const a: B = (
            c(
                d(
        ˇ
        ˇ                )
        ˇ   )
        );
    "});
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
        const a: B = (
            c(
                d(
                    ˇ
                        ˇ)
            ˇ)
        );
    "});
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
        const a: B = (
            c(
                d(
                        ˇ
                            ˇ)
                ˇ)
        );
    "});

    // test when current indent is more than suggested indent,
    // we just move cursor to current indent instead of suggested indent
    //
    // when some other cursor is at word boundary, it doesn't move
    cx.set_state(indoc! {"
        const a: B = (
            c(
                d(
        ˇ
        ˇ                )
            ˇ)
        );
    "});
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
        const a: B = (
            c(
                d(
                    ˇ
                        ˇ)
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
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
        const a: B = (
            c(
                ˇ
            ˇ)
        );
    "});
}

#[gpui::test]
async fn test_tab_with_mixed_whitespace_txt(cx: &mut TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.tab_size = NonZeroU32::new(3)
    });

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state(indoc! {"
         ˇ
        \t ˇ
        \t  ˇ
        \t   ˇ
         \t  \t\t \t      \t\t   \t\t    \t \t ˇ
    "});

    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
           ˇ
        \t   ˇ
        \t   ˇ
        \t      ˇ
         \t  \t\t \t      \t\t   \t\t    \t \t   ˇ
    "});
}

#[gpui::test]
async fn test_tab_with_mixed_whitespace_rust(cx: &mut TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.tab_size = NonZeroU32::new(4)
    });

    let language = Arc::new(
        Language::new(
            LanguageConfig::default(),
            Some(tree_sitter_rust::LANGUAGE.into()),
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

    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
        fn a() {
            if b {
                ˇc
            }
        }
    "});
}

#[gpui::test]
async fn test_indent_outdent(cx: &mut TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.tab_size = NonZeroU32::new(4);
    });

    let mut cx = EditorTestContext::new(cx).await;

    cx.set_state(indoc! {"
          «oneˇ» «twoˇ»
        three
         four
    "});
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
            «oneˇ» «twoˇ»
        three
         four
    "});

    cx.update_editor(|e, window, cx| e.backtab(&Backtab, window, cx));
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
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
        one two
            t«hree
        ˇ» four
    "});

    cx.update_editor(|e, window, cx| e.backtab(&Backtab, window, cx));
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
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
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
    cx.update_editor(|e, window, cx| e.backtab(&Backtab, window, cx));
    cx.assert_editor_state(indoc! {"
        one two
        ˇthree
            four
    "});
}

#[gpui::test]
async fn test_indent_outdent_with_hard_tabs(cx: &mut TestAppContext) {
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
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
        \t«oneˇ» «twoˇ»
        three
        four
    "});
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
        \t\t«oneˇ» «twoˇ»
        three
        four
    "});
    cx.update_editor(|e, window, cx| e.backtab(&Backtab, window, cx));
    cx.assert_editor_state(indoc! {"
        \t«oneˇ» «twoˇ»
        three
        four
    "});
    cx.update_editor(|e, window, cx| e.backtab(&Backtab, window, cx));
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
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
        one two
        \tt«hree
        ˇ»four
    "});
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
        one two
        \t\tt«hree
        ˇ»four
    "});
    cx.update_editor(|e, window, cx| e.backtab(&Backtab, window, cx));
    cx.assert_editor_state(indoc! {"
        one two
        \tt«hree
        ˇ»four
    "});
    cx.update_editor(|e, window, cx| e.backtab(&Backtab, window, cx));
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
    cx.update_editor(|e, window, cx| e.backtab(&Backtab, window, cx));
    cx.assert_editor_state(indoc! {"
        one two
        ˇthree
        four
    "});
    cx.update_editor(|e, window, cx| e.tab(&Tab, window, cx));
    cx.assert_editor_state(indoc! {"
        one two
        \tˇthree
        four
    "});
    cx.update_editor(|e, window, cx| e.backtab(&Backtab, window, cx));
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

    let toml_buffer =
        cx.new(|cx| Buffer::local("a = 1\nb = 2\n", cx).with_language(toml_language, cx));
    let rust_buffer =
        cx.new(|cx| Buffer::local("const c: usize = 3;\n", cx).with_language(rust_language, cx));
    let multibuffer = cx.new(|cx| {
        let mut multibuffer = MultiBuffer::new(ReadWrite);
        multibuffer.push_excerpts(
            toml_buffer.clone(),
            [ExcerptRange::new(Point::new(0, 0)..Point::new(2, 0))],
            cx,
        );
        multibuffer.push_excerpts(
            rust_buffer.clone(),
            [ExcerptRange::new(Point::new(0, 0)..Point::new(1, 0))],
            cx,
        );
        multibuffer
    });

    cx.add_window(|window, cx| {
        let mut editor = build_editor(multibuffer, window, cx);

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
            window,
            cx,
        );

        editor.tab(&Tab, window, cx);
        assert_text_with_selections(
            &mut editor,
            indoc! {"
                  «aˇ» = 1
                b = 2

                    «const c:ˇ» usize = 3;
            "},
            cx,
        );
        editor.backtab(&Backtab, window, cx);
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
async fn test_backspace(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    // Basic backspace
    cx.set_state(indoc! {"
        onˇe two three
        fou«rˇ» five six
        seven «ˇeight nine
        »ten
    "});
    cx.update_editor(|e, window, cx| e.backspace(&Backspace, window, cx));
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
    cx.update_editor(|e, window, cx| e.backspace(&Backspace, window, cx));
    cx.assert_editor_state(indoc! {"
        zero
        ˇone
            ˇtwo
        ˇ  threeˇ  four
    "});
}

#[gpui::test]
async fn test_delete(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state(indoc! {"
        onˇe two three
        fou«rˇ» five six
        seven «ˇeight nine
        »ten
    "});
    cx.update_editor(|e, window, cx| e.delete(&Delete, window, cx));
    cx.assert_editor_state(indoc! {"
        onˇ two three
        fouˇ five six
        seven ˇten
    "});
}

#[gpui::test]
fn test_delete_line(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        build_editor(buffer, window, cx)
    });
    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(0), 1),
                DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 1),
                DisplayPoint::new(DisplayRow(3), 0)..DisplayPoint::new(DisplayRow(3), 0),
            ])
        });
        editor.delete_line(&DeleteLine, window, cx);
        assert_eq!(editor.display_text(cx), "ghi");
        assert_eq!(
            editor.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 0),
                DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(0), 1)
            ]
        );
    });

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        build_editor(buffer, window, cx)
    });
    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(2), 0)..DisplayPoint::new(DisplayRow(0), 1)
            ])
        });
        editor.delete_line(&DeleteLine, window, cx);
        assert_eq!(editor.display_text(cx), "ghi\n");
        assert_eq!(
            editor.selections.display_ranges(cx),
            vec![DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(0), 1)]
        );
    });
}

#[gpui::test]
fn test_join_lines_with_single_selection(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("aaa\nbbb\nccc\nddd\n\n", cx);
        let mut editor = build_editor(buffer.clone(), window, cx);
        let buffer = buffer.read(cx).as_singleton().unwrap();

        assert_eq!(
            editor.selections.ranges::<Point>(cx),
            &[Point::new(0, 0)..Point::new(0, 0)]
        );

        // When on single line, replace newline at end by space
        editor.join_lines(&JoinLines, window, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb\nccc\nddd\n\n");
        assert_eq!(
            editor.selections.ranges::<Point>(cx),
            &[Point::new(0, 3)..Point::new(0, 3)]
        );

        // When multiple lines are selected, remove newlines that are spanned by the selection
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(0, 5)..Point::new(2, 2)])
        });
        editor.join_lines(&JoinLines, window, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb ccc ddd\n\n");
        assert_eq!(
            editor.selections.ranges::<Point>(cx),
            &[Point::new(0, 11)..Point::new(0, 11)]
        );

        // Undo should be transactional
        editor.undo(&Undo, window, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb\nccc\nddd\n\n");
        assert_eq!(
            editor.selections.ranges::<Point>(cx),
            &[Point::new(0, 5)..Point::new(2, 2)]
        );

        // When joining an empty line don't insert a space
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(2, 1)..Point::new(2, 2)])
        });
        editor.join_lines(&JoinLines, window, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb\nccc\nddd\n");
        assert_eq!(
            editor.selections.ranges::<Point>(cx),
            [Point::new(2, 3)..Point::new(2, 3)]
        );

        // We can remove trailing newlines
        editor.join_lines(&JoinLines, window, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb\nccc\nddd");
        assert_eq!(
            editor.selections.ranges::<Point>(cx),
            [Point::new(2, 3)..Point::new(2, 3)]
        );

        // We don't blow up on the last line
        editor.join_lines(&JoinLines, window, cx);
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
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(0, 1)..Point::new(0, 1)])
        });
        editor.join_lines(&JoinLines, window, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb c\n  \n\td");

        // We don't insert a space for a line containing only spaces
        editor.join_lines(&JoinLines, window, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb c\n\td");

        // We ignore any leading tabs
        editor.join_lines(&JoinLines, window, cx);
        assert_eq!(buffer.read(cx).text(), "aaa bbb c d");

        editor
    });
}

#[gpui::test]
fn test_join_lines_with_multi_selection(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("aaa\nbbb\nccc\nddd\n\n", cx);
        let mut editor = build_editor(buffer.clone(), window, cx);
        let buffer = buffer.read(cx).as_singleton().unwrap();

        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([
                Point::new(0, 2)..Point::new(1, 1),
                Point::new(1, 2)..Point::new(1, 2),
                Point::new(3, 1)..Point::new(3, 2),
            ])
        });

        editor.join_lines(&JoinLines, window, cx);
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
async fn test_join_lines_with_git_diff_base(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    let diff_base = r#"
        Line 0
        Line 1
        Line 2
        Line 3
        "#
    .unindent();

    cx.set_state(
        &r#"
        ˇLine 0
        Line 1
        Line 2
        Line 3
        "#
        .unindent(),
    );

    cx.set_head_text(&diff_base);
    executor.run_until_parked();

    // Join lines
    cx.update_editor(|editor, window, cx| {
        editor.join_lines(&JoinLines, window, cx);
    });
    executor.run_until_parked();

    cx.assert_editor_state(
        &r#"
        Line 0ˇ Line 1
        Line 2
        Line 3
        "#
        .unindent(),
    );
    // Join again
    cx.update_editor(|editor, window, cx| {
        editor.join_lines(&JoinLines, window, cx);
    });
    executor.run_until_parked();

    cx.assert_editor_state(
        &r#"
        Line 0 Line 1ˇ Line 2
        Line 3
        "#
        .unindent(),
    );
}

#[gpui::test]
async fn test_custom_newlines_cause_no_false_positive_diffs(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state("Line 0\r\nLine 1\rˇ\nLine 2\r\nLine 3");
    cx.set_head_text("Line 0\r\nLine 1\r\nLine 2\r\nLine 3");
    executor.run_until_parked();

    cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        assert_eq!(
            snapshot
                .buffer_snapshot
                .diff_hunks_in_range(0..snapshot.buffer_snapshot.len())
                .collect::<Vec<_>>(),
            Vec::new(),
            "Should not have any diffs for files with custom newlines"
        );
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
    cx.update_editor(|e, window, cx| {
        e.sort_lines_case_insensitive(&SortLinesCaseInsensitive, window, cx)
    });
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
    cx.update_editor(|e, window, cx| e.reverse_lines(&ReverseLines, window, cx));
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
    cx.update_editor(|e, window, cx| {
        e.sort_lines_case_sensitive(&SortLinesCaseSensitive, window, cx)
    });
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
    cx.update_editor(|e, window, cx| {
        e.sort_lines_case_sensitive(&SortLinesCaseSensitive, window, cx)
    });
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
    cx.update_editor(|e, window, cx| {
        e.sort_lines_case_sensitive(&SortLinesCaseSensitive, window, cx)
    });
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
    cx.update_editor(|e, window, cx| {
        e.manipulate_lines(window, cx, |lines| lines.push("added_line"))
    });
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
    cx.update_editor(|e, window, cx| {
        e.manipulate_lines(window, cx, |lines| {
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
    cx.update_editor(|e, window, cx| {
        e.manipulate_lines(window, cx, |lines| {
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
    cx.update_editor(|e, window, cx| {
        e.unique_lines_case_sensitive(&UniqueLinesCaseSensitive, window, cx)
    });
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
    cx.update_editor(|e, window, cx| {
        e.unique_lines_case_insensitive(&UniqueLinesCaseInsensitive, window, cx)
    });
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
    cx.update_editor(|e, window, cx| {
        e.unique_lines_case_sensitive(&UniqueLinesCaseSensitive, window, cx)
    });
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
    cx.update_editor(|e, window, cx| {
        e.unique_lines_case_sensitive(&UniqueLinesCaseSensitive, window, cx)
    });
    cx.assert_editor_state(indoc! {"
        «Aaa
        aAaˇ»
    "});

    cx.set_state(indoc! {"
        «Aaa
        aAa
        aaAˇ»
    "});
    cx.update_editor(|e, window, cx| {
        e.unique_lines_case_insensitive(&UniqueLinesCaseInsensitive, window, cx)
    });
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
    cx.update_editor(|e, window, cx| {
        e.sort_lines_case_sensitive(&SortLinesCaseSensitive, window, cx)
    });
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
    cx.update_editor(|e, window, cx| {
        e.sort_lines_case_sensitive(&SortLinesCaseSensitive, window, cx)
    });
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
    cx.update_editor(|e, window, cx| {
        e.manipulate_lines(window, cx, |lines| lines.push("added line"))
    });
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
    cx.update_editor(|e, window, cx| {
        e.manipulate_lines(window, cx, |lines| {
            lines.pop();
        })
    });
    cx.assert_editor_state(indoc! {"
        «2ˇ»

        «bbbbˇ»
    "});
}

#[gpui::test]
async fn test_toggle_case(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    // If all lower case -> upper case
    cx.set_state(indoc! {"
        «hello worldˇ»
    "});
    cx.update_editor(|e, window, cx| e.toggle_case(&ToggleCase, window, cx));
    cx.assert_editor_state(indoc! {"
        «HELLO WORLDˇ»
    "});

    // If all upper case -> lower case
    cx.set_state(indoc! {"
        «HELLO WORLDˇ»
    "});
    cx.update_editor(|e, window, cx| e.toggle_case(&ToggleCase, window, cx));
    cx.assert_editor_state(indoc! {"
        «hello worldˇ»
    "});

    // If any upper case characters are identified -> lower case
    // This matches JetBrains IDEs
    cx.set_state(indoc! {"
        «hEllo worldˇ»
    "});
    cx.update_editor(|e, window, cx| e.toggle_case(&ToggleCase, window, cx));
    cx.assert_editor_state(indoc! {"
        «hello worldˇ»
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
    cx.update_editor(|e, window, cx| e.convert_to_upper_case(&ConvertToUpperCase, window, cx));
    cx.assert_editor_state(indoc! {"
        «HELLO WORLDˇ»
    "});

    // Test convert_to_lower_case()
    cx.set_state(indoc! {"
        «HELLO WORLDˇ»
    "});
    cx.update_editor(|e, window, cx| e.convert_to_lower_case(&ConvertToLowerCase, window, cx));
    cx.assert_editor_state(indoc! {"
        «hello worldˇ»
    "});

    // Test multiple line, single selection case
    cx.set_state(indoc! {"
        «The quick brown
        fox jumps over
        the lazy dogˇ»
    "});
    cx.update_editor(|e, window, cx| e.convert_to_title_case(&ConvertToTitleCase, window, cx));
    cx.assert_editor_state(indoc! {"
        «The Quick Brown
        Fox Jumps Over
        The Lazy Dogˇ»
    "});

    // Test multiple line, single selection case
    cx.set_state(indoc! {"
        «The quick brown
        fox jumps over
        the lazy dogˇ»
    "});
    cx.update_editor(|e, window, cx| {
        e.convert_to_upper_camel_case(&ConvertToUpperCamelCase, window, cx)
    });
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
    cx.update_editor(|e, window, cx| e.convert_to_upper_case(&ConvertToUpperCase, window, cx));
    cx.assert_editor_state(indoc! {"
        «HELLOˇ» big «BEAUTIFULˇ» «WORLDˇ»
    "});

    // Test multiple selections on a single line and across multiple lines
    cx.set_state(indoc! {"
        «Theˇ» quick «brown
        foxˇ» jumps «overˇ»
        the «lazyˇ» dog
    "});
    cx.update_editor(|e, window, cx| e.convert_to_upper_case(&ConvertToUpperCase, window, cx));
    cx.assert_editor_state(indoc! {"
        «THEˇ» quick «BROWN
        FOXˇ» jumps «OVERˇ»
        the «LAZYˇ» dog
    "});

    // Test case where text length grows
    cx.set_state(indoc! {"
        «tschüßˇ»
    "});
    cx.update_editor(|e, window, cx| e.convert_to_upper_case(&ConvertToUpperCase, window, cx));
    cx.assert_editor_state(indoc! {"
        «TSCHÜSSˇ»
    "});

    // Test to make sure we don't crash when text shrinks
    cx.set_state(indoc! {"
        aaa_bbbˇ
    "});
    cx.update_editor(|e, window, cx| {
        e.convert_to_lower_camel_case(&ConvertToLowerCamelCase, window, cx)
    });
    cx.assert_editor_state(indoc! {"
        «aaaBbbˇ»
    "});

    // Test to make sure we all aware of the fact that each word can grow and shrink
    // Final selections should be aware of this fact
    cx.set_state(indoc! {"
        aaa_bˇbb bbˇb_ccc ˇccc_ddd
    "});
    cx.update_editor(|e, window, cx| {
        e.convert_to_lower_camel_case(&ConvertToLowerCamelCase, window, cx)
    });
    cx.assert_editor_state(indoc! {"
        «aaaBbbˇ» «bbbCccˇ» «cccDddˇ»
    "});

    cx.set_state(indoc! {"
        «hElLo, WoRld!ˇ»
    "});
    cx.update_editor(|e, window, cx| {
        e.convert_to_opposite_case(&ConvertToOppositeCase, window, cx)
    });
    cx.assert_editor_state(indoc! {"
        «HeLlO, wOrLD!ˇ»
    "});
}

#[gpui::test]
fn test_duplicate_line(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        build_editor(buffer, window, cx)
    });
    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 1),
                DisplayPoint::new(DisplayRow(0), 2)..DisplayPoint::new(DisplayRow(0), 2),
                DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 0),
                DisplayPoint::new(DisplayRow(3), 0)..DisplayPoint::new(DisplayRow(3), 0),
            ])
        });
        editor.duplicate_line_down(&DuplicateLineDown, window, cx);
        assert_eq!(editor.display_text(cx), "abc\nabc\ndef\ndef\nghi\n\n");
        assert_eq!(
            editor.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 1),
                DisplayPoint::new(DisplayRow(1), 2)..DisplayPoint::new(DisplayRow(1), 2),
                DisplayPoint::new(DisplayRow(3), 0)..DisplayPoint::new(DisplayRow(3), 0),
                DisplayPoint::new(DisplayRow(6), 0)..DisplayPoint::new(DisplayRow(6), 0),
            ]
        );
    });

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        build_editor(buffer, window, cx)
    });
    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(1), 1),
                DisplayPoint::new(DisplayRow(1), 2)..DisplayPoint::new(DisplayRow(2), 1),
            ])
        });
        editor.duplicate_line_down(&DuplicateLineDown, window, cx);
        assert_eq!(editor.display_text(cx), "abc\ndef\nghi\nabc\ndef\nghi\n");
        assert_eq!(
            editor.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(DisplayRow(3), 1)..DisplayPoint::new(DisplayRow(4), 1),
                DisplayPoint::new(DisplayRow(4), 2)..DisplayPoint::new(DisplayRow(5), 1),
            ]
        );
    });

    // With `move_upwards` the selections stay in place, except for
    // the lines inserted above them
    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        build_editor(buffer, window, cx)
    });
    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 1),
                DisplayPoint::new(DisplayRow(0), 2)..DisplayPoint::new(DisplayRow(0), 2),
                DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 0),
                DisplayPoint::new(DisplayRow(3), 0)..DisplayPoint::new(DisplayRow(3), 0),
            ])
        });
        editor.duplicate_line_up(&DuplicateLineUp, window, cx);
        assert_eq!(editor.display_text(cx), "abc\nabc\ndef\ndef\nghi\n\n");
        assert_eq!(
            editor.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 1),
                DisplayPoint::new(DisplayRow(0), 2)..DisplayPoint::new(DisplayRow(0), 2),
                DisplayPoint::new(DisplayRow(2), 0)..DisplayPoint::new(DisplayRow(2), 0),
                DisplayPoint::new(DisplayRow(6), 0)..DisplayPoint::new(DisplayRow(6), 0),
            ]
        );
    });

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        build_editor(buffer, window, cx)
    });
    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(1), 1),
                DisplayPoint::new(DisplayRow(1), 2)..DisplayPoint::new(DisplayRow(2), 1),
            ])
        });
        editor.duplicate_line_up(&DuplicateLineUp, window, cx);
        assert_eq!(editor.display_text(cx), "abc\ndef\nghi\nabc\ndef\nghi\n");
        assert_eq!(
            editor.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(1), 1),
                DisplayPoint::new(DisplayRow(1), 2)..DisplayPoint::new(DisplayRow(2), 1),
            ]
        );
    });

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("abc\ndef\nghi\n", cx);
        build_editor(buffer, window, cx)
    });
    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(1), 1),
                DisplayPoint::new(DisplayRow(1), 2)..DisplayPoint::new(DisplayRow(2), 1),
            ])
        });
        editor.duplicate_selection(&DuplicateSelection, window, cx);
        assert_eq!(editor.display_text(cx), "abc\ndbc\ndef\ngf\nghi\n");
        assert_eq!(
            editor.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(1), 1),
                DisplayPoint::new(DisplayRow(2), 2)..DisplayPoint::new(DisplayRow(3), 1),
            ]
        );
    });
}

#[gpui::test]
fn test_move_line_up_down(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple(&sample_text(10, 5, 'a'), cx);
        build_editor(buffer, window, cx)
    });
    _ = editor.update(cx, |editor, window, cx| {
        editor.fold_creases(
            vec![
                Crease::simple(Point::new(0, 2)..Point::new(1, 2), FoldPlaceholder::test()),
                Crease::simple(Point::new(2, 3)..Point::new(4, 1), FoldPlaceholder::test()),
                Crease::simple(Point::new(7, 0)..Point::new(8, 4), FoldPlaceholder::test()),
            ],
            true,
            window,
            cx,
        );
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(0), 1),
                DisplayPoint::new(DisplayRow(3), 1)..DisplayPoint::new(DisplayRow(3), 1),
                DisplayPoint::new(DisplayRow(3), 2)..DisplayPoint::new(DisplayRow(4), 3),
                DisplayPoint::new(DisplayRow(5), 0)..DisplayPoint::new(DisplayRow(5), 2),
            ])
        });
        assert_eq!(
            editor.display_text(cx),
            "aa⋯bbb\nccc⋯eeee\nfffff\nggggg\n⋯i\njjjjj"
        );

        editor.move_line_up(&MoveLineUp, window, cx);
        assert_eq!(
            editor.display_text(cx),
            "aa⋯bbb\nccc⋯eeee\nggggg\n⋯i\njjjjj\nfffff"
        );
        assert_eq!(
            editor.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(0), 1),
                DisplayPoint::new(DisplayRow(2), 1)..DisplayPoint::new(DisplayRow(2), 1),
                DisplayPoint::new(DisplayRow(2), 2)..DisplayPoint::new(DisplayRow(3), 3),
                DisplayPoint::new(DisplayRow(4), 0)..DisplayPoint::new(DisplayRow(4), 2)
            ]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.move_line_down(&MoveLineDown, window, cx);
        assert_eq!(
            editor.display_text(cx),
            "ccc⋯eeee\naa⋯bbb\nfffff\nggggg\n⋯i\njjjjj"
        );
        assert_eq!(
            editor.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(DisplayRow(1), 1)..DisplayPoint::new(DisplayRow(1), 1),
                DisplayPoint::new(DisplayRow(3), 1)..DisplayPoint::new(DisplayRow(3), 1),
                DisplayPoint::new(DisplayRow(3), 2)..DisplayPoint::new(DisplayRow(4), 3),
                DisplayPoint::new(DisplayRow(5), 0)..DisplayPoint::new(DisplayRow(5), 2)
            ]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.move_line_down(&MoveLineDown, window, cx);
        assert_eq!(
            editor.display_text(cx),
            "ccc⋯eeee\nfffff\naa⋯bbb\nggggg\n⋯i\njjjjj"
        );
        assert_eq!(
            editor.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(DisplayRow(2), 1)..DisplayPoint::new(DisplayRow(2), 1),
                DisplayPoint::new(DisplayRow(3), 1)..DisplayPoint::new(DisplayRow(3), 1),
                DisplayPoint::new(DisplayRow(3), 2)..DisplayPoint::new(DisplayRow(4), 3),
                DisplayPoint::new(DisplayRow(5), 0)..DisplayPoint::new(DisplayRow(5), 2)
            ]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.move_line_up(&MoveLineUp, window, cx);
        assert_eq!(
            editor.display_text(cx),
            "ccc⋯eeee\naa⋯bbb\nggggg\n⋯i\njjjjj\nfffff"
        );
        assert_eq!(
            editor.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(DisplayRow(1), 1)..DisplayPoint::new(DisplayRow(1), 1),
                DisplayPoint::new(DisplayRow(2), 1)..DisplayPoint::new(DisplayRow(2), 1),
                DisplayPoint::new(DisplayRow(2), 2)..DisplayPoint::new(DisplayRow(3), 3),
                DisplayPoint::new(DisplayRow(4), 0)..DisplayPoint::new(DisplayRow(4), 2)
            ]
        );
    });
}

#[gpui::test]
fn test_move_line_up_down_with_blocks(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple(&sample_text(10, 5, 'a'), cx);
        build_editor(buffer, window, cx)
    });
    _ = editor.update(cx, |editor, window, cx| {
        let snapshot = editor.buffer.read(cx).snapshot(cx);
        editor.insert_blocks(
            [BlockProperties {
                style: BlockStyle::Fixed,
                placement: BlockPlacement::Below(snapshot.anchor_after(Point::new(2, 0))),
                height: Some(1),
                render: Arc::new(|_| div().into_any()),
                priority: 0,
                render_in_minimap: true,
            }],
            Some(Autoscroll::fit()),
            cx,
        );
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(2, 0)..Point::new(2, 0)])
        });
        editor.move_line_down(&MoveLineDown, window, cx);
    });
}

#[gpui::test]
async fn test_selections_and_replace_blocks(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state(
        &"
            ˇzero
            one
            two
            three
            four
            five
        "
        .unindent(),
    );

    // Create a four-line block that replaces three lines of text.
    cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let snapshot = &snapshot.buffer_snapshot;
        let placement = BlockPlacement::Replace(
            snapshot.anchor_after(Point::new(1, 0))..=snapshot.anchor_after(Point::new(3, 0)),
        );
        editor.insert_blocks(
            [BlockProperties {
                placement,
                height: Some(4),
                style: BlockStyle::Sticky,
                render: Arc::new(|_| gpui::div().into_any_element()),
                priority: 0,
                render_in_minimap: true,
            }],
            None,
            cx,
        );
    });

    // Move down so that the cursor touches the block.
    cx.update_editor(|editor, window, cx| {
        editor.move_down(&Default::default(), window, cx);
    });
    cx.assert_editor_state(
        &"
            zero
            «one
            two
            threeˇ»
            four
            five
        "
        .unindent(),
    );

    // Move down past the block.
    cx.update_editor(|editor, window, cx| {
        editor.move_down(&Default::default(), window, cx);
    });
    cx.assert_editor_state(
        &"
            zero
            one
            two
            three
            ˇfour
            five
        "
        .unindent(),
    );
}

#[gpui::test]
fn test_transpose(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    _ = cx.add_window(|window, cx| {
        let mut editor = build_editor(MultiBuffer::build_simple("abc", cx), window, cx);
        editor.set_style(EditorStyle::default(), window, cx);
        editor.change_selections(None, window, cx, |s| s.select_ranges([1..1]));
        editor.transpose(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "bac");
        assert_eq!(editor.selections.ranges(cx), [2..2]);

        editor.transpose(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "bca");
        assert_eq!(editor.selections.ranges(cx), [3..3]);

        editor.transpose(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "bac");
        assert_eq!(editor.selections.ranges(cx), [3..3]);

        editor
    });

    _ = cx.add_window(|window, cx| {
        let mut editor = build_editor(MultiBuffer::build_simple("abc\nde", cx), window, cx);
        editor.set_style(EditorStyle::default(), window, cx);
        editor.change_selections(None, window, cx, |s| s.select_ranges([3..3]));
        editor.transpose(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "acb\nde");
        assert_eq!(editor.selections.ranges(cx), [3..3]);

        editor.change_selections(None, window, cx, |s| s.select_ranges([4..4]));
        editor.transpose(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "acbd\ne");
        assert_eq!(editor.selections.ranges(cx), [5..5]);

        editor.transpose(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "acbde\n");
        assert_eq!(editor.selections.ranges(cx), [6..6]);

        editor.transpose(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "acbd\ne");
        assert_eq!(editor.selections.ranges(cx), [6..6]);

        editor
    });

    _ = cx.add_window(|window, cx| {
        let mut editor = build_editor(MultiBuffer::build_simple("abc\nde", cx), window, cx);
        editor.set_style(EditorStyle::default(), window, cx);
        editor.change_selections(None, window, cx, |s| s.select_ranges([1..1, 2..2, 4..4]));
        editor.transpose(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "bacd\ne");
        assert_eq!(editor.selections.ranges(cx), [2..2, 3..3, 5..5]);

        editor.transpose(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "bcade\n");
        assert_eq!(editor.selections.ranges(cx), [3..3, 4..4, 6..6]);

        editor.transpose(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "bcda\ne");
        assert_eq!(editor.selections.ranges(cx), [4..4, 6..6]);

        editor.transpose(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "bcade\n");
        assert_eq!(editor.selections.ranges(cx), [4..4, 6..6]);

        editor.transpose(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "bcaed\n");
        assert_eq!(editor.selections.ranges(cx), [5..5, 6..6]);

        editor
    });

    _ = cx.add_window(|window, cx| {
        let mut editor = build_editor(MultiBuffer::build_simple("🍐🏀✋", cx), window, cx);
        editor.set_style(EditorStyle::default(), window, cx);
        editor.change_selections(None, window, cx, |s| s.select_ranges([4..4]));
        editor.transpose(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "🏀🍐✋");
        assert_eq!(editor.selections.ranges(cx), [8..8]);

        editor.transpose(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "🏀✋🍐");
        assert_eq!(editor.selections.ranges(cx), [11..11]);

        editor.transpose(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "🏀🍐✋");
        assert_eq!(editor.selections.ranges(cx), [11..11]);

        editor
    });
}

#[gpui::test]
async fn test_rewrap(cx: &mut TestAppContext) {
    init_test(cx, |settings| {
        settings.languages.extend([
            (
                "Markdown".into(),
                LanguageSettingsContent {
                    allow_rewrap: Some(language_settings::RewrapBehavior::Anywhere),
                    ..Default::default()
                },
            ),
            (
                "Plain Text".into(),
                LanguageSettingsContent {
                    allow_rewrap: Some(language_settings::RewrapBehavior::Anywhere),
                    ..Default::default()
                },
            ),
        ])
    });

    let mut cx = EditorTestContext::new(cx).await;

    let language_with_c_comments = Arc::new(Language::new(
        LanguageConfig {
            line_comments: vec!["// ".into()],
            ..LanguageConfig::default()
        },
        None,
    ));
    let language_with_pound_comments = Arc::new(Language::new(
        LanguageConfig {
            line_comments: vec!["# ".into()],
            ..LanguageConfig::default()
        },
        None,
    ));
    let markdown_language = Arc::new(Language::new(
        LanguageConfig {
            name: "Markdown".into(),
            ..LanguageConfig::default()
        },
        None,
    ));
    let language_with_doc_comments = Arc::new(Language::new(
        LanguageConfig {
            line_comments: vec!["// ".into(), "/// ".into()],
            ..LanguageConfig::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    ));

    let plaintext_language = Arc::new(Language::new(
        LanguageConfig {
            name: "Plain Text".into(),
            ..LanguageConfig::default()
        },
        None,
    ));

    assert_rewrap(
        indoc! {"
            // ˇLorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus mollis elit purus, a ornare lacus gravida vitae. Proin consectetur felis vel purus auctor, eu lacinia sapien scelerisque. Vivamus sit amet neque et quam tincidunt hendrerit. Praesent semper egestas tellus id dignissim. Pellentesque odio lectus, iaculis ac volutpat et, blandit quis urna. Sed vestibulum nisi sit amet nisl venenatis tempus. Donec molestie blandit quam, et porta nunc laoreet in. Integer sit amet scelerisque nisi. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras egestas porta metus, eu viverra ipsum efficitur quis. Donec luctus eros turpis, id vulputate turpis porttitor id. Aliquam id accumsan eros.
        "},
        indoc! {"
            // ˇLorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus mollis elit
            // purus, a ornare lacus gravida vitae. Proin consectetur felis vel purus
            // auctor, eu lacinia sapien scelerisque. Vivamus sit amet neque et quam
            // tincidunt hendrerit. Praesent semper egestas tellus id dignissim.
            // Pellentesque odio lectus, iaculis ac volutpat et, blandit quis urna. Sed
            // vestibulum nisi sit amet nisl venenatis tempus. Donec molestie blandit quam,
            // et porta nunc laoreet in. Integer sit amet scelerisque nisi. Lorem ipsum
            // dolor sit amet, consectetur adipiscing elit. Cras egestas porta metus, eu
            // viverra ipsum efficitur quis. Donec luctus eros turpis, id vulputate turpis
            // porttitor id. Aliquam id accumsan eros.
        "},
        language_with_c_comments.clone(),
        &mut cx,
    );

    // Test that rewrapping works inside of a selection
    assert_rewrap(
        indoc! {"
            «// Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus mollis elit purus, a ornare lacus gravida vitae. Proin consectetur felis vel purus auctor, eu lacinia sapien scelerisque. Vivamus sit amet neque et quam tincidunt hendrerit. Praesent semper egestas tellus id dignissim. Pellentesque odio lectus, iaculis ac volutpat et, blandit quis urna. Sed vestibulum nisi sit amet nisl venenatis tempus. Donec molestie blandit quam, et porta nunc laoreet in. Integer sit amet scelerisque nisi. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras egestas porta metus, eu viverra ipsum efficitur quis. Donec luctus eros turpis, id vulputate turpis porttitor id. Aliquam id accumsan eros.ˇ»
        "},
        indoc! {"
            «// Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus mollis elit
            // purus, a ornare lacus gravida vitae. Proin consectetur felis vel purus
            // auctor, eu lacinia sapien scelerisque. Vivamus sit amet neque et quam
            // tincidunt hendrerit. Praesent semper egestas tellus id dignissim.
            // Pellentesque odio lectus, iaculis ac volutpat et, blandit quis urna. Sed
            // vestibulum nisi sit amet nisl venenatis tempus. Donec molestie blandit quam,
            // et porta nunc laoreet in. Integer sit amet scelerisque nisi. Lorem ipsum
            // dolor sit amet, consectetur adipiscing elit. Cras egestas porta metus, eu
            // viverra ipsum efficitur quis. Donec luctus eros turpis, id vulputate turpis
            // porttitor id. Aliquam id accumsan eros.ˇ»
        "},
        language_with_c_comments.clone(),
        &mut cx,
    );

    // Test that cursors that expand to the same region are collapsed.
    assert_rewrap(
        indoc! {"
            // ˇLorem ipsum dolor sit amet, consectetur adipiscing elit.
            // ˇVivamus mollis elit purus, a ornare lacus gravida vitae. Proin consectetur felis vel purus auctor, eu lacinia sapien scelerisque.
            // ˇVivamus sit amet neque et quam tincidunt hendrerit. Praesent semper egestas tellus id dignissim. Pellentesque odio lectus, iaculis ac volutpat et,
            // ˇblandit quis urna. Sed vestibulum nisi sit amet nisl venenatis tempus. Donec molestie blandit quam, et porta nunc laoreet in. Integer sit amet scelerisque nisi. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras egestas porta metus, eu viverra ipsum efficitur quis. Donec luctus eros turpis, id vulputate turpis porttitor id. Aliquam id accumsan eros.
        "},
        indoc! {"
            // ˇLorem ipsum dolor sit amet, consectetur adipiscing elit. ˇVivamus mollis elit
            // purus, a ornare lacus gravida vitae. Proin consectetur felis vel purus
            // auctor, eu lacinia sapien scelerisque. ˇVivamus sit amet neque et quam
            // tincidunt hendrerit. Praesent semper egestas tellus id dignissim.
            // Pellentesque odio lectus, iaculis ac volutpat et, ˇblandit quis urna. Sed
            // vestibulum nisi sit amet nisl venenatis tempus. Donec molestie blandit quam,
            // et porta nunc laoreet in. Integer sit amet scelerisque nisi. Lorem ipsum
            // dolor sit amet, consectetur adipiscing elit. Cras egestas porta metus, eu
            // viverra ipsum efficitur quis. Donec luctus eros turpis, id vulputate turpis
            // porttitor id. Aliquam id accumsan eros.
        "},
        language_with_c_comments.clone(),
        &mut cx,
    );

    // Test that non-contiguous selections are treated separately.
    assert_rewrap(
        indoc! {"
            // ˇLorem ipsum dolor sit amet, consectetur adipiscing elit.
            // ˇVivamus mollis elit purus, a ornare lacus gravida vitae. Proin consectetur felis vel purus auctor, eu lacinia sapien scelerisque.
            //
            // ˇVivamus sit amet neque et quam tincidunt hendrerit. Praesent semper egestas tellus id dignissim. Pellentesque odio lectus, iaculis ac volutpat et,
            // ˇblandit quis urna. Sed vestibulum nisi sit amet nisl venenatis tempus. Donec molestie blandit quam, et porta nunc laoreet in. Integer sit amet scelerisque nisi. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras egestas porta metus, eu viverra ipsum efficitur quis. Donec luctus eros turpis, id vulputate turpis porttitor id. Aliquam id accumsan eros.
        "},
        indoc! {"
            // ˇLorem ipsum dolor sit amet, consectetur adipiscing elit. ˇVivamus mollis elit
            // purus, a ornare lacus gravida vitae. Proin consectetur felis vel purus
            // auctor, eu lacinia sapien scelerisque.
            //
            // ˇVivamus sit amet neque et quam tincidunt hendrerit. Praesent semper egestas
            // tellus id dignissim. Pellentesque odio lectus, iaculis ac volutpat et,
            // ˇblandit quis urna. Sed vestibulum nisi sit amet nisl venenatis tempus. Donec
            // molestie blandit quam, et porta nunc laoreet in. Integer sit amet scelerisque
            // nisi. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras egestas
            // porta metus, eu viverra ipsum efficitur quis. Donec luctus eros turpis, id
            // vulputate turpis porttitor id. Aliquam id accumsan eros.
        "},
        language_with_c_comments.clone(),
        &mut cx,
    );

    // Test that different comment prefixes are supported.
    assert_rewrap(
        indoc! {"
            # ˇLorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus mollis elit purus, a ornare lacus gravida vitae. Proin consectetur felis vel purus auctor, eu lacinia sapien scelerisque. Vivamus sit amet neque et quam tincidunt hendrerit. Praesent semper egestas tellus id dignissim. Pellentesque odio lectus, iaculis ac volutpat et, blandit quis urna. Sed vestibulum nisi sit amet nisl venenatis tempus. Donec molestie blandit quam, et porta nunc laoreet in. Integer sit amet scelerisque nisi. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras egestas porta metus, eu viverra ipsum efficitur quis. Donec luctus eros turpis, id vulputate turpis porttitor id. Aliquam id accumsan eros.
        "},
        indoc! {"
            # ˇLorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus mollis elit
            # purus, a ornare lacus gravida vitae. Proin consectetur felis vel purus auctor,
            # eu lacinia sapien scelerisque. Vivamus sit amet neque et quam tincidunt
            # hendrerit. Praesent semper egestas tellus id dignissim. Pellentesque odio
            # lectus, iaculis ac volutpat et, blandit quis urna. Sed vestibulum nisi sit
            # amet nisl venenatis tempus. Donec molestie blandit quam, et porta nunc laoreet
            # in. Integer sit amet scelerisque nisi. Lorem ipsum dolor sit amet, consectetur
            # adipiscing elit. Cras egestas porta metus, eu viverra ipsum efficitur quis.
            # Donec luctus eros turpis, id vulputate turpis porttitor id. Aliquam id
            # accumsan eros.
        "},
        language_with_pound_comments.clone(),
        &mut cx,
    );

    // Test that rewrapping is ignored outside of comments in most languages.
    assert_rewrap(
        indoc! {"
            /// Adds two numbers.
            /// Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus mollis elit purus, a ornare lacus gravida vitae.ˇ
            fn add(a: u32, b: u32) -> u32 {
                a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + bˇ
            }
        "},
        indoc! {"
            /// Adds two numbers. Lorem ipsum dolor sit amet, consectetur adipiscing elit.
            /// Vivamus mollis elit purus, a ornare lacus gravida vitae.ˇ
            fn add(a: u32, b: u32) -> u32 {
                a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + b + a + bˇ
            }
        "},
        language_with_doc_comments.clone(),
        &mut cx,
    );

    // Test that rewrapping works in Markdown and Plain Text languages.
    assert_rewrap(
        indoc! {"
            # Hello

            Lorem ipsum dolor sit amet, ˇconsectetur adipiscing elit. Vivamus mollis elit purus, a ornare lacus gravida vitae. Proin consectetur felis vel purus auctor, eu lacinia sapien scelerisque. Vivamus sit amet neque et quam tincidunt hendrerit. Praesent semper egestas tellus id dignissim. Pellentesque odio lectus, iaculis ac volutpat et, blandit quis urna. Sed vestibulum nisi sit amet nisl venenatis tempus. Donec molestie blandit quam, et porta nunc laoreet in. Integer sit amet scelerisque nisi.
        "},
        indoc! {"
            # Hello

            Lorem ipsum dolor sit amet, ˇconsectetur adipiscing elit. Vivamus mollis elit
            purus, a ornare lacus gravida vitae. Proin consectetur felis vel purus auctor,
            eu lacinia sapien scelerisque. Vivamus sit amet neque et quam tincidunt
            hendrerit. Praesent semper egestas tellus id dignissim. Pellentesque odio
            lectus, iaculis ac volutpat et, blandit quis urna. Sed vestibulum nisi sit amet
            nisl venenatis tempus. Donec molestie blandit quam, et porta nunc laoreet in.
            Integer sit amet scelerisque nisi.
        "},
        markdown_language,
        &mut cx,
    );

    assert_rewrap(
        indoc! {"
            Lorem ipsum dolor sit amet, ˇconsectetur adipiscing elit. Vivamus mollis elit purus, a ornare lacus gravida vitae. Proin consectetur felis vel purus auctor, eu lacinia sapien scelerisque. Vivamus sit amet neque et quam tincidunt hendrerit. Praesent semper egestas tellus id dignissim. Pellentesque odio lectus, iaculis ac volutpat et, blandit quis urna. Sed vestibulum nisi sit amet nisl venenatis tempus. Donec molestie blandit quam, et porta nunc laoreet in. Integer sit amet scelerisque nisi.
        "},
        indoc! {"
            Lorem ipsum dolor sit amet, ˇconsectetur adipiscing elit. Vivamus mollis elit
            purus, a ornare lacus gravida vitae. Proin consectetur felis vel purus auctor,
            eu lacinia sapien scelerisque. Vivamus sit amet neque et quam tincidunt
            hendrerit. Praesent semper egestas tellus id dignissim. Pellentesque odio
            lectus, iaculis ac volutpat et, blandit quis urna. Sed vestibulum nisi sit amet
            nisl venenatis tempus. Donec molestie blandit quam, et porta nunc laoreet in.
            Integer sit amet scelerisque nisi.
        "},
        plaintext_language,
        &mut cx,
    );

    // Test rewrapping unaligned comments in a selection.
    assert_rewrap(
        indoc! {"
            fn foo() {
                if true {
            «        // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus mollis elit purus, a ornare lacus gravida vitae.
            // Praesent semper egestas tellus id dignissim.ˇ»
                    do_something();
                } else {
                    //
                }
            }
        "},
        indoc! {"
            fn foo() {
                if true {
            «        // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus
                    // mollis elit purus, a ornare lacus gravida vitae. Praesent semper
                    // egestas tellus id dignissim.ˇ»
                    do_something();
                } else {
                    //
                }
            }
        "},
        language_with_doc_comments.clone(),
        &mut cx,
    );

    assert_rewrap(
        indoc! {"
            fn foo() {
                if true {
            «ˇ        // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus mollis elit purus, a ornare lacus gravida vitae.
            // Praesent semper egestas tellus id dignissim.»
                    do_something();
                } else {
                    //
                }

            }
        "},
        indoc! {"
            fn foo() {
                if true {
            «ˇ        // Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus
                    // mollis elit purus, a ornare lacus gravida vitae. Praesent semper
                    // egestas tellus id dignissim.»
                    do_something();
                } else {
                    //
                }

            }
        "},
        language_with_doc_comments.clone(),
        &mut cx,
    );

    #[track_caller]
    fn assert_rewrap(
        unwrapped_text: &str,
        wrapped_text: &str,
        language: Arc<Language>,
        cx: &mut EditorTestContext,
    ) {
        cx.update_buffer(|buffer, cx| buffer.set_language(Some(language), cx));
        cx.set_state(unwrapped_text);
        cx.update_editor(|e, window, cx| e.rewrap(&Rewrap, window, cx));
        cx.assert_editor_state(wrapped_text);
    }
}

#[gpui::test]
async fn test_hard_wrap(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;

    cx.update_buffer(|buffer, cx| buffer.set_language(Some(git_commit_lang()), cx));
    cx.update_editor(|editor, _, cx| {
        editor.set_hard_wrap(Some(14), cx);
    });

    cx.set_state(indoc!(
        "
        one two three ˇ
        "
    ));
    cx.simulate_input("four");
    cx.run_until_parked();

    cx.assert_editor_state(indoc!(
        "
        one two three
        fourˇ
        "
    ));

    cx.update_editor(|editor, window, cx| {
        editor.newline(&Default::default(), window, cx);
    });
    cx.run_until_parked();
    cx.assert_editor_state(indoc!(
        "
        one two three
        four
        ˇ
        "
    ));

    cx.simulate_input("five");
    cx.run_until_parked();
    cx.assert_editor_state(indoc!(
        "
        one two three
        four
        fiveˇ
        "
    ));

    cx.update_editor(|editor, window, cx| {
        editor.newline(&Default::default(), window, cx);
    });
    cx.run_until_parked();
    cx.simulate_input("# ");
    cx.run_until_parked();
    cx.assert_editor_state(indoc!(
        "
        one two three
        four
        five
        # ˇ
        "
    ));

    cx.update_editor(|editor, window, cx| {
        editor.newline(&Default::default(), window, cx);
    });
    cx.run_until_parked();
    cx.assert_editor_state(indoc!(
        "
        one two three
        four
        five
        #\x20
        #ˇ
        "
    ));

    cx.simulate_input(" 6");
    cx.run_until_parked();
    cx.assert_editor_state(indoc!(
        "
        one two three
        four
        five
        #
        # 6ˇ
        "
    ));
}

#[gpui::test]
async fn test_clipboard(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    cx.set_state("«one✅ ˇ»two «three ˇ»four «five ˇ»six ");
    cx.update_editor(|e, window, cx| e.cut(&Cut, window, cx));
    cx.assert_editor_state("ˇtwo ˇfour ˇsix ");

    // Paste with three cursors. Each cursor pastes one slice of the clipboard text.
    cx.set_state("two ˇfour ˇsix ˇ");
    cx.update_editor(|e, window, cx| e.paste(&Paste, window, cx));
    cx.assert_editor_state("two one✅ ˇfour three ˇsix five ˇ");

    // Paste again but with only two cursors. Since the number of cursors doesn't
    // match the number of slices in the clipboard, the entire clipboard text
    // is pasted at each cursor.
    cx.set_state("ˇtwo one✅ four three six five ˇ");
    cx.update_editor(|e, window, cx| {
        e.handle_input("( ", window, cx);
        e.paste(&Paste, window, cx);
        e.handle_input(") ", window, cx);
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
    cx.update_editor(|e, window, cx| e.cut(&Cut, window, cx));
    cx.assert_editor_state(indoc! {"
        1ˇ3
        ˇ9"});

    // Paste with three selections, noticing how the copied selection that was full-line
    // gets inserted before the second cursor.
    cx.set_state(indoc! {"
        1ˇ3
        9ˇ
        «oˇ»ne"});
    cx.update_editor(|e, window, cx| e.paste(&Paste, window, cx));
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
    cx.update_editor(|e, window, cx| e.copy(&Copy, window, cx));
    assert_eq!(
        cx.read_from_clipboard()
            .and_then(|item| item.text().as_deref().map(str::to_string)),
        Some("fox jumps over\n".to_string())
    );

    // Paste with three selections, noticing how the copied full-line selection is inserted
    // before the empty selections but replaces the selection that is non-empty.
    cx.set_state(indoc! {"
        Tˇhe quick brown
        «foˇ»x jumps over
        tˇhe lazy dog"});
    cx.update_editor(|e, window, cx| e.paste(&Paste, window, cx));
    cx.assert_editor_state(indoc! {"
        fox jumps over
        Tˇhe quick brown
        fox jumps over
        ˇx jumps over
        fox jumps over
        tˇhe lazy dog"});
}

#[gpui::test]
async fn test_copy_trim(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state(
        r#"            «for selection in selections.iter() {
            let mut start = selection.start;
            let mut end = selection.end;
            let is_entire_line = selection.is_empty();
            if is_entire_line {
                start = Point::new(start.row, 0);ˇ»
                end = cmp::min(max_point, Point::new(end.row + 1, 0));
            }
        "#,
    );
    cx.update_editor(|e, window, cx| e.copy(&Copy, window, cx));
    assert_eq!(
        cx.read_from_clipboard()
            .and_then(|item| item.text().as_deref().map(str::to_string)),
        Some(
            "for selection in selections.iter() {
            let mut start = selection.start;
            let mut end = selection.end;
            let is_entire_line = selection.is_empty();
            if is_entire_line {
                start = Point::new(start.row, 0);"
                .to_string()
        ),
        "Regular copying preserves all indentation selected",
    );
    cx.update_editor(|e, window, cx| e.copy_and_trim(&CopyAndTrim, window, cx));
    assert_eq!(
        cx.read_from_clipboard()
            .and_then(|item| item.text().as_deref().map(str::to_string)),
        Some(
            "for selection in selections.iter() {
let mut start = selection.start;
let mut end = selection.end;
let is_entire_line = selection.is_empty();
if is_entire_line {
    start = Point::new(start.row, 0);"
                .to_string()
        ),
        "Copying with stripping should strip all leading whitespaces"
    );

    cx.set_state(
        r#"       «     for selection in selections.iter() {
            let mut start = selection.start;
            let mut end = selection.end;
            let is_entire_line = selection.is_empty();
            if is_entire_line {
                start = Point::new(start.row, 0);ˇ»
                end = cmp::min(max_point, Point::new(end.row + 1, 0));
            }
        "#,
    );
    cx.update_editor(|e, window, cx| e.copy(&Copy, window, cx));
    assert_eq!(
        cx.read_from_clipboard()
            .and_then(|item| item.text().as_deref().map(str::to_string)),
        Some(
            "     for selection in selections.iter() {
            let mut start = selection.start;
            let mut end = selection.end;
            let is_entire_line = selection.is_empty();
            if is_entire_line {
                start = Point::new(start.row, 0);"
                .to_string()
        ),
        "Regular copying preserves all indentation selected",
    );
    cx.update_editor(|e, window, cx| e.copy_and_trim(&CopyAndTrim, window, cx));
    assert_eq!(
        cx.read_from_clipboard()
            .and_then(|item| item.text().as_deref().map(str::to_string)),
        Some(
            "for selection in selections.iter() {
let mut start = selection.start;
let mut end = selection.end;
let is_entire_line = selection.is_empty();
if is_entire_line {
    start = Point::new(start.row, 0);"
                .to_string()
        ),
        "Copying with stripping should strip all leading whitespaces, even if some of it was selected"
    );

    cx.set_state(
        r#"       «ˇ     for selection in selections.iter() {
            let mut start = selection.start;
            let mut end = selection.end;
            let is_entire_line = selection.is_empty();
            if is_entire_line {
                start = Point::new(start.row, 0);»
                end = cmp::min(max_point, Point::new(end.row + 1, 0));
            }
        "#,
    );
    cx.update_editor(|e, window, cx| e.copy(&Copy, window, cx));
    assert_eq!(
        cx.read_from_clipboard()
            .and_then(|item| item.text().as_deref().map(str::to_string)),
        Some(
            "     for selection in selections.iter() {
            let mut start = selection.start;
            let mut end = selection.end;
            let is_entire_line = selection.is_empty();
            if is_entire_line {
                start = Point::new(start.row, 0);"
                .to_string()
        ),
        "Regular copying for reverse selection works the same",
    );
    cx.update_editor(|e, window, cx| e.copy_and_trim(&CopyAndTrim, window, cx));
    assert_eq!(
        cx.read_from_clipboard()
            .and_then(|item| item.text().as_deref().map(str::to_string)),
        Some(
            "for selection in selections.iter() {
let mut start = selection.start;
let mut end = selection.end;
let is_entire_line = selection.is_empty();
if is_entire_line {
    start = Point::new(start.row, 0);"
                .to_string()
        ),
        "Copying with stripping for reverse selection works the same"
    );

    cx.set_state(
        r#"            for selection «in selections.iter() {
            let mut start = selection.start;
            let mut end = selection.end;
            let is_entire_line = selection.is_empty();
            if is_entire_line {
                start = Point::new(start.row, 0);ˇ»
                end = cmp::min(max_point, Point::new(end.row + 1, 0));
            }
        "#,
    );
    cx.update_editor(|e, window, cx| e.copy(&Copy, window, cx));
    assert_eq!(
        cx.read_from_clipboard()
            .and_then(|item| item.text().as_deref().map(str::to_string)),
        Some(
            "in selections.iter() {
            let mut start = selection.start;
            let mut end = selection.end;
            let is_entire_line = selection.is_empty();
            if is_entire_line {
                start = Point::new(start.row, 0);"
                .to_string()
        ),
        "When selecting past the indent, the copying works as usual",
    );
    cx.update_editor(|e, window, cx| e.copy_and_trim(&CopyAndTrim, window, cx));
    assert_eq!(
        cx.read_from_clipboard()
            .and_then(|item| item.text().as_deref().map(str::to_string)),
        Some(
            "in selections.iter() {
            let mut start = selection.start;
            let mut end = selection.end;
            let is_entire_line = selection.is_empty();
            if is_entire_line {
                start = Point::new(start.row, 0);"
                .to_string()
        ),
        "When selecting past the indent, nothing is trimmed"
    );

    cx.set_state(
        r#"            «for selection in selections.iter() {
            let mut start = selection.start;

            let mut end = selection.end;
            let is_entire_line = selection.is_empty();
            if is_entire_line {
                start = Point::new(start.row, 0);
ˇ»                end = cmp::min(max_point, Point::new(end.row + 1, 0));
            }
        "#,
    );
    cx.update_editor(|e, window, cx| e.copy_and_trim(&CopyAndTrim, window, cx));
    assert_eq!(
        cx.read_from_clipboard()
            .and_then(|item| item.text().as_deref().map(str::to_string)),
        Some(
            "for selection in selections.iter() {
let mut start = selection.start;

let mut end = selection.end;
let is_entire_line = selection.is_empty();
if is_entire_line {
    start = Point::new(start.row, 0);
"
            .to_string()
        ),
        "Copying with stripping should ignore empty lines"
    );
}

#[gpui::test]
async fn test_paste_multiline(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.update_buffer(|buffer, cx| buffer.set_language(Some(rust_lang()), cx));

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
    cx.update_editor(|e, window, cx| e.cut(&Cut, window, cx));
    cx.assert_editor_state(indoc! {"
        const a: B = (
            c(),
            ˇ
        );
    "});

    // Paste it at the same position.
    cx.update_editor(|e, window, cx| e.paste(&Paste, window, cx));
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
    cx.update_editor(|e, window, cx| e.paste(&Paste, window, cx));
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
    cx.update_editor(|e, window, cx| e.cut(&Cut, window, cx));
    cx.assert_editor_state(indoc! {"
        const a: B = (
            c(),
        ˇ);
    "});

    // Paste it at the same position.
    cx.update_editor(|e, window, cx| e.paste(&Paste, window, cx));
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
    cx.update_editor(|e, window, cx| e.paste(&Paste, window, cx));
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

    // Copy an indented block, starting mid-line
    cx.set_state(indoc! {"
        const a: B = (
            c(),
            somethin«g(
                e,
                f
            )ˇ»
        );
    "});
    cx.update_editor(|e, window, cx| e.copy(&Copy, window, cx));

    // Paste it on a line with a lower indent level
    cx.update_editor(|e, window, cx| e.move_to_end(&Default::default(), window, cx));
    cx.update_editor(|e, window, cx| e.paste(&Paste, window, cx));
    cx.assert_editor_state(indoc! {"
        const a: B = (
            c(),
            something(
                e,
                f
            )
        );
        g(
            e,
            f
        )ˇ"});
}

#[gpui::test]
async fn test_paste_content_from_other_app(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    cx.write_to_clipboard(ClipboardItem::new_string(
        "    d(\n        e\n    );\n".into(),
    ));

    let mut cx = EditorTestContext::new(cx).await;
    cx.update_buffer(|buffer, cx| buffer.set_language(Some(rust_lang()), cx));

    cx.set_state(indoc! {"
        fn a() {
            b();
            if c() {
                ˇ
            }
        }
    "});

    cx.update_editor(|e, window, cx| e.paste(&Paste, window, cx));
    cx.assert_editor_state(indoc! {"
        fn a() {
            b();
            if c() {
                d(
                    e
                );
        ˇ
            }
        }
    "});

    cx.set_state(indoc! {"
        fn a() {
            b();
            ˇ
        }
    "});

    cx.update_editor(|e, window, cx| e.paste(&Paste, window, cx));
    cx.assert_editor_state(indoc! {"
        fn a() {
            b();
            d(
                e
            );
        ˇ
        }
    "});
}

#[gpui::test]
fn test_select_all(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("abc\nde\nfgh", cx);
        build_editor(buffer, window, cx)
    });
    _ = editor.update(cx, |editor, window, cx| {
        editor.select_all(&SelectAll, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            &[DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(2), 3)]
        );
    });
}

#[gpui::test]
fn test_select_line(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple(&sample_text(6, 5, 'a'), cx);
        build_editor(buffer, window, cx)
    });
    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 1),
                DisplayPoint::new(DisplayRow(0), 2)..DisplayPoint::new(DisplayRow(0), 2),
                DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 0),
                DisplayPoint::new(DisplayRow(4), 2)..DisplayPoint::new(DisplayRow(4), 2),
            ])
        });
        editor.select_line(&SelectLine, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(2), 0),
                DisplayPoint::new(DisplayRow(4), 0)..DisplayPoint::new(DisplayRow(5), 0),
            ]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.select_line(&SelectLine, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            vec![
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(3), 0),
                DisplayPoint::new(DisplayRow(4), 0)..DisplayPoint::new(DisplayRow(5), 5),
            ]
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.select_line(&SelectLine, window, cx);
        assert_eq!(
            editor.selections.display_ranges(cx),
            vec![DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(5), 5)]
        );
    });
}

#[gpui::test]
async fn test_split_selection_into_lines(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;

    #[track_caller]
    fn test(cx: &mut EditorTestContext, initial_state: &'static str, expected_state: &'static str) {
        cx.set_state(initial_state);
        cx.update_editor(|e, window, cx| {
            e.split_selection_into_lines(&SplitSelectionIntoLines, window, cx)
        });
        cx.assert_editor_state(expected_state);
    }

    // Selection starts and ends at the middle of lines, left-to-right
    test(
        &mut cx,
        "aa\nb«ˇb\ncc\ndd\ne»e\nff",
        "aa\nbbˇ\nccˇ\nddˇ\neˇe\nff",
    );
    // Same thing, right-to-left
    test(
        &mut cx,
        "aa\nb«b\ncc\ndd\neˇ»e\nff",
        "aa\nbbˇ\nccˇ\nddˇ\neˇe\nff",
    );

    // Whole buffer, left-to-right, last line *doesn't* end with newline
    test(
        &mut cx,
        "«ˇaa\nbb\ncc\ndd\nee\nff»",
        "aaˇ\nbbˇ\nccˇ\nddˇ\neeˇ\nffˇ",
    );
    // Same thing, right-to-left
    test(
        &mut cx,
        "«aa\nbb\ncc\ndd\nee\nffˇ»",
        "aaˇ\nbbˇ\nccˇ\nddˇ\neeˇ\nffˇ",
    );

    // Whole buffer, left-to-right, last line ends with newline
    test(
        &mut cx,
        "«ˇaa\nbb\ncc\ndd\nee\nff\n»",
        "aaˇ\nbbˇ\nccˇ\nddˇ\neeˇ\nffˇ\n",
    );
    // Same thing, right-to-left
    test(
        &mut cx,
        "«aa\nbb\ncc\ndd\nee\nff\nˇ»",
        "aaˇ\nbbˇ\nccˇ\nddˇ\neeˇ\nffˇ\n",
    );

    // Starts at the end of a line, ends at the start of another
    test(
        &mut cx,
        "aa\nbb«ˇ\ncc\ndd\nee\n»ff\n",
        "aa\nbbˇ\nccˇ\nddˇ\neeˇ\nff\n",
    );
}

#[gpui::test]
async fn test_split_selection_into_lines_interacting_with_creases(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple(&sample_text(9, 5, 'a'), cx);
        build_editor(buffer, window, cx)
    });

    // setup
    _ = editor.update(cx, |editor, window, cx| {
        editor.fold_creases(
            vec![
                Crease::simple(Point::new(0, 2)..Point::new(1, 2), FoldPlaceholder::test()),
                Crease::simple(Point::new(2, 3)..Point::new(4, 1), FoldPlaceholder::test()),
                Crease::simple(Point::new(7, 0)..Point::new(8, 4), FoldPlaceholder::test()),
            ],
            true,
            window,
            cx,
        );
        assert_eq!(
            editor.display_text(cx),
            "aa⋯bbb\nccc⋯eeee\nfffff\nggggg\n⋯i"
        );
    });

    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 1),
                DisplayPoint::new(DisplayRow(0), 2)..DisplayPoint::new(DisplayRow(0), 2),
                DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 0),
                DisplayPoint::new(DisplayRow(4), 4)..DisplayPoint::new(DisplayRow(4), 4),
            ])
        });
        editor.split_selection_into_lines(&SplitSelectionIntoLines, window, cx);
        assert_eq!(
            editor.display_text(cx),
            "aaaaa\nbbbbb\nccc⋯eeee\nfffff\nggggg\n⋯i"
        );
    });
    EditorTestContext::for_editor(editor, cx)
        .await
        .assert_editor_state("aˇaˇaaa\nbbbbb\nˇccccc\nddddd\neeeee\nfffff\nggggg\nhhhhh\niiiiiˇ");

    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(5), 0)..DisplayPoint::new(DisplayRow(0), 1)
            ])
        });
        editor.split_selection_into_lines(&SplitSelectionIntoLines, window, cx);
        assert_eq!(
            editor.display_text(cx),
            "aaaaa\nbbbbb\nccccc\nddddd\neeeee\nfffff\nggggg\nhhhhh\niiiii"
        );
        assert_eq!(
            editor.selections.display_ranges(cx),
            [
                DisplayPoint::new(DisplayRow(0), 5)..DisplayPoint::new(DisplayRow(0), 5),
                DisplayPoint::new(DisplayRow(1), 5)..DisplayPoint::new(DisplayRow(1), 5),
                DisplayPoint::new(DisplayRow(2), 5)..DisplayPoint::new(DisplayRow(2), 5),
                DisplayPoint::new(DisplayRow(3), 5)..DisplayPoint::new(DisplayRow(3), 5),
                DisplayPoint::new(DisplayRow(4), 5)..DisplayPoint::new(DisplayRow(4), 5),
                DisplayPoint::new(DisplayRow(5), 5)..DisplayPoint::new(DisplayRow(5), 5),
                DisplayPoint::new(DisplayRow(6), 5)..DisplayPoint::new(DisplayRow(6), 5)
            ]
        );
    });
    EditorTestContext::for_editor(editor, cx)
        .await
        .assert_editor_state(
            "aaaaaˇ\nbbbbbˇ\ncccccˇ\ndddddˇ\neeeeeˇ\nfffffˇ\ngggggˇ\nhhhhh\niiiii",
        );
}

#[gpui::test]
async fn test_add_selection_above_below(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    cx.set_state(indoc!(
        r#"abc
           defˇghi

           jk
           nlmo
           "#
    ));

    cx.update_editor(|editor, window, cx| {
        editor.add_selection_above(&Default::default(), window, cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abcˇ
           defˇghi

           jk
           nlmo
           "#
    ));

    cx.update_editor(|editor, window, cx| {
        editor.add_selection_above(&Default::default(), window, cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abcˇ
            defˇghi

            jk
            nlmo
            "#
    ));

    cx.update_editor(|editor, window, cx| {
        editor.add_selection_below(&Default::default(), window, cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abc
           defˇghi

           jk
           nlmo
           "#
    ));

    cx.update_editor(|editor, window, cx| {
        editor.undo_selection(&Default::default(), window, cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abcˇ
           defˇghi

           jk
           nlmo
           "#
    ));

    cx.update_editor(|editor, window, cx| {
        editor.redo_selection(&Default::default(), window, cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abc
           defˇghi

           jk
           nlmo
           "#
    ));

    cx.update_editor(|editor, window, cx| {
        editor.add_selection_below(&Default::default(), window, cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abc
           defˇghi

           jk
           nlmˇo
           "#
    ));

    cx.update_editor(|editor, window, cx| {
        editor.add_selection_below(&Default::default(), window, cx);
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

    cx.update_editor(|editor, window, cx| {
        editor.add_selection_below(&Default::default(), window, cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abc
           def«ˇg»hi

           jk
           nlm«ˇo»
           "#
    ));

    cx.update_editor(|editor, window, cx| {
        editor.add_selection_below(&Default::default(), window, cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abc
           def«ˇg»hi

           jk
           nlm«ˇo»
           "#
    ));

    cx.update_editor(|editor, window, cx| {
        editor.add_selection_above(&Default::default(), window, cx);
    });

    cx.assert_editor_state(indoc!(
        r#"abc
           def«ˇg»hi

           jk
           nlmo
           "#
    ));

    cx.update_editor(|editor, window, cx| {
        editor.add_selection_above(&Default::default(), window, cx);
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

    cx.update_editor(|editor, window, cx| {
        editor.add_selection_below(&Default::default(), window, cx);
    });

    cx.assert_editor_state(indoc!(
        r#"a«bcˇ»
           d«efgˇ»hi

           j«kˇ»
           nlmo
           "#
    ));

    cx.update_editor(|editor, window, cx| {
        editor.add_selection_below(&Default::default(), window, cx);
    });
    cx.assert_editor_state(indoc!(
        r#"a«bcˇ»
           d«efgˇ»hi

           j«kˇ»
           n«lmoˇ»
           "#
    ));
    cx.update_editor(|editor, window, cx| {
        editor.add_selection_above(&Default::default(), window, cx);
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

    cx.update_editor(|editor, window, cx| {
        editor.add_selection_above(&Default::default(), window, cx);
    });

    cx.assert_editor_state(indoc!(
        r#"a«ˇbc»
           d«ˇef»ghi

           j«ˇk»
           n«ˇlm»o
           "#
    ));

    cx.update_editor(|editor, window, cx| {
        editor.add_selection_below(&Default::default(), window, cx);
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
async fn test_select_next(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state("abc\nˇabc abc\ndefabc\nabc");

    cx.update_editor(|e, window, cx| e.select_next(&SelectNext::default(), window, cx))
        .unwrap();
    cx.assert_editor_state("abc\n«abcˇ» abc\ndefabc\nabc");

    cx.update_editor(|e, window, cx| e.select_next(&SelectNext::default(), window, cx))
        .unwrap();
    cx.assert_editor_state("abc\n«abcˇ» «abcˇ»\ndefabc\nabc");

    cx.update_editor(|editor, window, cx| editor.undo_selection(&UndoSelection, window, cx));
    cx.assert_editor_state("abc\n«abcˇ» abc\ndefabc\nabc");

    cx.update_editor(|editor, window, cx| editor.redo_selection(&RedoSelection, window, cx));
    cx.assert_editor_state("abc\n«abcˇ» «abcˇ»\ndefabc\nabc");

    cx.update_editor(|e, window, cx| e.select_next(&SelectNext::default(), window, cx))
        .unwrap();
    cx.assert_editor_state("abc\n«abcˇ» «abcˇ»\ndefabc\n«abcˇ»");

    cx.update_editor(|e, window, cx| e.select_next(&SelectNext::default(), window, cx))
        .unwrap();
    cx.assert_editor_state("«abcˇ»\n«abcˇ» «abcˇ»\ndefabc\n«abcˇ»");

    // Test selection direction should be preserved
    cx.set_state("abc\n«ˇabc» abc\ndefabc\nabc");

    cx.update_editor(|e, window, cx| e.select_next(&SelectNext::default(), window, cx))
        .unwrap();
    cx.assert_editor_state("abc\n«ˇabc» «ˇabc»\ndefabc\nabc");
}

#[gpui::test]
async fn test_select_all_matches(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    // Test caret-only selections
    cx.set_state("abc\nˇabc abc\ndefabc\nabc");
    cx.update_editor(|e, window, cx| e.select_all_matches(&SelectAllMatches, window, cx))
        .unwrap();
    cx.assert_editor_state("«abcˇ»\n«abcˇ» «abcˇ»\ndefabc\n«abcˇ»");

    // Test left-to-right selections
    cx.set_state("abc\n«abcˇ»\nabc");
    cx.update_editor(|e, window, cx| e.select_all_matches(&SelectAllMatches, window, cx))
        .unwrap();
    cx.assert_editor_state("«abcˇ»\n«abcˇ»\n«abcˇ»");

    // Test right-to-left selections
    cx.set_state("abc\n«ˇabc»\nabc");
    cx.update_editor(|e, window, cx| e.select_all_matches(&SelectAllMatches, window, cx))
        .unwrap();
    cx.assert_editor_state("«ˇabc»\n«ˇabc»\n«ˇabc»");

    // Test selecting whitespace with caret selection
    cx.set_state("abc\nˇ   abc\nabc");
    cx.update_editor(|e, window, cx| e.select_all_matches(&SelectAllMatches, window, cx))
        .unwrap();
    cx.assert_editor_state("abc\n«   ˇ»abc\nabc");

    // Test selecting whitespace with left-to-right selection
    cx.set_state("abc\n«ˇ  »abc\nabc");
    cx.update_editor(|e, window, cx| e.select_all_matches(&SelectAllMatches, window, cx))
        .unwrap();
    cx.assert_editor_state("abc\n«ˇ  »abc\nabc");

    // Test no matches with right-to-left selection
    cx.set_state("abc\n«  ˇ»abc\nabc");
    cx.update_editor(|e, window, cx| e.select_all_matches(&SelectAllMatches, window, cx))
        .unwrap();
    cx.assert_editor_state("abc\n«  ˇ»abc\nabc");
}

#[gpui::test]
async fn test_select_all_matches_does_not_scroll(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    let large_body_1 = "\nd".repeat(200);
    let large_body_2 = "\ne".repeat(200);

    cx.set_state(&format!(
        "abc\nabc{large_body_1} «ˇa»bc{large_body_2}\nefabc\nabc"
    ));
    let initial_scroll_position = cx.update_editor(|editor, _, cx| {
        let scroll_position = editor.scroll_position(cx);
        assert!(scroll_position.y > 0.0, "Initial selection is between two large bodies and should have the editor scrolled to it");
        scroll_position
    });

    cx.update_editor(|e, window, cx| e.select_all_matches(&SelectAllMatches, window, cx))
        .unwrap();
    cx.assert_editor_state(&format!(
        "«ˇa»bc\n«ˇa»bc{large_body_1} «ˇa»bc{large_body_2}\nef«ˇa»bc\n«ˇa»bc"
    ));
    let scroll_position_after_selection =
        cx.update_editor(|editor, _, cx| editor.scroll_position(cx));
    assert_eq!(
        initial_scroll_position, scroll_position_after_selection,
        "Scroll position should not change after selecting all matches"
    );
}

#[gpui::test]
async fn test_undo_format_scrolls_to_last_edit_pos(cx: &mut TestAppContext) {
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
        line 1
        line 2
        linˇe 3
        line 4
        line 5
    "});

    // Make an edit
    cx.update_editor(|editor, window, cx| {
        editor.handle_input("X", window, cx);
    });

    // Move cursor to a different position
    cx.update_editor(|editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(4, 2)..Point::new(4, 2)]);
        });
    });

    cx.assert_editor_state(indoc! {"
        line 1
        line 2
        linXe 3
        line 4
        liˇne 5
    "});

    cx.lsp
        .set_request_handler::<lsp::request::Formatting, _, _>(move |_, _| async move {
            Ok(Some(vec![lsp::TextEdit::new(
                lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 0)),
                "PREFIX ".to_string(),
            )]))
        });

    cx.update_editor(|editor, window, cx| editor.format(&Default::default(), window, cx))
        .unwrap()
        .await
        .unwrap();

    cx.assert_editor_state(indoc! {"
        PREFIX line 1
        line 2
        linXe 3
        line 4
        liˇne 5
    "});

    // Undo formatting
    cx.update_editor(|editor, window, cx| {
        editor.undo(&Default::default(), window, cx);
    });

    // Verify cursor moved back to position after edit
    cx.assert_editor_state(indoc! {"
        line 1
        line 2
        linXˇe 3
        line 4
        line 5
    "});
}

#[gpui::test]
async fn test_select_next_with_multiple_carets(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state(
        r#"let foo = 2;
lˇet foo = 2;
let fooˇ = 2;
let foo = 2;
let foo = ˇ2;"#,
    );

    cx.update_editor(|e, window, cx| e.select_next(&SelectNext::default(), window, cx))
        .unwrap();
    cx.assert_editor_state(
        r#"let foo = 2;
«letˇ» foo = 2;
let «fooˇ» = 2;
let foo = 2;
let foo = «2ˇ»;"#,
    );

    // noop for multiple selections with different contents
    cx.update_editor(|e, window, cx| e.select_next(&SelectNext::default(), window, cx))
        .unwrap();
    cx.assert_editor_state(
        r#"let foo = 2;
«letˇ» foo = 2;
let «fooˇ» = 2;
let foo = 2;
let foo = «2ˇ»;"#,
    );

    // Test last selection direction should be preserved
    cx.set_state(
        r#"let foo = 2;
let foo = 2;
let «fooˇ» = 2;
let «ˇfoo» = 2;
let foo = 2;"#,
    );

    cx.update_editor(|e, window, cx| e.select_next(&SelectNext::default(), window, cx))
        .unwrap();
    cx.assert_editor_state(
        r#"let foo = 2;
let foo = 2;
let «fooˇ» = 2;
let «ˇfoo» = 2;
let «ˇfoo» = 2;"#,
    );
}

#[gpui::test]
async fn test_select_previous_multibuffer(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx =
        EditorTestContext::new_multibuffer(cx, ["aaa\n«bbb\nccc\n»ddd", "aaa\n«bbb\nccc\n»ddd"]);

    cx.assert_editor_state(indoc! {"
        ˇbbb
        ccc

        bbb
        ccc
        "});
    cx.dispatch_action(SelectPrevious::default());
    cx.assert_editor_state(indoc! {"
                «bbbˇ»
                ccc

                bbb
                ccc
                "});
    cx.dispatch_action(SelectPrevious::default());
    cx.assert_editor_state(indoc! {"
                «bbbˇ»
                ccc

                «bbbˇ»
                ccc
                "});
}

#[gpui::test]
async fn test_select_previous_with_single_caret(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state("abc\nˇabc abc\ndefabc\nabc");

    cx.update_editor(|e, window, cx| e.select_previous(&SelectPrevious::default(), window, cx))
        .unwrap();
    cx.assert_editor_state("abc\n«abcˇ» abc\ndefabc\nabc");

    cx.update_editor(|e, window, cx| e.select_previous(&SelectPrevious::default(), window, cx))
        .unwrap();
    cx.assert_editor_state("«abcˇ»\n«abcˇ» abc\ndefabc\nabc");

    cx.update_editor(|editor, window, cx| editor.undo_selection(&UndoSelection, window, cx));
    cx.assert_editor_state("abc\n«abcˇ» abc\ndefabc\nabc");

    cx.update_editor(|editor, window, cx| editor.redo_selection(&RedoSelection, window, cx));
    cx.assert_editor_state("«abcˇ»\n«abcˇ» abc\ndefabc\nabc");

    cx.update_editor(|e, window, cx| e.select_previous(&SelectPrevious::default(), window, cx))
        .unwrap();
    cx.assert_editor_state("«abcˇ»\n«abcˇ» abc\ndefabc\n«abcˇ»");

    cx.update_editor(|e, window, cx| e.select_previous(&SelectPrevious::default(), window, cx))
        .unwrap();
    cx.assert_editor_state("«abcˇ»\n«abcˇ» «abcˇ»\ndefabc\n«abcˇ»");
}

#[gpui::test]
async fn test_select_previous_empty_buffer(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state("aˇ");

    cx.update_editor(|e, window, cx| e.select_previous(&SelectPrevious::default(), window, cx))
        .unwrap();
    cx.assert_editor_state("«aˇ»");
    cx.update_editor(|e, window, cx| e.select_previous(&SelectPrevious::default(), window, cx))
        .unwrap();
    cx.assert_editor_state("«aˇ»");
}

#[gpui::test]
async fn test_select_previous_with_multiple_carets(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state(
        r#"let foo = 2;
lˇet foo = 2;
let fooˇ = 2;
let foo = 2;
let foo = ˇ2;"#,
    );

    cx.update_editor(|e, window, cx| e.select_previous(&SelectPrevious::default(), window, cx))
        .unwrap();
    cx.assert_editor_state(
        r#"let foo = 2;
«letˇ» foo = 2;
let «fooˇ» = 2;
let foo = 2;
let foo = «2ˇ»;"#,
    );

    // noop for multiple selections with different contents
    cx.update_editor(|e, window, cx| e.select_previous(&SelectPrevious::default(), window, cx))
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
async fn test_select_previous_with_single_selection(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state("abc\n«ˇabc» abc\ndefabc\nabc");

    cx.update_editor(|e, window, cx| e.select_previous(&SelectPrevious::default(), window, cx))
        .unwrap();
    // selection direction is preserved
    cx.assert_editor_state("«ˇabc»\n«ˇabc» abc\ndefabc\nabc");

    cx.update_editor(|e, window, cx| e.select_previous(&SelectPrevious::default(), window, cx))
        .unwrap();
    cx.assert_editor_state("«ˇabc»\n«ˇabc» abc\ndefabc\n«ˇabc»");

    cx.update_editor(|editor, window, cx| editor.undo_selection(&UndoSelection, window, cx));
    cx.assert_editor_state("«ˇabc»\n«ˇabc» abc\ndefabc\nabc");

    cx.update_editor(|editor, window, cx| editor.redo_selection(&RedoSelection, window, cx));
    cx.assert_editor_state("«ˇabc»\n«ˇabc» abc\ndefabc\n«ˇabc»");

    cx.update_editor(|e, window, cx| e.select_previous(&SelectPrevious::default(), window, cx))
        .unwrap();
    cx.assert_editor_state("«ˇabc»\n«ˇabc» abc\ndef«ˇabc»\n«ˇabc»");

    cx.update_editor(|e, window, cx| e.select_previous(&SelectPrevious::default(), window, cx))
        .unwrap();
    cx.assert_editor_state("«ˇabc»\n«ˇabc» «ˇabc»\ndef«ˇabc»\n«ˇabc»");
}

#[gpui::test]
async fn test_select_larger_smaller_syntax_node(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let language = Arc::new(Language::new(
        LanguageConfig::default(),
        Some(tree_sitter_rust::LANGUAGE.into()),
    ));

    let text = r#"
        use mod1::mod2::{mod3, mod4};

        fn fn_1(param1: bool, param2: &str) {
            let var1 = "text";
        }
    "#
    .unindent();

    let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(language, cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|window, cx| build_editor(buffer, window, cx));

    editor
        .condition::<crate::EditorEvent>(cx, |editor, cx| !editor.buffer.read(cx).is_parsing(cx))
        .await;

    editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 25)..DisplayPoint::new(DisplayRow(0), 25),
                DisplayPoint::new(DisplayRow(2), 24)..DisplayPoint::new(DisplayRow(2), 12),
                DisplayPoint::new(DisplayRow(3), 18)..DisplayPoint::new(DisplayRow(3), 18),
            ]);
        });
        editor.select_larger_syntax_node(&SelectLargerSyntaxNode, window, cx);
    });
    editor.update(cx, |editor, cx| {
        assert_text_with_selections(
            editor,
            indoc! {r#"
                use mod1::mod2::{mod3, «mod4ˇ»};

                fn fn_1«ˇ(param1: bool, param2: &str)» {
                    let var1 = "«ˇtext»";
                }
            "#},
            cx,
        );
    });

    editor.update_in(cx, |editor, window, cx| {
        editor.select_larger_syntax_node(&SelectLargerSyntaxNode, window, cx);
    });
    editor.update(cx, |editor, cx| {
        assert_text_with_selections(
            editor,
            indoc! {r#"
                use mod1::mod2::«{mod3, mod4}ˇ»;

                «ˇfn fn_1(param1: bool, param2: &str) {
                    let var1 = "text";
                }»
            "#},
            cx,
        );
    });

    editor.update_in(cx, |editor, window, cx| {
        editor.select_larger_syntax_node(&SelectLargerSyntaxNode, window, cx);
    });
    assert_eq!(
        editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
        &[DisplayPoint::new(DisplayRow(5), 0)..DisplayPoint::new(DisplayRow(0), 0)]
    );

    // Trying to expand the selected syntax node one more time has no effect.
    editor.update_in(cx, |editor, window, cx| {
        editor.select_larger_syntax_node(&SelectLargerSyntaxNode, window, cx);
    });
    assert_eq!(
        editor.update(cx, |editor, cx| editor.selections.display_ranges(cx)),
        &[DisplayPoint::new(DisplayRow(5), 0)..DisplayPoint::new(DisplayRow(0), 0)]
    );

    editor.update_in(cx, |editor, window, cx| {
        editor.select_smaller_syntax_node(&SelectSmallerSyntaxNode, window, cx);
    });
    editor.update(cx, |editor, cx| {
        assert_text_with_selections(
            editor,
            indoc! {r#"
                use mod1::mod2::«{mod3, mod4}ˇ»;

                «ˇfn fn_1(param1: bool, param2: &str) {
                    let var1 = "text";
                }»
            "#},
            cx,
        );
    });

    editor.update_in(cx, |editor, window, cx| {
        editor.select_smaller_syntax_node(&SelectSmallerSyntaxNode, window, cx);
    });
    editor.update(cx, |editor, cx| {
        assert_text_with_selections(
            editor,
            indoc! {r#"
                use mod1::mod2::{mod3, «mod4ˇ»};

                fn fn_1«ˇ(param1: bool, param2: &str)» {
                    let var1 = "«ˇtext»";
                }
            "#},
            cx,
        );
    });

    editor.update_in(cx, |editor, window, cx| {
        editor.select_smaller_syntax_node(&SelectSmallerSyntaxNode, window, cx);
    });
    editor.update(cx, |editor, cx| {
        assert_text_with_selections(
            editor,
            indoc! {r#"
                use mod1::mod2::{mod3, mo«ˇ»d4};

                fn fn_1(para«ˇm1: bool, pa»ram2: &str) {
                    let var1 = "te«ˇ»xt";
                }
            "#},
            cx,
        );
    });

    // Trying to shrink the selected syntax node one more time has no effect.
    editor.update_in(cx, |editor, window, cx| {
        editor.select_smaller_syntax_node(&SelectSmallerSyntaxNode, window, cx);
    });
    editor.update_in(cx, |editor, _, cx| {
        assert_text_with_selections(
            editor,
            indoc! {r#"
                use mod1::mod2::{mod3, mo«ˇ»d4};

                fn fn_1(para«ˇm1: bool, pa»ram2: &str) {
                    let var1 = "te«ˇ»xt";
                }
            "#},
            cx,
        );
    });

    // Ensure that we keep expanding the selection if the larger selection starts or ends within
    // a fold.
    editor.update_in(cx, |editor, window, cx| {
        editor.fold_creases(
            vec![
                Crease::simple(
                    Point::new(0, 21)..Point::new(0, 24),
                    FoldPlaceholder::test(),
                ),
                Crease::simple(
                    Point::new(3, 20)..Point::new(3, 22),
                    FoldPlaceholder::test(),
                ),
            ],
            true,
            window,
            cx,
        );
        editor.select_larger_syntax_node(&SelectLargerSyntaxNode, window, cx);
    });
    editor.update(cx, |editor, cx| {
        assert_text_with_selections(
            editor,
            indoc! {r#"
                use mod1::mod2::«{mod3, mod4}ˇ»;

                fn fn_1«ˇ(param1: bool, param2: &str)» {
                    let var1 = "«ˇtext»";
                }
            "#},
            cx,
        );
    });
}

#[gpui::test]
async fn test_select_larger_syntax_node_for_cursor_at_end(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let language = Arc::new(Language::new(
        LanguageConfig::default(),
        Some(tree_sitter_rust::LANGUAGE.into()),
    ));

    let text = "let a = 2;";

    let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(language, cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|window, cx| build_editor(buffer, window, cx));

    editor
        .condition::<crate::EditorEvent>(cx, |editor, cx| !editor.buffer.read(cx).is_parsing(cx))
        .await;

    // Test case 1: Cursor at end of word
    editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 5)..DisplayPoint::new(DisplayRow(0), 5)
            ]);
        });
    });
    editor.update(cx, |editor, cx| {
        assert_text_with_selections(editor, "let aˇ = 2;", cx);
    });
    editor.update_in(cx, |editor, window, cx| {
        editor.select_larger_syntax_node(&SelectLargerSyntaxNode, window, cx);
    });
    editor.update(cx, |editor, cx| {
        assert_text_with_selections(editor, "let «ˇa» = 2;", cx);
    });
    editor.update_in(cx, |editor, window, cx| {
        editor.select_larger_syntax_node(&SelectLargerSyntaxNode, window, cx);
    });
    editor.update(cx, |editor, cx| {
        assert_text_with_selections(editor, "«ˇlet a = 2;»", cx);
    });

    // Test case 2: Cursor at end of statement
    editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 11)..DisplayPoint::new(DisplayRow(0), 11)
            ]);
        });
    });
    editor.update(cx, |editor, cx| {
        assert_text_with_selections(editor, "let a = 2;ˇ", cx);
    });
    editor.update_in(cx, |editor, window, cx| {
        editor.select_larger_syntax_node(&SelectLargerSyntaxNode, window, cx);
    });
    editor.update(cx, |editor, cx| {
        assert_text_with_selections(editor, "«ˇlet a = 2;»", cx);
    });
}

#[gpui::test]
async fn test_select_larger_smaller_syntax_node_for_string(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let language = Arc::new(Language::new(
        LanguageConfig::default(),
        Some(tree_sitter_rust::LANGUAGE.into()),
    ));

    let text = r#"
        use mod1::mod2::{mod3, mod4};

        fn fn_1(param1: bool, param2: &str) {
            let var1 = "hello world";
        }
    "#
    .unindent();

    let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(language, cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|window, cx| build_editor(buffer, window, cx));

    editor
        .condition::<crate::EditorEvent>(cx, |editor, cx| !editor.buffer.read(cx).is_parsing(cx))
        .await;

    // Test 1: Cursor on a letter of a string word
    editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(3), 17)..DisplayPoint::new(DisplayRow(3), 17)
            ]);
        });
    });
    editor.update_in(cx, |editor, window, cx| {
        assert_text_with_selections(
            editor,
            indoc! {r#"
                use mod1::mod2::{mod3, mod4};

                fn fn_1(param1: bool, param2: &str) {
                    let var1 = "hˇello world";
                }
            "#},
            cx,
        );
        editor.select_larger_syntax_node(&SelectLargerSyntaxNode, window, cx);
        assert_text_with_selections(
            editor,
            indoc! {r#"
                use mod1::mod2::{mod3, mod4};

                fn fn_1(param1: bool, param2: &str) {
                    let var1 = "«ˇhello» world";
                }
            "#},
            cx,
        );
    });

    // Test 2: Partial selection within a word
    editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(3), 17)..DisplayPoint::new(DisplayRow(3), 19)
            ]);
        });
    });
    editor.update_in(cx, |editor, window, cx| {
        assert_text_with_selections(
            editor,
            indoc! {r#"
                use mod1::mod2::{mod3, mod4};

                fn fn_1(param1: bool, param2: &str) {
                    let var1 = "h«elˇ»lo world";
                }
            "#},
            cx,
        );
        editor.select_larger_syntax_node(&SelectLargerSyntaxNode, window, cx);
        assert_text_with_selections(
            editor,
            indoc! {r#"
                use mod1::mod2::{mod3, mod4};

                fn fn_1(param1: bool, param2: &str) {
                    let var1 = "«ˇhello» world";
                }
            "#},
            cx,
        );
    });

    // Test 3: Complete word already selected
    editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(3), 16)..DisplayPoint::new(DisplayRow(3), 21)
            ]);
        });
    });
    editor.update_in(cx, |editor, window, cx| {
        assert_text_with_selections(
            editor,
            indoc! {r#"
                use mod1::mod2::{mod3, mod4};

                fn fn_1(param1: bool, param2: &str) {
                    let var1 = "«helloˇ» world";
                }
            "#},
            cx,
        );
        editor.select_larger_syntax_node(&SelectLargerSyntaxNode, window, cx);
        assert_text_with_selections(
            editor,
            indoc! {r#"
                use mod1::mod2::{mod3, mod4};

                fn fn_1(param1: bool, param2: &str) {
                    let var1 = "«hello worldˇ»";
                }
            "#},
            cx,
        );
    });

    // Test 4: Selection spanning across words
    editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(3), 19)..DisplayPoint::new(DisplayRow(3), 24)
            ]);
        });
    });
    editor.update_in(cx, |editor, window, cx| {
        assert_text_with_selections(
            editor,
            indoc! {r#"
                use mod1::mod2::{mod3, mod4};

                fn fn_1(param1: bool, param2: &str) {
                    let var1 = "hel«lo woˇ»rld";
                }
            "#},
            cx,
        );
        editor.select_larger_syntax_node(&SelectLargerSyntaxNode, window, cx);
        assert_text_with_selections(
            editor,
            indoc! {r#"
                use mod1::mod2::{mod3, mod4};

                fn fn_1(param1: bool, param2: &str) {
                    let var1 = "«ˇhello world»";
                }
            "#},
            cx,
        );
    });

    // Test 5: Expansion beyond string
    editor.update_in(cx, |editor, window, cx| {
        editor.select_larger_syntax_node(&SelectLargerSyntaxNode, window, cx);
        editor.select_larger_syntax_node(&SelectLargerSyntaxNode, window, cx);
        assert_text_with_selections(
            editor,
            indoc! {r#"
                use mod1::mod2::{mod3, mod4};

                fn fn_1(param1: bool, param2: &str) {
                    «ˇlet var1 = "hello world";»
                }
            "#},
            cx,
        );
    });
}

#[gpui::test]
async fn test_fold_function_bodies(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let base_text = r#"
        impl A {
            // this is an uncommitted comment

            fn b() {
                c();
            }

            // this is another uncommitted comment

            fn d() {
                // e
                // f
            }
        }

        fn g() {
            // h
        }
    "#
    .unindent();

    let text = r#"
        ˇimpl A {

            fn b() {
                c();
            }

            fn d() {
                // e
                // f
            }
        }

        fn g() {
            // h
        }
    "#
    .unindent();

    let mut cx = EditorLspTestContext::new_rust(Default::default(), cx).await;
    cx.set_state(&text);
    cx.set_head_text(&base_text);
    cx.update_editor(|editor, window, cx| {
        editor.expand_all_diff_hunks(&Default::default(), window, cx);
    });

    cx.assert_state_with_diff(
        "
        ˇimpl A {
      -     // this is an uncommitted comment

            fn b() {
                c();
            }

      -     // this is another uncommitted comment
      -
            fn d() {
                // e
                // f
            }
        }

        fn g() {
            // h
        }
    "
        .unindent(),
    );

    let expected_display_text = "
        impl A {
            // this is an uncommitted comment

            fn b() {
                ⋯
            }

            // this is another uncommitted comment

            fn d() {
                ⋯
            }
        }

        fn g() {
            ⋯
        }
        "
    .unindent();

    cx.update_editor(|editor, window, cx| {
        editor.fold_function_bodies(&FoldFunctionBodies, window, cx);
        assert_eq!(editor.display_text(cx), expected_display_text);
    });
}

#[gpui::test]
async fn test_autoindent(cx: &mut TestAppContext) {
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
                            surround: false,
                            newline: true,
                        },
                        BracketPair {
                            start: "(".to_string(),
                            end: ")".to_string(),
                            close: false,
                            surround: false,
                            newline: true,
                        },
                    ],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
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

    let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(language, cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|window, cx| build_editor(buffer, window, cx));
    editor
        .condition::<crate::EditorEvent>(cx, |editor, cx| !editor.buffer.read(cx).is_parsing(cx))
        .await;

    editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| s.select_ranges([5..5, 8..8, 9..9]));
        editor.newline(&Newline, window, cx);
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
async fn test_autoindent_selections(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    {
        let mut cx = EditorLspTestContext::new_rust(Default::default(), cx).await;
        cx.set_state(indoc! {"
            impl A {

                fn b() {}

            «fn c() {

            }ˇ»
            }
        "});

        cx.update_editor(|editor, window, cx| {
            editor.autoindent(&Default::default(), window, cx);
        });

        cx.assert_editor_state(indoc! {"
            impl A {

                fn b() {}

                «fn c() {

                }ˇ»
            }
        "});
    }

    {
        let mut cx = EditorTestContext::new_multibuffer(
            cx,
            [indoc! { "
                impl A {
                «
                // a
                fn b(){}
                »
                «
                    }
                    fn c(){}
                »
            "}],
        );

        let buffer = cx.update_editor(|editor, _, cx| {
            let buffer = editor.buffer().update(cx, |buffer, _| {
                buffer.all_buffers().iter().next().unwrap().clone()
            });
            buffer.update(cx, |buffer, cx| buffer.set_language(Some(rust_lang()), cx));
            buffer
        });

        cx.run_until_parked();
        cx.update_editor(|editor, window, cx| {
            editor.select_all(&Default::default(), window, cx);
            editor.autoindent(&Default::default(), window, cx)
        });
        cx.run_until_parked();

        cx.update(|_, cx| {
            assert_eq!(
                buffer.read(cx).text(),
                indoc! { "
                    impl A {

                        // a
                        fn b(){}


                    }
                    fn c(){}

                " }
            )
        });
    }
}

#[gpui::test]
async fn test_autoclose_and_auto_surround_pairs(cx: &mut TestAppContext) {
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
                        surround: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "(".to_string(),
                        end: ")".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "/*".to_string(),
                        end: " */".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "[".to_string(),
                        end: "]".to_string(),
                        close: false,
                        surround: false,
                        newline: true,
                    },
                    BracketPair {
                        start: "\"".to_string(),
                        end: "\"".to_string(),
                        close: true,
                        surround: true,
                        newline: false,
                    },
                    BracketPair {
                        start: "<".to_string(),
                        end: ">".to_string(),
                        close: false,
                        surround: true,
                        newline: true,
                    },
                ],
                ..Default::default()
            },
            autoclose_before: "})]".to_string(),
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    ));

    cx.language_registry().add(language.clone());
    cx.update_buffer(|buffer, cx| {
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
    cx.update_editor(|editor, window, cx| {
        editor.handle_input("{", window, cx);
        editor.handle_input("{", window, cx);
        editor.handle_input("{", window, cx);
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
    cx.update_editor(|editor, window, cx| {
        editor.handle_input(")", window, cx);
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
    cx.update_editor(|editor, window, cx| {
        editor.move_right(&MoveRight, window, cx);
        editor.handle_input("}", window, cx);
        editor.handle_input("}", window, cx);
        editor.handle_input("}", window, cx);
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
    cx.update_editor(|editor, window, cx| {
        editor.handle_input("/", window, cx);
        editor.handle_input("*", window, cx);
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
    cx.update_editor(|editor, window, cx| editor.handle_input("*", window, cx));
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
    cx.update_editor(|editor, window, cx| editor.handle_input("{", window, cx));
    cx.assert_editor_state("{ˇa b");

    // Don't autoclose if `close` is false for the bracket pair
    cx.set_state("ˇ");
    cx.update_editor(|editor, window, cx| editor.handle_input("[", window, cx));
    cx.assert_editor_state("[ˇ");

    // Surround with brackets if text is selected
    cx.set_state("«aˇ» b");
    cx.update_editor(|editor, window, cx| editor.handle_input("{", window, cx));
    cx.assert_editor_state("{«aˇ»} b");

    // Autoclose when not immediately after a word character
    cx.set_state("a ˇ");
    cx.update_editor(|editor, window, cx| editor.handle_input("\"", window, cx));
    cx.assert_editor_state("a \"ˇ\"");

    // Autoclose pair where the start and end characters are the same
    cx.update_editor(|editor, window, cx| editor.handle_input("\"", window, cx));
    cx.assert_editor_state("a \"\"ˇ");

    // Don't autoclose when immediately after a word character
    cx.set_state("aˇ");
    cx.update_editor(|editor, window, cx| editor.handle_input("\"", window, cx));
    cx.assert_editor_state("a\"ˇ");

    // Do autoclose when after a non-word character
    cx.set_state("{ˇ");
    cx.update_editor(|editor, window, cx| editor.handle_input("\"", window, cx));
    cx.assert_editor_state("{\"ˇ\"");

    // Non identical pairs autoclose regardless of preceding character
    cx.set_state("aˇ");
    cx.update_editor(|editor, window, cx| editor.handle_input("{", window, cx));
    cx.assert_editor_state("a{ˇ}");

    // Don't autoclose pair if autoclose is disabled
    cx.set_state("ˇ");
    cx.update_editor(|editor, window, cx| editor.handle_input("<", window, cx));
    cx.assert_editor_state("<ˇ");

    // Surround with brackets if text is selected and auto_surround is enabled, even if autoclose is disabled
    cx.set_state("«aˇ» b");
    cx.update_editor(|editor, window, cx| editor.handle_input("<", window, cx));
    cx.assert_editor_state("<«aˇ»> b");
}

#[gpui::test]
async fn test_always_treat_brackets_as_autoclosed_skip_over(cx: &mut TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.always_treat_brackets_as_autoclosed = Some(true);
    });

    let mut cx = EditorTestContext::new(cx).await;

    let language = Arc::new(Language::new(
        LanguageConfig {
            brackets: BracketPairConfig {
                pairs: vec![
                    BracketPair {
                        start: "{".to_string(),
                        end: "}".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "(".to_string(),
                        end: ")".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "[".to_string(),
                        end: "]".to_string(),
                        close: false,
                        surround: false,
                        newline: true,
                    },
                ],
                ..Default::default()
            },
            autoclose_before: "})]".to_string(),
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    ));

    cx.language_registry().add(language.clone());
    cx.update_buffer(|buffer, cx| {
        buffer.set_language(Some(language), cx);
    });

    cx.set_state(
        &"
            ˇ
            ˇ
            ˇ
        "
        .unindent(),
    );

    // ensure only matching closing brackets are skipped over
    cx.update_editor(|editor, window, cx| {
        editor.handle_input("}", window, cx);
        editor.move_left(&MoveLeft, window, cx);
        editor.handle_input(")", window, cx);
        editor.move_left(&MoveLeft, window, cx);
    });
    cx.assert_editor_state(
        &"
            ˇ)}
            ˇ)}
            ˇ)}
        "
        .unindent(),
    );

    // skip-over closing brackets at multiple cursors
    cx.update_editor(|editor, window, cx| {
        editor.handle_input(")", window, cx);
        editor.handle_input("}", window, cx);
    });
    cx.assert_editor_state(
        &"
            )}ˇ
            )}ˇ
            )}ˇ
        "
        .unindent(),
    );

    // ignore non-close brackets
    cx.update_editor(|editor, window, cx| {
        editor.handle_input("]", window, cx);
        editor.move_left(&MoveLeft, window, cx);
        editor.handle_input("]", window, cx);
    });
    cx.assert_editor_state(
        &"
            )}]ˇ]
            )}]ˇ]
            )}]ˇ]
        "
        .unindent(),
    );
}

#[gpui::test]
async fn test_autoclose_with_embedded_language(cx: &mut TestAppContext) {
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
            Some(tree_sitter_html::LANGUAGE.into()),
        )
        .with_injection_query(
            r#"
            (script_element
                (raw_text) @injection.content
                (#set! injection.language "javascript"))
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
        Some(tree_sitter_typescript::LANGUAGE_TSX.into()),
    ));

    cx.language_registry().add(html_language.clone());
    cx.language_registry().add(javascript_language.clone());

    cx.update_buffer(|buffer, cx| {
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
    cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
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
    cx.update_editor(|editor, window, cx| {
        editor.handle_input("<", window, cx);
        editor.handle_input("a", window, cx);
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
    cx.update_editor(|editor, window, cx| {
        editor.handle_input(" b=", window, cx);
        editor.handle_input("{", window, cx);
        editor.handle_input("c", window, cx);
        editor.handle_input("(", window, cx);
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
    cx.update_editor(|editor, window, cx| {
        editor.handle_input(")", window, cx);
        editor.handle_input("d", window, cx);
        editor.handle_input("}", window, cx);
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
    cx.update_editor(|editor, window, cx| {
        editor.handle_input(">", window, cx);
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

    cx.update_editor(|editor, window, cx| {
        editor.handle_input("<", window, cx);
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
    cx.update_editor(|editor, window, cx| {
        editor.backspace(&Backspace, window, cx);
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
    cx.update_editor(|editor, window, cx| {
        editor.handle_input("/", window, cx);
        editor.handle_input("*", window, cx);
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
async fn test_autoclose_with_overrides(cx: &mut TestAppContext) {
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
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
        .with_override_query("(string_literal) @string")
        .unwrap(),
    );

    cx.language_registry().add(rust_language.clone());
    cx.update_buffer(|buffer, cx| {
        buffer.set_language(Some(rust_language), cx);
    });

    cx.set_state(
        &r#"
            let x = ˇ
        "#
        .unindent(),
    );

    // Inserting a quotation mark. A closing quotation mark is automatically inserted.
    cx.update_editor(|editor, window, cx| {
        editor.handle_input("\"", window, cx);
    });
    cx.assert_editor_state(
        &r#"
            let x = "ˇ"
        "#
        .unindent(),
    );

    // Inserting another quotation mark. The cursor moves across the existing
    // automatically-inserted quotation mark.
    cx.update_editor(|editor, window, cx| {
        editor.handle_input("\"", window, cx);
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
    cx.update_editor(|editor, window, cx| {
        editor.handle_input("\"", window, cx);
        editor.handle_input(" ", window, cx);
        editor.move_left(&Default::default(), window, cx);
        editor.handle_input("\\", window, cx);
        editor.handle_input("\"", window, cx);
    });
    cx.assert_editor_state(
        &r#"
            let x = "\"ˇ "
        "#
        .unindent(),
    );

    // Inserting a closing quotation mark at the position of an automatically-inserted quotation
    // mark. Nothing is inserted.
    cx.update_editor(|editor, window, cx| {
        editor.move_right(&Default::default(), window, cx);
        editor.handle_input("\"", window, cx);
    });
    cx.assert_editor_state(
        &r#"
            let x = "\" "ˇ
        "#
        .unindent(),
    );
}

#[gpui::test]
async fn test_surround_with_pair(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let language = Arc::new(Language::new(
        LanguageConfig {
            brackets: BracketPairConfig {
                pairs: vec![
                    BracketPair {
                        start: "{".to_string(),
                        end: "}".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "/* ".to_string(),
                        end: "*/".to_string(),
                        close: true,
                        surround: true,
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    ));

    let text = r#"
        a
        b
        c
    "#
    .unindent();

    let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(language, cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|window, cx| build_editor(buffer, window, cx));
    editor
        .condition::<crate::EditorEvent>(cx, |editor, cx| !editor.buffer.read(cx).is_parsing(cx))
        .await;

    editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 1),
                DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 1),
                DisplayPoint::new(DisplayRow(2), 0)..DisplayPoint::new(DisplayRow(2), 1),
            ])
        });

        editor.handle_input("{", window, cx);
        editor.handle_input("{", window, cx);
        editor.handle_input("{", window, cx);
        assert_eq!(
            editor.text(cx),
            "
                {{{a}}}
                {{{b}}}
                {{{c}}}
            "
            .unindent()
        );
        assert_eq!(
            editor.selections.display_ranges(cx),
            [
                DisplayPoint::new(DisplayRow(0), 3)..DisplayPoint::new(DisplayRow(0), 4),
                DisplayPoint::new(DisplayRow(1), 3)..DisplayPoint::new(DisplayRow(1), 4),
                DisplayPoint::new(DisplayRow(2), 3)..DisplayPoint::new(DisplayRow(2), 4)
            ]
        );

        editor.undo(&Undo, window, cx);
        editor.undo(&Undo, window, cx);
        editor.undo(&Undo, window, cx);
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
            editor.selections.display_ranges(cx),
            [
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 1),
                DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 1),
                DisplayPoint::new(DisplayRow(2), 0)..DisplayPoint::new(DisplayRow(2), 1)
            ]
        );

        // Ensure inserting the first character of a multi-byte bracket pair
        // doesn't surround the selections with the bracket.
        editor.handle_input("/", window, cx);
        assert_eq!(
            editor.text(cx),
            "
                /
                /
                /
            "
            .unindent()
        );
        assert_eq!(
            editor.selections.display_ranges(cx),
            [
                DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(0), 1),
                DisplayPoint::new(DisplayRow(1), 1)..DisplayPoint::new(DisplayRow(1), 1),
                DisplayPoint::new(DisplayRow(2), 1)..DisplayPoint::new(DisplayRow(2), 1)
            ]
        );

        editor.undo(&Undo, window, cx);
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
            editor.selections.display_ranges(cx),
            [
                DisplayPoint::new(DisplayRow(0), 0)..DisplayPoint::new(DisplayRow(0), 1),
                DisplayPoint::new(DisplayRow(1), 0)..DisplayPoint::new(DisplayRow(1), 1),
                DisplayPoint::new(DisplayRow(2), 0)..DisplayPoint::new(DisplayRow(2), 1)
            ]
        );

        // Ensure inserting the last character of a multi-byte bracket pair
        // doesn't surround the selections with the bracket.
        editor.handle_input("*", window, cx);
        assert_eq!(
            editor.text(cx),
            "
                *
                *
                *
            "
            .unindent()
        );
        assert_eq!(
            editor.selections.display_ranges(cx),
            [
                DisplayPoint::new(DisplayRow(0), 1)..DisplayPoint::new(DisplayRow(0), 1),
                DisplayPoint::new(DisplayRow(1), 1)..DisplayPoint::new(DisplayRow(1), 1),
                DisplayPoint::new(DisplayRow(2), 1)..DisplayPoint::new(DisplayRow(2), 1)
            ]
        );
    });
}

#[gpui::test]
async fn test_delete_autoclose_pair(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let language = Arc::new(Language::new(
        LanguageConfig {
            brackets: BracketPairConfig {
                pairs: vec![BracketPair {
                    start: "{".to_string(),
                    end: "}".to_string(),
                    close: true,
                    surround: true,
                    newline: true,
                }],
                ..Default::default()
            },
            autoclose_before: "}".to_string(),
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    ));

    let text = r#"
        a
        b
        c
    "#
    .unindent();

    let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(language, cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|window, cx| build_editor(buffer, window, cx));
    editor
        .condition::<crate::EditorEvent>(cx, |editor, cx| !editor.buffer.read(cx).is_parsing(cx))
        .await;

    editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([
                Point::new(0, 1)..Point::new(0, 1),
                Point::new(1, 1)..Point::new(1, 1),
                Point::new(2, 1)..Point::new(2, 1),
            ])
        });

        editor.handle_input("{", window, cx);
        editor.handle_input("{", window, cx);
        editor.handle_input("_", window, cx);
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

        editor.backspace(&Default::default(), window, cx);
        editor.backspace(&Default::default(), window, cx);
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

        editor.delete_to_previous_word_start(&Default::default(), window, cx);
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
async fn test_always_treat_brackets_as_autoclosed_delete(cx: &mut TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.always_treat_brackets_as_autoclosed = Some(true);
    });

    let mut cx = EditorTestContext::new(cx).await;

    let language = Arc::new(Language::new(
        LanguageConfig {
            brackets: BracketPairConfig {
                pairs: vec![
                    BracketPair {
                        start: "{".to_string(),
                        end: "}".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "(".to_string(),
                        end: ")".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "[".to_string(),
                        end: "]".to_string(),
                        close: false,
                        surround: true,
                        newline: true,
                    },
                ],
                ..Default::default()
            },
            autoclose_before: "})]".to_string(),
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    ));

    cx.language_registry().add(language.clone());
    cx.update_buffer(|buffer, cx| {
        buffer.set_language(Some(language), cx);
    });

    cx.set_state(
        &"
            {(ˇ)}
            [[ˇ]]
            {(ˇ)}
        "
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.backspace(&Default::default(), window, cx);
        editor.backspace(&Default::default(), window, cx);
    });

    cx.assert_editor_state(
        &"
            ˇ
            ˇ]]
            ˇ
        "
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.handle_input("{", window, cx);
        editor.handle_input("{", window, cx);
        editor.move_right(&MoveRight, window, cx);
        editor.move_right(&MoveRight, window, cx);
        editor.move_left(&MoveLeft, window, cx);
        editor.move_left(&MoveLeft, window, cx);
        editor.backspace(&Default::default(), window, cx);
    });

    cx.assert_editor_state(
        &"
            {ˇ}
            {ˇ}]]
            {ˇ}
        "
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.backspace(&Default::default(), window, cx);
    });

    cx.assert_editor_state(
        &"
            ˇ
            ˇ]]
            ˇ
        "
        .unindent(),
    );
}

#[gpui::test]
async fn test_auto_replace_emoji_shortcode(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let language = Arc::new(Language::new(
        LanguageConfig::default(),
        Some(tree_sitter_rust::LANGUAGE.into()),
    ));

    let buffer = cx.new(|cx| Buffer::local("", cx).with_language(language, cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|window, cx| build_editor(buffer, window, cx));
    editor
        .condition::<crate::EditorEvent>(cx, |editor, cx| !editor.buffer.read(cx).is_parsing(cx))
        .await;

    editor.update_in(cx, |editor, window, cx| {
        editor.set_auto_replace_emoji_shortcode(true);

        editor.handle_input("Hello ", window, cx);
        editor.handle_input(":wave", window, cx);
        assert_eq!(editor.text(cx), "Hello :wave".unindent());

        editor.handle_input(":", window, cx);
        assert_eq!(editor.text(cx), "Hello 👋".unindent());

        editor.handle_input(" :smile", window, cx);
        assert_eq!(editor.text(cx), "Hello 👋 :smile".unindent());

        editor.handle_input(":", window, cx);
        assert_eq!(editor.text(cx), "Hello 👋 😄".unindent());

        // Ensure shortcode gets replaced when it is part of a word that only consists of emojis
        editor.handle_input(":wave", window, cx);
        assert_eq!(editor.text(cx), "Hello 👋 😄:wave".unindent());

        editor.handle_input(":", window, cx);
        assert_eq!(editor.text(cx), "Hello 👋 😄👋".unindent());

        editor.handle_input(":1", window, cx);
        assert_eq!(editor.text(cx), "Hello 👋 😄👋:1".unindent());

        editor.handle_input(":", window, cx);
        assert_eq!(editor.text(cx), "Hello 👋 😄👋:1:".unindent());

        // Ensure shortcode does not get replaced when it is part of a word
        editor.handle_input(" Test:wave", window, cx);
        assert_eq!(editor.text(cx), "Hello 👋 😄👋:1: Test:wave".unindent());

        editor.handle_input(":", window, cx);
        assert_eq!(editor.text(cx), "Hello 👋 😄👋:1: Test:wave:".unindent());

        editor.set_auto_replace_emoji_shortcode(false);

        // Ensure shortcode does not get replaced when auto replace is off
        editor.handle_input(" :wave", window, cx);
        assert_eq!(
            editor.text(cx),
            "Hello 👋 😄👋:1: Test:wave: :wave".unindent()
        );

        editor.handle_input(":", window, cx);
        assert_eq!(
            editor.text(cx),
            "Hello 👋 😄👋:1: Test:wave: :wave:".unindent()
        );
    });
}

#[gpui::test]
async fn test_snippet_placeholder_choices(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let (text, insertion_ranges) = marked_text_ranges(
        indoc! {"
            ˇ
        "},
        false,
    );

    let buffer = cx.update(|cx| MultiBuffer::build_simple(&text, cx));
    let (editor, cx) = cx.add_window_view(|window, cx| build_editor(buffer, window, cx));

    _ = editor.update_in(cx, |editor, window, cx| {
        let snippet = Snippet::parse("type ${1|,i32,u32|} = $2").unwrap();

        editor
            .insert_snippet(&insertion_ranges, snippet, window, cx)
            .unwrap();

        fn assert(editor: &mut Editor, cx: &mut Context<Editor>, marked_text: &str) {
            let (expected_text, selection_ranges) = marked_text_ranges(marked_text, false);
            assert_eq!(editor.text(cx), expected_text);
            assert_eq!(editor.selections.ranges::<usize>(cx), selection_ranges);
        }

        assert(
            editor,
            cx,
            indoc! {"
            type «» =•
            "},
        );

        assert!(editor.context_menu_visible(), "There should be a matches");
    });
}

#[gpui::test]
async fn test_snippets(cx: &mut TestAppContext) {
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
    let (editor, cx) = cx.add_window_view(|window, cx| build_editor(buffer, window, cx));

    editor.update_in(cx, |editor, window, cx| {
        let snippet = Snippet::parse("f(${1:one}, ${2:two}, ${1:three})$0").unwrap();

        editor
            .insert_snippet(&insertion_ranges, snippet, window, cx)
            .unwrap();

        fn assert(editor: &mut Editor, cx: &mut Context<Editor>, marked_text: &str) {
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
        assert!(!editor.move_to_prev_snippet_tabstop(window, cx));
        assert(
            editor,
            cx,
            indoc! {"
                a.f(«one», two, «three») b
                a.f(«one», two, «three») b
                a.f(«one», two, «three») b
            "},
        );

        assert!(editor.move_to_next_snippet_tabstop(window, cx));
        assert(
            editor,
            cx,
            indoc! {"
                a.f(one, «two», three) b
                a.f(one, «two», three) b
                a.f(one, «two», three) b
            "},
        );

        editor.move_to_prev_snippet_tabstop(window, cx);
        assert(
            editor,
            cx,
            indoc! {"
                a.f(«one», two, «three») b
                a.f(«one», two, «three») b
                a.f(«one», two, «three») b
            "},
        );

        assert!(editor.move_to_next_snippet_tabstop(window, cx));
        assert(
            editor,
            cx,
            indoc! {"
                a.f(one, «two», three) b
                a.f(one, «two», three) b
                a.f(one, «two», three) b
            "},
        );
        assert!(editor.move_to_next_snippet_tabstop(window, cx));
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
        editor.move_to_prev_snippet_tabstop(window, cx);
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
async fn test_document_format_during_save(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    fs.insert_file(path!("/file.rs"), Default::default()).await;

    let project = Project::test(fs, [path!("/file.rs").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp(
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
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/file.rs"), cx)
        })
        .await
        .unwrap();

    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|window, cx| {
        build_editor_with_project(project.clone(), buffer, window, cx)
    });
    editor.update_in(cx, |editor, window, cx| {
        editor.set_text("one\ntwo\nthree\n", window, cx)
    });
    assert!(cx.read(|cx| editor.is_dirty(cx)));

    cx.executor().start_waiting();
    let fake_server = fake_servers.next().await.unwrap();

    {
        fake_server.set_request_handler::<lsp::request::Formatting, _, _>(
            move |params, _| async move {
                assert_eq!(
                    params.text_document.uri,
                    lsp::Url::from_file_path(path!("/file.rs")).unwrap()
                );
                assert_eq!(params.options.tab_size, 4);
                Ok(Some(vec![lsp::TextEdit::new(
                    lsp::Range::new(lsp::Position::new(0, 3), lsp::Position::new(1, 0)),
                    ", ".to_string(),
                )]))
            },
        );
        let save = editor
            .update_in(cx, |editor, window, cx| {
                editor.save(true, project.clone(), window, cx)
            })
            .unwrap();
        cx.executor().start_waiting();
        save.await;

        assert_eq!(
            editor.update(cx, |editor, cx| editor.text(cx)),
            "one, two\nthree\n"
        );
        assert!(!cx.read(|cx| editor.is_dirty(cx)));
    }

    {
        editor.update_in(cx, |editor, window, cx| {
            editor.set_text("one\ntwo\nthree\n", window, cx)
        });
        assert!(cx.read(|cx| editor.is_dirty(cx)));

        // Ensure we can still save even if formatting hangs.
        fake_server.set_request_handler::<lsp::request::Formatting, _, _>(
            move |params, _| async move {
                assert_eq!(
                    params.text_document.uri,
                    lsp::Url::from_file_path(path!("/file.rs")).unwrap()
                );
                futures::future::pending::<()>().await;
                unreachable!()
            },
        );
        let save = editor
            .update_in(cx, |editor, window, cx| {
                editor.save(true, project.clone(), window, cx)
            })
            .unwrap();
        cx.executor().advance_clock(super::FORMAT_TIMEOUT);
        cx.executor().start_waiting();
        save.await;
        assert_eq!(
            editor.update(cx, |editor, cx| editor.text(cx)),
            "one\ntwo\nthree\n"
        );
    }

    // For non-dirty buffer, no formatting request should be sent
    {
        assert!(!cx.read(|cx| editor.is_dirty(cx)));

        fake_server.set_request_handler::<lsp::request::Formatting, _, _>(move |_, _| async move {
            panic!("Should not be invoked on non-dirty buffer");
        });
        let save = editor
            .update_in(cx, |editor, window, cx| {
                editor.save(true, project.clone(), window, cx)
            })
            .unwrap();
        cx.executor().start_waiting();
        save.await;
    }

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

    {
        editor.update_in(cx, |editor, window, cx| {
            editor.set_text("somehting_new\n", window, cx)
        });
        assert!(cx.read(|cx| editor.is_dirty(cx)));
        let _formatting_request_signal = fake_server
            .set_request_handler::<lsp::request::Formatting, _, _>(move |params, _| async move {
                assert_eq!(
                    params.text_document.uri,
                    lsp::Url::from_file_path(path!("/file.rs")).unwrap()
                );
                assert_eq!(params.options.tab_size, 8);
                Ok(Some(vec![]))
            });
        let save = editor
            .update_in(cx, |editor, window, cx| {
                editor.save(true, project.clone(), window, cx)
            })
            .unwrap();
        cx.executor().start_waiting();
        save.await;
    }
}

#[gpui::test]
async fn test_multibuffer_format_during_save(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let cols = 4;
    let rows = 10;
    let sample_text_1 = sample_text(rows, cols, 'a');
    assert_eq!(
        sample_text_1,
        "aaaa\nbbbb\ncccc\ndddd\neeee\nffff\ngggg\nhhhh\niiii\njjjj"
    );
    let sample_text_2 = sample_text(rows, cols, 'l');
    assert_eq!(
        sample_text_2,
        "llll\nmmmm\nnnnn\noooo\npppp\nqqqq\nrrrr\nssss\ntttt\nuuuu"
    );
    let sample_text_3 = sample_text(rows, cols, 'v');
    assert_eq!(
        sample_text_3,
        "vvvv\nwwww\nxxxx\nyyyy\nzzzz\n{{{{\n||||\n}}}}\n~~~~\n\u{7f}\u{7f}\u{7f}\u{7f}"
    );

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/a"),
        json!({
            "main.rs": sample_text_1,
            "other.rs": sample_text_2,
            "lib.rs": sample_text_3,
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace.deref(), cx);

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                document_formatting_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let worktree = project.update(cx, |project, cx| {
        let mut worktrees = project.worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 1);
        worktrees.pop().unwrap()
    });
    let worktree_id = worktree.update(cx, |worktree, _| worktree.id());

    let buffer_1 = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, "main.rs"), cx)
        })
        .await
        .unwrap();
    let buffer_2 = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, "other.rs"), cx)
        })
        .await
        .unwrap();
    let buffer_3 = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, "lib.rs"), cx)
        })
        .await
        .unwrap();

    let multi_buffer = cx.new(|cx| {
        let mut multi_buffer = MultiBuffer::new(ReadWrite);
        multi_buffer.push_excerpts(
            buffer_1.clone(),
            [
                ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0)),
                ExcerptRange::new(Point::new(5, 0)..Point::new(7, 0)),
                ExcerptRange::new(Point::new(9, 0)..Point::new(10, 4)),
            ],
            cx,
        );
        multi_buffer.push_excerpts(
            buffer_2.clone(),
            [
                ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0)),
                ExcerptRange::new(Point::new(5, 0)..Point::new(7, 0)),
                ExcerptRange::new(Point::new(9, 0)..Point::new(10, 4)),
            ],
            cx,
        );
        multi_buffer.push_excerpts(
            buffer_3.clone(),
            [
                ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0)),
                ExcerptRange::new(Point::new(5, 0)..Point::new(7, 0)),
                ExcerptRange::new(Point::new(9, 0)..Point::new(10, 4)),
            ],
            cx,
        );
        multi_buffer
    });
    let multi_buffer_editor = cx.new_window_entity(|window, cx| {
        Editor::new(
            EditorMode::full(),
            multi_buffer,
            Some(project.clone()),
            window,
            cx,
        )
    });

    multi_buffer_editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(Some(Autoscroll::Next), window, cx, |s| {
            s.select_ranges(Some(1..2))
        });
        editor.insert("|one|two|three|", window, cx);
    });
    assert!(cx.read(|cx| multi_buffer_editor.is_dirty(cx)));
    multi_buffer_editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(Some(Autoscroll::Next), window, cx, |s| {
            s.select_ranges(Some(60..70))
        });
        editor.insert("|four|five|six|", window, cx);
    });
    assert!(cx.read(|cx| multi_buffer_editor.is_dirty(cx)));

    // First two buffers should be edited, but not the third one.
    assert_eq!(
        multi_buffer_editor.update(cx, |editor, cx| editor.text(cx)),
        "a|one|two|three|aa\nbbbb\ncccc\n\nffff\ngggg\n\njjjj\nllll\nmmmm\nnnnn|four|five|six|\nr\n\nuuuu\nvvvv\nwwww\nxxxx\n\n{{{{\n||||\n\n\u{7f}\u{7f}\u{7f}\u{7f}",
    );
    buffer_1.update(cx, |buffer, _| {
        assert!(buffer.is_dirty());
        assert_eq!(
            buffer.text(),
            "a|one|two|three|aa\nbbbb\ncccc\ndddd\neeee\nffff\ngggg\nhhhh\niiii\njjjj",
        )
    });
    buffer_2.update(cx, |buffer, _| {
        assert!(buffer.is_dirty());
        assert_eq!(
            buffer.text(),
            "llll\nmmmm\nnnnn|four|five|six|oooo\npppp\nr\nssss\ntttt\nuuuu",
        )
    });
    buffer_3.update(cx, |buffer, _| {
        assert!(!buffer.is_dirty());
        assert_eq!(buffer.text(), sample_text_3,)
    });
    cx.executor().run_until_parked();

    cx.executor().start_waiting();
    let save = multi_buffer_editor
        .update_in(cx, |editor, window, cx| {
            editor.save(true, project.clone(), window, cx)
        })
        .unwrap();

    let fake_server = fake_servers.next().await.unwrap();
    fake_server
        .server
        .on_request::<lsp::request::Formatting, _, _>(move |params, _| async move {
            Ok(Some(vec![lsp::TextEdit::new(
                lsp::Range::new(lsp::Position::new(0, 3), lsp::Position::new(1, 0)),
                format!("[{} formatted]", params.text_document.uri),
            )]))
        })
        .detach();
    save.await;

    // After multibuffer saving, only first two buffers should be reformatted, but not the third one (as it was not dirty).
    assert!(cx.read(|cx| !multi_buffer_editor.is_dirty(cx)));
    assert_eq!(
        multi_buffer_editor.update(cx, |editor, cx| editor.text(cx)),
        uri!(
            "a|o[file:///a/main.rs formatted]bbbb\ncccc\n\nffff\ngggg\n\njjjj\n\nlll[file:///a/other.rs formatted]mmmm\nnnnn|four|five|six|\nr\n\nuuuu\n\nvvvv\nwwww\nxxxx\n\n{{{{\n||||\n\n\u{7f}\u{7f}\u{7f}\u{7f}"
        ),
    );
    buffer_1.update(cx, |buffer, _| {
        assert!(!buffer.is_dirty());
        assert_eq!(
            buffer.text(),
            uri!("a|o[file:///a/main.rs formatted]bbbb\ncccc\ndddd\neeee\nffff\ngggg\nhhhh\niiii\njjjj\n"),
        )
    });
    buffer_2.update(cx, |buffer, _| {
        assert!(!buffer.is_dirty());
        assert_eq!(
            buffer.text(),
            uri!("lll[file:///a/other.rs formatted]mmmm\nnnnn|four|five|six|oooo\npppp\nr\nssss\ntttt\nuuuu\n"),
        )
    });
    buffer_3.update(cx, |buffer, _| {
        assert!(!buffer.is_dirty());
        assert_eq!(buffer.text(), sample_text_3,)
    });
}

#[gpui::test]
async fn test_range_format_during_save(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    fs.insert_file(path!("/file.rs"), Default::default()).await;

    let project = Project::test(fs, [path!("/").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp(
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
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/file.rs"), cx)
        })
        .await
        .unwrap();

    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|window, cx| {
        build_editor_with_project(project.clone(), buffer, window, cx)
    });
    editor.update_in(cx, |editor, window, cx| {
        editor.set_text("one\ntwo\nthree\n", window, cx)
    });
    assert!(cx.read(|cx| editor.is_dirty(cx)));

    cx.executor().start_waiting();
    let fake_server = fake_servers.next().await.unwrap();

    let save = editor
        .update_in(cx, |editor, window, cx| {
            editor.save(true, project.clone(), window, cx)
        })
        .unwrap();
    fake_server
        .set_request_handler::<lsp::request::RangeFormatting, _, _>(move |params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path(path!("/file.rs")).unwrap()
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

    editor.update_in(cx, |editor, window, cx| {
        editor.set_text("one\ntwo\nthree\n", window, cx)
    });
    assert!(cx.read(|cx| editor.is_dirty(cx)));

    // Ensure we can still save even if formatting hangs.
    fake_server.set_request_handler::<lsp::request::RangeFormatting, _, _>(
        move |params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path(path!("/file.rs")).unwrap()
            );
            futures::future::pending::<()>().await;
            unreachable!()
        },
    );
    let save = editor
        .update_in(cx, |editor, window, cx| {
            editor.save(true, project.clone(), window, cx)
        })
        .unwrap();
    cx.executor().advance_clock(super::FORMAT_TIMEOUT);
    cx.executor().start_waiting();
    save.await;
    assert_eq!(
        editor.update(cx, |editor, cx| editor.text(cx)),
        "one\ntwo\nthree\n"
    );
    assert!(!cx.read(|cx| editor.is_dirty(cx)));

    // For non-dirty buffer, no formatting request should be sent
    let save = editor
        .update_in(cx, |editor, window, cx| {
            editor.save(true, project.clone(), window, cx)
        })
        .unwrap();
    let _pending_format_request = fake_server
        .set_request_handler::<lsp::request::RangeFormatting, _, _>(move |_, _| async move {
            panic!("Should not be invoked on non-dirty buffer");
        })
        .next();
    cx.executor().start_waiting();
    save.await;

    // Set Rust language override and assert overridden tabsize is sent to language server
    update_test_language_settings(cx, |settings| {
        settings.languages.insert(
            "Rust".into(),
            LanguageSettingsContent {
                tab_size: NonZeroU32::new(8),
                ..Default::default()
            },
        );
    });

    editor.update_in(cx, |editor, window, cx| {
        editor.set_text("somehting_new\n", window, cx)
    });
    assert!(cx.read(|cx| editor.is_dirty(cx)));
    let save = editor
        .update_in(cx, |editor, window, cx| {
            editor.save(true, project.clone(), window, cx)
        })
        .unwrap();
    fake_server
        .set_request_handler::<lsp::request::RangeFormatting, _, _>(move |params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path(path!("/file.rs")).unwrap()
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
async fn test_document_format_manual_trigger(cx: &mut TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.formatter = Some(language_settings::SelectedFormatter::List(
            FormatterList(vec![Formatter::LanguageServer { name: None }].into()),
        ))
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_file(path!("/file.rs"), Default::default()).await;

    let project = Project::test(fs, [path!("/").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(Arc::new(Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            ..LanguageConfig::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    )));
    update_test_language_settings(cx, |settings| {
        // Enable Prettier formatting for the same buffer, and ensure
        // LSP is called instead of Prettier.
        settings.defaults.prettier = Some(PrettierSettings {
            allowed: true,
            ..PrettierSettings::default()
        });
    });
    let mut fake_servers = language_registry.register_fake_lsp(
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
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/file.rs"), cx)
        })
        .await
        .unwrap();

    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|window, cx| {
        build_editor_with_project(project.clone(), buffer, window, cx)
    });
    editor.update_in(cx, |editor, window, cx| {
        editor.set_text("one\ntwo\nthree\n", window, cx)
    });

    cx.executor().start_waiting();
    let fake_server = fake_servers.next().await.unwrap();

    let format = editor
        .update_in(cx, |editor, window, cx| {
            editor.perform_format(
                project.clone(),
                FormatTrigger::Manual,
                FormatTarget::Buffers,
                window,
                cx,
            )
        })
        .unwrap();
    fake_server
        .set_request_handler::<lsp::request::Formatting, _, _>(move |params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path(path!("/file.rs")).unwrap()
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

    editor.update_in(cx, |editor, window, cx| {
        editor.set_text("one\ntwo\nthree\n", window, cx)
    });
    // Ensure we don't lock if formatting hangs.
    fake_server.set_request_handler::<lsp::request::Formatting, _, _>(
        move |params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path(path!("/file.rs")).unwrap()
            );
            futures::future::pending::<()>().await;
            unreachable!()
        },
    );
    let format = editor
        .update_in(cx, |editor, window, cx| {
            editor.perform_format(
                project,
                FormatTrigger::Manual,
                FormatTarget::Buffers,
                window,
                cx,
            )
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
async fn test_multiple_formatters(cx: &mut TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.remove_trailing_whitespace_on_save = Some(true);
        settings.defaults.formatter =
            Some(language_settings::SelectedFormatter::List(FormatterList(
                vec![
                    Formatter::LanguageServer { name: None },
                    Formatter::CodeActions(
                        [
                            ("code-action-1".into(), true),
                            ("code-action-2".into(), true),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                ]
                .into(),
            )))
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_file(path!("/file.rs"), "one  \ntwo   \nthree".into())
        .await;

    let project = Project::test(fs, [path!("/").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());

    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                document_formatting_provider: Some(lsp::OneOf::Left(true)),
                execute_command_provider: Some(lsp::ExecuteCommandOptions {
                    commands: vec!["the-command-for-code-action-1".into()],
                    ..Default::default()
                }),
                code_action_provider: Some(lsp::CodeActionProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/file.rs"), cx)
        })
        .await
        .unwrap();

    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|window, cx| {
        build_editor_with_project(project.clone(), buffer, window, cx)
    });

    cx.executor().start_waiting();

    let fake_server = fake_servers.next().await.unwrap();
    fake_server.set_request_handler::<lsp::request::Formatting, _, _>(
        move |_params, _| async move {
            Ok(Some(vec![lsp::TextEdit::new(
                lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 0)),
                "applied-formatting\n".to_string(),
            )]))
        },
    );
    fake_server.set_request_handler::<lsp::request::CodeActionRequest, _, _>(
        move |params, _| async move {
            assert_eq!(
                params.context.only,
                Some(vec!["code-action-1".into(), "code-action-2".into()])
            );
            let uri = lsp::Url::from_file_path(path!("/file.rs")).unwrap();
            Ok(Some(vec![
                lsp::CodeActionOrCommand::CodeAction(lsp::CodeAction {
                    kind: Some("code-action-1".into()),
                    edit: Some(lsp::WorkspaceEdit::new(
                        [(
                            uri.clone(),
                            vec![lsp::TextEdit::new(
                                lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 0)),
                                "applied-code-action-1-edit\n".to_string(),
                            )],
                        )]
                        .into_iter()
                        .collect(),
                    )),
                    command: Some(lsp::Command {
                        command: "the-command-for-code-action-1".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                lsp::CodeActionOrCommand::CodeAction(lsp::CodeAction {
                    kind: Some("code-action-2".into()),
                    edit: Some(lsp::WorkspaceEdit::new(
                        [(
                            uri.clone(),
                            vec![lsp::TextEdit::new(
                                lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 0)),
                                "applied-code-action-2-edit\n".to_string(),
                            )],
                        )]
                        .into_iter()
                        .collect(),
                    )),
                    ..Default::default()
                }),
            ]))
        },
    );

    fake_server.set_request_handler::<lsp::request::CodeActionResolveRequest, _, _>({
        move |params, _| async move { Ok(params) }
    });

    let command_lock = Arc::new(futures::lock::Mutex::new(()));
    fake_server.set_request_handler::<lsp::request::ExecuteCommand, _, _>({
        let fake = fake_server.clone();
        let lock = command_lock.clone();
        move |params, _| {
            assert_eq!(params.command, "the-command-for-code-action-1");
            let fake = fake.clone();
            let lock = lock.clone();
            async move {
                lock.lock().await;
                fake.server
                    .request::<lsp::request::ApplyWorkspaceEdit>(lsp::ApplyWorkspaceEditParams {
                        label: None,
                        edit: lsp::WorkspaceEdit {
                            changes: Some(
                                [(
                                    lsp::Url::from_file_path(path!("/file.rs")).unwrap(),
                                    vec![lsp::TextEdit {
                                        range: lsp::Range::new(
                                            lsp::Position::new(0, 0),
                                            lsp::Position::new(0, 0),
                                        ),
                                        new_text: "applied-code-action-1-command\n".into(),
                                    }],
                                )]
                                .into_iter()
                                .collect(),
                            ),
                            ..Default::default()
                        },
                    })
                    .await
                    .into_response()
                    .unwrap();
                Ok(Some(json!(null)))
            }
        }
    });

    cx.executor().start_waiting();
    editor
        .update_in(cx, |editor, window, cx| {
            editor.perform_format(
                project.clone(),
                FormatTrigger::Manual,
                FormatTarget::Buffers,
                window,
                cx,
            )
        })
        .unwrap()
        .await;
    editor.update(cx, |editor, cx| {
        assert_eq!(
            editor.text(cx),
            r#"
                applied-code-action-2-edit
                applied-code-action-1-command
                applied-code-action-1-edit
                applied-formatting
                one
                two
                three
            "#
            .unindent()
        );
    });

    editor.update_in(cx, |editor, window, cx| {
        editor.undo(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "one  \ntwo   \nthree");
    });

    // Perform a manual edit while waiting for an LSP command
    // that's being run as part of a formatting code action.
    let lock_guard = command_lock.lock().await;
    let format = editor
        .update_in(cx, |editor, window, cx| {
            editor.perform_format(
                project.clone(),
                FormatTrigger::Manual,
                FormatTarget::Buffers,
                window,
                cx,
            )
        })
        .unwrap();
    cx.run_until_parked();
    editor.update(cx, |editor, cx| {
        assert_eq!(
            editor.text(cx),
            r#"
                applied-code-action-1-edit
                applied-formatting
                one
                two
                three
            "#
            .unindent()
        );

        editor.buffer.update(cx, |buffer, cx| {
            let ix = buffer.len(cx);
            buffer.edit([(ix..ix, "edited\n")], None, cx);
        });
    });

    // Allow the LSP command to proceed. Because the buffer was edited,
    // the second code action will not be run.
    drop(lock_guard);
    format.await;
    editor.update_in(cx, |editor, window, cx| {
        assert_eq!(
            editor.text(cx),
            r#"
                applied-code-action-1-command
                applied-code-action-1-edit
                applied-formatting
                one
                two
                three
                edited
            "#
            .unindent()
        );

        // The manual edit is undone first, because it is the last thing the user did
        // (even though the command completed afterwards).
        editor.undo(&Default::default(), window, cx);
        assert_eq!(
            editor.text(cx),
            r#"
                applied-code-action-1-command
                applied-code-action-1-edit
                applied-formatting
                one
                two
                three
            "#
            .unindent()
        );

        // All the formatting (including the command, which completed after the manual edit)
        // is undone together.
        editor.undo(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "one  \ntwo   \nthree");
    });
}

#[gpui::test]
async fn test_organize_imports_manual_trigger(cx: &mut TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.formatter = Some(language_settings::SelectedFormatter::List(
            FormatterList(vec![Formatter::LanguageServer { name: None }].into()),
        ))
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_file(path!("/file.ts"), Default::default()).await;

    let project = Project::test(fs, [path!("/").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(Arc::new(Language::new(
        LanguageConfig {
            name: "TypeScript".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["ts".to_string()],
                ..Default::default()
            },
            ..LanguageConfig::default()
        },
        Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
    )));
    update_test_language_settings(cx, |settings| {
        settings.defaults.prettier = Some(PrettierSettings {
            allowed: true,
            ..PrettierSettings::default()
        });
    });
    let mut fake_servers = language_registry.register_fake_lsp(
        "TypeScript",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                code_action_provider: Some(lsp::CodeActionProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/file.ts"), cx)
        })
        .await
        .unwrap();

    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|window, cx| {
        build_editor_with_project(project.clone(), buffer, window, cx)
    });
    editor.update_in(cx, |editor, window, cx| {
        editor.set_text(
            "import { a } from 'module';\nimport { b } from 'module';\n\nconst x = a;\n",
            window,
            cx,
        )
    });

    cx.executor().start_waiting();
    let fake_server = fake_servers.next().await.unwrap();

    let format = editor
        .update_in(cx, |editor, window, cx| {
            editor.perform_code_action_kind(
                project.clone(),
                CodeActionKind::SOURCE_ORGANIZE_IMPORTS,
                window,
                cx,
            )
        })
        .unwrap();
    fake_server
        .set_request_handler::<lsp::request::CodeActionRequest, _, _>(move |params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path(path!("/file.ts")).unwrap()
            );
            Ok(Some(vec![lsp::CodeActionOrCommand::CodeAction(
                lsp::CodeAction {
                    title: "Organize Imports".to_string(),
                    kind: Some(lsp::CodeActionKind::SOURCE_ORGANIZE_IMPORTS),
                    edit: Some(lsp::WorkspaceEdit {
                        changes: Some(
                            [(
                                params.text_document.uri.clone(),
                                vec![lsp::TextEdit::new(
                                    lsp::Range::new(
                                        lsp::Position::new(1, 0),
                                        lsp::Position::new(2, 0),
                                    ),
                                    "".to_string(),
                                )],
                            )]
                            .into_iter()
                            .collect(),
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )]))
        })
        .next()
        .await;
    cx.executor().start_waiting();
    format.await;
    assert_eq!(
        editor.update(cx, |editor, cx| editor.text(cx)),
        "import { a } from 'module';\n\nconst x = a;\n"
    );

    editor.update_in(cx, |editor, window, cx| {
        editor.set_text(
            "import { a } from 'module';\nimport { b } from 'module';\n\nconst x = a;\n",
            window,
            cx,
        )
    });
    // Ensure we don't lock if code action hangs.
    fake_server.set_request_handler::<lsp::request::CodeActionRequest, _, _>(
        move |params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path(path!("/file.ts")).unwrap()
            );
            futures::future::pending::<()>().await;
            unreachable!()
        },
    );
    let format = editor
        .update_in(cx, |editor, window, cx| {
            editor.perform_code_action_kind(
                project,
                CodeActionKind::SOURCE_ORGANIZE_IMPORTS,
                window,
                cx,
            )
        })
        .unwrap();
    cx.executor().advance_clock(super::CODE_ACTION_TIMEOUT);
    cx.executor().start_waiting();
    format.await;
    assert_eq!(
        editor.update(cx, |editor, cx| editor.text(cx)),
        "import { a } from 'module';\nimport { b } from 'module';\n\nconst x = a;\n"
    );
}

#[gpui::test]
async fn test_concurrent_format_requests(cx: &mut TestAppContext) {
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
        .set_request_handler::<lsp::request::Formatting, _, _>(move |_, cx| {
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
        .update_editor(|editor, window, cx| editor.format(&Format, window, cx))
        .unwrap();
    cx.executor().run_until_parked();

    // Submit a second format request.
    let format_2 = cx
        .update_editor(|editor, window, cx| editor.format(&Format, window, cx))
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
async fn test_strip_whitespace_and_format_via_lsp(cx: &mut TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.formatter = Some(language_settings::SelectedFormatter::Auto)
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
        .update_editor(|editor, window, cx| editor.format(&Format, window, cx))
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
    cx.lsp
        .set_request_handler::<lsp::request::Formatting, _, _>({
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
                            range: lsp::Range::new(
                                lsp::Position::new(1, 0),
                                lsp::Position::new(1, 0),
                            ),
                            new_text: "\n".into(),
                        },
                        lsp::TextEdit {
                            range: lsp::Range::new(
                                lsp::Position::new(2, 0),
                                lsp::Position::new(2, 0),
                            ),
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
async fn test_handle_input_for_show_signature_help_auto_signature_help_true(
    cx: &mut TestAppContext,
) {
    init_test(cx, |_| {});

    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|settings, cx| {
            settings.update_user_settings::<EditorSettings>(cx, |settings| {
                settings.auto_signature_help = Some(true);
            });
        });
    });

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            signature_help_provider: Some(lsp::SignatureHelpOptions {
                ..Default::default()
            }),
            ..Default::default()
        },
        cx,
    )
    .await;

    let language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            brackets: BracketPairConfig {
                pairs: vec![
                    BracketPair {
                        start: "{".to_string(),
                        end: "}".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "(".to_string(),
                        end: ")".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "/*".to_string(),
                        end: " */".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "[".to_string(),
                        end: "]".to_string(),
                        close: false,
                        surround: false,
                        newline: true,
                    },
                    BracketPair {
                        start: "\"".to_string(),
                        end: "\"".to_string(),
                        close: true,
                        surround: true,
                        newline: false,
                    },
                    BracketPair {
                        start: "<".to_string(),
                        end: ">".to_string(),
                        close: false,
                        surround: true,
                        newline: true,
                    },
                ],
                ..Default::default()
            },
            autoclose_before: "})]".to_string(),
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    );
    let language = Arc::new(language);

    cx.language_registry().add(language.clone());
    cx.update_buffer(|buffer, cx| {
        buffer.set_language(Some(language), cx);
    });

    cx.set_state(
        &r#"
            fn main() {
                sampleˇ
            }
        "#
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.handle_input("(", window, cx);
    });
    cx.assert_editor_state(
        &"
            fn main() {
                sample(ˇ)
            }
        "
        .unindent(),
    );

    let mocked_response = lsp::SignatureHelp {
        signatures: vec![lsp::SignatureInformation {
            label: "fn sample(param1: u8, param2: u8)".to_string(),
            documentation: None,
            parameters: Some(vec![
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("param1: u8".to_string()),
                    documentation: None,
                },
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("param2: u8".to_string()),
                    documentation: None,
                },
            ]),
            active_parameter: None,
        }],
        active_signature: Some(0),
        active_parameter: Some(0),
    };
    handle_signature_help_request(&mut cx, mocked_response).await;

    cx.condition(|editor, _| editor.signature_help_state.is_shown())
        .await;

    cx.editor(|editor, _, _| {
        let signature_help_state = editor.signature_help_state.popover().cloned();
        assert_eq!(
            signature_help_state.unwrap().label,
            "param1: u8, param2: u8"
        );
    });
}

#[gpui::test]
async fn test_handle_input_with_different_show_signature_settings(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|settings, cx| {
            settings.update_user_settings::<EditorSettings>(cx, |settings| {
                settings.auto_signature_help = Some(false);
                settings.show_signature_help_after_edits = Some(false);
            });
        });
    });

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            signature_help_provider: Some(lsp::SignatureHelpOptions {
                ..Default::default()
            }),
            ..Default::default()
        },
        cx,
    )
    .await;

    let language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            brackets: BracketPairConfig {
                pairs: vec![
                    BracketPair {
                        start: "{".to_string(),
                        end: "}".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "(".to_string(),
                        end: ")".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "/*".to_string(),
                        end: " */".to_string(),
                        close: true,
                        surround: true,
                        newline: true,
                    },
                    BracketPair {
                        start: "[".to_string(),
                        end: "]".to_string(),
                        close: false,
                        surround: false,
                        newline: true,
                    },
                    BracketPair {
                        start: "\"".to_string(),
                        end: "\"".to_string(),
                        close: true,
                        surround: true,
                        newline: false,
                    },
                    BracketPair {
                        start: "<".to_string(),
                        end: ">".to_string(),
                        close: false,
                        surround: true,
                        newline: true,
                    },
                ],
                ..Default::default()
            },
            autoclose_before: "})]".to_string(),
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    );
    let language = Arc::new(language);

    cx.language_registry().add(language.clone());
    cx.update_buffer(|buffer, cx| {
        buffer.set_language(Some(language), cx);
    });

    // Ensure that signature_help is not called when no signature help is enabled.
    cx.set_state(
        &r#"
            fn main() {
                sampleˇ
            }
        "#
        .unindent(),
    );
    cx.update_editor(|editor, window, cx| {
        editor.handle_input("(", window, cx);
    });
    cx.assert_editor_state(
        &"
            fn main() {
                sample(ˇ)
            }
        "
        .unindent(),
    );
    cx.editor(|editor, _, _| {
        assert!(editor.signature_help_state.task().is_none());
    });

    let mocked_response = lsp::SignatureHelp {
        signatures: vec![lsp::SignatureInformation {
            label: "fn sample(param1: u8, param2: u8)".to_string(),
            documentation: None,
            parameters: Some(vec![
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("param1: u8".to_string()),
                    documentation: None,
                },
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("param2: u8".to_string()),
                    documentation: None,
                },
            ]),
            active_parameter: None,
        }],
        active_signature: Some(0),
        active_parameter: Some(0),
    };

    // Ensure that signature_help is called when enabled afte edits
    cx.update(|_, cx| {
        cx.update_global::<SettingsStore, _>(|settings, cx| {
            settings.update_user_settings::<EditorSettings>(cx, |settings| {
                settings.auto_signature_help = Some(false);
                settings.show_signature_help_after_edits = Some(true);
            });
        });
    });
    cx.set_state(
        &r#"
            fn main() {
                sampleˇ
            }
        "#
        .unindent(),
    );
    cx.update_editor(|editor, window, cx| {
        editor.handle_input("(", window, cx);
    });
    cx.assert_editor_state(
        &"
            fn main() {
                sample(ˇ)
            }
        "
        .unindent(),
    );
    handle_signature_help_request(&mut cx, mocked_response.clone()).await;
    cx.condition(|editor, _| editor.signature_help_state.is_shown())
        .await;
    cx.update_editor(|editor, _, _| {
        let signature_help_state = editor.signature_help_state.popover().cloned();
        assert!(signature_help_state.is_some());
        assert_eq!(
            signature_help_state.unwrap().label,
            "param1: u8, param2: u8"
        );
        editor.signature_help_state = SignatureHelpState::default();
    });

    // Ensure that signature_help is called when auto signature help override is enabled
    cx.update(|_, cx| {
        cx.update_global::<SettingsStore, _>(|settings, cx| {
            settings.update_user_settings::<EditorSettings>(cx, |settings| {
                settings.auto_signature_help = Some(true);
                settings.show_signature_help_after_edits = Some(false);
            });
        });
    });
    cx.set_state(
        &r#"
            fn main() {
                sampleˇ
            }
        "#
        .unindent(),
    );
    cx.update_editor(|editor, window, cx| {
        editor.handle_input("(", window, cx);
    });
    cx.assert_editor_state(
        &"
            fn main() {
                sample(ˇ)
            }
        "
        .unindent(),
    );
    handle_signature_help_request(&mut cx, mocked_response).await;
    cx.condition(|editor, _| editor.signature_help_state.is_shown())
        .await;
    cx.editor(|editor, _, _| {
        let signature_help_state = editor.signature_help_state.popover().cloned();
        assert!(signature_help_state.is_some());
        assert_eq!(
            signature_help_state.unwrap().label,
            "param1: u8, param2: u8"
        );
    });
}

#[gpui::test]
async fn test_signature_help(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    cx.update(|cx| {
        cx.update_global::<SettingsStore, _>(|settings, cx| {
            settings.update_user_settings::<EditorSettings>(cx, |settings| {
                settings.auto_signature_help = Some(true);
            });
        });
    });

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            signature_help_provider: Some(lsp::SignatureHelpOptions {
                ..Default::default()
            }),
            ..Default::default()
        },
        cx,
    )
    .await;

    // A test that directly calls `show_signature_help`
    cx.update_editor(|editor, window, cx| {
        editor.show_signature_help(&ShowSignatureHelp, window, cx);
    });

    let mocked_response = lsp::SignatureHelp {
        signatures: vec![lsp::SignatureInformation {
            label: "fn sample(param1: u8, param2: u8)".to_string(),
            documentation: None,
            parameters: Some(vec![
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("param1: u8".to_string()),
                    documentation: None,
                },
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("param2: u8".to_string()),
                    documentation: None,
                },
            ]),
            active_parameter: None,
        }],
        active_signature: Some(0),
        active_parameter: Some(0),
    };
    handle_signature_help_request(&mut cx, mocked_response).await;

    cx.condition(|editor, _| editor.signature_help_state.is_shown())
        .await;

    cx.editor(|editor, _, _| {
        let signature_help_state = editor.signature_help_state.popover().cloned();
        assert!(signature_help_state.is_some());
        assert_eq!(
            signature_help_state.unwrap().label,
            "param1: u8, param2: u8"
        );
    });

    // When exiting outside from inside the brackets, `signature_help` is closed.
    cx.set_state(indoc! {"
        fn main() {
            sample(ˇ);
        }

        fn sample(param1: u8, param2: u8) {}
    "});

    cx.update_editor(|editor, window, cx| {
        editor.change_selections(None, window, cx, |s| s.select_ranges([0..0]));
    });

    let mocked_response = lsp::SignatureHelp {
        signatures: Vec::new(),
        active_signature: None,
        active_parameter: None,
    };
    handle_signature_help_request(&mut cx, mocked_response).await;

    cx.condition(|editor, _| !editor.signature_help_state.is_shown())
        .await;

    cx.editor(|editor, _, _| {
        assert!(!editor.signature_help_state.is_shown());
    });

    // When entering inside the brackets from outside, `show_signature_help` is automatically called.
    cx.set_state(indoc! {"
        fn main() {
            sample(ˇ);
        }

        fn sample(param1: u8, param2: u8) {}
    "});

    let mocked_response = lsp::SignatureHelp {
        signatures: vec![lsp::SignatureInformation {
            label: "fn sample(param1: u8, param2: u8)".to_string(),
            documentation: None,
            parameters: Some(vec![
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("param1: u8".to_string()),
                    documentation: None,
                },
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("param2: u8".to_string()),
                    documentation: None,
                },
            ]),
            active_parameter: None,
        }],
        active_signature: Some(0),
        active_parameter: Some(0),
    };
    handle_signature_help_request(&mut cx, mocked_response.clone()).await;
    cx.condition(|editor, _| editor.signature_help_state.is_shown())
        .await;
    cx.editor(|editor, _, _| {
        assert!(editor.signature_help_state.is_shown());
    });

    // Restore the popover with more parameter input
    cx.set_state(indoc! {"
        fn main() {
            sample(param1, param2ˇ);
        }

        fn sample(param1: u8, param2: u8) {}
    "});

    let mocked_response = lsp::SignatureHelp {
        signatures: vec![lsp::SignatureInformation {
            label: "fn sample(param1: u8, param2: u8)".to_string(),
            documentation: None,
            parameters: Some(vec![
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("param1: u8".to_string()),
                    documentation: None,
                },
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("param2: u8".to_string()),
                    documentation: None,
                },
            ]),
            active_parameter: None,
        }],
        active_signature: Some(0),
        active_parameter: Some(1),
    };
    handle_signature_help_request(&mut cx, mocked_response.clone()).await;
    cx.condition(|editor, _| editor.signature_help_state.is_shown())
        .await;

    // When selecting a range, the popover is gone.
    // Avoid using `cx.set_state` to not actually edit the document, just change its selections.
    cx.update_editor(|editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges(Some(Point::new(1, 25)..Point::new(1, 19)));
        })
    });
    cx.assert_editor_state(indoc! {"
        fn main() {
            sample(param1, «ˇparam2»);
        }

        fn sample(param1: u8, param2: u8) {}
    "});
    cx.editor(|editor, _, _| {
        assert!(!editor.signature_help_state.is_shown());
    });

    // When unselecting again, the popover is back if within the brackets.
    cx.update_editor(|editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges(Some(Point::new(1, 19)..Point::new(1, 19)));
        })
    });
    cx.assert_editor_state(indoc! {"
        fn main() {
            sample(param1, ˇparam2);
        }

        fn sample(param1: u8, param2: u8) {}
    "});
    handle_signature_help_request(&mut cx, mocked_response).await;
    cx.condition(|editor, _| editor.signature_help_state.is_shown())
        .await;
    cx.editor(|editor, _, _| {
        assert!(editor.signature_help_state.is_shown());
    });

    // Test to confirm that SignatureHelp does not appear after deselecting multiple ranges when it was hidden by pressing Escape.
    cx.update_editor(|editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges(Some(Point::new(0, 0)..Point::new(0, 0)));
            s.select_ranges(Some(Point::new(1, 19)..Point::new(1, 19)));
        })
    });
    cx.assert_editor_state(indoc! {"
        fn main() {
            sample(param1, ˇparam2);
        }

        fn sample(param1: u8, param2: u8) {}
    "});

    let mocked_response = lsp::SignatureHelp {
        signatures: vec![lsp::SignatureInformation {
            label: "fn sample(param1: u8, param2: u8)".to_string(),
            documentation: None,
            parameters: Some(vec![
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("param1: u8".to_string()),
                    documentation: None,
                },
                lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("param2: u8".to_string()),
                    documentation: None,
                },
            ]),
            active_parameter: None,
        }],
        active_signature: Some(0),
        active_parameter: Some(1),
    };
    handle_signature_help_request(&mut cx, mocked_response.clone()).await;
    cx.condition(|editor, _| editor.signature_help_state.is_shown())
        .await;
    cx.update_editor(|editor, _, cx| {
        editor.hide_signature_help(cx, SignatureHelpHiddenBy::Escape);
    });
    cx.condition(|editor, _| !editor.signature_help_state.is_shown())
        .await;
    cx.update_editor(|editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges(Some(Point::new(1, 25)..Point::new(1, 19)));
        })
    });
    cx.assert_editor_state(indoc! {"
        fn main() {
            sample(param1, «ˇparam2»);
        }

        fn sample(param1: u8, param2: u8) {}
    "});
    cx.update_editor(|editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges(Some(Point::new(1, 19)..Point::new(1, 19)));
        })
    });
    cx.assert_editor_state(indoc! {"
        fn main() {
            sample(param1, ˇparam2);
        }

        fn sample(param1: u8, param2: u8) {}
    "});
    cx.condition(|editor, _| !editor.signature_help_state.is_shown()) // because hidden by escape
        .await;
}

#[gpui::test]
async fn test_completion_mode(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            completion_provider: Some(lsp::CompletionOptions {
                resolve_provider: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        },
        cx,
    )
    .await;

    struct Run {
        run_description: &'static str,
        initial_state: String,
        buffer_marked_text: String,
        completion_text: &'static str,
        expected_with_insert_mode: String,
        expected_with_replace_mode: String,
        expected_with_replace_subsequence_mode: String,
        expected_with_replace_suffix_mode: String,
    }

    let runs = [
        Run {
            run_description: "Start of word matches completion text",
            initial_state: "before ediˇ after".into(),
            buffer_marked_text: "before <edi|> after".into(),
            completion_text: "editor",
            expected_with_insert_mode: "before editorˇ after".into(),
            expected_with_replace_mode: "before editorˇ after".into(),
            expected_with_replace_subsequence_mode: "before editorˇ after".into(),
            expected_with_replace_suffix_mode: "before editorˇ after".into(),
        },
        Run {
            run_description: "Accept same text at the middle of the word",
            initial_state: "before ediˇtor after".into(),
            buffer_marked_text: "before <edi|tor> after".into(),
            completion_text: "editor",
            expected_with_insert_mode: "before editorˇtor after".into(),
            expected_with_replace_mode: "before editorˇ after".into(),
            expected_with_replace_subsequence_mode: "before editorˇ after".into(),
            expected_with_replace_suffix_mode: "before editorˇ after".into(),
        },
        Run {
            run_description: "End of word matches completion text -- cursor at end",
            initial_state: "before torˇ after".into(),
            buffer_marked_text: "before <tor|> after".into(),
            completion_text: "editor",
            expected_with_insert_mode: "before editorˇ after".into(),
            expected_with_replace_mode: "before editorˇ after".into(),
            expected_with_replace_subsequence_mode: "before editorˇ after".into(),
            expected_with_replace_suffix_mode: "before editorˇ after".into(),
        },
        Run {
            run_description: "End of word matches completion text -- cursor at start",
            initial_state: "before ˇtor after".into(),
            buffer_marked_text: "before <|tor> after".into(),
            completion_text: "editor",
            expected_with_insert_mode: "before editorˇtor after".into(),
            expected_with_replace_mode: "before editorˇ after".into(),
            expected_with_replace_subsequence_mode: "before editorˇ after".into(),
            expected_with_replace_suffix_mode: "before editorˇ after".into(),
        },
        Run {
            run_description: "Prepend text containing whitespace",
            initial_state: "pˇfield: bool".into(),
            buffer_marked_text: "<p|field>: bool".into(),
            completion_text: "pub ",
            expected_with_insert_mode: "pub ˇfield: bool".into(),
            expected_with_replace_mode: "pub ˇ: bool".into(),
            expected_with_replace_subsequence_mode: "pub ˇfield: bool".into(),
            expected_with_replace_suffix_mode: "pub ˇfield: bool".into(),
        },
        Run {
            run_description: "Add element to start of list",
            initial_state: "[element_ˇelement_2]".into(),
            buffer_marked_text: "[<element_|element_2>]".into(),
            completion_text: "element_1",
            expected_with_insert_mode: "[element_1ˇelement_2]".into(),
            expected_with_replace_mode: "[element_1ˇ]".into(),
            expected_with_replace_subsequence_mode: "[element_1ˇelement_2]".into(),
            expected_with_replace_suffix_mode: "[element_1ˇelement_2]".into(),
        },
        Run {
            run_description: "Add element to start of list -- first and second elements are equal",
            initial_state: "[elˇelement]".into(),
            buffer_marked_text: "[<el|element>]".into(),
            completion_text: "element",
            expected_with_insert_mode: "[elementˇelement]".into(),
            expected_with_replace_mode: "[elementˇ]".into(),
            expected_with_replace_subsequence_mode: "[elementˇelement]".into(),
            expected_with_replace_suffix_mode: "[elementˇ]".into(),
        },
        Run {
            run_description: "Ends with matching suffix",
            initial_state: "SubˇError".into(),
            buffer_marked_text: "<Sub|Error>".into(),
            completion_text: "SubscriptionError",
            expected_with_insert_mode: "SubscriptionErrorˇError".into(),
            expected_with_replace_mode: "SubscriptionErrorˇ".into(),
            expected_with_replace_subsequence_mode: "SubscriptionErrorˇ".into(),
            expected_with_replace_suffix_mode: "SubscriptionErrorˇ".into(),
        },
        Run {
            run_description: "Suffix is a subsequence -- contiguous",
            initial_state: "SubˇErr".into(),
            buffer_marked_text: "<Sub|Err>".into(),
            completion_text: "SubscriptionError",
            expected_with_insert_mode: "SubscriptionErrorˇErr".into(),
            expected_with_replace_mode: "SubscriptionErrorˇ".into(),
            expected_with_replace_subsequence_mode: "SubscriptionErrorˇ".into(),
            expected_with_replace_suffix_mode: "SubscriptionErrorˇErr".into(),
        },
        Run {
            run_description: "Suffix is a subsequence -- non-contiguous -- replace intended",
            initial_state: "Suˇscrirr".into(),
            buffer_marked_text: "<Su|scrirr>".into(),
            completion_text: "SubscriptionError",
            expected_with_insert_mode: "SubscriptionErrorˇscrirr".into(),
            expected_with_replace_mode: "SubscriptionErrorˇ".into(),
            expected_with_replace_subsequence_mode: "SubscriptionErrorˇ".into(),
            expected_with_replace_suffix_mode: "SubscriptionErrorˇscrirr".into(),
        },
        Run {
            run_description: "Suffix is a subsequence -- non-contiguous -- replace unintended",
            initial_state: "foo(indˇix)".into(),
            buffer_marked_text: "foo(<ind|ix>)".into(),
            completion_text: "node_index",
            expected_with_insert_mode: "foo(node_indexˇix)".into(),
            expected_with_replace_mode: "foo(node_indexˇ)".into(),
            expected_with_replace_subsequence_mode: "foo(node_indexˇix)".into(),
            expected_with_replace_suffix_mode: "foo(node_indexˇix)".into(),
        },
    ];

    for run in runs {
        let run_variations = [
            (LspInsertMode::Insert, run.expected_with_insert_mode),
            (LspInsertMode::Replace, run.expected_with_replace_mode),
            (
                LspInsertMode::ReplaceSubsequence,
                run.expected_with_replace_subsequence_mode,
            ),
            (
                LspInsertMode::ReplaceSuffix,
                run.expected_with_replace_suffix_mode,
            ),
        ];

        for (lsp_insert_mode, expected_text) in run_variations {
            eprintln!(
                "run = {:?}, mode = {lsp_insert_mode:.?}",
                run.run_description,
            );

            update_test_language_settings(&mut cx, |settings| {
                settings.defaults.completions = Some(CompletionSettings {
                    lsp_insert_mode,
                    words: WordsCompletionMode::Disabled,
                    lsp: true,
                    lsp_fetch_timeout_ms: 0,
                });
            });

            cx.set_state(&run.initial_state);
            cx.update_editor(|editor, window, cx| {
                editor.show_completions(&ShowCompletions { trigger: None }, window, cx);
            });

            let counter = Arc::new(AtomicUsize::new(0));
            handle_completion_request_with_insert_and_replace(
                &mut cx,
                &run.buffer_marked_text,
                vec![run.completion_text],
                counter.clone(),
            )
            .await;
            cx.condition(|editor, _| editor.context_menu_visible())
                .await;
            assert_eq!(counter.load(atomic::Ordering::Acquire), 1);

            let apply_additional_edits = cx.update_editor(|editor, window, cx| {
                editor
                    .confirm_completion(&ConfirmCompletion::default(), window, cx)
                    .unwrap()
            });
            cx.assert_editor_state(&expected_text);
            handle_resolve_completion_request(&mut cx, None).await;
            apply_additional_edits.await.unwrap();
        }
    }
}

#[gpui::test]
async fn test_completion_with_mode_specified_by_action(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            completion_provider: Some(lsp::CompletionOptions {
                resolve_provider: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        },
        cx,
    )
    .await;

    let initial_state = "SubˇError";
    let buffer_marked_text = "<Sub|Error>";
    let completion_text = "SubscriptionError";
    let expected_with_insert_mode = "SubscriptionErrorˇError";
    let expected_with_replace_mode = "SubscriptionErrorˇ";

    update_test_language_settings(&mut cx, |settings| {
        settings.defaults.completions = Some(CompletionSettings {
            words: WordsCompletionMode::Disabled,
            // set the opposite here to ensure that the action is overriding the default behavior
            lsp_insert_mode: LspInsertMode::Insert,
            lsp: true,
            lsp_fetch_timeout_ms: 0,
        });
    });

    cx.set_state(initial_state);
    cx.update_editor(|editor, window, cx| {
        editor.show_completions(&ShowCompletions { trigger: None }, window, cx);
    });

    let counter = Arc::new(AtomicUsize::new(0));
    handle_completion_request_with_insert_and_replace(
        &mut cx,
        &buffer_marked_text,
        vec![completion_text],
        counter.clone(),
    )
    .await;
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    assert_eq!(counter.load(atomic::Ordering::Acquire), 1);

    let apply_additional_edits = cx.update_editor(|editor, window, cx| {
        editor
            .confirm_completion_replace(&ConfirmCompletionReplace, window, cx)
            .unwrap()
    });
    cx.assert_editor_state(&expected_with_replace_mode);
    handle_resolve_completion_request(&mut cx, None).await;
    apply_additional_edits.await.unwrap();

    update_test_language_settings(&mut cx, |settings| {
        settings.defaults.completions = Some(CompletionSettings {
            words: WordsCompletionMode::Disabled,
            // set the opposite here to ensure that the action is overriding the default behavior
            lsp_insert_mode: LspInsertMode::Replace,
            lsp: true,
            lsp_fetch_timeout_ms: 0,
        });
    });

    cx.set_state(initial_state);
    cx.update_editor(|editor, window, cx| {
        editor.show_completions(&ShowCompletions { trigger: None }, window, cx);
    });
    handle_completion_request_with_insert_and_replace(
        &mut cx,
        &buffer_marked_text,
        vec![completion_text],
        counter.clone(),
    )
    .await;
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    assert_eq!(counter.load(atomic::Ordering::Acquire), 2);

    let apply_additional_edits = cx.update_editor(|editor, window, cx| {
        editor
            .confirm_completion_insert(&ConfirmCompletionInsert, window, cx)
            .unwrap()
    });
    cx.assert_editor_state(&expected_with_insert_mode);
    handle_resolve_completion_request(&mut cx, None).await;
    apply_additional_edits.await.unwrap();
}

#[gpui::test]
async fn test_completion_replacing_surrounding_text_with_multicursors(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            completion_provider: Some(lsp::CompletionOptions {
                resolve_provider: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        },
        cx,
    )
    .await;

    // scenario: surrounding text matches completion text
    let completion_text = "to_offset";
    let initial_state = indoc! {"
        1. buf.to_offˇsuffix
        2. buf.to_offˇsuf
        3. buf.to_offˇfix
        4. buf.to_offˇ
        5. into_offˇensive
        6. ˇsuffix
        7. let ˇ //
        8. aaˇzz
        9. buf.to_off«zzzzzˇ»suffix
        10. buf.«ˇzzzzz»suffix
        11. to_off«ˇzzzzz»

        buf.to_offˇsuffix  // newest cursor
    "};
    let completion_marked_buffer = indoc! {"
        1. buf.to_offsuffix
        2. buf.to_offsuf
        3. buf.to_offfix
        4. buf.to_off
        5. into_offensive
        6. suffix
        7. let  //
        8. aazz
        9. buf.to_offzzzzzsuffix
        10. buf.zzzzzsuffix
        11. to_offzzzzz

        buf.<to_off|suffix>  // newest cursor
    "};
    let expected = indoc! {"
        1. buf.to_offsetˇ
        2. buf.to_offsetˇsuf
        3. buf.to_offsetˇfix
        4. buf.to_offsetˇ
        5. into_offsetˇensive
        6. to_offsetˇsuffix
        7. let to_offsetˇ //
        8. aato_offsetˇzz
        9. buf.to_offsetˇ
        10. buf.to_offsetˇsuffix
        11. to_offsetˇ

        buf.to_offsetˇ  // newest cursor
    "};
    cx.set_state(initial_state);
    cx.update_editor(|editor, window, cx| {
        editor.show_completions(&ShowCompletions { trigger: None }, window, cx);
    });
    handle_completion_request_with_insert_and_replace(
        &mut cx,
        completion_marked_buffer,
        vec![completion_text],
        Arc::new(AtomicUsize::new(0)),
    )
    .await;
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    let apply_additional_edits = cx.update_editor(|editor, window, cx| {
        editor
            .confirm_completion_replace(&ConfirmCompletionReplace, window, cx)
            .unwrap()
    });
    cx.assert_editor_state(expected);
    handle_resolve_completion_request(&mut cx, None).await;
    apply_additional_edits.await.unwrap();

    // scenario: surrounding text matches surroundings of newest cursor, inserting at the end
    let completion_text = "foo_and_bar";
    let initial_state = indoc! {"
        1. ooanbˇ
        2. zooanbˇ
        3. ooanbˇz
        4. zooanbˇz
        5. ooanˇ
        6. oanbˇ

        ooanbˇ
    "};
    let completion_marked_buffer = indoc! {"
        1. ooanb
        2. zooanb
        3. ooanbz
        4. zooanbz
        5. ooan
        6. oanb

        <ooanb|>
    "};
    let expected = indoc! {"
        1. foo_and_barˇ
        2. zfoo_and_barˇ
        3. foo_and_barˇz
        4. zfoo_and_barˇz
        5. ooanfoo_and_barˇ
        6. oanbfoo_and_barˇ

        foo_and_barˇ
    "};
    cx.set_state(initial_state);
    cx.update_editor(|editor, window, cx| {
        editor.show_completions(&ShowCompletions { trigger: None }, window, cx);
    });
    handle_completion_request_with_insert_and_replace(
        &mut cx,
        completion_marked_buffer,
        vec![completion_text],
        Arc::new(AtomicUsize::new(0)),
    )
    .await;
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    let apply_additional_edits = cx.update_editor(|editor, window, cx| {
        editor
            .confirm_completion_replace(&ConfirmCompletionReplace, window, cx)
            .unwrap()
    });
    cx.assert_editor_state(expected);
    handle_resolve_completion_request(&mut cx, None).await;
    apply_additional_edits.await.unwrap();

    // scenario: surrounding text matches surroundings of newest cursor, inserted at the middle
    // (expects the same as if it was inserted at the end)
    let completion_text = "foo_and_bar";
    let initial_state = indoc! {"
        1. ooˇanb
        2. zooˇanb
        3. ooˇanbz
        4. zooˇanbz

        ooˇanb
    "};
    let completion_marked_buffer = indoc! {"
        1. ooanb
        2. zooanb
        3. ooanbz
        4. zooanbz

        <oo|anb>
    "};
    let expected = indoc! {"
        1. foo_and_barˇ
        2. zfoo_and_barˇ
        3. foo_and_barˇz
        4. zfoo_and_barˇz

        foo_and_barˇ
    "};
    cx.set_state(initial_state);
    cx.update_editor(|editor, window, cx| {
        editor.show_completions(&ShowCompletions { trigger: None }, window, cx);
    });
    handle_completion_request_with_insert_and_replace(
        &mut cx,
        completion_marked_buffer,
        vec![completion_text],
        Arc::new(AtomicUsize::new(0)),
    )
    .await;
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    let apply_additional_edits = cx.update_editor(|editor, window, cx| {
        editor
            .confirm_completion_replace(&ConfirmCompletionReplace, window, cx)
            .unwrap()
    });
    cx.assert_editor_state(expected);
    handle_resolve_completion_request(&mut cx, None).await;
    apply_additional_edits.await.unwrap();
}

// This used to crash
#[gpui::test]
async fn test_completion_in_multibuffer_with_replace_range(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let buffer_text = indoc! {"
        fn main() {
            10.satu;

            //
            // separate cursors so they open in different excerpts (manually reproducible)
            //

            10.satu20;
        }
    "};
    let multibuffer_text_with_selections = indoc! {"
        fn main() {
            10.satuˇ;

            //

            //

            10.satuˇ20;
        }
    "};
    let expected_multibuffer = indoc! {"
        fn main() {
            10.saturating_sub()ˇ;

            //

            //

            10.saturating_sub()ˇ;
        }
    "};

    let first_excerpt_end = buffer_text.find("//").unwrap() + 3;
    let second_excerpt_end = buffer_text.rfind("//").unwrap() - 4;

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/a"),
        json!({
            "main.rs": buffer_text,
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    resolve_provider: None,
                    ..lsp::CompletionOptions::default()
                }),
                ..lsp::ServerCapabilities::default()
            },
            ..FakeLspAdapter::default()
        },
    );
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/a/main.rs"), cx)
        })
        .await
        .unwrap();

    let multi_buffer = cx.new(|cx| {
        let mut multi_buffer = MultiBuffer::new(Capability::ReadWrite);
        multi_buffer.push_excerpts(
            buffer.clone(),
            [ExcerptRange::new(0..first_excerpt_end)],
            cx,
        );
        multi_buffer.push_excerpts(
            buffer.clone(),
            [ExcerptRange::new(second_excerpt_end..buffer_text.len())],
            cx,
        );
        multi_buffer
    });

    let editor = workspace
        .update(cx, |_, window, cx| {
            cx.new(|cx| {
                Editor::new(
                    EditorMode::Full {
                        scale_ui_elements_with_buffer_font_size: false,
                        show_active_line_background: false,
                        sized_by_content: false,
                    },
                    multi_buffer.clone(),
                    Some(project.clone()),
                    window,
                    cx,
                )
            })
        })
        .unwrap();

    let pane = workspace
        .update(cx, |workspace, _, _| workspace.active_pane().clone())
        .unwrap();
    pane.update_in(cx, |pane, window, cx| {
        pane.add_item(Box::new(editor.clone()), true, true, None, window, cx);
    });

    let fake_server = fake_servers.next().await.unwrap();

    editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([
                Point::new(1, 11)..Point::new(1, 11),
                Point::new(7, 11)..Point::new(7, 11),
            ])
        });

        assert_text_with_selections(editor, multibuffer_text_with_selections, cx);
    });

    editor.update_in(cx, |editor, window, cx| {
        editor.show_completions(&ShowCompletions { trigger: None }, window, cx);
    });

    fake_server
        .set_request_handler::<lsp::request::Completion, _, _>(move |_, _| async move {
            let completion_item = lsp::CompletionItem {
                label: "saturating_sub()".into(),
                text_edit: Some(lsp::CompletionTextEdit::InsertAndReplace(
                    lsp::InsertReplaceEdit {
                        new_text: "saturating_sub()".to_owned(),
                        insert: lsp::Range::new(
                            lsp::Position::new(7, 7),
                            lsp::Position::new(7, 11),
                        ),
                        replace: lsp::Range::new(
                            lsp::Position::new(7, 7),
                            lsp::Position::new(7, 13),
                        ),
                    },
                )),
                ..lsp::CompletionItem::default()
            };

            Ok(Some(lsp::CompletionResponse::Array(vec![completion_item])))
        })
        .next()
        .await
        .unwrap();

    cx.condition(&editor, |editor, _| editor.context_menu_visible())
        .await;

    editor
        .update_in(cx, |editor, window, cx| {
            editor
                .confirm_completion_replace(&ConfirmCompletionReplace, window, cx)
                .unwrap()
        })
        .await
        .unwrap();

    editor.update(cx, |editor, cx| {
        assert_text_with_selections(editor, expected_multibuffer, cx);
    })
}

#[gpui::test]
async fn test_completion(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            completion_provider: Some(lsp::CompletionOptions {
                trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                resolve_provider: Some(true),
                ..Default::default()
            }),
            signature_help_provider: Some(lsp::SignatureHelpOptions::default()),
            ..Default::default()
        },
        cx,
    )
    .await;
    let counter = Arc::new(AtomicUsize::new(0));

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
        counter.clone(),
    )
    .await;
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    assert_eq!(counter.load(atomic::Ordering::Acquire), 1);

    let _handler = handle_signature_help_request(
        &mut cx,
        lsp::SignatureHelp {
            signatures: vec![lsp::SignatureInformation {
                label: "test signature".to_string(),
                documentation: None,
                parameters: Some(vec![lsp::ParameterInformation {
                    label: lsp::ParameterLabel::Simple("foo: u8".to_string()),
                    documentation: None,
                }]),
                active_parameter: None,
            }],
            active_signature: None,
            active_parameter: None,
        },
    );
    cx.update_editor(|editor, window, cx| {
        assert!(
            !editor.signature_help_state.is_shown(),
            "No signature help was called for"
        );
        editor.show_signature_help(&ShowSignatureHelp, window, cx);
    });
    cx.run_until_parked();
    cx.update_editor(|editor, _, _| {
        assert!(
            !editor.signature_help_state.is_shown(),
            "No signature help should be shown when completions menu is open"
        );
    });

    let apply_additional_edits = cx.update_editor(|editor, window, cx| {
        editor.context_menu_next(&Default::default(), window, cx);
        editor
            .confirm_completion(&ConfirmCompletion::default(), window, cx)
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
    assert!(cx.editor(|e, _, _| e.context_menu.borrow_mut().is_none()));
    cx.simulate_keystroke("s");
    assert!(cx.editor(|e, _, _| e.context_menu.borrow_mut().is_none()));

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
        counter.clone(),
    )
    .await;
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    assert_eq!(counter.load(atomic::Ordering::Acquire), 2);

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
        counter.clone(),
    )
    .await;
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    assert_eq!(counter.load(atomic::Ordering::Acquire), 3);

    let apply_additional_edits = cx.update_editor(|editor, window, cx| {
        editor
            .confirm_completion(&ConfirmCompletion::default(), window, cx)
            .unwrap()
    });
    cx.assert_editor_state(indoc! {"
        one.second_completion
        two sixth_completionˇ
        three sixth_completionˇ
        additional edit
    "});

    apply_additional_edits.await.unwrap();

    update_test_language_settings(&mut cx, |settings| {
        settings.defaults.show_completions_on_input = Some(false);
    });
    cx.set_state("editorˇ");
    cx.simulate_keystroke(".");
    assert!(cx.editor(|e, _, _| e.context_menu.borrow_mut().is_none()));
    cx.simulate_keystrokes("c l o");
    cx.assert_editor_state("editor.cloˇ");
    assert!(cx.editor(|e, _, _| e.context_menu.borrow_mut().is_none()));
    cx.update_editor(|editor, window, cx| {
        editor.show_completions(&ShowCompletions { trigger: None }, window, cx);
    });
    handle_completion_request(
        &mut cx,
        "editor.<clo|>",
        vec!["close", "clobber"],
        counter.clone(),
    )
    .await;
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    assert_eq!(counter.load(atomic::Ordering::Acquire), 4);

    let apply_additional_edits = cx.update_editor(|editor, window, cx| {
        editor
            .confirm_completion(&ConfirmCompletion::default(), window, cx)
            .unwrap()
    });
    cx.assert_editor_state("editor.closeˇ");
    handle_resolve_completion_request(&mut cx, None).await;
    apply_additional_edits.await.unwrap();
}

#[gpui::test]
async fn test_word_completion(cx: &mut TestAppContext) {
    let lsp_fetch_timeout_ms = 10;
    init_test(cx, |language_settings| {
        language_settings.defaults.completions = Some(CompletionSettings {
            words: WordsCompletionMode::Fallback,
            lsp: true,
            lsp_fetch_timeout_ms: 10,
            lsp_insert_mode: LspInsertMode::Insert,
        });
    });

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            completion_provider: Some(lsp::CompletionOptions {
                trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                ..lsp::CompletionOptions::default()
            }),
            signature_help_provider: Some(lsp::SignatureHelpOptions::default()),
            ..lsp::ServerCapabilities::default()
        },
        cx,
    )
    .await;

    let throttle_completions = Arc::new(AtomicBool::new(false));

    let lsp_throttle_completions = throttle_completions.clone();
    let _completion_requests_handler =
        cx.lsp
            .server
            .on_request::<lsp::request::Completion, _, _>(move |_, cx| {
                let lsp_throttle_completions = lsp_throttle_completions.clone();
                let cx = cx.clone();
                async move {
                    if lsp_throttle_completions.load(atomic::Ordering::Acquire) {
                        cx.background_executor()
                            .timer(Duration::from_millis(lsp_fetch_timeout_ms * 10))
                            .await;
                    }
                    Ok(Some(lsp::CompletionResponse::Array(vec![
                        lsp::CompletionItem {
                            label: "first".into(),
                            ..lsp::CompletionItem::default()
                        },
                        lsp::CompletionItem {
                            label: "last".into(),
                            ..lsp::CompletionItem::default()
                        },
                    ])))
                }
            });

    cx.set_state(indoc! {"
        oneˇ
        two
        three
    "});
    cx.simulate_keystroke(".");
    cx.executor().run_until_parked();
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    cx.update_editor(|editor, window, cx| {
        if let Some(CodeContextMenu::Completions(menu)) = editor.context_menu.borrow_mut().as_ref()
        {
            assert_eq!(
                completion_menu_entries(&menu),
                &["first", "last"],
                "When LSP server is fast to reply, no fallback word completions are used"
            );
        } else {
            panic!("expected completion menu to be open");
        }
        editor.cancel(&Cancel, window, cx);
    });
    cx.executor().run_until_parked();
    cx.condition(|editor, _| !editor.context_menu_visible())
        .await;

    throttle_completions.store(true, atomic::Ordering::Release);
    cx.simulate_keystroke(".");
    cx.executor()
        .advance_clock(Duration::from_millis(lsp_fetch_timeout_ms * 2));
    cx.executor().run_until_parked();
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    cx.update_editor(|editor, _, _| {
        if let Some(CodeContextMenu::Completions(menu)) = editor.context_menu.borrow_mut().as_ref()
        {
            assert_eq!(completion_menu_entries(&menu), &["one", "three", "two"],
                "When LSP server is slow, document words can be shown instead, if configured accordingly");
        } else {
            panic!("expected completion menu to be open");
        }
    });
}

#[gpui::test]
async fn test_word_completions_do_not_duplicate_lsp_ones(cx: &mut TestAppContext) {
    init_test(cx, |language_settings| {
        language_settings.defaults.completions = Some(CompletionSettings {
            words: WordsCompletionMode::Enabled,
            lsp: true,
            lsp_fetch_timeout_ms: 0,
            lsp_insert_mode: LspInsertMode::Insert,
        });
    });

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            completion_provider: Some(lsp::CompletionOptions {
                trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                ..lsp::CompletionOptions::default()
            }),
            signature_help_provider: Some(lsp::SignatureHelpOptions::default()),
            ..lsp::ServerCapabilities::default()
        },
        cx,
    )
    .await;

    let _completion_requests_handler =
        cx.lsp
            .server
            .on_request::<lsp::request::Completion, _, _>(move |_, _| async move {
                Ok(Some(lsp::CompletionResponse::Array(vec![
                    lsp::CompletionItem {
                        label: "first".into(),
                        ..lsp::CompletionItem::default()
                    },
                    lsp::CompletionItem {
                        label: "last".into(),
                        ..lsp::CompletionItem::default()
                    },
                ])))
            });

    cx.set_state(indoc! {"ˇ
        first
        last
        second
    "});
    cx.simulate_keystroke(".");
    cx.executor().run_until_parked();
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    cx.update_editor(|editor, _, _| {
        if let Some(CodeContextMenu::Completions(menu)) = editor.context_menu.borrow_mut().as_ref()
        {
            assert_eq!(
                completion_menu_entries(&menu),
                &["first", "last", "second"],
                "Word completions that has the same edit as the any of the LSP ones, should not be proposed"
            );
        } else {
            panic!("expected completion menu to be open");
        }
    });
}

#[gpui::test]
async fn test_word_completions_continue_on_typing(cx: &mut TestAppContext) {
    init_test(cx, |language_settings| {
        language_settings.defaults.completions = Some(CompletionSettings {
            words: WordsCompletionMode::Disabled,
            lsp: true,
            lsp_fetch_timeout_ms: 0,
            lsp_insert_mode: LspInsertMode::Insert,
        });
    });

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            completion_provider: Some(lsp::CompletionOptions {
                trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                ..lsp::CompletionOptions::default()
            }),
            signature_help_provider: Some(lsp::SignatureHelpOptions::default()),
            ..lsp::ServerCapabilities::default()
        },
        cx,
    )
    .await;

    let _completion_requests_handler =
        cx.lsp
            .server
            .on_request::<lsp::request::Completion, _, _>(move |_, _| async move {
                panic!("LSP completions should not be queried when dealing with word completions")
            });

    cx.set_state(indoc! {"ˇ
        first
        last
        second
    "});
    cx.update_editor(|editor, window, cx| {
        editor.show_word_completions(&ShowWordCompletions, window, cx);
    });
    cx.executor().run_until_parked();
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    cx.update_editor(|editor, _, _| {
        if let Some(CodeContextMenu::Completions(menu)) = editor.context_menu.borrow_mut().as_ref()
        {
            assert_eq!(
                completion_menu_entries(&menu),
                &["first", "last", "second"],
                "`ShowWordCompletions` action should show word completions"
            );
        } else {
            panic!("expected completion menu to be open");
        }
    });

    cx.simulate_keystroke("l");
    cx.executor().run_until_parked();
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    cx.update_editor(|editor, _, _| {
        if let Some(CodeContextMenu::Completions(menu)) = editor.context_menu.borrow_mut().as_ref()
        {
            assert_eq!(
                completion_menu_entries(&menu),
                &["last"],
                "After showing word completions, further editing should filter them and not query the LSP"
            );
        } else {
            panic!("expected completion menu to be open");
        }
    });
}

#[gpui::test]
async fn test_word_completions_usually_skip_digits(cx: &mut TestAppContext) {
    init_test(cx, |language_settings| {
        language_settings.defaults.completions = Some(CompletionSettings {
            words: WordsCompletionMode::Fallback,
            lsp: false,
            lsp_fetch_timeout_ms: 0,
            lsp_insert_mode: LspInsertMode::Insert,
        });
    });

    let mut cx = EditorLspTestContext::new_rust(lsp::ServerCapabilities::default(), cx).await;

    cx.set_state(indoc! {"ˇ
        0_usize
        let
        33
        4.5f32
    "});
    cx.update_editor(|editor, window, cx| {
        editor.show_completions(&ShowCompletions::default(), window, cx);
    });
    cx.executor().run_until_parked();
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    cx.update_editor(|editor, window, cx| {
        if let Some(CodeContextMenu::Completions(menu)) = editor.context_menu.borrow_mut().as_ref()
        {
            assert_eq!(
                completion_menu_entries(&menu),
                &["let"],
                "With no digits in the completion query, no digits should be in the word completions"
            );
        } else {
            panic!("expected completion menu to be open");
        }
        editor.cancel(&Cancel, window, cx);
    });

    cx.set_state(indoc! {"3ˇ
        0_usize
        let
        3
        33.35f32
    "});
    cx.update_editor(|editor, window, cx| {
        editor.show_completions(&ShowCompletions::default(), window, cx);
    });
    cx.executor().run_until_parked();
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    cx.update_editor(|editor, _, _| {
        if let Some(CodeContextMenu::Completions(menu)) = editor.context_menu.borrow_mut().as_ref()
        {
            assert_eq!(completion_menu_entries(&menu), &["33", "35f32"], "The digit is in the completion query, \
                return matching words with digits (`33`, `35f32`) but exclude query duplicates (`3`)");
        } else {
            panic!("expected completion menu to be open");
        }
    });
}

fn gen_text_edit(params: &CompletionParams, text: &str) -> Option<lsp::CompletionTextEdit> {
    let position = || lsp::Position {
        line: params.text_document_position.position.line,
        character: params.text_document_position.position.character,
    };
    Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
        range: lsp::Range {
            start: position(),
            end: position(),
        },
        new_text: text.to_string(),
    }))
}

#[gpui::test]
async fn test_multiline_completion(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/a"),
        json!({
            "main.ts": "a",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    let typescript_language = Arc::new(Language::new(
        LanguageConfig {
            name: "TypeScript".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["ts".to_string()],
                ..LanguageMatcher::default()
            },
            line_comments: vec!["// ".into()],
            ..LanguageConfig::default()
        },
        Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
    ));
    language_registry.add(typescript_language.clone());
    let mut fake_servers = language_registry.register_fake_lsp(
        "TypeScript",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    ..lsp::CompletionOptions::default()
                }),
                signature_help_provider: Some(lsp::SignatureHelpOptions::default()),
                ..lsp::ServerCapabilities::default()
            },
            // Emulate vtsls label generation
            label_for_completion: Some(Box::new(|item, _| {
                let text = if let Some(description) = item
                    .label_details
                    .as_ref()
                    .and_then(|label_details| label_details.description.as_ref())
                {
                    format!("{} {}", item.label, description)
                } else if let Some(detail) = &item.detail {
                    format!("{} {}", item.label, detail)
                } else {
                    item.label.clone()
                };
                let len = text.len();
                Some(language::CodeLabel {
                    text,
                    runs: Vec::new(),
                    filter_range: 0..len,
                })
            })),
            ..FakeLspAdapter::default()
        },
    );
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let worktree_id = workspace
        .update(cx, |workspace, _window, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        })
        .unwrap();
    let _buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/a/main.ts"), cx)
        })
        .await
        .unwrap();
    let editor = workspace
        .update(cx, |workspace, window, cx| {
            workspace.open_path((worktree_id, "main.ts"), None, true, window, cx)
        })
        .unwrap()
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();
    let fake_server = fake_servers.next().await.unwrap();

    let multiline_label = "StickyHeaderExcerpt {\n            excerpt,\n            next_excerpt_controls_present,\n            next_buffer_row,\n        }: StickyHeaderExcerpt<'_>,";
    let multiline_label_2 = "a\nb\nc\n";
    let multiline_detail = "[]struct {\n\tSignerId\tstruct {\n\t\tIssuer\t\t\tstring\t`json:\"issuer\"`\n\t\tSubjectSerialNumber\"`\n}}";
    let multiline_description = "d\ne\nf\n";
    let multiline_detail_2 = "g\nh\ni\n";

    let mut completion_handle = fake_server.set_request_handler::<lsp::request::Completion, _, _>(
        move |params, _| async move {
            Ok(Some(lsp::CompletionResponse::Array(vec![
                lsp::CompletionItem {
                    label: multiline_label.to_string(),
                    text_edit: gen_text_edit(&params, "new_text_1"),
                    ..lsp::CompletionItem::default()
                },
                lsp::CompletionItem {
                    label: "single line label 1".to_string(),
                    detail: Some(multiline_detail.to_string()),
                    text_edit: gen_text_edit(&params, "new_text_2"),
                    ..lsp::CompletionItem::default()
                },
                lsp::CompletionItem {
                    label: "single line label 2".to_string(),
                    label_details: Some(lsp::CompletionItemLabelDetails {
                        description: Some(multiline_description.to_string()),
                        detail: None,
                    }),
                    text_edit: gen_text_edit(&params, "new_text_2"),
                    ..lsp::CompletionItem::default()
                },
                lsp::CompletionItem {
                    label: multiline_label_2.to_string(),
                    detail: Some(multiline_detail_2.to_string()),
                    text_edit: gen_text_edit(&params, "new_text_3"),
                    ..lsp::CompletionItem::default()
                },
                lsp::CompletionItem {
                    label: "Label with many     spaces and \t but without newlines".to_string(),
                    detail: Some(
                        "Details with many     spaces and \t but without newlines".to_string(),
                    ),
                    text_edit: gen_text_edit(&params, "new_text_4"),
                    ..lsp::CompletionItem::default()
                },
            ])))
        },
    );

    editor.update_in(cx, |editor, window, cx| {
        cx.focus_self(window);
        editor.move_to_end(&MoveToEnd, window, cx);
        editor.handle_input(".", window, cx);
    });
    cx.run_until_parked();
    completion_handle.next().await.unwrap();

    editor.update(cx, |editor, _| {
        assert!(editor.context_menu_visible());
        if let Some(CodeContextMenu::Completions(menu)) = editor.context_menu.borrow_mut().as_ref()
        {
            let completion_labels = menu
                .completions
                .borrow()
                .iter()
                .map(|c| c.label.text.clone())
                .collect::<Vec<_>>();
            assert_eq!(
                completion_labels,
                &[
                    "StickyHeaderExcerpt { excerpt, next_excerpt_controls_present, next_buffer_row, }: StickyHeaderExcerpt<'_>,",
                    "single line label 1 []struct { SignerId struct { Issuer string `json:\"issuer\"` SubjectSerialNumber\"` }}",
                    "single line label 2 d e f ",
                    "a b c g h i ",
                    "Label with many     spaces and \t but without newlines Details with many     spaces and \t but without newlines",
                ],
                "Completion items should have their labels without newlines, also replacing excessive whitespaces. Completion items without newlines should not be altered.",
            );

            for completion in menu
                .completions
                .borrow()
                .iter() {
                    assert_eq!(
                        completion.label.filter_range,
                        0..completion.label.text.len(),
                        "Adjusted completion items should still keep their filter ranges for the entire label. Item: {completion:?}"
                    );
                }
        } else {
            panic!("expected completion menu to be open");
        }
    });
}

#[gpui::test]
async fn test_completion_page_up_down_keys(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            completion_provider: Some(lsp::CompletionOptions {
                trigger_characters: Some(vec![".".to_string()]),
                ..Default::default()
            }),
            ..Default::default()
        },
        cx,
    )
    .await;
    cx.lsp
        .set_request_handler::<lsp::request::Completion, _, _>(move |_, _| async move {
            Ok(Some(lsp::CompletionResponse::Array(vec![
                lsp::CompletionItem {
                    label: "first".into(),
                    ..Default::default()
                },
                lsp::CompletionItem {
                    label: "last".into(),
                    ..Default::default()
                },
            ])))
        });
    cx.set_state("variableˇ");
    cx.simulate_keystroke(".");
    cx.executor().run_until_parked();

    cx.update_editor(|editor, _, _| {
        if let Some(CodeContextMenu::Completions(menu)) = editor.context_menu.borrow_mut().as_ref()
        {
            assert_eq!(completion_menu_entries(&menu), &["first", "last"]);
        } else {
            panic!("expected completion menu to be open");
        }
    });

    cx.update_editor(|editor, window, cx| {
        editor.move_page_down(&MovePageDown::default(), window, cx);
        if let Some(CodeContextMenu::Completions(menu)) = editor.context_menu.borrow_mut().as_ref()
        {
            assert!(
                menu.selected_item == 1,
                "expected PageDown to select the last item from the context menu"
            );
        } else {
            panic!("expected completion menu to stay open after PageDown");
        }
    });

    cx.update_editor(|editor, window, cx| {
        editor.move_page_up(&MovePageUp::default(), window, cx);
        if let Some(CodeContextMenu::Completions(menu)) = editor.context_menu.borrow_mut().as_ref()
        {
            assert!(
                menu.selected_item == 0,
                "expected PageUp to select the first item from the context menu"
            );
        } else {
            panic!("expected completion menu to stay open after PageUp");
        }
    });
}

#[gpui::test]
async fn test_as_is_completions(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            completion_provider: Some(lsp::CompletionOptions {
                ..Default::default()
            }),
            ..Default::default()
        },
        cx,
    )
    .await;
    cx.lsp
        .set_request_handler::<lsp::request::Completion, _, _>(move |_, _| async move {
            Ok(Some(lsp::CompletionResponse::Array(vec![
                lsp::CompletionItem {
                    label: "unsafe".into(),
                    text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                        range: lsp::Range {
                            start: lsp::Position {
                                line: 1,
                                character: 2,
                            },
                            end: lsp::Position {
                                line: 1,
                                character: 3,
                            },
                        },
                        new_text: "unsafe".to_string(),
                    })),
                    insert_text_mode: Some(lsp::InsertTextMode::AS_IS),
                    ..Default::default()
                },
            ])))
        });
    cx.set_state("fn a() {}\n  nˇ");
    cx.executor().run_until_parked();
    cx.update_editor(|editor, window, cx| {
        editor.show_completions(
            &ShowCompletions {
                trigger: Some("\n".into()),
            },
            window,
            cx,
        );
    });
    cx.executor().run_until_parked();

    cx.update_editor(|editor, window, cx| {
        editor.confirm_completion(&Default::default(), window, cx)
    });
    cx.executor().run_until_parked();
    cx.assert_editor_state("fn a() {}\n  unsafeˇ");
}

#[gpui::test]
async fn test_no_duplicated_completion_requests(cx: &mut TestAppContext) {
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

    cx.set_state("fn main() { let a = 2ˇ; }");
    cx.simulate_keystroke(".");
    let completion_item = lsp::CompletionItem {
        label: "Some".into(),
        kind: Some(lsp::CompletionItemKind::SNIPPET),
        detail: Some("Wrap the expression in an `Option::Some`".to_string()),
        documentation: Some(lsp::Documentation::MarkupContent(lsp::MarkupContent {
            kind: lsp::MarkupKind::Markdown,
            value: "```rust\nSome(2)\n```".to_string(),
        })),
        deprecated: Some(false),
        sort_text: Some("Some".to_string()),
        filter_text: Some("Some".to_string()),
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
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();
    let mut request = cx.set_request_handler::<lsp::request::Completion, _, _>(move |_, _, _| {
        let task_completion_item = closure_completion_item.clone();
        counter_clone.fetch_add(1, atomic::Ordering::Release);
        async move {
            Ok(Some(lsp::CompletionResponse::Array(vec![
                task_completion_item,
            ])))
        }
    });

    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    cx.assert_editor_state("fn main() { let a = 2.ˇ; }");
    assert!(request.next().await.is_some());
    assert_eq!(counter.load(atomic::Ordering::Acquire), 1);

    cx.simulate_keystrokes("S o m");
    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    cx.assert_editor_state("fn main() { let a = 2.Somˇ; }");
    assert!(request.next().await.is_some());
    assert!(request.next().await.is_some());
    assert!(request.next().await.is_some());
    request.close();
    assert!(request.next().await.is_none());
    assert_eq!(
        counter.load(atomic::Ordering::Acquire),
        4,
        "With the completions menu open, only one LSP request should happen per input"
    );
}

#[gpui::test]
async fn test_toggle_comment(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;
    let language = Arc::new(Language::new(
        LanguageConfig {
            line_comments: vec!["// ".into(), "//! ".into(), "/// ".into()],
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
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

    cx.update_editor(|e, window, cx| e.toggle_comments(&ToggleComments::default(), window, cx));

    cx.assert_editor_state(indoc! {"
        fn a() {
            «b();
            c();
            ˇ» d();
        }
    "});

    // The comment prefix is inserted at the same column for every line in a
    // selection.
    cx.update_editor(|e, window, cx| e.toggle_comments(&ToggleComments::default(), window, cx));

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

    cx.update_editor(|e, window, cx| e.toggle_comments(&ToggleComments::default(), window, cx));

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

    cx.update_editor(|e, window, cx| e.toggle_comments(&ToggleComments::default(), window, cx));

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

    cx.update_editor(|e, window, cx| e.toggle_comments(&ToggleComments::default(), window, cx));

    cx.assert_editor_state(indoc! {"
        fn a() {
            // «a();

            // c();ˇ»
        }
    "});

    // If a selection includes multiple comment prefixes, all lines are uncommented.
    cx.set_state(indoc! {"
        fn a() {
            «// a();
            /// b();
            //! c();ˇ»
        }
    "});

    cx.update_editor(|e, window, cx| e.toggle_comments(&ToggleComments::default(), window, cx));

    cx.assert_editor_state(indoc! {"
        fn a() {
            «a();
            b();
            c();ˇ»
        }
    "});
}

#[gpui::test]
async fn test_toggle_comment_ignore_indent(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;
    let language = Arc::new(Language::new(
        LanguageConfig {
            line_comments: vec!["// ".into(), "//! ".into(), "/// ".into()],
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    ));
    cx.update_buffer(|buffer, cx| buffer.set_language(Some(language), cx));

    let toggle_comments = &ToggleComments {
        advance_downwards: false,
        ignore_indent: true,
    };

    // If multiple selections intersect a line, the line is only toggled once.
    cx.set_state(indoc! {"
        fn a() {
        //    «b();
        //    c();
        //    ˇ» d();
        }
    "});

    cx.update_editor(|e, window, cx| e.toggle_comments(toggle_comments, window, cx));

    cx.assert_editor_state(indoc! {"
        fn a() {
            «b();
            c();
            ˇ» d();
        }
    "});

    // The comment prefix is inserted at the beginning of each line
    cx.update_editor(|e, window, cx| e.toggle_comments(toggle_comments, window, cx));

    cx.assert_editor_state(indoc! {"
        fn a() {
        //    «b();
        //    c();
        //    ˇ» d();
        }
    "});

    // If a selection ends at the beginning of a line, that line is not toggled.
    cx.set_selections_state(indoc! {"
        fn a() {
        //    b();
        //    «c();
        ˇ»//     d();
        }
    "});

    cx.update_editor(|e, window, cx| e.toggle_comments(toggle_comments, window, cx));

    cx.assert_editor_state(indoc! {"
        fn a() {
        //    b();
            «c();
        ˇ»//     d();
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

    cx.update_editor(|e, window, cx| e.toggle_comments(toggle_comments, window, cx));

    cx.assert_editor_state(indoc! {"
        fn a() {
            a();
            b();
        //ˇ
        }
    "});

    // If a selection span multiple lines, empty lines are not toggled.
    cx.set_state(indoc! {"
        fn a() {
            «a();

            c();ˇ»
        }
    "});

    cx.update_editor(|e, window, cx| e.toggle_comments(toggle_comments, window, cx));

    cx.assert_editor_state(indoc! {"
        fn a() {
        //    «a();

        //    c();ˇ»
        }
    "});

    // If a selection includes multiple comment prefixes, all lines are uncommented.
    cx.set_state(indoc! {"
        fn a() {
        //    «a();
        ///    b();
        //!    c();ˇ»
        }
    "});

    cx.update_editor(|e, window, cx| e.toggle_comments(toggle_comments, window, cx));

    cx.assert_editor_state(indoc! {"
        fn a() {
            «a();
            b();
            c();ˇ»
        }
    "});
}

#[gpui::test]
async fn test_advance_downward_on_toggle_comment(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let language = Arc::new(Language::new(
        LanguageConfig {
            line_comments: vec!["// ".into()],
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    ));

    let mut cx = EditorTestContext::new(cx).await;

    cx.language_registry().add(language.clone());
    cx.update_buffer(|buffer, cx| {
        buffer.set_language(Some(language), cx);
    });

    let toggle_comments = &ToggleComments {
        advance_downwards: true,
        ignore_indent: false,
    };

    // Single cursor on one line -> advance
    // Cursor moves horizontally 3 characters as well on non-blank line
    cx.set_state(indoc!(
        "fn a() {
             ˇdog();
             cat();
        }"
    ));
    cx.update_editor(|editor, window, cx| {
        editor.toggle_comments(toggle_comments, window, cx);
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
    cx.update_editor(|editor, window, cx| {
        editor.toggle_comments(toggle_comments, window, cx);
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
    cx.update_editor(|editor, window, cx| {
        editor.toggle_comments(toggle_comments, window, cx);
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
    cx.update_editor(|editor, window, cx| {
        editor.toggle_comments(toggle_comments, window, cx);
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
    cx.update_editor(|editor, window, cx| {
        editor.toggle_comments(toggle_comments, window, cx);
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
    cx.update_editor(|editor, window, cx| {
        editor.toggle_comments(toggle_comments, window, cx);
    });
    cx.assert_editor_state(indoc!(
        "fn a() {
             // dog();
         ˇ    cat();
        }"
    ));
}

#[gpui::test]
async fn test_toggle_block_comment(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    let html_language = Arc::new(
        Language::new(
            LanguageConfig {
                name: "HTML".into(),
                block_comment: Some(("<!-- ".into(), " -->".into())),
                ..Default::default()
            },
            Some(tree_sitter_html::LANGUAGE.into()),
        )
        .with_injection_query(
            r#"
            (script_element
                (raw_text) @injection.content
                (#set! injection.language "javascript"))
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
        Some(tree_sitter_typescript::LANGUAGE_TSX.into()),
    ));

    cx.language_registry().add(html_language.clone());
    cx.language_registry().add(javascript_language.clone());
    cx.update_buffer(|buffer, cx| {
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
    cx.update_editor(|editor, window, cx| {
        editor.toggle_comments(&ToggleComments::default(), window, cx)
    });
    cx.assert_editor_state(
        &r#"
            <!-- <p>A</p>ˇ -->
            <!-- <p>B</p>ˇ -->
            <!-- <p>C</p>ˇ -->
        "#
        .unindent(),
    );
    cx.update_editor(|editor, window, cx| {
        editor.toggle_comments(&ToggleComments::default(), window, cx)
    });
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

    cx.update_editor(|editor, window, cx| {
        editor.toggle_comments(&ToggleComments::default(), window, cx)
    });
    cx.assert_editor_state(
        &r#"
            <!-- <p>A«</p>
            <p>ˇ»B</p>ˇ -->
            <!-- <p>C«</p>
            <p>ˇ»D</p>ˇ -->
        "#
        .unindent(),
    );
    cx.update_editor(|editor, window, cx| {
        editor.toggle_comments(&ToggleComments::default(), window, cx)
    });
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
    cx.update_editor(|editor, window, cx| {
        editor.toggle_comments(&ToggleComments::default(), window, cx)
    });
    // TODO this is how it actually worked in Zed Stable, which is not very ergonomic.
    // Uncommenting and commenting from this position brings in even more wrong artifacts.
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

    let buffer = cx.new(|cx| Buffer::local(sample_text(3, 4, 'a'), cx));
    let multibuffer = cx.new(|cx| {
        let mut multibuffer = MultiBuffer::new(ReadWrite);
        multibuffer.push_excerpts(
            buffer.clone(),
            [
                ExcerptRange::new(Point::new(0, 0)..Point::new(0, 4)),
                ExcerptRange::new(Point::new(1, 0)..Point::new(1, 4)),
            ],
            cx,
        );
        assert_eq!(multibuffer.read(cx).text(), "aaaa\nbbbb");
        multibuffer
    });

    let (editor, cx) = cx.add_window_view(|window, cx| build_editor(multibuffer, window, cx));
    editor.update_in(cx, |editor, window, cx| {
        assert_eq!(editor.text(cx), "aaaa\nbbbb");
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([
                Point::new(0, 0)..Point::new(0, 0),
                Point::new(1, 0)..Point::new(1, 0),
            ])
        });

        editor.handle_input("X", window, cx);
        assert_eq!(editor.text(cx), "Xaaaa\nXbbbb");
        assert_eq!(
            editor.selections.ranges(cx),
            [
                Point::new(0, 1)..Point::new(0, 1),
                Point::new(1, 1)..Point::new(1, 1),
            ]
        );

        // Ensure the cursor's head is respected when deleting across an excerpt boundary.
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(0, 2)..Point::new(1, 2)])
        });
        editor.backspace(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "Xa\nbbb");
        assert_eq!(
            editor.selections.ranges(cx),
            [Point::new(1, 0)..Point::new(1, 0)]
        );

        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(1, 1)..Point::new(0, 1)])
        });
        editor.backspace(&Default::default(), window, cx);
        assert_eq!(editor.text(cx), "X\nbb");
        assert_eq!(
            editor.selections.ranges(cx),
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
        ExcerptRange::new(context.clone())
    });
    let buffer = cx.new(|cx| Buffer::local(initial_text, cx));
    let multibuffer = cx.new(|cx| {
        let mut multibuffer = MultiBuffer::new(ReadWrite);
        multibuffer.push_excerpts(buffer, excerpt_ranges, cx);
        multibuffer
    });

    let (editor, cx) = cx.add_window_view(|window, cx| build_editor(multibuffer, window, cx));
    editor.update_in(cx, |editor, window, cx| {
        let (expected_text, selection_ranges) = marked_text_ranges(
            indoc! {"
                aaaa
                bˇbbb
                bˇbbˇb
                cccc"
            },
            true,
        );
        assert_eq!(editor.text(cx), expected_text);
        editor.change_selections(None, window, cx, |s| s.select_ranges(selection_ranges));

        editor.handle_input("X", window, cx);

        let (expected_text, expected_selections) = marked_text_ranges(
            indoc! {"
                aaaa
                bXˇbbXb
                bXˇbbXˇb
                cccc"
            },
            false,
        );
        assert_eq!(editor.text(cx), expected_text);
        assert_eq!(editor.selections.ranges(cx), expected_selections);

        editor.newline(&Newline, window, cx);
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
        assert_eq!(editor.text(cx), expected_text);
        assert_eq!(editor.selections.ranges(cx), expected_selections);
    });
}

#[gpui::test]
fn test_refresh_selections(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let buffer = cx.new(|cx| Buffer::local(sample_text(3, 4, 'a'), cx));
    let mut excerpt1_id = None;
    let multibuffer = cx.new(|cx| {
        let mut multibuffer = MultiBuffer::new(ReadWrite);
        excerpt1_id = multibuffer
            .push_excerpts(
                buffer.clone(),
                [
                    ExcerptRange::new(Point::new(0, 0)..Point::new(1, 4)),
                    ExcerptRange::new(Point::new(1, 0)..Point::new(2, 4)),
                ],
                cx,
            )
            .into_iter()
            .next();
        assert_eq!(multibuffer.read(cx).text(), "aaaa\nbbbb\nbbbb\ncccc");
        multibuffer
    });

    let editor = cx.add_window(|window, cx| {
        let mut editor = build_editor(multibuffer.clone(), window, cx);
        let snapshot = editor.snapshot(window, cx);
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(1, 3)..Point::new(1, 3)])
        });
        editor.begin_selection(
            Point::new(2, 1).to_display_point(&snapshot),
            true,
            1,
            window,
            cx,
        );
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
    _ = editor.update(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| s.refresh());
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
    _ = editor.update(cx, |editor, window, cx| {
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
        editor.change_selections(None, window, cx, |s| s.refresh());
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

    let buffer = cx.new(|cx| Buffer::local(sample_text(3, 4, 'a'), cx));
    let mut excerpt1_id = None;
    let multibuffer = cx.new(|cx| {
        let mut multibuffer = MultiBuffer::new(ReadWrite);
        excerpt1_id = multibuffer
            .push_excerpts(
                buffer.clone(),
                [
                    ExcerptRange::new(Point::new(0, 0)..Point::new(1, 4)),
                    ExcerptRange::new(Point::new(1, 0)..Point::new(2, 4)),
                ],
                cx,
            )
            .into_iter()
            .next();
        assert_eq!(multibuffer.read(cx).text(), "aaaa\nbbbb\nbbbb\ncccc");
        multibuffer
    });

    let editor = cx.add_window(|window, cx| {
        let mut editor = build_editor(multibuffer.clone(), window, cx);
        let snapshot = editor.snapshot(window, cx);
        editor.begin_selection(
            Point::new(1, 3).to_display_point(&snapshot),
            false,
            1,
            window,
            cx,
        );
        assert_eq!(
            editor.selections.ranges(cx),
            [Point::new(1, 3)..Point::new(1, 3)]
        );
        editor
    });

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.remove_excerpts([excerpt1_id.unwrap()], cx);
    });
    _ = editor.update(cx, |editor, window, cx| {
        assert_eq!(
            editor.selections.ranges(cx),
            [Point::new(0, 0)..Point::new(0, 0)]
        );

        // Ensure we don't panic when selections are refreshed and that the pending selection is finalized.
        editor.change_selections(None, window, cx, |s| s.refresh());
        assert_eq!(
            editor.selections.ranges(cx),
            [Point::new(0, 3)..Point::new(0, 3)]
        );
        assert!(editor.selections.pending_anchor().is_some());
    });
}

#[gpui::test]
async fn test_extra_newline_insertion(cx: &mut TestAppContext) {
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
                            surround: true,
                            newline: true,
                        },
                        BracketPair {
                            start: "/* ".to_string(),
                            end: " */".to_string(),
                            close: true,
                            surround: true,
                            newline: true,
                        },
                    ],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
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

    let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(language, cx));
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|window, cx| build_editor(buffer, window, cx));
    editor
        .condition::<crate::EditorEvent>(cx, |editor, cx| !editor.buffer.read(cx).is_parsing(cx))
        .await;

    editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_display_ranges([
                DisplayPoint::new(DisplayRow(0), 2)..DisplayPoint::new(DisplayRow(0), 3),
                DisplayPoint::new(DisplayRow(2), 5)..DisplayPoint::new(DisplayRow(2), 5),
                DisplayPoint::new(DisplayRow(4), 4)..DisplayPoint::new(DisplayRow(4), 4),
            ])
        });
        editor.newline(&Newline, window, cx);

        assert_eq!(
            editor.buffer().read(cx).read(cx).text(),
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

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple(&sample_text(16, 8, 'a'), cx);
        build_editor(buffer.clone(), window, cx)
    });

    _ = editor.update(cx, |editor, window, cx| {
        struct Type1;
        struct Type2;

        let buffer = editor.buffer.read(cx).snapshot(cx);

        let anchor_range =
            |range: Range<Point>| buffer.anchor_after(range.start)..buffer.anchor_after(range.end);

        editor.highlight_background::<Type1>(
            &[
                anchor_range(Point::new(2, 1)..Point::new(2, 3)),
                anchor_range(Point::new(4, 2)..Point::new(4, 4)),
                anchor_range(Point::new(6, 3)..Point::new(6, 5)),
                anchor_range(Point::new(8, 4)..Point::new(8, 6)),
            ],
            |_| Hsla::red(),
            cx,
        );
        editor.highlight_background::<Type2>(
            &[
                anchor_range(Point::new(3, 2)..Point::new(3, 5)),
                anchor_range(Point::new(5, 3)..Point::new(5, 6)),
                anchor_range(Point::new(7, 4)..Point::new(7, 7)),
                anchor_range(Point::new(9, 5)..Point::new(9, 8)),
            ],
            |_| Hsla::green(),
            cx,
        );

        let snapshot = editor.snapshot(window, cx);
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
                    DisplayPoint::new(DisplayRow(4), 2)..DisplayPoint::new(DisplayRow(4), 4),
                    Hsla::red(),
                ),
                (
                    DisplayPoint::new(DisplayRow(6), 3)..DisplayPoint::new(DisplayRow(6), 5),
                    Hsla::red(),
                ),
                (
                    DisplayPoint::new(DisplayRow(3), 2)..DisplayPoint::new(DisplayRow(3), 5),
                    Hsla::green(),
                ),
                (
                    DisplayPoint::new(DisplayRow(5), 3)..DisplayPoint::new(DisplayRow(5), 6),
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
                DisplayPoint::new(DisplayRow(6), 3)..DisplayPoint::new(DisplayRow(6), 5),
                Hsla::red(),
            )]
        );
    });
}

#[gpui::test]
async fn test_following(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, ["/file.rs".as_ref()], cx).await;

    let buffer = project.update(cx, |project, cx| {
        let buffer = project.create_local_buffer(&sample_text(16, 8, 'a'), None, cx);
        cx.new(|cx| MultiBuffer::singleton(buffer, cx))
    });
    let leader = cx.add_window(|window, cx| build_editor(buffer.clone(), window, cx));
    let follower = cx.update(|cx| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::from_corners(
                    gpui::Point::new(px(0.), px(0.)),
                    gpui::Point::new(px(10.), px(80.)),
                ))),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| build_editor(buffer.clone(), window, cx)),
        )
        .unwrap()
    });

    let is_still_following = Rc::new(RefCell::new(true));
    let follower_edit_event_count = Rc::new(RefCell::new(0));
    let pending_update = Rc::new(RefCell::new(None));
    let leader_entity = leader.root(cx).unwrap();
    let follower_entity = follower.root(cx).unwrap();
    _ = follower.update(cx, {
        let update = pending_update.clone();
        let is_still_following = is_still_following.clone();
        let follower_edit_event_count = follower_edit_event_count.clone();
        |_, window, cx| {
            cx.subscribe_in(
                &leader_entity,
                window,
                move |_, leader, event, window, cx| {
                    leader.read(cx).add_event_to_update_proto(
                        event,
                        &mut update.borrow_mut(),
                        window,
                        cx,
                    );
                },
            )
            .detach();

            cx.subscribe_in(
                &follower_entity,
                window,
                move |_, _, event: &EditorEvent, _window, _cx| {
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
    _ = leader.update(cx, |leader, window, cx| {
        leader.change_selections(None, window, cx, |s| s.select_ranges([1..1]));
    });
    follower
        .update(cx, |follower, window, cx| {
            follower.apply_update_proto(
                &project,
                pending_update.borrow_mut().take().unwrap(),
                window,
                cx,
            )
        })
        .unwrap()
        .await
        .unwrap();
    _ = follower.update(cx, |follower, _, cx| {
        assert_eq!(follower.selections.ranges(cx), vec![1..1]);
    });
    assert!(*is_still_following.borrow());
    assert_eq!(*follower_edit_event_count.borrow(), 0);

    // Update the scroll position only
    _ = leader.update(cx, |leader, window, cx| {
        leader.set_scroll_position(gpui::Point::new(1.5, 3.5), window, cx);
    });
    follower
        .update(cx, |follower, window, cx| {
            follower.apply_update_proto(
                &project,
                pending_update.borrow_mut().take().unwrap(),
                window,
                cx,
            )
        })
        .unwrap()
        .await
        .unwrap();
    assert_eq!(
        follower
            .update(cx, |follower, _, cx| follower.scroll_position(cx))
            .unwrap(),
        gpui::Point::new(1.5, 3.5)
    );
    assert!(*is_still_following.borrow());
    assert_eq!(*follower_edit_event_count.borrow(), 0);

    // Update the selections and scroll position. The follower's scroll position is updated
    // via autoscroll, not via the leader's exact scroll position.
    _ = leader.update(cx, |leader, window, cx| {
        leader.change_selections(None, window, cx, |s| s.select_ranges([0..0]));
        leader.request_autoscroll(Autoscroll::newest(), cx);
        leader.set_scroll_position(gpui::Point::new(1.5, 3.5), window, cx);
    });
    follower
        .update(cx, |follower, window, cx| {
            follower.apply_update_proto(
                &project,
                pending_update.borrow_mut().take().unwrap(),
                window,
                cx,
            )
        })
        .unwrap()
        .await
        .unwrap();
    _ = follower.update(cx, |follower, _, cx| {
        assert_eq!(follower.scroll_position(cx), gpui::Point::new(1.5, 0.0));
        assert_eq!(follower.selections.ranges(cx), vec![0..0]);
    });
    assert!(*is_still_following.borrow());

    // Creating a pending selection that precedes another selection
    _ = leader.update(cx, |leader, window, cx| {
        leader.change_selections(None, window, cx, |s| s.select_ranges([1..1]));
        leader.begin_selection(DisplayPoint::new(DisplayRow(0), 0), true, 1, window, cx);
    });
    follower
        .update(cx, |follower, window, cx| {
            follower.apply_update_proto(
                &project,
                pending_update.borrow_mut().take().unwrap(),
                window,
                cx,
            )
        })
        .unwrap()
        .await
        .unwrap();
    _ = follower.update(cx, |follower, _, cx| {
        assert_eq!(follower.selections.ranges(cx), vec![0..0, 1..1]);
    });
    assert!(*is_still_following.borrow());

    // Extend the pending selection so that it surrounds another selection
    _ = leader.update(cx, |leader, window, cx| {
        leader.extend_selection(DisplayPoint::new(DisplayRow(0), 2), 1, window, cx);
    });
    follower
        .update(cx, |follower, window, cx| {
            follower.apply_update_proto(
                &project,
                pending_update.borrow_mut().take().unwrap(),
                window,
                cx,
            )
        })
        .unwrap()
        .await
        .unwrap();
    _ = follower.update(cx, |follower, _, cx| {
        assert_eq!(follower.selections.ranges(cx), vec![0..2]);
    });

    // Scrolling locally breaks the follow
    _ = follower.update(cx, |follower, window, cx| {
        let top_anchor = follower.buffer().read(cx).read(cx).anchor_after(0);
        follower.set_scroll_anchor(
            ScrollAnchor {
                anchor: top_anchor,
                offset: gpui::Point::new(0.0, 0.5),
            },
            window,
            cx,
        );
    });
    assert!(!(*is_still_following.borrow()));
}

#[gpui::test]
async fn test_following_with_multiple_excerpts(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, ["/file.rs".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let pane = workspace
        .update(cx, |workspace, _, _| workspace.active_pane().clone())
        .unwrap();

    let cx = &mut VisualTestContext::from_window(*workspace.deref(), cx);

    let leader = pane.update_in(cx, |_, window, cx| {
        let multibuffer = cx.new(|_| MultiBuffer::new(ReadWrite));
        cx.new(|cx| build_editor(multibuffer.clone(), window, cx))
    });

    // Start following the editor when it has no excerpts.
    let mut state_message =
        leader.update_in(cx, |leader, window, cx| leader.to_state_proto(window, cx));
    let workspace_entity = workspace.root(cx).unwrap();
    let follower_1 = cx
        .update_window(*workspace.deref(), |_, window, cx| {
            Editor::from_state_proto(
                workspace_entity,
                ViewId {
                    creator: CollaboratorId::PeerId(PeerId::default()),
                    id: 0,
                },
                &mut state_message,
                window,
                cx,
            )
        })
        .unwrap()
        .unwrap()
        .await
        .unwrap();

    let update_message = Rc::new(RefCell::new(None));
    follower_1.update_in(cx, {
        let update = update_message.clone();
        |_, window, cx| {
            cx.subscribe_in(&leader, window, move |_, leader, event, window, cx| {
                leader.read(cx).add_event_to_update_proto(
                    event,
                    &mut update.borrow_mut(),
                    window,
                    cx,
                );
            })
            .detach();
        }
    });

    let (buffer_1, buffer_2) = project.update(cx, |project, cx| {
        (
            project.create_local_buffer("abc\ndef\nghi\njkl\n", None, cx),
            project.create_local_buffer("mno\npqr\nstu\nvwx\n", None, cx),
        )
    });

    // Insert some excerpts.
    leader.update(cx, |leader, cx| {
        leader.buffer.update(cx, |multibuffer, cx| {
            multibuffer.set_excerpts_for_path(
                PathKey::namespaced(1, Arc::from(Path::new("b.txt"))),
                buffer_1.clone(),
                vec![
                    Point::row_range(0..3),
                    Point::row_range(1..6),
                    Point::row_range(12..15),
                ],
                0,
                cx,
            );
            multibuffer.set_excerpts_for_path(
                PathKey::namespaced(1, Arc::from(Path::new("a.txt"))),
                buffer_2.clone(),
                vec![Point::row_range(0..6), Point::row_range(8..12)],
                0,
                cx,
            );
        });
    });

    // Apply the update of adding the excerpts.
    follower_1
        .update_in(cx, |follower, window, cx| {
            follower.apply_update_proto(
                &project,
                update_message.borrow().clone().unwrap(),
                window,
                cx,
            )
        })
        .await
        .unwrap();
    assert_eq!(
        follower_1.update(cx, |editor, cx| editor.text(cx)),
        leader.update(cx, |editor, cx| editor.text(cx))
    );
    update_message.borrow_mut().take();

    // Start following separately after it already has excerpts.
    let mut state_message =
        leader.update_in(cx, |leader, window, cx| leader.to_state_proto(window, cx));
    let workspace_entity = workspace.root(cx).unwrap();
    let follower_2 = cx
        .update_window(*workspace.deref(), |_, window, cx| {
            Editor::from_state_proto(
                workspace_entity,
                ViewId {
                    creator: CollaboratorId::PeerId(PeerId::default()),
                    id: 0,
                },
                &mut state_message,
                window,
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
    leader.update(cx, |leader, cx| {
        leader.buffer.update(cx, |multibuffer, cx| {
            let excerpt_ids = multibuffer.excerpt_ids();
            multibuffer.remove_excerpts([excerpt_ids[1], excerpt_ids[2]], cx);
            multibuffer.remove_excerpts([excerpt_ids[0]], cx);
        });
    });

    // Apply the update of removing the excerpts.
    follower_1
        .update_in(cx, |follower, window, cx| {
            follower.apply_update_proto(
                &project,
                update_message.borrow().clone().unwrap(),
                window,
                cx,
            )
        })
        .await
        .unwrap();
    follower_2
        .update_in(cx, |follower, window, cx| {
            follower.apply_update_proto(
                &project,
                update_message.borrow().clone().unwrap(),
                window,
                cx,
            )
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
async fn go_to_prev_overlapping_diagnostic(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    let lsp_store =
        cx.update_editor(|editor, _, cx| editor.project.as_ref().unwrap().read(cx).lsp_store());

    cx.set_state(indoc! {"
        ˇfn func(abc def: i32) -> u32 {
        }
    "});

    cx.update(|_, cx| {
        lsp_store.update(cx, |lsp_store, cx| {
            lsp_store
                .update_diagnostics(
                    LanguageServerId(0),
                    lsp::PublishDiagnosticsParams {
                        uri: lsp::Url::from_file_path(path!("/root/file")).unwrap(),
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

    cx.update_editor(|editor, window, cx| {
        editor.go_to_prev_diagnostic(&GoToPreviousDiagnostic, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        fn func(abc def: i32) -> ˇu32 {
        }
    "});

    cx.update_editor(|editor, window, cx| {
        editor.go_to_prev_diagnostic(&GoToPreviousDiagnostic, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        fn func(abc ˇdef: i32) -> u32 {
        }
    "});

    cx.update_editor(|editor, window, cx| {
        editor.go_to_prev_diagnostic(&GoToPreviousDiagnostic, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        fn func(abcˇ def: i32) -> u32 {
        }
    "});

    cx.update_editor(|editor, window, cx| {
        editor.go_to_prev_diagnostic(&GoToPreviousDiagnostic, window, cx);
    });

    cx.assert_editor_state(indoc! {"
        fn func(abc def: i32) -> ˇu32 {
        }
    "});
}

#[gpui::test]
async fn test_go_to_hunk(executor: BackgroundExecutor, cx: &mut TestAppContext) {
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

    cx.set_head_text(&diff_base);
    executor.run_until_parked();

    cx.update_editor(|editor, window, cx| {
        //Wrap around the bottom of the buffer
        for _ in 0..3 {
            editor.go_to_next_hunk(&GoToHunk, window, cx);
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

    cx.update_editor(|editor, window, cx| {
        //Wrap around the top of the buffer
        for _ in 0..2 {
            editor.go_to_prev_hunk(&GoToPreviousHunk, window, cx);
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

    cx.update_editor(|editor, window, cx| {
        editor.go_to_prev_hunk(&GoToPreviousHunk, window, cx);
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

    cx.update_editor(|editor, window, cx| {
        editor.go_to_prev_hunk(&GoToPreviousHunk, window, cx);
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

    cx.update_editor(|editor, window, cx| {
        for _ in 0..2 {
            editor.go_to_prev_hunk(&GoToPreviousHunk, window, cx);
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

    cx.update_editor(|editor, window, cx| {
        editor.fold(&Fold, window, cx);
    });

    cx.update_editor(|editor, window, cx| {
        editor.go_to_next_hunk(&GoToHunk, window, cx);
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

    assert_eq!(split(":do_the_thing"), &[":", "do_", "the_", "thing"]);
}

#[gpui::test]
async fn test_move_to_enclosing_bracket(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorLspTestContext::new_typescript(Default::default(), cx).await;
    let mut assert = |before, after| {
        let _state_context = cx.set_state(before);
        cx.run_until_parked();
        cx.update_editor(|editor, window, cx| {
            editor.move_to_enclosing_bracket(&MoveToEnclosingBracket, window, cx)
        });
        cx.run_until_parked();
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

#[gpui::test]
async fn test_on_type_formatting_not_triggered(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/a"),
        json!({
            "main.rs": "fn main() { let a = 5; }",
            "other.rs": "// Test file",
        }),
    )
    .await;
    let project = Project::test(fs, [path!("/a").as_ref()], cx).await;

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
                    surround: true,
                    newline: true,
                }],
                disabled_scopes_by_bracket_ix: Vec::new(),
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    )));
    let mut fake_servers = language_registry.register_fake_lsp(
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

    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));

    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let worktree_id = workspace
        .update(cx, |workspace, _, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        })
        .unwrap();

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/a/main.rs"), cx)
        })
        .await
        .unwrap();
    let editor_handle = workspace
        .update(cx, |workspace, window, cx| {
            workspace.open_path((worktree_id, "main.rs"), None, true, window, cx)
        })
        .unwrap()
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    cx.executor().start_waiting();
    let fake_server = fake_servers.next().await.unwrap();

    fake_server.set_request_handler::<lsp::request::OnTypeFormatting, _, _>(
        |params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri,
                lsp::Url::from_file_path(path!("/a/main.rs")).unwrap(),
            );
            assert_eq!(
                params.text_document_position.position,
                lsp::Position::new(0, 21),
            );

            Ok(Some(vec![lsp::TextEdit {
                new_text: "]".to_string(),
                range: lsp::Range::new(lsp::Position::new(0, 22), lsp::Position::new(0, 22)),
            }]))
        },
    );

    editor_handle.update_in(cx, |editor, window, cx| {
        window.focus(&editor.focus_handle(cx));
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(0, 21)..Point::new(0, 20)])
        });
        editor.handle_input("{", window, cx);
    });

    cx.executor().run_until_parked();

    buffer.update(cx, |buffer, _| {
        assert_eq!(
            buffer.text(),
            "fn main() { let a = {5}; }",
            "No extra braces from on type formatting should appear in the buffer"
        )
    });
}

#[gpui::test]
async fn test_language_server_restart_due_to_settings_change(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/a"),
        json!({
            "main.rs": "fn main() { let a = 5; }",
            "other.rs": "// Test file",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/a").as_ref()], cx).await;

    let server_restarts = Arc::new(AtomicUsize::new(0));
    let closure_restarts = Arc::clone(&server_restarts);
    let language_server_name = "test language server";
    let language_name: LanguageName = "Rust".into();

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(Arc::new(Language::new(
        LanguageConfig {
            name: language_name.clone(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    )));
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: language_server_name,
            initialization_options: Some(json!({
                "testOptionValue": true
            })),
            initializer: Some(Box::new(move |fake_server| {
                let task_restarts = Arc::clone(&closure_restarts);
                fake_server.set_request_handler::<lsp::request::Shutdown, _, _>(move |_, _| {
                    task_restarts.fetch_add(1, atomic::Ordering::Release);
                    futures::future::ready(Ok(()))
                });
            })),
            ..Default::default()
        },
    );

    let _window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let _buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/a/main.rs"), cx)
        })
        .await
        .unwrap();
    let _fake_server = fake_servers.next().await.unwrap();
    update_test_language_settings(cx, |language_settings| {
        language_settings.languages.insert(
            language_name.clone(),
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
                binary: None,
                settings: None,
                initialization_options: Some(json!({
                    "some other init value": false
                })),
                enable_lsp_tasks: false,
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
                binary: None,
                settings: None,
                initialization_options: Some(json!({
                    "anotherInitValue": false
                })),
                enable_lsp_tasks: false,
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
                binary: None,
                settings: None,
                initialization_options: Some(json!({
                    "anotherInitValue": false
                })),
                enable_lsp_tasks: false,
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
                binary: None,
                settings: None,
                initialization_options: None,
                enable_lsp_tasks: false,
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
async fn test_completions_with_additional_edits(cx: &mut TestAppContext) {
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

    cx.set_state("fn main() { let a = 2ˇ; }");
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
    let mut request = cx.set_request_handler::<lsp::request::Completion, _, _>(move |_, _, _| {
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
    let apply_additional_edits = cx.update_editor(|editor, window, cx| {
        editor
            .confirm_completion(&ConfirmCompletion::default(), window, cx)
            .unwrap()
    });
    cx.assert_editor_state("fn main() { let a = 2.Some(2)ˇ; }");

    cx.set_request_handler::<lsp::request::ResolveCompletionItem, _, _>(move |_, _, _| {
        let task_completion_item = completion_item.clone();
        async move { Ok(task_completion_item) }
    })
    .next()
    .await
    .unwrap();
    apply_additional_edits.await.unwrap();
    cx.assert_editor_state("fn main() { let a = Some(2)ˇ; }");
}

#[gpui::test]
async fn test_completions_resolve_updates_labels_if_filter_text_matches(cx: &mut TestAppContext) {
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

    cx.set_state("fn main() { let a = 2ˇ; }");
    cx.simulate_keystroke(".");

    let item1 = lsp::CompletionItem {
        label: "method id()".to_string(),
        filter_text: Some("id".to_string()),
        detail: None,
        documentation: None,
        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
            range: lsp::Range::new(lsp::Position::new(0, 22), lsp::Position::new(0, 22)),
            new_text: ".id".to_string(),
        })),
        ..lsp::CompletionItem::default()
    };

    let item2 = lsp::CompletionItem {
        label: "other".to_string(),
        filter_text: Some("other".to_string()),
        detail: None,
        documentation: None,
        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
            range: lsp::Range::new(lsp::Position::new(0, 22), lsp::Position::new(0, 22)),
            new_text: ".other".to_string(),
        })),
        ..lsp::CompletionItem::default()
    };

    let item1 = item1.clone();
    cx.set_request_handler::<lsp::request::Completion, _, _>({
        let item1 = item1.clone();
        move |_, _, _| {
            let item1 = item1.clone();
            let item2 = item2.clone();
            async move { Ok(Some(lsp::CompletionResponse::Array(vec![item1, item2]))) }
        }
    })
    .next()
    .await;

    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    cx.update_editor(|editor, _, _| {
        let context_menu = editor.context_menu.borrow_mut();
        let context_menu = context_menu
            .as_ref()
            .expect("Should have the context menu deployed");
        match context_menu {
            CodeContextMenu::Completions(completions_menu) => {
                let completions = completions_menu.completions.borrow_mut();
                assert_eq!(
                    completions
                        .iter()
                        .map(|completion| &completion.label.text)
                        .collect::<Vec<_>>(),
                    vec!["method id()", "other"]
                )
            }
            CodeContextMenu::CodeActions(_) => panic!("Should show the completions menu"),
        }
    });

    cx.set_request_handler::<lsp::request::ResolveCompletionItem, _, _>({
        let item1 = item1.clone();
        move |_, item_to_resolve, _| {
            let item1 = item1.clone();
            async move {
                if item1 == item_to_resolve {
                    Ok(lsp::CompletionItem {
                        label: "method id()".to_string(),
                        filter_text: Some("id".to_string()),
                        detail: Some("Now resolved!".to_string()),
                        documentation: Some(lsp::Documentation::String("Docs".to_string())),
                        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                            range: lsp::Range::new(
                                lsp::Position::new(0, 22),
                                lsp::Position::new(0, 22),
                            ),
                            new_text: ".id".to_string(),
                        })),
                        ..lsp::CompletionItem::default()
                    })
                } else {
                    Ok(item_to_resolve)
                }
            }
        }
    })
    .next()
    .await
    .unwrap();
    cx.run_until_parked();

    cx.update_editor(|editor, window, cx| {
        editor.context_menu_next(&Default::default(), window, cx);
    });

    cx.update_editor(|editor, _, _| {
        let context_menu = editor.context_menu.borrow_mut();
        let context_menu = context_menu
            .as_ref()
            .expect("Should have the context menu deployed");
        match context_menu {
            CodeContextMenu::Completions(completions_menu) => {
                let completions = completions_menu.completions.borrow_mut();
                assert_eq!(
                    completions
                        .iter()
                        .map(|completion| &completion.label.text)
                        .collect::<Vec<_>>(),
                    vec!["method id() Now resolved!", "other"],
                    "Should update first completion label, but not second as the filter text did not match."
                );
            }
            CodeContextMenu::CodeActions(_) => panic!("Should show the completions menu"),
        }
    });
}

#[gpui::test]
async fn test_completions_resolve_happens_once(cx: &mut TestAppContext) {
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

    cx.set_state("fn main() { let a = 2ˇ; }");
    cx.simulate_keystroke(".");

    let unresolved_item_1 = lsp::CompletionItem {
        label: "id".to_string(),
        filter_text: Some("id".to_string()),
        detail: None,
        documentation: None,
        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
            range: lsp::Range::new(lsp::Position::new(0, 22), lsp::Position::new(0, 22)),
            new_text: ".id".to_string(),
        })),
        ..lsp::CompletionItem::default()
    };
    let resolved_item_1 = lsp::CompletionItem {
        additional_text_edits: Some(vec![lsp::TextEdit {
            range: lsp::Range::new(lsp::Position::new(0, 20), lsp::Position::new(0, 22)),
            new_text: "!!".to_string(),
        }]),
        ..unresolved_item_1.clone()
    };
    let unresolved_item_2 = lsp::CompletionItem {
        label: "other".to_string(),
        filter_text: Some("other".to_string()),
        detail: None,
        documentation: None,
        text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
            range: lsp::Range::new(lsp::Position::new(0, 22), lsp::Position::new(0, 22)),
            new_text: ".other".to_string(),
        })),
        ..lsp::CompletionItem::default()
    };
    let resolved_item_2 = lsp::CompletionItem {
        additional_text_edits: Some(vec![lsp::TextEdit {
            range: lsp::Range::new(lsp::Position::new(0, 20), lsp::Position::new(0, 22)),
            new_text: "??".to_string(),
        }]),
        ..unresolved_item_2.clone()
    };

    let resolve_requests_1 = Arc::new(AtomicUsize::new(0));
    let resolve_requests_2 = Arc::new(AtomicUsize::new(0));
    cx.lsp
        .server
        .on_request::<lsp::request::ResolveCompletionItem, _, _>({
            let unresolved_item_1 = unresolved_item_1.clone();
            let resolved_item_1 = resolved_item_1.clone();
            let unresolved_item_2 = unresolved_item_2.clone();
            let resolved_item_2 = resolved_item_2.clone();
            let resolve_requests_1 = resolve_requests_1.clone();
            let resolve_requests_2 = resolve_requests_2.clone();
            move |unresolved_request, _| {
                let unresolved_item_1 = unresolved_item_1.clone();
                let resolved_item_1 = resolved_item_1.clone();
                let unresolved_item_2 = unresolved_item_2.clone();
                let resolved_item_2 = resolved_item_2.clone();
                let resolve_requests_1 = resolve_requests_1.clone();
                let resolve_requests_2 = resolve_requests_2.clone();
                async move {
                    if unresolved_request == unresolved_item_1 {
                        resolve_requests_1.fetch_add(1, atomic::Ordering::Release);
                        Ok(resolved_item_1.clone())
                    } else if unresolved_request == unresolved_item_2 {
                        resolve_requests_2.fetch_add(1, atomic::Ordering::Release);
                        Ok(resolved_item_2.clone())
                    } else {
                        panic!("Unexpected completion item {unresolved_request:?}")
                    }
                }
            }
        })
        .detach();

    cx.set_request_handler::<lsp::request::Completion, _, _>(move |_, _, _| {
        let unresolved_item_1 = unresolved_item_1.clone();
        let unresolved_item_2 = unresolved_item_2.clone();
        async move {
            Ok(Some(lsp::CompletionResponse::Array(vec![
                unresolved_item_1,
                unresolved_item_2,
            ])))
        }
    })
    .next()
    .await;

    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    cx.update_editor(|editor, _, _| {
        let context_menu = editor.context_menu.borrow_mut();
        let context_menu = context_menu
            .as_ref()
            .expect("Should have the context menu deployed");
        match context_menu {
            CodeContextMenu::Completions(completions_menu) => {
                let completions = completions_menu.completions.borrow_mut();
                assert_eq!(
                    completions
                        .iter()
                        .map(|completion| &completion.label.text)
                        .collect::<Vec<_>>(),
                    vec!["id", "other"]
                )
            }
            CodeContextMenu::CodeActions(_) => panic!("Should show the completions menu"),
        }
    });
    cx.run_until_parked();

    cx.update_editor(|editor, window, cx| {
        editor.context_menu_next(&ContextMenuNext, window, cx);
    });
    cx.run_until_parked();
    cx.update_editor(|editor, window, cx| {
        editor.context_menu_prev(&ContextMenuPrevious, window, cx);
    });
    cx.run_until_parked();
    cx.update_editor(|editor, window, cx| {
        editor.context_menu_next(&ContextMenuNext, window, cx);
    });
    cx.run_until_parked();
    cx.update_editor(|editor, window, cx| {
        editor
            .compose_completion(&ComposeCompletion::default(), window, cx)
            .expect("No task returned")
    })
    .await
    .expect("Completion failed");
    cx.run_until_parked();

    cx.update_editor(|editor, _, cx| {
        assert_eq!(
            resolve_requests_1.load(atomic::Ordering::Acquire),
            1,
            "Should always resolve once despite multiple selections"
        );
        assert_eq!(
            resolve_requests_2.load(atomic::Ordering::Acquire),
            1,
            "Should always resolve once after multiple selections and applying the completion"
        );
        assert_eq!(
            editor.text(cx),
            "fn main() { let a = ??.other; }",
            "Should use resolved data when applying the completion"
        );
    });
}

#[gpui::test]
async fn test_completions_default_resolve_data_handling(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let item_0 = lsp::CompletionItem {
        label: "abs".into(),
        insert_text: Some("abs".into()),
        data: Some(json!({ "very": "special"})),
        insert_text_mode: Some(lsp::InsertTextMode::ADJUST_INDENTATION),
        text_edit: Some(lsp::CompletionTextEdit::InsertAndReplace(
            lsp::InsertReplaceEdit {
                new_text: "abs".to_string(),
                insert: lsp::Range::default(),
                replace: lsp::Range::default(),
            },
        )),
        ..lsp::CompletionItem::default()
    };
    let items = iter::once(item_0.clone())
        .chain((11..51).map(|i| lsp::CompletionItem {
            label: format!("item_{}", i),
            insert_text: Some(format!("item_{}", i)),
            insert_text_format: Some(lsp::InsertTextFormat::PLAIN_TEXT),
            ..lsp::CompletionItem::default()
        }))
        .collect::<Vec<_>>();

    let default_commit_characters = vec!["?".to_string()];
    let default_data = json!({ "default": "data"});
    let default_insert_text_format = lsp::InsertTextFormat::SNIPPET;
    let default_insert_text_mode = lsp::InsertTextMode::AS_IS;
    let default_edit_range = lsp::Range {
        start: lsp::Position {
            line: 0,
            character: 5,
        },
        end: lsp::Position {
            line: 0,
            character: 5,
        },
    };

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

    cx.set_state("fn main() { let a = 2ˇ; }");
    cx.simulate_keystroke(".");

    let completion_data = default_data.clone();
    let completion_characters = default_commit_characters.clone();
    let completion_items = items.clone();
    cx.set_request_handler::<lsp::request::Completion, _, _>(move |_, _, _| {
        let default_data = completion_data.clone();
        let default_commit_characters = completion_characters.clone();
        let items = completion_items.clone();
        async move {
            Ok(Some(lsp::CompletionResponse::List(lsp::CompletionList {
                items,
                item_defaults: Some(lsp::CompletionListItemDefaults {
                    data: Some(default_data.clone()),
                    commit_characters: Some(default_commit_characters.clone()),
                    edit_range: Some(lsp::CompletionListItemDefaultsEditRange::Range(
                        default_edit_range,
                    )),
                    insert_text_format: Some(default_insert_text_format),
                    insert_text_mode: Some(default_insert_text_mode),
                }),
                ..lsp::CompletionList::default()
            })))
        }
    })
    .next()
    .await;

    let resolved_items = Arc::new(Mutex::new(Vec::new()));
    cx.lsp
        .server
        .on_request::<lsp::request::ResolveCompletionItem, _, _>({
            let closure_resolved_items = resolved_items.clone();
            move |item_to_resolve, _| {
                let closure_resolved_items = closure_resolved_items.clone();
                async move {
                    closure_resolved_items.lock().push(item_to_resolve.clone());
                    Ok(item_to_resolve)
                }
            }
        })
        .detach();

    cx.condition(|editor, _| editor.context_menu_visible())
        .await;
    cx.run_until_parked();
    cx.update_editor(|editor, _, _| {
        let menu = editor.context_menu.borrow_mut();
        match menu.as_ref().expect("should have the completions menu") {
            CodeContextMenu::Completions(completions_menu) => {
                assert_eq!(
                    completions_menu
                        .entries
                        .borrow()
                        .iter()
                        .map(|mat| mat.string.clone())
                        .collect::<Vec<String>>(),
                    items
                        .iter()
                        .map(|completion| completion.label.clone())
                        .collect::<Vec<String>>()
                );
            }
            CodeContextMenu::CodeActions(_) => panic!("Expected to have the completions menu"),
        }
    });
    // Approximate initial displayed interval is 0..12. With extra item padding of 4 this is 0..16
    // with 4 from the end.
    assert_eq!(
        *resolved_items.lock(),
        [&items[0..16], &items[items.len() - 4..items.len()]]
            .concat()
            .iter()
            .cloned()
            .map(|mut item| {
                if item.data.is_none() {
                    item.data = Some(default_data.clone());
                }
                item
            })
            .collect::<Vec<lsp::CompletionItem>>(),
        "Items sent for resolve should be unchanged modulo resolve `data` filled with default if missing"
    );
    resolved_items.lock().clear();

    cx.update_editor(|editor, window, cx| {
        editor.context_menu_prev(&ContextMenuPrevious, window, cx);
    });
    cx.run_until_parked();
    // Completions that have already been resolved are skipped.
    assert_eq!(
        *resolved_items.lock(),
        items[items.len() - 16..items.len() - 4]
            .iter()
            .cloned()
            .map(|mut item| {
                if item.data.is_none() {
                    item.data = Some(default_data.clone());
                }
                item
            })
            .collect::<Vec<lsp::CompletionItem>>()
    );
    resolved_items.lock().clear();
}

#[gpui::test]
async fn test_completions_in_languages_with_extra_word_characters(cx: &mut TestAppContext) {
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
                        completion_query_characters: Override::Set(['-'].into_iter().collect()),
                        ..Default::default()
                    },
                )]
                .into_iter()
                .collect(),
                ..Default::default()
            },
            Some(tree_sitter_typescript::LANGUAGE_TSX.into()),
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
        .set_request_handler::<lsp::request::Completion, _, _>(move |_, _| async move {
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
    cx.update_editor(|editor, _, _| {
        if let Some(CodeContextMenu::Completions(menu)) = editor.context_menu.borrow_mut().as_ref()
        {
            assert_eq!(
                completion_menu_entries(&menu),
                &["bg-red", "bg-blue", "bg-yellow"]
            );
        } else {
            panic!("expected completion menu to be open");
        }
    });

    cx.simulate_keystroke("l");
    cx.executor().run_until_parked();
    cx.update_editor(|editor, _, _| {
        if let Some(CodeContextMenu::Completions(menu)) = editor.context_menu.borrow_mut().as_ref()
        {
            assert_eq!(completion_menu_entries(&menu), &["bg-blue", "bg-yellow"]);
        } else {
            panic!("expected completion menu to be open");
        }
    });

    // When filtering completions, consider the character after the '-' to
    // be the start of a subword.
    cx.set_state(r#"<p class="yelˇ" />"#);
    cx.simulate_keystroke("l");
    cx.executor().run_until_parked();
    cx.update_editor(|editor, _, _| {
        if let Some(CodeContextMenu::Completions(menu)) = editor.context_menu.borrow_mut().as_ref()
        {
            assert_eq!(completion_menu_entries(&menu), &["bg-yellow"]);
        } else {
            panic!("expected completion menu to be open");
        }
    });
}

fn completion_menu_entries(menu: &CompletionsMenu) -> Vec<String> {
    let entries = menu.entries.borrow();
    entries.iter().map(|mat| mat.string.clone()).collect()
}

#[gpui::test]
async fn test_document_format_with_prettier(cx: &mut TestAppContext) {
    init_test(cx, |settings| {
        settings.defaults.formatter = Some(language_settings::SelectedFormatter::List(
            FormatterList(vec![Formatter::Prettier].into()),
        ))
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_file(path!("/file.ts"), Default::default()).await;

    let project = Project::test(fs, [path!("/file.ts").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());

    language_registry.add(Arc::new(Language::new(
        LanguageConfig {
            name: "TypeScript".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["ts".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
    )));
    update_test_language_settings(cx, |settings| {
        settings.defaults.prettier = Some(PrettierSettings {
            allowed: true,
            ..PrettierSettings::default()
        });
    });

    let test_plugin = "test_plugin";
    let _ = language_registry.register_fake_lsp(
        "TypeScript",
        FakeLspAdapter {
            prettier_plugins: vec![test_plugin],
            ..Default::default()
        },
    );

    let prettier_format_suffix = project::TEST_PRETTIER_FORMAT_SUFFIX;
    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/file.ts"), cx)
        })
        .await
        .unwrap();

    let buffer_text = "one\ntwo\nthree\n";
    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let (editor, cx) = cx.add_window_view(|window, cx| build_editor(buffer, window, cx));
    editor.update_in(cx, |editor, window, cx| {
        editor.set_text(buffer_text, window, cx)
    });

    editor
        .update_in(cx, |editor, window, cx| {
            editor.perform_format(
                project.clone(),
                FormatTrigger::Manual,
                FormatTarget::Buffers,
                window,
                cx,
            )
        })
        .unwrap()
        .await;
    assert_eq!(
        editor.update(cx, |editor, cx| editor.text(cx)),
        buffer_text.to_string() + prettier_format_suffix,
        "Test prettier formatting was not applied to the original buffer text",
    );

    update_test_language_settings(cx, |settings| {
        settings.defaults.formatter = Some(language_settings::SelectedFormatter::Auto)
    });
    let format = editor.update_in(cx, |editor, window, cx| {
        editor.perform_format(
            project.clone(),
            FormatTrigger::Manual,
            FormatTarget::Buffers,
            window,
            cx,
        )
    });
    format.await.unwrap();
    assert_eq!(
        editor.update(cx, |editor, cx| editor.text(cx)),
        buffer_text.to_string() + prettier_format_suffix + "\n" + prettier_format_suffix,
        "Autoformatting (via test prettier) was not applied to the original buffer text",
    );
}

#[gpui::test]
async fn test_addition_reverts(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorLspTestContext::new_rust(lsp::ServerCapabilities::default(), cx).await;
    let base_text = indoc! {r#"
        struct Row;
        struct Row1;
        struct Row2;

        struct Row4;
        struct Row5;
        struct Row6;

        struct Row8;
        struct Row9;
        struct Row10;"#};

    // When addition hunks are not adjacent to carets, no hunk revert is performed
    assert_hunk_revert(
        indoc! {r#"struct Row;
                   struct Row1;
                   struct Row1.1;
                   struct Row1.2;
                   struct Row2;ˇ

                   struct Row4;
                   struct Row5;
                   struct Row6;

                   struct Row8;
                   ˇstruct Row9;
                   struct Row9.1;
                   struct Row9.2;
                   struct Row9.3;
                   struct Row10;"#},
        vec![DiffHunkStatusKind::Added, DiffHunkStatusKind::Added],
        indoc! {r#"struct Row;
                   struct Row1;
                   struct Row1.1;
                   struct Row1.2;
                   struct Row2;ˇ

                   struct Row4;
                   struct Row5;
                   struct Row6;

                   struct Row8;
                   ˇstruct Row9;
                   struct Row9.1;
                   struct Row9.2;
                   struct Row9.3;
                   struct Row10;"#},
        base_text,
        &mut cx,
    );
    // Same for selections
    assert_hunk_revert(
        indoc! {r#"struct Row;
                   struct Row1;
                   struct Row2;
                   struct Row2.1;
                   struct Row2.2;
                   «ˇ
                   struct Row4;
                   struct» Row5;
                   «struct Row6;
                   ˇ»
                   struct Row9.1;
                   struct Row9.2;
                   struct Row9.3;
                   struct Row8;
                   struct Row9;
                   struct Row10;"#},
        vec![DiffHunkStatusKind::Added, DiffHunkStatusKind::Added],
        indoc! {r#"struct Row;
                   struct Row1;
                   struct Row2;
                   struct Row2.1;
                   struct Row2.2;
                   «ˇ
                   struct Row4;
                   struct» Row5;
                   «struct Row6;
                   ˇ»
                   struct Row9.1;
                   struct Row9.2;
                   struct Row9.3;
                   struct Row8;
                   struct Row9;
                   struct Row10;"#},
        base_text,
        &mut cx,
    );

    // When carets and selections intersect the addition hunks, those are reverted.
    // Adjacent carets got merged.
    assert_hunk_revert(
        indoc! {r#"struct Row;
                   ˇ// something on the top
                   struct Row1;
                   struct Row2;
                   struct Roˇw3.1;
                   struct Row2.2;
                   struct Row2.3;ˇ

                   struct Row4;
                   struct ˇRow5.1;
                   struct Row5.2;
                   struct «Rowˇ»5.3;
                   struct Row5;
                   struct Row6;
                   ˇ
                   struct Row9.1;
                   struct «Rowˇ»9.2;
                   struct «ˇRow»9.3;
                   struct Row8;
                   struct Row9;
                   «ˇ// something on bottom»
                   struct Row10;"#},
        vec![
            DiffHunkStatusKind::Added,
            DiffHunkStatusKind::Added,
            DiffHunkStatusKind::Added,
            DiffHunkStatusKind::Added,
            DiffHunkStatusKind::Added,
        ],
        indoc! {r#"struct Row;
                   ˇstruct Row1;
                   struct Row2;
                   ˇ
                   struct Row4;
                   ˇstruct Row5;
                   struct Row6;
                   ˇ
                   ˇstruct Row8;
                   struct Row9;
                   ˇstruct Row10;"#},
        base_text,
        &mut cx,
    );
}

#[gpui::test]
async fn test_modification_reverts(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorLspTestContext::new_rust(lsp::ServerCapabilities::default(), cx).await;
    let base_text = indoc! {r#"
        struct Row;
        struct Row1;
        struct Row2;

        struct Row4;
        struct Row5;
        struct Row6;

        struct Row8;
        struct Row9;
        struct Row10;"#};

    // Modification hunks behave the same as the addition ones.
    assert_hunk_revert(
        indoc! {r#"struct Row;
                   struct Row1;
                   struct Row33;
                   ˇ
                   struct Row4;
                   struct Row5;
                   struct Row6;
                   ˇ
                   struct Row99;
                   struct Row9;
                   struct Row10;"#},
        vec![DiffHunkStatusKind::Modified, DiffHunkStatusKind::Modified],
        indoc! {r#"struct Row;
                   struct Row1;
                   struct Row33;
                   ˇ
                   struct Row4;
                   struct Row5;
                   struct Row6;
                   ˇ
                   struct Row99;
                   struct Row9;
                   struct Row10;"#},
        base_text,
        &mut cx,
    );
    assert_hunk_revert(
        indoc! {r#"struct Row;
                   struct Row1;
                   struct Row33;
                   «ˇ
                   struct Row4;
                   struct» Row5;
                   «struct Row6;
                   ˇ»
                   struct Row99;
                   struct Row9;
                   struct Row10;"#},
        vec![DiffHunkStatusKind::Modified, DiffHunkStatusKind::Modified],
        indoc! {r#"struct Row;
                   struct Row1;
                   struct Row33;
                   «ˇ
                   struct Row4;
                   struct» Row5;
                   «struct Row6;
                   ˇ»
                   struct Row99;
                   struct Row9;
                   struct Row10;"#},
        base_text,
        &mut cx,
    );

    assert_hunk_revert(
        indoc! {r#"ˇstruct Row1.1;
                   struct Row1;
                   «ˇstr»uct Row22;

                   struct ˇRow44;
                   struct Row5;
                   struct «Rˇ»ow66;ˇ

                   «struˇ»ct Row88;
                   struct Row9;
                   struct Row1011;ˇ"#},
        vec![
            DiffHunkStatusKind::Modified,
            DiffHunkStatusKind::Modified,
            DiffHunkStatusKind::Modified,
            DiffHunkStatusKind::Modified,
            DiffHunkStatusKind::Modified,
            DiffHunkStatusKind::Modified,
        ],
        indoc! {r#"struct Row;
                   ˇstruct Row1;
                   struct Row2;
                   ˇ
                   struct Row4;
                   ˇstruct Row5;
                   struct Row6;
                   ˇ
                   struct Row8;
                   ˇstruct Row9;
                   struct Row10;ˇ"#},
        base_text,
        &mut cx,
    );
}

#[gpui::test]
async fn test_deleting_over_diff_hunk(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorLspTestContext::new_rust(lsp::ServerCapabilities::default(), cx).await;
    let base_text = indoc! {r#"
        one

        two
        three
        "#};

    cx.set_head_text(base_text);
    cx.set_state("\nˇ\n");
    cx.executor().run_until_parked();
    cx.update_editor(|editor, _window, cx| {
        editor.expand_selected_diff_hunks(cx);
    });
    cx.executor().run_until_parked();
    cx.update_editor(|editor, window, cx| {
        editor.backspace(&Default::default(), window, cx);
    });
    cx.run_until_parked();
    cx.assert_state_with_diff(
        indoc! {r#"

        - two
        - threeˇ
        +
        "#}
        .to_string(),
    );
}

#[gpui::test]
async fn test_deletion_reverts(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorLspTestContext::new_rust(lsp::ServerCapabilities::default(), cx).await;
    let base_text = indoc! {r#"struct Row;
struct Row1;
struct Row2;

struct Row4;
struct Row5;
struct Row6;

struct Row8;
struct Row9;
struct Row10;"#};

    // Deletion hunks trigger with carets on adjacent rows, so carets and selections have to stay farther to avoid the revert
    assert_hunk_revert(
        indoc! {r#"struct Row;
                   struct Row2;

                   ˇstruct Row4;
                   struct Row5;
                   struct Row6;
                   ˇ
                   struct Row8;
                   struct Row10;"#},
        vec![DiffHunkStatusKind::Deleted, DiffHunkStatusKind::Deleted],
        indoc! {r#"struct Row;
                   struct Row2;

                   ˇstruct Row4;
                   struct Row5;
                   struct Row6;
                   ˇ
                   struct Row8;
                   struct Row10;"#},
        base_text,
        &mut cx,
    );
    assert_hunk_revert(
        indoc! {r#"struct Row;
                   struct Row2;

                   «ˇstruct Row4;
                   struct» Row5;
                   «struct Row6;
                   ˇ»
                   struct Row8;
                   struct Row10;"#},
        vec![DiffHunkStatusKind::Deleted, DiffHunkStatusKind::Deleted],
        indoc! {r#"struct Row;
                   struct Row2;

                   «ˇstruct Row4;
                   struct» Row5;
                   «struct Row6;
                   ˇ»
                   struct Row8;
                   struct Row10;"#},
        base_text,
        &mut cx,
    );

    // Deletion hunks are ephemeral, so it's impossible to place the caret into them — Zed triggers reverts for lines, adjacent to carets and selections.
    assert_hunk_revert(
        indoc! {r#"struct Row;
                   ˇstruct Row2;

                   struct Row4;
                   struct Row5;
                   struct Row6;

                   struct Row8;ˇ
                   struct Row10;"#},
        vec![DiffHunkStatusKind::Deleted, DiffHunkStatusKind::Deleted],
        indoc! {r#"struct Row;
                   struct Row1;
                   ˇstruct Row2;

                   struct Row4;
                   struct Row5;
                   struct Row6;

                   struct Row8;ˇ
                   struct Row9;
                   struct Row10;"#},
        base_text,
        &mut cx,
    );
    assert_hunk_revert(
        indoc! {r#"struct Row;
                   struct Row2«ˇ;
                   struct Row4;
                   struct» Row5;
                   «struct Row6;

                   struct Row8;ˇ»
                   struct Row10;"#},
        vec![
            DiffHunkStatusKind::Deleted,
            DiffHunkStatusKind::Deleted,
            DiffHunkStatusKind::Deleted,
        ],
        indoc! {r#"struct Row;
                   struct Row1;
                   struct Row2«ˇ;

                   struct Row4;
                   struct» Row5;
                   «struct Row6;

                   struct Row8;ˇ»
                   struct Row9;
                   struct Row10;"#},
        base_text,
        &mut cx,
    );
}

#[gpui::test]
async fn test_multibuffer_reverts(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let base_text_1 = "aaaa\nbbbb\ncccc\ndddd\neeee\nffff\ngggg\nhhhh\niiii\njjjj";
    let base_text_2 = "llll\nmmmm\nnnnn\noooo\npppp\nqqqq\nrrrr\nssss\ntttt\nuuuu";
    let base_text_3 =
        "vvvv\nwwww\nxxxx\nyyyy\nzzzz\n{{{{\n||||\n}}}}\n~~~~\n\u{7f}\u{7f}\u{7f}\u{7f}";

    let text_1 = edit_first_char_of_every_line(base_text_1);
    let text_2 = edit_first_char_of_every_line(base_text_2);
    let text_3 = edit_first_char_of_every_line(base_text_3);

    let buffer_1 = cx.new(|cx| Buffer::local(text_1.clone(), cx));
    let buffer_2 = cx.new(|cx| Buffer::local(text_2.clone(), cx));
    let buffer_3 = cx.new(|cx| Buffer::local(text_3.clone(), cx));

    let multibuffer = cx.new(|cx| {
        let mut multibuffer = MultiBuffer::new(ReadWrite);
        multibuffer.push_excerpts(
            buffer_1.clone(),
            [
                ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0)),
                ExcerptRange::new(Point::new(5, 0)..Point::new(7, 0)),
                ExcerptRange::new(Point::new(9, 0)..Point::new(10, 4)),
            ],
            cx,
        );
        multibuffer.push_excerpts(
            buffer_2.clone(),
            [
                ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0)),
                ExcerptRange::new(Point::new(5, 0)..Point::new(7, 0)),
                ExcerptRange::new(Point::new(9, 0)..Point::new(10, 4)),
            ],
            cx,
        );
        multibuffer.push_excerpts(
            buffer_3.clone(),
            [
                ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0)),
                ExcerptRange::new(Point::new(5, 0)..Point::new(7, 0)),
                ExcerptRange::new(Point::new(9, 0)..Point::new(10, 4)),
            ],
            cx,
        );
        multibuffer
    });

    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, [path!("/").as_ref()], cx).await;
    let (editor, cx) = cx
        .add_window_view(|window, cx| build_editor_with_project(project, multibuffer, window, cx));
    editor.update_in(cx, |editor, _window, cx| {
        for (buffer, diff_base) in [
            (buffer_1.clone(), base_text_1),
            (buffer_2.clone(), base_text_2),
            (buffer_3.clone(), base_text_3),
        ] {
            let diff = cx.new(|cx| BufferDiff::new_with_base_text(&diff_base, &buffer, cx));
            editor
                .buffer
                .update(cx, |buffer, cx| buffer.add_diff(diff, cx));
        }
    });
    cx.executor().run_until_parked();

    editor.update_in(cx, |editor, window, cx| {
        assert_eq!(editor.text(cx), "Xaaa\nXbbb\nXccc\n\nXfff\nXggg\n\nXjjj\nXlll\nXmmm\nXnnn\n\nXqqq\nXrrr\n\nXuuu\nXvvv\nXwww\nXxxx\n\nX{{{\nX|||\n\nX\u{7f}\u{7f}\u{7f}");
        editor.select_all(&SelectAll, window, cx);
        editor.git_restore(&Default::default(), window, cx);
    });
    cx.executor().run_until_parked();

    // When all ranges are selected, all buffer hunks are reverted.
    editor.update(cx, |editor, cx| {
        assert_eq!(editor.text(cx), "aaaa\nbbbb\ncccc\ndddd\neeee\nffff\ngggg\nhhhh\niiii\njjjj\n\n\nllll\nmmmm\nnnnn\noooo\npppp\nqqqq\nrrrr\nssss\ntttt\nuuuu\n\n\nvvvv\nwwww\nxxxx\nyyyy\nzzzz\n{{{{\n||||\n}}}}\n~~~~\n\u{7f}\u{7f}\u{7f}\u{7f}\n\n");
    });
    buffer_1.update(cx, |buffer, _| {
        assert_eq!(buffer.text(), base_text_1);
    });
    buffer_2.update(cx, |buffer, _| {
        assert_eq!(buffer.text(), base_text_2);
    });
    buffer_3.update(cx, |buffer, _| {
        assert_eq!(buffer.text(), base_text_3);
    });

    editor.update_in(cx, |editor, window, cx| {
        editor.undo(&Default::default(), window, cx);
    });

    editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges(Some(Point::new(0, 0)..Point::new(6, 0)));
        });
        editor.git_restore(&Default::default(), window, cx);
    });

    // Now, when all ranges selected belong to buffer_1, the revert should succeed,
    // but not affect buffer_2 and its related excerpts.
    editor.update(cx, |editor, cx| {
        assert_eq!(
            editor.text(cx),
            "aaaa\nbbbb\ncccc\ndddd\neeee\nffff\ngggg\nhhhh\niiii\njjjj\n\n\nXlll\nXmmm\nXnnn\n\nXqqq\nXrrr\n\nXuuu\nXvvv\nXwww\nXxxx\n\nX{{{\nX|||\n\nX\u{7f}\u{7f}\u{7f}"
        );
    });
    buffer_1.update(cx, |buffer, _| {
        assert_eq!(buffer.text(), base_text_1);
    });
    buffer_2.update(cx, |buffer, _| {
        assert_eq!(
            buffer.text(),
            "Xlll\nXmmm\nXnnn\nXooo\nXppp\nXqqq\nXrrr\nXsss\nXttt\nXuuu"
        );
    });
    buffer_3.update(cx, |buffer, _| {
        assert_eq!(
            buffer.text(),
            "Xvvv\nXwww\nXxxx\nXyyy\nXzzz\nX{{{\nX|||\nX}}}\nX~~~\nX\u{7f}\u{7f}\u{7f}"
        );
    });

    fn edit_first_char_of_every_line(text: &str) -> String {
        text.split('\n')
            .map(|line| format!("X{}", &line[1..]))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[gpui::test]
async fn test_mutlibuffer_in_navigation_history(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let cols = 4;
    let rows = 10;
    let sample_text_1 = sample_text(rows, cols, 'a');
    assert_eq!(
        sample_text_1,
        "aaaa\nbbbb\ncccc\ndddd\neeee\nffff\ngggg\nhhhh\niiii\njjjj"
    );
    let sample_text_2 = sample_text(rows, cols, 'l');
    assert_eq!(
        sample_text_2,
        "llll\nmmmm\nnnnn\noooo\npppp\nqqqq\nrrrr\nssss\ntttt\nuuuu"
    );
    let sample_text_3 = sample_text(rows, cols, 'v');
    assert_eq!(
        sample_text_3,
        "vvvv\nwwww\nxxxx\nyyyy\nzzzz\n{{{{\n||||\n}}}}\n~~~~\n\u{7f}\u{7f}\u{7f}\u{7f}"
    );

    let buffer_1 = cx.new(|cx| Buffer::local(sample_text_1.clone(), cx));
    let buffer_2 = cx.new(|cx| Buffer::local(sample_text_2.clone(), cx));
    let buffer_3 = cx.new(|cx| Buffer::local(sample_text_3.clone(), cx));

    let multi_buffer = cx.new(|cx| {
        let mut multibuffer = MultiBuffer::new(ReadWrite);
        multibuffer.push_excerpts(
            buffer_1.clone(),
            [
                ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0)),
                ExcerptRange::new(Point::new(5, 0)..Point::new(7, 0)),
                ExcerptRange::new(Point::new(9, 0)..Point::new(10, 4)),
            ],
            cx,
        );
        multibuffer.push_excerpts(
            buffer_2.clone(),
            [
                ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0)),
                ExcerptRange::new(Point::new(5, 0)..Point::new(7, 0)),
                ExcerptRange::new(Point::new(9, 0)..Point::new(10, 4)),
            ],
            cx,
        );
        multibuffer.push_excerpts(
            buffer_3.clone(),
            [
                ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0)),
                ExcerptRange::new(Point::new(5, 0)..Point::new(7, 0)),
                ExcerptRange::new(Point::new(9, 0)..Point::new(10, 4)),
            ],
            cx,
        );
        multibuffer
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/a",
        json!({
            "main.rs": sample_text_1,
            "other.rs": sample_text_2,
            "lib.rs": sample_text_3,
        }),
    )
    .await;
    let project = Project::test(fs, ["/a".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace.deref(), cx);
    let multi_buffer_editor = cx.new_window_entity(|window, cx| {
        Editor::new(
            EditorMode::full(),
            multi_buffer,
            Some(project.clone()),
            window,
            cx,
        )
    });
    let multibuffer_item_id = workspace
        .update(cx, |workspace, window, cx| {
            assert!(
                workspace.active_item(cx).is_none(),
                "active item should be None before the first item is added"
            );
            workspace.add_item_to_active_pane(
                Box::new(multi_buffer_editor.clone()),
                None,
                true,
                window,
                cx,
            );
            let active_item = workspace
                .active_item(cx)
                .expect("should have an active item after adding the multi buffer");
            assert!(
                !active_item.is_singleton(cx),
                "A multi buffer was expected to active after adding"
            );
            active_item.item_id()
        })
        .unwrap();
    cx.executor().run_until_parked();

    multi_buffer_editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(Some(Autoscroll::Next), window, cx, |s| {
            s.select_ranges(Some(1..2))
        });
        editor.open_excerpts(&OpenExcerpts, window, cx);
    });
    cx.executor().run_until_parked();
    let first_item_id = workspace
        .update(cx, |workspace, window, cx| {
            let active_item = workspace
                .active_item(cx)
                .expect("should have an active item after navigating into the 1st buffer");
            let first_item_id = active_item.item_id();
            assert_ne!(
                first_item_id, multibuffer_item_id,
                "Should navigate into the 1st buffer and activate it"
            );
            assert!(
                active_item.is_singleton(cx),
                "New active item should be a singleton buffer"
            );
            assert_eq!(
                active_item
                    .act_as::<Editor>(cx)
                    .expect("should have navigated into an editor for the 1st buffer")
                    .read(cx)
                    .text(cx),
                sample_text_1
            );

            workspace
                .go_back(workspace.active_pane().downgrade(), window, cx)
                .detach_and_log_err(cx);

            first_item_id
        })
        .unwrap();
    cx.executor().run_until_parked();
    workspace
        .update(cx, |workspace, _, cx| {
            let active_item = workspace
                .active_item(cx)
                .expect("should have an active item after navigating back");
            assert_eq!(
                active_item.item_id(),
                multibuffer_item_id,
                "Should navigate back to the multi buffer"
            );
            assert!(!active_item.is_singleton(cx));
        })
        .unwrap();

    multi_buffer_editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(Some(Autoscroll::Next), window, cx, |s| {
            s.select_ranges(Some(39..40))
        });
        editor.open_excerpts(&OpenExcerpts, window, cx);
    });
    cx.executor().run_until_parked();
    let second_item_id = workspace
        .update(cx, |workspace, window, cx| {
            let active_item = workspace
                .active_item(cx)
                .expect("should have an active item after navigating into the 2nd buffer");
            let second_item_id = active_item.item_id();
            assert_ne!(
                second_item_id, multibuffer_item_id,
                "Should navigate away from the multibuffer"
            );
            assert_ne!(
                second_item_id, first_item_id,
                "Should navigate into the 2nd buffer and activate it"
            );
            assert!(
                active_item.is_singleton(cx),
                "New active item should be a singleton buffer"
            );
            assert_eq!(
                active_item
                    .act_as::<Editor>(cx)
                    .expect("should have navigated into an editor")
                    .read(cx)
                    .text(cx),
                sample_text_2
            );

            workspace
                .go_back(workspace.active_pane().downgrade(), window, cx)
                .detach_and_log_err(cx);

            second_item_id
        })
        .unwrap();
    cx.executor().run_until_parked();
    workspace
        .update(cx, |workspace, _, cx| {
            let active_item = workspace
                .active_item(cx)
                .expect("should have an active item after navigating back from the 2nd buffer");
            assert_eq!(
                active_item.item_id(),
                multibuffer_item_id,
                "Should navigate back from the 2nd buffer to the multi buffer"
            );
            assert!(!active_item.is_singleton(cx));
        })
        .unwrap();

    multi_buffer_editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(Some(Autoscroll::Next), window, cx, |s| {
            s.select_ranges(Some(70..70))
        });
        editor.open_excerpts(&OpenExcerpts, window, cx);
    });
    cx.executor().run_until_parked();
    workspace
        .update(cx, |workspace, window, cx| {
            let active_item = workspace
                .active_item(cx)
                .expect("should have an active item after navigating into the 3rd buffer");
            let third_item_id = active_item.item_id();
            assert_ne!(
                third_item_id, multibuffer_item_id,
                "Should navigate into the 3rd buffer and activate it"
            );
            assert_ne!(third_item_id, first_item_id);
            assert_ne!(third_item_id, second_item_id);
            assert!(
                active_item.is_singleton(cx),
                "New active item should be a singleton buffer"
            );
            assert_eq!(
                active_item
                    .act_as::<Editor>(cx)
                    .expect("should have navigated into an editor")
                    .read(cx)
                    .text(cx),
                sample_text_3
            );

            workspace
                .go_back(workspace.active_pane().downgrade(), window, cx)
                .detach_and_log_err(cx);
        })
        .unwrap();
    cx.executor().run_until_parked();
    workspace
        .update(cx, |workspace, _, cx| {
            let active_item = workspace
                .active_item(cx)
                .expect("should have an active item after navigating back from the 3rd buffer");
            assert_eq!(
                active_item.item_id(),
                multibuffer_item_id,
                "Should navigate back from the 3rd buffer to the multi buffer"
            );
            assert!(!active_item.is_singleton(cx));
        })
        .unwrap();
}

#[gpui::test]
async fn test_toggle_selected_diff_hunks(executor: BackgroundExecutor, cx: &mut TestAppContext) {
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

    cx.set_head_text(&diff_base);
    executor.run_until_parked();

    cx.update_editor(|editor, window, cx| {
        editor.go_to_next_hunk(&GoToHunk, window, cx);
        editor.toggle_selected_diff_hunks(&ToggleSelectedDiffHunks, window, cx);
    });
    executor.run_until_parked();
    cx.assert_state_with_diff(
        r#"
          use some::modified;


          fn main() {
        -     println!("hello");
        + ˇ    println!("hello there");

              println!("around the");
              println!("world");
          }
        "#
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        for _ in 0..2 {
            editor.go_to_next_hunk(&GoToHunk, window, cx);
            editor.toggle_selected_diff_hunks(&ToggleSelectedDiffHunks, window, cx);
        }
    });
    executor.run_until_parked();
    cx.assert_state_with_diff(
        r#"
        - use some::mod;
        + ˇuse some::modified;


          fn main() {
        -     println!("hello");
        +     println!("hello there");

        +     println!("around the");
              println!("world");
          }
        "#
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.go_to_next_hunk(&GoToHunk, window, cx);
        editor.toggle_selected_diff_hunks(&ToggleSelectedDiffHunks, window, cx);
    });
    executor.run_until_parked();
    cx.assert_state_with_diff(
        r#"
        - use some::mod;
        + use some::modified;

        - const A: u32 = 42;
          ˇ
          fn main() {
        -     println!("hello");
        +     println!("hello there");

        +     println!("around the");
              println!("world");
          }
        "#
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.cancel(&Cancel, window, cx);
    });

    cx.assert_state_with_diff(
        r#"
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
}

#[gpui::test]
async fn test_diff_base_change_with_expanded_diff_hunks(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    let diff_base = r#"
        use some::mod1;
        use some::mod2;

        const A: u32 = 42;
        const B: u32 = 42;
        const C: u32 = 42;

        fn main() {
            println!("hello");

            println!("world");
        }
        "#
    .unindent();

    cx.set_state(
        &r#"
        use some::mod2;

        const A: u32 = 42;
        const C: u32 = 42;

        fn main(ˇ) {
            //println!("hello");

            println!("world");
            //
            //
        }
        "#
        .unindent(),
    );

    cx.set_head_text(&diff_base);
    executor.run_until_parked();

    cx.update_editor(|editor, window, cx| {
        editor.expand_all_diff_hunks(&ExpandAllDiffHunks, window, cx);
    });
    executor.run_until_parked();
    cx.assert_state_with_diff(
        r#"
        - use some::mod1;
          use some::mod2;

          const A: u32 = 42;
        - const B: u32 = 42;
          const C: u32 = 42;

          fn main(ˇ) {
        -     println!("hello");
        +     //println!("hello");

              println!("world");
        +     //
        +     //
          }
        "#
        .unindent(),
    );

    cx.set_head_text("new diff base!");
    executor.run_until_parked();
    cx.assert_state_with_diff(
        r#"
        - new diff base!
        + use some::mod2;
        +
        + const A: u32 = 42;
        + const C: u32 = 42;
        +
        + fn main(ˇ) {
        +     //println!("hello");
        +
        +     println!("world");
        +     //
        +     //
        + }
        "#
        .unindent(),
    );
}

#[gpui::test]
async fn test_toggle_diff_expand_in_multi_buffer(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let file_1_old = "aaa\nbbb\nccc\nddd\neee\nfff\nggg\nhhh\niii\njjj";
    let file_1_new = "aaa\nccc\nddd\neee\nfff\nggg\nhhh\niii\njjj";
    let file_2_old = "lll\nmmm\nnnn\nooo\nppp\nqqq\nrrr\nsss\nttt\nuuu";
    let file_2_new = "lll\nmmm\nNNN\nooo\nppp\nqqq\nrrr\nsss\nttt\nuuu";
    let file_3_old = "111\n222\n333\n444\n555\n777\n888\n999\n000\n!!!";
    let file_3_new = "111\n222\n333\n444\n555\n666\n777\n888\n999\n000\n!!!";

    let buffer_1 = cx.new(|cx| Buffer::local(file_1_new.to_string(), cx));
    let buffer_2 = cx.new(|cx| Buffer::local(file_2_new.to_string(), cx));
    let buffer_3 = cx.new(|cx| Buffer::local(file_3_new.to_string(), cx));

    let multi_buffer = cx.new(|cx| {
        let mut multibuffer = MultiBuffer::new(ReadWrite);
        multibuffer.push_excerpts(
            buffer_1.clone(),
            [
                ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0)),
                ExcerptRange::new(Point::new(5, 0)..Point::new(7, 0)),
                ExcerptRange::new(Point::new(9, 0)..Point::new(10, 3)),
            ],
            cx,
        );
        multibuffer.push_excerpts(
            buffer_2.clone(),
            [
                ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0)),
                ExcerptRange::new(Point::new(5, 0)..Point::new(7, 0)),
                ExcerptRange::new(Point::new(9, 0)..Point::new(10, 3)),
            ],
            cx,
        );
        multibuffer.push_excerpts(
            buffer_3.clone(),
            [
                ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0)),
                ExcerptRange::new(Point::new(5, 0)..Point::new(7, 0)),
                ExcerptRange::new(Point::new(9, 0)..Point::new(10, 3)),
            ],
            cx,
        );
        multibuffer
    });

    let editor =
        cx.add_window(|window, cx| Editor::new(EditorMode::full(), multi_buffer, None, window, cx));
    editor
        .update(cx, |editor, _window, cx| {
            for (buffer, diff_base) in [
                (buffer_1.clone(), file_1_old),
                (buffer_2.clone(), file_2_old),
                (buffer_3.clone(), file_3_old),
            ] {
                let diff = cx.new(|cx| BufferDiff::new_with_base_text(&diff_base, &buffer, cx));
                editor
                    .buffer
                    .update(cx, |buffer, cx| buffer.add_diff(diff, cx));
            }
        })
        .unwrap();

    let mut cx = EditorTestContext::for_editor(editor, cx).await;
    cx.run_until_parked();

    cx.assert_editor_state(
        &"
            ˇaaa
            ccc
            ddd

            ggg
            hhh


            lll
            mmm
            NNN

            qqq
            rrr

            uuu
            111
            222
            333

            666
            777

            000
            !!!"
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.select_all(&SelectAll, window, cx);
        editor.toggle_selected_diff_hunks(&ToggleSelectedDiffHunks, window, cx);
    });
    cx.executor().run_until_parked();

    cx.assert_state_with_diff(
        "
            «aaa
          - bbb
            ccc
            ddd

            ggg
            hhh


            lll
            mmm
          - nnn
          + NNN

            qqq
            rrr

            uuu
            111
            222
            333

          + 666
            777

            000
            !!!ˇ»"
            .unindent(),
    );
}

#[gpui::test]
async fn test_expand_diff_hunk_at_excerpt_boundary(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let base = "aaa\nbbb\nccc\nddd\neee\nfff\nggg\n";
    let text = "aaa\nBBB\nBB2\nccc\nDDD\nEEE\nfff\nggg\nhhh\niii\n";

    let buffer = cx.new(|cx| Buffer::local(text.to_string(), cx));
    let multi_buffer = cx.new(|cx| {
        let mut multibuffer = MultiBuffer::new(ReadWrite);
        multibuffer.push_excerpts(
            buffer.clone(),
            [
                ExcerptRange::new(Point::new(0, 0)..Point::new(2, 0)),
                ExcerptRange::new(Point::new(4, 0)..Point::new(7, 0)),
                ExcerptRange::new(Point::new(9, 0)..Point::new(10, 0)),
            ],
            cx,
        );
        multibuffer
    });

    let editor =
        cx.add_window(|window, cx| Editor::new(EditorMode::full(), multi_buffer, None, window, cx));
    editor
        .update(cx, |editor, _window, cx| {
            let diff = cx.new(|cx| BufferDiff::new_with_base_text(base, &buffer, cx));
            editor
                .buffer
                .update(cx, |buffer, cx| buffer.add_diff(diff, cx))
        })
        .unwrap();

    let mut cx = EditorTestContext::for_editor(editor, cx).await;
    cx.run_until_parked();

    cx.update_editor(|editor, window, cx| {
        editor.expand_all_diff_hunks(&Default::default(), window, cx)
    });
    cx.executor().run_until_parked();

    // When the start of a hunk coincides with the start of its excerpt,
    // the hunk is expanded. When the start of a a hunk is earlier than
    // the start of its excerpt, the hunk is not expanded.
    cx.assert_state_with_diff(
        "
            ˇaaa
          - bbb
          + BBB

          - ddd
          - eee
          + DDD
          + EEE
            fff

            iii
        "
        .unindent(),
    );
}

#[gpui::test]
async fn test_edits_around_expanded_insertion_hunks(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    let diff_base = r#"
        use some::mod1;
        use some::mod2;

        const A: u32 = 42;

        fn main() {
            println!("hello");

            println!("world");
        }
        "#
    .unindent();
    executor.run_until_parked();
    cx.set_state(
        &r#"
        use some::mod1;
        use some::mod2;

        const A: u32 = 42;
        const B: u32 = 42;
        const C: u32 = 42;
        ˇ

        fn main() {
            println!("hello");

            println!("world");
        }
        "#
        .unindent(),
    );

    cx.set_head_text(&diff_base);
    executor.run_until_parked();

    cx.update_editor(|editor, window, cx| {
        editor.expand_all_diff_hunks(&ExpandAllDiffHunks, window, cx);
    });
    executor.run_until_parked();

    cx.assert_state_with_diff(
        r#"
        use some::mod1;
        use some::mod2;

        const A: u32 = 42;
      + const B: u32 = 42;
      + const C: u32 = 42;
      + ˇ

        fn main() {
            println!("hello");

            println!("world");
        }
      "#
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| editor.handle_input("const D: u32 = 42;\n", window, cx));
    executor.run_until_parked();

    cx.assert_state_with_diff(
        r#"
        use some::mod1;
        use some::mod2;

        const A: u32 = 42;
      + const B: u32 = 42;
      + const C: u32 = 42;
      + const D: u32 = 42;
      + ˇ

        fn main() {
            println!("hello");

            println!("world");
        }
      "#
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| editor.handle_input("const E: u32 = 42;\n", window, cx));
    executor.run_until_parked();

    cx.assert_state_with_diff(
        r#"
        use some::mod1;
        use some::mod2;

        const A: u32 = 42;
      + const B: u32 = 42;
      + const C: u32 = 42;
      + const D: u32 = 42;
      + const E: u32 = 42;
      + ˇ

        fn main() {
            println!("hello");

            println!("world");
        }
      "#
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.delete_line(&DeleteLine, window, cx);
    });
    executor.run_until_parked();

    cx.assert_state_with_diff(
        r#"
        use some::mod1;
        use some::mod2;

        const A: u32 = 42;
      + const B: u32 = 42;
      + const C: u32 = 42;
      + const D: u32 = 42;
      + const E: u32 = 42;
        ˇ
        fn main() {
            println!("hello");

            println!("world");
        }
      "#
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.move_up(&MoveUp, window, cx);
        editor.delete_line(&DeleteLine, window, cx);
        editor.move_up(&MoveUp, window, cx);
        editor.delete_line(&DeleteLine, window, cx);
        editor.move_up(&MoveUp, window, cx);
        editor.delete_line(&DeleteLine, window, cx);
    });
    executor.run_until_parked();
    cx.assert_state_with_diff(
        r#"
        use some::mod1;
        use some::mod2;

        const A: u32 = 42;
      + const B: u32 = 42;
        ˇ
        fn main() {
            println!("hello");

            println!("world");
        }
      "#
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.select_up_by_lines(&SelectUpByLines { lines: 5 }, window, cx);
        editor.delete_line(&DeleteLine, window, cx);
    });
    executor.run_until_parked();
    cx.assert_state_with_diff(
        r#"
        ˇ
        fn main() {
            println!("hello");

            println!("world");
        }
      "#
        .unindent(),
    );
}

#[gpui::test]
async fn test_toggling_adjacent_diff_hunks(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_head_text(indoc! { "
        one
        two
        three
        four
        five
        "
    });
    cx.set_state(indoc! { "
        one
        ˇthree
        five
    "});
    cx.run_until_parked();
    cx.update_editor(|editor, window, cx| {
        editor.toggle_selected_diff_hunks(&Default::default(), window, cx);
    });
    cx.assert_state_with_diff(
        indoc! { "
        one
      - two
        ˇthree
      - four
        five
    "}
        .to_string(),
    );
    cx.update_editor(|editor, window, cx| {
        editor.toggle_selected_diff_hunks(&Default::default(), window, cx);
    });

    cx.assert_state_with_diff(
        indoc! { "
        one
        ˇthree
        five
    "}
        .to_string(),
    );

    cx.set_state(indoc! { "
        one
        ˇTWO
        three
        four
        five
    "});
    cx.run_until_parked();
    cx.update_editor(|editor, window, cx| {
        editor.toggle_selected_diff_hunks(&Default::default(), window, cx);
    });

    cx.assert_state_with_diff(
        indoc! { "
            one
          - two
          + ˇTWO
            three
            four
            five
        "}
        .to_string(),
    );
    cx.update_editor(|editor, window, cx| {
        editor.move_up(&Default::default(), window, cx);
        editor.toggle_selected_diff_hunks(&Default::default(), window, cx);
    });
    cx.assert_state_with_diff(
        indoc! { "
            one
            ˇTWO
            three
            four
            five
        "}
        .to_string(),
    );
}

#[gpui::test]
async fn test_edits_around_expanded_deletion_hunks(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    let diff_base = r#"
        use some::mod1;
        use some::mod2;

        const A: u32 = 42;
        const B: u32 = 42;
        const C: u32 = 42;


        fn main() {
            println!("hello");

            println!("world");
        }
    "#
    .unindent();
    executor.run_until_parked();
    cx.set_state(
        &r#"
        use some::mod1;
        use some::mod2;

        ˇconst B: u32 = 42;
        const C: u32 = 42;


        fn main() {
            println!("hello");

            println!("world");
        }
        "#
        .unindent(),
    );

    cx.set_head_text(&diff_base);
    executor.run_until_parked();

    cx.update_editor(|editor, window, cx| {
        editor.expand_all_diff_hunks(&ExpandAllDiffHunks, window, cx);
    });
    executor.run_until_parked();

    cx.assert_state_with_diff(
        r#"
        use some::mod1;
        use some::mod2;

      - const A: u32 = 42;
        ˇconst B: u32 = 42;
        const C: u32 = 42;


        fn main() {
            println!("hello");

            println!("world");
        }
      "#
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.delete_line(&DeleteLine, window, cx);
    });
    executor.run_until_parked();
    cx.assert_state_with_diff(
        r#"
        use some::mod1;
        use some::mod2;

      - const A: u32 = 42;
      - const B: u32 = 42;
        ˇconst C: u32 = 42;


        fn main() {
            println!("hello");

            println!("world");
        }
      "#
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.delete_line(&DeleteLine, window, cx);
    });
    executor.run_until_parked();
    cx.assert_state_with_diff(
        r#"
        use some::mod1;
        use some::mod2;

      - const A: u32 = 42;
      - const B: u32 = 42;
      - const C: u32 = 42;
        ˇ

        fn main() {
            println!("hello");

            println!("world");
        }
      "#
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.handle_input("replacement", window, cx);
    });
    executor.run_until_parked();
    cx.assert_state_with_diff(
        r#"
        use some::mod1;
        use some::mod2;

      - const A: u32 = 42;
      - const B: u32 = 42;
      - const C: u32 = 42;
      -
      + replacementˇ

        fn main() {
            println!("hello");

            println!("world");
        }
      "#
        .unindent(),
    );
}

#[gpui::test]
async fn test_backspace_after_deletion_hunk(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    let base_text = r#"
        one
        two
        three
        four
        five
    "#
    .unindent();
    executor.run_until_parked();
    cx.set_state(
        &r#"
        one
        two
        fˇour
        five
        "#
        .unindent(),
    );

    cx.set_head_text(&base_text);
    executor.run_until_parked();

    cx.update_editor(|editor, window, cx| {
        editor.expand_all_diff_hunks(&ExpandAllDiffHunks, window, cx);
    });
    executor.run_until_parked();

    cx.assert_state_with_diff(
        r#"
          one
          two
        - three
          fˇour
          five
        "#
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.backspace(&Backspace, window, cx);
        editor.backspace(&Backspace, window, cx);
    });
    executor.run_until_parked();
    cx.assert_state_with_diff(
        r#"
          one
          two
        - threeˇ
        - four
        + our
          five
        "#
        .unindent(),
    );
}

#[gpui::test]
async fn test_edit_after_expanded_modification_hunk(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    let diff_base = r#"
        use some::mod1;
        use some::mod2;

        const A: u32 = 42;
        const B: u32 = 42;
        const C: u32 = 42;
        const D: u32 = 42;


        fn main() {
            println!("hello");

            println!("world");
        }"#
    .unindent();

    cx.set_state(
        &r#"
        use some::mod1;
        use some::mod2;

        const A: u32 = 42;
        const B: u32 = 42;
        const C: u32 = 43ˇ
        const D: u32 = 42;


        fn main() {
            println!("hello");

            println!("world");
        }"#
        .unindent(),
    );

    cx.set_head_text(&diff_base);
    executor.run_until_parked();
    cx.update_editor(|editor, window, cx| {
        editor.expand_all_diff_hunks(&ExpandAllDiffHunks, window, cx);
    });
    executor.run_until_parked();

    cx.assert_state_with_diff(
        r#"
        use some::mod1;
        use some::mod2;

        const A: u32 = 42;
        const B: u32 = 42;
      - const C: u32 = 42;
      + const C: u32 = 43ˇ
        const D: u32 = 42;


        fn main() {
            println!("hello");

            println!("world");
        }"#
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.handle_input("\nnew_line\n", window, cx);
    });
    executor.run_until_parked();

    cx.assert_state_with_diff(
        r#"
        use some::mod1;
        use some::mod2;

        const A: u32 = 42;
        const B: u32 = 42;
      - const C: u32 = 42;
      + const C: u32 = 43
      + new_line
      + ˇ
        const D: u32 = 42;


        fn main() {
            println!("hello");

            println!("world");
        }"#
        .unindent(),
    );
}

#[gpui::test]
async fn test_stage_and_unstage_added_file_hunk(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.update_editor(|editor, _, cx| {
        editor.set_expand_all_diff_hunks(cx);
    });

    let working_copy = r#"
            ˇfn main() {
                println!("hello, world!");
            }
        "#
    .unindent();

    cx.set_state(&working_copy);
    executor.run_until_parked();

    cx.assert_state_with_diff(
        r#"
            + ˇfn main() {
            +     println!("hello, world!");
            + }
        "#
        .unindent(),
    );
    cx.assert_index_text(None);

    cx.update_editor(|editor, window, cx| {
        editor.toggle_staged_selected_diff_hunks(&Default::default(), window, cx);
    });
    executor.run_until_parked();
    cx.assert_index_text(Some(&working_copy.replace("ˇ", "")));
    cx.assert_state_with_diff(
        r#"
            + ˇfn main() {
            +     println!("hello, world!");
            + }
        "#
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.toggle_staged_selected_diff_hunks(&Default::default(), window, cx);
    });
    executor.run_until_parked();
    cx.assert_index_text(None);
}

async fn setup_indent_guides_editor(
    text: &str,
    cx: &mut TestAppContext,
) -> (BufferId, EditorTestContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;

    let buffer_id = cx.update_editor(|editor, window, cx| {
        editor.set_text(text, window, cx);
        let buffer_ids = editor.buffer().read(cx).excerpt_buffer_ids();

        buffer_ids[0]
    });

    (buffer_id, cx)
}

fn assert_indent_guides(
    range: Range<u32>,
    expected: Vec<IndentGuide>,
    active_indices: Option<Vec<usize>>,
    cx: &mut EditorTestContext,
) {
    let indent_guides = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx).display_snapshot;
        let mut indent_guides: Vec<_> = crate::indent_guides::indent_guides_in_range(
            editor,
            MultiBufferRow(range.start)..MultiBufferRow(range.end),
            true,
            &snapshot,
            cx,
        );

        indent_guides.sort_by(|a, b| {
            a.depth.cmp(&b.depth).then(
                a.start_row
                    .cmp(&b.start_row)
                    .then(a.end_row.cmp(&b.end_row)),
            )
        });
        indent_guides
    });

    if let Some(expected) = active_indices {
        let active_indices = cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx).display_snapshot;
            editor.find_active_indent_guide_indices(&indent_guides, &snapshot, window, cx)
        });

        assert_eq!(
            active_indices.unwrap().into_iter().collect::<Vec<_>>(),
            expected,
            "Active indent guide indices do not match"
        );
    }

    assert_eq!(indent_guides, expected, "Indent guides do not match");
}

fn indent_guide(buffer_id: BufferId, start_row: u32, end_row: u32, depth: u32) -> IndentGuide {
    IndentGuide {
        buffer_id,
        start_row: MultiBufferRow(start_row),
        end_row: MultiBufferRow(end_row),
        depth,
        tab_size: 4,
        settings: IndentGuideSettings {
            enabled: true,
            line_width: 1,
            active_line_width: 1,
            ..Default::default()
        },
    }
}

#[gpui::test]
async fn test_indent_guide_single_line(cx: &mut TestAppContext) {
    let (buffer_id, mut cx) = setup_indent_guides_editor(
        &"
    fn main() {
        let a = 1;
    }"
        .unindent(),
        cx,
    )
    .await;

    assert_indent_guides(0..3, vec![indent_guide(buffer_id, 1, 1, 0)], None, &mut cx);
}

#[gpui::test]
async fn test_indent_guide_simple_block(cx: &mut TestAppContext) {
    let (buffer_id, mut cx) = setup_indent_guides_editor(
        &"
    fn main() {
        let a = 1;
        let b = 2;
    }"
        .unindent(),
        cx,
    )
    .await;

    assert_indent_guides(0..4, vec![indent_guide(buffer_id, 1, 2, 0)], None, &mut cx);
}

#[gpui::test]
async fn test_indent_guide_nested(cx: &mut TestAppContext) {
    let (buffer_id, mut cx) = setup_indent_guides_editor(
        &"
    fn main() {
        let a = 1;
        if a == 3 {
            let b = 2;
        } else {
            let c = 3;
        }
    }"
        .unindent(),
        cx,
    )
    .await;

    assert_indent_guides(
        0..8,
        vec![
            indent_guide(buffer_id, 1, 6, 0),
            indent_guide(buffer_id, 3, 3, 1),
            indent_guide(buffer_id, 5, 5, 1),
        ],
        None,
        &mut cx,
    );
}

#[gpui::test]
async fn test_indent_guide_tab(cx: &mut TestAppContext) {
    let (buffer_id, mut cx) = setup_indent_guides_editor(
        &"
    fn main() {
        let a = 1;
            let b = 2;
        let c = 3;
    }"
        .unindent(),
        cx,
    )
    .await;

    assert_indent_guides(
        0..5,
        vec![
            indent_guide(buffer_id, 1, 3, 0),
            indent_guide(buffer_id, 2, 2, 1),
        ],
        None,
        &mut cx,
    );
}

#[gpui::test]
async fn test_indent_guide_continues_on_empty_line(cx: &mut TestAppContext) {
    let (buffer_id, mut cx) = setup_indent_guides_editor(
        &"
        fn main() {
            let a = 1;

            let c = 3;
        }"
        .unindent(),
        cx,
    )
    .await;

    assert_indent_guides(0..5, vec![indent_guide(buffer_id, 1, 3, 0)], None, &mut cx);
}

#[gpui::test]
async fn test_indent_guide_complex(cx: &mut TestAppContext) {
    let (buffer_id, mut cx) = setup_indent_guides_editor(
        &"
        fn main() {
            let a = 1;

            let c = 3;

            if a == 3 {
                let b = 2;
            } else {
                let c = 3;
            }
        }"
        .unindent(),
        cx,
    )
    .await;

    assert_indent_guides(
        0..11,
        vec![
            indent_guide(buffer_id, 1, 9, 0),
            indent_guide(buffer_id, 6, 6, 1),
            indent_guide(buffer_id, 8, 8, 1),
        ],
        None,
        &mut cx,
    );
}

#[gpui::test]
async fn test_indent_guide_starts_off_screen(cx: &mut TestAppContext) {
    let (buffer_id, mut cx) = setup_indent_guides_editor(
        &"
        fn main() {
            let a = 1;

            let c = 3;

            if a == 3 {
                let b = 2;
            } else {
                let c = 3;
            }
        }"
        .unindent(),
        cx,
    )
    .await;

    assert_indent_guides(
        1..11,
        vec![
            indent_guide(buffer_id, 1, 9, 0),
            indent_guide(buffer_id, 6, 6, 1),
            indent_guide(buffer_id, 8, 8, 1),
        ],
        None,
        &mut cx,
    );
}

#[gpui::test]
async fn test_indent_guide_ends_off_screen(cx: &mut TestAppContext) {
    let (buffer_id, mut cx) = setup_indent_guides_editor(
        &"
        fn main() {
            let a = 1;

            let c = 3;

            if a == 3 {
                let b = 2;
            } else {
                let c = 3;
            }
        }"
        .unindent(),
        cx,
    )
    .await;

    assert_indent_guides(
        1..10,
        vec![
            indent_guide(buffer_id, 1, 9, 0),
            indent_guide(buffer_id, 6, 6, 1),
            indent_guide(buffer_id, 8, 8, 1),
        ],
        None,
        &mut cx,
    );
}

#[gpui::test]
async fn test_indent_guide_without_brackets(cx: &mut TestAppContext) {
    let (buffer_id, mut cx) = setup_indent_guides_editor(
        &"
        block1
            block2
                block3
                    block4
            block2
        block1
        block1"
            .unindent(),
        cx,
    )
    .await;

    assert_indent_guides(
        1..10,
        vec![
            indent_guide(buffer_id, 1, 4, 0),
            indent_guide(buffer_id, 2, 3, 1),
            indent_guide(buffer_id, 3, 3, 2),
        ],
        None,
        &mut cx,
    );
}

#[gpui::test]
async fn test_indent_guide_ends_before_empty_line(cx: &mut TestAppContext) {
    let (buffer_id, mut cx) = setup_indent_guides_editor(
        &"
        block1
            block2
                block3

        block1
        block1"
            .unindent(),
        cx,
    )
    .await;

    assert_indent_guides(
        0..6,
        vec![
            indent_guide(buffer_id, 1, 2, 0),
            indent_guide(buffer_id, 2, 2, 1),
        ],
        None,
        &mut cx,
    );
}

#[gpui::test]
async fn test_indent_guide_continuing_off_screen(cx: &mut TestAppContext) {
    let (buffer_id, mut cx) = setup_indent_guides_editor(
        &"
        block1



            block2
        "
        .unindent(),
        cx,
    )
    .await;

    assert_indent_guides(0..1, vec![indent_guide(buffer_id, 1, 1, 0)], None, &mut cx);
}

#[gpui::test]
async fn test_indent_guide_tabs(cx: &mut TestAppContext) {
    let (buffer_id, mut cx) = setup_indent_guides_editor(
        &"
        def a:
        \tb = 3
        \tif True:
        \t\tc = 4
        \t\td = 5
        \tprint(b)
        "
        .unindent(),
        cx,
    )
    .await;

    assert_indent_guides(
        0..6,
        vec![
            indent_guide(buffer_id, 1, 5, 0),
            indent_guide(buffer_id, 3, 4, 1),
        ],
        None,
        &mut cx,
    );
}

#[gpui::test]
async fn test_active_indent_guide_single_line(cx: &mut TestAppContext) {
    let (buffer_id, mut cx) = setup_indent_guides_editor(
        &"
    fn main() {
        let a = 1;
    }"
        .unindent(),
        cx,
    )
    .await;

    cx.update_editor(|editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(1, 0)..Point::new(1, 0)])
        });
    });

    assert_indent_guides(
        0..3,
        vec![indent_guide(buffer_id, 1, 1, 0)],
        Some(vec![0]),
        &mut cx,
    );
}

#[gpui::test]
async fn test_active_indent_guide_respect_indented_range(cx: &mut TestAppContext) {
    let (buffer_id, mut cx) = setup_indent_guides_editor(
        &"
    fn main() {
        if 1 == 2 {
            let a = 1;
        }
    }"
        .unindent(),
        cx,
    )
    .await;

    cx.update_editor(|editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(1, 0)..Point::new(1, 0)])
        });
    });

    assert_indent_guides(
        0..4,
        vec![
            indent_guide(buffer_id, 1, 3, 0),
            indent_guide(buffer_id, 2, 2, 1),
        ],
        Some(vec![1]),
        &mut cx,
    );

    cx.update_editor(|editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(2, 0)..Point::new(2, 0)])
        });
    });

    assert_indent_guides(
        0..4,
        vec![
            indent_guide(buffer_id, 1, 3, 0),
            indent_guide(buffer_id, 2, 2, 1),
        ],
        Some(vec![1]),
        &mut cx,
    );

    cx.update_editor(|editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(3, 0)..Point::new(3, 0)])
        });
    });

    assert_indent_guides(
        0..4,
        vec![
            indent_guide(buffer_id, 1, 3, 0),
            indent_guide(buffer_id, 2, 2, 1),
        ],
        Some(vec![0]),
        &mut cx,
    );
}

#[gpui::test]
async fn test_active_indent_guide_empty_line(cx: &mut TestAppContext) {
    let (buffer_id, mut cx) = setup_indent_guides_editor(
        &"
    fn main() {
        let a = 1;

        let b = 2;
    }"
        .unindent(),
        cx,
    )
    .await;

    cx.update_editor(|editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(2, 0)..Point::new(2, 0)])
        });
    });

    assert_indent_guides(
        0..5,
        vec![indent_guide(buffer_id, 1, 3, 0)],
        Some(vec![0]),
        &mut cx,
    );
}

#[gpui::test]
async fn test_active_indent_guide_non_matching_indent(cx: &mut TestAppContext) {
    let (buffer_id, mut cx) = setup_indent_guides_editor(
        &"
    def m:
        a = 1
        pass"
            .unindent(),
        cx,
    )
    .await;

    cx.update_editor(|editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(1, 0)..Point::new(1, 0)])
        });
    });

    assert_indent_guides(
        0..3,
        vec![indent_guide(buffer_id, 1, 2, 0)],
        Some(vec![0]),
        &mut cx,
    );
}

#[gpui::test]
async fn test_indent_guide_with_expanded_diff_hunks(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;
    let text = indoc! {
        "
        impl A {
            fn b() {
                0;
                3;
                5;
                6;
                7;
            }
        }
        "
    };
    let base_text = indoc! {
        "
        impl A {
            fn b() {
                0;
                1;
                2;
                3;
                4;
            }
            fn c() {
                5;
                6;
                7;
            }
        }
        "
    };

    cx.update_editor(|editor, window, cx| {
        editor.set_text(text, window, cx);

        editor.buffer().update(cx, |multibuffer, cx| {
            let buffer = multibuffer.as_singleton().unwrap();
            let diff = cx.new(|cx| BufferDiff::new_with_base_text(base_text, &buffer, cx));

            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer.add_diff(diff, cx);

            buffer.read(cx).remote_id()
        })
    });
    cx.run_until_parked();

    cx.assert_state_with_diff(
        indoc! { "
          impl A {
              fn b() {
                  0;
        -         1;
        -         2;
                  3;
        -         4;
        -     }
        -     fn c() {
                  5;
                  6;
                  7;
              }
          }
          ˇ"
        }
        .to_string(),
    );

    let mut actual_guides = cx.update_editor(|editor, window, cx| {
        editor
            .snapshot(window, cx)
            .buffer_snapshot
            .indent_guides_in_range(Anchor::min()..Anchor::max(), false, cx)
            .map(|guide| (guide.start_row..=guide.end_row, guide.depth))
            .collect::<Vec<_>>()
    });
    actual_guides.sort_by_key(|item| (*item.0.start(), item.1));
    assert_eq!(
        actual_guides,
        vec![
            (MultiBufferRow(1)..=MultiBufferRow(12), 0),
            (MultiBufferRow(2)..=MultiBufferRow(6), 1),
            (MultiBufferRow(9)..=MultiBufferRow(11), 1),
        ]
    );
}

#[gpui::test]
async fn test_adjacent_diff_hunks(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;

    let diff_base = r#"
        a
        b
        c
        "#
    .unindent();

    cx.set_state(
        &r#"
        ˇA
        b
        C
        "#
        .unindent(),
    );
    cx.set_head_text(&diff_base);
    cx.update_editor(|editor, window, cx| {
        editor.expand_all_diff_hunks(&ExpandAllDiffHunks, window, cx);
    });
    executor.run_until_parked();

    let both_hunks_expanded = r#"
        - a
        + ˇA
          b
        - c
        + C
        "#
    .unindent();

    cx.assert_state_with_diff(both_hunks_expanded.clone());

    let hunk_ranges = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let hunks = editor
            .diff_hunks_in_ranges(&[Anchor::min()..Anchor::max()], &snapshot.buffer_snapshot)
            .collect::<Vec<_>>();
        let excerpt_id = editor.buffer.read(cx).excerpt_ids()[0];
        let buffer_id = hunks[0].buffer_id;
        hunks
            .into_iter()
            .map(|hunk| Anchor::range_in_buffer(excerpt_id, buffer_id, hunk.buffer_range.clone()))
            .collect::<Vec<_>>()
    });
    assert_eq!(hunk_ranges.len(), 2);

    cx.update_editor(|editor, _, cx| {
        editor.toggle_single_diff_hunk(hunk_ranges[0].clone(), cx);
    });
    executor.run_until_parked();

    let second_hunk_expanded = r#"
          ˇA
          b
        - c
        + C
        "#
    .unindent();

    cx.assert_state_with_diff(second_hunk_expanded);

    cx.update_editor(|editor, _, cx| {
        editor.toggle_single_diff_hunk(hunk_ranges[0].clone(), cx);
    });
    executor.run_until_parked();

    cx.assert_state_with_diff(both_hunks_expanded.clone());

    cx.update_editor(|editor, _, cx| {
        editor.toggle_single_diff_hunk(hunk_ranges[1].clone(), cx);
    });
    executor.run_until_parked();

    let first_hunk_expanded = r#"
        - a
        + ˇA
          b
          C
        "#
    .unindent();

    cx.assert_state_with_diff(first_hunk_expanded);

    cx.update_editor(|editor, _, cx| {
        editor.toggle_single_diff_hunk(hunk_ranges[1].clone(), cx);
    });
    executor.run_until_parked();

    cx.assert_state_with_diff(both_hunks_expanded);

    cx.set_state(
        &r#"
        ˇA
        b
        "#
        .unindent(),
    );
    cx.run_until_parked();

    // TODO this cursor position seems bad
    cx.assert_state_with_diff(
        r#"
        - ˇa
        + A
          b
        "#
        .unindent(),
    );

    cx.update_editor(|editor, window, cx| {
        editor.expand_all_diff_hunks(&ExpandAllDiffHunks, window, cx);
    });

    cx.assert_state_with_diff(
        r#"
            - ˇa
            + A
              b
            - c
            "#
        .unindent(),
    );

    let hunk_ranges = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let hunks = editor
            .diff_hunks_in_ranges(&[Anchor::min()..Anchor::max()], &snapshot.buffer_snapshot)
            .collect::<Vec<_>>();
        let excerpt_id = editor.buffer.read(cx).excerpt_ids()[0];
        let buffer_id = hunks[0].buffer_id;
        hunks
            .into_iter()
            .map(|hunk| Anchor::range_in_buffer(excerpt_id, buffer_id, hunk.buffer_range.clone()))
            .collect::<Vec<_>>()
    });
    assert_eq!(hunk_ranges.len(), 2);

    cx.update_editor(|editor, _, cx| {
        editor.toggle_single_diff_hunk(hunk_ranges[1].clone(), cx);
    });
    executor.run_until_parked();

    cx.assert_state_with_diff(
        r#"
        - ˇa
        + A
          b
        "#
        .unindent(),
    );
}

#[gpui::test]
async fn test_toggle_deletion_hunk_at_start_of_file(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;

    let diff_base = r#"
        a
        b
        c
        "#
    .unindent();

    cx.set_state(
        &r#"
        ˇb
        c
        "#
        .unindent(),
    );
    cx.set_head_text(&diff_base);
    cx.update_editor(|editor, window, cx| {
        editor.expand_all_diff_hunks(&ExpandAllDiffHunks, window, cx);
    });
    executor.run_until_parked();

    let hunk_expanded = r#"
        - a
          ˇb
          c
        "#
    .unindent();

    cx.assert_state_with_diff(hunk_expanded.clone());

    let hunk_ranges = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let hunks = editor
            .diff_hunks_in_ranges(&[Anchor::min()..Anchor::max()], &snapshot.buffer_snapshot)
            .collect::<Vec<_>>();
        let excerpt_id = editor.buffer.read(cx).excerpt_ids()[0];
        let buffer_id = hunks[0].buffer_id;
        hunks
            .into_iter()
            .map(|hunk| Anchor::range_in_buffer(excerpt_id, buffer_id, hunk.buffer_range.clone()))
            .collect::<Vec<_>>()
    });
    assert_eq!(hunk_ranges.len(), 1);

    cx.update_editor(|editor, _, cx| {
        editor.toggle_single_diff_hunk(hunk_ranges[0].clone(), cx);
    });
    executor.run_until_parked();

    let hunk_collapsed = r#"
          ˇb
          c
        "#
    .unindent();

    cx.assert_state_with_diff(hunk_collapsed);

    cx.update_editor(|editor, _, cx| {
        editor.toggle_single_diff_hunk(hunk_ranges[0].clone(), cx);
    });
    executor.run_until_parked();

    cx.assert_state_with_diff(hunk_expanded.clone());
}

#[gpui::test]
async fn test_display_diff_hunks(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/test"),
        json!({
            ".git": {},
            "file-1": "ONE\n",
            "file-2": "TWO\n",
            "file-3": "THREE\n",
        }),
    )
    .await;

    fs.set_head_for_repo(
        path!("/test/.git").as_ref(),
        &[
            ("file-1".into(), "one\n".into()),
            ("file-2".into(), "two\n".into()),
            ("file-3".into(), "three\n".into()),
        ],
    );

    let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
    let mut buffers = vec![];
    for i in 1..=3 {
        let buffer = project
            .update(cx, |project, cx| {
                let path = format!(path!("/test/file-{}"), i);
                project.open_local_buffer(path, cx)
            })
            .await
            .unwrap();
        buffers.push(buffer);
    }

    let multibuffer = cx.new(|cx| {
        let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
        multibuffer.set_all_diff_hunks_expanded(cx);
        for buffer in &buffers {
            let snapshot = buffer.read(cx).snapshot();
            multibuffer.set_excerpts_for_path(
                PathKey::namespaced(0, buffer.read(cx).file().unwrap().path().clone()),
                buffer.clone(),
                vec![text::Anchor::MIN.to_point(&snapshot)..text::Anchor::MAX.to_point(&snapshot)],
                DEFAULT_MULTIBUFFER_CONTEXT,
                cx,
            );
        }
        multibuffer
    });

    let editor = cx.add_window(|window, cx| {
        Editor::new(EditorMode::full(), multibuffer, Some(project), window, cx)
    });
    cx.run_until_parked();

    let snapshot = editor
        .update(cx, |editor, window, cx| editor.snapshot(window, cx))
        .unwrap();
    let hunks = snapshot
        .display_diff_hunks_for_rows(DisplayRow(0)..DisplayRow(u32::MAX), &Default::default())
        .map(|hunk| match hunk {
            DisplayDiffHunk::Unfolded {
                display_row_range, ..
            } => display_row_range,
            DisplayDiffHunk::Folded { .. } => unreachable!(),
        })
        .collect::<Vec<_>>();
    assert_eq!(
        hunks,
        [
            DisplayRow(2)..DisplayRow(4),
            DisplayRow(7)..DisplayRow(9),
            DisplayRow(12)..DisplayRow(14),
        ]
    );
}

#[gpui::test]
async fn test_partially_staged_hunk(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_head_text(indoc! { "
        one
        two
        three
        four
        five
        "
    });
    cx.set_index_text(indoc! { "
        one
        two
        three
        four
        five
        "
    });
    cx.set_state(indoc! {"
        one
        TWO
        ˇTHREE
        FOUR
        five
    "});
    cx.run_until_parked();
    cx.update_editor(|editor, window, cx| {
        editor.toggle_staged_selected_diff_hunks(&Default::default(), window, cx);
    });
    cx.run_until_parked();
    cx.assert_index_text(Some(indoc! {"
        one
        TWO
        THREE
        FOUR
        five
    "}));
    cx.set_state(indoc! { "
        one
        TWO
        ˇTHREE-HUNDRED
        FOUR
        five
    "});
    cx.run_until_parked();
    cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let hunks = editor
            .diff_hunks_in_ranges(&[Anchor::min()..Anchor::max()], &snapshot.buffer_snapshot)
            .collect::<Vec<_>>();
        assert_eq!(hunks.len(), 1);
        assert_eq!(
            hunks[0].status(),
            DiffHunkStatus {
                kind: DiffHunkStatusKind::Modified,
                secondary: DiffHunkSecondaryStatus::OverlapsWithSecondaryHunk
            }
        );

        editor.toggle_staged_selected_diff_hunks(&Default::default(), window, cx);
    });
    cx.run_until_parked();
    cx.assert_index_text(Some(indoc! {"
        one
        TWO
        THREE-HUNDRED
        FOUR
        five
    "}));
}

#[gpui::test]
fn test_crease_insertion_and_rendering(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let editor = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple("aaaaaa\nbbbbbb\ncccccc\nddddddd\n", cx);
        build_editor(buffer, window, cx)
    });

    let render_args = Arc::new(Mutex::new(None));
    let snapshot = editor
        .update(cx, |editor, window, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let range =
                snapshot.anchor_before(Point::new(1, 0))..snapshot.anchor_after(Point::new(2, 6));

            struct RenderArgs {
                row: MultiBufferRow,
                folded: bool,
                callback: Arc<dyn Fn(bool, &mut Window, &mut App) + Send + Sync>,
            }

            let crease = Crease::inline(
                range,
                FoldPlaceholder::test(),
                {
                    let toggle_callback = render_args.clone();
                    move |row, folded, callback, _window, _cx| {
                        *toggle_callback.lock() = Some(RenderArgs {
                            row,
                            folded,
                            callback,
                        });
                        div()
                    }
                },
                |_row, _folded, _window, _cx| div(),
            );

            editor.insert_creases(Some(crease), cx);
            let snapshot = editor.snapshot(window, cx);
            let _div = snapshot.render_crease_toggle(
                MultiBufferRow(1),
                false,
                cx.entity().clone(),
                window,
                cx,
            );
            snapshot
        })
        .unwrap();

    let render_args = render_args.lock().take().unwrap();
    assert_eq!(render_args.row, MultiBufferRow(1));
    assert!(!render_args.folded);
    assert!(!snapshot.is_line_folded(MultiBufferRow(1)));

    cx.update_window(*editor, |_, window, cx| {
        (render_args.callback)(true, window, cx)
    })
    .unwrap();
    let snapshot = editor
        .update(cx, |editor, window, cx| editor.snapshot(window, cx))
        .unwrap();
    assert!(snapshot.is_line_folded(MultiBufferRow(1)));

    cx.update_window(*editor, |_, window, cx| {
        (render_args.callback)(false, window, cx)
    })
    .unwrap();
    let snapshot = editor
        .update(cx, |editor, window, cx| editor.snapshot(window, cx))
        .unwrap();
    assert!(!snapshot.is_line_folded(MultiBufferRow(1)));
}

#[gpui::test]
async fn test_input_text(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;

    cx.set_state(
        &r#"ˇone
        two

        three
        fourˇ
        five

        siˇx"#
            .unindent(),
    );

    cx.dispatch_action(HandleInput(String::new()));
    cx.assert_editor_state(
        &r#"ˇone
        two

        three
        fourˇ
        five

        siˇx"#
            .unindent(),
    );

    cx.dispatch_action(HandleInput("AAAA".to_string()));
    cx.assert_editor_state(
        &r#"AAAAˇone
        two

        three
        fourAAAAˇ
        five

        siAAAAˇx"#
            .unindent(),
    );
}

#[gpui::test]
async fn test_scroll_cursor_center_top_bottom(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorTestContext::new(cx).await;
    cx.set_state(
        r#"let foo = 1;
let foo = 2;
let foo = 3;
let fooˇ = 4;
let foo = 5;
let foo = 6;
let foo = 7;
let foo = 8;
let foo = 9;
let foo = 10;
let foo = 11;
let foo = 12;
let foo = 13;
let foo = 14;
let foo = 15;"#,
    );

    cx.update_editor(|e, window, cx| {
        assert_eq!(
            e.next_scroll_position,
            NextScrollCursorCenterTopBottom::Center,
            "Default next scroll direction is center",
        );

        e.scroll_cursor_center_top_bottom(&ScrollCursorCenterTopBottom, window, cx);
        assert_eq!(
            e.next_scroll_position,
            NextScrollCursorCenterTopBottom::Top,
            "After center, next scroll direction should be top",
        );

        e.scroll_cursor_center_top_bottom(&ScrollCursorCenterTopBottom, window, cx);
        assert_eq!(
            e.next_scroll_position,
            NextScrollCursorCenterTopBottom::Bottom,
            "After top, next scroll direction should be bottom",
        );

        e.scroll_cursor_center_top_bottom(&ScrollCursorCenterTopBottom, window, cx);
        assert_eq!(
            e.next_scroll_position,
            NextScrollCursorCenterTopBottom::Center,
            "After bottom, scrolling should start over",
        );

        e.scroll_cursor_center_top_bottom(&ScrollCursorCenterTopBottom, window, cx);
        assert_eq!(
            e.next_scroll_position,
            NextScrollCursorCenterTopBottom::Top,
            "Scrolling continues if retriggered fast enough"
        );
    });

    cx.executor()
        .advance_clock(SCROLL_CENTER_TOP_BOTTOM_DEBOUNCE_TIMEOUT + Duration::from_millis(200));
    cx.executor().run_until_parked();
    cx.update_editor(|e, _, _| {
        assert_eq!(
            e.next_scroll_position,
            NextScrollCursorCenterTopBottom::Center,
            "If scrolling is not triggered fast enough, it should reset"
        );
    });
}

#[gpui::test]
async fn test_goto_definition_with_find_all_references_fallback(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            definition_provider: Some(lsp::OneOf::Left(true)),
            references_provider: Some(lsp::OneOf::Left(true)),
            ..lsp::ServerCapabilities::default()
        },
        cx,
    )
    .await;

    let set_up_lsp_handlers = |empty_go_to_definition: bool, cx: &mut EditorLspTestContext| {
        let go_to_definition = cx
            .lsp
            .set_request_handler::<lsp::request::GotoDefinition, _, _>(
                move |params, _| async move {
                    if empty_go_to_definition {
                        Ok(None)
                    } else {
                        Ok(Some(lsp::GotoDefinitionResponse::Scalar(lsp::Location {
                            uri: params.text_document_position_params.text_document.uri,
                            range: lsp::Range::new(
                                lsp::Position::new(4, 3),
                                lsp::Position::new(4, 6),
                            ),
                        })))
                    }
                },
            );
        let references = cx
            .lsp
            .set_request_handler::<lsp::request::References, _, _>(move |params, _| async move {
                Ok(Some(vec![lsp::Location {
                    uri: params.text_document_position.text_document.uri,
                    range: lsp::Range::new(lsp::Position::new(0, 8), lsp::Position::new(0, 11)),
                }]))
            });
        (go_to_definition, references)
    };

    cx.set_state(
        &r#"fn one() {
            let mut a = ˇtwo();
        }

        fn two() {}"#
            .unindent(),
    );
    set_up_lsp_handlers(false, &mut cx);
    let navigated = cx
        .update_editor(|editor, window, cx| editor.go_to_definition(&GoToDefinition, window, cx))
        .await
        .expect("Failed to navigate to definition");
    assert_eq!(
        navigated,
        Navigated::Yes,
        "Should have navigated to definition from the GetDefinition response"
    );
    cx.assert_editor_state(
        &r#"fn one() {
            let mut a = two();
        }

        fn «twoˇ»() {}"#
            .unindent(),
    );

    let editors = cx.update_workspace(|workspace, _, cx| {
        workspace.items_of_type::<Editor>(cx).collect::<Vec<_>>()
    });
    cx.update_editor(|_, _, test_editor_cx| {
        assert_eq!(
            editors.len(),
            1,
            "Initially, only one, test, editor should be open in the workspace"
        );
        assert_eq!(
            test_editor_cx.entity(),
            editors.last().expect("Asserted len is 1").clone()
        );
    });

    set_up_lsp_handlers(true, &mut cx);
    let navigated = cx
        .update_editor(|editor, window, cx| editor.go_to_definition(&GoToDefinition, window, cx))
        .await
        .expect("Failed to navigate to lookup references");
    assert_eq!(
        navigated,
        Navigated::Yes,
        "Should have navigated to references as a fallback after empty GoToDefinition response"
    );
    // We should not change the selections in the existing file,
    // if opening another milti buffer with the references
    cx.assert_editor_state(
        &r#"fn one() {
            let mut a = two();
        }

        fn «twoˇ»() {}"#
            .unindent(),
    );
    let editors = cx.update_workspace(|workspace, _, cx| {
        workspace.items_of_type::<Editor>(cx).collect::<Vec<_>>()
    });
    cx.update_editor(|_, _, test_editor_cx| {
        assert_eq!(
            editors.len(),
            2,
            "After falling back to references search, we open a new editor with the results"
        );
        let references_fallback_text = editors
            .into_iter()
            .find(|new_editor| *new_editor != test_editor_cx.entity())
            .expect("Should have one non-test editor now")
            .read(test_editor_cx)
            .text(test_editor_cx);
        assert_eq!(
            references_fallback_text, "fn one() {\n    let mut a = two();\n}",
            "Should use the range from the references response and not the GoToDefinition one"
        );
    });
}

#[gpui::test]
async fn test_goto_definition_no_fallback(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    cx.update(|cx| {
        let mut editor_settings = EditorSettings::get_global(cx).clone();
        editor_settings.go_to_definition_fallback = GoToDefinitionFallback::None;
        EditorSettings::override_global(editor_settings, cx);
    });
    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            definition_provider: Some(lsp::OneOf::Left(true)),
            references_provider: Some(lsp::OneOf::Left(true)),
            ..lsp::ServerCapabilities::default()
        },
        cx,
    )
    .await;
    let original_state = r#"fn one() {
        let mut a = ˇtwo();
    }

    fn two() {}"#
        .unindent();
    cx.set_state(&original_state);

    let mut go_to_definition = cx
        .lsp
        .set_request_handler::<lsp::request::GotoDefinition, _, _>(
            move |_, _| async move { Ok(None) },
        );
    let _references = cx
        .lsp
        .set_request_handler::<lsp::request::References, _, _>(move |_, _| async move {
            panic!("Should not call for references with no go to definition fallback")
        });

    let navigated = cx
        .update_editor(|editor, window, cx| editor.go_to_definition(&GoToDefinition, window, cx))
        .await
        .expect("Failed to navigate to lookup references");
    go_to_definition
        .next()
        .await
        .expect("Should have called the go_to_definition handler");

    assert_eq!(
        navigated,
        Navigated::No,
        "Should have navigated to references as a fallback after empty GoToDefinition response"
    );
    cx.assert_editor_state(&original_state);
    let editors = cx.update_workspace(|workspace, _, cx| {
        workspace.items_of_type::<Editor>(cx).collect::<Vec<_>>()
    });
    cx.update_editor(|_, _, _| {
        assert_eq!(
            editors.len(),
            1,
            "After unsuccessful fallback, no other editor should have been opened"
        );
    });
}

#[gpui::test]
async fn test_find_enclosing_node_with_task(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let language = Arc::new(Language::new(
        LanguageConfig::default(),
        Some(tree_sitter_rust::LANGUAGE.into()),
    ));

    let text = r#"
        #[cfg(test)]
        mod tests() {
            #[test]
            fn runnable_1() {
                let a = 1;
            }

            #[test]
            fn runnable_2() {
                let a = 1;
                let b = 2;
            }
        }
    "#
    .unindent();

    let fs = FakeFs::new(cx.executor());
    fs.insert_file("/file.rs", Default::default()).await;

    let project = Project::test(fs, ["/a".as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace.deref(), cx);
    let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(language, cx));
    let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

    let editor = cx.new_window_entity(|window, cx| {
        Editor::new(
            EditorMode::full(),
            multi_buffer,
            Some(project.clone()),
            window,
            cx,
        )
    });

    editor.update_in(cx, |editor, window, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        editor.tasks.insert(
            (buffer.read(cx).remote_id(), 3),
            RunnableTasks {
                templates: vec![],
                offset: snapshot.anchor_before(43),
                column: 0,
                extra_variables: HashMap::default(),
                context_range: BufferOffset(43)..BufferOffset(85),
            },
        );
        editor.tasks.insert(
            (buffer.read(cx).remote_id(), 8),
            RunnableTasks {
                templates: vec![],
                offset: snapshot.anchor_before(86),
                column: 0,
                extra_variables: HashMap::default(),
                context_range: BufferOffset(86)..BufferOffset(191),
            },
        );

        // Test finding task when cursor is inside function body
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(4, 5)..Point::new(4, 5)])
        });
        let (_, row, _) = editor.find_enclosing_node_task(cx).unwrap();
        assert_eq!(row, 3, "Should find task for cursor inside runnable_1");

        // Test finding task when cursor is on function name
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(8, 4)..Point::new(8, 4)])
        });
        let (_, row, _) = editor.find_enclosing_node_task(cx).unwrap();
        assert_eq!(row, 8, "Should find task when cursor is on function name");
    });
}

#[gpui::test]
async fn test_folding_buffers(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let sample_text_1 = "aaaa\nbbbb\ncccc\ndddd\neeee\nffff\ngggg\nhhhh\niiii\njjjj".to_string();
    let sample_text_2 = "llll\nmmmm\nnnnn\noooo\npppp\nqqqq\nrrrr\nssss\ntttt\nuuuu".to_string();
    let sample_text_3 = "vvvv\nwwww\nxxxx\nyyyy\nzzzz\n1111\n2222\n3333\n4444\n5555".to_string();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/a"),
        json!({
            "first.rs": sample_text_1,
            "second.rs": sample_text_2,
            "third.rs": sample_text_3,
        }),
    )
    .await;
    let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace.deref(), cx);
    let worktree = project.update(cx, |project, cx| {
        let mut worktrees = project.worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 1);
        worktrees.pop().unwrap()
    });
    let worktree_id = worktree.update(cx, |worktree, _| worktree.id());

    let buffer_1 = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, "first.rs"), cx)
        })
        .await
        .unwrap();
    let buffer_2 = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, "second.rs"), cx)
        })
        .await
        .unwrap();
    let buffer_3 = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, "third.rs"), cx)
        })
        .await
        .unwrap();

    let multi_buffer = cx.new(|cx| {
        let mut multi_buffer = MultiBuffer::new(ReadWrite);
        multi_buffer.push_excerpts(
            buffer_1.clone(),
            [
                ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0)),
                ExcerptRange::new(Point::new(5, 0)..Point::new(7, 0)),
                ExcerptRange::new(Point::new(9, 0)..Point::new(10, 4)),
            ],
            cx,
        );
        multi_buffer.push_excerpts(
            buffer_2.clone(),
            [
                ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0)),
                ExcerptRange::new(Point::new(5, 0)..Point::new(7, 0)),
                ExcerptRange::new(Point::new(9, 0)..Point::new(10, 4)),
            ],
            cx,
        );
        multi_buffer.push_excerpts(
            buffer_3.clone(),
            [
                ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0)),
                ExcerptRange::new(Point::new(5, 0)..Point::new(7, 0)),
                ExcerptRange::new(Point::new(9, 0)..Point::new(10, 4)),
            ],
            cx,
        );
        multi_buffer
    });
    let multi_buffer_editor = cx.new_window_entity(|window, cx| {
        Editor::new(
            EditorMode::full(),
            multi_buffer.clone(),
            Some(project.clone()),
            window,
            cx,
        )
    });

    assert_eq!(
        multi_buffer_editor.update(cx, |editor, cx| editor.display_text(cx)),
        "\n\naaaa\nbbbb\ncccc\n\n\nffff\ngggg\n\n\njjjj\n\n\nllll\nmmmm\nnnnn\n\n\nqqqq\nrrrr\n\n\nuuuu\n\n\nvvvv\nwwww\nxxxx\n\n\n1111\n2222\n\n\n5555",
    );

    multi_buffer_editor.update(cx, |editor, cx| {
        editor.fold_buffer(buffer_1.read(cx).remote_id(), cx)
    });
    assert_eq!(
        multi_buffer_editor.update(cx, |editor, cx| editor.display_text(cx)),
        "\n\n\n\nllll\nmmmm\nnnnn\n\n\nqqqq\nrrrr\n\n\nuuuu\n\n\nvvvv\nwwww\nxxxx\n\n\n1111\n2222\n\n\n5555",
        "After folding the first buffer, its text should not be displayed"
    );

    multi_buffer_editor.update(cx, |editor, cx| {
        editor.fold_buffer(buffer_2.read(cx).remote_id(), cx)
    });
    assert_eq!(
        multi_buffer_editor.update(cx, |editor, cx| editor.display_text(cx)),
        "\n\n\n\n\n\nvvvv\nwwww\nxxxx\n\n\n1111\n2222\n\n\n5555",
        "After folding the second buffer, its text should not be displayed"
    );

    multi_buffer_editor.update(cx, |editor, cx| {
        editor.fold_buffer(buffer_3.read(cx).remote_id(), cx)
    });
    assert_eq!(
        multi_buffer_editor.update(cx, |editor, cx| editor.display_text(cx)),
        "\n\n\n\n\n",
        "After folding the third buffer, its text should not be displayed"
    );

    // Emulate selection inside the fold logic, that should work
    multi_buffer_editor.update_in(cx, |editor, window, cx| {
        editor
            .snapshot(window, cx)
            .next_line_boundary(Point::new(0, 4));
    });

    multi_buffer_editor.update(cx, |editor, cx| {
        editor.unfold_buffer(buffer_2.read(cx).remote_id(), cx)
    });
    assert_eq!(
        multi_buffer_editor.update(cx, |editor, cx| editor.display_text(cx)),
        "\n\n\n\nllll\nmmmm\nnnnn\n\n\nqqqq\nrrrr\n\n\nuuuu\n\n",
        "After unfolding the second buffer, its text should be displayed"
    );

    // Typing inside of buffer 1 causes that buffer to be unfolded.
    multi_buffer_editor.update_in(cx, |editor, window, cx| {
        assert_eq!(
            multi_buffer
                .read(cx)
                .snapshot(cx)
                .text_for_range(Point::new(1, 0)..Point::new(1, 4))
                .collect::<String>(),
            "bbbb"
        );
        editor.change_selections(None, window, cx, |selections| {
            selections.select_ranges(vec![Point::new(1, 0)..Point::new(1, 0)]);
        });
        editor.handle_input("B", window, cx);
    });

    assert_eq!(
        multi_buffer_editor.update(cx, |editor, cx| editor.display_text(cx)),
        "\n\nB\n\n\n\n\n\n\nllll\nmmmm\nnnnn\n\n\nqqqq\nrrrr\n\n\nuuuu\n\n",
        "After unfolding the first buffer, its and 2nd buffer's text should be displayed"
    );

    multi_buffer_editor.update(cx, |editor, cx| {
        editor.unfold_buffer(buffer_3.read(cx).remote_id(), cx)
    });
    assert_eq!(
        multi_buffer_editor.update(cx, |editor, cx| editor.display_text(cx)),
        "\n\nB\n\n\n\n\n\n\nllll\nmmmm\nnnnn\n\n\nqqqq\nrrrr\n\n\nuuuu\n\n\nvvvv\nwwww\nxxxx\n\n\n1111\n2222\n\n\n5555",
        "After unfolding the all buffers, all original text should be displayed"
    );
}

#[gpui::test]
async fn test_folding_buffers_with_one_excerpt(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let sample_text_1 = "1111\n2222\n3333".to_string();
    let sample_text_2 = "4444\n5555\n6666".to_string();
    let sample_text_3 = "7777\n8888\n9999".to_string();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/a"),
        json!({
            "first.rs": sample_text_1,
            "second.rs": sample_text_2,
            "third.rs": sample_text_3,
        }),
    )
    .await;
    let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace.deref(), cx);
    let worktree = project.update(cx, |project, cx| {
        let mut worktrees = project.worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 1);
        worktrees.pop().unwrap()
    });
    let worktree_id = worktree.update(cx, |worktree, _| worktree.id());

    let buffer_1 = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, "first.rs"), cx)
        })
        .await
        .unwrap();
    let buffer_2 = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, "second.rs"), cx)
        })
        .await
        .unwrap();
    let buffer_3 = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, "third.rs"), cx)
        })
        .await
        .unwrap();

    let multi_buffer = cx.new(|cx| {
        let mut multi_buffer = MultiBuffer::new(ReadWrite);
        multi_buffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0))],
            cx,
        );
        multi_buffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0))],
            cx,
        );
        multi_buffer.push_excerpts(
            buffer_3.clone(),
            [ExcerptRange::new(Point::new(0, 0)..Point::new(3, 0))],
            cx,
        );
        multi_buffer
    });

    let multi_buffer_editor = cx.new_window_entity(|window, cx| {
        Editor::new(
            EditorMode::full(),
            multi_buffer,
            Some(project.clone()),
            window,
            cx,
        )
    });

    let full_text = "\n\n1111\n2222\n3333\n\n\n4444\n5555\n6666\n\n\n7777\n8888\n9999";
    assert_eq!(
        multi_buffer_editor.update(cx, |editor, cx| editor.display_text(cx)),
        full_text,
    );

    multi_buffer_editor.update(cx, |editor, cx| {
        editor.fold_buffer(buffer_1.read(cx).remote_id(), cx)
    });
    assert_eq!(
        multi_buffer_editor.update(cx, |editor, cx| editor.display_text(cx)),
        "\n\n\n\n4444\n5555\n6666\n\n\n7777\n8888\n9999",
        "After folding the first buffer, its text should not be displayed"
    );

    multi_buffer_editor.update(cx, |editor, cx| {
        editor.fold_buffer(buffer_2.read(cx).remote_id(), cx)
    });

    assert_eq!(
        multi_buffer_editor.update(cx, |editor, cx| editor.display_text(cx)),
        "\n\n\n\n\n\n7777\n8888\n9999",
        "After folding the second buffer, its text should not be displayed"
    );

    multi_buffer_editor.update(cx, |editor, cx| {
        editor.fold_buffer(buffer_3.read(cx).remote_id(), cx)
    });
    assert_eq!(
        multi_buffer_editor.update(cx, |editor, cx| editor.display_text(cx)),
        "\n\n\n\n\n",
        "After folding the third buffer, its text should not be displayed"
    );

    multi_buffer_editor.update(cx, |editor, cx| {
        editor.unfold_buffer(buffer_2.read(cx).remote_id(), cx)
    });
    assert_eq!(
        multi_buffer_editor.update(cx, |editor, cx| editor.display_text(cx)),
        "\n\n\n\n4444\n5555\n6666\n\n",
        "After unfolding the second buffer, its text should be displayed"
    );

    multi_buffer_editor.update(cx, |editor, cx| {
        editor.unfold_buffer(buffer_1.read(cx).remote_id(), cx)
    });
    assert_eq!(
        multi_buffer_editor.update(cx, |editor, cx| editor.display_text(cx)),
        "\n\n1111\n2222\n3333\n\n\n4444\n5555\n6666\n\n",
        "After unfolding the first buffer, its text should be displayed"
    );

    multi_buffer_editor.update(cx, |editor, cx| {
        editor.unfold_buffer(buffer_3.read(cx).remote_id(), cx)
    });
    assert_eq!(
        multi_buffer_editor.update(cx, |editor, cx| editor.display_text(cx)),
        full_text,
        "After unfolding all buffers, all original text should be displayed"
    );
}

#[gpui::test]
async fn test_folding_buffer_when_multibuffer_has_only_one_excerpt(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let sample_text = "aaaa\nbbbb\ncccc\ndddd\neeee\nffff\ngggg\nhhhh\niiii\njjjj".to_string();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/a"),
        json!({
            "main.rs": sample_text,
        }),
    )
    .await;
    let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace.deref(), cx);
    let worktree = project.update(cx, |project, cx| {
        let mut worktrees = project.worktrees(cx).collect::<Vec<_>>();
        assert_eq!(worktrees.len(), 1);
        worktrees.pop().unwrap()
    });
    let worktree_id = worktree.update(cx, |worktree, _| worktree.id());

    let buffer_1 = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, "main.rs"), cx)
        })
        .await
        .unwrap();

    let multi_buffer = cx.new(|cx| {
        let mut multi_buffer = MultiBuffer::new(ReadWrite);
        multi_buffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange::new(
                Point::new(0, 0)
                    ..Point::new(
                        sample_text.chars().filter(|&c| c == '\n').count() as u32 + 1,
                        0,
                    ),
            )],
            cx,
        );
        multi_buffer
    });
    let multi_buffer_editor = cx.new_window_entity(|window, cx| {
        Editor::new(
            EditorMode::full(),
            multi_buffer,
            Some(project.clone()),
            window,
            cx,
        )
    });

    let selection_range = Point::new(1, 0)..Point::new(2, 0);
    multi_buffer_editor.update_in(cx, |editor, window, cx| {
        enum TestHighlight {}
        let multi_buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
        let highlight_range = selection_range.clone().to_anchors(&multi_buffer_snapshot);
        editor.highlight_text::<TestHighlight>(
            vec![highlight_range.clone()],
            HighlightStyle::color(Hsla::green()),
            cx,
        );
        editor.change_selections(None, window, cx, |s| s.select_ranges(Some(highlight_range)));
    });

    let full_text = format!("\n\n{sample_text}");
    assert_eq!(
        multi_buffer_editor.update(cx, |editor, cx| editor.display_text(cx)),
        full_text,
    );
}

#[gpui::test]
async fn test_multi_buffer_navigation_with_folded_buffers(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    cx.update(|cx| {
        let default_key_bindings = settings::KeymapFile::load_asset_allow_partial_failure(
            "keymaps/default-linux.json",
            cx,
        )
        .unwrap();
        cx.bind_keys(default_key_bindings);
    });

    let (editor, cx) = cx.add_window_view(|window, cx| {
        let multi_buffer = MultiBuffer::build_multi(
            [
                ("a0\nb0\nc0\nd0\ne0\n", vec![Point::row_range(0..2)]),
                ("a1\nb1\nc1\nd1\ne1\n", vec![Point::row_range(0..2)]),
                ("a2\nb2\nc2\nd2\ne2\n", vec![Point::row_range(0..2)]),
                ("a3\nb3\nc3\nd3\ne3\n", vec![Point::row_range(0..2)]),
            ],
            cx,
        );
        let mut editor = Editor::new(EditorMode::full(), multi_buffer.clone(), None, window, cx);

        let buffer_ids = multi_buffer.read(cx).excerpt_buffer_ids();
        // fold all but the second buffer, so that we test navigating between two
        // adjacent folded buffers, as well as folded buffers at the start and
        // end the multibuffer
        editor.fold_buffer(buffer_ids[0], cx);
        editor.fold_buffer(buffer_ids[2], cx);
        editor.fold_buffer(buffer_ids[3], cx);

        editor
    });
    cx.simulate_resize(size(px(1000.), px(1000.)));

    let mut cx = EditorTestContext::for_editor_in(editor.clone(), cx).await;
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        ˇ[FOLDED]
        [EXCERPT]
        a1
        b1
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    cx.simulate_keystroke("down");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        ˇa1
        b1
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    cx.simulate_keystroke("down");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        a1
        ˇb1
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    cx.simulate_keystroke("down");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        a1
        b1
        ˇ[EXCERPT]
        [FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    cx.simulate_keystroke("down");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        a1
        b1
        [EXCERPT]
        ˇ[FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    for _ in 0..5 {
        cx.simulate_keystroke("down");
        cx.assert_excerpts_with_selections(indoc! {"
            [EXCERPT]
            [FOLDED]
            [EXCERPT]
            a1
            b1
            [EXCERPT]
            [FOLDED]
            [EXCERPT]
            ˇ[FOLDED]
            "
        });
    }

    cx.simulate_keystroke("up");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        a1
        b1
        [EXCERPT]
        ˇ[FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    cx.simulate_keystroke("up");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        a1
        b1
        ˇ[EXCERPT]
        [FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    cx.simulate_keystroke("up");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        a1
        ˇb1
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    cx.simulate_keystroke("up");
    cx.assert_excerpts_with_selections(indoc! {"
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        ˇa1
        b1
        [EXCERPT]
        [FOLDED]
        [EXCERPT]
        [FOLDED]
        "
    });
    for _ in 0..5 {
        cx.simulate_keystroke("up");
        cx.assert_excerpts_with_selections(indoc! {"
            [EXCERPT]
            ˇ[FOLDED]
            [EXCERPT]
            a1
            b1
            [EXCERPT]
            [FOLDED]
            [EXCERPT]
            [FOLDED]
            "
        });
    }
}

#[gpui::test]
async fn test_inline_completion_text(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    // Simple insertion
    assert_highlighted_edits(
        "Hello, world!",
        vec![(Point::new(0, 6)..Point::new(0, 6), " beautiful".into())],
        true,
        cx,
        |highlighted_edits, cx| {
            assert_eq!(highlighted_edits.text, "Hello, beautiful world!");
            assert_eq!(highlighted_edits.highlights.len(), 1);
            assert_eq!(highlighted_edits.highlights[0].0, 6..16);
            assert_eq!(
                highlighted_edits.highlights[0].1.background_color,
                Some(cx.theme().status().created_background)
            );
        },
    )
    .await;

    // Replacement
    assert_highlighted_edits(
        "This is a test.",
        vec![(Point::new(0, 0)..Point::new(0, 4), "That".into())],
        false,
        cx,
        |highlighted_edits, cx| {
            assert_eq!(highlighted_edits.text, "That is a test.");
            assert_eq!(highlighted_edits.highlights.len(), 1);
            assert_eq!(highlighted_edits.highlights[0].0, 0..4);
            assert_eq!(
                highlighted_edits.highlights[0].1.background_color,
                Some(cx.theme().status().created_background)
            );
        },
    )
    .await;

    // Multiple edits
    assert_highlighted_edits(
        "Hello, world!",
        vec![
            (Point::new(0, 0)..Point::new(0, 5), "Greetings".into()),
            (Point::new(0, 12)..Point::new(0, 12), " and universe".into()),
        ],
        false,
        cx,
        |highlighted_edits, cx| {
            assert_eq!(highlighted_edits.text, "Greetings, world and universe!");
            assert_eq!(highlighted_edits.highlights.len(), 2);
            assert_eq!(highlighted_edits.highlights[0].0, 0..9);
            assert_eq!(highlighted_edits.highlights[1].0, 16..29);
            assert_eq!(
                highlighted_edits.highlights[0].1.background_color,
                Some(cx.theme().status().created_background)
            );
            assert_eq!(
                highlighted_edits.highlights[1].1.background_color,
                Some(cx.theme().status().created_background)
            );
        },
    )
    .await;

    // Multiple lines with edits
    assert_highlighted_edits(
        "First line\nSecond line\nThird line\nFourth line",
        vec![
            (Point::new(1, 7)..Point::new(1, 11), "modified".to_string()),
            (
                Point::new(2, 0)..Point::new(2, 10),
                "New third line".to_string(),
            ),
            (Point::new(3, 6)..Point::new(3, 6), " updated".to_string()),
        ],
        false,
        cx,
        |highlighted_edits, cx| {
            assert_eq!(
                highlighted_edits.text,
                "Second modified\nNew third line\nFourth updated line"
            );
            assert_eq!(highlighted_edits.highlights.len(), 3);
            assert_eq!(highlighted_edits.highlights[0].0, 7..15); // "modified"
            assert_eq!(highlighted_edits.highlights[1].0, 16..30); // "New third line"
            assert_eq!(highlighted_edits.highlights[2].0, 37..45); // " updated"
            for highlight in &highlighted_edits.highlights {
                assert_eq!(
                    highlight.1.background_color,
                    Some(cx.theme().status().created_background)
                );
            }
        },
    )
    .await;
}

#[gpui::test]
async fn test_inline_completion_text_with_deletions(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    // Deletion
    assert_highlighted_edits(
        "Hello, world!",
        vec![(Point::new(0, 5)..Point::new(0, 11), "".to_string())],
        true,
        cx,
        |highlighted_edits, cx| {
            assert_eq!(highlighted_edits.text, "Hello, world!");
            assert_eq!(highlighted_edits.highlights.len(), 1);
            assert_eq!(highlighted_edits.highlights[0].0, 5..11);
            assert_eq!(
                highlighted_edits.highlights[0].1.background_color,
                Some(cx.theme().status().deleted_background)
            );
        },
    )
    .await;

    // Insertion
    assert_highlighted_edits(
        "Hello, world!",
        vec![(Point::new(0, 6)..Point::new(0, 6), " digital".to_string())],
        true,
        cx,
        |highlighted_edits, cx| {
            assert_eq!(highlighted_edits.highlights.len(), 1);
            assert_eq!(highlighted_edits.highlights[0].0, 6..14);
            assert_eq!(
                highlighted_edits.highlights[0].1.background_color,
                Some(cx.theme().status().created_background)
            );
        },
    )
    .await;
}

async fn assert_highlighted_edits(
    text: &str,
    edits: Vec<(Range<Point>, String)>,
    include_deletions: bool,
    cx: &mut TestAppContext,
    assertion_fn: impl Fn(HighlightedText, &App),
) {
    let window = cx.add_window(|window, cx| {
        let buffer = MultiBuffer::build_simple(text, cx);
        Editor::new(EditorMode::full(), buffer, None, window, cx)
    });
    let cx = &mut VisualTestContext::from_window(*window, cx);

    let (buffer, snapshot) = window
        .update(cx, |editor, _window, cx| {
            (
                editor.buffer().clone(),
                editor.buffer().read(cx).snapshot(cx),
            )
        })
        .unwrap();

    let edits = edits
        .into_iter()
        .map(|(range, edit)| {
            (
                snapshot.anchor_after(range.start)..snapshot.anchor_before(range.end),
                edit,
            )
        })
        .collect::<Vec<_>>();

    let text_anchor_edits = edits
        .clone()
        .into_iter()
        .map(|(range, edit)| (range.start.text_anchor..range.end.text_anchor, edit))
        .collect::<Vec<_>>();

    let edit_preview = window
        .update(cx, |_, _window, cx| {
            buffer
                .read(cx)
                .as_singleton()
                .unwrap()
                .read(cx)
                .preview_edits(text_anchor_edits.into(), cx)
        })
        .unwrap()
        .await;

    cx.update(|_window, cx| {
        let highlighted_edits = inline_completion_edit_text(
            &snapshot.as_singleton().unwrap().2,
            &edits,
            &edit_preview,
            include_deletions,
            cx,
        );
        assertion_fn(highlighted_edits, cx)
    });
}

#[track_caller]
fn assert_breakpoint(
    breakpoints: &BTreeMap<Arc<Path>, Vec<SourceBreakpoint>>,
    path: &Arc<Path>,
    expected: Vec<(u32, Breakpoint)>,
) {
    if expected.len() == 0usize {
        assert!(!breakpoints.contains_key(path), "{}", path.display());
    } else {
        let mut breakpoint = breakpoints
            .get(path)
            .unwrap()
            .into_iter()
            .map(|breakpoint| {
                (
                    breakpoint.row,
                    Breakpoint {
                        message: breakpoint.message.clone(),
                        state: breakpoint.state,
                        condition: breakpoint.condition.clone(),
                        hit_condition: breakpoint.hit_condition.clone(),
                    },
                )
            })
            .collect::<Vec<_>>();

        breakpoint.sort_by_key(|(cached_position, _)| *cached_position);

        assert_eq!(expected, breakpoint);
    }
}

fn add_log_breakpoint_at_cursor(
    editor: &mut Editor,
    log_message: &str,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    let (anchor, bp) = editor
        .breakpoints_at_cursors(window, cx)
        .first()
        .and_then(|(anchor, bp)| {
            if let Some(bp) = bp {
                Some((*anchor, bp.clone()))
            } else {
                None
            }
        })
        .unwrap_or_else(|| {
            let cursor_position: Point = editor.selections.newest(cx).head();

            let breakpoint_position = editor
                .snapshot(window, cx)
                .display_snapshot
                .buffer_snapshot
                .anchor_before(Point::new(cursor_position.row, 0));

            (breakpoint_position, Breakpoint::new_log(&log_message))
        });

    editor.edit_breakpoint_at_anchor(
        anchor,
        bp,
        BreakpointEditAction::EditLogMessage(log_message.into()),
        cx,
    );
}

#[gpui::test]
async fn test_breakpoint_toggling(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let sample_text = "First line\nSecond line\nThird line\nFourth line".to_string();
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/a"),
        json!({
            "main.rs": sample_text,
        }),
    )
    .await;
    let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace.deref(), cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/a"),
        json!({
            "main.rs": sample_text,
        }),
    )
    .await;
    let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace.deref(), cx);
    let worktree_id = workspace
        .update(cx, |workspace, _window, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        })
        .unwrap();

    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, "main.rs"), cx)
        })
        .await
        .unwrap();

    let (editor, cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(buffer, cx),
            Some(project.clone()),
            window,
            cx,
        )
    });

    let project_path = editor.update(cx, |editor, cx| editor.project_path(cx).unwrap());
    let abs_path = project.read_with(cx, |project, cx| {
        project
            .absolute_path(&project_path, cx)
            .map(|path_buf| Arc::from(path_buf.to_owned()))
            .unwrap()
    });

    // assert we can add breakpoint on the first line
    editor.update_in(cx, |editor, window, cx| {
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
        editor.move_to_end(&MoveToEnd, window, cx);
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
    });

    let breakpoints = editor.update(cx, |editor, cx| {
        editor
            .breakpoint_store()
            .as_ref()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
    });

    assert_eq!(1, breakpoints.len());
    assert_breakpoint(
        &breakpoints,
        &abs_path,
        vec![
            (0, Breakpoint::new_standard()),
            (3, Breakpoint::new_standard()),
        ],
    );

    editor.update_in(cx, |editor, window, cx| {
        editor.move_to_beginning(&MoveToBeginning, window, cx);
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
    });

    let breakpoints = editor.update(cx, |editor, cx| {
        editor
            .breakpoint_store()
            .as_ref()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
    });

    assert_eq!(1, breakpoints.len());
    assert_breakpoint(
        &breakpoints,
        &abs_path,
        vec![(3, Breakpoint::new_standard())],
    );

    editor.update_in(cx, |editor, window, cx| {
        editor.move_to_end(&MoveToEnd, window, cx);
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
    });

    let breakpoints = editor.update(cx, |editor, cx| {
        editor
            .breakpoint_store()
            .as_ref()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
    });

    assert_eq!(0, breakpoints.len());
    assert_breakpoint(&breakpoints, &abs_path, vec![]);
}

#[gpui::test]
async fn test_log_breakpoint_editing(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let sample_text = "First line\nSecond line\nThird line\nFourth line".to_string();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/a"),
        json!({
            "main.rs": sample_text,
        }),
    )
    .await;
    let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    let worktree_id = workspace.update(cx, |workspace, cx| {
        workspace.project().update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        })
    });

    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, "main.rs"), cx)
        })
        .await
        .unwrap();

    let (editor, cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(buffer, cx),
            Some(project.clone()),
            window,
            cx,
        )
    });

    let project_path = editor.update(cx, |editor, cx| editor.project_path(cx).unwrap());
    let abs_path = project.read_with(cx, |project, cx| {
        project
            .absolute_path(&project_path, cx)
            .map(|path_buf| Arc::from(path_buf.to_owned()))
            .unwrap()
    });

    editor.update_in(cx, |editor, window, cx| {
        add_log_breakpoint_at_cursor(editor, "hello world", window, cx);
    });

    let breakpoints = editor.update(cx, |editor, cx| {
        editor
            .breakpoint_store()
            .as_ref()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
    });

    assert_breakpoint(
        &breakpoints,
        &abs_path,
        vec![(0, Breakpoint::new_log("hello world"))],
    );

    // Removing a log message from a log breakpoint should remove it
    editor.update_in(cx, |editor, window, cx| {
        add_log_breakpoint_at_cursor(editor, "", window, cx);
    });

    let breakpoints = editor.update(cx, |editor, cx| {
        editor
            .breakpoint_store()
            .as_ref()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
    });

    assert_breakpoint(&breakpoints, &abs_path, vec![]);

    editor.update_in(cx, |editor, window, cx| {
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
        editor.move_to_end(&MoveToEnd, window, cx);
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
        // Not adding a log message to a standard breakpoint shouldn't remove it
        add_log_breakpoint_at_cursor(editor, "", window, cx);
    });

    let breakpoints = editor.update(cx, |editor, cx| {
        editor
            .breakpoint_store()
            .as_ref()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
    });

    assert_breakpoint(
        &breakpoints,
        &abs_path,
        vec![
            (0, Breakpoint::new_standard()),
            (3, Breakpoint::new_standard()),
        ],
    );

    editor.update_in(cx, |editor, window, cx| {
        add_log_breakpoint_at_cursor(editor, "hello world", window, cx);
    });

    let breakpoints = editor.update(cx, |editor, cx| {
        editor
            .breakpoint_store()
            .as_ref()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
    });

    assert_breakpoint(
        &breakpoints,
        &abs_path,
        vec![
            (0, Breakpoint::new_standard()),
            (3, Breakpoint::new_log("hello world")),
        ],
    );

    editor.update_in(cx, |editor, window, cx| {
        add_log_breakpoint_at_cursor(editor, "hello Earth!!", window, cx);
    });

    let breakpoints = editor.update(cx, |editor, cx| {
        editor
            .breakpoint_store()
            .as_ref()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
    });

    assert_breakpoint(
        &breakpoints,
        &abs_path,
        vec![
            (0, Breakpoint::new_standard()),
            (3, Breakpoint::new_log("hello Earth!!")),
        ],
    );
}

/// This also tests that Editor::breakpoint_at_cursor_head is working properly
/// we had some issues where we wouldn't find a breakpoint at Point {row: 0, col: 0}
/// or when breakpoints were placed out of order. This tests for a regression too
#[gpui::test]
async fn test_breakpoint_enabling_and_disabling(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let sample_text = "First line\nSecond line\nThird line\nFourth line".to_string();
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/a"),
        json!({
            "main.rs": sample_text,
        }),
    )
    .await;
    let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace.deref(), cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/a"),
        json!({
            "main.rs": sample_text,
        }),
    )
    .await;
    let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace.deref(), cx);
    let worktree_id = workspace
        .update(cx, |workspace, _window, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        })
        .unwrap();

    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, "main.rs"), cx)
        })
        .await
        .unwrap();

    let (editor, cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(buffer, cx),
            Some(project.clone()),
            window,
            cx,
        )
    });

    let project_path = editor.update(cx, |editor, cx| editor.project_path(cx).unwrap());
    let abs_path = project.read_with(cx, |project, cx| {
        project
            .absolute_path(&project_path, cx)
            .map(|path_buf| Arc::from(path_buf.to_owned()))
            .unwrap()
    });

    // assert we can add breakpoint on the first line
    editor.update_in(cx, |editor, window, cx| {
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
        editor.move_to_end(&MoveToEnd, window, cx);
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
        editor.move_up(&MoveUp, window, cx);
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
    });

    let breakpoints = editor.update(cx, |editor, cx| {
        editor
            .breakpoint_store()
            .as_ref()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
    });

    assert_eq!(1, breakpoints.len());
    assert_breakpoint(
        &breakpoints,
        &abs_path,
        vec![
            (0, Breakpoint::new_standard()),
            (2, Breakpoint::new_standard()),
            (3, Breakpoint::new_standard()),
        ],
    );

    editor.update_in(cx, |editor, window, cx| {
        editor.move_to_beginning(&MoveToBeginning, window, cx);
        editor.disable_breakpoint(&actions::DisableBreakpoint, window, cx);
        editor.move_to_end(&MoveToEnd, window, cx);
        editor.disable_breakpoint(&actions::DisableBreakpoint, window, cx);
        // Disabling a breakpoint that doesn't exist should do nothing
        editor.move_up(&MoveUp, window, cx);
        editor.move_up(&MoveUp, window, cx);
        editor.disable_breakpoint(&actions::DisableBreakpoint, window, cx);
    });

    let breakpoints = editor.update(cx, |editor, cx| {
        editor
            .breakpoint_store()
            .as_ref()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
    });

    let disable_breakpoint = {
        let mut bp = Breakpoint::new_standard();
        bp.state = BreakpointState::Disabled;
        bp
    };

    assert_eq!(1, breakpoints.len());
    assert_breakpoint(
        &breakpoints,
        &abs_path,
        vec![
            (0, disable_breakpoint.clone()),
            (2, Breakpoint::new_standard()),
            (3, disable_breakpoint.clone()),
        ],
    );

    editor.update_in(cx, |editor, window, cx| {
        editor.move_to_beginning(&MoveToBeginning, window, cx);
        editor.enable_breakpoint(&actions::EnableBreakpoint, window, cx);
        editor.move_to_end(&MoveToEnd, window, cx);
        editor.enable_breakpoint(&actions::EnableBreakpoint, window, cx);
        editor.move_up(&MoveUp, window, cx);
        editor.disable_breakpoint(&actions::DisableBreakpoint, window, cx);
    });

    let breakpoints = editor.update(cx, |editor, cx| {
        editor
            .breakpoint_store()
            .as_ref()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
    });

    assert_eq!(1, breakpoints.len());
    assert_breakpoint(
        &breakpoints,
        &abs_path,
        vec![
            (0, Breakpoint::new_standard()),
            (2, disable_breakpoint),
            (3, Breakpoint::new_standard()),
        ],
    );
}

#[gpui::test]
async fn test_rename_with_duplicate_edits(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let capabilities = lsp::ServerCapabilities {
        rename_provider: Some(lsp::OneOf::Right(lsp::RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: Default::default(),
        })),
        ..Default::default()
    };
    let mut cx = EditorLspTestContext::new_rust(capabilities, cx).await;

    cx.set_state(indoc! {"
        struct Fˇoo {}
    "});

    cx.update_editor(|editor, _, cx| {
        let highlight_range = Point::new(0, 7)..Point::new(0, 10);
        let highlight_range = highlight_range.to_anchors(&editor.buffer().read(cx).snapshot(cx));
        editor.highlight_background::<DocumentHighlightRead>(
            &[highlight_range],
            |c| c.editor_document_highlight_read_background,
            cx,
        );
    });

    let mut prepare_rename_handler = cx
        .set_request_handler::<lsp::request::PrepareRenameRequest, _, _>(
            move |_, _, _| async move {
                Ok(Some(lsp::PrepareRenameResponse::Range(lsp::Range {
                    start: lsp::Position {
                        line: 0,
                        character: 7,
                    },
                    end: lsp::Position {
                        line: 0,
                        character: 10,
                    },
                })))
            },
        );
    let prepare_rename_task = cx
        .update_editor(|e, window, cx| e.rename(&Rename, window, cx))
        .expect("Prepare rename was not started");
    prepare_rename_handler.next().await.unwrap();
    prepare_rename_task.await.expect("Prepare rename failed");

    let mut rename_handler =
        cx.set_request_handler::<lsp::request::Rename, _, _>(move |url, _, _| async move {
            let edit = lsp::TextEdit {
                range: lsp::Range {
                    start: lsp::Position {
                        line: 0,
                        character: 7,
                    },
                    end: lsp::Position {
                        line: 0,
                        character: 10,
                    },
                },
                new_text: "FooRenamed".to_string(),
            };
            Ok(Some(lsp::WorkspaceEdit::new(
                // Specify the same edit twice
                std::collections::HashMap::from_iter(Some((url, vec![edit.clone(), edit]))),
            )))
        });
    let rename_task = cx
        .update_editor(|e, window, cx| e.confirm_rename(&ConfirmRename, window, cx))
        .expect("Confirm rename was not started");
    rename_handler.next().await.unwrap();
    rename_task.await.expect("Confirm rename failed");
    cx.run_until_parked();

    // Despite two edits, only one is actually applied as those are identical
    cx.assert_editor_state(indoc! {"
        struct FooRenamedˇ {}
    "});
}

#[gpui::test]
async fn test_rename_without_prepare(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    // These capabilities indicate that the server does not support prepare rename.
    let capabilities = lsp::ServerCapabilities {
        rename_provider: Some(lsp::OneOf::Left(true)),
        ..Default::default()
    };
    let mut cx = EditorLspTestContext::new_rust(capabilities, cx).await;

    cx.set_state(indoc! {"
        struct Fˇoo {}
    "});

    cx.update_editor(|editor, _window, cx| {
        let highlight_range = Point::new(0, 7)..Point::new(0, 10);
        let highlight_range = highlight_range.to_anchors(&editor.buffer().read(cx).snapshot(cx));
        editor.highlight_background::<DocumentHighlightRead>(
            &[highlight_range],
            |c| c.editor_document_highlight_read_background,
            cx,
        );
    });

    cx.update_editor(|e, window, cx| e.rename(&Rename, window, cx))
        .expect("Prepare rename was not started")
        .await
        .expect("Prepare rename failed");

    let mut rename_handler =
        cx.set_request_handler::<lsp::request::Rename, _, _>(move |url, _, _| async move {
            let edit = lsp::TextEdit {
                range: lsp::Range {
                    start: lsp::Position {
                        line: 0,
                        character: 7,
                    },
                    end: lsp::Position {
                        line: 0,
                        character: 10,
                    },
                },
                new_text: "FooRenamed".to_string(),
            };
            Ok(Some(lsp::WorkspaceEdit::new(
                std::collections::HashMap::from_iter(Some((url, vec![edit]))),
            )))
        });
    let rename_task = cx
        .update_editor(|e, window, cx| e.confirm_rename(&ConfirmRename, window, cx))
        .expect("Confirm rename was not started");
    rename_handler.next().await.unwrap();
    rename_task.await.expect("Confirm rename failed");
    cx.run_until_parked();

    // Correct range is renamed, as `surrounding_word` is used to find it.
    cx.assert_editor_state(indoc! {"
        struct FooRenamedˇ {}
    "});
}

#[gpui::test]
async fn test_tree_sitter_brackets_newline_insertion(cx: &mut TestAppContext) {
    init_test(cx, |_| {});
    let mut cx = EditorTestContext::new(cx).await;

    let language = Arc::new(
        Language::new(
            LanguageConfig::default(),
            Some(tree_sitter_html::LANGUAGE.into()),
        )
        .with_brackets_query(
            r#"
            ("<" @open "/>" @close)
            ("</" @open ">" @close)
            ("<" @open ">" @close)
            ("\"" @open "\"" @close)
            ((element (start_tag) @open (end_tag) @close) (#set! newline.only))
        "#,
        )
        .unwrap(),
    );
    cx.update_buffer(|buffer, cx| buffer.set_language(Some(language), cx));

    cx.set_state(indoc! {"
        <span>ˇ</span>
    "});
    cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
    cx.assert_editor_state(indoc! {"
        <span>
        ˇ
        </span>
    "});

    cx.set_state(indoc! {"
        <span><span></span>ˇ</span>
    "});
    cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
    cx.assert_editor_state(indoc! {"
        <span><span></span>
        ˇ</span>
    "});

    cx.set_state(indoc! {"
        <span>ˇ
        </span>
    "});
    cx.update_editor(|e, window, cx| e.newline(&Newline, window, cx));
    cx.assert_editor_state(indoc! {"
        <span>
        ˇ
        </span>
    "});
}

#[gpui::test(iterations = 10)]
async fn test_apply_code_lens_actions_with_commands(cx: &mut gpui::TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/dir"),
        json!({
            "a.ts": "a",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace.deref(), cx);

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(Arc::new(Language::new(
        LanguageConfig {
            name: "TypeScript".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["ts".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
    )));
    let mut fake_language_servers = language_registry.register_fake_lsp(
        "TypeScript",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                code_lens_provider: Some(lsp::CodeLensOptions {
                    resolve_provider: Some(true),
                }),
                execute_command_provider: Some(lsp::ExecuteCommandOptions {
                    commands: vec!["_the/command".to_string()],
                    ..lsp::ExecuteCommandOptions::default()
                }),
                ..lsp::ServerCapabilities::default()
            },
            ..FakeLspAdapter::default()
        },
    );

    let (buffer, _handle) = project
        .update(cx, |p, cx| {
            p.open_local_buffer_with_lsp(path!("/dir/a.ts"), cx)
        })
        .await
        .unwrap();
    cx.executor().run_until_parked();

    let fake_server = fake_language_servers.next().await.unwrap();

    let buffer_snapshot = buffer.update(cx, |buffer, _| buffer.snapshot());
    let anchor = buffer_snapshot.anchor_at(0, text::Bias::Left);
    drop(buffer_snapshot);
    let actions = cx
        .update_window(*workspace, |_, window, cx| {
            project.code_actions(&buffer, anchor..anchor, window, cx)
        })
        .unwrap();

    fake_server
        .set_request_handler::<lsp::request::CodeLensRequest, _, _>(|_, _| async move {
            Ok(Some(vec![
                lsp::CodeLens {
                    range: lsp::Range::default(),
                    command: Some(lsp::Command {
                        title: "Code lens command".to_owned(),
                        command: "_the/command".to_owned(),
                        arguments: None,
                    }),
                    data: None,
                },
                lsp::CodeLens {
                    range: lsp::Range::default(),
                    command: Some(lsp::Command {
                        title: "Command not in capabilities".to_owned(),
                        command: "not in capabilities".to_owned(),
                        arguments: None,
                    }),
                    data: None,
                },
                lsp::CodeLens {
                    range: lsp::Range {
                        start: lsp::Position {
                            line: 1,
                            character: 1,
                        },
                        end: lsp::Position {
                            line: 1,
                            character: 1,
                        },
                    },
                    command: Some(lsp::Command {
                        title: "Command not in range".to_owned(),
                        command: "_the/command".to_owned(),
                        arguments: None,
                    }),
                    data: None,
                },
            ]))
        })
        .next()
        .await;

    let actions = actions.await.unwrap();
    assert_eq!(
        actions.len(),
        1,
        "Should have only one valid action for the 0..0 range"
    );
    let action = actions[0].clone();
    let apply = project.update(cx, |project, cx| {
        project.apply_code_action(buffer.clone(), action, true, cx)
    });

    // Resolving the code action does not populate its edits. In absence of
    // edits, we must execute the given command.
    fake_server.set_request_handler::<lsp::request::CodeLensResolve, _, _>(
        |mut lens, _| async move {
            let lens_command = lens.command.as_mut().expect("should have a command");
            assert_eq!(lens_command.title, "Code lens command");
            lens_command.arguments = Some(vec![json!("the-argument")]);
            Ok(lens)
        },
    );

    // While executing the command, the language server sends the editor
    // a `workspaceEdit` request.
    fake_server
        .set_request_handler::<lsp::request::ExecuteCommand, _, _>({
            let fake = fake_server.clone();
            move |params, _| {
                assert_eq!(params.command, "_the/command");
                let fake = fake.clone();
                async move {
                    fake.server
                        .request::<lsp::request::ApplyWorkspaceEdit>(
                            lsp::ApplyWorkspaceEditParams {
                                label: None,
                                edit: lsp::WorkspaceEdit {
                                    changes: Some(
                                        [(
                                            lsp::Url::from_file_path(path!("/dir/a.ts")).unwrap(),
                                            vec![lsp::TextEdit {
                                                range: lsp::Range::new(
                                                    lsp::Position::new(0, 0),
                                                    lsp::Position::new(0, 0),
                                                ),
                                                new_text: "X".into(),
                                            }],
                                        )]
                                        .into_iter()
                                        .collect(),
                                    ),
                                    ..Default::default()
                                },
                            },
                        )
                        .await
                        .into_response()
                        .unwrap();
                    Ok(Some(json!(null)))
                }
            }
        })
        .next()
        .await;

    // Applying the code lens command returns a project transaction containing the edits
    // sent by the language server in its `workspaceEdit` request.
    let transaction = apply.await.unwrap();
    assert!(transaction.0.contains_key(&buffer));
    buffer.update(cx, |buffer, cx| {
        assert_eq!(buffer.text(), "Xa");
        buffer.undo(cx);
        assert_eq!(buffer.text(), "a");
    });
}

#[gpui::test]
async fn test_editor_restore_data_different_in_panes(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    let main_text = r#"fn main() {
println!("1");
println!("2");
println!("3");
println!("4");
println!("5");
}"#;
    let lib_text = "mod foo {}";
    fs.insert_tree(
        path!("/a"),
        json!({
            "lib.rs": lib_text,
            "main.rs": main_text,
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let worktree_id = workspace.update(cx, |workspace, cx| {
        workspace.project().update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        })
    });

    let expected_ranges = vec![
        Point::new(0, 0)..Point::new(0, 0),
        Point::new(1, 0)..Point::new(1, 1),
        Point::new(2, 0)..Point::new(2, 2),
        Point::new(3, 0)..Point::new(3, 3),
    ];

    let pane_1 = workspace.update(cx, |workspace, _| workspace.active_pane().clone());
    let editor_1 = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path(
                (worktree_id, "main.rs"),
                Some(pane_1.downgrade()),
                true,
                window,
                cx,
            )
        })
        .unwrap()
        .await
        .downcast::<Editor>()
        .unwrap();
    pane_1.update(cx, |pane, cx| {
        let open_editor = pane.active_item().unwrap().downcast::<Editor>().unwrap();
        open_editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.display_text(cx),
                main_text,
                "Original main.rs text on initial open",
            );
            assert_eq!(
                editor
                    .selections
                    .all::<Point>(cx)
                    .into_iter()
                    .map(|s| s.range())
                    .collect::<Vec<_>>(),
                vec![Point::zero()..Point::zero()],
                "Default selections on initial open",
            );
        })
    });
    editor_1.update_in(cx, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges(expected_ranges.clone());
        });
    });

    let pane_2 = workspace.update_in(cx, |workspace, window, cx| {
        workspace.split_pane(pane_1.clone(), SplitDirection::Right, window, cx)
    });
    let editor_2 = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path(
                (worktree_id, "main.rs"),
                Some(pane_2.downgrade()),
                true,
                window,
                cx,
            )
        })
        .unwrap()
        .await
        .downcast::<Editor>()
        .unwrap();
    pane_2.update(cx, |pane, cx| {
        let open_editor = pane.active_item().unwrap().downcast::<Editor>().unwrap();
        open_editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.display_text(cx),
                main_text,
                "Original main.rs text on initial open in another panel",
            );
            assert_eq!(
                editor
                    .selections
                    .all::<Point>(cx)
                    .into_iter()
                    .map(|s| s.range())
                    .collect::<Vec<_>>(),
                vec![Point::zero()..Point::zero()],
                "Default selections on initial open in another panel",
            );
        })
    });

    editor_2.update_in(cx, |editor, window, cx| {
        editor.fold_ranges(expected_ranges.clone(), false, window, cx);
    });

    let _other_editor_1 = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path(
                (worktree_id, "lib.rs"),
                Some(pane_1.downgrade()),
                true,
                window,
                cx,
            )
        })
        .unwrap()
        .await
        .downcast::<Editor>()
        .unwrap();
    pane_1
        .update_in(cx, |pane, window, cx| {
            pane.close_inactive_items(&CloseInactiveItems::default(), window, cx)
                .unwrap()
        })
        .await
        .unwrap();
    drop(editor_1);
    pane_1.update(cx, |pane, cx| {
        pane.active_item()
            .unwrap()
            .downcast::<Editor>()
            .unwrap()
            .update(cx, |editor, cx| {
                assert_eq!(
                    editor.display_text(cx),
                    lib_text,
                    "Other file should be open and active",
                );
            });
        assert_eq!(pane.items().count(), 1, "No other editors should be open");
    });

    let _other_editor_2 = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path(
                (worktree_id, "lib.rs"),
                Some(pane_2.downgrade()),
                true,
                window,
                cx,
            )
        })
        .unwrap()
        .await
        .downcast::<Editor>()
        .unwrap();
    pane_2
        .update_in(cx, |pane, window, cx| {
            pane.close_inactive_items(&CloseInactiveItems::default(), window, cx)
                .unwrap()
        })
        .await
        .unwrap();
    drop(editor_2);
    pane_2.update(cx, |pane, cx| {
        let open_editor = pane.active_item().unwrap().downcast::<Editor>().unwrap();
        open_editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.display_text(cx),
                lib_text,
                "Other file should be open and active in another panel too",
            );
        });
        assert_eq!(
            pane.items().count(),
            1,
            "No other editors should be open in another pane",
        );
    });

    let _editor_1_reopened = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path(
                (worktree_id, "main.rs"),
                Some(pane_1.downgrade()),
                true,
                window,
                cx,
            )
        })
        .unwrap()
        .await
        .downcast::<Editor>()
        .unwrap();
    let _editor_2_reopened = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path(
                (worktree_id, "main.rs"),
                Some(pane_2.downgrade()),
                true,
                window,
                cx,
            )
        })
        .unwrap()
        .await
        .downcast::<Editor>()
        .unwrap();
    pane_1.update(cx, |pane, cx| {
        let open_editor = pane.active_item().unwrap().downcast::<Editor>().unwrap();
        open_editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.display_text(cx),
                main_text,
                "Previous editor in the 1st panel had no extra text manipulations and should get none on reopen",
            );
            assert_eq!(
                editor
                    .selections
                    .all::<Point>(cx)
                    .into_iter()
                    .map(|s| s.range())
                    .collect::<Vec<_>>(),
                expected_ranges,
                "Previous editor in the 1st panel had selections and should get them restored on reopen",
            );
        })
    });
    pane_2.update(cx, |pane, cx| {
        let open_editor = pane.active_item().unwrap().downcast::<Editor>().unwrap();
        open_editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.display_text(cx),
                r#"fn main() {
⋯rintln!("1");
⋯intln!("2");
⋯ntln!("3");
println!("4");
println!("5");
}"#,
                "Previous editor in the 2nd pane had folds and should restore those on reopen in the same pane",
            );
            assert_eq!(
                editor
                    .selections
                    .all::<Point>(cx)
                    .into_iter()
                    .map(|s| s.range())
                    .collect::<Vec<_>>(),
                vec![Point::zero()..Point::zero()],
                "Previous editor in the 2nd pane had no selections changed hence should restore none",
            );
        })
    });
}

#[gpui::test]
async fn test_editor_does_not_restore_data_when_turned_off(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    let main_text = r#"fn main() {
println!("1");
println!("2");
println!("3");
println!("4");
println!("5");
}"#;
    let lib_text = "mod foo {}";
    fs.insert_tree(
        path!("/a"),
        json!({
            "lib.rs": lib_text,
            "main.rs": main_text,
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let worktree_id = workspace.update(cx, |workspace, cx| {
        workspace.project().update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        })
    });

    let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());
    let editor = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path(
                (worktree_id, "main.rs"),
                Some(pane.downgrade()),
                true,
                window,
                cx,
            )
        })
        .unwrap()
        .await
        .downcast::<Editor>()
        .unwrap();
    pane.update(cx, |pane, cx| {
        let open_editor = pane.active_item().unwrap().downcast::<Editor>().unwrap();
        open_editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.display_text(cx),
                main_text,
                "Original main.rs text on initial open",
            );
        })
    });
    editor.update_in(cx, |editor, window, cx| {
        editor.fold_ranges(vec![Point::new(0, 0)..Point::new(0, 0)], false, window, cx);
    });

    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings::<WorkspaceSettings>(cx, |s| {
            s.restore_on_file_reopen = Some(false);
        });
    });
    editor.update_in(cx, |editor, window, cx| {
        editor.fold_ranges(
            vec![
                Point::new(1, 0)..Point::new(1, 1),
                Point::new(2, 0)..Point::new(2, 2),
                Point::new(3, 0)..Point::new(3, 3),
            ],
            false,
            window,
            cx,
        );
    });
    pane.update_in(cx, |pane, window, cx| {
        pane.close_all_items(&CloseAllItems::default(), window, cx)
            .unwrap()
    })
    .await
    .unwrap();
    pane.update(cx, |pane, _| {
        assert!(pane.active_item().is_none());
    });
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.update_user_settings::<WorkspaceSettings>(cx, |s| {
            s.restore_on_file_reopen = Some(true);
        });
    });

    let _editor_reopened = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path(
                (worktree_id, "main.rs"),
                Some(pane.downgrade()),
                true,
                window,
                cx,
            )
        })
        .unwrap()
        .await
        .downcast::<Editor>()
        .unwrap();
    pane.update(cx, |pane, cx| {
        let open_editor = pane.active_item().unwrap().downcast::<Editor>().unwrap();
        open_editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.display_text(cx),
                main_text,
                "No folds: even after enabling the restoration, previous editor's data should not be saved to be used for the restoration"
            );
        })
    });
}

#[gpui::test]
async fn test_hide_mouse_context_menu_on_modal_opened(cx: &mut TestAppContext) {
    struct EmptyModalView {
        focus_handle: gpui::FocusHandle,
    }
    impl EventEmitter<DismissEvent> for EmptyModalView {}
    impl Render for EmptyModalView {
        fn render(&mut self, _: &mut Window, _: &mut Context<'_, Self>) -> impl IntoElement {
            div()
        }
    }
    impl Focusable for EmptyModalView {
        fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
            self.focus_handle.clone()
        }
    }
    impl workspace::ModalView for EmptyModalView {}
    fn new_empty_modal_view(cx: &App) -> EmptyModalView {
        EmptyModalView {
            focus_handle: cx.focus_handle(),
        }
    }

    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, [], cx).await;
    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let buffer = cx.update(|cx| MultiBuffer::build_simple("hello world!", cx));
    let cx = &mut VisualTestContext::from_window(*workspace.deref(), cx);
    let editor = cx.new_window_entity(|window, cx| {
        Editor::new(
            EditorMode::full(),
            buffer,
            Some(project.clone()),
            window,
            cx,
        )
    });
    workspace
        .update(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, window, cx);
        })
        .unwrap();
    editor.update_in(cx, |editor, window, cx| {
        editor.open_context_menu(&OpenContextMenu, window, cx);
        assert!(editor.mouse_context_menu.is_some());
    });
    workspace
        .update(cx, |workspace, window, cx| {
            workspace.toggle_modal(window, cx, |_, cx| new_empty_modal_view(cx));
        })
        .unwrap();
    cx.read(|cx| {
        assert!(editor.read(cx).mouse_context_menu.is_none());
    });
}

#[gpui::test]
async fn test_html_linked_edits_on_completion(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    fs.insert_file(path!("/file.html"), Default::default())
        .await;

    let project = Project::test(fs, [path!("/").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    let html_language = Arc::new(Language::new(
        LanguageConfig {
            name: "HTML".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["html".to_string()],
                ..LanguageMatcher::default()
            },
            brackets: BracketPairConfig {
                pairs: vec![BracketPair {
                    start: "<".into(),
                    end: ">".into(),
                    close: true,
                    ..Default::default()
                }],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_html::LANGUAGE.into()),
    ));
    language_registry.add(html_language);
    let mut fake_servers = language_registry.register_fake_lsp(
        "HTML",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                completion_provider: Some(lsp::CompletionOptions {
                    resolve_provider: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let workspace = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let worktree_id = workspace
        .update(cx, |workspace, _window, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        })
        .unwrap();
    project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/file.html"), cx)
        })
        .await
        .unwrap();
    let editor = workspace
        .update(cx, |workspace, window, cx| {
            workspace.open_path((worktree_id, "file.html"), None, true, window, cx)
        })
        .unwrap()
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let fake_server = fake_servers.next().await.unwrap();
    editor.update_in(cx, |editor, window, cx| {
        editor.set_text("<ad></ad>", window, cx);
        editor.change_selections(None, window, cx, |selections| {
            selections.select_ranges([Point::new(0, 3)..Point::new(0, 3)]);
        });
        let Some((buffer, _)) = editor
            .buffer
            .read(cx)
            .text_anchor_for_position(editor.selections.newest_anchor().start, cx)
        else {
            panic!("Failed to get buffer for selection position");
        };
        let buffer = buffer.read(cx);
        let buffer_id = buffer.remote_id();
        let opening_range =
            buffer.anchor_before(Point::new(0, 1))..buffer.anchor_after(Point::new(0, 3));
        let closing_range =
            buffer.anchor_before(Point::new(0, 6))..buffer.anchor_after(Point::new(0, 8));
        let mut linked_ranges = HashMap::default();
        linked_ranges.insert(
            buffer_id,
            vec![(opening_range.clone(), vec![closing_range.clone()])],
        );
        editor.linked_edit_ranges = LinkedEditingRanges(linked_ranges);
    });
    let mut completion_handle =
        fake_server.set_request_handler::<lsp::request::Completion, _, _>(move |_, _| async move {
            Ok(Some(lsp::CompletionResponse::Array(vec![
                lsp::CompletionItem {
                    label: "head".to_string(),
                    text_edit: Some(lsp::CompletionTextEdit::InsertAndReplace(
                        lsp::InsertReplaceEdit {
                            new_text: "head".to_string(),
                            insert: lsp::Range::new(
                                lsp::Position::new(0, 1),
                                lsp::Position::new(0, 3),
                            ),
                            replace: lsp::Range::new(
                                lsp::Position::new(0, 1),
                                lsp::Position::new(0, 3),
                            ),
                        },
                    )),
                    ..Default::default()
                },
            ])))
        });
    editor.update_in(cx, |editor, window, cx| {
        editor.show_completions(&ShowCompletions { trigger: None }, window, cx);
    });
    cx.run_until_parked();
    completion_handle.next().await.unwrap();
    editor.update(cx, |editor, _| {
        assert!(
            editor.context_menu_visible(),
            "Completion menu should be visible"
        );
    });
    editor.update_in(cx, |editor, window, cx| {
        editor.confirm_completion(&ConfirmCompletion::default(), window, cx)
    });
    cx.executor().run_until_parked();
    editor.update(cx, |editor, cx| {
        assert_eq!(editor.text(cx), "<head></head>");
    });
}

#[gpui::test]
async fn test_invisible_worktree_servers(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "a": {
                "main.rs": "fn main() {}",
            },
            "foo": {
                "bar": {
                    "external_file.rs": "pub mod external {}",
                }
            }
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/root/a").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let _fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            ..FakeLspAdapter::default()
        },
    );
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let worktree_id = workspace.update(cx, |workspace, cx| {
        workspace.project().update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        })
    });

    let assert_language_servers_count =
        |expected: usize, context: &str, cx: &mut VisualTestContext| {
            project.update(cx, |project, cx| {
                let current = project
                    .lsp_store()
                    .read(cx)
                    .as_local()
                    .unwrap()
                    .language_servers
                    .len();
                assert_eq!(expected, current, "{context}");
            });
        };

    assert_language_servers_count(
        0,
        "No servers should be running before any file is open",
        cx,
    );
    let pane = workspace.update(cx, |workspace, _| workspace.active_pane().clone());
    let main_editor = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path(
                (worktree_id, "main.rs"),
                Some(pane.downgrade()),
                true,
                window,
                cx,
            )
        })
        .unwrap()
        .await
        .downcast::<Editor>()
        .unwrap();
    pane.update(cx, |pane, cx| {
        let open_editor = pane.active_item().unwrap().downcast::<Editor>().unwrap();
        open_editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.display_text(cx),
                "fn main() {}",
                "Original main.rs text on initial open",
            );
        });
        assert_eq!(open_editor, main_editor);
    });
    assert_language_servers_count(1, "First *.rs file starts a language server", cx);

    let external_editor = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_abs_path(
                PathBuf::from("/root/foo/bar/external_file.rs"),
                OpenOptions::default(),
                window,
                cx,
            )
        })
        .await
        .expect("opening external file")
        .downcast::<Editor>()
        .expect("downcasted external file's open element to editor");
    pane.update(cx, |pane, cx| {
        let open_editor = pane.active_item().unwrap().downcast::<Editor>().unwrap();
        open_editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.display_text(cx),
                "pub mod external {}",
                "External file is open now",
            );
        });
        assert_eq!(open_editor, external_editor);
    });
    assert_language_servers_count(
        1,
        "Second, external, *.rs file should join the existing server",
        cx,
    );

    pane.update_in(cx, |pane, window, cx| {
        pane.close_active_item(&CloseActiveItem::default(), window, cx)
    })
    .unwrap()
    .await
    .unwrap();
    pane.update_in(cx, |pane, window, cx| {
        pane.navigate_backward(window, cx);
    });
    cx.run_until_parked();
    pane.update(cx, |pane, cx| {
        let open_editor = pane.active_item().unwrap().downcast::<Editor>().unwrap();
        open_editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.display_text(cx),
                "pub mod external {}",
                "External file is open now",
            );
        });
    });
    assert_language_servers_count(
        1,
        "After closing and reopening (with navigate back) of an external file, no extra language servers should appear",
        cx,
    );

    cx.update(|_, cx| {
        workspace::reload(&workspace::Reload::default(), cx);
    });
    assert_language_servers_count(
        1,
        "After reloading the worktree with local and external files opened, only one project should be started",
        cx,
    );
}

fn empty_range(row: usize, column: usize) -> Range<DisplayPoint> {
    let point = DisplayPoint::new(DisplayRow(row as u32), column as u32);
    point..point
}

fn assert_selection_ranges(marked_text: &str, editor: &mut Editor, cx: &mut Context<Editor>) {
    let (text, ranges) = marked_text_ranges(marked_text, true);
    assert_eq!(editor.text(cx), text);
    assert_eq!(
        editor.selections.ranges(cx),
        ranges,
        "Assert selections are {}",
        marked_text
    );
}

pub fn handle_signature_help_request(
    cx: &mut EditorLspTestContext,
    mocked_response: lsp::SignatureHelp,
) -> impl Future<Output = ()> + use<> {
    let mut request =
        cx.set_request_handler::<lsp::request::SignatureHelpRequest, _, _>(move |_, _, _| {
            let mocked_response = mocked_response.clone();
            async move { Ok(Some(mocked_response)) }
        });

    async move {
        request.next().await;
    }
}

/// Handle completion request passing a marked string specifying where the completion
/// should be triggered from using '|' character, what range should be replaced, and what completions
/// should be returned using '<' and '>' to delimit the range.
///
/// Also see `handle_completion_request_with_insert_and_replace`.
#[track_caller]
pub fn handle_completion_request(
    cx: &mut EditorLspTestContext,
    marked_string: &str,
    completions: Vec<&'static str>,
    counter: Arc<AtomicUsize>,
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

    let mut request =
        cx.set_request_handler::<lsp::request::Completion, _, _>(move |url, params, _| {
            let completions = completions.clone();
            counter.fetch_add(1, atomic::Ordering::Release);
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

/// Similar to `handle_completion_request`, but a [`CompletionTextEdit::InsertAndReplace`] will be
/// given instead, which also contains an `insert` range.
///
/// This function uses the cursor position to mimic what Rust-Analyzer provides as the `insert` range,
/// that is, `replace_range.start..cursor_pos`.
pub fn handle_completion_request_with_insert_and_replace(
    cx: &mut EditorLspTestContext,
    marked_string: &str,
    completions: Vec<&'static str>,
    counter: Arc<AtomicUsize>,
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

    let mut request =
        cx.set_request_handler::<lsp::request::Completion, _, _>(move |url, params, _| {
            let completions = completions.clone();
            counter.fetch_add(1, atomic::Ordering::Release);
            async move {
                assert_eq!(params.text_document_position.text_document.uri, url.clone());
                assert_eq!(
                    params.text_document_position.position, complete_from_position,
                    "marker `|` position doesn't match",
                );
                Ok(Some(lsp::CompletionResponse::Array(
                    completions
                        .iter()
                        .map(|completion_text| lsp::CompletionItem {
                            label: completion_text.to_string(),
                            text_edit: Some(lsp::CompletionTextEdit::InsertAndReplace(
                                lsp::InsertReplaceEdit {
                                    insert: lsp::Range {
                                        start: replace_range.start,
                                        end: complete_from_position,
                                    },
                                    replace: replace_range,
                                    new_text: completion_text.to_string(),
                                },
                            )),
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
        cx.set_request_handler::<lsp::request::ResolveCompletionItem, _, _>(move |_, _, _| {
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

pub(crate) fn update_test_language_settings(
    cx: &mut TestAppContext,
    f: impl Fn(&mut AllLanguageSettingsContent),
) {
    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings::<AllLanguageSettings>(cx, f);
        });
    });
}

pub(crate) fn update_test_project_settings(
    cx: &mut TestAppContext,
    f: impl Fn(&mut ProjectSettings),
) {
    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings::<ProjectSettings>(cx, f);
        });
    });
}

pub(crate) fn init_test(cx: &mut TestAppContext, f: fn(&mut AllLanguageSettingsContent)) {
    cx.update(|cx| {
        assets::Assets.load_test_fonts(cx);
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        theme::init(theme::LoadThemes::JustBase, cx);
        release_channel::init(SemanticVersion::default(), cx);
        client::init_settings(cx);
        language::init(cx);
        Project::init_settings(cx);
        workspace::init_settings(cx);
        crate::init(cx);
    });

    update_test_language_settings(cx, f);
}

#[track_caller]
fn assert_hunk_revert(
    not_reverted_text_with_selections: &str,
    expected_hunk_statuses_before: Vec<DiffHunkStatusKind>,
    expected_reverted_text_with_selections: &str,
    base_text: &str,
    cx: &mut EditorLspTestContext,
) {
    cx.set_state(not_reverted_text_with_selections);
    cx.set_head_text(base_text);
    cx.executor().run_until_parked();

    let actual_hunk_statuses_before = cx.update_editor(|editor, window, cx| {
        let snapshot = editor.snapshot(window, cx);
        let reverted_hunk_statuses = snapshot
            .buffer_snapshot
            .diff_hunks_in_range(0..snapshot.buffer_snapshot.len())
            .map(|hunk| hunk.status().kind)
            .collect::<Vec<_>>();

        editor.git_restore(&Default::default(), window, cx);
        reverted_hunk_statuses
    });
    cx.executor().run_until_parked();
    cx.assert_editor_state(expected_reverted_text_with_selections);
    assert_eq!(actual_hunk_statuses_before, expected_hunk_statuses_before);
}

#[gpui::test]
async fn test_pulling_diagnostics(cx: &mut TestAppContext) {
    init_test(cx, |_| {});

    let mut cx = EditorLspTestContext::new_rust(
        lsp::ServerCapabilities {
            diagnostic_provider: Some(lsp::DiagnosticServerCapabilities::Options(
                lsp::DiagnosticOptions {
                    identifier: Some("rust-analyzer".into()),
                    inter_file_dependencies: true,
                    workspace_diagnostics: true,
                    work_done_progress_options: Default::default(),
                },
            )),
            ..Default::default()
        },
        cx,
    )
    .await;

    let diagnostic_requests = Arc::new(AtomicUsize::new(0));
    let counter = diagnostic_requests.clone();

    cx.lsp
        .handle_request::<lsp::request::DocumentDiagnosticRequest, _, _>(move |params, _| {
            counter.fetch_add(1, atomic::Ordering::Release);
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path("/root/dir/file.rs").unwrap()
            );
            async move {
                Ok(lsp::DocumentDiagnosticReportResult::Report(
                    lsp::DocumentDiagnosticReport::Full(lsp::RelatedFullDocumentDiagnosticReport {
                        related_documents: None,
                        full_document_diagnostic_report: lsp::FullDocumentDiagnosticReport {
                            items: vec![],
                            result_id: None,
                        },
                    }),
                ))
            }
        });

    // Opening file should trigger diagnostics
    cx.set_state(indoc! {"
            fn main() {
                let a = ˇ1;
            }
        "});
    cx.executor().run_until_parked();
    cx.executor().advance_clock(Duration::from_millis(300));
    cx.executor().run_until_parked();
    assert_eq!(
        diagnostic_requests.load(atomic::Ordering::Acquire),
        1,
        "Opening file should trigger diagnostic request"
    );

    // Editing should trigger diagnostics
    cx.update_editor(|editor, window, cx| editor.handle_input("2", window, cx));
    cx.executor().run_until_parked();
    cx.executor().advance_clock(Duration::from_millis(300));
    cx.executor().run_until_parked();
    assert_eq!(
        diagnostic_requests.load(atomic::Ordering::Acquire),
        2,
        "Editing should trigger diagnostic request"
    );

    // Moving cursor should not trigger diagnostic request
    cx.update_editor(|editor, window, cx| {
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(0, 0)..Point::new(0, 0)])
        });
    });
    cx.executor().run_until_parked();
    cx.executor().advance_clock(Duration::from_millis(300));
    cx.executor().run_until_parked();
    assert_eq!(
        diagnostic_requests.load(atomic::Ordering::Acquire),
        2,
        "Cursor movement should not trigger diagnostic request"
    );

    // Multiple rapid edits should be debounced
    for _ in 0..5 {
        cx.update_editor(|editor, window, cx| editor.handle_input("x", window, cx));
    }
    cx.executor().run_until_parked();
    cx.executor().advance_clock(Duration::from_millis(300));
    cx.executor().run_until_parked();

    let final_requests = diagnostic_requests.load(atomic::Ordering::Acquire);
    assert!(
        final_requests <= 4,
        "Multiple rapid edits should be debounced (got {} requests)",
        final_requests
    );
}
