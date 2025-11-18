use super::*;
use buffer_diff::{DiffHunkStatus, DiffHunkStatusKind};
use gpui::{App, TestAppContext};
use indoc::indoc;
use language::{Buffer, Rope};
use parking_lot::RwLock;
use rand::prelude::*;
use settings::SettingsStore;
use std::env;
use std::time::{Duration, Instant};
use util::RandomCharIter;
use util::rel_path::rel_path;
use util::test::sample_text;

#[ctor::ctor]
fn init_logger() {
    zlog::init_test();
}

#[gpui::test]
fn test_empty_singleton(cx: &mut App) {
    let buffer = cx.new(|cx| Buffer::local("", cx));
    let buffer_id = buffer.read(cx).remote_id();
    let multibuffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));
    let snapshot = multibuffer.read(cx).snapshot(cx);
    assert_eq!(snapshot.text(), "");
    assert_eq!(
        snapshot.row_infos(MultiBufferRow(0)).collect::<Vec<_>>(),
        [RowInfo {
            buffer_id: Some(buffer_id),
            buffer_row: Some(0),
            multibuffer_row: Some(MultiBufferRow(0)),
            diff_status: None,
            expand_info: None,
            wrapped_buffer_row: None,
        }]
    );
}

#[gpui::test]
fn test_singleton(cx: &mut App) {
    let buffer = cx.new(|cx| Buffer::local(sample_text(6, 6, 'a'), cx));
    let multibuffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

    let snapshot = multibuffer.read(cx).snapshot(cx);
    assert_eq!(snapshot.text(), buffer.read(cx).text());

    assert_eq!(
        snapshot
            .row_infos(MultiBufferRow(0))
            .map(|info| info.buffer_row)
            .collect::<Vec<_>>(),
        (0..buffer.read(cx).row_count())
            .map(Some)
            .collect::<Vec<_>>()
    );
    assert_consistent_line_numbers(&snapshot);

    buffer.update(cx, |buffer, cx| buffer.edit([(1..3, "XXX\n")], None, cx));
    let snapshot = multibuffer.read(cx).snapshot(cx);

    assert_eq!(snapshot.text(), buffer.read(cx).text());
    assert_eq!(
        snapshot
            .row_infos(MultiBufferRow(0))
            .map(|info| info.buffer_row)
            .collect::<Vec<_>>(),
        (0..buffer.read(cx).row_count())
            .map(Some)
            .collect::<Vec<_>>()
    );
    assert_consistent_line_numbers(&snapshot);
}

#[gpui::test]
fn test_remote(cx: &mut App) {
    let host_buffer = cx.new(|cx| Buffer::local("a", cx));
    let guest_buffer = cx.new(|cx| {
        let state = host_buffer.read(cx).to_proto(cx);
        let ops = cx
            .background_executor()
            .block(host_buffer.read(cx).serialize_ops(None, cx));
        let mut buffer =
            Buffer::from_proto(ReplicaId::REMOTE_SERVER, Capability::ReadWrite, state, None)
                .unwrap();
        buffer.apply_ops(
            ops.into_iter()
                .map(|op| language::proto::deserialize_operation(op).unwrap()),
            cx,
        );
        buffer
    });
    let multibuffer = cx.new(|cx| MultiBuffer::singleton(guest_buffer.clone(), cx));
    let snapshot = multibuffer.read(cx).snapshot(cx);
    assert_eq!(snapshot.text(), "a");

    guest_buffer.update(cx, |buffer, cx| buffer.edit([(1..1, "b")], None, cx));
    let snapshot = multibuffer.read(cx).snapshot(cx);
    assert_eq!(snapshot.text(), "ab");

    guest_buffer.update(cx, |buffer, cx| buffer.edit([(2..2, "c")], None, cx));
    let snapshot = multibuffer.read(cx).snapshot(cx);
    assert_eq!(snapshot.text(), "abc");
}

#[gpui::test]
fn test_excerpt_boundaries_and_clipping(cx: &mut App) {
    let buffer_1 = cx.new(|cx| Buffer::local(sample_text(6, 6, 'a'), cx));
    let buffer_2 = cx.new(|cx| Buffer::local(sample_text(6, 6, 'g'), cx));
    let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));

    let events = Arc::new(RwLock::new(Vec::<Event>::new()));
    multibuffer.update(cx, |_, cx| {
        let events = events.clone();
        cx.subscribe(&multibuffer, move |_, _, event, _| {
            if let Event::Edited { .. } = event {
                events.write().push(event.clone())
            }
        })
        .detach();
    });

    let subscription = multibuffer.update(cx, |multibuffer, cx| {
        let subscription = multibuffer.subscribe();
        multibuffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange::new(Point::new(1, 2)..Point::new(2, 5))],
            cx,
        );
        assert_eq!(
            subscription.consume().into_inner(),
            [Edit {
                old: 0..0,
                new: 0..10
            }]
        );

        multibuffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange::new(Point::new(3, 3)..Point::new(4, 4))],
            cx,
        );
        multibuffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange::new(Point::new(3, 1)..Point::new(3, 3))],
            cx,
        );
        assert_eq!(
            subscription.consume().into_inner(),
            [Edit {
                old: 10..10,
                new: 10..22
            }]
        );

        subscription
    });

    // Adding excerpts emits an edited event.
    assert_eq!(
        events.read().as_slice(),
        &[
            Event::Edited {
                edited_buffer: None,
            },
            Event::Edited {
                edited_buffer: None,
            },
            Event::Edited {
                edited_buffer: None,
            }
        ]
    );

    let snapshot = multibuffer.read(cx).snapshot(cx);
    assert_eq!(
        snapshot.text(),
        indoc!(
            "
            bbbb
            ccccc
            ddd
            eeee
            jj"
        ),
    );
    assert_eq!(
        snapshot
            .row_infos(MultiBufferRow(0))
            .map(|info| info.buffer_row)
            .collect::<Vec<_>>(),
        [Some(1), Some(2), Some(3), Some(4), Some(3)]
    );
    assert_eq!(
        snapshot
            .row_infos(MultiBufferRow(2))
            .map(|info| info.buffer_row)
            .collect::<Vec<_>>(),
        [Some(3), Some(4), Some(3)]
    );
    assert_eq!(
        snapshot
            .row_infos(MultiBufferRow(4))
            .map(|info| info.buffer_row)
            .collect::<Vec<_>>(),
        [Some(3)]
    );
    assert!(
        snapshot
            .row_infos(MultiBufferRow(5))
            .map(|info| info.buffer_row)
            .collect::<Vec<_>>()
            .is_empty()
    );

    assert_eq!(
        boundaries_in_range(Point::new(0, 0)..Point::new(4, 2), &snapshot),
        &[
            (MultiBufferRow(0), "bbbb\nccccc".to_string(), true),
            (MultiBufferRow(2), "ddd\neeee".to_string(), false),
            (MultiBufferRow(4), "jj".to_string(), true),
        ]
    );
    assert_eq!(
        boundaries_in_range(Point::new(0, 0)..Point::new(2, 0), &snapshot),
        &[(MultiBufferRow(0), "bbbb\nccccc".to_string(), true)]
    );
    assert_eq!(
        boundaries_in_range(Point::new(1, 0)..Point::new(1, 5), &snapshot),
        &[]
    );
    assert_eq!(
        boundaries_in_range(Point::new(1, 0)..Point::new(2, 0), &snapshot),
        &[]
    );
    assert_eq!(
        boundaries_in_range(Point::new(1, 0)..Point::new(4, 0), &snapshot),
        &[(MultiBufferRow(2), "ddd\neeee".to_string(), false)]
    );
    assert_eq!(
        boundaries_in_range(Point::new(1, 0)..Point::new(4, 0), &snapshot),
        &[(MultiBufferRow(2), "ddd\neeee".to_string(), false)]
    );
    assert_eq!(
        boundaries_in_range(Point::new(2, 0)..Point::new(3, 0), &snapshot),
        &[(MultiBufferRow(2), "ddd\neeee".to_string(), false)]
    );
    assert_eq!(
        boundaries_in_range(Point::new(4, 0)..Point::new(4, 2), &snapshot),
        &[(MultiBufferRow(4), "jj".to_string(), true)]
    );
    assert_eq!(
        boundaries_in_range(Point::new(4, 2)..Point::new(4, 2), &snapshot),
        &[]
    );

    buffer_1.update(cx, |buffer, cx| {
        let text = "\n";
        buffer.edit(
            [
                (Point::new(0, 0)..Point::new(0, 0), text),
                (Point::new(2, 1)..Point::new(2, 3), text),
            ],
            None,
            cx,
        );
    });

    let snapshot = multibuffer.read(cx).snapshot(cx);
    assert_eq!(
        snapshot.text(),
        concat!(
            "bbbb\n", // Preserve newlines
            "c\n",    //
            "cc\n",   //
            "ddd\n",  //
            "eeee\n", //
            "jj"      //
        )
    );

    assert_eq!(
        subscription.consume().into_inner(),
        [Edit {
            old: 6..8,
            new: 6..7
        }]
    );

    let snapshot = multibuffer.read(cx).snapshot(cx);
    assert_eq!(
        snapshot.clip_point(Point::new(0, 5), Bias::Left),
        Point::new(0, 4)
    );
    assert_eq!(
        snapshot.clip_point(Point::new(0, 5), Bias::Right),
        Point::new(0, 4)
    );
    assert_eq!(
        snapshot.clip_point(Point::new(5, 1), Bias::Right),
        Point::new(5, 1)
    );
    assert_eq!(
        snapshot.clip_point(Point::new(5, 2), Bias::Right),
        Point::new(5, 2)
    );
    assert_eq!(
        snapshot.clip_point(Point::new(5, 3), Bias::Right),
        Point::new(5, 2)
    );

    let snapshot = multibuffer.update(cx, |multibuffer, cx| {
        let (buffer_2_excerpt_id, _) =
            multibuffer.excerpts_for_buffer(buffer_2.read(cx).remote_id(), cx)[0].clone();
        multibuffer.remove_excerpts([buffer_2_excerpt_id], cx);
        multibuffer.snapshot(cx)
    });

    assert_eq!(
        snapshot.text(),
        concat!(
            "bbbb\n", // Preserve newlines
            "c\n",    //
            "cc\n",   //
            "ddd\n",  //
            "eeee",   //
        )
    );

    fn boundaries_in_range(
        range: Range<Point>,
        snapshot: &MultiBufferSnapshot,
    ) -> Vec<(MultiBufferRow, String, bool)> {
        snapshot
            .excerpt_boundaries_in_range(range)
            .map(|boundary| {
                let starts_new_buffer = boundary.starts_new_buffer();
                (
                    boundary.row,
                    boundary
                        .next
                        .buffer
                        .text_for_range(boundary.next.range.context)
                        .collect::<String>(),
                    starts_new_buffer,
                )
            })
            .collect::<Vec<_>>()
    }
}

#[gpui::test]
fn test_diff_boundary_anchors(cx: &mut TestAppContext) {
    let base_text = "one\ntwo\nthree\n";
    let text = "one\nthree\n";
    let buffer = cx.new(|cx| Buffer::local(text, cx));
    let diff = cx.new(|cx| BufferDiff::new_with_base_text(base_text, &buffer, cx));
    let multibuffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    multibuffer.update(cx, |multibuffer, cx| multibuffer.add_diff(diff, cx));

    let (before, after) = multibuffer.update(cx, |multibuffer, cx| {
        let before = multibuffer.snapshot(cx).anchor_before(Point::new(1, 0));
        let after = multibuffer.snapshot(cx).anchor_after(Point::new(1, 0));
        multibuffer.set_all_diff_hunks_expanded(cx);
        (before, after)
    });
    cx.run_until_parked();

    let snapshot = multibuffer.read_with(cx, |multibuffer, cx| multibuffer.snapshot(cx));
    let actual_text = snapshot.text();
    let actual_row_infos = snapshot.row_infos(MultiBufferRow(0)).collect::<Vec<_>>();
    let actual_diff = format_diff(&actual_text, &actual_row_infos, &Default::default(), None);
    pretty_assertions::assert_eq!(
        actual_diff,
        indoc! {
            "  one
             - two
               three
             "
        },
    );

    multibuffer.update(cx, |multibuffer, cx| {
        let snapshot = multibuffer.snapshot(cx);
        assert_eq!(before.to_point(&snapshot), Point::new(1, 0));
        assert_eq!(after.to_point(&snapshot), Point::new(2, 0));
        assert_eq!(
            vec![Point::new(1, 0), Point::new(2, 0),],
            snapshot.summaries_for_anchors::<Point, _>(&[before, after]),
        )
    })
}

#[gpui::test]
fn test_diff_hunks_in_range(cx: &mut TestAppContext) {
    let base_text = "one\ntwo\nthree\nfour\nfive\nsix\nseven\neight\n";
    let text = "one\nfour\nseven\n";
    let buffer = cx.new(|cx| Buffer::local(text, cx));
    let diff = cx.new(|cx| BufferDiff::new_with_base_text(base_text, &buffer, cx));
    let multibuffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let (mut snapshot, mut subscription) = multibuffer.update(cx, |multibuffer, cx| {
        (multibuffer.snapshot(cx), multibuffer.subscribe())
    });

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.add_diff(diff, cx);
        multibuffer.expand_diff_hunks(vec![Anchor::min()..Anchor::max()], cx);
    });

    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc! {
            "  one
             - two
             - three
               four
             - five
             - six
               seven
             - eight
            "
        },
    );

    assert_eq!(
        snapshot
            .diff_hunks_in_range(Point::new(1, 0)..Point::MAX)
            .map(|hunk| hunk.row_range.start.0..hunk.row_range.end.0)
            .collect::<Vec<_>>(),
        vec![1..3, 4..6, 7..8]
    );

    assert_eq!(snapshot.diff_hunk_before(Point::new(1, 1)), None,);
    assert_eq!(
        snapshot.diff_hunk_before(Point::new(7, 0)),
        Some(MultiBufferRow(4))
    );
    assert_eq!(
        snapshot.diff_hunk_before(Point::new(4, 0)),
        Some(MultiBufferRow(1))
    );

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.collapse_diff_hunks(vec![Anchor::min()..Anchor::max()], cx);
    });

    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc! {
            "
            one
            four
            seven
            "
        },
    );

    assert_eq!(
        snapshot.diff_hunk_before(Point::new(2, 0)),
        Some(MultiBufferRow(1)),
    );
    assert_eq!(
        snapshot.diff_hunk_before(Point::new(4, 0)),
        Some(MultiBufferRow(2))
    );
}

