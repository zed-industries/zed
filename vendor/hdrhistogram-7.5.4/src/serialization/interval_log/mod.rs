//! Interval log parsing and writing.
//!
//! Interval logs, as handled by the Java implementation's `HistogramLogWriter`,
//! `HistogramLogReader`, and `HistogramLogProcessor`, are a way to record a sequence of histograms
//! over time. Suppose you were running a load test for an hour: you might want to record a
//! histogram per second or minute so that you could correlate measurements with behavior you might
//! see in logs, etc.
//!
//! An interval log contains some initial metadata, then a sequence of histograms, each with some
//! additional metadata (timestamps, etc). See `IntervalLogHistogram`.
//!
//! The intervals in the log should be ordered by start timestamp. It's possible to write (and
//! parse) logs with intervals in any order, but the expectation is that they will be sorted.
//!
//! To parse a log, see `IntervalLogIterator`. To write a log, see `IntervalLogWriterBuilder`.
//!
//! # Timestamps
//!
//! Each interval has a timestamp in seconds associated with it. However, it's not necessarily as
//! simple as just interpreting the number as seconds since the epoch. There are two optional pieces
//! of header metadata: "StartTime" and "BaseTime". Neither, one, or both of these may be present.
//! It is possible to have multiple StartTime or BaseTime entries in the log, or even interleaved
//! with interval histograms, but that is confusing, so this API prevents you from doing so.
//!
//! When BaseTime is present, per-interval timestamps are the number of seconds since BaseTime. When
//! it is absent, the per-interval timestamps represent fractional seconds since the epoch (aka
//! Unix time). BaseTime is useful because it allows you to have somewhat human-readable timestamps
//! for each interval -- it's easier to see 245.3 and see that that's 4 minutes after the start of
//! whatever it is you're doing than it is to mentally parse a Unix timestamp. Naturally, you can
//! always calculate the deltas after the fact if you're willing to write a little tooling, but in
//! some cases log files are consumed by humans too.
//!
//! While BaseTime is used to redefine per-interval timestamps for human readability, StartTime
//! provides information about when the process that is generating the log reached some "start"
//! condition. It's frequently the case that intervals will start some seconds after the process
//! started, whether due to initial warmup before a benchmark or because it just takes a
//! while to start up. If this is relevant to your workload, use StartTime to record the actual
//! process start time (or other relevant "start" condition, like when a benchmark begins on an
//! already long-running process). You could then use this when processing a log to more accurately
//! plot interval data over time. Put differently, StartTime doesn't change the interpretation of
//! interval timestamps directly like BaseTime; instead, it provides a hint to the consumer of the
//! log that the "start" (whatever that means to you -- process start, etc) was at a different time
//! than that associated with the first interval.
//!
//! #### Example scenario
//!
//! To explain their usage, suppose we're running a multi-hour benchmark on a process that starts
//! up at a Unix time of 1500000000. We'll be recording separate log files per hour just to
//! demonstrate the interaction between BaseTime, StartTime, and interval log timestamps.
//!
//! The process starts up, warms up its caches, JIT compiles, etc and is ready to start its
//! benchmark 40 seconds later, so we create the first interval log file and record a StartTime of
//! 1500000040. If the actual process start (rather than benchmark start) is more useful to you,
//! using a StartTime of 1500000000 would be reasonable, but we'll stick with 1500000040.
//!
//! We'll use a BaseTime of 1500000040 because that's when the benchmark began, but 1500000000 would
//! also be a reasonable choice here -- it would just make the per-interval deltas 40 seconds
//! larger, which might be a more useful way of recording them, depending on the situation.
//!
//! The benchmark produces an interval histogram for each 60 seconds of workload, so the first one
//! is ready at 1500000100, and is recorded with a delta timestamp of 60. This goes on for another
//! hour, with the last one being 3540 seconds after the start of the benchmark with a corresponding
//! delta of 3540.
//!
//! At the start of the second hour, the time is 1500003640 and the first log file is ended (which
//! is straightforward: just stop writing to it and close the file) and the second log file is
//! opened. It still uses a StartTime of 1500000040 because we want to represent that this log
//! pertains to something that started an hour ago, but we'll use a BaseTime of 1500003640 so that
//! our delta timestamps start over at 0. Again, you could just as well decide to use the same
//! BaseTime of 1500000040 if you prefer, but it's arguably easier for a human to parse "h hours
//! into the benchmark, s seconds past the hour" than it is to mentally divide by 3600, so we'll
//! go with the new BaseTime.
//!
//! Suppose now you wanted to process these logs and show information (e.g. the 0.999th quantile of
//! each interval) as the y-axis on a plot where time is the x-axis. You would want to have
//! StartTime be the zero of the x-axis. For each interval, calculate the Unix time by adding its
//! timestamp to BaseTime, where BaseTime is assumed to be zero if it's not specified. The point on
//! the x-axis for that interval would be the result of subtracting StartTime from that Unix time.
//! As an example, the 17th minute in the 2nd hour would have an interval timestamp of approximately
//! 1020, which when added to 1500003640 is 1500004660. The difference between StartTime and then is
//! 4620. You might choose to display that in units of minutes, which would be 4620 / 60 = 77.
//!
//! #### Java interop
//!
//! Since you may be interoperating with the Java reference implementation, here's a summary of the
//! logic used by the `HistogramLogReader` class for StartTime and BaseTime. It's slightly different
//! than what was described above, presumably for legacy compatibility reasons. This class stores
//! the StartTime as a field which is exposed via a getter, and also integrates filtering intervals
//! based on windows for either "absolute" (Unix time) timestamps or "offset" timestamps (delta vs
//! the StartTime), so if you're using that filtering functionality, you need to understand how it
//! ends up setting its internal version of StartTime.
//!
//! - Neither StartTime nor BaseTime are present: interval timestamps are interpreted as seconds
//! since the epoch. The first interval's timestamp is stored to the StartTime field.
//! - StartTime is present: StartTime is a number of seconds since epoch, and interval timestamps
//! may be interpreted as deltas to be added to StartTime or as "absolute" Unix time depending on a
//! heuristic. In other words, the heuristic chooses between setting the effective BaseTime to 0 or
//! to StartTime. Specifically, the heuristic interprets interval timestamps as deltas if they are
//! more than a year's worth of seconds smaller than StartTime and as absolute timestamps otherwise.
//! - BaseTime is present: BaseTime is a number of seconds since epoch, and interval timestamps are
//! interpreted as deltas. The first interval's (delta) timestamp is stored to the StartTime field.
//! This is likely a bug, since StartTime should be an absolute timestamp, and appears to cause
//! erroneous behavior when filtering by offset timestamps.
//! - BaseTime and StartTime are present: The BaseTime is used like it is when it's the only one
//! present: it's a number of seconds since epoch that serves as the starting point for the
//! per-interval deltas to get a wall-clock time for each interval. No heuristics are applied to
//! guess whether or not the intervals are absolute or deltas.
//!
//! The Java implementation also supports re-setting the StartTime and BaseTime if those entries
//! exist more than once in the log. Suppose that you had an hour's worth of per-minute intervals,
//! and then you set another StartTime to the current time and added a second hour's worth of
//! intervals. Filtering for "all intervals between 5 and 10 minutes past the start" would result in
//! two disjoint sequences of intervals, which is at the very least arguably unintuitive. However,
//! you may well be working with log files that are structured that way, so this implementation's
//! `IntervalLogIterator` will expose multiple StartTime, etc, entries as they appear in the log
//! file.
//!
//! # Examples
//!
//! Parse a single interval from a log.
//!
//! ```
//! use std::time;
//! use hdrhistogram::serialization::interval_log;
//!
//! // two newline-separated log lines: a comment, then an interval
//! let log = b"#I'm a comment\nTag=t,0.127,1.007,2.769,base64EncodedHisto\n";
//!
//! let mut iter = interval_log::IntervalLogIterator::new(&log[..]);
//!
//! // the comment is consumed and ignored by the parser, so the first event is an Interval
//! match iter.next().unwrap() {
//!     Ok(interval_log::LogEntry::Interval(h)) => {
//!         assert_eq!(time::Duration::new(0, 127_000_000), h.start_timestamp());
//!     }
//!     _ => panic!()
//! }
//!
//! // there are no more lines in the log; iteration complete
//! assert_eq!(None, iter.next());
//! ```
//!
//! Skip logs that started before 3 seconds.
//!
//! ```
//! use hdrhistogram::serialization::interval_log;
//!
//! let log = "\
//!     #I'm a comment\n\
//!     Tag=a,0.123,1.007,2.769,base64EncodedHisto\n\
//!     1.456,1.007,2.769,base64EncodedHisto\n\
//!     3.789,1.007,2.769,base64EncodedHisto\n\
//!     Tag=b,4.123,1.007,2.769,base64EncodedHisto\n\
//!     5.456,1.007,2.769,base64EncodedHisto\n\
//!     #Another comment\n"
//! .as_bytes();
//!
//! let iter = interval_log::IntervalLogIterator::new(&log);
//!
//! let count = iter
//!     // only look at intervals (which are the only non-comment lines in this log)
//!     .filter_map(|e| match e {
//!         Ok(interval_log::LogEntry::Interval(ilh)) => Some(ilh),
//!          _ => None
//!     })
//!     // do any filtering you want
//!     .filter(|ilh| ilh.start_timestamp().as_secs() >= 3)
//!     .count();
//!
//! assert_eq!(3, count);
//! ```
//!
//! Write a log.
//!
//! ```
//! use std::{str, time};
//! use hdrhistogram;
//! use hdrhistogram::serialization;
//! use hdrhistogram::serialization::interval_log;
//!
//! let mut buf = Vec::new();
//! let mut serializer = serialization::V2Serializer::new();
//!
//! let mut h = hdrhistogram::Histogram::<u64>::new_with_bounds(
//!     1, u64::max_value(), 3).unwrap();
//! h.record(12345).unwrap();
//!
//! // limit scope of mutable borrow of `buf`
//! {
//!     let now = time::SystemTime::now();
//!     let mut log_writer = interval_log::IntervalLogWriterBuilder::new()
//!         .add_comment("Comments are great")
//!         .with_start_time(now)
//!         .begin_log_with(&mut buf, &mut serializer)
//!         .unwrap();
//!
//!     log_writer.write_comment(
//!         "You can have comments anywhere in the log").unwrap();
//!
//!     log_writer
//!         .write_histogram(
//!             &h,
//!             now.elapsed().unwrap(),
//!             time::Duration::new(12, 345_678_901),
//!             interval_log::Tag::new("im-a-tag")
//!         )
//!         .unwrap();
//! }
//!
//! // `buf` is now full of stuff; we check for the first line
//! assert_eq!("#Comments are great\n", &str::from_utf8(&buf).unwrap()[0..20]);
//! ```

