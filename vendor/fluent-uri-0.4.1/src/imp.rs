#![allow(missing_debug_implementations)]

use crate::{
    component::{Authority, IAuthority, Scheme},
    convert::ConvertError,
    parse::{self, ParseError},
    pct_enc::{encoder::*, EStr, Encoder},
};
use borrow_or_share::{BorrowOrShare, Bos};
use core::{borrow::Borrow, cmp::Ordering, fmt, hash, num::NonZeroUsize, str};

#[cfg(feature = "alloc")]
use crate::{
    build::{
        state::{NonRefStart, Start},
        Builder,
    },
    normalize::Normalizer,
    resolve::{self, ResolveError},
};
#[cfg(feature = "alloc")]
use alloc::{borrow::ToOwned, string::String};
#[cfg(feature = "alloc")]
use core::str::FromStr;

#[cfg(feature = "net")]
use crate::net::{Ipv4Addr, Ipv6Addr};

#[cfg(feature = "serde")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

pub trait Value: Default {}

impl Value for &str {}

#[cfg(feature = "alloc")]
impl Value for String {}

pub struct Constraints {
    pub ascii_only: bool,
    pub scheme_required: bool,
}

pub trait RiMaybeRef: Sized {
    type Val;
    type WithVal<T>: RiMaybeRef<Val = T>;

    type UserinfoE: Encoder;
    type RegNameE: Encoder;
    type PathE: Encoder;
    type QueryE: Encoder;
    type FragmentE: Encoder;

    const CONSTRAINTS: Constraints;

    fn new(val: Self::Val, meta: Meta) -> Self;

    fn from_pair((val, meta): (Self::Val, Meta)) -> Self {
        Self::new(val, meta)
    }

    fn make_ref<'i, 'o>(&'i self) -> RmrRef<'o, 'i>
    where
        Self::Val: BorrowOrShare<'i, 'o, str>;
}

#[cfg(feature = "alloc")]
pub trait Ri: RiMaybeRef {
    type Ref<T>: RiMaybeRef<Val = T>;
}

pub trait Parse {
    type Val;
    type Err;

    fn parse<R: RiMaybeRef<Val = Self::Val>>(self) -> Result<R, Self::Err>;
}

impl<'a> Parse for &'a str {
    type Val = &'a str;
    type Err = ParseError;

    fn parse<R: RiMaybeRef<Val = Self::Val>>(self) -> Result<R, Self::Err> {
        parse::parse(self.as_bytes(), R::CONSTRAINTS).map(|meta| R::new(self, meta))
    }
}

#[cfg(feature = "alloc")]
impl Parse for String {
    type Val = Self;
    type Err = (ParseError, Self);

