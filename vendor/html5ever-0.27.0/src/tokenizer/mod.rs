// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The HTML5 tokenizer.

pub use self::interface::{CharacterTokens, EOFToken, NullCharacterToken, ParseError};
pub use self::interface::{CommentToken, DoctypeToken, TagToken, Token};
pub use self::interface::{Doctype, EndTag, StartTag, Tag, TagKind};
pub use self::interface::{TokenSink, TokenSinkResult};

use self::states::{DoctypeIdKind, Public, System};
use self::states::{DoubleEscaped, Escaped};
use self::states::{DoubleQuoted, SingleQuoted, Unquoted};
use self::states::{Rawtext, Rcdata, ScriptData, ScriptDataEscaped};

use self::char_ref::{CharRef, CharRefTokenizer};

use crate::util::str::lower_ascii_letter;

use log::{debug, trace};
use mac::format_if;
use markup5ever::{namespace_url, ns, small_char_set};
use std::borrow::Cow::{self, Borrowed};
use std::collections::BTreeMap;
use std::mem;

pub use crate::buffer_queue::{BufferQueue, FromSet, NotFromSet, SetResult};
use crate::tendril::StrTendril;
use crate::{Attribute, LocalName, QualName, SmallCharSet};

mod char_ref;
mod interface;
pub mod states;

pub enum ProcessResult<Handle> {
    Continue,
    Suspend,
    Script(Handle),
}

#[must_use]
#[derive(Debug)]
pub enum TokenizerResult<Handle> {
    Done,
    Script(Handle),
}

fn option_push(opt_str: &mut Option<StrTendril>, c: char) {
    match *opt_str {
        Some(ref mut s) => s.push_char(c),
        None => *opt_str = Some(StrTendril::from_char(c)),
    }
}

/// Tokenizer options, with an impl for `Default`.
#[derive(Clone)]
pub struct TokenizerOpts {
    /// Report all parse errors described in the spec, at some
    /// performance penalty?  Default: false
    pub exact_errors: bool,

    /// Discard a `U+FEFF BYTE ORDER MARK` if we see one at the beginning
    /// of the stream?  Default: true
    pub discard_bom: bool,

    /// Keep a record of how long we spent in each state?  Printed
    /// when `end()` is called.  Default: false
    pub profile: bool,

    /// Initial state override.  Only the test runner should use
    /// a non-`None` value!
    pub initial_state: Option<states::State>,

    /// Last start tag.  Only the test runner should use a
    /// non-`None` value!
    ///
    /// FIXME: Can't use Tendril because we want TokenizerOpts
    /// to be Send.
    pub last_start_tag_name: Option<String>,
}

impl Default for TokenizerOpts {
    fn default() -> TokenizerOpts {
        TokenizerOpts {
            exact_errors: false,
            discard_bom: true,
            profile: false,
            initial_state: None,
            last_start_tag_name: None,
        }
    }
}

/// The HTML tokenizer.
pub struct Tokenizer<Sink> {
    /// Options controlling the behavior of the tokenizer.
    opts: TokenizerOpts,

    /// Destination for tokens we emit.
    pub sink: Sink,

    /// The abstract machine state as described in the spec.
    state: states::State,

    /// Are we at the end of the file, once buffers have been processed
    /// completely? This affects whether we will wait for lookahead or not.
    at_eof: bool,

    /// Tokenizer for character references, if we're tokenizing
    /// one at the moment.
    char_ref_tokenizer: Option<Box<CharRefTokenizer>>,

    /// Current input character.  Just consumed, may reconsume.
    current_char: char,

    /// Should we reconsume the current input character?
    reconsume: bool,

    /// Did we just consume \r, translating it to \n?  In that case we need
    /// to ignore the next character if it's \n.
    ignore_lf: bool,

    /// Discard a U+FEFF BYTE ORDER MARK if we see one?  Only done at the
    /// beginning of the stream.
    discard_bom: bool,

    /// Current tag kind.
    current_tag_kind: TagKind,

    /// Current tag name.
    current_tag_name: StrTendril,

    /// Current tag is self-closing?
    current_tag_self_closing: bool,

    /// Current tag attributes.
    current_tag_attrs: Vec<Attribute>,

    /// Current attribute name.
    current_attr_name: StrTendril,

    /// Current attribute value.
    current_attr_value: StrTendril,

    /// Current comment.
    current_comment: StrTendril,

    /// Current doctype token.
    current_doctype: Doctype,

    /// Last start tag name, for use in checking "appropriate end tag".
    last_start_tag_name: Option<LocalName>,

    /// The "temporary buffer" mentioned in the spec.
    temp_buf: StrTendril,

    /// Record of how many ns we spent in each state, if profiling is enabled.
    state_profile: BTreeMap<states::State, u64>,

    /// Record of how many ns we spent in the token sink.
    time_in_sink: u64,

    /// Track current line
    current_line: u64,
}

impl<Sink: TokenSink> Tokenizer<Sink> {
    /// Create a new tokenizer which feeds tokens to a particular `TokenSink`.
    pub fn new(sink: Sink, mut opts: TokenizerOpts) -> Tokenizer<Sink> {
        let start_tag_name = opts
            .last_start_tag_name
            .take()
            .map(|s| LocalName::from(&*s));
        let state = opts.initial_state.unwrap_or(states::Data);
        let discard_bom = opts.discard_bom;
        Tokenizer {
            opts,
            sink,
            state,
            char_ref_tokenizer: None,
            at_eof: false,
            current_char: '\0',
            reconsume: false,
            ignore_lf: false,
            discard_bom,
            current_tag_kind: StartTag,
            current_tag_name: StrTendril::new(),
            current_tag_self_closing: false,
            current_tag_attrs: vec![],
            current_attr_name: StrTendril::new(),
            current_attr_value: StrTendril::new(),
            current_comment: StrTendril::new(),
            current_doctype: Doctype::default(),
            last_start_tag_name: start_tag_name,
            temp_buf: StrTendril::new(),
            state_profile: BTreeMap::new(),
            time_in_sink: 0,
            current_line: 1,
        }
    }

    /// Feed an input string into the tokenizer.
    pub fn feed(&mut self, input: &mut BufferQueue) -> TokenizerResult<Sink::Handle> {
        if input.is_empty() {
            return TokenizerResult::Done;
        }

        if self.discard_bom {
            if let Some(c) = input.peek() {
                if c == '\u{feff}' {
                    input.next();
                }
            } else {
                return TokenizerResult::Done;
            }
        };

        self.run(input)
    }

    pub fn set_plaintext_state(&mut self) {
        self.state = states::Plaintext;
    }

    fn process_token(&mut self, token: Token) -> TokenSinkResult<Sink::Handle> {
        if self.opts.profile {
            let (ret, dt) = time!(self.sink.process_token(token, self.current_line));
            self.time_in_sink += dt;
            ret
        } else {
            self.sink.process_token(token, self.current_line)
        }
    }

    fn process_token_and_continue(&mut self, token: Token) {
        assert!(matches!(
            self.process_token(token),
            TokenSinkResult::Continue
        ));
    }

    //§ preprocessing-the-input-stream
    // Get the next input character, which might be the character
    // 'c' that we already consumed from the buffers.
    fn get_preprocessed_char(&mut self, mut c: char, input: &mut BufferQueue) -> Option<char> {
        if self.ignore_lf {
            self.ignore_lf = false;
            if c == '\n' {
                c = unwrap_or_return!(input.next(), None);
            }
        }

        if c == '\r' {
            self.ignore_lf = true;
            c = '\n';
        }

        if c == '\n' {
            self.current_line += 1;
        }

        if self.opts.exact_errors
            && match c as u32 {
                0x01..=0x08 | 0x0B | 0x0E..=0x1F | 0x7F..=0x9F | 0xFDD0..=0xFDEF => true,
                n if (n & 0xFFFE) == 0xFFFE => true,
                _ => false,
            }
        {
            let msg = format!("Bad character {}", c);
            self.emit_error(Cow::Owned(msg));
        }

        trace!("got character {}", c);
        self.current_char = c;
        Some(c)
    }

