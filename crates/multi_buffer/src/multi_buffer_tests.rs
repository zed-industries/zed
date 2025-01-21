use super::*;
use gpui::{AppContext, Context, TestAppContext};
use language::{Buffer, Rope};
use parking_lot::RwLock;
use rand::prelude::*;
use settings::SettingsStore;
use std::env;
use util::test::sample_text;

#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

#[gpui::test]
fn test_singleton(cx: &mut AppContext) {
    let buffer = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'a'), cx));
    let multibuffer = cx.new_model(|cx| MultiBuffer::singleton(buffer.clone(), cx));

    let snapshot = multibuffer.read(cx).snapshot(cx);
    assert_eq!(snapshot.text(), buffer.read(cx).text());

    assert_eq!(
        snapshot.buffer_rows(MultiBufferRow(0)).collect::<Vec<_>>(),
        (0..buffer.read(cx).row_count())
            .map(Some)
            .collect::<Vec<_>>()
    );

    buffer.update(cx, |buffer, cx| buffer.edit([(1..3, "XXX\n")], None, cx));
    let snapshot = multibuffer.read(cx).snapshot(cx);

    assert_eq!(snapshot.text(), buffer.read(cx).text());
    assert_eq!(
        snapshot.buffer_rows(MultiBufferRow(0)).collect::<Vec<_>>(),
        (0..buffer.read(cx).row_count())
            .map(Some)
            .collect::<Vec<_>>()
    );
}

#[gpui::test]
fn test_remote(cx: &mut AppContext) {
    let host_buffer = cx.new_model(|cx| Buffer::local("a", cx));
    let guest_buffer = cx.new_model(|cx| {
        let state = host_buffer.read(cx).to_proto(cx);
        let ops = cx
            .background_executor()
            .block(host_buffer.read(cx).serialize_ops(None, cx));
        let mut buffer = Buffer::from_proto(1, Capability::ReadWrite, state, None).unwrap();
        buffer.apply_ops(
            ops.into_iter()
                .map(|op| language::proto::deserialize_operation(op).unwrap()),
            cx,
        );
        buffer
    });
    let multibuffer = cx.new_model(|cx| MultiBuffer::singleton(guest_buffer.clone(), cx));
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
fn test_excerpt_boundaries_and_clipping(cx: &mut AppContext) {
    let buffer_1 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'a'), cx));
    let buffer_2 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'g'), cx));
    let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));

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
            [ExcerptRange {
                context: Point::new(1, 2)..Point::new(2, 5),
                primary: None,
            }],
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
            [ExcerptRange {
                context: Point::new(3, 3)..Point::new(4, 4),
                primary: None,
            }],
            cx,
        );
        multibuffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange {
                context: Point::new(3, 1)..Point::new(3, 3),
                primary: None,
            }],
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
                singleton_buffer_edited: false,
                edited_buffer: None,
            },
            Event::Edited {
                singleton_buffer_edited: false,
                edited_buffer: None,
            },
            Event::Edited {
                singleton_buffer_edited: false,
                edited_buffer: None,
            }
        ]
    );

    let snapshot = multibuffer.read(cx).snapshot(cx);
    assert_eq!(
        snapshot.text(),
        concat!(
            "bbbb\n",  // Preserve newlines
            "ccccc\n", //
            "ddd\n",   //
            "eeee\n",  //
            "jj"       //
        )
    );
    assert_eq!(
        snapshot.buffer_rows(MultiBufferRow(0)).collect::<Vec<_>>(),
        [Some(1), Some(2), Some(3), Some(4), Some(3)]
    );
    assert_eq!(
        snapshot.buffer_rows(MultiBufferRow(2)).collect::<Vec<_>>(),
        [Some(3), Some(4), Some(3)]
    );
    assert_eq!(
        snapshot.buffer_rows(MultiBufferRow(4)).collect::<Vec<_>>(),
        [Some(3)]
    );
    assert_eq!(
        snapshot.buffer_rows(MultiBufferRow(5)).collect::<Vec<_>>(),
        []
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
        let (buffer_2_excerpt_id, _) = multibuffer.excerpts_for_buffer(&buffer_2, cx)[0].clone();
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
            .filter_map(|boundary| {
                let starts_new_buffer = boundary.starts_new_buffer();
                boundary.next.map(|next| {
                    (
                        boundary.row,
                        next.buffer
                            .text_for_range(next.range.context)
                            .collect::<String>(),
                        starts_new_buffer,
                    )
                })
            })
            .collect::<Vec<_>>()
    }
}

