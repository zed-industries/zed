//! Module for handling names according to the W3C [Namespaces in XML 1.1 (Second Edition)][spec]
//! specification
//!
//! [spec]: https://www.w3.org/TR/xml-names11

use crate::errors::{Error, Result};
use crate::events::attributes::Attribute;
use crate::events::BytesStart;
use crate::utils::write_byte_string;
use memchr::memchr;
use std::convert::TryFrom;
use std::fmt::{self, Debug, Formatter};

/// A [qualified name] of an element or an attribute, including an optional
/// namespace [prefix](Prefix) and a [local name](LocalName).
///
/// [qualified name]: https://www.w3.org/TR/xml-names11/#dt-qualname
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-types", derive(serde::Deserialize, serde::Serialize))]
pub struct QName<'a>(pub &'a [u8]);
impl<'a> QName<'a> {
    /// Converts this name to an internal slice representation.
    #[inline(always)]
    pub fn into_inner(self) -> &'a [u8] {
        self.0
    }

    /// Returns local part of this qualified name.
    ///
    /// All content up to and including the first `:` character is removed from
    /// the tag name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use quick_xml::name::QName;
    /// let simple = QName(b"simple-name");
    /// assert_eq!(simple.local_name().as_ref(), b"simple-name");
    ///
    /// let qname = QName(b"namespace:simple-name");
    /// assert_eq!(qname.local_name().as_ref(), b"simple-name");
    /// ```
    pub fn local_name(&self) -> LocalName<'a> {
        LocalName(self.index().map_or(self.0, |i| &self.0[i + 1..]))
    }

    /// Returns namespace part of this qualified name or `None` if namespace part
    /// is not defined (symbol `':'` not found).
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::convert::AsRef;
    /// # use quick_xml::name::QName;
    /// let simple = QName(b"simple-name");
    /// assert_eq!(simple.prefix(), None);
    ///
    /// let qname = QName(b"prefix:simple-name");
    /// assert_eq!(qname.prefix().as_ref().map(|n| n.as_ref()), Some(b"prefix".as_ref()));
    /// ```
    pub fn prefix(&self) -> Option<Prefix<'a>> {
        self.index().map(|i| Prefix(&self.0[..i]))
    }

    /// The same as `(qname.local_name(), qname.prefix())`, but does only one
    /// lookup for a `':'` symbol.
    pub fn decompose(&self) -> (LocalName<'a>, Option<Prefix<'a>>) {
        match self.index() {
            None => (LocalName(self.0), None),
            Some(i) => (LocalName(&self.0[i + 1..]), Some(Prefix(&self.0[..i]))),
        }
    }

    /// If that `QName` represents `"xmlns"` series of names, returns `Some`,
    /// otherwise `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use quick_xml::name::{QName, PrefixDeclaration};
    /// let qname = QName(b"xmlns");
    /// assert_eq!(qname.as_namespace_binding(), Some(PrefixDeclaration::Default));
    ///
    /// let qname = QName(b"xmlns:prefix");
    /// assert_eq!(qname.as_namespace_binding(), Some(PrefixDeclaration::Named(b"prefix")));
    ///
    /// // Be aware that this method does not check the validity of the prefix - it can be empty!
    /// let qname = QName(b"xmlns:");
    /// assert_eq!(qname.as_namespace_binding(), Some(PrefixDeclaration::Named(b"")));
    ///
    /// let qname = QName(b"other-name");
    /// assert_eq!(qname.as_namespace_binding(), None);
    ///
    /// // https://www.w3.org/TR/xml-names11/#xmlReserved
    /// let qname = QName(b"xmlns-reserved-name");
    /// assert_eq!(qname.as_namespace_binding(), None);
    /// ```
    pub fn as_namespace_binding(&self) -> Option<PrefixDeclaration<'a>> {
        if self.0.starts_with(b"xmlns") {
            return match self.0.get(5) {
                None => Some(PrefixDeclaration::Default),
                Some(&b':') => Some(PrefixDeclaration::Named(&self.0[6..])),
                _ => None,
            };
        }
        None
    }

    /// Returns the index in the name where prefix ended
    #[inline(always)]
    fn index(&self) -> Option<usize> {
        memchr(b':', self.0)
    }
}
impl<'a> Debug for QName<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "QName(")?;
        write_byte_string(f, self.0)?;
        write!(f, ")")
    }
}
impl<'a> AsRef<[u8]> for QName<'a> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// A [local (unqualified) name] of an element or an attribute, i.e. a name
/// without [prefix](Prefix).
///
/// [local (unqualified) name]: https://www.w3.org/TR/xml-names11/#dt-localname
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-types", derive(serde::Deserialize, serde::Serialize))]
pub struct LocalName<'a>(&'a [u8]);
impl<'a> LocalName<'a> {
    /// Converts this name to an internal slice representation.
    #[inline(always)]
    pub fn into_inner(self) -> &'a [u8] {
        self.0
    }
}
impl<'a> Debug for LocalName<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "LocalName(")?;
        write_byte_string(f, self.0)?;
        write!(f, ")")
    }
}
impl<'a> AsRef<[u8]> for LocalName<'a> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}
impl<'a> From<QName<'a>> for LocalName<'a> {
    /// Creates `LocalName` from a [`QName`]
    ///
    /// # Examples
    ///
    /// ```
    /// # use quick_xml::name::{LocalName, QName};
    ///
    /// let local: LocalName = QName(b"unprefixed").into();
    /// assert_eq!(local.as_ref(), b"unprefixed");
    ///
    /// let local: LocalName = QName(b"some:prefix").into();
    /// assert_eq!(local.as_ref(), b"prefix");
    /// ```
    #[inline]
    fn from(name: QName<'a>) -> Self {
        Self(name.index().map_or(name.0, |i| &name.0[i + 1..]))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// A [namespace prefix] part of the [qualified name](QName) of an element tag
/// or an attribute: a `prefix` in `<prefix:local-element-name>` or
/// `prefix:local-attribute-name="attribute value"`.
///
/// [namespace prefix]: https://www.w3.org/TR/xml-names11/#dt-prefix
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-types", derive(serde::Deserialize, serde::Serialize))]
pub struct Prefix<'a>(&'a [u8]);
impl<'a> Prefix<'a> {
    /// Extracts internal slice
    #[inline(always)]
    pub fn into_inner(self) -> &'a [u8] {
        self.0
    }
}
impl<'a> Debug for Prefix<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Prefix(")?;
        write_byte_string(f, self.0)?;
        write!(f, ")")
    }
}
impl<'a> AsRef<[u8]> for Prefix<'a> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// A namespace prefix declaration, `xmlns` or `xmlns:<name>`, as defined in
/// [XML Schema specification](https://www.w3.org/TR/xml-names11/#ns-decl)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrefixDeclaration<'a> {
    /// XML attribute binds a default namespace. Corresponds to `xmlns` in `xmlns="..."`
    Default,
    /// XML attribute binds a specified prefix to a namespace. Corresponds to a
    /// `prefix` in `xmlns:prefix="..."`, which is stored as payload of this variant.
    Named(&'a [u8]),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// A [namespace name] that is declared in a `xmlns[:prefix]="namespace name"`.
///
/// [namespace name]: https://www.w3.org/TR/xml-names11/#dt-NSName
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-types", derive(serde::Deserialize, serde::Serialize))]
pub struct Namespace<'a>(pub &'a [u8]);
impl<'a> Namespace<'a> {
    /// Converts this namespace to an internal slice representation.
    ///
    /// This is [non-normalized] attribute value, i.e. any entity references is
    /// not expanded and space characters are not removed. This means, that
    /// different byte slices, returned from this method, can represent the same
    /// namespace and would be treated by parser as identical.
    ///
    /// For example, if the entity **eacute** has been defined to be **é**,
    /// the empty tags below all contain namespace declarations binding the
    /// prefix `p` to the same [IRI reference], `http://example.org/rosé`.
    ///
    /// ```xml
    /// <p:foo xmlns:p="http://example.org/rosé" />
    /// <p:foo xmlns:p="http://example.org/ros&#xe9;" />
    /// <p:foo xmlns:p="http://example.org/ros&#xE9;" />
    /// <p:foo xmlns:p="http://example.org/ros&#233;" />
    /// <p:foo xmlns:p="http://example.org/ros&eacute;" />
    /// ```
    ///
    /// This is because XML entity references are expanded during attribute value
    /// normalization.
    ///
    /// [non-normalized]: https://www.w3.org/TR/xml11/#AVNormalize
    /// [IRI reference]: https://datatracker.ietf.org/doc/html/rfc3987
    #[inline(always)]
    pub fn into_inner(self) -> &'a [u8] {
        self.0
    }
    //TODO: implement value normalization and use it when comparing namespaces
}
impl<'a> Debug for Namespace<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Namespace(")?;
        write_byte_string(f, self.0)?;
        write!(f, ")")
    }
}
impl<'a> AsRef<[u8]> for Namespace<'a> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Result of [prefix] resolution which creates by [`NsReader::resolve_attribute`],
/// [`NsReader::resolve_element`], [`NsReader::read_resolved_event`] and
/// [`NsReader::read_resolved_event_into`] methods.
///
/// [prefix]: Prefix
/// [`NsReader::resolve_attribute`]: crate::reader::NsReader::resolve_attribute
/// [`NsReader::resolve_element`]: crate::reader::NsReader::resolve_element
/// [`NsReader::read_resolved_event`]: crate::reader::NsReader::read_resolved_event
/// [`NsReader::read_resolved_event_into`]: crate::reader::NsReader::read_resolved_event_into
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ResolveResult<'ns> {
    /// Qualified name does not contain prefix, and resolver does not define
    /// default namespace, so name is not bound to any namespace
    Unbound,
    /// [`Prefix`] resolved to the specified namespace
    Bound(Namespace<'ns>),
    /// Specified prefix was not found in scope
    Unknown(Vec<u8>),
}
impl<'ns> Debug for ResolveResult<'ns> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Unbound => write!(f, "Unbound"),
            Self::Bound(ns) => write!(f, "Bound({:?})", ns),
            Self::Unknown(p) => {
                write!(f, "Unknown(")?;
                write_byte_string(f, p)?;
                write!(f, ")")
            }
        }
    }
}

