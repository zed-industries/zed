/*!
This module bypasses alsa-lib and directly read and write into memory mapped kernel memory.
In case of the sample memory, this is in many cases the DMA buffers that is transferred to the sound card.

The reasons for doing this are:

 * Minimum overhead where it matters most: let alsa-lib do the code heavy setup -
   then steal its file descriptor and deal with sample streaming from Rust.
 * RT-safety to the maximum extent possible. Creating/dropping any of these structs causes syscalls,
   but function calls on these are just read and write from memory. No syscalls, no memory allocations,
   not even loops (with the exception of `MmapPlayback::write` that loops over samples to write).
 * Possibility to allow Send + Sync for structs
 * It's a fun experiment and an interesting deep dive into how alsa-lib does things.

Note: Not all sound card drivers support this direct method of communication; although almost all
modern/common ones do. It only works with hardware devices though (such as "hw:xxx" device strings),
don't expect it to work with, e g, the PulseAudio plugin or so.

For an example of how to use this mode, look in the "synth-example" directory.
*/

use libc;
use std::{mem, ptr, fmt, cmp};
use crate::error::{Error, Result};
use std::os::unix::io::RawFd;
use crate::{pcm, PollDescriptors, Direction};
use crate::pcm::Frames;
use std::marker::PhantomData;

use super::ffi::*;

/// Read PCM status via a simple kernel syscall, bypassing alsa-lib.
///
/// If Status is not available on your architecture, this is the second best option.
pub struct SyncPtrStatus(snd_pcm_mmap_status);

impl SyncPtrStatus {
    /// Executes sync_ptr syscall.
    ///
    /// Unsafe because
    ///  - setting appl_ptr and avail_min might make alsa-lib confused
    ///  - no check that the fd is really a PCM
    pub unsafe fn sync_ptr(fd: RawFd, hwsync: bool, appl_ptr: Option<pcm::Frames>, avail_min: Option<pcm::Frames>) -> Result<Self> {
        let mut data = snd_pcm_sync_ptr {
			flags: (if hwsync { SNDRV_PCM_SYNC_PTR_HWSYNC } else { 0 }) +
				(if appl_ptr.is_some() { SNDRV_PCM_SYNC_PTR_APPL } else { 0 }) +
				(if avail_min.is_some() { SNDRV_PCM_SYNC_PTR_AVAIL_MIN } else { 0 }),
			c: snd_pcm_mmap_control_r {
				control: snd_pcm_mmap_control {
					appl_ptr: appl_ptr.unwrap_or(0) as snd_pcm_uframes_t,
					avail_min: avail_min.unwrap_or(0) as snd_pcm_uframes_t,
				}
			},
			s: mem::zeroed()
		};

        sndrv_pcm_ioctl_sync_ptr(fd, &mut data)?;

        let i = data.s.status.state;
        if (i >= (pcm::State::Open as snd_pcm_state_t)) && (i <= (pcm::State::Disconnected as snd_pcm_state_t)) {
            Ok(SyncPtrStatus(data.s.status))
        } else {
            Err(Error::unsupported("SNDRV_PCM_IOCTL_SYNC_PTR returned broken state"))
        }
    }

    pub fn hw_ptr(&self) -> pcm::Frames { self.0.hw_ptr as pcm::Frames }
    pub fn state(&self) -> pcm::State { unsafe { mem::transmute(self.0.state as u8) } /* valid range checked in sync_ptr */ }
    pub fn htstamp(&self) -> libc::timespec { self.0.tstamp }
}



/// Read PCM status directly from memory, bypassing alsa-lib.
///
/// This means that it's
/// 1) less overhead for reading status (no syscall, no allocations, no virtual dispatch, just a read from memory)
/// 2) Send + Sync, and
/// 3) will only work for "hw" / "plughw" devices (not e g PulseAudio plugins), and not
/// all of those are supported, although all common ones are (as of 2017, and a kernel from the same decade).
/// Kernel supported archs are: x86, PowerPC, Alpha. Use "SyncPtrStatus" for other archs.
///
/// The values are updated every now and then by the kernel. Many functions will force an update to happen,
/// e g `PCM::avail()` and `PCM::delay()`.
///
/// Note: Even if you close the original PCM device, ALSA will not actually close the device until all
/// Status structs are dropped too.
///
#[derive(Debug)]
pub struct Status(DriverMemory<snd_pcm_mmap_status>);