#[gpui::test]
fn test_editing_text_in_diff_hunks(cx: &mut TestAppContext) {
    let base_text = "one\ntwo\nfour\nfive\nsix\nseven\n";
    let text = "one\ntwo\nTHREE\nfour\nfive\nseven\n";
    let buffer = cx.new(|cx| Buffer::local(text, cx));
    let diff = cx.new(|cx| BufferDiff::new_with_base_text(base_text, &buffer, cx));
    let multibuffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

    let (mut snapshot, mut subscription) = multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.add_diff(diff.clone(), cx);
        (multibuffer.snapshot(cx), multibuffer.subscribe())
    });

    cx.executor().run_until_parked();
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.set_all_diff_hunks_expanded(cx);
    });

    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc! {
            "
              one
              two
            + THREE
              four
              five
            - six
              seven
            "
        },
    );

    // Insert a newline within an insertion hunk
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.edit([(Point::new(2, 0)..Point::new(2, 0), "__\n__")], None, cx);
    });
    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc! {
            "
              one
              two
            + __
            + __THREE
              four
              five
            - six
              seven
            "
        },
    );

    // Delete the newline before a deleted hunk.
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.edit([(Point::new(5, 4)..Point::new(6, 0), "")], None, cx);
    });
    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc! {
            "
              one
              two
            + __
            + __THREE
              four
              fiveseven
            "
        },
    );

    multibuffer.update(cx, |multibuffer, cx| multibuffer.undo(cx));
    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc! {
            "
              one
              two
            + __
            + __THREE
              four
              five
            - six
              seven
            "
        },
    );

    // Cannot (yet) insert at the beginning of a deleted hunk.
    // (because it would put the newline in the wrong place)
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.edit([(Point::new(6, 0)..Point::new(6, 0), "\n")], None, cx);
    });
    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc! {
            "
              one
              two
            + __
            + __THREE
              four
              five
            - six
              seven
            "
        },
    );

    // Replace a range that ends in a deleted hunk.
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.edit([(Point::new(5, 2)..Point::new(6, 2), "fty-")], None, cx);
    });
    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc! {
            "
              one
              two
            + __
            + __THREE
              four
              fifty-seven
            "
        },
    );
}

#[gpui::test]
fn test_excerpt_events(cx: &mut App) {
    let buffer_1 = cx.new(|cx| Buffer::local(sample_text(10, 3, 'a'), cx));
    let buffer_2 = cx.new(|cx| Buffer::local(sample_text(10, 3, 'm'), cx));

    let leader_multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
    let follower_multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
    let follower_edit_event_count = Arc::new(RwLock::new(0));

    follower_multibuffer.update(cx, |_, cx| {
        let follower_edit_event_count = follower_edit_event_count.clone();
        cx.subscribe(
            &leader_multibuffer,
            move |follower, _, event, cx| match event.clone() {
                Event::ExcerptsAdded {
                    buffer,
                    predecessor,
                    excerpts,
                } => follower.insert_excerpts_with_ids_after(predecessor, buffer, excerpts, cx),
                Event::ExcerptsRemoved { ids, .. } => follower.remove_excerpts(ids, cx),
                Event::Edited { .. } => {
                    *follower_edit_event_count.write() += 1;
                }
                _ => {}
            },
        )
        .detach();
    });

    leader_multibuffer.update(cx, |leader, cx| {
        leader.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange::new(0..8), ExcerptRange::new(12..16)],
            cx,
        );
        leader.insert_excerpts_after(
            leader.excerpt_ids()[0],
            buffer_2.clone(),
            [ExcerptRange::new(0..5), ExcerptRange::new(10..15)],
            cx,
        )
    });
    assert_eq!(
        leader_multibuffer.read(cx).snapshot(cx).text(),
        follower_multibuffer.read(cx).snapshot(cx).text(),
    );
    assert_eq!(*follower_edit_event_count.read(), 2);

    leader_multibuffer.update(cx, |leader, cx| {
        let excerpt_ids = leader.excerpt_ids();
        leader.remove_excerpts([excerpt_ids[1], excerpt_ids[3]], cx);
    });
    assert_eq!(
        leader_multibuffer.read(cx).snapshot(cx).text(),
        follower_multibuffer.read(cx).snapshot(cx).text(),
    );
    assert_eq!(*follower_edit_event_count.read(), 3);

    // Removing an empty set of excerpts is a noop.
    leader_multibuffer.update(cx, |leader, cx| {
        leader.remove_excerpts([], cx);
    });
    assert_eq!(
        leader_multibuffer.read(cx).snapshot(cx).text(),
        follower_multibuffer.read(cx).snapshot(cx).text(),
    );
    assert_eq!(*follower_edit_event_count.read(), 3);

    // Adding an empty set of excerpts is a noop.
    leader_multibuffer.update(cx, |leader, cx| {
        leader.push_excerpts::<usize>(buffer_2.clone(), [], cx);
    });
    assert_eq!(
        leader_multibuffer.read(cx).snapshot(cx).text(),
        follower_multibuffer.read(cx).snapshot(cx).text(),
    );
    assert_eq!(*follower_edit_event_count.read(), 3);

    leader_multibuffer.update(cx, |leader, cx| {
        leader.clear(cx);
    });
    assert_eq!(
        leader_multibuffer.read(cx).snapshot(cx).text(),
        follower_multibuffer.read(cx).snapshot(cx).text(),
    );
    assert_eq!(*follower_edit_event_count.read(), 4);
}

#[gpui::test]
fn test_expand_excerpts(cx: &mut App) {
    let buffer = cx.new(|cx| Buffer::local(sample_text(20, 3, 'a'), cx));
    let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.set_excerpts_for_path(
            PathKey::for_buffer(&buffer, cx),
            buffer,
            vec![
                // Note that in this test, this first excerpt
                // does not contain a new line
                Point::new(3, 2)..Point::new(3, 3),
                Point::new(7, 1)..Point::new(7, 3),
                Point::new(15, 0)..Point::new(15, 0),
            ],
            1,
            cx,
        )
    });

    let snapshot = multibuffer.read(cx).snapshot(cx);

    assert_eq!(
        snapshot.text(),
        concat!(
            "ccc\n", //
            "ddd\n", //
            "eee",   //
            "\n",    // End of excerpt
            "ggg\n", //
            "hhh\n", //
            "iii",   //
            "\n",    // End of excerpt
            "ooo\n", //
            "ppp\n", //
            "qqq",   // End of excerpt
        )
    );
    drop(snapshot);

    multibuffer.update(cx, |multibuffer, cx| {
        let line_zero = multibuffer.snapshot(cx).anchor_before(Point::new(0, 0));
        multibuffer.expand_excerpts(
            multibuffer.excerpt_ids(),
            1,
            ExpandExcerptDirection::UpAndDown,
            cx,
        );
        let snapshot = multibuffer.snapshot(cx);
        let line_two = snapshot.anchor_before(Point::new(2, 0));
        assert_eq!(line_two.cmp(&line_zero, &snapshot), cmp::Ordering::Greater);
    });

    let snapshot = multibuffer.read(cx).snapshot(cx);

    assert_eq!(
        snapshot.text(),
        concat!(
            "bbb\n", //
            "ccc\n", //
            "ddd\n", //
            "eee\n", //
            "fff\n", //
            "ggg\n", //
            "hhh\n", //
            "iii\n", //
            "jjj\n", // End of excerpt
            "nnn\n", //
            "ooo\n", //
            "ppp\n", //
            "qqq\n", //
            "rrr",   // End of excerpt
        )
    );
}

#[gpui::test(iterations = 100)]
async fn test_set_anchored_excerpts_for_path(cx: &mut TestAppContext) {
    let buffer_1 = cx.new(|cx| Buffer::local(sample_text(20, 3, 'a'), cx));
    let buffer_2 = cx.new(|cx| Buffer::local(sample_text(15, 4, 'a'), cx));
    let snapshot_1 = buffer_1.update(cx, |buffer, _| buffer.snapshot());
    let snapshot_2 = buffer_2.update(cx, |buffer, _| buffer.snapshot());
    let ranges_1 = vec![
        snapshot_1.anchor_before(Point::new(3, 2))..snapshot_1.anchor_before(Point::new(4, 2)),
        snapshot_1.anchor_before(Point::new(7, 1))..snapshot_1.anchor_before(Point::new(7, 3)),
        snapshot_1.anchor_before(Point::new(15, 0))..snapshot_1.anchor_before(Point::new(15, 0)),
    ];
    let ranges_2 = vec![
        snapshot_2.anchor_before(Point::new(2, 1))..snapshot_2.anchor_before(Point::new(3, 1)),
        snapshot_2.anchor_before(Point::new(10, 0))..snapshot_2.anchor_before(Point::new(10, 2)),
    ];

    let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
    let anchor_ranges_1 = multibuffer
        .update(cx, |multibuffer, cx| {
            multibuffer.set_anchored_excerpts_for_path(
                PathKey::for_buffer(&buffer_1, cx),
                buffer_1.clone(),
                ranges_1,
                2,
                cx,
            )
        })
        .await;
    let snapshot_1 = multibuffer.update(cx, |multibuffer, cx| multibuffer.snapshot(cx));
    assert_eq!(
        anchor_ranges_1
            .iter()
            .map(|range| range.to_point(&snapshot_1))
            .collect::<Vec<_>>(),
        vec![
            Point::new(2, 2)..Point::new(3, 2),
            Point::new(6, 1)..Point::new(6, 3),
            Point::new(11, 0)..Point::new(11, 0),
        ]
    );
    let anchor_ranges_2 = multibuffer
        .update(cx, |multibuffer, cx| {
            multibuffer.set_anchored_excerpts_for_path(
                PathKey::for_buffer(&buffer_2, cx),
                buffer_2.clone(),
                ranges_2,
                2,
                cx,
            )
        })
        .await;
    let snapshot_2 = multibuffer.update(cx, |multibuffer, cx| multibuffer.snapshot(cx));
    assert_eq!(
        anchor_ranges_2
            .iter()
            .map(|range| range.to_point(&snapshot_2))
            .collect::<Vec<_>>(),
        vec![
            Point::new(16, 1)..Point::new(17, 1),
            Point::new(22, 0)..Point::new(22, 2)
        ]
    );

    let snapshot = multibuffer.update(cx, |multibuffer, cx| multibuffer.snapshot(cx));
    assert_eq!(
        snapshot.text(),
        concat!(
            "bbb\n", // buffer_1
            "ccc\n", //
            "ddd\n", // <-- excerpt 1
            "eee\n", // <-- excerpt 1
            "fff\n", //
            "ggg\n", //
            "hhh\n", // <-- excerpt 2
            "iii\n", //
            "jjj\n", //
            //
            "nnn\n", //
            "ooo\n", //
            "ppp\n", // <-- excerpt 3
            "qqq\n", //
            "rrr\n", //
            //
            "aaaa\n", // buffer 2
            "bbbb\n", //
            "cccc\n", // <-- excerpt 4
            "dddd\n", // <-- excerpt 4
            "eeee\n", //
            "ffff\n", //
            //
            "iiii\n", //
            "jjjj\n", //
            "kkkk\n", // <-- excerpt 5
            "llll\n", //
            "mmmm",   //
        )
    );
}

#[gpui::test]
fn test_empty_multibuffer(cx: &mut App) {
    let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));

    let snapshot = multibuffer.read(cx).snapshot(cx);
    assert_eq!(snapshot.text(), "");
    assert_eq!(
        snapshot
            .row_infos(MultiBufferRow(0))
            .map(|info| info.buffer_row)
            .collect::<Vec<_>>(),
        &[Some(0)]
    );
    assert!(
        snapshot
            .row_infos(MultiBufferRow(1))
            .map(|info| info.buffer_row)
            .collect::<Vec<_>>()
            .is_empty(),
    );
}

#[gpui::test]
fn test_empty_diff_excerpt(cx: &mut TestAppContext) {
    let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
    let buffer = cx.new(|cx| Buffer::local("", cx));
    let base_text = "a\nb\nc";

    let diff = cx.new(|cx| BufferDiff::new_with_base_text(base_text, &buffer, cx));
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.push_excerpts(buffer.clone(), [ExcerptRange::new(0..0)], cx);
        multibuffer.set_all_diff_hunks_expanded(cx);
        multibuffer.add_diff(diff.clone(), cx);
    });
    cx.run_until_parked();

    let snapshot = multibuffer.update(cx, |multibuffer, cx| multibuffer.snapshot(cx));
    assert_eq!(snapshot.text(), "a\nb\nc\n");

    let hunk = snapshot
        .diff_hunks_in_range(Point::new(1, 1)..Point::new(1, 1))
        .next()
        .unwrap();

    assert_eq!(hunk.diff_base_byte_range.start, 0);

    let buf2 = cx.new(|cx| Buffer::local("X", cx));
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.push_excerpts(buf2, [ExcerptRange::new(0..1)], cx);
    });

    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "a\nb\nc")], None, cx);
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(buffer.snapshot().text, cx);
        });
        assert_eq!(buffer.text(), "a\nb\nc")
    });
    cx.run_until_parked();

    let snapshot = multibuffer.update(cx, |multibuffer, cx| multibuffer.snapshot(cx));
    assert_eq!(snapshot.text(), "a\nb\nc\nX");

    buffer.update(cx, |buffer, cx| {
        buffer.undo(cx);
        diff.update(cx, |diff, cx| {
            diff.recalculate_diff_sync(buffer.snapshot().text, cx);
        });
        assert_eq!(buffer.text(), "")
    });
    cx.run_until_parked();

    let snapshot = multibuffer.update(cx, |multibuffer, cx| multibuffer.snapshot(cx));
    assert_eq!(snapshot.text(), "a\nb\nc\n\nX");
}

#[gpui::test]
fn test_singleton_multibuffer_anchors(cx: &mut App) {
    let buffer = cx.new(|cx| Buffer::local("abcd", cx));
    let multibuffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));
    let old_snapshot = multibuffer.read(cx).snapshot(cx);
    buffer.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "X")], None, cx);
        buffer.edit([(5..5, "Y")], None, cx);
    });
    let new_snapshot = multibuffer.read(cx).snapshot(cx);

    assert_eq!(old_snapshot.text(), "abcd");
    assert_eq!(new_snapshot.text(), "XabcdY");

    assert_eq!(old_snapshot.anchor_before(0).to_offset(&new_snapshot), 0);
    assert_eq!(old_snapshot.anchor_after(0).to_offset(&new_snapshot), 1);
    assert_eq!(old_snapshot.anchor_before(4).to_offset(&new_snapshot), 5);
    assert_eq!(old_snapshot.anchor_after(4).to_offset(&new_snapshot), 6);
}

