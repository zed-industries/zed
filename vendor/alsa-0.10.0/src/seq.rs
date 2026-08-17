//! MIDI sequencer I/O and enumeration

use libc::{c_uint, c_int, c_short, c_uchar, c_void, c_long, size_t, pollfd};
use super::error::*;
use crate::alsa;
use super::{Direction, poll};
use std::{ptr, fmt, mem, slice, time, cell};
use std::str::{FromStr, Split};
use std::ffi::CStr;
use std::borrow::Cow;

// Workaround for improper alignment of snd_seq_ev_ext_t in alsa-sys
#[repr(packed)]
struct EvExtPacked {
    len: c_uint,
    ptr: *mut c_void,
}

/// [snd_seq_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___sequencer.html) wrapper
///
/// To access the functions `event_input`, `event_input_pending` and `set_input_buffer_size`,
/// you first have to obtain an instance of `Input` by calling `input()`. Only one instance of
/// `Input` may exist at any time for a given `Seq`.
pub struct Seq(*mut alsa::snd_seq_t, cell::Cell<bool>);

unsafe impl Send for Seq {}

impl Drop for Seq {
    fn drop(&mut self) { unsafe { alsa::snd_seq_close(self.0) }; }
}

impl Seq {
    fn check_has_input(&self) {
        if self.1.get() { panic!("No additional Input object allowed")}
    }

    /// Opens the sequencer.
    ///
    /// If name is None, "default" will be used. That's almost always what you usually want to use anyway.
    pub fn open(name: Option<&CStr>, dir: Option<Direction>, nonblock: bool) -> Result<Seq> {
        let n2 = name.unwrap_or(unsafe { CStr::from_bytes_with_nul_unchecked(b"default\0") });
        let mut h = ptr::null_mut();
        let mode = if nonblock { alsa::SND_SEQ_NONBLOCK } else { 0 };
        let streams = match dir {
            None => alsa::SND_SEQ_OPEN_DUPLEX,
            Some(Direction::Playback) => alsa::SND_SEQ_OPEN_OUTPUT,
            Some(Direction::Capture) => alsa::SND_SEQ_OPEN_INPUT,
        };
        acheck!(snd_seq_open(&mut h, n2.as_ptr(), streams, mode))
            .map(|_| Seq(h, cell::Cell::new(false)))
    }

    pub fn set_client_name(&self, name: &CStr) -> Result<()> {
        acheck!(snd_seq_set_client_name(self.0, name.as_ptr())).map(|_| ())
    }

    pub fn set_client_event_filter(&self, event_type: i32) -> Result<()> {
        acheck!(snd_seq_set_client_event_filter(self.0, event_type as c_int)).map(|_| ())
    }

    pub fn set_client_pool_output(&self, size: u32) -> Result<()> {
        acheck!(snd_seq_set_client_pool_output(self.0, size as size_t)).map(|_| ())
    }

    pub fn set_client_pool_input(&self, size: u32) -> Result<()> {
        acheck!(snd_seq_set_client_pool_input(self.0, size as size_t)).map(|_| ())
    }

    pub fn set_client_pool_output_room(&self, size: u32) -> Result<()> {
        acheck!(snd_seq_set_client_pool_output_room(self.0, size as size_t)).map(|_| ())
    }

    pub fn client_id(&self) -> Result<i32> {
        acheck!(snd_seq_client_id(self.0)).map(|q| q as i32)
    }

    pub fn drain_output(&self) -> Result<i32> {
        acheck!(snd_seq_drain_output(self.0)).map(|q| q as i32)
    }

    pub fn get_any_client_info(&self, client: i32) -> Result<ClientInfo> {
        let c = ClientInfo::new()?;
        acheck!(snd_seq_get_any_client_info(self.0, client, c.0)).map(|_| c)
    }

    pub fn get_any_port_info(&self, a: Addr) -> Result<PortInfo> {
        let c = PortInfo::new()?;
        acheck!(snd_seq_get_any_port_info(self.0, a.client as c_int, a.port as c_int, c.0)).map(|_| c)
    }

    pub fn create_port(&self, port: &PortInfo) -> Result<()> {
        acheck!(snd_seq_create_port(self.0, port.0)).map(|_| ())
    }

    pub fn create_simple_port(&self, name: &CStr, caps: PortCap, t: PortType) -> Result<i32> {
        acheck!(snd_seq_create_simple_port(self.0, name.as_ptr(), caps.bits() as c_uint, t.bits() as c_uint)).map(|q| q as i32)
    }

    pub fn set_port_info(&self, port: i32, info: &mut PortInfo) -> Result<()> {
        acheck!(snd_seq_set_port_info(self.0, port, info.0)).map(|_| ())
    }

    pub fn delete_port(&self, port: i32) -> Result<()> {
        acheck!(snd_seq_delete_port(self.0, port as c_int)).map(|_| ())
    }

    pub fn subscribe_port(&self, info: &PortSubscribe) -> Result<()> {
        acheck!(snd_seq_subscribe_port(self.0, info.0)).map(|_| ())
    }

    pub fn unsubscribe_port(&self, sender: Addr, dest: Addr) -> Result<()> {
        let z = PortSubscribe::new()?;
        z.set_sender(sender);
        z.set_dest(dest);
        acheck!(snd_seq_unsubscribe_port(self.0, z.0)).map(|_| ())
    }

    pub fn control_queue(&self, q: i32, t: EventType, value: i32, e: Option<&mut Event>) -> Result<()> {
        assert!(EvQueueControl::<()>::has_data(t) || EvQueueControl::<i32>::has_data(t) || EvQueueControl::<u32>::has_data(t));
        let p = e.map(|e| &mut e.0 as *mut _).unwrap_or(ptr::null_mut());
        acheck!(snd_seq_control_queue(self.0, q as c_int, t as c_int, value as c_int, p)).map(|_| ())
    }

    pub fn event_output(&self, e: &mut Event) -> Result<u32> {
        e.ensure_buf();
        acheck!(snd_seq_event_output(self.0, &mut e.0)).map(|q| q as u32)
    }

    pub fn event_output_buffer(&self, e: &mut Event) -> Result<u32> {
        e.ensure_buf();
        acheck!(snd_seq_event_output_buffer(self.0, &mut e.0)).map(|q| q as u32)
    }

    pub fn event_output_direct(&self, e: &mut Event) -> Result<u32> {
        e.ensure_buf();
        acheck!(snd_seq_event_output_direct(self.0, &mut e.0)).map(|q| q as u32)
    }

    pub fn get_queue_tempo(&self, q: i32) -> Result<QueueTempo> {
        let value = QueueTempo::new()?;
        acheck!(snd_seq_get_queue_tempo(self.0, q as c_int, value.0)).map(|_| value)
    }

    pub fn set_queue_tempo(&self, q: i32, value: &QueueTempo) -> Result<()> {
        acheck!(snd_seq_set_queue_tempo(self.0, q as c_int, value.0)).map(|_| ())
    }

    pub fn get_queue_status(&self, q: i32) -> Result<QueueStatus> {
        let value = QueueStatus::new()?;
        acheck!(snd_seq_get_queue_status(self.0, q as c_int, value.0)).map(|_| value)
    }

    pub fn free_queue(&self, q: i32) -> Result<()> { acheck!(snd_seq_free_queue(self.0, q)).map(|_| ()) }
    pub fn alloc_queue(&self) -> Result<i32> { acheck!(snd_seq_alloc_queue(self.0)).map(|q| q as i32) }
    pub fn alloc_named_queue(&self, n: &CStr) -> Result<i32> {
        acheck!(snd_seq_alloc_named_queue(self.0, n.as_ptr())).map(|q| q as i32)
    }

    pub fn sync_output_queue(&self) -> Result<()> {
        acheck!(snd_seq_sync_output_queue(self.0)).map(|_| ())
    }

    pub fn drop_output(&self) -> Result<()> {
        acheck!(snd_seq_drop_output(self.0)).map(|_| ())
    }

    /// Call this function to obtain an instance of `Input` to access the functions `event_input`,
    /// `event_input_pending` and `set_input_buffer_size`. See the documentation of `Input` for details.
    pub fn input(&self) -> Input {
        Input::new(self)
    }

    pub fn remove_events(&self, condition: RemoveEvents) -> Result<()> {
        acheck!(snd_seq_remove_events(self.0, condition.0)).map(|_| ())
    }
}

/// Struct for receiving input events from a sequencer. The methods offered by this
/// object may modify the internal input buffer of the sequencer, which must not happen
/// while an `Event` is alive that has been obtained from a call to `event_input` (which
/// takes `Input` by mutable reference for this reason). This is because the event might
/// directly reference the sequencer's input buffer for variable-length messages (e.g. Sysex).
///
/// Note: Only one `Input` object is allowed in scope at a time.
pub struct Input<'a>(&'a Seq);

impl<'a> Drop for Input<'a> {
    fn drop(&mut self) { (self.0).1.set(false) }
}

