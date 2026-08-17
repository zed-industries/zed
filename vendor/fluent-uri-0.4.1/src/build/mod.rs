//! Module for URI/IRI (reference) building.

#![allow(missing_debug_implementations)]

mod imp;
pub(crate) mod state;

use imp::*;
use state::*;

use crate::{
    component::{Authority, Scheme},
    imp::{Meta, RiMaybeRef},
    pct_enc::EStr,
};
use alloc::string::String;
use core::{fmt, marker::PhantomData};

/// An error occurred when building a URI/IRI (reference).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuildError {
    /// Authority is present, but the path is not empty and does not start with `'/'`.
    NonemptyRootlessPath,
    /// Authority is not present, but the path starts with `"//"`.
    PathStartsWithDoubleSlash,
    /// Neither scheme nor authority is present, but the first path segment contains `':'`.
    FirstPathSegmentContainsColon,
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::NonemptyRootlessPath => {
                "when authority is present, path should either be empty or start with '/'"
            }
            Self::PathStartsWithDoubleSlash => {
                "when authority is not present, path should not start with \"//\""
            }
            Self::FirstPathSegmentContainsColon => {
                "when neither scheme nor authority is present, first path segment should not contain ':'"
            }
        };
        f.write_str(msg)
    }
}

#[cfg(feature = "impl-error")]
impl crate::Error for BuildError {}

/// A builder for URI/IRI (reference).
///
/// This struct is created by the `builder` associated
/// functions on [`Uri`], [`UriRef`], [`Iri`], and [`IriRef`].
///
/// [`Uri`]: crate::Uri
/// [`UriRef`]: crate::UriRef
/// [`Iri`]: crate::Iri
/// [`IriRef`]: crate::IriRef
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use fluent_uri::{component::Scheme, pct_enc::EStr, Uri};
///
/// const SCHEME_FOO: &Scheme = Scheme::new_or_panic("foo");
///
/// let uri = Uri::builder()
///     .scheme(SCHEME_FOO)
///     .authority_with(|b| {
///         b.userinfo(EStr::new_or_panic("user"))
///             .host(EStr::new_or_panic("example.com"))
///             .port(8042)
///     })
///     .path(EStr::new_or_panic("/over/there"))
///     .query(EStr::new_or_panic("name=ferret"))
///     .fragment(EStr::new_or_panic("nose"))
///     .build()
///     .unwrap();
///
/// assert_eq!(
///     uri.as_str(),
///     "foo://user@example.com:8042/over/there?name=ferret#nose"
/// );
/// ```
///
/// Note that [`EStr::new_or_panic`] *panics* on invalid input and
/// should normally be used with constant strings.
/// If you want to build a percent-encoded string from scratch,
/// use [`EString`] instead.
///
/// [`EString`]: crate::pct_enc::EString
///
/// # Constraints
///
/// Typestates are used to avoid misconfigurations,
/// which puts the following constraints:
///
/// - Components must be set from start to end, no repetition allowed.
/// - Setting [`scheme`] is mandatory when building a URI/IRI.
/// - Setting [`path`] is mandatory.
/// - Methods [`userinfo`], [`host`], and [`port`] are only available
///   within a call to [`authority_with`].
/// - Setting [`host`] is mandatory within a call to [`authority_with`].
///
/// You may otherwise skip setting optional components
/// (scheme, authority, userinfo, port, query, and fragment)
/// with [`advance`] or set them optionally with [`optional`].
///
/// The builder typestates are currently private. Please open an issue
/// if it is a problem not being able to name the type of a builder.
///
/// [`advance`]: Self::advance
/// [`optional`]: Self::optional
/// [`scheme`]: Self::scheme
/// [`authority_with`]: Self::authority_with
/// [`userinfo`]: Self::userinfo
/// [`host`]: Self::host
/// [`port`]: Self::port
/// [`path`]: Self::path
/// [`build`]: Self::build
#[must_use]
pub struct Builder<R, S> {
    inner: BuilderInner,
    _marker: PhantomData<(R, S)>,
}

