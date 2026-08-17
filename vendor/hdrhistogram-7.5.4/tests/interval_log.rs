#[cfg(all(feature = "serialization", test))]
mod tests {
    use base64::engine::general_purpose::STANDARD as B64STANDARD;
    use base64::Engine as _;
    use hdrhistogram::serialization::interval_log::{
        IntervalLogHistogram, IntervalLogIterator, IntervalLogWriterBuilder, LogEntry,
        LogIteratorError, Tag,
    };
    use hdrhistogram::serialization::{Deserializer, Serializer, V2Serializer};
    use hdrhistogram::Histogram;
    use rand::Rng;
    use std::fs::File;
    use std::io::{BufRead, Read};
    use std::path::Path;
    use std::{io, iter, str, time};

    #[test]
    fn parse_sample_tagged_interval_log_start_timestamp() {
        let data = load_iterator_from_file(Path::new("tests/data/tagged-Log.logV2.hlog"));
        let start_count = data
            .into_iter()
            .map(|r| r.unwrap())
            .filter_map(|e| match e {
                LogEntry::StartTime(t) => Some(t),
                _ => None,
            })
            .count();
        assert_eq!(1, start_count);
    }

    #[test]
    fn parse_sample_tagged_interval_log_interval_count() {
        let data = load_iterator_from_file(Path::new("tests/data/tagged-Log.logV2.hlog"));
        let intervals = data
            .into_iter()
            .map(|r| r.unwrap())
            .filter_map(|e| match e {
                LogEntry::Interval(ilh) => Some(ilh),
                _ => None,
            })
            .collect::<Vec<IntervalLogHistogram>>();

        assert_eq!(42, intervals.len());

        // half have tags, half do not
        assert_eq!(
            21,
            intervals.iter().filter(|ilh| ilh.tag().is_none()).count()
        );
        assert_eq!(
            21,
            intervals.iter().filter(|ilh| !ilh.tag().is_none()).count()
        );
    }

    #[test]
    fn parse_sample_tagged_interval_log_interval_metadata() {
        let data = load_iterator_from_file(Path::new("tests/data/tagged-Log.logV2.hlog"));
        let intervals = data
            .into_iter()
            .map(|r| r.unwrap())
            .filter_map(|e| match e {
                LogEntry::Interval(ilh) => Some(ilh),
                _ => None,
            })
            .collect::<Vec<IntervalLogHistogram>>();

        let expected = vec![
            (0.127, 1.007, 2.769),
            (1.134, 0.999, 0.442),
            (2.133, 1.001, 0.426),
            (3.134, 1.001, 0.426),
            (4.135, 0.997, 0.426),
            (5.132, 1.002, 0.426),
            (6.134, 0.999, 0.442),
            (7.133, 0.999, 0.459),
            (8.132, 1.0, 0.459),
            (9.132, 1.751, 1551.892),
            (10.883, 0.25, 0.426),
            (11.133, 1.003, 0.524),
            (12.136, 0.997, 0.459),
            (13.133, 0.998, 0.459),
            (14.131, 1.0, 0.492),
            (15.131, 1.001, 0.442),
            (16.132, 1.001, 0.524),
            (17.133, 0.998, 0.459),
            (18.131, 1.0, 0.459),
            (19.131, 1.0, 0.475),
            (20.131, 1.004, 0.475),
        ];

        // tagged and un-tagged are identical

        assert_eq!(
            expected,
            intervals
                .iter()
                .filter(|ilh| ilh.tag().is_none())
                .map(|ilh| (
                    round(duration_as_fp_seconds(ilh.start_timestamp())),
                    round(duration_as_fp_seconds(ilh.duration())),
                    ilh.max(),
                ))
                .collect::<Vec<(f64, f64, f64)>>()
        );

        assert_eq!(
            expected,
            intervals
                .iter()
                .filter(|ilh| !ilh.tag().is_none())
                .map(|ilh| (
                    round(duration_as_fp_seconds(ilh.start_timestamp())),
                    round(duration_as_fp_seconds(ilh.duration())),
                    ilh.max(),
                ))
                .collect::<Vec<(f64, f64, f64)>>()
        );

        let mut deserializer = Deserializer::new();
        for ilh in intervals {
            let serialized_histogram = B64STANDARD.decode(ilh.encoded_histogram()).unwrap();
            let decoded_hist: Histogram<u64> = deserializer
                .deserialize(&mut io::Cursor::new(&serialized_histogram))
                .unwrap();

            // this log happened to use 1000000 as the scaling factor. It was also formatted to 3
            // decimal places.
            assert_eq!(round(decoded_hist.max() as f64 / 1_000_000_f64), ilh.max());
        }
    }

