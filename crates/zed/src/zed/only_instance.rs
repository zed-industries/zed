use std::{
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    thread,
    time::Duration,
};

use release_channel::ReleaseChannel;

const LOCALHOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const CONNECT_TIMEOUT: Duration = Duration::from_millis(10);
const RECEIVE_TIMEOUT: Duration = Duration::from_millis(35);
const SEND_TIMEOUT: Duration = Duration::from_millis(20);

fn address() -> SocketAddr {
    let port = match *release_channel::RELEASE_CHANNEL {
        ReleaseChannel::Dev => 43737,
        ReleaseChannel::Preview => 43738,
        ReleaseChannel::Stable => 43739,
        ReleaseChannel::Nightly => 43740,
    };

    SocketAddr::V4(SocketAddrV4::new(LOCALHOST, port))
}

fn instance_handshake() -> &'static str {
    match *release_channel::RELEASE_CHANNEL {
        ReleaseChannel::Dev => "Zed Editor Dev Instance Running",
        ReleaseChannel::Nightly => "Zed Editor Nightly Instance Running",
        ReleaseChannel::Preview => "Zed Editor Preview Instance Running",
        ReleaseChannel::Stable => "Zed Editor Stable Instance Running",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsOnlyInstance {
    Yes,
    No,
}

pub fn ensure_only_instance() -> IsOnlyInstance {
    if *db::ZED_STATELESS || *release_channel::RELEASE_CHANNEL == ReleaseChannel::Dev {
        return IsOnlyInstance::Yes;
    }

    if check_got_handshake() {
        return IsOnlyInstance::No;
    }

    let listener = match TcpListener::bind(address()) {
        Ok(listener) => listener,

        Err(err) => {
            log::warn!("Error binding to single instance port: {err}");
            if check_got_handshake() {
                return IsOnlyInstance::No;
            }

            // Avoid failing to start when some other application by chance already has
            // a claim on the port. This is sub-par as any other instance that gets launched
            // will be unable to communicate with this instance and will duplicate
            log::warn!("Backup handshake request failed, continuing without handshake");
            return IsOnlyInstance::Yes;
        }
    };

    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = match stream {
                Ok(stream) => stream,
                Err(_) => return,
            };

            _ = stream.set_nodelay(true);
            _ = stream.set_read_timeout(Some(SEND_TIMEOUT));
            _ = stream.write_all(instance_handshake().as_bytes());
        }
    });

    IsOnlyInstance::Yes
}

fn check_got_handshake() -> bool {
    match TcpStream::connect_timeout(&address(), CONNECT_TIMEOUT) {
        Ok(mut stream) => {
            let mut buf = vec![0u8; instance_handshake().len()];

            stream.set_read_timeout(Some(RECEIVE_TIMEOUT)).unwrap();
            if let Err(err) = stream.read_exact(&mut buf) {
                log::warn!("Connected to single instance port but failed to read: {err}");
                return false;
            }

            if buf == instance_handshake().as_bytes() {
                log::info!("Got instance handshake");
                return true;
            }

            log::warn!("Got wrong instance handshake value");
            false
        }

        Err(_) => false,
    }
}