    //§ tokenization
    // Get the next input character, if one is available.
    fn get_char(&mut self, input: &mut BufferQueue) -> Option<char> {
        if self.reconsume {
            self.reconsume = false;
            Some(self.current_char)
        } else {
            input
                .next()
                .and_then(|c| self.get_preprocessed_char(c, input))
        }
    }

    fn pop_except_from(&mut self, input: &mut BufferQueue, set: SmallCharSet) -> Option<SetResult> {
        // Bail to the slow path for various corner cases.
        // This means that `FromSet` can contain characters not in the set!
        // It shouldn't matter because the fallback `FromSet` case should
        // always do the same thing as the `NotFromSet` case.
        if self.opts.exact_errors || self.reconsume || self.ignore_lf {
            return self.get_char(input).map(FromSet);
        }

        let d = input.pop_except_from(set);
        trace!("got characters {:?}", d);
        match d {
            Some(FromSet(c)) => self.get_preprocessed_char(c, input).map(FromSet),

            // NB: We don't set self.current_char for a run of characters not
            // in the set.  It shouldn't matter for the codepaths that use
            // this.
            _ => d,
        }
    }

    // Check if the next characters are an ASCII case-insensitive match.  See
    // BufferQueue::eat.
    //
    // NB: this doesn't set the current input character.
    fn eat(
        &mut self,
        input: &mut BufferQueue,
        pat: &str,
        eq: fn(&u8, &u8) -> bool,
    ) -> Option<bool> {
        if self.ignore_lf {
            self.ignore_lf = false;
            if self.peek(input) == Some('\n') {
                self.discard_char(input);
            }
        }

        input.push_front(mem::take(&mut self.temp_buf));
        match input.eat(pat, eq) {
            None if self.at_eof => Some(false),
            None => {
                self.temp_buf.extend(input);
                None
            },
            Some(matched) => Some(matched),
        }
    }

    /// Run the state machine for as long as we can.
    fn run(&mut self, input: &mut BufferQueue) -> TokenizerResult<Sink::Handle> {
        if self.opts.profile {
            loop {
                let state = self.state;
                let old_sink = self.time_in_sink;
                let (run, mut dt) = time!(self.step(input));
                dt -= (self.time_in_sink - old_sink);
                let new = match self.state_profile.get_mut(&state) {
                    Some(x) => {
                        *x += dt;
                        false
                    },
                    None => true,
                };
                if new {
                    // do this here because of borrow shenanigans
                    self.state_profile.insert(state, dt);
                }
                match run {
                    ProcessResult::Continue => (),
                    ProcessResult::Suspend => break,
                    ProcessResult::Script(node) => return TokenizerResult::Script(node),
                }
            }
        } else {
            loop {
                match self.step(input) {
                    ProcessResult::Continue => (),
                    ProcessResult::Suspend => break,
                    ProcessResult::Script(node) => return TokenizerResult::Script(node),
                }
            }
        }
        TokenizerResult::Done
    }

    fn bad_char_error(&mut self) {
        let msg = format_if!(
            self.opts.exact_errors,
            "Bad character",
            "Saw {} in state {:?}",
            self.current_char,
            self.state
        );
        self.emit_error(msg);
    }

    fn bad_eof_error(&mut self) {
        let msg = format_if!(
            self.opts.exact_errors,
            "Unexpected EOF",
            "Saw EOF in state {:?}",
            self.state
        );
        self.emit_error(msg);
    }

    fn emit_char(&mut self, c: char) {
        self.process_token_and_continue(match c {
            '\0' => NullCharacterToken,
            _ => CharacterTokens(StrTendril::from_char(c)),
        });
    }

    // The string must not contain '\0'!
    fn emit_chars(&mut self, b: StrTendril) {
        self.process_token_and_continue(CharacterTokens(b));
    }

    fn emit_current_tag(&mut self) -> ProcessResult<Sink::Handle> {
        self.finish_attribute();

        let name = LocalName::from(&*self.current_tag_name);
        self.current_tag_name.clear();

        match self.current_tag_kind {
            StartTag => {
                self.last_start_tag_name = Some(name.clone());
            },
            EndTag => {
                if !self.current_tag_attrs.is_empty() {
                    self.emit_error(Borrowed("Attributes on an end tag"));
                }
                if self.current_tag_self_closing {
                    self.emit_error(Borrowed("Self-closing end tag"));
                }
            },
        }

        let token = TagToken(Tag {
            kind: self.current_tag_kind,
            name,
            self_closing: self.current_tag_self_closing,
            attrs: std::mem::take(&mut self.current_tag_attrs),
        });

        match self.process_token(token) {
            TokenSinkResult::Continue => ProcessResult::Continue,
            TokenSinkResult::Plaintext => {
                self.state = states::Plaintext;
                ProcessResult::Continue
            },
            TokenSinkResult::Script(node) => {
                self.state = states::Data;
                ProcessResult::Script(node)
            },
            TokenSinkResult::RawData(kind) => {
                self.state = states::RawData(kind);
                ProcessResult::Continue
            },
        }
    }

    fn emit_temp_buf(&mut self) {
        // FIXME: Make sure that clearing on emit is spec-compatible.
        let buf = mem::take(&mut self.temp_buf);
        self.emit_chars(buf);
    }

    fn clear_temp_buf(&mut self) {
        // Do this without a new allocation.
        self.temp_buf.clear();
    }

    fn emit_current_comment(&mut self) {
        let comment = mem::take(&mut self.current_comment);
        self.process_token_and_continue(CommentToken(comment));
    }

    fn discard_tag(&mut self) {
        self.current_tag_name.clear();
        self.current_tag_self_closing = false;
        self.current_tag_attrs = vec![];
    }

    fn create_tag(&mut self, kind: TagKind, c: char) {
        self.discard_tag();
        self.current_tag_name.push_char(c);
        self.current_tag_kind = kind;
    }

    fn have_appropriate_end_tag(&self) -> bool {
        match self.last_start_tag_name.as_ref() {
            Some(last) => (self.current_tag_kind == EndTag) && (*self.current_tag_name == **last),
            None => false,
        }
    }

    fn create_attribute(&mut self, c: char) {
        self.finish_attribute();

        self.current_attr_name.push_char(c);
    }

    fn finish_attribute(&mut self) {
        if self.current_attr_name.is_empty() {
            return;
        }

        // Check for a duplicate attribute.
        // FIXME: the spec says we should error as soon as the name is finished.
        let dup = {
            let name = &*self.current_attr_name;
            self.current_tag_attrs
                .iter()
                .any(|a| &*a.name.local == name)
        };

        if dup {
            self.emit_error(Borrowed("Duplicate attribute"));
            self.current_attr_name.clear();
            self.current_attr_value.clear();
        } else {
            let name = LocalName::from(&*self.current_attr_name);
            self.current_attr_name.clear();
            self.current_tag_attrs.push(Attribute {
                // The tree builder will adjust the namespace if necessary.
                // This only happens in foreign elements.
                name: QualName::new(None, ns!(), name),
                value: mem::take(&mut self.current_attr_value),
            });
        }
    }

    fn emit_current_doctype(&mut self) {
        let doctype = mem::take(&mut self.current_doctype);
        self.process_token_and_continue(DoctypeToken(doctype));
    }

    fn doctype_id(&mut self, kind: DoctypeIdKind) -> &mut Option<StrTendril> {
        match kind {
            Public => &mut self.current_doctype.public_id,
            System => &mut self.current_doctype.system_id,
        }
    }

