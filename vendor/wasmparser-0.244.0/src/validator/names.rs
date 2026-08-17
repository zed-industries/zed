//! Definitions of name-related helpers and newtypes, primarily for the
//! component model.

use crate::prelude::*;
use crate::{Result, WasmFeatures};
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::Deref;
use semver::Version;

/// Represents a kebab string slice used in validation.
///
/// This is a wrapper around `str` that ensures the slice is
/// a valid kebab case string according to the component model
/// specification.
///
/// It also provides an equality and hashing implementation
/// that ignores ASCII case.
#[derive(Debug, Eq)]
#[repr(transparent)]
pub struct KebabStr(str);

impl KebabStr {
    /// Creates a new kebab string slice.
    ///
    /// Returns `None` if the given string is not a valid kebab string.
    pub fn new<'a>(s: impl AsRef<str> + 'a) -> Option<&'a Self> {
        let s = Self::new_unchecked(s);
        if s.is_kebab_case() { Some(s) } else { None }
    }

    pub(crate) fn new_unchecked<'a>(s: impl AsRef<str> + 'a) -> &'a Self {
        // Safety: `KebabStr` is a transparent wrapper around `str`
        // Therefore transmuting `&str` to `&KebabStr` is safe.
        #[allow(unsafe_code)]
        unsafe {
            core::mem::transmute::<_, &Self>(s.as_ref())
        }
    }

    /// Gets the underlying string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts the slice to an owned string.
    pub fn to_kebab_string(&self) -> KebabString {
        KebabString(self.to_string())
    }

    fn is_kebab_case(&self) -> bool {
        let mut lower = false;
        let mut upper = false;
        let mut is_first = true;
        let mut has_digit = false;
        for c in self.chars() {
            match c {
                'a'..='z' if !lower && !upper => lower = true,
                'A'..='Z' if !lower && !upper => upper = true,
                '0'..='9' if !lower && !upper && !is_first => has_digit = true,
                'a'..='z' if lower => {}
                'A'..='Z' if upper => {}
                '0'..='9' if lower || upper => has_digit = true,
                '-' if lower || upper || has_digit => {
                    lower = false;
                    upper = false;
                    is_first = false;
                    has_digit = false;
                }
                _ => return false,
            }
        }

        !self.is_empty() && !self.ends_with('-')
    }
}

impl Deref for KebabStr {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq for KebabStr {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.chars()
            .zip(other.chars())
            .all(|(a, b)| a.to_ascii_lowercase() == b.to_ascii_lowercase())
    }
}

impl PartialEq<KebabString> for KebabStr {
    fn eq(&self, other: &KebabString) -> bool {
        self.eq(other.as_kebab_str())
    }
}

impl Ord for KebabStr {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_chars = self.chars().map(|c| c.to_ascii_lowercase());
        let other_chars = other.chars().map(|c| c.to_ascii_lowercase());
        self_chars.cmp(other_chars)
    }
}

impl PartialOrd for KebabStr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for KebabStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);

        for b in self.chars() {
            b.to_ascii_lowercase().hash(state);
        }
    }
}

impl fmt::Display for KebabStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self as &str).fmt(f)
    }
}

impl ToOwned for KebabStr {
    type Owned = KebabString;

    fn to_owned(&self) -> Self::Owned {
        self.to_kebab_string()
    }
}

/// Represents an owned kebab string for validation.
///
/// This is a wrapper around `String` that ensures the string is
/// a valid kebab case string according to the component model
/// specification.
///
/// It also provides an equality and hashing implementation
/// that ignores ASCII case.
#[derive(Debug, Clone, Eq)]
pub struct KebabString(String);

impl KebabString {
    /// Creates a new kebab string.
    ///
    /// Returns `None` if the given string is not a valid kebab string.
    pub fn new(s: impl Into<String>) -> Option<Self> {
        let s = s.into();
        if KebabStr::new(&s).is_some() {
            Some(Self(s))
        } else {
            None
        }
    }

    /// Gets the underlying string.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Converts the kebab string to a kebab string slice.
    pub fn as_kebab_str(&self) -> &KebabStr {
        // Safety: internal string is always valid kebab-case
        KebabStr::new_unchecked(self.as_str())
    }
}