    fn parse<R: RiMaybeRef<Val = Self::Val>>(self) -> Result<R, Self::Err> {
        match parse::parse(self.as_bytes(), R::CONSTRAINTS) {
            Ok(meta) => Ok(R::new(self, meta)),
            Err(e) => Err((e, self)),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Meta {
    // The index of the trailing colon.
    pub scheme_end: Option<NonZeroUsize>,
    pub auth_meta: Option<AuthMeta>,
    pub path_bounds: (usize, usize),
    // One byte past the last byte of query.
    pub query_end: Option<NonZeroUsize>,
}

impl Meta {
    #[inline]
    pub fn query_or_path_end(&self) -> usize {
        self.query_end.map_or(self.path_bounds.1, |i| i.get())
    }
}

#[derive(Clone, Copy, Default)]
pub struct AuthMeta {
    pub host_bounds: (usize, usize),
    pub host_meta: HostMeta,
}

impl AuthMeta {
    pub const EMPTY: Self = Self {
        host_bounds: (0, 0),
        host_meta: HostMeta::RegName,
    };
}

#[derive(Clone, Copy, Default)]
pub enum HostMeta {
    Ipv4(#[cfg(feature = "net")] Ipv4Addr),
    Ipv6(#[cfg(feature = "net")] Ipv6Addr),
    IpvFuture,
    #[default]
    RegName,
}

pub trait PathEncoder: Encoder {}

impl PathEncoder for Path {}
impl PathEncoder for IPath {}

macro_rules! cond {
    (if true { $($then:tt)* } else { $($else:tt)* }) => { $($then)* };
    (if false { $($then:tt)* } else { $($else:tt)* }) => { $($else)* };
}

macro_rules! ri_maybe_ref {
    (
        Type = $Ty:ident,
        type_name = $ty:literal,
        variable_name = $var:literal,
        name = $name:literal,
        indefinite_article = $art:literal,
        description = $desc:literal,
        ascii_only = $ascii_only:literal,
        scheme_required = $scheme_required:tt,
        rfc = $rfc:literal,
        abnf_rule = ($abnf:literal, $abnf_link:literal),
        $(
            NonRefType = $NonRefTy:ident,
            non_ref_name = $nr_name:literal,
            non_ref_link = $nr_link:literal,
            abnf_rule_absolute = ($abnf_abs:literal, $abnf_abs_link:literal),
        )?
        $(
            RefType = $RefTy:ident,
            ref_name = $ref_name:literal,
        )?
        AuthorityType = $Authority:ident,
        UserinfoEncoderType = $UserinfoE:ident,
        RegNameEncoderType = $RegNameE:ident,
        PathEncoderType = $PathE:ident,
        QueryEncoderType = $QueryE:ident,
        FragmentEncoderType = $FragmentE:ident,
    ) => {
        #[doc = $desc]
        ///
        /// See the [crate-level documentation](crate#terminology) for an explanation of the above term(s).
        ///
        /// # Variants
        ///
        #[doc = concat!("Two variants of `", $ty, "` are available: ")]
        #[doc = concat!("`", $ty, "<&str>` (borrowed) and `", $ty, "<String>` (owned).")]
        ///
        #[doc = concat!("`", $ty, "<&'a str>`")]
        /// outputs references with lifetime `'a` where possible
        /// (thanks to [`borrow-or-share`](borrow_or_share)):
        ///
        /// ```
        #[doc = concat!("use fluent_uri::", $ty, ";")]
        ///
        #[doc = concat!("// Keep a reference to the path after dropping the `", $ty, "`.")]
        #[doc = concat!("let path = ", $ty, "::parse(\"foo:bar\")?.path();")]
        /// assert_eq!(path, "bar");
        /// # Ok::<_, fluent_uri::ParseError>(())
        /// ```
        ///
        /// # Comparison
        ///
        #[doc = concat!("`", $ty, "`s")]
        /// are compared [lexicographically](Ord#lexicographical-comparison)
        /// by their byte values. Normalization is **not** performed prior to comparison.
        ///
        /// # Examples
        ///
        /// Parse and extract components from
        #[doc = concat!($art, " ", $name, ":")]
        ///
        /// ```
        /// use fluent_uri::{
        ///     component::{Host, Scheme},
        ///     pct_enc::EStr,
        #[doc = concat!("    ", $ty, ",")]
        /// };
        ///
        /// const SCHEME_FOO: &Scheme = Scheme::new_or_panic("foo");
        ///
        /// let s = "foo://user@example.com:8042/over/there?name=ferret#nose";
        #[doc = concat!("let ", $var, " = ", $ty, "::parse(s)?;")]
        ///
        #[doc = concat!("assert_eq!(", $var, ".scheme()",
            cond!(if $scheme_required { "" } else { ".unwrap()" }), ", SCHEME_FOO);")]
        ///
        #[doc = concat!("let auth = ", $var, ".authority().unwrap();")]
        /// assert_eq!(auth.as_str(), "user@example.com:8042");
        /// assert_eq!(auth.userinfo().unwrap(), "user");
        /// assert_eq!(auth.host(), "example.com");
        /// assert!(matches!(auth.host_parsed(), Host::RegName(name) if name == "example.com"));
        /// assert_eq!(auth.port().unwrap(), "8042");
        /// assert_eq!(auth.port_to_u16(), Ok(Some(8042)));
        ///
        #[doc = concat!("assert_eq!(", $var, ".path(), \"/over/there\");")]
        #[doc = concat!("assert_eq!(", $var, ".query().unwrap(), \"name=ferret\");")]
        #[doc = concat!("assert_eq!(", $var, ".fragment().unwrap(), \"nose\");")]
        /// # Ok::<_, fluent_uri::ParseError>(())
        /// ```
        ///
        /// Parse into and convert between
        #[doc = concat!("`", $ty, "<&str>` and `", $ty, "<String>`:")]
        ///
        /// ```
        #[doc = concat!("use fluent_uri::", $ty, ";")]
        ///
        /// let s = "http://example.com/";
        ///
        #[doc = concat!("// Parse into a `", $ty, "<&str>` from a string slice.")]
        #[doc = concat!("let ", $var, ": ", $ty, "<&str> = ", $ty, "::parse(s)?;")]
        ///
        #[doc = concat!("// Parse into a `", $ty, "<String>` from an owned string.")]
        #[doc = concat!("let ", $var, "_owned: ", $ty, "<String> = ", $ty, "::parse(s.to_owned()).map_err(|e| e.0)?;")]
        ///
        #[doc = concat!("// Convert a `", $ty, "<&str>` to `", $ty, "<String>`.")]
        #[doc = concat!("let ", $var, "_owned: ", $ty, "<String> = ", $var, ".to_owned();")]
        ///
        #[doc = concat!("// Borrow a `", $ty, "<String>` as `", $ty, "<&str>`.")]
        #[doc = concat!("let ", $var, ": ", $ty, "<&str> = ", $var, "_owned.borrow();")]
        /// # Ok::<_, fluent_uri::ParseError>(())
        /// ```
        #[derive(Clone, Copy)]
        pub struct $Ty<T> {
            /// Value of the URI/IRI (reference).
            pub(crate) val: T,
            /// Metadata of the URI/IRI (reference).
            /// Should be identical to parser output with `val` as input.
            pub(crate) meta: Meta,
        }

        impl<T> RiMaybeRef for $Ty<T> {
            type Val = T;
            type WithVal<U> = $Ty<U>;

            type UserinfoE = $UserinfoE;
            type RegNameE = $RegNameE;
            type PathE = $PathE;
            type QueryE = $QueryE;
            type FragmentE = $FragmentE;

            const CONSTRAINTS: Constraints = Constraints {
                ascii_only: $ascii_only,
                scheme_required: $scheme_required,
            };

            fn new(val: T, meta: Meta) -> Self {
                Self { val, meta }
            }

            fn make_ref<'i, 'o>(&'i self) -> RmrRef<'o, 'i>
            where
                Self::Val: BorrowOrShare<'i, 'o, str>,
            {
                RmrRef::new(self.as_str(), &self.meta)
            }
        }

        $(
            #[cfg(feature = "alloc")]
            impl<T: Bos<str>> Ri for $Ty<T> {
                type Ref<U> = $RefTy<U>;
            }
        )?

        impl<T> $Ty<T> {
            #[doc = concat!("Parses ", $art, " ", $name, " from a string into ", $art, " `", $ty, "`.")]
            ///
            /// The return type is
            ///
            #[doc = concat!("- `Result<", $ty, "<&str>, ParseError>` for `I = &str`;")]
            #[doc = concat!("- `Result<", $ty, "<String>, (ParseError, String)>` for `I = String`.")]
            ///
            /// # Errors
            ///
            /// Returns `Err` if the string does not match the
            #[doc = concat!("[`", $abnf, "`][abnf] ABNF rule from RFC ", $rfc, ".")]
            ///
            #[doc = concat!("[abnf]: ", $abnf_link)]
            pub fn parse<I>(input: I) -> Result<Self, I::Err>
            where
                I: Parse<Val = T>,
            {
                input.parse()
            }
        }

        #[cfg(feature = "alloc")]
        impl $Ty<String> {
            #[doc = concat!("Creates a new builder for ", $name, ".")]
            #[inline]
            pub fn builder() -> Builder<Self, cond!(if $scheme_required { NonRefStart } else { Start })> {
                Builder::new()
            }

            #[doc = concat!("Borrows this `", $ty, "<String>` as `", $ty, "<&str>`.")]
            #[allow(clippy::should_implement_trait)]
            #[inline]
            #[must_use]
            pub fn borrow(&self) -> $Ty<&str> {
                $Ty {
                    val: &self.val,
                    meta: self.meta,
                }
            }

            #[doc = concat!("Consumes this `", $ty, "<String>` and yields the underlying [`String`].")]
            #[inline]
            #[must_use]
            pub fn into_string(self) -> String {
                self.val
            }
        }

        #[cfg(feature = "alloc")]
        impl $Ty<&str> {
            #[doc = concat!("Creates a new `", $ty, "<String>` by cloning the contents of this `", $ty, "<&str>`.")]
            #[inline]
            #[must_use]
            pub fn to_owned(&self) -> $Ty<String> {
                $Ty {
                    val: self.val.to_owned(),
                    meta: self.meta,
                }
            }
        }

        impl<'i, 'o, T: BorrowOrShare<'i, 'o, str>> $Ty<T> {
            #[doc = concat!("Returns the ", $name, " as a string slice.")]
            #[must_use]
            pub fn as_str(&'i self) -> &'o str {
                self.val.borrow_or_share()
            }

            cond!(if $scheme_required {
                /// Returns the [scheme] component.
                ///
                /// Note that the scheme component is *case-insensitive*.
                /// See the documentation of [`Scheme`] for more details on comparison.
                ///
                /// [scheme]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.1
                ///
                /// # Examples
                ///
                /// ```
                #[doc = concat!("use fluent_uri::{component::Scheme, ", $ty, "};")]
                ///
                /// const SCHEME_HTTP: &Scheme = Scheme::new_or_panic("http");
                ///
                #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"http://example.com/\")?;")]
                #[doc = concat!("assert_eq!(", $var, ".scheme(), SCHEME_HTTP);")]
                /// # Ok::<_, fluent_uri::ParseError>(())
                /// ```
                #[must_use]
                pub fn scheme(&'i self) -> &'o Scheme {
                    self.make_ref().scheme()
                }
            } else {
                /// Returns the optional [scheme] component.
                ///
                /// Note that the scheme component is *case-insensitive*.
                /// See the documentation of [`Scheme`] for more details on comparison.
                ///
                /// [scheme]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.1
                ///
                /// # Examples
                ///
                /// ```
                #[doc = concat!("use fluent_uri::{component::Scheme, ", $ty, "};")]
                ///
                /// const SCHEME_HTTP: &Scheme = Scheme::new_or_panic("http");
                ///
                #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"http://example.com/\")?;")]
                #[doc = concat!("assert_eq!(", $var, ".scheme(), Some(SCHEME_HTTP));")]
                ///
                #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"/path/to/file\")?;")]
                #[doc = concat!("assert_eq!(", $var, ".scheme(), None);")]
                /// # Ok::<_, fluent_uri::ParseError>(())
                /// ```
                #[must_use]
                pub fn scheme(&'i self) -> Option<&'o Scheme> {
                    self.make_ref().scheme_opt()
                }
            });

            /// Returns the optional [authority] component.
            ///
            /// [authority]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.2
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use fluent_uri::", $ty, ";")]
            ///
            #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"http://example.com/\")?;")]
            #[doc = concat!("assert!(", $var, ".authority().is_some());")]
            ///
            #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"mailto:user@example.com\")?;")]
            #[doc = concat!("assert!(", $var, ".authority().is_none());")]
            /// # Ok::<_, fluent_uri::ParseError>(())
            /// ```
            #[must_use]
            pub fn authority(&'i self) -> Option<$Authority<'o>> {
                self.make_ref().authority().map(Authority::cast)
            }

            /// Returns the [path] component.
            ///
            /// The path component is always present, although it may be empty.
            ///
            /// The returned `EStr` slice has [extension methods] for the path component.
            ///
            /// [path]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.3
            /// [extension methods]: EStr#impl-EStr<E>-1
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use fluent_uri::", $ty, ";")]
            ///
            #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"http://example.com/\")?;")]
            #[doc = concat!("assert_eq!(", $var, ".path(), \"/\");")]
            ///
            #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"mailto:user@example.com\")?;")]
            #[doc = concat!("assert_eq!(", $var, ".path(), \"user@example.com\");")]
            ///
            #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"http://example.com\")?;")]
            #[doc = concat!("assert_eq!(", $var, ".path(), \"\");")]
            /// # Ok::<_, fluent_uri::ParseError>(())
            /// ```
            #[must_use]
            pub fn path(&'i self) -> &'o EStr<$PathE> {
                self.make_ref().path().cast()
            }

            /// Returns the optional [query] component.
            ///
            /// [query]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.4
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use fluent_uri::{pct_enc::EStr, ", $ty, "};")]
            ///
            #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"http://example.com/?lang=en\")?;")]
            #[doc = concat!("assert_eq!(", $var, ".query(), Some(EStr::new_or_panic(\"lang=en\")));")]
            ///
            #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"ftp://192.0.2.1/\")?;")]
            #[doc = concat!("assert_eq!(", $var, ".query(), None);")]
            /// # Ok::<_, fluent_uri::ParseError>(())
            /// ```
            #[must_use]
            pub fn query(&'i self) -> Option<&'o EStr<$QueryE>> {
                self.make_ref().query().map(EStr::cast)
            }

            /// Returns the optional [fragment] component.
            ///
            /// [fragment]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.5
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use fluent_uri::{pct_enc::EStr, ", $ty, "};")]
            ///
            #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"http://example.com/#usage\")?;")]
            #[doc = concat!("assert_eq!(", $var, ".fragment(), Some(EStr::new_or_panic(\"usage\")));")]
            ///
            #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"ftp://192.0.2.1/\")?;")]
            #[doc = concat!("assert_eq!(", $var, ".fragment(), None);")]
            /// # Ok::<_, fluent_uri::ParseError>(())
            /// ```
            #[must_use]
            pub fn fragment(&'i self) -> Option<&'o EStr<$FragmentE>> {
                self.make_ref().fragment().map(EStr::cast)
            }
        }

        impl<T: Bos<str>> $Ty<T> {
            $(
                #[doc = concat!("Resolves the ", $name, " against the given base ", $nr_name)]
                #[doc = concat!("and returns the target ", $nr_name, ".")]
                ///
                #[doc = concat!("The base ", $nr_name)]
                /// **must** have no fragment, i.e., match the
                #[doc = concat!("[`", $abnf_abs, "`][abnf] ABNF rule from RFC ", $rfc, ".")]
                ///
                #[doc = concat!("To prepare a base ", $nr_name, ",")]
                /// you can use [`strip_fragment`], [`with_fragment`] or [`set_fragment`] to remove the fragment
                #[doc = concat!("from any ", $nr_name, ".")]
                /// Note that a base without fragment does **not** guarantee a successful resolution
                /// (see the **must** below).
                ///
                /// This method applies the reference resolution algorithm defined in
                /// [Section 5 of RFC 3986](https://datatracker.ietf.org/doc/html/rfc3986#section-5),
                /// except for the following deviations:
                ///
                /// - If `base` has a [rootless] path and no authority, then
                ///   `self` **must** either have a scheme, be empty, or start with `'#'`.
                /// - When the target has no authority and its path would start
                ///   with `"//"`, the string `"/."` is prepended to the path. This closes a
                ///   loophole in the original algorithm that resolving `".//@@"` against
                ///   `"foo:/"` yields `"foo://@@"` which is not a valid URI/IRI.
                /// - Percent-encoded dot segments (e.g. `"%2E"` and `".%2e"`) are also removed.
                ///   This closes a loophole in the original algorithm that resolving `".."`
                ///   against `"foo:/bar/baz/.%2E/"` yields `"foo:/bar/baz/"`, while first normalizing
                ///   the base and then resolving `".."` against it yields `"foo:/"`.
                /// - A slash (`'/'`) is appended to the base when it ends with a double-dot
                ///   segment. This closes a loophole in the original algorithm that resolving
                ///   `"."` against `"foo:/bar/.."` yields `"foo:/bar/"`, while first normalizing
                ///   the base and then resolving `"."` against it yields `"foo:/"`.
                /// - When `base` has an [absolute] path and `self` has an empty path and
                ///   no scheme nor authority, dot segments are removed from the base path before
                ///   using it as the target path. This closes a loophole in the original algorithm
                ///   that resolving `""` against `"foo:/."` yields `"foo:/."` in which
                ///   dot segments are not removed.
                ///
                /// No normalization except the removal of dot segments will be performed.
                /// Use [`normalize`] if necessary.
                ///
                #[doc = concat!("[abnf]: ", $abnf_abs_link)]
                #[doc = concat!("[`strip_fragment`]: ", stringify!($NonRefTy), "::strip_fragment")]
                #[doc = concat!("[`with_fragment`]: ", stringify!($NonRefTy), "::with_fragment")]
                #[doc = concat!("[`set_fragment`]: ", stringify!($NonRefTy), "::set_fragment")]
                /// [rootless]: EStr::<Path>::is_rootless
                /// [absolute]: EStr::<Path>::is_absolute
                /// [`normalize`]: Self::normalize
                ///
                /// This method has the property that
                /// `self.resolve_against(base).map(|r| r.normalize()).ok()` equals
                /// `self.normalize().resolve_against(&base.normalize()).ok()`.
                ///
                /// If you need to resolve multiple references against a common base or configure the behavior
                /// of resolution, consider using [`Resolver`](crate::resolve::Resolver) instead.
                ///
                /// # Errors
                ///
                /// Returns `Err` if any of the above two **must**s is violated.
                ///
                /// # Examples
                ///
                /// ```
                #[doc = concat!("use fluent_uri::{", stringify!($NonRefTy), ", ", $ty, "};")]
                ///
                #[doc = concat!("let base = ", stringify!($NonRefTy), "::parse(\"http://example.com/foo/bar\")?;")]
                ///
                #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"baz\")?;")]
                #[doc = concat!("assert_eq!(", $var, ".resolve_against(&base).unwrap(), \"http://example.com/foo/baz\");")]
                ///
                #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"../baz\")?;")]
                #[doc = concat!("assert_eq!(", $var, ".resolve_against(&base).unwrap(), \"http://example.com/baz\");")]
                ///
                #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"?baz\")?;")]
                #[doc = concat!("assert_eq!(", $var, ".resolve_against(&base).unwrap(), \"http://example.com/foo/bar?baz\");")]
                /// # Ok::<_, fluent_uri::ParseError>(())
                /// ```
                #[cfg(feature = "alloc")]
                pub fn resolve_against<U: Bos<str>>(
                    &self,
                    base: &$NonRefTy<U>,
                ) -> Result<$NonRefTy<String>, ResolveError> {
                    resolve::resolve(base.make_ref(), self.make_ref(), true).map(RiMaybeRef::from_pair)
                }
            )?

            #[doc = concat!("Normalizes the ", $name, ".")]
            ///
            /// This method applies syntax-based normalization described in
            /// [Section 6.2.2 of RFC 3986](https://datatracker.ietf.org/doc/html/rfc3986#section-6.2.2)
            /// and [Section 5.3.2 of RFC 3987](https://datatracker.ietf.org/doc/html/rfc3987#section-5.3.2),
            /// along with IPv6 address and default port normalization.
            /// This is effectively equivalent to taking the following steps in order:
            ///
            /// - Decode any percent-encoded octets that correspond to an allowed character which is not reserved.
            /// - Uppercase the hexadecimal digits within all percent-encoded octets.
            /// - Lowercase all ASCII characters within the scheme and the host except the percent-encoded octets.
            /// - Turn any IPv6 literal address into its canonical form as per
            ///   [RFC 5952](https://datatracker.ietf.org/doc/html/rfc5952).
            /// - If the port is empty or equals the [scheme's default], remove it along with the `':'` delimiter.
            /// - If `self` has a scheme and an [absolute] path, apply the
            ///   [`remove_dot_segments`] algorithm to the path, taking account of
            ///   percent-encoded dot segments as described at [`UriRef::resolve_against`].
            /// - If `self` has no authority and its path would start with
            ///   `"//"`, prepend `"/."` to the path.
            ///
            /// This method is idempotent: `self.normalize()` equals `self.normalize().normalize()`.
            ///
            /// If you need to configure the behavior of normalization, consider using [`Normalizer`] instead.
            ///
            /// [`UriRef::resolve_against`]: crate::UriRef::resolve_against
            /// [scheme's default]: Scheme::default_port
            /// [absolute]: EStr::<Path>::is_absolute
            /// [`remove_dot_segments`]: https://datatracker.ietf.org/doc/html/rfc3986#section-5.2.4
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use fluent_uri::", $ty, ";")]
            ///
            #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"eXAMPLE://a/./b/../b/%63/%7bfoo%7d\")?;")]
            #[doc = concat!("assert_eq!(", $var, ".normalize(), \"example://a/b/c/%7Bfoo%7D\");")]
            /// # Ok::<_, fluent_uri::ParseError>(())
            /// ```
            #[cfg(feature = "alloc")]
            #[must_use]
            pub fn normalize(&self) -> $Ty<String> {
                Normalizer::new().normalize(self).unwrap()
            }

            cond!(if $scheme_required {} else {
                /// Checks whether a scheme component is present.
                ///
                /// # Examples
                ///
                /// ```
                #[doc = concat!("use fluent_uri::", $ty, ";")]
                ///
                #[doc = concat!("assert!(", $ty, "::parse(\"http://example.com/\")?.has_scheme());")]
                #[doc = concat!("assert!(!", $ty, "::parse(\"/path/to/file\")?.has_scheme());")]
                /// # Ok::<_, fluent_uri::ParseError>(())
                /// ```
                #[must_use]
                pub fn has_scheme(&self) -> bool {
                    self.make_ref().has_scheme()
                }
            });

            /// Checks whether an authority component is present.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use fluent_uri::", $ty, ";")]
            ///
            #[doc = concat!("assert!(", $ty, "::parse(\"http://example.com/\")?.has_authority());")]
            #[doc = concat!("assert!(!", $ty, "::parse(\"mailto:user@example.com\")?.has_authority());")]
            /// # Ok::<_, fluent_uri::ParseError>(())
            /// ```
            #[must_use]
            pub fn has_authority(&self) -> bool {
                self.make_ref().has_authority()
            }

            /// Checks whether a query component is present.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use fluent_uri::", $ty, ";")]
            ///
            #[doc = concat!("assert!(", $ty, "::parse(\"http://example.com/?lang=en\")?.has_query());")]
            #[doc = concat!("assert!(!", $ty, "::parse(\"ftp://192.0.2.1/\")?.has_query());")]
            /// # Ok::<_, fluent_uri::ParseError>(())
            /// ```
            #[must_use]
            pub fn has_query(&self) -> bool {
                self.make_ref().has_query()
            }

            /// Checks whether a fragment component is present.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use fluent_uri::", $ty, ";")]
            ///
            #[doc = concat!("assert!(", $ty, "::parse(\"http://example.com/#usage\")?.has_fragment());")]
            #[doc = concat!("assert!(!", $ty, "::parse(\"ftp://192.0.2.1/\")?.has_fragment());")]
            /// # Ok::<_, fluent_uri::ParseError>(())
            /// ```
            #[must_use]
            pub fn has_fragment(&self) -> bool {
                self.make_ref().has_fragment()
            }

            #[doc = concat!("Returns a slice of this ", $name)]
            /// with the fragment component removed.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use fluent_uri::", $ty, ";")]
            ///
            #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"http://example.com/#fragment\")?;")]
            #[doc = concat!("assert_eq!(", $var, ".strip_fragment(), \"http://example.com/\");")]
            /// # Ok::<_, fluent_uri::ParseError>(())
            /// ```
            #[must_use]
            pub fn strip_fragment(&self) -> $Ty<&str> {
                // Altering only the fragment does not change the metadata.
                RiMaybeRef::new(self.make_ref().strip_fragment(), self.meta)
            }

            #[doc = concat!("Creates a new ", $name)]
            /// by replacing the fragment component of `self` with the given one.
            ///
            /// The fragment component is removed when `opt.is_none()`.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use fluent_uri::{pct_enc::EStr, ", $ty, "};")]
            ///
            #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"http://example.com/\")?;")]
            /// assert_eq!(
            #[doc = concat!("    ", $var, ".with_fragment(Some(EStr::new_or_panic(\"fragment\"))),")]
            ///     "http://example.com/#fragment"
            /// );
            ///
            #[doc = concat!("let ", $var, " = ", $ty, "::parse(\"http://example.com/#fragment\")?;")]
            #[doc = concat!("assert_eq!(", $var, ".with_fragment(None), \"http://example.com/\");")]
            /// # Ok::<_, fluent_uri::ParseError>(())
            /// ```
            #[cfg(feature = "alloc")]
            #[must_use]
            pub fn with_fragment(&self, opt: Option<&EStr<$FragmentE>>) -> $Ty<String> {
                // Altering only the fragment does not change the metadata.
                RiMaybeRef::new(self.make_ref().with_fragment(opt.map(EStr::as_str)), self.meta)
            }
        }

        #[cfg(feature = "alloc")]
        impl $Ty<String> {
            /// Replaces the fragment component of `self` with the given one.
            ///
            /// The fragment component is removed when `opt.is_none()`.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use fluent_uri::{pct_enc::EStr, ", $ty, "};")]
            ///
            #[doc = concat!("let mut ", $var, " = ", $ty, "::parse(\"http://example.com/\")?.to_owned();")]
            ///
            #[doc = concat!($var, ".set_fragment(Some(EStr::new_or_panic(\"fragment\")));")]
            #[doc = concat!("assert_eq!(", $var, ", \"http://example.com/#fragment\");")]
            ///
            #[doc = concat!($var, ".set_fragment(None);")]
            #[doc = concat!("assert_eq!(", $var, ", \"http://example.com/\");")]
            /// # Ok::<_, fluent_uri::ParseError>(())
            /// ```
            pub fn set_fragment(&mut self, opt: Option<&EStr<$FragmentE>>) {
                // Altering only the fragment does not change the metadata.
                RmrRef::set_fragment(&mut self.val, &self.meta, opt.map(EStr::as_str))
            }
        }

        impl<T: Value> Default for $Ty<T> {
            #[doc = concat!("Creates an empty ", $name, ".")]
            fn default() -> Self {
                Self {
                    val: T::default(),
                    meta: Meta::default(),
                }
            }
        }

        impl<T: Bos<str>, U: Bos<str>> PartialEq<$Ty<U>> for $Ty<T> {
            fn eq(&self, other: &$Ty<U>) -> bool {
                self.as_str() == other.as_str()
            }
        }

        impl<T: Bos<str>> PartialEq<str> for $Ty<T> {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }

        impl<T: Bos<str>> PartialEq<$Ty<T>> for str {
            fn eq(&self, other: &$Ty<T>) -> bool {
                self == other.as_str()
            }
        }

        impl<T: Bos<str>> PartialEq<&str> for $Ty<T> {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }

        impl<T: Bos<str>> PartialEq<$Ty<T>> for &str {
            fn eq(&self, other: &$Ty<T>) -> bool {
                *self == other.as_str()
            }
        }

        impl<T: Bos<str>> Eq for $Ty<T> {}

        impl<T: Bos<str>> hash::Hash for $Ty<T> {
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                self.as_str().hash(state);
            }
        }

        impl<T: Bos<str>> PartialOrd for $Ty<T> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl<T: Bos<str>> Ord for $Ty<T> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.as_str().cmp(other.as_str())
            }
        }

        impl<T: Bos<str>> AsRef<str> for $Ty<T> {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl<T: Bos<str>> Borrow<str> for $Ty<T> {
            fn borrow(&self) -> &str {
                self.as_str()
            }
        }

        impl<'a> TryFrom<&'a str> for $Ty<&'a str> {
            type Error = ParseError;

            /// Equivalent to [`parse`](Self::parse).
            #[inline]
            fn try_from(value: &'a str) -> Result<Self, Self::Error> {
                $Ty::parse(value)
            }
        }

        #[cfg(feature = "alloc")]
        impl TryFrom<String> for $Ty<String> {
            type Error = (ParseError, String);

            /// Equivalent to [`parse`](Self::parse).
            #[inline]
            fn try_from(value: String) -> Result<Self, Self::Error> {
                $Ty::parse(value)
            }
        }

        impl<'a> From<$Ty<&'a str>> for &'a str {
            #[doc = concat!("Equivalent to [`as_str`](", $ty, "::as_str).")]
            #[inline]
            fn from(value: $Ty<&'a str>) -> &'a str {
                value.val
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a> From<$Ty<String>> for String {
            #[doc = concat!("Equivalent to [`into_string`](", $ty, "::into_string).")]
            #[inline]
            fn from(value: $Ty<String>) -> String {
                value.val
            }
        }

        #[cfg(feature = "alloc")]
        impl From<$Ty<&str>> for $Ty<String> {
            /// Equivalent to [`to_owned`](Self::to_owned).
            #[inline]
            fn from(value: $Ty<&str>) -> Self {
                value.to_owned()
            }
        }

        #[cfg(feature = "alloc")]
        impl FromStr for $Ty<String> {
            type Err = ParseError;

            #[doc = concat!("Equivalent to `", $ty, "::parse(s).map(|r| r.to_owned())`.")]
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $Ty::parse(s).map(|r| r.to_owned())
            }
        }

        impl<T: Bos<str>> fmt::Debug for $Ty<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct($ty)
                    .field("scheme", &self.scheme())
                    .field("authority", &self.authority())
                    .field("path", &self.path())
                    .field("query", &self.query())
                    .field("fragment", &self.fragment())
                    .finish()
            }
        }

        impl<T: Bos<str>> fmt::Display for $Ty<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(self.as_str(), f)
            }
        }

        #[cfg(feature = "serde")]
        impl<T: Bos<str>> Serialize for $Ty<T> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> Deserialize<'de> for $Ty<&'de str> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = <&str>::deserialize(deserializer)?;
                $Ty::parse(s).map_err(|e| {
                    de::Error::custom(format_args!(
                        "failed to parse {s:?} as {}: {e}",
                        $name
                    ))
                })
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> Deserialize<'de> for $Ty<String> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                $Ty::parse(s).map_err(|(s, e)| {
                    de::Error::custom(format_args!(
                        "failed to parse {s:?} as {}: {e}",
                        $name
                    ))
                })
            }
        }
    };
}

/// References to the value and the metadata of an `RiMaybeRef`.
#[derive(Clone, Copy)]
pub struct RmrRef<'v, 'm> {
    val: &'v str,
    meta: &'m Meta,
}

impl<'v, 'm> RmrRef<'v, 'm> {
    pub fn new(val: &'v str, meta: &'m Meta) -> Self {
        Self { val, meta }
    }

