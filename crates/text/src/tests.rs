use super::{network::Network, *};
use clock::ReplicaId;
use rand::prelude::*;
use std::{
    cmp::Ordering,
    env,
    iter::Iterator,
    time::{Duration, Instant},
};

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

#[test]
fn test_edit() {
    let mut buffer = Buffer::new(0, 0, History::new("abc".into()));
    assert_eq!(buffer.text(), "abc");
    buffer.edit(vec![3..3], "def");
    assert_eq!(buffer.text(), "abcdef");
    buffer.edit(vec![0..0], "ghi");
    assert_eq!(buffer.text(), "ghiabcdef");
    buffer.edit(vec![5..5], "jkl");
    assert_eq!(buffer.text(), "ghiabjklcdef");
    buffer.edit(vec![6..7], "");
    assert_eq!(buffer.text(), "ghiabjlcdef");
    buffer.edit(vec![4..9], "mno");
    assert_eq!(buffer.text(), "ghiamnoef");
}

#[gpui::test(iterations = 100)]
fn test_random_edits(mut rng: StdRng) {
    let operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(10);

    let reference_string_len = rng.gen_range(0..3);
    let mut reference_string = RandomCharIter::new(&mut rng)
        .take(reference_string_len)
        .collect::<String>();
    let mut buffer = Buffer::new(0, 0, History::new(reference_string.clone().into()));
    buffer.history.group_interval = Duration::from_millis(rng.gen_range(0..=200));
    let mut buffer_versions = Vec::new();
    log::info!(
        "buffer text {:?}, version: {:?}",
        buffer.text(),
        buffer.version()
    );

    for _i in 0..operations {
        let (old_ranges, new_text, _) = buffer.randomly_edit(&mut rng, 5);
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
            buffer.randomly_undo_redo(&mut rng);
            reference_string = buffer.text();
            log::info!(
                "buffer text {:?}, version: {:?}",
                buffer.text(),
                buffer.version()
            );
        }

        let range = buffer.random_byte_range(0, &mut rng);
        assert_eq!(
            buffer.text_summary_for_range::<TextSummary, _>(range.clone()),
            TextSummary::from(&reference_string[range])
        );

        buffer.check_invariants();

        if rng.gen_bool(0.3) {
            buffer_versions.push((buffer.clone(), buffer.subscribe()));
        }
    }

    for (old_buffer, subscription) in buffer_versions {
        let edits = buffer
            .edits_since::<usize>(&old_buffer.version)
            .collect::<Vec<_>>();

        log::info!(
            "applying edits since version {:?} to old text: {:?}: {:?}",
            old_buffer.version(),
            old_buffer.text(),
            edits,
        );

        let mut text = old_buffer.visible_text.clone();
        for edit in edits {
            let new_text: String = buffer.text_for_range(edit.new.clone()).collect();
            text.replace(edit.new.start..edit.new.start + edit.old.len(), &new_text);
        }
        assert_eq!(text.to_string(), buffer.text());

        for _ in 0..5 {
            let end_ix = old_buffer.clip_offset(rng.gen_range(0..=old_buffer.len()), Bias::Right);
            let start_ix = old_buffer.clip_offset(rng.gen_range(0..=end_ix), Bias::Left);
            let range = old_buffer.anchor_before(start_ix)..old_buffer.anchor_after(end_ix);
            let mut old_text = old_buffer.text_for_range(range.clone()).collect::<String>();
            let edits = buffer
                .edits_since_in_range::<usize>(&old_buffer.version, range.clone())
                .collect::<Vec<_>>();
            log::info!(
                "applying edits since version {:?} to old text in range {:?}: {:?}: {:?}",
                old_buffer.version(),
                start_ix..end_ix,
                old_text,
                edits,
            );

            let new_text = buffer.text_for_range(range).collect::<String>();
            for edit in edits {
                old_text.replace_range(
                    edit.new.start..edit.new.start + edit.old_len(),
                    &new_text[edit.new],
                );
            }
            assert_eq!(old_text, new_text);
        }

        let subscription_edits = subscription.consume();
        log::info!(
            "applying subscription edits since version {:?} to old text: {:?}: {:?}",
            old_buffer.version(),
            old_buffer.text(),
            subscription_edits,
        );

        let mut text = old_buffer.visible_text.clone();
        for edit in subscription_edits.into_inner() {
            let new_text: String = buffer.text_for_range(edit.new.clone()).collect();
            text.replace(edit.new.start..edit.new.start + edit.old.len(), &new_text);
        }
        assert_eq!(text.to_string(), buffer.text());
    }
}

