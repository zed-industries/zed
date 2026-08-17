use crate::{
    component::{Authority, Host, Scheme},
    parse::{ParseError, ParseErrorKind},
    pct_enc::{EStr, Encoder},
    ConvertError,
};
use core::fmt::{Debug, Display, Formatter, Result};

impl<E: Encoder> Debug for EStr<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(self.as_str(), f)
    }
}

impl<E: Encoder> Display for EStr<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self.as_str(), f)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let msg = match self.kind {
            ParseErrorKind::InvalidPctEncodedOctet => "invalid percent-encoded octet at index ",
            ParseErrorKind::UnexpectedChar => "unexpected character at index ",
            ParseErrorKind::InvalidIpv6Addr => "invalid IPv6 address at index ",
        };
        write!(f, "{}{}", msg, self.index)
    }
}

impl Display for ConvertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::NotAscii { index } => write!(f, "non-ASCII character at index {index}"),
            Self::NoScheme => f.write_str("scheme not present"),
        }
    }
}

impl Debug for Scheme {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(self.as_str(), f)
    }
}

impl Display for Scheme {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self.as_str(), f)
    }
}

impl<RegNameE: Encoder> Debug for Host<'_, RegNameE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            #[cfg(feature = "net")]
            Host::Ipv4(addr) => f.debug_tuple("Ipv4").field(addr).finish(),
            #[cfg(feature = "net")]
            Host::Ipv6(addr) => f.debug_tuple("Ipv6").field(addr).finish(),

            #[cfg(not(feature = "net"))]
            Host::Ipv4() => f.debug_struct("Ipv4").finish_non_exhaustive(),
            #[cfg(not(feature = "net"))]
            Host::Ipv6() => f.debug_struct("Ipv6").finish_non_exhaustive(),

            Host::IpvFuture => f.debug_struct("IpvFuture").finish_non_exhaustive(),
            Host::RegName(name) => f.debug_tuple("RegName").field(name).finish(),
        }
    }
}

impl<UserinfoE: Encoder, RegNameE: Encoder> Debug for Authority<'_, UserinfoE, RegNameE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Authority")
            .field("userinfo", &self.userinfo())
            .field("host", &self.host())
            .field("host_parsed", &self.host_parsed())
            .field("port", &self.port())
            .finish()
    }
}

impl Display for Authority<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self.as_str(), f)
    }
}