impl<'ns> TryFrom<ResolveResult<'ns>> for Option<Namespace<'ns>> {
    type Error = Error;

    /// Try to convert this result to an optional namespace and returns
    /// [`Error::UnknownPrefix`] if this result represents unknown prefix
    fn try_from(result: ResolveResult<'ns>) -> Result<Self> {
        use ResolveResult::*;

        match result {
            Unbound => Ok(None),
            Bound(ns) => Ok(Some(ns)),
            Unknown(p) => Err(Error::UnknownPrefix(p)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// An entry that contains index into the buffer with namespace bindings.
///
/// Defines a mapping from *[namespace prefix]* to *[namespace name]*.
/// If prefix is empty, defines a *default namespace* binding that applies to
/// unprefixed element names (unprefixed attribute names do not bind to any
/// namespace and they processing is dependent on the element in which their
/// defined).
///
/// [namespace prefix]: https://www.w3.org/TR/xml-names11/#dt-prefix
/// [namespace name]: https://www.w3.org/TR/xml-names11/#dt-NSName
#[derive(Debug, Clone)]
struct NamespaceEntry {
    /// Index of the namespace in the buffer
    start: usize,
    /// Length of the prefix
    /// * if greater than zero, then binds this namespace to the slice
    ///   `[start..start + prefix_len]` in the buffer.
    /// * else defines the current default namespace.
    prefix_len: usize,
    /// The length of a namespace name (the URI) of this namespace declaration.
    /// Name started just after prefix and extend for `value_len` bytes.
    ///
    /// The XML standard [specifies] that an empty namespace value 'removes' a namespace declaration
    /// for the extent of its scope. For prefix declarations that's not very interesting, but it is
    /// vital for default namespace declarations. With `xmlns=""` you can revert back to the default
    /// behaviour of leaving unqualified element names unqualified.
    ///
    /// [specifies]: https://www.w3.org/TR/xml-names11/#scoping
    value_len: usize,
    /// Level of nesting at which this namespace was declared. The declaring element is included,
    /// i.e., a declaration on the document root has `level = 1`.
    /// This is used to pop the namespace when the element gets closed.
    level: i32,
}

impl NamespaceEntry {
    /// Get the namespace prefix, bound to this namespace declaration, or `None`,
    /// if this declaration is for default namespace (`xmlns="..."`).
    #[inline]
    fn prefix<'b>(&self, ns_buffer: &'b [u8]) -> Option<Prefix<'b>> {
        if self.prefix_len == 0 {
            None
        } else {
            Some(Prefix(&ns_buffer[self.start..self.start + self.prefix_len]))
        }
    }

    /// Gets the namespace name (the URI) slice out of namespace buffer
    ///
    /// Returns `None` if namespace for this prefix was explicitly removed from
    /// scope, using `xmlns[:prefix]=""`
    #[inline]
    fn namespace<'ns>(&self, buffer: &'ns [u8]) -> ResolveResult<'ns> {
        if self.value_len == 0 {
            ResolveResult::Unbound
        } else {
            let start = self.start + self.prefix_len;
            ResolveResult::Bound(Namespace(&buffer[start..start + self.value_len]))
        }
    }
}

