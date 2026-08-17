// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! Traits for serializing elements. The serializer expects the data to be xml-like (with a name,
//! and optional children, attrs, text, comments, doctypes, and [processing instructions]). It uses
//! the visitor pattern, where the serializer and the serializable objects are decoupled and
//! implement their own traits.
//!
//! [processing instructions]: https://en.wikipedia.org/wiki/Processing_Instruction

use crate::QualName;
use std::io;

//§ serializing-html-fragments
/// Used as a parameter to `serialize`, telling it if we want to skip the parent.
#[derive(Clone, PartialEq)]
pub enum TraversalScope {
    /// Include the parent node when serializing.
    IncludeNode,
    /// Only serialize the children of the node, treating any provided qualified name as the
    /// parent while serializing.
    ///
    /// This is used in the implementation of [`html5ever::serialize::serialize`]
    ///
    /// [`html5ever::serialize::serialize`]: ../../html5ever/serialize/fn.serialize.html
    ChildrenOnly(Option<QualName>),
}

/// Types that can be serialized (according to the xml-like scheme in `Serializer`) implement this
/// trait.
pub trait Serialize {
    /// Take the serializer and call its methods to serialize this type. The type will dictate
    /// which methods are called and with what parameters.
    fn serialize<S>(&self, serializer: &mut S, traversal_scope: TraversalScope) -> io::Result<()>
    where
        S: Serializer;
}

/// Types that are capable of serializing implement this trait
pub trait Serializer {
    /// Serialize the start of an element, for example `<div class="test">`.
    fn start_elem<'a, AttrIter>(&mut self, name: QualName, attrs: AttrIter) -> io::Result<()>
    where
        AttrIter: Iterator<Item = AttrRef<'a>>;

    /// Serialize the end of an element, for example `</div>`.
    fn end_elem(&mut self, name: QualName) -> io::Result<()>;

    /// Serialize a plain text node.
    fn write_text(&mut self, text: &str) -> io::Result<()>;

    /// Serialize a comment node, for example `<!-- comment -->`.
    fn write_comment(&mut self, text: &str) -> io::Result<()>;

    /// Serialize a doctype node, for example `<!doctype html>`.
    fn write_doctype(&mut self, name: &str) -> io::Result<()>;

    /// Serialize a processing instruction node, for example
    /// `<?xml-stylesheet type="text/xsl" href="style.xsl"?>`.
    fn write_processing_instruction(&mut self, target: &str, data: &str) -> io::Result<()>;
}

/// A type alias for an attribute name and value (e.g. the `class="test"` in `<div class="test">`
/// is represented as `(<QualName of type class>, "test")`.
///
/// This is used in [`Serializer::start_elem`] where the value being serialized must supply an
/// iterator over the attributes for the current element
///
/// [`Serializer::start_elem`]: trait.Serializer.html#tymethod.start_elem
pub type AttrRef<'a> = (&'a QualName, &'a str);
