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
    zlog::init_test();
}

#[test]
fn test_edit() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "abc");
    assert_eq!(buffer.text(), "abc");
    buffer.edit([(3..3, "def")]);
    assert_eq!(buffer.text(), "abcdef");
    buffer.edit([(0..0, "ghi")]);
    assert_eq!(buffer.text(), "ghiabcdef");
    buffer.edit([(5..5, "jkl")]);
    assert_eq!(buffer.text(), "ghiabjklcdef");
    buffer.edit([(6..7, "")]);
    assert_eq!(buffer.text(), "ghiabjlcdef");
    buffer.edit([(4..9, "mno")]);
    assert_eq!(buffer.text(), "ghiamnoef");
}

#[gpui::test(iterations = 100)]
fn test_random_edits(mut rng: StdRng) {
    let operations = env::var("OPERATIONS")
        .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
        .unwrap_or(10);

    let reference_string_len = rng.random_range(0..3);
    let mut reference_string = RandomCharIter::new(&mut rng)
        .take(reference_string_len)
        .collect::<String>();
    let mut buffer = Buffer::new(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        reference_string.clone(),
    );
    LineEnding::normalize(&mut reference_string);

    buffer.set_group_interval(Duration::from_millis(rng.random_range(0..=200)));
    let mut buffer_versions = Vec::new();
    log::info!(
        "buffer text {:?}, version: {:?}",
        buffer.text(),
        buffer.version()
    );

    for _i in 0..operations {
        let (edits, _) = buffer.randomly_edit(&mut rng, 5);
        for (old_range, new_text) in edits.iter().rev() {
            reference_string.replace_range(old_range.clone(), new_text);
        }

        assert_eq!(buffer.text(), reference_string);
        log::info!(
            "buffer text {:?}, version: {:?}",
            buffer.text(),
            buffer.version()
        );

        if rng.random_bool(0.25) {
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

        if rng.random_bool(0.3) {
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

        assert_eq!(
            buffer.rope_for_version(old_buffer.version()).to_string(),
            old_buffer.text()
        );

        for _ in 0..5 {
            let end_ix =
                old_buffer.clip_offset(rng.random_range(0..=old_buffer.len()), Bias::Right);
            let start_ix = old_buffer.clip_offset(rng.random_range(0..=end_ix), Bias::Left);
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

        assert_eq!(
            buffer.has_edits_since(&old_buffer.version),
            buffer
                .edits_since::<usize>(&old_buffer.version)
                .next()
                .is_some(),
        );

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
fn test_line_endings() {
    assert_eq!(LineEnding::detect(&"🍐✅\n".repeat(1000)), LineEnding::Unix);
    assert_eq!(LineEnding::detect(&"abcd\n".repeat(1000)), LineEnding::Unix);
    assert_eq!(
        LineEnding::detect(&"🍐✅\r\n".repeat(1000)),
        LineEnding::Windows
    );
    assert_eq!(
        LineEnding::detect(&"abcd\r\n".repeat(1000)),
        LineEnding::Windows
    );

    let mut buffer = Buffer::new(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        "one\r\ntwo\rthree",
    );
    assert_eq!(buffer.text(), "one\ntwo\nthree");
    assert_eq!(buffer.line_ending(), LineEnding::Windows);
    buffer.check_invariants();

    buffer.edit([(buffer.len()..buffer.len(), "\r\nfour")]);
    buffer.edit([(0..0, "zero\r\n")]);
    assert_eq!(buffer.text(), "zero\none\ntwo\nthree\nfour");
    assert_eq!(buffer.line_ending(), LineEnding::Windows);
    buffer.check_invariants();
}

#[test]
fn test_line_len() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "");
    buffer.edit([(0..0, "abcd\nefg\nhij")]);
    buffer.edit([(12..12, "kl\nmno")]);
    buffer.edit([(18..18, "\npqrs\n")]);
    buffer.edit([(18..21, "\nPQ")]);

    assert_eq!(buffer.line_len(0), 4);
    assert_eq!(buffer.line_len(1), 3);
    assert_eq!(buffer.line_len(2), 5);
    assert_eq!(buffer.line_len(3), 3);
    assert_eq!(buffer.line_len(4), 4);
    assert_eq!(buffer.line_len(5), 0);
}

#[test]
fn test_common_prefix_at_position() {
    let text = "a = str; b = δα";
    let buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), text);

    let offset1 = offset_after(text, "str");
    let offset2 = offset_after(text, "δα");

    // the preceding word is a prefix of the suggestion
    assert_eq!(
        buffer.common_prefix_at(offset1, "string"),
        range_of(text, "str"),
    );
    // a suffix of the preceding word is a prefix of the suggestion
    assert_eq!(
        buffer.common_prefix_at(offset1, "tree"),
        range_of(text, "tr"),
    );
    // the preceding word is a substring of the suggestion, but not a prefix
    assert_eq!(
        buffer.common_prefix_at(offset1, "astro"),
        empty_range_after(text, "str"),
    );

    // prefix matching is case insensitive.
    assert_eq!(
        buffer.common_prefix_at(offset1, "Strαngε"),
        range_of(text, "str"),
    );
    assert_eq!(
        buffer.common_prefix_at(offset2, "ΔΑΜΝ"),
        range_of(text, "δα"),
    );

    fn offset_after(text: &str, part: &str) -> usize {
        text.find(part).unwrap() + part.len()
    }

    fn empty_range_after(text: &str, part: &str) -> Range<usize> {
        let offset = offset_after(text, part);
        offset..offset
    }

    fn range_of(text: &str, part: &str) -> Range<usize> {
        let start = text.find(part).unwrap();
        start..start + part.len()
    }
}

#[test]
fn test_text_summary_for_range() {
    let buffer = Buffer::new(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        "ab\nefg\nhklm\nnopqrs\ntuvwxyz",
    );
    assert_eq!(
        buffer.text_summary_for_range::<TextSummary, _>(0..2),
        TextSummary {
            len: 2,
            chars: 2,
            len_utf16: OffsetUtf16(2),
            lines: Point::new(0, 2),
            first_line_chars: 2,
            last_line_chars: 2,
            last_line_len_utf16: 2,
            longest_row: 0,
            longest_row_chars: 2,
        }
    );
    assert_eq!(
        buffer.text_summary_for_range::<TextSummary, _>(1..3),
        TextSummary {
            len: 2,
            chars: 2,
            len_utf16: OffsetUtf16(2),
            lines: Point::new(1, 0),
            first_line_chars: 1,
            last_line_chars: 0,
            last_line_len_utf16: 0,
            longest_row: 0,
            longest_row_chars: 1,
        }
    );
    assert_eq!(
        buffer.text_summary_for_range::<TextSummary, _>(1..12),
        TextSummary {
            len: 11,
            chars: 11,
            len_utf16: OffsetUtf16(11),
            lines: Point::new(3, 0),
            first_line_chars: 1,
            last_line_chars: 0,
            last_line_len_utf16: 0,
            longest_row: 2,
            longest_row_chars: 4,
        }
    );
    assert_eq!(
        buffer.text_summary_for_range::<TextSummary, _>(0..20),
        TextSummary {
            len: 20,
            chars: 20,
            len_utf16: OffsetUtf16(20),
            lines: Point::new(4, 1),
            first_line_chars: 2,
            last_line_chars: 1,
            last_line_len_utf16: 1,
            longest_row: 3,
            longest_row_chars: 6,
        }
    );
    assert_eq!(
        buffer.text_summary_for_range::<TextSummary, _>(0..22),
        TextSummary {
            len: 22,
            chars: 22,
            len_utf16: OffsetUtf16(22),
            lines: Point::new(4, 3),
            first_line_chars: 2,
            last_line_chars: 3,
            last_line_len_utf16: 3,
            longest_row: 3,
            longest_row_chars: 6,
        }
    );
    assert_eq!(
        buffer.text_summary_for_range::<TextSummary, _>(7..22),
        TextSummary {
            len: 15,
            chars: 15,
            len_utf16: OffsetUtf16(15),
            lines: Point::new(2, 3),
            first_line_chars: 4,
            last_line_chars: 3,
            last_line_len_utf16: 3,
            longest_row: 1,
            longest_row_chars: 6,
        }
    );
}

#[test]
fn test_chars_at() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "");
    buffer.edit([(0..0, "abcd\nefgh\nij")]);
    buffer.edit([(12..12, "kl\nmno")]);
    buffer.edit([(18..18, "\npqrs")]);
    buffer.edit([(18..21, "\nPQ")]);

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
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "");
    buffer.edit([(0..0, "[workspace]\nmembers = [\n    \"xray_core\",\n    \"xray_server\",\n    \"xray_cli\",\n    \"xray_wasm\",\n]\n")]);
    buffer.edit([(60..60, "\n")]);

    let chars = buffer.chars_at(Point::new(6, 0));
    assert_eq!(chars.collect::<String>(), "    \"xray_wasm\",\n]\n");
}

