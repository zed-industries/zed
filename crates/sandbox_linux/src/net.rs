//! Network access from the sandbox.
//!
//! Each execution has its own network namespace in which every address is
//! local (`local` routes for `0.0.0.0/0` and `::/0` on loopback) and none
//! leads out. A packet to any address arrives back in the namespace, where
//! the supervisor picks it up:
//!
//! - TCP: `connect` is trapped. For an allowed destination the helper
//!   listens at exactly the address the command asked for, and the
//!   supervisor connects the command's socket to that listener and relays
//!   to a host connection. Blocking sockets wait for the host first;
//!   non-blocking sockets connect locally first to avoid stalling the
//!   command's event loop, and are reset if the host connection fails.
//! - UDP: the supervisor owns a wildcard socket in the namespace for each
//!   port a rule might allow, and relays each flow (source, destination)
//!   through a host socket of its own once the policy allows it. Ports are
//!   set up when the execution starts, when the policy changes, and when a
//!   UDP socket is connected; datagrams sent with `sendto` to any other
//!   port go nowhere.
//! - DNS: wildcard sockets on port 53 answer every query ([`crate::dns`]).
//!   Names map to fake addresses, so a connection says which name it's for.
//!
//! Sockets in the namespace can only be created from inside it, so the
//! helper keeps serving [`FactoryRequest`]s on its attach socket.

use std::collections::{HashMap, HashSet};
use std::io::{self, IoSlice, IoSliceMut};
use std::net::{
    IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV6, TcpListener, TcpStream,
    ToSocketAddrs as _, UdpSocket,
};
use std::os::fd::{AsFd as _, AsRawFd as _, BorrowedFd, OwnedFd, RawFd};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use anyhow::Context as _;
use nix::sys::socket::{
    ControlMessage, ControlMessageOwned, MsgFlags, SockaddrLike as _, SockaddrStorage, recvmsg,
    sendmsg,
};
use serde::{Deserialize, Serialize};

use crate::dns;
use crate::policy::{AddressClass, NetHost, Protocol, RequestedAccess};
use crate::process::ProcessTree;
use crate::protocol::{FactoryRequest, FactoryResponse, receive_message, send_message};
use crate::supervisor::{Execution, SandboxState, Shared};
use crate::sys::{self, cvt, errno};

pub(crate) const DNS_PORT: u16 = 53;

const UPSTREAM_CONNECT_TIMEOUT: Duration = Duration::from_secs(30);
/// Connecting the command's socket to a listener in its own namespace.
const BRIDGE_TIMEOUT: Duration = Duration::from_secs(5);
const FLOW_IDLE_TIMEOUT: Duration = Duration::from_secs(60);
/// How long a denied UDP flow's datagrams are dropped before the policy is
/// checked (and maybe asked) again.
const DENIED_FLOW_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_QUEUED_DATAGRAMS: usize = 16;
const MAX_DATAGRAM_SIZE: usize = 65536;
/// `IPV6_FREEBIND` (Linux 4.15), missing from `libc`.
const IPV6_FREEBIND: i32 = 78;

pub(crate) fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn spawn_thread(name: &str, body: impl FnOnce() + Send + 'static) {
    if let Err(error) = std::thread::Builder::new()
        .name(name.to_string())
        .spawn(body)
    {
        eprintln!("zed sandbox supervisor: starting {name}: {error}");
    }
}

// Socket addresses as the kernel lays them out.

pub(crate) fn encode_address(address: SocketAddr) -> Vec<u8> {
    match address {
        SocketAddr::V4(address) => {
            let mut bytes = vec![0; size_of::<libc::sockaddr_in>()];
            bytes[0..2].copy_from_slice(&(libc::AF_INET as u16).to_ne_bytes());
            bytes[2..4].copy_from_slice(&address.port().to_be_bytes());
            bytes[4..8].copy_from_slice(&address.ip().octets());
            bytes
        }
        SocketAddr::V6(address) => {
            let mut bytes = vec![0; size_of::<libc::sockaddr_in6>()];
            bytes[0..2].copy_from_slice(&(libc::AF_INET6 as u16).to_ne_bytes());
            bytes[2..4].copy_from_slice(&address.port().to_be_bytes());
            bytes[8..24].copy_from_slice(&address.ip().octets());
            bytes[24..28].copy_from_slice(&address.scope_id().to_ne_bytes());
            bytes
        }
    }
}

pub(crate) fn decode_address(bytes: &[u8]) -> Option<SocketAddr> {
    let family = u16::from_ne_bytes(bytes.get(0..2)?.try_into().ok()?);
    let port = u16::from_be_bytes(bytes.get(2..4)?.try_into().ok()?);
    match i32::from(family) {
        libc::AF_INET => {
            let octets: [u8; 4] = bytes.get(4..8)?.try_into().ok()?;
            Some(SocketAddr::from((octets, port)))
        }
        libc::AF_INET6 => {
            let octets: [u8; 16] = bytes.get(8..24)?.try_into().ok()?;
            let scope = bytes
                .get(24..28)
                .and_then(|scope| scope.try_into().ok())
                .map_or(0, u32::from_ne_bytes);
            Some(SocketAddr::V6(SocketAddrV6::new(
                octets.into(),
                port,
                0,
                scope,
            )))
        }
        _ => None,
    }
}

fn socket_name(fd: BorrowedFd<'_>, peer: bool) -> io::Result<SocketAddr> {
    // SAFETY: plain old data, filled in by the kernel.
    let mut storage: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
    let mut length = size_of::<libc::sockaddr_storage>() as libc::socklen_t;
    let pointer = (&mut storage as *mut libc::sockaddr_storage).cast();
    // SAFETY: a sockaddr_storage-sized buffer with its length.
    cvt(unsafe {
        if peer {
            libc::getpeername(fd.as_raw_fd(), pointer, &mut length)
        } else {
            libc::getsockname(fd.as_raw_fd(), pointer, &mut length)
        }
    })?;
    // SAFETY: the kernel initialized the first `length` bytes.
    let bytes = unsafe {
        std::slice::from_raw_parts(
            (&storage as *const libc::sockaddr_storage).cast::<u8>(),
            (length as usize).min(size_of::<libc::sockaddr_storage>()),
        )
    };
    decode_address(bytes).ok_or_else(|| errno(libc::EAFNOSUPPORT))
}

fn connect_raw(fd: BorrowedFd<'_>, address: SocketAddr) -> io::Result<()> {
    let bytes = encode_address(address);
    // SAFETY: a correctly laid out sockaddr with its length.
    cvt(unsafe {
        libc::connect(
            fd.as_raw_fd(),
            bytes.as_ptr().cast(),
            bytes.len() as libc::socklen_t,
        )
    })?;
    Ok(())
}