#[gpui::test]
fn test_excerpt_events(cx: &mut AppContext) {
    let buffer_1 = cx.new_model(|cx| Buffer::local(sample_text(10, 3, 'a'), cx));
    let buffer_2 = cx.new_model(|cx| Buffer::local(sample_text(10, 3, 'm'), cx));

    let leader_multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));
    let follower_multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));
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
                Event::ExcerptsRemoved { ids } => follower.remove_excerpts(ids, cx),
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
            [
                ExcerptRange {
                    context: 0..8,
                    primary: None,
                },
                ExcerptRange {
                    context: 12..16,
                    primary: None,
                },
            ],
            cx,
        );
        leader.insert_excerpts_after(
            leader.excerpt_ids()[0],
            buffer_2.clone(),
            [
                ExcerptRange {
                    context: 0..5,
                    primary: None,
                },
                ExcerptRange {
                    context: 10..15,
                    primary: None,
                },
            ],
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
fn test_expand_excerpts(cx: &mut AppContext) {
    let buffer = cx.new_model(|cx| Buffer::local(sample_text(20, 3, 'a'), cx));
    let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.push_excerpts_with_context_lines(
            buffer.clone(),
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
        multibuffer.expand_excerpts(
            multibuffer.excerpt_ids(),
            1,
            ExpandExcerptDirection::UpAndDown,
            cx,
        )
    });

    let snapshot = multibuffer.read(cx).snapshot(cx);

    // Expanding context lines causes the line containing 'fff' to appear in two different excerpts.
    // We don't attempt to merge them, because removing the excerpt could create inconsistency with other layers
    // that are tracking excerpt ids.
    assert_eq!(
        snapshot.text(),
        concat!(
            "bbb\n", //
            "ccc\n", //
            "ddd\n", //
            "eee\n", //
            "fff\n", // End of excerpt
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

#[gpui::test]
fn test_push_excerpts_with_context_lines(cx: &mut AppContext) {
    let buffer = cx.new_model(|cx| Buffer::local(sample_text(20, 3, 'a'), cx));
    let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));
    let anchor_ranges = multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.push_excerpts_with_context_lines(
            buffer.clone(),
            vec![
                // Note that in this test, this first excerpt
                // does contain a new line
                Point::new(3, 2)..Point::new(4, 2),
                Point::new(7, 1)..Point::new(7, 3),
                Point::new(15, 0)..Point::new(15, 0),
            ],
            2,
            cx,
        )
    });

    let snapshot = multibuffer.read(cx).snapshot(cx);
    assert_eq!(
        snapshot.text(),
        concat!(
            "bbb\n", // Preserve newlines
            "ccc\n", //
            "ddd\n", //
            "eee\n", //
            "fff\n", //
            "ggg\n", //
            "hhh\n", //
            "iii\n", //
            "jjj\n", //
            "nnn\n", //
            "ooo\n", //
            "ppp\n", //
            "qqq\n", //
            "rrr",   //
        )
    );

    assert_eq!(
        anchor_ranges
            .iter()
            .map(|range| range.to_point(&snapshot))
            .collect::<Vec<_>>(),
        vec![
            Point::new(2, 2)..Point::new(3, 2),
            Point::new(6, 1)..Point::new(6, 3),
            Point::new(11, 0)..Point::new(11, 0)
        ]
    );
}

#[gpui::test(iterations = 100)]
async fn test_push_multiple_excerpts_with_context_lines(cx: &mut TestAppContext) {
    let buffer_1 = cx.new_model(|cx| Buffer::local(sample_text(20, 3, 'a'), cx));
    let buffer_2 = cx.new_model(|cx| Buffer::local(sample_text(15, 4, 'a'), cx));
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

    let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));
    let anchor_ranges = multibuffer
        .update(cx, |multibuffer, cx| {
            multibuffer.push_multiple_excerpts_with_context_lines(
                vec![(buffer_1.clone(), ranges_1), (buffer_2.clone(), ranges_2)],
                2,
                cx,
            )
        })
        .await;

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

    assert_eq!(
        anchor_ranges
            .iter()
            .map(|range| range.to_point(&snapshot))
            .collect::<Vec<_>>(),
        vec![
            Point::new(2, 2)..Point::new(3, 2),
            Point::new(6, 1)..Point::new(6, 3),
            Point::new(11, 0)..Point::new(11, 0),
            Point::new(16, 1)..Point::new(17, 1),
            Point::new(22, 0)..Point::new(22, 2)
        ]
    );
}

#[gpui::test]
fn test_empty_multibuffer(cx: &mut AppContext) {
    let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));

    let snapshot = multibuffer.read(cx).snapshot(cx);
    assert_eq!(snapshot.text(), "");
    assert_eq!(
        snapshot.buffer_rows(MultiBufferRow(0)).collect::<Vec<_>>(),
        &[Some(0)]
    );
    assert_eq!(
        snapshot.buffer_rows(MultiBufferRow(1)).collect::<Vec<_>>(),
        &[]
    );
}

