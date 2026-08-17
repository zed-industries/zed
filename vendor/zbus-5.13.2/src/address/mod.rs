//! D-Bus address handling.
//!
//! Server addresses consist of a transport name followed by a colon, and then an optional,
//! comma-separated list of keys and values in the form key=value.
//!
//! See also:
//!
//! * [Server addresses] in the D-Bus specification.
//!
//! [Server addresses]: https://dbus.freedesktop.org/doc/dbus-specification.html#addresses

pub mod transport;

use crate::{Error, Guid, OwnedGuid, Result};
#[cfg(all(unix, not(target_os = "macos")))]
use rustix::process::geteuid;
use std::{collections::HashMap, env, str::FromStr};

use std::fmt::{Display, Formatter};

use self::transport::Stream;
pub use self::transport::Transport;

/// A bus address.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Address {
    guid: Option<OwnedGuid>,
    transport: Transport,
}

impl Address {
    /// Create a new `Address` from a `Transport`.
    pub fn new(transport: Transport) -> Self {
        Self {
            transport,
            guid: None,
        }
    }

    /// Set the GUID for this address.
    pub fn set_guid<G>(mut self, guid: G) -> Result<Self>
    where
        G: TryInto<OwnedGuid>,
        G::Error: Into<crate::Error>,
    {
        self.guid = Some(guid.try_into().map_err(Into::into)?);

        Ok(self)
    }

    /// The transport details for this address.
    pub fn transport(&self) -> &Transport {
        &self.transport
    }

    #[cfg_attr(any(target_os = "macos", windows), async_recursion::async_recursion)]
    pub(crate) async fn connect(self) -> Result<Stream> {
        self.transport.connect().await
    }

    /// Get the address for the session socket respecting the `DBUS_SESSION_BUS_ADDRESS` environment
    /// variable. If we don't recognize the value (or it's not set) we fall back to
    /// `$XDG_RUNTIME_DIR/bus`.
    pub fn session() -> Result<Self> {
        match env::var("DBUS_SESSION_BUS_ADDRESS") {
            Ok(val) => Self::from_str(&val),
            _ => {
                #[cfg(windows)]
                return Self::from_str("autolaunch:");

                #[cfg(all(unix, not(target_os = "macos")))]
                {
                    let runtime_dir = env::var("XDG_RUNTIME_DIR")
                        .unwrap_or_else(|_| format!("/run/user/{}", geteuid().as_raw()));
                    let path = format!("unix:path={runtime_dir}/bus");

                    Self::from_str(&path)
                }

                #[cfg(target_os = "macos")]
                return Self::from_str("launchd:env=DBUS_LAUNCHD_SESSION_BUS_SOCKET");
            }
        }
    }

    /// Get the address for the system bus respecting the `DBUS_SYSTEM_BUS_ADDRESS` environment
    /// variable. If we don't recognize the value (or it's not set) we fall back to
    /// `/var/run/dbus/system_bus_socket`.
    pub fn system() -> Result<Self> {
        match env::var("DBUS_SYSTEM_BUS_ADDRESS") {
            Ok(val) => Self::from_str(&val),
            _ => {
                #[cfg(all(unix, not(target_os = "macos")))]
                return Self::from_str("unix:path=/var/run/dbus/system_bus_socket");

                #[cfg(windows)]
                return Self::from_str("autolaunch:");

                #[cfg(target_os = "macos")]
                return Self::from_str("launchd:env=DBUS_LAUNCHD_SESSION_BUS_SOCKET");
            }
        }
    }

    /// The GUID for this address, if known.
    pub fn guid(&self) -> Option<&Guid<'_>> {
        self.guid.as_ref().map(|guid| guid.inner())
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.transport.fmt(f)?;

        if let Some(guid) = &self.guid {
            write!(f, ",guid={guid}")?;
        }

        Ok(())
    }
}

impl FromStr for Address {
    type Err = Error;