/// `connect` that may complete asynchronously (non-blocking sockets).
fn connect_and_wait(
    fd: BorrowedFd<'_>,
    address: SocketAddr,
    timeout: Duration,
    keep_waiting: &dyn Fn() -> bool,
) -> io::Result<()> {
    match connect_raw(fd, address) {
        Ok(()) => return Ok(()),
        Err(error) if error.raw_os_error() == Some(libc::EINPROGRESS) => {}
        Err(error) => return Err(error),
    }
    sys::wait_for(fd, libc::POLLOUT, timeout, keep_waiting)?;
    match sys::socket_option(fd, libc::SOL_SOCKET, libc::SO_ERROR)? {
        0 => Ok(()),
        error => Err(errno(error)),
    }
}

fn canonical(address: SocketAddr) -> SocketAddr {
    SocketAddr::new(address.ip().to_canonical(), address.port())
}

// Creating sockets in the sandbox's namespace (the helper's side).

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub(crate) enum SocketKind {
    TcpListener,
    /// Reports each datagram's destination address.
    Udp,
    /// A `NETLINK_SOCK_DIAG` socket, for listing the namespace's sockets.
    /// Not bound: the address is ignored.
    Diagnostics,
}

/// A socket bound to `address`, which needn't belong to any interface.
pub(crate) fn create_socket(kind: SocketKind, address: SocketAddr) -> io::Result<OwnedFd> {
    if let SocketKind::Diagnostics = kind {
        return sys::socket(libc::AF_NETLINK, libc::SOCK_DGRAM, libc::NETLINK_SOCK_DIAG);
    }
    let (domain, ipv6) = match address {
        SocketAddr::V4(_) => (libc::AF_INET, false),
        SocketAddr::V6(_) => (libc::AF_INET6, true),
    };
    let socket_type = match kind {
        SocketKind::TcpListener => libc::SOCK_STREAM,
        SocketKind::Udp | SocketKind::Diagnostics => libc::SOCK_DGRAM,
    };
    let socket = sys::socket(domain, socket_type, 0)?;
    let fd = socket.as_fd();
    sys::set_socket_option(fd, libc::SOL_SOCKET, libc::SO_REUSEADDR, 1)?;
    if ipv6 {
        sys::set_socket_option(fd, libc::IPPROTO_IPV6, libc::IPV6_V6ONLY, 1)?;
        // Unlike IPv4, IPv6 only binds (and sends from) addresses an
        // interface has, local route or not.
        sys::set_socket_option(fd, libc::IPPROTO_IPV6, IPV6_FREEBIND, 1)?;
    }
    if let SocketKind::Udp = kind {
        if ipv6 {
            sys::set_socket_option(fd, libc::IPPROTO_IPV6, libc::IPV6_RECVPKTINFO, 1)?;
        } else {
            sys::set_socket_option(fd, libc::IPPROTO_IP, libc::IP_PKTINFO, 1)?;
        }
    }
    let bytes = encode_address(address);
    // SAFETY: a correctly laid out sockaddr with its length.
    cvt(unsafe {
        libc::bind(
            fd.as_raw_fd(),
            bytes.as_ptr().cast(),
            bytes.len() as libc::socklen_t,
        )
    })?;
    if let SocketKind::TcpListener = kind {
        // SAFETY: plain listen.
        cvt(unsafe { libc::listen(fd.as_raw_fd(), 128) })?;
    }
    Ok(socket)
}

/// The address [`configure_namespace`] gives loopback besides `127.0.0.1`,
/// from TEST-NET-1 (RFC 5737), which no network routes and which is outside
/// the DNS's fake addresses.
const OUTWARD_ADDRESS: Ipv4Addr = Ipv4Addr::new(192, 0, 2, 1);

/// Route every address to loopback, and give loopback an address that isn't
/// a loopback one. Called by the helper in its new network namespace. (Ping
/// would now get replies from every address, so the seccomp filter refuses
/// ping sockets.)
///
/// Programs such as Nix decide they're offline when no interface has an
/// address outside loopback, and then never connect, so they would never
/// reach a rule that asks. The extra address changes nothing else: every
/// address already routes back into the namespace.
pub(crate) fn configure_namespace() -> anyhow::Result<()> {
    add_local_route(libc::AF_INET).context("routing IPv4 addresses to loopback")?;
    if ipv6_available() {
        add_local_route(libc::AF_INET6).context("routing IPv6 addresses to loopback")?;
    }
    add_loopback_address(OUTWARD_ADDRESS)
        .with_context(|| format!("giving loopback the address {OUTWARD_ADDRESS}"))?;
    Ok(())
}

/// The DNS sockets for a new namespace: UDP and TCP on port 53 of every
/// address.
pub(crate) fn create_dns_sockets() -> anyhow::Result<Vec<OwnedFd>> {
    let mut addresses = vec![SocketAddr::from((Ipv4Addr::UNSPECIFIED, DNS_PORT))];
    if ipv6_available() {
        addresses.push(SocketAddr::from((Ipv6Addr::UNSPECIFIED, DNS_PORT)));
    }
    let mut sockets = Vec::new();
    for kind in [SocketKind::Udp, SocketKind::TcpListener] {
        for &address in &addresses {
            sockets.push(
                create_socket(kind, address)
                    .with_context(|| format!("creating the DNS socket {address}"))?,
            );
        }
    }
    Ok(sockets)
}

fn ipv6_available() -> bool {
    std::path::Path::new("/proc/net/if_inet6").exists()
}

/// The size of a netlink message header, `struct nlmsghdr`.
const NETLINK_HEADER: usize = 16;

fn loopback_index() -> io::Result<u32> {
    // SAFETY: valid NUL-terminated name.
    let loopback = unsafe { libc::if_nametoindex(c"lo".as_ptr()) };
    if loopback == 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(loopback)
}

/// Starts a netlink request of `message_type` whose body is `body_len`
/// bytes long.
fn netlink_request(message_type: u16, body_len: usize) -> Vec<u8> {
    let mut message = Vec::with_capacity(NETLINK_HEADER + body_len);
    // struct nlmsghdr
    message.extend(((NETLINK_HEADER + body_len) as u32).to_ne_bytes());
    message.extend(message_type.to_ne_bytes());
    let flags = libc::NLM_F_REQUEST | libc::NLM_F_ACK | libc::NLM_F_CREATE | libc::NLM_F_EXCL;
    message.extend((flags as u16).to_ne_bytes());
    message.extend(1u32.to_ne_bytes());
    message.extend(0u32.to_ne_bytes());
    message
}