use std::cmp::Ordering;
use std::error::Error;
use std::fmt::Write;
use std::str::FromStr;
use std::{fmt, io, ops, str, time};

use base64::Engine as _;
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_until, take_while1};
use nom::character::complete::char;
use nom::character::is_digit;
use nom::combinator::{complete, map_res, opt, recognize};
use nom::error::ErrorKind;
use nom::number::complete::double;
use nom::{Err, IResult};

use super::super::{Counter, Histogram};
use super::Serializer;

/// Prepare an `IntervalLogWriter`.
///
/// This type only allows writing comments and headers. Once you're done writing those things, use
/// `into_log_writer()` to convert this into an `IntervalLogWriter`.
pub struct IntervalLogWriterBuilder {
    comments: Vec<String>,
    start_time: Option<f64>,
    base_time: Option<f64>,
    max_value_divisor: f64,
}

impl Default for IntervalLogWriterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl IntervalLogWriterBuilder {
    /// Create a new log writer that writes to `writer` and serializes histograms with `serializer`.
    pub fn new() -> IntervalLogWriterBuilder {
        IntervalLogWriterBuilder {
            comments: Vec::new(),
            start_time: None,
            base_time: None,
            max_value_divisor: 1.0,
        }
    }

    /// Add a comment line to be written when the writer is built.
    ///
    /// Comments containing '\n' will be transformed into multiple lines of comments.
    pub fn add_comment(&mut self, s: &str) -> &mut Self {
        self.comments.push(s.to_owned());
        self
    }