#[test]
fn test_anchors() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "");
    buffer.edit([(0..0, "abc")]);
    let left_anchor = buffer.anchor_before(2);
    let right_anchor = buffer.anchor_after(2);

    buffer.edit([(1..1, "def\n")]);
    assert_eq!(buffer.text(), "adef\nbc");
    assert_eq!(left_anchor.to_offset(&buffer), 6);
    assert_eq!(right_anchor.to_offset(&buffer), 6);
    assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
    assert_eq!(right_anchor.to_point(&buffer), Point { row: 1, column: 1 });

    buffer.edit([(2..3, "")]);
    assert_eq!(buffer.text(), "adf\nbc");
    assert_eq!(left_anchor.to_offset(&buffer), 5);
    assert_eq!(right_anchor.to_offset(&buffer), 5);
    assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
    assert_eq!(right_anchor.to_point(&buffer), Point { row: 1, column: 1 });

    buffer.edit([(5..5, "ghi\n")]);
    assert_eq!(buffer.text(), "adf\nbghi\nc");
    assert_eq!(left_anchor.to_offset(&buffer), 5);
    assert_eq!(right_anchor.to_offset(&buffer), 9);
    assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
    assert_eq!(right_anchor.to_point(&buffer), Point { row: 2, column: 0 });

    buffer.edit([(7..9, "")]);
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
        anchor_at_offset_0.cmp(&anchor_at_offset_0, &buffer),
        Ordering::Equal
    );
    assert_eq!(
        anchor_at_offset_1.cmp(&anchor_at_offset_1, &buffer),
        Ordering::Equal
    );
    assert_eq!(
        anchor_at_offset_2.cmp(&anchor_at_offset_2, &buffer),
        Ordering::Equal
    );

    assert_eq!(
        anchor_at_offset_0.cmp(&anchor_at_offset_1, &buffer),
        Ordering::Less
    );
    assert_eq!(
        anchor_at_offset_1.cmp(&anchor_at_offset_2, &buffer),
        Ordering::Less
    );
    assert_eq!(
        anchor_at_offset_0.cmp(&anchor_at_offset_2, &buffer),
        Ordering::Less
    );

    assert_eq!(
        anchor_at_offset_1.cmp(&anchor_at_offset_0, &buffer),
        Ordering::Greater
    );
    assert_eq!(
        anchor_at_offset_2.cmp(&anchor_at_offset_1, &buffer),
        Ordering::Greater
    );
    assert_eq!(
        anchor_at_offset_2.cmp(&anchor_at_offset_0, &buffer),
        Ordering::Greater
    );
}