    pub fn as_str(self) -> &'v str {
        self.val
    }

    fn slice(self, start: usize, end: usize) -> &'v str {
        &self.val[start..end]
    }

    fn eslice<E: Encoder>(self, start: usize, end: usize) -> &'v EStr<E> {
        EStr::new_validated(self.slice(start, end))
    }

    pub fn scheme_opt(self) -> Option<&'v Scheme> {
        let end = self.meta.scheme_end?.get();
        Some(Scheme::new_validated(self.slice(0, end)))
    }

    pub fn scheme(self) -> &'v Scheme {
        let end = self.meta.scheme_end.map_or(0, |i| i.get());
        Scheme::new_validated(self.slice(0, end))
    }

    pub fn authority(self) -> Option<IAuthority<'v>> {
        let mut meta = self.meta.auth_meta?;
        let start = match self.meta.scheme_end {
            Some(i) => i.get() + 3,
            None => 2,
        };
        let end = self.meta.path_bounds.0;

        meta.host_bounds.0 -= start;
        meta.host_bounds.1 -= start;

        Some(IAuthority::new(self.slice(start, end), meta))
    }

    pub fn path(self) -> &'v EStr<IPath> {
        self.eslice(self.meta.path_bounds.0, self.meta.path_bounds.1)
    }

    pub fn query(self) -> Option<&'v EStr<IQuery>> {
        let end = self.meta.query_end?.get();
        Some(self.eslice(self.meta.path_bounds.1 + 1, end))
    }

    fn fragment_start(self) -> Option<usize> {
        Some(self.meta.query_or_path_end())
            .filter(|&i| i != self.val.len())
            .map(|i| i + 1)
    }

    pub fn fragment(self) -> Option<&'v EStr<IFragment>> {
        self.fragment_start()
            .map(|i| self.eslice(i, self.val.len()))
    }

    #[cfg(feature = "alloc")]
    pub fn set_fragment(buf: &mut String, meta: &Meta, opt: Option<&str>) {
        buf.truncate(meta.query_or_path_end());
        if let Some(s) = opt {
            buf.reserve_exact(s.len() + 1);
            buf.push('#');
            buf.push_str(s);
        }
    }

    pub fn strip_fragment(self) -> &'v str {
        &self.val[..self.meta.query_or_path_end()]
    }

    #[cfg(feature = "alloc")]
    pub fn with_fragment(self, opt: Option<&str>) -> String {
        let stripped = self.strip_fragment();
        if let Some(s) = opt {
            [stripped, "#", s].concat()
        } else {
            stripped.to_owned()
        }
    }

    #[inline]
    pub fn has_scheme(self) -> bool {
        self.meta.scheme_end.is_some()
    }

    #[inline]
    pub fn has_authority(self) -> bool {
        self.meta.auth_meta.is_some()
    }

    #[inline]
    pub fn has_query(self) -> bool {
        self.meta.query_end.is_some()
    }

    #[inline]
    pub fn has_fragment(self) -> bool {
        self.meta.query_or_path_end() != self.val.len()
    }

    pub fn ensure_has_scheme(self) -> Result<(), ConvertError> {
        if self.has_scheme() {
            Ok(())
        } else {
            Err(ConvertError::NoScheme)
        }
    }

    pub fn ensure_ascii(self) -> Result<(), ConvertError> {
        match self.as_str().bytes().position(|x| !x.is_ascii()) {
            Some(index) => Err(ConvertError::NotAscii { index }),
            None => Ok(()),
        }
    }
}