fn pcm_to_fd(p: &pcm::PCM) -> Result<RawFd> {
    let mut fds: [libc::pollfd; 1] = unsafe { mem::zeroed() };
    let c = PollDescriptors::fill(p, &mut fds)?;
    if c != 1 {
        return Err(Error::unsupported("snd_pcm_poll_descriptors returned wrong number of fds"))
    }
    Ok(fds[0].fd)
}

impl Status {
    pub fn new(p: &pcm::PCM) -> Result<Self> { Status::from_fd(pcm_to_fd(p)?) }

    pub fn from_fd(fd: RawFd) -> Result<Self> {
        DriverMemory::new(fd, 1, SNDRV_PCM_MMAP_OFFSET_STATUS as libc::off_t, false).map(Status)
    }

    /// Current PCM state.
    pub fn state(&self) -> pcm::State {
        unsafe {
            let i = ptr::read_volatile(&(*self.0.ptr).state);
            assert!((i >= (pcm::State::Open as snd_pcm_state_t)) && (i <= (pcm::State::Disconnected as snd_pcm_state_t)));
            mem::transmute(i as u8)
        }
    }

    /// Number of frames hardware has read or written
    ///
    /// This number is updated every now and then by the kernel.
    /// Calling most functions on the PCM will update it, so will usually a period interrupt.
    /// No guarantees given.
    ///
    /// This value wraps at "boundary" (a large value you can read from SwParams).
    pub fn hw_ptr(&self) -> pcm::Frames {
        unsafe {
            ptr::read_volatile(&(*self.0.ptr).hw_ptr) as pcm::Frames
        }
    }

    /// Timestamp - fast version of alsa-lib's Status::get_htstamp
    ///
    /// Note: This just reads the actual value in memory.
    /// Unfortunately, the timespec is too big to be read atomically on most archs.
    /// Therefore, this function can potentially give bogus result at times, at least in theory...?
    pub fn htstamp(&self) -> libc::timespec {
        unsafe {
            ptr::read_volatile(&(*self.0.ptr).tstamp)
        }
    }

    /// Audio timestamp - fast version of alsa-lib's Status::get_audio_htstamp
    ///
    /// Note: This just reads the actual value in memory.
    /// Unfortunately, the timespec is too big to be read atomically on most archs.
    /// Therefore, this function can potentially give bogus result at times, at least in theory...?
    pub fn audio_htstamp(&self) -> libc::timespec {
        unsafe {
            ptr::read_volatile(&(*self.0.ptr).audio_tstamp)
        }
    }
}

/// Write PCM appl ptr directly, bypassing alsa-lib.
///
/// Provides direct access to appl ptr and avail min, without the overhead of
/// alsa-lib or a syscall. Caveats that apply to Status applies to this struct too.
#[derive(Debug)]
pub struct Control(DriverMemory<snd_pcm_mmap_control>);

impl Control {
    pub fn new(p: &pcm::PCM) -> Result<Self> { Self::from_fd(pcm_to_fd(p)?) }

    pub fn from_fd(fd: RawFd) -> Result<Self> {
        DriverMemory::new(fd, 1, SNDRV_PCM_MMAP_OFFSET_CONTROL as libc::off_t, true).map(Control)
    }

    /// Read number of frames application has read or written
    ///
    /// This value wraps at "boundary" (a large value you can read from SwParams).
    pub fn appl_ptr(&self) -> pcm::Frames {
        unsafe {
            ptr::read_volatile(&(*self.0.ptr).appl_ptr) as pcm::Frames
        }
    }

    /// Set number of frames application has read or written
    ///
    /// When the kernel wakes up due to a period interrupt, this value will
    /// be checked by the kernel. An XRUN will happen in case the application
    /// has not read or written enough data.
    pub fn set_appl_ptr(&self, value: pcm::Frames) {
        unsafe {
            ptr::write_volatile(&mut (*self.0.ptr).appl_ptr, value as snd_pcm_uframes_t)
        }
    }