    /// Set a StartTime. See the module-level documentation for more info.
    ///
    /// This can be called multiple times, but only the value for the most recent invocation will
    /// be written.
    pub fn with_start_time(&mut self, time: time::SystemTime) -> &mut Self {
        self.start_time = Some(system_time_as_fp_seconds(time));
        self
    }

    /// Set a BaseTime. See the module-level documentation for more info.
    ///
    /// This can be called multiple times, but only the value for the most recent invocation will
    /// be written.
    pub fn with_base_time(&mut self, time: time::SystemTime) -> &mut Self {
        self.base_time = Some(system_time_as_fp_seconds(time));
        self
    }

    /// Set a max value divisor.
    ///
    /// This is used to scale down the max value part of an interval log to something that may be
    /// more human readable. The max value in the log is only for human consumption, so you might
    /// prefer to divide by 10<sup>9</sup> to turn nanoseconds into fractional seconds, for
    /// instance.
    ///
    /// If this is not set, 1.0 will be used.
    ///
    /// This can be called multiple times, but only the value for the most recent invocation will
    /// be written.
    pub fn with_max_value_divisor(&mut self, max_value_divisor: f64) -> &mut Self {
        self.max_value_divisor = max_value_divisor;
        self
    }

    /// Build a LogWriter and apply any configured headers.
    #[allow(clippy::float_cmp)]
    pub fn begin_log_with<'a, 'b, W: 'a + io::Write, S: 'b + Serializer>(
        &self,
        writer: &'a mut W,
        serializer: &'b mut S,
    ) -> Result<IntervalLogWriter<'a, 'b, W, S>, io::Error> {
        let mut internal_writer = InternalLogWriter {
            writer,
            serializer,
            text_buf: String::new(),
            serialize_buf: Vec::new(),
            max_value_divisor: self.max_value_divisor,
        };

        for c in &self.comments {
            internal_writer.write_comment(&c)?;
        }

        if let Some(st) = self.start_time {
            internal_writer.write_fmt(format_args!(
                "#[StartTime: {:.3} (seconds since epoch)]\n",
                st
            ))?;
        }

        if let Some(bt) = self.base_time {
            internal_writer.write_fmt(format_args!(
                "#[BaseTime: {:.3} (seconds since epoch)]\n",
                bt
            ))?;
        }

        // The Java impl doesn't write a comment for this but it's confusing to silently modify the
        // max value without leaving a trace
        if self.max_value_divisor != 1.0_f64 {
            internal_writer.write_fmt(format_args!(
                "#[MaxValueDivisor: {:.3}]\n",
                self.max_value_divisor
            ))?;
        }

        Ok(IntervalLogWriter { internal_writer })
    }
}

