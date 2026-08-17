//! Module for reference resolution.

use crate::imp::{Meta, Ri, RiMaybeRef, RmrRef};
use alloc::string::String;
use borrow_or_share::Bos;
use core::{fmt, num::NonZeroUsize};

/// An error occurred when resolving a URI/IRI reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResolveError {
    /// The base has a fragment.
    BaseWithFragment,
    /// The base has no authority and its path is rootless, but the reference
    /// is relative, is not empty and does not start with `'#'`.
    InvalidReferenceAgainstOpaqueBase,
    /// An underflow occurred in path resolution.
    ///
    /// Used only when [`Resolver::allow_path_underflow`] is set to `false`.
    PathUnderflow,
}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::BaseWithFragment => "base should not have fragment",
            Self::InvalidReferenceAgainstOpaqueBase => {
                "when base has a rootless path and no authority, reference should either have scheme, be empty or start with '#'"
            }
            Self::PathUnderflow => "underflow occurred in path resolution",
        };
        f.write_str(msg)
    }
}

#[cfg(feature = "impl-error")]
impl crate::Error for ResolveError {}

/// A configurable URI/IRI reference resolver against a fixed base.
///
/// # Examples
///
/// ```
/// use fluent_uri::{resolve::Resolver, Uri, UriRef};
///
/// let base = Uri::parse("http://example.com/foo/bar")?;
/// let resolver = Resolver::with_base(base);
///
/// assert_eq!(resolver.resolve(&UriRef::parse("baz")?).unwrap(), "http://example.com/foo/baz");
/// assert_eq!(resolver.resolve(&UriRef::parse("../baz")?).unwrap(), "http://example.com/baz");
/// assert_eq!(resolver.resolve(&UriRef::parse("?baz")?).unwrap(), "http://example.com/foo/bar?baz");
/// # Ok::<_, fluent_uri::ParseError>(())
/// ```
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct Resolver<R> {
    base: R,
    allow_path_underflow: bool,
}

impl<R: Ri> Resolver<R>
where
    R::Val: Bos<str>,
{
    /// Creates a new `Resolver` with the given base and default configuration.
    pub fn with_base(base: R) -> Self {
        Self {
            base,
            allow_path_underflow: true,
        }
    }

    /// Sets whether to allow underflow in path resolution.
    ///
    /// This defaults to `true`. A value of `false` is a deviation from the
    /// reference resolution algorithm defined in
    /// [Section 5 of RFC 3986](https://datatracker.ietf.org/doc/html/rfc3986/#section-5).
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_uri::{resolve::{Resolver, ResolveError}, Uri, UriRef};
    ///
    /// let base = Uri::parse("http://example.com/foo/bar")?;
    /// let resolver = Resolver::with_base(base).allow_path_underflow(false);
    ///
    /// assert_eq!(resolver.resolve(&UriRef::parse("../../baz")?).unwrap_err(), ResolveError::PathUnderflow);
    /// assert_eq!(resolver.resolve(&UriRef::parse("../../../baz")?).unwrap_err(), ResolveError::PathUnderflow);
    /// assert_eq!(resolver.resolve(&UriRef::parse("/../baz")?).unwrap_err(), ResolveError::PathUnderflow);
    /// # Ok::<_, fluent_uri::ParseError>(())
    /// ```
    pub fn allow_path_underflow(mut self, value: bool) -> Self {
        self.allow_path_underflow = value;
        self
    }

    /// Resolves the given reference against the configured base.
    ///
    /// See [`resolve_against`] for the exact behavior of this method.
    ///
    /// # Errors
    ///
    /// Returns `Err` on the same conditions as [`resolve_against`] or if an underflow
    /// occurred in path resolution when [`allow_path_underflow`] is set to `false`.
    ///
    /// [`resolve_against`]: crate::UriRef::resolve_against
    /// [`allow_path_underflow`]: Self::allow_path_underflow
    pub fn resolve<T: Bos<str>>(
        &self,
        reference: &R::Ref<T>,
    ) -> Result<R::WithVal<String>, ResolveError> {
        resolve(
            self.base.make_ref(),
            reference.make_ref(),
            self.allow_path_underflow,
        )
        .map(RiMaybeRef::from_pair)
    }
}