impl Deref for KebabString {
    type Target = KebabStr;

    fn deref(&self) -> &Self::Target {
        self.as_kebab_str()
    }
}

impl Borrow<KebabStr> for KebabString {
    fn borrow(&self) -> &KebabStr {
        self.as_kebab_str()
    }
}

impl Ord for KebabString {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_kebab_str().cmp(other.as_kebab_str())
    }
}

impl PartialOrd for KebabString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_kebab_str().partial_cmp(other.as_kebab_str())
    }
}

impl PartialEq for KebabString {
    fn eq(&self, other: &Self) -> bool {
        self.as_kebab_str().eq(other.as_kebab_str())
    }
}

impl PartialEq<KebabStr> for KebabString {
    fn eq(&self, other: &KebabStr) -> bool {
        self.as_kebab_str().eq(other)
    }
}

impl Hash for KebabString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_kebab_str().hash(state)
    }
}

impl fmt::Display for KebabString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_kebab_str().fmt(f)
    }
}

impl From<KebabString> for String {
    fn from(s: KebabString) -> String {
        s.0
    }
}

/// An import or export name in the component model which is backed by `T`,
/// which defaults to `String`.
///
/// This name can be either:
///
/// * a plain label or "kebab string": `a-b-c`
/// * a plain method name : `[method]a-b.c-d`
/// * a plain static method name : `[static]a-b.c-d`
/// * a plain constructor: `[constructor]a-b`
/// * an interface name: `wasi:cli/reactor@0.1.0`
/// * a dependency name: `locked-dep=foo:bar/baz`
/// * a URL name: `url=https://..`
/// * a hash name: `integrity=sha256:...`
///
/// # Equality and hashing
///
/// Note that this type the `[method]...` and `[static]...` variants are
/// considered equal and hash to the same value. This enables disallowing
/// clashes between the two where method name overlap cannot happen.
#[derive(Clone)]
pub struct ComponentName {
    raw: String,
    kind: ParsedComponentNameKind,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ParsedComponentNameKind {
    Label,
    Constructor,
    Method,
    Static,
    Interface,
    Dependency,
    Url,
    Hash,
}

/// Created via [`ComponentName::kind`] and classifies a name.
#[derive(Debug, Clone)]
pub enum ComponentNameKind<'a> {
    /// `a-b-c`
    Label(&'a KebabStr),
    /// `[constructor]a-b`
    Constructor(&'a KebabStr),
    /// `[method]a-b.c-d`
    #[allow(missing_docs)]
    Method(ResourceFunc<'a>),
    /// `[static]a-b.c-d`
    #[allow(missing_docs)]
    Static(ResourceFunc<'a>),
    /// `wasi:http/types@2.0`
    #[allow(missing_docs)]
    Interface(InterfaceName<'a>),
    /// `locked-dep=foo:bar/baz`
    #[allow(missing_docs)]
    Dependency(DependencyName<'a>),
    /// `url=https://...`
    #[allow(missing_docs)]
    Url(UrlName<'a>),
    /// `integrity=sha256:...`
    #[allow(missing_docs)]
    Hash(HashName<'a>),
}

const CONSTRUCTOR: &str = "[constructor]";
const METHOD: &str = "[method]";
const STATIC: &str = "[static]";

impl ComponentName {
    /// Attempts to parse `name` as a valid component name, returning `Err` if
    /// it's not valid.
    pub fn new(name: &str, offset: usize) -> Result<ComponentName> {
        Self::new_with_features(name, offset, WasmFeatures::default())
    }

    /// Attempts to parse `name` as a valid component name, returning `Err` if
    /// it's not valid.
    ///
    /// `features` can be used to enable or disable validation of certain forms
    /// of supported import names.
    pub fn new_with_features(name: &str, offset: usize, features: WasmFeatures) -> Result<Self> {
        let mut parser = ComponentNameParser {
            next: name,
            offset,
            features,
        };
        let kind = parser.parse()?;
        if !parser.next.is_empty() {
            bail!(offset, "trailing characters found: `{}`", parser.next);
        }
        Ok(ComponentName {
            raw: name.to_string(),
            kind,
        })
    }

    /// Returns the [`ComponentNameKind`] corresponding to this name.
    pub fn kind(&self) -> ComponentNameKind<'_> {
        use ComponentNameKind::*;
        use ParsedComponentNameKind as PK;
        match self.kind {
            PK::Label => Label(KebabStr::new_unchecked(&self.raw)),
            PK::Constructor => Constructor(KebabStr::new_unchecked(&self.raw[CONSTRUCTOR.len()..])),
            PK::Method => Method(ResourceFunc(&self.raw[METHOD.len()..])),
            PK::Static => Static(ResourceFunc(&self.raw[STATIC.len()..])),
            PK::Interface => Interface(InterfaceName(&self.raw)),
            PK::Dependency => Dependency(DependencyName(&self.raw)),
            PK::Url => Url(UrlName(&self.raw)),
            PK::Hash => Hash(HashName(&self.raw)),
        }
    }

    /// Returns the raw underlying name as a string.
    pub fn as_str(&self) -> &str {
        &self.raw
    }
}

impl From<ComponentName> for String {
    fn from(name: ComponentName) -> String {
        name.raw
    }
}

impl Hash for ComponentName {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.kind().hash(hasher)
    }
}

impl PartialEq for ComponentName {
    fn eq(&self, other: &ComponentName) -> bool {
        self.kind().eq(&other.kind())
    }
}

impl Eq for ComponentName {}

impl Ord for ComponentName {
    fn cmp(&self, other: &ComponentName) -> Ordering {
        self.kind().cmp(&other.kind())
    }
}

impl PartialOrd for ComponentName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.kind.partial_cmp(&other.kind)
    }
}

impl fmt::Display for ComponentName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}

impl fmt::Debug for ComponentName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}

impl ComponentNameKind<'_> {
    /// Returns the [`ParsedComponentNameKind`] of the [`ComponentNameKind`].
    fn kind(&self) -> ParsedComponentNameKind {
        match self {
            Self::Label(_) => ParsedComponentNameKind::Label,
            Self::Constructor(_) => ParsedComponentNameKind::Constructor,
            Self::Method(_) => ParsedComponentNameKind::Method,
            Self::Static(_) => ParsedComponentNameKind::Static,
            Self::Interface(_) => ParsedComponentNameKind::Interface,
            Self::Dependency(_) => ParsedComponentNameKind::Dependency,
            Self::Url(_) => ParsedComponentNameKind::Url,
            Self::Hash(_) => ParsedComponentNameKind::Hash,
        }
    }
}

impl Ord for ComponentNameKind<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        use ComponentNameKind::*;

