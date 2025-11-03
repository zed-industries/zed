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

#[gpui::test]
fn test_edit(cx: &mut gpui::TestAppContext) {
    let mut buffer = Buffer::new(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        "abc",
        cx.background_executor(),
    );
    assert_eq!(buffer.text(), "abc");
    buffer.edit([(3..3, "def")], cx.background_executor());
    assert_eq!(buffer.text(), "abcdef");
    buffer.edit([(0..0, "ghi")], cx.background_executor());
    assert_eq!(buffer.text(), "ghiabcdef");
    buffer.edit([(5..5, "jkl")], cx.background_executor());
    assert_eq!(buffer.text(), "ghiabjklcdef");
    buffer.edit([(6..7, "")], cx.background_executor());
    assert_eq!(buffer.text(), "ghiabjlcdef");
    buffer.edit([(4..9, "mno")], cx.background_executor());
    assert_eq!(buffer.text(), "ghiamnoef");
}

#[gpui::test(iterations = 100)]
fn test_random_edits(cx: &mut gpui::TestAppContext, mut rng: StdRng) {
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
        cx.background_executor(),
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
        let (edits, _) = buffer.randomly_edit(&mut rng, 5, cx.background_executor());
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
            text.replace(
                edit.new.start..edit.new.start + edit.old.len(),
                &new_text,
                cx.background_executor(),
            );
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
            text.replace(
                edit.new.start..edit.new.start + edit.old.len(),
                &new_text,
                cx.background_executor(),
            );
        }
        assert_eq!(text.to_string(), buffer.text());
    }
}

#[gpui::test]
fn test_line_endings(cx: &mut gpui::TestAppContext) {
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
        cx.background_executor(),
    );
    assert_eq!(buffer.text(), "one\ntwo\nthree");
    assert_eq!(buffer.line_ending(), LineEnding::Windows);
    buffer.check_invariants();

    buffer.edit(
        [(buffer.len()..buffer.len(), "\r\nfour")],
        cx.background_executor(),
    );
    buffer.edit([(0..0, "zero\r\n")], cx.background_executor());
    assert_eq!(buffer.text(), "zero\none\ntwo\nthree\nfour");
    assert_eq!(buffer.line_ending(), LineEnding::Windows);
    buffer.check_invariants();
}

#[gpui::test]
fn test_line_len(cx: &mut gpui::TestAppContext) {
    let mut buffer = Buffer::new(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        "",
        cx.background_executor(),
    );
    buffer.edit([(0..0, "abcd\nefg\nhij")], cx.background_executor());
    buffer.edit([(12..12, "kl\nmno")], cx.background_executor());
    buffer.edit([(18..18, "\npqrs\n")], cx.background_executor());
    buffer.edit([(18..21, "\nPQ")], cx.background_executor());

    assert_eq!(buffer.line_len(0), 4);
    assert_eq!(buffer.line_len(1), 3);
    assert_eq!(buffer.line_len(2), 5);
    assert_eq!(buffer.line_len(3), 3);
    assert_eq!(buffer.line_len(4), 4);
    assert_eq!(buffer.line_len(5), 0);
}

