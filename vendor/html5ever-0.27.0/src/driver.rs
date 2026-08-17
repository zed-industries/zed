// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! High-level interface to the parser.

use crate::buffer_queue::BufferQueue;
use crate::tokenizer::{Tokenizer, TokenizerOpts, TokenizerResult};
use crate::tree_builder::{create_element, TreeBuilder, TreeBuilderOpts, TreeSink};
use crate::{Attribute, QualName};

use std::borrow::Cow;

use crate::tendril;
use crate::tendril::stream::{TendrilSink, Utf8LossyDecoder};
use crate::tendril::StrTendril;

/// All-encompassing options struct for the parser.
#[derive(Clone, Default)]
pub struct ParseOpts {
    /// Tokenizer options.
    pub tokenizer: TokenizerOpts,

    /// Tree builder options.
    pub tree_builder: TreeBuilderOpts,
}

/// Parse an HTML document
///
/// The returned value implements `tendril::TendrilSink`
/// so that Unicode input may be provided incrementally,
/// or all at once with the `one` method.
///
/// If your input is bytes, use `Parser::from_utf8`.
pub fn parse_document<Sink>(sink: Sink, opts: ParseOpts) -> Parser<Sink>
where
    Sink: TreeSink,
{
    let tb = TreeBuilder::new(sink, opts.tree_builder);
    let tok = Tokenizer::new(tb, opts.tokenizer);
    Parser {
        tokenizer: tok,
        input_buffer: BufferQueue::default(),
    }
}

/// Parse an HTML fragment
///
/// The returned value implements `tendril::TendrilSink`
/// so that Unicode input may be provided incrementally,
/// or all at once with the `one` method.
///
/// If your input is bytes, use `Parser::from_utf8`.
pub fn parse_fragment<Sink>(
    mut sink: Sink,
    opts: ParseOpts,
    context_name: QualName,
    context_attrs: Vec<Attribute>,
) -> Parser<Sink>
where
    Sink: TreeSink,
{
    let context_elem = create_element(&mut sink, context_name, context_attrs);
    parse_fragment_for_element(sink, opts, context_elem, None)
}

/// Like `parse_fragment`, but with an existing context element
/// and optionally a form element.
pub fn parse_fragment_for_element<Sink>(
    sink: Sink,
    opts: ParseOpts,
    context_element: Sink::Handle,
    form_element: Option<Sink::Handle>,
) -> Parser<Sink>
where
    Sink: TreeSink,
{
    let tb = TreeBuilder::new_for_fragment(sink, context_element, form_element, opts.tree_builder);
    let tok_opts = TokenizerOpts {
        initial_state: Some(tb.tokenizer_state_for_context_elem()),
        ..opts.tokenizer
    };
    let tok = Tokenizer::new(tb, tok_opts);
    Parser {
        tokenizer: tok,
        input_buffer: BufferQueue::default(),
    }
}

/// An HTML parser,
/// ready to receive Unicode input through the `tendril::TendrilSink` trait’s methods.
pub struct Parser<Sink>
where
    Sink: TreeSink,
{
    pub tokenizer: Tokenizer<TreeBuilder<Sink::Handle, Sink>>,
    pub input_buffer: BufferQueue,
}

impl<Sink: TreeSink> TendrilSink<tendril::fmt::UTF8> for Parser<Sink> {
    fn process(&mut self, t: StrTendril) {
        self.input_buffer.push_back(t);
        // FIXME: Properly support </script> somehow.
        while let TokenizerResult::Script(_) = self.tokenizer.feed(&mut self.input_buffer) {}
    }

    // FIXME: Is it too noisy to report every character decoding error?
    fn error(&mut self, desc: Cow<'static, str>) {
        self.tokenizer.sink.sink.parse_error(desc)
    }

    type Output = Sink::Output;

    fn finish(mut self) -> Self::Output {
        // FIXME: Properly support </script> somehow.
        while let TokenizerResult::Script(_) = self.tokenizer.feed(&mut self.input_buffer) {}
        assert!(self.input_buffer.is_empty());
        self.tokenizer.end();
        self.tokenizer.sink.sink.finish()
    }
}

impl<Sink: TreeSink> Parser<Sink> {
    /// Wrap this parser into a `TendrilSink` that accepts UTF-8 bytes.
    ///
    /// Use this when your input is bytes that are known to be in the UTF-8 encoding.
    /// Decoding is lossy, like `String::from_utf8_lossy`.
    #[allow(clippy::wrong_self_convention)]
    pub fn from_utf8(self) -> Utf8LossyDecoder<Self> {
        Utf8LossyDecoder::new(self)
    }
}