/// Writes interval histograms in an interval log.
///
/// This isn't created directly; start with an `IntervalLogWriterBuilder`. Once you've written the
/// headers and ended up with an `IntervalLogWriter`, typical usage would be to write a histogram
/// at regular intervals (e.g. once a second).
///
/// ```
/// use hdrhistogram::serialization;
/// use hdrhistogram::serialization::interval_log;
///
/// let mut buf = Vec::new();
/// let mut serializer = serialization::V2Serializer::new();
///
/// // create a writer via a builder
/// let mut writer = interval_log::IntervalLogWriterBuilder::new()
///     .begin_log_with(&mut buf, &mut serializer)
///     .unwrap();
///
/// writer.write_comment("Comment 2").unwrap();
///
/// // .. write some intervals
/// ```
pub struct IntervalLogWriter<'a, 'b, W: 'a + io::Write, S: 'b + Serializer> {
    internal_writer: InternalLogWriter<'a, 'b, W, S>,
}

impl<'a, 'b, W: 'a + io::Write, S: 'b + Serializer> IntervalLogWriter<'a, 'b, W, S> {
    /// Write a comment line.
    ///
    /// Comments containing '\n' will be transformed into multiple lines of comments.
    pub fn write_comment(&mut self, s: &str) -> io::Result<()> {
        self.internal_writer.write_comment(s)
    }