#[test]
fn test_anchors_at_start_and_end() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "");
    let before_start_anchor = buffer.anchor_before(0);
    let after_end_anchor = buffer.anchor_after(0);

    buffer.edit([(0..0, "abc")]);
    assert_eq!(buffer.text(), "abc");
    assert_eq!(before_start_anchor.to_offset(&buffer), 0);
    assert_eq!(after_end_anchor.to_offset(&buffer), 3);

    let after_start_anchor = buffer.anchor_after(0);
    let before_end_anchor = buffer.anchor_before(3);

    buffer.edit([(3..3, "def")]);
    buffer.edit([(0..0, "ghi")]);
    assert_eq!(buffer.text(), "ghiabcdef");
    assert_eq!(before_start_anchor.to_offset(&buffer), 0);
    assert_eq!(after_start_anchor.to_offset(&buffer), 3);
    assert_eq!(before_end_anchor.to_offset(&buffer), 6);
    assert_eq!(after_end_anchor.to_offset(&buffer), 9);
}

#[test]
fn test_undo_redo() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "1234");
    // Set group interval to zero so as to not group edits in the undo stack.
    buffer.set_group_interval(Duration::from_secs(0));

    buffer.edit([(1..1, "abx")]);
    buffer.edit([(3..4, "yzef")]);
    buffer.edit([(3..5, "cd")]);
    assert_eq!(buffer.text(), "1abcdef234");

    let entries = buffer.history.undo_stack.clone();
    assert_eq!(entries.len(), 3);

    buffer.undo_or_redo(entries[0].transaction.clone());
    assert_eq!(buffer.text(), "1cdef234");
    buffer.undo_or_redo(entries[0].transaction.clone());
    assert_eq!(buffer.text(), "1abcdef234");

    buffer.undo_or_redo(entries[1].transaction.clone());
    assert_eq!(buffer.text(), "1abcdx234");
    buffer.undo_or_redo(entries[2].transaction.clone());
    assert_eq!(buffer.text(), "1abx234");
    buffer.undo_or_redo(entries[1].transaction.clone());
    assert_eq!(buffer.text(), "1abyzef234");
    buffer.undo_or_redo(entries[2].transaction.clone());
    assert_eq!(buffer.text(), "1abcdef234");

    buffer.undo_or_redo(entries[2].transaction.clone());
    assert_eq!(buffer.text(), "1abyzef234");
    buffer.undo_or_redo(entries[0].transaction.clone());
    assert_eq!(buffer.text(), "1yzef234");
    buffer.undo_or_redo(entries[1].transaction.clone());
    assert_eq!(buffer.text(), "1234");
}

