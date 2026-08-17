use super::EndpointError;
use crate::ZmqError;

use std::convert::TryFrom;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

/// Represents a host address. Does not include the port, and may be either an
/// ip address or a domain name
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Host {
    /// An IPv4 address
    Ipv4(Ipv4Addr),
    /// An Ipv6 address
    Ipv6(Ipv6Addr),
    /// A domain name, such as `example.com` in `tcp://example.com:4567`.
    Domain(String),
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Host::Ipv4(addr) => write!(f, "{}", addr),
            Host::Ipv6(addr) => write!(f, "{}", addr),
            Host::Domain(name) => write!(f, "{}", name),
        }
    }
}

impl TryFrom<Host> for IpAddr {
    type Error = ZmqError;

    fn try_from(h: Host) -> Result<Self, Self::Error> {
        match h {
            Host::Ipv4(a) => Ok(IpAddr::V4(a)),
            Host::Ipv6(a) => Ok(IpAddr::V6(a)),
            Host::Domain(_) => Err(ZmqError::Other("Host was neither Ipv4 nor Ipv6")),
        }
    }
}

impl From<IpAddr> for Host {
    fn from(a: IpAddr) -> Self {
        match a {
            IpAddr::V4(a) => Host::Ipv4(a),
            IpAddr::V6(a) => Host::Ipv6(a),
        }
    }
}

impl TryFrom<String> for Host {
    type Error = EndpointError;

    /// An Ipv6 address must be enclosed by `[` and `]`.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.is_empty() {
            return Err(EndpointError::Syntax("Host string should not be empty"));
        }
        if let Ok(addr) = s.parse::<Ipv4Addr>() {
            return Ok(Host::Ipv4(addr));
        }

        // Attempt to parse ipv6 from either ::1 or [::1] using ascii
        let ipv6_substr =
            if s.starts_with('[') && s.len() >= 4 && *s.as_bytes().last().unwrap() == b']' {
                let substr = &s[1..s.len() - 1];
                debug_assert_eq!(substr.len(), s.len() - 2);
                substr
            } else {
                &s
            };
        if let Ok(addr) = ipv6_substr.parse::<Ipv6Addr>() {
            return Ok(Host::Ipv6(addr));
        }

        Ok(Host::Domain(s))
    }
}

impl FromStr for Host {
    type Err = EndpointError;

    /// Equivalent to [`TryFrom<String>`]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_string();
        Self::try_from(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // These two tests on std are more for reference than any real test of
    // functionality
    #[test]
    fn std_ipv6_parse() {
        assert_eq!(Ipv6Addr::LOCALHOST, "::1".parse::<Ipv6Addr>().unwrap());
        assert!("[::1]".parse::<Ipv6Addr>().is_err());
    }

    #[test]
    fn std_ipv6_display() {
        assert_eq!("::1", &Ipv6Addr::LOCALHOST.to_string());
    }

    #[test]
    fn parse_and_display_nobracket_ipv6_same_as_std() {
        let valid_addr_strs = vec![
            "::1",
            "::",
            "2001:db8:a::123",
            "2001:db8:0:0:0:0:2:1",
            "2001:db8::2:1",
        ];
        let invalid_addr_strs = vec!["", "[]", "[:]", ":"];

        for valid in valid_addr_strs {
            let parsed_std = valid.parse::<Ipv6Addr>().unwrap();
            let parsed_host = valid.parse::<Host>().unwrap();
            if let Host::Ipv6(parsed_host) = &parsed_host {
                // Check that both are structurally the same
                assert_eq!(&parsed_std, parsed_host);
            } else {
                panic!("Did not parse as IPV6!");
            }
            // Check that both display as the same
            assert_eq!(parsed_std.to_string(), parsed_host.to_string());
        }

        for invalid in invalid_addr_strs {
            invalid.parse::<Ipv6Addr>().unwrap_err();
            let parsed_host = invalid.parse::<Host>();
            if parsed_host.is_err() {
                continue;
            }
            let parsed_host = parsed_host.unwrap();
            if let Host::Domain(_) = parsed_host {
                continue;
            }
            panic!(
                "Expected that \"{}\" would not parse as Ipv6 or Ipv4, but instead it parsed as {:?}",
                invalid, parsed_host
            );
        }
    }

    #[test]
    fn parse_and_display_bracket_ipv6() {
        let addr_strs = vec![
            "[::1]",
            "[::]",
            "[2001:db8:a::123]",
            "[2001:db8:0:0:0:0:2:1]",
            "[2001:db8::2:1]",
        ];

        fn remove_brackets(s: &str) -> &str {
            assert!(s.starts_with('['));
            assert!(s.ends_with(']'));
            let result = &s[1..s.len() - 1];
            assert_eq!(result.len(), s.len() - 2);
            result
        }

        for addr_str in addr_strs {
            let parsed_host: Host = addr_str.parse().unwrap();
            assert!(addr_str.parse::<Ipv6Addr>().is_err());

            if let Host::Ipv6(host_ipv6) = parsed_host {
                assert_eq!(
                    host_ipv6,
                    remove_brackets(addr_str).parse::<Ipv6Addr>().unwrap()
                );
                assert_eq!(parsed_host.to_string(), host_ipv6.to_string());
            } else {
                panic!(
                    "Expected host to parse as Ipv6, but instead got {:?}",
                    parsed_host
                );
            }
        }
    }
}
