//! MIDI devices I/O and enumeration

use libc::{c_int, c_uint, c_void, size_t, c_short, pollfd};
use super::ctl_int::{ctl_ptr, Ctl};
use super::{Direction, poll};
use super::error::*;
use crate::alsa;
use std::{ptr, io};
use std::ffi::{CStr, CString};

/// Iterator over [Rawmidi](http://www.alsa-project.org/alsa-doc/alsa-lib/group___raw_midi.html) devices and subdevices
pub struct Iter<'a> {
    ctl: &'a Ctl,
    device: c_int,
    in_count: i32,
    out_count: i32,
    current: i32,
}

/// [snd_rawmidi_info_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___raw_midi.html) wrapper
pub struct Info(*mut alsa::snd_rawmidi_info_t);

impl Drop for Info {
    fn drop(&mut self) { unsafe { alsa::snd_rawmidi_info_free(self.0) }; }
}

impl Info {
    fn new() -> Result<Info> {
        let mut p = ptr::null_mut();
        acheck!(snd_rawmidi_info_malloc(&mut p)).map(|_| Info(p))
    }

    fn from_iter(c: &Ctl, device: i32, sub: i32, dir: Direction) -> Result<Info> {
        let r = Info::new()?;
        unsafe { alsa::snd_rawmidi_info_set_device(r.0, device as c_uint) };
        let d = match dir {
            Direction::Playback => alsa::SND_RAWMIDI_STREAM_OUTPUT,
            Direction::Capture => alsa::SND_RAWMIDI_STREAM_INPUT,
        };
        unsafe { alsa::snd_rawmidi_info_set_stream(r.0, d) };
        unsafe { alsa::snd_rawmidi_info_set_subdevice(r.0, sub as c_uint) };
        acheck!(snd_ctl_rawmidi_info(ctl_ptr(c), r.0)).map(|_| r)
    }

    fn subdev_count(c: &Ctl, device: c_int) -> Result<(i32, i32)> {
        let i = Info::from_iter(c, device, 0, Direction::Capture)?;
        let o = Info::from_iter(c, device, 0, Direction::Playback)?;
        Ok((unsafe { alsa::snd_rawmidi_info_get_subdevices_count(o.0) as i32 },
            unsafe { alsa::snd_rawmidi_info_get_subdevices_count(i.0) as i32 }))
    }

    pub fn get_device(&self) -> i32 { unsafe { alsa::snd_rawmidi_info_get_device(self.0) as i32 }}
    pub fn get_subdevice(&self) -> i32 { unsafe { alsa::snd_rawmidi_info_get_subdevice(self.0) as i32 }}
    pub fn get_stream(&self) -> super::Direction {
        if unsafe { alsa::snd_rawmidi_info_get_stream(self.0) } == alsa::SND_RAWMIDI_STREAM_OUTPUT { super::Direction::Playback }
        else { super::Direction::Capture }
    }

    pub fn get_subdevice_name(&self) -> Result<String> {
        let c = unsafe { alsa::snd_rawmidi_info_get_subdevice_name(self.0) };
        from_const("snd_rawmidi_info_get_subdevice_name", c).map(|s| s.to_string())
    }
    pub fn get_id(&self) -> Result<String> {
        let c = unsafe { alsa::snd_rawmidi_info_get_id(self.0) };
        from_const("snd_rawmidi_info_get_id", c).map(|s| s.to_string())
    }
}

/// [snd_rawmidi_info_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___raw_midi.html) wrapper
pub struct Status(*mut alsa::snd_rawmidi_status_t);

impl Status {
    fn new() -> Result<Self> {
        let mut p = ptr::null_mut();
        acheck!(snd_rawmidi_status_malloc(&mut p)).map(|_| Status(p))
    }
}

impl Status {
    pub fn get_avail(&self) -> usize { unsafe { alsa::snd_rawmidi_status_get_avail(self.0 as *const _) } }
    pub fn get_xruns(&self) -> usize { unsafe { alsa::snd_rawmidi_status_get_xruns(self.0 as *const _) } }
}

impl Drop for Status {
    fn drop(&mut self) { unsafe { alsa::snd_rawmidi_status_free(self.0) }; }
}