/// `ip address add <address>/32 dev lo`.
fn add_loopback_address(address: Ipv4Addr) -> io::Result<()> {
    const ADDRESS_MESSAGE: usize = 8;
    const ATTRIBUTE: usize = 8;
    let mut message = netlink_request(libc::RTM_NEWADDR, ADDRESS_MESSAGE + 2 * ATTRIBUTE);
    // struct ifaddrmsg: one host address, scoped like a real interface's.
    message.extend([libc::AF_INET as u8, 32, 0, libc::RT_SCOPE_UNIVERSE]);
    message.extend(loopback_index()?.to_ne_bytes());
    for attribute in [libc::IFA_LOCAL, libc::IFA_ADDRESS] {
        // struct rtattr
        message.extend((ATTRIBUTE as u16).to_ne_bytes());
        message.extend(attribute.to_ne_bytes());
        message.extend(address.octets());
    }
    send_netlink_request(&message)
}

/// `ip route add local <family's whole space> dev lo table local`.
fn add_local_route(family: i32) -> io::Result<()> {
    const ROUTE: usize = 12;
    const ATTRIBUTE: usize = 8;
    let loopback = loopback_index()?;
    let mut message = netlink_request(libc::RTM_NEWROUTE, ROUTE + ATTRIBUTE);
    // struct rtmsg: no destination prefix bits, i.e. every address.
    message.extend([
        family as u8,
        0,
        0,
        0,
        libc::RT_TABLE_LOCAL,
        libc::RTPROT_BOOT,
        libc::RT_SCOPE_HOST,
        libc::RTN_LOCAL,
    ]);
    message.extend(0u32.to_ne_bytes());
    // struct rtattr: RTA_OIF
    message.extend((ATTRIBUTE as u16).to_ne_bytes());
    message.extend(libc::RTA_OIF.to_ne_bytes());
    message.extend(loopback.to_ne_bytes());
    send_netlink_request(&message)
}

/// Sends one request to the kernel's routing netlink and waits for its
/// acknowledgement.
fn send_netlink_request(message: &[u8]) -> io::Result<()> {
    let socket = sys::socket(libc::AF_NETLINK, libc::SOCK_RAW, libc::NETLINK_ROUTE)?;
    // SAFETY: sending a buffer we own; netlink addresses the kernel by
    // default.
    let sent = unsafe {
        libc::send(
            socket.as_raw_fd(),
            message.as_ptr().cast(),
            message.len(),
            0,
        )
    };
    if sent < 0 {
        return Err(io::Error::last_os_error());
    }
    let mut reply = [0u8; 1024];
    // SAFETY: receiving into a buffer we own.
    let received = unsafe {
        libc::recv(
            socket.as_raw_fd(),
            reply.as_mut_ptr().cast(),
            reply.len(),
            0,
        )
    };
    if received < 0 {
        return Err(io::Error::last_os_error());
    }
    // An NLMSG_ERROR acknowledgement: the header, then a negated errno.
    let message_type = reply
        .get(4..6)
        .and_then(|bytes| bytes.try_into().ok())
        .map(u16::from_ne_bytes);
    let error = reply
        .get(NETLINK_HEADER..NETLINK_HEADER + 4)
        .and_then(|bytes| bytes.try_into().ok())
        .map(i32::from_ne_bytes);
    match (message_type, error) {
        (Some(message_type), Some(0)) if i32::from(message_type) == libc::NLMSG_ERROR => Ok(()),
        (Some(message_type), Some(error)) if i32::from(message_type) == libc::NLMSG_ERROR => {
            Err(errno(-error))
        }
        _ => Err(errno(libc::EPROTO)),
    }
}

/// Serve the supervisor's [`FactoryRequest`]s until it hangs up (the
/// helper's side).
pub(crate) fn serve_factory(socket: OwnedFd) {
    loop {
        let request = match receive_message::<FactoryRequest>(socket.as_fd()) {
            Ok(Some((request, _))) => request,
            Ok(None) => return,
            Err(error) => {
                eprintln!("zed sandbox: {error:#}");
                return;
            }
        };
        let (response, fd) = match request {
            FactoryRequest::CreateSocket { kind, address } => match create_socket(kind, address) {
                Ok(created) => (FactoryResponse::Created, Some(created)),
                Err(error) => (
                    FactoryResponse::Failed {
                        errno: error.raw_os_error().unwrap_or(libc::EIO),
                    },
                    None,
                ),
            },
            FactoryRequest::FetchFd { pid, fd } => match sys::pidfd_getfd(pid, fd) {
                Ok(fetched) => (FactoryResponse::Fetched, Some(fetched)),
                Err(error) => (
                    FactoryResponse::Failed {
                        errno: error.raw_os_error().unwrap_or(libc::EIO),
                    },
                    None,
                ),
            },
        };
        let sent = match fd {
            Some(fd) => send_message(socket.as_fd(), &response, &[fd.as_raw_fd()]),
            None => send_message(socket.as_fd(), &response, &[]),
        };
        if let Err(error) = sent {
            eprintln!("zed sandbox: {error:#}");
            return;
        }
    }
}

// The supervisor's side.

/// One execution's network: its namespace's factory, and the relays.
pub(crate) struct Network {
    factory: Mutex<OwnedFd>,
    /// Held while connecting a command's socket to a temporary listener (and
    /// while checking for the command's own listeners), so no two bridges
    /// see each other's listeners.
    bridge: Mutex<()>,
    /// Sockets threads block on, shut down when the execution ends.
    sockets: Mutex<Vec<Arc<OwnedFd>>>,
    udp_ports: Mutex<HashSet<u16>>,
    flows: Mutex<HashMap<(SocketAddr, SocketAddr), Arc<Flow>>>,
    /// A socket-diagnostics socket in the namespace, once created.
    diagnostics: Mutex<Option<OwnedFd>>,
    closed: AtomicBool,
}

/// A socket in the sandbox's namespace, as socket diagnostics report it.
struct DiagnosedSocket {
    local: SocketAddr,
    inode: u64,
}