#[gpui::test]
fn test_multibuffer_anchors(cx: &mut App) {
    let buffer_1 = cx.new(|cx| Buffer::local("abcd", cx));
    let buffer_2 = cx.new(|cx| Buffer::local("efghi", cx));
    let multibuffer = cx.new(|cx| {
        let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
        multibuffer.push_excerpts(buffer_1.clone(), [ExcerptRange::new(0..4)], cx);
        multibuffer.push_excerpts(buffer_2.clone(), [ExcerptRange::new(0..5)], cx);
        multibuffer
    });
    let old_snapshot = multibuffer.read(cx).snapshot(cx);

    assert_eq!(old_snapshot.anchor_before(0).to_offset(&old_snapshot), 0);
    assert_eq!(old_snapshot.anchor_after(0).to_offset(&old_snapshot), 0);
    assert_eq!(Anchor::min().to_offset(&old_snapshot), 0);
    assert_eq!(Anchor::min().to_offset(&old_snapshot), 0);
    assert_eq!(Anchor::max().to_offset(&old_snapshot), 10);
    assert_eq!(Anchor::max().to_offset(&old_snapshot), 10);

    buffer_1.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "W")], None, cx);
        buffer.edit([(5..5, "X")], None, cx);
    });
    buffer_2.update(cx, |buffer, cx| {
        buffer.edit([(0..0, "Y")], None, cx);
        buffer.edit([(6..6, "Z")], None, cx);
    });
    let new_snapshot = multibuffer.read(cx).snapshot(cx);

    assert_eq!(old_snapshot.text(), "abcd\nefghi");
    assert_eq!(new_snapshot.text(), "WabcdX\nYefghiZ");

    assert_eq!(old_snapshot.anchor_before(0).to_offset(&new_snapshot), 0);
    assert_eq!(old_snapshot.anchor_after(0).to_offset(&new_snapshot), 1);
    assert_eq!(old_snapshot.anchor_before(1).to_offset(&new_snapshot), 2);
    assert_eq!(old_snapshot.anchor_after(1).to_offset(&new_snapshot), 2);
    assert_eq!(old_snapshot.anchor_before(2).to_offset(&new_snapshot), 3);
    assert_eq!(old_snapshot.anchor_after(2).to_offset(&new_snapshot), 3);
    assert_eq!(old_snapshot.anchor_before(5).to_offset(&new_snapshot), 7);
    assert_eq!(old_snapshot.anchor_after(5).to_offset(&new_snapshot), 8);
    assert_eq!(old_snapshot.anchor_before(10).to_offset(&new_snapshot), 13);
    assert_eq!(old_snapshot.anchor_after(10).to_offset(&new_snapshot), 14);
}

#[gpui::test]
fn test_resolving_anchors_after_replacing_their_excerpts(cx: &mut App) {
    let buffer_1 = cx.new(|cx| Buffer::local("abcd", cx));
    let buffer_2 = cx.new(|cx| Buffer::local("ABCDEFGHIJKLMNOP", cx));
    let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));

    // Create an insertion id in buffer 1 that doesn't exist in buffer 2.
    // Add an excerpt from buffer 1 that spans this new insertion.
    buffer_1.update(cx, |buffer, cx| buffer.edit([(4..4, "123")], None, cx));
    let excerpt_id_1 = multibuffer.update(cx, |multibuffer, cx| {
        multibuffer
            .push_excerpts(buffer_1.clone(), [ExcerptRange::new(0..7)], cx)
            .pop()
            .unwrap()
    });

    let snapshot_1 = multibuffer.read(cx).snapshot(cx);
    assert_eq!(snapshot_1.text(), "abcd123");

    // Replace the buffer 1 excerpt with new excerpts from buffer 2.
    let (excerpt_id_2, excerpt_id_3) = multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.remove_excerpts([excerpt_id_1], cx);
        let mut ids = multibuffer
            .push_excerpts(
                buffer_2.clone(),
                [
                    ExcerptRange::new(0..4),
                    ExcerptRange::new(6..10),
                    ExcerptRange::new(12..16),
                ],
                cx,
            )
            .into_iter();
        (ids.next().unwrap(), ids.next().unwrap())
    });
    let snapshot_2 = multibuffer.read(cx).snapshot(cx);
    assert_eq!(snapshot_2.text(), "ABCD\nGHIJ\nMNOP");

    // The old excerpt id doesn't get reused.
    assert_ne!(excerpt_id_2, excerpt_id_1);

    // Resolve some anchors from the previous snapshot in the new snapshot.
    // The current excerpts are from a different buffer, so we don't attempt to
    // resolve the old text anchor in the new buffer.
    assert_eq!(
        snapshot_2.summary_for_anchor::<usize>(&snapshot_1.anchor_before(2)),
        0
    );
    assert_eq!(
        snapshot_2.summaries_for_anchors::<usize, _>(&[
            snapshot_1.anchor_before(2),
            snapshot_1.anchor_after(3)
        ]),
        vec![0, 0]
    );

    // Refresh anchors from the old snapshot. The return value indicates that both
    // anchors lost their original excerpt.
    let refresh =
        snapshot_2.refresh_anchors(&[snapshot_1.anchor_before(2), snapshot_1.anchor_after(3)]);
    assert_eq!(
        refresh,
        &[
            (0, snapshot_2.anchor_before(0), false),
            (1, snapshot_2.anchor_after(0), false),
        ]
    );

    // Replace the middle excerpt with a smaller excerpt in buffer 2,
    // that intersects the old excerpt.
    let excerpt_id_5 = multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.remove_excerpts([excerpt_id_3], cx);
        multibuffer
            .insert_excerpts_after(
                excerpt_id_2,
                buffer_2.clone(),
                [ExcerptRange::new(5..8)],
                cx,
            )
            .pop()
            .unwrap()
    });

    let snapshot_3 = multibuffer.read(cx).snapshot(cx);
    assert_eq!(snapshot_3.text(), "ABCD\nFGH\nMNOP");
    assert_ne!(excerpt_id_5, excerpt_id_3);

    // Resolve some anchors from the previous snapshot in the new snapshot.
    // The third anchor can't be resolved, since its excerpt has been removed,
    // so it resolves to the same position as its predecessor.
    let anchors = [
        snapshot_2.anchor_before(0),
        snapshot_2.anchor_after(2),
        snapshot_2.anchor_after(6),
        snapshot_2.anchor_after(14),
    ];
    assert_eq!(
        snapshot_3.summaries_for_anchors::<usize, _>(&anchors),
        &[0, 2, 9, 13]
    );

    let new_anchors = snapshot_3.refresh_anchors(&anchors);
    assert_eq!(
        new_anchors.iter().map(|a| (a.0, a.2)).collect::<Vec<_>>(),
        &[(0, true), (1, true), (2, true), (3, true)]
    );
    assert_eq!(
        snapshot_3.summaries_for_anchors::<usize, _>(new_anchors.iter().map(|a| &a.1)),
        &[0, 2, 7, 13]
    );
}

#[gpui::test]
fn test_basic_diff_hunks(cx: &mut TestAppContext) {
    let text = indoc!(
        "
        ZERO
        one
        TWO
        three
        six
        "
    );
    let base_text = indoc!(
        "
        one
        two
        three
        four
        five
        six
        "
    );

    let buffer = cx.new(|cx| Buffer::local(text, cx));
    let diff = cx.new(|cx| BufferDiff::new_with_base_text(base_text, &buffer, cx));
    cx.run_until_parked();

    let multibuffer = cx.new(|cx| {
        let mut multibuffer = MultiBuffer::singleton(buffer.clone(), cx);
        multibuffer.add_diff(diff.clone(), cx);
        multibuffer
    });

    let (mut snapshot, mut subscription) = multibuffer.update(cx, |multibuffer, cx| {
        (multibuffer.snapshot(cx), multibuffer.subscribe())
    });
    assert_eq!(
        snapshot.text(),
        indoc!(
            "
            ZERO
            one
            TWO
            three
            six
            "
        ),
    );

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.expand_diff_hunks(vec![Anchor::min()..Anchor::max()], cx);
    });

    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc!(
            "
            + ZERO
              one
            - two
            + TWO
              three
            - four
            - five
              six
            "
        ),
    );

    assert_eq!(
        snapshot
            .row_infos(MultiBufferRow(0))
            .map(|info| (info.buffer_row, info.diff_status))
            .collect::<Vec<_>>(),
        vec![
            (Some(0), Some(DiffHunkStatus::added_none())),
            (Some(1), None),
            (Some(1), Some(DiffHunkStatus::deleted_none())),
            (Some(2), Some(DiffHunkStatus::added_none())),
            (Some(3), None),
            (Some(3), Some(DiffHunkStatus::deleted_none())),
            (Some(4), Some(DiffHunkStatus::deleted_none())),
            (Some(4), None),
            (Some(5), None)
        ]
    );

    assert_chunks_in_ranges(&snapshot);
    assert_consistent_line_numbers(&snapshot);
    assert_position_translation(&snapshot);
    assert_line_indents(&snapshot);

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.collapse_diff_hunks(vec![Anchor::min()..Anchor::max()], cx)
    });
    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc!(
            "
            ZERO
            one
            TWO
            three
            six
            "
        ),
    );

    assert_chunks_in_ranges(&snapshot);
    assert_consistent_line_numbers(&snapshot);
    assert_position_translation(&snapshot);
    assert_line_indents(&snapshot);

    // Expand the first diff hunk
    multibuffer.update(cx, |multibuffer, cx| {
        let position = multibuffer.read(cx).anchor_before(Point::new(2, 2));
        multibuffer.expand_diff_hunks(vec![position..position], cx)
    });
    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc!(
            "
              ZERO
              one
            - two
            + TWO
              three
              six
            "
        ),
    );

    // Expand the second diff hunk
    multibuffer.update(cx, |multibuffer, cx| {
        let start = multibuffer.read(cx).anchor_before(Point::new(4, 0));
        let end = multibuffer.read(cx).anchor_before(Point::new(5, 0));
        multibuffer.expand_diff_hunks(vec![start..end], cx)
    });
    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc!(
            "
              ZERO
              one
            - two
            + TWO
              three
            - four
            - five
              six
            "
        ),
    );

    assert_chunks_in_ranges(&snapshot);
    assert_consistent_line_numbers(&snapshot);
    assert_position_translation(&snapshot);
    assert_line_indents(&snapshot);

    // Edit the buffer before the first hunk
    buffer.update(cx, |buffer, cx| {
        buffer.edit_via_marked_text(
            indoc!(
                "
                ZERO
                one« hundred
                  thousand»
                TWO
                three
                six
                "
            ),
            None,
            cx,
        );
    });
    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc!(
            "
              ZERO
              one hundred
                thousand
            - two
            + TWO
              three
            - four
            - five
              six
            "
        ),
    );

    assert_chunks_in_ranges(&snapshot);
    assert_consistent_line_numbers(&snapshot);
    assert_position_translation(&snapshot);
    assert_line_indents(&snapshot);

    // Recalculate the diff, changing the first diff hunk.
    diff.update(cx, |diff, cx| {
        diff.recalculate_diff_sync(buffer.read(cx).text_snapshot(), cx);
    });
    cx.run_until_parked();
    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc!(
            "
              ZERO
              one hundred
                thousand
              TWO
              three
            - four
            - five
              six
            "
        ),
    );

    assert_eq!(
        snapshot
            .diff_hunks_in_range(0..snapshot.len())
            .map(|hunk| hunk.row_range.start.0..hunk.row_range.end.0)
            .collect::<Vec<_>>(),
        &[0..4, 5..7]
    );
}

#[gpui::test]
fn test_repeatedly_expand_a_diff_hunk(cx: &mut TestAppContext) {
    let text = indoc!(
        "
        one
        TWO
        THREE
        four
        FIVE
        six
        "
    );
    let base_text = indoc!(
        "
        one
        four
        five
        six
        "
    );

    let buffer = cx.new(|cx| Buffer::local(text, cx));
    let diff = cx.new(|cx| BufferDiff::new_with_base_text(base_text, &buffer, cx));
    cx.run_until_parked();

    let multibuffer = cx.new(|cx| {
        let mut multibuffer = MultiBuffer::singleton(buffer.clone(), cx);
        multibuffer.add_diff(diff.clone(), cx);
        multibuffer
    });

    let (mut snapshot, mut subscription) = multibuffer.update(cx, |multibuffer, cx| {
        (multibuffer.snapshot(cx), multibuffer.subscribe())
    });

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.expand_diff_hunks(vec![Anchor::min()..Anchor::max()], cx);
    });

    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc!(
            "
              one
            + TWO
            + THREE
              four
            - five
            + FIVE
              six
            "
        ),
    );

    // Regression test: expanding diff hunks that are already expanded should not change anything.
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.expand_diff_hunks(
            vec![
                snapshot.anchor_before(Point::new(2, 0))..snapshot.anchor_before(Point::new(2, 0)),
            ],
            cx,
        );
    });

    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc!(
            "
              one
            + TWO
            + THREE
              four
            - five
            + FIVE
              six
            "
        ),
    );

    // Now collapse all diff hunks
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.collapse_diff_hunks(vec![Anchor::min()..Anchor::max()], cx);
    });

    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc!(
            "
            one
            TWO
            THREE
            four
            FIVE
            six
            "
        ),
    );

    // Expand the hunks again, but this time provide two ranges that are both within the same hunk
    // Target the first hunk which is between "one" and "four"
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.expand_diff_hunks(
            vec![
                snapshot.anchor_before(Point::new(4, 0))..snapshot.anchor_before(Point::new(4, 0)),
                snapshot.anchor_before(Point::new(4, 2))..snapshot.anchor_before(Point::new(4, 2)),
            ],
            cx,
        );
    });
    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc!(
            "
              one
              TWO
              THREE
              four
            - five
            + FIVE
              six
            "
        ),
    );
}

