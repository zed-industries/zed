use core::fmt;
use core::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use core::str::FromStr as _;
use core::time::Duration;

use cap_net_ext::{AddressFamily, Blocking, UdpSocketExt};
use rustix::fd::AsFd;
use rustix::io::Errno;
use rustix::net::{bind, connect_unspec, sockopt};
use tracing::debug;

use crate::sockets::SocketAddressFamily;

#[derive(Debug)]
pub enum ErrorCode {
    Unknown,
    AccessDenied,
    NotSupported,
    InvalidArgument,
    OutOfMemory,
    Timeout,
    InvalidState,
    AddressNotBindable,
    AddressInUse,
    RemoteUnreachable,
    ConnectionRefused,
    ConnectionReset,
    ConnectionAborted,
    DatagramTooLarge,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for ErrorCode {}

fn is_deprecated_ipv4_compatible(addr: Ipv6Addr) -> bool {
    matches!(addr.segments(), [0, 0, 0, 0, 0, 0, _, _])
        && addr != Ipv6Addr::UNSPECIFIED
        && addr != Ipv6Addr::LOCALHOST
}

pub fn is_valid_address_family(addr: IpAddr, socket_family: SocketAddressFamily) -> bool {
    match (socket_family, addr) {
        (SocketAddressFamily::Ipv4, IpAddr::V4(..)) => true,
        (SocketAddressFamily::Ipv6, IpAddr::V6(ipv6)) => {
            // Reject IPv4-*compatible* IPv6 addresses. They have been deprecated
            // since 2006, OS handling of them is inconsistent and our own
            // validations don't take them into account either.
            // Note that these are not the same as IPv4-*mapped* IPv6 addresses.
            !is_deprecated_ipv4_compatible(ipv6) && ipv6.to_ipv4_mapped().is_none()
        }
        _ => false,
    }
}

pub fn is_valid_remote_address(addr: SocketAddr) -> bool {
    !addr.ip().to_canonical().is_unspecified() && addr.port() != 0
}

pub fn is_valid_unicast_address(addr: IpAddr) -> bool {
    match addr.to_canonical() {
        IpAddr::V4(ipv4) => !ipv4.is_multicast() && !ipv4.is_broadcast(),
        IpAddr::V6(ipv6) => !ipv6.is_multicast(),
    }
}

pub fn to_ipv4_addr(addr: (u8, u8, u8, u8)) -> Ipv4Addr {
    let (x0, x1, x2, x3) = addr;
    Ipv4Addr::new(x0, x1, x2, x3)
}

pub fn from_ipv4_addr(addr: Ipv4Addr) -> (u8, u8, u8, u8) {
    let [x0, x1, x2, x3] = addr.octets();
    (x0, x1, x2, x3)
}

pub fn to_ipv6_addr(addr: (u16, u16, u16, u16, u16, u16, u16, u16)) -> Ipv6Addr {
    let (x0, x1, x2, x3, x4, x5, x6, x7) = addr;
    Ipv6Addr::new(x0, x1, x2, x3, x4, x5, x6, x7)
}

pub fn from_ipv6_addr(addr: Ipv6Addr) -> (u16, u16, u16, u16, u16, u16, u16, u16) {
    let [x0, x1, x2, x3, x4, x5, x6, x7] = addr.segments();
    (x0, x1, x2, x3, x4, x5, x6, x7)
}

/*
 * Syscalls wrappers with (opinionated) portability fixes.
 */

pub fn normalize_get_buffer_size(value: usize) -> usize {
    if cfg!(target_os = "linux") {
        // Linux doubles the value passed to setsockopt to allow space for bookkeeping overhead.
        // getsockopt returns this internally doubled value.
        // We'll half the value to at least get it back into the same ballpark that the application requested it in.
        //
        // This normalized behavior is tested for in: test-programs/src/bin/preview2_tcp_sockopts.rs
        value / 2
    } else {
        value
    }
}

pub fn normalize_set_buffer_size(value: usize) -> usize {
    value.clamp(1, i32::MAX as usize)
}

impl From<std::io::Error> for ErrorCode {
    fn from(value: std::io::Error) -> Self {
        (&value).into()
    }
}

impl From<&std::io::Error> for ErrorCode {
    fn from(value: &std::io::Error) -> Self {
        // Attempt the more detailed native error code first:
        if let Some(errno) = Errno::from_io_error(value) {
            return errno.into();
        }

        match value.kind() {
            std::io::ErrorKind::AddrInUse => Self::AddressInUse,
            std::io::ErrorKind::AddrNotAvailable => Self::AddressNotBindable,
            std::io::ErrorKind::ConnectionAborted => Self::ConnectionAborted,
            std::io::ErrorKind::ConnectionRefused => Self::ConnectionRefused,
            std::io::ErrorKind::ConnectionReset => Self::ConnectionReset,
            std::io::ErrorKind::InvalidInput => Self::InvalidArgument,
            std::io::ErrorKind::NotConnected => Self::InvalidState,
            std::io::ErrorKind::OutOfMemory => Self::OutOfMemory,
            std::io::ErrorKind::PermissionDenied => Self::AccessDenied,
            std::io::ErrorKind::TimedOut => Self::Timeout,
            std::io::ErrorKind::Unsupported => Self::NotSupported,
            _ => {
                debug!("unknown I/O error: {value}");
                Self::Unknown
            }
        }
    }
}

impl From<Errno> for ErrorCode {
    fn from(value: Errno) -> Self {
        (&value).into()
    }
}

impl From<&Errno> for ErrorCode {
    fn from(value: &Errno) -> Self {
        match *value {
            #[cfg(not(windows))]
            Errno::PERM => Self::AccessDenied,
            Errno::ACCESS => Self::AccessDenied,
            Errno::ADDRINUSE => Self::AddressInUse,
            Errno::ADDRNOTAVAIL => Self::AddressNotBindable,
            Errno::TIMEDOUT => Self::Timeout,
            Errno::CONNREFUSED => Self::ConnectionRefused,
            Errno::CONNRESET => Self::ConnectionReset,
            Errno::CONNABORTED => Self::ConnectionAborted,
            Errno::INVAL => Self::InvalidArgument,
            Errno::HOSTUNREACH => Self::RemoteUnreachable,
            Errno::HOSTDOWN => Self::RemoteUnreachable,
            Errno::NETDOWN => Self::RemoteUnreachable,
            Errno::NETUNREACH => Self::RemoteUnreachable,
            #[cfg(target_os = "linux")]
            Errno::NONET => Self::RemoteUnreachable,
            Errno::ISCONN => Self::InvalidState,
            Errno::NOTCONN => Self::InvalidState,
            Errno::DESTADDRREQ => Self::InvalidState,
            Errno::MSGSIZE => Self::DatagramTooLarge,
            #[cfg(not(windows))]
            Errno::NOMEM => Self::OutOfMemory,
            Errno::NOBUFS => Self::OutOfMemory,
            Errno::OPNOTSUPP => Self::NotSupported,
            Errno::NOPROTOOPT => Self::NotSupported,
            Errno::PFNOSUPPORT => Self::NotSupported,
            Errno::PROTONOSUPPORT => Self::NotSupported,
            Errno::PROTOTYPE => Self::NotSupported,
            Errno::SOCKTNOSUPPORT => Self::NotSupported,
            Errno::AFNOSUPPORT => Self::NotSupported,

            // FYI, EINPROGRESS should have already been handled by connect.
            _ => {
                debug!("unknown I/O error: {value}");
                Self::Unknown
            }
        }
    }
}

pub fn get_ip_ttl(fd: impl AsFd) -> Result<u8, ErrorCode> {
    let v = sockopt::ip_ttl(fd)?;
    let Ok(v) = v.try_into() else {
        return Err(ErrorCode::NotSupported);
    };
    Ok(v)
}

pub fn get_ipv6_unicast_hops(fd: impl AsFd) -> Result<u8, ErrorCode> {
    let v = sockopt::ipv6_unicast_hops(fd)?;
    Ok(v)
}

pub fn get_unicast_hop_limit(fd: impl AsFd, family: SocketAddressFamily) -> Result<u8, ErrorCode> {
    match family {
        SocketAddressFamily::Ipv4 => get_ip_ttl(fd),
        SocketAddressFamily::Ipv6 => get_ipv6_unicast_hops(fd),
    }
}

pub fn set_unicast_hop_limit(
    fd: impl AsFd,
    family: SocketAddressFamily,
    value: u8,
) -> Result<(), ErrorCode> {
    if value == 0 {
        // WIT: "If the provided value is 0, an `invalid-argument` error is returned."
        //
        // A well-behaved IP application should never send out new packets with TTL 0.
        // We validate the value ourselves because OS'es are not consistent in this.
        // On Linux the validation is even inconsistent between their IPv4 and IPv6 implementation.
        return Err(ErrorCode::InvalidArgument);
    }
    match family {
        SocketAddressFamily::Ipv4 => {
            sockopt::set_ip_ttl(fd, value.into())?;
        }
        SocketAddressFamily::Ipv6 => {
            sockopt::set_ipv6_unicast_hops(fd, Some(value))?;
        }
    }
    Ok(())
}

pub fn receive_buffer_size(fd: impl AsFd) -> Result<u64, ErrorCode> {
    let v = sockopt::socket_recv_buffer_size(fd)?;
    Ok(normalize_get_buffer_size(v).try_into().unwrap_or(u64::MAX))
}

pub fn set_receive_buffer_size(fd: impl AsFd, value: u64) -> Result<usize, ErrorCode> {
    if value == 0 {
        // WIT: "If the provided value is 0, an `invalid-argument` error is returned."
        return Err(ErrorCode::InvalidArgument);
    }
    let value = value.try_into().unwrap_or(usize::MAX);
    let value = normalize_set_buffer_size(value);
    match sockopt::set_socket_recv_buffer_size(fd, value) {
        // Most platforms (Linux, Windows, Fuchsia, Solaris, Illumos, Haiku, ESP-IDF, ..and more?) treat the value
        // passed to SO_SNDBUF/SO_RCVBUF as a performance tuning hint and silently clamp the input if it exceeds
        // their capability.
        // As far as I can see, only the *BSD family views this option as a hard requirement and fails when the
        // value is out of range. We normalize this behavior in favor of the more commonly understood
        // "performance hint" semantics. In other words; even ENOBUFS is "Ok".
        // A future improvement could be to query the corresponding sysctl on *BSD platforms and clamp the input
        // `size` ourselves, to completely close the gap with other platforms.
        //
        // This normalized behavior is tested for in: test-programs/src/bin/preview2_tcp_sockopts.rs
        Err(Errno::NOBUFS) => {}
        Err(err) => return Err(err.into()),
        _ => {}
    };
    Ok(value)
}

pub fn send_buffer_size(fd: impl AsFd) -> Result<u64, ErrorCode> {
    let v = sockopt::socket_send_buffer_size(fd)?;
    Ok(normalize_get_buffer_size(v).try_into().unwrap_or(u64::MAX))
}

pub fn set_send_buffer_size(fd: impl AsFd, value: u64) -> Result<usize, ErrorCode> {
    if value == 0 {
        // WIT: "If the provided value is 0, an `invalid-argument` error is returned."
        return Err(ErrorCode::InvalidArgument);
    }
    let value = value.try_into().unwrap_or(usize::MAX);
    let value = normalize_set_buffer_size(value);
    match sockopt::set_socket_send_buffer_size(fd, value) {
        Err(Errno::NOBUFS) => {}
        Err(err) => return Err(err.into()),
        _ => {}
    };
    Ok(value)
}

pub fn set_keep_alive_idle_time(fd: impl AsFd, value: u64) -> Result<u64, ErrorCode> {
    const NANOS_PER_SEC: u64 = 1_000_000_000;

    // Ensure that the value passed to the actual syscall never gets rounded down to 0.
    const MIN: u64 = NANOS_PER_SEC;

    // Cap it at Linux' maximum, which appears to have the lowest limit across our supported platforms.
    const MAX: u64 = (i16::MAX as u64) * NANOS_PER_SEC;

    if value <= 0 {
        // WIT: "If the provided value is 0, an `invalid-argument` error is returned."
        return Err(ErrorCode::InvalidArgument);
    }
    let value = value.clamp(MIN, MAX);
    sockopt::set_tcp_keepidle(fd, Duration::from_nanos(value))?;
    Ok(value)
}

pub fn set_keep_alive_interval(fd: impl AsFd, value: Duration) -> Result<(), ErrorCode> {
    // Ensure that any fractional value passed to the actual syscall never gets rounded down to 0.
    const MIN: Duration = Duration::from_secs(1);

    // Cap it at Linux' maximum, which appears to have the lowest limit across our supported platforms.
    const MAX: Duration = Duration::from_secs(i16::MAX as u64);

    if value <= Duration::ZERO {
        // WIT: "If the provided value is 0, an `invalid-argument` error is returned."
        return Err(ErrorCode::InvalidArgument);
    }
    sockopt::set_tcp_keepintvl(fd, value.clamp(MIN, MAX))?;
    Ok(())
}

pub fn set_keep_alive_count(fd: impl AsFd, value: u32) -> Result<(), ErrorCode> {
    const MIN_CNT: u32 = 1;
    // Cap it at Linux' maximum, which appears to have the lowest limit across our supported platforms.
    const MAX_CNT: u32 = i8::MAX as u32;

    if value == 0 {
        // WIT: "If the provided value is 0, an `invalid-argument` error is returned."
        return Err(ErrorCode::InvalidArgument);
    }
    sockopt::set_tcp_keepcnt(fd, value.clamp(MIN_CNT, MAX_CNT))?;
    Ok(())
}

pub fn tcp_bind(
    socket: &tokio::net::TcpSocket,
    local_address: SocketAddr,
) -> Result<(), ErrorCode> {
    // Automatically bypass the TIME_WAIT state when binding to a specific port
    // Unconditionally (re)set SO_REUSEADDR, even when the value is false.
    // This ensures we're not accidentally affected by any socket option
    // state left behind by a previous failed call to this method.
    #[cfg(not(windows))]
    if let Err(err) = sockopt::set_socket_reuseaddr(&socket, local_address.port() > 0) {
        return Err(err.into());
    }

    // Perform the OS bind call.
    socket
        .bind(local_address)
        .map_err(|err| match Errno::from_io_error(&err) {
            // From https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html:
            // > [EAFNOSUPPORT] The specified address is not a valid address for the address family of the specified socket
            //
            // The most common reasons for this error should have already
            // been handled by our own validation slightly higher up in this
            // function. This error mapping is here just in case there is
            // an edge case we didn't catch.
            Some(Errno::AFNOSUPPORT) => ErrorCode::InvalidArgument,
            // See: https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-bind#:~:text=WSAENOBUFS
            // Windows returns WSAENOBUFS when the ephemeral ports have been exhausted.
            #[cfg(windows)]
            Some(Errno::NOBUFS) => ErrorCode::AddressInUse,
            _ => err.into(),
        })
}

pub fn udp_socket(family: AddressFamily) -> std::io::Result<cap_std::net::UdpSocket> {
    // Delegate socket creation to cap_net_ext. They handle a couple of things for us:
    // - On Windows: call WSAStartup if not done before.
    // - Set the NONBLOCK and CLOEXEC flags. Either immediately during socket creation,
    //   or afterwards using ioctl or fcntl. Exact method depends on the platform.

    let socket = cap_std::net::UdpSocket::new(family, Blocking::No)?;
    Ok(socket)
}

pub fn udp_bind(sockfd: impl AsFd, addr: SocketAddr) -> Result<(), ErrorCode> {
    bind(sockfd, &addr).map_err(|err| match err {
        // See: https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-bind#:~:text=WSAENOBUFS
        // Windows returns WSAENOBUFS when the ephemeral ports have been exhausted.
        #[cfg(windows)]
        Errno::NOBUFS => ErrorCode::AddressInUse,
        // From https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html:
        // > [EAFNOSUPPORT] The specified address is not a valid address for the address family of the specified socket
        //
        // The most common reasons for this error should have already
        // been handled by our own validation slightly higher up in this
        // function. This error mapping is here just in case there is
        // an edge case we didn't catch.
        Errno::AFNOSUPPORT => ErrorCode::InvalidArgument,
        _ => err.into(),
    })
}

pub fn udp_disconnect(sockfd: impl AsFd) -> Result<(), ErrorCode> {
    match connect_unspec(sockfd) {
        // BSD platforms return an error even if the UDP socket was disconnected successfully.
        //
        // MacOS was kind enough to document this: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/connect.2.html
        // > Datagram sockets may dissolve the association by connecting to an
        // > invalid address, such as a null address or an address with the address
        // > family set to AF_UNSPEC (the error EAFNOSUPPORT will be harmlessly
        // > returned).
        //
        // ... except that this appears to be incomplete, because experiments
        // have shown that MacOS actually returns EINVAL, depending on the
        // address family of the socket.
        #[cfg(target_os = "macos")]
        Err(Errno::INVAL | Errno::AFNOSUPPORT) => Ok(()),
        Err(err) => Err(err.into()),
        Ok(()) => Ok(()),
    }
}

pub fn parse_host(name: &str) -> Result<url::Host, ErrorCode> {
    // `url::Host::parse` serves us two functions:
    // 1. validate the input is a valid domain name or IP,
    // 2. convert unicode domains to punycode.
    match url::Host::parse(&name) {
        Ok(host) => Ok(host),

        // `url::Host::parse` doesn't understand bare IPv6 addresses without [brackets]
        Err(_) => {
            if let Ok(addr) = Ipv6Addr::from_str(name) {
                Ok(url::Host::Ipv6(addr))
            } else {
                Err(ErrorCode::InvalidArgument)
            }
        }
    }
}