impl Network {
    pub fn new(factory: OwnedFd) -> Self {
        Self {
            factory: Mutex::new(factory),
            diagnostics: Mutex::new(None),
            bridge: Mutex::new(()),
            sockets: Mutex::default(),
            udp_ports: Mutex::default(),
            flows: Mutex::default(),
            closed: AtomicBool::new(false),
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    /// The execution has ended: stop serving its namespace. (Its sockets
    /// would otherwise keep the namespace, and their threads, alive.)
    pub fn close(&self) {
        self.closed.store(true, Ordering::SeqCst);
        for socket in lock(&self.sockets).drain(..) {
            // SAFETY: shutdown on a socket we own; wakes its readers.
            unsafe { libc::shutdown(socket.as_raw_fd(), libc::SHUT_RDWR) };
        }
    }

    fn watch(&self, socket: OwnedFd) -> Arc<OwnedFd> {
        let socket = Arc::new(socket);
        let mut sockets = lock(&self.sockets);
        if self.is_closed() {
            // SAFETY: as in `close`.
            unsafe { libc::shutdown(socket.as_raw_fd(), libc::SHUT_RDWR) };
        } else {
            sockets.push(socket.clone());
        }
        socket
    }

    /// The namespace's IPv4 and IPv6 sockets of `protocol` in any of
    /// `states` (a bit per TCP state), from `NETLINK_SOCK_DIAG`. Unlike
    /// `/proc/net/tcp`, which walks the whole host's connection table, a
    /// listening-state query only walks listeners.
    fn diagnose(&self, protocol: u8, states: u32) -> io::Result<Vec<DiagnosedSocket>> {
        const SOCK_DIAG_BY_FAMILY: u16 = 20;
        const HEADER: usize = 16;
        let mut diagnostics = lock(&self.diagnostics);
        if diagnostics.is_none() {
            *diagnostics = Some(self.create(
                SocketKind::Diagnostics,
                SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0)),
            )?);
        }
        let Some(socket) = diagnostics.as_ref() else {
            return Err(errno(libc::ENETDOWN));
        };
        let mut sockets = Vec::new();
        for family in [libc::AF_INET as u8, libc::AF_INET6 as u8] {
            // struct nlmsghdr, then struct inet_diag_req_v2 with a zeroed
            // inet_diag_sockid: dump everything in `states`.
            let mut request = Vec::with_capacity(HEADER + 56);
            request.extend(((HEADER + 56) as u32).to_ne_bytes());
            request.extend(SOCK_DIAG_BY_FAMILY.to_ne_bytes());
            request.extend(((libc::NLM_F_REQUEST | libc::NLM_F_DUMP) as u16).to_ne_bytes());
            request.extend(1u32.to_ne_bytes());
            request.extend(0u32.to_ne_bytes());
            request.extend([family, protocol, 0, 0]);
            request.extend(states.to_ne_bytes());
            request.extend([0u8; 48]);
            // SAFETY: sending a buffer we own.
            let sent = unsafe {
                libc::send(
                    socket.as_raw_fd(),
                    request.as_ptr().cast(),
                    request.len(),
                    0,
                )
            };
            if sent < 0 {
                return Err(io::Error::last_os_error());
            }
            let mut buffer = vec![0u8; 32 * 1024];
            'replies: loop {
                // SAFETY: receiving into a buffer we own.
                let received = unsafe {
                    libc::recv(
                        socket.as_raw_fd(),
                        buffer.as_mut_ptr().cast(),
                        buffer.len(),
                        0,
                    )
                };
                if received < 0 {
                    return Err(io::Error::last_os_error());
                }
                let mut messages = &buffer[..received as usize];
                while messages.len() >= HEADER {
                    let length =
                        u32::from_ne_bytes([messages[0], messages[1], messages[2], messages[3]])
                            as usize;
                    let message_type = u16::from_ne_bytes([messages[4], messages[5]]);
                    if length < HEADER || length > messages.len() {
                        return Err(errno(libc::EPROTO));
                    }
                    match i32::from(message_type) {
                        libc::NLMSG_DONE => break 'replies,
                        libc::NLMSG_ERROR => {
                            let code = messages
                                .get(HEADER..HEADER + 4)
                                .map_or(-libc::EPROTO, |code| {
                                    i32::from_ne_bytes([code[0], code[1], code[2], code[3]])
                                });
                            return Err(errno(-code));
                        }
                        _ => {
                            if let Some(socket) = parse_diagnosed(&messages[HEADER..length]) {
                                sockets.push(socket);
                            }
                        }
                    }
                    // Messages are 4-byte aligned.
                    messages = &messages[((length + 3) & !3).min(messages.len())..];
                }
            }
        }
        Ok(sockets)
    }

    fn create(&self, kind: SocketKind, address: SocketAddr) -> io::Result<OwnedFd> {
        let factory = lock(&self.factory);
        send_message(
            factory.as_fd(),
            &FactoryRequest::CreateSocket { kind, address },
            &[],
        )
        .map_err(|_| errno(libc::ENETDOWN))?;
        match receive_message::<FactoryResponse>(factory.as_fd()) {
            Ok(Some((FactoryResponse::Created, mut fds))) if fds.len() == 1 => Ok(fds.remove(0)),
            Ok(Some((FactoryResponse::Failed { errno: code }, _))) => Err(errno(code)),
            _ => Err(errno(libc::ENETDOWN)),
        }
    }

    /// A duplicate of process `pid`'s descriptor `fd`, fetched by the helper:
    /// the supervisor may not fetch it itself (see [`FactoryRequest::FetchFd`]).
    pub(crate) fn fetch_fd(&self, pid: i32, fd: RawFd) -> io::Result<OwnedFd> {
        let factory = lock(&self.factory);
        send_message(factory.as_fd(), &FactoryRequest::FetchFd { pid, fd }, &[])
            .map_err(|_| errno(libc::ENETDOWN))?;
        match receive_message::<FactoryResponse>(factory.as_fd()) {
            Ok(Some((FactoryResponse::Fetched, mut fds))) if fds.len() == 1 => Ok(fds.remove(0)),
            Ok(Some((FactoryResponse::Failed { errno: code }, _))) => Err(errno(code)),
            _ => Err(errno(libc::ENETDOWN)),
        }
    }

    /// Connect the command's `socket` to a temporary listener at
    /// `destination` (where it asked to connect) and return the accepted
    /// end.
    fn bridge(&self, socket: BorrowedFd<'_>, destination: SocketAddr) -> io::Result<TcpStream> {
        let target = canonical(destination);
        let _bridge = lock(&self.bridge);
        let listener = match self.create(SocketKind::TcpListener, target) {
            // Something in the sandbox holds that exact address; the command
            // will see another port as its peer's.
            Err(error) if error.raw_os_error() == Some(libc::EADDRINUSE) => {
                self.create(SocketKind::TcpListener, SocketAddr::new(target.ip(), 0))?
            }
            listener => listener?,
        };
        let port = socket_name(listener.as_fd(), false)?.port();
        // In the command's own family (an IPv6 socket may have asked for an
        // IPv4-mapped address).
        let listener_address = SocketAddr::new(destination.ip(), port);
        connect_and_wait(socket, listener_address, BRIDGE_TIMEOUT, &|| true)?;
        let command_address = canonical(socket_name(socket, false)?);

        let listener = TcpListener::from(listener);
        listener.set_nonblocking(true)?;
        let deadline = Instant::now() + BRIDGE_TIMEOUT;
        loop {
            match listener.accept() {
                Ok((stream, peer)) if canonical(peer) == command_address => {
                    stream.set_nonblocking(false)?;
                    return Ok(stream);
                }
                // Something else raced to the listener (e.g. a TCP Fast Open
                // `sendto`, which isn't trapped). Turn it away.
                Ok(_) => {}
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                    if Instant::now() >= deadline {
                        return Err(errno(libc::ETIMEDOUT));
                    }
                    match sys::wait_for(listener.as_fd(), libc::POLLIN, BRIDGE_TIMEOUT, &|| true) {
                        Ok(()) => {}
                        Err(error) if error.raw_os_error() == Some(libc::ETIMEDOUT) => {}
                        Err(error) => return Err(error),
                    }
                }
                Err(error) => return Err(error),
            }
        }
    }
}

