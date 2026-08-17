use rand::Rng;

use std::ops::Add;
use std::{iter, time};

use super::super::super::*;
use super::super::*;
use super::*;

#[test]
fn write_header_comment() {
    let mut buf = Vec::new();
    let mut serializer = V2Serializer::new();

    let _ = IntervalLogWriterBuilder::new()
        .add_comment("foo")
        .begin_log_with(&mut buf, &mut serializer)
        .unwrap();

    assert_eq!(&b"#foo\n"[..], &buf[..]);
}

#[test]
fn write_header_then_interval_comment() {
    let mut buf = Vec::new();
    let mut serializer = V2Serializer::new();

    {
        let mut log_writer = IntervalLogWriterBuilder::new()
            .add_comment("foo")
            .add_comment("bar")
            .begin_log_with(&mut buf, &mut serializer)
            .unwrap();
        log_writer.write_comment("baz").unwrap();
    }

    assert_eq!("#foo\n#bar\n#baz\n", str::from_utf8(&buf[..]).unwrap());
}

#[test]
fn write_comment_control_characters_still_parseable() {
    let mut buf = Vec::new();
    let mut serializer = V2Serializer::new();

    let mut control_chars = String::new();

    // control chars are U+0000-001F, 7F, 80-9F
    for c_byte in (0..0x20_u8).chain(iter::once(0x7F)).chain(0x80..0xA0) {
        let c = c_byte as char;
        assert!(c.is_control());
        control_chars.push(c);
    }

    assert_eq!(2 * 16 + 1 + 2 * 16, control_chars.chars().count());

    {
        let mut log_writer = IntervalLogWriterBuilder::new()
            .add_comment("unicode")
            .add_comment(&control_chars)
            .add_comment("whew")
            .with_start_time(system_time_after_epoch(123, 456_000_000))
            .begin_log_with(&mut buf, &mut serializer)
            .unwrap();
        log_writer.write_comment("baz").unwrap();
    }

    let before_newline = &control_chars[0..10];
    let after_newline = &control_chars[11..];
    let expected = format!(
        "#unicode\n#{}\n#{}\n#whew\n#[StartTime: 123.456 (seconds since epoch)]\n#baz\n",
        before_newline, after_newline
    );
    assert_eq!(&expected, str::from_utf8(&buf[..]).unwrap());

    let mut i = IntervalLogIterator::new(&buf);
    assert_eq!(
        Some(Ok(LogEntry::StartTime(time::Duration::new(
            123,
            456_000_000
        )))),
        i.next()
    );
    assert_eq!(None, i.next());
}

#[test]
fn write_comment_newline_wraps() {
    let mut buf = Vec::new();
    let mut serializer = V2Serializer::new();

    {
        let _ = IntervalLogWriterBuilder::new()
            .add_comment("before")
            .add_comment("new\nline")
            .add_comment("after")
            .begin_log_with(&mut buf, &mut serializer)
            .unwrap();
    }

    assert_eq!(
        "#before\n#new\n#line\n#after\n",
        str::from_utf8(&buf[..]).unwrap()
    );
}

#[test]
fn write_headers_multiple_times_only_last_is_used() {
    let mut buf = Vec::new();
    let mut serializer = V2Serializer::new();

    {
        let _ = IntervalLogWriterBuilder::new()
            .with_start_time(system_time_after_epoch(10, 0))
            .with_base_time(system_time_after_epoch(20, 0))
            .with_start_time(system_time_after_epoch(100, 0))
            .with_base_time(system_time_after_epoch(200, 0))
            .with_max_value_divisor(1_000.0)
            .with_max_value_divisor(1_000_000.0)
            .begin_log_with(&mut buf, &mut serializer)
            .unwrap();
    }

    let expected = "\
                    #[StartTime: 100.000 (seconds since epoch)]\n\
                    #[BaseTime: 200.000 (seconds since epoch)]\n\
                    #[MaxValueDivisor: 1000000.000]\n";

    assert_eq!(expected, str::from_utf8(&buf[..]).unwrap());
}

#[test]
fn write_interval_histo_no_tag() {
    let mut buf = Vec::new();
    let mut serializer = V2Serializer::new();

    let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();
    h.record(1000).unwrap();

    {
        let mut log_writer = IntervalLogWriterBuilder::new()
            .with_max_value_divisor(10.0)
            .begin_log_with(&mut buf, &mut serializer)
            .unwrap();

        log_writer
            .write_histogram(
                &h,
                time::Duration::new(1, 234_567_890),
                time::Duration::new(5, 670_000_000),
                None,
            )
            .unwrap();
    }

    let expected = "\
         #[MaxValueDivisor: 10.000]\n\
         1.235,5.670,100.000,HISTEwAAAAMAAAAAAAAAAwAAAAAAAAAB//////////8/8AAAAAAAAM8PAg==\n";

    assert_eq!(expected, str::from_utf8(&buf[..]).unwrap());
}