impl<R, S> Builder<R, S> {
    pub(crate) fn new() -> Self {
        Self {
            inner: BuilderInner {
                buf: String::new(),
                meta: Meta::default(),
            },
            _marker: PhantomData,
        }
    }
}

impl<R, S> Builder<R, S> {
    fn cast<T>(self) -> Builder<R, T>
    where
        S: To<T>,
    {
        Builder {
            inner: self.inner,
            _marker: PhantomData,
        }
    }

    /// Advances the builder state, skipping optional components in between.
    ///
    /// Variable rebinding may be necessary as this changes the type of the builder.
    ///
    /// ```
    /// use fluent_uri::{component::Scheme, pct_enc::EStr, UriRef};
    ///
    /// fn build(relative: bool) -> UriRef<String> {
    ///     let b = UriRef::builder();
    ///     let b = if relative {
    ///         b.advance()
    ///     } else {
    ///         b.scheme(Scheme::new_or_panic("http"))
    ///             .authority_with(|b| b.host(EStr::new_or_panic("example.com")))
    ///     };
    ///     b.path(EStr::new_or_panic("/foo")).build().unwrap()
    /// }
    ///
    /// assert_eq!(build(false).as_str(), "http://example.com/foo");
    /// assert_eq!(build(true).as_str(), "/foo");
    /// ```
    pub fn advance<T>(self) -> Builder<R, T>
    where
        S: AdvanceTo<T>,
    {
        self.cast()
    }

    /// Optionally calls a builder method with a value.
    ///
    /// ```
    /// use fluent_uri::{build::Builder, pct_enc::EStr, UriRef};
    ///
    /// let uri_ref = UriRef::builder()
    ///     .path(EStr::new_or_panic("foo"))
    ///     .optional(Builder::query, Some(EStr::new_or_panic("bar")))
    ///     .optional(Builder::fragment, None)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(uri_ref.as_str(), "foo?bar");
    /// ```
    pub fn optional<F, V, T>(self, f: F, opt: Option<V>) -> Builder<R, T>
    where
        F: FnOnce(Self, V) -> Builder<R, T>,
        S: AdvanceTo<T>,
    {
        match opt {
            Some(value) => f(self, value),
            None => self.advance(),
        }
    }
}

impl<R, S: To<SchemeEnd>> Builder<R, S> {
    /// Sets the [scheme] component.
    ///
    /// Note that the scheme component is *case-insensitive* and its canonical form is
    /// *lowercase*. For consistency, you should only produce lowercase scheme names.
    ///
    /// [scheme]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.1
    pub fn scheme(mut self, scheme: &Scheme) -> Builder<R, SchemeEnd> {
        self.inner.push_scheme(scheme.as_str());
        self.cast()
    }
}

impl<R: RiMaybeRef, S: To<AuthorityStart>> Builder<R, S> {
    /// Builds the [authority] component with the given function.
    ///
    /// [authority]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.2
    pub fn authority_with<F, T>(mut self, f: F) -> Builder<R, AuthorityEnd>
    where
        F: FnOnce(Builder<R, AuthorityStart>) -> Builder<R, T>,
        T: To<AuthorityEnd>,
    {
        self.inner.start_authority();
        f(self.cast()).cast()
    }

