//! Contains namespace manipulation types and functions.

use std::borrow::Cow;
use std::collections::btree_map::Iter as Entries;
use std::collections::btree_map::{BTreeMap, Entry};
use std::collections::HashSet;
use std::iter::{Map, Rev};
use std::slice::Iter;

/// Designates prefix for namespace definitions.
///
/// See [Namespaces in XML][namespace] spec for more information.
///
///   [namespace]: http://www.w3.org/TR/xml-names/#ns-decl
pub const NS_XMLNS_PREFIX: &str = "xmlns";

/// Designates the standard URI for `xmlns` prefix.
///
/// See [A Namespace Name for xmlns Attributes][namespace] for more information.
///
///   [namespace]: http://www.w3.org/2000/xmlns/
pub const NS_XMLNS_URI: &str = "http://www.w3.org/2000/xmlns/";

/// Designates prefix for a namespace containing several special predefined attributes.
///
/// See [2.10 White Space handling][1],  [2.1 Language Identification][2],
/// [XML Base specification][3] and [xml:id specification][4] for more information.
///
///   [1]: http://www.w3.org/TR/REC-xml/#sec-white-space
///   [2]: http://www.w3.org/TR/REC-xml/#sec-lang-tag
///   [3]: http://www.w3.org/TR/xmlbase/
///   [4]: http://www.w3.org/TR/xml-id/
pub const NS_XML_PREFIX: &str = "xml";

/// Designates the standard URI for `xml` prefix.
///
/// See `NS_XML_PREFIX` documentation for more information.
pub const NS_XML_URI: &str = "http://www.w3.org/XML/1998/namespace";

/// Designates the absence of prefix in a qualified name.
///
/// This constant should be used to define or query default namespace which should be used
/// for element or attribute names without prefix. For example, if a namespace mapping
/// at a particular point in the document contains correspondence like
///
/// ```none
///   NS_NO_PREFIX  -->  urn:some:namespace
/// ```
///
/// then all names declared without an explicit prefix `urn:some:namespace` is assumed as
/// a namespace URI.
///
/// By default empty prefix corresponds to absence of namespace, but this can change either
/// when writing an XML document (manually) or when reading an XML document (based on namespace
/// declarations).
pub const NS_NO_PREFIX: &str = "";

/// Designates an empty namespace URI, which is equivalent to absence of namespace.
///
/// This constant should not usually be used directly; it is used to designate that
/// empty prefix corresponds to absent namespace in `NamespaceStack` instances created with
/// `NamespaceStack::default()`. Therefore, it can be used to restore `NS_NO_PREFIX` mapping
/// in a namespace back to its default value.
pub const NS_EMPTY_URI: &str = "";

/// Namespace is a map from prefixes to namespace URIs.
///
/// No prefix (i.e. default namespace) is designated by `NS_NO_PREFIX` constant.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Namespace(pub BTreeMap<String, String>);

