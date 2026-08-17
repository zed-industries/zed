//! Enumerate devices in the alsa library configuration
//!
//! # Example
//! Print all devices found in various categories.
//!
//! ```
//! use std::ffi::CString;
//! use alsa::device_name::HintIter;
//!
//! for t in &["pcm", "ctl", "rawmidi", "timer", "seq", "hwdep"] {
//!     println!("{} devices:", t);
//!     let i = HintIter::new(None, &*CString::new(*t).unwrap()).unwrap();
//!     for a in i { println!("  {:?}", a) }
//! }
//! ```

use std::ptr;
use libc::{c_void, c_int};
use crate::alsa;
use super::{Card, Direction};
use super::error::*;
use std::ffi::{CStr, CString};

/// [snd_device_name_hint](http://www.alsa-project.org/alsa-doc/alsa-lib/group___control.html) wrapper
pub struct HintIter(*mut *mut c_void, isize);

impl Drop for HintIter {
    fn drop(&mut self) { unsafe { alsa::snd_device_name_free_hint(self.0); }}
}

impl HintIter {
    /// typical interfaces are: "pcm", "ctl", "rawmidi", "timer", "seq" and "hwdep".
    pub fn new(card: Option<&Card>, iface: &CStr) -> Result<HintIter> {
        let mut p = ptr::null_mut();
        let cnr = card.map(|c| c.get_index()).unwrap_or(-1) as c_int;
        acheck!(snd_device_name_hint(cnr, iface.as_ptr(), &mut p))
            .map(|_| HintIter(p, 0))
    }

    /// A constructor variant that takes the interface as a Rust string slice.
    pub fn new_str(card: Option<&Card>, iface: &str) -> Result<HintIter> {
        HintIter::new(card, &CString::new(iface).unwrap())
    }
}

impl Iterator for HintIter {
    type Item = Hint;
    fn next(&mut self) -> Option<Hint> {
        if self.0.is_null() { return None; }
        let p = unsafe { *self.0.offset(self.1) };
        if p.is_null() { return None; }
        self.1 += 1;
        Some(Hint::new(p))
    }
}


/// [snd_device_name_get_hint](http://www.alsa-project.org/alsa-doc/alsa-lib/group___control.html) wrapper
#[derive(Debug, Clone)]
pub struct Hint {
    pub name: Option<String>,
    pub desc: Option<String>,
    pub direction: Option<Direction>,
}

impl Hint {
    fn get_str(p: *const c_void, name: &str) -> Option<String> {
        let name = CString::new(name).unwrap();
        let c = unsafe { alsa::snd_device_name_get_hint(p, name.as_ptr()) };
        from_alloc("snd_device_name_get_hint", c).ok()
    }

    fn new(p: *const c_void) -> Hint {
       let d = Hint::get_str(p, "IOID").and_then(|x| match &*x {
            "Input" => Some(Direction::Capture),
            "Output" => Some(Direction::Playback),
            _ => None,
       });
       Hint { name: Hint::get_str(p, "NAME"), desc: Hint::get_str(p, "DESC"), direction: d }
    }
}

#[test]
fn print_hints() {
    for t in &["pcm", "ctl", "rawmidi", "timer", "seq", "hwdep"] {
        println!("{} devices:", t);
        let i = HintIter::new(None, &*CString::new(*t).unwrap()).unwrap();
        for a in i { println!("  {:?}", a) }
    }
}