#[gpui::test]
fn test_set_excerpts_for_buffer_ordering(cx: &mut TestAppContext) {
    let buf1 = cx.new(|cx| {
        Buffer::local(
            indoc! {
            "zero
            one
            two
            two.five
            three
            four
            five
            six
            seven
            eight
            nine
            ten
            eleven
            ",
            },
            cx,
        )
    });
    let path1: PathKey = PathKey::with_sort_prefix(0, rel_path("root").into_arc());

    let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.set_excerpts_for_path(
            path1.clone(),
            buf1.clone(),
            vec![
                Point::row_range(1..2),
                Point::row_range(6..7),
                Point::row_range(11..12),
            ],
            1,
            cx,
        );
    });

    assert_excerpts_match(
        &multibuffer,
        cx,
        indoc! {
            "-----
            zero
            one
            two
            two.five
            -----
            four
            five
            six
            seven
            -----
            nine
            ten
            eleven
            "
        },
    );

    buf1.update(cx, |buffer, cx| buffer.edit([(0..5, "")], None, cx));

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.set_excerpts_for_path(
            path1.clone(),
            buf1.clone(),
            vec![
                Point::row_range(0..3),
                Point::row_range(5..7),
                Point::row_range(10..11),
            ],
            1,
            cx,
        );
    });

    assert_excerpts_match(
        &multibuffer,
        cx,
        indoc! {
            "-----
             one
             two
             two.five
             three
             four
             five
             six
             seven
             eight
             nine
             ten
             eleven
            "
        },
    );
}

#[gpui::test]
fn test_set_excerpts_for_buffer(cx: &mut TestAppContext) {
    let buf1 = cx.new(|cx| {
        Buffer::local(
            indoc! {
            "zero
            one
            two
            three
            four
            five
            six
            seven
            ",
            },
            cx,
        )
    });
    let path1: PathKey = PathKey::with_sort_prefix(0, rel_path("root").into_arc());
    let buf2 = cx.new(|cx| {
        Buffer::local(
            indoc! {
            "000
            111
            222
            333
            444
            555
            666
            777
            888
            999
            "
            },
            cx,
        )
    });
    let path2 = PathKey::with_sort_prefix(1, rel_path("root").into_arc());

    let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.set_excerpts_for_path(
            path1.clone(),
            buf1.clone(),
            vec![Point::row_range(0..1)],
            2,
            cx,
        );
    });

    assert_excerpts_match(
        &multibuffer,
        cx,
        indoc! {
        "-----
        zero
        one
        two
        three
        "
        },
    );

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.set_excerpts_for_path(path1.clone(), buf1.clone(), vec![], 2, cx);
    });

    assert_excerpts_match(&multibuffer, cx, "");

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.set_excerpts_for_path(
            path1.clone(),
            buf1.clone(),
            vec![Point::row_range(0..1), Point::row_range(7..8)],
            2,
            cx,
        );
    });

    assert_excerpts_match(
        &multibuffer,
        cx,
        indoc! {"-----
                zero
                one
                two
                three
                -----
                five
                six
                seven
                "},
    );

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.set_excerpts_for_path(
            path1.clone(),
            buf1.clone(),
            vec![Point::row_range(0..1), Point::row_range(5..6)],
            2,
            cx,
        );
    });

    assert_excerpts_match(
        &multibuffer,
        cx,
        indoc! {"-----
                    zero
                    one
                    two
                    three
                    four
                    five
                    six
                    seven
                    "},
    );

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.set_excerpts_for_path(
            path2.clone(),
            buf2.clone(),
            vec![Point::row_range(2..3)],
            2,
            cx,
        );
    });

    assert_excerpts_match(
        &multibuffer,
        cx,
        indoc! {"-----
                zero
                one
                two
                three
                four
                five
                six
                seven
                -----
                000
                111
                222
                333
                444
                555
                "},
    );

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.set_excerpts_for_path(path1.clone(), buf1.clone(), vec![], 2, cx);
    });

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.set_excerpts_for_path(
            path1.clone(),
            buf1.clone(),
            vec![Point::row_range(3..4)],
            2,
            cx,
        );
    });

    assert_excerpts_match(
        &multibuffer,
        cx,
        indoc! {"-----
                one
                two
                three
                four
                five
                six
                -----
                000
                111
                222
                333
                444
                555
                "},
    );

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.set_excerpts_for_path(
            path1.clone(),
            buf1.clone(),
            vec![Point::row_range(3..4)],
            2,
            cx,
        );
    });
}

#[gpui::test]
fn test_set_excerpts_for_buffer_rename(cx: &mut TestAppContext) {
    let buf1 = cx.new(|cx| {
        Buffer::local(
            indoc! {
            "zero
            one
            two
            three
            four
            five
            six
            seven
            ",
            },
            cx,
        )
    });
    let path: PathKey = PathKey::with_sort_prefix(0, rel_path("root").into_arc());
    let buf2 = cx.new(|cx| {
        Buffer::local(
            indoc! {
            "000
            111
            222
            333
            "
            },
            cx,
        )
    });

    let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.set_excerpts_for_path(
            path.clone(),
            buf1.clone(),
            vec![Point::row_range(1..1), Point::row_range(4..5)],
            1,
            cx,
        );
    });

    assert_excerpts_match(
        &multibuffer,
        cx,
        indoc! {
        "-----
        zero
        one
        two
        three
        four
        five
        six
        "
        },
    );

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.set_excerpts_for_path(
            path.clone(),
            buf2.clone(),
            vec![Point::row_range(0..1)],
            2,
            cx,
        );
    });

    assert_excerpts_match(
        &multibuffer,
        cx,
        indoc! {"-----
                000
                111
                222
                333
                "},
    );
}

#[gpui::test]
fn test_diff_hunks_with_multiple_excerpts(cx: &mut TestAppContext) {
    let base_text_1 = indoc!(
        "
        one
        two
            three
        four
        five
        six
        "
    );
    let text_1 = indoc!(
        "
        ZERO
        one
        TWO
            three
        six
        "
    );
    let base_text_2 = indoc!(
        "
        seven
          eight
        nine
        ten
        eleven
        twelve
        "
    );
    let text_2 = indoc!(
        "
          eight
        nine
        eleven
        THIRTEEN
        FOURTEEN
        "
    );

    let buffer_1 = cx.new(|cx| Buffer::local(text_1, cx));
    let buffer_2 = cx.new(|cx| Buffer::local(text_2, cx));
    let diff_1 = cx.new(|cx| BufferDiff::new_with_base_text(base_text_1, &buffer_1, cx));
    let diff_2 = cx.new(|cx| BufferDiff::new_with_base_text(base_text_2, &buffer_2, cx));
    cx.run_until_parked();

    let multibuffer = cx.new(|cx| {
        let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
        multibuffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange::new(text::Anchor::MIN..text::Anchor::MAX)],
            cx,
        );
        multibuffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange::new(text::Anchor::MIN..text::Anchor::MAX)],
            cx,
        );
        multibuffer.add_diff(diff_1.clone(), cx);
        multibuffer.add_diff(diff_2.clone(), cx);
        multibuffer
    });

    let (mut snapshot, mut subscription) = multibuffer.update(cx, |multibuffer, cx| {
        (multibuffer.snapshot(cx), multibuffer.subscribe())
    });
    assert_eq!(
        snapshot.text(),
        indoc!(
            "
            ZERO
            one
            TWO
                three
            six

              eight
            nine
            eleven
            THIRTEEN
            FOURTEEN
            "
        ),
    );

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.expand_diff_hunks(vec![Anchor::min()..Anchor::max()], cx);
    });

    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc!(
            "
            + ZERO
              one
            - two
            + TWO
                  three
            - four
            - five
              six

            - seven
                eight
              nine
            - ten
              eleven
            - twelve
            + THIRTEEN
            + FOURTEEN
            "
        ),
    );

    let id_1 = buffer_1.read_with(cx, |buffer, _| buffer.remote_id());
    let id_2 = buffer_2.read_with(cx, |buffer, _| buffer.remote_id());
    let base_id_1 = diff_1.read_with(cx, |diff, _| diff.base_text().remote_id());
    let base_id_2 = diff_2.read_with(cx, |diff, _| diff.base_text().remote_id());

    let buffer_lines = (0..=snapshot.max_row().0)
        .map(|row| {
            let (buffer, range) = snapshot.buffer_line_for_row(MultiBufferRow(row))?;
            Some((
                buffer.remote_id(),
                buffer.text_for_range(range).collect::<String>(),
            ))
        })
        .collect::<Vec<_>>();
    pretty_assertions::assert_eq!(
        buffer_lines,
        [
            Some((id_1, "ZERO".into())),
            Some((id_1, "one".into())),
            Some((base_id_1, "two".into())),
            Some((id_1, "TWO".into())),
            Some((id_1, "    three".into())),
            Some((base_id_1, "four".into())),
            Some((base_id_1, "five".into())),
            Some((id_1, "six".into())),
            Some((id_1, "".into())),
            Some((base_id_2, "seven".into())),
            Some((id_2, "  eight".into())),
            Some((id_2, "nine".into())),
            Some((base_id_2, "ten".into())),
            Some((id_2, "eleven".into())),
            Some((base_id_2, "twelve".into())),
            Some((id_2, "THIRTEEN".into())),
            Some((id_2, "FOURTEEN".into())),
            Some((id_2, "".into())),
        ]
    );

    let buffer_ids_by_range = [
        (Point::new(0, 0)..Point::new(0, 0), &[id_1] as &[_]),
        (Point::new(0, 0)..Point::new(2, 0), &[id_1]),
        (Point::new(2, 0)..Point::new(2, 0), &[id_1]),
        (Point::new(3, 0)..Point::new(3, 0), &[id_1]),
        (Point::new(8, 0)..Point::new(9, 0), &[id_1]),
        (Point::new(8, 0)..Point::new(10, 0), &[id_1, id_2]),
        (Point::new(9, 0)..Point::new(9, 0), &[id_2]),
    ];
    for (range, buffer_ids) in buffer_ids_by_range {
        assert_eq!(
            snapshot
                .buffer_ids_for_range(range.clone())
                .collect::<Vec<_>>(),
            buffer_ids,
            "buffer_ids_for_range({range:?}"
        );
    }

    assert_position_translation(&snapshot);
    assert_line_indents(&snapshot);

    assert_eq!(
        snapshot
            .diff_hunks_in_range(0..snapshot.len())
            .map(|hunk| hunk.row_range.start.0..hunk.row_range.end.0)
            .collect::<Vec<_>>(),
        &[0..1, 2..4, 5..7, 9..10, 12..13, 14..17]
    );

    buffer_2.update(cx, |buffer, cx| {
        buffer.edit_via_marked_text(
            indoc!(
                "
                  eight
                «»eleven
                THIRTEEN
                FOURTEEN
                "
            ),
            None,
            cx,
        );
    });

    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc!(
            "
            + ZERO
              one
            - two
            + TWO
                  three
            - four
            - five
              six

            - seven
                eight
              eleven
            - twelve
            + THIRTEEN
            + FOURTEEN
            "
        ),
    );

    assert_line_indents(&snapshot);
}

/// A naive implementation of a multi-buffer that does not maintain
/// any derived state, used for comparison in a randomized test.
#[derive(Default)]
struct ReferenceMultibuffer {
    excerpts: Vec<ReferenceExcerpt>,
    diffs: HashMap<BufferId, Entity<BufferDiff>>,
}

#[derive(Debug)]
struct ReferenceExcerpt {
    id: ExcerptId,
    buffer: Entity<Buffer>,
    range: Range<text::Anchor>,
    expanded_diff_hunks: Vec<text::Anchor>,
}

#[derive(Debug)]
struct ReferenceRegion {
    buffer_id: Option<BufferId>,
    range: Range<usize>,
    buffer_start: Option<Point>,
    status: Option<DiffHunkStatus>,
    excerpt_id: Option<ExcerptId>,
}

impl ReferenceMultibuffer {
    fn expand_excerpts(&mut self, excerpts: &HashSet<ExcerptId>, line_count: u32, cx: &App) {
        if line_count == 0 {
            return;
        }

        for id in excerpts {
            let excerpt = self.excerpts.iter_mut().find(|e| e.id == *id).unwrap();
            let snapshot = excerpt.buffer.read(cx).snapshot();
            let mut point_range = excerpt.range.to_point(&snapshot);
            point_range.start = Point::new(point_range.start.row.saturating_sub(line_count), 0);
            point_range.end =
                snapshot.clip_point(Point::new(point_range.end.row + line_count, 0), Bias::Left);
            point_range.end.column = snapshot.line_len(point_range.end.row);
            excerpt.range =
                snapshot.anchor_before(point_range.start)..snapshot.anchor_after(point_range.end);
        }
    }

    fn remove_excerpt(&mut self, id: ExcerptId, cx: &App) {
        let ix = self
            .excerpts
            .iter()
            .position(|excerpt| excerpt.id == id)
            .unwrap();
        let excerpt = self.excerpts.remove(ix);
        let buffer = excerpt.buffer.read(cx);
        let id = buffer.remote_id();
        log::info!(
            "Removing excerpt {}: {:?}",
            ix,
            buffer
                .text_for_range(excerpt.range.to_offset(buffer))
                .collect::<String>(),
        );
        if !self
            .excerpts
            .iter()
            .any(|excerpt| excerpt.buffer.read(cx).remote_id() == id)
        {
            self.diffs.remove(&id);
        }
    }

    fn insert_excerpt_after(
        &mut self,
        prev_id: ExcerptId,
        new_excerpt_id: ExcerptId,
        (buffer_handle, anchor_range): (Entity<Buffer>, Range<text::Anchor>),
    ) {
        let excerpt_ix = if prev_id == ExcerptId::max() {
            self.excerpts.len()
        } else {
            self.excerpts
                .iter()
                .position(|excerpt| excerpt.id == prev_id)
                .unwrap()
                + 1
        };
        self.excerpts.insert(
            excerpt_ix,
            ReferenceExcerpt {
                id: new_excerpt_id,
                buffer: buffer_handle,
                range: anchor_range,
                expanded_diff_hunks: Vec::new(),
            },
        );
    }

    fn expand_diff_hunks(&mut self, excerpt_id: ExcerptId, range: Range<text::Anchor>, cx: &App) {
        let excerpt = self
            .excerpts
            .iter_mut()
            .find(|e| e.id == excerpt_id)
            .unwrap();
        let buffer = excerpt.buffer.read(cx).snapshot();
        let buffer_id = buffer.remote_id();
        let Some(diff) = self.diffs.get(&buffer_id) else {
            return;
        };
        let excerpt_range = excerpt.range.to_offset(&buffer);
        for hunk in diff.read(cx).hunks_intersecting_range(range, &buffer, cx) {
            let hunk_range = hunk.buffer_range.to_offset(&buffer);
            if hunk_range.start < excerpt_range.start || hunk_range.start > excerpt_range.end {
                continue;
            }
            if let Err(ix) = excerpt
                .expanded_diff_hunks
                .binary_search_by(|anchor| anchor.cmp(&hunk.buffer_range.start, &buffer))
            {
                log::info!(
                    "expanding diff hunk {:?}. excerpt:{:?}, excerpt range:{:?}",
                    hunk_range,
                    excerpt_id,
                    excerpt_range
                );
                excerpt
                    .expanded_diff_hunks
                    .insert(ix, hunk.buffer_range.start);
            } else {
                log::trace!("hunk {hunk_range:?} already expanded in excerpt {excerpt_id:?}");
            }
        }
    }