impl<'a> Input<'a> {
    fn new(s: &'a Seq) -> Input<'a> {
        s.check_has_input();
        s.1.set(true);
        Input(s)
    }

    pub fn event_input(&mut self) -> Result<Event> {
        // The returned event might reference the input buffer of the `Seq`.
        // Therefore we mutably borrow the `Input` structure, preventing any
        // other function call that might change the input buffer while the
        // event is alive.
        let mut z = ptr::null_mut();
        acheck!(snd_seq_event_input((self.0).0, &mut z))?;
        unsafe { Event::extract (&mut *z, "snd_seq_event_input") }
    }

    pub fn event_input_pending(&self, fetch_sequencer: bool) -> Result<u32> {
        acheck!(snd_seq_event_input_pending((self.0).0, if fetch_sequencer {1} else {0})).map(|q| q as u32)
    }

    pub fn set_input_buffer_size(&self, size: u32)  -> Result<()> {
        acheck!(snd_seq_set_input_buffer_size((self.0).0, size as size_t)).map(|_| ())
    }

    pub fn drop_input(&self) -> Result<()> {
        acheck!(snd_seq_drop_input((self.0).0)).map(|_| ())
    }
}

fn polldir(o: Option<Direction>) -> c_short {
    match o {
        None => poll::Flags::IN | poll::Flags::OUT,
        Some(Direction::Playback) => poll::Flags::OUT,
        Some(Direction::Capture) => poll::Flags::IN,
    }.bits()
}

impl<'a> poll::Descriptors for (&'a Seq, Option<Direction>) {

    fn count(&self) -> usize {
        unsafe { alsa::snd_seq_poll_descriptors_count((self.0).0, polldir(self.1)) as usize }
    }

    fn fill(&self, p: &mut [pollfd]) -> Result<usize> {
        let z = unsafe { alsa::snd_seq_poll_descriptors((self.0).0, p.as_mut_ptr(), p.len() as c_uint, polldir(self.1)) };
        from_code("snd_seq_poll_descriptors", z).map(|_| z as usize)
    }

    fn revents(&self, p: &[pollfd]) -> Result<poll::Flags> {
        let mut r = 0;
        let z = unsafe { alsa::snd_seq_poll_descriptors_revents((self.0).0, p.as_ptr() as *mut pollfd, p.len() as c_uint, &mut r) };
        from_code("snd_seq_poll_descriptors_revents", z).map(|_| poll::Flags::from_bits_truncate(r as c_short))
    }
}

/// [snd_seq_client_info_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___seq_client.html) wrapper
pub struct ClientInfo(*mut alsa::snd_seq_client_info_t);

unsafe impl Send for ClientInfo {}

impl Drop for ClientInfo {
    fn drop(&mut self) {
        unsafe { alsa::snd_seq_client_info_free(self.0) };
    }
}

impl ClientInfo {
    fn new() -> Result<Self> {
        let mut p = ptr::null_mut();
        acheck!(snd_seq_client_info_malloc(&mut p)).map(|_| ClientInfo(p))
    }

    // Not sure if it's useful for this one to be public.
    fn set_client(&self, client: i32) {
        unsafe { alsa::snd_seq_client_info_set_client(self.0, client as c_int) };
    }

    pub fn get_client(&self) -> i32 {
        unsafe { alsa::snd_seq_client_info_get_client(self.0) as i32 }
    }

    pub fn get_name(&self) -> Result<&str> {
        let c = unsafe { alsa::snd_seq_client_info_get_name(self.0) };
        from_const("snd_seq_client_info_get_name", c)
    }
}

impl fmt::Debug for ClientInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClientInfo({},{:?})", self.get_client(), self.get_name())
    }
}

#[derive(Copy, Clone)]
/// Iterates over clients connected to the seq API (both kernel and userspace clients).
pub struct ClientIter<'a>(&'a Seq, i32);

impl<'a> ClientIter<'a> {
    pub fn new(seq: &'a Seq) -> Self { ClientIter(seq, -1) }
}

impl<'a> Iterator for ClientIter<'a> {
    type Item = ClientInfo;
    fn next(&mut self) -> Option<Self::Item> {
        let z = ClientInfo::new().unwrap();
        z.set_client(self.1);
        let r = unsafe { alsa::snd_seq_query_next_client((self.0).0, z.0) };
        if r < 0 { self.1 = -1; return None };
        self.1 = z.get_client();
        Some(z)
    }
}

/// [snd_seq_port_info_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___seq_port.html) wrapper
pub struct PortInfo(*mut alsa::snd_seq_port_info_t);

unsafe impl Send for PortInfo {}

impl Drop for PortInfo {
    fn drop(&mut self) {
        unsafe { alsa::snd_seq_port_info_free(self.0) };
    }
}

impl PortInfo {
    fn new() -> Result<Self> {
        let mut p = ptr::null_mut();
        acheck!(snd_seq_port_info_malloc(&mut p)).map(|_| PortInfo(p))
    }

    /// Creates a new PortInfo with all fields set to zero.
    pub fn empty() -> Result<Self> {
        let z = Self::new()?;
        unsafe { ptr::write_bytes(z.0 as *mut u8, 0, alsa::snd_seq_port_info_sizeof()) };
        Ok(z)
    }

    pub fn get_client(&self) -> i32 {
        unsafe { alsa::snd_seq_port_info_get_client(self.0) as i32 }
    }

    pub fn get_port(&self) -> i32 {
        unsafe { alsa::snd_seq_port_info_get_port(self.0) as i32 }
    }

    // Not sure if it's useful for this one to be public.
    fn set_client(&self, client: i32) {
        unsafe { alsa::snd_seq_port_info_set_client(self.0, client as c_int) };
    }

    // Not sure if it's useful for this one to be public.
    fn set_port(&self, port: i32) {
        unsafe { alsa::snd_seq_port_info_set_port(self.0, port as c_int) };
    }

    pub fn get_name(&self) -> Result<&str> {
        let c = unsafe { alsa::snd_seq_port_info_get_name(self.0) };
        from_const("snd_seq_port_info_get_name", c)
    }

    pub fn set_name(&mut self, name: &CStr) {
        // Note: get_name returns an interior reference, so this one must take &mut self
        unsafe { alsa::snd_seq_port_info_set_name(self.0, name.as_ptr()) };
    }

    pub fn get_capability(&self) -> PortCap {
        PortCap::from_bits_truncate(unsafe { alsa::snd_seq_port_info_get_capability(self.0) as u32 })
    }

    pub fn get_type(&self) -> PortType {
        PortType::from_bits_truncate(unsafe { alsa::snd_seq_port_info_get_type(self.0) as u32 })
    }

    pub fn set_capability(&self, c: PortCap) {
        unsafe { alsa::snd_seq_port_info_set_capability(self.0, c.bits() as c_uint) }
    }

    pub fn set_type(&self, c: PortType) {
        unsafe { alsa::snd_seq_port_info_set_type(self.0, c.bits() as c_uint) }
    }

    /// Returns an Addr containing this PortInfo's client and port id.
    pub fn addr(&self) -> Addr {
        Addr {
            client: self.get_client(),
            port: self.get_port(),
        }
    }

    pub fn get_midi_channels(&self) -> i32 { unsafe { alsa::snd_seq_port_info_get_midi_channels(self.0) as i32 } }
    pub fn get_midi_voices(&self) -> i32 { unsafe { alsa::snd_seq_port_info_get_midi_voices(self.0) as i32 } }
    pub fn get_synth_voices(&self) -> i32 { unsafe { alsa::snd_seq_port_info_get_synth_voices(self.0) as i32 } }
    pub fn get_read_use(&self) -> i32 { unsafe { alsa::snd_seq_port_info_get_read_use(self.0) as i32 } }
    pub fn get_write_use(&self) -> i32 { unsafe { alsa::snd_seq_port_info_get_write_use(self.0) as i32 } }
    pub fn get_port_specified(&self) -> bool { unsafe { alsa::snd_seq_port_info_get_port_specified(self.0) == 1 } }
    pub fn get_timestamping(&self) -> bool { unsafe { alsa::snd_seq_port_info_get_timestamping(self.0) == 1 } }
    pub fn get_timestamp_real(&self) -> bool { unsafe { alsa::snd_seq_port_info_get_timestamp_real(self.0) == 1 } }
    pub fn get_timestamp_queue(&self) -> i32 { unsafe { alsa::snd_seq_port_info_get_timestamp_queue(self.0) as i32 } }

    pub fn set_midi_channels(&self, value: i32) { unsafe { alsa::snd_seq_port_info_set_midi_channels(self.0, value as c_int) } }
    pub fn set_midi_voices(&self, value: i32) { unsafe { alsa::snd_seq_port_info_set_midi_voices(self.0, value as c_int) } }
    pub fn set_synth_voices(&self, value: i32) { unsafe { alsa::snd_seq_port_info_set_synth_voices(self.0, value as c_int) } }
    pub fn set_port_specified(&self, value: bool) { unsafe { alsa::snd_seq_port_info_set_port_specified(self.0, if value { 1 } else { 0 } ) } }
    pub fn set_timestamping(&self, value: bool) { unsafe { alsa::snd_seq_port_info_set_timestamping(self.0, if value { 1 } else { 0 } ) } }
    pub fn set_timestamp_real(&self, value: bool) { unsafe { alsa::snd_seq_port_info_set_timestamp_real(self.0, if value { 1 } else { 0 } ) } }
    pub fn set_timestamp_queue(&self, value: i32) { unsafe { alsa::snd_seq_port_info_set_timestamp_queue(self.0, value as c_int) } }
}

impl fmt::Debug for PortInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PortInfo({}:{},{:?})", self.get_client(), self.get_port(), self.get_name())
    }
}