/// A namespace management buffer.
///
/// Holds all internal logic to push/pop namespaces with their levels.
#[derive(Debug, Default, Clone)]
pub(crate) struct NamespaceResolver {
    /// A stack of namespace bindings to prefixes that currently in scope
    bindings: Vec<NamespaceEntry>,
    /// The number of open tags at the moment. We need to keep track of this to know which namespace
    /// declarations to remove when we encounter an `End` event.
    nesting_level: i32,
}

impl NamespaceResolver {
    /// Begins a new scope and add to it all [namespace bindings] that found in
    /// the specified start element.
    ///
    /// [namespace binding]: https://www.w3.org/TR/xml-names11/#dt-NSDecl
    pub fn push(&mut self, start: &BytesStart, buffer: &mut Vec<u8>) {
        self.nesting_level += 1;
        let level = self.nesting_level;
        // adds new namespaces for attributes starting with 'xmlns:' and for the 'xmlns'
        // (default namespace) attribute.
        for a in start.attributes().with_checks(false) {
            if let Ok(Attribute { key: k, value: v }) = a {
                match k.as_namespace_binding() {
                    Some(PrefixDeclaration::Default) => {
                        let start = buffer.len();
                        buffer.extend_from_slice(&v);
                        self.bindings.push(NamespaceEntry {
                            start,
                            prefix_len: 0,
                            value_len: v.len(),
                            level,
                        });
                    }
                    Some(PrefixDeclaration::Named(prefix)) => {
                        let start = buffer.len();
                        buffer.extend_from_slice(prefix);
                        buffer.extend_from_slice(&v);
                        self.bindings.push(NamespaceEntry {
                            start,
                            prefix_len: prefix.len(),
                            value_len: v.len(),
                            level,
                        });
                    }
                    None => {}
                }
            } else {
                break;
            }
        }
    }