    /// Read minimum number of frames in buffer in order to wakeup process
    pub fn avail_min(&self) -> pcm::Frames {
        unsafe {
            ptr::read_volatile(&(*self.0.ptr).avail_min) as pcm::Frames
        }
    }

    /// Write minimum number of frames in buffer in order to wakeup process
    pub fn set_avail_min(&self, value: pcm::Frames) {
        unsafe {
            ptr::write_volatile(&mut (*self.0.ptr).avail_min, value as snd_pcm_uframes_t)
        }
    }
}

struct DriverMemory<S> {
   ptr: *mut S,
   size: libc::size_t,
}

impl<S> fmt::Debug for DriverMemory<S> {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "DriverMemory({:?})", self.ptr) }
}

impl<S> DriverMemory<S> {
    fn new(fd: RawFd, count: usize, offs: libc::off_t, writable: bool) -> Result<Self> {
        let mut total = count * mem::size_of::<S>();
        let ps = pagesize();
        assert!(total > 0);
        if total % ps != 0 { total += ps - total % ps };
        let flags = if writable { libc::PROT_WRITE | libc::PROT_READ } else { libc::PROT_READ };
        let p = unsafe { libc::mmap(ptr::null_mut(), total, flags, libc::MAP_FILE | libc::MAP_SHARED, fd, offs) };
        if p.is_null() || p == libc::MAP_FAILED {
            Err(Error::last("mmap (of driver memory)"))
        } else {
            Ok(DriverMemory { ptr: p as *mut S, size: total })
        }
    }
}

unsafe impl<S> Send for DriverMemory<S> {}
unsafe impl<S> Sync for DriverMemory<S> {}

impl<S> Drop for DriverMemory<S> {
    fn drop(&mut self) {
        unsafe {{ libc::munmap(self.ptr as *mut libc::c_void, self.size); } }
    }
}

#[derive(Debug)]
struct SampleData<S> {
    mem: DriverMemory<S>,
    frames: pcm::Frames,
    channels: u32,
}

impl<S> SampleData<S> {
    pub fn new(p: &pcm::PCM) -> Result<Self> {
        let params = p.hw_params_current()?;
        let bufsize = params.get_buffer_size()?;
        let channels = params.get_channels()?;
        if params.get_access()? != pcm::Access::MMapInterleaved {
            return Err(Error::unsupported("Not MMAP interleaved data"))
        }

        let fd = pcm_to_fd(p)?;
        let info = unsafe {
            let mut info: snd_pcm_channel_info = mem::zeroed();
            sndrv_pcm_ioctl_channel_info(fd, &mut info)?;
            info
        };
        // println!("{:?}", info);
        if (info.step != channels * mem::size_of::<S>() as u32 * 8) || (info.first != 0) {
            return Err(Error::unsupported("MMAP data size mismatch"))
        }
        Ok(SampleData {
            mem: DriverMemory::new(fd, (bufsize as usize) * (channels as usize), info.offset as libc::off_t, true)?,
            frames: bufsize,
            channels,
        })
    }
}


/// Dummy trait for better generics
pub trait MmapDir: fmt::Debug {
    const DIR: Direction;
    fn avail(hwptr: Frames, applptr: Frames, buffersize: Frames, boundary: Frames) -> Frames;
}

/// Dummy struct for better generics
#[derive(Copy, Clone, Debug)]
pub struct Playback;

impl MmapDir for Playback {
    const DIR: Direction = Direction::Playback;
    #[inline]
    fn avail(hwptr: Frames, applptr: Frames, buffersize: Frames, boundary: Frames) -> Frames {
	let r = hwptr.wrapping_add(buffersize).wrapping_sub(applptr);
	let r = if r < 0 { r.wrapping_add(boundary) } else { r };
        if r as usize >= boundary as usize { r.wrapping_sub(boundary) } else { r }
    }
}

/// Dummy struct for better generics
#[derive(Copy, Clone, Debug)]
pub struct Capture;