    fn expected_content(&self, cx: &App) -> (String, Vec<RowInfo>, HashSet<MultiBufferRow>) {
        let mut text = String::new();
        let mut regions = Vec::<ReferenceRegion>::new();
        let mut excerpt_boundary_rows = HashSet::default();
        for excerpt in &self.excerpts {
            excerpt_boundary_rows.insert(MultiBufferRow(text.matches('\n').count() as u32));
            let buffer = excerpt.buffer.read(cx);
            let buffer_range = excerpt.range.to_offset(buffer);
            let diff = self.diffs.get(&buffer.remote_id()).unwrap().read(cx);
            let base_buffer = diff.base_text();

            let mut offset = buffer_range.start;
            let hunks = diff
                .hunks_intersecting_range(excerpt.range.clone(), buffer, cx)
                .peekable();

            for hunk in hunks {
                // Ignore hunks that are outside the excerpt range.
                let mut hunk_range = hunk.buffer_range.to_offset(buffer);

                hunk_range.end = hunk_range.end.min(buffer_range.end);
                if hunk_range.start > buffer_range.end || hunk_range.start < buffer_range.start {
                    log::trace!("skipping hunk outside excerpt range");
                    continue;
                }

                if !excerpt.expanded_diff_hunks.iter().any(|expanded_anchor| {
                    expanded_anchor.to_offset(buffer).max(buffer_range.start)
                        == hunk_range.start.max(buffer_range.start)
                }) {
                    log::trace!("skipping a hunk that's not marked as expanded");
                    continue;
                }

                if !hunk.buffer_range.start.is_valid(buffer) {
                    log::trace!("skipping hunk with deleted start: {:?}", hunk.range);
                    continue;
                }

                if hunk_range.start >= offset {
                    // Add the buffer text before the hunk
                    let len = text.len();
                    text.extend(buffer.text_for_range(offset..hunk_range.start));
                    regions.push(ReferenceRegion {
                        buffer_id: Some(buffer.remote_id()),
                        range: len..text.len(),
                        buffer_start: Some(buffer.offset_to_point(offset)),
                        status: None,
                        excerpt_id: Some(excerpt.id),
                    });

                    // Add the deleted text for the hunk.
                    if !hunk.diff_base_byte_range.is_empty() {
                        let mut base_text = base_buffer
                            .text_for_range(hunk.diff_base_byte_range.clone())
                            .collect::<String>();
                        if !base_text.ends_with('\n') {
                            base_text.push('\n');
                        }
                        let len = text.len();
                        text.push_str(&base_text);
                        regions.push(ReferenceRegion {
                            buffer_id: Some(base_buffer.remote_id()),
                            range: len..text.len(),
                            buffer_start: Some(
                                base_buffer.offset_to_point(hunk.diff_base_byte_range.start),
                            ),
                            status: Some(DiffHunkStatus::deleted(hunk.secondary_status)),
                            excerpt_id: Some(excerpt.id),
                        });
                    }

                    offset = hunk_range.start;
                }

                // Add the inserted text for the hunk.
                if hunk_range.end > offset {
                    let len = text.len();
                    text.extend(buffer.text_for_range(offset..hunk_range.end));
                    regions.push(ReferenceRegion {
                        buffer_id: Some(buffer.remote_id()),
                        range: len..text.len(),
                        buffer_start: Some(buffer.offset_to_point(offset)),
                        status: Some(DiffHunkStatus::added(hunk.secondary_status)),
                        excerpt_id: Some(excerpt.id),
                    });
                    offset = hunk_range.end;
                }
            }

            // Add the buffer text for the rest of the excerpt.
            let len = text.len();
            text.extend(buffer.text_for_range(offset..buffer_range.end));
            text.push('\n');
            regions.push(ReferenceRegion {
                buffer_id: Some(buffer.remote_id()),
                range: len..text.len(),
                buffer_start: Some(buffer.offset_to_point(offset)),
                status: None,
                excerpt_id: Some(excerpt.id),
            });
        }

        // Remove final trailing newline.
        if self.excerpts.is_empty() {
            regions.push(ReferenceRegion {
                buffer_id: None,
                range: 0..1,
                buffer_start: Some(Point::new(0, 0)),
                status: None,
                excerpt_id: None,
            });
        } else {
            text.pop();
        }

        // Retrieve the row info using the region that contains
        // the start of each multi-buffer line.
        let mut ix = 0;
        let row_infos = text
            .split('\n')
            .map(|line| {
                let row_info = regions
                    .iter()
                    .position(|region| region.range.contains(&ix))
                    .map_or(RowInfo::default(), |region_ix| {
                        let region = &regions[region_ix];
                        let buffer_row = region.buffer_start.map(|start_point| {
                            start_point.row
                                + text[region.range.start..ix].matches('\n').count() as u32
                        });
                        let is_excerpt_start = region_ix == 0
                            || &regions[region_ix - 1].excerpt_id != &region.excerpt_id
                            || regions[region_ix - 1].range.is_empty();
                        let mut is_excerpt_end = region_ix == regions.len() - 1
                            || &regions[region_ix + 1].excerpt_id != &region.excerpt_id;
                        let is_start = !text[region.range.start..ix].contains('\n');
                        let mut is_end = if region.range.end > text.len() {
                            !text[ix..].contains('\n')
                        } else {
                            text[ix..region.range.end.min(text.len())]
                                .matches('\n')
                                .count()
                                == 1
                        };
                        if region_ix < regions.len() - 1
                            && !text[ix..].contains("\n")
                            && region.status == Some(DiffHunkStatus::added_none())
                            && regions[region_ix + 1].excerpt_id == region.excerpt_id
                            && regions[region_ix + 1].range.start == text.len()
                        {
                            is_end = true;
                            is_excerpt_end = true;
                        }
                        let mut expand_direction = None;
                        if let Some(buffer) = &self
                            .excerpts
                            .iter()
                            .find(|e| e.id == region.excerpt_id.unwrap())
                            .map(|e| e.buffer.clone())
                        {
                            let needs_expand_up =
                                is_excerpt_start && is_start && buffer_row.unwrap() > 0;
                            let needs_expand_down = is_excerpt_end
                                && is_end
                                && buffer.read(cx).max_point().row > buffer_row.unwrap();
                            expand_direction = if needs_expand_up && needs_expand_down {
                                Some(ExpandExcerptDirection::UpAndDown)
                            } else if needs_expand_up {
                                Some(ExpandExcerptDirection::Up)
                            } else if needs_expand_down {
                                Some(ExpandExcerptDirection::Down)
                            } else {
                                None
                            };
                        }
                        RowInfo {
                            buffer_id: region.buffer_id,
                            diff_status: region.status,
                            buffer_row,
                            wrapped_buffer_row: None,

                            multibuffer_row: Some(MultiBufferRow(
                                text[..ix].matches('\n').count() as u32
                            )),
                            expand_info: expand_direction.zip(region.excerpt_id).map(
                                |(direction, excerpt_id)| ExpandInfo {
                                    direction,
                                    excerpt_id,
                                },
                            ),
                        }
                    });
                ix += line.len() + 1;
                row_info
            })
            .collect();

        (text, row_infos, excerpt_boundary_rows)
    }

    fn diffs_updated(&mut self, cx: &App) {
        for excerpt in &mut self.excerpts {
            let buffer = excerpt.buffer.read(cx).snapshot();
            let excerpt_range = excerpt.range.to_offset(&buffer);
            let buffer_id = buffer.remote_id();
            let diff = self.diffs.get(&buffer_id).unwrap().read(cx);
            let mut hunks = diff.hunks_in_row_range(0..u32::MAX, &buffer, cx).peekable();
            excerpt.expanded_diff_hunks.retain(|hunk_anchor| {
                if !hunk_anchor.is_valid(&buffer) {
                    return false;
                }
                while let Some(hunk) = hunks.peek() {
                    match hunk.buffer_range.start.cmp(hunk_anchor, &buffer) {
                        cmp::Ordering::Less => {
                            hunks.next();
                        }
                        cmp::Ordering::Equal => {
                            let hunk_range = hunk.buffer_range.to_offset(&buffer);
                            return hunk_range.end >= excerpt_range.start
                                && hunk_range.start <= excerpt_range.end;
                        }
                        cmp::Ordering::Greater => break,
                    }
                }
                false
            });
        }
    }

    fn add_diff(&mut self, diff: Entity<BufferDiff>, cx: &mut App) {
        let buffer_id = diff.read(cx).buffer_id;
        self.diffs.insert(buffer_id, diff);
    }
}

#[gpui::test(iterations = 100)]
async fn test_random_set_ranges(cx: &mut TestAppContext, mut rng: StdRng) {
    let base_text = "a\n".repeat(100);
    let buf = cx.update(|cx| cx.new(|cx| Buffer::local(base_text, cx)));
    let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));

    let operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(10);

    fn row_ranges(ranges: &Vec<Range<Point>>) -> Vec<Range<u32>> {
        ranges
            .iter()
            .map(|range| range.start.row..range.end.row)
            .collect()
    }

    for _ in 0..operations {
        let snapshot = buf.update(cx, |buf, _| buf.snapshot());
        let num_ranges = rng.random_range(0..=10);
        let max_row = snapshot.max_point().row;
        let mut ranges = (0..num_ranges)
            .map(|_| {
                let start = rng.random_range(0..max_row);
                let end = rng.random_range(start + 1..max_row + 1);
                Point::row_range(start..end)
            })
            .collect::<Vec<_>>();
        ranges.sort_by_key(|range| range.start);
        log::info!("Setting ranges: {:?}", row_ranges(&ranges));
        let (created, _) = multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.set_excerpts_for_path(
                PathKey::for_buffer(&buf, cx),
                buf.clone(),
                ranges.clone(),
                2,
                cx,
            )
        });

        assert_eq!(created.len(), ranges.len());

        let snapshot = multibuffer.update(cx, |multibuffer, cx| multibuffer.snapshot(cx));
        let mut last_end = None;
        let mut seen_ranges = Vec::default();

        for (_, buf, range) in snapshot.excerpts() {
            let start = range.context.start.to_point(buf);
            let end = range.context.end.to_point(buf);
            seen_ranges.push(start..end);

            if let Some(last_end) = last_end.take() {
                assert!(
                    start > last_end,
                    "multibuffer has out-of-order ranges: {:?}; {:?} <= {:?}",
                    row_ranges(&seen_ranges),
                    start,
                    last_end
                )
            }

            ranges.retain(|range| range.start < start || range.end > end);

            last_end = Some(end)
        }

        assert!(
            ranges.is_empty(),
            "multibuffer {:?} did not include all ranges: {:?}",
            row_ranges(&seen_ranges),
            row_ranges(&ranges)
        );
    }
}