    /// Ends a top-most scope by popping all [namespace binding], that was added by
    /// last call to [`Self::push()`].
    ///
    /// [namespace binding]: https://www.w3.org/TR/xml-names11/#dt-NSDecl
    pub fn pop(&mut self, buffer: &mut Vec<u8>) {
        self.nesting_level -= 1;
        let current_level = self.nesting_level;
        // from the back (most deeply nested scope), look for the first scope that is still valid
        match self.bindings.iter().rposition(|n| n.level <= current_level) {
            // none of the namespaces are valid, remove all of them
            None => {
                buffer.clear();
                self.bindings.clear();
            }
            // drop all namespaces past the last valid namespace
            Some(last_valid_pos) => {
                if let Some(len) = self.bindings.get(last_valid_pos + 1).map(|n| n.start) {
                    buffer.truncate(len);
                    self.bindings.truncate(last_valid_pos + 1);
                }
            }
        }
    }

    /// Resolves a potentially qualified **element name** or **attribute name**
    /// into (namespace name, local name).
    ///
    /// *Qualified* names have the form `prefix:local-name` where the `prefix` is
    /// defined on any containing XML element via `xmlns:prefix="the:namespace:uri"`.
    /// The namespace prefix can be defined on the same element as the element or
    /// attribute in question.
    ///
    /// *Unqualified* attribute names do *not* inherit the current *default namespace*.
    ///
    /// # Lifetimes
    ///
    /// - `'n`: lifetime of an attribute or an element name
    /// - `'ns`: lifetime of a namespaces buffer, where all found namespaces are stored
    #[inline]
    pub fn resolve<'n, 'ns>(
        &self,
        name: QName<'n>,
        buffer: &'ns [u8],
        use_default: bool,
    ) -> (ResolveResult<'ns>, LocalName<'n>) {
        let (local_name, prefix) = name.decompose();
        (self.resolve_prefix(prefix, buffer, use_default), local_name)
    }

