//! Audio playback and capture
//!
//! # Example
//! Playback a sine wave through the "default" device.
//!
//! ```
//! use alsa::{Direction, ValueOr};
//! use alsa::pcm::{PCM, HwParams, Format, Access, State};
//!
//! // Open default playback device
//! let pcm = PCM::new("default", Direction::Playback, false).unwrap();
//!
//! // Set hardware parameters: 44100 Hz / Mono / 16 bit
//! let hwp = HwParams::any(&pcm).unwrap();
//! hwp.set_channels(1).unwrap();
//! hwp.set_rate(44100, ValueOr::Nearest).unwrap();
//! hwp.set_format(Format::s16()).unwrap();
//! hwp.set_access(Access::RWInterleaved).unwrap();
//! pcm.hw_params(&hwp).unwrap();
//! let io = pcm.io_i16().unwrap();
//!
//! // Make sure we don't start the stream too early
//! let hwp = pcm.hw_params_current().unwrap();
//! let swp = pcm.sw_params_current().unwrap();
//! swp.set_start_threshold(hwp.get_buffer_size().unwrap()).unwrap();
//! pcm.sw_params(&swp).unwrap();
//!
//! // Make a sine wave
//! let mut buf = [0i16; 1024];
//! for (i, a) in buf.iter_mut().enumerate() {
//!     *a = ((i as f32 * 2.0 * ::std::f32::consts::PI / 128.0).sin() * 8192.0) as i16
//! }
//!
//! // Play it back for 2 seconds.
//! for _ in 0..2*44100/1024 {
//!     assert_eq!(io.writei(&buf[..]).unwrap(), 1024);
//! }
//!
//! // In case the buffer was larger than 2 seconds, start the stream manually.
//! if pcm.state() != State::Running { pcm.start().unwrap() };
//! // Wait for the stream to finish playback.
//! pcm.drain().unwrap();
//! ```


use libc::{c_int, c_uint, c_void, ssize_t, c_short, timespec, pollfd};
use crate::alsa;
use std::convert::Infallible;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ffi::{CStr, CString};
use std::str::FromStr;
use std::{io, fmt, ptr, cell};
use super::error::*;
use super::{Direction, Output, poll, ValueOr, chmap};

pub use super::chmap::{Chmap, ChmapPosition, ChmapType, ChmapsQuery};

/// [snd_pcm_sframes_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m.html)
pub type Frames = alsa::snd_pcm_sframes_t;

/// [snd_pcm_info_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m.html) wrapper - PCM generic info container
pub struct Info(pub(crate) *mut alsa::snd_pcm_info_t);

impl Info {
    pub fn new() -> Result<Info> {
        let mut p = ptr::null_mut();
        acheck!(snd_pcm_info_malloc(&mut p)).map(|_| Info(p))
    }

    pub fn get_card(&self) -> i32 {
        unsafe { alsa::snd_pcm_info_get_card(self.0) }
    }

    pub fn get_device(&self) -> u32 {
        unsafe { alsa::snd_pcm_info_get_device(self.0) }
    }

    pub fn get_subdevice(&self) -> u32 {
        unsafe { alsa::snd_pcm_info_get_subdevice(self.0) }
    }

    pub fn get_id(&self) -> Result<&str> {
        let c = unsafe { alsa::snd_pcm_info_get_id(self.0) };
        from_const("snd_pcm_info_get_id", c)
    }

    pub fn get_name(&self) -> Result<&str> {
        let c = unsafe { alsa::snd_pcm_info_get_name(self.0) };
        from_const("snd_pcm_info_get_name", c)
    }

    pub fn get_subdevice_name(&self) -> Result<&str> {
        let c = unsafe { alsa::snd_pcm_info_get_subdevice_name(self.0) };
        from_const("snd_pcm_info_get_subdevice_name", c)
    }

    pub fn get_stream(&self) -> Direction {
        match unsafe { alsa::snd_pcm_info_get_stream(self.0) } {
            alsa::SND_PCM_STREAM_CAPTURE => Direction::Capture,
            alsa::SND_PCM_STREAM_PLAYBACK => Direction::Playback,
            n @ _ => panic!("snd_pcm_info_get_stream invalid direction '{}'", n),
        }
    }

    pub fn get_subdevices_count(&self) -> u32 {
        unsafe { alsa::snd_pcm_info_get_subdevices_count(self.0) }
    }

    pub fn get_subdevices_avail(&self) -> u32 {
        unsafe { alsa::snd_pcm_info_get_subdevices_avail(self.0) }
    }

    pub(crate) fn set_device(&mut self, device: u32) {
        unsafe { alsa::snd_pcm_info_set_device(self.0, device) }
    }

    pub(crate) fn set_stream(&mut self, direction: Direction) {
        let stream = match direction {
            Direction::Capture => alsa::SND_PCM_STREAM_CAPTURE,
            Direction::Playback => alsa::SND_PCM_STREAM_PLAYBACK,
        };
        unsafe { alsa::snd_pcm_info_set_stream(self.0, stream) }
    }

    pub(crate) fn set_subdevice(&mut self, subdevice: u32) {
        unsafe { alsa::snd_pcm_info_set_subdevice(self.0, subdevice) }
    }
}

impl Drop for Info {
    fn drop(&mut self) { unsafe { alsa::snd_pcm_info_free(self.0) }; }
}

/// [snd_pcm_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m.html) wrapper - start here for audio playback and recording
pub struct PCM(*mut alsa::snd_pcm_t, cell::Cell<bool>);

unsafe impl Send for PCM {}

impl PCM {
    fn check_has_io(&self) {
        if self.1.get() { panic!("No hw_params call or additional IO objects allowed") }
    }

    /// Wrapper around open that takes a &str instead of a &CStr
    pub fn new(name: &str, dir: Direction, nonblock: bool) -> Result<PCM> {
        Self::open(&CString::new(name).unwrap(), dir, nonblock)
    }

    // Does not offer async mode (it's not very Rustic anyway)
    pub fn open(name: &CStr, dir: Direction, nonblock: bool) -> Result<PCM> {
        let mut r = ptr::null_mut();
        let stream = match dir {
            Direction::Capture => alsa::SND_PCM_STREAM_CAPTURE,
            Direction::Playback => alsa::SND_PCM_STREAM_PLAYBACK
        };
        let flags = if nonblock { alsa::SND_PCM_NONBLOCK } else { 0 };
        acheck!(snd_pcm_open(&mut r, name.as_ptr(), stream, flags)).map(|_| PCM(r, cell::Cell::new(false)))
    }

    pub fn start(&self) -> Result<()> { acheck!(snd_pcm_start(self.0)).map(|_| ()) }
    pub fn drop(&self) -> Result<()> { acheck!(snd_pcm_drop(self.0)).map(|_| ()) }
    pub fn pause(&self, pause: bool) -> Result<()> {
        acheck!(snd_pcm_pause(self.0, if pause { 1 } else { 0 })).map(|_| ()) }
    pub fn resume(&self) -> Result<()> { acheck!(snd_pcm_resume(self.0)).map(|_| ()) }
    pub fn drain(&self) -> Result<()> { acheck!(snd_pcm_drain(self.0)).map(|_| ()) }
    pub fn prepare(&self) -> Result<()> { acheck!(snd_pcm_prepare(self.0)).map(|_| ()) }
    pub fn reset(&self) -> Result<()> { acheck!(snd_pcm_reset(self.0)).map(|_| ()) }
    pub fn recover(&self, err: c_int, silent: bool) -> Result<()> {
        acheck!(snd_pcm_recover(self.0, err, if silent { 1 } else { 0 })).map(|_| ()) }

    /// Wrapper around snd_pcm_recover.
    ///
    /// Returns Ok if the error was successfully recovered from, or the original
    /// error if the error was unhandled.
    pub fn try_recover(&self, err: Error, silent: bool) -> Result<()> {
        self.recover(err.errno() as c_int, silent)
    }