    /// Write an interval histogram.
    ///
    /// `start_timestamp` is the time since the epoch in seconds that measurements started being
    /// recorded in this interval. If you're using a StartTime or BaseTime offset, you should
    /// instead use a delta since that time. See the discussion about timestamps in the module-level
    /// documentation.
    ///
    /// `duration` is the duration of the interval in seconds.
    ///
    /// `tag` is an optional tag for this histogram.
    pub fn write_histogram<T: Counter>(
        &mut self,
        h: &Histogram<T>,
        start_timestamp: time::Duration,
        duration: time::Duration,
        tag: Option<Tag>,
    ) -> Result<(), IntervalLogWriterError<S::SerializeError>> {
        self.internal_writer
            .write_histogram(h, start_timestamp, duration, tag)
    }
}

/// Errors that can occur while writing a log.
#[derive(Debug)]
pub enum IntervalLogWriterError<E> {
    /// Histogram serialization failed.
    SerializeError(E),
    /// An i/o error occurred.
    IoError(io::Error),
}

impl<E> From<io::Error> for IntervalLogWriterError<E> {
    fn from(e: io::Error) -> Self {
        IntervalLogWriterError::IoError(e)
    }
}

impl<E: fmt::Display + fmt::Debug> fmt::Display for IntervalLogWriterError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IntervalLogWriterError::SerializeError(e) => {
                write!(f, "Histogram serialization failed: {}", e)
            }
            IntervalLogWriterError::IoError(e) => write!(f, "An i/o error occurred: {}", e),
        }
    }
}

impl<E: Error + 'static> Error for IntervalLogWriterError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            IntervalLogWriterError::SerializeError(e) => Some(e),
            IntervalLogWriterError::IoError(e) => Some(e),
        }
    }
}

/// Write interval logs.
struct InternalLogWriter<'a, 'b, W: 'a + io::Write, S: 'b + Serializer> {
    writer: &'a mut W,
    serializer: &'b mut S,
    text_buf: String,
    serialize_buf: Vec<u8>,
    max_value_divisor: f64,
}

impl<'a, 'b, W: 'a + io::Write, S: 'b + Serializer> InternalLogWriter<'a, 'b, W, S> {
    fn write_fmt(&mut self, args: fmt::Arguments) -> io::Result<()> {
        self.writer.write_fmt(args)
    }

    fn write_comment(&mut self, s: &str) -> io::Result<()> {
        for l in s.split('\n') {
            writeln!(self.writer, "#{}", l)?;
        }

        Ok(())
    }

