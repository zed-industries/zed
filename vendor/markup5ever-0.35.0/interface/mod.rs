// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! Types for tag and attribute names, and tree-builder functionality.

use std::cell::Ref;
use std::fmt;
use tendril::StrTendril;
use web_atoms::{LocalName, Namespace, Prefix};

pub use self::tree_builder::{create_element, AppendNode, AppendText, ElementFlags, NodeOrText};
pub use self::tree_builder::{ElemName, Tracer, TreeSink};
pub use self::tree_builder::{LimitedQuirks, NoQuirks, Quirks, QuirksMode};

/// An [expanded name], containing the tag and the namespace.
///
/// [expanded name]: https://www.w3.org/TR/REC-xml-names/#dt-expname
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct ExpandedName<'a> {
    pub ns: &'a Namespace,
    pub local: &'a LocalName,
}

impl ElemName for ExpandedName<'_> {
    #[inline(always)]
    fn ns(&self) -> &Namespace {
        self.ns
    }

    #[inline(always)]
    fn local_name(&self) -> &LocalName {
        self.local
    }
}

impl<'a> ElemName for Ref<'a, ExpandedName<'a>> {
    #[inline(always)]
    fn ns(&self) -> &Namespace {
        self.ns
    }

    #[inline(always)]
    fn local_name(&self) -> &LocalName {
        self.local
    }
}

impl fmt::Debug for ExpandedName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.ns.is_empty() {
            write!(f, "{}", self.local)
        } else {
            write!(f, "{{{}}}:{}", self.ns, self.local)
        }
    }
}

#[must_use]
#[derive(Debug, PartialEq)]
pub enum TokenizerResult<Handle> {
    Done,
    Script(Handle),
}

/// Helper to quickly create an expanded name.
///
/// Can be used with no namespace as `expanded_name!("", "some_name")`
/// or with a namespace as `expanded_name!(ns "some_name")`.  In the
/// latter case, `ns` is one of the symbols which the [`ns!`][ns]
/// macro accepts; note the lack of a comma between the `ns` and
/// `"some_name"`.
///
/// [ns]: macro.ns.html
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate markup5ever;
///
/// # fn main() {
/// use markup5ever::ExpandedName;
///
/// assert_eq!(
///     expanded_name!("", "div"),
///     ExpandedName {
///         ns: &ns!(),
///         local: &local_name!("div")
///     }
/// );
///
/// assert_eq!(
///     expanded_name!(html "div"),
///     ExpandedName {
///         ns: &ns!(html),
///         local: &local_name!("div")
///     }
/// );
/// # }
#[macro_export]
macro_rules! expanded_name {
    ("", $local: tt) => {
        $crate::interface::ExpandedName {
            ns: &ns!(),
            local: &local_name!($local),
        }
    };
    ($ns: ident $local: tt) => {
        $crate::interface::ExpandedName {
            ns: &ns!($ns),
            local: &local_name!($local),
        }
    };
}

pub mod tree_builder;