impl Namespace {
    /// Returns an empty namespace.
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Self(BTreeMap::new())
    }

    /// Checks whether this namespace is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Checks whether this namespace is essentially empty, that is, it does not contain
    /// anything but default mappings.
    #[must_use]
    pub fn is_essentially_empty(&self) -> bool {
        // a shortcut for a namespace which is definitely not empty
        if self.0.len() > 3 { return false; }

        self.0.iter().all(|(k, v)| matches!((&**k, &**v),
            (NS_NO_PREFIX,    NS_EMPTY_URI) |
            (NS_XMLNS_PREFIX, NS_XMLNS_URI) |
            (NS_XML_PREFIX,   NS_XML_URI))
        )
    }

    /// Checks whether this namespace mapping contains the given prefix.
    ///
    /// # Parameters
    /// * `prefix`  --- namespace prefix.
    ///
    /// # Return value
    /// `true` if this namespace contains the given prefix, `false` otherwise.
    #[inline]
    pub fn contains<P: ?Sized + AsRef<str>>(&self, prefix: &P) -> bool {
        self.0.contains_key(prefix.as_ref())
    }

    /// Puts a mapping into this namespace.
    ///
    /// This method does not override any already existing mappings.
    ///
    /// Returns a boolean flag indicating whether the map already contained
    /// the given prefix.
    ///
    /// # Parameters
    /// * `prefix` --- namespace prefix;
    /// * `uri`    --- namespace URI.
    ///
    /// # Return value
    /// `true` if `prefix` has been inserted successfully; `false` if the `prefix`
    /// was already present in the namespace.
    pub fn put<P, U>(&mut self, prefix: P, uri: U) -> bool
        where P: Into<String>, U: Into<String>
    {
        match self.0.entry(prefix.into()) {
            Entry::Occupied(_) => false,
            Entry::Vacant(ve) => {
                ve.insert(uri.into());
                true
            },
        }
    }

    /// Puts a mapping into this namespace forcefully.
    ///
    /// This method, unlike `put()`, does replace an already existing mapping.
    ///
    /// Returns previous URI which was assigned to the given prefix, if it is present.
    ///
    /// # Parameters
    /// * `prefix` --- namespace prefix;
    /// * `uri`    --- namespace URI.
    ///
    /// # Return value
    /// `Some(uri)` with `uri` being a previous URI assigned to the `prefix`, or
    /// `None` if such prefix was not present in the namespace before.
    pub fn force_put<P, U>(&mut self, prefix: P, uri: U) -> Option<String>
        where P: Into<String>, U: Into<String>
    {
        self.0.insert(prefix.into(), uri.into())
    }

    /// Queries the namespace for the given prefix.
    ///
    /// # Parameters
    /// * `prefix` --- namespace prefix.
    ///
    /// # Return value
    /// Namespace URI corresponding to the given prefix, if it is present.
    pub fn get<'a, P: ?Sized + AsRef<str>>(&'a self, prefix: &P) -> Option<&'a str> {
        self.0.get(prefix.as_ref()).map(|s| &**s)
    }

    /// Borrowed namespace for the writer
    #[must_use]
    pub const fn borrow(&self) -> Cow<'_, Self> {
        Cow::Borrowed(self)
    }

    /// Namespace mappings contained in a namespace.
    pub fn iter(&self) -> NamespaceMappings<'_> {
        self.into_iter()
    }
}

/// An alias for iterator type for namespace mappings contained in a namespace.
pub type NamespaceMappings<'a> = Map<
    Entries<'a, String, String>,
    for<'b> fn((&'b String, &'b String)) -> UriMapping<'b>
>;

impl<'a> IntoIterator for &'a Namespace {
    type IntoIter = NamespaceMappings<'a>;
    type Item = UriMapping<'a>;

    fn into_iter(self) -> Self::IntoIter {
        fn mapper<'a>((prefix, uri): (&'a String, &'a String)) -> UriMapping<'a> {
            (prefix, uri)
        }
        self.0.iter().map(mapper)
    }
}

/// Namespace stack is a sequence of namespaces.
///
/// Namespace stack is used to represent cumulative namespace consisting of
/// combined namespaces from nested elements.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct NamespaceStack(pub Vec<Namespace>);