    fn write_histogram<T: Counter>(
        &mut self,
        h: &Histogram<T>,
        start_timestamp: time::Duration,
        duration: time::Duration,
        tag: Option<Tag>,
    ) -> Result<(), IntervalLogWriterError<S::SerializeError>> {
        self.serialize_buf.clear();
        self.text_buf.clear();

        if let Some(Tag(s)) = tag {
            write!(self.text_buf, "Tag={},", &s).expect("Writes to a String can't fail");
        }

        write!(
            self.writer,
            "{}{:.3},{:.3},{:.3},",
            self.text_buf,
            duration_as_fp_seconds(start_timestamp),
            duration_as_fp_seconds(duration),
            h.max() as f64 / self.max_value_divisor // because the Java impl does it this way
        )?;

        self.text_buf.clear();
        let _len = self
            .serializer
            .serialize(h, &mut self.serialize_buf)
            .map_err(IntervalLogWriterError::SerializeError)?;
        base64::engine::general_purpose::STANDARD
            .encode_string(&self.serialize_buf, &mut self.text_buf);

        self.writer.write_all(self.text_buf.as_bytes())?;
        self.writer.write_all(b"\n")?;

        Ok(())
    }
}

/// A tag for an interval histogram.
///
/// Tags are just `str`s that do not contain a few disallowed characters: ',', '\r', '\n', and ' '.
///
/// To get the wrapped `str` back out, use `as_str()` or the `Deref<str>` implementation
/// (`&some_tag`).
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tag<'a>(&'a str);

impl<'a> Tag<'a> {
    /// Create a new Tag.
    ///
    /// If a disallowed character is present, this will return `None`.
    pub fn new(s: &'a str) -> Option<Tag<'a>> {
        if s.chars()
            .any(|c| c == ',' || c == '\r' || c == '\n' || c == ' ')
        {
            None
        } else {
            Some(Tag(s))
        }
    }

    /// Returns the tag contents as a str.
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

impl<'a> ops::Deref for Tag<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

/// An individual interval histogram parsed from an interval log.
#[derive(PartialEq, Debug)]
pub struct IntervalLogHistogram<'a> {
    tag: Option<Tag<'a>>,
    start_timestamp: time::Duration,
    duration: time::Duration,
    max: f64,
    encoded_histogram: &'a str,
}

impl<'a> IntervalLogHistogram<'a> {
    /// Tag, if any is present.
    pub fn tag(&self) -> Option<Tag<'a>> {
        self.tag
    }

    /// Timestamp of the start of the interval in seconds, expressed as a `Duration` relative to
    /// some start point.
    ///
    /// The timestamp may be absolute vs the epoch, or there may be a `StartTime` or `BaseTime` for
    /// the log, in which case you may wish to consider this number as a delta vs those timestamps.
    /// See the module-level documentation about timestamps.
    pub fn start_timestamp(&self) -> time::Duration {
        self.start_timestamp
    }

    /// Duration of the interval in seconds.
    pub fn duration(&self) -> time::Duration {
        self.duration
    }

    /// Max value in the encoded histogram
    ///
    /// This max value is the max of the histogram divided by some scaling factor (which may be
    /// 1.0).
    pub fn max(&self) -> f64 {
        self.max
    }

    /// Base64-encoded serialized histogram.
    ///
    /// If you need the deserialized histogram, base64-decode and use a `Deserializer` on the
    /// resulting bytes.
    pub fn encoded_histogram(&self) -> &'a str {
        self.encoded_histogram
    }
}

#[derive(PartialEq, Debug)]
/// Represents one non-comment line in an interval log.
///
/// One thing to note is that the way your interval timestamps work can vary. If your log was
/// written with a StartTime or BaseTime, that metadata will appear in header comments, and that
/// will be represented by the iterator providing the corresponding variants here. The presence
/// of those timestamps will affect how you should interpret the timestamps for individual
/// intervals. See the module-level documentation.
#[allow(variant_size_differences)]
pub enum LogEntry<'a> {
    /// Logs may include a StartTime. If present, it represents seconds since the epoch.
    StartTime(time::Duration),
    /// Logs may include a BaseTime. If present, it represents seconds since the epoch.
    BaseTime(time::Duration),
    /// An individual interval histogram.
    Interval(IntervalLogHistogram<'a>),
}