    /// Parse the transport part of a D-Bus address into a `Transport`.
    fn from_str(address: &str) -> Result<Self> {
        use std::str::from_utf8_unchecked;
        use winnow::{
            Parser,
            ascii::alphanumeric1,
            combinator::separated,
            token::{take_until, take_while},
        };

        // All currently defined keys are alphanumber only. Change the paser when/if this changes.
        let key = alphanumeric1::<_, ()>;
        let value = take_while(1.., |b| b != b',');
        let kv = (key, b'=', value).map(|(k, _, v)| {
            // SAFETY: We got the bytes off a `&str` so they're guaranteed to be UTF-8 only.
            unsafe { (from_utf8_unchecked(k), from_utf8_unchecked(v)) }
        });
        let options_parse = separated(0.., kv, b',');

        let transport_parse = take_until(1.., b':').map(|bytes| {
            // SAFETY: We got the bytes off a `&str` so they're guaranteed to be UTF-8 only.
            unsafe { from_utf8_unchecked(bytes) }
        });

        (transport_parse, b':', options_parse)
            .parse(address.as_bytes())
            .map_err(|_| {
                Error::Address(
                    "Invalid address. \
                    See https://dbus.freedesktop.org/doc/dbus-specification.html#addresses"
                        .to_string(),
                )
            })
            .and_then(|(transport, _, opts): (_, _, HashMap<_, _>)| {
                let guid = opts
                    .get("guid")
                    .map(|s| Guid::from_str(s).map(|guid| OwnedGuid::from(guid).to_owned()))
                    .transpose()?;
                let transport = Transport::from_options(transport, opts)?;

                Ok(Address { guid, transport })
            })
    }
}

impl TryFrom<&str> for Address {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::from_str(value)
    }
}

impl From<Transport> for Address {
    fn from(transport: Transport) -> Self {
        Self::new(transport)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Address,
        transport::{Tcp, TcpTransportFamily, Transport},
    };
    #[cfg(target_os = "macos")]
    use crate::address::transport::Launchd;
    #[cfg(unix)]
    use crate::address::transport::Unixexec;
    #[cfg(windows)]
    use crate::address::transport::{Autolaunch, AutolaunchScope};
    use crate::address::transport::{Unix, UnixSocket};
    use std::str::FromStr;
    use test_log::test;