    /// Sets the [authority] component.
    ///
    /// This method takes an [`Authority`] (for URI) or [`IAuthority`] (for IRI) as argument.
    ///
    /// This method is normally used with an authority which is empty ([`Authority::EMPTY`])
    /// or is obtained from a URI/IRI (reference). If you need to build an authority from its
    /// subcomponents (userinfo, host, and port), use [`authority_with`] instead.
    ///
    /// [authority]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.2
    /// [`IAuthority`]: crate::component::IAuthority
    /// [`authority_with`]: Self::authority_with
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_uri::{
    ///     build::Builder,
    ///     component::{Authority, Scheme},
    ///     pct_enc::EStr,
    ///     Uri,
    /// };
    ///
    /// let uri = Uri::builder()
    ///     .scheme(Scheme::new_or_panic("file"))
    ///     .authority(Authority::EMPTY)
    ///     .path(EStr::new_or_panic("/path/to/file"))
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(uri, "file:///path/to/file");
    ///
    /// let auth = Uri::parse("foo://user@example.com:8042")?
    ///     .authority()
    ///     .unwrap();
    /// let uri = Uri::builder()
    ///     .scheme(Scheme::new_or_panic("http"))
    ///     .authority(auth)
    ///     .path(EStr::EMPTY)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(uri, "http://user@example.com:8042");
    /// # Ok::<_, fluent_uri::ParseError>(())
    /// ```
    pub fn authority(
        mut self,
        authority: Authority<'_, R::UserinfoE, R::RegNameE>,
    ) -> Builder<R, AuthorityEnd> {
        self.inner.push_authority(authority.cast());
        self.cast::<AuthorityEnd>()
    }
}

impl<R: RiMaybeRef, S: To<UserinfoEnd>> Builder<R, S> {
    /// Sets the [userinfo][userinfo-spec] subcomponent of authority.
    ///
    /// This method takes an <code>&amp;[EStr]&lt;[Userinfo]&gt;</code> (for URI)
    /// or <code>&amp;[EStr]&lt;[IUserinfo]&gt;</code> (for IRI) as argument.
    ///
    /// [userinfo-spec]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.1
    /// [Userinfo]: crate::pct_enc::encoder::Userinfo
    /// [IUserinfo]: crate::pct_enc::encoder::IUserinfo
    pub fn userinfo(mut self, userinfo: &EStr<R::UserinfoE>) -> Builder<R, UserinfoEnd> {
        self.inner.push_userinfo(userinfo.as_str());
        self.cast()
    }
}

impl<R: RiMaybeRef, S: To<HostEnd>> Builder<R, S> {
    /// Sets the [host] subcomponent of authority.
    ///
    /// This method takes either an [`Ipv4Addr`], [`Ipv6Addr`], [`IpAddr`],
    /// <code>&amp;[EStr]&lt;[RegName]&gt;</code> (for URI)
    /// or <code>&amp;[EStr]&lt;[IRegName]&gt;</code> (for IRI) as argument.
    /// Crate feature `net` is required for this method to take an IP address as argument.
    ///
    /// If the contents of an input `EStr` slice matches the
    /// `IPv4address` ABNF rule defined in [Section 3.2.2 of RFC 3986][host],
    /// the resulting URI/IRI (reference) will output a [`Host::Ipv4`] variant instead.
    ///
    /// Note that ASCII characters within a host are *case-insensitive*.
    /// For consistency, you should only produce [normalized] hosts.
    ///
    /// [host]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.2
    /// [`Ipv4Addr`]: core::net::Ipv4Addr
    /// [`Ipv6Addr`]: core::net::Ipv6Addr
    /// [`IpAddr`]: core::net::IpAddr
    /// [RegName]: crate::pct_enc::encoder::RegName
    /// [IRegName]: crate::pct_enc::encoder::IRegName
    /// [`Host::Ipv4`]: crate::component::Host::Ipv4
    /// [normalized]: crate::Uri::normalize
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_uri::{component::Host, pct_enc::EStr, UriRef};
    ///
    /// let uri_ref = UriRef::builder()
    ///     .authority_with(|b| b.host(EStr::new_or_panic("127.0.0.1")))
    ///     .path(EStr::EMPTY)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(matches!(uri_ref.authority().unwrap().host_parsed(), Host::Ipv4 { .. }));
    /// ```
    pub fn host<'a>(
        mut self,
        host: impl AsHost<'a> + WithEncoder<R::RegNameE>,
    ) -> Builder<R, HostEnd> {
        host.push_to(&mut self.inner);
        self.cast()
    }
}