    /// Finds a [namespace name] for a given qualified **element name**, borrow
    /// it from the specified buffer.
    ///
    /// Returns `None`, if:
    /// - name is unqualified
    /// - prefix not found in the current scope
    /// - prefix was [unbound] using `xmlns:prefix=""`
    ///
    /// # Lifetimes
    ///
    /// - `'ns`: lifetime of a namespaces buffer, where all found namespaces are stored
    ///
    /// [namespace name]: https://www.w3.org/TR/xml-names11/#dt-NSName
    /// [unbound]: https://www.w3.org/TR/xml-names11/#scoping
    #[inline]
    pub fn find<'ns>(&self, element_name: QName, buffer: &'ns [u8]) -> ResolveResult<'ns> {
        self.resolve_prefix(element_name.prefix(), buffer, true)
    }

    fn resolve_prefix<'ns>(
        &self,
        prefix: Option<Prefix>,
        buffer: &'ns [u8],
        use_default: bool,
    ) -> ResolveResult<'ns> {
        self.bindings
            .iter()
            // Find the last defined binding that corresponds to the given prefix
            .rev()
            .find_map(|n| match (n.prefix(buffer), prefix) {
                // This is default namespace definition and name has no explicit prefix
                (None, None) if use_default => Some(n.namespace(buffer)),
                (None, None) => Some(ResolveResult::Unbound),

                // One part has prefix but other is not -> skip
                (None, Some(_)) => None,
                (Some(_), None) => None,

                // Prefixes does not match -> skip
                (Some(definition), Some(usage)) if definition != usage => None,

                // Prefixes the same, entry defines binding reset (corresponds to `xmlns:p=""`)
                _ if n.value_len == 0 => Some(Self::maybe_unknown(prefix)),
                // Prefixes the same, returns corresponding namespace
                _ => Some(n.namespace(buffer)),
            })
            .unwrap_or_else(|| Self::maybe_unknown(prefix))
    }

    #[inline]
    fn maybe_unknown(prefix: Option<Prefix>) -> ResolveResult<'static> {
        match prefix {
            Some(p) => ResolveResult::Unknown(p.into_inner().to_vec()),
            None => ResolveResult::Unbound,
        }
    }
}

#[cfg(test)]
mod namespaces {
    use super::*;
    use pretty_assertions::assert_eq;
    use ResolveResult::*;

    /// Unprefixed attribute names (resolved with `false` flag) never have a namespace
    /// according to <https://www.w3.org/TR/xml-names11/#defaulting>:
    ///
    /// > A default namespace declaration applies to all unprefixed element names
    /// > within its scope. Default namespace declarations do not apply directly
    /// > to attribute names; the interpretation of unprefixed attributes is
    /// > determined by the element on which they appear.
    mod unprefixed {
        use super::*;
        use pretty_assertions::assert_eq;

        /// Basic tests that checks that basic resolver functionality is working
        #[test]
        fn basic() {
            let name = QName(b"simple");
            let ns = Namespace(b"default");

            let mut resolver = NamespaceResolver::default();
            let mut buffer = Vec::new();

            resolver.push(
                &BytesStart::from_content(" xmlns='default'", 0),
                &mut buffer,
            );
            assert_eq!(buffer, b"default");

            // Check that tags without namespaces does not change result
            resolver.push(&BytesStart::from_content("", 0), &mut buffer);
            assert_eq!(buffer, b"default");
            resolver.pop(&mut buffer);

            assert_eq!(buffer, b"default");
            assert_eq!(
                resolver.resolve(name, &buffer, true),
                (Bound(ns), LocalName(b"simple"))
            );
            assert_eq!(
                resolver.resolve(name, &buffer, false),
                (Unbound, LocalName(b"simple"))
            );
            assert_eq!(resolver.find(name, &buffer), Bound(ns));
        }

