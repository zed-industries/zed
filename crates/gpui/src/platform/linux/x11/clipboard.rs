/*
 * Copyright 2022 - 2025 Zed Industries, Inc.
 * License: Apache-2.0
 * See LICENSE-APACHE for complete license terms
 *
 * Adapted from x11-clipboard https://github.com/quininer/x11-clipboard
 *
 * MIT License
 * Copyright (c) 2017 quininer@live.com
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::cmp;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::os::fd::{AsFd, AsRawFd, FromRawFd, OwnedFd};
use std::sync::mpsc::SendError;
use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};
use x11rb::connection::{Connection, RequestConnection};
use x11rb::errors::{ConnectError, ConnectionError, ReplyError, ReplyOrIdError};
use x11rb::protocol::xfixes::SELECTION_NOTIFY_EVENT;
pub use x11rb::protocol::xproto::{Atom, SelectionNotifyEvent, Window};
use x11rb::protocol::xproto::{
    AtomEnum, ChangeWindowAttributesAux, ConnectionExt, CreateWindowAux, EventMask, PropMode,
    Property, WindowClass,
};
use x11rb::protocol::{Event, xfixes};
use x11rb::rust_connection::RustConnection;
use x11rb::{COPY_DEPTH_FROM_PARENT, CURRENT_TIME};

const INCR_CHUNK_SIZE: usize = 4000;
const POLL_DURATION: u64 = 50;

type SetMap = Arc<RwLock<HashMap<Atom, (Atom, Vec<u8>)>>>;

#[derive(Clone, Debug)]
pub struct Atoms {
    pub primary: Atom,
    pub clipboard: Atom,
    pub property: Atom,
    pub targets: Atom,
    pub string: Atom,
    pub utf8_string: Atom,
    pub incr: Atom,
}

impl Atoms {
    fn intern_all(conn: &RustConnection) -> Result<Atoms, Error> {
        let clipboard = conn.intern_atom(false, b"CLIPBOARD")?;
        let property = conn.intern_atom(false, b"THIS_CLIPBOARD_OUT")?;
        let targets = conn.intern_atom(false, b"TARGETS")?;
        let utf8_string = conn.intern_atom(false, b"UTF8_STRING")?;
        let incr = conn.intern_atom(false, b"INCR")?;
        Ok(Atoms {
            primary: Atom::from(AtomEnum::PRIMARY),
            clipboard: clipboard.reply()?.atom,
            property: property.reply()?.atom,
            targets: targets.reply()?.atom,
            string: Atom::from(AtomEnum::STRING),
            utf8_string: utf8_string.reply()?.atom,
            incr: incr.reply()?.atom,
        })
    }
}

/// X11 Clipboard
pub struct Clipboard {
    pub getter: Context,
    pub setter: Arc<Context>,
    setmap: SetMap,
    send: Sender<Atom>,
    // Relying on the Drop in OwnedFd to close the fd
    _drop_fd: OwnedFd,
}

pub struct Context {
    pub connection: RustConnection,
    pub screen: usize,
    pub window: Window,
    pub atoms: Atoms,
}

#[inline]
fn get_atom(connection: &RustConnection, name: &str) -> Result<Atom, Error> {
    let intern_atom = connection.intern_atom(false, name.as_bytes())?;
    let reply = intern_atom.reply().map_err(Error::XcbReply)?;
    Ok(reply.atom)
}

impl Context {
    pub fn new(displayname: Option<&str>) -> Result<Self, Error> {
        let (connection, screen) = RustConnection::connect(displayname)?;
        let window = connection.generate_id()?;

        {
            let screen = connection
                .setup()
                .roots
                .get(screen)
                .ok_or(Error::XcbConnect(ConnectError::InvalidScreen))?;
            connection
                .create_window(
                    COPY_DEPTH_FROM_PARENT,
                    window,
                    screen.root,
                    0,
                    0,
                    1,
                    1,
                    0,
                    WindowClass::INPUT_OUTPUT,
                    screen.root_visual,
                    &CreateWindowAux::new()
                        .event_mask(EventMask::STRUCTURE_NOTIFY | EventMask::PROPERTY_CHANGE),
                )?
                .check()?;
        }

        let atoms = Atoms::intern_all(&connection)?;

        Ok(Context {
            connection,
            screen,
            window,
            atoms,
        })
    }

    pub fn get_atom(&self, name: &str) -> Result<Atom, Error> {
        get_atom(&self.connection, name)
    }
}

impl Clipboard {
    /// Create Clipboard.
    pub fn new() -> Result<Self, Error> {
        let getter = Context::new(None)?;
        let setter = Arc::new(Context::new(None)?);
        let setter2 = Arc::clone(&setter);
        let setmap = Arc::new(RwLock::new(HashMap::new()));
        let setmap2 = Arc::clone(&setmap);

        let PipeDropFds {
            read_pipe,
            write_pipe,
        } = create_pipe_drop_fd()?;
        let (sender, receiver) = channel();
        let max_length = setter.connection.maximum_request_bytes();
        thread::spawn(move || run(setter2, setmap2, max_length, receiver, read_pipe));

        Ok(Clipboard {
            getter,
            setter,
            setmap,
            send: sender,
            _drop_fd: write_pipe,
        })
    }

    fn process_event<T>(
        &self,
        buff: &mut Vec<u8>,
        selection: Atom,
        target: Atom,
        property: Atom,
        timeout: T,
        use_xfixes: bool,
        sequence_number: u64,
    ) -> Result<(), Error>
    where
        T: Into<Option<Duration>>,
    {
        let mut is_incr = false;
        let timeout = timeout.into();
        let start_time = if timeout.is_some() {
            Some(Instant::now())
        } else {
            None
        };

        loop {
            if timeout
                .into_iter()
                .zip(start_time)
                .next()
                .map(|(timeout, time)| (Instant::now() - time) >= timeout)
                .unwrap_or(false)
            {
                return Err(Error::Timeout);
            }

            let (event, seq) = match use_xfixes {
                true => self.getter.connection.wait_for_event_with_sequence()?,
                false => match self.getter.connection.poll_for_event_with_sequence()? {
                    Some(event) => event,
                    None => {
                        thread::park_timeout(Duration::from_millis(POLL_DURATION));
                        continue;
                    }
                },
            };

            if seq < sequence_number {
                continue;
            }

            match event {
                Event::XfixesSelectionNotify(event) if use_xfixes => {
                    self.getter
                        .connection
                        .convert_selection(
                            self.getter.window,
                            selection,
                            target,
                            property,
                            event.timestamp,
                        )?
                        .check()?;
                }
                Event::SelectionNotify(event) => {
                    if event.selection != selection {
                        continue;
                    };

                    // Note that setting the property argument to None indicates that the
                    // conversion requested could not be made.
                    if event.property == Atom::from(AtomEnum::NONE) {
                        break;
                    }

                    let reply = self
                        .getter
                        .connection
                        .get_property(
                            false,
                            self.getter.window,
                            event.property,
                            AtomEnum::NONE,
                            buff.len() as u32,
                            u32::MAX,
                        )?
                        .reply()?;

                    if reply.type_ == self.getter.atoms.incr {
                        if let Some(mut value) = reply.value32() {
                            if let Some(size) = value.next() {
                                buff.reserve(size as usize);
                            }
                        }
                        self.getter
                            .connection
                            .delete_property(self.getter.window, property)?
                            .check()?;
                        is_incr = true;
                        continue;
                    } else if reply.type_ != target {
                        return Err(Error::UnexpectedType(reply.type_));
                    }

                    buff.extend_from_slice(&reply.value);
                    break;
                }

                Event::PropertyNotify(event) if is_incr => {
                    if event.state != Property::NEW_VALUE {
                        continue;
                    };

                    let cookie = self.getter.connection.get_property(
                        false,
                        self.getter.window,
                        property,
                        AtomEnum::NONE,
                        0,
                        0,
                    )?;

                    let length = cookie.reply()?.bytes_after;

                    let cookie = self.getter.connection.get_property(
                        true,
                        self.getter.window,
                        property,
                        AtomEnum::NONE,
                        0,
                        length,
                    )?;
                    let reply = cookie.reply()?;
                    if reply.type_ != target {
                        continue;
                    };

                    let value = reply.value;

                    if !value.is_empty() {
                        buff.extend_from_slice(&value);
                    } else {
                        break;
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }

    /// load value.
    pub fn load<T>(
        &self,
        selection: Atom,
        target: Atom,
        property: Atom,
        timeout: T,
    ) -> Result<Vec<u8>, Error>
    where
        T: Into<Option<Duration>>,
    {
        let mut buff = Vec::new();
        let timeout = timeout.into();

        let cookie = self.getter.connection.convert_selection(
            self.getter.window,
            selection,
            target,
            property,
            CURRENT_TIME,
            // FIXME ^
            // Clients should not use CurrentTime for the time argument of a ConvertSelection request.
            // Instead, they should use the timestamp of the event that caused the request to be made.
        )?;

        let sequence_number = cookie.sequence_number();
        cookie.check()?;

        self.process_event(
            &mut buff,
            selection,
            target,
            property,
            timeout,
            false,
            sequence_number,
        )?;

        self.getter
            .connection
            .delete_property(self.getter.window, property)?
            .check()?;

        Ok(buff)
    }

    /// wait for a new value and load it
    pub fn load_wait(
        &self,
        selection: Atom,
        target: Atom,
        property: Atom,
    ) -> Result<Vec<u8>, Error> {
        let mut buff = Vec::new();

        let screen = &self
            .getter
            .connection
            .setup()
            .roots
            .get(self.getter.screen)
            .ok_or(Error::XcbConnect(ConnectError::InvalidScreen))?;

        xfixes::query_version(&self.getter.connection, 5, 0)?;
        // Clear selection sources...
        xfixes::select_selection_input(
            &self.getter.connection,
            screen.root,
            self.getter.atoms.primary,
            xfixes::SelectionEventMask::default(),
        )?;
        xfixes::select_selection_input(
            &self.getter.connection,
            screen.root,
            self.getter.atoms.clipboard,
            xfixes::SelectionEventMask::default(),
        )?;
        // ...and set the one requested now
        let cookie = xfixes::select_selection_input(
            &self.getter.connection,
            screen.root,
            selection,
            xfixes::SelectionEventMask::SET_SELECTION_OWNER
                | xfixes::SelectionEventMask::SELECTION_CLIENT_CLOSE
                | xfixes::SelectionEventMask::SELECTION_WINDOW_DESTROY,
        )?;

        let sequence_number = cookie.sequence_number();
        cookie.check()?;

        self.process_event(
            &mut buff,
            selection,
            target,
            property,
            None,
            true,
            sequence_number,
        )?;

        self.getter
            .connection
            .delete_property(self.getter.window, property)?
            .check()?;

        Ok(buff)
    }

    /// store value.
    pub fn store<T: Into<Vec<u8>>>(
        &self,
        selection: Atom,
        target: Atom,
        value: T,
    ) -> Result<(), Error> {
        self.send.send(selection)?;
        self.setmap
            .write()
            .map_err(|_| Error::Lock)?
            .insert(selection, (target, value.into()));

        self.setter
            .connection
            .set_selection_owner(self.setter.window, selection, CURRENT_TIME)?
            .check()?;

        if self
            .setter
            .connection
            .get_selection_owner(selection)?
            .reply()
            .map(|reply| reply.owner == self.setter.window)
            .unwrap_or(false)
        {
            Ok(())
        } else {
            Err(Error::Owner)
        }
    }
}

macro_rules! try_continue {
    ( $expr:expr ) => {
        match $expr {
            Some(val) => val,
            None => continue,
        }
    };
}

struct IncrState {
    selection: Atom,
    requestor: Window,
    property: Atom,
    pos: usize,
}

pub(crate) struct PipeDropFds {
    pub(crate) read_pipe: OwnedFd,
    pub(crate) write_pipe: OwnedFd,
}

pub(crate) fn create_pipe_drop_fd() -> Result<PipeDropFds, Error> {
    let pipe_drop_fds = unsafe {
        // Docs Linux: https://man7.org/linux/man-pages/man2/pipe.2.html
        // Posix: https://pubs.opengroup.org/onlinepubs/9699919799/
        // Safety: See above docs, api expects a 2-long array of file descriptors, and flags
        let mut pipes: [libc::c_int; 2] = [0, 0];
        let pipe_create_res = libc::pipe2(pipes.as_mut_ptr(), libc::O_CLOEXEC);
        if pipe_create_res < 0 {
            // Don't want to have to read from errno_location, just skip propagating errno.
            return Err(Error::EventFdCreate);
        }
        // Safety: Trusting the OS to give correct FDs
        let read_pipe = OwnedFd::from_raw_fd(pipes[0]);
        let write_pipe = OwnedFd::from_raw_fd(pipes[1]);
        PipeDropFds {
            read_pipe,
            write_pipe,
        }
    };
    Ok(pipe_drop_fds)
}

pub(crate) fn run(
    context: Arc<Context>,
    setmap: SetMap,
    max_length: usize,
    receiver: Receiver<Atom>,
    read_pipe: OwnedFd,
) {
    let mut incr_map = HashMap::<Atom, Atom>::new();
    let mut state_map = HashMap::<Atom, IncrState>::new();

    let stream_fd = context.connection.stream().as_fd();
    let borrowed_fd = read_pipe.as_fd();
    // Poll stream for new Read-ready events, check if the other side of the pipe has been dropped
    let mut pollfds: [libc::pollfd; 2] = [
        libc::pollfd {
            fd: stream_fd.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        },
        libc::pollfd {
            fd: borrowed_fd.as_raw_fd(),
            // If the other end is dropped, this pipe will get a HUP on poll
            events: libc::POLLHUP,
            revents: 0,
        },
    ];
    let len = pollfds.len();
    loop {
        unsafe {
            // Docs Linux: https://man7.org/linux/man-pages/man2/poll.2.html
            // Posix: https://pubs.opengroup.org/onlinepubs/9699919799/
            // Safety: Passing in a mutable pointer that lives for the duration of the call, the length is
            // set to the length of that pointer.
            // Any negative value (-1 for example) means infinite timeout.
            let poll_res = libc::poll(&mut pollfds as *mut libc::pollfd, len as libc::nfds_t, -1);
            if poll_res < 0 {
                // Error polling, can't continue
                return;
            }
        }
        if pollfds[1].revents & libc::POLLHUP != 0 {
            // kill-signal on pollfd
            return;
        }
        loop {
            let evt = if let Ok(evt) = context.connection.poll_for_event() {
                evt
            } else {
                // Connection died, exit
                return;
            };
            let event = if let Some(evt) = evt {
                evt
            } else {
                // No event on POLLIN happens, fd being readable doesn't mean there's a complete event ready to read.
                // Poll again.
                break;
            };
            loop {
                match receiver.try_recv() {
                    Ok(selection) => {
                        if let Some(property) = incr_map.remove(&selection) {
                            state_map.remove(&property);
                        }
                    }
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        if state_map.is_empty() {
                            return;
                        }
                    }
                }
            }

            match event {
                Event::SelectionRequest(event) => {
                    let read_map = try_continue!(setmap.read().ok());
                    let &(target, ref value) = try_continue!(read_map.get(&event.selection));

                    if event.target == context.atoms.targets {
                        let _ = x11rb::wrapper::ConnectionExt::change_property32(
                            &context.connection,
                            PropMode::REPLACE,
                            event.requestor,
                            event.property,
                            Atom::from(AtomEnum::ATOM),
                            &[context.atoms.targets, target],
                        );
                    } else if value.len() < max_length - 24 {
                        let _ = x11rb::wrapper::ConnectionExt::change_property8(
                            &context.connection,
                            PropMode::REPLACE,
                            event.requestor,
                            event.property,
                            target,
                            value,
                        );
                    } else {
                        let _ = context.connection.change_window_attributes(
                            event.requestor,
                            &ChangeWindowAttributesAux::new()
                                .event_mask(EventMask::PROPERTY_CHANGE),
                        );
                        let _ = x11rb::wrapper::ConnectionExt::change_property32(
                            &context.connection,
                            PropMode::REPLACE,
                            event.requestor,
                            event.property,
                            context.atoms.incr,
                            &[0u32; 0],
                        );
                        incr_map.insert(event.selection, event.property);
                        state_map.insert(
                            event.property,
                            IncrState {
                                selection: event.selection,
                                requestor: event.requestor,
                                property: event.property,
                                pos: 0,
                            },
                        );
                    }
                    let _ = context.connection.send_event(
                        false,
                        event.requestor,
                        EventMask::default(),
                        SelectionNotifyEvent {
                            response_type: SELECTION_NOTIFY_EVENT,
                            sequence: 0,
                            time: event.time,
                            requestor: event.requestor,
                            selection: event.selection,
                            target: event.target,
                            property: event.property,
                        },
                    );
                    let _ = context.connection.flush();
                }
                Event::PropertyNotify(event) => {
                    if event.state != Property::DELETE {
                        continue;
                    };

                    let is_end = {
                        let state = try_continue!(state_map.get_mut(&event.atom));
                        let read_setmap = try_continue!(setmap.read().ok());
                        let &(target, ref value) = try_continue!(read_setmap.get(&state.selection));

                        let len = cmp::min(INCR_CHUNK_SIZE, value.len() - state.pos);
                        let _ = x11rb::wrapper::ConnectionExt::change_property8(
                            &context.connection,
                            PropMode::REPLACE,
                            state.requestor,
                            state.property,
                            target,
                            &value[state.pos..][..len],
                        );
                        state.pos += len;
                        len == 0
                    };

                    if is_end {
                        state_map.remove(&event.atom);
                    }
                    let _ = context.connection.flush();
                }
                Event::SelectionClear(event) => {
                    if let Some(property) = incr_map.remove(&event.selection) {
                        state_map.remove(&property);
                    }
                    if let Ok(mut write_setmap) = setmap.write() {
                        write_setmap.remove(&event.selection);
                    }
                }
                _ => (),
            }
        }
    }
}

#[must_use]
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Set(SendError<Atom>),
    XcbConnect(ConnectError),
    XcbConnection(ConnectionError),
    XcbReplyOrId(ReplyOrIdError),
    XcbReply(ReplyError),
    Lock,
    Timeout,
    Owner,
    UnexpectedType(Atom),
    // Could change name on next major, since this uses pipes now.
    EventFdCreate,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::Error::*;
        match self {
            Set(e) => write!(f, "XCB - couldn't set atom: {:?}", e),
            XcbConnect(e) => write!(f, "XCB - couldn't establish conection: {:?}", e),
            XcbConnection(e) => write!(f, "XCB connection error: {:?}", e),
            XcbReplyOrId(e) => write!(f, "XCB reply error: {:?}", e),
            XcbReply(e) => write!(f, "XCB reply error: {:?}", e),
            Lock => write!(f, "XCB: Lock is poisoned"),
            Timeout => write!(f, "Selection timed out"),
            Owner => write!(f, "Failed to set new owner of XCB selection"),
            UnexpectedType(target) => write!(f, "Unexpected Reply type: {:?}", target),
            EventFdCreate => write!(f, "Failed to create eventfd"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use self::Error::*;
        match self {
            Set(e) => Some(e),
            XcbConnection(e) => Some(e),
            XcbReply(e) => Some(e),
            XcbReplyOrId(e) => Some(e),
            XcbConnect(e) => Some(e),
            Lock | Timeout | Owner | UnexpectedType(_) | EventFdCreate => None,
        }
    }
}

macro_rules! define_from {
    ( $item:ident from $err:ty ) => {
        impl From<$err> for Error {
            fn from(err: $err) -> Error {
                Error::$item(err)
            }
        }
    };
}

define_from!(Set from SendError<Atom>);
define_from!(XcbConnect from ConnectError);
define_from!(XcbConnection from ConnectionError);
define_from!(XcbReply from ReplyError);
define_from!(XcbReplyOrId from ReplyOrIdError);
