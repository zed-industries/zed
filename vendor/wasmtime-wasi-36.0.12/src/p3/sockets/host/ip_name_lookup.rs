use tokio::net::lookup_host;
use wasmtime::component::Accessor;

use crate::p3::bindings::sockets::ip_name_lookup::{ErrorCode, Host, HostWithStore};
use crate::p3::bindings::sockets::types;
use crate::p3::sockets::WasiSockets;
use crate::sockets::WasiSocketsCtxView;
use crate::sockets::util::{from_ipv4_addr, from_ipv6_addr, parse_host};

impl HostWithStore for WasiSockets {
    async fn resolve_addresses<U>(
        store: &Accessor<U, Self>,
        name: String,
    ) -> wasmtime::Result<Result<Vec<types::IpAddress>, ErrorCode>> {
        let Ok(host) = parse_host(&name) else {
            return Ok(Err(ErrorCode::InvalidArgument));
        };
        if !store.with(|mut view| view.get().ctx.allowed_network_uses.ip_name_lookup) {
            return Ok(Err(ErrorCode::PermanentResolverFailure));
        }
        match host {
            url::Host::Ipv4(addr) => Ok(Ok(vec![types::IpAddress::Ipv4(from_ipv4_addr(addr))])),
            url::Host::Ipv6(addr) => Ok(Ok(vec![types::IpAddress::Ipv6(from_ipv6_addr(addr))])),
            url::Host::Domain(domain) => {
                // This is only resolving names, not ports, so force the port to be 0.
                if let Ok(addrs) = lookup_host((domain.as_str(), 0)).await {
                    Ok(Ok(addrs
                        .map(|addr| addr.ip().to_canonical().into())
                        .collect()))
                } else {
                    // If/when we use `getaddrinfo` directly, map the error properly.
                    Ok(Err(ErrorCode::NameUnresolvable))
                }
            }
        }
    }
}

impl Host for WasiSocketsCtxView<'_> {}
