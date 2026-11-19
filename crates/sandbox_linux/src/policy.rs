//! Policy types.
//!
//! [`SandboxPolicy`] is what callers write: rules can carry [`Handler`]s,
//! which are closures in the caller's process. [`WirePolicy`] is what the
//! supervisor enforces: the same rules with each handler replaced by an id
//! the caller keeps a table of.

use std::collections::BTreeSet;
use std::fmt;
use std::net::IpAddr;
use std::ops::RangeInclusive;
use std::os::unix::ffi::OsStrExt as _;
use std::path::{Component, Components, Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

use crate::Responder;

/// What a sandbox may do.
///
/// Each resource has its own ordered rule list. Within a list, for each
/// permission, the last rule that matches and sets that permission decides;
/// a permission no matching rule sets is denied. Order between lists means
/// nothing.
#[derive(Clone, Debug, Default)]
pub struct SandboxPolicy {
    /// Filesystem access, by host path.
    ///
    /// - Read means opening a file for its contents. Listing directories,
    ///   `stat`, `readlink` and `execve` are never restricted.
    /// - Write means any modification: creating, deleting, renaming,
    ///   linking, opening for writing, `chmod`, `truncate`, timestamps,
    ///   binding a unix socket.
    /// - Connect means connecting to a unix socket. Abstract sockets (which
    ///   have no path) are local to the execution and always allowed.
    ///
    /// Every permission matches paths the same way.
    pub fs: Vec<FsRule>,
    /// Outgoing network access, by destination: TCP connections and UDP
    /// flows leaving the execution. Processes of one execution can always
    /// reach each other over loopback.
    ///
    /// DNS queries are answered by the sandbox itself; names no rule could
    /// allow don't resolve.
    pub net: Vec<NetRule>,
}

impl SandboxPolicy {
    /// Every path readable, writable and connectable, every destination
    /// reachable.
    pub fn unrestricted() -> Self {
        Self {
            fs: vec![FsRule {
                path: PathPattern::root(),
                read: Some(Access::Allow),
                write: Some(Access::Allow),
                connect: Some(Access::Allow),
            }],
            net: vec![NetRule::new(NetTarget::Any, Access::Allow)],
        }
    }
}

/// A rule of any resource kind, for handlers that add rules
/// ([`Decision::remember`]).
#[derive(Clone, Debug)]
pub enum Rule {
    Fs(FsRule),
    Net(NetRule),
}

#[derive(Clone, Debug)]
pub struct FsRule {
    pub path: PathPattern,
    /// `None`: this rule has no opinion, and earlier rules still apply.
    pub read: Option<Access>,
    pub write: Option<Access>,
    pub connect: Option<Access>,
}

impl FsRule {
    pub fn new(path: impl Into<String>) -> Result<Self> {
        Ok(Self {
            path: PathPattern::new(path)?,
            read: None,
            write: None,
            connect: None,
        })
    }

    pub fn connect(mut self, access: Access) -> Self {
        self.connect = Some(access);
        self
    }

    pub fn read(mut self, access: Access) -> Self {
        self.read = Some(access);
        self
    }

    pub fn write(mut self, access: Access) -> Self {
        self.write = Some(access);
        self
    }
}

/// Whether connections to `target` on `ports` may be made.
#[derive(Clone, Debug)]
pub struct NetRule {
    pub target: NetTarget,
    pub ports: PortSet,
    pub tcp: bool,
    pub udp: bool,
    pub connect: Access,
}

impl NetRule {
    /// A rule for every port, over both TCP and UDP.
    pub fn new(target: NetTarget, connect: Access) -> Self {
        Self {
            target,
            ports: PortSet::All,
            tcp: true,
            udp: true,
            connect,
        }
    }

    pub fn ports(mut self, ports: PortSet) -> Self {
        self.ports = ports;
        self
    }

    pub fn tcp_only(mut self) -> Self {
        self.udp = false;
        self
    }

    pub fn udp_only(mut self) -> Self {
        self.tcp = false;
        self
    }

    /// A rule from a destination written `[tcp/|udp/]TARGET[:PORTS]`, for
    /// command lines and text fields:
    /// - TARGET is `*` (anything), `loopback`, `local` (the local network),
    ///   an IP address or CIDR block (IPv6 in brackets when followed by
    ///   ports), or a domain (`*.example.com` for its subdomains);
    /// - PORTS is `*` (the default) or a list like `80,443,8000-8999`;
    /// - without a protocol prefix, the rule covers TCP and UDP.
    pub fn parse(destination: &str, connect: Access) -> Result<Self> {
        let destination = destination.trim();
        let (protocol, rest) = match destination.split_once('/') {
            Some((protocol @ ("tcp" | "udp"), rest)) => (Some(protocol), rest),
            _ => (None, destination),
        };
        let (target, ports) = if let Some(bracketed) = rest.strip_prefix('[') {
            let Some((target, after)) = bracketed.split_once(']') else {
                bail!("`{destination}` has an unclosed `[`");
            };
            match after.strip_prefix(':') {
                Some(ports) => (target, Some(ports)),
                None if after.is_empty() => (target, None),
                None => bail!("`{destination}`: expected `:` after `]`"),
            }
        } else if rest.matches(':').count() > 1 {
            // A bare IPv6 address or block.
            (rest, None)
        } else {
            match rest.split_once(':') {
                Some((target, ports)) => (target, Some(ports)),
                None => (rest, None),
            }
        };
        let target = match target {
            "*" => NetTarget::Any,
            "loopback" => NetTarget::Loopback,
            "local" => NetTarget::LocalNetwork,
            target => match target.parse() {
                Ok(network) => NetTarget::Ip(network),
                Err(_) => NetTarget::Domain(DomainPattern::new(target)?),
            },
        };
        let ports = match ports {
            None | Some("*") => PortSet::All,
            Some(ports) => {
                let mut ranges = Vec::new();
                for range in ports.split(',') {
                    let (start, end) = range.split_once('-').unwrap_or((range, range));
                    let (Ok(start), Ok(end)) = (start.trim().parse(), end.trim().parse()) else {
                        bail!("`{destination}` has an invalid port `{range}`");
                    };
                    ranges.push(start..=end);
                }
                PortSet::Ranges(ranges)
            }
        };
        let rule = NetRule::new(target, connect).ports(ports);
        Ok(match protocol {
            Some("tcp") => rule.tcp_only(),
            Some(_) => rule.udp_only(),
            None => rule,
        })
    }
}

/// Where a connection goes. A connection to a name (resolved through the
/// sandbox's DNS) only matches [`NetTarget::Domain`] and [`NetTarget::Any`];
/// a connection to a literal address matches the others.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetTarget {
    Any,
    Domain(DomainPattern),
    Ip(IpNet),
    /// Services on the host: `127.0.0.0/8`, `::1` and the unspecified
    /// addresses.
    Loopback,
    /// Private, link-local, carrier-grade NAT and multicast addresses.
    LocalNetwork,
}