impl<R, S: To<PortEnd>> Builder<R, S> {
    /// Sets the [port][port-spec] subcomponent of authority.
    ///
    /// This method takes either a `u16` or <code>&amp;[EStr]&lt;[Port]&gt;</code> as argument.
    ///
    /// For consistency, you should not produce an empty or [default] port.
    ///
    /// [port-spec]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.3
    /// [Port]: crate::pct_enc::encoder::Port
    /// [default]: Scheme::default_port
    pub fn port(mut self, port: impl AsPort) -> Builder<R, PortEnd> {
        port.push_to(&mut self.inner.buf);
        self.cast()
    }

    /// Sets the [port] subcomponent of authority, omitting it when it equals the default value.
    ///
    /// [port]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.2.3
    #[cfg(fluent_uri_unstable)]
    pub fn port_with_default(self, port: u16, default: u16) -> Builder<R, PortEnd> {
        if port != default {
            self.cast()
        } else {
            self.port(port)
        }
    }
}

impl<R: RiMaybeRef, S: To<PathEnd>> Builder<R, S> {
    /// Sets the [path][path-spec] component.
    ///
    /// This method takes an <code>&amp;[EStr]&lt;[Path]&gt;</code> (for URI)
    /// or <code>&amp;[EStr]&lt;[IPath]&gt;</code> (for IRI) as argument.
    ///
    /// [path-spec]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.3
    /// [Path]: crate::pct_enc::encoder::Path
    /// [IPath]: crate::pct_enc::encoder::IPath
    pub fn path(mut self, path: &EStr<R::PathE>) -> Builder<R, PathEnd> {
        self.inner.push_path(path.as_str());
        self.cast()
    }
}

impl<R: RiMaybeRef, S: To<QueryEnd>> Builder<R, S> {
    /// Sets the [query][query-spec] component.
    ///
    /// This method takes an <code>&amp;[EStr]&lt;[Query]&gt;</code> (for URI)
    /// or <code>&amp;[EStr]&lt;[IQuery]&gt;</code> (for IRI) as argument.
    ///
    /// [query-spec]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.4
    /// [Query]: crate::pct_enc::encoder::Query
    /// [IQuery]: crate::pct_enc::encoder::IQuery
    pub fn query(mut self, query: &EStr<R::QueryE>) -> Builder<R, QueryEnd> {
        self.inner.push_query(query.as_str());
        self.cast()
    }
}

impl<R: RiMaybeRef, S: To<FragmentEnd>> Builder<R, S> {
    /// Sets the [fragment][fragment-spec] component.
    ///
    /// This method takes an <code>&amp;[EStr]&lt;[Fragment]&gt;</code> (for URI)
    /// or <code>&amp;[EStr]&lt;[IFragment]&gt;</code> (for IRI) as argument.
    ///
    /// [fragment-spec]: https://datatracker.ietf.org/doc/html/rfc3986#section-3.5
    /// [Fragment]: crate::pct_enc::encoder::Fragment
    /// [IFragment]: crate::pct_enc::encoder::IFragment
    pub fn fragment(mut self, fragment: &EStr<R::FragmentE>) -> Builder<R, FragmentEnd> {
        self.inner.push_fragment(fragment.as_str());
        self.cast()
    }
}

impl<R: RiMaybeRef<Val = String>, S: To<End>> Builder<R, S> {
    /// Builds the URI/IRI (reference).
    ///
    /// # Errors
    ///
    /// Returns `Err` if any of the following conditions is not met.
    ///
    /// - When authority is present, the path must either be empty or start with `'/'`.
    /// - When authority is not present, the path cannot start with `"//"`.
    /// - When neither scheme nor authority is present, the first path segment cannot contain `':'`.
    ///
    /// [rel-ref]: https://datatracker.ietf.org/doc/html/rfc3986#section-4.2
    pub fn build(self) -> Result<R, BuildError> {
        self.inner
            .validate()
            .map(|()| R::new(self.inner.buf, self.inner.meta))
    }
}