#[gpui::test]
fn test_singleton_multibuffer_anchors(cx: &mut AppContext) {
    let buffer = cx.new_model(|cx| Buffer::local("abcd", cx));
    let multibuffer = cx.new_model(|cx| MultiBuffer::singleton(buffer.clone(), cx));
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
fn test_multibuffer_anchors(cx: &mut AppContext) {
    let buffer_1 = cx.new_model(|cx| Buffer::local("abcd", cx));
    let buffer_2 = cx.new_model(|cx| Buffer::local("efghi", cx));
    let multibuffer = cx.new_model(|cx| {
        let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
        multibuffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange {
                context: 0..4,
                primary: None,
            }],
            cx,
        );
        multibuffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange {
                context: 0..5,
                primary: None,
            }],
            cx,
        );
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
fn test_resolving_anchors_after_replacing_their_excerpts(cx: &mut AppContext) {
    let buffer_1 = cx.new_model(|cx| Buffer::local("abcd", cx));
    let buffer_2 = cx.new_model(|cx| Buffer::local("ABCDEFGHIJKLMNOP", cx));
    let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));

    // Create an insertion id in buffer 1 that doesn't exist in buffer 2.
    // Add an excerpt from buffer 1 that spans this new insertion.
    buffer_1.update(cx, |buffer, cx| buffer.edit([(4..4, "123")], None, cx));
    let excerpt_id_1 = multibuffer.update(cx, |multibuffer, cx| {
        multibuffer
            .push_excerpts(
                buffer_1.clone(),
                [ExcerptRange {
                    context: 0..7,
                    primary: None,
                }],
                cx,
            )
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
                    ExcerptRange {
                        context: 0..4,
                        primary: None,
                    },
                    ExcerptRange {
                        context: 6..10,
                        primary: None,
                    },
                    ExcerptRange {
                        context: 12..16,
                        primary: None,
                    },
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
                [ExcerptRange {
                    context: 5..8,
                    primary: None,
                }],
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

#[gpui::test(iterations = 100)]
fn test_random_multibuffer(cx: &mut AppContext, mut rng: StdRng) {
    let operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(10);

    let mut buffers: Vec<Model<Buffer>> = Vec::new();
    let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));
    let mut excerpt_ids = Vec::<ExcerptId>::new();
    let mut expected_excerpts = Vec::<(Model<Buffer>, Range<text::Anchor>)>::new();
    let mut anchors = Vec::new();
    let mut old_versions = Vec::new();

    for _ in 0..operations {
        match rng.gen_range(0..100) {
            0..=14 if !buffers.is_empty() => {
                let buffer = buffers.choose(&mut rng).unwrap();
                buffer.update(cx, |buf, cx| buf.randomly_edit(&mut rng, 5, cx));
            }
            15..=19 if !expected_excerpts.is_empty() => {
                multibuffer.update(cx, |multibuffer, cx| {
                    let ids = multibuffer.excerpt_ids();
                    let mut excerpts = HashSet::default();
                    for _ in 0..rng.gen_range(0..ids.len()) {
                        excerpts.extend(ids.choose(&mut rng).copied());
                    }

                    let line_count = rng.gen_range(0..5);

                    let excerpt_ixs = excerpts
                        .iter()
                        .map(|id| excerpt_ids.iter().position(|i| i == id).unwrap())
                        .collect::<Vec<_>>();
                    log::info!("Expanding excerpts {excerpt_ixs:?} by {line_count} lines");
                    multibuffer.expand_excerpts(
                        excerpts.iter().cloned(),
                        line_count,
                        ExpandExcerptDirection::UpAndDown,
                        cx,
                    );

                    if line_count > 0 {
                        for id in excerpts {
                            let excerpt_ix = excerpt_ids.iter().position(|&i| i == id).unwrap();
                            let (buffer, range) = &mut expected_excerpts[excerpt_ix];
                            let snapshot = buffer.read(cx).snapshot();
                            let mut point_range = range.to_point(&snapshot);
                            point_range.start =
                                Point::new(point_range.start.row.saturating_sub(line_count), 0);
                            point_range.end = snapshot.clip_point(
                                Point::new(point_range.end.row + line_count, 0),
                                Bias::Left,
                            );
                            point_range.end.column = snapshot.line_len(point_range.end.row);
                            *range = snapshot.anchor_before(point_range.start)
                                ..snapshot.anchor_after(point_range.end);
                        }
                    }
                });
            }
            20..=29 if !expected_excerpts.is_empty() => {
                let mut ids_to_remove = vec![];
                for _ in 0..rng.gen_range(1..=3) {
                    if expected_excerpts.is_empty() {
                        break;
                    }

                    let ix = rng.gen_range(0..expected_excerpts.len());
                    ids_to_remove.push(excerpt_ids.remove(ix));
                    let (buffer, range) = expected_excerpts.remove(ix);
                    let buffer = buffer.read(cx);
                    log::info!(
                        "Removing excerpt {}: {:?}",
                        ix,
                        buffer
                            .text_for_range(range.to_offset(buffer))
                            .collect::<String>(),
                    );
                }
                let snapshot = multibuffer.read(cx).read(cx);
                ids_to_remove.sort_unstable_by(|a, b| a.cmp(b, &snapshot));
                drop(snapshot);
                multibuffer.update(cx, |multibuffer, cx| {
                    multibuffer.remove_excerpts(ids_to_remove, cx)
                });
            }
            30..=39 if !expected_excerpts.is_empty() => {
                let multibuffer = multibuffer.read(cx).read(cx);
                let offset =
                    multibuffer.clip_offset(rng.gen_range(0..=multibuffer.len()), Bias::Left);
                let bias = if rng.gen() { Bias::Left } else { Bias::Right };
                log::info!("Creating anchor at {} with bias {:?}", offset, bias);
                anchors.push(multibuffer.anchor_at(offset, bias));
                anchors.sort_by(|a, b| a.cmp(b, &multibuffer));
            }
            40..=44 if !anchors.is_empty() => {
                let multibuffer = multibuffer.read(cx).read(cx);
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
            _ => {
                let buffer_handle = if buffers.is_empty() || rng.gen_bool(0.4) {
                    let base_text = util::RandomCharIter::new(&mut rng)
                        .take(25)
                        .collect::<String>();

                    buffers.push(cx.new_model(|cx| Buffer::local(base_text, cx)));
                    buffers.last().unwrap()
                } else {
                    buffers.choose(&mut rng).unwrap()
                };

                let buffer = buffer_handle.read(cx);
                let end_ix = buffer.clip_offset(rng.gen_range(0..=buffer.len()), Bias::Right);
                let start_ix = buffer.clip_offset(rng.gen_range(0..=end_ix), Bias::Left);
                let anchor_range = buffer.anchor_before(start_ix)..buffer.anchor_after(end_ix);
                let prev_excerpt_ix = rng.gen_range(0..=expected_excerpts.len());
                let prev_excerpt_id = excerpt_ids
                    .get(prev_excerpt_ix)
                    .cloned()
                    .unwrap_or_else(ExcerptId::max);
                let excerpt_ix = (prev_excerpt_ix + 1).min(expected_excerpts.len());

                log::info!(
                    "Inserting excerpt at {} of {} for buffer {}: {:?}[{:?}] = {:?}",
                    excerpt_ix,
                    expected_excerpts.len(),
                    buffer_handle.read(cx).remote_id(),
                    buffer.text(),
                    start_ix..end_ix,
                    &buffer.text()[start_ix..end_ix]
                );

                let excerpt_id = multibuffer.update(cx, |multibuffer, cx| {
                    multibuffer
                        .insert_excerpts_after(
                            prev_excerpt_id,
                            buffer_handle.clone(),
                            [ExcerptRange {
                                context: start_ix..end_ix,
                                primary: None,
                            }],
                            cx,
                        )
                        .pop()
                        .unwrap()
                });

                excerpt_ids.insert(excerpt_ix, excerpt_id);
                expected_excerpts.insert(excerpt_ix, (buffer_handle.clone(), anchor_range));
            }
        }

        if rng.gen_bool(0.3) {
            multibuffer.update(cx, |multibuffer, cx| {
                old_versions.push((multibuffer.snapshot(cx), multibuffer.subscribe()));
            })
        }

        let snapshot = multibuffer.read(cx).snapshot(cx);

        let mut excerpt_starts = Vec::new();
        let mut expected_text = String::new();
        let mut expected_buffer_rows = Vec::new();
        for (buffer, range) in &expected_excerpts {
            let buffer = buffer.read(cx);
            let buffer_range = range.to_offset(buffer);

            excerpt_starts.push(TextSummary::from(expected_text.as_str()));
            expected_text.extend(buffer.text_for_range(buffer_range.clone()));
            expected_text.push('\n');

            let buffer_row_range = buffer.offset_to_point(buffer_range.start).row
                ..=buffer.offset_to_point(buffer_range.end).row;
            for row in buffer_row_range {
                expected_buffer_rows.push(Some(row));
            }
        }
        // Remove final trailing newline.
        if !expected_excerpts.is_empty() {
            expected_text.pop();
        }

        // Always report one buffer row
        if expected_buffer_rows.is_empty() {
            expected_buffer_rows.push(Some(0));
        }

        assert_eq!(snapshot.text(), expected_text);
        log::info!("MultiBuffer text: {:?}", expected_text);

        assert_eq!(
            snapshot.buffer_rows(MultiBufferRow(0)).collect::<Vec<_>>(),
            expected_buffer_rows,
        );

        for _ in 0..5 {
            let start_row = rng.gen_range(0..=expected_buffer_rows.len());
            assert_eq!(
                snapshot
                    .buffer_rows(MultiBufferRow(start_row as u32))
                    .collect::<Vec<_>>(),
                &expected_buffer_rows[start_row..],
                "buffer_rows({})",
                start_row
            );
        }

        assert_eq!(
            snapshot.widest_line_number(),
            expected_buffer_rows.into_iter().flatten().max().unwrap() + 1
        );

        let mut excerpt_starts = excerpt_starts.into_iter();
        for (buffer, range) in &expected_excerpts {
            let buffer = buffer.read(cx);
            let buffer_id = buffer.remote_id();
            let buffer_range = range.to_offset(buffer);
            let buffer_start_point = buffer.offset_to_point(buffer_range.start);
            let buffer_start_point_utf16 =
                buffer.text_summary_for_range::<PointUtf16, _>(0..buffer_range.start);

            let excerpt_start = excerpt_starts.next().unwrap();
            let mut offset = excerpt_start.len;
            let mut buffer_offset = buffer_range.start;
            let mut point = excerpt_start.lines;
            let mut buffer_point = buffer_start_point;
            let mut point_utf16 = excerpt_start.lines_utf16();
            let mut buffer_point_utf16 = buffer_start_point_utf16;
            for ch in buffer
                .snapshot()
                .chunks(buffer_range.clone(), false)
                .flat_map(|c| c.text.chars())
            {
                for _ in 0..ch.len_utf8() {
                    let left_offset = snapshot.clip_offset(offset, Bias::Left);
                    let right_offset = snapshot.clip_offset(offset, Bias::Right);
                    let buffer_left_offset = buffer.clip_offset(buffer_offset, Bias::Left);
                    let buffer_right_offset = buffer.clip_offset(buffer_offset, Bias::Right);
                    assert_eq!(
                        left_offset,
                        excerpt_start.len + (buffer_left_offset - buffer_range.start),
                        "clip_offset({:?}, Left). buffer: {:?}, buffer offset: {:?}",
                        offset,
                        buffer_id,
                        buffer_offset,
                    );
                    assert_eq!(
                        right_offset,
                        excerpt_start.len + (buffer_right_offset - buffer_range.start),
                        "clip_offset({:?}, Right). buffer: {:?}, buffer offset: {:?}",
                        offset,
                        buffer_id,
                        buffer_offset,
                    );

                    let left_point = snapshot.clip_point(point, Bias::Left);
                    let right_point = snapshot.clip_point(point, Bias::Right);
                    let buffer_left_point = buffer.clip_point(buffer_point, Bias::Left);
                    let buffer_right_point = buffer.clip_point(buffer_point, Bias::Right);
                    assert_eq!(
                        left_point,
                        excerpt_start.lines + (buffer_left_point - buffer_start_point),
                        "clip_point({:?}, Left). buffer: {:?}, buffer point: {:?}",
                        point,
                        buffer_id,
                        buffer_point,
                    );
                    assert_eq!(
                        right_point,
                        excerpt_start.lines + (buffer_right_point - buffer_start_point),
                        "clip_point({:?}, Right). buffer: {:?}, buffer point: {:?}",
                        point,
                        buffer_id,
                        buffer_point,
                    );

                    assert_eq!(
                        snapshot.point_to_offset(left_point),
                        left_offset,
                        "point_to_offset({:?})",
                        left_point,
                    );
                    assert_eq!(
                        snapshot.offset_to_point(left_offset),
                        left_point,
                        "offset_to_point({:?})",
                        left_offset,
                    );

                    offset += 1;
                    buffer_offset += 1;
                    if ch == '\n' {
                        point += Point::new(1, 0);
                        buffer_point += Point::new(1, 0);
                    } else {
                        point += Point::new(0, 1);
                        buffer_point += Point::new(0, 1);
                    }
                }

                for _ in 0..ch.len_utf16() {
                    let left_point_utf16 =
                        snapshot.clip_point_utf16(Unclipped(point_utf16), Bias::Left);
                    let right_point_utf16 =
                        snapshot.clip_point_utf16(Unclipped(point_utf16), Bias::Right);
                    let buffer_left_point_utf16 =
                        buffer.clip_point_utf16(Unclipped(buffer_point_utf16), Bias::Left);
                    let buffer_right_point_utf16 =
                        buffer.clip_point_utf16(Unclipped(buffer_point_utf16), Bias::Right);
                    assert_eq!(
                        left_point_utf16,
                        excerpt_start.lines_utf16()
                            + (buffer_left_point_utf16 - buffer_start_point_utf16),
                        "clip_point_utf16({:?}, Left). buffer: {:?}, buffer point_utf16: {:?}",
                        point_utf16,
                        buffer_id,
                        buffer_point_utf16,
                    );
                    assert_eq!(
                        right_point_utf16,
                        excerpt_start.lines_utf16()
                            + (buffer_right_point_utf16 - buffer_start_point_utf16),
                        "clip_point_utf16({:?}, Right). buffer: {:?}, buffer point_utf16: {:?}",
                        point_utf16,
                        buffer_id,
                        buffer_point_utf16,
                    );

                    if ch == '\n' {
                        point_utf16 += PointUtf16::new(1, 0);
                        buffer_point_utf16 += PointUtf16::new(1, 0);
                    } else {
                        point_utf16 += PointUtf16::new(0, 1);
                        buffer_point_utf16 += PointUtf16::new(0, 1);
                    }
                }
            }
        }

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
            let end_ix = text_rope.clip_offset(rng.gen_range(0..=text_rope.len()), Bias::Right);
            let start_ix = text_rope.clip_offset(rng.gen_range(0..=end_ix), Bias::Left);

            let text_for_range = snapshot
                .text_for_range(start_ix..end_ix)
                .collect::<String>();
            assert_eq!(
                text_for_range,
                &expected_text[start_ix..end_ix],
                "incorrect text for range {:?}",
                start_ix..end_ix
            );

            let snapshot = multibuffer.read(cx).snapshot(cx);
            let excerpted_buffer_ranges = snapshot.range_to_buffer_ranges(start_ix..end_ix);
            let excerpted_buffers_text = excerpted_buffer_ranges
                .iter()
                .map(|(excerpt, buffer_range)| {
                    excerpt
                        .buffer()
                        .text_for_range(buffer_range.clone())
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n");
            assert_eq!(excerpted_buffers_text, text_for_range);
            if !expected_excerpts.is_empty() {
                assert!(!excerpted_buffer_ranges.is_empty());
            }

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
                resolved_offset
            );
        }

        for _ in 0..10 {
            let end_ix = text_rope.clip_offset(rng.gen_range(0..=text_rope.len()), Bias::Right);
            assert_eq!(
                snapshot.reversed_chars_at(end_ix).collect::<String>(),
                expected_text[..end_ix].chars().rev().collect::<String>(),
            );
        }

        for _ in 0..10 {
            let end_ix = rng.gen_range(0..=text_rope.len());
            let start_ix = rng.gen_range(0..=end_ix);
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

    let snapshot = multibuffer.read(cx).snapshot(cx);
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
fn test_history(cx: &mut AppContext) {
    let test_settings = SettingsStore::test(cx);
    cx.set_global(test_settings);
    let group_interval: Duration = Duration::from_millis(1);
    let buffer_1 = cx.new_model(|cx| {
        let mut buf = Buffer::local("1234", cx);
        buf.set_group_interval(group_interval);
        buf
    });
    let buffer_2 = cx.new_model(|cx| {
        let mut buf = Buffer::local("5678", cx);
        buf.set_group_interval(group_interval);
        buf
    });
    let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));
    multibuffer.update(cx, |this, _| {
        this.history.group_interval = group_interval;
    });
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange {
                context: 0..buffer_1.read(cx).len(),
                primary: None,
            }],
            cx,
        );
        multibuffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange {
                context: 0..buffer_2.read(cx).len(),
                primary: None,
            }],
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
fn test_excerpts_in_ranges_no_ranges(cx: &mut AppContext) {
    let buffer_1 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'a'), cx));
    let buffer_2 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'g'), cx));
    let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange {
                context: 0..buffer_1.read(cx).len(),
                primary: None,
            }],
            cx,
        );
        multibuffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange {
                context: 0..buffer_2.read(cx).len(),
                primary: None,
            }],
            cx,
        );
    });

    let snapshot = multibuffer.update(cx, |multibuffer, cx| multibuffer.snapshot(cx));

    let mut excerpts = snapshot.excerpts_in_ranges(iter::from_fn(|| None));

    assert!(excerpts.next().is_none());
}