/// Start serving a new execution's namespace: answer DNS on `dns_sockets`
/// and relay the UDP ports its policy names.
pub(crate) fn start(shared: &Arc<Shared>, execution: &Arc<Execution>, dns_sockets: Vec<OwnedFd>) {
    for socket in dns_sockets {
        let tcp = sys::socket_option(socket.as_fd(), libc::SOL_SOCKET, libc::SO_TYPE)
            .is_ok_and(|kind| kind == libc::SOCK_STREAM);
        let socket = execution.network.watch(socket);
        let execution = execution.clone();
        spawn_thread("zed-sandbox-dns", move || {
            if tcp {
                dns::serve_tcp(&execution, &socket);
            } else {
                dns::serve_udp(&execution, &socket);
            }
        });
    }
    ensure_udp_ports(shared, execution);
}

/// Relay every UDP port the policy names explicitly.
pub(crate) fn ensure_udp_ports(shared: &Arc<Shared>, execution: &Arc<Execution>) {
    for port in execution.sandbox.policy().udp_ports() {
        ensure_udp_port(shared, execution, port);
    }
}

fn ensure_udp_port(shared: &Arc<Shared>, execution: &Arc<Execution>, port: u16) {
    let network = &execution.network;
    if port == DNS_PORT || !lock(&network.udp_ports).insert(port) {
        return;
    }
    let mut addresses = vec![SocketAddr::from((Ipv4Addr::UNSPECIFIED, port))];
    if ipv6_available() {
        addresses.push(SocketAddr::from((Ipv6Addr::UNSPECIFIED, port)));
    }
    for address in addresses {
        match network.create(SocketKind::Udp, address) {
            Ok(socket) => {
                let socket = network.watch(socket);
                let shared = shared.clone();
                let execution = execution.clone();
                spawn_thread("zed-sandbox-udp", move || {
                    serve_udp_port(&shared, &execution, &socket, port)
                });
            }
            Err(error) => {
                eprintln!("zed sandbox supervisor: relaying UDP on {address}: {error}");
            }
        }
    }
}

/// The host a connection to `address` in the sandbox is for. `None` for a
/// fake address no name was given.
fn destination_host(sandbox: &SandboxState, address: IpAddr) -> Option<NetHost> {
    match address {
        IpAddr::V4(address) if dns::is_fake(address) => {
            lock(&sandbox.names).name_for(address).map(NetHost::Domain)
        }
        address => Some(NetHost::Ip(address)),
    }
}

/// The thread a decision is made for, and whether it still waits for one.
pub(crate) struct Caller<'a> {
    pub tid: i32,
    pub still_valid: &'a dyn Fn() -> bool,
}

pub(crate) enum TcpOutcome {
    /// Let the kernel connect: the destination is in the sandbox.
    Continue,
    /// Connected and relayed.
    Relayed,
}

/// A trapped `connect` of the TCP socket `socket` to `destination`.
pub(crate) fn connect_tcp(
    shared: &Arc<Shared>,
    execution: &Arc<Execution>,
    socket: BorrowedFd<'_>,
    destination: SocketAddr,
    caller: &Caller<'_>,
) -> io::Result<TcpOutcome> {
    let target = canonical(destination);
    // Served by the sandbox's own DNS listener.
    if target.port() == DNS_PORT {
        return Ok(TcpOutcome::Continue);
    }
    if socket_name(socket, true).is_ok() {
        return Err(errno(libc::EISCONN));
    }
    let network = &execution.network;
    if AddressClass::of(target.ip()) == AddressClass::Loopback {
        let _bridge = lock(&network.bridge);
        if listening_in_sandbox(network, caller.tid, target) {
            return Ok(TcpOutcome::Continue);
        }
    }
    let sandbox = &execution.sandbox;
    let host = destination_host(sandbox, target.ip()).ok_or_else(|| errno(libc::ECONNREFUSED))?;
    let request = RequestedAccess::Net {
        host: host.clone(),
        port: target.port(),
        protocol: Protocol::Tcp,
    };
    if !shared.authorize(sandbox, &request, caller.tid, caller.still_valid) {
        return Err(errno(libc::EACCES));
    }

    if is_nonblocking(socket)? {
        // Holding the syscall until the host answers would freeze the
        // caller's event loop for a round trip per connection. Connect it
        // now and the host in the background instead; if that fails, its
        // connection is reset (rather than `connect` failing).
        let inside = network.bridge(socket, destination)?;
        let shared = shared.clone();
        let execution = execution.clone();
        let tid = caller.tid;
        std::thread::Builder::new()
            .name("zed-sandbox-connect".to_string())
            .spawn(move || {
                let network = &execution.network;
                let still_valid = || !network.is_closed();
                let caller = Caller {
                    tid,
                    still_valid: &still_valid,
                };
                match connect_host(&shared, &execution.sandbox, &host, target.port(), &caller) {
                    Ok(upstream) if still_valid() => relay(inside, upstream),
                    _ => reset(&inside),
                }
            })?;
        return Ok(TcpOutcome::Relayed);
    }

    let upstream = connect_host(shared, sandbox, &host, target.port(), caller)?;
    let inside = network.bridge(socket, destination)?;
    relay(inside, upstream);
    Ok(TcpOutcome::Relayed)
}

fn is_nonblocking(socket: BorrowedFd<'_>) -> io::Result<bool> {
    // SAFETY: F_GETFL on a valid descriptor. Our duplicate shares the
    // target's open file description, and so its flags.
    let flags = cvt(unsafe { libc::fcntl(socket.as_raw_fd(), libc::F_GETFL) })?;
    Ok(flags & libc::O_NONBLOCK != 0)
}