#[derive(Copy, Clone)]
/// Iterates over clients connected to the seq API (both kernel and userspace clients).
pub struct PortIter<'a>(&'a Seq, i32, i32);

impl<'a> PortIter<'a> {
    pub fn new(seq: &'a Seq, client: i32) -> Self { PortIter(seq, client, -1) }
}

impl<'a> Iterator for PortIter<'a> {
    type Item = PortInfo;
    fn next(&mut self) -> Option<Self::Item> {
        let z = PortInfo::new().unwrap();
        z.set_client(self.1);
        z.set_port(self.2);
        let r = unsafe { alsa::snd_seq_query_next_port((self.0).0, z.0) };
        if r < 0 { self.2 = -1; return None };
        self.2 = z.get_port();
        Some(z)
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /// [SND_SEQ_PORT_CAP_xxx](http://www.alsa-project.org/alsa-doc/alsa-lib/group___seq_port.html) constants
    pub struct PortCap: u32 {
        const READ = 1<<0;
        const WRITE = 1<<1;
        const SYNC_READ = 1<<2;
        const SYNC_WRITE = 1<<3;
        const DUPLEX = 1<<4;
        const SUBS_READ = 1<<5;
        const SUBS_WRITE = 1<<6;
        const NO_EXPORT = 1<<7;
   }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /// [SND_SEQ_PORT_TYPE_xxx](http://www.alsa-project.org/alsa-doc/alsa-lib/group___seq_port.html) constants
    pub struct PortType: u32 {
        const SPECIFIC = (1<<0);
        const MIDI_GENERIC = (1<<1);
        const MIDI_GM = (1<<2);
        const MIDI_GS = (1<<3);
        const MIDI_XG = (1<<4);
        const MIDI_MT32 = (1<<5);
        const MIDI_GM2 = (1<<6);
        const SYNTH = (1<<10);
        const DIRECT_SAMPLE = (1<<11);
        const SAMPLE = (1<<12);
        const HARDWARE = (1<<16);
        const SOFTWARE = (1<<17);
        const SYNTHESIZER = (1<<18);
        const PORT = (1<<19);
        const APPLICATION = (1<<20);
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /// [SND_SEQ_REMOVE_xxx](https://www.alsa-project.org/alsa-doc/alsa-lib/group___seq_event.html) constants
    pub struct Remove: u32 {
        const INPUT = (1<<0);
        const OUTPUT = (1<<1);
        const DEST = (1<<2);
        const DEST_CHANNEL = (1<<3);
        const TIME_BEFORE = (1<<4);
        const TIME_AFTER = (1<<5);
        const TIME_TICK = (1<<6);
        const EVENT_TYPE = (1<<7);
        const IGNORE_OFF = (1<<8);
        const TAG_MATCH = (1<<9);
    }
}


/// [snd_seq_addr_t](http://www.alsa-project.org/alsa-doc/alsa-lib/structsnd__seq__addr__t.html) wrapper
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Addr {
    pub client: i32,
    pub port: i32,
}

impl FromStr for Addr {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut split: Split<'_, char> = s.trim().split(':');
        let client = split.next()
                          .ok_or("no client provided")?
                          .parse::<i32>()?;
        let port = split.next()
                        .ok_or("no port provided")?
                        .parse::<i32>()?;
        match split.next() {
            Some(_) => {
                Err("too many arguments".into())
            },
            None => {
                Ok(Addr { client, port })
            }
        }
    }
}

impl Addr {
    pub fn system_timer() -> Addr { Addr { client: alsa::SND_SEQ_CLIENT_SYSTEM as i32, port: alsa::SND_SEQ_PORT_SYSTEM_TIMER as i32 } }
    pub fn system_announce() -> Addr { Addr { client: alsa::SND_SEQ_CLIENT_SYSTEM as i32, port: alsa::SND_SEQ_PORT_SYSTEM_ANNOUNCE as i32 } }
    pub fn broadcast() -> Addr { Addr { client: alsa::SND_SEQ_ADDRESS_BROADCAST as i32, port: alsa::SND_SEQ_ADDRESS_BROADCAST as i32 } }
}

/// [snd_seq_port_subscribe_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___seq_subscribe.html) wrapper
pub struct PortSubscribe(*mut alsa::snd_seq_port_subscribe_t);

unsafe impl Send for PortSubscribe {}

impl Drop for PortSubscribe {
    fn drop(&mut self) { unsafe { alsa::snd_seq_port_subscribe_free(self.0) }; }
}

impl PortSubscribe {
    fn new() -> Result<Self> {
        let mut p = ptr::null_mut();
        acheck!(snd_seq_port_subscribe_malloc(&mut p)).map(|_| PortSubscribe(p))
    }

    /// Creates a new PortSubscribe with all fields set to zero.
    pub fn empty() -> Result<Self> {
        let z = Self::new()?;
        unsafe { ptr::write_bytes(z.0 as *mut u8, 0, alsa::snd_seq_port_subscribe_sizeof()) };
        Ok(z)
    }

    pub fn get_sender(&self) -> Addr { unsafe {
        let z = alsa::snd_seq_port_subscribe_get_sender(self.0);
        Addr { client: (*z).client as i32, port: (*z).port as i32 }
    } }

    pub fn get_dest(&self) -> Addr { unsafe {
        let z = alsa::snd_seq_port_subscribe_get_dest(self.0);
        Addr { client: (*z).client as i32, port: (*z).port as i32 }
    } }

    pub fn get_queue(&self) -> i32 { unsafe { alsa::snd_seq_port_subscribe_get_queue(self.0) as i32 } }
    pub fn get_exclusive(&self) -> bool { unsafe { alsa::snd_seq_port_subscribe_get_exclusive(self.0) == 1 } }
    pub fn get_time_update(&self) -> bool { unsafe { alsa::snd_seq_port_subscribe_get_time_update(self.0) == 1 } }
    pub fn get_time_real(&self) -> bool { unsafe { alsa::snd_seq_port_subscribe_get_time_real(self.0) == 1 } }

    pub fn set_sender(&self, value: Addr) {
        let z = alsa::snd_seq_addr_t { client: value.client as c_uchar, port: value.port as c_uchar };
        unsafe { alsa::snd_seq_port_subscribe_set_sender(self.0, &z) };
    }

    pub fn set_dest(&self, value: Addr) {
        let z = alsa::snd_seq_addr_t { client: value.client as c_uchar, port: value.port as c_uchar };
        unsafe { alsa::snd_seq_port_subscribe_set_dest(self.0, &z) };
    }

    pub fn set_queue(&self, value: i32) { unsafe { alsa::snd_seq_port_subscribe_set_queue(self.0, value as c_int) } }
    pub fn set_exclusive(&self, value: bool) { unsafe { alsa::snd_seq_port_subscribe_set_exclusive(self.0, if value { 1 } else { 0 } ) } }
    pub fn set_time_update(&self, value: bool) { unsafe { alsa::snd_seq_port_subscribe_set_time_update(self.0, if value { 1 } else { 0 } ) } }
    pub fn set_time_real(&self, value: bool) { unsafe { alsa::snd_seq_port_subscribe_set_time_real(self.0, if value { 1 } else { 0 } ) } }

}

/// [snd_seq_query_subs_type_t](https://www.alsa-project.org/alsa-doc/alsa-lib/group___seq_subscribe.html) wrapper
#[derive(Copy, Clone)]
pub enum QuerySubsType {
    READ = alsa::SND_SEQ_QUERY_SUBS_READ as isize,
    WRITE = alsa::SND_SEQ_QUERY_SUBS_WRITE as isize,
}

/// [snd_seq_query_subscribe_t](https://www.alsa-project.org/alsa-doc/alsa-lib/group___seq_subscribe.html) wrapper
//(kept private, functionality exposed by PortSubscribeIter)
struct QuerySubscribe(*mut alsa::snd_seq_query_subscribe_t);

unsafe impl Send for QuerySubscribe {}

impl Drop for QuerySubscribe {
    fn drop(&mut self) { unsafe { alsa::snd_seq_query_subscribe_free(self.0) } }
}

impl QuerySubscribe {
    pub fn new() -> Result<Self> {
        let mut q = ptr::null_mut();
        acheck!(snd_seq_query_subscribe_malloc(&mut q)).map(|_| QuerySubscribe(q))
    }

    pub fn get_index(&self) -> i32 { unsafe { alsa::snd_seq_query_subscribe_get_index(self.0) as i32 } }
    pub fn get_addr(&self) -> Addr { unsafe {
        let a = &(*alsa::snd_seq_query_subscribe_get_addr(self.0));
        Addr { client: a.client as i32, port: a.port as i32 }
    } }
    pub fn get_queue(&self) -> i32 { unsafe { alsa::snd_seq_query_subscribe_get_queue(self.0) as i32 } }
    pub fn get_exclusive(&self) -> bool { unsafe { alsa::snd_seq_query_subscribe_get_exclusive(self.0) == 1 } }
    pub fn get_time_update(&self) -> bool { unsafe { alsa::snd_seq_query_subscribe_get_time_update(self.0) == 1 } }
    pub fn get_time_real(&self) -> bool { unsafe { alsa::snd_seq_query_subscribe_get_time_real(self.0) == 1 } }