#[test]
fn test_line_len() {
    let mut buffer = Buffer::new(0, 0, History::new("".into()));
    buffer.edit(vec![0..0], "abcd\nefg\nhij");
    buffer.edit(vec![12..12], "kl\nmno");
    buffer.edit(vec![18..18], "\npqrs\n");
    buffer.edit(vec![18..21], "\nPQ");

    assert_eq!(buffer.line_len(0), 4);
    assert_eq!(buffer.line_len(1), 3);
    assert_eq!(buffer.line_len(2), 5);
    assert_eq!(buffer.line_len(3), 3);
    assert_eq!(buffer.line_len(4), 4);
    assert_eq!(buffer.line_len(5), 0);
}

#[test]
fn test_text_summary_for_range() {
    let buffer = Buffer::new(0, 0, History::new("ab\nefg\nhklm\nnopqrs\ntuvwxyz".into()));
    assert_eq!(
        buffer.text_summary_for_range::<TextSummary, _>(1..3),
        TextSummary {
            bytes: 2,
            lines: Point::new(1, 0),
            lines_utf16: PointUtf16::new(1, 0),
            first_line_chars: 1,
            last_line_chars: 0,
            longest_row: 0,
            longest_row_chars: 1,
        }
    );
    assert_eq!(
        buffer.text_summary_for_range::<TextSummary, _>(1..12),
        TextSummary {
            bytes: 11,
            lines: Point::new(3, 0),
            lines_utf16: PointUtf16::new(3, 0),
            first_line_chars: 1,
            last_line_chars: 0,
            longest_row: 2,
            longest_row_chars: 4,
        }
    );
    assert_eq!(
        buffer.text_summary_for_range::<TextSummary, _>(0..20),
        TextSummary {
            bytes: 20,
            lines: Point::new(4, 1),
            lines_utf16: PointUtf16::new(4, 1),
            first_line_chars: 2,
            last_line_chars: 1,
            longest_row: 3,
            longest_row_chars: 6,
        }
    );
    assert_eq!(
        buffer.text_summary_for_range::<TextSummary, _>(0..22),
        TextSummary {
            bytes: 22,
            lines: Point::new(4, 3),
            lines_utf16: PointUtf16::new(4, 3),
            first_line_chars: 2,
            last_line_chars: 3,
            longest_row: 3,
            longest_row_chars: 6,
        }
    );
    assert_eq!(
        buffer.text_summary_for_range::<TextSummary, _>(7..22),
        TextSummary {
            bytes: 15,
            lines: Point::new(2, 3),
            lines_utf16: PointUtf16::new(2, 3),
            first_line_chars: 4,
            last_line_chars: 3,
            longest_row: 1,
            longest_row_chars: 6,
        }
    );
}

#[test]
fn test_chars_at() {
    let mut buffer = Buffer::new(0, 0, History::new("".into()));
    buffer.edit(vec![0..0], "abcd\nefgh\nij");
    buffer.edit(vec![12..12], "kl\nmno");
    buffer.edit(vec![18..18], "\npqrs");
    buffer.edit(vec![18..21], "\nPQ");

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
    let mut buffer = Buffer::new(0, 0, History::new("".into()));
    buffer.edit(vec![0..0], "[workspace]\nmembers = [\n    \"xray_core\",\n    \"xray_server\",\n    \"xray_cli\",\n    \"xray_wasm\",\n]\n");
    buffer.edit(vec![60..60], "\n");

    let chars = buffer.chars_at(Point::new(6, 0));
    assert_eq!(chars.collect::<String>(), "    \"xray_wasm\",\n]\n");
}

#[test]
fn test_anchors() {
    let mut buffer = Buffer::new(0, 0, History::new("".into()));
    buffer.edit(vec![0..0], "abc");
    let left_anchor = buffer.anchor_before(2);
    let right_anchor = buffer.anchor_after(2);

    buffer.edit(vec![1..1], "def\n");
    assert_eq!(buffer.text(), "adef\nbc");
    assert_eq!(left_anchor.to_offset(&buffer), 6);
    assert_eq!(right_anchor.to_offset(&buffer), 6);
    assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
    assert_eq!(right_anchor.to_point(&buffer), Point { row: 1, column: 1 });

    buffer.edit(vec![2..3], "");
    assert_eq!(buffer.text(), "adf\nbc");
    assert_eq!(left_anchor.to_offset(&buffer), 5);
    assert_eq!(right_anchor.to_offset(&buffer), 5);
    assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
    assert_eq!(right_anchor.to_point(&buffer), Point { row: 1, column: 1 });

    buffer.edit(vec![5..5], "ghi\n");
    assert_eq!(buffer.text(), "adf\nbghi\nc");
    assert_eq!(left_anchor.to_offset(&buffer), 5);
    assert_eq!(right_anchor.to_offset(&buffer), 9);
    assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
    assert_eq!(right_anchor.to_point(&buffer), Point { row: 2, column: 0 });

    buffer.edit(vec![7..9], "");
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
}

