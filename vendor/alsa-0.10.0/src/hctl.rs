//! HCtl API - for mixer control and jack detection
//!
//! # Example
//! Print all jacks and their status
//!
//! ```
//! for a in ::alsa::card::Iter::new().map(|x| x.unwrap()) {
//!     use std::ffi::CString;
//!     use alsa::hctl::HCtl;
//!     let h = HCtl::open(&CString::new(format!("hw:{}", a.get_index())).unwrap(), false).unwrap();
//!     h.load().unwrap();
//!     for b in h.elem_iter() {
//!         use alsa::ctl::ElemIface;
//!         let id = b.get_id().unwrap();
//!         if id.get_interface() != ElemIface::Card { continue; }
//!         let name = id.get_name().unwrap();
//!         if !name.ends_with(" Jack") { continue; }
//!         if name.ends_with(" Phantom Jack") {
//!             println!("{} is always present", &name[..name.len()-13])
//!         }
//!         else { println!("{} is {}", &name[..name.len()-5],
//!             if b.read().unwrap().get_boolean(0).unwrap() { "plugged in" } else { "unplugged" })
//!         }
//!     }
//! }
//! ```

#![allow(dead_code)]
/* There is a "field is never read" warning for the ElemIter struct. We still need to hold on
   to the HCtl pointer somehow; to guarantee that the HCtl does not go out of scope while we use
   ElemIter, which would make the snd_hctl_elem_t pointer invalid. Hence the decision to allow dead code here.
   I suppose there is a better solution for this but I'm not sure how.
*/


use crate::{alsa, Card};
use std::ffi::{CStr, CString};
use super::error::*;
use std::ptr;
use super::{ctl_int, poll};
use libc::{c_short, c_uint, c_int, pollfd};


/// [snd_hctl_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___h_control.html) wrapper
pub struct HCtl(*mut alsa::snd_hctl_t);

unsafe impl Send for HCtl {}

impl Drop for HCtl {
    fn drop(&mut self) { unsafe { alsa::snd_hctl_close(self.0) }; }
}

impl HCtl {
    /// Wrapper around open that takes a &str instead of a &CStr
    pub fn new(c: &str, nonblock: bool) -> Result<HCtl> {
        Self::open(&CString::new(c).unwrap(), nonblock)
    }

    /// Open does not support async mode (it's not very Rustic anyway)
    /// Note: You probably want to call `load` afterwards.
    pub fn open(c: &CStr, nonblock: bool) -> Result<HCtl> {
        let mut r = ptr::null_mut();
        let flags = if nonblock { 1 } else { 0 }; // FIXME: alsa::SND_CTL_NONBLOCK does not exist in alsa-sys
        acheck!(snd_hctl_open(&mut r, c.as_ptr(), flags))
            .map(|_| HCtl(r))
    }

    /// Wrapper around open. You probably want to call `load` afterwards.
    pub fn from_card(c: &Card, nonblock: bool) -> Result<HCtl> {
        let s = format!("hw:{}", c.get_index());
        HCtl::new(&s, nonblock)
    }

    pub fn load(&self) -> Result<()> { acheck!(snd_hctl_load(self.0)).map(|_| ()) }

    pub fn elem_iter(&self) -> ElemIter { ElemIter(self, ptr::null_mut()) }

    pub fn find_elem(&self, id: &ctl_int::ElemId) -> Option<Elem> {
        let p = unsafe { alsa::snd_hctl_find_elem(self.0, ctl_int::elem_id_ptr(id)) };
        if p.is_null() { None } else { Some(Elem(self, p)) }
    }

    pub fn handle_events(&self) -> Result<u32> {
        acheck!(snd_hctl_handle_events(self.0)).map(|x| x as u32)
    }

    pub fn wait(&self, timeout_ms: Option<u32>) -> Result<bool> {
        acheck!(snd_hctl_wait(self.0, timeout_ms.map(|x| x as c_int).unwrap_or(-1))).map(|i| i == 1) }
}