impl NamespaceStack {
    /// Returns an empty namespace stack.
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Self(Vec::with_capacity(2))
    }

    /// Returns a namespace stack with default items in it.
    ///
    /// Default items are the following:
    ///
    /// * `xml` → `http://www.w3.org/XML/1998/namespace`;
    /// * `xmlns` → `http://www.w3.org/2000/xmlns/`.
    #[inline]
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        let mut nst = Self::empty();
        nst.push_empty();
        // xml namespace
        nst.put(NS_XML_PREFIX, NS_XML_URI);
        // xmlns namespace
        nst.put(NS_XMLNS_PREFIX, NS_XMLNS_URI);
        // empty namespace
        nst.put(NS_NO_PREFIX, NS_EMPTY_URI);
        nst
    }

    /// Adds an empty namespace to the top of this stack.
    #[inline]
    pub fn push_empty(&mut self) -> &mut Self {
        self.0.push(Namespace::empty());
        self
    }

    /// Removes the topmost namespace in this stack.
    ///
    /// Panics if the stack is empty.
    #[inline]
    #[track_caller]
    pub fn pop(&mut self) -> Namespace {
        self.0.pop().unwrap()
    }

    /// Removes the topmost namespace in this stack.
    ///
    /// Returns `Some(namespace)` if this stack is not empty and `None` otherwise.
    #[inline]
    pub fn try_pop(&mut self) -> Option<Namespace> {
        self.0.pop()
    }

    /// Borrows the topmost namespace mutably, leaving the stack intact.
    ///
    /// Panics if the stack is empty.
    #[inline]
    #[track_caller]
    pub fn peek_mut(&mut self) -> &mut Namespace {
        self.0.last_mut().unwrap()
    }

    /// Borrows the topmost namespace immutably, leaving the stack intact.
    ///
    /// Panics if the stack is empty.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn peek(&self) -> &Namespace {
        self.0.last().unwrap()
    }

    /// Puts a mapping into the topmost namespace if this stack does not already contain one.
    ///
    /// Returns a boolean flag indicating whether the insertion has completed successfully.
    /// Note that both key and value are matched and the mapping is inserted if either
    /// namespace prefix is not already mapped, or if it is mapped, but to a different URI.
    ///
    /// # Parameters
    /// * `prefix` --- namespace prefix;
    /// * `uri`    --- namespace URI.
    ///
    /// # Return value
    /// `true` if `prefix` has been inserted successfully; `false` if the `prefix`
    /// was already present in the namespace stack.
    pub fn put_checked<P, U>(&mut self, prefix: P, uri: U) -> bool
        where P: Into<String> + AsRef<str>,
              U: Into<String> + AsRef<str>
    {
        if self.0.iter().any(|ns| ns.get(&prefix) == Some(uri.as_ref())) {
            false
        } else {
            self.put(prefix, uri);
            true
        }
    }

    /// Puts a mapping into the topmost namespace in this stack.
    ///
    /// This method does not override a mapping in the topmost namespace if it is
    /// already present, however, it does not depend on other namespaces in the stack,
    /// so it is possible to put a mapping which is present in lower namespaces.
    ///
    /// Returns a boolean flag indicating whether the insertion has completed successfully.
    ///
    /// # Parameters
    /// * `prefix` --- namespace prefix;
    /// * `uri`    --- namespace URI.
    ///
    /// # Return value
    /// `true` if `prefix` has been inserted successfully; `false` if the `prefix`
    /// was already present in the namespace.
    #[inline]
    pub fn put<P, U>(&mut self, prefix: P, uri: U) -> bool
        where P: Into<String>, U: Into<String>
    {
        if let Some(ns) = self.0.last_mut() {
            ns.put(prefix, uri)
        } else {
            false
        }
    }

    /// Performs a search for the given prefix in the whole stack.
    ///
    /// This method walks the stack from top to bottom, querying each namespace
    /// in order for the given prefix. If none of the namespaces contains the prefix,
    /// `None` is returned.
    ///
    /// # Parameters
    /// * `prefix` --- namespace prefix.
    #[inline]
    pub fn get<'a, P: ?Sized + AsRef<str>>(&'a self, prefix: &P) -> Option<&'a str> {
        let prefix = prefix.as_ref();
        for ns in self.0.iter().rev() {
            match ns.get(prefix) {
                None => {},
                r => return r,
            }
        }
        None
    }

    /// Combines this stack of namespaces into a single namespace.
    ///
    /// Namespaces are combined in left-to-right order, that is, rightmost namespace
    /// elements take priority over leftmost ones.
    #[must_use]
    pub fn squash(&self) -> Namespace {
        let mut result = BTreeMap::new();
        for ns in &self.0 {
            result.extend(ns.0.iter().map(|(k, v)| (k.clone(), v.clone())));
        }
        Namespace(result)
    }

    /// Returns an object which implements `Extend` using `put_checked()` instead of `put()`.
    ///
    /// See `CheckedTarget` for more information.
    #[inline]
    pub fn checked_target(&mut self) -> CheckedTarget<'_> {
        CheckedTarget(self)
    }

    /// Returns an iterator over all mappings in this namespace stack.
    #[inline]
    #[must_use]
    pub fn iter(&self) -> NamespaceStackMappings<'_> {
        self.into_iter()
    }
}

/// An iterator over mappings from prefixes to URIs in a namespace stack.
///
/// # Example
/// ```
/// # use xml::namespace::NamespaceStack;
/// let mut nst = NamespaceStack::empty();
/// nst.push_empty();
/// nst.put("a", "urn:A");
/// nst.put("b", "urn:B");
/// nst.push_empty();
/// nst.put("c", "urn:C");
///
/// assert_eq!(vec![("c", "urn:C"), ("a", "urn:A"), ("b", "urn:B")], nst.iter().collect::<Vec<_>>());
/// ```
pub struct NamespaceStackMappings<'a> {
    namespaces: Rev<Iter<'a, Namespace>>,
    current_namespace: Option<NamespaceMappings<'a>>,
    used_keys: HashSet<&'a str>,
}

