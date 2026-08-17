//! Module for normalization.

use crate::{
    component::Scheme,
    imp::{HostMeta, Meta, RiMaybeRef, RmrRef},
    parse,
    pct_enc::{
        self,
        encoder::{Data, IData},
        Decode, DecodedChunk, DecodedUtf8Chunk, Encode, EncodedChunk, Encoder, Table,
    },
    resolve,
};
use alloc::string::String;
use borrow_or_share::Bos;
use core::{
    fmt::{self, Write},
    num::NonZeroUsize,
};

/// An error occurred when normalizing a URI/IRI (reference).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NormalizeError {
    /// An underflow occurred in path normalization.
    ///
    /// Used only when [`Normalizer::allow_path_underflow`] is set to `false`.
    PathUnderflow,
}

impl fmt::Display for NormalizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::PathUnderflow => "underflow occurred in path resolution",
        };
        f.write_str(msg)
    }
}

#[cfg(feature = "impl-error")]
impl crate::Error for NormalizeError {}

/// A configurable URI/IRI (reference) normalizer.
#[derive(Clone, Copy)]
#[allow(missing_debug_implementations)]
#[must_use]
pub struct Normalizer {
    allow_path_underflow: bool,
    default_port_f: fn(&Scheme) -> Option<u16>,
}

impl Normalizer {
    /// Creates a new `Normalizer` with default configuration.
    pub fn new() -> Self {
        Self {
            allow_path_underflow: true,
            default_port_f: Scheme::default_port,
        }
    }

    /// Sets whether to allow underflow in path normalization.
    ///
    /// This defaults to `true`. A value of `false` is a deviation from the
    /// normalization methods described in
    /// [Section 6 of RFC 3986](https://datatracker.ietf.org/doc/html/rfc3986/#section-6).
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_uri::{normalize::{Normalizer, NormalizeError}, Uri};
    ///
    /// let normalizer = Normalizer::new().allow_path_underflow(false);
    /// let uri = Uri::parse("http://example.com/..")?;
    ///
    /// assert_eq!(normalizer.normalize(&uri).unwrap_err(), NormalizeError::PathUnderflow);
    /// # Ok::<_, fluent_uri::ParseError>(())
    /// ```
    pub fn allow_path_underflow(mut self, value: bool) -> Self {
        self.allow_path_underflow = value;
        self
    }

    /// Sets the function with which to get the default port of a scheme.
    ///
    /// This defaults to [`Scheme::default_port`].
    ///
    /// # Examples
    ///
    /// ```
    /// use fluent_uri::{component::Scheme, normalize::Normalizer, Uri};
    ///
    /// const SCHEME_FOO: &Scheme = Scheme::new_or_panic("foo");
    ///
    /// let normalizer = Normalizer::new().default_port_with(|scheme| {
    ///     if scheme == SCHEME_FOO {
    ///         Some(4673)
    ///     } else {
    ///         scheme.default_port()
    ///     }
    /// });
    /// let uri = Uri::parse("foo://localhost:4673")?;
    ///
    /// assert_eq!(normalizer.normalize(&uri).unwrap(), "foo://localhost");
    /// # Ok::<_, fluent_uri::ParseError>(())
    /// ```
    pub fn default_port_with(mut self, f: fn(&Scheme) -> Option<u16>) -> Self {
        self.default_port_f = f;
        self
    }

    /// Normalizes the given URI/IRI (reference).
    ///
    /// See [`Uri::normalize`][crate::Uri::normalize] for the exact behavior of this method.
    ///
    /// # Errors
    ///
    /// Returns `Err` if an underflow occurred in path normalization
    /// when [`allow_path_underflow`] is set to `false`.
    ///
    /// [`allow_path_underflow`]: Self::allow_path_underflow
    pub fn normalize<R: RiMaybeRef>(&self, r: &R) -> Result<R::WithVal<String>, NormalizeError>
    where
        R::Val: Bos<str>,
    {
        normalize(
            r.make_ref(),
            R::CONSTRAINTS.ascii_only,
            self.allow_path_underflow,
            self.default_port_f,
        )
        .map(RiMaybeRef::from_pair)
    }
}

