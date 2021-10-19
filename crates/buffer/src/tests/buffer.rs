use crate::*;
use clock::ReplicaId;
use rand::prelude::*;
use std::{
    cell::RefCell,
    cmp::Ordering,
    env,
    iter::Iterator,
    mem,
    rc::Rc,
    time::{Duration, Instant},
};

#[gpui::test]
fn test_edit(cx: &mut gpui::MutableAppContext) {
    cx.add_model(|cx| {
        let mut buffer = Buffer::new(0, "abc", cx);
        assert_eq!(buffer.text(), "abc");
        buffer.edit(vec![3..3], "def", cx);
        assert_eq!(buffer.text(), "abcdef");
        buffer.edit(vec![0..0], "ghi", cx);
        assert_eq!(buffer.text(), "ghiabcdef");
        buffer.edit(vec![5..5], "jkl", cx);
        assert_eq!(buffer.text(), "ghiabjklcdef");
        buffer.edit(vec![6..7], "", cx);
        assert_eq!(buffer.text(), "ghiabjlcdef");
        buffer.edit(vec![4..9], "mno", cx);
        assert_eq!(buffer.text(), "ghiamnoef");
        buffer
    });
}

#[gpui::test]
fn test_edit_events(cx: &mut gpui::MutableAppContext) {
    let mut now = Instant::now();
    let buffer_1_events = Rc::new(RefCell::new(Vec::new()));
    let buffer_2_events = Rc::new(RefCell::new(Vec::new()));

    let buffer1 = cx.add_model(|cx| Buffer::new(0, "abcdef", cx));
    let buffer2 = cx.add_model(|cx| Buffer::new(1, "abcdef", cx));
    let buffer_ops = buffer1.update(cx, |buffer, cx| {
        let buffer_1_events = buffer_1_events.clone();
        cx.subscribe(&buffer1, move |_, _, event, _| {
            buffer_1_events.borrow_mut().push(event.clone())
        })
        .detach();
        let buffer_2_events = buffer_2_events.clone();
        cx.subscribe(&buffer2, move |_, _, event, _| {
            buffer_2_events.borrow_mut().push(event.clone())
        })
        .detach();

        // An edit emits an edited event, followed by a dirtied event,
        // since the buffer was previously in a clean state.
        buffer.edit(Some(2..4), "XYZ", cx);

        // An empty transaction does not emit any events.
        buffer.start_transaction(None).unwrap();
        buffer.end_transaction(None, cx).unwrap();

        // A transaction containing two edits emits one edited event.
        now += Duration::from_secs(1);
        buffer.start_transaction_at(None, now).unwrap();
        buffer.edit(Some(5..5), "u", cx);
        buffer.edit(Some(6..6), "w", cx);
        buffer.end_transaction_at(None, now, cx).unwrap();

        // Undoing a transaction emits one edited event.
        buffer.undo(cx);

        buffer.operations.clone()
    });

    // Incorporating a set of remote ops emits a single edited event,
    // followed by a dirtied event.
    buffer2.update(cx, |buffer, cx| {
        buffer.apply_ops(buffer_ops, cx).unwrap();
    });

    let buffer_1_events = buffer_1_events.borrow();
    assert_eq!(
        *buffer_1_events,
        vec![Event::Edited, Event::Dirtied, Event::Edited, Event::Edited]
    );

    let buffer_2_events = buffer_2_events.borrow();
    assert_eq!(*buffer_2_events, vec![Event::Edited, Event::Dirtied]);
}