impl NamespaceStackMappings<'_> {
    fn go_to_next_namespace(&mut self) -> bool {
        self.current_namespace = self.namespaces.next().map(|ns| ns.into_iter());
        self.current_namespace.is_some()
    }
}

impl<'a> Iterator for NamespaceStackMappings<'a> {
    type Item = UriMapping<'a>;

    fn next(&mut self) -> Option<UriMapping<'a>> {
        // If there is no current namespace and no next namespace, we're finished
        if self.current_namespace.is_none() && !self.go_to_next_namespace() {
            return None;
        }
        let next_item = self.current_namespace.as_mut()?.next();

        match next_item {
            // There is an element in the current namespace
            Some((k, v)) => if self.used_keys.contains(&k) {
                // If the current key is used, go to the next one
                self.next()
            } else {
                // Otherwise insert the current key to the set of used keys and
                // return the mapping
                self.used_keys.insert(k);
                Some((k, v))
            },
            // Current namespace is exhausted
            None => if self.go_to_next_namespace() {
                // If there is next namespace, continue from it
                self.next()
            } else {
                // No next namespace, exiting
                None
            }
        }
    }
}

impl<'a> IntoIterator for &'a NamespaceStack {
    type IntoIter = NamespaceStackMappings<'a>;
    type Item = UriMapping<'a>;

    fn into_iter(self) -> Self::IntoIter {
        NamespaceStackMappings {
            namespaces: self.0.iter().rev(),
            current_namespace: None,
            used_keys: HashSet::new(),
        }
    }
}

/// A type alias for a pair of `(prefix, uri)` values returned by namespace iterators.
pub type UriMapping<'a> = (&'a str, &'a str);

impl<'a> Extend<UriMapping<'a>> for Namespace {
    fn extend<T>(&mut self, iterable: T) where T: IntoIterator<Item=UriMapping<'a>> {
        for (prefix, uri) in iterable {
            self.put(prefix, uri);
        }
    }
}

impl<'a> Extend<UriMapping<'a>> for NamespaceStack {
    fn extend<T>(&mut self, iterable: T) where T: IntoIterator<Item=UriMapping<'a>> {
        for (prefix, uri) in iterable {
            self.put(prefix, uri);
        }
    }
}

/// A wrapper around `NamespaceStack` which implements `Extend` using `put_checked()`.
///
/// # Example
///
/// ```
/// # use xml::namespace::NamespaceStack;
///
/// let mut nst = NamespaceStack::empty();
/// nst.push_empty();
/// nst.put("a", "urn:A");
/// nst.put("b", "urn:B");
/// nst.push_empty();
/// nst.put("c", "urn:C");
///
/// nst.checked_target().extend(vec![("a", "urn:Z"), ("b", "urn:B"), ("c", "urn:Y"), ("d", "urn:D")]);
/// assert_eq!(
///     vec![("a", "urn:Z"), ("c", "urn:C"), ("d", "urn:D"), ("b", "urn:B")],
///     nst.iter().collect::<Vec<_>>()
/// );
/// ```
///
/// Compare:
///
/// ```
/// # use xml::namespace::NamespaceStack;
/// # let mut nst = NamespaceStack::empty();
/// # nst.push_empty();
/// # nst.put("a", "urn:A");
/// # nst.put("b", "urn:B");
/// # nst.push_empty();
/// # nst.put("c", "urn:C");
///
/// nst.extend(vec![("a", "urn:Z"), ("b", "urn:B"), ("c", "urn:Y"), ("d", "urn:D")]);
/// assert_eq!(
///     vec![("a", "urn:Z"), ("b", "urn:B"), ("c", "urn:C"), ("d", "urn:D")],
///     nst.iter().collect::<Vec<_>>()
/// );
/// ```
pub struct CheckedTarget<'a>(&'a mut NamespaceStack);

impl<'b> Extend<UriMapping<'b>> for CheckedTarget<'_> {
    fn extend<T>(&mut self, iterable: T) where T: IntoIterator<Item=UriMapping<'b>> {
        for (prefix, uri) in iterable {
            self.0.put_checked(prefix, uri);
        }
    }
}