    pub fn set_root(&self, value: Addr) { unsafe {
        let a = alsa::snd_seq_addr_t { client: value.client as c_uchar, port: value.port as c_uchar};
        alsa::snd_seq_query_subscribe_set_root(self.0, &a);
    } }
    pub fn set_type(&self, value: QuerySubsType) { unsafe {
        alsa::snd_seq_query_subscribe_set_type(self.0, value as alsa::snd_seq_query_subs_type_t)
    } }
    pub fn set_index(&self, value: i32) { unsafe { alsa::snd_seq_query_subscribe_set_index(self.0, value as c_int) } }
}

#[derive(Copy, Clone)]
/// Iterates over port subscriptions for a given client:port/type.
pub struct PortSubscribeIter<'a> {
    seq: &'a Seq,
    addr: Addr,
    query_subs_type: QuerySubsType,
    index: i32
}

impl<'a> PortSubscribeIter<'a> {
    pub fn new(seq: &'a Seq, addr: Addr, query_subs_type: QuerySubsType) -> Self {
        PortSubscribeIter {seq, addr, query_subs_type, index: 0 }
    }
}

impl<'a> Iterator for PortSubscribeIter<'a> {
    type Item = PortSubscribe;

    fn next(&mut self) -> Option<Self::Item> {
        let query = QuerySubscribe::new().unwrap();

        query.set_root(self.addr);
        query.set_type(self.query_subs_type);
        query.set_index(self.index);

        let r = unsafe { alsa::snd_seq_query_port_subscribers((self.seq).0, query.0) };
        if r < 0 {
            self.index = 0;
            return None;
        }

        self.index = query.get_index() + 1;
        let vtr = PortSubscribe::new().unwrap();
        match self.query_subs_type {
            QuerySubsType::READ => {
                vtr.set_sender(self.addr);
                vtr.set_dest(query.get_addr());
            },
            QuerySubsType:: WRITE => {
                vtr.set_sender(query.get_addr());
                vtr.set_dest(self.addr);
            }
        };
        vtr.set_queue(query.get_queue());
        vtr.set_exclusive(query.get_exclusive());
        vtr.set_time_update(query.get_time_update());
        vtr.set_time_real(query.get_time_real());

        Some(vtr)
    }
}

/// [snd_seq_event_t](http://www.alsa-project.org/alsa-doc/alsa-lib/structsnd__seq__event__t.html) wrapper
///
/// Fields of the event is not directly exposed. Instead call `Event::new` to set data (which can be, e g, an EvNote).
/// Use `get_type` and `get_data` to retrieve data.
///
/// The lifetime parameter refers to the lifetime of an associated external buffer that might be used for
/// variable-length messages (e.g. SysEx).
pub struct Event<'a>(alsa::snd_seq_event_t, EventType, Option<Cow<'a, [u8]>>);

unsafe impl<'a> Send for Event<'a> {}

impl<'a> Event<'a> {
    /// Creates a new event. For events that carry variable-length data (e.g. Sysex), `new_ext` has to be used instead.
    pub fn new<D: EventData>(t: EventType, data: &D) -> Event<'static> {
        assert!(!Event::has_ext_data(t), "event type must not carry variable-length data");
        let mut z = Event(unsafe { mem::zeroed() }, t, None);
        (z.0).type_ = t as c_uchar;
        (z.0).flags |= Event::get_length_flag(t);
        debug_assert!(D::has_data(t));
        data.set_data(&mut z);
        z
    }

    /// Creates a new event carrying variable-length data. This is required for event types `Sysex`, `Bounce`, and the `UsrVar` types.
    pub fn new_ext<D: Into<Cow<'a, [u8]>>>(t: EventType, data: D) -> Event<'a> {
        assert!(Event::has_ext_data(t), "event type must carry variable-length data");
        let mut z = Event(unsafe { mem::zeroed() }, t, Some(data.into()));
        (z.0).type_ = t as c_uchar;
        (z.0).flags |= Event::get_length_flag(t);
        z
    }

    /// Consumes this event and returns an (otherwise unchanged) event where the externally referenced
    /// buffer for variable length messages (e.g. SysEx) has been copied into the event.
    /// The returned event has a static lifetime, i e, it's decoupled from the original buffer.
    pub fn into_owned(self) -> Event<'static> {
        Event(self.0, self.1, self.2.map(|cow| Cow::Owned(cow.into_owned())))
    }

    fn get_length_flag(t: EventType) -> u8 {
        match t {
            EventType::Sysex => alsa::SND_SEQ_EVENT_LENGTH_VARIABLE,
            EventType::Bounce => alsa::SND_SEQ_EVENT_LENGTH_VARIABLE, // not clear whether this should be VARIABLE or VARUSR
            EventType::UsrVar0 => alsa::SND_SEQ_EVENT_LENGTH_VARUSR,
            EventType::UsrVar1 => alsa::SND_SEQ_EVENT_LENGTH_VARUSR,
            EventType::UsrVar2 => alsa::SND_SEQ_EVENT_LENGTH_VARUSR,
            EventType::UsrVar3 => alsa::SND_SEQ_EVENT_LENGTH_VARUSR,
            EventType::UsrVar4 => alsa::SND_SEQ_EVENT_LENGTH_VARUSR,
            _ => alsa::SND_SEQ_EVENT_LENGTH_FIXED
        }
    }

    fn has_ext_data(t: EventType) -> bool {
        Event::get_length_flag(t) != alsa::SND_SEQ_EVENT_LENGTH_FIXED
    }

    /// Extracts event type and data. Produces a result with an arbitrary lifetime, hence the unsafety.
    unsafe fn extract<'any>(z: &mut alsa::snd_seq_event_t, func: &'static str) -> Result<Event<'any>> {
        let t = EventType::from_c_int((*z).type_ as c_int, func)?;
        let ext_data = if Event::has_ext_data(t) {
            assert_ne!((*z).flags & alsa::SND_SEQ_EVENT_LENGTH_MASK, alsa::SND_SEQ_EVENT_LENGTH_FIXED);
            Some(Cow::Borrowed({
                let zz: &EvExtPacked = &*(&(*z).data as *const alsa::snd_seq_event__bindgen_ty_1 as *const _);
                slice::from_raw_parts((*zz).ptr as *mut u8, (*zz).len as usize)
            }))
        } else {
            None
        };
        Ok(Event(ptr::read(z), t, ext_data))
    }

    /// Ensures that the ev.ext union element points to the correct resize_buffer for events
    /// with variable length content
    fn ensure_buf(&mut self) {
        if !Event::has_ext_data(self.1) { return; }
        let slice: &[u8] = match self.2 {
            Some(Cow::Owned(ref mut vec)) => &vec[..],
            Some(Cow::Borrowed(buf)) => buf,
            // The following case is always a logic error in the program, thus panicking is okay.
            None => panic!("event type requires variable-length data, but none was provided")
        };
        let z: &mut EvExtPacked = unsafe { &mut *(&mut self.0.data as *mut alsa::snd_seq_event__bindgen_ty_1 as *mut _) };
        z.len = slice.len() as c_uint;
        z.ptr = slice.as_ptr() as *mut c_void;
    }

    #[inline]
    pub fn get_type(&self) -> EventType { self.1 }

    /// Extract the event data from an event.
    /// Use `get_ext` instead for events carrying variable-length data.
    pub fn get_data<D: EventData>(&self) -> Option<D> { if D::has_data(self.1) { Some(D::get_data(self)) } else { None } }

    /// Extract the variable-length data carried by events of type `Sysex`, `Bounce`, or the `UsrVar` types.
    pub fn get_ext(&self) -> Option<&[u8]> {
        if Event::has_ext_data(self.1) {
            match self.2 {
                Some(Cow::Owned(ref vec)) => Some(&vec[..]),
                Some(Cow::Borrowed(buf)) => Some(buf),
                // The following case is always a logic error in the program, thus panicking is okay.
                None => panic!("event type requires variable-length data, but none was found")
            }
        } else {
            None
        }
    }

    pub fn set_subs(&mut self) {
        self.0.dest.client = alsa::SND_SEQ_ADDRESS_SUBSCRIBERS;
        self.0.dest.port = alsa::SND_SEQ_ADDRESS_UNKNOWN;
    }

    pub fn set_source(&mut self, p: i32) { self.0.source.port = p as u8 }
    pub fn set_dest(&mut self, d: Addr) { self.0.dest.client = d.client as c_uchar; self.0.dest.port = d.port as c_uchar; }
    pub fn set_tag(&mut self, t: u8) { self.0.tag = t as c_uchar;  }
    pub fn set_queue(&mut self, q: i32) { self.0.queue = q as c_uchar;  }

    pub fn get_source(&self) -> Addr { Addr { client: self.0.source.client as i32, port: self.0.source.port as i32 } }
    pub fn get_dest(&self) -> Addr { Addr { client: self.0.dest.client as i32, port: self.0.dest.port as i32 } }
    pub fn get_tag(&self) -> u8 { self.0.tag as u8  }
    pub fn get_queue(&self) -> i32 { self.0.queue as i32 }

    pub fn schedule_real(&mut self, queue: i32, relative: bool, rtime: time::Duration) {
        self.0.flags &= !(alsa::SND_SEQ_TIME_STAMP_MASK | alsa::SND_SEQ_TIME_MODE_MASK);
        self.0.flags |= alsa::SND_SEQ_TIME_STAMP_REAL | (if relative { alsa::SND_SEQ_TIME_MODE_REL } else { alsa::SND_SEQ_TIME_MODE_ABS });
        self.0.queue = queue as u8;
        let t = unsafe { &mut self.0.time.time };
        t.tv_sec = rtime.as_secs() as c_uint;
        t.tv_nsec = rtime.subsec_nanos() as c_uint;
    }

    pub fn schedule_tick(&mut self, queue: i32, relative: bool, ttime: u32) {
        self.0.flags &= !(alsa::SND_SEQ_TIME_STAMP_MASK | alsa::SND_SEQ_TIME_MODE_MASK);
        self.0.flags |= alsa::SND_SEQ_TIME_STAMP_TICK | (if relative { alsa::SND_SEQ_TIME_MODE_REL } else { alsa::SND_SEQ_TIME_MODE_ABS });
        self.0.queue = queue as u8;
        let t = unsafe { &mut self.0.time.tick };
        *t = ttime as c_uint;
    }

    pub fn set_direct(&mut self) { self.0.queue = alsa::SND_SEQ_QUEUE_DIRECT }

    pub fn get_relative(&self) -> bool { (self.0.flags & alsa::SND_SEQ_TIME_MODE_REL) != 0 }

    pub fn get_time(&self) -> Option<time::Duration> {
        if (self.0.flags & alsa::SND_SEQ_TIME_STAMP_REAL) != 0 {
            let d = self.0.time;
            let t = unsafe { &d.time };
            Some(time::Duration::new(t.tv_sec as u64, t.tv_nsec as u32))
        } else { None }
    }

    pub fn get_tick(&self) -> Option<u32> {
        if (self.0.flags & alsa::SND_SEQ_TIME_STAMP_REAL) == 0 {
            let d = self.0.time;
            let t = unsafe { &d.tick };
            Some(*t)
        } else { None }
    }

    /// Returns true if the message is high priority.
    pub fn get_priority(&self) -> bool { (self.0.flags & alsa::SND_SEQ_PRIORITY_HIGH) != 0 }

    pub fn set_priority(&mut self, is_high_prio: bool) {
        if is_high_prio { self.0.flags |= alsa::SND_SEQ_PRIORITY_HIGH; }
        else { self.0.flags &= !alsa::SND_SEQ_PRIORITY_HIGH; }
    }
}