#[test]
fn test_anchors_at_start_and_end() {
    let mut buffer = Buffer::new(0, 0, History::new("".into()));
    let before_start_anchor = buffer.anchor_before(0);
    let after_end_anchor = buffer.anchor_after(0);

    buffer.edit(vec![0..0], "abc");
    assert_eq!(buffer.text(), "abc");
    assert_eq!(before_start_anchor.to_offset(&buffer), 0);
    assert_eq!(after_end_anchor.to_offset(&buffer), 3);

    let after_start_anchor = buffer.anchor_after(0);
    let before_end_anchor = buffer.anchor_before(3);

    buffer.edit(vec![3..3], "def");
    buffer.edit(vec![0..0], "ghi");
    assert_eq!(buffer.text(), "ghiabcdef");
    assert_eq!(before_start_anchor.to_offset(&buffer), 0);
    assert_eq!(after_start_anchor.to_offset(&buffer), 3);
    assert_eq!(before_end_anchor.to_offset(&buffer), 6);
    assert_eq!(after_end_anchor.to_offset(&buffer), 9);
}

#[test]
fn test_undo_redo() {
    let mut buffer = Buffer::new(0, 0, History::new("1234".into()));
    // Set group interval to zero so as to not group edits in the undo stack.
    buffer.history.group_interval = Duration::from_secs(0);

    buffer.edit(vec![1..1], "abx");
    buffer.edit(vec![3..4], "yzef");
    buffer.edit(vec![3..5], "cd");
    assert_eq!(buffer.text(), "1abcdef234");

    let entries = buffer.history.undo_stack.clone();
    assert_eq!(entries.len(), 3);

    buffer.undo_or_redo(entries[0].transaction.clone()).unwrap();
    assert_eq!(buffer.text(), "1cdef234");
    buffer.undo_or_redo(entries[0].transaction.clone()).unwrap();
    assert_eq!(buffer.text(), "1abcdef234");

    buffer.undo_or_redo(entries[1].transaction.clone()).unwrap();
    assert_eq!(buffer.text(), "1abcdx234");
    buffer.undo_or_redo(entries[2].transaction.clone()).unwrap();
    assert_eq!(buffer.text(), "1abx234");
    buffer.undo_or_redo(entries[1].transaction.clone()).unwrap();
    assert_eq!(buffer.text(), "1abyzef234");
    buffer.undo_or_redo(entries[2].transaction.clone()).unwrap();
    assert_eq!(buffer.text(), "1abcdef234");

    buffer.undo_or_redo(entries[2].transaction.clone()).unwrap();
    assert_eq!(buffer.text(), "1abyzef234");
    buffer.undo_or_redo(entries[0].transaction.clone()).unwrap();
    assert_eq!(buffer.text(), "1yzef234");
    buffer.undo_or_redo(entries[1].transaction.clone()).unwrap();
    assert_eq!(buffer.text(), "1234");
}

#[test]
fn test_history() {
    let mut now = Instant::now();
    let mut buffer = Buffer::new(0, 0, History::new("123456".into()));

    buffer.start_transaction_at(now);
    buffer.edit(vec![2..4], "cd");
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "12cd56");

    buffer.start_transaction_at(now);
    buffer.edit(vec![4..5], "e");
    buffer.end_transaction_at(now).unwrap();
    assert_eq!(buffer.text(), "12cde6");

    now += buffer.history.group_interval + Duration::from_millis(1);
    buffer.start_transaction_at(now);
    buffer.edit(vec![0..1], "a");
    buffer.edit(vec![1..1], "b");
    buffer.end_transaction_at(now).unwrap();
    assert_eq!(buffer.text(), "ab2cde6");

    // Last transaction happened past the group interval, undo it on its own.
    buffer.undo();
    assert_eq!(buffer.text(), "12cde6");

    // First two transactions happened within the group interval, undo them together.
    buffer.undo();
    assert_eq!(buffer.text(), "123456");

    // Redo the first two transactions together.
    buffer.redo();
    assert_eq!(buffer.text(), "12cde6");

    // Redo the last transaction on its own.
    buffer.redo();
    assert_eq!(buffer.text(), "ab2cde6");

    buffer.start_transaction_at(now);
    assert!(buffer.end_transaction_at(now).is_none());
    buffer.undo();
    assert_eq!(buffer.text(), "12cde6");
}