#[gpui::test(iterations = 100)]
fn test_random_edits(cx: &mut gpui::MutableAppContext, mut rng: StdRng) {
    let operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(10);

    let reference_string_len = rng.gen_range(0..3);
    let mut reference_string = RandomCharIter::new(&mut rng)
        .take(reference_string_len)
        .collect::<String>();
    cx.add_model(|cx| {
        let mut buffer = Buffer::new(0, reference_string.as_str(), cx);
        buffer.history.group_interval = Duration::from_millis(rng.gen_range(0..=200));
        let mut buffer_versions = Vec::new();
        log::info!(
            "buffer text {:?}, version: {:?}",
            buffer.text(),
            buffer.version()
        );

        for _i in 0..operations {
            let (old_ranges, new_text) = buffer.randomly_mutate(&mut rng, cx);
            for old_range in old_ranges.iter().rev() {
                reference_string.replace_range(old_range.clone(), &new_text);
            }
            assert_eq!(buffer.text(), reference_string);
            log::info!(
                "buffer text {:?}, version: {:?}",
                buffer.text(),
                buffer.version()
            );

            if rng.gen_bool(0.25) {
                buffer.randomly_undo_redo(&mut rng, cx);
                reference_string = buffer.text();
                log::info!(
                    "buffer text {:?}, version: {:?}",
                    buffer.text(),
                    buffer.version()
                );
            }

            let range = buffer.random_byte_range(0, &mut rng);
            assert_eq!(
                buffer.text_summary_for_range(range.clone()),
                TextSummary::from(&reference_string[range])
            );

            if rng.gen_bool(0.3) {
                buffer_versions.push(buffer.clone());
            }
        }

        for mut old_buffer in buffer_versions {
            let edits = buffer
                .edits_since(old_buffer.version.clone())
                .collect::<Vec<_>>();

            log::info!(
                "mutating old buffer version {:?}, text: {:?}, edits since: {:?}",
                old_buffer.version(),
                old_buffer.text(),
                edits,
            );

            let mut delta = 0_isize;
            for edit in edits {
                let old_start = (edit.old_bytes.start as isize + delta) as usize;
                let new_text: String = buffer.text_for_range(edit.new_bytes.clone()).collect();
                old_buffer.edit(
                    Some(old_start..old_start + edit.deleted_bytes()),
                    new_text,
                    cx,
                );
                delta += edit.delta();
            }
            assert_eq!(old_buffer.text(), buffer.text());
        }

        buffer
    });
}

#[gpui::test]
fn test_line_len(cx: &mut gpui::MutableAppContext) {
    cx.add_model(|cx| {
        let mut buffer = Buffer::new(0, "", cx);
        buffer.edit(vec![0..0], "abcd\nefg\nhij", cx);
        buffer.edit(vec![12..12], "kl\nmno", cx);
        buffer.edit(vec![18..18], "\npqrs\n", cx);
        buffer.edit(vec![18..21], "\nPQ", cx);

        assert_eq!(buffer.line_len(0), 4);
        assert_eq!(buffer.line_len(1), 3);
        assert_eq!(buffer.line_len(2), 5);
        assert_eq!(buffer.line_len(3), 3);
        assert_eq!(buffer.line_len(4), 4);
        assert_eq!(buffer.line_len(5), 0);
        buffer
    });
}

#[gpui::test]
fn test_text_summary_for_range(cx: &mut gpui::MutableAppContext) {
    cx.add_model(|cx| {
        let buffer = Buffer::new(0, "ab\nefg\nhklm\nnopqrs\ntuvwxyz", cx);
        assert_eq!(
            buffer.text_summary_for_range(1..3),
            TextSummary {
                bytes: 2,
                lines: Point::new(1, 0),
                first_line_chars: 1,
                last_line_chars: 0,
                longest_row: 0,
                longest_row_chars: 1,
            }
        );
        assert_eq!(
            buffer.text_summary_for_range(1..12),
            TextSummary {
                bytes: 11,
                lines: Point::new(3, 0),
                first_line_chars: 1,
                last_line_chars: 0,
                longest_row: 2,
                longest_row_chars: 4,
            }
        );
        assert_eq!(
            buffer.text_summary_for_range(0..20),
            TextSummary {
                bytes: 20,
                lines: Point::new(4, 1),
                first_line_chars: 2,
                last_line_chars: 1,
                longest_row: 3,
                longest_row_chars: 6,
            }
        );
        assert_eq!(
            buffer.text_summary_for_range(0..22),
            TextSummary {
                bytes: 22,
                lines: Point::new(4, 3),
                first_line_chars: 2,
                last_line_chars: 3,
                longest_row: 3,
                longest_row_chars: 6,
            }
        );
        assert_eq!(
            buffer.text_summary_for_range(7..22),
            TextSummary {
                bytes: 15,
                lines: Point::new(2, 3),
                first_line_chars: 4,
                last_line_chars: 3,
                longest_row: 1,
                longest_row_chars: 6,
            }
        );
        buffer
    });
}