impl<'a> Clone for Event<'a> {
    fn clone(&self) -> Self { Event(unsafe { ptr::read(&self.0) }, self.1, self.2.clone()) }
}

impl<'a> fmt::Debug for Event<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut x = f.debug_tuple("Event");
        x.field(&self.1);
        if let Some(z) = self.get_data::<EvNote>() { x.field(&z); }
        if let Some(z) = self.get_data::<EvCtrl>() { x.field(&z); }
        if let Some(z) = self.get_data::<Addr>() { x.field(&z); }
        if let Some(z) = self.get_data::<Connect>() { x.field(&z); }
        if let Some(z) = self.get_data::<EvQueueControl<()>>() { x.field(&z); }
        if let Some(z) = self.get_data::<EvQueueControl<i32>>() { x.field(&z); }
        if let Some(z) = self.get_data::<EvQueueControl<u32>>() { x.field(&z); }
        if let Some(z) = self.get_data::<EvQueueControl<time::Duration>>() { x.field(&z); }
        if let Some(z) = self.get_data::<EvResult>() { x.field(&z); }
        if let Some(z) = self.get_data::<[u8; 12]>() { x.field(&z); }
        if let Some(z) = self.get_ext() { x.field(&z); }
        x.finish()
    }
}

/// Internal trait implemented for different event type structs (`EvNote`, `EvCtrl`, etc).
///
/// Use it through `Event::get_data` and `Event::new`.
pub trait EventData {
    #[doc(hidden)]
    fn get_data(ev: &Event) -> Self;
    #[doc(hidden)]
    fn has_data(e: EventType) -> bool;
    #[doc(hidden)]
    fn set_data(&self, ev: &mut Event);
}

impl EventData for () {
    fn get_data(_: &Event) -> Self {}
    fn has_data(e: EventType) -> bool {
         matches!(e,
             EventType::TuneRequest |
             EventType::Reset |
             EventType::Sensing |
             EventType::None)
    }
    fn set_data(&self, _: &mut Event) {}
}

