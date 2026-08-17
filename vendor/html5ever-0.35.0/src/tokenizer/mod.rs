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
use markup5ever::{ns, small_char_set, TokenizerResult};
use std::borrow::Cow::{self, Borrowed};
use std::cell::{Cell, RefCell, RefMut};
use std::collections::BTreeMap;
use std::mem;

pub use crate::buffer_queue::{BufferQueue, FromSet, NotFromSet, SetResult};
use crate::macros::{time, unwrap_or_return};
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
    state: Cell<states::State>,

    /// Are we at the end of the file, once buffers have been processed
    /// completely? This affects whether we will wait for lookahead or not.
    at_eof: Cell<bool>,

    /// Tokenizer for character references, if we're tokenizing
    /// one at the moment.
    char_ref_tokenizer: RefCell<Option<Box<CharRefTokenizer>>>,

    /// Current input character.  Just consumed, may reconsume.
    current_char: Cell<char>,

    /// Should we reconsume the current input character?
    reconsume: Cell<bool>,

    /// Did we just consume \r, translating it to \n?  In that case we need
    /// to ignore the next character if it's \n.
    ignore_lf: Cell<bool>,

    /// Discard a U+FEFF BYTE ORDER MARK if we see one?  Only done at the
    /// beginning of the stream.
    discard_bom: Cell<bool>,

    /// Current tag kind.
    current_tag_kind: Cell<TagKind>,

    /// Current tag name.
    current_tag_name: RefCell<StrTendril>,

    /// Current tag is self-closing?
    current_tag_self_closing: Cell<bool>,

    /// Current tag attributes.
    current_tag_attrs: RefCell<Vec<Attribute>>,

    /// Current attribute name.
    current_attr_name: RefCell<StrTendril>,

    /// Current attribute value.
    current_attr_value: RefCell<StrTendril>,

    /// Current comment.
    current_comment: RefCell<StrTendril>,

    /// Current doctype token.
    current_doctype: RefCell<Doctype>,

    /// Last start tag name, for use in checking "appropriate end tag".
    last_start_tag_name: RefCell<Option<LocalName>>,

    /// The "temporary buffer" mentioned in the spec.
    temp_buf: RefCell<StrTendril>,

    /// Record of how many ns we spent in each state, if profiling is enabled.
    state_profile: RefCell<BTreeMap<states::State, u64>>,

    /// Record of how many ns we spent in the token sink.
    time_in_sink: Cell<u64>,

    /// Track current line
    current_line: Cell<u64>,
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
            state: Cell::new(state),
            char_ref_tokenizer: RefCell::new(None),
            at_eof: Cell::new(false),
            current_char: Cell::new('\0'),
            reconsume: Cell::new(false),
            ignore_lf: Cell::new(false),
            discard_bom: Cell::new(discard_bom),
            current_tag_kind: Cell::new(StartTag),
            current_tag_name: RefCell::new(StrTendril::new()),
            current_tag_self_closing: Cell::new(false),
            current_tag_attrs: RefCell::new(vec![]),
            current_attr_name: RefCell::new(StrTendril::new()),
            current_attr_value: RefCell::new(StrTendril::new()),
            current_comment: RefCell::new(StrTendril::new()),
            current_doctype: RefCell::new(Doctype::default()),
            last_start_tag_name: RefCell::new(start_tag_name),
            temp_buf: RefCell::new(StrTendril::new()),
            state_profile: RefCell::new(BTreeMap::new()),
            time_in_sink: Cell::new(0),
            current_line: Cell::new(1),
        }
    }

    /// Feed an input string into the tokenizer.
    pub fn feed(&self, input: &BufferQueue) -> TokenizerResult<Sink::Handle> {
        if input.is_empty() {
            return TokenizerResult::Done;
        }

        if self.discard_bom.get() {
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

    pub fn set_plaintext_state(&self) {
        self.state.set(states::Plaintext);
    }

    fn process_token(&self, token: Token) -> TokenSinkResult<Sink::Handle> {
        if self.opts.profile {
            let (ret, dt) = time!(self.sink.process_token(token, self.current_line.get()));
            self.time_in_sink.set(self.time_in_sink.get() + dt);
            ret
        } else {
            self.sink.process_token(token, self.current_line.get())
        }
    }

    fn process_token_and_continue(&self, token: Token) {
        assert!(matches!(
            self.process_token(token),
            TokenSinkResult::Continue
        ));
    }

    //§ preprocessing-the-input-stream
    // Get the next input character, which might be the character
    // 'c' that we already consumed from the buffers.
    fn get_preprocessed_char(&self, mut c: char, input: &BufferQueue) -> Option<char> {
        if self.ignore_lf.get() {
            self.ignore_lf.set(false);
            if c == '\n' {
                c = input.next()?;
            }
        }

        if c == '\r' {
            self.ignore_lf.set(true);
            c = '\n';
        }

        if c == '\n' {
            self.current_line.set(self.current_line.get() + 1);
        }

        if self.opts.exact_errors
            && match c as u32 {
                0x01..=0x08 | 0x0B | 0x0E..=0x1F | 0x7F..=0x9F | 0xFDD0..=0xFDEF => true,
                n if (n & 0xFFFE) == 0xFFFE => true,
                _ => false,
            }
        {
            let msg = format!("Bad character {c}");
            self.emit_error(Cow::Owned(msg));
        }

        trace!("got character {c}");
        self.current_char.set(c);
        Some(c)
    }

    //§ tokenization
    // Get the next input character, if one is available.
    fn get_char(&self, input: &BufferQueue) -> Option<char> {
        if self.reconsume.get() {
            self.reconsume.set(false);
            Some(self.current_char.get())
        } else {
            input
                .next()
                .and_then(|c| self.get_preprocessed_char(c, input))
        }
    }

    fn pop_except_from(&self, input: &BufferQueue, set: SmallCharSet) -> Option<SetResult> {
        // Bail to the slow path for various corner cases.
        // This means that `FromSet` can contain characters not in the set!
        // It shouldn't matter because the fallback `FromSet` case should
        // always do the same thing as the `NotFromSet` case.
        if self.opts.exact_errors || self.reconsume.get() || self.ignore_lf.get() {
            return self.get_char(input).map(FromSet);
        }

        let d = input.pop_except_from(set);
        trace!("got characters {d:?}");
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
    fn eat(&self, input: &BufferQueue, pat: &str, eq: fn(&u8, &u8) -> bool) -> Option<bool> {
        if self.ignore_lf.get() {
            self.ignore_lf.set(false);
            if self.peek(input) == Some('\n') {
                self.discard_char(input);
            }
        }

        input.push_front(mem::take(&mut self.temp_buf.borrow_mut()));
        match input.eat(pat, eq) {
            None if self.at_eof.get() => Some(false),
            None => {
                while let Some(data) = input.next() {
                    self.temp_buf.borrow_mut().push_char(data);
                }
                None
            },
            Some(matched) => Some(matched),
        }
    }

    /// Run the state machine for as long as we can.
    fn run(&self, input: &BufferQueue) -> TokenizerResult<Sink::Handle> {
        if self.opts.profile {
            loop {
                let state = self.state.get();
                let old_sink = self.time_in_sink.get();
                let (run, mut dt) = time!(self.step(input));
                dt -= (self.time_in_sink.get() - old_sink);
                let new = match self.state_profile.borrow_mut().get_mut(&state) {
                    Some(x) => {
                        *x += dt;
                        false
                    },
                    None => true,
                };
                if new {
                    // do this here because of borrow shenanigans
                    self.state_profile.borrow_mut().insert(state, dt);
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

    #[inline]
    fn bad_char_error(&self) {
        #[cfg(feature = "trace_tokenizer")]
        trace!("  error");

        let msg = if self.opts.exact_errors {
            Cow::from("Bad character")
        } else {
            let c = self.current_char.get();
            let state = self.state.get();
            Cow::from(format!("Saw {c} in state {state:?}"))
        };
        self.emit_error(msg);
    }

    #[inline]
    fn bad_eof_error(&self) {
        #[cfg(feature = "trace_tokenizer")]
        trace!("  error_eof");

        let msg = if self.opts.exact_errors {
            Cow::from("Unexpected EOF")
        } else {
            let state = self.state.get();
            Cow::from(format!("Saw EOF in state {state:?}"))
        };
        self.emit_error(msg);
    }

    fn emit_char(&self, c: char) {
        #[cfg(feature = "trace_tokenizer")]
        trace!("  emit");

        self.process_token_and_continue(match c {
            '\0' => NullCharacterToken,
            _ => CharacterTokens(StrTendril::from_char(c)),
        });
    }

    // The string must not contain '\0'!
    fn emit_chars(&self, b: StrTendril) {
        self.process_token_and_continue(CharacterTokens(b));
    }

    fn emit_current_tag(&self) -> ProcessResult<Sink::Handle> {
        self.finish_attribute();

        let name = LocalName::from(&**self.current_tag_name.borrow());
        self.current_tag_name.borrow_mut().clear();

        match self.current_tag_kind.get() {
            StartTag => {
                *self.last_start_tag_name.borrow_mut() = Some(name.clone());
            },
            EndTag => {
                if !self.current_tag_attrs.borrow().is_empty() {
                    self.emit_error(Borrowed("Attributes on an end tag"));
                }
                if self.current_tag_self_closing.get() {
                    self.emit_error(Borrowed("Self-closing end tag"));
                }
            },
        }

        let token = TagToken(Tag {
            kind: self.current_tag_kind.get(),
            name,
            self_closing: self.current_tag_self_closing.get(),
            attrs: std::mem::take(&mut self.current_tag_attrs.borrow_mut()),
        });

        match self.process_token(token) {
            TokenSinkResult::Continue => ProcessResult::Continue,
            TokenSinkResult::Plaintext => {
                self.state.set(states::Plaintext);
                ProcessResult::Continue
            },
            TokenSinkResult::Script(node) => {
                self.state.set(states::Data);
                ProcessResult::Script(node)
            },
            TokenSinkResult::RawData(kind) => {
                self.state.set(states::RawData(kind));
                ProcessResult::Continue
            },
        }
    }

    fn emit_temp_buf(&self) {
        #[cfg(feature = "trace_tokenizer")]
        trace!("  emit_temp");

        // FIXME: Make sure that clearing on emit is spec-compatible.
        let buf = mem::take(&mut *self.temp_buf.borrow_mut());
        self.emit_chars(buf);
    }

    fn clear_temp_buf(&self) {
        // Do this without a new allocation.
        self.temp_buf.borrow_mut().clear();
    }

    fn emit_current_comment(&self) {
        let comment = mem::take(&mut *self.current_comment.borrow_mut());
        self.process_token_and_continue(CommentToken(comment));
    }

    fn discard_tag(&self) {
        self.current_tag_name.borrow_mut().clear();
        self.current_tag_self_closing.set(false);
        *self.current_tag_attrs.borrow_mut() = vec![];
    }

    fn create_tag(&self, kind: TagKind, c: char) {
        self.discard_tag();
        self.current_tag_name.borrow_mut().push_char(c);
        self.current_tag_kind.set(kind);
    }

    fn have_appropriate_end_tag(&self) -> bool {
        match self.last_start_tag_name.borrow().as_ref() {
            Some(last) => {
                (self.current_tag_kind.get() == EndTag)
                    && (**self.current_tag_name.borrow() == **last)
            },
            None => false,
        }
    }

    fn create_attribute(&self, c: char) {
        self.finish_attribute();

        self.current_attr_name.borrow_mut().push_char(c);
    }

    fn finish_attribute(&self) {
        if self.current_attr_name.borrow().is_empty() {
            return;
        }

        // Check for a duplicate attribute.
        // FIXME: the spec says we should error as soon as the name is finished.
        let dup = {
            let name = &*self.current_attr_name.borrow();
            self.current_tag_attrs
                .borrow()
                .iter()
                .any(|a| *a.name.local == **name)
        };

        if dup {
            self.emit_error(Borrowed("Duplicate attribute"));
            self.current_attr_name.borrow_mut().clear();
            self.current_attr_value.borrow_mut().clear();
        } else {
            let name = LocalName::from(&**self.current_attr_name.borrow());
            self.current_attr_name.borrow_mut().clear();
            self.current_tag_attrs.borrow_mut().push(Attribute {
                // The tree builder will adjust the namespace if necessary.
                // This only happens in foreign elements.
                name: QualName::new(None, ns!(), name),
                value: mem::take(&mut self.current_attr_value.borrow_mut()),
            });
        }
    }

    fn emit_current_doctype(&self) {
        let doctype = self.current_doctype.take();
        self.process_token_and_continue(DoctypeToken(doctype));
    }

    fn doctype_id(&self, kind: DoctypeIdKind) -> RefMut<'_, Option<StrTendril>> {
        let current_doctype = self.current_doctype.borrow_mut();
        match kind {
            Public => RefMut::map(current_doctype, |d| &mut d.public_id),
            System => RefMut::map(current_doctype, |d| &mut d.system_id),
        }
    }

    fn clear_doctype_id(&self, kind: DoctypeIdKind) {
        let mut id = self.doctype_id(kind);
        match *id {
            Some(ref mut s) => s.clear(),
            None => *id = Some(StrTendril::new()),
        }
    }

    fn consume_char_ref(&self) {
        *self.char_ref_tokenizer.borrow_mut() = Some(Box::new(CharRefTokenizer::new(matches!(
            self.state.get(),
            states::AttributeValue(_)
        ))));
    }

    fn emit_eof(&self) {
        self.process_token_and_continue(EOFToken);
    }

    fn peek(&self, input: &BufferQueue) -> Option<char> {
        if self.reconsume.get() {
            Some(self.current_char.get())
        } else {
            input.peek()
        }
    }

    fn discard_char(&self, input: &BufferQueue) {
        // peek() deals in un-processed characters (no newline normalization), while get_char()
        // does.
        //
        // since discard_char is supposed to be used in combination with peek(), discard_char must
        // discard a single raw input character, not a normalized newline.
        if self.reconsume.get() {
            self.reconsume.set(false);
        } else {
            input.next();
        }
    }

    fn emit_error(&self, error: Cow<'static, str>) {
        self.process_token_and_continue(ParseError(error));
    }
}
//§ END

// Shorthand for common state machine behaviors.
macro_rules! shorthand (
    ( $me:ident : create_tag $kind:ident $c:expr   ) => ( $me.create_tag($kind, $c)                           );
    ( $me:ident : push_tag $c:expr                 ) => ( $me.current_tag_name.borrow_mut().push_char($c)     );
    ( $me:ident : discard_tag                      ) => ( $me.discard_tag()                                   );
    ( $me:ident : discard_char $input:expr         ) => ( $me.discard_char($input)                            );
    ( $me:ident : push_temp $c:expr                ) => ( $me.temp_buf.borrow_mut().push_char($c)             );
    ( $me:ident : clear_temp                       ) => ( $me.clear_temp_buf()                                );
    ( $me:ident : create_attr $c:expr              ) => ( $me.create_attribute($c)                            );
    ( $me:ident : push_name $c:expr                ) => ( $me.current_attr_name.borrow_mut().push_char($c)    );
    ( $me:ident : push_value $c:expr               ) => ( $me.current_attr_value.borrow_mut().push_char($c)   );
    ( $me:ident : append_value $c:expr             ) => ( $me.current_attr_value.borrow_mut().push_tendril($c));
    ( $me:ident : push_comment $c:expr             ) => ( $me.current_comment.borrow_mut().push_char($c)      );
    ( $me:ident : append_comment $c:expr           ) => ( $me.current_comment.borrow_mut().push_slice($c)     );
    ( $me:ident : emit_comment                     ) => ( $me.emit_current_comment()                          );
    ( $me:ident : clear_comment                    ) => ( $me.current_comment.borrow_mut().clear()            );
    ( $me:ident : create_doctype                   ) => ( *$me.current_doctype.borrow_mut() = Doctype::default() );
    ( $me:ident : push_doctype_name $c:expr        ) => ( option_push(&mut $me.current_doctype.borrow_mut().name, $c) );
    ( $me:ident : push_doctype_id $k:ident $c:expr ) => ( option_push(&mut $me.doctype_id($k), $c)            );
    ( $me:ident : clear_doctype_id $k:ident        ) => ( $me.clear_doctype_id($k)                            );
    ( $me:ident : force_quirks                     ) => ( $me.current_doctype.borrow_mut().force_quirks = true);
    ( $me:ident : emit_doctype                     ) => ( $me.emit_current_doctype()                          );
);

// Tracing of tokenizer actions.  This adds significant bloat and compile time,
// so it's behind a cfg flag.
#[cfg(feature = "trace_tokenizer")]
macro_rules! sh_trace ( ( $me:ident : $($cmds:tt)* ) => ({
    trace!("  {:?}", stringify!($($cmds)*));
    shorthand!($me : $($cmds)*);
}));

#[cfg(not(feature = "trace_tokenizer"))]
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

    ( $me:ident : to $s:ident                    ) => ({ $me.state.set(states::$s); return ProcessResult::Continue;           });
    ( $me:ident : to $s:ident $k1:expr           ) => ({ $me.state.set(states::$s($k1)); return ProcessResult::Continue;      });
    ( $me:ident : to $s:ident $k1:ident $k2:expr ) => ({ $me.state.set(states::$s($k1($k2))); return ProcessResult::Continue; });

    ( $me:ident : reconsume $s:ident                    ) => ({ $me.reconsume.set(true); go!($me: to $s);         });
    ( $me:ident : reconsume $s:ident $k1:expr           ) => ({ $me.reconsume.set(true); go!($me: to $s $k1);     });
    ( $me:ident : reconsume $s:ident $k1:ident $k2:expr ) => ({ $me.reconsume.set(true); go!($me: to $s $k1 $k2); });

    ( $me:ident : consume_char_ref             ) => ({ $me.consume_char_ref(); return ProcessResult::Continue;         });

    // We have a default next state after emitting a tag, but the sink can override.
    ( $me:ident : emit_tag $s:ident ) => ({
        $me.state.set(states::$s);
        return $me.emit_current_tag();
    });

    ( $me:ident : eof ) => ({ $me.emit_eof(); return ProcessResult::Suspend; });

    // If nothing else matched, it's a single command
    ( $me:ident : $($cmd:tt)+ ) => ( sh_trace!($me: $($cmd)+) );

    // or nothing.
    ( $me:ident : ) => (());
);

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
    fn step(&self, input: &BufferQueue) -> ProcessResult<Sink::Handle> {
        if self.char_ref_tokenizer.borrow().is_some() {
            return self.step_char_ref_tokenizer(input);
        }

        trace!("processing in state {:?}", self.state);
        match self.state.get() {
            //§ data-state
            states::Data => loop {
                let set = small_char_set!('\r' '\0' '&' '<' '\n');

                #[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
                let set_result = if !(self.opts.exact_errors
                    || self.reconsume.get()
                    || self.ignore_lf.get())
                    && Self::is_supported_simd_feature_detected()
                {
                    let front_buffer = input.peek_front_chunk_mut();
                    let Some(mut front_buffer) = front_buffer else {
                        return ProcessResult::Suspend;
                    };

                    // Special case: The fast path is not worth taking if the first character is already in the set,
                    // which is fairly common
                    let first_char = front_buffer
                        .chars()
                        .next()
                        .expect("Input buffers are never empty");

                    if matches!(first_char, '\r' | '\0' | '&' | '<' | '\n') {
                        drop(front_buffer);
                        self.pop_except_from(input, set)
                    } else {
                        // SAFETY:
                        // This CPU is guaranteed to support SIMD due to the is_supported_simd_feature_detected check above
                        let result = unsafe { self.data_state_simd_fast_path(&mut front_buffer) };

                        if front_buffer.is_empty() {
                            drop(front_buffer);
                            input.pop_front();
                        }

                        result
                    }
                } else {
                    self.pop_except_from(input, set)
                };

                #[cfg(not(any(
                    target_arch = "x86",
                    target_arch = "x86_64",
                    target_arch = "aarch64"
                )))]
                let set_result = self.pop_except_from(input, set);

                let Some(set_result) = set_result else {
                    return ProcessResult::Suspend;
                };
                match set_result {
                    FromSet('\0') => {
                        self.bad_char_error();
                        self.emit_char('\0');
                    },
                    FromSet('&') => go!(self: consume_char_ref),
                    FromSet('<') => go!(self: to TagOpen),
                    FromSet(c) => {
                        self.emit_char(c);
                    },
                    NotFromSet(b) => self.emit_chars(b),
                }
            },

            //§ rcdata-state
            states::RawData(Rcdata) => loop {
                match pop_except_from!(self, input, small_char_set!('\r' '\0' '&' '<' '\n')) {
                    FromSet('\0') => {
                        self.bad_char_error();
                        self.emit_char('\u{fffd}');
                    },
                    FromSet('&') => go!(self: consume_char_ref),
                    FromSet('<') => go!(self: to RawLessThanSign Rcdata),
                    FromSet(c) => self.emit_char(c),
                    NotFromSet(b) => self.emit_chars(b),
                }
            },

            //§ rawtext-state
            states::RawData(Rawtext) => loop {
                match pop_except_from!(self, input, small_char_set!('\r' '\0' '<' '\n')) {
                    FromSet('\0') => {
                        self.bad_char_error();
                        self.emit_char('\u{fffd}');
                    },
                    FromSet('<') => go!(self: to RawLessThanSign Rawtext),
                    FromSet(c) => self.emit_char(c),
                    NotFromSet(b) => self.emit_chars(b),
                }
            },

            //§ script-data-state
            states::RawData(ScriptData) => loop {
                match pop_except_from!(self, input, small_char_set!('\r' '\0' '<' '\n')) {
                    FromSet('\0') => {
                        self.bad_char_error();
                        self.emit_char('\u{fffd}');
                    },
                    FromSet('<') => go!(self: to RawLessThanSign ScriptData),
                    FromSet(c) => self.emit_char(c),
                    NotFromSet(b) => self.emit_chars(b),
                }
            },

            //§ script-data-escaped-state
            states::RawData(ScriptDataEscaped(Escaped)) => loop {
                match pop_except_from!(self, input, small_char_set!('\r' '\0' '-' '<' '\n')) {
                    FromSet('\0') => {
                        self.bad_char_error();
                        self.emit_char('\u{fffd}');
                    },
                    FromSet('-') => {
                        self.emit_char('-');
                        go!(self: to ScriptDataEscapedDash Escaped);
                    },
                    FromSet('<') => go!(self: to RawLessThanSign ScriptDataEscaped Escaped),
                    FromSet(c) => self.emit_char(c),
                    NotFromSet(b) => self.emit_chars(b),
                }
            },

            //§ script-data-double-escaped-state
            states::RawData(ScriptDataEscaped(DoubleEscaped)) => loop {
                match pop_except_from!(self, input, small_char_set!('\r' '\0' '-' '<' '\n')) {
                    FromSet('\0') => {
                        self.bad_char_error();
                        self.emit_char('\u{fffd}');
                    },
                    FromSet('-') => {
                        self.emit_char('-');
                        go!(self: to ScriptDataEscapedDash DoubleEscaped);
                    },
                    FromSet('<') => {
                        self.emit_char('<');
                        go!(self: to RawLessThanSign ScriptDataEscaped DoubleEscaped)
                    },
                    FromSet(c) => self.emit_char(c),
                    NotFromSet(b) => self.emit_chars(b),
                }
            },

            //§ plaintext-state
            states::Plaintext => loop {
                match pop_except_from!(self, input, small_char_set!('\r' '\0' '\n')) {
                    FromSet('\0') => {
                        self.bad_char_error();
                        self.emit_char('\u{fffd}');
                    },
                    FromSet(c) => self.emit_char(c),
                    NotFromSet(b) => self.emit_chars(b),
                }
            },

            //§ tag-open-state
            states::TagOpen => loop {
                match get_char!(self, input) {
                    '!' => go!(self: to MarkupDeclarationOpen),
                    '/' => go!(self: to EndTagOpen),
                    '?' => {
                        self.bad_char_error();
                        go!(self: clear_comment; reconsume BogusComment)
                    },
                    c => match lower_ascii_letter(c) {
                        Some(cl) => go!(self: create_tag StartTag cl; to TagName),
                        None => {
                            self.bad_char_error();
                            self.emit_char('<');
                            go!(self: reconsume Data)
                        },
                    },
                }
            },

            //§ end-tag-open-state
            states::EndTagOpen => loop {
                match get_char!(self, input) {
                    '>' => {
                        self.bad_char_error();
                        go!(self: to Data)
                    },
                    c => match lower_ascii_letter(c) {
                        Some(cl) => go!(self: create_tag EndTag cl; to TagName),
                        None => {
                            self.bad_char_error();
                            go!(self: clear_comment; reconsume BogusComment)
                        },
                    },
                }
            },

            //§ tag-name-state
            states::TagName => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => go!(self: to BeforeAttributeName),
                    '/' => go!(self: to SelfClosingStartTag),
                    '>' => go!(self: emit_tag Data),
                    '\0' => {
                        self.bad_char_error();
                        go!(self: push_tag '\u{fffd}')
                    },
                    c => go!(self: push_tag (c.to_ascii_lowercase())),
                }
            },

            //§ script-data-escaped-less-than-sign-state
            states::RawLessThanSign(ScriptDataEscaped(Escaped)) => loop {
                match get_char!(self, input) {
                    '/' => go!(self: clear_temp; to RawEndTagOpen ScriptDataEscaped Escaped),
                    c => match lower_ascii_letter(c) {
                        Some(cl) => {
                            go!(self: clear_temp; push_temp cl);
                            self.emit_char('<');
                            self.emit_char(c);
                            go!(self: to ScriptDataEscapeStart DoubleEscaped);
                        },
                        None => {
                            self.emit_char('<');
                            go!(self: reconsume RawData ScriptDataEscaped Escaped);
                        },
                    },
                }
            },

            //§ script-data-double-escaped-less-than-sign-state
            states::RawLessThanSign(ScriptDataEscaped(DoubleEscaped)) => loop {
                match get_char!(self, input) {
                    '/' => {
                        go!(self: clear_temp);
                        self.emit_char('/');
                        go!(self: to ScriptDataDoubleEscapeEnd);
                    },
                    _ => go!(self: reconsume RawData ScriptDataEscaped DoubleEscaped),
                }
            },

            //§ rcdata-less-than-sign-state rawtext-less-than-sign-state script-data-less-than-sign-state
            // otherwise
            states::RawLessThanSign(kind) => loop {
                match get_char!(self, input) {
                    '/' => go!(self: clear_temp; to RawEndTagOpen kind),
                    '!' if kind == ScriptData => {
                        self.emit_char('<');
                        self.emit_char('!');
                        go!(self: to ScriptDataEscapeStart Escaped);
                    },
                    _ => {
                        self.emit_char('<');
                        go!(self: reconsume RawData kind);
                    },
                }
            },

            //§ rcdata-end-tag-open-state rawtext-end-tag-open-state script-data-end-tag-open-state script-data-escaped-end-tag-open-state
            states::RawEndTagOpen(kind) => loop {
                let c = get_char!(self, input);
                match lower_ascii_letter(c) {
                    Some(cl) => go!(self: create_tag EndTag cl; push_temp c; to RawEndTagName kind),
                    None => {
                        self.emit_char('<');
                        self.emit_char('/');
                        go!(self: reconsume RawData kind);
                    },
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
                        go!(self: discard_tag);
                        self.emit_char('<');
                        self.emit_char('/');
                        self.emit_temp_buf();
                        go!(self: reconsume RawData kind);
                    },
                }
            },

            //§ script-data-double-escape-start-state
            states::ScriptDataEscapeStart(DoubleEscaped) => loop {
                let c = get_char!(self, input);
                match c {
                    '\t' | '\n' | '\x0C' | ' ' | '/' | '>' => {
                        let esc = if &**self.temp_buf.borrow() == "script" {
                            DoubleEscaped
                        } else {
                            Escaped
                        };
                        self.emit_char(c);
                        go!(self: to RawData ScriptDataEscaped esc);
                    },
                    _ => match lower_ascii_letter(c) {
                        Some(cl) => {
                            go!(self: push_temp cl);
                            self.emit_char(c);
                        },
                        None => go!(self: reconsume RawData ScriptDataEscaped Escaped),
                    },
                }
            },

            //§ script-data-escape-start-state
            states::ScriptDataEscapeStart(Escaped) => loop {
                match get_char!(self, input) {
                    '-' => {
                        self.emit_char('-');
                        go!(self: to ScriptDataEscapeStartDash);
                    },
                    _ => go!(self: reconsume RawData ScriptData),
                }
            },

            //§ script-data-escape-start-dash-state
            states::ScriptDataEscapeStartDash => loop {
                match get_char!(self, input) {
                    '-' => {
                        self.emit_char('-');
                        go!(self: to ScriptDataEscapedDashDash Escaped);
                    },
                    _ => go!(self: reconsume RawData ScriptData),
                }
            },

            //§ script-data-escaped-dash-state script-data-double-escaped-dash-state
            states::ScriptDataEscapedDash(kind) => loop {
                match get_char!(self, input) {
                    '-' => {
                        self.emit_char('-');
                        go!(self: to ScriptDataEscapedDashDash kind);
                    },
                    '<' => {
                        if kind == DoubleEscaped {
                            self.emit_char('<');
                        }
                        go!(self: to RawLessThanSign ScriptDataEscaped kind);
                    },
                    '\0' => {
                        self.bad_char_error();
                        self.emit_char('\u{fffd}');
                        go!(self: to RawData ScriptDataEscaped kind)
                    },
                    c => {
                        self.emit_char(c);
                        go!(self: to RawData ScriptDataEscaped kind);
                    },
                }
            },

            //§ script-data-escaped-dash-dash-state script-data-double-escaped-dash-dash-state
            states::ScriptDataEscapedDashDash(kind) => loop {
                match get_char!(self, input) {
                    '-' => {
                        self.emit_char('-');
                    },
                    '<' => {
                        if kind == DoubleEscaped {
                            self.emit_char('<');
                        }
                        go!(self: to RawLessThanSign ScriptDataEscaped kind);
                    },
                    '>' => {
                        self.emit_char('>');
                        go!(self: to RawData ScriptData);
                    },
                    '\0' => {
                        self.bad_char_error();
                        self.emit_char('\u{fffd}');
                        go!(self: to RawData ScriptDataEscaped kind)
                    },
                    c => {
                        self.emit_char(c);
                        go!(self: to RawData ScriptDataEscaped kind);
                    },
                }
            },

            //§ script-data-double-escape-end-state
            states::ScriptDataDoubleEscapeEnd => loop {
                let c = get_char!(self, input);
                match c {
                    '\t' | '\n' | '\x0C' | ' ' | '/' | '>' => {
                        let esc = if &**self.temp_buf.borrow() == "script" {
                            Escaped
                        } else {
                            DoubleEscaped
                        };
                        self.emit_char(c);
                        go!(self: to RawData ScriptDataEscaped esc);
                    },
                    _ => match lower_ascii_letter(c) {
                        Some(cl) => {
                            go!(self: push_temp cl);
                            self.emit_char(c);
                        },
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
                    '\0' => {
                        self.bad_char_error();
                        go!(self: create_attr '\u{fffd}'; to AttributeName)
                    },
                    c => match lower_ascii_letter(c) {
                        Some(cl) => go!(self: create_attr cl; to AttributeName),
                        None => {
                            if matches!(c, '"' | '\'' | '<' | '=') {
                                self.bad_char_error();
                            }

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
                    '\0' => {
                        self.bad_char_error();
                        go!(self: push_name '\u{fffd}')
                    },
                    c => match lower_ascii_letter(c) {
                        Some(cl) => go!(self: push_name cl),
                        None => {
                            if matches!(c, '"' | '\'' | '<') {
                                self.bad_char_error();
                            }
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
                    '\0' => {
                        self.bad_char_error();
                        go!(self: create_attr '\u{fffd}'; to AttributeName)
                    },
                    c => match lower_ascii_letter(c) {
                        Some(cl) => go!(self: create_attr cl; to AttributeName),
                        None => {
                            if matches!(c, '"' | '\'' | '<') {
                                self.bad_char_error();
                            }

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
                    '>' => {
                        go!(self: discard_char input);
                        self.bad_char_error();
                        go!(self: emit_tag Data)
                    },
                    _ => go!(self: to AttributeValue Unquoted),
                }
            },

            //§ attribute-value-(double-quoted)-state
            states::AttributeValue(DoubleQuoted) => loop {
                match pop_except_from!(self, input, small_char_set!('\r' '"' '&' '\0' '\n')) {
                    FromSet('"') => go!(self: to AfterAttributeValueQuoted),
                    FromSet('&') => go!(self: consume_char_ref),
                    FromSet('\0') => {
                        self.bad_char_error();
                        go!(self: push_value '\u{fffd}')
                    },
                    FromSet(c) => go!(self: push_value c),
                    NotFromSet(ref b) => go!(self: append_value b),
                }
            },

            //§ attribute-value-(single-quoted)-state
            states::AttributeValue(SingleQuoted) => loop {
                match pop_except_from!(self, input, small_char_set!('\r' '\'' '&' '\0' '\n')) {
                    FromSet('\'') => go!(self: to AfterAttributeValueQuoted),
                    FromSet('&') => go!(self: consume_char_ref),
                    FromSet('\0') => {
                        self.bad_char_error();
                        go!(self: push_value '\u{fffd}')
                    },
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
                    FromSet('\0') => {
                        self.bad_char_error();
                        go!(self: push_value '\u{fffd}')
                    },
                    FromSet(c) => {
                        if matches!(c, '"' | '\'' | '<' | '=' | '`') {
                            self.bad_char_error();
                        }
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
                    _ => {
                        self.bad_char_error();
                        go!(self: reconsume BeforeAttributeName)
                    },
                }
            },

            //§ self-closing-start-tag-state
            states::SelfClosingStartTag => loop {
                match get_char!(self, input) {
                    '>' => {
                        self.current_tag_self_closing.set(true);
                        go!(self: emit_tag Data);
                    },
                    _ => {
                        self.bad_char_error();
                        go!(self: reconsume BeforeAttributeName)
                    },
                }
            },

            //§ comment-start-state
            states::CommentStart => loop {
                match get_char!(self, input) {
                    '-' => go!(self: to CommentStartDash),
                    '\0' => {
                        self.bad_char_error();
                        go!(self: push_comment '\u{fffd}'; to Comment)
                    },
                    '>' => {
                        self.bad_char_error();
                        go!(self: emit_comment; to Data)
                    },
                    c => go!(self: push_comment c; to Comment),
                }
            },

            //§ comment-start-dash-state
            states::CommentStartDash => loop {
                match get_char!(self, input) {
                    '-' => go!(self: to CommentEnd),
                    '\0' => {
                        self.bad_char_error();
                        go!(self: append_comment "-\u{fffd}"; to Comment)
                    },
                    '>' => {
                        self.bad_char_error();
                        go!(self: emit_comment; to Data)
                    },
                    c => go!(self: push_comment '-'; push_comment c; to Comment),
                }
            },

            //§ comment-state
            states::Comment => loop {
                match get_char!(self, input) {
                    c @ '<' => go!(self: push_comment c; to CommentLessThanSign),
                    '-' => go!(self: to CommentEndDash),
                    '\0' => {
                        self.bad_char_error();
                        go!(self: push_comment '\u{fffd}')
                    },
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
                    _ => {
                        self.bad_char_error();
                        go!(self: reconsume CommentEnd)
                    },
                }
            },

            //§ comment-end-dash-state
            states::CommentEndDash => loop {
                match get_char!(self, input) {
                    '-' => go!(self: to CommentEnd),
                    '\0' => {
                        self.bad_char_error();
                        go!(self: append_comment "-\u{fffd}"; to Comment)
                    },
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
                    '>' => {
                        self.bad_char_error();
                        go!(self: emit_comment; to Data)
                    },
                    '\0' => {
                        self.bad_char_error();
                        go!(self: append_comment "--!\u{fffd}"; to Comment)
                    },
                    c => go!(self: append_comment "--!"; push_comment c; to Comment),
                }
            },

            //§ doctype-state
            states::Doctype => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => go!(self: to BeforeDoctypeName),
                    '>' => go!(self: reconsume BeforeDoctypeName),
                    _ => {
                        self.bad_char_error();
                        go!(self: reconsume BeforeDoctypeName)
                    },
                }
            },

            //§ before-doctype-name-state
            states::BeforeDoctypeName => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => (),
                    '\0' => {
                        self.bad_char_error();
                        go!(self: create_doctype; push_doctype_name '\u{fffd}'; to DoctypeName)
                    },
                    '>' => {
                        self.bad_char_error();
                        go!(self: create_doctype; force_quirks; emit_doctype; to Data)
                    },
                    c => go!(self: create_doctype; push_doctype_name (c.to_ascii_lowercase());
                                  to DoctypeName),
                }
            },

            //§ doctype-name-state
            states::DoctypeName => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => go!(self: clear_temp; to AfterDoctypeName),
                    '>' => go!(self: emit_doctype; to Data),
                    '\0' => {
                        self.bad_char_error();
                        go!(self: push_doctype_name '\u{fffd}')
                    },
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
                        _ => {
                            self.bad_char_error();
                            go!(self: force_quirks; reconsume BogusDoctype)
                        },
                    }
                }
            },

            //§ after-doctype-public-keyword-state after-doctype-system-keyword-state
            states::AfterDoctypeKeyword(kind) => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => go!(self: to BeforeDoctypeIdentifier kind),
                    '"' => {
                        self.bad_char_error();
                        go!(self: clear_doctype_id kind; to DoctypeIdentifierDoubleQuoted kind)
                    },
                    '\'' => {
                        self.bad_char_error();
                        go!(self: clear_doctype_id kind; to DoctypeIdentifierSingleQuoted kind)
                    },
                    '>' => {
                        self.bad_char_error();
                        go!(self: force_quirks; emit_doctype; to Data)
                    },
                    _ => {
                        self.bad_char_error();
                        go!(self: force_quirks; reconsume BogusDoctype)
                    },
                }
            },

            //§ before-doctype-public-identifier-state before-doctype-system-identifier-state
            states::BeforeDoctypeIdentifier(kind) => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => (),
                    '"' => go!(self: clear_doctype_id kind; to DoctypeIdentifierDoubleQuoted kind),
                    '\'' => go!(self: clear_doctype_id kind; to DoctypeIdentifierSingleQuoted kind),
                    '>' => {
                        self.bad_char_error();
                        go!(self: force_quirks; emit_doctype; to Data)
                    },
                    _ => {
                        self.bad_char_error();
                        go!(self: force_quirks; reconsume BogusDoctype)
                    },
                }
            },

            //§ doctype-public-identifier-(double-quoted)-state doctype-system-identifier-(double-quoted)-state
            states::DoctypeIdentifierDoubleQuoted(kind) => loop {
                match get_char!(self, input) {
                    '"' => go!(self: to AfterDoctypeIdentifier kind),
                    '\0' => {
                        self.bad_char_error();
                        go!(self: push_doctype_id kind '\u{fffd}')
                    },
                    '>' => {
                        self.bad_char_error();
                        go!(self: force_quirks; emit_doctype; to Data)
                    },
                    c => go!(self: push_doctype_id kind c),
                }
            },

            //§ doctype-public-identifier-(single-quoted)-state doctype-system-identifier-(single-quoted)-state
            states::DoctypeIdentifierSingleQuoted(kind) => loop {
                match get_char!(self, input) {
                    '\'' => go!(self: to AfterDoctypeIdentifier kind),
                    '\0' => {
                        self.bad_char_error();
                        go!(self: push_doctype_id kind '\u{fffd}')
                    },
                    '>' => {
                        self.bad_char_error();
                        go!(self: force_quirks; emit_doctype; to Data)
                    },
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
                        self.bad_char_error();
                        go!(self: clear_doctype_id System; to DoctypeIdentifierDoubleQuoted System)
                    },
                    '\'' => {
                        self.bad_char_error();
                        go!(self: clear_doctype_id System; to DoctypeIdentifierSingleQuoted System)
                    },
                    _ => {
                        self.bad_char_error();
                        go!(self: force_quirks; reconsume BogusDoctype)
                    },
                }
            },

            //§ after-doctype-system-identifier-state
            states::AfterDoctypeIdentifier(System) => loop {
                match get_char!(self, input) {
                    '\t' | '\n' | '\x0C' | ' ' => (),
                    '>' => go!(self: emit_doctype; to Data),
                    _ => {
                        self.bad_char_error();
                        go!(self: reconsume BogusDoctype)
                    },
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
                    _ => {
                        self.bad_char_error();
                        go!(self: force_quirks; reconsume BogusDoctype)
                    },
                }
            },

            //§ bogus-doctype-state
            states::BogusDoctype => loop {
                match get_char!(self, input) {
                    '>' => go!(self: emit_doctype; to Data),
                    '\0' => {
                        self.bad_char_error();
                    },
                    _ => (),
                }
            },

            //§ bogus-comment-state
            states::BogusComment => loop {
                match get_char!(self, input) {
                    '>' => go!(self: emit_comment; to Data),
                    '\0' => {
                        self.bad_char_error();
                        go!(self: push_comment '\u{fffd}')
                    },
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
                    self.bad_char_error();
                    go!(self: clear_comment; to BogusComment);
                }
            },

            //§ cdata-section-state
            states::CdataSection => loop {
                match get_char!(self, input) {
                    ']' => go!(self: to CdataSectionBracket),
                    '\0' => {
                        self.emit_temp_buf();
                        self.emit_char('\0');
                    },
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
                    '>' => {
                        self.emit_temp_buf();
                        go!(self: to Data);
                    },
                    _ => go!(self: push_temp ']'; push_temp ']'; reconsume CdataSection),
                }
            },
            //§ END
        }
    }

    fn step_char_ref_tokenizer(&self, input: &BufferQueue) -> ProcessResult<Sink::Handle> {
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

        *self.char_ref_tokenizer.borrow_mut() = Some(tok);
        progress
    }

    fn process_char_ref(&self, char_ref: CharRef) {
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
            match self.state.get() {
                states::Data | states::RawData(states::Rcdata) => self.emit_char(c),

                states::AttributeValue(_) => go!(self: push_value c),

                _ => panic!(
                    "state {:?} should not be reachable in process_char_ref",
                    self.state.get()
                ),
            }
        }
    }

    /// Indicate that we have reached the end of the input.
    pub fn end(&self) {
        // Handle EOF in the char ref sub-tokenizer, if there is one.
        // Do this first because it might un-consume stuff.
        let input = BufferQueue::default();
        match self.char_ref_tokenizer.take() {
            None => (),
            Some(mut tok) => {
                tok.end_of_file(self, &input);
                self.process_char_ref(tok.get_result());
            },
        }

        // Process all remaining buffered input.
        // If we're waiting for lookahead, we're not gonna get it.
        self.at_eof.set(true);
        assert!(matches!(self.run(&input), TokenizerResult::Done));
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
        let mut results: Vec<(states::State, u64)> = self
            .state_profile
            .borrow()
            .iter()
            .map(|(s, t)| (*s, *t))
            .collect();
        results.sort_by(|&(_, x), &(_, y)| y.cmp(&x));

        let total: u64 = results
            .iter()
            .map(|&(_, t)| t)
            .fold(0, ::std::ops::Add::add);
        println!("\nTokenizer profile, in nanoseconds");
        println!(
            "\n{:12}         total in token sink",
            self.time_in_sink.get()
        );
        println!("\n{total:12}         total in tokenizer");

        for (k, v) in results.into_iter() {
            let pct = 100.0 * (v as f64) / (total as f64);
            println!("{v:12}  {pct:4.1}%  {k:?}");
        }
    }

    fn eof_step(&self) -> ProcessResult<Sink::Handle> {
        debug!("processing EOF in state {:?}", self.state.get());
        match self.state.get() {
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
            | states::ScriptDataEscapedDashDash(_) => {
                self.bad_eof_error();
                go!(self: to Data)
            },

            states::BeforeAttributeValue => go!(self: reconsume AttributeValue Unquoted),

            states::TagOpen => {
                self.bad_eof_error();
                self.emit_char('<');
                go!(self: to Data);
            },

            states::EndTagOpen => {
                self.bad_eof_error();
                self.emit_char('<');
                self.emit_char('/');
                go!(self: to Data);
            },

            states::RawLessThanSign(ScriptDataEscaped(DoubleEscaped)) => {
                go!(self: to RawData ScriptDataEscaped DoubleEscaped)
            },

            states::RawLessThanSign(kind) => {
                self.emit_char('<');
                go!(self: to RawData kind);
            },

            states::RawEndTagOpen(kind) => {
                self.emit_char('<');
                self.emit_char('/');
                go!(self: to RawData kind);
            },

            states::RawEndTagName(kind) => {
                self.emit_char('<');
                self.emit_char('/');
                self.emit_temp_buf();
                go!(self: to RawData kind)
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
            | states::CommentEndBang => {
                self.bad_eof_error();
                go!(self: emit_comment; to Data)
            },

            states::CommentLessThanSign | states::CommentLessThanSignBang => {
                go!(self: reconsume Comment)
            },

            states::CommentLessThanSignBangDash => go!(self: reconsume CommentEndDash),

            states::CommentLessThanSignBangDashDash => go!(self: reconsume CommentEnd),

            states::Doctype | states::BeforeDoctypeName => {
                self.bad_eof_error();
                go!(self: create_doctype; force_quirks; emit_doctype; to Data)
            },

            states::DoctypeName
            | states::AfterDoctypeName
            | states::AfterDoctypeKeyword(_)
            | states::BeforeDoctypeIdentifier(_)
            | states::DoctypeIdentifierDoubleQuoted(_)
            | states::DoctypeIdentifierSingleQuoted(_)
            | states::AfterDoctypeIdentifier(_)
            | states::BetweenDoctypePublicAndSystemIdentifiers => {
                self.bad_eof_error();
                go!(self: force_quirks; emit_doctype; to Data)
            },

            states::BogusDoctype => go!(self: emit_doctype; to Data),

            states::BogusComment => go!(self: emit_comment; to Data),

            states::MarkupDeclarationOpen => {
                self.bad_char_error();
                go!(self: to BogusComment)
            },

            states::CdataSection => {
                self.emit_temp_buf();
                self.bad_eof_error();
                go!(self: to Data)
            },

            states::CdataSectionBracket => go!(self: push_temp ']'; to CdataSection),

            states::CdataSectionEnd => go!(self: push_temp ']'; push_temp ']'; to CdataSection),
        }
    }

    /// Checks for supported SIMD feature, which is now either SSE2 for x86/x86_64 or NEON for aarch64.
    fn is_supported_simd_feature_detected() -> bool {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            is_x86_feature_detected!("sse2")
        }

        #[cfg(target_arch = "aarch64")]
        {
            std::arch::is_aarch64_feature_detected!("neon")
        }

        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
        false
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
    /// Implements the [data state] with SIMD instructions.
    /// Calls SSE2- or NEON-specific function for chunks and processes any remaining bytes.
    ///
    /// The algorithm implemented is the naive SIMD approach described [here].
    ///
    /// ### SAFETY:
    /// Calling this function on a CPU that supports neither SSE2 nor NEON causes undefined behaviour.
    ///
    /// [data state]: https://html.spec.whatwg.org/#data-state
    /// [here]: https://lemire.me/blog/2024/06/08/scan-html-faster-with-simd-instructions-chrome-edition/
    unsafe fn data_state_simd_fast_path(&self, input: &mut StrTendril) -> Option<SetResult> {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        let (mut i, mut n_newlines) = self.data_state_sse2_fast_path(input);

        #[cfg(target_arch = "aarch64")]
        let (mut i, mut n_newlines) = self.data_state_neon_fast_path(input);

        // Process any remaining bytes (less than STRIDE)
        while let Some(c) = input.as_bytes().get(i) {
            if matches!(*c, b'<' | b'&' | b'\r' | b'\0') {
                break;
            }
            if *c == b'\n' {
                n_newlines += 1;
            }

            i += 1;
        }

        let set_result = if i == 0 {
            let first_char = input.pop_front_char().unwrap();
            debug_assert!(matches!(first_char, '<' | '&' | '\r' | '\0'));

            // FIXME: Passing a bogus input queue is only relevant when c is \n, which can never happen in this case.
            // Still, it would be nice to not have to do that.
            // The same is true for the unwrap call.
            let preprocessed_char = self
                .get_preprocessed_char(first_char, &BufferQueue::default())
                .unwrap();
            SetResult::FromSet(preprocessed_char)
        } else {
            debug_assert!(
                input.len() >= i,
                "Trying to remove {:?} bytes from a tendril that is only {:?} bytes long",
                i,
                input.len()
            );
            let consumed_chunk = input.unsafe_subtendril(0, i as u32);
            input.unsafe_pop_front(i as u32);
            SetResult::NotFromSet(consumed_chunk)
        };

        self.current_line.set(self.current_line.get() + n_newlines);

        Some(set_result)
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[target_feature(enable = "sse2")]
    /// Implements the [data state] with SSE2 instructions for x86/x86_64.
    /// Returns a pair of the number of bytes processed and the number of newlines found.
    ///
    /// ### SAFETY:
    /// Calling this function on a CPU that does not support NEON causes undefined behaviour.
    ///
    /// [data state]: https://html.spec.whatwg.org/#data-state
    unsafe fn data_state_sse2_fast_path(&self, input: &mut StrTendril) -> (usize, u64) {
        #[cfg(target_arch = "x86")]
        use std::arch::x86::{
            __m128i, _mm_cmpeq_epi8, _mm_loadu_si128, _mm_movemask_epi8, _mm_or_si128,
            _mm_set1_epi8,
        };
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64::{
            __m128i, _mm_cmpeq_epi8, _mm_loadu_si128, _mm_movemask_epi8, _mm_or_si128,
            _mm_set1_epi8,
        };

        debug_assert!(!input.is_empty());

        let quote_mask = _mm_set1_epi8('<' as i8);
        let escape_mask = _mm_set1_epi8('&' as i8);
        let carriage_return_mask = _mm_set1_epi8('\r' as i8);
        let zero_mask = _mm_set1_epi8('\0' as i8);
        let newline_mask = _mm_set1_epi8('\n' as i8);

        let raw_bytes: &[u8] = input.as_bytes();
        let start = raw_bytes.as_ptr();

        const STRIDE: usize = 16;
        let mut i = 0;
        let mut n_newlines = 0;
        while i + STRIDE <= raw_bytes.len() {
            // Load a 16 byte chunk from the input
            let data = _mm_loadu_si128(start.add(i) as *const __m128i);

            // Compare the chunk against each mask
            let quotes = _mm_cmpeq_epi8(data, quote_mask);
            let escapes = _mm_cmpeq_epi8(data, escape_mask);
            let carriage_returns = _mm_cmpeq_epi8(data, carriage_return_mask);
            let zeros = _mm_cmpeq_epi8(data, zero_mask);
            let newlines = _mm_cmpeq_epi8(data, newline_mask);

            // Combine all test results and create a bitmask from them.
            // Each bit in the mask will be 1 if the character at the bit position is in the set and 0 otherwise.
            let test_result = _mm_or_si128(
                _mm_or_si128(quotes, zeros),
                _mm_or_si128(escapes, carriage_returns),
            );
            let bitmask = _mm_movemask_epi8(test_result);
            let newline_mask = _mm_movemask_epi8(newlines);

            if (bitmask != 0) {
                // We have reached one of the characters that cause the state machine to transition
                let position = if cfg!(target_endian = "little") {
                    bitmask.trailing_zeros() as usize
                } else {
                    bitmask.leading_zeros() as usize
                };

                n_newlines += (newline_mask & ((1 << position) - 1)).count_ones() as u64;
                i += position;
                break;
            } else {
                n_newlines += newline_mask.count_ones() as u64;
            }

            i += STRIDE;
        }

        (i, n_newlines)
    }

    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    /// Implements the [data state] with NEON SIMD instructions for AArch64.
    /// Returns a pair of the number of bytes processed and the number of newlines found.
    ///
    /// ### SAFETY:
    /// Calling this function on a CPU that does not support NEON causes undefined behaviour.
    ///
    /// [data state]: https://html.spec.whatwg.org/#data-state
    unsafe fn data_state_neon_fast_path(&self, input: &mut StrTendril) -> (usize, u64) {
        use std::arch::aarch64::{vceqq_u8, vdupq_n_u8, vld1q_u8, vmaxvq_u8, vorrq_u8};

        debug_assert!(!input.is_empty());

        let quote_mask = vdupq_n_u8(b'<');
        let escape_mask = vdupq_n_u8(b'&');
        let carriage_return_mask = vdupq_n_u8(b'\r');
        let zero_mask = vdupq_n_u8(b'\0');
        let newline_mask = vdupq_n_u8(b'\n');

        let raw_bytes: &[u8] = input.as_bytes();
        let start = raw_bytes.as_ptr();

        const STRIDE: usize = 16;
        let mut i = 0;
        let mut n_newlines = 0;
        while i + STRIDE <= raw_bytes.len() {
            // Load a 16 byte chunk from the input
            let data = vld1q_u8(start.add(i));

            // Compare the chunk against each mask
            let quotes = vceqq_u8(data, quote_mask);
            let escapes = vceqq_u8(data, escape_mask);
            let carriage_returns = vceqq_u8(data, carriage_return_mask);
            let zeros = vceqq_u8(data, zero_mask);
            let newlines = vceqq_u8(data, newline_mask);

            // Combine all test results and create a bitmask from them.
            // Each bit in the mask will be 1 if the character at the bit position is in the set and 0 otherwise.
            let test_result =
                vorrq_u8(vorrq_u8(quotes, zeros), vorrq_u8(escapes, carriage_returns));
            let bitmask = vmaxvq_u8(test_result);
            let newline_mask = vmaxvq_u8(newlines);
            if bitmask != 0 {
                // We have reached one of the characters that cause the state machine to transition
                let chunk_bytes = std::slice::from_raw_parts(start.add(i), STRIDE);
                let position = chunk_bytes
                    .iter()
                    .position(|&b| matches!(b, b'<' | b'&' | b'\r' | b'\0'))
                    .unwrap();

                n_newlines += chunk_bytes[..position]
                    .iter()
                    .filter(|&&b| b == b'\n')
                    .count() as u64;

                i += position;
                break;
            } else if newline_mask != 0 {
                let chunk_bytes = std::slice::from_raw_parts(start.add(i), STRIDE);
                n_newlines += chunk_bytes.iter().filter(|&&b| b == b'\n').count() as u64;
            }

            i += STRIDE;
        }

        (i, n_newlines)
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
    use std::cell::RefCell;

    use crate::LocalName;

    // LinesMatch implements the TokenSink trait. It is used for testing to see
    // if current_line is being updated when process_token is called. The lines
    // vector is a collection of the line numbers that each token is on.
    struct LinesMatch {
        tokens: RefCell<Vec<Token>>,
        current_str: RefCell<StrTendril>,
        lines: RefCell<Vec<(Token, u64)>>,
    }

    impl LinesMatch {
        fn new() -> LinesMatch {
            LinesMatch {
                tokens: RefCell::new(vec![]),
                current_str: RefCell::new(StrTendril::new()),
                lines: RefCell::new(vec![]),
            }
        }

        fn push(&self, token: Token, line_number: u64) {
            self.finish_str();
            self.lines.borrow_mut().push((token, line_number));
        }

        fn finish_str(&self) {
            if !self.current_str.borrow().is_empty() {
                let s = self.current_str.take();
                self.tokens.borrow_mut().push(CharacterTokens(s));
            }
        }
    }

    impl TokenSink for LinesMatch {
        type Handle = ();

        fn process_token(&self, token: Token, line_number: u64) -> TokenSinkResult<Self::Handle> {
            match token {
                CharacterTokens(b) => {
                    self.current_str.borrow_mut().push_slice(&b);
                },

                NullCharacterToken => {
                    self.current_str.borrow_mut().push_char('\0');
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
        let tok = Tokenizer::new(sink, opts);
        let buffer = BufferQueue::default();
        for chunk in input.into_iter() {
            buffer.push_back(chunk);
            let _ = tok.feed(&buffer);
        }
        tok.end();
        tok.sink.lines.take()
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
