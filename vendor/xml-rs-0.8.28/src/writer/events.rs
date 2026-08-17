//! Contains `XmlEvent` datatype, instances of which are consumed by the writer.

use std::borrow::Cow;

use crate::attribute::Attribute;
use crate::common::XmlVersion;
use crate::name::Name;
use crate::namespace::{Namespace, NS_NO_PREFIX};

/// A part of an XML output stream.
///
/// Objects of this enum are consumed by `EventWriter`. They correspond to different parts of
/// an XML document.
#[derive(Debug, Clone)]
pub enum XmlEvent<'a> {
    /// Corresponds to XML document declaration.
    ///
    /// This event should always be written before any other event. If it is not written
    /// at all, a default XML declaration will be outputted if the corresponding option
    /// is set in the configuration. Otherwise an error will be returned.
    StartDocument {
        /// XML version.
        ///
        /// Defaults to `XmlVersion::Version10`.
        version: XmlVersion,

        /// XML document encoding.
        ///
        /// Defaults to `Some("UTF-8")`.
        encoding: Option<&'a str>,

        /// XML standalone declaration.
        ///
        /// Defaults to `None`.
        standalone: Option<bool>,
    },

    /// Denotes an XML processing instruction.
    ProcessingInstruction {
        /// Processing instruction target.
        name: &'a str,

        /// Processing instruction content.
        data: Option<&'a str>,
    },

    /// Denotes a beginning of an XML element.
    StartElement {
        /// Qualified name of the element.
        name: Name<'a>,

        /// A list of attributes associated with the element.
        ///
        /// Currently attributes are not checked for duplicates (TODO). Attribute values
        /// will be escaped, and all characters invalid for attribute values like `"` or `<`
        /// will be changed into character entities.
        attributes: Cow<'a, [Attribute<'a>]>,

        /// Contents of the namespace mapping at this point of the document.
        ///
        /// This mapping will be inspected for "new" entries, and if at this point of the document
        /// a particular pair of prefix and namespace URI is already defined, no namespace
        /// attributes will be emitted.
        namespace: Cow<'a, Namespace>,
    },

    /// Denotes an end of an XML element.
    EndElement {
        /// Optional qualified name of the element.
        ///
        /// If `None`, then it is assumed that the element name should be the last valid one.
        /// If `Some` and element names tracking is enabled, then the writer will check it for
        /// correctness.
        name: Option<Name<'a>>,
    },

    /// Denotes CDATA content.
    ///
    /// This event contains unparsed data, and no escaping will be performed when writing it
    /// to the output stream.
    CData(&'a str),

    /// Denotes a comment.
    ///
    /// The string will be checked for invalid sequences and error will be returned by the
    /// write operation
    Comment(&'a str),

    /// Denotes character data outside of tags.
    ///
    /// Contents of this event will be escaped if `perform_escaping` option is enabled,
    /// that is, every character invalid for PCDATA will appear as a character entity.
    Characters(&'a str),
}

impl<'a> XmlEvent<'a> {
    /// Returns an writer event for a processing instruction.
    #[inline]
    #[must_use]
    pub const fn processing_instruction(name: &'a str, data: Option<&'a str>) -> Self {
        XmlEvent::ProcessingInstruction { name, data }
    }

    /// Returns a builder for a starting element.
    ///
    /// This builder can then be used to tweak attributes and namespace starting at
    /// this element.
    #[inline]
    pub fn start_element<S>(name: S) -> StartElementBuilder<'a> where S: Into<Name<'a>> {
        StartElementBuilder {
            name: name.into(),
            attributes: Vec::new(),
            namespace: Namespace::empty(),
        }
    }

    /// Returns a builder for an closing element.
    ///
    /// This method, unline `start_element()`, does not accept a name because by default
    /// the writer is able to determine it automatically. However, when this functionality
    /// is disabled, it is possible to specify the name with `name()` method on the builder.
    #[inline]
    #[must_use]
    pub const fn end_element() -> EndElementBuilder<'a> {
        EndElementBuilder { name: None }
    }

    /// Returns a CDATA event.
    ///
    /// Naturally, the provided string won't be escaped, except for closing CDATA token `]]>`
    /// (depending on the configuration).
    #[inline]
    #[must_use]
    pub const fn cdata(data: &'a str) -> Self {
        XmlEvent::CData(data)
    }

    /// Returns a regular characters (PCDATA) event.
    ///
    /// All offending symbols, in particular, `&` and `<`, will be escaped by the writer.
    #[inline]
    #[must_use]
    pub const fn characters(data: &'a str) -> Self {
        XmlEvent::Characters(data)
    }

    /// Returns a comment event.
    #[inline]
    #[must_use]
    pub const fn comment(data: &'a str) -> Self {
        XmlEvent::Comment(data)
    }
}

impl<'a> From<&'a str> for XmlEvent<'a> {
    #[inline]
    fn from(s: &'a str) -> Self {
        XmlEvent::Characters(s)
    }
}