fn validate_excerpts(
    actual: &[(ExcerptId, BufferId, Range<Anchor>)],
    expected: &Vec<(ExcerptId, BufferId, Range<Anchor>)>,
) {
    assert_eq!(actual.len(), expected.len());

    actual
        .iter()
        .zip(expected)
        .map(|(actual, expected)| {
            assert_eq!(actual.0, expected.0);
            assert_eq!(actual.1, expected.1);
            assert_eq!(actual.2.start, expected.2.start);
            assert_eq!(actual.2.end, expected.2.end);
        })
        .collect_vec();
}

fn map_range_from_excerpt(
    snapshot: &MultiBufferSnapshot,
    excerpt_id: ExcerptId,
    excerpt_buffer: &BufferSnapshot,
    range: Range<usize>,
) -> Range<Anchor> {
    snapshot
        .anchor_in_excerpt(excerpt_id, excerpt_buffer.anchor_before(range.start))
        .unwrap()
        ..snapshot
            .anchor_in_excerpt(excerpt_id, excerpt_buffer.anchor_after(range.end))
            .unwrap()
}

fn make_expected_excerpt_info(
    snapshot: &MultiBufferSnapshot,
    cx: &mut AppContext,
    excerpt_id: ExcerptId,
    buffer: &Model<Buffer>,
    range: Range<usize>,
) -> (ExcerptId, BufferId, Range<Anchor>) {
    (
        excerpt_id,
        buffer.read(cx).remote_id(),
        map_range_from_excerpt(snapshot, excerpt_id, &buffer.read(cx).snapshot(), range),
    )
}