        match (self, other) {
            (Label(lhs), Label(rhs)) => lhs.cmp(rhs),
            (Constructor(lhs), Constructor(rhs)) => lhs.cmp(rhs),
            (Method(lhs) | Static(lhs), Method(rhs) | Static(rhs)) => lhs.cmp(rhs),

            // `[..]l.l` is equivalent to `l`
            (Label(plain), Method(method) | Static(method))
            | (Method(method) | Static(method), Label(plain))
                if *plain == method.resource() && *plain == method.method() =>
            {
                Ordering::Equal
            }

            (Interface(lhs), Interface(rhs)) => lhs.cmp(rhs),
            (Dependency(lhs), Dependency(rhs)) => lhs.cmp(rhs),
            (Url(lhs), Url(rhs)) => lhs.cmp(rhs),
            (Hash(lhs), Hash(rhs)) => lhs.cmp(rhs),

            (Label(_), _)
            | (Constructor(_), _)
            | (Method(_), _)
            | (Static(_), _)
            | (Interface(_), _)
            | (Dependency(_), _)
            | (Url(_), _)
            | (Hash(_), _) => self.kind().cmp(&other.kind()),
        }
    }
}

impl PartialOrd for ComponentNameKind<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for ComponentNameKind<'_> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        use ComponentNameKind::*;
        match self {
            Label(name) => (0u8, name).hash(hasher),
            Constructor(name) => (1u8, name).hash(hasher),

            Method(name) | Static(name) => {
                // `l.l` hashes the same as `l` since they're equal above,
                // otherwise everything is hashed as `a.b` with a unique
                // prefix.
                if name.resource() == name.method() {
                    (0u8, name.resource()).hash(hasher)
                } else {
                    (2u8, name).hash(hasher)
                }
            }

            Interface(name) => (3u8, name).hash(hasher),
            Dependency(name) => (4u8, name).hash(hasher),
            Url(name) => (5u8, name).hash(hasher),
            Hash(name) => (6u8, name).hash(hasher),
        }
    }
}

