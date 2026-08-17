//! There are few ambigious cases where we can't determine correct
//! parsing context having a limited information about the current
//! state of tree builder. This caused issues in the past where
//! Cloudflare's security features were used as XSS gadgets
//! (see <https://portswigger.net/blog/when-security-features-collide>).
//! Therefore, due to these safety concerns in such cases we prefer
//! to bail out from tokenization process.
//!
//! In tree builder simulation we need to switch parser to one
//! of standalone text parsing state machines if we encounter some
//! specific tags. E.g. if we encounter `<script>` start tag we should
//! treat all content up to the closing `</script>` tag as text.
//! Without having a full-featured tree construction stage there is way
//! to trick parser into parsing content that has actual tags in it
//! as text. E.g. by putting `<script>` start tag into context where
//! it will be ignored.
//!
//! There are just a few tree builder insertion modes in which text
//! parsing mode switching start tags can be ignored: in `<select>` and in
//! or after `<frameset>`.
//!
//! There are numerous not so obvious ways to get into or get out of these
//! insertion modes. So, for safety reasons we try to be pro-active here
//! and just bailout in case if we see text parsing mode switching start tags
//! between `<select>` start and end tag, or anywhere after the `<frameset>`
//! start tag. These cases shouldn't trigger bailout for any *conforming*
//! markup.
//!
//! However, there is a case where bailout could happen even with conforming
//! markup: if we encounter text parsing mode switching start tag in `<template>`
//! which is inside `<select>` element content. Unfortunately, rules required
//! to track template parsing context are way to complicated in such a case
//! and will require an implementation of the significant part of the tree
//! construction state. Though, current assumption is that markup that can
//! trigger this bailout case should be seen quite rarely in the wild.
use crate::html::{LocalNameHash, Tag};
use std::fmt::{self, Display};
use thiserror::Error;

/// An error that occurs when HTML parser runs into an ambigious state in the [`strict`] mode.
///
/// Since the rewriter operates on a token stream and doesn't have access to a full
/// DOM-tree, there are certain rare cases of non-conforming HTML markup which can't be
/// guaranteed to be parsed correctly without an ability to backtrace the tree.
///
/// Therefore, due to security considerations, sometimes it's preferable to abort the
/// rewriting process in case of such uncertainty.
///
/// One of the simplest examples of such markup is the following:
///
/// ```html
/// ...
/// <select><xmp><script>"use strict";</script></select>
/// ...
/// ```
///
/// The `<xmp>` element is not allowed inside the `<select>` element, so in a browser the start
/// tag for `<xmp>` will be ignored and following `<script>` element will be parsed and executed.
///
/// On the other hand, the `<select>` element itself can be also ignored depending on the
/// context in which it was parsed. In this case, the `<xmp>` element will not be ignored
/// and the `<script>` element along with its content will be parsed as a simple text inside
/// it.
///
/// So, in this case the parser needs an ability to backtrace the DOM-tree to figure out the
/// correct parsing context.
///
/// [`strict`]: ../struct.Settings.html#structfield.strict
#[derive(Error, Debug, Eq, PartialEq)]
pub struct ParsingAmbiguityError {
    on_tag_name: Box<str>,
}

impl Display for ParsingAmbiguityError {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            concat!(
                "The parser has encountered a text content tag (`<{}>`) in the context where it is ",
                "ambiguous whether this tag should be ignored or not. And, thus, is is unclear is ",
                "consequent content should be parsed as raw text or HTML markup.",
                "\n\n",
                "This error occurs due to the limited capabilities of the streaming parsing. However, ",
                "almost all of the cases of this error are caused by a non-conforming markup (e.g. a ",
                "`<script>` element in `<select>` element)."
            ),
            self.on_tag_name
        )
    }
}

// NOTE: use macro for the assertion function definition, so we can
// provide ambiguity error with a string representation of the tag
// name without a necessity to implement conversion from u64 tag name
// hash to a string. This also allows us to be consistent about asserted
// tag name hashes and the corresponding tag name strings.
macro_rules! create_assert_for_tags {
    ( $($tag:ident),+ ) => {
        #[cold]
        fn tag_hash_to_string(tag_name: LocalNameHash) -> Box<str> {
            let s = match tag_name {
                $(t if t == Tag::$tag => stringify!($tag),)+
                _ => "no string representation",
            };
            s.to_ascii_lowercase().into_boxed_str()
        }

        #[inline]
        fn assert_not_ambigious_text_type_switch(
            tag_name: LocalNameHash,
        ) -> Result<(), ParsingAmbiguityError> {
            if tag_is_one_of!(tag_name, [ $($tag),+ ]) {
                Err(ParsingAmbiguityError {
                    on_tag_name: tag_hash_to_string(tag_name)
                })
            } else {
                Ok(())
            }
        }
    };
}

create_assert_for_tags!(
    Textarea, Title, Plaintext, Script, Style, Iframe, Xmp, Noembed, Noframes, Noscript
);

#[derive(Copy, Clone)]
enum State {
    Default,
    InSelect,
    InTemplateInSelect(u64),
    InOrAfterFrameset,
}

pub(crate) struct AmbiguityGuard {
    state: State,
}

impl Default for AmbiguityGuard {
    fn default() -> Self {
        Self {
            state: State::Default,
        }
    }
}

impl AmbiguityGuard {
    pub fn track_start_tag(
        &mut self,
        tag_name: LocalNameHash,
    ) -> Result<(), ParsingAmbiguityError> {
        match self.state {
            State::Default => {
                if tag_name == Tag::Select {
                    self.state = State::InSelect;
                } else if tag_name == Tag::Frameset {
                    self.state = State::InOrAfterFrameset;
                }
            }
            State::InSelect => {
                // NOTE: these start tags cause premature exit
                // from "in select" insertion mode.
                if tag_is_one_of!(tag_name, [Select, Textarea, Input, Keygen]) {
                    self.state = State::Default;
                } else if tag_name == Tag::Template {
                    self.state = State::InTemplateInSelect(1);
                }
                // NOTE: <script> is allowed in "in select" insertion mode.
                else if tag_name != Tag::Script {
                    assert_not_ambigious_text_type_switch(tag_name)?;
                }
            }
            State::InTemplateInSelect(depth) => {
                if tag_name == Tag::Template {
                    self.state = State::InTemplateInSelect(depth + 1);
                } else {
                    assert_not_ambigious_text_type_switch(tag_name)?;
                }
            }
            State::InOrAfterFrameset => {
                // NOTE: <noframes> is allowed in and after <frameset>.
                if tag_name != Tag::Noframes {
                    assert_not_ambigious_text_type_switch(tag_name)?;
                }
            }
        }

        Ok(())
    }

    pub fn track_end_tag(&mut self, tag_name: LocalNameHash) {
        match self.state {
            State::InSelect if tag_name == Tag::Select => {
                self.state = State::Default;
            }
            State::InTemplateInSelect(depth) if tag_name == Tag::Template => {
                self.state = if depth == 1 {
                    State::InSelect
                } else {
                    State::InTemplateInSelect(depth - 1)
                }
            }
            _ => (),
        }
    }
}