    pub fn wait(&self, timeout_ms: Option<u32>) -> Result<bool> {
        acheck!(snd_pcm_wait(self.0, timeout_ms.map(|x| x as c_int).unwrap_or(-1))).map(|i| i == 1) }

    pub fn state(&self) -> State {
        let rawstate = self.state_raw();
        if let Ok(state) = State::from_c_int(rawstate, "snd_pcm_state") {
            state
        }
        else {
            panic!("snd_pcm_state returned an invalid value of {}", rawstate);
        }
    }

    /// Only used internally, and for debugging the alsa library. Please use the "state" function instead.
    pub fn state_raw(&self) -> c_int { unsafe { alsa::snd_pcm_state(self.0) as c_int } }

    pub fn bytes_to_frames(&self, i: isize) -> Frames { unsafe { alsa::snd_pcm_bytes_to_frames(self.0, i as ssize_t) }}
    pub fn frames_to_bytes(&self, i: Frames) -> isize { unsafe { alsa::snd_pcm_frames_to_bytes(self.0, i) as isize }}

    pub fn avail_update(&self) -> Result<Frames> { acheck!(snd_pcm_avail_update(self.0)) }
    pub fn avail(&self) -> Result<Frames> { acheck!(snd_pcm_avail(self.0)) }

    pub fn avail_delay(&self) -> Result<(Frames, Frames)> {
        let (mut a, mut d) = (0, 0);
        acheck!(snd_pcm_avail_delay(self.0, &mut a, &mut d)).map(|_| (a, d))
    }
    pub fn delay(&self) -> Result<Frames> {
        let mut d = 0;
        acheck!(snd_pcm_delay(self.0, &mut d)).map(|_| d)
    }

    pub fn status(&self) -> Result<Status> {
        StatusBuilder::new().build(self)
    }

    fn verify_format(&self, f: Format) -> Result<()> {
        let ff = self.hw_params_current().and_then(|h| h.get_format())?;
        if ff == f { Ok(()) }
        else {
            // let s = format!("Invalid sample format ({:?}, expected {:?})", ff, f);
            Err(Error::unsupported("io_xx"))
        }
    }

    pub fn io_i8(&self) -> Result<IO<i8>> { self.io_checked() }
    pub fn io_u8(&self) -> Result<IO<u8>> { self.io_checked() }
    pub fn io_i16(&self) -> Result<IO<i16>> { self.io_checked() }
    pub fn io_u16(&self) -> Result<IO<u16>> { self.io_checked() }
    pub fn io_i32(&self) -> Result<IO<i32>> { self.io_checked() }
    pub fn io_u32(&self) -> Result<IO<u32>> { self.io_checked() }
    pub fn io_f32(&self) -> Result<IO<f32>> { self.io_checked() }
    pub fn io_f64(&self) -> Result<IO<f64>> { self.io_checked() }

    /// For the `s24` format, represented by i32
    pub fn io_i32_s24(&self) -> Result<IO<i32>> { self.verify_format(Format::s24()).map(|_| IO::new(self)) }
    /// For the `u24` format, represented by u32
    pub fn io_u32_u24(&self) -> Result<IO<u32>> { self.verify_format(Format::u24()).map(|_| IO::new(self)) }

    pub fn io_checked<S: IoFormat>(&self) -> Result<IO<S>> {
        self.verify_format(S::FORMAT).map(|_| IO::new(self))
    }

    /// Creates IO without checking `S` is valid type.
    ///
    /// SAFETY: Caller must guarantee `S` is valid type for this PCM stream
    /// and that no other IO objects exist at the same time for the same stream
    /// (or in some other way guarantee mmap safety)
    pub unsafe fn io_unchecked<S: IoFormat>(&self) -> IO<S> {
        IO::new_unchecked(self)
    }

    #[deprecated(note = "renamed to io_bytes")]
    pub fn io(&self) -> IO<u8> { IO::new(self) }

    /// Call this if you have an unusual format, not supported by the regular access methods
    /// (io_i16 etc). It will succeed regardless of the sample format, but conversion to and from
    /// bytes to your format is up to you.
    pub fn io_bytes(&self) -> IO<u8> { IO::new(self) }

    /// Read buffers by talking to the kernel directly, bypassing alsa-lib.
    pub fn direct_mmap_capture<S>(&self) -> Result<crate::direct::pcm::MmapCapture<S>> {
        self.check_has_io();
        crate::direct::pcm::new_mmap(self)
    }

    /// Write buffers by talking to the kernel directly, bypassing alsa-lib.
    pub fn direct_mmap_playback<S>(&self) -> Result<crate::direct::pcm::MmapPlayback<S>> {
        self.check_has_io();
        crate::direct::pcm::new_mmap(self)
    }

    /// Sets hw parameters. Note: No IO object can exist for this PCM
    /// when hw parameters are set.
    pub fn hw_params(&self, h: &HwParams) -> Result<()> {
        self.check_has_io();
        acheck!(snd_pcm_hw_params(self.0, h.0)).map(|_| ())
    }

    /// Retreive current PCM hardware configuration.
    pub fn hw_params_current(&self) -> Result<HwParams> {
        HwParams::new(self).and_then(|h|
            acheck!(snd_pcm_hw_params_current(self.0, h.0)).map(|_| h))
    }

    pub fn sw_params(&self, h: &SwParams) -> Result<()> {
        acheck!(snd_pcm_sw_params(self.0, h.0)).map(|_| ())
    }

    pub fn sw_params_current(&self) -> Result<SwParams> {
        SwParams::new(self).and_then(|h|
            acheck!(snd_pcm_sw_params_current(self.0, h.0)).map(|_| h))
    }

    /// Wraps `snd_pcm_get_params`, returns `(buffer_size, period_size)`.
    pub fn get_params(&self) -> Result<(u64, u64)> {
        let mut buffer_size = 0;
        let mut period_size = 0;
        acheck!(snd_pcm_get_params(self.0, &mut buffer_size, &mut period_size))
            .map(|_| (buffer_size as u64, period_size as u64))

    }

    pub fn info(&self) -> Result<Info> {
        Info::new().and_then(|info|
            acheck!(snd_pcm_info(self.0, info.0)).map(|_| info ))
    }

    pub fn dump(&self, o: &mut Output) -> Result<()> {
        acheck!(snd_pcm_dump(self.0, super::io::output_handle(o))).map(|_| ())
    }

    pub fn dump_hw_setup(&self, o: &mut Output) -> Result<()> {
        acheck!(snd_pcm_dump_hw_setup(self.0, super::io::output_handle(o))).map(|_| ())
    }

    pub fn dump_sw_setup(&self, o: &mut Output) -> Result<()> {
        acheck!(snd_pcm_dump_sw_setup(self.0, super::io::output_handle(o))).map(|_| ())
    }

    pub fn query_chmaps(&self) -> ChmapsQuery {
        chmap::chmaps_query_new(unsafe { alsa::snd_pcm_query_chmaps(self.0) })
    }

    pub fn set_chmap(&self, c: &Chmap) -> Result<()> {
        acheck!(snd_pcm_set_chmap(self.0, chmap::chmap_handle(c))).map(|_| ())
    }

    pub fn get_chmap(&self) -> Result<Chmap> {
        let p = unsafe { alsa::snd_pcm_get_chmap(self.0) };
        if p.is_null() { Err(Error::unsupported("snd_pcm_get_chmap")) }
        else { Ok(chmap::chmap_new(p)) }
    }

    pub fn link(&self, other: &PCM) -> Result<()> {
        acheck!(snd_pcm_link(self.0, other.0)).map(|_| ())
    }

    pub fn unlink(&self) -> Result<()> {
        acheck!(snd_pcm_unlink(self.0)).map(|_| ())
    }
}