impl PartialEq for ComponentNameKind<'_> {
    fn eq(&self, other: &ComponentNameKind<'_>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for ComponentNameKind<'_> {}

/// A resource name and its function, stored as `a.b`.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ResourceFunc<'a>(&'a str);

impl<'a> ResourceFunc<'a> {
    /// Returns the underlying string as `a.b`
    pub fn as_str(&self) -> &'a str {
        self.0
    }

    /// Returns the resource name or the `a` in `a.b`
    pub fn resource(&self) -> &'a KebabStr {
        let dot = self.0.find('.').unwrap();
        KebabStr::new_unchecked(&self.0[..dot])
    }

    /// Returns the method name or the `b` in `a.b`
    pub fn method(&self) -> &'a KebabStr {
        let dot = self.0.find('.').unwrap();
        KebabStr::new_unchecked(&self.0[dot + 1..])
    }
}

/// An interface name, stored as `a:b/c@1.2.3`
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct InterfaceName<'a>(&'a str);

impl<'a> InterfaceName<'a> {
    /// Returns the entire underlying string.
    pub fn as_str(&self) -> &'a str {
        self.0
    }

    /// Returns the `a:b` in `a:b:c/d/e`
    pub fn namespace(&self) -> &'a KebabStr {
        let colon = self.0.rfind(':').unwrap();
        KebabStr::new_unchecked(&self.0[..colon])
    }

    /// Returns the `c` in `a:b:c/d/e`
    pub fn package(&self) -> &'a KebabStr {
        let colon = self.0.rfind(':').unwrap();
        let slash = self.0.find('/').unwrap();
        KebabStr::new_unchecked(&self.0[colon + 1..slash])
    }

    /// Returns the `d` in `a:b:c/d/e`.
    pub fn interface(&self) -> &'a KebabStr {
        let projection = self.projection();
        let slash = projection.find('/').unwrap_or(projection.len());
        KebabStr::new_unchecked(&projection[..slash])
    }

    /// Returns the `d/e` in `a:b:c/d/e`
    pub fn projection(&self) -> &'a KebabStr {
        let slash = self.0.find('/').unwrap();
        let at = self.0.find('@').unwrap_or(self.0.len());
        KebabStr::new_unchecked(&self.0[slash + 1..at])
    }

    /// Returns the `1.2.3` in `a:b:c/d/e@1.2.3`
    pub fn version(&self) -> Option<Version> {
        let at = self.0.find('@')?;
        Some(Version::parse(&self.0[at + 1..]).unwrap())
    }
}

/// A dependency on an implementation either as `locked-dep=...` or
/// `unlocked-dep=...`
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct DependencyName<'a>(&'a str);

impl<'a> DependencyName<'a> {
    /// Returns entire underlying import string
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

/// A dependency on an implementation either as `url=...`
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct UrlName<'a>(&'a str);