impl Default for Normalizer {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) fn normalize(
    r: RmrRef<'_, '_>,
    ascii_only: bool,
    allow_path_underflow: bool,
    default_port_f: fn(&Scheme) -> Option<u16>,
) -> Result<(String, Meta), NormalizeError> {
    // For "a://[::ffff:5:9]/" the capacity is not enough,
    // but it's fine since this rarely happens.
    let mut buf = String::with_capacity(r.as_str().len());

    let path = r.path().as_str();
    let mut path_buf = String::with_capacity(path.len());

    let data_table = if ascii_only {
        Data::TABLE
    } else {
        IData::TABLE
    };

    if r.has_scheme() && path.starts_with('/') {
        normalize_estr(&mut buf, path, false, data_table);

        let underflow_occurred = resolve::remove_dot_segments(&mut path_buf, 0, &[&buf]);
        if underflow_occurred && !allow_path_underflow {
            return Err(NormalizeError::PathUnderflow);
        }

        buf.clear();
    } else {
        // Don't remove dot segments from relative reference or rootless path.
        normalize_estr(&mut path_buf, path, false, data_table);
    }

    let mut meta = Meta::default();

    if let Some(scheme) = r.scheme_opt() {
        buf.push_str(scheme.as_str());
        buf.make_ascii_lowercase();
        meta.scheme_end = NonZeroUsize::new(buf.len());
        buf.push(':');
    }

    if let Some(auth) = r.authority() {
        buf.push_str("//");

        if let Some(userinfo) = auth.userinfo() {
            normalize_estr(&mut buf, userinfo.as_str(), false, data_table);
            buf.push('@');
        }

        let mut auth_meta = auth.meta();
        auth_meta.host_bounds.0 = buf.len();
        match auth_meta.host_meta {
            // An IPv4 address is always canonical.
            HostMeta::Ipv4(..) => buf.push_str(auth.host()),
            #[cfg(feature = "net")]
            HostMeta::Ipv6(addr) => write!(buf, "[{addr}]").unwrap(),
            #[cfg(not(feature = "net"))]
            HostMeta::Ipv6() => {
                buf.push('[');
                write_v6(&mut buf, parse::parse_v6(&auth.host().as_bytes()[1..]));
                buf.push(']');
            }
            HostMeta::IpvFuture => {
                let start = buf.len();
                buf.push_str(auth.host());

                buf[start..].make_ascii_lowercase();
            }
            HostMeta::RegName => {
                let start = buf.len();
                let host = auth.host();
                normalize_estr(&mut buf, host, true, data_table);

                if buf.len() < start + host.len() {
                    // Only reparse when the length is less than before.
                    auth_meta.host_meta = parse::parse_v4_or_reg_name(&buf.as_bytes()[start..]);
                }
            }
        }
        auth_meta.host_bounds.1 = buf.len();
        meta.auth_meta = Some(auth_meta);

        if let Some(port) = auth.port() {
            if !port.is_empty() {
                let mut eq_default = false;
                if let Some(scheme) = r.scheme_opt() {
                    if let Some(default) = default_port_f(scheme) {
                        eq_default = port.as_str().parse().ok() == Some(default);
                    }
                }
                if !eq_default {
                    buf.push(':');
                    buf.push_str(port.as_str());
                }
            }
        }
    }

    meta.path_bounds.0 = buf.len();
    // Make sure that the output is a valid URI/IRI reference.
    if r.has_scheme() && !r.has_authority() && path_buf.starts_with("//") {
        buf.push_str("/.");
    }
    buf.push_str(&path_buf);
    meta.path_bounds.1 = buf.len();

    if let Some(query) = r.query() {
        buf.push('?');

        const IQUERY_DATA: &Table = &IData::TABLE.or_iprivate();
        let query_data_table = if ascii_only { Data::TABLE } else { IQUERY_DATA };

        normalize_estr(&mut buf, query.as_str(), false, query_data_table);
        meta.query_end = NonZeroUsize::new(buf.len());
    }

    if let Some(fragment) = r.fragment() {
        buf.push('#');
        normalize_estr(&mut buf, fragment.as_str(), false, data_table);
    }

    Ok((buf, meta))
}

fn normalize_estr(buf: &mut String, s: &str, to_ascii_lowercase: bool, table: &Table) {
    if table.allows_non_ascii() {
        Decode::new(s).decode_utf8(|chunk| match chunk {
            DecodedUtf8Chunk::Unencoded(s) => {
                let i = buf.len();
                buf.push_str(s);
                if to_ascii_lowercase {
                    buf[i..].make_ascii_lowercase();
                }
            }
            DecodedUtf8Chunk::Decoded { valid, invalid } => {
                for chunk in Encode::new(table, valid) {
                    match chunk {
                        EncodedChunk::Unencoded(s) => {
                            let i = buf.len();
                            buf.push_str(s);
                            if to_ascii_lowercase {
                                buf[i..].make_ascii_lowercase();
                            }
                        }
                        EncodedChunk::PctEncoded(s) => buf.push_str(s),
                    }
                }
                for &x in invalid {
                    buf.push_str(pct_enc::encode_byte(x));
                }
            }
        });
    } else {
        for chunk in Decode::new(s) {
            match chunk {
                DecodedChunk::Unencoded(s) => {
                    let i = buf.len();
                    buf.push_str(s);
                    if to_ascii_lowercase {
                        buf[i..].make_ascii_lowercase();
                    }
                }
                DecodedChunk::PctDecoded(mut x) => {
                    if table.allows_ascii(x) {
                        if to_ascii_lowercase {
                            x.make_ascii_lowercase();
                        }
                        buf.push(x as char);
                    } else {
                        buf.push_str(pct_enc::encode_byte(x));
                    }
                }
            }
        }
    }
}

// Taken from `impl Display for Ipv6Addr`.
#[cfg(not(feature = "net"))]
fn write_v6(buf: &mut String, segments: [u16; 8]) {
    if let [0, 0, 0, 0, 0, 0xffff, ab, cd] = segments {
        let [a, b] = ab.to_be_bytes();
        let [c, d] = cd.to_be_bytes();
        write!(buf, "::ffff:{a}.{b}.{c}.{d}").unwrap();
    } else {
        #[derive(Copy, Clone, Default)]
        struct Span {
            start: usize,
            len: usize,
        }

        // Find the inner 0 span
        let zeroes = {
            let mut longest = Span::default();
            let mut current = Span::default();

            for (i, &segment) in segments.iter().enumerate() {
                if segment == 0 {
                    if current.len == 0 {
                        current.start = i;
                    }

                    current.len += 1;

                    if current.len > longest.len {
                        longest = current;
                    }
                } else {
                    current = Span::default();
                }
            }

            longest
        };

        /// Write a colon-separated part of the address
        #[inline]
        fn write_subslice(buf: &mut String, chunk: &[u16]) {
            if let Some((first, tail)) = chunk.split_first() {
                write!(buf, "{first:x}").unwrap();
                for segment in tail {
                    write!(buf, ":{segment:x}").unwrap();
                }
            }
        }

        if zeroes.len > 1 {
            write_subslice(buf, &segments[..zeroes.start]);
            buf.push_str("::");
            write_subslice(buf, &segments[zeroes.start + zeroes.len..]);
        } else {
            write_subslice(buf, &segments);
        }
    }
}