    fn clear_doctype_id(&mut self, kind: DoctypeIdKind) {
        let id = self.doctype_id(kind);
        match *id {
            Some(ref mut s) => s.clear(),
            None => *id = Some(StrTendril::new()),
        }
    }

    fn consume_char_ref(&mut self) {
        self.char_ref_tokenizer = Some(Box::new(CharRefTokenizer::new(matches!(
            self.state,
            states::AttributeValue(_)
        ))));
    }

    fn emit_eof(&mut self) {
        self.process_token_and_continue(EOFToken);
    }

    fn peek(&mut self, input: &BufferQueue) -> Option<char> {
        if self.reconsume {
            Some(self.current_char)
        } else {
            input.peek()
        }
    }

    fn discard_char(&mut self, input: &mut BufferQueue) {
        // peek() deals in un-processed characters (no newline normalization), while get_char()
        // does.
        //
        // since discard_char is supposed to be used in combination with peek(), discard_char must
        // discard a single raw input character, not a normalized newline.
        if self.reconsume {
            self.reconsume = false;
        } else {
            input.next();
        }
    }

    fn emit_error(&mut self, error: Cow<'static, str>) {
        self.process_token_and_continue(ParseError(error));
    }
}
//§ END

// Shorthand for common state machine behaviors.
macro_rules! shorthand (
    ( $me:ident : emit $c:expr                     ) => ( $me.emit_char($c)                                   );
    ( $me:ident : create_tag $kind:ident $c:expr   ) => ( $me.create_tag($kind, $c)                           );
    ( $me:ident : push_tag $c:expr                 ) => ( $me.current_tag_name.push_char($c)                  );
    ( $me:ident : discard_tag                      ) => ( $me.discard_tag()                                   );
    ( $me:ident : discard_char $input:expr         ) => ( $me.discard_char($input)                            );
    ( $me:ident : push_temp $c:expr                ) => ( $me.temp_buf.push_char($c)                          );
    ( $me:ident : emit_temp                        ) => ( $me.emit_temp_buf()                                 );
    ( $me:ident : clear_temp                       ) => ( $me.clear_temp_buf()                                );
    ( $me:ident : create_attr $c:expr              ) => ( $me.create_attribute($c)                            );
    ( $me:ident : push_name $c:expr                ) => ( $me.current_attr_name.push_char($c)                 );
    ( $me:ident : push_value $c:expr               ) => ( $me.current_attr_value.push_char($c)                );
    ( $me:ident : append_value $c:expr             ) => ( $me.current_attr_value.push_tendril($c)             );
    ( $me:ident : push_comment $c:expr             ) => ( $me.current_comment.push_char($c)                   );
    ( $me:ident : append_comment $c:expr           ) => ( $me.current_comment.push_slice($c)                  );
    ( $me:ident : emit_comment                     ) => ( $me.emit_current_comment()                          );
    ( $me:ident : clear_comment                    ) => ( $me.current_comment.clear()                         );
    ( $me:ident : create_doctype                   ) => ( $me.current_doctype = Doctype::default()            );
    ( $me:ident : push_doctype_name $c:expr        ) => ( option_push(&mut $me.current_doctype.name, $c)      );
    ( $me:ident : push_doctype_id $k:ident $c:expr ) => ( option_push($me.doctype_id($k), $c)                 );
    ( $me:ident : clear_doctype_id $k:ident        ) => ( $me.clear_doctype_id($k)                            );
    ( $me:ident : force_quirks                     ) => ( $me.current_doctype.force_quirks = true             );
    ( $me:ident : emit_doctype                     ) => ( $me.emit_current_doctype()                          );
    ( $me:ident : error                            ) => ( $me.bad_char_error()                                );
    ( $me:ident : error_eof                        ) => ( $me.bad_eof_error()                                 );
);

// Tracing of tokenizer actions.  This adds significant bloat and compile time,
// so it's behind a cfg flag.
#[cfg(trace_tokenizer)]
macro_rules! sh_trace ( ( $me:ident : $($cmds:tt)* ) => ({
    trace!("  {:s}", stringify!($($cmds)*));
    shorthand!($me:expr : $($cmds)*);
}));

#[cfg(not(trace_tokenizer))]
macro_rules! sh_trace ( ( $me:ident : $($cmds:tt)* ) => ( shorthand!($me: $($cmds)*) ) );

// A little DSL for sequencing shorthand actions.
macro_rules! go (
    // A pattern like $($cmd:tt)* ; $($rest:tt)* causes parse ambiguity.
    // We have to tell the parser how much lookahead we need.

    ( $me:ident : $a:tt                   ; $($rest:tt)* ) => ({ sh_trace!($me: $a);          go!($me: $($rest)*); });
    ( $me:ident : $a:tt $b:tt             ; $($rest:tt)* ) => ({ sh_trace!($me: $a $b);       go!($me: $($rest)*); });
    ( $me:ident : $a:tt $b:tt $c:tt       ; $($rest:tt)* ) => ({ sh_trace!($me: $a $b $c);    go!($me: $($rest)*); });
    ( $me:ident : $a:tt $b:tt $c:tt $d:tt ; $($rest:tt)* ) => ({ sh_trace!($me: $a $b $c $d); go!($me: $($rest)*); });

    // These can only come at the end.

    ( $me:ident : to $s:ident                    ) => ({ $me.state = states::$s; return ProcessResult::Continue;           });
    ( $me:ident : to $s:ident $k1:expr           ) => ({ $me.state = states::$s($k1); return ProcessResult::Continue;      });
    ( $me:ident : to $s:ident $k1:ident $k2:expr ) => ({ $me.state = states::$s($k1($k2)); return ProcessResult::Continue; });

    ( $me:ident : reconsume $s:ident                    ) => ({ $me.reconsume = true; go!($me: to $s);         });
    ( $me:ident : reconsume $s:ident $k1:expr           ) => ({ $me.reconsume = true; go!($me: to $s $k1);     });
    ( $me:ident : reconsume $s:ident $k1:ident $k2:expr ) => ({ $me.reconsume = true; go!($me: to $s $k1 $k2); });

    ( $me:ident : consume_char_ref             ) => ({ $me.consume_char_ref(); return ProcessResult::Continue;         });

    // We have a default next state after emitting a tag, but the sink can override.
    ( $me:ident : emit_tag $s:ident ) => ({
        $me.state = states::$s;
        return $me.emit_current_tag();
    });

    ( $me:ident : eof ) => ({ $me.emit_eof(); return ProcessResult::Suspend; });

    // If nothing else matched, it's a single command
    ( $me:ident : $($cmd:tt)+ ) => ( sh_trace!($me: $($cmd)+) );

    // or nothing.
    ( $me:ident : ) => (());
);

macro_rules! go_match ( ( $me:ident : $x:expr, $($pats:pat),+ => $($cmds:tt)* ) => (
    match $x {
        $($pats)|+ => go!($me: $($cmds)*),
        _ => (),
    }
));

// This is a macro because it can cause early return
// from the function where it is used.
macro_rules! get_char ( ($me:expr, $input:expr) => (
    unwrap_or_return!($me.get_char($input), ProcessResult::Suspend)
));

macro_rules! peek ( ($me:expr, $input:expr) => (
    unwrap_or_return!($me.peek($input), ProcessResult::Suspend)
));

macro_rules! pop_except_from ( ($me:expr, $input:expr, $set:expr) => (
    unwrap_or_return!($me.pop_except_from($input, $set), ProcessResult::Suspend)
));

macro_rules! eat ( ($me:expr, $input:expr, $pat:expr) => (
    unwrap_or_return!($me.eat($input, $pat, u8::eq_ignore_ascii_case), ProcessResult::Suspend)
));

