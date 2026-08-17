use crate::p2::bindings::sockets::ip_name_lookup::{Host, HostResolveAddressStream};
use crate::p2::bindings::sockets::network::{ErrorCode, IpAddress, Network};
use crate::p2::{SocketError, WasiCtxView};
use crate::runtime::{AbortOnDropJoinHandle, spawn_blocking};
use anyhow::Result;
use std::mem;
use std::net::ToSocketAddrs;
use std::pin::Pin;
use std::vec;
use wasmtime::component::Resource;
use wasmtime_wasi_io::poll::{DynPollable, Pollable, subscribe};

use crate::sockets::util::{from_ipv4_addr, from_ipv6_addr, parse_host};

pub enum ResolveAddressStream {
    Waiting(AbortOnDropJoinHandle<Result<Vec<IpAddress>, SocketError>>),
    Done(Result<vec::IntoIter<IpAddress>, SocketError>),
}

impl Host for WasiCtxView<'_> {
    fn resolve_addresses(
        &mut self,
        network: Resource<Network>,
        name: String,
    ) -> Result<Resource<ResolveAddressStream>, SocketError> {
        let network = self.table.get(&network)?;

        let host = parse_host(&name)?;

        if !network.allow_ip_name_lookup {
            return Err(ErrorCode::PermanentResolverFailure.into());
        }

        let task = spawn_blocking(move || blocking_resolve(&host));
        let resource = self.table.push(ResolveAddressStream::Waiting(task))?;
        Ok(resource)
    }
}

impl HostResolveAddressStream for WasiCtxView<'_> {
    fn resolve_next_address(
        &mut self,
        resource: Resource<ResolveAddressStream>,
    ) -> Result<Option<IpAddress>, SocketError> {
        let stream: &mut ResolveAddressStream = self.table.get_mut(&resource)?;
        loop {
            match stream {
                ResolveAddressStream::Waiting(future) => {
                    match crate::runtime::poll_noop(Pin::new(future)) {
                        Some(result) => {
                            *stream = ResolveAddressStream::Done(result.map(|v| v.into_iter()));
                        }
                        None => return Err(ErrorCode::WouldBlock.into()),
                    }
                }
                ResolveAddressStream::Done(slot @ Err(_)) => {
                    mem::replace(slot, Ok(Vec::new().into_iter()))?;
                    unreachable!();
                }
                ResolveAddressStream::Done(Ok(iter)) => return Ok(iter.next()),
            }
        }
    }

    fn subscribe(
        &mut self,
        resource: Resource<ResolveAddressStream>,
    ) -> Result<Resource<DynPollable>> {
        subscribe(self.table, resource)
    }

    fn drop(&mut self, resource: Resource<ResolveAddressStream>) -> Result<()> {
        self.table.delete(resource)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Pollable for ResolveAddressStream {
    async fn ready(&mut self) {
        if let ResolveAddressStream::Waiting(future) = self {
            *self = ResolveAddressStream::Done(future.await.map(|v| v.into_iter()));
        }
    }
}

fn blocking_resolve(host: &url::Host) -> Result<Vec<IpAddress>, SocketError> {
    match host {
        url::Host::Ipv4(v4addr) => Ok(vec![IpAddress::Ipv4(from_ipv4_addr(*v4addr))]),
        url::Host::Ipv6(v6addr) => Ok(vec![IpAddress::Ipv6(from_ipv6_addr(*v6addr))]),
        url::Host::Domain(domain) => {
            // For now use the standard library to perform actual resolution through
            // the usage of the `ToSocketAddrs` trait. This is only
            // resolving names, not ports, so force the port to be 0.
            let addresses = (domain.as_str(), 0)
                .to_socket_addrs()
                .map_err(|_| ErrorCode::NameUnresolvable)? // If/when we use `getaddrinfo` directly, map the error properly.
                .map(|addr| addr.ip().to_canonical().into())
                .collect();

            Ok(addresses)
        }
    }
}