/// A fully qualified name (with a namespace), used to depict names of tags and attributes.
///
/// Namespaces can be used to differentiate between similar XML fragments. For example:
///
/// ```text
/// // HTML
/// <table>
///   <tr>
///     <td>Apples</td>
///     <td>Bananas</td>
///   </tr>
/// </table>
///
/// // Furniture XML
/// <table>
///   <name>African Coffee Table</name>
///   <width>80</width>
///   <length>120</length>
/// </table>
/// ```
///
/// Without XML namespaces, we can't use those two fragments in the same document
/// at the same time. However if we declare a namespace we could instead say:
///
/// ```text
///
/// // Furniture XML
/// <furn:table xmlns:furn="https://furniture.rs">
///   <furn:name>African Coffee Table</furn:name>
///   <furn:width>80</furn:width>
///   <furn:length>120</furn:length>
/// </furn:table>
/// ```
///
/// and bind the prefix `furn` to a different namespace.
///
/// For this reason we parse names that contain a colon in the following way:
///
/// ```text
/// <furn:table>
///    |    |
///    |    +- local name
///    |
///  prefix (when resolved gives namespace_url `https://furniture.rs`)
/// ```
///
/// NOTE: `Prefix`, `LocalName` and `Prefix` all implement `Deref<str>`.
///
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
#[cfg_attr(feature = "heap_size", derive(HeapSizeOf))]
pub struct QualName {
    /// The prefix of qualified (e.g. `furn` in `<furn:table>` above).
    /// Optional (since some namespaces can be empty or inferred), and
    /// only useful for namespace resolution (since different prefix
    /// can still resolve to same namespace)
    ///
    /// ```
    ///
    /// # fn main() {
    /// use markup5ever::{QualName, Namespace, LocalName, Prefix};
    ///
    /// let qual = QualName::new(
    ///     Some(Prefix::from("furn")),
    ///     Namespace::from("https://furniture.rs"),
    ///     LocalName::from("table"),
    /// );
    ///
    /// assert_eq!("furn", &qual.prefix.unwrap());
    ///
    /// # }
    /// ```
    pub prefix: Option<Prefix>,
    /// The namespace after resolution (e.g. `https://furniture.rs` in example above).
    ///
    /// ```
    /// # use markup5ever::{QualName, Namespace, LocalName, Prefix};
    ///
    /// # fn main() {
    /// # let qual = QualName::new(
    /// #    Some(Prefix::from("furn")),
    /// #    Namespace::from("https://furniture.rs"),
    /// #    LocalName::from("table"),
    /// # );
    ///
    /// assert_eq!("https://furniture.rs", &qual.ns);
    /// # }
    /// ```
    ///
    /// When matching namespaces used by HTML we can use `ns!` macro.
    /// Although keep in mind that ns! macro only works with namespaces
    /// that are present in HTML spec (like `html`, `xmlns`, `svg`, etc.).
    ///
    /// ```
    /// #[macro_use] extern crate markup5ever;
    ///
    /// # use markup5ever::{QualName, Namespace, LocalName, Prefix};
    ///
    /// let html_table = QualName::new(
    ///    None,
    ///    ns!(html),
    ///    LocalName::from("table"),
    /// );
    ///
    /// assert!(
    ///   match html_table.ns {
    ///     ns!(html) => true,
    ///     _ => false,
    ///   }
    /// );
    ///
    /// ```
    pub ns: Namespace,
    /// The local name (e.g. `table` in `<furn:table>` above).
    ///
    /// ```
    /// # use markup5ever::{QualName, Namespace, LocalName, Prefix};
    ///
    /// # fn main() {
    /// # let qual = QualName::new(
    /// #    Some(Prefix::from("furn")),
    /// #    Namespace::from("https://furniture.rs"),
    /// #    LocalName::from("table"),
    /// # );
    ///
    /// assert_eq!("table", &qual.local);
    /// # }
    /// ```
    /// When matching local name we can also use the `local_name!` macro:
    ///
    /// ```
    /// #[macro_use] extern crate markup5ever;
    ///
    /// # use markup5ever::{QualName, Namespace, LocalName, Prefix};
    ///
    /// # let qual = QualName::new(
    /// #    Some(Prefix::from("furn")),
    /// #    Namespace::from("https://furniture.rs"),
    /// #    LocalName::from("table"),
    /// # );
    ///
    /// // Initialize qual to furniture example
    ///
    /// assert!(
    ///   match qual.local {
    ///     local_name!("table") => true,
    ///     _ => false,
    ///   }
    /// );
    ///
    /// ```
    pub local: LocalName,
}

impl ElemName for Ref<'_, QualName> {
    #[inline(always)]
    fn ns(&self) -> &Namespace {
        &self.ns
    }

    #[inline(always)]
    fn local_name(&self) -> &LocalName {
        &self.local
    }
}

impl ElemName for &QualName {
    #[inline(always)]
    fn ns(&self) -> &Namespace {
        &self.ns
    }

    #[inline(always)]
    fn local_name(&self) -> &LocalName {
        &self.local
    }
}