macro_rules! eat_exact ( ($me:expr, $input:expr, $pat:expr) => (
    unwrap_or_return!($me.eat($input, $pat, u8::eq), ProcessResult::Suspend)
));

impl<Sink: TokenSink> Tokenizer<Sink> {
    // Run the state machine for a while.
    // Return true if we should be immediately re-invoked
    // (this just simplifies control flow vs. break / continue).
    #[allow(clippy::never_loop)]
    fn step(&mut self, input: &mut BufferQueue) -> ProcessResult<Sink::Handle> {
        if self.char_ref_tokenizer.is_some() {
            return self.step_char_ref_tokenizer(input);
        }

        trace!("processing in state {:?}", self.state);
        match self.state {
            //§ data-state
            states::Data => loop {
                match pop_except_from!(self, input, small_char_set!('\r' '\0' '&' '<' '\n')) {
                    FromSet('\0') => go!(self: error; emit '\0'),
                    FromSet('&') => go!(self: consume_char_ref),
                    FromSet('<') => go!(self: to TagOpen),
                    FromSet(c) => go!(self: emit c),
                    NotFromSet(b) => self.emit_chars(b),
                }
            },

            //§ rcdata-state
            states::RawData(Rcdata) => loop {
                match pop_except_from!(self, input, small_char_set!('\r' '\0' '&' '<' '\n')) {
                    FromSet('\0') => go!(self: error; emit '\u{fffd}'),
                    FromSet('&') => go!(self: consume_char_ref),
                    FromSet('<') => go!(self: to RawLessThanSign Rcdata),
                    FromSet(c) => go!(self: emit c),
                    NotFromSet(b) => self.emit_chars(b),
                }
            },

            //§ rawtext-state
            states::RawData(Rawtext) => loop {
                match pop_except_from!(self, input, small_char_set!('\r' '\0' '<' '\n')) {
                    FromSet('\0') => go!(self: error; emit '\u{fffd}'),
                    FromSet('<') => go!(self: to RawLessThanSign Rawtext),
                    FromSet(c) => go!(self: emit c),
                    NotFromSet(b) => self.emit_chars(b),
                }
            },

            //§ script-data-state
            states::RawData(ScriptData) => loop {
                match pop_except_from!(self, input, small_char_set!('\r' '\0' '<' '\n')) {
                    FromSet('\0') => go!(self: error; emit '\u{fffd}'),
                    FromSet('<') => go!(self: to RawLessThanSign ScriptData),
                    FromSet(c) => go!(self: emit c),
                    NotFromSet(b) => self.emit_chars(b),
                }
            },

            //§ script-data-escaped-state
            states::RawData(ScriptDataEscaped(Escaped)) => loop {
                match pop_except_from!(self, input, small_char_set!('\r' '\0' '-' '<' '\n')) {
                    FromSet('\0') => go!(self: error; emit '\u{fffd}'),
                    FromSet('-') => go!(self: emit '-'; to ScriptDataEscapedDash Escaped),
                    FromSet('<') => go!(self: to RawLessThanSign ScriptDataEscaped Escaped),
                    FromSet(c) => go!(self: emit c),
                    NotFromSet(b) => self.emit_chars(b),
                }
            },

            //§ script-data-double-escaped-state
            states::RawData(ScriptDataEscaped(DoubleEscaped)) => loop {
                match pop_except_from!(self, input, small_char_set!('\r' '\0' '-' '<' '\n')) {
                    FromSet('\0') => go!(self: error; emit '\u{fffd}'),
                    FromSet('-') => go!(self: emit '-'; to ScriptDataEscapedDash DoubleEscaped),
                    FromSet('<') => {
                        go!(self: emit '<'; to RawLessThanSign ScriptDataEscaped DoubleEscaped)
                    },
                    FromSet(c) => go!(self: emit c),
                    NotFromSet(b) => self.emit_chars(b),
                }
            },

            //§ plaintext-state
            states::Plaintext => loop {
                match pop_except_from!(self, input, small_char_set!('\r' '\0' '\n')) {
                    FromSet('\0') => go!(self: error; emit '\u{fffd}'),
                    FromSet(c) => go!(self: emit c),
                    NotFromSet(b) => self.emit_chars(b),
                }
            },

            //§ tag-open-state
            states::TagOpen => loop {
                match get_char!(self, input) {
                    '!' => go!(self: to MarkupDeclarationOpen),
                    '/' => go!(self: to EndTagOpen),
                    '?' => go!(self: error; clear_comment; reconsume BogusComment),
                    c => match lower_ascii_letter(c) {
                        Some(cl) => go!(self: create_tag StartTag cl; to TagName),
                        None => go!(self: error; emit '<'; reconsume Data),
                    },
                }
            },

            //§ end-tag-open-state
            states::EndTagOpen => loop {
                match get_char!(self, input) {
                    '>' => go!(self: error; to Data),
                    c => match lower_ascii_letter(c) {
                        Some(cl) => go!(self: create_tag EndTag cl; to TagName),
                        None => go!(self: error; clear_comment; reconsume BogusComment),
                    },
                }
            },

            //§ tag-name-state
            states::TagName => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => go!(self: to BeforeAttributeName),
                    '/' => go!(self: to SelfClosingStartTag),
                    '>' => go!(self: emit_tag Data),
                    '\0' => go!(self: error; push_tag '\u{fffd}'),
                    c => go!(self: push_tag (c.to_ascii_lowercase())),
                }
            },

            //§ script-data-escaped-less-than-sign-state
            states::RawLessThanSign(ScriptDataEscaped(Escaped)) => loop {
                match get_char!(self, input) {
                    '/' => go!(self: clear_temp; to RawEndTagOpen ScriptDataEscaped Escaped),
                    c => match lower_ascii_letter(c) {
                        Some(cl) => go!(self: clear_temp; push_temp cl; emit '<'; emit c;
                                    to ScriptDataEscapeStart DoubleEscaped),
                        None => go!(self: emit '<'; reconsume RawData ScriptDataEscaped Escaped),
                    },
                }
            },

            //§ script-data-double-escaped-less-than-sign-state
            states::RawLessThanSign(ScriptDataEscaped(DoubleEscaped)) => loop {
                match get_char!(self, input) {
                    '/' => go!(self: clear_temp; emit '/'; to ScriptDataDoubleEscapeEnd),
                    _ => go!(self: reconsume RawData ScriptDataEscaped DoubleEscaped),
                }
            },

            //§ rcdata-less-than-sign-state rawtext-less-than-sign-state script-data-less-than-sign-state
            // otherwise
            states::RawLessThanSign(kind) => loop {
                match get_char!(self, input) {
                    '/' => go!(self: clear_temp; to RawEndTagOpen kind),
                    '!' if kind == ScriptData => {
                        go!(self: emit '<'; emit '!'; to ScriptDataEscapeStart Escaped)
                    },
                    _ => go!(self: emit '<'; reconsume RawData kind),
                }
            },

            //§ rcdata-end-tag-open-state rawtext-end-tag-open-state script-data-end-tag-open-state script-data-escaped-end-tag-open-state
            states::RawEndTagOpen(kind) => loop {
                let c = get_char!(self, input);
                match lower_ascii_letter(c) {
                    Some(cl) => go!(self: create_tag EndTag cl; push_temp c; to RawEndTagName kind),
                    None => go!(self: emit '<'; emit '/'; reconsume RawData kind),
                }
            },

