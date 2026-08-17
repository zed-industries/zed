//! Contains XML attributes manipulation types and functions.

use std::fmt;

use crate::escape::{AttributeEscapes, Escaped};
use crate::name::{Name, OwnedName};

/// A borrowed version of an XML attribute.
///
/// Consists of a borrowed qualified name and a borrowed string value.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Attribute<'a> {
    /// Attribute name.
    pub name: Name<'a>,

    /// Attribute value.
    pub value: &'a str,
}

impl fmt::Display for Attribute<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}=\"{}\"", self.name, Escaped::<AttributeEscapes>::new(self.value))
    }
}

impl<'a> Attribute<'a> {
    /// Creates an owned attribute out of this borrowed one.
    #[inline]
    #[must_use]
    pub fn to_owned(&self) -> OwnedAttribute {
        OwnedAttribute {
            name: self.name.into(),
            value: self.value.into(),
        }
    }

    /// Creates a borrowed attribute using the provided borrowed name and a borrowed string value.
    #[inline]
    #[must_use]
    pub const fn new(name: Name<'a>, value: &'a str) -> Self {
        Attribute { name, value }
    }
}

/// An owned version of an XML attribute.
///
/// Consists of an owned qualified name and an owned string value.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct OwnedAttribute {
    /// Attribute name.
    pub name: OwnedName,

    /// Attribute value.
    pub value: String,
}

impl OwnedAttribute {
    /// Returns a borrowed `Attribute` out of this owned one.
    #[must_use]
    #[inline]
    pub fn borrow(&self) -> Attribute<'_> {
        Attribute {
            name: self.name.borrow(),
            value: &self.value,
        }
    }

    /// Creates a new owned attribute using the provided owned name and an owned string value.
    #[inline]
    pub fn new<S: Into<String>>(name: OwnedName, value: S) -> Self {
        Self { name, value: value.into() }
    }
}

impl fmt::Display for OwnedAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}=\"{}\"", self.name, Escaped::<AttributeEscapes>::new(&self.value))
    }
}

#[cfg(test)]
mod tests {
    use super::Attribute;

    use crate::name::Name;

    #[test]
    fn attribute_display() {
        let attr = Attribute::new(
            Name::qualified("attribute", "urn:namespace", Some("n")),
            "its value with > & \" ' < weird symbols",
        );

        assert_eq!(
            &*attr.to_string(),
            "{urn:namespace}n:attribute=\"its value with &gt; &amp; &quot; &apos; &lt; weird symbols\""
        );
    }
}