#[test]
fn test_history() {
    let mut now = Instant::now();
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "123456");
    buffer.set_group_interval(Duration::from_millis(300));

    let transaction_1 = buffer.start_transaction_at(now).unwrap();
    buffer.edit([(2..4, "cd")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "12cd56");

    buffer.start_transaction_at(now);
    buffer.edit([(4..5, "e")]);
    buffer.end_transaction_at(now).unwrap();
    assert_eq!(buffer.text(), "12cde6");

    now += buffer.transaction_group_interval() + Duration::from_millis(1);
    buffer.start_transaction_at(now);
    buffer.edit([(0..1, "a")]);
    buffer.edit([(1..1, "b")]);
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

    // Redo stack gets cleared after performing an edit.
    buffer.start_transaction_at(now);
    buffer.edit([(0..0, "X")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "X12cde6");
    buffer.redo();
    assert_eq!(buffer.text(), "X12cde6");
    buffer.undo();
    assert_eq!(buffer.text(), "12cde6");
    buffer.undo();
    assert_eq!(buffer.text(), "123456");

    // Transactions can be grouped manually.
    buffer.redo();
    buffer.redo();
    assert_eq!(buffer.text(), "X12cde6");
    buffer.group_until_transaction(transaction_1);
    buffer.undo();
    assert_eq!(buffer.text(), "123456");
    buffer.redo();
    assert_eq!(buffer.text(), "X12cde6");
}

#[test]
fn test_finalize_last_transaction() {
    let now = Instant::now();
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "123456");
    buffer.history.group_interval = Duration::from_millis(1);

    buffer.start_transaction_at(now);
    buffer.edit([(2..4, "cd")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "12cd56");

    buffer.finalize_last_transaction();
    buffer.start_transaction_at(now);
    buffer.edit([(4..5, "e")]);
    buffer.end_transaction_at(now).unwrap();
    assert_eq!(buffer.text(), "12cde6");

    buffer.start_transaction_at(now);
    buffer.edit([(0..1, "a")]);
    buffer.edit([(1..1, "b")]);
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
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "1234567");

    buffer.start_transaction_at(now);
    buffer.edit([(2..4, "cd")]);
    buffer.edit([(6..6, "efg")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "12cd56efg7");

    let tx = buffer.finalize_last_transaction().unwrap().clone();
    assert_eq!(
        buffer
            .edited_ranges_for_transaction::<usize>(&tx)
            .collect::<Vec<_>>(),
        [2..4, 6..9]
    );

    buffer.edit([(5..5, "hijk")]);
    assert_eq!(buffer.text(), "12cd5hijk6efg7");
    assert_eq!(
        buffer
            .edited_ranges_for_transaction::<usize>(&tx)
            .collect::<Vec<_>>(),
        [2..4, 10..13]
    );

    buffer.edit([(4..4, "l")]);
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

    let mut buffer1 = Buffer::new(ReplicaId::new(1), BufferId::new(1).unwrap(), text);
    let mut buffer2 = Buffer::new(ReplicaId::new(2), BufferId::new(1).unwrap(), text);
    let mut buffer3 = Buffer::new(ReplicaId::new(3), BufferId::new(1).unwrap(), text);

    let buf1_op = buffer1.edit([(1..2, "12")]);
    assert_eq!(buffer1.text(), "a12cdef");
    let buf2_op = buffer2.edit([(3..4, "34")]);
    assert_eq!(buffer2.text(), "abc34ef");
    let buf3_op = buffer3.edit([(5..6, "56")]);
    assert_eq!(buffer3.text(), "abcde56");

    buffer1.apply_op(buf2_op.clone());
    buffer1.apply_op(buf3_op.clone());
    buffer2.apply_op(buf1_op.clone());
    buffer2.apply_op(buf3_op);
    buffer3.apply_op(buf1_op);
    buffer3.apply_op(buf2_op);

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

    let base_text_len = rng.random_range(0..10);
    let base_text = RandomCharIter::new(&mut rng)
        .take(base_text_len)
        .collect::<String>();
    let mut replica_ids = Vec::new();
    let mut buffers = Vec::new();
    let mut network = Network::new(rng.clone());

    for i in 0..peers {
        let mut buffer = Buffer::new(
            ReplicaId::new(i as u16),
            BufferId::new(1).unwrap(),
            base_text.clone(),
        );
        buffer.history.group_interval = Duration::from_millis(rng.random_range(0..=200));
        buffers.push(buffer);
        replica_ids.push(ReplicaId::new(i as u16));
        network.add_peer(ReplicaId::new(i as u16));
    }

    log::info!("initial text: {:?}", base_text);

    let mut mutation_count = operations;
    loop {
        let replica_index = rng.random_range(0..peers);
        let replica_id = replica_ids[replica_index];
        let buffer = &mut buffers[replica_index];
        match rng.random_range(0..=100) {
            0..=50 if mutation_count != 0 => {
                let op = buffer.randomly_edit(&mut rng, 5).1;
                network.broadcast(buffer.replica_id, vec![op]);
                log::info!("buffer {:?} text: {:?}", buffer.replica_id, buffer.text());
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
                        "peer {:?} applying {} ops from the network.",
                        replica_id,
                        ops.len()
                    );
                    buffer.apply_ops(ops);
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
            "Replica {:?} text != Replica 0 text",
            buffer.replica_id
        );
        buffer.check_invariants();
    }
}

#[test]
fn test_new_normalized_splits_large_base_text() {
    // ASCII text that exceeds max_insertion_len
    let text = "abcdefghij".repeat(10); // 100 bytes
    let rope = Rope::from(text.as_str());
    let buffer = Buffer::new_normalized(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        LineEnding::Unix,
        rope,
    );
    assert_eq!(buffer.text(), text);
    buffer.check_invariants();

    // Verify anchors at various positions, including across chunk boundaries
    for offset in [0, 1, 15, 16, 17, 50, 99] {
        let anchor = buffer.anchor_before(offset);
        assert_eq!(
            anchor.to_offset(&buffer),
            offset,
            "anchor_before({offset}) round-tripped incorrectly"
        );
        let anchor = buffer.anchor_after(offset);
        assert_eq!(
            anchor.to_offset(&buffer),
            offset,
            "anchor_after({offset}) round-tripped incorrectly"
        );
    }

    // Verify editing works after a split initialization
    let mut buffer = buffer;
    buffer.edit([(50..60, "XYZ")]);
    let mut expected = text;
    expected.replace_range(50..60, "XYZ");
    assert_eq!(buffer.text(), expected);
    buffer.check_invariants();
}

#[test]
fn test_new_normalized_splits_large_base_text_with_multibyte_chars() {
    // Use multi-byte chars (é is 2 bytes in UTF-8) so that a naive byte-level
    // split would land in the middle of a character.
    let unit = "ééééééééé"; // 9 chars × 2 bytes = 18 bytes
    let text = unit.repeat(6); // 108 bytes
    let rope = Rope::from(text.as_str());
    let buffer = Buffer::new_normalized(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        LineEnding::Unix,
        rope,
    );
    assert_eq!(buffer.text(), text);
    buffer.check_invariants();

    // Every anchor should resolve correctly even though chunks had to be
    // rounded down to a char boundary.
    let snapshot = buffer.snapshot();
    for offset in (0..text.len()).filter(|o| text.is_char_boundary(*o)) {
        let anchor = snapshot.anchor_before(offset);
        assert_eq!(
            anchor.to_offset(snapshot),
            offset,
            "anchor round-trip failed at byte offset {offset}"
        );
    }
}

#[test]
fn test_new_normalized_small_text_unchanged() {
    // Text that fits in a single chunk should produce exactly one fragment,
    // matching the original single-fragment behaviour.
    let text = "hello world";
    let rope = Rope::from(text);
    let buffer = Buffer::new_normalized(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        LineEnding::Unix,
        rope,
    );
    assert_eq!(buffer.text(), text);
    buffer.check_invariants();
    assert_eq!(buffer.snapshot().fragments.items(&None).len(), 1);
}

#[test]
fn test_edit_splits_large_insertion() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "abcdefghij");

    let large_text: Arc<str> = "X".repeat(100).into();
    let edits = vec![(3..7, large_text.clone())];

    buffer.edit(edits);

    let expected = format!("abc{}hij", large_text);
    assert_eq!(buffer.text(), expected);
    buffer.check_invariants();

    // Anchors should resolve correctly throughout the buffer.
    for offset in [0, 3, 50, 103, expected.len()] {
        let anchor = buffer.anchor_before(offset);
        assert_eq!(
            anchor.to_offset(&buffer),
            offset,
            "anchor_before({offset}) round-tripped incorrectly"
        );
    }
}

