//! Contains XML qualified names manipulation types and functions.

use std::fmt;
use std::str::FromStr;

use crate::namespace::NS_NO_PREFIX;

/// Represents a qualified XML name.
///
/// A qualified name always consists at least of a local name. It can optionally contain
/// a prefix; when reading an XML document, if it contains a prefix, it must also contain a
/// namespace URI, but this is not enforced statically; see below. The name can contain a
/// namespace without a prefix; in that case a default, empty prefix is assumed.
///
/// When writing XML documents, it is possible to omit the namespace URI, leaving only
/// the prefix. In this case the writer will check that the specifed prefix is bound to some
/// URI in the current namespace context. If both prefix and namespace URI are specified,
/// it is checked that the current namespace context contains this exact correspondence
/// between prefix and namespace URI.
///
/// # Prefixes and URIs
///
/// A qualified name with a prefix must always contain a proper namespace URI --- names with
/// a prefix but without a namespace associated with that prefix are meaningless. However,
/// it is impossible to obtain proper namespace URI by a prefix without a context, and such
/// context is only available when parsing a document (or it can be constructed manually
/// when writing a document). Tying a name to a context statically seems impractical. This
/// may change in future, though.
///
/// # Conversions
///
/// `Name` implements some `From` instances for conversion from strings and tuples. For example:
///
/// ```rust
/// # use xml::name::Name;
/// let n1: Name = "p:some-name".into();
/// let n2: Name = ("p", "some-name").into();
///
/// assert_eq!(n1, n2);
/// assert_eq!(n1.local_name, "some-name");
/// assert_eq!(n1.prefix, Some("p"));
/// assert!(n1.namespace.is_none());
/// ```
///
/// This is added to support easy specification of XML elements when writing XML documents.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Name<'a> {
    /// A local name, e.g. `string` in `xsi:string`.
    pub local_name: &'a str,

    /// A namespace URI, e.g. `http://www.w3.org/2000/xmlns/`.
    pub namespace: Option<&'a str>,

    /// A name prefix, e.g. `xsi` in `xsi:string`.
    pub prefix: Option<&'a str>,
}

impl<'a> From<&'a str> for Name<'a> {
    fn from(s: &'a str) -> Self {
        if let Some((prefix, name)) = s.split_once(':') {
            Name::prefixed(name, prefix)
        } else {
            Name::local(s)
        }
    }
}

impl<'a> From<(&'a str, &'a str)> for Name<'a> {
    fn from((prefix, name): (&'a str, &'a str)) -> Self {
        Name::prefixed(name, prefix)
    }
}

impl fmt::Display for Name<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(namespace) = self.namespace {
            write!(f, "{{{namespace}}}")?;
        }

        if let Some(prefix) = self.prefix {
            write!(f, "{prefix}:")?;
        }

        f.write_str(self.local_name)
    }
}

impl<'a> Name<'a> {
    /// Returns an owned variant of the qualified name.
    #[must_use]
    pub fn to_owned(&self) -> OwnedName {
        OwnedName {
            local_name: self.local_name.into(),
            namespace: self.namespace.map(std::convert::Into::into),
            prefix: self.prefix.map(std::convert::Into::into),
        }
    }

    /// Returns a new `Name` instance representing plain local name.
    #[inline]
    #[must_use]
    pub const fn local(local_name: &str) -> Name<'_> {
        Name {
            local_name,
            prefix: None,
            namespace: None,
        }
    }

    /// Returns a new `Name` instance with the given local name and prefix.
    #[inline]
    #[must_use]
    pub const fn prefixed(local_name: &'a str, prefix: &'a str) -> Self {
        Name {
            local_name,
            namespace: None,
            prefix: Some(prefix),
        }
    }

    /// Returns a new `Name` instance representing a qualified name with or without a prefix and
    /// with a namespace URI.
    #[inline]
    #[must_use]
    pub const fn qualified(local_name: &'a str, namespace: &'a str, prefix: Option<&'a str>) -> Self {
        Name {
            local_name,
            namespace: Some(namespace),
            prefix,
        }
    }

    /// Returns a correct XML representation of this local name and prefix.
    ///
    /// This method is different from the autoimplemented `to_string()` because it does not
    /// include namespace URI in the result.
    #[must_use]
    pub fn to_repr(&self) -> String {
        self.repr_display().to_string()
    }

    /// Returns a structure which can be displayed with `std::fmt` machinery to obtain this
    /// local name and prefix.
    ///
    /// This method is needed for efficiency purposes in order not to create unnecessary
    /// allocations.
    #[inline]
    #[must_use]
    pub const fn repr_display(&self) -> ReprDisplay<'_, '_> {
        ReprDisplay(self)
    }

    /// Returns either a prefix of this name or `namespace::NS_NO_PREFIX` constant.
    #[inline]
    #[must_use]
    pub fn prefix_repr(&self) -> &str {
        self.prefix.unwrap_or(NS_NO_PREFIX)
    }
}