ri_maybe_ref! {
    Type = Uri,
    type_name = "Uri",
    variable_name = "uri",
    name = "URI",
    indefinite_article = "a",
    description = "A URI.",
    ascii_only = true,
    scheme_required = true,
    rfc = 3986,
    abnf_rule = ("URI", "https://datatracker.ietf.org/doc/html/rfc3986#section-3"),
    RefType = UriRef,
    ref_name = "URI reference",
    AuthorityType = Authority,
    UserinfoEncoderType = Userinfo,
    RegNameEncoderType = RegName,
    PathEncoderType = Path,
    QueryEncoderType = Query,
    FragmentEncoderType = Fragment,
}

ri_maybe_ref! {
    Type = UriRef,
    type_name = "UriRef",
    variable_name = "uri_ref",
    name = "URI reference",
    indefinite_article = "a",
    description = "A URI reference, i.e., either a URI or a relative reference.",
    ascii_only = true,
    scheme_required = false,
    rfc = 3986,
    abnf_rule = ("URI-reference", "https://datatracker.ietf.org/doc/html/rfc3986#section-4.1"),
    NonRefType = Uri,
    non_ref_name = "URI",
    non_ref_link = "https://datatracker.ietf.org/doc/html/rfc3986#section-3",
    abnf_rule_absolute = ("absolute-URI", "https://datatracker.ietf.org/doc/html/rfc3986#section-4.3"),
    AuthorityType = Authority,
    UserinfoEncoderType = Userinfo,
    RegNameEncoderType = RegName,
    PathEncoderType = Path,
    QueryEncoderType = Query,
    FragmentEncoderType = Fragment,
}

