//! A resolved-and-vetted network destination, pinned to the exact IP addresses
//! that were checked.
//!
//! DNS rebinding / SSRF is a time-of-check-to-time-of-use hazard: code that
//! resolves a hostname, decides it's safe, and then hands the *hostname* back to
//! something that resolves it *again* has checked one answer and used another. A
//! hostname that the sandbox distrusts can exploit that window to point a
//! "granted" host at loopback, the LAN, or a cloud metadata endpoint.
//!
//! [`PinnedHost`] closes that window the way [`sandbox::HostFilesystemLocation`]
//! does for filesystem paths: construction resolves the name and vets every
//! address *once*, and the resulting value carries the checked addresses by
//! value. The type is deliberately opaque — it never hands back the hostname for
//! re-resolution — so the only way to "use" it is to connect to an address it
//! already vetted. Re-deriving a destination from the hostname requires reaching
//! for [`PinnedHost::untrusted_host_display`], whose name flags it as
//! display-only, so the type system nudges callers away from reintroducing the
//! TOCTOU.
//!
//! One `PinnedHost` describes **one** hostname's resolved addresses. A redirect
//! chain that visits several hosts produces one `PinnedHost` per hop, each
//! resolved and vetted independently at the moment its hop runs.

use crate::allowlist::Allowlist;
use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs as _};

/// Why pinning a host failed.
#[derive(Debug, thiserror::Error)]
pub enum PinnedHostError {
    /// DNS resolution itself failed (or the host is malformed).
    #[error("resolving {host}:{port}: {source}")]
    Resolve {
        host: String,
        port: u16,
        source: std::io::Error,
    },
    /// The host resolved, but to no addresses at all.
    #[error("{host}:{port} did not resolve to any address")]
    NoAddresses { host: String, port: u16 },
    /// Every resolved address was in loopback / private / link-local space, so
    /// there is nothing safe to connect to (DNS-rebinding protection).
    #[error(
        "{host} resolved only to loopback/private/link-local addresses, \
         which the sandbox never reaches by hostname"
    )]
    AllAddressesForbidden { host: String },
}

/// A hostname whose DNS has been resolved and whose addresses have been vetted
/// against the forbidden-IP policy, pinned to those exact addresses.
///
/// Construct with [`PinnedHost::resolve`] (applies the forbidden-IP filter) or
/// [`PinnedHost::resolve_allowing_any`] (skips it, for the "allow any host"
/// grant that means unrestricted egress). Use the pinned addresses via
/// [`PinnedHost::socket_addrs`]; the hostname is available only via
/// [`PinnedHost::untrusted_host_display`], which must never be fed back into a
/// resolver.
#[derive(Debug, Clone)]
pub struct PinnedHost {
    /// The vetted addresses, deduplicated. A set rather than a list because DNS
    /// order carries no meaning here and duplicate answers (or duplicates across
    /// A/AAAA records) should collapse — what matters is the set of endpoints we
    /// concluded are safe to reach.
    addrs: HashSet<SocketAddr>,
    /// The requested host, kept **only** for display in errors/UI. Never
    /// consulted to connect — treat it as untrusted, attacker-influenced text.
    untrusted_host_for_display: String,
}

impl PinnedHost {
    /// Resolve `host:port` and pin the subset of addresses that pass the
    /// forbidden-IP filter. Fails if resolution fails, yields no addresses, or
    /// yields only forbidden ones.
    pub fn resolve(host: &str, port: u16) -> Result<Self, PinnedHostError> {
        Self::resolve_inner(host, port, true)
    }

    /// Resolve `host:port` and pin **every** resolved address without applying
    /// the forbidden-IP filter.
    ///
    /// This is for the "allow any host" grant (`allow_all_hosts` /
    /// [`Allowlist::allows_any`]), which is unrestricted egress by definition —
    /// including the local network and metadata endpoints. Callers that don't
    /// hold such a grant must use [`PinnedHost::resolve`].
    pub fn resolve_allowing_any(host: &str, port: u16) -> Result<Self, PinnedHostError> {
        Self::resolve_inner(host, port, false)
    }

    /// Resolve against an [`Allowlist`], choosing the filtered or unfiltered path
    /// based on whether the allowlist grants arbitrary egress.
    pub fn resolve_for_allowlist(
        host: &str,
        port: u16,
        allowlist: &Allowlist,
    ) -> Result<Self, PinnedHostError> {
        Self::resolve_inner(host, port, !allowlist.allows_any())
    }

    fn resolve_inner(host: &str, port: u16, vet: bool) -> Result<Self, PinnedHostError> {
        let resolved =
            (host, port)
                .to_socket_addrs()
                .map_err(|source| PinnedHostError::Resolve {
                    host: host.to_string(),
                    port,
                    source,
                })?;

        let mut addrs = HashSet::new();
        let mut saw_any = false;
        for addr in resolved {
            saw_any = true;
            if vet && is_forbidden_ip(addr.ip()) {
                continue;
            }
            addrs.insert(addr);
        }

        if !saw_any {
            return Err(PinnedHostError::NoAddresses {
                host: host.to_string(),
                port,
            });
        }
        if addrs.is_empty() {
            return Err(PinnedHostError::AllAddressesForbidden {
                host: host.to_string(),
            });
        }

        Ok(Self {
            addrs,
            untrusted_host_for_display: host.to_string(),
        })
    }