    #[test]
    fn parse_sample_tagged_interval_log_rewrite_identical() {
        // trim off the comments and legend line
        let reader =
            io::BufReader::new(File::open(Path::new("tests/data/tagged-Log.logV2.hlog")).unwrap());

        // the sample log uses DEFLATE compressed histograms, which we can't match exactly, so the
        // best we can do is to re-serialize each one as uncompressed.

        let mut serializer = V2Serializer::new();
        let mut deserializer = Deserializer::new();

        let mut serialize_buf = Vec::new();
        let mut log_without_headers = Vec::new();
        reader
            .lines()
            .skip(4)
            .map(|r| r.unwrap())
            .for_each(|mut line| {
                let hist_index = line.rfind("HISTF").unwrap();
                let serialized = B64STANDARD.decode(&line[hist_index..]).unwrap();

                let decoded_hist: Histogram<u64> = deserializer
                    .deserialize(&mut io::Cursor::new(serialized))
                    .unwrap();

                serialize_buf.clear();
                serializer
                    .serialize(&decoded_hist, &mut serialize_buf)
                    .unwrap();

                // replace the deflate histogram with the predictable non-deflate one
                line.truncate(hist_index);
                B64STANDARD.encode_string(&serialize_buf, &mut line);

                log_without_headers.extend_from_slice(line.as_bytes());
                log_without_headers.extend_from_slice("\n".as_bytes());
            });

        let mut duplicate_log = Vec::new();

        {
            let mut writer = IntervalLogWriterBuilder::new()
                .with_max_value_divisor(1_000_000.0)
                .begin_log_with(&mut duplicate_log, &mut serializer)
                .unwrap();

            IntervalLogIterator::new(&log_without_headers)
                .map(|r| r.unwrap())
                .filter_map(|e| match e {
                    LogEntry::Interval(ilh) => Some(ilh),
                    _ => None,
                })
                .for_each(|ilh| {
                    let serialized_histogram = B64STANDARD.decode(ilh.encoded_histogram()).unwrap();
                    let decoded_hist: Histogram<u64> = deserializer
                        .deserialize(&mut io::Cursor::new(&serialized_histogram))
                        .unwrap();

                    writer
                        .write_histogram(
                            &decoded_hist,
                            ilh.start_timestamp(),
                            ilh.duration(),
                            ilh.tag(),
                        )
                        .unwrap();
                });
        }

        let orig_str = str::from_utf8(&log_without_headers).unwrap();
        let rewritten_str = str::from_utf8(&duplicate_log)
            .unwrap()
            .lines()
            // remove our #[MaxValueDivisor] comment
            .filter(|l| !l.starts_with("#[MaxValueDivisor: "))
            // put newlines back in
            .flat_map(|l| iter::once(l).chain(iter::once("\n")))
            .collect::<String>();

        assert_eq!(orig_str, rewritten_str);
    }