#[test]
fn test_edit_splits_large_insertion_with_multibyte_chars() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "abcdefghij");

    // 4-byte chars so that naive byte splits would land mid-character.
    let large_text: Arc<str> = "😀".repeat(30).into(); // 30 × 4 = 120 bytes
    let edits = vec![(5..5, large_text.clone())];

    buffer.edit(edits);

    let expected = format!("abcde{}fghij", large_text);
    assert_eq!(buffer.text(), expected);
    buffer.check_invariants();
}

#[test]
fn test_edit_splits_large_insertion_among_multiple_edits() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "ABCDEFGHIJ");

    let large_text: Arc<str> = "x".repeat(60).into();
    // Three edits: small, large, small. The large one must be split while
    // preserving the correct positions of the surrounding edits.
    let edits = vec![
        (1..2, Arc::from("y")),     // replace "B" with "y"
        (4..6, large_text.clone()), // replace "EF" with 60 x's
        (9..9, Arc::from("z")),     // insert "z" before "J"
    ];

    buffer.edit(edits);

    // Original: A B C D E F G H I J
    // After (1..2, "y"):       A y C D E F G H I J
    // After (4..6, large):     A y C D <60 x's> G H I J
    // After (9..9, "z"):       A y C D <60 x's> G H I z J
    let expected = format!("AyCD{}GHIzJ", large_text);
    assert_eq!(buffer.text(), expected);
    buffer.check_invariants();
}