ri_maybe_ref! {
    Type = Iri,
    type_name = "Iri",
    variable_name = "iri",
    name = "IRI",
    indefinite_article = "an",
    description = "An IRI.",
    ascii_only = false,
    scheme_required = true,
    rfc = 3987,
    abnf_rule = ("IRI", "https://datatracker.ietf.org/doc/html/rfc3987#section-2.2"),
    RefType = IriRef,
    ref_name = "IRI reference",
    AuthorityType = IAuthority,
    UserinfoEncoderType = IUserinfo,
    RegNameEncoderType = IRegName,
    PathEncoderType = IPath,
    QueryEncoderType = IQuery,
    FragmentEncoderType = IFragment,
}

ri_maybe_ref! {
    Type = IriRef,
    type_name = "IriRef",
    variable_name = "iri_ref",
    name = "IRI reference",
    indefinite_article = "an",
    description = "An IRI reference, i.e., either a IRI or a relative reference.",
    ascii_only = false,
    scheme_required = false,
    rfc = 3987,
    abnf_rule = ("IRI-reference", "https://datatracker.ietf.org/doc/html/rfc3987#section-2.2"),
    NonRefType = Iri,
    non_ref_name = "IRI",
    non_ref_link = "https://datatracker.ietf.org/doc/html/rfc3987#section-2.2",
    abnf_rule_absolute = ("absolute-IRI", "https://datatracker.ietf.org/doc/html/rfc3987#section-2.2"),
    AuthorityType = IAuthority,
    UserinfoEncoderType = IUserinfo,
    RegNameEncoderType = IRegName,
    PathEncoderType = IPath,
    QueryEncoderType = IQuery,
    FragmentEncoderType = IFragment,
}
