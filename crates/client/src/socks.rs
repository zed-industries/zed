//! socks proxy
use anyhow::{Context, Result, anyhow};
use http_client::Url;
use tokio_socks::tcp::{Socks4Stream, Socks5Stream};

/// Identification to a Socks V4 Proxy
struct Socks4Identification<'a> {
    user_id: &'a str,
}

/// Authorization to a Socks V5 Proxy
struct Socks5Authorization<'a> {
    username: &'a str,
    password: &'a str,
}

/// Socks Proxy Protocol Version
///
/// V4 allows idenfication using a user_id
/// V5 allows authorization using a username and password
enum SocksVersion<'a> {
    V4(Option<Socks4Identification<'a>>),
    V5(Option<Socks5Authorization<'a>>),
}

pub(crate) async fn connect_socks_proxy_stream(
    proxy: &Url,
    rpc_host: (&str, u16),
) -> Result<Box<dyn AsyncReadWrite>> {
    let Some((socks_proxy, version)) = parse_socks_proxy(proxy) else {
        // If parsing the proxy URL fails, we must avoid falling back to an insecure connection.
        // SOCKS proxies are often used in contexts where security and privacy are critical,
        // so any fallback could expose users to significant risks.
        return Err(anyhow!("Parsing proxy url failed"));
    };

    // Connect to proxy and wrap protocol later
    let stream = tokio::net::TcpStream::connect(socks_proxy)
        .await
        .context("Failed to connect to socks proxy")?;

    let socks: Box<dyn AsyncReadWrite> = match version {
        SocksVersion::V4(None) => {
            let socks = Socks4Stream::connect_with_socket(stream, rpc_host)
                .await
                .map_err(|err| anyhow!("error connecting to socks {}", err))?;
            Box::new(socks)
        }
        SocksVersion::V4(Some(Socks4Identification { user_id })) => {
            let socks = Socks4Stream::connect_with_userid_and_socket(stream, rpc_host, user_id)
                .await
                .map_err(|err| anyhow!("error connecting to socks {}", err))?;
            Box::new(socks)
        }
        SocksVersion::V5(None) => {
            let socks = Socks5Stream::connect_with_socket(stream, rpc_host)
                .await
                .map_err(|err| anyhow!("error connecting to socks {}", err))?;
            Box::new(socks)
        }
        SocksVersion::V5(Some(Socks5Authorization { username, password })) => {
            let socks = Socks5Stream::connect_with_password_and_socket(
                stream, rpc_host, username, password,
            )
            .await
            .map_err(|err| anyhow!("error connecting to socks {}", err))?;
            Box::new(socks)
        }
    };

    Ok(socks)
}

fn parse_socks_proxy(proxy: &Url) -> Option<((String, u16), SocksVersion<'_>)> {
    let scheme = proxy.scheme();
    let socks_version = if scheme.starts_with("socks4") {
        let identification = match proxy.username() {
            "" => None,
            username => Some(Socks4Identification { user_id: username }),
        };
        SocksVersion::V4(identification)
    } else if scheme.starts_with("socks") {
        let authorization = proxy.password().map(|password| Socks5Authorization {
            username: proxy.username(),
            password,
        });
        SocksVersion::V5(authorization)
    } else {
        return None;
    };

    let host = proxy.host()?.to_string();
    let port = proxy.port_or_known_default()?;

    Some(((host, port), socks_version))
}

pub(crate) trait AsyncReadWrite:
    tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static
{
}
impl<T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static> AsyncReadWrite
    for T
{
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

    /// If parsing the proxy URL fails, we must avoid falling back to an insecure connection.
    /// SOCKS proxies are often used in contexts where security and privacy are critical,
    /// so any fallback could expose users to significant risks.
    #[tokio::test]
    async fn fails_on_bad_proxy() {
        // Should fail connecting because http is not a valid Socks proxy scheme
        let proxy = Url::parse("http://localhost:2313").unwrap();

        let result = connect_socks_proxy_stream(&proxy, ("test", 1080)).await;
        match result {
            Err(e) => assert_eq!(e.to_string(), "Parsing proxy url failed"),
            Ok(_) => panic!("Connecting on bad proxy should fail"),
        };
    }
}