#[gpui::test]
fn test_excerpts_in_ranges_range_inside_the_excerpt(cx: &mut AppContext) {
    let buffer_1 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'a'), cx));
    let buffer_2 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'g'), cx));
    let buffer_len = buffer_1.read(cx).len();
    let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));
    let mut expected_excerpt_id = ExcerptId(0);

    multibuffer.update(cx, |multibuffer, cx| {
        expected_excerpt_id = multibuffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange {
                context: 0..buffer_1.read(cx).len(),
                primary: None,
            }],
            cx,
        )[0];
        multibuffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange {
                context: 0..buffer_2.read(cx).len(),
                primary: None,
            }],
            cx,
        );
    });

    let snapshot = multibuffer.update(cx, |multibuffer, cx| multibuffer.snapshot(cx));

    let range = snapshot
        .anchor_in_excerpt(expected_excerpt_id, buffer_1.read(cx).anchor_before(1))
        .unwrap()
        ..snapshot
            .anchor_in_excerpt(
                expected_excerpt_id,
                buffer_1.read(cx).anchor_after(buffer_len / 2),
            )
            .unwrap();

    let expected_excerpts = vec![make_expected_excerpt_info(
        &snapshot,
        cx,
        expected_excerpt_id,
        &buffer_1,
        1..(buffer_len / 2),
    )];

    let excerpts = snapshot
        .excerpts_in_ranges(vec![range.clone()].into_iter())
        .map(|(excerpt_id, buffer, actual_range)| {
            (
                excerpt_id,
                buffer.remote_id(),
                map_range_from_excerpt(&snapshot, excerpt_id, buffer, actual_range),
            )
        })
        .collect_vec();

    validate_excerpts(&excerpts, &expected_excerpts);
}