        /// Test adding a second level of namespaces, which replaces the previous binding
        #[test]
        fn override_namespace() {
            let name = QName(b"simple");
            let old_ns = Namespace(b"old");
            let new_ns = Namespace(b"new");

            let mut resolver = NamespaceResolver::default();
            let mut buffer = Vec::new();

            resolver.push(&BytesStart::from_content(" xmlns='old'", 0), &mut buffer);
            resolver.push(&BytesStart::from_content(" xmlns='new'", 0), &mut buffer);

            assert_eq!(buffer, b"oldnew");
            assert_eq!(
                resolver.resolve(name, &buffer, true),
                (Bound(new_ns), LocalName(b"simple"))
            );
            assert_eq!(
                resolver.resolve(name, &buffer, false),
                (Unbound, LocalName(b"simple"))
            );
            assert_eq!(resolver.find(name, &buffer), Bound(new_ns));

            resolver.pop(&mut buffer);
            assert_eq!(buffer, b"old");
            assert_eq!(
                resolver.resolve(name, &buffer, true),
                (Bound(old_ns), LocalName(b"simple"))
            );
            assert_eq!(
                resolver.resolve(name, &buffer, false),
                (Unbound, LocalName(b"simple"))
            );
            assert_eq!(resolver.find(name, &buffer), Bound(old_ns));
        }

        /// Test adding a second level of namespaces, which reset the previous binding
        /// to not bound state by specifying an empty namespace name.
        ///
        /// See <https://www.w3.org/TR/xml-names11/#scoping>
        #[test]
        fn reset() {
            let name = QName(b"simple");
            let old_ns = Namespace(b"old");

            let mut resolver = NamespaceResolver::default();
            let mut buffer = Vec::new();

            resolver.push(&BytesStart::from_content(" xmlns='old'", 0), &mut buffer);
            resolver.push(&BytesStart::from_content(" xmlns=''", 0), &mut buffer);

            assert_eq!(buffer, b"old");
            assert_eq!(
                resolver.resolve(name, &buffer, true),
                (Unbound, LocalName(b"simple"))
            );
            assert_eq!(
                resolver.resolve(name, &buffer, false),
                (Unbound, LocalName(b"simple"))
            );
            assert_eq!(resolver.find(name, &buffer), Unbound);

            resolver.pop(&mut buffer);
            assert_eq!(buffer, b"old");
            assert_eq!(
                resolver.resolve(name, &buffer, true),
                (Bound(old_ns), LocalName(b"simple"))
            );
            assert_eq!(
                resolver.resolve(name, &buffer, false),
                (Unbound, LocalName(b"simple"))
            );
            assert_eq!(resolver.find(name, &buffer), Bound(old_ns));
        }
    }

    mod declared_prefix {
        use super::*;
        use pretty_assertions::assert_eq;

        /// Basic tests that checks that basic resolver functionality is working
        #[test]
        fn basic() {
            let name = QName(b"p:with-declared-prefix");
            let ns = Namespace(b"default");

            let mut resolver = NamespaceResolver::default();
            let mut buffer = Vec::new();

            resolver.push(
                &BytesStart::from_content(" xmlns:p='default'", 0),
                &mut buffer,
            );
            assert_eq!(buffer, b"pdefault");

            // Check that tags without namespaces does not change result
            resolver.push(&BytesStart::from_content("", 0), &mut buffer);
            assert_eq!(buffer, b"pdefault");
            resolver.pop(&mut buffer);

            assert_eq!(buffer, b"pdefault");
            assert_eq!(
                resolver.resolve(name, &buffer, true),
                (Bound(ns), LocalName(b"with-declared-prefix"))
            );
            assert_eq!(
                resolver.resolve(name, &buffer, false),
                (Bound(ns), LocalName(b"with-declared-prefix"))
            );
            assert_eq!(resolver.find(name, &buffer), Bound(ns));
        }