#[test]
fn write_interval_histo_with_tag() {
    let mut buf = Vec::new();
    let mut serializer = V2Serializer::new();

    let h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

    {
        let mut log_writer = IntervalLogWriterBuilder::new()
            .begin_log_with(&mut buf, &mut serializer)
            .unwrap();

        log_writer
            .write_histogram(
                &h,
                time::Duration::new(1, 234_000_000),
                time::Duration::new(5, 678_000_000),
                Tag::new("t"),
            )
            .unwrap();
    }

    assert_eq!(
        "Tag=t,1.234,5.678,0.000,HISTEwAAAAEAAAAAAAAAAwAAAAAAAAAB//////////8/8AAAAAAAAAA=\n",
        str::from_utf8(&buf[..]).unwrap()
    );
}

#[test]
fn write_start_time() {
    let mut buf = Vec::new();
    let mut serializer = V2Serializer::new();

    let _ = IntervalLogWriterBuilder::new()
        .with_start_time(system_time_after_epoch(123, 456_789_012))
        .begin_log_with(&mut buf, &mut serializer)
        .unwrap();

    assert_eq!(
        "#[StartTime: 123.457 (seconds since epoch)]\n",
        str::from_utf8(&buf[..]).unwrap()
    );
}

#[test]
fn write_base_time() {
    let mut buf = Vec::new();
    let mut serializer = V2Serializer::new();

    {
        let _ = IntervalLogWriterBuilder::new()
            .with_base_time(system_time_after_epoch(123, 456_789_012))
            .begin_log_with(&mut buf, &mut serializer)
            .unwrap();
    }

    assert_eq!(
        "#[BaseTime: 123.457 (seconds since epoch)]\n",
        str::from_utf8(&buf[..]).unwrap()
    );
}

#[test]
fn parse_duration_full_ns() {
    let (rest, dur) = fract_sec_duration(b"123456.789012345foo").unwrap();

    assert_eq!(time::Duration::new(123456, 789_012_345), dur);
    assert_eq!(b"foo", rest);
}

#[test]
fn parse_duration_scale_ns() {
    let (rest, dur) = fract_sec_duration(b"123456.789012foo").unwrap();

    assert_eq!(time::Duration::new(123456, 789_012_000), dur);
    assert_eq!(b"foo", rest);
}

#[test]
fn parse_duration_too_many_ns() {
    let (rest, dur) = fract_sec_duration(b"123456.7890123456foo").unwrap();

    // consumes all the numbers, but only parses the first 9
    assert_eq!(time::Duration::new(123456, 789_012_345), dur);
    assert_eq!(b"foo", rest);
}

#[test]
fn duration_fp_roundtrip_accuracy() {
    let mut rng = rand::thread_rng();

    let mut buf = String::new();
    let mut errors = Vec::new();
    for _ in 0..100_000 {
        buf.clear();

        // pick seconds
        let secs = rng.gen_range(0..2_000_000_000);
        // pick nsecs that only has ms accuracy
        let nsecs = rng.gen_range(0..1000) * 1000_000;

        let dur = time::Duration::new(secs, nsecs);
        let fp_secs = duration_as_fp_seconds(dur);

        write!(&mut buf, "{:.3}", fp_secs).unwrap();

        let (_, dur2) = fract_sec_duration(buf.as_bytes()).unwrap();

        if dur != dur2 {
            errors.push((dur, dur2));
        }
    }

    if !errors.is_empty() {
        for &(dur, dur2) in &errors {
            println!("{:?} -> {:?}", dur, dur2);
        }
    }

    assert_eq!(0, errors.len());
}

#[test]
fn parse_start_time_with_human_date() {
    let (rest, e) = start_time(
        b"#[StartTime: 1441812279.474 (seconds since epoch), Wed Sep 09 08:24:39 PDT 2015]\nfoo",
    )
    .unwrap();

    let expected = LogEntry::StartTime(time::Duration::new(1441812279, 474_000_000));

    assert_eq!(expected, e);
    assert_eq!(b"foo", rest);
}

#[test]
fn parse_start_time_without_human_date() {
    // Can't be bothered to format a timestamp for humans, so we don't write that data. It's just
    // another part that could be wrong -- what if it disagrees with the seconds since epoch?
    // Also, BaseTime doesn't have a human-formatted time.
    let (rest, e) = start_time(b"#[StartTime: 1441812279.474 (seconds since epoch)]\nfoo").unwrap();

    let expected = LogEntry::StartTime(time::Duration::new(1441812279, 474_000_000));

    assert_eq!(expected, e);
    assert_eq!(b"foo", rest);
}

#[test]
fn parse_base_time() {
    let (rest, e) = base_time(b"#[BaseTime: 1441812279.474 (seconds since epoch)]\nfoo").unwrap();

    let expected = LogEntry::BaseTime(time::Duration::new(1441812279, 474_000_000));

    assert_eq!(expected, e);
    assert_eq!(b"foo", rest);
}

#[test]
fn parse_legend() {
    let input = b"\"StartTimestamp\",\"Interval_Length\",\"Interval_Max\",\
    \"Interval_Compressed_Histogram\"\nfoo";
    let (rest, _) = legend(input).unwrap();

    assert_eq!(b"foo", rest);
}