impl Drop for PCM {
    fn drop(&mut self) { unsafe { alsa::snd_pcm_close(self.0) }; }
}


impl poll::Descriptors for PCM {
    fn count(&self) -> usize {
        unsafe { alsa::snd_pcm_poll_descriptors_count(self.0) as usize }
    }
    fn fill(&self, p: &mut [pollfd]) -> Result<usize> {
        let z = unsafe { alsa::snd_pcm_poll_descriptors(self.0, p.as_mut_ptr(), p.len() as c_uint) };
        from_code("snd_pcm_poll_descriptors", z).map(|_| z as usize)
    }
    fn revents(&self, p: &[pollfd]) -> Result<poll::Flags> {
        let mut r = 0;
        let z = unsafe { alsa::snd_pcm_poll_descriptors_revents(self.0, p.as_ptr() as *mut pollfd, p.len() as c_uint, &mut r) };
        from_code("snd_pcm_poll_descriptors_revents", z).map(|_| poll::Flags::from_bits_truncate(r as c_short))
    }
}

/// Sample format dependent struct for reading from and writing data to a `PCM`.
/// Also implements `std::io::Read` and `std::io::Write`.
///
/// Note: Only one IO object is allowed in scope at a time (for mmap safety).
pub struct IO<'a, S: Copy>(&'a PCM, PhantomData<S>);

impl<'a, S: Copy> Drop for IO<'a, S> {
    fn drop(&mut self) { (self.0).1.set(false) }
}

impl<'a, S: Copy> IO<'a, S> {

    fn new(a: &'a PCM) -> IO<'a, S> {
        a.check_has_io();
        a.1.set(true);
        IO(a, PhantomData)
    }

    unsafe fn new_unchecked(a: &'a PCM) -> IO<'a, S> {
        a.1.set(true);
        IO(a, PhantomData)
    }

    fn to_frames(&self, b: usize) -> alsa::snd_pcm_uframes_t {
        // TODO: Do we need to check for overflow here?
        self.0.bytes_to_frames((b * size_of::<S>()) as isize) as alsa::snd_pcm_uframes_t
    }

    fn from_frames(&self, b: alsa::snd_pcm_uframes_t) -> usize {
        // TODO: Do we need to check for overflow here?
        (self.0.frames_to_bytes(b as Frames) as usize) / size_of::<S>()
    }

    /// On success, returns number of *frames* written.
    /// (Multiply with number of channels to get number of items in buf successfully written.)
    pub fn writei(&self, buf: &[S]) -> Result<usize> {
        acheck!(snd_pcm_writei((self.0).0, buf.as_ptr() as *const c_void, self.to_frames(buf.len()))).map(|r| r as usize)
    }

    /// On success, returns number of *frames* read.
    /// (Multiply with number of channels to get number of items in buf successfully read.)
    pub fn readi(&self, buf: &mut [S]) -> Result<usize> {
        acheck!(snd_pcm_readi((self.0).0, buf.as_mut_ptr() as *mut c_void, self.to_frames(buf.len()))).map(|r| r as usize)
    }

    /// Write non-interleaved frames to pcm. On success, returns number of frames written.
    ///
    /// # Safety
    ///
    /// Caller must ensure that the length of `bufs` is at least the number of
    /// channels, and that each element of bufs is a valid pointer to an array
    /// of at least `frames` length.
    pub unsafe fn writen(&self, bufs: &[*const S], frames: usize) -> Result<usize> {
        let frames = frames as alsa::snd_pcm_uframes_t;
        acheck!(snd_pcm_writen((self.0).0, bufs.as_ptr() as *mut *mut c_void, frames)).map(|r| r as usize)
    }

    /// Read non-interleaved frames to pcm. On success, returns number of frames read.
    ///
    /// # Safety
    ///
    /// Caller must ensure that the length of `bufs` is at least the number of
    /// channels, and that each element of bufs is a valid pointer to an array
    /// of at least `frames` length.
    pub unsafe fn readn(&self, bufs: &mut [*mut S], frames: usize) -> Result<usize> {
        let frames = frames as alsa::snd_pcm_uframes_t;
        acheck!(snd_pcm_readn((self.0).0, bufs.as_mut_ptr() as *mut *mut c_void, frames)).map(|r| r as usize)
    }

    /// Wrapper around snd_pcm_mmap_begin and snd_pcm_mmap_commit.
    ///
    /// You can read/write into the sound card's buffer during the call to the closure.
    /// According to alsa-lib docs, you should call avail_update before calling this function.
    ///
    /// All calculations are in *frames*, i e, the closure should return number of frames processed.
    /// Also, there might not be as many frames to read/write as requested, and there can even be
    /// an empty buffer supplied to the closure.
    ///
    /// Note: This function works only with interleaved access mode.
    pub fn mmap<F: FnOnce(&mut [S]) -> usize>(&self, frames: usize, func: F) -> Result<usize> {
        let mut f = frames as alsa::snd_pcm_uframes_t;
        let mut offs: alsa::snd_pcm_uframes_t = 0;
        let mut areas = ptr::null();
        acheck!(snd_pcm_mmap_begin((self.0).0, &mut areas, &mut offs, &mut f))?;

        let (first, step) = unsafe { ((*areas).first, (*areas).step) };
        if first != 0 || step as isize != self.0.frames_to_bytes(1) * 8 {
            unsafe { alsa::snd_pcm_mmap_commit((self.0).0, offs, 0) };
            // let s = format!("Can only mmap a single interleaved buffer (first = {:?}, step = {:?})", first, step);
            return Err(Error::unsupported("snd_pcm_mmap_begin"));
        }

        let buf = unsafe {
            let p = ((*areas).addr as *mut S).add(self.from_frames(offs));
            ::std::slice::from_raw_parts_mut(p, self.from_frames(f))
        };
        let fres = func(buf);
        debug_assert!(fres <= f as usize);
        acheck!(snd_pcm_mmap_commit((self.0).0, offs, fres as alsa::snd_pcm_uframes_t)).map(|r| r as usize)
    }
}

impl<'a, S: Copy> io::Read for IO<'a, S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let size = self.0.bytes_to_frames(buf.len() as isize) as alsa::snd_pcm_uframes_t; // TODO: Do we need to check for overflow here?
        let r = unsafe { alsa::snd_pcm_readi((self.0).0, buf.as_mut_ptr() as *mut c_void, size) };
        if r < 0 { Err(io::Error::from_raw_os_error(r as i32)) }
        else { Ok(self.0.frames_to_bytes(r) as usize) }
    }
}

impl<'a, S: Copy> io::Write for IO<'a, S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let size = self.0.bytes_to_frames(buf.len() as isize) as alsa::snd_pcm_uframes_t; // TODO: Do we need to check for overflow here?
        let r = unsafe { alsa::snd_pcm_writei((self.0).0, buf.as_ptr() as *const c_void, size) };
        if r < 0 { Err(io::Error::from_raw_os_error(r as i32)) }
        else { Ok(self.0.frames_to_bytes(r) as usize) }
    }
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}


alsa_enum!(
    /// [SND_PCM_STATE_xxx](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m.html) constants
    State, ALL_STATES[9],

    Open = SND_PCM_STATE_OPEN,
    Setup = SND_PCM_STATE_SETUP,
    Prepared = SND_PCM_STATE_PREPARED,
    Running = SND_PCM_STATE_RUNNING,
    XRun = SND_PCM_STATE_XRUN,
    Draining = SND_PCM_STATE_DRAINING,
    Paused = SND_PCM_STATE_PAUSED,
    Suspended = SND_PCM_STATE_SUSPENDED,
    Disconnected = SND_PCM_STATE_DISCONNECTED,
);

