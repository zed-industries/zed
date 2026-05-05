//! macOS port observer using libproc.
//!
//! macOS is typically the local (developer) machine rather than the remote
//! server, so accuracy is secondary to stability.  We poll at an adaptive
//! rate: 500ms for the first ~10s, then 2s when ports are active, 10s when
//! the set is stable.

use anyhow::Result;
use libproc::libproc::net_info::{TcpSocketState, SocketFDInfo};
use libproc::libproc::proc_pid;
use libproc::processes;
use project::port_store::PortResource;
use std::{
    net::IpAddr,
    sync::Arc,
};

/// dump all currently listening TCP sockets visible to this process.
pub fn dump_listeners() -> Result<Vec<PortResource>> {
    let pids = processes::pids_by_type(processes::ProcFilter::All)
        .map_err(|e| anyhow::anyhow!("pids_by_type: {e}"))?;

    let mut resources = Vec::new();
    for pid in pids {
        if let Ok(fds) = proc_pid::listpidinfo::<proc_pid::ListFDs>(pid as i32, 256) {
            for fd in fds {
                if let Some(resource) = socket_fd_to_resource(pid, fd) {
                    resources.push(resource);
                }
            }
        }
    }

    // deduplicate by id (multiple fds can share a socket)
    resources.sort_by(|a, b| a.id.cmp(&b.id));
    resources.dedup_by(|a, b| a.id == b.id);
    Ok(resources)
}

fn socket_fd_to_resource(pid: u32, fd: SocketFDInfo) -> Option<PortResource> {
    let tcp = fd.tcp_info?;
    if tcp.tcpsi_state != TcpSocketState::LISTEN as u8 {
        return None;
    }

    let port = tcp.tcpsi_ini.insi_lport as u16;
    let addr_bytes = tcp.tcpsi_ini.insi_laddr.ina_46.i46a_addr4;
    let addr: IpAddr = std::net::Ipv4Addr::from(addr_bytes.s_addr.to_be_bytes()).into();

    let proto_str: Arc<str> = "tcp4".into();
    let bind_addr: Arc<str> = addr.to_string().into();
    let id: Arc<str> = format!("{proto_str}:{bind_addr}:{port}").into();
    let exposure = classify_exposure(&addr);

    Some(PortResource {
        id,
        version: 0,
        proto: proto_str,
        bind_addr,
        port: port as u32,
        uid: 0,
        inode: 0,
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