#[gpui::test]
fn test_chars_at(cx: &mut gpui::MutableAppContext) {
    cx.add_model(|cx| {
        let mut buffer = Buffer::new(0, "", cx);
        buffer.edit(vec![0..0], "abcd\nefgh\nij", cx);
        buffer.edit(vec![12..12], "kl\nmno", cx);
        buffer.edit(vec![18..18], "\npqrs", cx);
        buffer.edit(vec![18..21], "\nPQ", cx);

        let chars = buffer.chars_at(Point::new(0, 0));
        assert_eq!(chars.collect::<String>(), "abcd\nefgh\nijkl\nmno\nPQrs");

        let chars = buffer.chars_at(Point::new(1, 0));
        assert_eq!(chars.collect::<String>(), "efgh\nijkl\nmno\nPQrs");

        let chars = buffer.chars_at(Point::new(2, 0));
        assert_eq!(chars.collect::<String>(), "ijkl\nmno\nPQrs");

        let chars = buffer.chars_at(Point::new(3, 0));
        assert_eq!(chars.collect::<String>(), "mno\nPQrs");

        let chars = buffer.chars_at(Point::new(4, 0));
        assert_eq!(chars.collect::<String>(), "PQrs");

        // Regression test:
        let mut buffer = Buffer::new(0, "", cx);
        buffer.edit(vec![0..0], "[workspace]\nmembers = [\n    \"xray_core\",\n    \"xray_server\",\n    \"xray_cli\",\n    \"xray_wasm\",\n]\n", cx);
        buffer.edit(vec![60..60], "\n", cx);

        let chars = buffer.chars_at(Point::new(6, 0));
        assert_eq!(chars.collect::<String>(), "    \"xray_wasm\",\n]\n");

        buffer
    });
}