alsa_enum!(
    #[non_exhaustive]
    /// [SND_PCM_FORMAT_xxx](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m.html) constants
    Format, ALL_FORMATS[52],

    Unknown = SND_PCM_FORMAT_UNKNOWN,
    S8 = SND_PCM_FORMAT_S8,
    U8 = SND_PCM_FORMAT_U8,
    S16LE = SND_PCM_FORMAT_S16_LE,
    S16BE = SND_PCM_FORMAT_S16_BE,
    U16LE = SND_PCM_FORMAT_U16_LE,
    U16BE = SND_PCM_FORMAT_U16_BE,
    S24LE = SND_PCM_FORMAT_S24_LE,
    S24BE = SND_PCM_FORMAT_S24_BE,
    U24LE = SND_PCM_FORMAT_U24_LE,
    U24BE = SND_PCM_FORMAT_U24_BE,
    S32LE = SND_PCM_FORMAT_S32_LE,
    S32BE = SND_PCM_FORMAT_S32_BE,
    U32LE = SND_PCM_FORMAT_U32_LE,
    U32BE = SND_PCM_FORMAT_U32_BE,
    FloatLE = SND_PCM_FORMAT_FLOAT_LE,
    FloatBE = SND_PCM_FORMAT_FLOAT_BE,
    Float64LE = SND_PCM_FORMAT_FLOAT64_LE,
    Float64BE = SND_PCM_FORMAT_FLOAT64_BE,
    IEC958SubframeLE = SND_PCM_FORMAT_IEC958_SUBFRAME_LE,
    IEC958SubframeBE = SND_PCM_FORMAT_IEC958_SUBFRAME_BE,
    MuLaw = SND_PCM_FORMAT_MU_LAW,
    ALaw = SND_PCM_FORMAT_A_LAW,
    ImaAdPCM = SND_PCM_FORMAT_IMA_ADPCM,
    MPEG = SND_PCM_FORMAT_MPEG,
    GSM = SND_PCM_FORMAT_GSM,
    S20LE = SND_PCM_FORMAT_S20_LE,
    S20BE = SND_PCM_FORMAT_S20_BE,
    U20LE = SND_PCM_FORMAT_U20_LE,
    U20BE = SND_PCM_FORMAT_U20_BE,
    Special = SND_PCM_FORMAT_SPECIAL,
    S243LE = SND_PCM_FORMAT_S24_3LE,
    S243BE = SND_PCM_FORMAT_S24_3BE,
    U243LE = SND_PCM_FORMAT_U24_3LE,
    U243BE = SND_PCM_FORMAT_U24_3BE,
    S203LE = SND_PCM_FORMAT_S20_3LE,
    S203BE = SND_PCM_FORMAT_S20_3BE,
    U203LE = SND_PCM_FORMAT_U20_3LE,
    U203BE = SND_PCM_FORMAT_U20_3BE,
    S183LE = SND_PCM_FORMAT_S18_3LE,
    S183BE = SND_PCM_FORMAT_S18_3BE,
    U183LE = SND_PCM_FORMAT_U18_3LE,
    U183BE = SND_PCM_FORMAT_U18_3BE,
    G72324 = SND_PCM_FORMAT_G723_24,
    G723241B = SND_PCM_FORMAT_G723_24_1B,
    G72340 = SND_PCM_FORMAT_G723_40,
    G723401B = SND_PCM_FORMAT_G723_40_1B,
    DSDU8 = SND_PCM_FORMAT_DSD_U8,
    DSDU16LE = SND_PCM_FORMAT_DSD_U16_LE,
    DSDU32LE = SND_PCM_FORMAT_DSD_U32_LE,
    DSDU16BE = SND_PCM_FORMAT_DSD_U16_BE,
    DSDU32BE = SND_PCM_FORMAT_DSD_U32_BE,
);

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Format::*;
        match *self {
            S8 => write!(f, "S8"),
            U8 => write!(f, "U8"),
            S16LE => write!(f, "S16_LE"),
            S16BE => write!(f, "S16_BE"),
            U16LE => write!(f, "U16_LE"),
            U16BE => write!(f, "U16_BE"),
            S24LE => write!(f, "S24_LE"),
            S24BE => write!(f, "S24_BE"),
            U24LE => write!(f, "U24_LE"),
            U24BE => write!(f, "U24_BE"),
            S32LE => write!(f, "S32_LE"),
            S32BE => write!(f, "S32_BE"),
            U32LE => write!(f, "U32_LE"),
            U32BE => write!(f, "U32_BE"),
            FloatLE => write!(f, "FLOAT_LE"),
            FloatBE => write!(f, "FLOAT_BE"),
            Float64LE => write!(f, "FLOAT64_LE"),
            Float64BE => write!(f, "FLOAT64_BE"),
            IEC958SubframeLE => write!(f, "IEC958_SUBFRAME_LE"),
            IEC958SubframeBE => write!(f, "IEC958_SUBFRAME_BE"),
            MuLaw => write!(f, "MU_LAW"),
            ALaw => write!(f, "A_LAW"),
            ImaAdPCM => write!(f, "IMA_ADPCM"),
            MPEG => write!(f, "MPEG"),
            GSM => write!(f, "GSM"),
            S20LE => write!(f, "S20_LE"),
            S20BE => write!(f, "S20_BE"),
            U20LE => write!(f, "U20_LE"),
            U20BE => write!(f, "U20_BE"),
            Special => write!(f, "SPECIAL"),
            S243LE => write!(f, "S24_3LE"),
            S243BE => write!(f, "S24_3BE"),
            U243LE => write!(f, "U24_3LE"),
            U243BE => write!(f, "U24_3BE"),
            S203LE => write!(f, "S20_3LE"),
            S203BE => write!(f, "S20_3BE"),
            U203LE => write!(f, "U20_3LE"),
            U203BE => write!(f, "U20_3BE"),
            S183LE => write!(f, "S18_3LE"),
            S183BE => write!(f, "S18_3BE"),
            U183LE => write!(f, "U18_3LE"),
            U183BE => write!(f, "U18_3BE"),
            G72324 => write!(f, "G723_24"),
            G723241B => write!(f, "G723_24_1B"),
            G72340 => write!(f, "G723_40"),
            G723401B => write!(f, "G723_40_1B"),
            DSDU8 => write!(f, "DSD_U8"),
            DSDU16LE => write!(f, "DSD_U16_LE"),
            DSDU32LE => write!(f, "DSD_U32_LE"),
            DSDU16BE => write!(f, "DSD_U16_BE"),
            DSDU32BE => write!(f, "DSD_U32_BE"),
            _ => write!(f, "UNKNOWN"),
        }
    }
}

impl FromStr for Format {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use Format::*;
        Ok(match s.to_ascii_uppercase().as_str() {
            "S8" => S8,
            "U8" => U8,
            "S16_LE" => S16LE,
            "S16_BE" => S16BE,
            "U16_LE" => U16LE,
            "U16_BE" => U16BE,
            "S24_LE" => S24LE,
            "S24_BE" => S24BE,
            "U24_LE" => U24LE,
            "U24_BE" => U24BE,
            "S32_LE" => S32LE,
            "S32_BE" => S32BE,
            "U32_LE" => U32LE,
            "U32_BE" => U32BE,
            "FLOAT_LE" => FloatLE,
            "FLOAT_BE" => FloatBE,
            "FLOAT64_LE" => Float64LE,
            "FLOAT64_BE" => Float64BE,
            "IEC958_SUBFRAME_LE" => IEC958SubframeLE,
            "IEC958_SUBFRAME_BE" => IEC958SubframeBE,
            "MU_LAW" => MuLaw,
            "A_LAW" => ALaw,
            "IMA_ADPCM" => ImaAdPCM,
            "MPEG" => MPEG,
            "GSM" => GSM,
            "S20_LE" => S20LE,
            "S20_BE" => S20BE,
            "U20_LE" => U20LE,
            "U20_BE" => U20BE,
            "SPECIAL" => Special,
            "S24_3LE" => S243LE,
            "S24_3BE" => S243BE,
            "U24_3LE" => U243LE,
            "U24_3BE" => U243BE,
            "S20_3LE" => S203LE,
            "S20_3BE" => S203BE,
            "U20_3LE" => U203LE,
            "U20_3BE" => U203BE,
            "S18_3LE" => S183LE,
            "S18_3BE" => S183BE,
            "U18_3LE" => U183LE,
            "U18_3BE" => U183BE,
            "G723_24" => G72324,
            "G723_24_1B" => G723241B,
            "G723_40" => G72340,
            "G723_40_1B" => G723401B,
            "DSD_U8" => DSDU8,
            "DSD_U16_LE" => DSDU16LE,
            "DSD_U32_LE" => DSDU32LE,
            "DSD_U16_BE" => DSDU16BE,
            "DSD_U32_BE" => DSDU32BE,
            _ => Unknown,
        })
    }
}