#[test]
fn parse_comment() {
    let (rest, _) = comment_line(b"#SomeOtherComment\nfoo").unwrap();

    assert_eq!(b"foo", rest);
}

#[test]
fn parse_interval_hist_no_tag() {
    let (rest, e) = interval_hist(b"0.127,1.007,2.769,couldBeBase64\nfoo").unwrap();

    let expected = LogEntry::Interval(IntervalLogHistogram {
        tag: None,
        start_timestamp: time::Duration::new(0, 127_000_000),
        duration: time::Duration::new(1, 7_000_000),
        max: 2.769,
        encoded_histogram: "couldBeBase64",
    });

    assert_eq!(expected, e);
    assert_eq!(b"foo", rest);
}

#[test]
fn parse_interval_hist_with_tag() {
    let (rest, e) = interval_hist(b"Tag=t,0.127,1.007,2.769,couldBeBase64\nfoo").unwrap();

    let expected = LogEntry::Interval(IntervalLogHistogram {
        tag: Some(Tag("t")),
        start_timestamp: time::Duration::new(0, 127_000_000),
        duration: time::Duration::new(1, 7_000_000),
        max: 2.769,
        encoded_histogram: "couldBeBase64",
    });

    assert_eq!(expected, e);
    assert_eq!(b"foo", rest);
}

#[test]
fn iter_with_ignored_prefix() {
    let mut data = Vec::new();
    data.extend_from_slice(b"#I'm a comment\n");
    data.extend_from_slice(b"\"StartTimestamp\",etc\n");
    data.extend_from_slice(b"Tag=t,0.127,1.007,2.769,couldBeBase64\n");
    data.extend_from_slice(b"#[StartTime: 1441812279.474 ...\n");

    let entries: Vec<LogEntry> = IntervalLogIterator::new(&data)
        .map(|r| r.unwrap())
        .collect();

    let expected0 = LogEntry::Interval(IntervalLogHistogram {
        tag: Some(Tag("t")),
        start_timestamp: time::Duration::new(0, 127_000_000),
        duration: time::Duration::new(1, 7_000_000),
        max: 2.769,
        encoded_histogram: "couldBeBase64",
    });

    let expected1 = LogEntry::StartTime(time::Duration::new(1441812279, 474_000_000));

    assert_eq!(vec![expected0, expected1], entries)
}

#[test]
fn iter_without_ignored_prefix() {
    let mut data = Vec::new();
    data.extend_from_slice(b"Tag=t,0.127,1.007,2.769,couldBeBase64\n");
    data.extend_from_slice(b"#[StartTime: 1441812279.474 ...\n");

    let entries: Vec<LogEntry> = IntervalLogIterator::new(&data)
        .map(|r| r.unwrap())
        .collect();

    let expected0 = LogEntry::Interval(IntervalLogHistogram {
        tag: Some(Tag("t")),
        start_timestamp: time::Duration::new(0, 127_000_000),
        duration: time::Duration::new(1, 7_000_000),
        max: 2.769,
        encoded_histogram: "couldBeBase64",
    });

    let expected1 = LogEntry::StartTime(time::Duration::new(1441812279, 474_000_000));

    assert_eq!(vec![expected0, expected1], entries)
}

#[test]
fn iter_multiple_entrties_with_interleaved_ignored() {
    let mut data = Vec::new();
    data.extend_from_slice(b"#I'm a comment\n");
    data.extend_from_slice(b"\"StartTimestamp\",etc\n");
    data.extend_from_slice(b"Tag=t,0.127,1.007,2.769,couldBeBase64\n");
    data.extend_from_slice(b"#Another comment\n");
    data.extend_from_slice(b"#[StartTime: 1441812279.474 ...\n");
    data.extend_from_slice(b"#Yet another comment\n");
    data.extend_from_slice(b"#[BaseTime: 1441812279.474 ...\n");
    data.extend_from_slice(b"#Enough with the comments\n");

    let entries: Vec<LogEntry> = IntervalLogIterator::new(&data)
        .map(|r| r.unwrap())
        .collect();

    let expected0 = LogEntry::Interval(IntervalLogHistogram {
        tag: Some(Tag("t")),
        start_timestamp: time::Duration::new(0, 127_000_000),
        duration: time::Duration::new(1, 7_000_000),
        max: 2.769,
        encoded_histogram: "couldBeBase64",
    });

    let expected1 = LogEntry::StartTime(time::Duration::new(1441812279, 474_000_000));
    let expected2 = LogEntry::BaseTime(time::Duration::new(1441812279, 474_000_000));

    assert_eq!(vec![expected0, expected1, expected2], entries)
}

#[test]
fn iter_all_ignored_empty_iter() {
    let mut data = Vec::new();
    data.extend_from_slice(b"#I'm a comment\n");
    data.extend_from_slice(b"\"StartTimestamp\",etc\n");
    data.extend_from_slice(b"#Another comment\n");

    assert_eq!(0, IntervalLogIterator::new(&data).count());
}

fn system_time_after_epoch(secs: u64, nanos: u32) -> time::SystemTime {
    time::UNIX_EPOCH.add(time::Duration::new(secs, nanos))
}
