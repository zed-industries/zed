//! Linux port observer using NETLINK_SOCK_DIAG.
//!
//! Performs a full dump on startup and on ENOBUFS.  Subscribes to
//! SKNLGRP_INET_TCP_DESTROY / SKNLGRP_INET6_TCP_DESTROY for invalidation hints
//! (best-effort; degrades silently to dump-only if subscription fails).

use anyhow::{Context as _, Result};
use netlink_packet_core::{
    NetlinkHeader, NetlinkMessage, NetlinkPayload, NLM_F_DUMP, NLM_F_REQUEST,
};
use netlink_packet_sock_diag::{
    AF_INET, AF_INET6, IPPROTO_TCP, SockDiagMessage,
    inet::{ExtensionFlags, InetRequest, SocketId, StateFlags},
};
use netlink_sys::{AsyncSocket, AsyncSocketExt, SmolSocket, SocketAddr, protocols::NETLINK_SOCK_DIAG};
use project::port_store::PortResource;
use std::{net::IpAddr, sync::Arc};

// Multicast groups for TCP socket destruction events (Linux 4.2+).
// Not re-exported by netlink-sys — declared here to avoid depending on
// linux-raw-sys directly.
// Used by subscribe_destroy_events, which is wired up once multicast
// invalidation is implemented in the driver loop.
#[allow(dead_code)]
const SKNLGRP_INET_TCP_DESTROY: u32 = 1;
#[allow(dead_code)]
const SKNLGRP_INET6_TCP_DESTROY: u32 = 3;

/// dump all currently listening TCP sockets (IPv4 + IPv6).
pub async fn dump_listeners() -> Result<Vec<PortResource>> {
    let mut resources = Vec::new();
    resources.extend(dump_family(AF_INET).await?);
    resources.extend(dump_family(AF_INET6).await?);
    Ok(resources)
}

async fn dump_family(family: u8) -> Result<Vec<PortResource>> {
    let mut socket = SmolSocket::new(NETLINK_SOCK_DIAG)
        .context("open NETLINK_SOCK_DIAG socket")?;

    // bind_auto assigns a port number; connect routes responses back here
    socket.socket_mut().bind_auto().context("bind sock_diag")?;
    socket
        .socket_ref()
        .connect(&SocketAddr::new(0, 0))
        .context("connect sock_diag")?;

    let socket_id = if family == AF_INET {
        SocketId::new_v4()
    } else {
        SocketId::new_v6()
    };

    let mut header = NetlinkHeader::default();
    header.flags = NLM_F_REQUEST | NLM_F_DUMP;

    let mut packet = NetlinkMessage::new(
        header,
        SockDiagMessage::InetRequest(InetRequest {
            family,
            protocol: IPPROTO_TCP,
            extensions: ExtensionFlags::empty(),
            states: StateFlags::LISTEN,
            socket_id,
        })
        .into(),
    );
    packet.finalize();

    let mut buf = vec![0u8; packet.header.length as usize];
    packet.serialize(&mut buf);
    socket.send(&buf).await.context("send sock_diag request")?;

    let mut resources = Vec::new();

    'outer: loop {
        let (recv_buf, _addr) = socket
            .recv_from_full()
            .await
            .context("recv sock_diag")?;
        let mut offset = 0;

        loop {
            let msg_bytes = &recv_buf[offset..];
            if msg_bytes.len() < 4 {
                break;
            }

            let response = NetlinkMessage::<SockDiagMessage>::deserialize(msg_bytes)
                .context("deserialize sock_diag")?;
            let msg_len = response.header.length as usize;
            if msg_len == 0 {
                break;
            }
            offset += msg_len;

            match response.payload {
                NetlinkPayload::InnerMessage(SockDiagMessage::InetResponse(diag)) => {
                    if let Some(resource) = inet_response_to_resource(&diag) {
                        resources.push(resource);
                    }
                }
                NetlinkPayload::Done(_) => break 'outer,
                NetlinkPayload::Error(e) => {
                    return Err(anyhow::anyhow!("sock_diag error: {e}"));
                }
                _ => {}
            }

            if offset >= recv_buf.len() {
                break;
            }
        }
    }

    Ok(resources)
}

fn inet_response_to_resource(
    diag: &netlink_packet_sock_diag::inet::InetResponse,
) -> Option<PortResource> {
    let header = &diag.header;
    let port = header.socket_id.source_port;
    let addr = header.socket_id.source_address;

    let proto_str: Arc<str> = if header.family == AF_INET {
        "tcp4"
    } else {
        "tcp6"
    }
    .into();
    let bind_addr: Arc<str> = addr.to_string().into();
    let id: Arc<str> = format!("{proto_str}:{bind_addr}:{port}").into();
    let exposure = classify_exposure(&addr);

    Some(PortResource {
        id,
        version: 0,
        proto: proto_str,
        bind_addr,
        port: port as u32,
        uid: header.uid,
        inode: header.inode as u64,
        process: "".into(),
        exposure,
    })
}

fn classify_exposure(addr: &IpAddr) -> Arc<str> {
    match addr {
        IpAddr::V4(a) if a.is_loopback() => "loopback".into(),
        IpAddr::V4(a) if a.is_unspecified() => "wildcard".into(),
        IpAddr::V6(a) if a.is_loopback() => "loopback".into(),
        IpAddr::V6(a) if a.is_unspecified() => "wildcard".into(),
        _ => "specific".into(),
    }
}

/// try to subscribe to TCP destruction multicast groups.
///
/// best-effort: callers fall back to dump-only mode if this fails.
/// wired up by the driver loop once multicast invalidation is implemented.
#[allow(dead_code)]
pub fn subscribe_destroy_events(socket: &SmolSocket) -> Result<()> {
    socket
        .socket_ref()
        .add_membership(SKNLGRP_INET_TCP_DESTROY)
        .context("subscribe SKNLGRP_INET_TCP_DESTROY")?;
    socket
        .socket_ref()
        .add_membership(SKNLGRP_INET6_TCP_DESTROY)
        .context("subscribe SKNLGRP_INET6_TCP_DESTROY")?;
    Ok(())
}

// ─── Integration tests ─────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observer_detects_real_listener() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        let resources = smol::block_on(dump_listeners()).unwrap();
        assert!(
            resources.iter().any(|r| r.port == port as u32),
            "expected port {port} in dump, got: {:?}",
            resources.iter().map(|r| r.port).collect::<Vec<_>>()
        );
    }

    #[test]
    fn observer_detects_listener_removal() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        let before = smol::block_on(dump_listeners()).unwrap();
        assert!(before.iter().any(|r| r.port == port as u32));

        drop(listener);

        let after = smol::block_on(dump_listeners()).unwrap();
        assert!(!after.iter().any(|r| r.port == port as u32));
    }
}