/// A wrapper around `Name` whose `Display` implementation prints the wrapped name as it is
/// displayed in an XML document.
pub struct ReprDisplay<'a, 'b>(&'a Name<'b>);

impl<'a, 'b: 'a> fmt::Display for ReprDisplay<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.prefix {
            Some(prefix) => write!(f, "{}:{}", prefix, self.0.local_name),
            None => self.0.local_name.fmt(f),
        }
    }
}

/// An owned variant of `Name`.
///
/// Everything about `Name` applies to this structure as well.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct OwnedName {
    /// A local name, e.g. `string` in `xsi:string`.
    pub local_name: String,

    /// A namespace URI, e.g. `http://www.w3.org/2000/xmlns/`.
    pub namespace: Option<String>,

    /// A name prefix, e.g. `xsi` in `xsi:string`.
    pub prefix: Option<String>,
}

impl fmt::Display for OwnedName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.borrow(), f)
    }
}

impl OwnedName {
    /// Constructs a borrowed `Name` based on this owned name.
    #[must_use]
    #[inline]
    pub fn borrow(&self) -> Name<'_> {
        Name {
            local_name: &self.local_name,
            namespace: self.namespace.as_deref(),
            prefix: self.prefix.as_deref(),
        }
    }

    /// Returns a new `OwnedName` instance representing a plain local name.
    #[inline]
    pub fn local<S>(local_name: S) -> Self where S: Into<String> {
        Self {
            local_name: local_name.into(),
            namespace: None,
            prefix: None,
        }
    }

    /// Returns a new `OwnedName` instance representing a qualified name with or without
    /// a prefix and with a namespace URI.
    #[inline]
    pub fn qualified<S1, S2, S3>(local_name: S1, namespace: S2, prefix: Option<S3>) -> Self
        where S1: Into<String>, S2: Into<String>, S3: Into<String>
    {
        Self {
            local_name: local_name.into(),
            namespace: Some(namespace.into()),
            prefix: prefix.map(std::convert::Into::into),
        }
    }

    /// Returns an optional prefix by reference, equivalent to `self.borrow().prefix`
    /// but avoids extra work.
    #[inline]
    #[must_use]
    pub fn prefix_ref(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    /// Returns an optional namespace by reference, equivalen to `self.borrow().namespace`
    /// but avoids extra work.
    #[inline]
    #[must_use]
    pub fn namespace_ref(&self) -> Option<&str> {
        self.namespace.as_deref()
    }
}

impl<'a> From<Name<'a>> for OwnedName {
    #[inline]
    fn from(n: Name<'a>) -> Self {
        n.to_owned()
    }
}

impl FromStr for OwnedName {
    type Err = ();

    /// Parses the given string slice into a qualified name.
    ///
    /// This function, when finishes sucessfully, always return a qualified
    /// name without a namespace (`name.namespace == None`). It should be filled later
    /// using proper `NamespaceStack`.
    ///
    /// It is supposed that all characters in the argument string are correct
    /// as defined by the XML specification. No additional checks except a check
    /// for emptiness are done.
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut it = s.split(':');

        let r = match (it.next(), it.next(), it.next()) {
            (Some(prefix), Some(local_name), None) if !prefix.is_empty() &&
                                                      !local_name.is_empty() =>
                Some((local_name.into(), Some(prefix.into()))),
            (Some(local_name), None, None) if !local_name.is_empty() =>
                Some((local_name.into(), None)),
            (_, _, _) => None
        };
        r.map(|(local_name, prefix)| Self {
            local_name,
            namespace: None,
            prefix
        }).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::OwnedName;

    #[test]
    fn test_owned_name_from_str() {
        assert_eq!("prefix:name".parse(), Ok(OwnedName {
            local_name: "name".into(),
            namespace: None,
            prefix: Some("prefix".into())
        }));

        assert_eq!("name".parse(), Ok(OwnedName {
            local_name: "name".into(),
            namespace: None,
            prefix: None
        }));

        assert_eq!("".parse(), Err::<OwnedName, ()>(()));
        assert_eq!(":".parse(), Err::<OwnedName, ()>(()));
        assert_eq!(":a".parse(), Err::<OwnedName, ()>(()));
        assert_eq!("a:".parse(), Err::<OwnedName, ()>(()));
        assert_eq!("a:b:c".parse(), Err::<OwnedName, ()>(()));
    }
}