#[gpui::test(iterations = 100)]
async fn test_random_multibuffer(cx: &mut TestAppContext, mut rng: StdRng) {
    let operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(10);

    let mut buffers: Vec<Entity<Buffer>> = Vec::new();
    let mut base_texts: HashMap<BufferId, String> = HashMap::default();
    let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
    let mut reference = ReferenceMultibuffer::default();
    let mut anchors = Vec::new();
    let mut old_versions = Vec::new();
    let mut needs_diff_calculation = false;

    for _ in 0..operations {
        match rng.random_range(0..100) {
            0..=14 if !buffers.is_empty() => {
                let buffer = buffers.choose(&mut rng).unwrap();
                buffer.update(cx, |buf, cx| {
                    let edit_count = rng.random_range(1..5);
                    buf.randomly_edit(&mut rng, edit_count, cx);
                    log::info!("buffer text:\n{}", buf.text());
                    needs_diff_calculation = true;
                });
                cx.update(|cx| reference.diffs_updated(cx));
            }
            15..=19 if !reference.excerpts.is_empty() => {
                multibuffer.update(cx, |multibuffer, cx| {
                    let ids = multibuffer.excerpt_ids();
                    let mut excerpts = HashSet::default();
                    for _ in 0..rng.random_range(0..ids.len()) {
                        excerpts.extend(ids.choose(&mut rng).copied());
                    }

                    let line_count = rng.random_range(0..5);

                    let excerpt_ixs = excerpts
                        .iter()
                        .map(|id| reference.excerpts.iter().position(|e| e.id == *id).unwrap())
                        .collect::<Vec<_>>();
                    log::info!("Expanding excerpts {excerpt_ixs:?} by {line_count} lines");
                    multibuffer.expand_excerpts(
                        excerpts.iter().cloned(),
                        line_count,
                        ExpandExcerptDirection::UpAndDown,
                        cx,
                    );

                    reference.expand_excerpts(&excerpts, line_count, cx);
                });
            }
            20..=29 if !reference.excerpts.is_empty() => {
                let mut ids_to_remove = vec![];
                for _ in 0..rng.random_range(1..=3) {
                    let Some(excerpt) = reference.excerpts.choose(&mut rng) else {
                        break;
                    };
                    let id = excerpt.id;
                    cx.update(|cx| reference.remove_excerpt(id, cx));
                    ids_to_remove.push(id);
                }
                let snapshot =
                    multibuffer.read_with(cx, |multibuffer, cx| multibuffer.snapshot(cx));
                ids_to_remove.sort_unstable_by(|a, b| a.cmp(b, &snapshot));
                drop(snapshot);
                multibuffer.update(cx, |multibuffer, cx| {
                    multibuffer.remove_excerpts(ids_to_remove, cx)
                });
            }
            30..=39 if !reference.excerpts.is_empty() => {
                let multibuffer =
                    multibuffer.read_with(cx, |multibuffer, cx| multibuffer.snapshot(cx));
                let offset =
                    multibuffer.clip_offset(rng.random_range(0..=multibuffer.len()), Bias::Left);
                let bias = if rng.random() {
                    Bias::Left
                } else {
                    Bias::Right
                };
                log::info!("Creating anchor at {} with bias {:?}", offset, bias);
                anchors.push(multibuffer.anchor_at(offset, bias));
                anchors.sort_by(|a, b| a.cmp(b, &multibuffer));
            }
            40..=44 if !anchors.is_empty() => {
                let multibuffer =
                    multibuffer.read_with(cx, |multibuffer, cx| multibuffer.snapshot(cx));
                let prev_len = anchors.len();
                anchors = multibuffer
                    .refresh_anchors(&anchors)
                    .into_iter()
                    .map(|a| a.1)
                    .collect();

                // Ensure the newly-refreshed anchors point to a valid excerpt and don't
                // overshoot its boundaries.
                assert_eq!(anchors.len(), prev_len);
                for anchor in &anchors {
                    if anchor.excerpt_id == ExcerptId::min()
                        || anchor.excerpt_id == ExcerptId::max()
                    {
                        continue;
                    }

                    let excerpt = multibuffer.excerpt(anchor.excerpt_id).unwrap();
                    assert_eq!(excerpt.id, anchor.excerpt_id);
                    assert!(excerpt.contains(anchor));
                }
            }
            45..=55 if !reference.excerpts.is_empty() => {
                multibuffer.update(cx, |multibuffer, cx| {
                    let snapshot = multibuffer.snapshot(cx);
                    let excerpt_ix = rng.random_range(0..reference.excerpts.len());
                    let excerpt = &reference.excerpts[excerpt_ix];
                    let start = excerpt.range.start;
                    let end = excerpt.range.end;
                    let range = snapshot.anchor_in_excerpt(excerpt.id, start).unwrap()
                        ..snapshot.anchor_in_excerpt(excerpt.id, end).unwrap();

                    log::info!(
                        "expanding diff hunks in range {:?} (excerpt id {:?}, index {excerpt_ix:?}, buffer id {:?})",
                        range.to_offset(&snapshot),
                        excerpt.id,
                        excerpt.buffer.read(cx).remote_id(),
                    );
                    reference.expand_diff_hunks(excerpt.id, start..end, cx);
                    multibuffer.expand_diff_hunks(vec![range], cx);
                });
            }
            56..=85 if needs_diff_calculation => {
                multibuffer.update(cx, |multibuffer, cx| {
                    for buffer in multibuffer.all_buffers() {
                        let snapshot = buffer.read(cx).snapshot();
                        multibuffer.diff_for(snapshot.remote_id()).unwrap().update(
                            cx,
                            |diff, cx| {
                                log::info!(
                                    "recalculating diff for buffer {:?}",
                                    snapshot.remote_id(),
                                );
                                diff.recalculate_diff_sync(snapshot.text, cx);
                            },
                        );
                    }
                    reference.diffs_updated(cx);
                    needs_diff_calculation = false;
                });
            }
            _ => {
                let buffer_handle = if buffers.is_empty() || rng.random_bool(0.4) {
                    let mut base_text = util::RandomCharIter::new(&mut rng)
                        .take(256)
                        .collect::<String>();

                    let buffer = cx.new(|cx| Buffer::local(base_text.clone(), cx));
                    text::LineEnding::normalize(&mut base_text);
                    base_texts.insert(
                        buffer.read_with(cx, |buffer, _| buffer.remote_id()),
                        base_text,
                    );
                    buffers.push(buffer);
                    buffers.last().unwrap()
                } else {
                    buffers.choose(&mut rng).unwrap()
                };

                let prev_excerpt_ix = rng.random_range(0..=reference.excerpts.len());
                let prev_excerpt_id = reference
                    .excerpts
                    .get(prev_excerpt_ix)
                    .map_or(ExcerptId::max(), |e| e.id);
                let excerpt_ix = (prev_excerpt_ix + 1).min(reference.excerpts.len());

                let (range, anchor_range) = buffer_handle.read_with(cx, |buffer, _| {
                    let end_row = rng.random_range(0..=buffer.max_point().row);
                    let start_row = rng.random_range(0..=end_row);
                    let end_ix = buffer.point_to_offset(Point::new(end_row, 0));
                    let start_ix = buffer.point_to_offset(Point::new(start_row, 0));
                    let anchor_range = buffer.anchor_before(start_ix)..buffer.anchor_after(end_ix);

                    log::info!(
                        "Inserting excerpt at {} of {} for buffer {}: {:?}[{:?}] = {:?}",
                        excerpt_ix,
                        reference.excerpts.len(),
                        buffer.remote_id(),
                        buffer.text(),
                        start_ix..end_ix,
                        &buffer.text()[start_ix..end_ix]
                    );

                    (start_ix..end_ix, anchor_range)
                });

                multibuffer.update(cx, |multibuffer, cx| {
                    let id = buffer_handle.read(cx).remote_id();
                    if multibuffer.diff_for(id).is_none() {
                        let base_text = base_texts.get(&id).unwrap();
                        let diff = cx
                            .new(|cx| BufferDiff::new_with_base_text(base_text, buffer_handle, cx));
                        reference.add_diff(diff.clone(), cx);
                        multibuffer.add_diff(diff, cx)
                    }
                });

                let excerpt_id = multibuffer.update(cx, |multibuffer, cx| {
                    multibuffer
                        .insert_excerpts_after(
                            prev_excerpt_id,
                            buffer_handle.clone(),
                            [ExcerptRange::new(range.clone())],
                            cx,
                        )
                        .pop()
                        .unwrap()
                });

                reference.insert_excerpt_after(
                    prev_excerpt_id,
                    excerpt_id,
                    (buffer_handle.clone(), anchor_range),
                );
            }
        }

        if rng.random_bool(0.3) {
            multibuffer.update(cx, |multibuffer, cx| {
                old_versions.push((multibuffer.snapshot(cx), multibuffer.subscribe()));
            })
        }

        let snapshot = multibuffer.read_with(cx, |multibuffer, cx| multibuffer.snapshot(cx));
        let actual_text = snapshot.text();
        let actual_boundary_rows = snapshot
            .excerpt_boundaries_in_range(0..)
            .map(|b| b.row)
            .collect::<HashSet<_>>();
        let actual_row_infos = snapshot.row_infos(MultiBufferRow(0)).collect::<Vec<_>>();

        let (expected_text, expected_row_infos, expected_boundary_rows) =
            cx.update(|cx| reference.expected_content(cx));

        let has_diff = actual_row_infos
            .iter()
            .any(|info| info.diff_status.is_some())
            || expected_row_infos
                .iter()
                .any(|info| info.diff_status.is_some());
        let actual_diff = format_diff(
            &actual_text,
            &actual_row_infos,
            &actual_boundary_rows,
            Some(has_diff),
        );
        let expected_diff = format_diff(
            &expected_text,
            &expected_row_infos,
            &expected_boundary_rows,
            Some(has_diff),
        );

        log::info!("Multibuffer content:\n{}", actual_diff);

        assert_eq!(
            actual_row_infos.len(),
            actual_text.split('\n').count(),
            "line count: {}",
            actual_text.split('\n').count()
        );
        pretty_assertions::assert_eq!(actual_diff, expected_diff);
        pretty_assertions::assert_eq!(actual_text, expected_text);
        pretty_assertions::assert_eq!(actual_row_infos, expected_row_infos);

        for _ in 0..5 {
            let start_row = rng.random_range(0..=expected_row_infos.len());
            assert_eq!(
                snapshot
                    .row_infos(MultiBufferRow(start_row as u32))
                    .collect::<Vec<_>>(),
                &expected_row_infos[start_row..],
                "buffer_rows({})",
                start_row
            );
        }

        assert_eq!(
            snapshot.widest_line_number(),
            expected_row_infos
                .into_iter()
                .filter_map(|info| {
                    if info.diff_status.is_some_and(|status| status.is_deleted()) {
                        None
                    } else {
                        info.buffer_row
                    }
                })
                .max()
                .unwrap()
                + 1
        );
        let reference_ranges = cx.update(|cx| {
            reference
                .excerpts
                .iter()
                .map(|excerpt| {
                    (
                        excerpt.id,
                        excerpt.range.to_offset(&excerpt.buffer.read(cx).snapshot()),
                    )
                })
                .collect::<HashMap<_, _>>()
        });
        for i in 0..snapshot.len() {
            let excerpt = snapshot.excerpt_containing(i..i).unwrap();
            assert_eq!(excerpt.buffer_range(), reference_ranges[&excerpt.id()]);
        }

        assert_consistent_line_numbers(&snapshot);
        assert_position_translation(&snapshot);

        for (row, line) in expected_text.split('\n').enumerate() {
            assert_eq!(
                snapshot.line_len(MultiBufferRow(row as u32)),
                line.len() as u32,
                "line_len({}).",
                row
            );
        }

        let text_rope = Rope::from(expected_text.as_str());
        for _ in 0..10 {
            let end_ix = text_rope.clip_offset(rng.random_range(0..=text_rope.len()), Bias::Right);
            let start_ix = text_rope.clip_offset(rng.random_range(0..=end_ix), Bias::Left);

            let text_for_range = snapshot
                .text_for_range(start_ix..end_ix)
                .collect::<String>();
            assert_eq!(
                text_for_range,
                &expected_text[start_ix..end_ix],
                "incorrect text for range {:?}",
                start_ix..end_ix
            );

            let expected_summary = TextSummary::from(&expected_text[start_ix..end_ix]);
            assert_eq!(
                snapshot.text_summary_for_range::<TextSummary, _>(start_ix..end_ix),
                expected_summary,
                "incorrect summary for range {:?}",
                start_ix..end_ix
            );
        }

        // Anchor resolution
        let summaries = snapshot.summaries_for_anchors::<usize, _>(&anchors);
        assert_eq!(anchors.len(), summaries.len());
        for (anchor, resolved_offset) in anchors.iter().zip(summaries) {
            assert!(resolved_offset <= snapshot.len());
            assert_eq!(
                snapshot.summary_for_anchor::<usize>(anchor),
                resolved_offset,
                "anchor: {:?}",
                anchor
            );
        }

        for _ in 0..10 {
            let end_ix = text_rope.clip_offset(rng.random_range(0..=text_rope.len()), Bias::Right);
            assert_eq!(
                snapshot.reversed_chars_at(end_ix).collect::<String>(),
                expected_text[..end_ix].chars().rev().collect::<String>(),
            );
        }

        for _ in 0..10 {
            let end_ix = rng.random_range(0..=text_rope.len());
            let start_ix = rng.random_range(0..=end_ix);
            assert_eq!(
                snapshot
                    .bytes_in_range(start_ix..end_ix)
                    .flatten()
                    .copied()
                    .collect::<Vec<_>>(),
                expected_text.as_bytes()[start_ix..end_ix].to_vec(),
                "bytes_in_range({:?})",
                start_ix..end_ix,
            );
        }
    }

    let snapshot = multibuffer.read_with(cx, |multibuffer, cx| multibuffer.snapshot(cx));
    for (old_snapshot, subscription) in old_versions {
        let edits = subscription.consume().into_inner();

        log::info!(
            "applying subscription edits to old text: {:?}: {:?}",
            old_snapshot.text(),
            edits,
        );

        let mut text = old_snapshot.text();
        for edit in edits {
            let new_text: String = snapshot.text_for_range(edit.new.clone()).collect();
            text.replace_range(edit.new.start..edit.new.start + edit.old.len(), &new_text);
        }
        assert_eq!(text.to_string(), snapshot.text());
    }
}

#[gpui::test]
fn test_history(cx: &mut App) {
    let test_settings = SettingsStore::test(cx);
    cx.set_global(test_settings);
    let group_interval: Duration = Duration::from_millis(1);
    let buffer_1 = cx.new(|cx| {
        let mut buf = Buffer::local("1234", cx);
        buf.set_group_interval(group_interval);
        buf
    });
    let buffer_2 = cx.new(|cx| {
        let mut buf = Buffer::local("5678", cx);
        buf.set_group_interval(group_interval);
        buf
    });
    let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
    multibuffer.update(cx, |this, _| {
        this.set_group_interval(group_interval);
    });
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange::new(0..buffer_1.read(cx).len())],
            cx,
        );
        multibuffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange::new(0..buffer_2.read(cx).len())],
            cx,
        );
    });

    let mut now = Instant::now();

    multibuffer.update(cx, |multibuffer, cx| {
        let transaction_1 = multibuffer.start_transaction_at(now, cx).unwrap();
        multibuffer.edit(
            [
                (Point::new(0, 0)..Point::new(0, 0), "A"),
                (Point::new(1, 0)..Point::new(1, 0), "A"),
            ],
            None,
            cx,
        );
        multibuffer.edit(
            [
                (Point::new(0, 1)..Point::new(0, 1), "B"),
                (Point::new(1, 1)..Point::new(1, 1), "B"),
            ],
            None,
            cx,
        );
        multibuffer.end_transaction_at(now, cx);
        assert_eq!(multibuffer.read(cx).text(), "AB1234\nAB5678");

        // Verify edited ranges for transaction 1
        assert_eq!(
            multibuffer.edited_ranges_for_transaction(transaction_1, cx),
            &[
                Point::new(0, 0)..Point::new(0, 2),
                Point::new(1, 0)..Point::new(1, 2)
            ]
        );

        // Edit buffer 1 through the multibuffer
        now += 2 * group_interval;
        multibuffer.start_transaction_at(now, cx);
        multibuffer.edit([(2..2, "C")], None, cx);
        multibuffer.end_transaction_at(now, cx);
        assert_eq!(multibuffer.read(cx).text(), "ABC1234\nAB5678");

        // Edit buffer 1 independently
        buffer_1.update(cx, |buffer_1, cx| {
            buffer_1.start_transaction_at(now);
            buffer_1.edit([(3..3, "D")], None, cx);
            buffer_1.end_transaction_at(now, cx);

            now += 2 * group_interval;
            buffer_1.start_transaction_at(now);
            buffer_1.edit([(4..4, "E")], None, cx);
            buffer_1.end_transaction_at(now, cx);
        });
        assert_eq!(multibuffer.read(cx).text(), "ABCDE1234\nAB5678");

        // An undo in the multibuffer undoes the multibuffer transaction
        // and also any individual buffer edits that have occurred since
        // that transaction.
        multibuffer.undo(cx);
        assert_eq!(multibuffer.read(cx).text(), "AB1234\nAB5678");

        multibuffer.undo(cx);
        assert_eq!(multibuffer.read(cx).text(), "1234\n5678");

        multibuffer.redo(cx);
        assert_eq!(multibuffer.read(cx).text(), "AB1234\nAB5678");

        multibuffer.redo(cx);
        assert_eq!(multibuffer.read(cx).text(), "ABCDE1234\nAB5678");

        // Undo buffer 2 independently.
        buffer_2.update(cx, |buffer_2, cx| buffer_2.undo(cx));
        assert_eq!(multibuffer.read(cx).text(), "ABCDE1234\n5678");

        // An undo in the multibuffer undoes the components of the
        // the last multibuffer transaction that are not already undone.
        multibuffer.undo(cx);
        assert_eq!(multibuffer.read(cx).text(), "AB1234\n5678");

        multibuffer.undo(cx);
        assert_eq!(multibuffer.read(cx).text(), "1234\n5678");

        multibuffer.redo(cx);
        assert_eq!(multibuffer.read(cx).text(), "AB1234\nAB5678");

        buffer_1.update(cx, |buffer_1, cx| buffer_1.redo(cx));
        assert_eq!(multibuffer.read(cx).text(), "ABCD1234\nAB5678");

        // Redo stack gets cleared after an edit.
        now += 2 * group_interval;
        multibuffer.start_transaction_at(now, cx);
        multibuffer.edit([(0..0, "X")], None, cx);
        multibuffer.end_transaction_at(now, cx);
        assert_eq!(multibuffer.read(cx).text(), "XABCD1234\nAB5678");
        multibuffer.redo(cx);
        assert_eq!(multibuffer.read(cx).text(), "XABCD1234\nAB5678");
        multibuffer.undo(cx);
        assert_eq!(multibuffer.read(cx).text(), "ABCD1234\nAB5678");
        multibuffer.undo(cx);
        assert_eq!(multibuffer.read(cx).text(), "1234\n5678");

        // Transactions can be grouped manually.
        multibuffer.redo(cx);
        multibuffer.redo(cx);
        assert_eq!(multibuffer.read(cx).text(), "XABCD1234\nAB5678");
        multibuffer.group_until_transaction(transaction_1, cx);
        multibuffer.undo(cx);
        assert_eq!(multibuffer.read(cx).text(), "1234\n5678");
        multibuffer.redo(cx);
        assert_eq!(multibuffer.read(cx).text(), "XABCD1234\nAB5678");
    });
}