pub(crate) fn resolve(
    base: RmrRef<'_, '_>,
    /* reference */ r: RmrRef<'_, '_>,
    allow_path_underflow: bool,
) -> Result<(String, Meta), ResolveError> {
    assert!(base.has_scheme());

    if base.has_fragment() {
        return Err(ResolveError::BaseWithFragment);
    }
    if !base.has_authority()
        && base.path().is_rootless()
        && !r.has_scheme()
        && !matches!(r.as_str().bytes().next(), None | Some(b'#'))
    {
        return Err(ResolveError::InvalidReferenceAgainstOpaqueBase);
    }

    let (t_scheme, t_authority, t_path, t_query, t_fragment);

    let r_scheme = r.scheme_opt();
    let r_authority = r.authority();
    let r_path = r.path();
    let r_query = r.query();
    let r_fragment = r.fragment();

    if let Some(r_scheme) = r_scheme {
        t_scheme = r_scheme;
        t_authority = r_authority;
        t_path = (r_path.as_str(), None);
        t_query = r_query;
    } else {
        if r_authority.is_some() {
            t_authority = r_authority;
            t_path = (r_path.as_str(), None);
            t_query = r_query;
        } else {
            if r_path.is_empty() {
                t_path = (base.path().as_str(), None);
                if r_query.is_some() {
                    t_query = r_query;
                } else {
                    t_query = base.query();
                }
            } else {
                if r_path.is_absolute() {
                    t_path = (r_path.as_str(), None);
                } else {
                    let base_path = base.path();
                    let base_path = if base_path.is_empty() {
                        "/"
                    } else {
                        base_path.as_str()
                    };

                    // Make sure that swapping the order of resolution and normalization
                    // does not change the result.
                    let last_slash_i = base_path.rfind('/').unwrap();
                    let last_seg = &base_path[last_slash_i + 1..];
                    let base_path_stripped = match classify_segment(last_seg) {
                        SegKind::DoubleDot => base_path,
                        _ => &base_path[..=last_slash_i],
                    };

                    // Instead of merging the paths, remove dot segments incrementally.
                    t_path = (base_path_stripped, Some(r_path.as_str()));
                }
                t_query = r_query;
            }
            t_authority = base.authority();
        }
        t_scheme = base.scheme();
    }
    t_fragment = r_fragment;

    // Calculate the output length.
    let mut len = t_scheme.as_str().len() + 1;
    if let Some(authority) = t_authority {
        len += authority.as_str().len() + 2;
    }
    len += t_path.0.len() + t_path.1.map_or(0, |s| s.len());
    if let Some(query) = t_query {
        len += query.len() + 1;
    }
    if let Some(fragment) = t_fragment {
        len += fragment.len() + 1;
    }

    let mut buf = String::with_capacity(len);
    let mut meta = Meta::default();

    buf.push_str(t_scheme.as_str());
    meta.scheme_end = NonZeroUsize::new(buf.len());
    buf.push(':');

    if let Some(authority) = t_authority {
        let mut auth_meta = authority.meta();
        buf.push_str("//");

        auth_meta.host_bounds.0 += buf.len();
        auth_meta.host_bounds.1 += buf.len();

        buf.push_str(authority.as_str());
        meta.auth_meta = Some(auth_meta);
    }

    let path_start = buf.len();
    meta.path_bounds.0 = path_start;

    if t_path.0.starts_with('/') {
        let path = [t_path.0, t_path.1.unwrap_or("")];
        let path = &path[..t_path.1.is_some() as usize + 1];

        let underflow_occurred = remove_dot_segments(&mut buf, path_start, path);
        if underflow_occurred && !allow_path_underflow {
            return Err(ResolveError::PathUnderflow);
        }
    } else {
        buf.push_str(t_path.0);
    }

    // Close the loophole in the original algorithm.
    if t_authority.is_none() && buf[path_start..].starts_with("//") {
        buf.insert_str(path_start, "/.");
    }

    meta.path_bounds.1 = buf.len();

    if let Some(query) = t_query {
        buf.push('?');
        buf.push_str(query.as_str());
        meta.query_end = NonZeroUsize::new(buf.len());
    }

    if let Some(fragment) = t_fragment {
        buf.push('#');
        buf.push_str(fragment.as_str());
    }

    debug_assert!(buf.len() <= len);

    Ok((buf, meta))
}

pub(crate) fn remove_dot_segments(buf: &mut String, start: usize, path: &[&str]) -> bool {
    let mut underflow_occurred = false;
    for seg in path.iter().flat_map(|s| s.split_inclusive('/')) {
        let seg_stripped = seg.strip_suffix('/').unwrap_or(seg);
        match classify_segment(seg_stripped) {
            SegKind::Dot => {}
            SegKind::DoubleDot => {
                if buf.len() > start + 1 {
                    buf.truncate(buf[..buf.len() - 1].rfind('/').unwrap() + 1);
                } else {
                    underflow_occurred = true;
                }
            }
            SegKind::Normal => buf.push_str(seg),
        }
    }
    underflow_occurred
}

enum SegKind {
    Dot,
    DoubleDot,
    Normal,
}

fn classify_segment(mut seg: &str) -> SegKind {
    if seg.is_empty() {
        return SegKind::Normal;
    }
    if let Some(rem) = seg.strip_prefix('.') {
        seg = rem;
    } else if let Some(rem) = seg.strip_prefix("%2E") {
        seg = rem;
    } else if let Some(rem) = seg.strip_prefix("%2e") {
        seg = rem;
    }
    if seg.is_empty() {
        SegKind::Dot
    } else if seg == "." || seg == "%2E" || seg == "%2e" {
        SegKind::DoubleDot
    } else {
        SegKind::Normal
    }
}