    #[test]
    fn write_random_histograms_to_interval_log_then_read() {
        let mut rng = rand::thread_rng();

        let mut histograms = Vec::new();
        let mut tags = Vec::new();

        let mut log_buf = Vec::new();
        let mut serializer = V2Serializer::new();

        let max_scaling_factor = 1_000_000.0;

        {
            let mut writer = IntervalLogWriterBuilder::new()
                .with_max_value_divisor(max_scaling_factor)
                .begin_log_with(&mut log_buf, &mut serializer)
                .unwrap();

            writer.write_comment("start").unwrap();

            for i in 0_u32..100 {
                let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

                for _ in 0..100 {
                    // ensure no count above i64::max_value(), even when many large values are
                    // bucketed together
                    h.record_n(rng.gen::<u64>() >> 32, rng.gen::<u64>() >> 32)
                        .unwrap();
                }

                if rng.gen() {
                    tags.push(Some(format!("t{}", i)));
                } else {
                    tags.push(None);
                };
                let current_tag_str = tags.last().unwrap();
                let tag = current_tag_str
                    .as_ref()
                    .map(|s| Tag::new(s.as_str()).unwrap());

                writer
                    .write_histogram(
                        &h,
                        time::Duration::from_secs(i as u64),
                        time::Duration::new(10_000 + i as u64, 0),
                        tag,
                    )
                    .unwrap();

                writer.write_comment(&format!("line {}", i)).unwrap();

                histograms.push(h);
            }
        }

        let parsed = IntervalLogIterator::new(&log_buf)
            .filter_map(|e| match e {
                Ok(LogEntry::Interval(ilh)) => Some(ilh),
                _ => None,
            })
            .collect::<Vec<IntervalLogHistogram>>();

        assert_eq!(histograms.len(), parsed.len());

        let mut deserializer = Deserializer::new();
        for (index, ilh) in parsed.iter().enumerate() {
            let serialized_histogram = B64STANDARD.decode(ilh.encoded_histogram()).unwrap();
            let decoded_hist: Histogram<u64> = deserializer
                .deserialize(&mut io::Cursor::new(&serialized_histogram))
                .unwrap();
            let original_hist = &histograms[index];

            assert_eq!(original_hist, &decoded_hist);

            assert_eq!(index as u64, ilh.start_timestamp().as_secs());
            assert_eq!(
                time::Duration::new(10_000 + index as u64, 0),
                ilh.duration()
            );
            assert_eq!(
                round(original_hist.max() as f64 / max_scaling_factor),
                ilh.max()
            );
            let tag_string: Option<String> = tags.get(index).unwrap().as_ref().map(|s| s.clone());
            assert_eq!(tag_string, ilh.tag().map(|t| t.as_str().to_owned()));
        }
    }

    #[test]
    fn parse_interval_log_syntax_error_then_returns_none() {
        let log = "#Foo\nBar\n".as_bytes();

        let mut iter = IntervalLogIterator::new(&log);

        assert_eq!(
            Some(Err(LogIteratorError::ParseError { offset: 5 })),
            iter.next()
        );
        assert_eq!(None, iter.next());
    }

    /// Round to 3 digits the way floats are in the log
    fn round(f: f64) -> f64 {
        format!("{:.3}", f).parse::<f64>().unwrap()
    }

    fn duration_as_fp_seconds(d: time::Duration) -> f64 {
        d.as_secs() as f64 + d.subsec_nanos() as f64 / 1_000_000_000_f64
    }

    fn load_iterator_from_file<'a>(path: &Path) -> IntervalLogBufHolder {
        let mut buf = Vec::new();
        let _ = File::open(path).unwrap().read_to_end(&mut buf).unwrap();

        IntervalLogBufHolder { data: buf }
    }

    struct IntervalLogBufHolder {
        data: Vec<u8>,
    }

    impl<'a> IntoIterator for &'a IntervalLogBufHolder {
        type Item = Result<LogEntry<'a>, LogIteratorError>;
        type IntoIter = IntervalLogIterator<'a>;

        fn into_iter(self) -> Self::IntoIter {
            IntervalLogIterator::new(self.data.as_slice())
        }
    }
}