    #[test]
    fn parse_dbus_addresses() {
        assert!(Address::from_str("").is_err());
        assert!(Address::from_str("foo").is_err());
        assert!(Address::from_str("foo:opt").is_err());
        assert!(Address::from_str("foo:opt=1,opt=2").is_err());
        assert!(Address::from_str("tcp:host=localhost").is_err());
        assert!(Address::from_str("tcp:host=localhost,port=32f").is_err());
        assert!(Address::from_str("tcp:host=localhost,port=123,family=ipv7").is_err());
        assert!(Address::from_str("unix:foo=blah").is_err());
        #[cfg(target_os = "linux")]
        assert!(Address::from_str("unix:path=/tmp,abstract=foo").is_err());
        #[cfg(unix)]
        assert!(Address::from_str("unixexec:foo=blah").is_err());
        assert_eq!(
            Address::from_str("unix:path=/tmp/dbus-foo").unwrap(),
            Transport::Unix(Unix::new(UnixSocket::File("/tmp/dbus-foo".into()))).into(),
        );
        #[cfg(target_os = "linux")]
        assert_eq!(
            Address::from_str("unix:abstract=/tmp/dbus-foo").unwrap(),
            Transport::Unix(Unix::new(UnixSocket::Abstract("/tmp/dbus-foo".into()))).into(),
        );
        #[cfg(feature = "p2p")]
        {
            let guid = crate::Guid::generate();
            assert_eq!(
                Address::from_str(&format!("unix:path=/tmp/dbus-foo,guid={guid}")).unwrap(),
                Address::from(Transport::Unix(Unix::new(UnixSocket::File(
                    "/tmp/dbus-foo".into()
                ))))
                .set_guid(guid.clone())
                .unwrap(),
            );
        }
        #[cfg(unix)]
        assert_eq!(
            Address::from_str("unixexec:path=/tmp/dbus-foo").unwrap(),
            Transport::Unixexec(Unixexec::new("/tmp/dbus-foo".into(), None, Vec::new())).into(),
        );
        assert_eq!(
            Address::from_str("tcp:host=localhost,port=4142").unwrap(),
            Transport::Tcp(Tcp::new("localhost", 4142)).into(),
        );
        assert_eq!(
            Address::from_str("tcp:host=localhost,port=4142,family=ipv4").unwrap(),
            Transport::Tcp(Tcp::new("localhost", 4142).set_family(Some(TcpTransportFamily::Ipv4)))
                .into(),
        );
        assert_eq!(
            Address::from_str("tcp:host=localhost,port=4142,family=ipv6").unwrap(),
            Transport::Tcp(Tcp::new("localhost", 4142).set_family(Some(TcpTransportFamily::Ipv6)))
                .into(),
        );
        assert_eq!(
            Address::from_str("tcp:host=localhost,port=4142,family=ipv6,noncefile=/a/file/path")
                .unwrap(),
            Transport::Tcp(
                Tcp::new("localhost", 4142)
                    .set_family(Some(TcpTransportFamily::Ipv6))
                    .set_nonce_file(Some(b"/a/file/path".to_vec()))
            )
            .into(),
        );
        assert_eq!(
            Address::from_str(
                "nonce-tcp:host=localhost,port=4142,family=ipv6,noncefile=/a/file/path%20to%20file%201234"
            )
            .unwrap(),
            Transport::Tcp(
                Tcp::new("localhost", 4142)
                    .set_family(Some(TcpTransportFamily::Ipv6))
                    .set_nonce_file(Some(b"/a/file/path to file 1234".to_vec()))
            ).into()
        );
        #[cfg(windows)]
        assert_eq!(
            Address::from_str("autolaunch:").unwrap(),
            Transport::Autolaunch(Autolaunch::new()).into(),
        );
        #[cfg(windows)]
        assert_eq!(
            Address::from_str("autolaunch:scope=*my_cool_scope*").unwrap(),
            Transport::Autolaunch(
                Autolaunch::new()
                    .set_scope(Some(AutolaunchScope::Other("*my_cool_scope*".to_string())))
            )
            .into(),
        );
        #[cfg(target_os = "macos")]
        assert_eq!(
            Address::from_str("launchd:env=my_cool_env_key").unwrap(),
            Transport::Launchd(Launchd::new("my_cool_env_key")).into(),
        );

        #[cfg(all(feature = "vsock", feature = "p2p", not(feature = "tokio")))]
        {
            let guid = crate::Guid::generate();
            assert_eq!(
                Address::from_str(&format!("vsock:cid=98,port=2934,guid={guid}")).unwrap(),
                Address::from(Transport::Vsock(super::transport::Vsock::new(98, 2934)))
                    .set_guid(guid)
                    .unwrap(),
            );
        }
        assert_eq!(
            Address::from_str("unix:dir=/some/dir").unwrap(),
            Transport::Unix(Unix::new(UnixSocket::Dir("/some/dir".into()))).into(),
        );
        assert_eq!(
            Address::from_str("unix:tmpdir=/some/dir").unwrap(),
            Transport::Unix(Unix::new(UnixSocket::TmpDir("/some/dir".into()))).into(),
        );
    }