/// Errors that occur when parsing an interval log.
#[derive(Debug, PartialEq)]
pub enum LogIteratorError {
    /// Parsing failed
    ParseError {
        /// Offset in the input where the failed parse started
        offset: usize,
    },
}

/// Parse interval logs.
///
/// This iterator exposes each item (excluding comments and other information-free lines). See
/// `LogEntry`.
///
/// Because histogram deserialization is deferred, parsing logs is fast. See the `interval_log`
/// benchmark if you wish to see how it does on your hardware. As a baseline, parsing a log of 1000
/// random histograms of 10,000 values each takes 2ms total on an E5-1650v3.
///
/// Deferring deserialization is handy because it allows you to cheaply navigate the log to find
/// the records you care about (e.g. ones in a certain time range, or with a certain tag) without
/// doing all the allocation, etc, of deserialization.
///
/// If you're looking for a direct port of the Java impl's `HistogramLogReader`, this isn't one: it
/// won't deserialize for you, and it pushes the burden of figuring out what to do with StartTime,
/// BaseTime, etc to you, and there aren't built in functions to filter by timestamp. On the other
/// hand, because it doesn't do those things, it is much more flexible: you can easily build any
/// sort of filtering you want, not just timestamp ranges, because you have cheap access to all the
/// metadata before incurring the cost of deserialization. If you're not using any timestamp
/// headers, or at least using them in straightforward ways, it is easy to accumulate the
/// timestamp state you need. Since all the parsing is taken care of already, writing your own
/// `HistogramLogReader` equivalent that fits the way your logs are assembled is just a couple of
/// lines. (And if you're doing complex stuff, we probably wouldn't have built something that fits
/// your quirky logs anyway!)
///
/// This parses from a slice representing the complete file because it made implementation easier
/// (and also supports mmap'd files for maximum parsing speed). If parsing from a `Read` is
/// important for your use case, open an issue about it.
pub struct IntervalLogIterator<'a> {
    orig_len: usize,
    input: &'a [u8],
    ended: bool,
}

impl<'a> IntervalLogIterator<'a> {
    /// Create a new iterator from the UTF-8 bytes of an interval log.
    pub fn new(input: &'a [u8]) -> IntervalLogIterator<'a> {
        IntervalLogIterator {
            orig_len: input.len(),
            input,
            ended: false,
        }
    }
}

impl<'a> Iterator for IntervalLogIterator<'a> {
    type Item = Result<LogEntry<'a>, LogIteratorError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.ended {
                return None;
            }

            if self.input.is_empty() {
                self.ended = true;
                return None;
            }

            // Look for magic comments first otherwise they will get matched by the simple comment
            // parser
            if let Ok((rest, e)) = log_entry(self.input) {
                self.input = rest;
                return Some(Ok(e));
            }

            // it wasn't a log entry; try parsing a comment
            match ignored_line(self.input) {
                Ok((rest, _)) => {
                    self.input = rest;
                    continue;
                }
                _ => {
                    self.ended = true;
                    return Some(Err(LogIteratorError::ParseError {
                        offset: self.orig_len - self.input.len(),
                    }));
                }
            }
        }
    }
}

fn duration_as_fp_seconds(d: time::Duration) -> f64 {
    d.as_secs() as f64 + f64::from(d.subsec_nanos()) / 1_000_000_000_f64
}

fn system_time_as_fp_seconds(time: time::SystemTime) -> f64 {
    match time.duration_since(time::UNIX_EPOCH) {
        Ok(dur_after_epoch) => duration_as_fp_seconds(dur_after_epoch),
        // Doesn't seem possible to be before the epoch, but using a negative number seems like
        // a reasonable representation if it does occur
        Err(t) => duration_as_fp_seconds(t.duration()) * -1_f64,
    }
}