impl<'a> Iter<'a> {
    pub fn new(c: &'a Ctl) -> Iter<'a> { Iter { ctl: c, device: -1, in_count: 0, out_count: 0, current: 0 }}
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result<Info>;
    fn next(&mut self) -> Option<Result<Info>> {
        if self.current < self.in_count {
            self.current += 1;
            return Some(Info::from_iter(self.ctl, self.device, self.current-1, Direction::Capture));
        }
        if self.current - self.in_count < self.out_count {
            self.current += 1;
            return Some(Info::from_iter(self.ctl, self.device, self.current-1-self.in_count, Direction::Playback));
        }

        let r = acheck!(snd_ctl_rawmidi_next_device(ctl_ptr(self.ctl), &mut self.device));
        match r {
            Err(e) => return Some(Err(e)),
            Ok(_) if self.device == -1 => return None,
            _ => {},
        }
        self.current = 0;
        match Info::subdev_count(self.ctl, self.device) {
            Err(e) => Some(Err(e)),
            Ok((oo, ii)) => {
                self.in_count = ii;
                self.out_count = oo;
                self.next()
            }
        }
    }
}

/// [snd_rawmidi_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___raw_midi.html) wrapper
pub struct Rawmidi(*mut alsa::snd_rawmidi_t);

unsafe impl Send for Rawmidi {}

impl Drop for Rawmidi {
    fn drop(&mut self) { unsafe { alsa::snd_rawmidi_close(self.0) }; }
}

impl Rawmidi {

    /// Wrapper around open that takes a &str instead of a &CStr
    pub fn new(name: &str, dir: Direction, nonblock: bool) -> Result<Self> {
        Self::open(&CString::new(name).unwrap(), dir, nonblock)
    }

    pub fn open(name: &CStr, dir: Direction, nonblock: bool) -> Result<Rawmidi> {
        let mut h = ptr::null_mut();
        let flags = if nonblock { 2 } else { 0 }; // FIXME: alsa::SND_RAWMIDI_NONBLOCK does not exist in alsa-sys
        acheck!(snd_rawmidi_open(
            if dir == Direction::Capture { &mut h } else { ptr::null_mut() },
            if dir == Direction::Playback { &mut h } else { ptr::null_mut() },
            name.as_ptr(), flags))
            .map(|_| Rawmidi(h))
    }

    pub fn info(&self) -> Result<Info> {
        Info::new().and_then(|i| acheck!(snd_rawmidi_info(self.0, i.0)).map(|_| i))
    }

    pub fn status(&self) -> Result<Status> {
        Status::new().and_then(|i| acheck!(snd_rawmidi_status(self.0, i.0)).map(|_| i))
    }

    pub fn drop(&self) -> Result<()> { acheck!(snd_rawmidi_drop(self.0)).map(|_| ()) }
    pub fn drain(&self) -> Result<()> { acheck!(snd_rawmidi_drain(self.0)).map(|_| ()) }
    pub fn name(&self) -> Result<String> {
        let c = unsafe { alsa::snd_rawmidi_name(self.0) };
        from_const("snd_rawmidi_name", c).map(|s| s.to_string())
    }

    pub fn io(&self) -> IO { IO(self) }
}

impl poll::Descriptors for Rawmidi {
    fn count(&self) -> usize {
        unsafe { alsa::snd_rawmidi_poll_descriptors_count(self.0) as usize }
    }
    fn fill(&self, p: &mut [pollfd]) -> Result<usize> {
        let z = unsafe { alsa::snd_rawmidi_poll_descriptors(self.0, p.as_mut_ptr(), p.len() as c_uint) };
        from_code("snd_rawmidi_poll_descriptors", z).map(|_| z as usize)
    }
    fn revents(&self, p: &[pollfd]) -> Result<poll::Flags> {
        let mut r = 0;
        let z = unsafe { alsa::snd_rawmidi_poll_descriptors_revents(self.0, p.as_ptr() as *mut pollfd, p.len() as c_uint, &mut r) };
        from_code("snd_rawmidi_poll_descriptors_revents", z).map(|_| poll::Flags::from_bits_truncate(r as c_short))
    }
}

/// Implements `std::io::Read` and `std::io::Write` for `Rawmidi`
pub struct IO<'a>(&'a Rawmidi);

impl<'a> io::Read for IO<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let r = unsafe { alsa::snd_rawmidi_read((self.0).0, buf.as_mut_ptr() as *mut c_void, buf.len() as size_t) };
        if r < 0 { Err(io::Error::from_raw_os_error(r as i32)) }
        else { Ok(r as usize) }
    }
}

impl<'a> io::Write for IO<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let r = unsafe { alsa::snd_rawmidi_write((self.0).0, buf.as_ptr() as *const c_void, buf.len() as size_t) };
        if r < 0 { Err(io::Error::from_raw_os_error(r as i32)) }
        else { Ok(r as usize) }
    }
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}


#[test]
fn print_rawmidis() {
    for a in super::card::Iter::new().map(|a| a.unwrap()) {
        for b in Iter::new(&Ctl::from_card(&a, false).unwrap()).map(|b| b.unwrap()) {
            println!("Rawmidi {:?} (hw:{},{},{}) {} - {}", b.get_stream(), a.get_index(), b.get_device(), b.get_subdevice(),
                 a.get_name().unwrap(), b.get_subdevice_name().unwrap())
        }
    }
}
