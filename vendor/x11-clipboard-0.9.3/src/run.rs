use std::cmp;
use std::sync::Arc;
use std::sync::mpsc::{ Receiver, TryRecvError };
use std::collections::HashMap;
use std::os::fd::{AsFd, AsRawFd, FromRawFd, OwnedFd};
use ::{AtomEnum, EventMask};
use x11rb::connection::Connection;
use x11rb::protocol::Event;
use x11rb::protocol::xproto::{Atom, ChangeWindowAttributesAux, ConnectionExt, Property, PropMode, SELECTION_NOTIFY_EVENT, SelectionNotifyEvent, Window};
use ::{ INCR_CHUNK_SIZE, Context, SetMap };
use error::Error;

macro_rules! try_continue {
    ( $expr:expr ) => {
        match $expr {
            Some(val) => val,
            None => continue
        }
    };
}

struct IncrState {
    selection: Atom,
    requestor: Window,
    property: Atom,
    pos: usize
}

pub(crate) struct PipeDropFds {
    pub(crate) read_pipe: OwnedFd,
    pub(crate) write_pipe: OwnedFd,
}

pub(crate) fn create_pipe_drop_fd() -> Result<PipeDropFds, Error>{
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

pub(crate) fn run(context: Arc<Context>, setmap: SetMap, max_length: usize, receiver: Receiver<Atom>, read_pipe: OwnedFd) {
    let mut incr_map = HashMap::<Atom, Atom>::new();
    let mut state_map = HashMap::<Atom, IncrState>::new();

    let stream_fd = context.connection.stream().as_fd();
    let borrowed_fd = read_pipe.as_fd();
    // Poll stream for new Read-ready events, check if the other side of the pipe has been dropped
    let mut pollfds: [libc::pollfd; 2] = [libc::pollfd {
        fd: stream_fd.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    }, libc::pollfd {
        fd: borrowed_fd.as_raw_fd(),
        // If the other end is dropped, this pipe will get a HUP on poll
        events: libc::POLLHUP,
        revents: 0,
    }];
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
                    Ok(selection) => if let Some(property) = incr_map.remove(&selection) {
                        state_map.remove(&property);
                    },
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => if state_map.is_empty() {
                        return
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
                            &[context.atoms.targets, target]
                        );
                    } else if value.len() < max_length - 24 {
                        let _ = x11rb::wrapper::ConnectionExt::change_property8(
                            &context.connection,
                            PropMode::REPLACE,
                            event.requestor,
                            event.property,
                            target,
                            value
                        );
                    } else {
                        let _ = context.connection.change_window_attributes(
                            event.requestor,
                            &ChangeWindowAttributesAux::new()
                                .event_mask(EventMask::PROPERTY_CHANGE)
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
                                pos: 0
                            }
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
                            property: event.property
                        }
                    );
                    let _ = context.connection.flush();
                },
                Event::PropertyNotify(event) => {
                    if event.state != Property::DELETE { continue };

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
                            &value[state.pos..][..len]
                        );
                        state.pos += len;
                        len == 0
                    };

                    if is_end {
                        state_map.remove(&event.atom);
                    }
                    let _ = context.connection.flush();
                },
                Event::SelectionClear(event) => {
                    if let Some(property) = incr_map.remove(&event.selection) {
                        state_map.remove(&property);
                    }
                    if let Ok(mut write_setmap) = setmap.write() {
                        write_setmap.remove(&event.selection);
                    }
                }
                _ => ()
            }
        }
    }
}