#[test]
fn test_edit_splits_multiple_large_insertions() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "ABCDE");

    let text1: Arc<str> = "a".repeat(40).into();
    let text2: Arc<str> = "b".repeat(40).into();
    let edits = vec![
        (1..2, text1.clone()), // replace "B" with 40 a's
        (3..4, text2.clone()), // replace "D" with 40 b's
    ];

    buffer.edit(edits);

    let expected = format!("A{}C{}E", text1, text2);
    assert_eq!(buffer.text(), expected);
    buffer.check_invariants();
}

#[test]
fn test_edit_undo_after_split() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "hello world");
    buffer.set_group_interval(Duration::from_secs(0));
    let original = buffer.text();

    let large_text: Arc<str> = "Z".repeat(50).into();
    let edits = vec![(5..6, large_text)];
    buffer.edit(edits);
    assert_ne!(buffer.text(), original);
    buffer.check_invariants();

    // Undo should restore the original text even though the edit was split
    // into multiple internal operations grouped in one transaction.
    buffer.undo();
    assert_eq!(buffer.text(), original);
    buffer.check_invariants();
}

#[test]
fn test_branch_preservation() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "hello");
    buffer.set_group_interval(Duration::from_secs(0));

    // Build linear history: hello -> hello world -> hello brave world
    let now = Instant::now();
    buffer.start_transaction_at(now);
    buffer.edit([(5..5, " world")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "hello world");

    buffer.start_transaction_at(now);
    buffer.edit([(6..6, "brave ")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "hello brave world");

    // Undo back to "hello world"
    buffer.undo();
    assert_eq!(buffer.text(), "hello world");

    // Make a new edit — this would have cleared the redo stack before.
    // Now the "brave " transaction should be preserved.
    buffer.start_transaction_at(now);
    buffer.edit([(5..5, "!")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "hello! world");

    // The redo stack should be empty (new edit clears it),
    // but the branch should be preserved in the undo tree.
    assert!(buffer.peek_redo_stack().is_none());

    // The "brave " transaction's edit_ids are still in the CRDT —
    // verify the branch is preserved in the tree (3 nodes: world, brave, !).
    let tree = buffer.undo_tree_snapshot();
    assert_eq!(
        tree.live_count(),
        3,
        "branch should be preserved in the tree"
    );
}

#[test]
fn test_undo_tree_snapshot_basic() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "hello");
    buffer.set_group_interval(Duration::from_secs(0));

    // Before any edits, snapshot should be empty.
    let tree = buffer.undo_tree_snapshot();
    assert!(tree.is_empty(), "Tree should be empty before any edits");

    // Make an edit.
    let now = Instant::now();
    buffer.start_transaction_at(now);
    buffer.edit([(5..5, " world")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "hello world");

    let tree = buffer.undo_tree_snapshot();
    assert_eq!(tree.len(), 1, "Tree should have 1 node after one edit");
    assert!(tree.current().is_some(), "Current should point to the edit");

    // Make a second edit.
    buffer.start_transaction_at(now);
    buffer.edit([(11..11, "!")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "hello world!");

    let tree = buffer.undo_tree_snapshot();
    assert_eq!(tree.len(), 2, "Tree should have 2 nodes after two edits");
    assert_eq!(tree.current(), Some(1), "Current should be at second node");

    // Undo, then branch.
    buffer.undo();
    assert_eq!(buffer.text(), "hello world");

    buffer.start_transaction_at(now);
    buffer.edit([(5..5, ",")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "hello, world");

    let tree = buffer.undo_tree_snapshot();
    assert_eq!(tree.len(), 3, "Tree should have 3 nodes after branching");
    assert_eq!(tree.current(), Some(2), "Current should be at the branch tip");
}

#[test]
fn test_all_transactions_chronological_order() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "abc");
    buffer.set_group_interval(Duration::from_secs(0));

    let now = Instant::now();

    // Create three transactions: A, B, C
    buffer.start_transaction_at(now);
    buffer.edit([(3..3, "d")]);
    let (id_a, _) = buffer.end_transaction_at(now).unwrap();
    assert_eq!(buffer.text(), "abcd");

    buffer.start_transaction_at(now);
    buffer.edit([(4..4, "e")]);
    let (id_b, _) = buffer.end_transaction_at(now).unwrap();
    assert_eq!(buffer.text(), "abcde");

    buffer.start_transaction_at(now);
    buffer.edit([(5..5, "f")]);
    let (id_c, _) = buffer.end_transaction_at(now).unwrap();
    assert_eq!(buffer.text(), "abcdef");

    // All three should be in chronological order
    let all = buffer.all_transaction_ids();
    assert_eq!(all.len(), 3);
    assert_eq!(all[0], id_a);
    assert_eq!(all[1], id_b);
    assert_eq!(all[2], id_c);

    // Undo B and C, make a new edit D — should create a branch
    buffer.undo(); // undo C
    buffer.undo(); // undo B
    assert_eq!(buffer.text(), "abcd");

    buffer.start_transaction_at(now);
    buffer.edit([(4..4, "X")]);
    let (id_d, _) = buffer.end_transaction_at(now).unwrap();
    assert_eq!(buffer.text(), "abcdX");

    // Now we should have 4 transactions: A, B, C, D
    let all = buffer.all_transaction_ids();
    assert_eq!(all.len(), 4);
    assert_eq!(all[0], id_a);
    assert_eq!(all[1], id_b);
    assert_eq!(all[2], id_c);
    assert_eq!(all[3], id_d);
}

#[test]
fn test_undo_earlier_later_linear() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "abc");
    buffer.set_group_interval(Duration::from_secs(0));

    let now = Instant::now();

    // Create A, B, C
    buffer.start_transaction_at(now);
    buffer.edit([(3..3, "d")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "abcd");

    buffer.start_transaction_at(now);
    buffer.edit([(4..4, "e")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "abcde");

    buffer.start_transaction_at(now);
    buffer.edit([(5..5, "f")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "abcdef");

    // undo_earlier walks backward: abcdef -> abcde -> abcd -> abc
    let result = buffer.undo_earlier();
    assert!(result.is_some());
    assert_eq!(buffer.text(), "abcde");

    let result = buffer.undo_earlier();
    assert!(result.is_some());
    assert_eq!(buffer.text(), "abcd");

    let result = buffer.undo_earlier();
    assert!(result.is_some());
    assert_eq!(buffer.text(), "abc");

    // At the beginning, no earlier state
    let result = buffer.undo_earlier();
    assert!(result.is_none());
    assert_eq!(buffer.text(), "abc");

    // undo_later walks forward: abc -> abcd -> abcde -> abcdef
    let result = buffer.undo_later();
    assert!(result.is_some());
    assert_eq!(buffer.text(), "abcd");

    let result = buffer.undo_later();
    assert!(result.is_some());
    assert_eq!(buffer.text(), "abcde");

    let result = buffer.undo_later();
    assert!(result.is_some());
    assert_eq!(buffer.text(), "abcdef");

    // At the end, no later state
    let result = buffer.undo_later();
    assert!(result.is_none());
    assert_eq!(buffer.text(), "abcdef");
}

#[test]
fn test_undo_earlier_later_across_branches() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "base");
    buffer.set_group_interval(Duration::from_secs(0));

    let now = Instant::now();

    // Create: base -> base A
    buffer.start_transaction_at(now);
    buffer.edit([(4..4, " A")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "base A");

    // Create: base A -> base A B
    buffer.start_transaction_at(now);
    buffer.edit([(6..6, " B")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "base A B");

    // Undo both, back to "base"
    buffer.undo();
    buffer.undo();
    assert_eq!(buffer.text(), "base");

    // Branch: base -> baseX (this is transaction D, chronologically 3rd)
    buffer.start_transaction_at(now);
    buffer.edit([(4..4, "X")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "baseX");

    // Now the chronological order is: A, B, D (X)
    // Current state: baseX (D is active, on a different branch from A,B)

    // undo_earlier should go to B's state (chronologically before D)
    let result = buffer.undo_earlier();
    assert!(result.is_some());
    assert_eq!(buffer.text(), "base A B");

    // undo_earlier should go to A's state
    let result = buffer.undo_earlier();
    assert!(result.is_some());
    assert_eq!(buffer.text(), "base A");

    // undo_earlier should go to initial state (undo A)
    let result = buffer.undo_earlier();
    assert!(result.is_some());
    assert_eq!(buffer.text(), "base");

    // undo_later should go back to A
    let result = buffer.undo_later();
    assert!(result.is_some());
    assert_eq!(buffer.text(), "base A");

    // undo_later should go to B
    let result = buffer.undo_later();
    assert!(result.is_some());
    assert_eq!(buffer.text(), "base A B");

    // undo_later should go to D (baseX)
    let result = buffer.undo_later();
    assert!(result.is_some());
    assert_eq!(buffer.text(), "baseX");
}

#[test]
fn test_goto_transaction_across_branches() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "start");
    buffer.set_group_interval(Duration::from_secs(0));

    let now = Instant::now();

    // Build: start -> start1
    buffer.start_transaction_at(now);
    buffer.edit([(5..5, "1")]);
    let (id_a, _) = buffer.end_transaction_at(now).unwrap();
    assert_eq!(buffer.text(), "start1");

    // Build: start1 -> start12
    buffer.start_transaction_at(now);
    buffer.edit([(6..6, "2")]);
    let (id_b, _) = buffer.end_transaction_at(now).unwrap();
    assert_eq!(buffer.text(), "start12");

    // Undo both
    buffer.undo();
    buffer.undo();
    assert_eq!(buffer.text(), "start");

    // Branch: start -> startX
    buffer.start_transaction_at(now);
    buffer.edit([(5..5, "X")]);
    let (id_c, _) = buffer.end_transaction_at(now).unwrap();
    assert_eq!(buffer.text(), "startX");

    // Jump directly to id_b (start12), crossing branches
    buffer.goto_transaction(id_b);
    assert_eq!(buffer.text(), "start12");

    // Jump directly to id_c (startX), crossing back
    buffer.goto_transaction(id_c);
    assert_eq!(buffer.text(), "startX");

    // Jump to id_a (start1)
    buffer.goto_transaction(id_a);
    assert_eq!(buffer.text(), "start1");
}

#[test]
fn test_tree_parent_tracking() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "root");
    buffer.set_group_interval(Duration::from_secs(0));

    let now = Instant::now();

    // A: root -> rootA
    buffer.start_transaction_at(now);
    buffer.edit([(4..4, "A")]);
    let (id_a, _) = buffer.end_transaction_at(now).unwrap();

    // B: rootA -> rootAB
    buffer.start_transaction_at(now);
    buffer.edit([(5..5, "B")]);
    let (id_b, _) = buffer.end_transaction_at(now).unwrap();

    // A's parent should be None (first transaction, root child)
    let tree = &buffer.history.tree;
    let idx_a = tree.index_for_transaction(id_a).unwrap();
    assert_eq!(tree.node(idx_a).unwrap().parent, None);

    // B's parent should be A
    let idx_b = tree.index_for_transaction(id_b).unwrap();
    assert_eq!(tree.node(idx_b).unwrap().parent, Some(idx_a));

    // Undo B, create C from A
    buffer.undo();
    buffer.start_transaction_at(now);
    buffer.edit([(5..5, "C")]);
    let (id_c, _) = buffer.end_transaction_at(now).unwrap();

    // C's parent should also be A (branching)
    let tree = &buffer.history.tree;
    let idx_c = tree.index_for_transaction(id_c).unwrap();
    assert_eq!(tree.node(idx_c).unwrap().parent, Some(idx_a));

    // path to B should be [A, B]
    let path_b = tree.transaction_ids_on_path(idx_b);
    assert_eq!(path_b, vec![id_a, id_b]);

    // path to C should be [A, C]
    let path_c = tree.transaction_ids_on_path(idx_c);
    assert_eq!(path_c, vec![id_a, id_c]);
}

#[test]
fn test_regular_undo_redo_still_works_after_branching() {
    let mut buffer = Buffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), "hello");
    buffer.set_group_interval(Duration::from_secs(0));

    let now = Instant::now();

    // A: hello -> hello world
    buffer.start_transaction_at(now);
    buffer.edit([(5..5, " world")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "hello world");

    // B: hello world -> hello brave world
    buffer.start_transaction_at(now);
    buffer.edit([(6..6, "brave ")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "hello brave world");

    // Undo B
    buffer.undo();
    assert_eq!(buffer.text(), "hello world");

    // New edit C: hello world -> hello! world (creates branch)
    buffer.start_transaction_at(now);
    buffer.edit([(5..5, "!")]);
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "hello! world");

    // Regular undo should undo C
    buffer.undo();
    assert_eq!(buffer.text(), "hello world");

    // Regular redo should redo C (not B — B is on the old branch)
    buffer.redo();
    assert_eq!(buffer.text(), "hello! world");

    // Undo C and A
    buffer.undo();
    assert_eq!(buffer.text(), "hello world");
    buffer.undo();
    assert_eq!(buffer.text(), "hello");

    // Redo A
    buffer.redo();
    assert_eq!(buffer.text(), "hello world");
}