    #[test]
    fn stringify_dbus_addresses() {
        assert_eq!(
            Address::from(Transport::Unix(Unix::new(UnixSocket::File(
                "/tmp/dbus-foo".into()
            ))))
            .to_string(),
            "unix:path=/tmp/dbus-foo",
        );
        assert_eq!(
            Address::from(Transport::Unix(Unix::new(UnixSocket::Dir(
                "/tmp/dbus-foo".into()
            ))))
            .to_string(),
            "unix:dir=/tmp/dbus-foo",
        );
        assert_eq!(
            Address::from(Transport::Unix(Unix::new(UnixSocket::TmpDir(
                "/tmp/dbus-foo".into()
            ))))
            .to_string(),
            "unix:tmpdir=/tmp/dbus-foo"
        );
        // FIXME: figure out how to handle abstract on Windows
        #[cfg(target_os = "linux")]
        assert_eq!(
            Address::from(Transport::Unix(Unix::new(UnixSocket::Abstract(
                "/tmp/dbus-foo".into()
            ))))
            .to_string(),
            "unix:abstract=/tmp/dbus-foo"
        );
        assert_eq!(
            Address::from(Transport::Tcp(Tcp::new("localhost", 4142))).to_string(),
            "tcp:host=localhost,port=4142"
        );
        assert_eq!(
            Address::from(Transport::Tcp(
                Tcp::new("localhost", 4142).set_family(Some(TcpTransportFamily::Ipv4))
            ))
            .to_string(),
            "tcp:host=localhost,port=4142,family=ipv4"
        );
        assert_eq!(
            Address::from(Transport::Tcp(
                Tcp::new("localhost", 4142).set_family(Some(TcpTransportFamily::Ipv6))
            ))
            .to_string(),
            "tcp:host=localhost,port=4142,family=ipv6"
        );
        assert_eq!(
            Address::from(Transport::Tcp(
                Tcp::new("localhost", 4142)
                    .set_family(Some(TcpTransportFamily::Ipv6))
                    .set_nonce_file(Some(b"/a/file/path to file 1234".to_vec()))
            ))
            .to_string(),
            "nonce-tcp:noncefile=/a/file/path%20to%20file%201234,host=localhost,port=4142,family=ipv6"
        );
        #[cfg(windows)]
        assert_eq!(
            Address::from(Transport::Autolaunch(Autolaunch::new())).to_string(),
            "autolaunch:"
        );
        #[cfg(windows)]
        assert_eq!(
            Address::from(Transport::Autolaunch(Autolaunch::new().set_scope(Some(
                AutolaunchScope::Other("*my_cool_scope*".to_string())
            ))))
            .to_string(),
            "autolaunch:scope=*my_cool_scope*"
        );
        #[cfg(target_os = "macos")]
        assert_eq!(
            Address::from(Transport::Launchd(Launchd::new("my_cool_key"))).to_string(),
            "launchd:env=my_cool_key"
        );

        #[cfg(all(feature = "vsock", feature = "p2p", not(feature = "tokio")))]
        {
            let guid = crate::Guid::generate();
            assert_eq!(
                Address::from(Transport::Vsock(super::transport::Vsock::new(98, 2934)))
                    .set_guid(guid.clone())
                    .unwrap()
                    .to_string(),
                format!("vsock:cid=98,port=2934,guid={guid}"),
            );
        }
    }

    #[test]
    fn connect_tcp() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let addr = Address::from_str(&format!("tcp:host=localhost,port={port}")).unwrap();
        crate::utils::block_on(async { addr.connect().await }).unwrap();
    }

    #[test]
    fn connect_nonce_tcp() {
        struct PercentEncoded<'a>(&'a [u8]);

        impl std::fmt::Display for PercentEncoded<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                super::transport::encode_percents(f, self.0)
            }
        }

        use std::io::Write;

        const TEST_COOKIE: &[u8] = b"VERILY SECRETIVE";

        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        let mut cookie = tempfile::NamedTempFile::new().unwrap();
        cookie.as_file_mut().write_all(TEST_COOKIE).unwrap();

        let encoded_path = format!(
            "{}",
            PercentEncoded(cookie.path().to_str().unwrap().as_ref())
        );

        let addr = Address::from_str(&format!(
            "nonce-tcp:host=localhost,port={port},noncefile={encoded_path}"
        ))
        .unwrap();

        let (sender, receiver) = std::sync::mpsc::sync_channel(1);

        std::thread::spawn(move || {
            use std::io::Read;

            let mut client = listener.incoming().next().unwrap().unwrap();

            let mut buf = [0u8; 16];
            client.read_exact(&mut buf).unwrap();

            sender.send(buf == TEST_COOKIE).unwrap();
        });

        crate::utils::block_on(addr.connect()).unwrap();

        let saw_cookie = receiver
            .recv_timeout(std::time::Duration::from_millis(100))
            .expect("nonce file content hasn't been received by server thread in time");

        assert!(
            saw_cookie,
            "nonce file content has been received, but was invalid"
        );
    }
}
