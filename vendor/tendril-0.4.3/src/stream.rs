// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Streams of tendrils.

use fmt;
use tendril::{Atomicity, NonAtomic, Tendril};

use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::marker::PhantomData;
use std::path::Path;

#[cfg(feature = "encoding")]
use encoding;
#[cfg(feature = "encoding_rs")]
use encoding_rs::{self, DecoderResult};
use utf8;

/// Trait for types that can process a tendril.
///
/// This is a "push" interface, unlike the "pull" interface of
/// `Iterator<Item=Tendril<F>>`. The push interface matches
/// [html5ever][] and other incremental parsers with a similar
/// architecture.
///
/// [html5ever]: https://github.com/servo/html5ever
pub trait TendrilSink<F, A = NonAtomic>
where
    F: fmt::Format,
    A: Atomicity,
{
    /// Process this tendril.
    fn process(&mut self, t: Tendril<F, A>);

    /// Indicates that an error has occurred.
    fn error(&mut self, desc: Cow<'static, str>);

    /// What the overall result of processing is.
    type Output;

    /// Indicates the end of the stream.
    fn finish(self) -> Self::Output;

    /// Process one tendril and finish.
    fn one<T>(mut self, t: T) -> Self::Output
    where
        Self: Sized,
        T: Into<Tendril<F, A>>,
    {
        self.process(t.into());
        self.finish()
    }

    /// Consume an iterator of tendrils, processing each item, then finish.
    fn from_iter<I>(mut self, i: I) -> Self::Output
    where
        Self: Sized,
        I: IntoIterator,
        I::Item: Into<Tendril<F, A>>,
    {
        for t in i {
            self.process(t.into())
        }
        self.finish()
    }

    /// Read from the given stream of bytes until exhaustion and process incrementally,
    /// then finish. Return `Err` at the first I/O error.
    fn read_from<R>(mut self, r: &mut R) -> io::Result<Self::Output>
    where
        Self: Sized,
        R: io::Read,
        F: fmt::SliceFormat<Slice = [u8]>,
    {
        const BUFFER_SIZE: u32 = 4 * 1024;
        loop {
            let mut tendril = Tendril::<F, A>::new();
            // FIXME: this exposes uninitialized bytes to a generic R type
            // this is fine for R=File which never reads these bytes,
            // but user-defined types might.
            // The standard library pushes zeros to `Vec<u8>` for that reason.
            unsafe {
                tendril.push_uninitialized(BUFFER_SIZE);
            }
            loop {
                match r.read(&mut tendril) {
                    Ok(0) => return Ok(self.finish()),
                    Ok(n) => {
                        tendril.pop_back(BUFFER_SIZE - n as u32);
                        self.process(tendril);
                        break;
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
                    Err(e) => return Err(e),
                }
            }
        }
    }

    /// Read from the file at the given path and process incrementally,
    /// then finish. Return `Err` at the first I/O error.
    fn from_file<P>(self, path: P) -> io::Result<Self::Output>
    where
        Self: Sized,
        P: AsRef<Path>,
        F: fmt::SliceFormat<Slice = [u8]>,
    {
        self.read_from(&mut File::open(path)?)
    }
}

/// A `TendrilSink` adaptor that takes bytes, decodes them as UTF-8,
/// lossily replace ill-formed byte sequences with U+FFFD replacement characters,
/// and emits Unicode (`StrTendril`).
///
/// This does not allocate memory: the output is either subtendrils on the input,
/// on inline tendrils for a single code point.
pub struct Utf8LossyDecoder<Sink, A = NonAtomic>
where
    Sink: TendrilSink<fmt::UTF8, A>,
    A: Atomicity,
{
    pub inner_sink: Sink,
    incomplete: Option<utf8::Incomplete>,
    marker: PhantomData<A>,
}

impl<Sink, A> Utf8LossyDecoder<Sink, A>
where
    Sink: TendrilSink<fmt::UTF8, A>,
    A: Atomicity,
{
    /// Create a new incremental UTF-8 decoder.
    #[inline]
    pub fn new(inner_sink: Sink) -> Self {
        Utf8LossyDecoder {
            inner_sink: inner_sink,
            incomplete: None,
            marker: PhantomData,
        }
    }
}

impl<Sink, A> TendrilSink<fmt::Bytes, A> for Utf8LossyDecoder<Sink, A>
where
    Sink: TendrilSink<fmt::UTF8, A>,
    A: Atomicity,
{
    #[inline]
    fn process(&mut self, mut t: Tendril<fmt::Bytes, A>) {
        // FIXME: remove take() and map() when non-lexical borrows are stable.
        if let Some(mut incomplete) = self.incomplete.take() {
            let resume_at = incomplete.try_complete(&t).map(|(result, rest)| {
                match result {
                    Ok(s) => self.inner_sink.process(Tendril::from_slice(s)),
                    Err(_) => {
                        self.inner_sink.error("invalid byte sequence".into());
                        self.inner_sink
                            .process(Tendril::from_slice(utf8::REPLACEMENT_CHARACTER));
                    }
                }
                t.len() - rest.len()
            });
            match resume_at {
                None => {
                    self.incomplete = Some(incomplete);
                    return;
                }
                Some(resume_at) => t.pop_front(resume_at as u32),
            }
        }
        while !t.is_empty() {
            let unborrowed_result = match utf8::decode(&t) {
                Ok(s) => {
                    debug_assert!(s.as_ptr() == t.as_ptr());
                    debug_assert!(s.len() == t.len());
                    Ok(())
                }
                Err(utf8::DecodeError::Invalid {
                    valid_prefix,
                    invalid_sequence,
                    ..
                }) => {
                    debug_assert!(valid_prefix.as_ptr() == t.as_ptr());
                    debug_assert!(valid_prefix.len() <= t.len());
                    Err((
                        valid_prefix.len(),
                        Err(valid_prefix.len() + invalid_sequence.len()),
                    ))
                }
                Err(utf8::DecodeError::Incomplete {
                    valid_prefix,
                    incomplete_suffix,
                }) => {
                    debug_assert!(valid_prefix.as_ptr() == t.as_ptr());
                    debug_assert!(valid_prefix.len() <= t.len());
                    Err((valid_prefix.len(), Ok(incomplete_suffix)))
                }
            };
            match unborrowed_result {
                Ok(()) => {
                    unsafe { self.inner_sink.process(t.reinterpret_without_validating()) }
                    return;
                }
                Err((valid_len, and_then)) => {
                    if valid_len > 0 {
                        let subtendril = t.subtendril(0, valid_len as u32);
                        unsafe {
                            self.inner_sink
                                .process(subtendril.reinterpret_without_validating())
                        }
                    }
                    match and_then {
                        Ok(incomplete) => {
                            self.incomplete = Some(incomplete);
                            return;
                        }
                        Err(offset) => {
                            self.inner_sink.error("invalid byte sequence".into());
                            self.inner_sink
                                .process(Tendril::from_slice(utf8::REPLACEMENT_CHARACTER));
                            t.pop_front(offset as u32);
                        }
                    }
                }
            }
        }
    }

    #[inline]
    fn error(&mut self, desc: Cow<'static, str>) {
        self.inner_sink.error(desc);
    }

    type Output = Sink::Output;

    #[inline]
    fn finish(mut self) -> Sink::Output {
        if self.incomplete.is_some() {
            self.inner_sink
                .error("incomplete byte sequence at end of stream".into());
            self.inner_sink
                .process(Tendril::from_slice(utf8::REPLACEMENT_CHARACTER));
        }
        self.inner_sink.finish()
    }
}

/// A `TendrilSink` adaptor that takes bytes, decodes them as the given character encoding,
/// lossily replace ill-formed byte sequences with U+FFFD replacement characters,
/// and emits Unicode (`StrTendril`).
///
/// This allocates new tendrils for encodings other than UTF-8.
#[cfg(any(feature = "encoding", feature = "encoding_rs"))]
pub struct LossyDecoder<Sink, A = NonAtomic>
where
    Sink: TendrilSink<fmt::UTF8, A>,
    A: Atomicity,
{
    inner: LossyDecoderInner<Sink, A>,
}

#[cfg(any(feature = "encoding", feature = "encoding_rs"))]
enum LossyDecoderInner<Sink, A>
where
    Sink: TendrilSink<fmt::UTF8, A>,
    A: Atomicity,
{
    Utf8(Utf8LossyDecoder<Sink, A>),
    #[cfg(feature = "encoding")]
    Encoding(Box<encoding::RawDecoder>, Sink),
    #[cfg(feature = "encoding_rs")]
    EncodingRs(encoding_rs::Decoder, Sink),
}

#[cfg(any(feature = "encoding", feature = "encoding_rs"))]
impl<Sink, A> LossyDecoder<Sink, A>
where
    Sink: TendrilSink<fmt::UTF8, A>,
    A: Atomicity,
{
    /// Create a new incremental decoder using the encoding crate.
    #[cfg(feature = "encoding")]
    #[inline]
    pub fn new(encoding: encoding::EncodingRef, sink: Sink) -> Self {
        if encoding.name() == "utf-8" {
            LossyDecoder::utf8(sink)
        } else {
            LossyDecoder {
                inner: LossyDecoderInner::Encoding(encoding.raw_decoder(), sink),
            }
        }
    }

    /// Create a new incremental decoder using the encoding_rs crate.
    #[cfg(feature = "encoding_rs")]
    #[inline]
    pub fn new_encoding_rs(encoding: &'static encoding_rs::Encoding, sink: Sink) -> Self {
        if encoding == encoding_rs::UTF_8 {
            return Self::utf8(sink);
        }
        Self {
            inner: LossyDecoderInner::EncodingRs(encoding.new_decoder(), sink),
        }
    }

    /// Create a new incremental decoder for the UTF-8 encoding.
    ///
    /// This is useful for content that is known at run-time to be UTF-8
    /// (whereas `Utf8LossyDecoder` requires knowning at compile-time.)
    #[inline]
    pub fn utf8(sink: Sink) -> LossyDecoder<Sink, A> {
        LossyDecoder {
            inner: LossyDecoderInner::Utf8(Utf8LossyDecoder::new(sink)),
        }
    }

    /// Give a reference to the inner sink.
    pub fn inner_sink(&self) -> &Sink {
        match self.inner {
            LossyDecoderInner::Utf8(ref utf8) => &utf8.inner_sink,
            #[cfg(feature = "encoding")]
            LossyDecoderInner::Encoding(_, ref inner_sink) => inner_sink,
            #[cfg(feature = "encoding_rs")]
            LossyDecoderInner::EncodingRs(_, ref inner_sink) => inner_sink,
        }
    }

    /// Give a mutable reference to the inner sink.
    pub fn inner_sink_mut(&mut self) -> &mut Sink {
        match self.inner {
            LossyDecoderInner::Utf8(ref mut utf8) => &mut utf8.inner_sink,
            #[cfg(feature = "encoding")]
            LossyDecoderInner::Encoding(_, ref mut inner_sink) => inner_sink,
            #[cfg(feature = "encoding_rs")]
            LossyDecoderInner::EncodingRs(_, ref mut inner_sink) => inner_sink,
        }
    }
}

#[cfg(any(feature = "encoding", feature = "encoding_rs"))]
impl<Sink, A> TendrilSink<fmt::Bytes, A> for LossyDecoder<Sink, A>
where
    Sink: TendrilSink<fmt::UTF8, A>,
    A: Atomicity,
{
    #[inline]
    fn process(&mut self, t: Tendril<fmt::Bytes, A>) {
        match self.inner {
            LossyDecoderInner::Utf8(ref mut utf8) => return utf8.process(t),
            #[cfg(feature = "encoding")]
            LossyDecoderInner::Encoding(ref mut decoder, ref mut sink) => {
                let mut out = Tendril::new();
                let mut t = t;
                loop {
                    match decoder.raw_feed(&*t, &mut out) {
                        (_, Some(err)) => {
                            out.push_char('\u{fffd}');
                            sink.error(err.cause);
                            debug_assert!(err.upto >= 0);
                            t.pop_front(err.upto as u32);
                            // continue loop and process remainder of t
                        }
                        (_, None) => break,
                    }
                }
                if out.len() > 0 {
                    sink.process(out);
                }
            }
            #[cfg(feature = "encoding_rs")]
            LossyDecoderInner::EncodingRs(ref mut decoder, ref mut sink) => {
                if t.is_empty() {
                    return;
                }
                decode_to_sink(t, decoder, sink, false);
            }
        }
    }

    #[inline]
    fn error(&mut self, desc: Cow<'static, str>) {
        match self.inner {
            LossyDecoderInner::Utf8(ref mut utf8) => utf8.error(desc),
            #[cfg(feature = "encoding")]
            LossyDecoderInner::Encoding(_, ref mut sink) => sink.error(desc),
            #[cfg(feature = "encoding_rs")]
            LossyDecoderInner::EncodingRs(_, ref mut sink) => sink.error(desc),
        }
    }

    type Output = Sink::Output;

    #[inline]
    fn finish(self) -> Sink::Output {
        match self.inner {
            LossyDecoderInner::Utf8(utf8) => return utf8.finish(),
            #[cfg(feature = "encoding")]
            LossyDecoderInner::Encoding(mut decoder, mut sink) => {
                let mut out = Tendril::new();
                if let Some(err) = decoder.raw_finish(&mut out) {
                    out.push_char('\u{fffd}');
                    sink.error(err.cause);
                }
                if out.len() > 0 {
                    sink.process(out);
                }
                sink.finish()
            }
            #[cfg(feature = "encoding_rs")]
            LossyDecoderInner::EncodingRs(mut decoder, mut sink) => {
                decode_to_sink(Tendril::new(), &mut decoder, &mut sink, true);
                sink.finish()
            }
        }
    }
}

#[cfg(feature = "encoding_rs")]
fn decode_to_sink<Sink, A>(
    mut t: Tendril<fmt::Bytes, A>,
    decoder: &mut encoding_rs::Decoder,
    sink: &mut Sink,
    last: bool,
) where
    Sink: TendrilSink<fmt::UTF8, A>,
    A: Atomicity,
{
    loop {
        let mut out = <Tendril<fmt::Bytes, A>>::new();
        let max_len = decoder
            .max_utf8_buffer_length_without_replacement(t.len())
            .unwrap_or(8192);
        unsafe {
            out.push_uninitialized(std::cmp::min(max_len as u32, 8192));
        }
        let (result, bytes_read, bytes_written) =
            decoder.decode_to_utf8_without_replacement(&t, &mut out, last);
        if bytes_written > 0 {
            sink.process(unsafe {
                out.subtendril(0, bytes_written as u32)
                    .reinterpret_without_validating()
            });
        }
        match result {
            DecoderResult::InputEmpty => return,
            DecoderResult::OutputFull => {}
            DecoderResult::Malformed(_, _) => {
                sink.error(Cow::Borrowed("invalid sequence"));
                sink.process("\u{FFFD}".into());
            }
        }
        t.pop_front(bytes_read as u32);
        if t.is_empty() {
            return;
        }
    }
}

#[cfg(test)]
mod test {
    use super::{TendrilSink, Utf8LossyDecoder};
    use fmt;
    use std::borrow::Cow;
    use tendril::{Atomicity, NonAtomic, Tendril};

    #[cfg(any(feature = "encoding", feature = "encoding_rs"))]
    use super::LossyDecoder;
    #[cfg(any(feature = "encoding", feature = "encoding_rs"))]
    use tendril::SliceExt;

    #[cfg(feature = "encoding")]
    use encoding::all as enc;
    #[cfg(feature = "encoding_rs")]
    use encoding_rs as enc_rs;

    struct Accumulate<A>
    where
        A: Atomicity,
    {
        tendrils: Vec<Tendril<fmt::UTF8, A>>,
        errors: Vec<String>,
    }

    impl<A> Accumulate<A>
    where
        A: Atomicity,
    {
        fn new() -> Accumulate<A> {
            Accumulate {
                tendrils: vec![],
                errors: vec![],
            }
        }
    }

    impl<A> TendrilSink<fmt::UTF8, A> for Accumulate<A>
    where
        A: Atomicity,
    {
        fn process(&mut self, t: Tendril<fmt::UTF8, A>) {
            self.tendrils.push(t);
        }

        fn error(&mut self, desc: Cow<'static, str>) {
            self.errors.push(desc.into_owned());
        }

        type Output = (Vec<Tendril<fmt::UTF8, A>>, Vec<String>);

        fn finish(self) -> Self::Output {
            (self.tendrils, self.errors)
        }
    }

    fn check_utf8(input: &[&[u8]], expected: &[&str], errs: usize) {
        let decoder = Utf8LossyDecoder::new(Accumulate::<NonAtomic>::new());
        let (tendrils, errors) = decoder.from_iter(input.iter().cloned());
        assert_eq!(
            expected,
            &*tendrils.iter().map(|t| &**t).collect::<Vec<_>>()
        );
        assert_eq!(errs, errors.len());
    }

    #[test]
    fn utf8() {
        check_utf8(&[], &[], 0);
        check_utf8(&[b""], &[], 0);
        check_utf8(&[b"xyz"], &["xyz"], 0);
        check_utf8(&[b"x", b"y", b"z"], &["x", "y", "z"], 0);

        check_utf8(&[b"xy\xEA\x99\xAEzw"], &["xy\u{a66e}zw"], 0);
        check_utf8(&[b"xy\xEA", b"\x99\xAEzw"], &["xy", "\u{a66e}z", "w"], 0);
        check_utf8(&[b"xy\xEA\x99", b"\xAEzw"], &["xy", "\u{a66e}z", "w"], 0);
        check_utf8(
            &[b"xy\xEA", b"\x99", b"\xAEzw"],
            &["xy", "\u{a66e}z", "w"],
            0,
        );
        check_utf8(&[b"\xEA", b"", b"\x99", b"", b"\xAE"], &["\u{a66e}"], 0);
        check_utf8(
            &[b"", b"\xEA", b"", b"\x99", b"", b"\xAE", b""],
            &["\u{a66e}"],
            0,
        );

        check_utf8(
            &[b"xy\xEA", b"\xFF", b"\x99\xAEz"],
            &["xy", "\u{fffd}", "\u{fffd}", "\u{fffd}", "\u{fffd}", "z"],
            4,
        );
        check_utf8(
            &[b"xy\xEA\x99", b"\xFFz"],
            &["xy", "\u{fffd}", "\u{fffd}", "z"],
            2,
        );

        check_utf8(&[b"\xC5\x91\xC5\x91\xC5\x91"], &["őőő"], 0);
        check_utf8(
            &[b"\xC5\x91", b"\xC5\x91", b"\xC5\x91"],
            &["ő", "ő", "ő"],
            0,
        );
        check_utf8(
            &[b"\xC5", b"\x91\xC5", b"\x91\xC5", b"\x91"],
            &["ő", "ő", "ő"],
            0,
        );
        check_utf8(
            &[b"\xC5", b"\x91\xff", b"\x91\xC5", b"\x91"],
            &["ő", "\u{fffd}", "\u{fffd}", "ő"],
            2,
        );

        // incomplete char at end of input
        check_utf8(&[b"\xC0"], &["\u{fffd}"], 1);
        check_utf8(&[b"\xEA\x99"], &["\u{fffd}"], 1);
    }

    #[cfg(any(feature = "encoding", feature = "encoding_rs"))]
    fn check_decode(
        mut decoder: LossyDecoder<Accumulate<NonAtomic>>,
        input: &[&[u8]],
        expected: &str,
        errs: usize,
    ) {
        for x in input {
            decoder.process(x.to_tendril());
        }
        let (tendrils, errors) = decoder.finish();
        let mut tendril: Tendril<fmt::UTF8> = Tendril::new();
        for t in tendrils {
            tendril.push_tendril(&t);
        }
        assert_eq!(expected, &*tendril);
        assert_eq!(errs, errors.len());
    }

    #[cfg(any(feature = "encoding", feature = "encoding_rs"))]
    pub type Tests = &'static [(&'static [&'static [u8]], &'static str, usize)];

    #[cfg(any(feature = "encoding"))]
    const ASCII: Tests = &[
        (&[], "", 0),
        (&[b""], "", 0),
        (&[b"xyz"], "xyz", 0),
        (&[b"xy", b"", b"", b"z"], "xyz", 0),
        (&[b"x", b"y", b"z"], "xyz", 0),
        (&[b"\xFF"], "\u{fffd}", 1),
        (&[b"x\xC0yz"], "x\u{fffd}yz", 1),
        (&[b"x", b"\xC0y", b"z"], "x\u{fffd}yz", 1),
        (&[b"x\xC0yz\xFF\xFFw"], "x\u{fffd}yz\u{fffd}\u{fffd}w", 3),
    ];

    #[cfg(feature = "encoding")]
    #[test]
    fn decode_ascii() {
        for &(input, expected, errs) in ASCII {
            let decoder = LossyDecoder::new(enc::ASCII, Accumulate::new());
            check_decode(decoder, input, expected, errs);
        }
    }

    #[cfg(any(feature = "encoding", feature = "encoding_rs"))]
    const UTF_8: Tests = &[
        (&[], "", 0),
        (&[b""], "", 0),
        (&[b"xyz"], "xyz", 0),
        (&[b"x", b"y", b"z"], "xyz", 0),
        (&[b"\xEA\x99\xAE"], "\u{a66e}", 0),
        (&[b"\xEA", b"\x99\xAE"], "\u{a66e}", 0),
        (&[b"\xEA\x99", b"\xAE"], "\u{a66e}", 0),
        (&[b"\xEA", b"\x99", b"\xAE"], "\u{a66e}", 0),
        (&[b"\xEA", b"", b"\x99", b"", b"\xAE"], "\u{a66e}", 0),
        (
            &[b"", b"\xEA", b"", b"\x99", b"", b"\xAE", b""],
            "\u{a66e}",
            0,
        ),
        (&[b"xy\xEA", b"\x99\xAEz"], "xy\u{a66e}z", 0),
        (
            &[b"xy\xEA", b"\xFF", b"\x99\xAEz"],
            "xy\u{fffd}\u{fffd}\u{fffd}\u{fffd}z",
            4,
        ),
        (&[b"xy\xEA\x99", b"\xFFz"], "xy\u{fffd}\u{fffd}z", 2),
        // incomplete char at end of input
        (&[b"\xC0"], "\u{fffd}", 1),
        (&[b"\xEA\x99"], "\u{fffd}", 1),
    ];

    #[cfg(feature = "encoding")]
    #[test]
    fn decode_utf8() {
        for &(input, expected, errs) in UTF_8 {
            let decoder = LossyDecoder::new(enc::UTF_8, Accumulate::new());
            check_decode(decoder, input, expected, errs);
        }
    }

    #[cfg(feature = "encoding_rs")]
    #[test]
    fn decode_utf8_encoding_rs() {
        for &(input, expected, errs) in UTF_8 {
            let decoder = LossyDecoder::new_encoding_rs(enc_rs::UTF_8, Accumulate::new());
            check_decode(decoder, input, expected, errs);
        }
    }

    #[cfg(any(feature = "encoding", feature = "encoding_rs"))]
    const KOI8_U: Tests = &[
        (&[b"\xfc\xce\xc5\xd2\xc7\xc9\xd1"], "Энергия", 0),
        (&[b"\xfc\xce", b"\xc5\xd2\xc7\xc9\xd1"], "Энергия", 0),
        (&[b"\xfc\xce", b"\xc5\xd2\xc7", b"\xc9\xd1"], "Энергия", 0),
        (
            &[b"\xfc\xce", b"", b"\xc5\xd2\xc7", b"\xc9\xd1", b""],
            "Энергия",
            0,
        ),
    ];

    #[cfg(feature = "encoding")]
    #[test]
    fn decode_koi8_u() {
        for &(input, expected, errs) in KOI8_U {
            let decoder = LossyDecoder::new(enc::KOI8_U, Accumulate::new());
            check_decode(decoder, input, expected, errs);
        }
    }

    #[cfg(feature = "encoding_rs")]
    #[test]
    fn decode_koi8_u_encoding_rs() {
        for &(input, expected, errs) in KOI8_U {
            let decoder = LossyDecoder::new_encoding_rs(enc_rs::KOI8_U, Accumulate::new());
            check_decode(decoder, input, expected, errs);
        }
    }

    #[cfg(any(feature = "encoding", feature = "encoding_rs"))]
    const WINDOWS_949: Tests = &[
        (&[], "", 0),
        (&[b""], "", 0),
        (&[b"\xbe\xc8\xb3\xe7"], "안녕", 0),
        (&[b"\xbe", b"\xc8\xb3\xe7"], "안녕", 0),
        (&[b"\xbe", b"", b"\xc8\xb3\xe7"], "안녕", 0),
        (
            &[b"\xbe\xc8\xb3\xe7\xc7\xcf\xbc\xbc\xbf\xe4"],
            "안녕하세요",
            0,
        ),
        (&[b"\xbe\xc8\xb3\xe7\xc7"], "안녕\u{fffd}", 1),
        (&[b"\xbe", b"", b"\xc8\xb3"], "안\u{fffd}", 1),
        (&[b"\xbe\x28\xb3\xe7"], "\u{fffd}(녕", 1),
    ];

    #[cfg(feature = "encoding")]
    #[test]
    fn decode_windows_949() {
        for &(input, expected, errs) in WINDOWS_949 {
            let decoder = LossyDecoder::new(enc::WINDOWS_949, Accumulate::new());
            check_decode(decoder, input, expected, errs);
        }
    }

    #[cfg(feature = "encoding_rs")]
    #[test]
    fn decode_windows_949_encoding_rs() {
        for &(input, expected, errs) in WINDOWS_949 {
            let decoder = LossyDecoder::new_encoding_rs(enc_rs::EUC_KR, Accumulate::new());
            check_decode(decoder, input, expected, errs);
        }
    }

    #[test]
    fn read_from() {
        let decoder = Utf8LossyDecoder::new(Accumulate::<NonAtomic>::new());
        let mut bytes: &[u8] = b"foo\xffbar";
        let (tendrils, errors) = decoder.read_from(&mut bytes).unwrap();
        assert_eq!(
            &*tendrils.iter().map(|t| &**t).collect::<Vec<_>>(),
            &["foo", "\u{FFFD}", "bar"]
        );
        assert_eq!(errors, &["invalid byte sequence"]);
    }
}