impl MmapDir for Capture {
    const DIR: Direction = Direction::Capture;
    #[inline]
    fn avail(hwptr: Frames, applptr: Frames, _buffersize: Frames, boundary: Frames) -> Frames {
	let r = hwptr.wrapping_sub(applptr);
	if r < 0 { r.wrapping_add(boundary) } else { r }
    }
}

pub type MmapPlayback<S> = MmapIO<S, Playback>;

pub type MmapCapture<S> = MmapIO<S, Capture>;

#[derive(Debug)]
/// Struct containing direct I/O functions shared between playback and capture.
pub struct MmapIO<S, D> {
    data: SampleData<S>,
    c: Control,
    ss: Status,
    bound: Frames,
    dir: PhantomData<*const D>,
}

#[derive(Debug, Clone, Copy)]
/// A raw pointer to samples, and the amount of samples readable or writable.
pub struct RawSamples<S> {
    pub ptr: *mut S,
    pub frames: Frames,
    pub channels: u32,
}

impl<S> RawSamples<S> {
    #[inline]
    /// Returns `frames` * `channels`, i e the amount of samples (of type `S`) that can be read/written.
    pub fn samples(&self) -> isize { self.frames as isize * (self.channels as isize) }

    /// Writes samples from an iterator.
    ///
    /// Returns true if iterator was depleted, and the number of samples written.
    /// This is just raw read/write of memory.
    pub unsafe fn write_samples<I: Iterator<Item=S>>(&self, i: &mut I) -> (bool, isize) {
        let mut z = 0;
        let max_samples = self.samples();
        while z < max_samples {
            let b = if let Some(b) = i.next() { b } else { return (true, z) };
            ptr::write_volatile(self.ptr.offset(z), b);
            z += 1;
        };
        (false, z)
    }

}

impl<S, D: MmapDir> MmapIO<S, D> {
    fn new(p: &pcm::PCM) -> Result<Self> {
        if p.info()?.get_stream() != D::DIR {
            return Err(Error::unsupported("Wrong direction"));
        }
        let boundary = p.sw_params_current()?.get_boundary()?;
        Ok(MmapIO {
            data: SampleData::new(p)?,
            c: Control::new(p)?,
            ss: Status::new(p)?,
            bound: boundary,
            dir: PhantomData,
        })
    }
}

pub (crate) fn new_mmap<S, D: MmapDir>(p: &pcm::PCM) -> Result<MmapIO<S, D>> { MmapIO::new(p) }

impl<S, D: MmapDir> MmapIO<S, D> {
    /// Read current status
    pub fn status(&self) -> &Status { &self.ss }

    /// Read current number of frames committed by application
    ///
    /// This number wraps at 'boundary'.
    #[inline]
    pub fn appl_ptr(&self) -> Frames { self.c.appl_ptr() }

    /// Read current number of frames read / written by hardware
    ///
    /// This number wraps at 'boundary'.
    #[inline]
    pub fn hw_ptr(&self) -> Frames { self.ss.hw_ptr() }

    /// The number at which hw_ptr and appl_ptr wraps.
    #[inline]
    pub fn boundary(&self) -> Frames { self.bound }

    /// Total number of frames in hardware buffer
    #[inline]
    pub fn buffer_size(&self) -> Frames { self.data.frames }

    /// Number of channels in stream
    #[inline]
    pub fn channels(&self) -> u32 { self.data.channels }

    /// Notifies the kernel that frames have now been read / written by the application
    ///
    /// This will allow the kernel to write new data into this part of the buffer.
    pub fn commit(&self, v: Frames) {
        let mut z = self.appl_ptr() + v;
        if z + v >= self.boundary() { z -= self.boundary() };
        self.c.set_appl_ptr(z)
    }

    /// Number of frames available to read / write.
    ///
    /// In case of an underrun, this value might be bigger than the buffer size.
    pub fn avail(&self) -> Frames { D::avail(self.hw_ptr(), self.appl_ptr(), self.buffer_size(), self.boundary()) }