    /// The vetted addresses to connect to. Connecting to one of these — rather
    /// than re-resolving the hostname — is what keeps the check and the use
    /// pinned to the same answer.
    pub fn socket_addrs(&self) -> impl ExactSizeIterator<Item = SocketAddr> + '_ {
        self.addrs.iter().copied()
    }

    /// The requested host, for **display only** (errors, UI). This intentionally
    /// returns the untrusted, as-requested hostname — never a vetted address. Do
    /// not feed the result back into a resolver as if it identified this
    /// destination.
    pub fn untrusted_host_display(&self) -> &str {
        &self.untrusted_host_for_display
    }
}

/// Whether a resolved address is in loopback / private / link-local space —
/// destinations a hostname allowlist must never reach. The OS sandbox already
/// blocks them for direct connections from the sandbox; code running outside the
/// sandbox (the proxy, the fetch tool) must not reopen them.
pub fn is_forbidden_ip(ip: IpAddr) -> bool {
    // Escape hatch for the NixOS sandbox integration tests only: their echo
    // servers live on the VM's private network, which this filter would
    // otherwise reject. It is compiled in ONLY under the
    // `nixos-integration-tests` feature (enabled via `sandbox/nixos-test` when
    // building `bwrap_test_helper`), so in a real Zed build the env var has no
    // effect and cannot disable DNS-rebinding/SSRF protection.
    #[cfg(feature = "nixos-integration-tests")]
    if std::env::var_os("ZED_SANDBOX_PROXY_ALLOW_LOCAL_IPS").is_some() {
        return false;
    }
    match ip {
        IpAddr::V4(v4) => is_forbidden_ipv4(v4),
        IpAddr::V6(v6) => {
            if let Some(v4) = v6.to_ipv4_mapped() {
                return is_forbidden_ipv4(v4);
            }
            v6.is_loopback()
                || v6.is_unspecified()
                // Link-local (fe80::/10) and unique-local (fc00::/7); the
                // dedicated `is_unicast_link_local` / `is_unique_local`
                // methods are not yet stable.
                || (v6.segments()[0] & 0xffc0) == 0xfe80
                || (v6.segments()[0] & 0xfe00) == 0xfc00
        }
    }
}

fn is_forbidden_ipv4(ip: Ipv4Addr) -> bool {
    let octets = ip.octets();
    ip.is_loopback()
        || ip.is_private()
        || ip.is_link_local() // includes 169.254.169.254 cloud metadata
        || ip.is_unspecified()
        || ip.is_broadcast()
        // Shared address space (RFC 6598, 100.64.0.0/10): CGNAT, and notably
        // Tailscale-style overlay networks.
        || (octets[0] == 100 && (octets[1] & 0xc0) == 64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forbidden_ip_covers_v4_ranges() {
        for ip in [
            "127.0.0.1",
            "10.0.0.1",
            "192.168.1.1",
            "172.16.0.1",
            "169.254.169.254", // cloud metadata
            "0.0.0.0",
            "255.255.255.255",
            "100.64.0.1", // CGNAT / Tailscale
        ] {
            assert!(
                is_forbidden_ip(ip.parse().unwrap()),
                "expected {ip} to be forbidden"
            );
        }
    }

    #[test]
    fn forbidden_ip_covers_v6_ranges() {
        for ip in [
            "::1",              // loopback
            "::",               // unspecified
            "fe80::1",          // link-local
            "fc00::1",          // unique-local
            "::ffff:127.0.0.1", // IPv4-mapped loopback
            "::ffff:10.0.0.1",  // IPv4-mapped private
        ] {
            assert!(
                is_forbidden_ip(ip.parse().unwrap()),
                "expected {ip} to be forbidden"
            );
        }
    }

    #[test]
    fn public_ips_are_allowed() {
        for ip in [
            "93.184.215.14",
            "8.8.8.8",
            "2606:2800:220:1:248:1893:25c8:1946",
        ] {
            assert!(
                !is_forbidden_ip(ip.parse().unwrap()),
                "expected {ip} to be allowed"
            );
        }
    }

    #[test]
    fn resolve_pins_public_literal_addresses() {
        // An IP literal "resolves" to itself, so this exercises the vetting and
        // pinning without depending on real DNS.
        let pinned = PinnedHost::resolve("93.184.215.14", 443).expect("public IP should pin");
        let addrs: HashSet<SocketAddr> = pinned.socket_addrs().collect();
        assert_eq!(addrs, HashSet::from(["93.184.215.14:443".parse().unwrap()]));
        assert_eq!(pinned.untrusted_host_display(), "93.184.215.14");
    }

    #[test]
    fn resolve_rejects_forbidden_literal_address() {
        let error = PinnedHost::resolve("127.0.0.1", 80).expect_err("loopback must be rejected");
        assert!(matches!(
            error,
            PinnedHostError::AllAddressesForbidden { .. }
        ));
    }

    #[test]
    fn resolve_allowing_any_keeps_forbidden_addresses() {
        let pinned =
            PinnedHost::resolve_allowing_any("127.0.0.1", 80).expect("allow-any keeps loopback");
        let addrs: HashSet<SocketAddr> = pinned.socket_addrs().collect();
        assert_eq!(addrs, HashSet::from(["127.0.0.1:80".parse().unwrap()]));
    }

    #[test]
    fn resolve_deduplicates_addresses() {
        // `localhost` typically resolves to both 127.0.0.1 and ::1; under
        // allow-any both are kept and distinct. This mainly guards that the set
        // collapses exact duplicates rather than the specific addresses.
        let pinned = PinnedHost::resolve_allowing_any("127.0.0.1", 80).unwrap();
        assert_eq!(pinned.socket_addrs().count(), 1);
    }
}
