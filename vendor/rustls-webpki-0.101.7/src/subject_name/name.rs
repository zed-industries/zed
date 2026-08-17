// Copyright 2015-2020 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use crate::DnsNameRef;

use super::ip_address::{self, IpAddrRef};

/// A DNS name or IP address, which borrows its text representation.
#[derive(Debug, Clone, Copy)]
pub enum SubjectNameRef<'a> {
    /// A valid DNS name
    DnsName(DnsNameRef<'a>),

    /// A valid IP address
    IpAddress(IpAddrRef<'a>),
}

/// An error indicating that a `SubjectNameRef` could not built
/// because the input is not a syntactically-valid DNS Name or IP
/// address.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidSubjectNameError;

impl<'a> SubjectNameRef<'a> {
    /// Attempts to decode an encodingless string as either an IPv4 address, IPv6 address or
    /// DNS name; in that order.  In practice this space is non-overlapping because
    /// DNS name components are separated by periods but cannot be wholly numeric (so cannot
    /// overlap with a valid IPv4 address), and IPv6 addresses are separated by colons but
    /// cannot contain periods.
    ///
    /// The IPv6 address encoding supported here is extremely simplified; it does not support
    /// compression, all leading zeroes must be present in each 16-bit word, etc.  Generally
    /// this is not suitable as a parse for human-provided addresses for this reason.  Instead:
    /// consider parsing these with `std::net::IpAddr` and then using
    /// `IpAddr::from<std::net::IpAddr>`.
    pub fn try_from_ascii(subject_name: &'a [u8]) -> Result<Self, InvalidSubjectNameError> {
        if let Ok(ip_address) = ip_address::parse_ipv4_address(subject_name) {
            return Ok(SubjectNameRef::IpAddress(ip_address));
        } else if let Ok(ip_address) = ip_address::parse_ipv6_address(subject_name) {
            return Ok(SubjectNameRef::IpAddress(ip_address));
        } else {
            Ok(SubjectNameRef::DnsName(
                DnsNameRef::try_from_ascii(subject_name).map_err(|_| InvalidSubjectNameError)?,
            ))
        }
    }

    /// Constructs a `SubjectNameRef` from the given input if the
    /// input is a syntactically-valid DNS name or IP address.
    pub fn try_from_ascii_str(subject_name: &'a str) -> Result<Self, InvalidSubjectNameError> {
        Self::try_from_ascii(subject_name.as_bytes())
    }
}

impl<'a> From<DnsNameRef<'a>> for SubjectNameRef<'a> {
    fn from(dns_name: DnsNameRef<'a>) -> SubjectNameRef {
        SubjectNameRef::DnsName(DnsNameRef(dns_name.0))
    }
}

impl<'a> From<IpAddrRef<'a>> for SubjectNameRef<'a> {
    fn from(dns_name: IpAddrRef<'a>) -> SubjectNameRef {
        match dns_name {
            IpAddrRef::V4(ip_address, ip_address_octets) => {
                SubjectNameRef::IpAddress(IpAddrRef::V4(ip_address, ip_address_octets))
            }
            IpAddrRef::V6(ip_address, ip_address_octets) => {
                SubjectNameRef::IpAddress(IpAddrRef::V6(ip_address, ip_address_octets))
            }
        }
    }
}

impl AsRef<[u8]> for SubjectNameRef<'_> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        match self {
            SubjectNameRef::DnsName(dns_name) => dns_name.0,
            SubjectNameRef::IpAddress(ip_address) => match ip_address {
                IpAddrRef::V4(ip_address, _) | IpAddrRef::V6(ip_address, _) => ip_address,
            },
        }
    }
}