    /// Returns raw pointers to data to read / write.
    ///
    /// Use this if you want to read/write data yourself (instead of using iterators). If you do,
    /// using `write_volatile` or `read_volatile` is recommended, since it's DMA memory and can
    /// change at any time.
    ///
    /// Since this is a ring buffer, there might be more data to read/write in the beginning
    /// of the buffer as well. If so this is returned as the second return value.
    pub fn data_ptr(&self) -> (RawSamples<S>, Option<RawSamples<S>>) {
        let (hwptr, applptr) = (self.hw_ptr(), self.appl_ptr());
        let c = self.channels();
        let bufsize = self.buffer_size();

        // These formulas mostly mimic the behaviour of
        // snd_pcm_mmap_begin (in alsa-lib/src/pcm/pcm.c).
        let offs = applptr % bufsize;
        let mut a = D::avail(hwptr, applptr, bufsize, self.boundary());
        a = cmp::min(a, bufsize);
        let b = bufsize - offs;
        let more_data = if b < a {
            let z = a - b;
            a = b;
            Some( RawSamples { ptr: self.data.mem.ptr, frames: z, channels: c })
        } else { None };

        let p = unsafe { self.data.mem.ptr.offset(offs as isize * self.data.channels as isize) };
        (RawSamples { ptr: p, frames: a, channels: c }, more_data)
    }
}

impl<S> MmapPlayback<S> {
    /// Write samples to the kernel ringbuffer.
    pub fn write<I: Iterator<Item=S>>(&mut self, i: &mut I) -> Frames {
        let (data, more_data) = self.data_ptr();
        let (iter_end, samples) = unsafe { data.write_samples(i) };
        let mut z = samples / data.channels as isize;
        if !iter_end {
            if let Some(data2) = more_data {
                let (_, samples2) = unsafe {  data2.write_samples(i) };
                z += samples2 / data2.channels as isize;
            }
        }
        let z = z as Frames;
        self.commit(z);
        z
    }
}

impl<S> MmapCapture<S> {
    /// Read samples from the kernel ringbuffer.
    ///
    /// When the iterator is dropped or depleted, the read samples will be committed, i e,
    /// the kernel can then write data to the location again. So do this ASAP.
    pub fn iter(&mut self) -> CaptureIter<S> {
        let (data, more_data) = self.data_ptr();
        CaptureIter {
            m: self,
            samples: data,
            p_offs: 0,
            read_samples: 0,
            next_p: more_data,
        }
    }
}

/// Iterator over captured samples
pub struct CaptureIter<'a, S: 'static> {
    m: &'a MmapCapture<S>,
    samples: RawSamples<S>,
    p_offs: isize,
    read_samples: isize,
    next_p: Option<RawSamples<S>>,
}

impl<'a, S: 'static + Copy> CaptureIter<'a, S> {
    fn handle_max(&mut self) {
        self.p_offs = 0;
        if let Some(p2) = self.next_p.take() {
            self.samples = p2;
        } else {
            self.m.commit((self.read_samples / self.samples.channels as isize) as Frames);
            self.read_samples = 0;
            self.samples.frames = 0; // Shortcut to "None" in case anyone calls us again
        }
    }
}

impl<'a, S: 'static + Copy> Iterator for CaptureIter<'a, S> {
    type Item = S;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.p_offs >= self.samples.samples() {
            self.handle_max();
            if self.samples.frames <= 0 { return None; }
        }
        let s = unsafe { ptr::read_volatile(self.samples.ptr.offset(self.p_offs)) };
        self.p_offs += 1;
        self.read_samples += 1;
        Some(s)
    }
}

impl<'a, S: 'static> Drop for CaptureIter<'a, S> {
    fn drop(&mut self) {
        self.m.commit((self.read_samples / self.m.data.channels as isize) as Frames);
    }
}


