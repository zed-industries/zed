use std::{
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    thread,
    time::Duration,
};

use sysinfo::System;

use release_channel::ReleaseChannel;

const LOCALHOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const CONNECT_TIMEOUT: Duration = Duration::from_millis(10);
const RECEIVE_TIMEOUT: Duration = Duration::from_millis(35);
const SEND_TIMEOUT: Duration = Duration::from_millis(20);
const USER_BLOCK: u16 = 100;

fn address() -> SocketAddr {
    // These port numbers are offset by the user ID to avoid conflicts between
    // different users on the same machine. In addition to that the ports for each
    // release channel are spaced out by 100 to avoid conflicts between different
    // users running different release channels on the same machine. This ends up
    // interleaving the ports between different users and different release channels.
    //
    // On macOS user IDs start at 501 and on Linux they start at 1000. The first user
    // on a Mac with ID 501 running a dev channel build will use port 44238, and the
    // second user with ID 502 will use port 44239, and so on. User 501 will use ports
    // 44338, 44438, and 44538 for the preview, stable, and nightly channels,
    // respectively. User 502 will use ports 44339, 44439, and 44539 for the preview,
    // stable, and nightly channels, respectively.
    let port = match *release_channel::RELEASE_CHANNEL {
        ReleaseChannel::Dev => 43737,
        ReleaseChannel::Preview => 43737 + USER_BLOCK,
        ReleaseChannel::Stable => 43737 + (2 * USER_BLOCK),
        ReleaseChannel::Nightly => 43737 + (3 * USER_BLOCK),
    };
    let mut user_port = port;
    let mut sys = System::new_all();
    sys.refresh_all();
    if let Ok(current_pid) = sysinfo::get_current_pid()
        && let Some(uid) = sys
            .process(current_pid)
            .and_then(|process| process.user_id())
    {
        let uid_u32 = get_uid_as_u32(uid);
        // Ensure that the user ID is not too large to avoid overflow when
        // calculating the port number. This seems unlikely but it doesn't
        // hurt to be safe.
        let max_port = 65535;
        let max_uid: u32 = max_port - port as u32;
        let wrapped_uid: u16 = (uid_u32 % max_uid) as u16;
        user_port += wrapped_uid;
    }

    SocketAddr::V4(SocketAddrV4::new(LOCALHOST, user_port))
}

#[cfg(unix)]
fn get_uid_as_u32(uid: &sysinfo::Uid) -> u32 {
    *uid.clone()
}

#[cfg(windows)]
fn get_uid_as_u32(uid: &sysinfo::Uid) -> u32 {
    // Extract the RID which is an integer
    uid.to_string()
        .rsplit('-')
        .next()
        .and_then(|rid| rid.parse::<u32>().ok())
        .unwrap_or(0)
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