impl<'a> UrlName<'a> {
    /// Returns entire underlying import string
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

/// A dependency on an implementation either as `integrity=...`.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct HashName<'a>(&'a str);

impl<'a> HashName<'a> {
    /// Returns entire underlying import string.
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

// A small helper structure to parse `self.next` which is an import or export
// name.
//
// Methods will update `self.next` as they go along and `self.offset` is used
// for error messages.
struct ComponentNameParser<'a> {
    next: &'a str,
    offset: usize,
    features: WasmFeatures,
}

impl<'a> ComponentNameParser<'a> {
    fn parse(&mut self) -> Result<ParsedComponentNameKind> {
        if self.eat_str(CONSTRUCTOR) {
            self.expect_kebab()?;
            return Ok(ParsedComponentNameKind::Constructor);
        }
        if self.eat_str(METHOD) {
            let resource = self.take_until('.')?;
            self.kebab(resource)?;
            self.expect_kebab()?;
            return Ok(ParsedComponentNameKind::Method);
        }
        if self.eat_str(STATIC) {
            let resource = self.take_until('.')?;
            self.kebab(resource)?;
            self.expect_kebab()?;
            return Ok(ParsedComponentNameKind::Static);
        }

        // 'unlocked-dep=<' <pkgnamequery> '>'
        if self.eat_str("unlocked-dep=") {
            self.expect_str("<")?;
            self.pkg_name_query()?;
            self.expect_str(">")?;
            return Ok(ParsedComponentNameKind::Dependency);
        }

        // 'locked-dep=<' <pkgname> '>' ( ',' <hashname> )?
        if self.eat_str("locked-dep=") {
            self.expect_str("<")?;
            self.pkg_name(false)?;
            self.expect_str(">")?;
            self.eat_optional_hash()?;
            return Ok(ParsedComponentNameKind::Dependency);
        }

        // 'url=<' <nonbrackets> '>' (',' <hashname>)?
        if self.eat_str("url=") {
            self.expect_str("<")?;
            let url = self.take_up_to('>')?;
            if url.contains('<') {
                bail!(self.offset, "url cannot contain `<`");
            }
            self.expect_str(">")?;
            self.eat_optional_hash()?;
            return Ok(ParsedComponentNameKind::Url);
        }

        // 'integrity=<' <integrity-metadata> '>'
        if self.eat_str("integrity=") {
            self.expect_str("<")?;
            let _hash = self.parse_hash()?;
            self.expect_str(">")?;
            return Ok(ParsedComponentNameKind::Hash);
        }

        if self.next.contains(':') {
            self.pkg_name(true)?;
            Ok(ParsedComponentNameKind::Interface)
        } else {
            self.expect_kebab()?;
            Ok(ParsedComponentNameKind::Label)
        }
    }

    // pkgnamequery ::= <pkgpath> <verrange>?
    fn pkg_name_query(&mut self) -> Result<()> {
        self.pkg_path(false)?;

        if self.eat_str("@") {
            if self.eat_str("*") {
                return Ok(());
            }

            self.expect_str("{")?;
            let range = self.take_up_to('}')?;
            self.expect_str("}")?;
            self.semver_range(range)?;
        }

        Ok(())
    }

    // pkgname ::= <pkgpath> <version>?
    fn pkg_name(&mut self, require_projection: bool) -> Result<()> {
        self.pkg_path(require_projection)?;

        if self.eat_str("@") {
            let version = match self.eat_up_to('>') {
                Some(version) => version,
                None => self.take_rest(),
            };

            self.semver(version)?;
        }

        Ok(())
    }

    // pkgpath ::= <namespace>+ <label> <projection>*
    fn pkg_path(&mut self, require_projection: bool) -> Result<()> {
        // There must be at least one package namespace
        self.take_lowercase_kebab()?;
        self.expect_str(":")?;
        self.take_lowercase_kebab()?;

        if self.features.cm_nested_names() {
            // Take the remaining package namespaces and name
            while self.next.starts_with(':') {
                self.expect_str(":")?;
                self.take_lowercase_kebab()?;
            }
        }

        // Take the projections
        if self.next.starts_with('/') {
            self.expect_str("/")?;
            self.take_kebab()?;

            if self.features.cm_nested_names() {
                while self.next.starts_with('/') {
                    self.expect_str("/")?;
                    self.take_kebab()?;
                }
            }
        } else if require_projection {
            bail!(self.offset, "expected `/` after package name");
        }

        Ok(())
    }