#[gpui::test]
fn test_excerpts_in_ranges_range_crosses_excerpts_boundary(cx: &mut AppContext) {
    let buffer_1 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'a'), cx));
    let buffer_2 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'g'), cx));
    let buffer_len = buffer_1.read(cx).len();
    let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));
    let mut excerpt_1_id = ExcerptId(0);
    let mut excerpt_2_id = ExcerptId(0);

    multibuffer.update(cx, |multibuffer, cx| {
        excerpt_1_id = multibuffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange {
                context: 0..buffer_1.read(cx).len(),
                primary: None,
            }],
            cx,
        )[0];
        excerpt_2_id = multibuffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange {
                context: 0..buffer_2.read(cx).len(),
                primary: None,
            }],
            cx,
        )[0];
    });

    let snapshot = multibuffer.read(cx).snapshot(cx);

    let expected_range = snapshot
        .anchor_in_excerpt(
            excerpt_1_id,
            buffer_1.read(cx).anchor_before(buffer_len / 2),
        )
        .unwrap()
        ..snapshot
            .anchor_in_excerpt(excerpt_2_id, buffer_2.read(cx).anchor_after(buffer_len / 2))
            .unwrap();

    let expected_excerpts = vec![
        make_expected_excerpt_info(
            &snapshot,
            cx,
            excerpt_1_id,
            &buffer_1,
            (buffer_len / 2)..buffer_len,
        ),
        make_expected_excerpt_info(&snapshot, cx, excerpt_2_id, &buffer_2, 0..buffer_len / 2),
    ];

    let excerpts = snapshot
        .excerpts_in_ranges(vec![expected_range.clone()].into_iter())
        .map(|(excerpt_id, buffer, actual_range)| {
            (
                excerpt_id,
                buffer.remote_id(),
                map_range_from_excerpt(&snapshot, excerpt_id, buffer, actual_range),
            )
        })
        .collect_vec();

    validate_excerpts(&excerpts, &expected_excerpts);
}