#[test]
#[ignore] // Not everyone has a recording device on plughw:1. So let's ignore this test by default.
fn record_from_plughw_rw() {
    use crate::pcm::*;
    use crate::{ValueOr, Direction};
    use std::ffi::CString;
    let pcm = PCM::open(&*CString::new("plughw:1").unwrap(), Direction::Capture, false).unwrap();
    let ss = self::Status::new(&pcm).unwrap();
    let c = self::Control::new(&pcm).unwrap();
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(2).unwrap();
    hwp.set_rate(44100, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::s16()).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();

    {
        let swp = pcm.sw_params_current().unwrap();
        swp.set_tstamp_mode(true).unwrap();
        pcm.sw_params(&swp).unwrap();
    }
    assert_eq!(ss.state(), State::Prepared);
    pcm.start().unwrap();
    assert_eq!(c.appl_ptr(), 0);
    println!("{:?}, {:?}", ss, c);
    let mut buf = [0i16; 512*2];
    assert_eq!(pcm.io_i16().unwrap().readi(&mut buf).unwrap(), 512);
    assert_eq!(c.appl_ptr(), 512);

    assert_eq!(ss.state(), State::Running);
    assert!(ss.hw_ptr() >= 512);
    let t2 = ss.htstamp();
    assert!(t2.tv_sec > 0 || t2.tv_nsec > 0);
}


#[test]
#[ignore] // Not everyone has a record device on plughw:1. So let's ignore this test by default.
fn record_from_plughw_mmap() {
    use crate::pcm::*;
    use crate::{ValueOr, Direction};
    use std::ffi::CString;
    use std::{thread, time};

    let pcm = PCM::open(&*CString::new("plughw:1").unwrap(), Direction::Capture, false).unwrap();
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(2).unwrap();
    hwp.set_rate(44100, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::s16()).unwrap();
    hwp.set_access(Access::MMapInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();

    let ss = unsafe { SyncPtrStatus::sync_ptr(pcm_to_fd(&pcm).unwrap(), false, None, None).unwrap() };
    assert_eq!(ss.state(), State::Prepared);

    let mut m = pcm.direct_mmap_capture::<i16>().unwrap();

    assert_eq!(m.status().state(), State::Prepared);
    assert_eq!(m.appl_ptr(), 0);
    assert_eq!(m.hw_ptr(), 0);


    println!("{:?}", m);

    let now = time::Instant::now();
    pcm.start().unwrap();
    while m.avail() < 256 { thread::sleep(time::Duration::from_millis(1)) };
    assert!(now.elapsed() >= time::Duration::from_millis(256 * 1000 / 44100));
    let (ptr1, md) = m.data_ptr();
    assert_eq!(ptr1.channels, 2);
    assert!(ptr1.frames >= 256);
    assert!(md.is_none());
    println!("Has {:?} frames at {:?} in {:?}", m.avail(), ptr1.ptr, now.elapsed());
    let samples: Vec<i16> = m.iter().collect();
    assert!(samples.len() >= ptr1.frames as usize * 2);
    println!("Collected {} samples", samples.len());
    let (ptr2, _md) = m.data_ptr();
    assert!(unsafe { ptr1.ptr.offset(256 * 2) } <= ptr2.ptr);
}

#[test]
#[ignore]
fn playback_to_plughw_mmap() {
    use crate::pcm::*;
    use crate::{ValueOr, Direction};
    use std::ffi::CString;

    let pcm = PCM::open(&*CString::new("plughw:1").unwrap(), Direction::Playback, false).unwrap();
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(2).unwrap();
    hwp.set_rate(44100, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::s16()).unwrap();
    hwp.set_access(Access::MMapInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
    let mut m = pcm.direct_mmap_playback::<i16>().unwrap();

    assert_eq!(m.status().state(), State::Prepared);
    assert_eq!(m.appl_ptr(), 0);
    assert_eq!(m.hw_ptr(), 0);

    println!("{:?}", m);
    let mut i = (0..(m.buffer_size() * 2)).map(|i|
        (((i / 2) as f32 * 2.0 * ::std::f32::consts::PI / 128.0).sin() * 8192.0) as i16);
    m.write(&mut i);
    assert_eq!(m.appl_ptr(), m.buffer_size());

    pcm.start().unwrap();
    pcm.drain().unwrap();
    assert_eq!(m.appl_ptr(), m.buffer_size());
    assert!(m.hw_ptr() >= m.buffer_size());
}
