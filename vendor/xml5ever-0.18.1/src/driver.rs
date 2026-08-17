// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::tokenizer::{XmlTokenizer, XmlTokenizerOpts};
use crate::tree_builder::{TreeSink, XmlTreeBuilder, XmlTreeBuilderOpts};

use std::borrow::Cow;

use crate::tendril;
use crate::tendril::stream::{TendrilSink, Utf8LossyDecoder};
use crate::tendril::StrTendril;
use markup5ever::buffer_queue::BufferQueue;

/// All-encompasing parser setting structure.
#[derive(Clone, Default)]
pub struct XmlParseOpts {
    /// Xml tokenizer options.
    pub tokenizer: XmlTokenizerOpts,
    /// Xml tree builder .
    pub tree_builder: XmlTreeBuilderOpts,
}

/// Parse and send results to a `TreeSink`.
///
/// ## Example
///
/// ```ignore
/// let mut sink = MySink;
/// parse_document(&mut sink, iter::once(my_str), Default::default());
/// ```
pub fn parse_document<Sink>(sink: Sink, opts: XmlParseOpts) -> XmlParser<Sink>
where
    Sink: TreeSink,
{
    let tb = XmlTreeBuilder::new(sink, opts.tree_builder);
    let tok = XmlTokenizer::new(tb, opts.tokenizer);
    XmlParser {
        tokenizer: tok,
        input_buffer: BufferQueue::default(),
    }
}

/// An XML parser,
/// ready to receive Unicode input through the `tendril::TendrilSink` trait’s methods.
pub struct XmlParser<Sink>
where
    Sink: TreeSink,
{
    /// Tokenizer used by XmlParser.
    pub tokenizer: XmlTokenizer<XmlTreeBuilder<Sink::Handle, Sink>>,
    /// Input used by XmlParser.
    pub input_buffer: BufferQueue,
}

impl<Sink: TreeSink> TendrilSink<tendril::fmt::UTF8> for XmlParser<Sink> {
    type Output = Sink::Output;

    fn process(&mut self, t: StrTendril) {
        self.input_buffer.push_back(t);
        self.tokenizer.feed(&mut self.input_buffer);
    }

    // FIXME: Is it too noisy to report every character decoding error?
    fn error(&mut self, desc: Cow<'static, str>) {
        self.tokenizer.sink.sink.parse_error(desc)
    }

    fn finish(mut self) -> Self::Output {
        self.tokenizer.end();
        self.tokenizer.sink.sink.finish()
    }
}

impl<Sink: TreeSink> XmlParser<Sink> {
    /// Wrap this parser into a `TendrilSink` that accepts UTF-8 bytes.
    ///
    /// Use this when your input is bytes that are known to be in the UTF-8 encoding.
    /// Decoding is lossy, like `String::from_utf8_lossy`.
    #[allow(clippy::wrong_self_convention)]
    pub fn from_utf8(self) -> Utf8LossyDecoder<Self> {
        Utf8LossyDecoder::new(self)
    }
}