impl Format {
    pub const fn s16() -> Format { <i16 as IoFormat>::FORMAT }
    pub const fn u16() -> Format { <u16 as IoFormat>::FORMAT }
    pub const fn s32() -> Format { <i32 as IoFormat>::FORMAT }
    pub const fn u32() -> Format { <u32 as IoFormat>::FORMAT }
    pub const fn float() -> Format { <f32 as IoFormat>::FORMAT }
    pub const fn float64() -> Format { <f64 as IoFormat>::FORMAT }

    #[cfg(target_endian = "little")] pub const fn s24() -> Format { Format::S24LE }
    #[cfg(target_endian = "big")] pub const fn s24() -> Format { Format::S24BE }

    #[cfg(target_endian = "little")] pub const fn s24_3() -> Format { Format::S243LE }
    #[cfg(target_endian = "big")] pub const fn s24_3() -> Format { Format::S243BE }

    #[cfg(target_endian = "little")] pub const fn u24() -> Format { Format::U24LE }
    #[cfg(target_endian = "big")] pub const fn u24() -> Format { Format::U24BE }

    #[cfg(target_endian = "little")] pub const fn u24_3() -> Format { Format::U243LE }
    #[cfg(target_endian = "big")] pub const fn u24_3() -> Format { Format::U243BE }

    #[cfg(target_endian = "little")] pub const fn s20() -> Format { Format::S20LE }
    #[cfg(target_endian = "big")] pub const fn s20() -> Format { Format::S20BE }

    #[cfg(target_endian = "little")] pub const fn s20_3() -> Format { Format::S203LE }
    #[cfg(target_endian = "big")] pub const fn s20_3() -> Format { Format::S203BE }

    #[cfg(target_endian = "little")] pub const fn u20() -> Format { Format::U20LE }
    #[cfg(target_endian = "big")] pub const fn u20() -> Format { Format::U20BE }

    #[cfg(target_endian = "little")] pub const fn u20_3() -> Format { Format::U203LE }
    #[cfg(target_endian = "big")] pub const fn u20_3() -> Format { Format::U203BE }

    #[cfg(target_endian = "little")] pub const fn s18_3() -> Format { Format::S183LE }
    #[cfg(target_endian = "big")] pub const fn s18_3() -> Format { Format::S183BE }

    #[cfg(target_endian = "little")] pub const fn u18_3() -> Format { Format::U183LE }
    #[cfg(target_endian = "big")] pub const fn u18_3() -> Format { Format::U183BE }

    #[cfg(target_endian = "little")] pub const fn dsd_u16() -> Format { Format::DSDU16LE }
    #[cfg(target_endian = "big")] pub const fn dsd_u16() -> Format { Format::DSDU16BE }

    #[cfg(target_endian = "little")] pub const fn dsd_u32() -> Format { Format::DSDU32LE }
    #[cfg(target_endian = "big")] pub const fn dsd_u32() -> Format { Format::DSDU32BE }

    #[cfg(target_endian = "little")] pub const fn iec958_subframe() -> Format { Format::IEC958SubframeLE }
    #[cfg(target_endian = "big")] pub const fn iec958_subframe() -> Format { Format::IEC958SubframeBE }

    pub fn physical_width(&self) -> Result<i32> {
        acheck!(snd_pcm_format_physical_width(self.to_c_int()))
    }

    pub fn width(&self) -> Result<i32> {
        acheck!(snd_pcm_format_width(self.to_c_int()))
    }

    pub fn silence_16(&self) -> u16 {
        unsafe { alsa::snd_pcm_format_silence_16(self.to_c_int()) }
    }

    pub fn little_endian(&self) -> Result<bool> {
        acheck!(snd_pcm_format_little_endian(self.to_c_int())).map(|v| v != 0)
    }
}


pub trait IoFormat: Copy {
    const FORMAT: Format;
}

impl IoFormat for i8 { const FORMAT: Format = Format::S8; }
impl IoFormat for u8 { const FORMAT: Format = Format::U8; }

impl IoFormat for i16 {
    #[cfg(target_endian = "little")]
    const FORMAT: Format = Format::S16LE;
    #[cfg(target_endian = "big")]
    const FORMAT: Format = Format::S16BE;
}
impl IoFormat for u16 {
    #[cfg(target_endian = "little")]
    const FORMAT: Format = Format::U16LE;
    #[cfg(target_endian = "big")]
    const FORMAT: Format = Format::U16BE;
}
impl IoFormat for i32 {
    #[cfg(target_endian = "little")]
    const FORMAT: Format = Format::S32LE;
    #[cfg(target_endian = "big")]
    const FORMAT: Format = Format::S32BE;
}
impl IoFormat for u32 {
    #[cfg(target_endian = "little")]
    const FORMAT: Format = Format::U32LE;
    #[cfg(target_endian = "big")]
    const FORMAT: Format = Format::U32BE;
}
impl IoFormat for f32 {
    #[cfg(target_endian = "little")]
    const FORMAT: Format = Format::FloatLE;
    #[cfg(target_endian = "big")]
    const FORMAT: Format = Format::FloatBE;
}
impl IoFormat for f64 {
    #[cfg(target_endian = "little")]
    const FORMAT: Format = Format::Float64LE;
    #[cfg(target_endian = "big")]
    const FORMAT: Format = Format::Float64BE;
}


alsa_enum!(
    /// [SND_PCM_ACCESS_xxx](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m.html) constants
    Access, ALL_ACCESSES[5],

    MMapInterleaved = SND_PCM_ACCESS_MMAP_INTERLEAVED,
    MMapNonInterleaved = SND_PCM_ACCESS_MMAP_NONINTERLEAVED,
    MMapComplex = SND_PCM_ACCESS_MMAP_COMPLEX,
    RWInterleaved = SND_PCM_ACCESS_RW_INTERLEAVED,
    RWNonInterleaved = SND_PCM_ACCESS_RW_NONINTERLEAVED,
);

alsa_enum!(
    /// [SND_PCM_TSTAMP_TYPE_xxx](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m.html) constants
    TstampType, ALL_TSTAMP_TYPES[3],

    Gettimeofday = SND_PCM_TSTAMP_TYPE_GETTIMEOFDAY,
    Monotonic = SND_PCM_TSTAMP_TYPE_MONOTONIC,
    MonotonicRaw = SND_PCM_TSTAMP_TYPE_MONOTONIC_RAW,
);

/// [snd_pcm_hw_params_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m___h_w___params.html) wrapper
pub struct HwParams<'a>(*mut alsa::snd_pcm_hw_params_t, &'a PCM);

impl<'a> Drop for HwParams<'a> {
    fn drop(&mut self) { unsafe { alsa::snd_pcm_hw_params_free(self.0) }; }
}

