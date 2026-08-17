use super::BuildError;
use crate::{
    component::IAuthority,
    imp::{AuthMeta, HostMeta, Meta},
    parse,
    pct_enc::{
        encoder::{IRegName, Port, RegName},
        EStr,
    },
};
use alloc::string::String;
use core::{fmt::Write, num::NonZeroUsize};

#[cfg(feature = "net")]
use crate::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub struct BuilderInner {
    pub buf: String,
    pub meta: Meta,
}

impl BuilderInner {
    pub fn push_scheme(&mut self, v: &str) {
        self.buf.push_str(v);
        self.meta.scheme_end = NonZeroUsize::new(self.buf.len());
        self.buf.push(':');
    }

    pub fn start_authority(&mut self) {
        self.buf.push_str("//");
    }

    pub fn push_authority(&mut self, v: IAuthority<'_>) {
        self.buf.push_str("//");
        let start = self.buf.len();
        self.buf.push_str(v.as_str());

        let mut meta = v.meta();
        meta.host_bounds.0 += start;
        meta.host_bounds.1 += start;
        self.meta.auth_meta = Some(meta);
    }

    pub fn push_userinfo(&mut self, v: &str) {
        self.buf.push_str(v);
        self.buf.push('@');
    }

    pub fn push_host(&mut self, meta: HostMeta, f: impl FnOnce(&mut String)) {
        let start = self.buf.len();
        f(&mut self.buf);
        self.meta.auth_meta = Some(AuthMeta {
            host_bounds: (start, self.buf.len()),
            host_meta: meta,
        });
    }

    pub fn push_path(&mut self, v: &str) {
        self.meta.path_bounds.0 = self.buf.len();
        self.buf.push_str(v);
        self.meta.path_bounds.1 = self.buf.len();
    }

    pub fn push_query(&mut self, v: &str) {
        self.buf.push('?');
        self.buf.push_str(v);
        self.meta.query_end = NonZeroUsize::new(self.buf.len());
    }

    pub fn push_fragment(&mut self, v: &str) {
        self.buf.push('#');
        self.buf.push_str(v);
    }

    pub fn validate(&self) -> Result<(), BuildError> {
        fn first_segment_contains_colon(path: &str) -> bool {
            path.split_once('/').map_or(path, |x| x.0).contains(':')
        }

        let (start, end) = self.meta.path_bounds;
        let path = &self.buf[start..end];

        if self.meta.auth_meta.is_some() {
            if !path.is_empty() && !path.starts_with('/') {
                return Err(BuildError::NonemptyRootlessPath);
            }
        } else {
            if path.starts_with("//") {
                return Err(BuildError::PathStartsWithDoubleSlash);
            }
            if self.meta.scheme_end.is_none() && first_segment_contains_colon(path) {
                return Err(BuildError::FirstPathSegmentContainsColon);
            }
        }
        Ok(())
    }
}

pub trait AsHost<'a> {
    fn push_to(self, b: &mut BuilderInner);
}

#[cfg(feature = "net")]
impl<'a> AsHost<'a> for Ipv4Addr {
    fn push_to(self, b: &mut BuilderInner) {
        b.push_host(HostMeta::Ipv4(self), |buf| {
            write!(buf, "{self}").unwrap();
        });
    }
}

#[cfg(feature = "net")]
impl<'a> AsHost<'a> for Ipv6Addr {
    fn push_to(self, b: &mut BuilderInner) {
        b.push_host(HostMeta::Ipv6(self), |buf| {
            write!(buf, "[{self}]").unwrap();
        });
    }
}

#[cfg(feature = "net")]
impl<'a> AsHost<'a> for IpAddr {
    fn push_to(self, b: &mut BuilderInner) {
        match self {
            IpAddr::V4(addr) => addr.push_to(b),
            IpAddr::V6(addr) => addr.push_to(b),
        }
    }
}

impl<'a> AsHost<'a> for &'a EStr<RegName> {
    #[inline]
    fn push_to(self, b: &mut BuilderInner) {
        self.cast::<IRegName>().push_to(b);
    }
}

impl<'a> AsHost<'a> for &'a EStr<IRegName> {
    fn push_to(self, b: &mut BuilderInner) {
        let meta = parse::parse_v4_or_reg_name(self.as_str().as_bytes());
        b.push_host(meta, |buf| {
            buf.push_str(self.as_str());
        });
    }
}

pub trait WithEncoder<E> {}

#[cfg(feature = "net")]
impl<E> WithEncoder<E> for Ipv4Addr {}
#[cfg(feature = "net")]
impl<E> WithEncoder<E> for Ipv6Addr {}
#[cfg(feature = "net")]
impl<E> WithEncoder<E> for IpAddr {}

impl WithEncoder<RegName> for &EStr<RegName> {}
impl WithEncoder<IRegName> for &EStr<IRegName> {}

pub trait AsPort {
    fn push_to(self, buf: &mut String);
}

impl AsPort for u16 {
    fn push_to(self, buf: &mut String) {
        write!(buf, ":{self}").unwrap();
    }
}

impl AsPort for &EStr<Port> {
    fn push_to(self, buf: &mut String) {
        buf.push(':');
        buf.push_str(self.as_str());
    }
}