/// Connect on the host to the first of `host`'s addresses that may be
/// reached and answers.
fn connect_host(
    shared: &Shared,
    sandbox: &SandboxState,
    host: &NetHost,
    port: u16,
    caller: &Caller<'_>,
) -> io::Result<TcpStream> {
    let mut last_error = errno(libc::EHOSTUNREACH);
    for address in upstream_addresses(host, port)? {
        if !(caller.still_valid)() {
            return Err(errno(libc::ECONNABORTED));
        }
        if !may_reach(shared, sandbox, host, address, Protocol::Tcp, caller) {
            last_error = errno(libc::EACCES);
            continue;
        }
        if !(caller.still_valid)() {
            return Err(errno(libc::ECONNABORTED));
        }
        match connect_upstream(address, caller.still_valid) {
            Ok(stream) => return Ok(stream),
            Err(error) => last_error = error,
        }
        if !(caller.still_valid)() {
            break;
        }
    }
    Err(last_error)
}

/// Close `stream` with a reset, as a refused connection would look once
/// established.
fn reset(stream: &TcpStream) {
    let linger = libc::linger {
        l_onoff: 1,
        l_linger: 0,
    };
    // SAFETY: a `linger` with its length. Closing with a zero linger time
    // sends a reset.
    let result = unsafe {
        libc::setsockopt(
            stream.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_LINGER,
            (&linger as *const libc::linger).cast(),
            size_of::<libc::linger>() as libc::socklen_t,
        )
    };
    if result < 0 {
        // A plain close still ends the connection.
        stream.shutdown(std::net::Shutdown::Both).ok();
    }
}

/// A trapped `connect` of a UDP socket: the kernel does it, but datagrams
/// to that port need a relay socket first.
pub(crate) fn connect_udp(
    shared: &Arc<Shared>,
    execution: &Arc<Execution>,
    destination: SocketAddr,
) {
    let port = destination.port();
    if execution.sandbox.policy().udp_may_allow(port) {
        ensure_udp_port(shared, execution, port);
    }
}

fn upstream_addresses(host: &NetHost, port: u16) -> io::Result<Vec<SocketAddr>> {
    match host {
        NetHost::Ip(address) => Ok(vec![SocketAddr::new(*address, port)]),
        NetHost::Domain(name) => match (name.as_str(), port).to_socket_addrs() {
            Ok(addresses) => Ok(addresses.collect()),
            Err(_) => Err(errno(libc::EHOSTUNREACH)),
        },
    }
}

/// A name that resolves to a loopback or local-network address (e.g. by DNS
/// rebinding) only reaches it if the address is allowed too.
fn may_reach(
    shared: &Shared,
    sandbox: &SandboxState,
    host: &NetHost,
    address: SocketAddr,
    protocol: Protocol,
    caller: &Caller<'_>,
) -> bool {
    if matches!(host, NetHost::Ip(_)) || AddressClass::of(address.ip()) == AddressClass::Public {
        return true;
    }
    let request = RequestedAccess::Net {
        host: NetHost::Ip(address.ip().to_canonical()),
        port: address.port(),
        protocol,
    };
    shared.authorize(sandbox, &request, caller.tid, caller.still_valid)
}

fn connect_upstream(address: SocketAddr, still_valid: &dyn Fn() -> bool) -> io::Result<TcpStream> {
    let domain = if address.is_ipv4() {
        libc::AF_INET
    } else {
        libc::AF_INET6
    };
    let socket = sys::socket(domain, libc::SOCK_STREAM | libc::SOCK_NONBLOCK, 0)?;
    connect_and_wait(
        socket.as_fd(),
        address,
        UPSTREAM_CONNECT_TIMEOUT,
        still_valid,
    )?;
    let stream = TcpStream::from(socket);
    stream.set_nonblocking(false)?;
    Ok(stream)
}

/// `struct inet_diag_msg`: family, state, timer, retransmits, then
/// `inet_diag_sockid` (source port and address, in network order), ...,
/// and the inode at offset 68.
fn parse_diagnosed(message: &[u8]) -> Option<DiagnosedSocket> {
    let family = i32::from(*message.first()?);
    let port = u16::from_be_bytes(message.get(4..6)?.try_into().ok()?);
    let ip = match family {
        libc::AF_INET => IpAddr::from(<[u8; 4]>::try_from(message.get(8..12)?).ok()?),
        libc::AF_INET6 => IpAddr::from(<[u8; 16]>::try_from(message.get(8..24)?).ok()?),
        _ => return None,
    };
    let inode = u32::from_ne_bytes(message.get(68..72)?.try_into().ok()?);
    Some(DiagnosedSocket {
        local: SocketAddr::new(ip.to_canonical(), port),
        inode: u64::from(inode),
    })
}

/// Whether something in the sandbox listens where `target` (a loopback
/// address) points, so connecting there stays inside.
fn listening_in_sandbox(network: &Network, tid: i32, target: SocketAddr) -> bool {
    const TCP_LISTEN: u32 = 10;
    let listens_at = |local: SocketAddr| {
        local.port() == target.port()
            && (local.ip().is_unspecified()
                || target.ip().is_unspecified()
                || local.ip() == target.ip())
    };
    match network.diagnose(libc::IPPROTO_TCP as u8, 1 << TCP_LISTEN) {
        Ok(listeners) => listeners.iter().any(|socket| listens_at(socket.local)),
        // Socket diagnostics unavailable (no `inet_diag`): read `/proc`.
        Err(_) => {
            proc_sockets(tid, "tcp").any(|socket| socket.state == "0A" && listens_at(socket.local))
        }
    }
}

struct ProcSocket {
    local: SocketAddr,
    state: String,
}

/// The sockets in `pid`'s network namespace from `/proc/<pid>/net/<table>`
/// and its IPv6 counterpart, with canonical addresses.
fn proc_sockets(pid: i32, table: &str) -> impl Iterator<Item = ProcSocket> {
    let contents: Vec<String> = [table.to_string(), format!("{table}6")]
        .iter()
        .filter_map(|table| std::fs::read_to_string(format!("/proc/{pid}/net/{table}")).ok())
        .collect();
    contents.into_iter().flat_map(|contents| {
        contents
            .lines()
            .skip(1)
            .filter_map(parse_proc_socket)
            .collect::<Vec<_>>()
    })
}

/// One line of `/proc/net/{tcp,udp}{,6}`: addresses are hex words in host
/// byte order.
fn parse_proc_socket(line: &str) -> Option<ProcSocket> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    let (address, port) = fields.get(1)?.split_once(':')?;
    let port = u16::from_str_radix(port, 16).ok()?;
    let mut octets = Vec::with_capacity(16);
    for word in address.as_bytes().chunks(8) {
        let word = u32::from_str_radix(std::str::from_utf8(word).ok()?, 16).ok()?;
        octets.extend(word.to_ne_bytes());
    }
    let ip = match octets.len() {
        4 => IpAddr::from(<[u8; 4]>::try_from(octets).ok()?),
        16 => IpAddr::from(<[u8; 16]>::try_from(octets).ok()?),
        _ => return None,
    };
    Some(ProcSocket {
        local: SocketAddr::new(ip.to_canonical(), port),
        state: fields.get(3)?.to_string(),
    })
}

