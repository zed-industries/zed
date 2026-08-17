use super::{Connection, Header, Listener, SocketName};
use crate::{Error, LoopAction};
use polling::{Event, Poller};
use std::io::ErrorKind;
use std::time::{Duration, Instant};

/// Server side of the connection, which runs in the monitor process that is
/// meant to monitor the process where the [`super::Client`] resides
pub struct Server {
    listener: Option<Listener>,
    #[cfg(target_os = "macos")]
    port: crash_context::ipc::Server,
    /// For abstract sockets, we don't have to worry about cleanup as it is
    /// handled by the OS, but on Windows and `MacOS` we need to clean them up
    /// manually. We basically rely on the crash monitor program this Server
    /// is running in to exit cleanly, which should be mostly true, but we
    /// may need to harden this code if people experience issues with socket
    /// paths not being cleaned up reliably
    socket_path: Option<std::path::PathBuf>,
}

struct ClientConn {
    /// The actual socket connection we established with accept
    socket: Connection,
    /// The key we associated with the socket
    key: usize,
    /// Last time a message was sent from the client
    last_update: Instant,
    /// We pair the pid of the client process so that we know which connection
    /// to drop when a crash is received on the mach port
    #[cfg(target_os = "macos")]
    pid: Option<u32>,
}

impl ClientConn {
    fn recv(&mut self, handler: &dyn crate::ServerHandler) -> Option<(u32, Vec<u8>)> {
        use std::io::IoSliceMut;

        let mut hdr_buf = [0u8; std::mem::size_of::<Header>()];
        cfg_if::cfg_if! {
            if #[cfg(any(target_os = "linux", target_os = "android"))] {
                let len = self.socket.0.peek(&mut hdr_buf).ok()?;
            } else {
                let len = self.socket.peek(&mut hdr_buf).ok()?;
            }
        }

        if len == 0 {
            return None;
        }

        let header = Header::from_bytes(&hdr_buf)?;

        if header.size == 0 {
            self.socket.recv(&mut hdr_buf).ok()?;
            Some((header.kind, Vec::new()))
        } else {
            let mut buffer = handler.message_alloc();

            buffer.resize(header.size as usize, 0);

            self.socket
                .recv_vectored(&mut [IoSliceMut::new(&mut hdr_buf), IoSliceMut::new(&mut buffer)])
                .ok()?;

            Some((header.kind, buffer))
        }
    }
}