#[gpui::test]
fn test_anchors(cx: &mut gpui::MutableAppContext) {
    cx.add_model(|cx| {
        let mut buffer = Buffer::new(0, "", cx);
        buffer.edit(vec![0..0], "abc", cx);
        let left_anchor = buffer.anchor_before(2);
        let right_anchor = buffer.anchor_after(2);

        buffer.edit(vec![1..1], "def\n", cx);
        assert_eq!(buffer.text(), "adef\nbc");
        assert_eq!(left_anchor.to_offset(&buffer), 6);
        assert_eq!(right_anchor.to_offset(&buffer), 6);
        assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
        assert_eq!(right_anchor.to_point(&buffer), Point { row: 1, column: 1 });

        buffer.edit(vec![2..3], "", cx);
        assert_eq!(buffer.text(), "adf\nbc");
        assert_eq!(left_anchor.to_offset(&buffer), 5);
        assert_eq!(right_anchor.to_offset(&buffer), 5);
        assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
        assert_eq!(right_anchor.to_point(&buffer), Point { row: 1, column: 1 });

        buffer.edit(vec![5..5], "ghi\n", cx);
        assert_eq!(buffer.text(), "adf\nbghi\nc");
        assert_eq!(left_anchor.to_offset(&buffer), 5);
        assert_eq!(right_anchor.to_offset(&buffer), 9);
        assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
        assert_eq!(right_anchor.to_point(&buffer), Point { row: 2, column: 0 });

        buffer.edit(vec![7..9], "", cx);
        assert_eq!(buffer.text(), "adf\nbghc");
        assert_eq!(left_anchor.to_offset(&buffer), 5);
        assert_eq!(right_anchor.to_offset(&buffer), 7);
        assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 },);
        assert_eq!(right_anchor.to_point(&buffer), Point { row: 1, column: 3 });

        // Ensure anchoring to a point is equivalent to anchoring to an offset.
        assert_eq!(
            buffer.anchor_before(Point { row: 0, column: 0 }),
            buffer.anchor_before(0)
        );
        assert_eq!(
            buffer.anchor_before(Point { row: 0, column: 1 }),
            buffer.anchor_before(1)
        );
        assert_eq!(
            buffer.anchor_before(Point { row: 0, column: 2 }),
            buffer.anchor_before(2)
        );
        assert_eq!(
            buffer.anchor_before(Point { row: 0, column: 3 }),
            buffer.anchor_before(3)
        );
        assert_eq!(
            buffer.anchor_before(Point { row: 1, column: 0 }),
            buffer.anchor_before(4)
        );
        assert_eq!(
            buffer.anchor_before(Point { row: 1, column: 1 }),
            buffer.anchor_before(5)
        );
        assert_eq!(
            buffer.anchor_before(Point { row: 1, column: 2 }),
            buffer.anchor_before(6)
        );
        assert_eq!(
            buffer.anchor_before(Point { row: 1, column: 3 }),
            buffer.anchor_before(7)
        );
        assert_eq!(
            buffer.anchor_before(Point { row: 1, column: 4 }),
            buffer.anchor_before(8)
        );

        // Comparison between anchors.
        let anchor_at_offset_0 = buffer.anchor_before(0);
        let anchor_at_offset_1 = buffer.anchor_before(1);
        let anchor_at_offset_2 = buffer.anchor_before(2);

        assert_eq!(
            anchor_at_offset_0
                .cmp(&anchor_at_offset_0, &buffer)
                .unwrap(),
            Ordering::Equal
        );
        assert_eq!(
            anchor_at_offset_1
                .cmp(&anchor_at_offset_1, &buffer)
                .unwrap(),
            Ordering::Equal
        );
        assert_eq!(
            anchor_at_offset_2
                .cmp(&anchor_at_offset_2, &buffer)
                .unwrap(),
            Ordering::Equal
        );

        assert_eq!(
            anchor_at_offset_0
                .cmp(&anchor_at_offset_1, &buffer)
                .unwrap(),
            Ordering::Less
        );
        assert_eq!(
            anchor_at_offset_1
                .cmp(&anchor_at_offset_2, &buffer)
                .unwrap(),
            Ordering::Less
        );
        assert_eq!(
            anchor_at_offset_0
                .cmp(&anchor_at_offset_2, &buffer)
                .unwrap(),
            Ordering::Less
        );

        assert_eq!(
            anchor_at_offset_1
                .cmp(&anchor_at_offset_0, &buffer)
                .unwrap(),
            Ordering::Greater
        );
        assert_eq!(
            anchor_at_offset_2
                .cmp(&anchor_at_offset_1, &buffer)
                .unwrap(),
            Ordering::Greater
        );
        assert_eq!(
            anchor_at_offset_2
                .cmp(&anchor_at_offset_0, &buffer)
                .unwrap(),
            Ordering::Greater
        );
        buffer
    });
}

#[gpui::test]
fn test_anchors_at_start_and_end(cx: &mut gpui::MutableAppContext) {
    cx.add_model(|cx| {
        let mut buffer = Buffer::new(0, "", cx);
        let before_start_anchor = buffer.anchor_before(0);
        let after_end_anchor = buffer.anchor_after(0);

        buffer.edit(vec![0..0], "abc", cx);
        assert_eq!(buffer.text(), "abc");
        assert_eq!(before_start_anchor.to_offset(&buffer), 0);
        assert_eq!(after_end_anchor.to_offset(&buffer), 3);

        let after_start_anchor = buffer.anchor_after(0);
        let before_end_anchor = buffer.anchor_before(3);

        buffer.edit(vec![3..3], "def", cx);
        buffer.edit(vec![0..0], "ghi", cx);
        assert_eq!(buffer.text(), "ghiabcdef");
        assert_eq!(before_start_anchor.to_offset(&buffer), 0);
        assert_eq!(after_start_anchor.to_offset(&buffer), 3);
        assert_eq!(before_end_anchor.to_offset(&buffer), 6);
        assert_eq!(after_end_anchor.to_offset(&buffer), 9);
        buffer
    });
}