        /// Test adding a second level of namespaces, which replaces the previous binding
        #[test]
        fn override_namespace() {
            let name = QName(b"p:with-declared-prefix");
            let old_ns = Namespace(b"old");
            let new_ns = Namespace(b"new");

            let mut resolver = NamespaceResolver::default();
            let mut buffer = Vec::new();

            resolver.push(&BytesStart::from_content(" xmlns:p='old'", 0), &mut buffer);
            resolver.push(&BytesStart::from_content(" xmlns:p='new'", 0), &mut buffer);

            assert_eq!(buffer, b"poldpnew");
            assert_eq!(
                resolver.resolve(name, &buffer, true),
                (Bound(new_ns), LocalName(b"with-declared-prefix"))
            );
            assert_eq!(
                resolver.resolve(name, &buffer, false),
                (Bound(new_ns), LocalName(b"with-declared-prefix"))
            );
            assert_eq!(resolver.find(name, &buffer), Bound(new_ns));

            resolver.pop(&mut buffer);
            assert_eq!(buffer, b"pold");
            assert_eq!(
                resolver.resolve(name, &buffer, true),
                (Bound(old_ns), LocalName(b"with-declared-prefix"))
            );
            assert_eq!(
                resolver.resolve(name, &buffer, false),
                (Bound(old_ns), LocalName(b"with-declared-prefix"))
            );
            assert_eq!(resolver.find(name, &buffer), Bound(old_ns));
        }

        /// Test adding a second level of namespaces, which reset the previous binding
        /// to not bound state by specifying an empty namespace name.
        ///
        /// See <https://www.w3.org/TR/xml-names11/#scoping>
        #[test]
        fn reset() {
            let name = QName(b"p:with-declared-prefix");
            let old_ns = Namespace(b"old");

            let mut resolver = NamespaceResolver::default();
            let mut buffer = Vec::new();

            resolver.push(&BytesStart::from_content(" xmlns:p='old'", 0), &mut buffer);
            resolver.push(&BytesStart::from_content(" xmlns:p=''", 0), &mut buffer);

            assert_eq!(buffer, b"poldp");
            assert_eq!(
                resolver.resolve(name, &buffer, true),
                (Unknown(b"p".to_vec()), LocalName(b"with-declared-prefix"))
            );
            assert_eq!(
                resolver.resolve(name, &buffer, false),
                (Unknown(b"p".to_vec()), LocalName(b"with-declared-prefix"))
            );
            assert_eq!(resolver.find(name, &buffer), Unknown(b"p".to_vec()));

            resolver.pop(&mut buffer);
            assert_eq!(buffer, b"pold");
            assert_eq!(
                resolver.resolve(name, &buffer, true),
                (Bound(old_ns), LocalName(b"with-declared-prefix"))
            );
            assert_eq!(
                resolver.resolve(name, &buffer, false),
                (Bound(old_ns), LocalName(b"with-declared-prefix"))
            );
            assert_eq!(resolver.find(name, &buffer), Bound(old_ns));
        }
    }

    #[test]
    fn undeclared_prefix() {
        let name = QName(b"unknown:prefix");

        let resolver = NamespaceResolver::default();
        let buffer = Vec::new();

        assert_eq!(buffer, b"");
        assert_eq!(
            resolver.resolve(name, &buffer, true),
            (Unknown(b"unknown".to_vec()), LocalName(b"prefix"))
        );
        assert_eq!(
            resolver.resolve(name, &buffer, false),
            (Unknown(b"unknown".to_vec()), LocalName(b"prefix"))
        );
        assert_eq!(resolver.find(name, &buffer), Unknown(b"unknown".to_vec()));
    }

    /// Checks how the QName is decomposed to a prefix and a local name
    #[test]
    fn prefix_and_local_name() {
        let name = QName(b"foo:bus");
        assert_eq!(name.prefix(), Some(Prefix(b"foo")));
        assert_eq!(name.local_name(), LocalName(b"bus"));
        assert_eq!(name.decompose(), (LocalName(b"bus"), Some(Prefix(b"foo"))));

        let name = QName(b"foo:");
        assert_eq!(name.prefix(), Some(Prefix(b"foo")));
        assert_eq!(name.local_name(), LocalName(b""));
        assert_eq!(name.decompose(), (LocalName(b""), Some(Prefix(b"foo"))));

        let name = QName(b":foo");
        assert_eq!(name.prefix(), Some(Prefix(b"")));
        assert_eq!(name.local_name(), LocalName(b"foo"));
        assert_eq!(name.decompose(), (LocalName(b"foo"), Some(Prefix(b""))));

        let name = QName(b"foo:bus:baz");
        assert_eq!(name.prefix(), Some(Prefix(b"foo")));
        assert_eq!(name.local_name(), LocalName(b"bus:baz"));
        assert_eq!(
            name.decompose(),
            (LocalName(b"bus:baz"), Some(Prefix(b"foo")))
        );
    }
}