fn start_time(input: &[u8]) -> IResult<&[u8], LogEntry> {
    let (input, _) = tag("#[StartTime: ")(input)?;
    let (input, duration) = fract_sec_duration(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = take(1_usize)(input)?;
    Ok((input, LogEntry::StartTime(duration)))
}

fn base_time(input: &[u8]) -> IResult<&[u8], LogEntry> {
    let (input, _) = tag("#[BaseTime: ")(input)?;
    let (input, duration) = fract_sec_duration(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = take(1_usize)(input)?;
    Ok((input, LogEntry::BaseTime(duration)))
}

fn tag_bytes(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, _) = tag("Tag=")(input)?;
    let (input, tag) = take_until(",")(input)?;
    let (input, _) = take(1_usize)(input)?;
    Ok((input, tag))
}

fn tag_parser(input: &[u8]) -> IResult<&[u8], Tag> {
    let (input, tag) = map_res(tag_bytes, str::from_utf8)(input)?;
    Ok((input, Tag(tag)))
}

fn interval_hist(input: &[u8]) -> IResult<&[u8], LogEntry> {
    let (input, tag) = opt(tag_parser)(input)?;
    let (input, start_timestamp) = fract_sec_duration(input)?;
    let (input, _) = char(',')(input)?;
    let (input, duration) = fract_sec_duration(input)?;
    let (input, _) = char(',')(input)?;
    let (input, max) = double(input)?;
    let (input, _) = char(',')(input)?;
    let (input, encoded_histogram) = map_res(take_until("\n"), str::from_utf8)(input)?;
    // Be nice to Windows users:
    let encoded_histogram = encoded_histogram.trim_end_matches('\r');
    let (input, _) = take(1_usize)(input)?;

    Ok((
        input,
        LogEntry::Interval(IntervalLogHistogram {
            tag,
            start_timestamp,
            duration,
            max,
            encoded_histogram,
        }),
    ))
}

fn log_entry(input: &[u8]) -> IResult<&[u8], LogEntry<'_>> {
    complete(alt((start_time, base_time, interval_hist)))(input)
}

fn comment_line(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = tag("#")(input)?;
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = take(1_usize)(input)?;
    Ok((input, ()))
}

fn legend(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = tag("\"StartTimestamp\"")(input)?;
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = take(1_usize)(input)?;
    Ok((input, ()))
}

fn ignored_line(input: &[u8]) -> IResult<&[u8], ()> {
    alt((comment_line, legend))(input)
}

fn fract_sec_duration(input: &[u8]) -> IResult<&[u8], time::Duration> {
    let (rest, data) = fract_sec_tuple(input)?;
    let (secs, nanos_str) = data;

    // only read up to 9 digits since we can only support nanos, not smaller precision
    let nanos_parse_res = match nanos_str.len().cmp(&9) {
        Ordering::Greater => nanos_str[0..9].parse::<u32>(),
        Ordering::Equal => nanos_str.parse::<u32>(),
        Ordering::Less => nanos_str
            .parse::<u32>()
            // subtraction will not overflow because len is < 9
            .map(|n| n * 10_u32.pow(9 - nanos_str.len() as u32)),
    };

    if let Ok(nanos) = nanos_parse_res {
        return Ok((rest, time::Duration::new(secs, nanos)));
    }

    // nanos were invalid utf8. We don't expose these errors, so don't bother defining a
    // custom error type.
    Err(Err::Error(error_position!(input, ErrorKind::Alpha)))
}

type FResult<'a> = IResult<&'a [u8], (u64, &'a str)>;

fn fract_sec_tuple(input: &[u8]) -> FResult {
    let (input, secs) = map_res(
        map_res(recognize(take_until(".")), str::from_utf8),
        u64::from_str,
    )(input)?;
    let (input, _) = tag(".")(input)?;
    let (input, nanos_str) = map_res(complete(take_while1(is_digit)), str::from_utf8)(input)?;
    Ok((input, (secs, nanos_str)))
}

#[cfg(test)]
mod tests;