#[test]
fn test_finalize_last_transaction() {
    let now = Instant::now();
    let mut buffer = Buffer::new(0, 0, History::new("123456".into()));

    buffer.start_transaction_at(now);
    buffer.edit(vec![2..4], "cd");
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "12cd56");

    buffer.finalize_last_transaction();
    buffer.start_transaction_at(now);
    buffer.edit(vec![4..5], "e");
    buffer.end_transaction_at(now).unwrap();
    assert_eq!(buffer.text(), "12cde6");

    buffer.start_transaction_at(now);
    buffer.edit(vec![0..1], "a");
    buffer.edit(vec![1..1], "b");
    buffer.end_transaction_at(now).unwrap();
    assert_eq!(buffer.text(), "ab2cde6");

    buffer.undo();
    assert_eq!(buffer.text(), "12cd56");

    buffer.undo();
    assert_eq!(buffer.text(), "123456");

    buffer.redo();
    assert_eq!(buffer.text(), "12cd56");

    buffer.redo();
    assert_eq!(buffer.text(), "ab2cde6");
}

#[test]
fn test_edited_ranges_for_transaction() {
    let now = Instant::now();
    let mut buffer = Buffer::new(0, 0, History::new("1234567".into()));

    buffer.start_transaction_at(now);
    buffer.edit(vec![2..4], "cd");
    buffer.edit(vec![6..6], "efg");
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "12cd56efg7");

    let tx = buffer.finalize_last_transaction().unwrap().clone();
    assert_eq!(
        buffer
            .edited_ranges_for_transaction::<usize>(&tx)
            .collect::<Vec<_>>(),
        [2..4, 6..9]
    );

    buffer.edit(vec![5..5], "hijk");
    assert_eq!(buffer.text(), "12cd5hijk6efg7");
    assert_eq!(
        buffer
            .edited_ranges_for_transaction::<usize>(&tx)
            .collect::<Vec<_>>(),
        [2..4, 10..13]
    );

    buffer.edit(vec![4..4], "l");
    assert_eq!(buffer.text(), "12cdl5hijk6efg7");
    assert_eq!(
        buffer
            .edited_ranges_for_transaction::<usize>(&tx)
            .collect::<Vec<_>>(),
        [2..4, 11..14]
    );
}

#[test]
fn test_concurrent_edits() {
    let text = "abcdef";

    let mut buffer1 = Buffer::new(1, 0, History::new(text.into()));
    let mut buffer2 = Buffer::new(2, 0, History::new(text.into()));
    let mut buffer3 = Buffer::new(3, 0, History::new(text.into()));

    let buf1_op = buffer1.edit(vec![1..2], "12");
    assert_eq!(buffer1.text(), "a12cdef");
    let buf2_op = buffer2.edit(vec![3..4], "34");
    assert_eq!(buffer2.text(), "abc34ef");
    let buf3_op = buffer3.edit(vec![5..6], "56");
    assert_eq!(buffer3.text(), "abcde56");

    buffer1.apply_op(buf2_op.clone()).unwrap();
    buffer1.apply_op(buf3_op.clone()).unwrap();
    buffer2.apply_op(buf1_op.clone()).unwrap();
    buffer2.apply_op(buf3_op.clone()).unwrap();
    buffer3.apply_op(buf1_op.clone()).unwrap();
    buffer3.apply_op(buf2_op.clone()).unwrap();

    assert_eq!(buffer1.text(), "a12c34e56");
    assert_eq!(buffer2.text(), "a12c34e56");
    assert_eq!(buffer3.text(), "a12c34e56");
}

#[gpui::test(iterations = 100)]
fn test_random_concurrent_edits(mut rng: StdRng) {
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
        let mut buffer = Buffer::new(i as ReplicaId, 0, History::new(base_text.clone().into()));
        buffer.history.group_interval = Duration::from_millis(rng.gen_range(0..=200));
        buffers.push(buffer);
        replica_ids.push(i as u16);
        network.add_peer(i as u16);
    }

    log::info!("initial text: {:?}", base_text);

    let mut mutation_count = operations;
    loop {
        let replica_index = rng.gen_range(0..peers);
        let replica_id = replica_ids[replica_index];
        let buffer = &mut buffers[replica_index];
        match rng.gen_range(0..=100) {
            0..=50 if mutation_count != 0 => {
                let op = buffer.randomly_edit(&mut rng, 5).2;
                network.broadcast(buffer.replica_id, vec![op]);
                log::info!("buffer {} text: {:?}", buffer.replica_id, buffer.text());
                mutation_count -= 1;
            }
            51..=70 if mutation_count != 0 => {
                let ops = buffer.randomly_undo_redo(&mut rng);
                network.broadcast(buffer.replica_id, ops);
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
                    buffer.apply_ops(ops).unwrap();
                }
            }
            _ => {}
        }
        buffer.check_invariants();

        if mutation_count == 0 && network.is_idle() {
            break;
        }
    }

    let first_buffer = &buffers[0];
    for buffer in &buffers[1..] {
        assert_eq!(
            buffer.text(),
            first_buffer.text(),
            "Replica {} text != Replica 0 text",
            buffer.replica_id
        );
        buffer.check_invariants();
    }
}