#[gpui::test]
async fn test_apply_diff(mut cx: gpui::TestAppContext) {
    let text = "a\nbb\nccc\ndddd\neeeee\nffffff\n";
    let buffer = cx.add_model(|cx| Buffer::new(0, text, cx));

    let text = "a\nccc\ndddd\nffffff\n";
    let diff = buffer.read_with(&cx, |b, cx| b.diff(text.into(), cx)).await;
    buffer.update(&mut cx, |b, cx| b.apply_diff(diff, cx));
    cx.read(|cx| assert_eq!(buffer.read(cx).text(), text));

    let text = "a\n1\n\nccc\ndd2dd\nffffff\n";
    let diff = buffer.read_with(&cx, |b, cx| b.diff(text.into(), cx)).await;
    buffer.update(&mut cx, |b, cx| b.apply_diff(diff, cx));
    cx.read(|cx| assert_eq!(buffer.read(cx).text(), text));
}

#[gpui::test]
fn test_undo_redo(cx: &mut gpui::MutableAppContext) {
    cx.add_model(|cx| {
        let mut buffer = Buffer::new(0, "1234", cx);
        // Set group interval to zero so as to not group edits in the undo stack.
        buffer.history.group_interval = Duration::from_secs(0);

        buffer.edit(vec![1..1], "abx", cx);
        buffer.edit(vec![3..4], "yzef", cx);
        buffer.edit(vec![3..5], "cd", cx);
        assert_eq!(buffer.text(), "1abcdef234");

        let transactions = buffer.history.undo_stack.clone();
        assert_eq!(transactions.len(), 3);

        buffer.undo_or_redo(transactions[0].clone(), cx).unwrap();
        assert_eq!(buffer.text(), "1cdef234");
        buffer.undo_or_redo(transactions[0].clone(), cx).unwrap();
        assert_eq!(buffer.text(), "1abcdef234");

        buffer.undo_or_redo(transactions[1].clone(), cx).unwrap();
        assert_eq!(buffer.text(), "1abcdx234");
        buffer.undo_or_redo(transactions[2].clone(), cx).unwrap();
        assert_eq!(buffer.text(), "1abx234");
        buffer.undo_or_redo(transactions[1].clone(), cx).unwrap();
        assert_eq!(buffer.text(), "1abyzef234");
        buffer.undo_or_redo(transactions[2].clone(), cx).unwrap();
        assert_eq!(buffer.text(), "1abcdef234");

        buffer.undo_or_redo(transactions[2].clone(), cx).unwrap();
        assert_eq!(buffer.text(), "1abyzef234");
        buffer.undo_or_redo(transactions[0].clone(), cx).unwrap();
        assert_eq!(buffer.text(), "1yzef234");
        buffer.undo_or_redo(transactions[1].clone(), cx).unwrap();
        assert_eq!(buffer.text(), "1234");

        buffer
    });
}

#[gpui::test]
fn test_history(cx: &mut gpui::MutableAppContext) {
    cx.add_model(|cx| {
        let mut now = Instant::now();
        let mut buffer = Buffer::new(0, "123456", cx);

        let set_id =
            buffer.add_selection_set(buffer.selections_from_ranges(vec![4..4]).unwrap(), cx);
        buffer.start_transaction_at(Some(set_id), now).unwrap();
        buffer.edit(vec![2..4], "cd", cx);
        buffer.end_transaction_at(Some(set_id), now, cx).unwrap();
        assert_eq!(buffer.text(), "12cd56");
        assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![4..4]);

        buffer.start_transaction_at(Some(set_id), now).unwrap();
        buffer
            .update_selection_set(
                set_id,
                buffer.selections_from_ranges(vec![1..3]).unwrap(),
                cx,
            )
            .unwrap();
        buffer.edit(vec![4..5], "e", cx);
        buffer.end_transaction_at(Some(set_id), now, cx).unwrap();
        assert_eq!(buffer.text(), "12cde6");
        assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![1..3]);

        now += buffer.history.group_interval + Duration::from_millis(1);
        buffer.start_transaction_at(Some(set_id), now).unwrap();
        buffer
            .update_selection_set(
                set_id,
                buffer.selections_from_ranges(vec![2..2]).unwrap(),
                cx,
            )
            .unwrap();
        buffer.edit(vec![0..1], "a", cx);
        buffer.edit(vec![1..1], "b", cx);
        buffer.end_transaction_at(Some(set_id), now, cx).unwrap();
        assert_eq!(buffer.text(), "ab2cde6");
        assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![3..3]);

        // Last transaction happened past the group interval, undo it on its
        // own.
        buffer.undo(cx);
        assert_eq!(buffer.text(), "12cde6");
        assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![1..3]);

        // First two transactions happened within the group interval, undo them
        // together.
        buffer.undo(cx);
        assert_eq!(buffer.text(), "123456");
        assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![4..4]);

        // Redo the first two transactions together.
        buffer.redo(cx);
        assert_eq!(buffer.text(), "12cde6");
        assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![1..3]);

        // Redo the last transaction on its own.
        buffer.redo(cx);
        assert_eq!(buffer.text(), "ab2cde6");
        assert_eq!(buffer.selection_ranges(set_id).unwrap(), vec![3..3]);

        buffer.start_transaction_at(None, now).unwrap();
        buffer.end_transaction_at(None, now, cx).unwrap();
        buffer.undo(cx);
        assert_eq!(buffer.text(), "12cde6");

        buffer
    });
}