impl<'a> HwParams<'a> {
    fn new(a: &'a PCM) -> Result<HwParams<'a>> {
        let mut p = ptr::null_mut();
        acheck!(snd_pcm_hw_params_malloc(&mut p)).map(|_| HwParams(p, a))
    }

    pub fn any(a: &'a PCM) -> Result<HwParams<'a>> { HwParams::new(a).and_then(|p|
        acheck!(snd_pcm_hw_params_any(a.0, p.0)).map(|_| p)
    )}

    pub fn get_rate_resample(&self) -> Result<bool> {
        let mut v = 0;
        acheck!(snd_pcm_hw_params_get_rate_resample((self.1).0, self.0, &mut v)).map(|_| v != 0)
    }

    pub fn set_rate_resample(&self, resample: bool) -> Result<()> {
        acheck!(snd_pcm_hw_params_set_rate_resample((self.1).0, self.0, if resample {1} else {0})).map(|_| ())
    }

    pub fn set_channels_near(&self, v: u32) -> Result<u32> {
        let mut r = v as c_uint;
        acheck!(snd_pcm_hw_params_set_channels_near((self.1).0, self.0, &mut r)).map(|_| r)
    }

    pub fn set_channels(&self, v: u32) -> Result<()> {
        acheck!(snd_pcm_hw_params_set_channels((self.1).0, self.0, v as c_uint)).map(|_| ())
    }

    pub fn get_channels(&self) -> Result<u32> {
        let mut v = 0;
        acheck!(snd_pcm_hw_params_get_channels(self.0, &mut v)).map(|_| v as u32)
    }

    pub fn get_channels_max(&self) -> Result<u32> {
        let mut v = 0;
        acheck!(snd_pcm_hw_params_get_channels_max(self.0, &mut v)).map(|_| v as u32)
    }

    pub fn get_channels_min(&self) -> Result<u32> {
        let mut v = 0;
        acheck!(snd_pcm_hw_params_get_channels_min(self.0, &mut v)).map(|_| v as u32)
    }

    pub fn test_channels(&self, v: u32) -> Result<()> {
        acheck!(snd_pcm_hw_params_test_channels((self.1).0, self.0, v as c_uint)).map(|_| ())
    }

    pub fn set_rate_near(&self, v: u32, dir: ValueOr) -> Result<u32> {
        let mut d = dir as c_int;
        let mut r = v as c_uint;
        acheck!(snd_pcm_hw_params_set_rate_near((self.1).0, self.0, &mut r, &mut d)).map(|_| r)
    }

    pub fn set_rate(&self, v: u32, dir: ValueOr) -> Result<()> {
        acheck!(snd_pcm_hw_params_set_rate((self.1).0, self.0, v as c_uint, dir as c_int)).map(|_| ())
    }

    pub fn get_rate(&self) -> Result<u32> {
        let (mut v, mut d) = (0,0);
        acheck!(snd_pcm_hw_params_get_rate(self.0, &mut v, &mut d)).map(|_| v as u32)
    }

    pub fn get_rate_max(&self) -> Result<u32> {
        let mut v = 0;
        // Note on the null ptr: if this ptr is not null, then the value behind it is replaced with
        // -1 if the suprenum is not in the set (i.e. it's an open range), 0 otherwise. This could
        // be returned along with the value, but it's safe to pass a null ptr in, in which case the
        // pointer is not dereferenced.
        acheck!(snd_pcm_hw_params_get_rate_max(self.0, &mut v, ptr::null_mut())).map(|_| v as u32)
    }

    pub fn get_rate_min(&self) -> Result<u32> {
        let mut v = 0;
        // Note on the null ptr: see get_rate_max but read +1 and infinum instead of -1 and
        // suprenum.
        acheck!(snd_pcm_hw_params_get_rate_min(self.0, &mut v, ptr::null_mut())).map(|_| v as u32)
    }

    pub fn test_rate(&self, rate: u32) -> Result<()> {
        acheck!(snd_pcm_hw_params_test_rate((self.1).0, self.0, rate as c_uint, 0)).map(|_| ())
    }

    pub fn set_format(&self, v: Format) -> Result<()> {
        acheck!(snd_pcm_hw_params_set_format((self.1).0, self.0, v as c_int)).map(|_| ())
    }

    pub fn get_format(&self) -> Result<Format> {
        let mut v = 0;
        acheck!(snd_pcm_hw_params_get_format(self.0, &mut v))
            .and_then(|_| Format::from_c_int(v, "snd_pcm_hw_params_get_format"))
    }

    pub fn test_format(&self, v: Format) -> Result<()> {
        acheck!(snd_pcm_hw_params_test_format((self.1).0, self.0, v as c_int)).map(|_| ())
    }

    pub fn test_access(&self, v: Access) -> Result<()> {
        acheck!(snd_pcm_hw_params_test_access((self.1).0, self.0, v as c_uint)).map(|_| ())
    }

    pub fn set_access(&self, v: Access) -> Result<()> {
        acheck!(snd_pcm_hw_params_set_access((self.1).0, self.0, v as c_uint)).map(|_| ())
    }

    pub fn get_access(&self) -> Result<Access> {
        let mut v = 0;
        acheck!(snd_pcm_hw_params_get_access(self.0, &mut v))
            .and_then(|_| Access::from_c_int(v as c_int, "snd_pcm_hw_params_get_access"))
    }

    pub fn set_period_size_near(&self, v: Frames, dir: ValueOr) -> Result<Frames> {
        let mut d = dir as c_int;
        let mut r = v as alsa::snd_pcm_uframes_t;
        acheck!(snd_pcm_hw_params_set_period_size_near((self.1).0, self.0, &mut r, &mut d)).map(|_| r as Frames)
    }

    pub fn set_period_size(&self, v: Frames, dir: ValueOr) -> Result<()> {
        acheck!(snd_pcm_hw_params_set_period_size((self.1).0, self.0, v as alsa::snd_pcm_uframes_t, dir as c_int)).map(|_| ())
    }

    pub fn set_period_time_near(&self, v: u32, dir: ValueOr) -> Result<u32> {
        let mut d = dir as c_int;
        let mut r = v as c_uint;
        acheck!(snd_pcm_hw_params_set_period_time_near((self.1).0, self.0, &mut r, &mut d)).map(|_| r as u32)
    }

    pub fn get_period_size(&self) -> Result<Frames> {
        let (mut v, mut d) = (0,0);
        acheck!(snd_pcm_hw_params_get_period_size(self.0, &mut v, &mut d)).map(|_| v as Frames)
    }

    pub fn get_period_size_min(&self) -> Result<Frames> {
        let (mut v, mut d) = (0,0);
        acheck!(snd_pcm_hw_params_get_period_size_min(self.0, &mut v, &mut d)).map(|_| v as Frames)
    }

    pub fn get_period_size_max(&self) -> Result<Frames> {
        let (mut v, mut d) = (0,0);
        acheck!(snd_pcm_hw_params_get_period_size_max(self.0, &mut v, &mut d)).map(|_| v as Frames)
    }

    pub fn set_periods(&self, v: u32, dir: ValueOr) -> Result<()> {
        acheck!(snd_pcm_hw_params_set_periods((self.1).0, self.0, v as c_uint, dir as c_int)).map(|_| ())
    }

    pub fn get_periods(&self) -> Result<u32> {
        let (mut v, mut d) = (0,0);
        acheck!(snd_pcm_hw_params_get_periods(self.0, &mut v, &mut d)).map(|_| v as u32)
    }

    pub fn set_buffer_size_near(&self, v: Frames) -> Result<Frames> {
        let mut r = v as alsa::snd_pcm_uframes_t;
        acheck!(snd_pcm_hw_params_set_buffer_size_near((self.1).0, self.0, &mut r)).map(|_| r as Frames)
    }

    pub fn set_buffer_size_max(&self, v: Frames) -> Result<Frames> {
        let mut r = v as alsa::snd_pcm_uframes_t;
        acheck!(snd_pcm_hw_params_set_buffer_size_max((self.1).0, self.0, &mut r)).map(|_| r as Frames)
    }

    pub fn set_buffer_size_min(&self, v: Frames) -> Result<Frames> {
        let mut r = v as alsa::snd_pcm_uframes_t;
        acheck!(snd_pcm_hw_params_set_buffer_size_min((self.1).0, self.0, &mut r)).map(|_| r as Frames)
    }

    pub fn set_buffer_size(&self, v: Frames) -> Result<()> {
        acheck!(snd_pcm_hw_params_set_buffer_size((self.1).0, self.0, v as alsa::snd_pcm_uframes_t)).map(|_| ())
    }

    pub fn set_buffer_time_near(&self, v: u32, dir: ValueOr) -> Result<u32> {
        let mut d = dir as c_int;
        let mut r = v as c_uint;
        acheck!(snd_pcm_hw_params_set_buffer_time_near((self.1).0, self.0, &mut r, &mut d)).map(|_| r as u32)
    }

    pub fn get_buffer_size(&self) -> Result<Frames> {
        let mut v = 0;
        acheck!(snd_pcm_hw_params_get_buffer_size(self.0, &mut v)).map(|_| v as Frames)
    }

    pub fn get_buffer_size_min(&self) -> Result<Frames> {
        let mut v = 0;
        acheck!(snd_pcm_hw_params_get_buffer_size_min(self.0, &mut v)).map(|_| v as Frames)
    }

    pub fn get_buffer_size_max(&self) -> Result<Frames> {
        let mut v = 0;
        acheck!(snd_pcm_hw_params_get_buffer_size_max(self.0, &mut v)).map(|_| v as Frames)
    }

    pub fn get_buffer_time_min(&self) -> Result<u32> {
        let (mut v, mut d) = (0,0);
        acheck!(snd_pcm_hw_params_get_buffer_time_min(self.0, &mut v, &mut d)).map(|_| v as u32)
    }

    pub fn get_buffer_time_max(&self) -> Result<u32> {
        let (mut v, mut d) = (0,0);
        acheck!(snd_pcm_hw_params_get_buffer_time_max(self.0, &mut v, &mut d)).map(|_| v as u32)
    }

    /// Returns true if the alsa stream can be paused, false if not.
    ///
    /// This function should only be called when the configuration space contains a single
    /// configuration. Call `PCM::hw_params` to choose a single configuration from the
    /// configuration space.
    pub fn can_pause(&self) -> bool {
        unsafe { alsa::snd_pcm_hw_params_can_pause(self.0) != 0 }
    }

    /// Returns true if the alsa stream can be resumed, false if not.
    ///
    /// This function should only be called when the configuration space contains a single
    /// configuration. Call `PCM::hw_params` to choose a single configuration from the
    /// configuration space.
    pub fn can_resume(&self) -> bool {
        unsafe { alsa::snd_pcm_hw_params_can_resume(self.0) != 0 }
    }

    /// Returns true if the alsa stream supports the provided `AudioTstampType`, false if not.
    ///
    /// This function should only be called when the configuration space contains a single
    /// configuration. Call `PCM::hw_params` to choose a single configuration from the
    /// configuration space.
    pub fn supports_audio_ts_type(&self, type_: AudioTstampType) -> bool {
        unsafe { alsa::snd_pcm_hw_params_supports_audio_ts_type(self.0, type_ as libc::c_int) != 0 }
    }

    pub fn dump(&self, o: &mut Output) -> Result<()> {
        acheck!(snd_pcm_hw_params_dump(self.0, super::io::output_handle(o))).map(|_| ())
    }

    pub fn copy_from(&mut self, other: &HwParams<'a>) {
        self.1 = other.1;
        unsafe { alsa::snd_pcm_hw_params_copy(self.0, other.0) };
    }
}

impl<'a> Clone for HwParams<'a> {
    fn clone(&self) -> HwParams<'a> {
        let mut r = HwParams::new(self.1).unwrap();
        r.copy_from(self);
        r
    }
}

impl<'a> fmt::Debug for HwParams<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("HwParams")
            .field("channels", &self.get_channels())
            .field("rate", &format!("{:?} Hz", self.get_rate()))
            .field("format", &self.get_format())
            .field("access", &self.get_access())
            .field("period_size", &format!("{:?} frames", self.get_period_size()))
            .field("buffer_size", &format!("{:?} frames", self.get_buffer_size()))
            .finish()
    }
}