impl EventData for [u8; 12] {
    fn get_data(ev: &Event) -> Self {
         let d = unsafe { ptr::read(&ev.0.data) };
         let z = unsafe { &d.raw8 };
         z.d
    }
    fn has_data(e: EventType) -> bool {
         matches!(e,
             EventType::Echo |
             EventType::Oss |
             EventType::Usr0 |
             EventType::Usr1 |
             EventType::Usr2 |
             EventType::Usr3 |
             EventType::Usr4 |
             EventType::Usr5 |
             EventType::Usr6 |
             EventType::Usr7 |
             EventType::Usr8 |
             EventType::Usr9)
    }
    fn set_data(&self, ev: &mut Event) {
         let z = unsafe { &mut ev.0.data.raw8 };
         z.d = *self;
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct EvNote {
    pub channel: u8,
    pub note: u8,
    pub velocity: u8,
    pub off_velocity: u8,
    pub duration: u32,
}

impl EventData for EvNote {
    fn get_data(ev: &Event) -> Self {
         let z: &alsa::snd_seq_ev_note_t = unsafe { &*(&ev.0.data as *const alsa::snd_seq_event__bindgen_ty_1 as *const _) };
         EvNote { channel: z.channel as u8, note: z.note as u8, velocity: z.velocity as u8, off_velocity: z.off_velocity as u8, duration: z.duration as u32 }
    }
    fn has_data(e: EventType) -> bool {
         matches!(e,
             EventType::Note |
             EventType::Noteon |
             EventType::Noteoff |
             EventType::Keypress)
    }
    fn set_data(&self, ev: &mut Event) {
         let z: &mut alsa::snd_seq_ev_note_t = unsafe { &mut *(&mut ev.0.data as *mut alsa::snd_seq_event__bindgen_ty_1 as *mut _) };
         z.channel = self.channel as c_uchar;
         z.note = self.note as c_uchar;
         z.velocity = self.velocity as c_uchar;
         z.off_velocity = self.off_velocity as c_uchar;
         z.duration = self.duration as c_uint;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct EvCtrl {
    pub channel: u8,
    pub param: u32,
    pub value: i32,
}

impl EventData for EvCtrl {
    fn get_data(ev: &Event) -> Self {
         let z: &alsa::snd_seq_ev_ctrl_t = unsafe { &*(&ev.0.data as *const alsa::snd_seq_event__bindgen_ty_1 as *const _) };
         EvCtrl { channel: z.channel as u8, param: z.param as u32, value: z.value as i32 }
    }
    fn has_data(e: EventType) -> bool {
         matches!(e,
             EventType::Controller |
             EventType::Pgmchange |
             EventType::Chanpress |
             EventType::Pitchbend |
             EventType::Control14 |
             EventType::Nonregparam |
             EventType::Regparam |
             EventType::Songpos |
             EventType::Songsel |
             EventType::Qframe |
             EventType::Timesign |
             EventType::Keysign)
    }
    fn set_data(&self, ev: &mut Event) {
         let z: &mut alsa::snd_seq_ev_ctrl_t = unsafe { &mut *(&mut ev.0.data as *mut alsa::snd_seq_event__bindgen_ty_1 as *mut _) };
         z.channel = self.channel as c_uchar;
         z.param = self.param as c_uint;
         z.value = self.value as c_int;
    }
}

impl EventData for Addr {
    fn get_data(ev: &Event) -> Self {
         let z: &alsa::snd_seq_addr_t = unsafe { &*(&ev.0.data as *const alsa::snd_seq_event__bindgen_ty_1 as *const _) };
         Addr { client: z.client as i32, port: z.port as i32 }
    }
    fn has_data(e: EventType) -> bool {
         matches!(e,
             EventType::ClientStart |
             EventType::ClientExit |
             EventType::ClientChange |
             EventType::PortStart |
             EventType::PortExit |
             EventType::PortChange)
    }
    fn set_data(&self, ev: &mut Event) {
         let z: &mut alsa::snd_seq_addr_t = unsafe { &mut *(&mut ev.0.data as *mut alsa::snd_seq_event__bindgen_ty_1 as *mut _) };
         z.client = self.client as c_uchar;
         z.port = self.port as c_uchar;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
/// [snd_seq_connect_t](http://www.alsa-project.org/alsa-doc/alsa-lib/structsnd__seq__connect__t.html) wrapper
pub struct Connect {
    pub sender: Addr,
    pub dest: Addr,
}

impl EventData for Connect {
    fn get_data(ev: &Event) -> Self {
         let d = unsafe { ptr::read(&ev.0.data) };
         let z = unsafe { &d.connect };
         Connect {
             sender: Addr { client: z.sender.client as i32, port: z.sender.port as i32 },
             dest: Addr { client: z.dest.client as i32, port: z.dest.port as i32 }
         }
    }
    fn has_data(e: EventType) -> bool {
         matches!(e,
             EventType::PortSubscribed |
             EventType::PortUnsubscribed)
    }
    fn set_data(&self, ev: &mut Event) {
         let z = unsafe { &mut ev.0.data.connect };
         z.sender.client = self.sender.client as c_uchar;
         z.sender.port = self.sender.port as c_uchar;
         z.dest.client = self.dest.client as c_uchar;
         z.dest.port = self.dest.port as c_uchar;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
/// [snd_seq_ev_queue_control_t](http://www.alsa-project.org/alsa-doc/alsa-lib/structsnd__seq__ev__queue__control__t.html) wrapper
///
/// Note: This struct is generic, but what types of T are required for the different EvQueueControl messages is
/// not very well documented in alsa-lib. Right now, Tempo is i32, Tick, SetposTick and SyncPos are u32, SetposTime is time::Duration,
/// and the rest is (). If I guessed wrong, let me know.
pub struct EvQueueControl<T> {
    pub queue: i32,
    pub value: T,
}

impl EventData for EvQueueControl<()> {
    fn get_data(ev: &Event) -> Self {
         let d = unsafe { ptr::read(&ev.0.data) };
         let z = unsafe { &d.queue };
         EvQueueControl { queue: z.queue as i32, value: () }
    }
    fn has_data(e: EventType) -> bool {
         matches!(e,
             EventType::Start |
             EventType::Continue |
             EventType::Stop |
             EventType::Clock |
             EventType::QueueSkew)
    }
    fn set_data(&self, ev: &mut Event) {
         let z = unsafe { &mut ev.0.data.queue };
         z.queue = self.queue as c_uchar;
    }
}

impl EventData for EvQueueControl<i32> {
    fn get_data(ev: &Event) -> Self { unsafe {
         let mut d = ptr::read(&ev.0.data);
         let z = &mut d.queue;
         EvQueueControl { queue: z.queue as i32, value: z.param.value as i32 }
    } }
    fn has_data(e: EventType) -> bool {
         matches!(e,
             EventType::Tempo)
    }
    fn set_data(&self, ev: &mut Event) { unsafe {
         let z = &mut ev.0.data.queue;
         z.queue = self.queue as c_uchar;
         z.param.value = self.value as c_int;
    } }
}

impl EventData for EvQueueControl<u32> {
    fn get_data(ev: &Event) -> Self { unsafe {
         let mut d = ptr::read(&ev.0.data);
         let z = &mut d.queue;
         EvQueueControl { queue: z.queue as i32, value: z.param.position as u32 }
    } }
    fn has_data(e: EventType) -> bool {
         matches!(e,
             EventType::SyncPos |
             EventType::Tick |
             EventType::SetposTick)
    }
    fn set_data(&self, ev: &mut Event) { unsafe {
         let z = &mut ev.0.data.queue;
         z.queue = self.queue as c_uchar;
         z.param.position = self.value as c_uint;
    } }
}

impl EventData for EvQueueControl<time::Duration> {
    fn get_data(ev: &Event) -> Self { unsafe {
         let mut d = ptr::read(&ev.0.data);
         let z = &mut d.queue;
         let t = &mut z.param.time.time;
         EvQueueControl { queue: z.queue as i32, value: time::Duration::new(t.tv_sec as u64, t.tv_nsec as u32) }
    } }
    fn has_data(e: EventType) -> bool {
         matches!(e,
             EventType::SetposTime)
    }
    fn set_data(&self, ev: &mut Event) { unsafe {
         let z = &mut ev.0.data.queue;
         z.queue = self.queue as c_uchar;
         let t = &mut z.param.time.time;
         t.tv_sec = self.value.as_secs() as c_uint;
         t.tv_nsec = self.value.subsec_nanos() as c_uint;
    } }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
/// [snd_seq_result_t](http://www.alsa-project.org/alsa-doc/alsa-lib/structsnd__seq__result__t.html) wrapper
///
/// It's called EvResult instead of Result, in order to not be confused with Rust's Result type.
pub struct EvResult {
    pub event: i32,
    pub result: i32,
}

impl EventData for EvResult {
    fn get_data(ev: &Event) -> Self {
         let d = unsafe { ptr::read(&ev.0.data) };
         let z = unsafe { &d.result };
         EvResult { event: z.event as i32, result: z.result as i32 }
    }
    fn has_data(e: EventType) -> bool {
         matches!(e,
             EventType::System |
             EventType::Result)
    }
    fn set_data(&self, ev: &mut Event) {
         let z = unsafe { &mut ev.0.data.result };
         z.event = self.event as c_int;
         z.result = self.result as c_int;
    }
}



alsa_enum!(
    /// [SND_SEQ_EVENT_xxx](http://www.alsa-project.org/alsa-doc/alsa-lib/group___seq_events.html) constants

    EventType, ALL_EVENT_TYPES[59],

    Bounce = SND_SEQ_EVENT_BOUNCE,
    Chanpress = SND_SEQ_EVENT_CHANPRESS,
    ClientChange = SND_SEQ_EVENT_CLIENT_CHANGE,
    ClientExit = SND_SEQ_EVENT_CLIENT_EXIT,
    ClientStart = SND_SEQ_EVENT_CLIENT_START,
    Clock = SND_SEQ_EVENT_CLOCK,
    Continue = SND_SEQ_EVENT_CONTINUE,
    Control14 = SND_SEQ_EVENT_CONTROL14,
    Controller = SND_SEQ_EVENT_CONTROLLER,
    Echo = SND_SEQ_EVENT_ECHO,
    Keypress = SND_SEQ_EVENT_KEYPRESS,
    Keysign = SND_SEQ_EVENT_KEYSIGN,
    None = SND_SEQ_EVENT_NONE,
    Nonregparam = SND_SEQ_EVENT_NONREGPARAM,
    Note = SND_SEQ_EVENT_NOTE,
    Noteoff = SND_SEQ_EVENT_NOTEOFF,
    Noteon = SND_SEQ_EVENT_NOTEON,
    Oss = SND_SEQ_EVENT_OSS,
    Pgmchange = SND_SEQ_EVENT_PGMCHANGE,
    Pitchbend = SND_SEQ_EVENT_PITCHBEND,
    PortChange = SND_SEQ_EVENT_PORT_CHANGE,
    PortExit = SND_SEQ_EVENT_PORT_EXIT,
    PortStart = SND_SEQ_EVENT_PORT_START,
    PortSubscribed = SND_SEQ_EVENT_PORT_SUBSCRIBED,
    PortUnsubscribed = SND_SEQ_EVENT_PORT_UNSUBSCRIBED,
    Qframe = SND_SEQ_EVENT_QFRAME,
    QueueSkew = SND_SEQ_EVENT_QUEUE_SKEW,
    Regparam = SND_SEQ_EVENT_REGPARAM,
    Reset = SND_SEQ_EVENT_RESET,
    Result = SND_SEQ_EVENT_RESULT,
    Sensing = SND_SEQ_EVENT_SENSING,
    SetposTick = SND_SEQ_EVENT_SETPOS_TICK,
    SetposTime = SND_SEQ_EVENT_SETPOS_TIME,
    Songpos = SND_SEQ_EVENT_SONGPOS,
    Songsel = SND_SEQ_EVENT_SONGSEL,
    Start = SND_SEQ_EVENT_START,
    Stop = SND_SEQ_EVENT_STOP,
    SyncPos = SND_SEQ_EVENT_SYNC_POS,
    Sysex = SND_SEQ_EVENT_SYSEX,
    System = SND_SEQ_EVENT_SYSTEM,
    Tempo = SND_SEQ_EVENT_TEMPO,
    Tick = SND_SEQ_EVENT_TICK,
    Timesign = SND_SEQ_EVENT_TIMESIGN,
    TuneRequest = SND_SEQ_EVENT_TUNE_REQUEST,
    Usr0 = SND_SEQ_EVENT_USR0,
    Usr1 = SND_SEQ_EVENT_USR1,
    Usr2 = SND_SEQ_EVENT_USR2,
    Usr3 = SND_SEQ_EVENT_USR3,
    Usr4 = SND_SEQ_EVENT_USR4,
    Usr5 = SND_SEQ_EVENT_USR5,
    Usr6 = SND_SEQ_EVENT_USR6,
    Usr7 = SND_SEQ_EVENT_USR7,
    Usr8 = SND_SEQ_EVENT_USR8,
    Usr9 = SND_SEQ_EVENT_USR9,
    UsrVar0 = SND_SEQ_EVENT_USR_VAR0,
    UsrVar1 = SND_SEQ_EVENT_USR_VAR1,
    UsrVar2 = SND_SEQ_EVENT_USR_VAR2,
    UsrVar3 = SND_SEQ_EVENT_USR_VAR3,
    UsrVar4 = SND_SEQ_EVENT_USR_VAR4,
);

/// [snd_seq_queue_tempo_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___seq_queue.html) wrapper
pub struct QueueTempo(*mut alsa::snd_seq_queue_tempo_t);

unsafe impl Send for QueueTempo {}

impl Drop for QueueTempo {
    fn drop(&mut self) { unsafe { alsa::snd_seq_queue_tempo_free(self.0) } }
}

impl QueueTempo {
    fn new() -> Result<Self> {
        let mut q = ptr::null_mut();
        acheck!(snd_seq_queue_tempo_malloc(&mut q)).map(|_| QueueTempo(q))
    }

    /// Creates a new QueueTempo with all fields set to zero.
    pub fn empty() -> Result<Self> {
        let q = QueueTempo::new()?;
        unsafe { ptr::write_bytes(q.0 as *mut u8, 0, alsa::snd_seq_queue_tempo_sizeof()) };
        Ok(q)
    }

    pub fn get_queue(&self) -> i32 { unsafe { alsa::snd_seq_queue_tempo_get_queue(self.0) as i32 } }
    pub fn get_tempo(&self) -> u32 { unsafe { alsa::snd_seq_queue_tempo_get_tempo(self.0) as u32 } }
    pub fn get_ppq(&self) -> i32 { unsafe { alsa::snd_seq_queue_tempo_get_ppq(self.0) as i32 } }
    pub fn get_skew(&self) -> u32 { unsafe { alsa::snd_seq_queue_tempo_get_skew(self.0) as u32 } }
    pub fn get_skew_base(&self) -> u32 { unsafe { alsa::snd_seq_queue_tempo_get_skew_base(self.0) as u32 } }

//    pub fn set_queue(&self, value: i32) { unsafe { alsa::snd_seq_queue_tempo_set_queue(self.0, value as c_int) } }
    pub fn set_tempo(&self, value: u32) { unsafe { alsa::snd_seq_queue_tempo_set_tempo(self.0, value as c_uint) } }
    pub fn set_ppq(&self, value: i32) { unsafe { alsa::snd_seq_queue_tempo_set_ppq(self.0, value as c_int) } }
    pub fn set_skew(&self, value: u32) { unsafe { alsa::snd_seq_queue_tempo_set_skew(self.0, value as c_uint) } }
    pub fn set_skew_base(&self, value: u32) { unsafe { alsa::snd_seq_queue_tempo_set_skew_base(self.0, value as c_uint) } }
}

/// [snd_seq_queue_status_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___seq_queue.html) wrapper
pub struct QueueStatus(*mut alsa::snd_seq_queue_status_t);

unsafe impl Send for QueueStatus {}

impl Drop for QueueStatus {
    fn drop(&mut self) { unsafe { alsa::snd_seq_queue_status_free(self.0) } }
}

impl QueueStatus {
    fn new() -> Result<Self> {
        let mut q = ptr::null_mut();
        acheck!(snd_seq_queue_status_malloc(&mut q)).map(|_| QueueStatus(q))
    }

    /// Creates a new QueueStatus with all fields set to zero.
    pub fn empty() -> Result<Self> {
        let q = QueueStatus::new()?;
        unsafe { ptr::write_bytes(q.0 as *mut u8, 0, alsa::snd_seq_queue_status_sizeof()) };
        Ok(q)
    }

    pub fn get_queue(&self) -> i32 { unsafe { alsa::snd_seq_queue_status_get_queue(self.0) as i32 } }
    pub fn get_events(&self) -> i32 { unsafe { alsa::snd_seq_queue_status_get_events(self.0) as i32 } }
    pub fn get_tick_time(&self) -> u32 { unsafe {alsa::snd_seq_queue_status_get_tick_time(self.0) as u32 } }
    pub fn get_real_time(&self) -> time::Duration { unsafe {
        let t = &(*alsa::snd_seq_queue_status_get_real_time(self.0));
        time::Duration::new(t.tv_sec as u64, t.tv_nsec as u32)
    } }
    pub fn get_status(&self) -> u32 { unsafe { alsa::snd_seq_queue_status_get_status(self.0) as u32 } }
}

/// [snd_seq_remove_events_t](https://www.alsa-project.org/alsa-doc/alsa-lib/group___seq_event.html) wrapper
pub struct RemoveEvents(*mut alsa::snd_seq_remove_events_t);

unsafe impl Send for RemoveEvents {}

impl Drop for RemoveEvents {
    fn drop(&mut self) { unsafe { alsa::snd_seq_remove_events_free(self.0) } }
}

impl RemoveEvents {
    pub fn new() -> Result<Self> {
        let mut q = ptr::null_mut();
        acheck!(snd_seq_remove_events_malloc(&mut q)).map(|_| RemoveEvents(q))
    }

    pub fn get_condition(&self) -> Remove { unsafe {
        Remove::from_bits_truncate(alsa::snd_seq_remove_events_get_condition(self.0) as u32)
    } }
    pub fn get_queue(&self) -> i32 { unsafe { alsa::snd_seq_remove_events_get_queue(self.0) as i32 } }
    pub fn get_time(&self) -> time::Duration { unsafe {
        let d = ptr::read(alsa::snd_seq_remove_events_get_time(self.0));
        let t = &d.time;

        time::Duration::new(t.tv_sec as u64, t.tv_nsec as u32)
    } }
    pub fn get_dest(&self) -> Addr { unsafe {
        let a = &(*alsa::snd_seq_remove_events_get_dest(self.0));

        Addr { client: a.client as i32, port: a.port as i32 }
    } }
    pub fn get_channel(&self) -> i32 { unsafe { alsa::snd_seq_remove_events_get_channel(self.0) as i32 } }
    pub fn get_event_type(&self) -> Result<EventType> { unsafe {
        EventType::from_c_int(alsa::snd_seq_remove_events_get_event_type(self.0), "snd_seq_remove_events_get_event_type")
    } }
    pub fn get_tag(&self) -> u8 { unsafe { alsa::snd_seq_remove_events_get_tag(self.0) as u8 } }


    pub fn set_condition(&self, value: Remove) { unsafe {
        alsa::snd_seq_remove_events_set_condition(self.0, value.bits() as c_uint);
    } }
    pub fn set_queue(&self, value: i32) { unsafe { alsa::snd_seq_remove_events_set_queue(self.0, value as c_int) } }
    pub fn set_time(&self, value: time::Duration) { unsafe {
        let mut d: alsa::snd_seq_timestamp_t = mem::zeroed();
        let t = &mut d.time;

        t.tv_sec = value.as_secs() as c_uint;
        t.tv_nsec = value.subsec_nanos() as c_uint;

        alsa::snd_seq_remove_events_set_time(self.0, &d);
    } }
    pub fn set_dest(&self, value: Addr) { unsafe {
        let a = alsa::snd_seq_addr_t { client: value.client as c_uchar, port: value.port as c_uchar};

        alsa::snd_seq_remove_events_set_dest(self.0, &a);
    } }
    pub fn set_channel(&self, value: i32) { unsafe { alsa::snd_seq_remove_events_set_channel(self.0, value as c_int) } }
    pub fn set_event_type(&self, value: EventType) { unsafe { alsa::snd_seq_remove_events_set_event_type(self.0, value as i32); } }
    pub fn set_tag(&self, value: u8) { unsafe { alsa::snd_seq_remove_events_set_tag(self.0, value as c_int) } }
}

/// [snd_midi_event_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___m_i_d_i___event.html) Wrapper
///
/// Sequencer event <-> MIDI byte stream coder
pub struct MidiEvent(*mut alsa::snd_midi_event_t);

impl Drop for MidiEvent {
    fn drop(&mut self) { unsafe { alsa::snd_midi_event_free(self.0) } }
}

impl MidiEvent {
    pub fn new(bufsize: u32) -> Result<MidiEvent> {
        let mut q = ptr::null_mut();
        acheck!(snd_midi_event_new(bufsize as size_t, &mut q)).map(|_| MidiEvent(q))
    }

    pub fn resize_buffer(&self, bufsize: u32) -> Result<()> { acheck!(snd_midi_event_resize_buffer(self.0, bufsize as size_t)).map(|_| ()) }

    /// Note: this corresponds to snd_midi_event_no_status, but on and off are switched.
    ///
    /// Alsa-lib is a bit confusing here. Anyhow, set "enable" to true to enable running status.
    pub fn enable_running_status(&self, enable: bool) { unsafe { alsa::snd_midi_event_no_status(self.0, if enable {0} else {1}) } }

    /// Resets both encoder and decoder
    pub fn init(&self) { unsafe { alsa::snd_midi_event_init(self.0) } }

    pub fn reset_encode(&self) { unsafe { alsa::snd_midi_event_reset_encode(self.0) } }

    pub fn reset_decode(&self) { unsafe { alsa::snd_midi_event_reset_decode(self.0) } }

    pub fn decode(&self, buf: &mut [u8], ev: &mut Event) -> Result<usize> {
        ev.ensure_buf();
        acheck!(snd_midi_event_decode(self.0, buf.as_mut_ptr() as *mut c_uchar, buf.len() as c_long, &ev.0)).map(|r| r as usize)
    }

    /// In case of success, returns a tuple of (bytes consumed from buf, found Event).
    pub fn encode<'a>(&'a mut self, buf: &[u8]) -> Result<(usize, Option<Event<'a>>)> {
        // The ALSA documentation clearly states that the event will be valid as long as the Encoder
        // is not messed with (because the data pointer for sysex events may point into the Encoder's
        // buffer). We make this safe by taking self by unique reference and coupling it to
        // the event's lifetime.
        let mut ev = unsafe { mem::zeroed() };
        let r = acheck!(snd_midi_event_encode(self.0, buf.as_ptr() as *const c_uchar, buf.len() as c_long, &mut ev))?;
        let e = if ev.type_ == alsa::SND_SEQ_EVENT_NONE as u8 {
                None
            } else {
                Some(unsafe { Event::extract(&mut ev, "snd_midi_event_encode") }?)
            };
        Ok((r as usize, e))
    }
}

#[test]
fn print_seqs() {
    use std::ffi::CString;
    let s = super::Seq::open(None, None, false).unwrap();
    s.set_client_name(&CString::new("rust_test_print_seqs").unwrap()).unwrap();
    let clients: Vec<_> = ClientIter::new(&s).collect();
    for a in &clients {
        let ports: Vec<_> = PortIter::new(&s, a.get_client()).collect();
        println!("{:?}: {:?}", a, ports);
    }
}

#[test]
fn seq_subscribe() {
    use std::ffi::CString;
    let s = super::Seq::open(None, None, false).unwrap();
    s.set_client_name(&CString::new("rust_test_seq_subscribe").unwrap()).unwrap();
    let timer_info = s.get_any_port_info(Addr { client: 0, port: 0 }).unwrap();
    assert_eq!(timer_info.get_name().unwrap(), "Timer");
    let info = PortInfo::empty().unwrap();
    let _port = s.create_port(&info);
    let subs = PortSubscribe::empty().unwrap();
    subs.set_sender(Addr { client: 0, port: 0 });
    subs.set_dest(Addr { client: s.client_id().unwrap(), port: info.get_port() });
    s.subscribe_port(&subs).unwrap();
}

#[test]
fn seq_loopback() {
    use std::ffi::CString;
    let s = super::Seq::open(Some(&CString::new("default").unwrap()), None, false).unwrap();
    s.set_client_name(&CString::new("rust_test_seq_loopback").unwrap()).unwrap();

    // Create ports
    let sinfo = PortInfo::empty().unwrap();
    sinfo.set_capability(PortCap::READ | PortCap::SUBS_READ);
    sinfo.set_type(PortType::MIDI_GENERIC | PortType::APPLICATION);
    s.create_port(&sinfo).unwrap();
    let sport = sinfo.get_port();
    let dinfo = PortInfo::empty().unwrap();
    dinfo.set_capability(PortCap::WRITE | PortCap::SUBS_WRITE);
    dinfo.set_type(PortType::MIDI_GENERIC | PortType::APPLICATION);
    s.create_port(&dinfo).unwrap();
    let dport = dinfo.get_port();

    // Connect them
    let subs = PortSubscribe::empty().unwrap();
    subs.set_sender(Addr { client: s.client_id().unwrap(), port: sport });
    subs.set_dest(Addr { client: s.client_id().unwrap(), port: dport });
    s.subscribe_port(&subs).unwrap();
    println!("Connected {:?} to {:?}", subs.get_sender(), subs.get_dest());

    // Send a note!
    let note = EvNote { channel: 0, note: 64, duration: 100, velocity: 100, off_velocity: 64 };
    let mut e = Event::new(EventType::Noteon, &note);
    e.set_subs();
    e.set_direct();
    e.set_source(sport);
    println!("Sending {:?}", e);
    s.event_output(&mut e).unwrap();
    s.drain_output().unwrap();

    // Receive the note!
    let mut input = s.input();
    let e2 = input.event_input().unwrap();
    println!("Receiving {:?}", e2);
    assert_eq!(e2.get_type(), EventType::Noteon);
    assert_eq!(e2.get_data(), Some(note));
}

#[test]
fn seq_encode_sysex() {
    let mut me = MidiEvent::new(16).unwrap();
    let sysex = &[0xf0, 1, 2, 3, 4, 5, 6, 7, 0xf7];
    let (s, ev) = me.encode(sysex).unwrap();
    assert_eq!(s, 9);
    let ev = ev.unwrap();
    let v = ev.get_ext().unwrap();
    assert_eq!(&*v, sysex);
}

#[test]
fn seq_decode_sysex() {
    let sysex = [0xf0, 1, 2, 3, 4, 5, 6, 7, 0xf7];
    let mut ev = Event::new_ext(EventType::Sysex, &sysex[..]);
    let me = MidiEvent::new(0).unwrap();
    let mut buffer = vec![0; sysex.len()];
    assert_eq!(me.decode(&mut buffer[..], &mut ev).unwrap(), sysex.len());
    assert_eq!(buffer, sysex);
}

#[test]
#[should_panic]
fn seq_get_input_twice() {
    use std::ffi::CString;
    let s = super::Seq::open(None, None, false).unwrap();
    s.set_client_name(&CString::new("rust_test_seq_get_input_twice").unwrap()).unwrap();
    let input1 = s.input();
    let input2 = s.input(); // this should panic
    let _ = (input1, input2);
}

#[test]
fn seq_has_data() {
    for v in EventType::all() {
        let v = *v;
        let mut i = 0;
        if <() as EventData>::has_data(v) { i += 1; }
        if <[u8; 12] as EventData>::has_data(v) { i += 1; }
        if Event::has_ext_data(v) { i += 1; }
        if EvNote::has_data(v) { i += 1; }
        if EvCtrl::has_data(v) { i += 1; }
        if Addr::has_data(v) { i += 1; }
        if Connect::has_data(v) { i += 1; }
        if EvResult::has_data(v) { i += 1; }
        if EvQueueControl::<()>::has_data(v) { i += 1; }
        if EvQueueControl::<u32>::has_data(v) { i += 1; }
        if EvQueueControl::<i32>::has_data(v) { i += 1; }
        if EvQueueControl::<time::Duration>::has_data(v) { i += 1; }
        if i != 1 { panic!("{:?}: {} has_data", v, i) }
    }
}

#[test]
fn seq_remove_events() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let info = RemoveEvents::new()?;


    info.set_condition(Remove::INPUT | Remove::DEST | Remove::TIME_BEFORE | Remove::TAG_MATCH);
    info.set_queue(123);
    info.set_time(time::Duration::new(456, 789));
    info.set_dest(Addr { client: 212, port: 121 });
    info.set_channel(15);
    info.set_event_type(EventType::Noteon);
    info.set_tag(213);

    assert_eq!(info.get_condition(), Remove::INPUT | Remove::DEST | Remove::TIME_BEFORE | Remove::TAG_MATCH);
    assert_eq!(info.get_queue(), 123);
    assert_eq!(info.get_time(), time::Duration::new(456, 789));
    assert_eq!(info.get_dest(), Addr { client: 212, port: 121 });
    assert_eq!(info.get_channel(), 15);
    assert_eq!(info.get_event_type()?, EventType::Noteon);
    assert_eq!(info.get_tag(), 213);

    Ok(())
}

#[test]
fn seq_portsubscribeiter() {
    let s = super::Seq::open(None, None, false).unwrap();

    // Create ports
    let sinfo = PortInfo::empty().unwrap();
    sinfo.set_capability(PortCap::READ | PortCap::SUBS_READ);
    sinfo.set_type(PortType::MIDI_GENERIC | PortType::APPLICATION);
    s.create_port(&sinfo).unwrap();
    let sport = sinfo.get_port();
    let dinfo = PortInfo::empty().unwrap();
    dinfo.set_capability(PortCap::WRITE | PortCap::SUBS_WRITE);
    dinfo.set_type(PortType::MIDI_GENERIC | PortType::APPLICATION);
    s.create_port(&dinfo).unwrap();
    let dport = dinfo.get_port();

    // Connect them
    let subs = PortSubscribe::empty().unwrap();
    subs.set_sender(Addr { client: s.client_id().unwrap(), port: sport });
    subs.set_dest(Addr { client: s.client_id().unwrap(), port: dport });
    s.subscribe_port(&subs).unwrap();

    // Query READ subs from sport's point of view
    let read_subs: Vec<PortSubscribe> = PortSubscribeIter::new(&s,
                        Addr {client: s.client_id().unwrap(), port: sport },
                        QuerySubsType::READ).collect();
    assert_eq!(read_subs.len(), 1);
    assert_eq!(read_subs[0].get_sender(), subs.get_sender());
    assert_eq!(read_subs[0].get_dest(), subs.get_dest());

    let write_subs: Vec<PortSubscribe> = PortSubscribeIter::new(&s,
                        Addr {client: s.client_id().unwrap(), port: sport },
                        QuerySubsType::WRITE).collect();
    assert_eq!(write_subs.len(), 0);

    // Now query WRITE subs from dport's point of view
    let write_subs: Vec<PortSubscribe> = PortSubscribeIter::new(&s,
                        Addr {client: s.client_id().unwrap(), port: dport },
                        QuerySubsType::WRITE).collect();
    assert_eq!(write_subs.len(), 1);
    assert_eq!(write_subs[0].get_sender(), subs.get_sender());
    assert_eq!(write_subs[0].get_dest(), subs.get_dest());
}