impl poll::Descriptors for HCtl {
    fn count(&self) -> usize {
        unsafe { alsa::snd_hctl_poll_descriptors_count(self.0) as usize }
    }
    fn fill(&self, p: &mut [pollfd]) -> Result<usize> {
        let z = unsafe { alsa::snd_hctl_poll_descriptors(self.0, p.as_mut_ptr(), p.len() as c_uint) };
        from_code("snd_hctl_poll_descriptors", z).map(|_| z as usize)
    }
    fn revents(&self, p: &[pollfd]) -> Result<poll::Flags> {
        let mut r = 0;
        let z = unsafe { alsa::snd_hctl_poll_descriptors_revents(self.0, p.as_ptr() as *mut pollfd, p.len() as c_uint, &mut r) };
        from_code("snd_hctl_poll_descriptors_revents", z).map(|_| poll::Flags::from_bits_truncate(r as c_short))
    }
}

/// Iterates over elements for a `HCtl`
pub struct ElemIter<'a>(&'a HCtl, *mut alsa::snd_hctl_elem_t);

impl<'a> Iterator for ElemIter<'a> {
    type Item = Elem<'a>;
    fn next(&mut self) -> Option<Elem<'a>> {
        self.1 = if self.1.is_null() { unsafe { alsa::snd_hctl_first_elem((self.0).0) }}
            else { unsafe { alsa::snd_hctl_elem_next(self.1) }};
        if self.1.is_null() { None }
        else { Some(Elem(self.0, self.1)) }
    }
}


/// [snd_hctl_elem_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___h_control.html) wrapper
pub struct Elem<'a>(&'a HCtl, *mut alsa::snd_hctl_elem_t);

impl<'a> Elem<'a> {
    pub fn get_id(&self) -> Result<ctl_int::ElemId> {
        let v = ctl_int::elem_id_new()?;
        unsafe { alsa::snd_hctl_elem_get_id(self.1, ctl_int::elem_id_ptr(&v)) };
        Ok(v)
    }
    pub fn info(&self) -> Result<ctl_int::ElemInfo> {
        let v = ctl_int::elem_info_new()?;
        acheck!(snd_hctl_elem_info(self.1, ctl_int::elem_info_ptr(&v))).map(|_| v)
    }
    pub fn read(&self) -> Result<ctl_int::ElemValue> {
        let i = self.info()?;
        let v = ctl_int::elem_value_new(i.get_type(), i.get_count())?;
        acheck!(snd_hctl_elem_read(self.1, ctl_int::elem_value_ptr(&v))).map(|_| v)
    }

    pub fn write(&self, v: &ctl_int::ElemValue) -> Result<bool> {
        acheck!(snd_hctl_elem_write(self.1, ctl_int::elem_value_ptr(v))).map(|e| e > 0)
    }
}

#[test]
fn print_hctls() {
    for a in super::card::Iter::new().map(|x| x.unwrap()) {
        use std::ffi::CString;
        let h = HCtl::open(&CString::new(format!("hw:{}", a.get_index())).unwrap(), false).unwrap();
        h.load().unwrap();
        println!("Card {}:", a.get_name().unwrap());
        for b in h.elem_iter() {
            println!("  {:?} - {:?}", b.get_id().unwrap(), b.read().unwrap());
        }
    }
}

#[test]
fn print_jacks() {
    for a in super::card::Iter::new().map(|x| x.unwrap()) {
        use std::ffi::CString;
        let h = HCtl::open(&CString::new(format!("hw:{}", a.get_index())).unwrap(), false).unwrap();
        h.load().unwrap();
        for b in h.elem_iter() {
            let id = b.get_id().unwrap();
            if id.get_interface() != super::ctl_int::ElemIface::Card { continue; }
            let name = id.get_name().unwrap();
            if !name.ends_with(" Jack") { continue; }
            if name.ends_with(" Phantom Jack") {
                println!("{} is always present", &name[..name.len()-13])
            }
            else { println!("{} is {}", &name[..name.len()-5],
                if b.read().unwrap().get_boolean(0).unwrap() { "plugged in" } else { "unplugged" })
            }
        }
    }
}