/// [snd_pcm_sw_params_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m___s_w___params.html) wrapper
pub struct SwParams<'a>(*mut alsa::snd_pcm_sw_params_t, &'a PCM);

impl<'a> Drop for SwParams<'a> {
    fn drop(&mut self) { unsafe { alsa::snd_pcm_sw_params_free(self.0) }; }
}

impl<'a> SwParams<'a> {

    fn new(a: &'a PCM) -> Result<SwParams<'a>> {
        let mut p = ptr::null_mut();
        acheck!(snd_pcm_sw_params_malloc(&mut p)).map(|_| SwParams(p, a))
    }

    pub fn set_avail_min(&self, v: Frames) -> Result<()> {
        acheck!(snd_pcm_sw_params_set_avail_min((self.1).0, self.0, v as alsa::snd_pcm_uframes_t)).map(|_| ())
    }

    pub fn get_avail_min(&self) -> Result<Frames> {
        let mut v = 0;
        acheck!(snd_pcm_sw_params_get_avail_min(self.0, &mut v)).map(|_| v as Frames)
    }

    pub fn get_boundary(&self) -> Result<Frames> {
        let mut v = 0;
        acheck!(snd_pcm_sw_params_get_boundary(self.0, &mut v)).map(|_| v as Frames)
    }

    pub fn set_start_threshold(&self, v: Frames) -> Result<()> {
        acheck!(snd_pcm_sw_params_set_start_threshold((self.1).0, self.0, v as alsa::snd_pcm_uframes_t)).map(|_| ())
    }

    pub fn get_start_threshold(&self) -> Result<Frames> {
        let mut v = 0;
        acheck!(snd_pcm_sw_params_get_start_threshold(self.0, &mut v)).map(|_| v as Frames)
    }

    pub fn set_stop_threshold(&self, v: Frames) -> Result<()> {
        acheck!(snd_pcm_sw_params_set_stop_threshold((self.1).0, self.0, v as alsa::snd_pcm_uframes_t)).map(|_| ())
    }

    pub fn get_stop_threshold(&self) -> Result<Frames> {
        let mut v = 0;
        acheck!(snd_pcm_sw_params_get_stop_threshold(self.0, &mut v)).map(|_| v as Frames)
    }

    pub fn set_tstamp_mode(&self, v: bool) -> Result<()> {
        let z = if v { alsa::SND_PCM_TSTAMP_ENABLE } else { alsa::SND_PCM_TSTAMP_NONE };
        acheck!(snd_pcm_sw_params_set_tstamp_mode((self.1).0, self.0, z)).map(|_| ())
    }

    pub fn get_tstamp_mode(&self) -> Result<bool> {
        let mut v = 0;
        acheck!(snd_pcm_sw_params_get_tstamp_mode(self.0, &mut v)).map(|_| v != 0)
    }

    pub fn set_tstamp_type(&self, v: TstampType) -> Result<()> {
        acheck!(snd_pcm_sw_params_set_tstamp_type((self.1).0, self.0, v as u32)).map(|_| ())
    }

    pub fn get_tstamp_type(&self) -> Result<TstampType> {
        let mut v = 0;
        acheck!(snd_pcm_sw_params_get_tstamp_type(self.0, &mut v))?;
        TstampType::from_c_int(v as c_int, "snd_pcm_sw_params_get_tstamp_type")
    }

    pub fn dump(&self, o: &mut Output) -> Result<()> {
        acheck!(snd_pcm_sw_params_dump(self.0, super::io::output_handle(o))).map(|_| ())
    }
}

impl<'a> fmt::Debug for SwParams<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
           "SwParams(avail_min: {:?} frames, start_threshold: {:?} frames, stop_threshold: {:?} frames)",
           self.get_avail_min(), self.get_start_threshold(), self.get_stop_threshold())
    }
}

