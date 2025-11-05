//! socks proxy

use anyhow::{Context as _, Result};
use http_client::Url;
use tokio::net::TcpStream;
use tokio_socks::{
    IntoTargetAddr, TargetAddr,
    tcp::{Socks4Stream, Socks5Stream},
};

use super::AsyncReadWrite;

/// Identification to a Socks V4 Proxy
pub(super) struct Socks4Identification<'a> {
    user_id: &'a str,
}

/// Authorization to a Socks V5 Proxy
pub(super) struct Socks5Authorization<'a> {
    username: &'a str,
    password: &'a str,
}

/// Socks Proxy Protocol Version
///
/// V4 allows identification using a user_id
/// V5 allows authorization using a username and password
pub(super) enum SocksVersion<'a> {
    V4 {
        local_dns: bool,
        identification: Option<Socks4Identification<'a>>,
    },
    V5 {
        local_dns: bool,
        authorization: Option<Socks5Authorization<'a>>,
    },
}

pub(super) fn parse_socks_proxy<'t>(scheme: &str, proxy: &'t Url) -> SocksVersion<'t> {
    if scheme.starts_with("socks4") {
        let identification = match proxy.username() {
            "" => None,
            username => Some(Socks4Identification { user_id: username }),
        };
        SocksVersion::V4 {
            local_dns: scheme != "socks4a",
            identification,
        }
    } else {
        let authorization = proxy.password().map(|password| Socks5Authorization {
            username: proxy.username(),
            password,
        });
        SocksVersion::V5 {
            local_dns: scheme != "socks5h",
            authorization,
        }
    }
}

pub(super) async fn connect_socks_proxy_stream(
    stream: TcpStream,
    socks_version: SocksVersion<'_>,
    rpc_host: (&str, u16),
) -> Result<Box<dyn AsyncReadWrite>> {
    let rpc_host = rpc_host
        .into_target_addr()
        .context("Failed to parse target addr")?;

    let local_dns = match &socks_version {
        SocksVersion::V4 { local_dns, .. } => local_dns,
        SocksVersion::V5 { local_dns, .. } => local_dns,
    };
    let rpc_host = match (rpc_host, local_dns) {
        (TargetAddr::Domain(domain, port), true) => {
            let ip_addr = tokio::net::lookup_host((domain.as_ref(), port))
                .await
                .with_context(|| format!("Failed to lookup domain {}", domain))?
                .next()
                .ok_or_else(|| anyhow::anyhow!("Failed to lookup domain {}", domain))?;
            TargetAddr::Ip(ip_addr)
        }
        (rpc_host, _) => rpc_host,
    };

    match socks_version {
        SocksVersion::V4 {
            identification: None,
            ..
        } => {
            let socks = Socks4Stream::connect_with_socket(stream, rpc_host)
                .await
                .context("error connecting to socks")?;
            Ok(Box::new(socks))
        }
        SocksVersion::V4 {
            identification: Some(Socks4Identification { user_id }),
            ..
        } => {
            let socks = Socks4Stream::connect_with_userid_and_socket(stream, rpc_host, user_id)
                .await
                .context("error connecting to socks")?;
            Ok(Box::new(socks))
        }
        SocksVersion::V5 {
            authorization: None,
            ..
        } => {
            let socks = Socks5Stream::connect_with_socket(stream, rpc_host)
                .await
                .context("error connecting to socks")?;
            Ok(Box::new(socks))
        }
        SocksVersion::V5 {
            authorization: Some(Socks5Authorization { username, password }),
            ..
        } => {
            let socks = Socks5Stream::connect_with_password_and_socket(
                stream, rpc_host, username, password,
            )
            .await
            .context("error connecting to socks")?;
            Ok(Box::new(socks))
        }
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::*;

    #[test]
    fn parse_socks4() {
        let proxy = Url::parse("socks4://proxy.example.com:1080").unwrap();
        let scheme = proxy.scheme();

        let version = parse_socks_proxy(scheme, &proxy);
        assert!(matches!(
            version,
            SocksVersion::V4 {
                local_dns: true,
                identification: None
            }
        ))
    }

    #[test]
    fn parse_socks4_with_identification() {
        let proxy = Url::parse("socks4://userid@proxy.example.com:1080").unwrap();
        let scheme = proxy.scheme();

        let version = parse_socks_proxy(scheme, &proxy);
        assert!(matches!(
            version,
            SocksVersion::V4 {
                local_dns: true,
                identification: Some(Socks4Identification { user_id: "userid" })
            }
        ))
    }

    #[test]
    fn parse_socks4_with_remote_dns() {
        let proxy = Url::parse("socks4a://proxy.example.com:1080").unwrap();
        let scheme = proxy.scheme();

        let version = parse_socks_proxy(scheme, &proxy);
        assert!(matches!(
            version,
            SocksVersion::V4 {
                local_dns: false,
                identification: None
            }
        ))
    }

    #[test]
    fn parse_socks5() {
        let proxy = Url::parse("socks5://proxy.example.com:1080").unwrap();
        let scheme = proxy.scheme();

        let version = parse_socks_proxy(scheme, &proxy);
        assert!(matches!(
            version,
            SocksVersion::V5 {
                local_dns: true,
                authorization: None
            }
        ))
    }

    #[test]
    fn parse_socks5_with_authorization() {
        let proxy = Url::parse("socks5://username:password@proxy.example.com:1080").unwrap();
        let scheme = proxy.scheme();

        let version = parse_socks_proxy(scheme, &proxy);
        assert!(matches!(
            version,
            SocksVersion::V5 {
                local_dns: true,
                authorization: Some(Socks5Authorization {
                    username: "username",
                    password: "password"
                })
            }
        ))
    }

    #[test]
    fn parse_socks5_with_remote_dns() {
        let proxy = Url::parse("socks5h://proxy.example.com:1080").unwrap();
        let scheme = proxy.scheme();

        let version = parse_socks_proxy(scheme, &proxy);
        assert!(matches!(
            version,
            SocksVersion::V5 {
                local_dns: false,
                authorization: None
            }
        ))
    }
}