#[gpui::test]
async fn test_enclosing_indent(cx: &mut TestAppContext) {
    async fn enclosing_indent(
        text: &str,
        buffer_row: u32,
        cx: &mut TestAppContext,
    ) -> Option<(Range<u32>, LineIndent)> {
        let buffer = cx.update(|cx| MultiBuffer::build_simple(text, cx));
        let snapshot = cx.read(|cx| buffer.read(cx).snapshot(cx));
        let (range, indent) = snapshot
            .enclosing_indent(MultiBufferRow(buffer_row))
            .await?;
        Some((range.start.0..range.end.0, indent))
    }

    assert_eq!(
        enclosing_indent(
            indoc!(
                "
                fn b() {
                    if c {
                        let d = 2;
                    }
                }
                "
            ),
            1,
            cx,
        )
        .await,
        Some((
            1..2,
            LineIndent {
                tabs: 0,
                spaces: 4,
                line_blank: false,
            }
        ))
    );

    assert_eq!(
        enclosing_indent(
            indoc!(
                "
                fn b() {
                    if c {
                        let d = 2;
                    }
                }
                "
            ),
            2,
            cx,
        )
        .await,
        Some((
            1..2,
            LineIndent {
                tabs: 0,
                spaces: 4,
                line_blank: false,
            }
        ))
    );

    assert_eq!(
        enclosing_indent(
            indoc!(
                "
                fn b() {
                    if c {
                        let d = 2;

                        let e = 5;
                    }
                }
                "
            ),
            3,
            cx,
        )
        .await,
        Some((
            1..4,
            LineIndent {
                tabs: 0,
                spaces: 4,
                line_blank: false,
            }
        ))
    );
}

#[gpui::test]
fn test_summaries_for_anchors(cx: &mut TestAppContext) {
    let base_text_1 = indoc!(
        "
        bar
        "
    );
    let text_1 = indoc!(
        "
        BAR
        "
    );
    let base_text_2 = indoc!(
        "
        foo
        "
    );
    let text_2 = indoc!(
        "
        FOO
        "
    );

    let buffer_1 = cx.new(|cx| Buffer::local(text_1, cx));
    let buffer_2 = cx.new(|cx| Buffer::local(text_2, cx));
    let diff_1 = cx.new(|cx| BufferDiff::new_with_base_text(base_text_1, &buffer_1, cx));
    let diff_2 = cx.new(|cx| BufferDiff::new_with_base_text(base_text_2, &buffer_2, cx));
    cx.run_until_parked();

    let mut ids = vec![];
    let multibuffer = cx.new(|cx| {
        let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
        multibuffer.set_all_diff_hunks_expanded(cx);
        ids.extend(multibuffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange::new(text::Anchor::MIN..text::Anchor::MAX)],
            cx,
        ));
        ids.extend(multibuffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange::new(text::Anchor::MIN..text::Anchor::MAX)],
            cx,
        ));
        multibuffer.add_diff(diff_1.clone(), cx);
        multibuffer.add_diff(diff_2.clone(), cx);
        multibuffer
    });

    let (mut snapshot, mut subscription) = multibuffer.update(cx, |multibuffer, cx| {
        (multibuffer.snapshot(cx), multibuffer.subscribe())
    });

    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc!(
            "
            - bar
            + BAR

            - foo
            + FOO
            "
        ),
    );

    let id_1 = buffer_1.read_with(cx, |buffer, _| buffer.remote_id());
    let id_2 = buffer_2.read_with(cx, |buffer, _| buffer.remote_id());

    let anchor_1 = Anchor::in_buffer(ids[0], id_1, text::Anchor::MIN);
    let point_1 = snapshot.summaries_for_anchors::<Point, _>([&anchor_1])[0];
    assert_eq!(point_1, Point::new(0, 0));

    let anchor_2 = Anchor::in_buffer(ids[1], id_2, text::Anchor::MIN);
    let point_2 = snapshot.summaries_for_anchors::<Point, _>([&anchor_2])[0];
    assert_eq!(point_2, Point::new(3, 0));
}

#[gpui::test]
fn test_trailing_deletion_without_newline(cx: &mut TestAppContext) {
    let base_text_1 = "one\ntwo".to_owned();
    let text_1 = "one\n".to_owned();

    let buffer_1 = cx.new(|cx| Buffer::local(text_1, cx));
    let diff_1 = cx.new(|cx| BufferDiff::new_with_base_text(&base_text_1, &buffer_1, cx));
    cx.run_until_parked();

    let multibuffer = cx.new(|cx| {
        let mut multibuffer = MultiBuffer::singleton(buffer_1.clone(), cx);
        multibuffer.add_diff(diff_1.clone(), cx);
        multibuffer.expand_diff_hunks(vec![Anchor::min()..Anchor::max()], cx);
        multibuffer
    });

    let (mut snapshot, mut subscription) = multibuffer.update(cx, |multibuffer, cx| {
        (multibuffer.snapshot(cx), multibuffer.subscribe())
    });

    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc!(
            "
              one
            - two
            "
        ),
    );

    assert_eq!(snapshot.max_point(), Point::new(2, 0));
    assert_eq!(snapshot.len(), 8);

    assert_eq!(
        snapshot
            .dimensions_from_points::<Point>([Point::new(2, 0)])
            .collect::<Vec<_>>(),
        vec![Point::new(2, 0)]
    );

    let (_, translated_offset) = snapshot.point_to_buffer_offset(Point::new(2, 0)).unwrap();
    assert_eq!(translated_offset, "one\n".len());
    let (_, translated_point, _) = snapshot.point_to_buffer_point(Point::new(2, 0)).unwrap();
    assert_eq!(translated_point, Point::new(1, 0));

    // The same, for an excerpt that's not at the end of the multibuffer.

    let text_2 = "foo\n".to_owned();
    let buffer_2 = cx.new(|cx| Buffer::local(&text_2, cx));
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange::new(Point::new(0, 0)..Point::new(1, 0))],
            cx,
        );
    });

    assert_new_snapshot(
        &multibuffer,
        &mut snapshot,
        &mut subscription,
        cx,
        indoc!(
            "
              one
            - two

              foo
            "
        ),
    );

    assert_eq!(
        snapshot
            .dimensions_from_points::<Point>([Point::new(2, 0)])
            .collect::<Vec<_>>(),
        vec![Point::new(2, 0)]
    );

    let buffer_1_id = buffer_1.read_with(cx, |buffer_1, _| buffer_1.remote_id());
    let (buffer, translated_offset) = snapshot.point_to_buffer_offset(Point::new(2, 0)).unwrap();
    assert_eq!(buffer.remote_id(), buffer_1_id);
    assert_eq!(translated_offset, "one\n".len());
    let (buffer, translated_point, _) = snapshot.point_to_buffer_point(Point::new(2, 0)).unwrap();
    assert_eq!(buffer.remote_id(), buffer_1_id);
    assert_eq!(translated_point, Point::new(1, 0));
}