const STATUS_SIZE: usize = 152;

/// [snd_pcm_status_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m___status.html) wrapper
#[derive(Debug)]
pub struct Status([u64; (STATUS_SIZE+7)/8]);

impl Status {
    fn new() -> Status {
        assert!(unsafe { alsa::snd_pcm_status_sizeof() } as usize <= STATUS_SIZE);
        Status([0; (STATUS_SIZE+7)/8])
    }

    fn ptr(&self) -> *mut alsa::snd_pcm_status_t { self.0.as_ptr() as *const _ as *mut alsa::snd_pcm_status_t }

    pub fn get_htstamp(&self) -> timespec {
        let mut h = timespec {tv_sec: 0, tv_nsec: 0};
        unsafe { alsa::snd_pcm_status_get_htstamp(self.ptr(), &mut h) };
        h
    }

    pub fn get_trigger_htstamp(&self) -> timespec {
        let mut h = timespec {tv_sec: 0, tv_nsec: 0};
        unsafe { alsa::snd_pcm_status_get_trigger_htstamp(self.ptr(), &mut h) };
        h
    }

    pub fn get_audio_htstamp(&self) -> timespec {
        let mut h = timespec {tv_sec: 0, tv_nsec: 0};
        unsafe { alsa::snd_pcm_status_get_audio_htstamp(self.ptr(), &mut h) };
        h
    }

    pub fn get_state(&self) -> State { State::from_c_int(
        unsafe { alsa::snd_pcm_status_get_state(self.ptr()) } as c_int, "snd_pcm_status_get_state").unwrap() }

    pub fn get_avail(&self) -> Frames { unsafe { alsa::snd_pcm_status_get_avail(self.ptr()) as Frames }}
    pub fn get_delay(&self) -> Frames { unsafe { alsa::snd_pcm_status_get_delay(self.ptr()) }}
    pub fn get_avail_max(&self) -> Frames { unsafe { alsa::snd_pcm_status_get_avail_max(self.ptr()) as Frames }}
    pub fn get_overrange(&self) -> Frames { unsafe { alsa::snd_pcm_status_get_overrange(self.ptr()) as Frames }}

    pub fn dump(&self, o: &mut Output) -> Result<()> {
        acheck!(snd_pcm_status_dump(self.ptr(), super::io::output_handle(o))).map(|_| ())
    }
}

/// Builder for [`Status`].
///
/// Allows setting the audio timestamp configuration before retrieving the
/// status from the stream.
pub struct StatusBuilder(Status);

impl StatusBuilder {
    pub fn new() -> Self {
        StatusBuilder(Status::new())
    }

    pub fn audio_htstamp_config(
        self,
        type_requested: AudioTstampType,
        report_delay: bool,
    ) -> Self {
        let mut cfg: alsa::snd_pcm_audio_tstamp_config_t = unsafe { std::mem::zeroed() };
        cfg.set_type_requested(type_requested as _);
        cfg.set_report_delay(report_delay as _);
        unsafe { alsa::snd_pcm_status_set_audio_htstamp_config(self.0.ptr(), &mut cfg) };
        self
    }

    pub fn build(mut self, pcm: &PCM) -> Result<Status> {
        let p = self.0.0.as_mut_ptr() as *mut alsa::snd_pcm_status_t;
        acheck!(snd_pcm_status(pcm.0, p)).map(|_| self.0)
    }
}

alsa_enum!(
    #[non_exhaustive]
    /// [SND_PCM_AUDIO_TSTAMP_TYPE_xxx](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m.html) constants
    AudioTstampType, ALL_AUDIO_TSTAMP_TYPES[6],

    Compat = SND_PCM_AUDIO_TSTAMP_TYPE_COMPAT,
    Default = SND_PCM_AUDIO_TSTAMP_TYPE_DEFAULT,
    Link = SND_PCM_AUDIO_TSTAMP_TYPE_LINK,
    LinkAbsolute = SND_PCM_AUDIO_TSTAMP_TYPE_LINK_ABSOLUTE,
    LinkEstimated = SND_PCM_AUDIO_TSTAMP_TYPE_LINK_ESTIMATED,
    LinkSynchronized = SND_PCM_AUDIO_TSTAMP_TYPE_LINK_SYNCHRONIZED,
);

#[test]
fn info_from_default() {
    use std::ffi::CString;
    let pcm = PCM::open(&*CString::new("default").unwrap(), Direction::Capture, false).unwrap();
    let info = pcm.info().unwrap();
    println!("PCM Info:");
    println!("\tCard: {}", info.get_card());
    println!("\tDevice: {}", info.get_device());
    println!("\tSubdevice: {}", info.get_subdevice());
    println!("\tId: {}", info.get_id().unwrap());
    println!("\tName: {}", info.get_name().unwrap());
    println!("\tSubdevice Name: {}", info.get_subdevice_name().unwrap());
}

#[test]
fn drop() {
    use std::ffi::CString;
    let pcm = PCM::open(&*CString::new("default").unwrap(), Direction::Capture, false).unwrap();
    // Verify that this does not cause a naming conflict (issue #14)
    let _ = pcm.drop();
}

#[test]
fn record_from_default() {
    use std::ffi::CString;
    let pcm = PCM::open(&*CString::new("default").unwrap(), Direction::Capture, false).unwrap();
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(2).unwrap();
    hwp.set_rate(44100, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::s16()).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
    pcm.start().unwrap();
    let mut buf = [0i16; 1024];
    assert_eq!(pcm.io_i16().unwrap().readi(&mut buf).unwrap(), 1024/2);
}

#[test]
fn open_s24() {
    let pcm = PCM::open(c"default", Direction::Playback, false).unwrap();
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(1).unwrap();
    hwp.set_rate(44100, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::s24()).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
    assert_eq!(Format::s24().physical_width(), Ok(32));
    let _io = pcm.io_i32_s24().unwrap();
}

#[test]
fn playback_to_default() {
    use std::ffi::CString;
    let pcm = PCM::open(&*CString::new("default").unwrap(), Direction::Playback, false).unwrap();
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(1).unwrap();
    hwp.set_rate(44100, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::s16()).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();

    let hwp = pcm.hw_params_current().unwrap();
    let swp = pcm.sw_params_current().unwrap();
    swp.set_start_threshold(hwp.get_buffer_size().unwrap()).unwrap();
    pcm.sw_params(&swp).unwrap();

    println!("PCM status: {:?}, {:?}", pcm.state(), pcm.hw_params_current().unwrap());
    let mut outp = Output::buffer_open().unwrap();
    pcm.dump(&mut outp).unwrap();
    println!("== PCM dump ==\n{}", outp);

    let mut buf = [0i16; 1024];
    for (i, a) in buf.iter_mut().enumerate() {
        *a = ((i as f32 * 2.0 * ::std::f32::consts::PI / 128.0).sin() * 8192.0) as i16
    }
    let io = pcm.io_i16().unwrap();
    for _ in 0..2*44100/1024 { // 2 seconds of playback
        println!("PCM state: {:?}", pcm.state());
        assert_eq!(io.writei(&buf[..]).unwrap(), 1024);
    }
    if pcm.state() != State::Running { pcm.start().unwrap() };

    let mut outp2 = Output::buffer_open().unwrap();
    pcm.status().unwrap().dump(&mut outp2).unwrap();
    println!("== PCM status dump ==\n{}", outp2);

    pcm.drain().unwrap();
}

#[test]
fn print_sizeof() {
    let s = unsafe { alsa::snd_pcm_status_sizeof() } as usize;
    println!("Status size: {}", s);

    assert!(s <= STATUS_SIZE);
}

#[test]
fn format_display_from_str() {
    for format in ALL_FORMATS {
        assert_eq!(format, format.to_string().parse().unwrap());
    }
}
