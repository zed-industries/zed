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

#[cfg(feature = "std")]
use core::fmt::Write;

use crate::Error;

#[cfg(feature = "alloc")]
use alloc::string::String;

const VALID_IP_BY_CONSTRUCTION: &str = "IP address is a valid string by construction";

/// Either a IPv4 or IPv6 address, plus its owned string representation
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum IpAddr {
    /// An IPv4 address and its owned string representation
    V4(String, [u8; 4]),
    /// An IPv6 address and its owned string representation
    V6(String, [u8; 16]),
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl AsRef<str> for IpAddr {
    fn as_ref(&self) -> &str {
        match self {
            IpAddr::V4(ip_address, _) | IpAddr::V6(ip_address, _) => ip_address.as_str(),
        }
    }
}

/// Either a IPv4 or IPv6 address, plus its borrowed string representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpAddrRef<'a> {
    /// An IPv4 address and its borrowed string representation
    V4(&'a [u8], [u8; 4]),
    /// An IPv6 address and its borrowed string representation
    V6(&'a [u8], [u8; 16]),
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a> From<IpAddrRef<'a>> for IpAddr {
    fn from(ip_address: IpAddrRef<'a>) -> IpAddr {
        match ip_address {
            IpAddrRef::V4(ip_address, ip_address_octets) => IpAddr::V4(
                String::from_utf8(ip_address.to_vec()).expect(VALID_IP_BY_CONSTRUCTION),
                ip_address_octets,
            ),
            IpAddrRef::V6(ip_address, ip_address_octets) => IpAddr::V6(
                String::from_utf8(ip_address.to_vec()).expect(VALID_IP_BY_CONSTRUCTION),
                ip_address_octets,
            ),
        }
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a> From<&'a IpAddr> for IpAddrRef<'a> {
    fn from(ip_address: &'a IpAddr) -> IpAddrRef<'a> {
        match ip_address {
            IpAddr::V4(ip_address, ip_address_octets) => {
                IpAddrRef::V4(ip_address.as_bytes(), *ip_address_octets)
            }
            IpAddr::V6(ip_address, ip_address_octets) => {
                IpAddrRef::V6(ip_address.as_bytes(), *ip_address_octets)
            }
        }
    }
}

/// An error indicating that an `IpAddrRef` could not built because
/// the input could not be parsed as an IP address.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AddrParseError;

impl core::fmt::Display for AddrParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl ::std::error::Error for AddrParseError {}

impl<'a> IpAddrRef<'a> {
    /// Constructs an `IpAddrRef` from the given input if the input is
    /// a valid IPv4 or IPv6 address.
    pub fn try_from_ascii(ip_address: &'a [u8]) -> Result<Self, AddrParseError> {
        if let Ok(ip_address) = parse_ipv4_address(ip_address) {
            Ok(ip_address)
        } else if let Ok(ip_address) = parse_ipv6_address(ip_address) {
            Ok(ip_address)
        } else {
            Err(AddrParseError)
        }
    }

    /// Constructs an `IpAddrRef` from the given input if the input is a
    /// valid IP address.
    pub fn try_from_ascii_str(ip_address: &'a str) -> Result<Self, AddrParseError> {
        Self::try_from_ascii(ip_address.as_bytes())
    }

    /// Constructs an `IpAddr` from this `IpAddrRef`
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub fn to_owned(&self) -> IpAddr {
        match self {
            IpAddrRef::V4(ip_address, ip_address_octets) => IpAddr::V4(
                String::from_utf8(ip_address.to_vec()).expect(VALID_IP_BY_CONSTRUCTION),
                *ip_address_octets,
            ),
            IpAddrRef::V6(ip_address, ip_address_octets) => IpAddr::V6(
                String::from_utf8(ip_address.to_vec()).expect(VALID_IP_BY_CONSTRUCTION),
                *ip_address_octets,
            ),
        }
    }
}

#[cfg(feature = "std")]
fn ipv6_to_uncompressed_string(octets: [u8; 16]) -> String {
    let mut result = String::with_capacity(39);
    for i in 0..7 {
        result
            .write_fmt(format_args!(
                "{:02x?}{:02x?}:",
                octets[i * 2],
                octets[(i * 2) + 1]
            ))
            .expect("unexpected error while formatting IPv6 address");
    }
    result
        .write_fmt(format_args!("{:02x?}{:02x?}", octets[14], octets[15]))
        .expect("unexpected error while formatting IPv6 address");

    result
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl From<std::net::IpAddr> for IpAddr {
    fn from(ip_address: std::net::IpAddr) -> IpAddr {
        match ip_address {
            std::net::IpAddr::V4(ip_address) => {
                IpAddr::V4(ip_address.to_string(), ip_address.octets())
            }
            std::net::IpAddr::V6(ip_address) => IpAddr::V6(
                // We cannot rely on the Display implementation of
                // std::net::Ipv6Addr given that it might return
                // compressed IPv6 addresses if the address can be
                // expressed in such form. However, given we don't
                // support the IPv6 compressed form, we should not
                // generate such format either when converting from a
                // type that supports it.
                ipv6_to_uncompressed_string(ip_address.octets()),
                ip_address.octets(),
            ),
        }
    }
}

impl<'a> From<IpAddrRef<'a>> for &'a str {
    fn from(ip_address: IpAddrRef<'a>) -> &'a str {
        match ip_address {
            IpAddrRef::V4(ip_address, _) | IpAddrRef::V6(ip_address, _) => {
                core::str::from_utf8(ip_address).expect(VALID_IP_BY_CONSTRUCTION)
            }
        }
    }
}

impl<'a> From<IpAddrRef<'a>> for &'a [u8] {
    fn from(ip_address: IpAddrRef<'a>) -> &'a [u8] {
        match ip_address {
            IpAddrRef::V4(ip_address, _) | IpAddrRef::V6(ip_address, _) => ip_address,
        }
    }
}

// https://tools.ietf.org/html/rfc5280#section-4.2.1.6 says:
//   When the subjectAltName extension contains an iPAddress, the address
//   MUST be stored in the octet string in "network byte order", as
//   specified in [RFC791].  The least significant bit (LSB) of each octet
//   is the LSB of the corresponding byte in the network address.  For IP
//   version 4, as specified in [RFC791], the octet string MUST contain
//   exactly four octets.  For IP version 6, as specified in
//   [RFC2460], the octet string MUST contain exactly sixteen octets.
pub(super) fn presented_id_matches_reference_id(
    presented_id: untrusted::Input,
    reference_id: untrusted::Input,
) -> bool {
    match (presented_id.len(), reference_id.len()) {
        (4, 4) => (),
        (16, 16) => (),
        _ => {
            return false;
        }
    };

    let mut presented_ip_address = untrusted::Reader::new(presented_id);
    let mut reference_ip_address = untrusted::Reader::new(reference_id);
    while !presented_ip_address.at_end() {
        let presented_ip_address_byte = presented_ip_address.read_byte().unwrap();
        let reference_ip_address_byte = reference_ip_address.read_byte().unwrap();
        if presented_ip_address_byte != reference_ip_address_byte {
            return false;
        }
    }

    true
}

// https://tools.ietf.org/html/rfc5280#section-4.2.1.10 says:
//
//     For IPv4 addresses, the iPAddress field of GeneralName MUST contain
//     eight (8) octets, encoded in the style of RFC 4632 (CIDR) to represent
//     an address range [RFC4632].  For IPv6 addresses, the iPAddress field
//     MUST contain 32 octets similarly encoded.  For example, a name
//     constraint for "class C" subnet 192.0.2.0 is represented as the
//     octets C0 00 02 00 FF FF FF 00, representing the CIDR notation
//     192.0.2.0/24 (mask 255.255.255.0).
pub(super) fn presented_id_matches_constraint(
    name: untrusted::Input,
    constraint: untrusted::Input,
) -> Result<bool, Error> {
    match (name.len(), constraint.len()) {
        (4, 8) => (),
        (16, 32) => (),

        // an IPv4 address never matches an IPv6 constraint, and vice versa.
        (4, 32) | (16, 8) => {
            return Ok(false);
        }

        // invalid constraint length
        (4, _) | (16, _) => {
            return Err(Error::InvalidNetworkMaskConstraint);
        }

        // invalid name length, or anything else
        _ => {
            return Err(Error::BadDer);
        }
    };

    let (constraint_address, constraint_mask) = constraint.read_all(Error::BadDer, |value| {
        let address = value.read_bytes(constraint.len() / 2).unwrap();
        let mask = value.read_bytes(constraint.len() / 2).unwrap();
        Ok((address, mask))
    })?;

    let mut name = untrusted::Reader::new(name);
    let mut constraint_address = untrusted::Reader::new(constraint_address);
    let mut constraint_mask = untrusted::Reader::new(constraint_mask);
    let mut seen_zero_bit = false;

    loop {
        // Iterate through the name, constraint address, and constraint mask
        // a byte at a time.
        let name_byte = name.read_byte().unwrap();
        let constraint_address_byte = constraint_address.read_byte().unwrap();
        let constraint_mask_byte = constraint_mask.read_byte().unwrap();

        // A valid mask consists of a sequence of 1 bits, followed by a
        // sequence of 0 bits.  Either sequence could be empty.

        let leading = constraint_mask_byte.leading_ones();
        let trailing = constraint_mask_byte.trailing_zeros();

        // At the resolution of a single octet, a valid mask is one where
        // leading_ones() and trailing_zeros() sums to 8.
        // This includes all-ones and all-zeroes.
        if leading + trailing != 8 {
            return Err(Error::InvalidNetworkMaskConstraint);
        }

        // There should be no bits set after the first octet with a zero bit is seen.
        if seen_zero_bit && constraint_mask_byte != 0x00 {
            return Err(Error::InvalidNetworkMaskConstraint);
        }

        // Note when a zero bit is seen for later octets.
        if constraint_mask_byte != 0xff {
            seen_zero_bit = true;
        }

        if ((name_byte ^ constraint_address_byte) & constraint_mask_byte) != 0 {
            return Ok(false);
        }
        if name.at_end() {
            break;
        }
    }

    Ok(true)
}

pub(crate) fn parse_ipv4_address(ip_address_: &[u8]) -> Result<IpAddrRef, AddrParseError> {
    let mut ip_address = untrusted::Reader::new(untrusted::Input::from(ip_address_));
    let mut is_first_byte = true;
    let mut current_octet: [u8; 3] = [0, 0, 0];
    let mut current_size = 0;
    let mut dot_count = 0;

    let mut octet = 0;
    let mut octets: [u8; 4] = [0, 0, 0, 0];

    // Returns a u32 so it's possible to identify (and error) when
    // provided textual octets > 255, not representable by u8.
    fn radix10_to_octet(textual_octets: &[u8]) -> u32 {
        let mut result: u32 = 0;
        for digit in textual_octets.iter() {
            result *= 10;
            result += u32::from(*digit);
        }
        result
    }

    loop {
        match ip_address.read_byte() {
            Ok(b'.') => {
                if is_first_byte {
                    // IPv4 address cannot start with a dot.
                    return Err(AddrParseError);
                }
                if ip_address.at_end() {
                    // IPv4 address cannot end with a dot.
                    return Err(AddrParseError);
                }
                if dot_count == 3 {
                    // IPv4 address cannot have more than three dots.
                    return Err(AddrParseError);
                }
                dot_count += 1;
                if current_size == 0 {
                    // IPv4 address cannot contain two dots in a row.
                    return Err(AddrParseError);
                }
                let current_raw_octet = radix10_to_octet(&current_octet[..current_size]);
                if current_raw_octet > 255 {
                    // No octet can be greater than 255.
                    return Err(AddrParseError);
                }
                octets[octet] =
                    TryInto::<u8>::try_into(current_raw_octet).expect("invalid character");
                octet += 1;
                // We move on to the next textual octet.
                current_octet = [0, 0, 0];
                current_size = 0;
            }
            Ok(number @ b'0'..=b'9') => {
                if number == b'0'
                    && current_size == 0
                    && !ip_address.peek(b'.')
                    && !ip_address.at_end()
                {
                    // No octet can start with 0 if a dot does not follow and if we are not at the end.
                    return Err(AddrParseError);
                }
                if current_size >= current_octet.len() {
                    // More than 3 octets in a triple
                    return Err(AddrParseError);
                }
                current_octet[current_size] = number - b'0';
                current_size += 1;
            }
            _ => {
                return Err(AddrParseError);
            }
        }
        is_first_byte = false;

        if ip_address.at_end() {
            let last_octet = radix10_to_octet(&current_octet[..current_size]);
            if current_size > 0 && last_octet > 255 {
                // No octet can be greater than 255.
                return Err(AddrParseError);
            }
            octets[octet] = TryInto::<u8>::try_into(last_octet).expect("invalid character");
            break;
        }
    }
    if dot_count != 3 {
        return Err(AddrParseError);
    }
    Ok(IpAddrRef::V4(ip_address_, octets))
}

pub(crate) fn parse_ipv6_address(ip_address_: &[u8]) -> Result<IpAddrRef, AddrParseError> {
    // Compressed addresses are not supported. Also, IPv4-mapped IPv6
    // addresses are not supported. This makes 8 groups of 4
    // hexadecimal characters + 7 colons.
    if ip_address_.len() != 39 {
        return Err(AddrParseError);
    }

    let mut ip_address = untrusted::Reader::new(untrusted::Input::from(ip_address_));
    let mut is_first_byte = true;
    let mut current_textual_block_size = 0;
    let mut colon_count = 0;

    let mut octet = 0;
    let mut previous_character = None;
    let mut octets: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    loop {
        match ip_address.read_byte() {
            Ok(b':') => {
                if is_first_byte {
                    // Uncompressed IPv6 address cannot start with a colon.
                    return Err(AddrParseError);
                }
                if ip_address.at_end() {
                    // Uncompressed IPv6 address cannot end with a colon.
                    return Err(AddrParseError);
                }
                if colon_count == 7 {
                    // IPv6 address cannot have more than seven colons.
                    return Err(AddrParseError);
                }
                colon_count += 1;
                if current_textual_block_size == 0 {
                    // Uncompressed IPv6 address cannot contain two colons in a row.
                    return Err(AddrParseError);
                }
                if current_textual_block_size != 4 {
                    // Compressed IPv6 addresses are not supported.
                    return Err(AddrParseError);
                }
                // We move on to the next textual block.
                current_textual_block_size = 0;
                previous_character = None;
            }
            Ok(character @ b'0'..=b'9')
            | Ok(character @ b'a'..=b'f')
            | Ok(character @ b'A'..=b'F') => {
                if current_textual_block_size == 4 {
                    // Blocks cannot contain more than 4 hexadecimal characters.
                    return Err(AddrParseError);
                }
                if let Some(previous_character_) = previous_character {
                    octets[octet] = (TryInto::<u8>::try_into(
                        TryInto::<u8>::try_into(
                            (TryInto::<char>::try_into(previous_character_)
                                .expect("invalid character"))
                            .to_digit(16)
                            // Safe to unwrap because we know character is within hexadecimal bounds ([0-9a-f])
                            .unwrap(),
                        )
                        .expect("invalid character"),
                    )
                    .expect("invalid character")
                        << 4)
                        | (TryInto::<u8>::try_into(
                            TryInto::<char>::try_into(character)
                                .expect("invalid character")
                                .to_digit(16)
                                // Safe to unwrap because we know character is within hexadecimal bounds ([0-9a-f])
                                .unwrap(),
                        )
                        .expect("invalid character"));
                    previous_character = None;
                    octet += 1;
                } else {
                    previous_character = Some(character);
                }
                current_textual_block_size += 1;
            }
            _ => {
                return Err(AddrParseError);
            }
        }
        is_first_byte = false;

        if ip_address.at_end() {
            break;
        }
    }
    if colon_count != 7 {
        return Err(AddrParseError);
    }
    Ok(IpAddrRef::V6(ip_address_, octets))
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn ipv4_address(
        ip_address: &[u8],
        octets: [u8; 4],
    ) -> (&[u8], Result<IpAddrRef, AddrParseError>) {
        (ip_address, Ok(IpAddrRef::V4(ip_address, octets)))
    }

    const IPV4_ADDRESSES: &[(&[u8], Result<IpAddrRef, AddrParseError>)] = &[
        // Valid IPv4 addresses
        ipv4_address(b"0.0.0.0", [0, 0, 0, 0]),
        ipv4_address(b"1.1.1.1", [1, 1, 1, 1]),
        ipv4_address(b"205.0.0.0", [205, 0, 0, 0]),
        ipv4_address(b"0.205.0.0", [0, 205, 0, 0]),
        ipv4_address(b"0.0.205.0", [0, 0, 205, 0]),
        ipv4_address(b"0.0.0.205", [0, 0, 0, 205]),
        ipv4_address(b"0.0.0.20", [0, 0, 0, 20]),
        // Invalid IPv4 addresses
        (b"", Err(AddrParseError)),
        (b"...", Err(AddrParseError)),
        (b".0.0.0.0", Err(AddrParseError)),
        (b"0.0.0.0.", Err(AddrParseError)),
        (b"0.0.0", Err(AddrParseError)),
        (b"0.0.0.", Err(AddrParseError)),
        (b"256.0.0.0", Err(AddrParseError)),
        (b"0.256.0.0", Err(AddrParseError)),
        (b"0.0.256.0", Err(AddrParseError)),
        (b"0.0.0.256", Err(AddrParseError)),
        (b"1..1.1.1", Err(AddrParseError)),
        (b"1.1..1.1", Err(AddrParseError)),
        (b"1.1.1..1", Err(AddrParseError)),
        (b"025.0.0.0", Err(AddrParseError)),
        (b"0.025.0.0", Err(AddrParseError)),
        (b"0.0.025.0", Err(AddrParseError)),
        (b"0.0.0.025", Err(AddrParseError)),
        (b"1234.0.0.0", Err(AddrParseError)),
        (b"0.1234.0.0", Err(AddrParseError)),
        (b"0.0.1234.0", Err(AddrParseError)),
        (b"0.0.0.1234", Err(AddrParseError)),
    ];

    #[test]
    fn parse_ipv4_address_test() {
        for &(ip_address, expected_result) in IPV4_ADDRESSES {
            assert_eq!(parse_ipv4_address(ip_address), expected_result,);
        }
    }

    const fn ipv6_address(
        ip_address: &[u8],
        octets: [u8; 16],
    ) -> (&[u8], Result<IpAddrRef, AddrParseError>) {
        (ip_address, Ok(IpAddrRef::V6(ip_address, octets)))
    }

    const IPV6_ADDRESSES: &[(&[u8], Result<IpAddrRef, AddrParseError>)] = &[
        // Valid IPv6 addresses
        ipv6_address(
            b"2a05:d018:076c:b685:e8ab:afd3:af51:3aed",
            [
                0x2a, 0x05, 0xd0, 0x18, 0x07, 0x6c, 0xb6, 0x85, 0xe8, 0xab, 0xaf, 0xd3, 0xaf, 0x51,
                0x3a, 0xed,
            ],
        ),
        ipv6_address(
            b"2A05:D018:076C:B685:E8AB:AFD3:AF51:3AED",
            [
                0x2a, 0x05, 0xd0, 0x18, 0x07, 0x6c, 0xb6, 0x85, 0xe8, 0xab, 0xaf, 0xd3, 0xaf, 0x51,
                0x3a, 0xed,
            ],
        ),
        ipv6_address(
            b"ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
            [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff,
            ],
        ),
        ipv6_address(
            b"FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF:FFFF",
            [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff,
            ],
        ),
        ipv6_address(
            b"FFFF:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
            [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff,
            ],
        ),
        // Invalid IPv6 addresses
        // Missing octets on uncompressed addresses. The unmatching letter has the violation
        (
            b"aaa:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:aaa:ffff:ffff:ffff:ffff:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:aaa:ffff:ffff:ffff:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:ffff:aaa:ffff:ffff:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:ffff:ffff:aaa:ffff:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:ffff:ffff:ffff:aaa:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:ffff:ffff:ffff:ffff:aaa:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:ffff:ffff:ffff:ffff:ffff:aaa",
            Err(AddrParseError),
        ),
        // Wrong hexadecimal characters on different positions
        (
            b"ffgf:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:gfff:ffff:ffff:ffff:ffff:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:fffg:ffff:ffff:ffff:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:ffff:ffgf:ffff:ffff:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:ffff:ffff:gfff:ffff:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:ffff:ffff:ffff:fgff:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:ffff:ffff:ffff:ffff:ffgf:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:ffff:ffff:ffff:ffff:ffgf:fffg",
            Err(AddrParseError),
        ),
        // Wrong colons on uncompressed addresses
        (
            b":ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff::ffff:ffff:ffff:ffff:ffff:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff::ffff:ffff:ffff:ffff:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:ffff::ffff:ffff:ffff:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:ffff:ffff::ffff:ffff:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:ffff:ffff:ffff::ffff:ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:ffff:ffff:ffff:ffff::ffff:ffff",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:ffff:ffff:ffff:ffff:ffff::ffff",
            Err(AddrParseError),
        ),
        // More colons than allowed
        (
            b"ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff:",
            Err(AddrParseError),
        ),
        (
            b"ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
            Err(AddrParseError),
        ),
        // v Invalid UTF-8 encoding
        (
            b"\xc3\x28a05:d018:076c:b685:e8ab:afd3:af51:3aed",
            Err(AddrParseError),
        ),
        // v Invalid hexadecimal
        (
            b"ga05:d018:076c:b685:e8ab:afd3:af51:3aed",
            Err(AddrParseError),
        ),
        // Cannot start with colon
        (
            b":a05:d018:076c:b685:e8ab:afd3:af51:3aed",
            Err(AddrParseError),
        ),
        // Cannot end with colon
        (
            b"2a05:d018:076c:b685:e8ab:afd3:af51:3ae:",
            Err(AddrParseError),
        ),
        // Cannot have more than seven colons
        (
            b"2a05:d018:076c:b685:e8ab:afd3:af51:3a::",
            Err(AddrParseError),
        ),
        // Cannot contain two colons in a row
        (
            b"2a05::018:076c:b685:e8ab:afd3:af51:3aed",
            Err(AddrParseError),
        ),
        // v Textual block size is longer
        (
            b"2a056:d018:076c:b685:e8ab:afd3:af51:3ae",
            Err(AddrParseError),
        ),
        // v Textual block size is shorter
        (
            b"2a0:d018:076c:b685:e8ab:afd3:af51:3aed ",
            Err(AddrParseError),
        ),
        // Shorter IPv6 address
        (b"d018:076c:b685:e8ab:afd3:af51:3aed", Err(AddrParseError)),
        // Longer IPv6 address
        (
            b"2a05:d018:076c:b685:e8ab:afd3:af51:3aed3aed",
            Err(AddrParseError),
        ),
        // These are valid IPv6 addresses, but we don't support compressed addresses
        (b"0:0:0:0:0:0:0:1", Err(AddrParseError)),
        (
            b"2a05:d018:76c:b685:e8ab:afd3:af51:3aed",
            Err(AddrParseError),
        ),
    ];

    #[test]
    fn parse_ipv6_address_test() {
        for &(ip_address, expected_result) in IPV6_ADDRESSES {
            assert_eq!(parse_ipv6_address(ip_address), expected_result,);
        }
    }

    #[test]
    fn try_from_ascii_ip_address_test() {
        const IP_ADDRESSES: &[(&[u8], Result<IpAddrRef, AddrParseError>)] = &[
            // Valid IPv4 addresses
            (
                b"127.0.0.1",
                Ok(IpAddrRef::V4(b"127.0.0.1", [127, 0, 0, 1])),
            ),
            // Invalid IPv4 addresses
            (
                // Ends with a dot; misses one octet
                b"127.0.0.",
                Err(AddrParseError),
            ),
            // Valid IPv6 addresses
            (
                b"0000:0000:0000:0000:0000:0000:0000:0001",
                Ok(IpAddrRef::V6(
                    b"0000:0000:0000:0000:0000:0000:0000:0001",
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                )),
            ),
            // Invalid IPv6 addresses
            (
                // IPv6 addresses in compressed form are not supported
                b"0:0:0:0:0:0:0:1",
                Err(AddrParseError),
            ),
            // Something else
            (
                // A hostname
                b"example.com",
                Err(AddrParseError),
            ),
        ];
        for &(ip_address, expected_result) in IP_ADDRESSES {
            assert_eq!(IpAddrRef::try_from_ascii(ip_address), expected_result)
        }
    }

    #[test]
    fn try_from_ascii_str_ip_address_test() {
        const IP_ADDRESSES: &[(&str, Result<IpAddrRef, AddrParseError>)] = &[
            // Valid IPv4 addresses
            ("127.0.0.1", Ok(IpAddrRef::V4(b"127.0.0.1", [127, 0, 0, 1]))),
            // Invalid IPv4 addresses
            (
                // Ends with a dot; misses one octet
                "127.0.0.",
                Err(AddrParseError),
            ),
            // Valid IPv6 addresses
            (
                "0000:0000:0000:0000:0000:0000:0000:0001",
                Ok(IpAddrRef::V6(
                    b"0000:0000:0000:0000:0000:0000:0000:0001",
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                )),
            ),
            // Invalid IPv6 addresses
            (
                // IPv6 addresses in compressed form are not supported
                "0:0:0:0:0:0:0:1",
                Err(AddrParseError),
            ),
            // Something else
            (
                // A hostname
                "example.com",
                Err(AddrParseError),
            ),
        ];
        for &(ip_address, expected_result) in IP_ADDRESSES {
            assert_eq!(IpAddrRef::try_from_ascii_str(ip_address), expected_result)
        }
    }

    #[test]
    fn str_from_ip_address_ref_test() {
        let ip_addresses = vec![
            // IPv4 addresses
            (IpAddrRef::V4(b"127.0.0.1", [127, 0, 0, 1]), "127.0.0.1"),
            // IPv6 addresses
            (
                IpAddrRef::V6(
                    b"0000:0000:0000:0000:0000:0000:0000:0001",
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                ),
                "0000:0000:0000:0000:0000:0000:0000:0001",
            ),
        ];
        for (ip_address, expected_ip_address) in ip_addresses {
            assert_eq!(Into::<&str>::into(ip_address), expected_ip_address,)
        }
    }

    #[test]
    fn u8_array_from_ip_address_ref_test() {
        let ip_addresses = vec![
            // IPv4 addresses
            (IpAddrRef::V4(b"127.0.0.1", [127, 0, 0, 1]), "127.0.0.1"),
            // IPv6 addresses
            (
                IpAddrRef::V6(
                    b"0000:0000:0000:0000:0000:0000:0000:0001",
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                ),
                "0000:0000:0000:0000:0000:0000:0000:0001",
            ),
        ];
        for (ip_address, expected_ip_address) in ip_addresses {
            assert_eq!(
                Into::<&[u8]>::into(ip_address),
                expected_ip_address.as_bytes()
            )
        }
    }

    #[test]
    fn presented_id_matches_constraint_ipv4_test() {
        let names_and_constraints = vec![
            (
                // 192.0.2.0 matches constraint 192.0.2.0/24
                [0xC0, 0x00, 0x02, 0x00],
                [0xC0, 0x00, 0x02, 0x00, 0xFF, 0xFF, 0xFF, 0x00],
                Ok(true),
            ),
            (
                // 192.0.2.1 matches constraint 192.0.2.0/24
                [0xC0, 0x00, 0x02, 0x01],
                [0xC0, 0x00, 0x02, 0x00, 0xFF, 0xFF, 0xFF, 0x00],
                Ok(true),
            ),
            (
                // 192.0.2.255 matches constraint 192.0.2.0/24
                [0xC0, 0x00, 0x02, 0xFF],
                [0xC0, 0x00, 0x02, 0x00, 0xFF, 0xFF, 0xFF, 0x00],
                Ok(true),
            ),
            (
                // 192.0.1.255 does not match constraint 192.0.2.0/24
                [0xC0, 0x00, 0x01, 0xFF],
                [0xC0, 0x00, 0x02, 0x00, 0xFF, 0xFF, 0xFF, 0x00],
                Ok(false),
            ),
            (
                // 192.0.3.0 does not match constraint 192.0.2.0/24
                [0xC0, 0x00, 0x03, 0x00],
                [0xC0, 0x00, 0x02, 0x00, 0xFF, 0xFF, 0xFF, 0x00],
                Ok(false),
            ),
        ];
        for (name, constraint, match_result) in names_and_constraints {
            assert_eq!(
                presented_id_matches_constraint(
                    untrusted::Input::from(&name),
                    untrusted::Input::from(&constraint),
                ),
                match_result
            )
        }

        // Invalid name length (shorter)
        assert_eq!(
            presented_id_matches_constraint(
                untrusted::Input::from(&[0xC0, 0x00, 0x02]),
                untrusted::Input::from(&[0xC0, 0x00, 0x02, 0x00, 0xFF, 0xFF, 0xFF, 0x00]),
            ),
            Err(Error::BadDer),
        );

        // Invalid name length (longer)
        assert_eq!(
            presented_id_matches_constraint(
                untrusted::Input::from(&[0xC0, 0x00, 0x02, 0x00, 0x00]),
                untrusted::Input::from(&[0xC0, 0x00, 0x02, 0x00, 0xFF, 0xFF, 0xFF, 0x00]),
            ),
            Err(Error::BadDer),
        );

        // Unmatching constraint size (shorter)
        assert_eq!(
            presented_id_matches_constraint(
                untrusted::Input::from(&[0xC0, 0x00, 0x02, 0x00]),
                untrusted::Input::from(&[0xC0, 0x00, 0x02, 0x00, 0xFF, 0xFF, 0xFF]),
            ),
            Err(Error::InvalidNetworkMaskConstraint),
        );

        // Unmatching constraint size (longer)
        assert_eq!(
            presented_id_matches_constraint(
                untrusted::Input::from(&[0xC0, 0x00, 0x02, 0x00]),
                untrusted::Input::from(&[0xC0, 0x00, 0x02, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00]),
            ),
            Err(Error::InvalidNetworkMaskConstraint),
        );

        // Unmatching constraint size (IPv6 constraint for IPv4 address)
        assert_eq!(
            presented_id_matches_constraint(
                untrusted::Input::from(&[0xC0, 0x00, 0x02, 0x00]),
                untrusted::Input::from(&[
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00
                ]),
            ),
            Ok(false),
        );
    }

    #[test]
    fn presented_id_matches_constraint_ipv6_test() {
        let names_and_constraints = vec![
            (
                // 2001:0DB8:ABCD:0012:0000:0000:0000:0000 matches constraint
                //   2001:0DB8:ABCD:0012:0000:0000:0000:0000/64
                [
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00,
                ],
                [
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
                Ok(true),
            ),
            (
                // 2001:0DB8:ABCD:0012:0000:0000:0000:0001 matches constraint
                //   2001:0DB8:ABCD:0012:0000:0000:0000:0000/64
                [
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x01,
                ],
                [
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
                Ok(true),
            ),
            (
                // 2001:0DB8:ABCD:0012:FFFF:FFFF:FFFF:FFFF matches constraint
                //   2001:0DB8:ABCD:0012:0000:0000:0000:0000/64
                [
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                    0xFF, 0xFF, 0xFF,
                ],
                [
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
                Ok(true),
            ),
            (
                // 2001:0DB8:ABCD:0011:0000:0000:0000:0000 does not match constraint
                //   2001:0DB8:ABCD:0012:0000:0000:0000:0000/64
                [
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00,
                ],
                [
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
                Ok(false),
            ),
            (
                // 2001:0DB8:ABCD:0013:0000:0000:0000:0000 does not match constraint
                //   2001:0DB8:ABCD:0012:0000:0000:0000:0000/64
                [
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00,
                ],
                [
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
                Ok(false),
            ),
        ];
        for (name, constraint, match_result) in names_and_constraints {
            assert_eq!(
                presented_id_matches_constraint(
                    untrusted::Input::from(&name),
                    untrusted::Input::from(&constraint),
                ),
                match_result
            )
        }

        // Invalid name length (shorter)
        assert_eq!(
            presented_id_matches_constraint(
                untrusted::Input::from(&[
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00
                ]),
                untrusted::Input::from(&[
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00
                ]),
            ),
            Err(Error::BadDer),
        );

        // Invalid name length (longer)
        assert_eq!(
            presented_id_matches_constraint(
                untrusted::Input::from(&[
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00
                ]),
                untrusted::Input::from(&[
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00
                ]),
            ),
            Err(Error::BadDer),
        );

        // Unmatching constraint size (shorter)
        assert_eq!(
            presented_id_matches_constraint(
                untrusted::Input::from(&[
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00,
                ]),
                untrusted::Input::from(&[
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00
                ]),
            ),
            Err(Error::InvalidNetworkMaskConstraint),
        );

        // Unmatching constraint size (longer)
        assert_eq!(
            presented_id_matches_constraint(
                untrusted::Input::from(&[
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00,
                ]),
                untrusted::Input::from(&[
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
                ]),
            ),
            Err(Error::InvalidNetworkMaskConstraint),
        );

        // Unmatching constraint size (IPv4 constraint for IPv6 address)
        assert_eq!(
            presented_id_matches_constraint(
                untrusted::Input::from(&[
                    0x20, 0x01, 0x0D, 0xB8, 0xAB, 0xCD, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00,
                ]),
                untrusted::Input::from(&[0xC0, 0x00, 0x02, 0x00, 0xFF, 0xFF, 0xFF, 0x00]),
            ),
            Ok(false),
        );
    }

    #[test]
    fn test_presented_id_matches_reference_id() {
        assert!(!presented_id_matches_reference_id(
            untrusted::Input::from(&[]),
            untrusted::Input::from(&[]),
        ));

        assert!(!presented_id_matches_reference_id(
            untrusted::Input::from(&[0x01]),
            untrusted::Input::from(&[])
        ));

        assert!(!presented_id_matches_reference_id(
            untrusted::Input::from(&[]),
            untrusted::Input::from(&[0x01])
        ));

        assert!(presented_id_matches_reference_id(
            untrusted::Input::from(&[1, 2, 3, 4]),
            untrusted::Input::from(&[1, 2, 3, 4])
        ));

        assert!(!presented_id_matches_reference_id(
            untrusted::Input::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            untrusted::Input::from(&[1, 2, 3, 4])
        ));

        assert!(!presented_id_matches_reference_id(
            untrusted::Input::from(&[1, 2, 3, 4]),
            untrusted::Input::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])
        ));

        assert!(presented_id_matches_reference_id(
            untrusted::Input::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            untrusted::Input::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])
        ));
    }

    #[test]
    fn presented_id_matches_constraint_rejects_incorrect_length_arguments() {
        // wrong length names
        assert_eq!(
            presented_id_matches_constraint(
                untrusted::Input::from(b"\x00\x00\x00"),
                untrusted::Input::from(b"")
            ),
            Err(Error::BadDer)
        );
        assert_eq!(
            presented_id_matches_constraint(
                untrusted::Input::from(b"\x00\x00\x00\x00\x00"),
                untrusted::Input::from(b"")
            ),
            Err(Error::BadDer)
        );

        assert_eq!(
            presented_id_matches_constraint(
                untrusted::Input::from(
                    b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"
                ),
                untrusted::Input::from(b"")
            ),
            Err(Error::BadDer)
        );
        assert_eq!(
            presented_id_matches_constraint(
                untrusted::Input::from(
                    b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"
                ),
                untrusted::Input::from(b"")
            ),
            Err(Error::BadDer)
        );

        // wrong length constraints
        assert_eq!(
            presented_id_matches_constraint(
                untrusted::Input::from(b"\x00\x00\x00\x00"),
                untrusted::Input::from(b"\x00\x00\x00\x00\xff\xff\xff")
            ),
            Err(Error::InvalidNetworkMaskConstraint)
        );
        assert_eq!(
            presented_id_matches_constraint(
                untrusted::Input::from(b"\x00\x00\x00\x00"),
                untrusted::Input::from(b"\x00\x00\x00\x00\xff\xff\xff\xff\x00")
            ),
            Err(Error::InvalidNetworkMaskConstraint)
        );
        assert_eq!(
            presented_id_matches_constraint(untrusted::Input::from(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                            untrusted::Input::from(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
                                                                     \xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff")),
            Err(Error::InvalidNetworkMaskConstraint)
        );
        assert_eq!(
            presented_id_matches_constraint(untrusted::Input::from(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                                            untrusted::Input::from(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
                                                                     \xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff")),
            Err(Error::InvalidNetworkMaskConstraint)
        );

        // ipv4-length not considered for ipv6-length name, and vv
        assert_eq!(
            presented_id_matches_constraint(untrusted::Input::from(b"\x00\x00\x00\x00"),
                                            untrusted::Input::from(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
                                                                     \xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff")),
            Ok(false)
        );
        assert_eq!(
            presented_id_matches_constraint(
                untrusted::Input::from(
                    b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"
                ),
                untrusted::Input::from(b"\x00\x00\x00\x00\xff\xff\xff\xff")
            ),
            Ok(false)
        );
    }
}

#[cfg(all(test, feature = "alloc"))]
mod alloc_tests {
    use super::*;

    #[test]
    fn as_ref_ip_address_test() {
        assert_eq!(
            IpAddr::V4(String::from("127.0.0.1"), [127, 0, 0, 1]).as_ref(),
            "127.0.0.1",
        );
        assert_eq!(
            IpAddr::V6(
                String::from("0000:0000:0000:0000:0000:0000:0000:0001"),
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
            )
            .as_ref(),
            "0000:0000:0000:0000:0000:0000:0000:0001",
        );
    }

    #[test]
    fn from_ip_address_ref_for_ip_address_test() {
        {
            let (ip_address, ip_address_octets) = ("127.0.0.1", [127, 0, 0, 1]);
            assert_eq!(
                IpAddr::from(IpAddrRef::V4(ip_address.as_bytes(), ip_address_octets)),
                IpAddr::V4(String::from(ip_address), ip_address_octets),
            )
        }
        {
            let (ip_address, ip_address_octets) = (
                "0000:0000:0000:0000:0000:0000:0000:0001",
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            );
            assert_eq!(
                IpAddr::from(IpAddrRef::V6(ip_address.as_bytes(), ip_address_octets)),
                IpAddr::V6(String::from(ip_address), ip_address_octets),
            )
        }
    }

    #[test]
    fn from_ip_address_for_ip_address_ref_test() {
        {
            let ip_address = IpAddr::V4(String::from("127.0.0.1"), [127, 0, 0, 1]);
            assert_eq!(
                IpAddrRef::from(&ip_address),
                IpAddrRef::V4(b"127.0.0.1", [127, 0, 0, 1]),
            )
        }
        {
            let ip_address = IpAddr::V6(
                String::from("0000:0000:0000:0000:0000:0000:0000:0001"),
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            );
            assert_eq!(
                IpAddrRef::from(&ip_address),
                IpAddrRef::V6(
                    b"0000:0000:0000:0000:0000:0000:0000:0001",
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
                ),
            )
        }
    }

    #[test]
    fn display_invalid_ip_address_error_test() {
        assert_eq!(AddrParseError.to_string(), String::from("AddrParseError"),)
    }

    #[test]
    fn ip_address_ref_to_owned_test() {
        {
            assert_eq!(
                IpAddrRef::V4(b"127.0.0.1", [127, 0, 0, 1]).to_owned(),
                IpAddr::V4(String::from("127.0.0.1"), [127, 0, 0, 1]),
            )
        }
        {
            assert_eq!(
                IpAddrRef::V6(
                    b"0000:0000:0000:0000:0000:0000:0000:0001",
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                )
                .to_owned(),
                IpAddr::V6(
                    String::from("0000:0000:0000:0000:0000:0000:0000:0001"),
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                ),
            )
        }
    }

    #[test]
    fn ip_address_from_std_net_ipaddr_test() {
        let ip_addresses = vec![
            (
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
                IpAddr::V4(String::from("127.0.0.1"), [127, 0, 0, 1]),
            ),
            (
                std::net::IpAddr::V6(std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
                IpAddr::V6(
                    String::from("0000:0000:0000:0000:0000:0000:0000:0001"),
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                ),
            ),
        ];
        for (ip_address, expected_ip_address) in ip_addresses {
            assert_eq!(IpAddr::from(ip_address), expected_ip_address,)
        }
    }

    #[test]
    fn ipv6_to_uncompressed_string_test() {
        let ip_addresses = vec![
            (
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                String::from("0000:0000:0000:0000:0000:0000:0000:0001"),
            ),
            (
                [
                    0x2a, 0x05, 0xd0, 0x18, 0x07, 0x6c, 0xb6, 0x84, 0x8e, 0x48, 0x47, 0xc9, 0x84,
                    0xaa, 0xb3, 0x4d,
                ],
                String::from("2a05:d018:076c:b684:8e48:47c9:84aa:b34d"),
            ),
        ];
        for (ip_address_octets, expected_result) in ip_addresses {
            assert_eq!(
                ipv6_to_uncompressed_string(ip_address_octets),
                expected_result,
            )
        }
    }

    // (presented_address, constraint_address, constraint_mask, expected_result)
    const PRESENTED_MATCHES_CONSTRAINT: &[(&str, &str, &str, Result<bool, Error>)] = &[
        // Cannot mix IpV4 with IpV6 and viceversa
        ("2001:db8::", "8.8.8.8", "255.255.255.255", Ok(false)),
        ("8.8.8.8", "2001:db8::", "ffff::", Ok(false)),
        // IpV4 non-contiguous masks
        (
            "8.8.8.8",
            "8.8.8.8",
            "255.255.255.1",
            Err(Error::InvalidNetworkMaskConstraint),
        ),
        (
            "8.8.8.8",
            "8.8.8.8",
            "255.255.0.255",
            Err(Error::InvalidNetworkMaskConstraint),
        ),
        (
            "8.8.8.8",
            "8.8.8.8",
            "255.0.255.255",
            Err(Error::InvalidNetworkMaskConstraint),
        ),
        (
            "8.8.8.8",
            "8.8.8.8",
            "0.255.255.255",
            Err(Error::InvalidNetworkMaskConstraint),
        ),
        (
            "8.8.8.8",
            "8.8.8.8",
            "1.255.255.255",
            Err(Error::InvalidNetworkMaskConstraint),
        ),
        (
            "8.8.8.8",
            "8.8.8.8",
            "128.128.128.128",
            Err(Error::InvalidNetworkMaskConstraint),
        ),
        // IpV4
        ("8.8.8.8", "8.8.8.8", "255.255.255.255", Ok(true)),
        ("8.8.8.9", "8.8.8.8", "255.255.255.255", Ok(false)),
        ("8.8.8.9", "8.8.8.8", "255.255.255.254", Ok(true)),
        ("8.8.8.10", "8.8.8.8", "255.255.255.254", Ok(false)),
        ("8.8.8.10", "8.8.8.8", "255.255.255.0", Ok(true)),
        ("8.8.15.10", "8.8.8.8", "255.255.248.0", Ok(true)),
        ("8.8.16.10", "8.8.8.8", "255.255.248.0", Ok(false)),
        ("8.8.16.10", "8.8.8.8", "255.255.0.0", Ok(true)),
        ("8.31.16.10", "8.8.8.8", "255.224.0.0", Ok(true)),
        ("8.32.16.10", "8.8.8.8", "255.224.0.0", Ok(false)),
        ("8.32.16.10", "8.8.8.8", "255.0.0.0", Ok(true)),
        ("63.32.16.10", "8.8.8.8", "192.0.0.0", Ok(true)),
        ("64.32.16.10", "8.8.8.8", "192.0.0.0", Ok(false)),
        ("64.32.16.10", "8.8.8.8", "0.0.0.0", Ok(true)),
        // IpV6 non-contiguous masks
        (
            "2001:db8::",
            "2001:db8::",
            "fffe:ffff::",
            Err(Error::InvalidNetworkMaskConstraint),
        ),
        (
            "2001:db8::",
            "2001:db8::",
            "ffff:fdff::",
            Err(Error::InvalidNetworkMaskConstraint),
        ),
        (
            "2001:db8::",
            "2001:db8::",
            "ffff:feff::",
            Err(Error::InvalidNetworkMaskConstraint),
        ),
        (
            "2001:db8::",
            "2001:db8::",
            "ffff:fcff::",
            Err(Error::InvalidNetworkMaskConstraint),
        ),
        (
            "2001:db8::",
            "2001:db8::",
            "7fff:ffff::",
            Err(Error::InvalidNetworkMaskConstraint),
        ),
        // IpV6
        ("2001:db8::", "2001:db8::", "ffff:ffff::", Ok(true)),
        ("2001:db9::", "2001:db8::", "ffff:ffff::", Ok(false)),
        ("2001:db9::", "2001:db8::", "ffff:fffe::", Ok(true)),
        ("2001:dba::", "2001:db8::", "ffff:fffe::", Ok(false)),
        ("2001:dba::", "2001:db8::", "ffff:ff00::", Ok(true)),
        ("2001:dca::", "2001:db8::", "ffff:fe00::", Ok(true)),
        ("2001:fca::", "2001:db8::", "ffff:fe00::", Ok(false)),
        ("2001:fca::", "2001:db8::", "ffff:0000::", Ok(true)),
        ("2000:fca::", "2001:db8::", "fffe:0000::", Ok(true)),
        ("2003:fca::", "2001:db8::", "fffe:0000::", Ok(false)),
        ("2003:fca::", "2001:db8::", "ff00:0000::", Ok(true)),
        ("1003:fca::", "2001:db8::", "e000:0000::", Ok(false)),
        ("1003:fca::", "2001:db8::", "0000:0000::", Ok(true)),
    ];

    #[cfg(feature = "std")]
    #[test]
    fn presented_matches_constraint_test() {
        use std::boxed::Box;
        use std::net::IpAddr;

        for &(presented, constraint_address, constraint_mask, expected_result) in
            PRESENTED_MATCHES_CONSTRAINT
        {
            let presented_bytes: Box<[u8]> = match presented.parse::<IpAddr>().unwrap() {
                IpAddr::V4(p) => Box::new(p.octets()),
                IpAddr::V6(p) => Box::new(p.octets()),
            };
            let ca_bytes: Box<[u8]> = match constraint_address.parse::<IpAddr>().unwrap() {
                IpAddr::V4(ca) => Box::new(ca.octets()),
                IpAddr::V6(ca) => Box::new(ca.octets()),
            };
            let cm_bytes: Box<[u8]> = match constraint_mask.parse::<IpAddr>().unwrap() {
                IpAddr::V4(cm) => Box::new(cm.octets()),
                IpAddr::V6(cm) => Box::new(cm.octets()),
            };
            let constraint_bytes = [ca_bytes, cm_bytes].concat();
            let actual_result = presented_id_matches_constraint(
                untrusted::Input::from(&presented_bytes),
                untrusted::Input::from(&constraint_bytes),
            );
            assert_eq!(
                actual_result, expected_result,
                "presented_id_matches_constraint(\"{:?}\", \"{:?}\")",
                presented_bytes, constraint_bytes
            );
        }
    }
}