    // verrange ::= '@*'
    //            | '@{' <verlower> '}'
    //            | '@{' <verupper> '}'
    //            | '@{' <verlower> ' ' <verupper> '}'
    // verlower ::= '>=' <valid semver>
    // verupper ::= '<' <valid semver>
    fn semver_range(&self, range: &str) -> Result<()> {
        if range == "*" {
            return Ok(());
        }

        if let Some(range) = range.strip_prefix(">=") {
            let (lower, upper) = range
                .split_once(' ')
                .map(|(l, u)| (l, Some(u)))
                .unwrap_or((range, None));
            self.semver(lower)?;

            if let Some(upper) = upper {
                match upper.strip_prefix('<') {
                    Some(upper) => {
                        self.semver(upper)?;
                    }
                    None => bail!(
                        self.offset,
                        "expected `<` at start of version range upper bounds"
                    ),
                }
            }
        } else if let Some(upper) = range.strip_prefix('<') {
            self.semver(upper)?;
        } else {
            bail!(
                self.offset,
                "expected `>=` or `<` at start of version range"
            );
        }

        Ok(())
    }

    fn parse_hash(&mut self) -> Result<&'a str> {
        let integrity = self.take_up_to('>')?;
        let mut any = false;
        for hash in integrity.split_whitespace() {
            any = true;
            let rest = hash
                .strip_prefix("sha256")
                .or_else(|| hash.strip_prefix("sha384"))
                .or_else(|| hash.strip_prefix("sha512"));
            let rest = match rest {
                Some(s) => s,
                None => bail!(self.offset, "unrecognized hash algorithm: `{hash}`"),
            };
            let rest = match rest.strip_prefix('-') {
                Some(s) => s,
                None => bail!(self.offset, "expected `-` after hash algorithm: {hash}"),
            };
            let (base64, _options) = match rest.find('?') {
                Some(i) => (&rest[..i], Some(&rest[i + 1..])),
                None => (rest, None),
            };
            if !is_base64(base64) {
                bail!(self.offset, "not valid base64: `{base64}`");
            }
        }
        if !any {
            bail!(self.offset, "integrity hash cannot be empty");
        }
        Ok(integrity)
    }

    fn eat_optional_hash(&mut self) -> Result<Option<&'a str>> {
        if !self.eat_str(",") {
            return Ok(None);
        }
        self.expect_str("integrity=<")?;
        let ret = self.parse_hash()?;
        self.expect_str(">")?;
        Ok(Some(ret))
    }

    fn eat_str(&mut self, prefix: &str) -> bool {
        match self.next.strip_prefix(prefix) {
            Some(rest) => {
                self.next = rest;
                true
            }
            None => false,
        }
    }

    fn expect_str(&mut self, prefix: &str) -> Result<()> {
        if self.eat_str(prefix) {
            Ok(())
        } else {
            bail!(self.offset, "expected `{prefix}` at `{}`", self.next);
        }
    }

    fn eat_until(&mut self, c: char) -> Option<&'a str> {
        let ret = self.eat_up_to(c);
        if ret.is_some() {
            self.next = &self.next[c.len_utf8()..];
        }
        ret
    }

    fn eat_up_to(&mut self, c: char) -> Option<&'a str> {
        let i = self.next.find(c)?;
        let (a, b) = self.next.split_at(i);
        self.next = b;
        Some(a)
    }

    fn kebab(&self, s: &'a str) -> Result<&'a KebabStr> {
        match KebabStr::new(s) {
            Some(name) => Ok(name),
            None => bail!(self.offset, "`{s}` is not in kebab case"),
        }
    }

    fn semver(&self, s: &str) -> Result<Version> {
        match Version::parse(s) {
            Ok(v) => Ok(v),
            Err(e) => bail!(self.offset, "`{s}` is not a valid semver: {e}"),
        }
    }

    fn take_until(&mut self, c: char) -> Result<&'a str> {
        match self.eat_until(c) {
            Some(s) => Ok(s),
            None => bail!(self.offset, "failed to find `{c}` character"),
        }
    }

    fn take_up_to(&mut self, c: char) -> Result<&'a str> {
        match self.eat_up_to(c) {
            Some(s) => Ok(s),
            None => bail!(self.offset, "failed to find `{c}` character"),
        }
    }

    fn take_rest(&mut self) -> &'a str {
        let ret = self.next;
        self.next = "";
        ret
    }

    fn take_kebab(&mut self) -> Result<&'a KebabStr> {
        self.next
            .find(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-'))
            .map(|i| {
                let (kebab, next) = self.next.split_at(i);
                self.next = next;
                self.kebab(kebab)
            })
            .unwrap_or_else(|| self.expect_kebab())
    }

    fn take_lowercase_kebab(&mut self) -> Result<&'a KebabStr> {
        let kebab = self.take_kebab()?;
        if let Some(c) = kebab
            .chars()
            .find(|c| c.is_alphabetic() && !c.is_lowercase())
        {
            bail!(
                self.offset,
                "character `{c}` is not lowercase in package name/namespace"
            );
        }
        Ok(kebab)
    }

    fn expect_kebab(&mut self) -> Result<&'a KebabStr> {
        let s = self.take_rest();
        self.kebab(s)
    }
}