#[gpui::test]
fn test_concurrent_edits(cx: &mut gpui::MutableAppContext) {
    let text = "abcdef";

    let buffer1 = cx.add_model(|cx| Buffer::new(1, text, cx));
    let buffer2 = cx.add_model(|cx| Buffer::new(2, text, cx));
    let buffer3 = cx.add_model(|cx| Buffer::new(3, text, cx));

    let buf1_op = buffer1.update(cx, |buffer, cx| {
        buffer.edit(vec![1..2], "12", cx);
        assert_eq!(buffer.text(), "a12cdef");
        buffer.operations.last().unwrap().clone()
    });
    let buf2_op = buffer2.update(cx, |buffer, cx| {
        buffer.edit(vec![3..4], "34", cx);
        assert_eq!(buffer.text(), "abc34ef");
        buffer.operations.last().unwrap().clone()
    });
    let buf3_op = buffer3.update(cx, |buffer, cx| {
        buffer.edit(vec![5..6], "56", cx);
        assert_eq!(buffer.text(), "abcde56");
        buffer.operations.last().unwrap().clone()
    });

    buffer1.update(cx, |buffer, _| {
        buffer.apply_op(buf2_op.clone()).unwrap();
        buffer.apply_op(buf3_op.clone()).unwrap();
    });
    buffer2.update(cx, |buffer, _| {
        buffer.apply_op(buf1_op.clone()).unwrap();
        buffer.apply_op(buf3_op.clone()).unwrap();
    });
    buffer3.update(cx, |buffer, _| {
        buffer.apply_op(buf1_op.clone()).unwrap();
        buffer.apply_op(buf2_op.clone()).unwrap();
    });

    assert_eq!(buffer1.read(cx).text(), "a12c34e56");
    assert_eq!(buffer2.read(cx).text(), "a12c34e56");
    assert_eq!(buffer3.read(cx).text(), "a12c34e56");
}