#[gpui::test]
fn test_excerpts_in_ranges_range_encloses_excerpt(cx: &mut AppContext) {
    let buffer_1 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'a'), cx));
    let buffer_2 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'g'), cx));
    let buffer_3 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'r'), cx));
    let buffer_len = buffer_1.read(cx).len();
    let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));
    let mut excerpt_1_id = ExcerptId(0);
    let mut excerpt_2_id = ExcerptId(0);
    let mut excerpt_3_id = ExcerptId(0);

    multibuffer.update(cx, |multibuffer, cx| {
        excerpt_1_id = multibuffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange {
                context: 0..buffer_1.read(cx).len(),
                primary: None,
            }],
            cx,
        )[0];
        excerpt_2_id = multibuffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange {
                context: 0..buffer_2.read(cx).len(),
                primary: None,
            }],
            cx,
        )[0];
        excerpt_3_id = multibuffer.push_excerpts(
            buffer_3.clone(),
            [ExcerptRange {
                context: 0..buffer_3.read(cx).len(),
                primary: None,
            }],
            cx,
        )[0];
    });

    let snapshot = multibuffer.read(cx).snapshot(cx);

    let expected_range = snapshot
        .anchor_in_excerpt(
            excerpt_1_id,
            buffer_1.read(cx).anchor_before(buffer_len / 2),
        )
        .unwrap()
        ..snapshot
            .anchor_in_excerpt(excerpt_3_id, buffer_3.read(cx).anchor_after(buffer_len / 2))
            .unwrap();

    let expected_excerpts = vec![
        make_expected_excerpt_info(
            &snapshot,
            cx,
            excerpt_1_id,
            &buffer_1,
            (buffer_len / 2)..buffer_len,
        ),
        make_expected_excerpt_info(&snapshot, cx, excerpt_2_id, &buffer_2, 0..buffer_len),
        make_expected_excerpt_info(&snapshot, cx, excerpt_3_id, &buffer_3, 0..buffer_len / 2),
    ];

    let excerpts = snapshot
        .excerpts_in_ranges(vec![expected_range.clone()].into_iter())
        .map(|(excerpt_id, buffer, actual_range)| {
            (
                excerpt_id,
                buffer.remote_id(),
                map_range_from_excerpt(&snapshot, excerpt_id, buffer, actual_range),
            )
        })
        .collect_vec();

    validate_excerpts(&excerpts, &expected_excerpts);
}

#[gpui::test]
fn test_excerpts_in_ranges_multiple_ranges(cx: &mut AppContext) {
    let buffer_1 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'a'), cx));
    let buffer_2 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'g'), cx));
    let buffer_len = buffer_1.read(cx).len();
    let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));
    let mut excerpt_1_id = ExcerptId(0);
    let mut excerpt_2_id = ExcerptId(0);

    multibuffer.update(cx, |multibuffer, cx| {
        excerpt_1_id = multibuffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange {
                context: 0..buffer_1.read(cx).len(),
                primary: None,
            }],
            cx,
        )[0];
        excerpt_2_id = multibuffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange {
                context: 0..buffer_2.read(cx).len(),
                primary: None,
            }],
            cx,
        )[0];
    });

    let snapshot = multibuffer.read(cx).snapshot(cx);

    let ranges = vec![
        1..(buffer_len / 4),
        (buffer_len / 3)..(buffer_len / 2),
        (buffer_len / 4 * 3)..(buffer_len),
    ];

    let expected_excerpts = ranges
        .iter()
        .map(|range| {
            make_expected_excerpt_info(&snapshot, cx, excerpt_1_id, &buffer_1, range.clone())
        })
        .collect_vec();

    let ranges = ranges.into_iter().map(|range| {
        map_range_from_excerpt(
            &snapshot,
            excerpt_1_id,
            &buffer_1.read(cx).snapshot(),
            range,
        )
    });

    let excerpts = snapshot
        .excerpts_in_ranges(ranges)
        .map(|(excerpt_id, buffer, actual_range)| {
            (
                excerpt_id,
                buffer.remote_id(),
                map_range_from_excerpt(&snapshot, excerpt_id, buffer, actual_range),
            )
        })
        .collect_vec();

    validate_excerpts(&excerpts, &expected_excerpts);
}