/// Copy both ways between the sandbox's end of a connection and the host's
/// until both sides have finished.
fn relay(inside: TcpStream, outside: TcpStream) {
    let inside = Arc::new(inside);
    let outside = Arc::new(outside);
    let pump = |from: Arc<TcpStream>, to: Arc<TcpStream>| {
        move || match splice_all(&from, &to) {
            Ok(()) => {
                // Pass the end of the stream on; the other direction may
                // still be going.
                to.shutdown(std::net::Shutdown::Write).ok();
            }
            Err(_) => {
                // A reset (or worse): end both directions.
                from.shutdown(std::net::Shutdown::Both).ok();
                to.shutdown(std::net::Shutdown::Both).ok();
            }
        }
    };
    spawn_thread("zed-sandbox-relay", pump(inside.clone(), outside.clone()));
    spawn_thread("zed-sandbox-relay", pump(outside, inside));
}

/// Move bytes from `from` to `to` through a pipe, without copying them
/// through userspace, until `from` ends.
fn splice_all(from: &TcpStream, to: &TcpStream) -> io::Result<()> {
    let mut pipe = [0; 2];
    // SAFETY: pipe2 fills in two descriptors.
    cvt(unsafe { libc::pipe2(pipe.as_mut_ptr(), libc::O_CLOEXEC) })?;
    // SAFETY: fresh descriptors that nothing else owns.
    let (pipe_read, pipe_write) = unsafe {
        use std::os::fd::FromRawFd as _;
        (OwnedFd::from_raw_fd(pipe[0]), OwnedFd::from_raw_fd(pipe[1]))
    };
    let splice = |from: i32, to: i32, length: usize| -> io::Result<usize> {
        loop {
            // SAFETY: splice between descriptors we hold.
            let moved = unsafe {
                libc::splice(
                    from,
                    std::ptr::null_mut(),
                    to,
                    std::ptr::null_mut(),
                    length,
                    libc::SPLICE_F_MOVE,
                )
            };
            if moved >= 0 {
                return Ok(moved as usize);
            }
            let error = io::Error::last_os_error();
            if error.raw_os_error() != Some(libc::EINTR) {
                return Err(error);
            }
        }
    };
    loop {
        let read = splice(from.as_raw_fd(), pipe_write.as_raw_fd(), 1 << 16)?;
        if read == 0 {
            return Ok(());
        }
        let mut left = read;
        while left > 0 {
            let written = splice(pipe_read.as_raw_fd(), to.as_raw_fd(), left)?;
            if written == 0 {
                return Err(errno(libc::EPIPE));
            }
            left -= written;
        }
    }
}

// UDP.

pub(crate) struct Datagram {
    pub length: usize,
    pub source: SocketAddr,
    /// The address it was sent to (any address reaches a wildcard socket).
    pub destination: IpAddr,
}

/// Receive into `buffer` from a [`SocketKind::Udp`] socket. `None` for a
/// datagram without addresses (as after shutting the socket down).
pub(crate) fn receive_datagram(
    socket: BorrowedFd<'_>,
    buffer: &mut [u8],
) -> io::Result<Option<Datagram>> {
    let mut control = nix::cmsg_space!(libc::in6_pktinfo);
    let mut iov = [IoSliceMut::new(buffer)];
    let message = recvmsg::<SockaddrStorage>(
        socket.as_raw_fd(),
        &mut iov,
        Some(&mut control),
        MsgFlags::MSG_CMSG_CLOEXEC,
    )?;
    let mut destination = None;
    for control_message in message.cmsgs()? {
        match control_message {
            ControlMessageOwned::Ipv4PacketInfo(info) => {
                // `s_addr` holds the address's bytes in network order.
                destination = Some(IpAddr::from(info.ipi_addr.s_addr.to_ne_bytes()));
            }
            ControlMessageOwned::Ipv6PacketInfo(info) => {
                destination = Some(IpAddr::from(info.ipi6_addr.s6_addr));
            }
            _ => {}
        }
    }
    let source = message.address.as_ref().and_then(storage_address);
    Ok(match (source, destination) {
        (Some(source), Some(destination)) => Some(Datagram {
            length: message.bytes,
            source,
            destination,
        }),
        _ => None,
    })
}

fn storage_address(storage: &SockaddrStorage) -> Option<SocketAddr> {
    // SAFETY: `as_ptr` and `len` describe the storage's initialized sockaddr.
    let bytes = unsafe {
        std::slice::from_raw_parts(storage.as_ptr().cast::<u8>(), storage.len() as usize)
    };
    decode_address(bytes)
}

/// Send `payload` to `to` from address `from` (which no interface needs to
/// have: every address is local in the sandbox).
pub(crate) fn send_datagram(
    socket: BorrowedFd<'_>,
    payload: &[u8],
    to: SocketAddr,
    from: IpAddr,
) -> io::Result<()> {
    let iov = [IoSlice::new(payload)];
    let to = SockaddrStorage::from(to);
    let flags = MsgFlags::MSG_NOSIGNAL;
    match from {
        IpAddr::V4(from) => {
            let info = libc::in_pktinfo {
                ipi_ifindex: 0,
                ipi_spec_dst: libc::in_addr {
                    s_addr: u32::from_ne_bytes(from.octets()),
                },
                ipi_addr: libc::in_addr { s_addr: 0 },
            };
            let control = [ControlMessage::Ipv4PacketInfo(&info)];
            sendmsg(socket.as_raw_fd(), &iov, &control, flags, Some(&to))?;
        }
        IpAddr::V6(from) => {
            let info = libc::in6_pktinfo {
                ipi6_addr: libc::in6_addr {
                    s6_addr: from.octets(),
                },
                ipi6_ifindex: 0,
            };
            let control = [ControlMessage::Ipv6PacketInfo(&info)];
            sendmsg(socket.as_raw_fd(), &iov, &control, flags, Some(&to))?;
        }
    }
    Ok(())
}

/// A UDP flow from a socket in the sandbox to one destination.
struct Flow {
    state: Mutex<FlowState>,
    last_active: Mutex<Instant>,
}

enum FlowState {
    /// Being decided; datagrams wait.
    Pending(Vec<Vec<u8>>),
    Open(Arc<UdpSocket>),
    Denied,
}

impl Flow {
    fn touch(&self) {
        *lock(&self.last_active) = Instant::now();
    }

    fn idle(&self) -> Duration {
        lock(&self.last_active).elapsed()
    }
}