impl NetTarget {
    fn matches(&self, host: &NetHost) -> bool {
        match host {
            NetHost::Domain(name) => self.matches_domain(name),
            NetHost::Ip(address) => match self {
                NetTarget::Any => true,
                NetTarget::Domain(_) => false,
                NetTarget::Ip(network) => network.contains(*address),
                NetTarget::Loopback => AddressClass::of(*address) == AddressClass::Loopback,
                NetTarget::LocalNetwork => AddressClass::of(*address) == AddressClass::LocalNetwork,
            },
        }
    }

    fn matches_domain(&self, name: &str) -> bool {
        match self {
            NetTarget::Any => true,
            NetTarget::Domain(pattern) => pattern.matches(name),
            _ => false,
        }
    }
}

/// A host name (`example.com`), or every subdomain of one (`*.example.com`,
/// which doesn't match `example.com` itself). Case-insensitive.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct DomainPattern {
    /// Lowercase, without a trailing dot.
    source: String,
}

impl DomainPattern {
    pub fn new(source: impl Into<String>) -> Result<Self> {
        let source = normalize_domain(&source.into());
        let name = source.strip_prefix("*.").unwrap_or(&source);
        let valid_label = |label: &str| {
            !label.is_empty()
                && label
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
        };
        if !name.split('.').all(valid_label) {
            bail!("`{source}` is not a domain name or `*.` followed by one");
        }
        Ok(Self { source })
    }

    pub fn as_str(&self) -> &str {
        &self.source
    }

    /// Whether `name` (normalized) matches.
    fn matches(&self, name: &str) -> bool {
        match self.source.strip_prefix("*.") {
            Some(parent) => name
                .strip_suffix(parent)
                .is_some_and(|prefix| prefix.len() > 1 && prefix.ends_with('.')),
            None => name == self.source,
        }
    }
}

/// Lowercase, without a trailing dot: how names are compared.
pub(crate) fn normalize_domain(name: &str) -> String {
    name.trim_end_matches('.').to_ascii_lowercase()
}

impl TryFrom<String> for DomainPattern {
    type Error = anyhow::Error;

    fn try_from(source: String) -> Result<Self> {
        Self::new(source)
    }
}

impl From<DomainPattern> for String {
    fn from(pattern: DomainPattern) -> Self {
        pattern.source
    }
}

/// A block of addresses in CIDR notation (`10.0.0.0/8`, `fd00::/8`), or a
/// single address.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct IpNet {
    address: IpAddr,
    prefix: u8,
}

impl IpNet {
    pub fn new(address: IpAddr, prefix: u8) -> Result<Self> {
        let address = address.to_canonical();
        let bits = if address.is_ipv4() { 32 } else { 128 };
        if prefix > bits {
            bail!("prefix /{prefix} is too long for {address}");
        }
        Ok(Self {
            address: mask(address, prefix),
            prefix,
        })
    }

    pub fn contains(&self, address: IpAddr) -> bool {
        let address = address.to_canonical();
        address.is_ipv4() == self.address.is_ipv4() && mask(address, self.prefix) == self.address
    }
}

fn mask(address: IpAddr, prefix: u8) -> IpAddr {
    match address {
        IpAddr::V4(address) => {
            let mask = u32::MAX.checked_shl(32 - u32::from(prefix)).unwrap_or(0);
            IpAddr::V4((u32::from(address) & mask).into())
        }
        IpAddr::V6(address) => {
            let mask = u128::MAX.checked_shl(128 - u32::from(prefix)).unwrap_or(0);
            IpAddr::V6((u128::from(address) & mask).into())
        }
    }
}

impl FromStr for IpNet {
    type Err = anyhow::Error;

    fn from_str(source: &str) -> Result<Self> {
        let (address, prefix) = match source.split_once('/') {
            Some((address, prefix)) => (address, Some(prefix)),
            None => (source, None),
        };
        let Ok(address) = address.parse::<IpAddr>() else {
            bail!("`{source}` is not an IP address or CIDR block");
        };
        let prefix = match prefix {
            Some(prefix) => match prefix.parse() {
                Ok(prefix) => prefix,
                Err(_) => bail!("`{source}` has an invalid prefix length"),
            },
            None if address.to_canonical().is_ipv4() => 32,
            None => 128,
        };
        Self::new(address, prefix)
    }
}

impl fmt::Display for IpNet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.address, self.prefix)
    }
}

impl TryFrom<String> for IpNet {
    type Error = anyhow::Error;

    fn try_from(source: String) -> Result<Self> {
        source.parse()
    }
}