impl QualName {
    /// Basic constructor function.
    ///
    /// First let's try it for the following example where `QualName`
    /// is defined as:
    /// ```text
    /// <furn:table> <!-- namespace url is https://furniture.rs -->
    /// ```
    ///
    /// Given this definition, we can define `QualName` using strings.
    ///
    /// ```
    /// use markup5ever::{QualName, Namespace, LocalName, Prefix};
    ///
    /// # fn main() {
    /// let qual_name = QualName::new(
    ///     Some(Prefix::from("furn")),
    ///     Namespace::from("https://furniture.rs"),
    ///     LocalName::from("table"),
    /// );
    /// # }
    /// ```
    ///
    /// If we were instead to construct this element instead:
    ///
    /// ```text
    ///
    /// <table>
    ///  ^^^^^---- no prefix and thus default html namespace
    ///
    /// ```
    ///
    /// Or could define it using macros, like so:
    ///
    /// ```
    /// #[macro_use] extern crate markup5ever;
    /// use markup5ever::{QualName, Namespace, LocalName, Prefix};
    ///
    /// # fn main() {
    /// let qual_name = QualName::new(
    ///     None,
    ///     ns!(html),
    ///     local_name!("table")
    /// );
    /// # }
    /// ```
    ///
    /// Let's analyse the above example.
    /// Since we have no prefix its value is None. Second we have html namespace.
    /// In html5ever html namespaces are supported out of the box,
    /// we can write `ns!(html)` instead of typing `Namespace::from("http://www.w3.org/1999/xhtml")`.
    /// Local name is also one of the HTML elements local names, so can
    /// use `local_name!("table")` macro.
    ///
    #[inline]
    pub fn new(prefix: Option<Prefix>, ns: Namespace, local: LocalName) -> QualName {
        QualName { prefix, ns, local }
    }

    /// Take a reference of `self` as an `ExpandedName`, dropping the unresolved prefix.
    ///
    /// In XML and HTML prefixes are only used to extract the relevant namespace URI.
    /// Expanded name only contains resolved namespace and tag name, which are only
    /// relevant parts of an XML or HTML tag and attribute name respectively.
    ///
    /// In lieu of our XML Namespace example
    ///
    /// ```text
    /// <furn:table> <!-- namespace url is https://furniture.rs -->
    /// ```
    /// For it the expanded name would become roughly equivalent to:
    ///
    /// ```text
    /// ExpandedName {
    ///    ns: "https://furniture.rs",
    ///    local: "table",
    /// }
    /// ```
    ///
    #[inline]
    pub fn expanded(&self) -> ExpandedName<'_> {
        ExpandedName {
            ns: &self.ns,
            local: &self.local,
        }
    }
}

/// A tag attribute, e.g. `class="test"` in `<div class="test" ...>`.
///
/// The namespace on the attribute name is almost always ns!("").
/// The tokenizer creates all attributes this way, but the tree
/// builder will adjust certain attribute names inside foreign
/// content (MathML, SVG).
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Attribute {
    /// The name of the attribute (e.g. the `class` in `<div class="test">`)
    pub name: QualName,
    /// The value of the attribute (e.g. the `"test"` in `<div class="test">`)
    pub value: StrTendril,
}

#[cfg(test)]
mod tests {
    use web_atoms::{ns, Namespace};

    #[test]
    fn ns_macro() {
        assert_eq!(ns!(), Namespace::from(""));

        assert_eq!(ns!(html), Namespace::from("http://www.w3.org/1999/xhtml"));
        assert_eq!(
            ns!(xml),
            Namespace::from("http://www.w3.org/XML/1998/namespace")
        );
        assert_eq!(ns!(xmlns), Namespace::from("http://www.w3.org/2000/xmlns/"));
        assert_eq!(ns!(xlink), Namespace::from("http://www.w3.org/1999/xlink"));
        assert_eq!(ns!(svg), Namespace::from("http://www.w3.org/2000/svg"));
        assert_eq!(
            ns!(mathml),
            Namespace::from("http://www.w3.org/1998/Math/MathML")
        );
    }
}
