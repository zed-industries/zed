//! socks proxy

use url::Url;

/// Identification to a Socks V4 Proxy
pub(super) struct Socks4Identification<'a> {
    pub(super) user_id: &'a str,
}

/// Authorization to a Socks V5 Proxy
pub(super) struct Socks5Authorization<'a> {
    pub(super) username: &'a str,
    pub(super) password: &'a str,
}

/// Socks Proxy Protocol Version
///
/// V4 allows idenfication using a user_id
/// V5 allows authorization using a username and password
pub(super) enum SocksVersion<'a> {
    V4(Option<Socks4Identification<'a>>),
    V5(Option<Socks5Authorization<'a>>),
}

pub(super) fn parse_socks_proxy<'t>(scheme: &str, proxy: &'t Url) -> Option<SocksVersion<'t>> {
    if scheme.starts_with("socks4") {
        let identification = match proxy.username() {
            "" => None,
            username => Some(Socks4Identification { user_id: username }),
        };
        Some(SocksVersion::V4(identification))
    } else if scheme.starts_with("socks") {
        let authorization = proxy.password().map(|password| Socks5Authorization {
            username: proxy.username(),
            password,
        });
        Some(SocksVersion::V5(authorization))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::*;

    #[test]
    fn parse_socks4() {
        let proxy = Url::parse("socks4://proxy.example.com:1080").unwrap();

        let ((host, port), version) = parse_socks_proxy(&proxy).unwrap();
        assert_eq!(host, "proxy.example.com");
        assert_eq!(port, 1080);
        assert!(matches!(version, SocksVersion::V4(None)))
    }

    #[test]
    fn parse_socks4_with_identification() {
        let proxy = Url::parse("socks4://userid@proxy.example.com:1080").unwrap();

        let ((host, port), version) = parse_socks_proxy(&proxy).unwrap();
        assert_eq!(host, "proxy.example.com");
        assert_eq!(port, 1080);
        assert!(matches!(
            version,
            SocksVersion::V4(Some(Socks4Identification { user_id: "userid" }))
        ))
    }

    #[test]
    fn parse_socks5() {
        let proxy = Url::parse("socks5://proxy.example.com:1080").unwrap();

        let ((host, port), version) = parse_socks_proxy(&proxy).unwrap();
        assert_eq!(host, "proxy.example.com");
        assert_eq!(port, 1080);
        assert!(matches!(version, SocksVersion::V5(None)))
    }

    #[test]
    fn parse_socks5_with_authorization() {
        let proxy = Url::parse("socks5://username:password@proxy.example.com:1080").unwrap();

        let ((host, port), version) = parse_socks_proxy(&proxy).unwrap();
        assert_eq!(host, "proxy.example.com");
        assert_eq!(port, 1080);
        assert!(matches!(
            version,
            SocksVersion::V5(Some(Socks5Authorization {
                username: "username",
                password: "password"
            }))
        ))
    }
}