#[gpui::test(iterations = 100)]
fn test_random_concurrent_edits(cx: &mut gpui::MutableAppContext, mut rng: StdRng) {
    let peers = env::var("PEERS")
        .map(|i| i.parse().expect("invalid `PEERS` variable"))
        .unwrap_or(5);
    let operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(10);

    let base_text_len = rng.gen_range(0..10);
    let base_text = RandomCharIter::new(&mut rng)
        .take(base_text_len)
        .collect::<String>();
    let mut replica_ids = Vec::new();
    let mut buffers = Vec::new();
    let mut network = Network::new(rng.clone());

    for i in 0..peers {
        let buffer = cx.add_model(|cx| {
            let mut buf = Buffer::new(i as ReplicaId, base_text.as_str(), cx);
            buf.history.group_interval = Duration::from_millis(rng.gen_range(0..=200));
            buf
        });
        buffers.push(buffer);
        replica_ids.push(i as u16);
        network.add_peer(i as u16);
    }

    log::info!("initial text: {:?}", base_text);

    let mut mutation_count = operations;
    loop {
        let replica_index = rng.gen_range(0..peers);
        let replica_id = replica_ids[replica_index];
        buffers[replica_index].update(cx, |buffer, cx| match rng.gen_range(0..=100) {
            0..=50 if mutation_count != 0 => {
                buffer.randomly_mutate(&mut rng, cx);
                network.broadcast(buffer.replica_id, mem::take(&mut buffer.operations));
                log::info!("buffer {} text: {:?}", buffer.replica_id, buffer.text());
                mutation_count -= 1;
            }
            51..=70 if mutation_count != 0 => {
                buffer.randomly_undo_redo(&mut rng, cx);
                network.broadcast(buffer.replica_id, mem::take(&mut buffer.operations));
                mutation_count -= 1;
            }
            71..=100 if network.has_unreceived(replica_id) => {
                let ops = network.receive(replica_id);
                if !ops.is_empty() {
                    log::info!(
                        "peer {} applying {} ops from the network.",
                        replica_id,
                        ops.len()
                    );
                    buffer.apply_ops(ops, cx).unwrap();
                }
            }
            _ => {}
        });

        if mutation_count == 0 && network.is_idle() {
            break;
        }
    }

    let first_buffer = buffers[0].read(cx);
    for buffer in &buffers[1..] {
        let buffer = buffer.read(cx);
        assert_eq!(
            buffer.text(),
            first_buffer.text(),
            "Replica {} text != Replica 0 text",
            buffer.replica_id
        );
        assert_eq!(
            buffer.selection_sets().collect::<HashMap<_, _>>(),
            first_buffer.selection_sets().collect::<HashMap<_, _>>()
        );
        assert_eq!(
            buffer.all_selection_ranges().collect::<HashMap<_, _>>(),
            first_buffer
                .all_selection_ranges()
                .collect::<HashMap<_, _>>()
        );
    }
}

#[derive(Clone)]
struct Envelope<T: Clone> {
    message: T,
    sender: ReplicaId,
}

struct Network<T: Clone, R: rand::Rng> {
    inboxes: std::collections::BTreeMap<ReplicaId, Vec<Envelope<T>>>,
    all_messages: Vec<T>,
    rng: R,
}

impl<T: Clone, R: rand::Rng> Network<T, R> {
    fn new(rng: R) -> Self {
        Network {
            inboxes: Default::default(),
            all_messages: Vec::new(),
            rng,
        }
    }

    fn add_peer(&mut self, id: ReplicaId) {
        self.inboxes.insert(id, Vec::new());
    }

    fn is_idle(&self) -> bool {
        self.inboxes.values().all(|i| i.is_empty())
    }

    fn broadcast(&mut self, sender: ReplicaId, messages: Vec<T>) {
        for (replica, inbox) in self.inboxes.iter_mut() {
            if *replica != sender {
                for message in &messages {
                    let min_index = inbox
                        .iter()
                        .enumerate()
                        .rev()
                        .find_map(|(index, envelope)| {
                            if sender == envelope.sender {
                                Some(index + 1)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(0);

                    // Insert one or more duplicates of this message *after* the previous
                    // message delivered by this replica.
                    for _ in 0..self.rng.gen_range(1..4) {
                        let insertion_index = self.rng.gen_range(min_index..inbox.len() + 1);
                        inbox.insert(
                            insertion_index,
                            Envelope {
                                message: message.clone(),
                                sender,
                            },
                        );
                    }
                }
            }
        }
        self.all_messages.extend(messages);
    }

    fn has_unreceived(&self, receiver: ReplicaId) -> bool {
        !self.inboxes[&receiver].is_empty()
    }

    fn receive(&mut self, receiver: ReplicaId) -> Vec<T> {
        let inbox = self.inboxes.get_mut(&receiver).unwrap();
        let count = self.rng.gen_range(0..inbox.len() + 1);
        inbox
            .drain(0..count)
            .map(|envelope| envelope.message)
            .collect()
    }
}