impl From<IpNet> for String {
    fn from(network: IpNet) -> Self {
        network.to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortSet {
    All,
    Ranges(Vec<RangeInclusive<u16>>),
}

impl PortSet {
    pub fn single(port: u16) -> Self {
        Self::Ranges(vec![port..=port])
    }

    pub fn contains(&self, port: u16) -> bool {
        match self {
            PortSet::All => true,
            PortSet::Ranges(ranges) => ranges.iter().any(|range| range.contains(&port)),
        }
    }
}

/// The kind of address a connection goes to, as [`NetTarget::Loopback`]
/// and [`NetTarget::LocalNetwork`] classify it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AddressClass {
    Loopback,
    LocalNetwork,
    Public,
}

impl AddressClass {
    pub fn of(address: IpAddr) -> Self {
        match address.to_canonical() {
            IpAddr::V4(address) => {
                let carrier_grade_nat =
                    address.octets()[0] == 100 && address.octets()[1] & 0xc0 == 64;
                if address.is_loopback() || address.is_unspecified() {
                    Self::Loopback
                } else if address.is_private()
                    || address.is_link_local()
                    || carrier_grade_nat
                    || address.is_multicast()
                    || address.is_broadcast()
                {
                    Self::LocalNetwork
                } else {
                    Self::Public
                }
            }
            IpAddr::V6(address) => {
                let first = address.segments()[0];
                let unique_local = first & 0xfe00 == 0xfc00;
                let link_local = first & 0xffc0 == 0xfe80;
                if address.is_loopback() || address.is_unspecified() {
                    Self::Loopback
                } else if unique_local || link_local || address.is_multicast() {
                    Self::LocalNetwork
                } else {
                    Self::Public
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Access {
    Allow,
    Deny,
    /// Defer the decision to `Handler`, each time. The syscall blocks until
    /// it responds.
    Ask(Handler),
}

/// Decides requests for access that a rule set to [`Access::Ask`].
///
/// Called on a background thread shared by every sandbox in the context, so
/// it must return promptly: answer through the [`Responder`], from any
/// thread, whenever the decision is made. A panicking handler denies.
///
/// Requests to one handler in one sandbox are asked one at a time. While one
/// is open, an identical request shares its decision, and other requests
/// wait and are then re-checked against the policy (including any rules the
/// decision remembered) before being asked.
#[derive(Clone)]
pub struct Handler(pub(crate) Arc<dyn Fn(AccessRequest, Responder) + Send + Sync>);

impl Handler {
    pub fn new(handler: impl Fn(AccessRequest, Responder) + Send + Sync + 'static) -> Self {
        Self(Arc::new(handler))
    }
}

impl fmt::Debug for Handler {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Handler")
    }
}

/// What one blocked syscall needs. A single decision covers all of it.
#[derive(Clone, Debug)]
pub struct AccessRequest {
    pub items: Vec<RequestedAccess>,
    /// The thread that made the syscall.
    pub pid: i32,
    pub executable: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestedAccess {
    Fs {
        permission: Permission,
        path: PathBuf,
    },
    /// A TCP connection, or the first datagram of a UDP flow.
    Net {
        host: NetHost,
        port: u16,
        protocol: Protocol,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NetHost {
    /// The name the command looked up (lowercase, no trailing dot).
    Domain(String),
    Ip(IpAddr),
}

impl fmt::Display for NetHost {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetHost::Domain(name) => formatter.write_str(name),
            NetHost::Ip(address) => write!(formatter, "{address}"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Protocol {
    Tcp,
    Udp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    Read,
    Write,
    /// Connecting to a unix socket.
    Connect,
}

impl Permission {
    pub const ALL: [Permission; 3] = [Permission::Read, Permission::Write, Permission::Connect];
}

/// A handler's answer.
#[derive(Clone, Debug, Default)]
pub struct Decision {
    pub allow: bool,
    /// Rules appended to the sandbox's live policy before the syscall is
    /// answered, so later requests they cover don't ask again. They last
    /// until the sandbox is dropped or its policy is replaced.
    pub remember: Vec<Rule>,
}

impl Decision {
    pub fn allow() -> Self {
        Self {
            allow: true,
            remember: Vec::new(),
        }
    }

    pub fn deny() -> Self {
        Self::default()
    }
}

pub(crate) type HandlerId = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WireAccess {
    Allow,
    Deny,
    Ask(HandlerId),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum WireRule {
    Fs(WireFsRule),
    Net(WireNetRule),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct WireFsRule {
    pub path: PathPattern,
    pub read: Option<WireAccess>,
    pub write: Option<WireAccess>,
    pub connect: Option<WireAccess>,
}

impl WireFsRule {
    /// The only way to get at a rule's access: every permission is then
    /// decided by the same path matching ([`WirePolicy::fs_access`]).
    fn sets(&self, permission: Permission) -> Option<WireAccess> {
        match permission {
            Permission::Read => self.read,
            Permission::Write => self.write,
            Permission::Connect => self.connect,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct WireNetRule {
    pub target: NetTarget,
    pub ports: PortSet,
    pub tcp: bool,
    pub udp: bool,
    pub connect: WireAccess,
}

impl WireNetRule {
    fn covers(&self, port: u16, protocol: Protocol) -> bool {
        let protocol = match protocol {
            Protocol::Tcp => self.tcp,
            Protocol::Udp => self.udp,
        };
        protocol && self.ports.contains(port)
    }
}

/// Whether a name could be connected to, by any port and protocol.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NameVerdict {
    Deny,
    /// Only after asking.
    Ask,
    Allow,
}

/// At most this many UDP ports get relay sockets up front.
const MAX_UDP_PORTS: usize = 256;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct WirePolicy {
    pub fs: Vec<WireFsRule>,
    pub net: Vec<WireNetRule>,
}

impl WirePolicy {
    pub fn add(&mut self, rule: WireRule) {
        match rule {
            WireRule::Fs(rule) => self.fs.push(rule),
            WireRule::Net(rule) => self.net.push(rule),
        }
    }

    pub fn access(&self, request: &RequestedAccess) -> WireAccess {
        match request {
            RequestedAccess::Fs { permission, path } => self.fs_access(path, *permission),
            RequestedAccess::Net {
                host,
                port,
                protocol,
            } => self
                .net
                .iter()
                .rev()
                .find(|rule| rule.covers(*port, *protocol) && rule.target.matches(host))
                .map_or(WireAccess::Deny, |rule| rule.connect),
        }
    }

    /// Approximate: an `Allow` for some ports that a later `Deny` shadows
    /// still counts. Only a `Deny` covering every port and protocol hides
    /// the rules before it.
    pub fn name_verdict(&self, name: &str) -> NameVerdict {
        let mut ask = false;
        for rule in self.net.iter().rev() {
            if !rule.target.matches_domain(name) {
                continue;
            }
            match rule.connect {
                WireAccess::Allow => return NameVerdict::Allow,
                WireAccess::Ask(_) => ask = true,
                WireAccess::Deny => {
                    if rule.ports == PortSet::All && rule.tcp && rule.udp {
                        break;
                    }
                }
            }
        }
        if ask {
            NameVerdict::Ask
        } else {
            NameVerdict::Deny
        }
    }

    /// Whether some rule might allow UDP to `port` (ignoring shadowing).
    pub fn udp_may_allow(&self, port: u16) -> bool {
        self.net
            .iter()
            .any(|rule| rule.connect != WireAccess::Deny && rule.covers(port, Protocol::Udp))
    }

    /// The ports that rules naming explicit UDP ports might allow, so they
    /// can be relayed before anything connects to them.
    pub fn udp_ports(&self) -> BTreeSet<u16> {
        let mut ports = BTreeSet::new();
        for rule in &self.net {
            let PortSet::Ranges(ranges) = &rule.ports else {
                continue;
            };
            if !rule.udp || rule.connect == WireAccess::Deny {
                continue;
            }
            for port in ranges.iter().cloned().flatten() {
                if ports.len() >= MAX_UDP_PORTS {
                    return ports;
                }
                ports.insert(port);
            }
        }
        ports
    }

    pub fn fs_access(&self, path: &Path, permission: Permission) -> WireAccess {
        let mut buffer = [&[][..]; 32];
        let Some(components) = matching_components(path, &mut buffer) else {
            return WireAccess::Deny;
        };
        self.fs
            .iter()
            .rev()
            .filter_map(|rule| Some((rule, rule.sets(permission)?)))
            .find(|(rule, _)| rule.path.matches_components(components.clone()))
            .map_or(WireAccess::Deny, |(_, access)| access)
    }

    /// Whether every rule about `permission` treats paths beneath `a` exactly
    /// as it treats the same relative paths beneath `b`. If so, moving a
    /// directory from `a` to `b` can't change `permission` for anything in it.
    pub fn same_beneath(&self, a: &Path, b: &Path, permission: Permission) -> bool {
        self.fs
            .iter()
            .filter(|rule| rule.sets(permission).is_some())
            .all(|rule| rule.path.state_after(a) == rule.path.state_after(b))
    }

    /// Whether giving the object at `from` the additional or new name `to`
    /// would let `permission` reach it where it previously couldn't.
    pub fn widens(&self, from: &Path, to: &Path, permission: Permission) -> bool {
        self.fs_access(from, permission) != WireAccess::Allow
            && self.fs_access(to, permission) == WireAccess::Allow
    }
}

impl Rule {
    pub(crate) fn to_wire(&self, register: &mut impl FnMut(&Handler) -> HandlerId) -> WireRule {
        match self {
            Rule::Fs(rule) => WireRule::Fs(rule.to_wire(register)),
            Rule::Net(rule) => WireRule::Net(rule.to_wire(register)),
        }
    }
}

impl Access {
    fn to_wire(&self, register: &mut impl FnMut(&Handler) -> HandlerId) -> WireAccess {
        match self {
            Access::Allow => WireAccess::Allow,
            Access::Deny => WireAccess::Deny,
            Access::Ask(handler) => WireAccess::Ask(register(handler)),
        }
    }
}

impl NetRule {
    pub(crate) fn to_wire(&self, register: &mut impl FnMut(&Handler) -> HandlerId) -> WireNetRule {
        WireNetRule {
            target: self.target.clone(),
            ports: self.ports.clone(),
            tcp: self.tcp,
            udp: self.udp,
            connect: self.connect.to_wire(register),
        }
    }
}

impl FsRule {
    pub(crate) fn to_wire(&self, register: &mut impl FnMut(&Handler) -> HandlerId) -> WireFsRule {
        let mut convert =
            |access: &Option<Access>| access.as_ref().map(|access| access.to_wire(register));
        WireFsRule {
            path: self.path.clone(),
            read: convert(&self.read),
            write: convert(&self.write),
            connect: convert(&self.connect),
        }
    }
}

/// An absolute host path pattern. A pattern matches a path and everything
/// beneath it. Components may contain `*` (any run of characters other than
/// `/`) and `?` (one character), and a whole component of `**` matches any
/// number of components.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PathPattern {
    source: String,
    segments: Vec<Segment>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Segment {
    Literal(Vec<u8>),
    Wildcard(Vec<u8>),
    AnyDepth,
}

impl Segment {
    fn matches(&self, component: &[u8]) -> bool {
        match self {
            Segment::Literal(name) => name == component,
            Segment::Wildcard(pattern) => wildcard_matches(pattern, component),
            Segment::AnyDepth => true,
        }
    }
}

#[derive(Clone)]
enum MatchingComponents<'a> {
    Cached(std::iter::Copied<std::slice::Iter<'a, &'a [u8]>>),
    Streaming(Components<'a>),
}

impl<'a> Iterator for MatchingComponents<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Cached(components) => components.next(),
            Self::Streaming(components) => components.find_map(|component| match component {
                Component::Normal(name) => Some(name.as_bytes()),
                _ => None,
            }),
        }
    }
}

/// Validate the whole path before a pattern can return early: `/a` must not
/// match `/a/../secret`. Cache typical paths on the stack so each rule needn't
/// split them again. Deep paths use a borrowed iterator, not a heap allocation
/// or a depth limit.
fn matching_components<'a>(
    path: &'a Path,
    buffer: &'a mut [&'a [u8]; 32],
) -> Option<MatchingComponents<'a>> {
    let mut length = 0;
    for component in path.components() {
        match component {
            Component::RootDir => {}
            Component::Normal(name) => {
                if let Some(slot) = buffer.get_mut(length) {
                    *slot = name.as_bytes();
                }
                length += 1;
            }
            _ => return None,
        }
    }
    Some(if length <= buffer.len() {
        MatchingComponents::Cached(buffer[..length].iter().copied())
    } else {
        MatchingComponents::Streaming(path.components())
    })
}

/// Where matching stands after consuming some leading path components.
#[derive(Debug, PartialEq, Eq)]
enum MatchState {
    /// The components so far match, so everything beneath them does too.
    Matched,
    /// Segment positions that could still lead to a match (empty: none can).
    Pending(BTreeSet<usize>),
}

impl PathPattern {
    pub fn new(source: impl Into<String>) -> Result<Self> {
        let source = source.into();
        if !source.starts_with('/') {
            bail!("path pattern `{source}` must be absolute");
        }
        let mut segments = Vec::new();
        for component in source.split('/').filter(|component| !component.is_empty()) {
            segments.push(match component {
                "." | ".." => bail!("path pattern `{source}` must not contain `.` or `..`"),
                "**" => Segment::AnyDepth,
                _ if component.contains("**") => {
                    bail!("`**` must be a whole component in path pattern `{source}`")
                }
                _ if component.contains(['*', '?']) => {
                    Segment::Wildcard(component.as_bytes().to_vec())
                }
                _ => Segment::Literal(component.as_bytes().to_vec()),
            });
        }
        Ok(Self { source, segments })
    }

    /// A pattern matching exactly `path` and everything beneath it.
    pub fn literal(path: &Path) -> Result<Self> {
        let Some(source) = path.to_str() else {
            bail!("path {} is not valid UTF-8", path.display());
        };
        if source.contains(['*', '?']) {
            bail!("path {source} contains wildcard characters");
        }
        Self::new(source)
    }

    fn root() -> Self {
        Self {
            source: "/".to_string(),
            segments: Vec::new(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.source
    }

    /// Whether `path` (absolute and normalized) is matched by this pattern or
    /// lies beneath a match.
    pub fn matches(&self, path: &Path) -> bool {
        let mut buffer = [&[][..]; 32];
        matching_components(path, &mut buffer)
            .is_some_and(|components| self.matches_components(components))
    }

    fn matches_components(&self, mut components: MatchingComponents<'_>) -> bool {
        let mut position = 0;
        // Only the latest `**` needs a bookmark. Earlier ones can stop as
        // soon as their suffix reaches this one: extending an earlier `**`
        // can't match anything the later one couldn't consume instead.
        let mut restart = None;
        loop {
            if position == self.segments.len() {
                // A matched pattern also covers every descendant.
                return true;
            }
            if matches!(self.segments.get(position), Some(Segment::AnyDepth)) {
                position += 1;
                restart = Some((position, components.clone()));
                continue;
            }
            match components.next() {
                Some(component) if self.segments[position].matches(component) => {
                    position += 1;
                }
                _ => {
                    let Some((after_star, saved)) = &mut restart else {
                        return false;
                    };
                    if saved.next().is_none() {
                        return false;
                    }
                    position = *after_star;
                    components = saved.clone();
                }
            }
        }
    }

    fn state_after(&self, path: &Path) -> MatchState {
        let mut components = Vec::new();
        for component in path.components() {
            match component {
                Component::RootDir => {}
                Component::Normal(name) => components.push(name.as_bytes()),
                Component::Prefix(_) | Component::CurDir | Component::ParentDir => {
                    return MatchState::Pending(BTreeSet::new());
                }
            }
        }

        let end = self.segments.len();
        let mut positions = self.closure(BTreeSet::from([0]));
        for component in components {
            if positions.contains(&end) {
                return MatchState::Matched;
            }
            let mut next = BTreeSet::new();
            for &position in &positions {
                match &self.segments[position] {
                    Segment::AnyDepth => {
                        next.insert(position);
                    }
                    Segment::Literal(name) => {
                        if component == name.as_slice() {
                            next.insert(position + 1);
                        }
                    }
                    Segment::Wildcard(pattern) => {
                        if wildcard_matches(pattern, component) {
                            next.insert(position + 1);
                        }
                    }
                }
            }
            positions = self.closure(next);
        }
        if positions.contains(&end) {
            MatchState::Matched
        } else {
            MatchState::Pending(positions)
        }
    }

    /// Add the positions reachable without consuming a component (`**` may
    /// match zero components).
    fn closure(&self, mut positions: BTreeSet<usize>) -> BTreeSet<usize> {
        let mut stack: Vec<usize> = positions.iter().copied().collect();
        while let Some(position) = stack.pop() {
            if matches!(self.segments.get(position), Some(Segment::AnyDepth))
                && positions.insert(position + 1)
            {
                stack.push(position + 1);
            }
        }
        positions
    }
}

impl TryFrom<String> for PathPattern {
    type Error = anyhow::Error;

    fn try_from(source: String) -> Result<Self> {
        Self::new(source)
    }
}

impl From<PathPattern> for String {
    fn from(pattern: PathPattern) -> Self {
        pattern.source
    }
}

fn wildcard_matches(pattern: &[u8], text: &[u8]) -> bool {
    let (mut position, mut consumed) = (0, 0);
    let mut restart = None;
    while consumed < text.len() {
        match pattern.get(position) {
            Some(b'*') => {
                position += 1;
                restart = Some((position, consumed));
            }
            Some(byte) if *byte == b'?' || *byte == text[consumed] => {
                position += 1;
                consumed += 1;
            }
            _ => {
                let Some((after_star, saved)) = &mut restart else {
                    return false;
                };
                *saved += 1;
                position = *after_star;
                consumed = *saved;
            }
        }
    }
    pattern[position..].iter().all(|byte| *byte == b'*')
}

#[cfg(test)]
mod tests {
    use super::*;
    use Access::{Allow, Deny};
    use Permission::*;

    struct CountingAllocator;

    thread_local! {
        static ALLOCATIONS: std::cell::Cell<Option<usize>> = const {
            std::cell::Cell::new(None)
        };
    }

    // Count only the calling test thread, not other tests running alongside it.
    #[global_allocator]
    static ALLOCATOR: CountingAllocator = CountingAllocator;

    unsafe impl std::alloc::GlobalAlloc for CountingAllocator {
        unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
            ALLOCATIONS.with(|count| count.set(count.get().map(|count| count + 1)));
            // SAFETY: forward the allocator contract unchanged.
            unsafe { std::alloc::System.alloc(layout) }
        }

        unsafe fn alloc_zeroed(&self, layout: std::alloc::Layout) -> *mut u8 {
            ALLOCATIONS.with(|count| count.set(count.get().map(|count| count + 1)));
            // SAFETY: forward the allocator contract unchanged.
            unsafe { std::alloc::System.alloc_zeroed(layout) }
        }

        unsafe fn realloc(
            &self,
            pointer: *mut u8,
            layout: std::alloc::Layout,
            size: usize,
        ) -> *mut u8 {
            ALLOCATIONS.with(|count| count.set(count.get().map(|count| count + 1)));
            // SAFETY: forward the allocator contract unchanged.
            unsafe { std::alloc::System.realloc(pointer, layout, size) }
        }

        unsafe fn dealloc(&self, pointer: *mut u8, layout: std::alloc::Layout) {
            // SAFETY: forward the allocator contract unchanged.
            unsafe { std::alloc::System.dealloc(pointer, layout) }
        }
    }

    fn pattern(source: &str) -> PathPattern {
        PathPattern::new(source).unwrap()
    }

    fn rule(source: &str) -> FsRule {
        FsRule::new(source).unwrap()
    }

    #[test]
    fn test_allocation_free_matcher_equivalence() {
        fn combinations(parts: &[&str], depth: usize) -> Vec<String> {
            let mut all = vec![String::new()];
            let mut level = vec![String::new()];
            for _ in 0..depth {
                level = level
                    .iter()
                    .flat_map(|prefix| parts.iter().map(move |part| format!("{prefix}/{part}")))
                    .collect();
                all.extend(level.iter().cloned());
            }
            all
        }
        let patterns = combinations(&["a", "b", "*a", "?", "**"], 4);
        let mut paths = combinations(&["a", "b", "aa", "ab"], 4);
        paths.extend(["a", "./a", "/a/../b", "/a/./b", "//a//b/", "/", ""].map(str::to_owned));
        let patterns: Vec<_> = patterns
            .iter()
            .map(|source| pattern(&format!("/{source}")))
            .collect();
        for pattern in &patterns {
            for path in &paths {
                let path = Path::new(path);
                assert_eq!(
                    pattern.matches(path),
                    pattern.state_after(path) == MatchState::Matched,
                    "{}: {path:?}",
                    pattern.as_str(),
                );
            }
        }
        for patterns in patterns.chunks(9) {
            let policy = wire(
                patterns
                    .iter()
                    .enumerate()
                    .map(|(index, path)| {
                        let access = match index % 3 {
                            0 => Allow,
                            1 => Deny,
                            _ => Access::Ask(Handler::new(|_, _| {})),
                        };
                        let mut rule = FsRule::new(path.as_str()).unwrap();
                        match index % 4 {
                            0 => rule.read = Some(access),
                            1 => rule.write = Some(access),
                            2 => rule.connect = Some(access),
                            _ => {
                                rule.read = Some(access.clone());
                                rule.write = Some(access.clone());
                                rule.connect = Some(access);
                            }
                        }
                        rule
                    })
                    .collect(),
            );
            for path in &paths {
                let path = Path::new(path);
                for permission in Permission::ALL {
                    let expected = policy
                        .fs
                        .iter()
                        .rev()
                        .filter_map(|rule| Some((rule, rule.sets(permission)?)))
                        .find(|(rule, _)| rule.path.state_after(path) == MatchState::Matched)
                        .map_or(WireAccess::Deny, |(_, access)| access);
                    assert_eq!(policy.fs_access(path, permission), expected);
                }
            }
        }
    }

    #[test]
    fn test_long_patterns_and_non_utf8_paths() {
        for length in [31, 32, 33, 63, 64, 65, 128, 256] {
            let prefix = "/a".repeat(length);
            let source = format!("{prefix}/**/b?*/**/tail");
            let pattern = pattern(&source);
            for suffix in ["/b0/tail", "/c/b12/c/tail/child", "/b/tail", "/wrong"] {
                let path = format!("{prefix}{suffix}");
                assert_eq!(
                    pattern.matches(Path::new(&path)),
                    pattern.state_after(Path::new(&path)) == MatchState::Matched,
                );
            }
            let path = format!("{prefix}/b0/tail");
            assert!(pattern.matches(Path::new(&path)));
        }
        let path = Path::new(std::ffi::OsStr::from_bytes(b"/a/\xff/tail"));
        assert!(pattern("/a/?/tail").matches(path));
        assert!(pattern("/**/tail").matches(path));
        assert!(!pattern("/a/a*/tail").matches(path));
        let prefix = "/a".repeat(33);
        for source in ["/**/a/*b/**/tail", "/**/a/**/b*/tail", "/**/tail"] {
            let pattern = pattern(source);
            for suffix in ["/ab/c/tail", "/a/b/tail", "/tail", "/wrong"] {
                let path = format!("{prefix}{suffix}");
                assert_eq!(
                    pattern.matches(Path::new(&path)),
                    pattern.state_after(Path::new(&path)) == MatchState::Matched,
                );
            }
        }
    }

    #[test]
    fn test_policy_queries_do_not_allocate() {
        use std::hint::black_box;

        let prefix = "/a".repeat(256);
        let policy = wire(vec![
            rule("/").read(Allow),
            rule("/**/*.env").read(Deny),
            rule("/proj/**/socket?").connect(Allow),
            rule(&format!("{prefix}/**/b?*/**/tail")).write(Allow),
        ]);
        let paths = [
            "/proj/.env".to_owned(),
            "/proj/sub/socket1".to_owned(),
            "/other/file".to_owned(),
            format!("{prefix}/b0/tail"),
        ];
        ALLOCATIONS.with(|count| count.set(Some(0)));
        for path in &paths {
            let path = black_box(Path::new(path));
            for rule in &policy.fs {
                black_box(rule.path.matches(path));
            }
            for permission in Permission::ALL {
                black_box(policy.fs_access(path, permission));
            }
        }
        let allocations = ALLOCATIONS.with(|count| count.replace(None)).unwrap();
        assert_eq!(allocations, 0);
    }

    #[test]
    fn test_component_glob_equivalence() {
        fn reference(pattern: &[u8], text: &[u8]) -> bool {
            match pattern.split_first() {
                None => text.is_empty(),
                Some((b'*', rest)) => (0..=text.len()).any(|skip| reference(rest, &text[skip..])),
                Some((b'?', rest)) => !text.is_empty() && reference(rest, &text[1..]),
                Some((byte, rest)) => text.first() == Some(byte) && reference(rest, &text[1..]),
            }
        }
        for encoded in 0..4096usize {
            let mut value = encoded;
            let mut pattern = Vec::new();
            for _ in 0..6 {
                pattern.push(b"ab*?"[value % 4]);
                value /= 4;
            }
            for length in 0..=5 {
                for encoded in 0..1usize << length {
                    let text: Vec<_> = (0..length)
                        .map(|position| b"ab"[(encoded >> position) & 1])
                        .collect();
                    assert_eq!(
                        wildcard_matches(&pattern, &text),
                        reference(&pattern, &text),
                        "{pattern:?}: {text:?}",
                    );
                }
            }
        }
        assert!(wildcard_matches(b"*", b""));
        assert!(!wildcard_matches(b"?", b""));
        // No recursive backtracking or call-stack limit.
        let mut pattern = b"*a".repeat(1024);
        pattern.push(b'b');
        assert!(!wildcard_matches(&pattern, &vec![b'a'; 2048]));
    }

    /// Convert as `Sandbox` does, numbering handlers in order of appearance.
    fn wire(rules: Vec<FsRule>) -> WirePolicy {
        let mut next = 0;
        let mut register = |_: &Handler| {
            next += 1;
            next - 1
        };
        WirePolicy {
            fs: rules
                .iter()
                .map(|rule| rule.to_wire(&mut register))
                .collect(),
            net: Vec::new(),
        }
    }

    fn wire_net(rules: Vec<NetRule>) -> WirePolicy {
        let mut next = 0;
        let mut register = |_: &Handler| {
            next += 1;
            next - 1
        };
        WirePolicy {
            fs: Vec::new(),
            net: rules
                .iter()
                .map(|rule| rule.to_wire(&mut register))
                .collect(),
        }
    }

    #[test]
    fn test_net_rules() {
        let domain = |source: &str| NetTarget::Domain(DomainPattern::new(source).unwrap());
        let ip = |source: &str| NetTarget::Ip(source.parse().unwrap());
        let policy = wire_net(vec![
            NetRule::new(domain("*.example.com"), Allow).ports(PortSet::single(443)),
            NetRule::new(domain("ask.test"), Access::Ask(Handler::new(|_, _| {}))),
            NetRule::new(ip("10.0.0.0/8"), Allow).tcp_only(),
            NetRule::new(ip("10.1.2.3"), Deny),
            NetRule::new(NetTarget::Loopback, Allow).ports(PortSet::Ranges(vec![5000..=5999])),
        ]);
        let access = |host: NetHost, port, protocol| {
            policy.access(&RequestedAccess::Net {
                host,
                port,
                protocol,
            })
        };
        let name = |name: &str| NetHost::Domain(name.to_string());
        let address = |address: &str| NetHost::Ip(address.parse().unwrap());
        use Protocol::*;
        assert_eq!(access(name("api.example.com"), 443, Tcp), WireAccess::Allow);
        assert_eq!(access(name("example.com"), 443, Tcp), WireAccess::Deny);
        assert_eq!(access(name("api.example.com"), 80, Tcp), WireAccess::Deny);
        assert_eq!(access(name("ask.test"), 22, Udp), WireAccess::Ask(0));
        assert_eq!(access(address("10.9.9.9"), 22, Tcp), WireAccess::Allow);
        assert_eq!(access(address("10.9.9.9"), 22, Udp), WireAccess::Deny);
        assert_eq!(access(address("10.1.2.3"), 22, Tcp), WireAccess::Deny);
        assert_eq!(
            access(address("::ffff:10.9.9.9"), 22, Tcp),
            WireAccess::Allow
        );
        assert_eq!(access(address("127.0.0.1"), 5432, Tcp), WireAccess::Allow);
        assert_eq!(access(address("::1"), 5432, Udp), WireAccess::Allow);
        assert_eq!(access(address("127.0.0.1"), 6000, Tcp), WireAccess::Deny);
        // Names only match name rules.
        assert_eq!(access(name("localhost"), 5432, Tcp), WireAccess::Deny);

        assert_eq!(policy.name_verdict("api.example.com"), NameVerdict::Allow);
        assert_eq!(policy.name_verdict("ask.test"), NameVerdict::Ask);
        assert_eq!(policy.name_verdict("other.test"), NameVerdict::Deny);
        // `ask.test` covers every UDP port.
        assert!(policy.udp_may_allow(22));
        assert!(!wire_net(Vec::new()).udp_may_allow(22));
        let ports = policy.udp_ports();
        assert!(ports.contains(&443) && ports.contains(&5000));
        assert_eq!(ports.len(), MAX_UDP_PORTS);
    }

    #[test]
    fn test_address_classes() {
        let class = |address: &str| AddressClass::of(address.parse().unwrap());
        assert_eq!(class("127.0.0.53"), AddressClass::Loopback);
        assert_eq!(class("0.0.0.0"), AddressClass::Loopback);
        assert_eq!(class("::ffff:127.0.0.1"), AddressClass::Loopback);
        assert_eq!(class("192.168.1.1"), AddressClass::LocalNetwork);
        assert_eq!(class("100.100.1.1"), AddressClass::LocalNetwork);
        assert_eq!(class("100.128.1.1"), AddressClass::Public);
        assert_eq!(class("fd12::1"), AddressClass::LocalNetwork);
        assert_eq!(class("fe80::1"), AddressClass::LocalNetwork);
        assert_eq!(class("2606:4700::1111"), AddressClass::Public);
        assert!(
            DomainPattern::new("*.Example.COM.")
                .unwrap()
                .matches("a.b.example.com")
        );
        assert!(DomainPattern::new("exa*mple.com").is_err());
        assert!(DomainPattern::new("*").is_err());

        let parse = |destination| NetRule::parse(destination, Allow).unwrap();
        let rule = parse("tcp/*.github.com:22,443,8000-8999");
        assert_eq!(
            rule.target,
            NetTarget::Domain(DomainPattern::new("*.github.com").unwrap())
        );
        assert_eq!(
            rule.ports,
            PortSet::Ranges(vec![22..=22, 443..=443, 8000..=8999])
        );
        assert!(rule.tcp && !rule.udp);
        assert_eq!(
            parse("[fd00::/8]:53").target,
            NetTarget::Ip("fd00::/8".parse().unwrap())
        );
        assert_eq!(parse("::1").ports, PortSet::All);
        assert_eq!(parse("udp/loopback").target, NetTarget::Loopback);
        assert!(NetRule::parse("example.com:http", Allow).is_err());
    }

    #[test]
    fn test_pattern_matching() {
        assert!(pattern("/").matches(Path::new("/anything/at/all")));
        assert!(pattern("/proj").matches(Path::new("/proj")));
        assert!(pattern("/proj").matches(Path::new("/proj/src/main.rs")));
        assert!(!pattern("/proj").matches(Path::new("/project")));
        assert!(!pattern("/proj").matches(Path::new("/")));
        assert!(pattern("/proj/*.env").matches(Path::new("/proj/prod.env")));
        assert!(pattern("/proj/*.env").matches(Path::new("/proj/.env")));
        assert!(!pattern("/proj/*.env").matches(Path::new("/proj/sub/prod.env")));
        assert!(pattern("/**/*.env").matches(Path::new("/a/b/c.env")));
        assert!(pattern("/**/*.env").matches(Path::new("/c.env")));
        assert!(pattern("/proj/**/.git").matches(Path::new("/proj/.git/config")));
        assert!(pattern("/proj/**/.git").matches(Path::new("/proj/a/b/.git")));
        assert!(!pattern("/proj/**/.git").matches(Path::new("/proj/a/b/git")));
        assert!(pattern("/a/?").matches(Path::new("/a/b")));
        assert!(!pattern("/a/?").matches(Path::new("/a/bc")));
        assert!(!pattern("/proj").matches(Path::new("/proj/../etc")));
    }

    #[test]
    fn test_pattern_validation() {
        assert!(PathPattern::new("relative").is_err());
        assert!(PathPattern::new("/a/../b").is_err());
        assert!(PathPattern::new("/a/b**").is_err());
        assert!(PathPattern::literal(Path::new("/a/b*")).is_err());
    }

    #[test]
    fn test_last_matching_rule_wins_per_permission() {
        let handler = Handler::new(|_, _| {});
        let policy = wire(vec![
            rule("/").read(Allow),
            rule("/proj").write(Allow),
            rule("/proj/.git").write(Deny),
            rule("/**/*.env").read(Deny),
            rule("/proj/ask").write(Access::Ask(handler)),
        ]);
        let access = |path: &str, permission| policy.fs_access(Path::new(path), permission);
        assert_eq!(access("/etc/passwd", Read), WireAccess::Allow);
        assert_eq!(access("/etc/passwd", Write), WireAccess::Deny);
        assert_eq!(access("/proj/src", Write), WireAccess::Allow);
        assert_eq!(access("/proj/.git/config", Write), WireAccess::Deny);
        assert_eq!(access("/proj/.env", Read), WireAccess::Deny);
        // The `.env` rule says nothing about writes.
        assert_eq!(access("/proj/.env", Write), WireAccess::Allow);
        assert_eq!(access("/proj/ask/x", Write), WireAccess::Ask(0));
        assert_eq!(
            WirePolicy::default().fs_access(Path::new("/x"), Read),
            WireAccess::Deny
        );
        assert_eq!(
            wire(SandboxPolicy::unrestricted().fs).fs_access(Path::new("/x"), Write),
            WireAccess::Allow
        );
    }

    #[test]
    fn test_directory_moves() {
        let policy = wire(vec![
            rule("/").read(Allow),
            rule("/proj").write(Allow),
            rule("/proj/.git").write(Deny),
            rule("/**/*.env").read(Deny),
            rule("/home/*/secret").read(Deny),
        ]);
        let same = |a: &str, b: &str, permission| {
            policy.same_beneath(Path::new(a), Path::new(b), permission)
        };
        // `**/*.env` doesn't care where a directory lives.
        assert!(same("/proj/a", "/proj/b", Read));
        assert!(same("/proj/a", "/proj/b", Write));
        // Moving the project would move `.git` out from under its rule.
        assert!(!same("/proj", "/proj2", Write));
        // Contents named `secret` are only denied directly under `/home/*`.
        assert!(!same("/home", "/tmp", Read));
        assert!(same("/home/u/a", "/tmp/a", Read));
        assert!(!same("/home/u", "/tmp/u", Read));
    }
}