fn serve_udp_port(
    shared: &Arc<Shared>,
    execution: &Arc<Execution>,
    socket: &Arc<OwnedFd>,
    port: u16,
) {
    let network = &execution.network;
    let mut buffer = vec![0; MAX_DATAGRAM_SIZE];
    while !network.is_closed() {
        let datagram = match receive_datagram(socket.as_fd(), &mut buffer) {
            Ok(Some(datagram)) => datagram,
            Ok(None) => continue,
            // A previous reply bounced (ICMP unreachable).
            Err(error)
                if matches!(
                    error.raw_os_error(),
                    Some(libc::EINTR | libc::ECONNREFUSED | libc::EHOSTUNREACH)
                ) =>
            {
                continue;
            }
            Err(error) => {
                eprintln!("zed sandbox supervisor: receiving UDP on port {port}: {error}");
                return;
            }
        };
        let destination = SocketAddr::new(datagram.destination, port);
        let payload = &buffer[..datagram.length];
        relay_datagram(
            shared,
            execution,
            socket,
            datagram.source,
            destination,
            payload,
        );
    }
}

fn relay_datagram(
    shared: &Arc<Shared>,
    execution: &Arc<Execution>,
    socket: &Arc<OwnedFd>,
    source: SocketAddr,
    destination: SocketAddr,
    payload: &[u8],
) {
    let key = (source, destination);
    let flow = {
        let mut flows = lock(&execution.network.flows);
        match flows.get(&key) {
            Some(flow) => flow.clone(),
            None => {
                let flow = Arc::new(Flow {
                    state: Mutex::new(FlowState::Pending(Vec::new())),
                    last_active: Mutex::new(Instant::now()),
                });
                flows.insert(key, flow.clone());
                let shared = shared.clone();
                let execution = execution.clone();
                let socket = socket.clone();
                let decided = flow.clone();
                spawn_thread("zed-sandbox-udp-flow", move || {
                    run_flow(&shared, &execution, &socket, key, &decided)
                });
                flow
            }
        }
    };
    flow.touch();
    match &mut *lock(&flow.state) {
        FlowState::Pending(queue) => {
            if queue.len() < MAX_QUEUED_DATAGRAMS {
                queue.push(payload.to_vec());
            }
        }
        FlowState::Open(upstream) => {
            // UDP is lossy: a failed send is a lost datagram.
            upstream.send(payload).ok();
        }
        FlowState::Denied => {}
    }
}

/// Decide a new flow, then relay its replies until it goes idle.
fn run_flow(
    shared: &Shared,
    execution: &Execution,
    socket: &OwnedFd,
    key: (SocketAddr, SocketAddr),
    flow: &Arc<Flow>,
) {
    let network = &execution.network;
    let (source, destination) = key;
    match open_flow(shared, execution, source, destination) {
        Ok(upstream) => {
            let upstream = Arc::new(upstream);
            let queued =
                match std::mem::replace(&mut *lock(&flow.state), FlowState::Open(upstream.clone()))
                {
                    FlowState::Pending(queued) => queued,
                    _ => Vec::new(),
                };
            for payload in queued {
                upstream.send(&payload).ok();
            }
            let mut buffer = vec![0; MAX_DATAGRAM_SIZE];
            while !network.is_closed() && flow.idle() < FLOW_IDLE_TIMEOUT {
                match upstream.recv(&mut buffer) {
                    Ok(length) => {
                        flow.touch();
                        // Replies come from where the command sent to.
                        send_datagram(socket.as_fd(), &buffer[..length], source, destination.ip())
                            .ok();
                    }
                    // Timeouts, and ICMP errors from the far end.
                    Err(_) => {}
                }
            }
        }
        Err(_) => {
            *lock(&flow.state) = FlowState::Denied;
            let denied = Instant::now();
            while !network.is_closed() && denied.elapsed() < DENIED_FLOW_TIMEOUT {
                std::thread::sleep(Duration::from_millis(250));
            }
        }
    }
    let mut flows = lock(&network.flows);
    if flows
        .get(&key)
        .is_some_and(|current| Arc::ptr_eq(current, flow))
    {
        flows.remove(&key);
    }
}

fn open_flow(
    shared: &Shared,
    execution: &Execution,
    source: SocketAddr,
    destination: SocketAddr,
) -> io::Result<UdpSocket> {
    let network = &execution.network;
    let sandbox = &execution.sandbox;
    let target = canonical(destination);
    let host = destination_host(sandbox, target.ip()).ok_or_else(|| errno(libc::ECONNREFUSED))?;
    let still_valid = || !network.is_closed();
    let caller = Caller {
        tid: udp_sender(network, &execution.tree, source),
        still_valid: &still_valid,
    };
    let request = RequestedAccess::Net {
        host: host.clone(),
        port: target.port(),
        protocol: Protocol::Udp,
    };
    if !shared.authorize(sandbox, &request, caller.tid, caller.still_valid) {
        return Err(errno(libc::EACCES));
    }
    let address = upstream_addresses(&host, target.port())?
        .into_iter()
        .find(|address| may_reach(shared, sandbox, &host, *address, Protocol::Udp, &caller))
        .ok_or_else(|| errno(libc::EACCES))?;
    let local: SocketAddr = if address.is_ipv4() {
        (Ipv4Addr::UNSPECIFIED, 0).into()
    } else {
        (Ipv6Addr::UNSPECIFIED, 0).into()
    };
    let upstream = UdpSocket::bind(local)?;
    upstream.connect(address)?;
    upstream.set_read_timeout(Some(Duration::from_secs(1)))?;
    Ok(upstream)
}

/// The process that owns the sandbox socket bound to `source`, for prompts
/// and denial reports; the helper if it can't be found.
fn udp_sender(network: &Network, tree: &ProcessTree, source: SocketAddr) -> i32 {
    let source = canonical(source);
    let inode = network
        .diagnose(libc::IPPROTO_UDP as u8, u32::MAX)
        .ok()
        .and_then(|sockets| {
            sockets
                .into_iter()
                .find(|socket| {
                    socket.local == source
                        || (socket.local.ip().is_unspecified()
                            && socket.local.port() == source.port())
                })
                .map(|socket| socket.inode)
        });
    let holder = inode.and_then(|inode| {
        let link = format!("socket:[{inode}]");
        tree.members().into_iter().find_map(|process| {
            let entries = std::fs::read_dir(format!("/proc/{}/fd", process.pid)).ok()?;
            entries
                .flatten()
                .any(|entry| {
                    std::fs::read_link(entry.path())
                        .is_ok_and(|target| target.as_os_str() == link.as_str())
                })
                .then_some(process.pid)
        })
    });
    holder.unwrap_or(tree.helper)
}