            //§ rcdata-end-tag-name-state rawtext-end-tag-name-state script-data-end-tag-name-state script-data-escaped-end-tag-name-state
            states::RawEndTagName(kind) => loop {
                let c = get_char!(self, input);
                if self.have_appropriate_end_tag() {
                    match c {
                        '\t' | '\n' | '\x0C' | ' ' => go!(self: clear_temp; to BeforeAttributeName),
                        '/' => go!(self: clear_temp; to SelfClosingStartTag),
                        '>' => go!(self: clear_temp; emit_tag Data),
                        _ => (),
                    }
                }

                match lower_ascii_letter(c) {
                    Some(cl) => go!(self: push_tag cl; push_temp c),
                    None => {
                        go!(self: discard_tag; emit '<'; emit '/'; emit_temp; reconsume RawData kind)
                    },
                }
            },

            //§ script-data-double-escape-start-state
            states::ScriptDataEscapeStart(DoubleEscaped) => loop {
                let c = get_char!(self, input);
                match c {
                    '\t' | '\n' | '\x0C' | ' ' | '/' | '>' => {
                        let esc = if &*self.temp_buf == "script" {
                            DoubleEscaped
                        } else {
                            Escaped
                        };
                        go!(self: emit c; to RawData ScriptDataEscaped esc);
                    },
                    _ => match lower_ascii_letter(c) {
                        Some(cl) => go!(self: push_temp cl; emit c),
                        None => go!(self: reconsume RawData ScriptDataEscaped Escaped),
                    },
                }
            },

            //§ script-data-escape-start-state
            states::ScriptDataEscapeStart(Escaped) => loop {
                match get_char!(self, input) {
                    '-' => go!(self: emit '-'; to ScriptDataEscapeStartDash),
                    _ => go!(self: reconsume RawData ScriptData),
                }
            },

            //§ script-data-escape-start-dash-state
            states::ScriptDataEscapeStartDash => loop {
                match get_char!(self, input) {
                    '-' => go!(self: emit '-'; to ScriptDataEscapedDashDash Escaped),
                    _ => go!(self: reconsume RawData ScriptData),
                }
            },

            //§ script-data-escaped-dash-state script-data-double-escaped-dash-state
            states::ScriptDataEscapedDash(kind) => loop {
                match get_char!(self, input) {
                    '-' => go!(self: emit '-'; to ScriptDataEscapedDashDash kind),
                    '<' => {
                        if kind == DoubleEscaped {
                            go!(self: emit '<');
                        }
                        go!(self: to RawLessThanSign ScriptDataEscaped kind);
                    },
                    '\0' => go!(self: error; emit '\u{fffd}'; to RawData ScriptDataEscaped kind),
                    c => go!(self: emit c; to RawData ScriptDataEscaped kind),
                }
            },

            //§ script-data-escaped-dash-dash-state script-data-double-escaped-dash-dash-state
            states::ScriptDataEscapedDashDash(kind) => loop {
                match get_char!(self, input) {
                    '-' => go!(self: emit '-'),
                    '<' => {
                        if kind == DoubleEscaped {
                            go!(self: emit '<');
                        }
                        go!(self: to RawLessThanSign ScriptDataEscaped kind);
                    },
                    '>' => go!(self: emit '>'; to RawData ScriptData),
                    '\0' => go!(self: error; emit '\u{fffd}'; to RawData ScriptDataEscaped kind),
                    c => go!(self: emit c; to RawData ScriptDataEscaped kind),
                }
            },

            //§ script-data-double-escape-end-state
            states::ScriptDataDoubleEscapeEnd => loop {
                let c = get_char!(self, input);
                match c {
                    '\t' | '\n' | '\x0C' | ' ' | '/' | '>' => {
                        let esc = if &*self.temp_buf == "script" {
                            Escaped
                        } else {
                            DoubleEscaped
                        };
                        go!(self: emit c; to RawData ScriptDataEscaped esc);
                    },
                    _ => match lower_ascii_letter(c) {
                        Some(cl) => go!(self: push_temp cl; emit c),
                        None => go!(self: reconsume RawData ScriptDataEscaped DoubleEscaped),
                    },
                }
            },