fn is_base64(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut equals = 0;
    for (i, byte) in s.as_bytes().iter().enumerate() {
        match byte {
            b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'+' | b'/' if equals == 0 => {}
            b'=' if i > 0 && equals < 2 => equals += 1,
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn parse_kebab_name(s: &str) -> Option<ComponentName> {
        ComponentName::new(s, 0).ok()
    }

    #[test]
    fn kebab_smoke() {
        assert!(KebabStr::new("").is_none());
        assert!(KebabStr::new("a").is_some());
        assert!(KebabStr::new("aB").is_none());
        assert!(KebabStr::new("a-B").is_some());
        assert!(KebabStr::new("a-").is_none());
        assert!(KebabStr::new("-").is_none());
        assert!(KebabStr::new("¶").is_none());
        assert!(KebabStr::new("0").is_none());
        assert!(KebabStr::new("a0").is_some());
        assert!(KebabStr::new("a-0").is_some());
        assert!(KebabStr::new("0-a").is_none());
        assert!(KebabStr::new("a-b--c").is_none());
        assert!(KebabStr::new("a0-000-3d4a-54FF").is_some());
        assert!(KebabStr::new("a0-000-3d4A-54Ff").is_none());
    }

    #[test]
    fn name_smoke() {
        assert!(parse_kebab_name("a").is_some());
        assert!(parse_kebab_name("[foo]a").is_none());
        assert!(parse_kebab_name("[constructor]a").is_some());
        assert!(parse_kebab_name("[method]a").is_none());
        assert!(parse_kebab_name("[method]a.b").is_some());
        assert!(parse_kebab_name("[method]a-0.b-1").is_some());
        assert!(parse_kebab_name("[method]a.b.c").is_none());
        assert!(parse_kebab_name("[static]a.b").is_some());
        assert!(parse_kebab_name("[static]a").is_none());
    }

    #[test]
    fn name_equality() {
        assert_eq!(parse_kebab_name("a"), parse_kebab_name("a"));
        assert_ne!(parse_kebab_name("a"), parse_kebab_name("b"));
        assert_eq!(
            parse_kebab_name("[constructor]a"),
            parse_kebab_name("[constructor]a")
        );
        assert_ne!(
            parse_kebab_name("[constructor]a"),
            parse_kebab_name("[constructor]b")
        );
        assert_eq!(
            parse_kebab_name("[method]a.b"),
            parse_kebab_name("[method]a.b")
        );
        assert_ne!(
            parse_kebab_name("[method]a.b"),
            parse_kebab_name("[method]b.b")
        );
        assert_eq!(
            parse_kebab_name("[static]a.b"),
            parse_kebab_name("[static]a.b")
        );
        assert_ne!(
            parse_kebab_name("[static]a.b"),
            parse_kebab_name("[static]b.b")
        );

        assert_eq!(
            parse_kebab_name("[static]a.b"),
            parse_kebab_name("[method]a.b")
        );
        assert_eq!(
            parse_kebab_name("[method]a.b"),
            parse_kebab_name("[static]a.b")
        );

        assert_ne!(
            parse_kebab_name("[method]b.b"),
            parse_kebab_name("[static]a.b")
        );

        let mut s = HashSet::new();
        assert!(s.insert(parse_kebab_name("a")));
        assert!(s.insert(parse_kebab_name("[constructor]a")));
        assert!(s.insert(parse_kebab_name("[method]a.b")));
        assert!(!s.insert(parse_kebab_name("[static]a.b")));
        assert!(s.insert(parse_kebab_name("[static]b.b")));
    }
}