impl Server {
    /// Creates a new server with the given name.
    ///
    /// Note that in the case of a path socket name, this method always attempts
    /// to delete the specified path if it exists as both Windows and Macos have
    /// issues around cleaning up these files if the process the server runs in
    /// aborts abnormally.
    ///
    /// # Errors
    ///
    /// The provided socket name is invalid, or the listener socket was unable
    /// to be bound to the specified socket name.
    pub fn with_name<'scope>(sn: SocketName<'scope>) -> Result<Self, Error> {
        #[allow(irrefutable_let_patterns)]
        let socket_path = if let SocketName::Path(path) = &sn {
            // There seems to be a bug, at least on Windows, where checking for
            // the existence of the file path will actually fail even if the file
            // is actually there, so we just unconditionally remove the path
            let _res = std::fs::remove_file(path);

            Some(std::path::PathBuf::from(path))
        } else {
            None
        };

        cfg_if::cfg_if! {
            if #[cfg(any(target_os = "linux", target_os = "android"))] {
                let socket_addr = match sn {
                    SocketName::Path(path) => {
                        uds::UnixSocketAddr::from_path(path).map_err(|_err| Error::InvalidName)?
                    }
                    SocketName::Abstract(name) => {
                        uds::UnixSocketAddr::from_abstract(name).map_err(|_err| Error::InvalidName)?
                    }
                };

                let listener = Listener(uds::nonblocking::UnixSeqpacketListener::bind_unix_addr(&socket_addr)?);
            } else if #[cfg(target_os = "windows")] {
                let SocketName::Path(path) = sn;
                let listener = Listener::bind(path)?;
                listener.set_nonblocking(true)?;
            } else if #[cfg(target_os = "macos")] {
                let SocketName::Path(path) = sn;
                let listener = Listener::bind(path)?;
                listener.set_nonblocking(true)?;

                // Note that sun_path is limited to 108 characters including null,
                // while a mach port name is limited to 128 including null, so
                // the length is already effectively checked here
                let port_name = std::ffi::CString::new(path.to_str().ok_or(Error::InvalidPortName)?).map_err(|_err| Error::InvalidPortName)?;
                let port = crash_context::ipc::Server::create(&port_name)?;
            } else {
                compile_error!("unimplemented target platform");
            }
        }

        Ok(Self {
            listener: Some(listener),
            #[cfg(target_os = "macos")]
            port,
            socket_path,
        })
    }

    /// Runs the server loop, accepting client connections and receiving IPC
    /// messages.
    ///
    /// If `stale_timeout` is specified, client connections that have not sent
    /// a message within that period will be shutdown and removed, to prevent
    /// potential issues with the server process from indefinitely outlasting
    /// the process(es) it was monitoring for crashes, in cases where the OS
    /// (read, Windows) might take longer than one would want to properly reap
    /// the client connections in the event of adrupt process termination.
    /// Sending messages will prevent the connection from going stale, but if
    /// messages are not guaranteed to be sent at a higher frequency than your
    /// specified timeout, you can use [`crate::Client::ping`] to fill in any
    /// message gaps to indicate the client is still alive.
    ///
    /// # Errors
    ///
    /// This method uses basic I/O event notification via [`polling`] which
    /// can fail for a number of different reasons
    #[allow(unsafe_code)]
    pub fn run(
        &mut self,
        handler: Box<dyn crate::ServerHandler>,
        shutdown: &std::sync::atomic::AtomicBool,
        stale_timeout: Option<std::time::Duration>,
    ) -> Result<(), Error> {
        let mut events = polling::Events::new();
        let listener = self.listener.take().unwrap();

        struct Poll {
            listener: Listener,
            clients: Vec<ClientConn>,
            poll: Poller,
        }

        impl Poll {
            fn new(listener: Listener) -> std::io::Result<Self> {
                let s = Self {
                    listener,
                    poll: Poller::new()?,
                    clients: Vec::new(),
                };

                // SAFETY: We ensure we delete the listener during drop
                unsafe {
                    s.poll.add(&s.listener, Event::readable(0))?;
                }

                Ok(s)
            }

            #[inline]
            fn add(
                &mut self,
                src: impl polling::AsRawSource,
                interest: Event,
            ) -> std::io::Result<()> {
                // SAFETY: We ensure we delete all sources we add before dropping the poll
                unsafe { self.poll.add(src, interest) }
            }
        }

        impl Drop for Poll {
            fn drop(&mut self) {
                for client in std::mem::take(&mut self.clients) {
                    if let Err(err) = self.poll.delete(client.socket) {
                        log::error!("failed to deregister socket: {err}");
                    }
                }

                if let Err(err) = self.poll.delete(&self.listener) {
                    log::error!("failed to deregister listener: {err}");
                }
            }
        }

        let mut polling = Poll::new(listener)?;
        let mut id = 1;

        loop {
            if shutdown.load(std::sync::atomic::Ordering::Relaxed) {
                return Ok(());
            }

            events.clear();
            let timeout = Duration::from_millis(10);
            let deadline = Instant::now() + timeout;
            let mut remaining = Some(timeout);
            while let Some(timeout) = remaining {
                match polling.poll.wait(&mut events, Some(timeout)) {
                    Ok(_) => {
                        break;
                    }
                    Err(e) => {
                        if matches!(e.kind(), ErrorKind::Interrupted) {
                            remaining = deadline.checked_duration_since(Instant::now());
                        } else {
                            return Err(e.into());
                        }
                    }
                }
            }

            #[cfg(target_os = "macos")]
            if self.check_mach_port(&polling.poll, &mut polling.clients, handler.as_ref())?
                == LoopAction::Exit
            {
                return Ok(());
            }

            for event in events.iter() {
                if event.key == 0 {
                    match polling.listener.accept_unix_addr() {
                        Ok((accepted, _addr)) => {
                            let key = id;
                            id += 1;

                            polling.add(&accepted, Event::readable(key))?;

                            log::debug!("accepted connection {key}");
                            polling.clients.push(ClientConn {
                                socket: accepted,
                                key,
                                last_update: Instant::now(),
                                #[cfg(target_os = "macos")]
                                pid: None,
                            });

                            if handler.on_client_connected(polling.clients.len())
                                == LoopAction::Exit
                            {
                                log::debug!("on_client_connected exited message loop");
                                return Ok(());
                            }
                        }
                        Err(err) => {
                            log::error!("failed to accept socket connection: {err}");
                        }
                    }

                    // We need to reregister insterest every time
                    polling.poll.modify(&polling.listener, Event::readable(0))?;
                } else if let Some(pos) = polling.clients.iter().position(|cc| cc.key == event.key)
                {
                    polling.clients[pos].last_update = Instant::now();

                    let deregister = match polling.clients[pos].recv(handler.as_ref()) {
                        Some((super::CRASH, buffer)) => {
                            cfg_if::cfg_if! {
                                if #[cfg(target_os = "macos")] {
                                    use scroll::Pread;
                                    let pid: u32 = buffer.pread(0)?;
                                    polling.clients[pos].pid = Some(pid);

                                    if let Err(err) = polling.clients[pos].socket.send(&[1]) {
                                        log::error!("failed to send ack: {err}");
                                    }

                                    None
                                } else {
                                    let cc = polling.clients.swap_remove(pos);

                                    cfg_if::cfg_if! {
                                        if #[cfg(any(target_os = "linux", target_os = "android"))] {
                                            let peer_creds = cc.socket.0.initial_peer_credentials()?;

                                            let pid = peer_creds.pid().ok_or(Error::UnknownClientPid)?;

                                            let crash_ctx = crash_context::CrashContext::from_bytes(&buffer).ok_or_else(|| {
                                                Error::from(std::io::Error::new(
                                                    std::io::ErrorKind::InvalidData,
                                                    "client sent an incorrectly sized buffer",
                                                ))
                                            })?;

                                            // Validate that the crash info and the socket agree on the pid
                                            if pid.get() != crash_ctx.pid as u32 {
                                                return Err(Error::UnknownClientPid);
                                            }
                                        } else if #[cfg(target_os = "windows")] {
                                            use scroll::Pread;
                                            let dump_request: super::DumpRequest = buffer.pread(0)?;

                                            // MiniDumpWriteDump primarily uses `EXCEPTION_POINTERS` for its crash
                                            // context information, but inside that is an `EXCEPTION_RECORD`, which
                                            // is an internally linked list, so rather than recurse and allocate until
                                            // the end of that linked list, we just retrieve the actual pointer from
                                            // the client process, and inform the dump writer that they are pointers
                                            // to a different process, as MiniDumpWriteDump will internally read
                                            // the processes memory as needed
                                            let exception_pointers = dump_request.exception_pointers as *const crash_context::EXCEPTION_POINTERS;

                                            let crash_ctx = crash_context::CrashContext {
                                                exception_pointers,
                                                process_id: dump_request.process_id,
                                                thread_id: dump_request.thread_id,
                                                exception_code: dump_request.exception_code,
                                            };
                                        }
                                    }

                                    let action =
                                        match Self::handle_crash_request(crash_ctx, handler.as_ref()) {
                                            Err(err) => {
                                                log::error!("failed to capture minidump: {err}");
                                                LoopAction::Continue
                                            }
                                            Ok(action) => {
                                                log::info!("captured minidump");
                                                action
                                            }
                                        };

                                    let ack = Header {
                                        kind: super::CRASH_ACK,
                                        size: 0,
                                    };

                                    if let Err(err) = cc.socket.send(ack.as_bytes()) {
                                        log::error!("failed to send ack: {err}");
                                    }

                                    if action == LoopAction::Exit {
                                        log::debug!("user handler requested exit after minidump creation");
                                        return Ok(());
                                    }

                                    Some(cc.socket)
                                }
                            }
                        }
                        Some((super::PING, _buffer)) => {
                            let pong = Header {
                                kind: super::PONG,
                                size: 0,
                            };

                            if let Err(err) = polling.clients[pos].socket.send(pong.as_bytes()) {
                                log::error!("failed to send PONG: {err}");

                                let cc = polling.clients.swap_remove(pos);
                                Some(cc.socket)
                            } else {
                                None
                            }
                        }
                        Some((super::PONG, _buffer)) => None,
                        Some((kind, buffer)) => {
                            handler.on_message(
                                kind - super::USER, /* give the user back the original code they specified */
                                buffer,
                            );

                            // We only send acks for crash dump requests
                            // if let Err(e) = clients[pos].socket.send(&[1]) {
                            //     log::error!("failed to send ack: {}", e);
                            // }

                            None
                        }
                        None => {
                            log::debug!("client closed socket {pos}");
                            let cc = polling.clients.swap_remove(pos);
                            Some(cc.socket)
                        }
                    };

                    if let Some(socket) = deregister {
                        if let Err(err) = polling.poll.delete(&socket) {
                            log::error!("failed to deregister socket: {err}");
                        }

                        if handler.on_client_disconnected(polling.clients.len()) == LoopAction::Exit
                        {
                            log::debug!("on_client_disconnected exited message loop");
                            return Ok(());
                        }
                    } else {
                        let conn = &polling.clients[pos];
                        polling
                            .poll
                            .modify(&conn.socket, Event::readable(conn.key))?;
                    }
                }
            }

            if let Some(st) = stale_timeout {
                let before = polling.clients.len();

                // Reap any connections that haven't sent a message in the period
                // specified by the user
                polling.clients.retain(|conn| {
                    let keep = conn.last_update.elapsed() < st;

                    if !keep {
                        log::debug!("dropping stale connection {:?}", conn.last_update.elapsed());
                        if let Err(err) = polling.poll.delete(&conn.socket) {
                            log::error!("failed to deregister timed-out socket: {err}");
                        }
                    }

                    keep
                });

                if before > polling.clients.len()
                    && handler.on_client_disconnected(polling.clients.len()) == LoopAction::Exit
                {
                    log::debug!("on_client_disconnected exited message loop");
                    return Ok(());
                }
            }
        }
    }

    fn handle_crash_request(
        crash_context: crash_context::CrashContext,
        handler: &dyn crate::ServerHandler,
    ) -> Result<LoopAction, Error> {
        let (mut minidump_file, minidump_path) = handler.create_minidump_file()?;

        cfg_if::cfg_if! {
            if #[cfg(any(target_os = "linux", target_os = "android"))] {
                #[allow(unsafe_code)]
                let crash_context = minidump_writer::crash_context::CrashContext {
                    inner: unsafe { std::mem::transmute_copy(&crash_context) },
                };

                let mut writer =
                    minidump_writer::minidump_writer::MinidumpWriterConfig::new(crash_context.inner.pid, crash_context.inner.tid);

                writer.set_crash_context(crash_context);
                let result = writer.write(&mut minidump_file);
            } else if #[cfg(target_os = "windows")] {
                // SAFETY: Unfortunately this is a bit dangerous since we are relying on the crashing process
                // to still be alive and still have the interior pointers in the crash context still at the
                // same location in memory, unfortunately it's a bit hard to communicate this through so
                // many layers, so really, we are falling back on Windows to actually correctly handle
                // if the interior pointers have become invalid which it should? do ok with
                let result =
                    minidump_writer::minidump_writer::MinidumpWriter::dump_crash_context(&crash_context, None, &mut minidump_file);
            } else if #[cfg(target_os = "macos")] {
                let mut writer = minidump_writer::minidump_writer::MinidumpWriter::with_crash_context(crash_context);
                let result = writer.dump(&mut minidump_file);
            }
        }

        // Notify the user handler about the minidump, even if we failed to write it
        Ok(handler.on_minidump_created(
            result
                .map(|_contents| crate::MinidumpBinary {
                    file: minidump_file,
                    path: minidump_path,
                    #[cfg(target_os = "windows")]
                    contents: None,
                    #[cfg(not(target_os = "windows"))]
                    contents: Some(_contents),
                })
                .map_err(crate::Error::from),
        ))
    }

    #[cfg(target_os = "macos")]
    fn check_mach_port(
        &mut self,
        poll: &Poller,
        clients: &mut Vec<ClientConn>,
        handler: &dyn crate::ServerHandler,
    ) -> Result<LoopAction, Error> {
        // We use a really short timeout for receiving on the mach port since we check it
        // frequently rather than spawning a separate thread and blocking
        if let Some(mut rcc) = self
            .port
            .try_recv_crash_context(Some(Duration::from_millis(1)))?
        {
            // Try to find a client connection that matches the port sender
            let pos = clients
                .iter()
                .position(|cc| cc.pid == Some(rcc.pid))
                .ok_or(Error::UnknownClientPid)?;
            let cc = clients.swap_remove(pos);

            let action = match Self::handle_crash_request(rcc.crash_context, handler) {
                Err(err) => {
                    log::error!("failed to capture minidump: {err}");
                    LoopAction::Continue
                }
                Ok(action) => {
                    log::info!("captured minidump");
                    action
                }
            };

            if let Err(err) = rcc.acker.send_ack(1, Some(Duration::from_secs(2))) {
                log::error!("failed to send ack: {err}");
            }

            if let Err(err) = poll.delete(&cc.socket) {
                log::error!("failed to deregister socket: {err}");
            }

            Ok(action)
        } else {
            Ok(LoopAction::Continue)
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.listener.take();

        if let Some(path) = self.socket_path.take() {
            // Note we don't check for the existence of the path since there
            // appears to be a bug on MacOS and Windows, or at least an oversight
            // in std, where checking the existence of the path always fails
            let _res = std::fs::remove_file(path);
        }
    }
}