            //§ before-attribute-name-state
            states::BeforeAttributeName => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => (),
                    '/' => go!(self: to SelfClosingStartTag),
                    '>' => go!(self: emit_tag Data),
                    '\0' => go!(self: error; create_attr '\u{fffd}'; to AttributeName),
                    c => match lower_ascii_letter(c) {
                        Some(cl) => go!(self: create_attr cl; to AttributeName),
                        None => {
                            go_match!(self: c,
                            '"' , '\'' , '<' , '=' => error);
                            go!(self: create_attr c; to AttributeName);
                        },
                    },
                }
            },

            //§ attribute-name-state
            states::AttributeName => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => go!(self: to AfterAttributeName),
                    '/' => go!(self: to SelfClosingStartTag),
                    '=' => go!(self: to BeforeAttributeValue),
                    '>' => go!(self: emit_tag Data),
                    '\0' => go!(self: error; push_name '\u{fffd}'),
                    c => match lower_ascii_letter(c) {
                        Some(cl) => go!(self: push_name cl),
                        None => {
                            go_match!(self: c,
                            '"' , '\'' , '<' => error);
                            go!(self: push_name c);
                        },
                    },
                }
            },

            //§ after-attribute-name-state
            states::AfterAttributeName => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => (),
                    '/' => go!(self: to SelfClosingStartTag),
                    '=' => go!(self: to BeforeAttributeValue),
                    '>' => go!(self: emit_tag Data),
                    '\0' => go!(self: error; create_attr '\u{fffd}'; to AttributeName),
                    c => match lower_ascii_letter(c) {
                        Some(cl) => go!(self: create_attr cl; to AttributeName),
                        None => {
                            go_match!(self: c,
                            '"' , '\'' , '<' => error);
                            go!(self: create_attr c; to AttributeName);
                        },
                    },
                }
            },

            //§ before-attribute-value-state
            // Use peek so we can handle the first attr character along with the rest,
            // hopefully in the same zero-copy buffer.
            states::BeforeAttributeValue => loop {
                match peek!(self, input) {
                    '\t' | '\n' | '\r' | '\x0C' | ' ' => go!(self: discard_char input),
                    '"' => go!(self: discard_char input; to AttributeValue DoubleQuoted),
                    '\'' => go!(self: discard_char input; to AttributeValue SingleQuoted),
                    '>' => go!(self: discard_char input; error; emit_tag Data),
                    _ => go!(self: to AttributeValue Unquoted),
                }
            },

            //§ attribute-value-(double-quoted)-state
            states::AttributeValue(DoubleQuoted) => loop {
                match pop_except_from!(self, input, small_char_set!('\r' '"' '&' '\0' '\n')) {
                    FromSet('"') => go!(self: to AfterAttributeValueQuoted),
                    FromSet('&') => go!(self: consume_char_ref),
                    FromSet('\0') => go!(self: error; push_value '\u{fffd}'),
                    FromSet(c) => go!(self: push_value c),
                    NotFromSet(ref b) => go!(self: append_value b),
                }
            },

            //§ attribute-value-(single-quoted)-state
            states::AttributeValue(SingleQuoted) => loop {
                match pop_except_from!(self, input, small_char_set!('\r' '\'' '&' '\0' '\n')) {
                    FromSet('\'') => go!(self: to AfterAttributeValueQuoted),
                    FromSet('&') => go!(self: consume_char_ref),
                    FromSet('\0') => go!(self: error; push_value '\u{fffd}'),
                    FromSet(c) => go!(self: push_value c),
                    NotFromSet(ref b) => go!(self: append_value b),
                }
            },

            //§ attribute-value-(unquoted)-state
            states::AttributeValue(Unquoted) => loop {
                match pop_except_from!(
                    self,
                    input,
                    small_char_set!('\r' '\t' '\n' '\x0C' ' ' '&' '>' '\0')
                ) {
                    FromSet('\t') | FromSet('\n') | FromSet('\x0C') | FromSet(' ') => {
                        go!(self: to BeforeAttributeName)
                    },
                    FromSet('&') => go!(self: consume_char_ref),
                    FromSet('>') => go!(self: emit_tag Data),
                    FromSet('\0') => go!(self: error; push_value '\u{fffd}'),
                    FromSet(c) => {
                        go_match!(self: c,
                            '"' , '\'' , '<' , '=' , '`' => error);
                        go!(self: push_value c);
                    },
                    NotFromSet(ref b) => go!(self: append_value b),
                }
            },

            //§ after-attribute-value-(quoted)-state
            states::AfterAttributeValueQuoted => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => go!(self: to BeforeAttributeName),
                    '/' => go!(self: to SelfClosingStartTag),
                    '>' => go!(self: emit_tag Data),
                    _ => go!(self: error; reconsume BeforeAttributeName),
                }
            },

            //§ self-closing-start-tag-state
            states::SelfClosingStartTag => loop {
                match get_char!(self, input) {
                    '>' => {
                        self.current_tag_self_closing = true;
                        go!(self: emit_tag Data);
                    },
                    _ => go!(self: error; reconsume BeforeAttributeName),
                }
            },

            //§ comment-start-state
            states::CommentStart => loop {
                match get_char!(self, input) {
                    '-' => go!(self: to CommentStartDash),
                    '\0' => go!(self: error; push_comment '\u{fffd}'; to Comment),
                    '>' => go!(self: error; emit_comment; to Data),
                    c => go!(self: push_comment c; to Comment),
                }
            },

            //§ comment-start-dash-state
            states::CommentStartDash => loop {
                match get_char!(self, input) {
                    '-' => go!(self: to CommentEnd),
                    '\0' => go!(self: error; append_comment "-\u{fffd}"; to Comment),
                    '>' => go!(self: error; emit_comment; to Data),
                    c => go!(self: push_comment '-'; push_comment c; to Comment),
                }
            },

            //§ comment-state
            states::Comment => loop {
                match get_char!(self, input) {
                    c @ '<' => go!(self: push_comment c; to CommentLessThanSign),
                    '-' => go!(self: to CommentEndDash),
                    '\0' => go!(self: error; push_comment '\u{fffd}'),
                    c => go!(self: push_comment c),
                }
            },

            //§ comment-less-than-sign-state
            states::CommentLessThanSign => loop {
                match get_char!(self, input) {
                    c @ '!' => go!(self: push_comment c; to CommentLessThanSignBang),
                    c @ '<' => go!(self: push_comment c),
                    _ => go!(self: reconsume Comment),
                }
            },

            //§ comment-less-than-sign-bang
            states::CommentLessThanSignBang => loop {
                match get_char!(self, input) {
                    '-' => go!(self: to CommentLessThanSignBangDash),
                    _ => go!(self: reconsume Comment),
                }
            },

            //§ comment-less-than-sign-bang-dash
            states::CommentLessThanSignBangDash => loop {
                match get_char!(self, input) {
                    '-' => go!(self: to CommentLessThanSignBangDashDash),
                    _ => go!(self: reconsume CommentEndDash),
                }
            },

            //§ comment-less-than-sign-bang-dash-dash
            states::CommentLessThanSignBangDashDash => loop {
                match get_char!(self, input) {
                    '>' => go!(self: reconsume CommentEnd),
                    _ => go!(self: error; reconsume CommentEnd),
                }
            },

            //§ comment-end-dash-state
            states::CommentEndDash => loop {
                match get_char!(self, input) {
                    '-' => go!(self: to CommentEnd),
                    '\0' => go!(self: error; append_comment "-\u{fffd}"; to Comment),
                    c => go!(self: push_comment '-'; push_comment c; to Comment),
                }
            },

            //§ comment-end-state
            states::CommentEnd => loop {
                match get_char!(self, input) {
                    '>' => go!(self: emit_comment; to Data),
                    '!' => go!(self: to CommentEndBang),
                    '-' => go!(self: push_comment '-'),
                    _ => go!(self: append_comment "--"; reconsume Comment),
                }
            },

            //§ comment-end-bang-state
            states::CommentEndBang => loop {
                match get_char!(self, input) {
                    '-' => go!(self: append_comment "--!"; to CommentEndDash),
                    '>' => go!(self: error; emit_comment; to Data),
                    '\0' => go!(self: error; append_comment "--!\u{fffd}"; to Comment),
                    c => go!(self: append_comment "--!"; push_comment c; to Comment),
                }
            },

            //§ doctype-state
            states::Doctype => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => go!(self: to BeforeDoctypeName),
                    '>' => go!(self: reconsume BeforeDoctypeName),
                    _ => go!(self: error; reconsume BeforeDoctypeName),
                }
            },

            //§ before-doctype-name-state
            states::BeforeDoctypeName => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => (),
                    '\0' => {
                        go!(self: error; create_doctype; push_doctype_name '\u{fffd}'; to DoctypeName)
                    },
                    '>' => go!(self: error; create_doctype; force_quirks; emit_doctype; to Data),
                    c => go!(self: create_doctype; push_doctype_name (c.to_ascii_lowercase());
                                  to DoctypeName),
                }
            },

            //§ doctype-name-state
            states::DoctypeName => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => go!(self: clear_temp; to AfterDoctypeName),
                    '>' => go!(self: emit_doctype; to Data),
                    '\0' => go!(self: error; push_doctype_name '\u{fffd}'),
                    c => go!(self: push_doctype_name (c.to_ascii_lowercase())),
                }
            },

            //§ after-doctype-name-state
            states::AfterDoctypeName => loop {
                if eat!(self, input, "public") {
                    go!(self: to AfterDoctypeKeyword Public);
                } else if eat!(self, input, "system") {
                    go!(self: to AfterDoctypeKeyword System);
                } else {
                    match get_char!(self, input) {
                        '\t' | '\n' | '\x0C' | ' ' => (),
                        '>' => go!(self: emit_doctype; to Data),
                        _ => go!(self: error; force_quirks; reconsume BogusDoctype),
                    }
                }
            },

            //§ after-doctype-public-keyword-state after-doctype-system-keyword-state
            states::AfterDoctypeKeyword(kind) => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => go!(self: to BeforeDoctypeIdentifier kind),
                    '"' => {
                        go!(self: error; clear_doctype_id kind; to DoctypeIdentifierDoubleQuoted kind)
                    },
                    '\'' => {
                        go!(self: error; clear_doctype_id kind; to DoctypeIdentifierSingleQuoted kind)
                    },
                    '>' => go!(self: error; force_quirks; emit_doctype; to Data),
                    _ => go!(self: error; force_quirks; reconsume BogusDoctype),
                }
            },

            //§ before-doctype-public-identifier-state before-doctype-system-identifier-state
            states::BeforeDoctypeIdentifier(kind) => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => (),
                    '"' => go!(self: clear_doctype_id kind; to DoctypeIdentifierDoubleQuoted kind),
                    '\'' => go!(self: clear_doctype_id kind; to DoctypeIdentifierSingleQuoted kind),
                    '>' => go!(self: error; force_quirks; emit_doctype; to Data),
                    _ => go!(self: error; force_quirks; reconsume BogusDoctype),
                }
            },

            //§ doctype-public-identifier-(double-quoted)-state doctype-system-identifier-(double-quoted)-state
            states::DoctypeIdentifierDoubleQuoted(kind) => loop {
                match get_char!(self, input) {
                    '"' => go!(self: to AfterDoctypeIdentifier kind),
                    '\0' => go!(self: error; push_doctype_id kind '\u{fffd}'),
                    '>' => go!(self: error; force_quirks; emit_doctype; to Data),
                    c => go!(self: push_doctype_id kind c),
                }
            },

            //§ doctype-public-identifier-(single-quoted)-state doctype-system-identifier-(single-quoted)-state
            states::DoctypeIdentifierSingleQuoted(kind) => loop {
                match get_char!(self, input) {
                    '\'' => go!(self: to AfterDoctypeIdentifier kind),
                    '\0' => go!(self: error; push_doctype_id kind '\u{fffd}'),
                    '>' => go!(self: error; force_quirks; emit_doctype; to Data),
                    c => go!(self: push_doctype_id kind c),
                }
            },

            //§ after-doctype-public-identifier-state
            states::AfterDoctypeIdentifier(Public) => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => {
                        go!(self: to BetweenDoctypePublicAndSystemIdentifiers)
                    },
                    '>' => go!(self: emit_doctype; to Data),
                    '"' => {
                        go!(self: error; clear_doctype_id System; to DoctypeIdentifierDoubleQuoted System)
                    },
                    '\'' => {
                        go!(self: error; clear_doctype_id System; to DoctypeIdentifierSingleQuoted System)
                    },
                    _ => go!(self: error; force_quirks; reconsume BogusDoctype),
                }
            },

            //§ after-doctype-system-identifier-state
            states::AfterDoctypeIdentifier(System) => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => (),
                    '>' => go!(self: emit_doctype; to Data),
                    _ => go!(self: error; reconsume BogusDoctype),
                }
            },

            //§ between-doctype-public-and-system-identifiers-state
            states::BetweenDoctypePublicAndSystemIdentifiers => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => (),
                    '>' => go!(self: emit_doctype; to Data),
                    '"' => {
                        go!(self: clear_doctype_id System; to DoctypeIdentifierDoubleQuoted System)
                    },
                    '\'' => {
                        go!(self: clear_doctype_id System; to DoctypeIdentifierSingleQuoted System)
                    },
                    _ => go!(self: error; force_quirks; reconsume BogusDoctype),
                }
            },

            //§ bogus-doctype-state
            states::BogusDoctype => loop {
                match get_char!(self, input) {
                    '>' => go!(self: emit_doctype; to Data),
                    '\0' => go!(self: error),
                    _ => (),
                }
            },

            //§ bogus-comment-state
            states::BogusComment => loop {
                match get_char!(self, input) {
                    '>' => go!(self: emit_comment; to Data),
                    '\0' => go!(self: error; push_comment '\u{fffd}'),
                    c => go!(self: push_comment c),
                }
            },

            //§ markup-declaration-open-state
            states::MarkupDeclarationOpen => loop {
                if eat_exact!(self, input, "--") {
                    go!(self: clear_comment; to CommentStart);
                } else if eat!(self, input, "doctype") {
                    go!(self: to Doctype);
                } else {
                    if self
                        .sink
                        .adjusted_current_node_present_but_not_in_html_namespace()
                        && eat_exact!(self, input, "[CDATA[")
                    {
                        go!(self: clear_temp; to CdataSection);
                    }
                    go!(self: error; clear_comment; to BogusComment);
                }
            },

            //§ cdata-section-state
            states::CdataSection => loop {
                match get_char!(self, input) {
                    ']' => go!(self: to CdataSectionBracket),
                    '\0' => go!(self: emit_temp; emit '\0'),
                    c => go!(self: push_temp c),
                }
            },

            //§ cdata-section-bracket
            states::CdataSectionBracket => match get_char!(self, input) {
                ']' => go!(self: to CdataSectionEnd),
                _ => go!(self: push_temp ']'; reconsume CdataSection),
            },

            //§ cdata-section-end
            states::CdataSectionEnd => loop {
                match get_char!(self, input) {
                    ']' => go!(self: push_temp ']'),
                    '>' => go!(self: emit_temp; to Data),
                    _ => go!(self: push_temp ']'; push_temp ']'; reconsume CdataSection),
                }
            },
            //§ END
        }
    }

    fn step_char_ref_tokenizer(&mut self, input: &mut BufferQueue) -> ProcessResult<Sink::Handle> {
        // FIXME HACK: Take and replace the tokenizer so we don't
        // double-mut-borrow self.  This is why it's boxed.
        let mut tok = self.char_ref_tokenizer.take().unwrap();
        let outcome = tok.step(self, input);

        let progress = match outcome {
            char_ref::Done => {
                self.process_char_ref(tok.get_result());
                return ProcessResult::Continue;
            },

            char_ref::Stuck => ProcessResult::Suspend,
            char_ref::Progress => ProcessResult::Continue,
        };

        self.char_ref_tokenizer = Some(tok);
        progress
    }

    fn process_char_ref(&mut self, char_ref: CharRef) {
        let CharRef {
            mut chars,
            mut num_chars,
        } = char_ref;

        if num_chars == 0 {
            chars[0] = '&';
            num_chars = 1;
        }

        for i in 0..num_chars {
            let c = chars[i as usize];
            match self.state {
                states::Data | states::RawData(states::Rcdata) => go!(self: emit c),

                states::AttributeValue(_) => go!(self: push_value c),

                _ => panic!(
                    "state {:?} should not be reachable in process_char_ref",
                    self.state
                ),
            }
        }
    }

    /// Indicate that we have reached the end of the input.
    pub fn end(&mut self) {
        // Handle EOF in the char ref sub-tokenizer, if there is one.
        // Do this first because it might un-consume stuff.
        let mut input = BufferQueue::default();
        match self.char_ref_tokenizer.take() {
            None => (),
            Some(mut tok) => {
                tok.end_of_file(self, &mut input);
                self.process_char_ref(tok.get_result());
            },
        }

        // Process all remaining buffered input.
        // If we're waiting for lookahead, we're not gonna get it.
        self.at_eof = true;
        assert!(matches!(self.run(&mut input), TokenizerResult::Done));
        assert!(input.is_empty());

        loop {
            match self.eof_step() {
                ProcessResult::Continue => (),
                ProcessResult::Suspend => break,
                ProcessResult::Script(_) => unreachable!(),
            }
        }

        self.sink.end();

        if self.opts.profile {
            self.dump_profile();
        }
    }

    fn dump_profile(&self) {
        let mut results: Vec<(states::State, u64)> =
            self.state_profile.iter().map(|(s, t)| (*s, *t)).collect();
        results.sort_by(|&(_, x), &(_, y)| y.cmp(&x));

        let total: u64 = results
            .iter()
            .map(|&(_, t)| t)
            .fold(0, ::std::ops::Add::add);
        println!("\nTokenizer profile, in nanoseconds");
        println!("\n{:12}         total in token sink", self.time_in_sink);
        println!("\n{:12}         total in tokenizer", total);

        for (k, v) in results.into_iter() {
            let pct = 100.0 * (v as f64) / (total as f64);
            println!("{:12}  {:4.1}%  {:?}", v, pct, k);
        }
    }

    fn eof_step(&mut self) -> ProcessResult<Sink::Handle> {
        debug!("processing EOF in state {:?}", self.state);
        match self.state {
            states::Data
            | states::RawData(Rcdata)
            | states::RawData(Rawtext)
            | states::RawData(ScriptData)
            | states::Plaintext => go!(self: eof),

            states::TagName
            | states::RawData(ScriptDataEscaped(_))
            | states::BeforeAttributeName
            | states::AttributeName
            | states::AfterAttributeName
            | states::AttributeValue(_)
            | states::AfterAttributeValueQuoted
            | states::SelfClosingStartTag
            | states::ScriptDataEscapedDash(_)
            | states::ScriptDataEscapedDashDash(_) => go!(self: error_eof; to Data),

            states::BeforeAttributeValue => go!(self: reconsume AttributeValue Unquoted),

            states::TagOpen => go!(self: error_eof; emit '<'; to Data),

            states::EndTagOpen => go!(self: error_eof; emit '<'; emit '/'; to Data),

            states::RawLessThanSign(ScriptDataEscaped(DoubleEscaped)) => {
                go!(self: to RawData ScriptDataEscaped DoubleEscaped)
            },

            states::RawLessThanSign(kind) => go!(self: emit '<'; to RawData kind),

            states::RawEndTagOpen(kind) => go!(self: emit '<'; emit '/'; to RawData kind),

            states::RawEndTagName(kind) => {
                go!(self: emit '<'; emit '/'; emit_temp; to RawData kind)
            },

            states::ScriptDataEscapeStart(kind) => go!(self: to RawData ScriptDataEscaped kind),

            states::ScriptDataEscapeStartDash => go!(self: to RawData ScriptData),

            states::ScriptDataDoubleEscapeEnd => {
                go!(self: to RawData ScriptDataEscaped DoubleEscaped)
            },

            states::CommentStart
            | states::CommentStartDash
            | states::Comment
            | states::CommentEndDash
            | states::CommentEnd
            | states::CommentEndBang => go!(self: error_eof; emit_comment; to Data),

            states::CommentLessThanSign | states::CommentLessThanSignBang => {
                go!(self: reconsume Comment)
            },

            states::CommentLessThanSignBangDash => go!(self: reconsume CommentEndDash),

            states::CommentLessThanSignBangDashDash => go!(self: reconsume CommentEnd),

            states::Doctype | states::BeforeDoctypeName => {
                go!(self: error_eof; create_doctype; force_quirks; emit_doctype; to Data)
            },

            states::DoctypeName
            | states::AfterDoctypeName
            | states::AfterDoctypeKeyword(_)
            | states::BeforeDoctypeIdentifier(_)
            | states::DoctypeIdentifierDoubleQuoted(_)
            | states::DoctypeIdentifierSingleQuoted(_)
            | states::AfterDoctypeIdentifier(_)
            | states::BetweenDoctypePublicAndSystemIdentifiers => {
                go!(self: error_eof; force_quirks; emit_doctype; to Data)
            },

            states::BogusDoctype => go!(self: emit_doctype; to Data),

            states::BogusComment => go!(self: emit_comment; to Data),

            states::MarkupDeclarationOpen => go!(self: error; to BogusComment),

            states::CdataSection => go!(self: emit_temp; error_eof; to Data),

            states::CdataSectionBracket => go!(self: push_temp ']'; to CdataSection),

            states::CdataSectionEnd => go!(self: push_temp ']'; push_temp ']'; to CdataSection),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::option_push; // private items
    use crate::tendril::{SliceExt, StrTendril};

    use super::{TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts};

    use super::interface::{CharacterTokens, EOFToken, NullCharacterToken, ParseError};
    use super::interface::{EndTag, StartTag, Tag, TagKind};
    use super::interface::{TagToken, Token};

    use markup5ever::buffer_queue::BufferQueue;
    use std::mem;

    use crate::LocalName;

    // LinesMatch implements the TokenSink trait. It is used for testing to see
    // if current_line is being updated when process_token is called. The lines
    // vector is a collection of the line numbers that each token is on.
    struct LinesMatch {
        tokens: Vec<Token>,
        current_str: StrTendril,
        lines: Vec<(Token, u64)>,
    }

    impl LinesMatch {
        fn new() -> LinesMatch {
            LinesMatch {
                tokens: vec![],
                current_str: StrTendril::new(),
                lines: vec![],
            }
        }

        fn push(&mut self, token: Token, line_number: u64) {
            self.finish_str();
            self.lines.push((token, line_number));
        }

        fn finish_str(&mut self) {
            if self.current_str.len() > 0 {
                let s = mem::take(&mut self.current_str);
                self.tokens.push(CharacterTokens(s));
            }
        }
    }

    impl TokenSink for LinesMatch {
        type Handle = ();

        fn process_token(
            &mut self,
            token: Token,
            line_number: u64,
        ) -> TokenSinkResult<Self::Handle> {
            match token {
                CharacterTokens(b) => {
                    self.current_str.push_slice(&b);
                },

                NullCharacterToken => {
                    self.current_str.push_char('\0');
                },

                ParseError(_) => {
                    panic!("unexpected parse error");
                },

                TagToken(mut t) => {
                    // The spec seems to indicate that one can emit
                    // erroneous end tags with attrs, but the test
                    // cases don't contain them.
                    match t.kind {
                        EndTag => {
                            t.self_closing = false;
                            t.attrs = vec![];
                        },
                        _ => t.attrs.sort_by(|a1, a2| a1.name.cmp(&a2.name)),
                    }
                    self.push(TagToken(t), line_number);
                },

                EOFToken => (),

                _ => self.push(token, line_number),
            }
            TokenSinkResult::Continue
        }
    }

    // Take in tokens, process them, and return vector with line
    // numbers that each token is on
    fn tokenize(input: Vec<StrTendril>, opts: TokenizerOpts) -> Vec<(Token, u64)> {
        let sink = LinesMatch::new();
        let mut tok = Tokenizer::new(sink, opts);
        let mut buffer = BufferQueue::default();
        for chunk in input.into_iter() {
            buffer.push_back(chunk);
            let _ = tok.feed(&mut buffer);
        }
        tok.end();
        tok.sink.lines
    }

    // Create a tag token
    fn create_tag(token: StrTendril, tagkind: TagKind) -> Token {
        let name = LocalName::from(&*token);

        TagToken(Tag {
            kind: tagkind,
            name,
            self_closing: false,
            attrs: vec![],
        })
    }

    #[test]
    fn push_to_None_gives_singleton() {
        let mut s: Option<StrTendril> = None;
        option_push(&mut s, 'x');
        assert_eq!(s, Some("x".to_tendril()));
    }

    #[test]
    fn push_to_empty_appends() {
        let mut s: Option<StrTendril> = Some(StrTendril::new());
        option_push(&mut s, 'x');
        assert_eq!(s, Some("x".to_tendril()));
    }

    #[test]
    fn push_to_nonempty_appends() {
        let mut s: Option<StrTendril> = Some(StrTendril::from_slice("y"));
        option_push(&mut s, 'x');
        assert_eq!(s, Some("yx".to_tendril()));
    }

    #[test]
    fn check_lines() {
        let opts = TokenizerOpts {
            exact_errors: false,
            discard_bom: true,
            profile: false,
            initial_state: None,
            last_start_tag_name: None,
        };
        let vector = vec![
            StrTendril::from("<a>\n"),
            StrTendril::from("<b>\n"),
            StrTendril::from("</b>\n"),
            StrTendril::from("</a>\n"),
        ];
        let expected = vec![
            (create_tag(StrTendril::from("a"), StartTag), 1),
            (create_tag(StrTendril::from("b"), StartTag), 2),
            (create_tag(StrTendril::from("b"), EndTag), 3),
            (create_tag(StrTendril::from("a"), EndTag), 4),
        ];
        let results = tokenize(vector, opts);
        assert_eq!(results, expected);
    }

    #[test]
    fn check_lines_with_new_line() {
        let opts = TokenizerOpts {
            exact_errors: false,
            discard_bom: true,
            profile: false,
            initial_state: None,
            last_start_tag_name: None,
        };
        let vector = vec![
            StrTendril::from("<a>\r\n"),
            StrTendril::from("<b>\r\n"),
            StrTendril::from("</b>\r\n"),
            StrTendril::from("</a>\r\n"),
        ];
        let expected = vec![
            (create_tag(StrTendril::from("a"), StartTag), 1),
            (create_tag(StrTendril::from("b"), StartTag), 2),
            (create_tag(StrTendril::from("b"), EndTag), 3),
            (create_tag(StrTendril::from("a"), EndTag), 4),
        ];
        let results = tokenize(vector, opts);
        assert_eq!(results, expected);
    }
}