fn format_diff(
    text: &str,
    row_infos: &Vec<RowInfo>,
    boundary_rows: &HashSet<MultiBufferRow>,
    has_diff: Option<bool>,
) -> String {
    let has_diff =
        has_diff.unwrap_or_else(|| row_infos.iter().any(|info| info.diff_status.is_some()));
    text.split('\n')
        .enumerate()
        .zip(row_infos)
        .map(|((ix, line), info)| {
            let marker = match info.diff_status.map(|status| status.kind) {
                Some(DiffHunkStatusKind::Added) => "+ ",
                Some(DiffHunkStatusKind::Deleted) => "- ",
                Some(DiffHunkStatusKind::Modified) => unreachable!(),
                None => {
                    if has_diff && !line.is_empty() {
                        "  "
                    } else {
                        ""
                    }
                }
            };
            let boundary_row = if boundary_rows.contains(&MultiBufferRow(ix as u32)) {
                if has_diff {
                    "  ----------\n"
                } else {
                    "---------\n"
                }
            } else {
                ""
            };
            format!("{boundary_row}{marker}{line}")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[track_caller]
fn assert_excerpts_match(
    multibuffer: &Entity<MultiBuffer>,
    cx: &mut TestAppContext,
    expected: &str,
) {
    let mut output = String::new();
    multibuffer.read_with(cx, |multibuffer, cx| {
        for (_, buffer, range) in multibuffer.snapshot(cx).excerpts() {
            output.push_str("-----\n");
            output.extend(buffer.text_for_range(range.context));
            if !output.ends_with('\n') {
                output.push('\n');
            }
        }
    });
    assert_eq!(output, expected);
}

#[track_caller]
fn assert_new_snapshot(
    multibuffer: &Entity<MultiBuffer>,
    snapshot: &mut MultiBufferSnapshot,
    subscription: &mut Subscription,
    cx: &mut TestAppContext,
    expected_diff: &str,
) {
    let new_snapshot = multibuffer.read_with(cx, |multibuffer, cx| multibuffer.snapshot(cx));
    let actual_text = new_snapshot.text();
    let line_infos = new_snapshot
        .row_infos(MultiBufferRow(0))
        .collect::<Vec<_>>();
    let actual_diff = format_diff(&actual_text, &line_infos, &Default::default(), None);
    pretty_assertions::assert_eq!(actual_diff, expected_diff);
    check_edits(
        snapshot,
        &new_snapshot,
        &subscription.consume().into_inner(),
    );
    *snapshot = new_snapshot;
}

#[track_caller]
fn check_edits(
    old_snapshot: &MultiBufferSnapshot,
    new_snapshot: &MultiBufferSnapshot,
    edits: &[Edit<usize>],
) {
    let mut text = old_snapshot.text();
    let new_text = new_snapshot.text();
    for edit in edits.iter().rev() {
        if !text.is_char_boundary(edit.old.start)
            || !text.is_char_boundary(edit.old.end)
            || !new_text.is_char_boundary(edit.new.start)
            || !new_text.is_char_boundary(edit.new.end)
        {
            panic!(
                "invalid edits: {:?}\nold text: {:?}\nnew text: {:?}",
                edits, text, new_text
            );
        }

        text.replace_range(
            edit.old.start..edit.old.end,
            &new_text[edit.new.start..edit.new.end],
        );
    }

    pretty_assertions::assert_eq!(text, new_text, "invalid edits: {:?}", edits);
}

#[track_caller]
fn assert_chunks_in_ranges(snapshot: &MultiBufferSnapshot) {
    let full_text = snapshot.text();
    for ix in 0..full_text.len() {
        let mut chunks = snapshot.chunks(0..snapshot.len(), false);
        chunks.seek(ix..snapshot.len());
        let tail = chunks.map(|chunk| chunk.text).collect::<String>();
        assert_eq!(tail, &full_text[ix..], "seek to range: {:?}", ix..);
    }
}

#[track_caller]
fn assert_consistent_line_numbers(snapshot: &MultiBufferSnapshot) {
    let all_line_numbers = snapshot.row_infos(MultiBufferRow(0)).collect::<Vec<_>>();
    for start_row in 1..all_line_numbers.len() {
        let line_numbers = snapshot
            .row_infos(MultiBufferRow(start_row as u32))
            .collect::<Vec<_>>();
        assert_eq!(
            line_numbers,
            all_line_numbers[start_row..],
            "start_row: {start_row}"
        );
    }
}

#[track_caller]
fn assert_position_translation(snapshot: &MultiBufferSnapshot) {
    let text = Rope::from(snapshot.text());

    let mut left_anchors = Vec::new();
    let mut right_anchors = Vec::new();
    let mut offsets = Vec::new();
    let mut points = Vec::new();
    for offset in 0..=text.len() + 1 {
        let clipped_left = snapshot.clip_offset(offset, Bias::Left);
        let clipped_right = snapshot.clip_offset(offset, Bias::Right);
        assert_eq!(
            clipped_left,
            text.clip_offset(offset, Bias::Left),
            "clip_offset({offset:?}, Left)"
        );
        assert_eq!(
            clipped_right,
            text.clip_offset(offset, Bias::Right),
            "clip_offset({offset:?}, Right)"
        );
        assert_eq!(
            snapshot.offset_to_point(clipped_left),
            text.offset_to_point(clipped_left),
            "offset_to_point({clipped_left})"
        );
        assert_eq!(
            snapshot.offset_to_point(clipped_right),
            text.offset_to_point(clipped_right),
            "offset_to_point({clipped_right})"
        );
        let anchor_after = snapshot.anchor_after(clipped_left);
        assert_eq!(
            anchor_after.to_offset(snapshot),
            clipped_left,
            "anchor_after({clipped_left}).to_offset {anchor_after:?}"
        );
        let anchor_before = snapshot.anchor_before(clipped_left);
        assert_eq!(
            anchor_before.to_offset(snapshot),
            clipped_left,
            "anchor_before({clipped_left}).to_offset"
        );
        left_anchors.push(anchor_before);
        right_anchors.push(anchor_after);
        offsets.push(clipped_left);
        points.push(text.offset_to_point(clipped_left));
    }

    for row in 0..text.max_point().row {
        for column in 0..text.line_len(row) + 1 {
            let point = Point { row, column };
            let clipped_left = snapshot.clip_point(point, Bias::Left);
            let clipped_right = snapshot.clip_point(point, Bias::Right);
            assert_eq!(
                clipped_left,
                text.clip_point(point, Bias::Left),
                "clip_point({point:?}, Left)"
            );
            assert_eq!(
                clipped_right,
                text.clip_point(point, Bias::Right),
                "clip_point({point:?}, Right)"
            );
            assert_eq!(
                snapshot.point_to_offset(clipped_left),
                text.point_to_offset(clipped_left),
                "point_to_offset({clipped_left:?})"
            );
            assert_eq!(
                snapshot.point_to_offset(clipped_right),
                text.point_to_offset(clipped_right),
                "point_to_offset({clipped_right:?})"
            );
        }
    }

    assert_eq!(
        snapshot.summaries_for_anchors::<usize, _>(&left_anchors),
        offsets,
        "left_anchors <-> offsets"
    );
    assert_eq!(
        snapshot.summaries_for_anchors::<Point, _>(&left_anchors),
        points,
        "left_anchors <-> points"
    );
    assert_eq!(
        snapshot.summaries_for_anchors::<usize, _>(&right_anchors),
        offsets,
        "right_anchors <-> offsets"
    );
    assert_eq!(
        snapshot.summaries_for_anchors::<Point, _>(&right_anchors),
        points,
        "right_anchors <-> points"
    );

    for (anchors, bias) in [(&left_anchors, Bias::Left), (&right_anchors, Bias::Right)] {
        for (ix, (offset, anchor)) in offsets.iter().zip(anchors).enumerate() {
            if ix > 0 && *offset == 252 && offset > &offsets[ix - 1] {
                let prev_anchor = left_anchors[ix - 1];
                assert!(
                    anchor.cmp(&prev_anchor, snapshot).is_gt(),
                    "anchor({}, {bias:?}).cmp(&anchor({}, {bias:?}).is_gt()",
                    offsets[ix],
                    offsets[ix - 1],
                );
                assert!(
                    prev_anchor.cmp(anchor, snapshot).is_lt(),
                    "anchor({}, {bias:?}).cmp(&anchor({}, {bias:?}).is_lt()",
                    offsets[ix - 1],
                    offsets[ix],
                );
            }
        }
    }

    if let Some((buffer, offset)) = snapshot.point_to_buffer_offset(snapshot.max_point()) {
        assert!(offset <= buffer.len());
    }
    if let Some((buffer, point, _)) = snapshot.point_to_buffer_point(snapshot.max_point()) {
        assert!(point <= buffer.max_point());
    }
}

fn assert_line_indents(snapshot: &MultiBufferSnapshot) {
    let max_row = snapshot.max_point().row;
    let buffer_id = snapshot.excerpts().next().unwrap().1.remote_id();
    let text = text::Buffer::new(ReplicaId::LOCAL, buffer_id, snapshot.text());
    let mut line_indents = text
        .line_indents_in_row_range(0..max_row + 1)
        .collect::<Vec<_>>();
    for start_row in 0..snapshot.max_point().row {
        pretty_assertions::assert_eq!(
            snapshot
                .line_indents(MultiBufferRow(start_row), |_| true)
                .map(|(row, indent, _)| (row.0, indent))
                .collect::<Vec<_>>(),
            &line_indents[(start_row as usize)..],
            "line_indents({start_row})"
        );
    }

    line_indents.reverse();
    pretty_assertions::assert_eq!(
        snapshot
            .reversed_line_indents(MultiBufferRow(max_row), |_| true)
            .map(|(row, indent, _)| (row.0, indent))
            .collect::<Vec<_>>(),
        &line_indents[..],
        "reversed_line_indents({max_row})"
    );
}

#[gpui::test]
fn test_new_empty_buffer_uses_untitled_title(cx: &mut App) {
    let buffer = cx.new(|cx| Buffer::local("", cx));
    let multibuffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

    assert_eq!(multibuffer.read(cx).title(cx), "untitled");
}

#[gpui::test]
fn test_new_empty_buffer_uses_untitled_title_when_only_contains_whitespace(cx: &mut App) {
    let buffer = cx.new(|cx| Buffer::local("\n ", cx));
    let multibuffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

    assert_eq!(multibuffer.read(cx).title(cx), "untitled");
}

#[gpui::test]
fn test_new_empty_buffer_takes_first_line_for_title(cx: &mut App) {
    let buffer = cx.new(|cx| Buffer::local("Hello World\nSecond line", cx));
    let multibuffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

    assert_eq!(multibuffer.read(cx).title(cx), "Hello World");
}

#[gpui::test]
fn test_new_empty_buffer_takes_trimmed_first_line_for_title(cx: &mut App) {
    let buffer = cx.new(|cx| Buffer::local("\nHello, World ", cx));
    let multibuffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

    assert_eq!(multibuffer.read(cx).title(cx), "Hello, World");
}

#[gpui::test]
fn test_new_empty_buffer_uses_truncated_first_line_for_title(cx: &mut App) {
    let title = "aaaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeee";
    let title_after = "aaaaaaaaaabbbbbbbbbbccccccccccdddddddddd";
    let buffer = cx.new(|cx| Buffer::local(title, cx));
    let multibuffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

    assert_eq!(multibuffer.read(cx).title(cx), title_after);
}

#[gpui::test]
fn test_new_empty_buffer_uses_truncated_first_line_for_title_after_merging_adjacent_spaces(
    cx: &mut App,
) {
    let title = "aaaaaaaaaabbbbbbbbbb    ccccccccccddddddddddeeeeeeeeee";
    let title_after = "aaaaaaaaaabbbbbbbbbb ccccccccccddddddddd";
    let buffer = cx.new(|cx| Buffer::local(title, cx));
    let multibuffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

    assert_eq!(multibuffer.read(cx).title(cx), title_after);
}

#[gpui::test]
fn test_new_empty_buffers_title_can_be_set(cx: &mut App) {
    let buffer = cx.new(|cx| Buffer::local("Hello World", cx));
    let multibuffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));
    assert_eq!(multibuffer.read(cx).title(cx), "Hello World");

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.set_title("Hey".into(), cx)
    });
    assert_eq!(multibuffer.read(cx).title(cx), "Hey");
}

#[gpui::test(iterations = 100)]
fn test_random_chunk_bitmaps(cx: &mut App, mut rng: StdRng) {
    let multibuffer = if rng.random() {
        let len = rng.random_range(0..10000);
        let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
        let buffer = cx.new(|cx| Buffer::local(text, cx));
        cx.new(|cx| MultiBuffer::singleton(buffer, cx))
    } else {
        MultiBuffer::build_random(&mut rng, cx)
    };

    let snapshot = multibuffer.read(cx).snapshot(cx);

    let chunks = snapshot.chunks(0..snapshot.len(), false);

    for chunk in chunks {
        let chunk_text = chunk.text;
        let chars_bitmap = chunk.chars;
        let tabs_bitmap = chunk.tabs;

        if chunk_text.is_empty() {
            assert_eq!(
                chars_bitmap, 0,
                "Empty chunk should have empty chars bitmap"
            );
            assert_eq!(tabs_bitmap, 0, "Empty chunk should have empty tabs bitmap");
            continue;
        }

        assert!(
            chunk_text.len() <= 128,
            "Chunk text length {} exceeds 128 bytes",
            chunk_text.len()
        );

        // Verify chars bitmap
        let char_indices = chunk_text
            .char_indices()
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        for byte_idx in 0..chunk_text.len() {
            let should_have_bit = char_indices.contains(&byte_idx);
            let has_bit = chars_bitmap & (1 << byte_idx) != 0;

            if has_bit != should_have_bit {
                eprintln!("Chunk text bytes: {:?}", chunk_text.as_bytes());
                eprintln!("Char indices: {:?}", char_indices);
                eprintln!("Chars bitmap: {:#b}", chars_bitmap);
            }

            assert_eq!(
                has_bit, should_have_bit,
                "Chars bitmap mismatch at byte index {} in chunk {:?}. Expected bit: {}, Got bit: {}",
                byte_idx, chunk_text, should_have_bit, has_bit
            );
        }

        for (byte_idx, byte) in chunk_text.bytes().enumerate() {
            let is_tab = byte == b'\t';
            let has_bit = tabs_bitmap & (1 << byte_idx) != 0;

            if has_bit != is_tab {
                eprintln!("Chunk text bytes: {:?}", chunk_text.as_bytes());
                eprintln!("Tabs bitmap: {:#b}", tabs_bitmap);
                assert_eq!(
                    has_bit, is_tab,
                    "Tabs bitmap mismatch at byte index {} in chunk {:?}. Byte: {:?}, Expected bit: {}, Got bit: {}",
                    byte_idx, chunk_text, byte as char, is_tab, has_bit
                );
            }
        }
    }
}

#[gpui::test(iterations = 100)]
fn test_random_chunk_bitmaps_with_diffs(cx: &mut App, mut rng: StdRng) {
    use buffer_diff::BufferDiff;
    use util::RandomCharIter;

    let multibuffer = if rng.random() {
        let len = rng.random_range(100..10000);
        let text = RandomCharIter::new(&mut rng).take(len).collect::<String>();
        let buffer = cx.new(|cx| Buffer::local(text, cx));
        cx.new(|cx| MultiBuffer::singleton(buffer, cx))
    } else {
        MultiBuffer::build_random(&mut rng, cx)
    };

    let _diff_count = rng.random_range(1..5);
    let mut diffs = Vec::new();

    multibuffer.update(cx, |multibuffer, cx| {
        for buffer_id in multibuffer.excerpt_buffer_ids() {
            if rng.random_bool(0.7) {
                if let Some(buffer_handle) = multibuffer.buffer(buffer_id) {
                    let buffer_text = buffer_handle.read(cx).text();
                    let mut base_text = String::new();

                    for line in buffer_text.lines() {
                        if rng.random_bool(0.3) {
                            continue;
                        } else if rng.random_bool(0.3) {
                            let line_len = rng.random_range(0..50);
                            let modified_line = RandomCharIter::new(&mut rng)
                                .take(line_len)
                                .collect::<String>();
                            base_text.push_str(&modified_line);
                            base_text.push('\n');
                        } else {
                            base_text.push_str(line);
                            base_text.push('\n');
                        }
                    }

                    if rng.random_bool(0.5) {
                        let extra_lines = rng.random_range(1..5);
                        for _ in 0..extra_lines {
                            let line_len = rng.random_range(0..50);
                            let extra_line = RandomCharIter::new(&mut rng)
                                .take(line_len)
                                .collect::<String>();
                            base_text.push_str(&extra_line);
                            base_text.push('\n');
                        }
                    }

                    let diff =
                        cx.new(|cx| BufferDiff::new_with_base_text(&base_text, &buffer_handle, cx));
                    diffs.push(diff.clone());
                    multibuffer.add_diff(diff, cx);
                }
            }
        }
    });

    multibuffer.update(cx, |multibuffer, cx| {
        if rng.random_bool(0.5) {
            multibuffer.set_all_diff_hunks_expanded(cx);
        } else {
            let snapshot = multibuffer.snapshot(cx);
            let text = snapshot.text();

            let mut ranges = Vec::new();
            for _ in 0..rng.random_range(1..5) {
                if snapshot.len() == 0 {
                    break;
                }

                let diff_size = rng.random_range(5..1000);
                let mut start = rng.random_range(0..snapshot.len());

                while !text.is_char_boundary(start) {
                    start = start.saturating_sub(1);
                }

                let mut end = rng.random_range(start..snapshot.len().min(start + diff_size));

                while !text.is_char_boundary(end) {
                    end = end.saturating_add(1);
                }
                let start_anchor = snapshot.anchor_after(start);
                let end_anchor = snapshot.anchor_before(end);
                ranges.push(start_anchor..end_anchor);
            }
            multibuffer.expand_diff_hunks(ranges, cx);
        }
    });

    let snapshot = multibuffer.read(cx).snapshot(cx);

    let chunks = snapshot.chunks(0..snapshot.len(), false);

    for chunk in chunks {
        let chunk_text = chunk.text;
        let chars_bitmap = chunk.chars;
        let tabs_bitmap = chunk.tabs;

        if chunk_text.is_empty() {
            assert_eq!(
                chars_bitmap, 0,
                "Empty chunk should have empty chars bitmap"
            );
            assert_eq!(tabs_bitmap, 0, "Empty chunk should have empty tabs bitmap");
            continue;
        }

        assert!(
            chunk_text.len() <= 128,
            "Chunk text length {} exceeds 128 bytes",
            chunk_text.len()
        );

        let char_indices = chunk_text
            .char_indices()
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        for byte_idx in 0..chunk_text.len() {
            let should_have_bit = char_indices.contains(&byte_idx);
            let has_bit = chars_bitmap & (1 << byte_idx) != 0;

            if has_bit != should_have_bit {
                eprintln!("Chunk text bytes: {:?}", chunk_text.as_bytes());
                eprintln!("Char indices: {:?}", char_indices);
                eprintln!("Chars bitmap: {:#b}", chars_bitmap);
            }

            assert_eq!(
                has_bit, should_have_bit,
                "Chars bitmap mismatch at byte index {} in chunk {:?}. Expected bit: {}, Got bit: {}",
                byte_idx, chunk_text, should_have_bit, has_bit
            );
        }

        for (byte_idx, byte) in chunk_text.bytes().enumerate() {
            let is_tab = byte == b'\t';
            let has_bit = tabs_bitmap & (1 << byte_idx) != 0;

            if has_bit != is_tab {
                eprintln!("Chunk text bytes: {:?}", chunk_text.as_bytes());
                eprintln!("Tabs bitmap: {:#b}", tabs_bitmap);
                assert_eq!(
                    has_bit, is_tab,
                    "Tabs bitmap mismatch at byte index {} in chunk {:?}. Byte: {:?}, Expected bit: {}, Got bit: {}",
                    byte_idx, chunk_text, byte as char, is_tab, has_bit
                );
            }
        }
    }
}