#[gpui::test]
fn test_excerpts_in_ranges_range_ends_at_excerpt_end(cx: &mut AppContext) {
    let buffer_1 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'a'), cx));
    let buffer_2 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'g'), cx));
    let buffer_len = buffer_1.read(cx).len();
    let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));
    let mut excerpt_1_id = ExcerptId(0);
    let mut excerpt_2_id = ExcerptId(0);

    multibuffer.update(cx, |multibuffer, cx| {
        excerpt_1_id = multibuffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange {
                context: 0..buffer_1.read(cx).len(),
                primary: None,
            }],
            cx,
        )[0];
        excerpt_2_id = multibuffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange {
                context: 0..buffer_2.read(cx).len(),
                primary: None,
            }],
            cx,
        )[0];
    });

    let snapshot = multibuffer.read(cx).snapshot(cx);

    let ranges = [0..buffer_len, (buffer_len / 3)..(buffer_len / 2)];

    let expected_excerpts = vec![
        make_expected_excerpt_info(&snapshot, cx, excerpt_1_id, &buffer_1, ranges[0].clone()),
        make_expected_excerpt_info(&snapshot, cx, excerpt_2_id, &buffer_2, ranges[1].clone()),
    ];

    let ranges = [
        map_range_from_excerpt(
            &snapshot,
            excerpt_1_id,
            &buffer_1.read(cx).snapshot(),
            ranges[0].clone(),
        ),
        map_range_from_excerpt(
            &snapshot,
            excerpt_2_id,
            &buffer_2.read(cx).snapshot(),
            ranges[1].clone(),
        ),
    ];

    let excerpts = snapshot
        .excerpts_in_ranges(ranges.into_iter())
        .map(|(excerpt_id, buffer, actual_range)| {
            (
                excerpt_id,
                buffer.remote_id(),
                map_range_from_excerpt(&snapshot, excerpt_id, buffer, actual_range),
            )
        })
        .collect_vec();

    validate_excerpts(&excerpts, &expected_excerpts);
}

#[gpui::test]
fn test_split_ranges(cx: &mut AppContext) {
    let buffer_1 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'a'), cx));
    let buffer_2 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'g'), cx));
    let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange {
                context: 0..buffer_1.read(cx).len(),
                primary: None,
            }],
            cx,
        );
        multibuffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange {
                context: 0..buffer_2.read(cx).len(),
                primary: None,
            }],
            cx,
        );
    });

    let snapshot = multibuffer.read(cx).snapshot(cx);

    let buffer_1_len = buffer_1.read(cx).len();
    let buffer_2_len = buffer_2.read(cx).len();
    let buffer_1_midpoint = buffer_1_len / 2;
    let buffer_2_start = buffer_1_len + '\n'.len_utf8();
    let buffer_2_midpoint = buffer_2_start + buffer_2_len / 2;
    let total_len = buffer_2_start + buffer_2_len;

    let input_ranges = [
        0..buffer_1_midpoint,
        buffer_1_midpoint..buffer_2_midpoint,
        buffer_2_midpoint..total_len,
    ]
    .map(|range| snapshot.anchor_before(range.start)..snapshot.anchor_after(range.end));

    let actual_ranges = snapshot
        .split_ranges(input_ranges.into_iter())
        .map(|range| range.to_offset(&snapshot))
        .collect::<Vec<_>>();

    let expected_ranges = vec![
        0..buffer_1_midpoint,
        buffer_1_midpoint..buffer_1_len,
        buffer_2_start..buffer_2_midpoint,
        buffer_2_midpoint..total_len,
    ];

    assert_eq!(actual_ranges, expected_ranges);
}

#[gpui::test]
fn test_split_ranges_single_range_spanning_three_excerpts(cx: &mut AppContext) {
    let buffer_1 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'a'), cx));
    let buffer_2 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'g'), cx));
    let buffer_3 = cx.new_model(|cx| Buffer::local(sample_text(6, 6, 'm'), cx));
    let multibuffer = cx.new_model(|_| MultiBuffer::new(Capability::ReadWrite));
    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.push_excerpts(
            buffer_1.clone(),
            [ExcerptRange {
                context: 0..buffer_1.read(cx).len(),
                primary: None,
            }],
            cx,
        );
        multibuffer.push_excerpts(
            buffer_2.clone(),
            [ExcerptRange {
                context: 0..buffer_2.read(cx).len(),
                primary: None,
            }],
            cx,
        );
        multibuffer.push_excerpts(
            buffer_3.clone(),
            [ExcerptRange {
                context: 0..buffer_3.read(cx).len(),
                primary: None,
            }],
            cx,
        );
    });

    let snapshot = multibuffer.read(cx).snapshot(cx);

    let buffer_1_len = buffer_1.read(cx).len();
    let buffer_2_len = buffer_2.read(cx).len();
    let buffer_3_len = buffer_3.read(cx).len();
    let buffer_2_start = buffer_1_len + '\n'.len_utf8();
    let buffer_3_start = buffer_2_start + buffer_2_len + '\n'.len_utf8();
    let buffer_1_midpoint = buffer_1_len / 2;
    let buffer_3_midpoint = buffer_3_start + buffer_3_len / 2;

    let input_range =
        snapshot.anchor_before(buffer_1_midpoint)..snapshot.anchor_after(buffer_3_midpoint);

    let actual_ranges = snapshot
        .split_ranges(std::iter::once(input_range))
        .map(|range| range.to_offset(&snapshot))
        .collect::<Vec<_>>();

    let expected_ranges = vec![
        buffer_1_midpoint..buffer_1_len,
        buffer_2_start..buffer_2_start + buffer_2_len,
        buffer_3_start..buffer_3_midpoint,
    ];

    assert_eq!(actual_ranges, expected_ranges);
}