#[gpui::test]
fn test_common_prefix_at_position(cx: &mut gpui::TestAppContext) {
    let text = "a = str; b = δα";
    let buffer = Buffer::new(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        text,
        cx.background_executor(),
    );

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

#[gpui::test]
fn test_text_summary_for_range(cx: &mut gpui::TestAppContext) {
    let buffer = Buffer::new(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        "ab\nefg\nhklm\nnopqrs\ntuvwxyz",
        cx.background_executor(),
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

#[gpui::test]
fn test_chars_at(cx: &mut gpui::TestAppContext) {
    let mut buffer = Buffer::new(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        "",
        cx.background_executor(),
    );
    buffer.edit([(0..0, "abcd\nefgh\nij")], cx.background_executor());
    buffer.edit([(12..12, "kl\nmno")], cx.background_executor());
    buffer.edit([(18..18, "\npqrs")], cx.background_executor());
    buffer.edit([(18..21, "\nPQ")], cx.background_executor());

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
    let mut buffer = Buffer::new(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        "",
        cx.background_executor(),
    );
    buffer.edit([(0..0, "[workspace]\nmembers = [\n    \"xray_core\",\n    \"xray_server\",\n    \"xray_cli\",\n    \"xray_wasm\",\n]\n")], cx.background_executor());
    buffer.edit([(60..60, "\n")], cx.background_executor());

    let chars = buffer.chars_at(Point::new(6, 0));
    assert_eq!(chars.collect::<String>(), "    \"xray_wasm\",\n]\n");
}

#[gpui::test]
fn test_anchors(cx: &mut gpui::TestAppContext) {
    let mut buffer = Buffer::new(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        "",
        cx.background_executor(),
    );
    buffer.edit([(0..0, "abc")], cx.background_executor());
    let left_anchor = buffer.anchor_before(2);
    let right_anchor = buffer.anchor_after(2);

    buffer.edit([(1..1, "def\n")], cx.background_executor());
    assert_eq!(buffer.text(), "adef\nbc");
    assert_eq!(left_anchor.to_offset(&buffer), 6);
    assert_eq!(right_anchor.to_offset(&buffer), 6);
    assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
    assert_eq!(right_anchor.to_point(&buffer), Point { row: 1, column: 1 });

    buffer.edit([(2..3, "")], cx.background_executor());
    assert_eq!(buffer.text(), "adf\nbc");
    assert_eq!(left_anchor.to_offset(&buffer), 5);
    assert_eq!(right_anchor.to_offset(&buffer), 5);
    assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
    assert_eq!(right_anchor.to_point(&buffer), Point { row: 1, column: 1 });

    buffer.edit([(5..5, "ghi\n")], cx.background_executor());
    assert_eq!(buffer.text(), "adf\nbghi\nc");
    assert_eq!(left_anchor.to_offset(&buffer), 5);
    assert_eq!(right_anchor.to_offset(&buffer), 9);
    assert_eq!(left_anchor.to_point(&buffer), Point { row: 1, column: 1 });
    assert_eq!(right_anchor.to_point(&buffer), Point { row: 2, column: 0 });

    buffer.edit([(7..9, "")], cx.background_executor());
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

#[gpui::test]
fn test_anchors_at_start_and_end(cx: &mut gpui::TestAppContext) {
    let mut buffer = Buffer::new(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        "",
        cx.background_executor(),
    );
    let before_start_anchor = buffer.anchor_before(0);
    let after_end_anchor = buffer.anchor_after(0);

    buffer.edit([(0..0, "abc")], cx.background_executor());
    assert_eq!(buffer.text(), "abc");
    assert_eq!(before_start_anchor.to_offset(&buffer), 0);
    assert_eq!(after_end_anchor.to_offset(&buffer), 3);

    let after_start_anchor = buffer.anchor_after(0);
    let before_end_anchor = buffer.anchor_before(3);

    buffer.edit([(3..3, "def")], cx.background_executor());
    buffer.edit([(0..0, "ghi")], cx.background_executor());
    assert_eq!(buffer.text(), "ghiabcdef");
    assert_eq!(before_start_anchor.to_offset(&buffer), 0);
    assert_eq!(after_start_anchor.to_offset(&buffer), 3);
    assert_eq!(before_end_anchor.to_offset(&buffer), 6);
    assert_eq!(after_end_anchor.to_offset(&buffer), 9);
}

#[gpui::test]
fn test_undo_redo(cx: &mut gpui::TestAppContext) {
    let mut buffer = Buffer::new(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        "1234",
        cx.background_executor(),
    );
    // Set group interval to zero so as to not group edits in the undo stack.
    buffer.set_group_interval(Duration::from_secs(0));

    buffer.edit([(1..1, "abx")], cx.background_executor());
    buffer.edit([(3..4, "yzef")], cx.background_executor());
    buffer.edit([(3..5, "cd")], cx.background_executor());
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

#[gpui::test]
fn test_history(cx: &mut gpui::TestAppContext) {
    let mut now = Instant::now();
    let mut buffer = Buffer::new(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        "123456",
        cx.background_executor(),
    );
    buffer.set_group_interval(Duration::from_millis(300));

    let transaction_1 = buffer.start_transaction_at(now).unwrap();
    buffer.edit([(2..4, "cd")], cx.background_executor());
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "12cd56");

    buffer.start_transaction_at(now);
    buffer.edit([(4..5, "e")], cx.background_executor());
    buffer.end_transaction_at(now).unwrap();
    assert_eq!(buffer.text(), "12cde6");

    now += buffer.transaction_group_interval() + Duration::from_millis(1);
    buffer.start_transaction_at(now);
    buffer.edit([(0..1, "a")], cx.background_executor());
    buffer.edit([(1..1, "b")], cx.background_executor());
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
    buffer.edit([(0..0, "X")], cx.background_executor());
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

#[gpui::test]
fn test_finalize_last_transaction(cx: &mut gpui::TestAppContext) {
    let now = Instant::now();
    let mut buffer = Buffer::new(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        "123456",
        cx.background_executor(),
    );
    buffer.history.group_interval = Duration::from_millis(1);

    buffer.start_transaction_at(now);
    buffer.edit([(2..4, "cd")], cx.background_executor());
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "12cd56");

    buffer.finalize_last_transaction();
    buffer.start_transaction_at(now);
    buffer.edit([(4..5, "e")], cx.background_executor());
    buffer.end_transaction_at(now).unwrap();
    assert_eq!(buffer.text(), "12cde6");

    buffer.start_transaction_at(now);
    buffer.edit([(0..1, "a")], cx.background_executor());
    buffer.edit([(1..1, "b")], cx.background_executor());
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

#[gpui::test]
fn test_edited_ranges_for_transaction(cx: &mut gpui::TestAppContext) {
    let now = Instant::now();
    let mut buffer = Buffer::new(
        ReplicaId::LOCAL,
        BufferId::new(1).unwrap(),
        "1234567",
        cx.background_executor(),
    );

    buffer.start_transaction_at(now);
    buffer.edit([(2..4, "cd")], cx.background_executor());
    buffer.edit([(6..6, "efg")], cx.background_executor());
    buffer.end_transaction_at(now);
    assert_eq!(buffer.text(), "12cd56efg7");

    let tx = buffer.finalize_last_transaction().unwrap().clone();
    assert_eq!(
        buffer
            .edited_ranges_for_transaction::<usize>(&tx)
            .collect::<Vec<_>>(),
        [2..4, 6..9]
    );

    buffer.edit([(5..5, "hijk")], cx.background_executor());
    assert_eq!(buffer.text(), "12cd5hijk6efg7");
    assert_eq!(
        buffer
            .edited_ranges_for_transaction::<usize>(&tx)
            .collect::<Vec<_>>(),
        [2..4, 10..13]
    );

    buffer.edit([(4..4, "l")], cx.background_executor());
    assert_eq!(buffer.text(), "12cdl5hijk6efg7");
    assert_eq!(
        buffer
            .edited_ranges_for_transaction::<usize>(&tx)
            .collect::<Vec<_>>(),
        [2..4, 11..14]
    );
}

#[gpui::test]
fn test_concurrent_edits(cx: &mut gpui::TestAppContext) {
    let text = "abcdef";

    let mut buffer1 = Buffer::new(
        ReplicaId::new(1),
        BufferId::new(1).unwrap(),
        text,
        cx.background_executor(),
    );
    let mut buffer2 = Buffer::new(
        ReplicaId::new(2),
        BufferId::new(1).unwrap(),
        text,
        cx.background_executor(),
    );
    let mut buffer3 = Buffer::new(
        ReplicaId::new(3),
        BufferId::new(1).unwrap(),
        text,
        cx.background_executor(),
    );

    let buf1_op = buffer1.edit([(1..2, "12")], cx.background_executor());
    assert_eq!(buffer1.text(), "a12cdef");
    let buf2_op = buffer2.edit([(3..4, "34")], cx.background_executor());
    assert_eq!(buffer2.text(), "abc34ef");
    let buf3_op = buffer3.edit([(5..6, "56")], cx.background_executor());
    assert_eq!(buffer3.text(), "abcde56");

    buffer1.apply_op(buf2_op.clone(), Some(cx.background_executor()));
    buffer1.apply_op(buf3_op.clone(), Some(cx.background_executor()));
    buffer2.apply_op(buf1_op.clone(), Some(cx.background_executor()));
    buffer2.apply_op(buf3_op, Some(cx.background_executor()));
    buffer3.apply_op(buf1_op, Some(cx.background_executor()));
    buffer3.apply_op(buf2_op, Some(cx.background_executor()));

    assert_eq!(buffer1.text(), "a12c34e56");
    assert_eq!(buffer2.text(), "a12c34e56");
    assert_eq!(buffer3.text(), "a12c34e56");
}

#[gpui::test(iterations = 100)]
fn test_random_concurrent_edits(mut rng: StdRng, cx: &mut gpui::TestAppContext) {
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
            cx.background_executor(),
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
                let op = buffer
                    .randomly_edit(&mut rng, 5, cx.background_executor())
                    .1;
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
                    buffer.apply_ops(ops, Some(cx.background_executor()));
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