/// A builder for a closing element event.
pub struct EndElementBuilder<'a> {
    name: Option<Name<'a>>,
}

/// A builder for a closing element event.
impl<'a> EndElementBuilder<'a> {
    /// Sets the name of this closing element.
    ///
    /// Usually the writer is able to determine closing element names automatically. If
    /// this functionality is enabled (by default it is), then this name is checked for correctness.
    /// It is possible, however, to disable such behavior; then the user must ensure that
    /// closing element name is correct manually.
    #[inline]
    #[must_use]
    pub fn name<N>(mut self, name: N) -> Self where N: Into<Name<'a>> {
        self.name = Some(name.into());
        self
    }
}

impl<'a> From<EndElementBuilder<'a>> for XmlEvent<'a> {
    fn from(b: EndElementBuilder<'a>) -> Self {
        XmlEvent::EndElement { name: b.name }
    }
}

/// A builder for a starting element event.
pub struct StartElementBuilder<'a> {
    name: Name<'a>,
    attributes: Vec<Attribute<'a>>,
    namespace: Namespace,
}

impl<'a> StartElementBuilder<'a> {
    /// Sets an attribute value of this element to the given string.
    ///
    /// This method can be used to add attributes to the starting element. Name is a qualified
    /// name; its namespace is ignored, but its prefix is checked for correctness, that is,
    /// it is checked that the prefix is bound to some namespace in the current context.
    ///
    /// Currently attributes are not checked for duplicates. Note that duplicate attributes
    /// are a violation of XML document well-formedness.
    ///
    /// The writer checks that you don't specify reserved prefix names, for example `xmlns`.
    #[inline]
    #[must_use]
    pub fn attr<N>(mut self, name: N, value: &'a str) -> Self
    where N: Into<Name<'a>> {
        self.attributes.push(Attribute::new(name.into(), value));
        self
    }

    /// Adds a namespace to the current namespace context.
    ///
    /// If no namespace URI was bound to the provided prefix at this point of the document,
    /// then the mapping from the prefix to the provided namespace URI will be written as
    /// a part of this element attribute set.
    ///
    /// If the same namespace URI was bound to the provided prefix at this point of the document,
    /// then no namespace attributes will be emitted.
    ///
    /// If some other namespace URI was bound to the provided prefix at this point of the document,
    /// then another binding will be added as a part of this element attribute set, shadowing
    /// the outer binding.
    #[inline]
    #[must_use]
    pub fn ns<S1, S2>(mut self, prefix: S1, uri: S2) -> Self
        where S1: Into<String>, S2: Into<String>
    {
        self.namespace.put(prefix, uri);
        self
    }

    /// Adds a default namespace mapping to the current namespace context.
    ///
    /// Same rules as for `ns()` are also valid for the default namespace mapping.
    #[inline]
    #[must_use]
    pub fn default_ns<S>(mut self, uri: S) -> Self
    where S: Into<String> {
        self.namespace.put(NS_NO_PREFIX, uri);
        self
    }
}

impl<'a> From<StartElementBuilder<'a>> for XmlEvent<'a> {
    #[inline]
    fn from(b: StartElementBuilder<'a>) -> Self {
        XmlEvent::StartElement {
            name: b.name,
            attributes: Cow::Owned(b.attributes),
            namespace: Cow::Owned(b.namespace),
        }
    }
}

impl<'a> TryFrom<&'a crate::reader::XmlEvent> for XmlEvent<'a> {
    type Error = crate::reader::Error;

    fn try_from(event: &crate::reader::XmlEvent) -> Result<XmlEvent<'_>, Self::Error> {
        event.as_writer_event().ok_or(crate::reader::Error {
            pos: crate::common::TextPosition::new(),
            kind: crate::reader::ErrorKind::UnexpectedEof,
        })
    }
}
