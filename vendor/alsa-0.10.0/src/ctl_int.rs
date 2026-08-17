
use crate::alsa;
use super::pcm::Info;
use std::ffi::{CStr, CString};
use super::Direction;
use super::error::*;
use super::mixer::MilliBel;
use super::Round;
use std::{ptr, mem, fmt, cmp};
use crate::{Card, poll};
use std::cell::UnsafeCell;
use libc::{c_uint, c_void, size_t, c_long, c_int, pollfd, c_short};

/// We prefer not to allocate for every ElemId, ElemInfo or ElemValue.
/// But we don't know if these will increase in the future or on other platforms.
/// Unfortunately, Rust does not support alloca, so hard-code the sizes for now.

const ELEM_ID_SIZE: usize = 64;
// const ELEM_VALUE_SIZE: usize = 1224;
// const ELEM_INFO_SIZE: usize = 272;

/// [snd_ctl_pcm_next_device](https://www.alsa-project.org/alsa-doc/alsa-lib/control_8c.html#accbb0be6e5ca7361ffec0ea304ed1b05) wrapper.
/// Iterate over devices of a card.
pub struct DeviceIter<'a>(&'a Ctl, c_int);

impl<'a> DeviceIter<'a>{
    pub fn new(ctl: &'a Ctl) -> DeviceIter<'a> {
        DeviceIter(ctl, -1)
    }
}

impl<'a> Iterator for DeviceIter<'a> {
    type Item = c_int;

    fn next(&mut self) -> Option<c_int> {
        match acheck!(snd_ctl_pcm_next_device(self.0.0, &mut self.1)) {
            Ok(_) if self.1 == -1 => None,
            Ok(_) => Some(self.1),
            Err(_) => None,
        }
    }
}

/// [snd_ctl_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___control.html) wrapper
pub struct Ctl(*mut alsa::snd_ctl_t);

unsafe impl Send for Ctl {}

impl Ctl {
    /// Wrapper around open that takes a &str instead of a &CStr
    pub fn new(c: &str, nonblock: bool) -> Result<Self> {
        Self::open(&CString::new(c).unwrap(), nonblock)
    }

    /// Open does not support async mode (it's not very Rustic anyway)
    pub fn open(c: &CStr, nonblock: bool) -> Result<Ctl> {
        let mut r = ptr::null_mut();
        let flags = if nonblock { 1 } else { 0 }; // FIXME: alsa::SND_CTL_NONBLOCK does not exist in alsa-sys
        acheck!(snd_ctl_open(&mut r, c.as_ptr(), flags)).map(|_| Ctl(r))
    }

    pub fn from_card(c: &Card, nonblock: bool) -> Result<Ctl> {
        let s = format!("hw:{}", c.get_index());
        Ctl::open(&CString::new(s).unwrap(), nonblock)
    }

    pub fn card_info(&self) -> Result<CardInfo> { CardInfo::new().and_then(|c|
        acheck!(snd_ctl_card_info(self.0, c.0)).map(|_| c)) }

    pub fn wait(&self, timeout_ms: Option<u32>) -> Result<bool> {
        acheck!(snd_ctl_wait(self.0, timeout_ms.map(|x| x as c_int).unwrap_or(-1))).map(|i| i == 1) }

    pub fn get_db_range(&self, id: &ElemId) -> Result<(MilliBel, MilliBel)> {
        let mut min: c_long = 0;
        let mut max: c_long = 0;
        acheck!(snd_ctl_get_dB_range(self.0, elem_id_ptr(id), &mut min, &mut max))
            .map(|_| (MilliBel(min as i64), MilliBel(max as i64)))
    }

    pub fn convert_to_db(&self, id: &ElemId, volume: i64) -> Result<MilliBel> {
        let mut m: c_long = 0;
        acheck!(snd_ctl_convert_to_dB(self.0, elem_id_ptr(id), volume as c_long, &mut m))
            .map(|_| (MilliBel(m as i64)))
    }

    pub fn convert_from_db(&self, id: &ElemId, mb: MilliBel, dir: Round) -> Result<i64> {
        let mut m: c_long = 0;
        acheck!(snd_ctl_convert_from_dB(self.0, elem_id_ptr(id), mb.0 as c_long, &mut m, dir as c_int))
            .map(|_| m as i64)
    }

    pub fn elem_read(&self, val: &mut ElemValue) -> Result<()> {
        acheck!(snd_ctl_elem_read(self.0, elem_value_ptr(val))).map(|_| ())
    }

    pub fn elem_write(&self, val: &ElemValue) -> Result<()> {
        acheck!(snd_ctl_elem_write(self.0, elem_value_ptr(val))).map(|_| ())
    }

    pub fn elem_lock(&self, id: &ElemId) -> Result<i32> {
        acheck!(snd_ctl_elem_lock(self.0, elem_id_ptr(id)))
    }

    pub fn elem_unlock(&self, id: &ElemId) -> Result<i32> {
        acheck!(snd_ctl_elem_unlock(self.0, elem_id_ptr(id)))
    }

    pub fn elem_list(&self) -> Result<ElemList> {
        // obtain the list of all the elements now that we know how many there are
        let list = elem_list_new(|list| {
            acheck!(snd_ctl_elem_list(self.0, list.0))?;
            Ok(list.get_count())
        })?;
        acheck!(snd_ctl_elem_list(self.0, list.0))?;
        Ok(list)
    }

    /// Note: According to alsa-lib documentation, you're also supposed to have functionality for
    /// returning whether or not you are subscribed. This does not work in practice, so I'm not
    /// including that here.
    pub fn subscribe_events(&self, subscribe: bool) -> Result<()> {
        acheck!(snd_ctl_subscribe_events(self.0, if subscribe { 1 } else { 0 })).map(|_| ())
    }

    pub fn read(&self) -> Result<Option<Event>> {
        let e = event_new()?;
        acheck!(snd_ctl_read(self.0, e.0)).map(|r| if r == 1 { Some(e) } else { None })
    }

    pub fn pcm_info(&self, device: u32, subdevice: u32, direction: Direction) -> Result<Info> {
        Info::new().and_then(|mut info| {
            info.set_device(device);
            info.set_subdevice(subdevice);
            info.set_stream(direction);
            acheck!(snd_ctl_pcm_info(self.0, info.0)).map(|_| info )
        })
    }
}

impl Drop for Ctl {
    fn drop(&mut self) { unsafe { alsa::snd_ctl_close(self.0) }; }
}

impl poll::Descriptors for Ctl {
    fn count(&self) -> usize {
        unsafe { alsa::snd_ctl_poll_descriptors_count(self.0) as usize }
    }
    fn fill(&self, p: &mut [pollfd]) -> Result<usize> {
        let z = unsafe { alsa::snd_ctl_poll_descriptors(self.0, p.as_mut_ptr(), p.len() as c_uint) };
        from_code("snd_ctl_poll_descriptors", z).map(|_| z as usize)
    }
    fn revents(&self, p: &[pollfd]) -> Result<poll::Flags> {
        let mut r = 0;
        let z = unsafe { alsa::snd_ctl_poll_descriptors_revents(self.0, p.as_ptr() as *mut pollfd, p.len() as c_uint, &mut r) };
        from_code("snd_ctl_poll_descriptors_revents", z).map(|_| poll::Flags::from_bits_truncate(r as c_short))
    }
}


pub fn ctl_ptr(a: &Ctl) -> *mut alsa::snd_ctl_t { a.0 }

/// [snd_ctl_card_info_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___control.html) wrapper
pub struct CardInfo(*mut alsa::snd_ctl_card_info_t);

impl Drop for CardInfo {
    fn drop(&mut self) { unsafe { alsa::snd_ctl_card_info_free(self.0) }}
}

impl CardInfo {
    fn new() -> Result<CardInfo> {
        let mut p = ptr::null_mut();
        acheck!(snd_ctl_card_info_malloc(&mut p)).map(|_| CardInfo(p))
    }

    pub fn get_id(&self) -> Result<&str> {
        from_const("snd_ctl_card_info_get_id", unsafe { alsa::snd_ctl_card_info_get_id(self.0) })}
    pub fn get_driver(&self) -> Result<&str> {
        from_const("snd_ctl_card_info_get_driver", unsafe { alsa::snd_ctl_card_info_get_driver(self.0) })}
    pub fn get_components(&self) -> Result<&str> {
        from_const("snd_ctl_card_info_get_components", unsafe { alsa::snd_ctl_card_info_get_components(self.0) })}
    pub fn get_longname(&self) -> Result<&str> {
        from_const("snd_ctl_card_info_get_longname", unsafe { alsa::snd_ctl_card_info_get_longname(self.0) })}
    pub fn get_name(&self) -> Result<&str> {
        from_const("snd_ctl_card_info_get_name", unsafe { alsa::snd_ctl_card_info_get_name(self.0) })}
    pub fn get_mixername(&self) -> Result<&str> {
        from_const("snd_ctl_card_info_get_mixername", unsafe { alsa::snd_ctl_card_info_get_mixername(self.0) })}
    pub fn get_card(&self) -> Card { Card::new(unsafe { alsa::snd_ctl_card_info_get_card(self.0) })}
}

alsa_enum!(
    /// [SND_CTL_ELEM_IFACE_xxx](http://www.alsa-project.org/alsa-doc/alsa-lib/group___control.html) constants
    ElemIface, ALL_ELEMIFACE[7],

    Card = SND_CTL_ELEM_IFACE_CARD,
    Hwdep = SND_CTL_ELEM_IFACE_HWDEP,
    Mixer = SND_CTL_ELEM_IFACE_MIXER,
    PCM = SND_CTL_ELEM_IFACE_PCM,
    Rawmidi = SND_CTL_ELEM_IFACE_RAWMIDI,
    Timer = SND_CTL_ELEM_IFACE_TIMER,
    Sequencer = SND_CTL_ELEM_IFACE_SEQUENCER,
);

alsa_enum!(
    /// [SND_CTL_ELEM_TYPE_xxx](http://www.alsa-project.org/alsa-doc/alsa-lib/group___control.html) constants
    ElemType, ALL_ELEMTYPE[7],

    None = SND_CTL_ELEM_TYPE_NONE,
    Boolean = SND_CTL_ELEM_TYPE_BOOLEAN,
    Integer = SND_CTL_ELEM_TYPE_INTEGER,
    Enumerated = SND_CTL_ELEM_TYPE_ENUMERATED,
    Bytes = SND_CTL_ELEM_TYPE_BYTES,
    IEC958 = SND_CTL_ELEM_TYPE_IEC958,
    Integer64 = SND_CTL_ELEM_TYPE_INTEGER64,
);

/// [snd_ctl_elem_value_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___control.html) wrapper
pub struct ElemValue {
    ptr: *mut alsa::snd_ctl_elem_value_t,
    etype: ElemType,
    count: u32,
}

impl Drop for ElemValue {
    fn drop(&mut self) { unsafe { alsa::snd_ctl_elem_value_free(self.ptr) }; }
}

pub fn elem_value_ptr(a: &ElemValue) -> *mut alsa::snd_ctl_elem_value_t { a.ptr }

pub fn elem_value_new(t: ElemType, count: u32) -> Result<ElemValue> {
    let mut p = ptr::null_mut();
    acheck!(snd_ctl_elem_value_malloc(&mut p))
        .map(|_| ElemValue { ptr: p, etype: t, count })
}

impl ElemValue {

    pub fn set_id(&mut self, id: &ElemId) {
        unsafe { alsa::snd_ctl_elem_value_set_id(self.ptr, elem_id_ptr(id)) }
    }

    // Note: The get_bytes hands out a reference to inside the object. Therefore, we can't treat
    // the content as "cell"ed, but must take a "&mut self" (to make sure the reference
    // from get_bytes has been dropped when calling a set_* function).

    pub fn get_boolean(&self, idx: u32) -> Option<bool> {
        if self.etype != ElemType::Boolean || idx >= self.count { None }
        else { Some( unsafe { alsa::snd_ctl_elem_value_get_boolean(self.ptr, idx as c_uint) } != 0) }
    }

    pub fn set_boolean(&mut self, idx: u32, val: bool) -> Option<()> {
        if self.etype != ElemType::Boolean || idx >= self.count { None }
        else { unsafe { alsa::snd_ctl_elem_value_set_boolean(self.ptr, idx as c_uint, if val {1} else {0}) }; Some(()) }
    }

    pub fn get_integer(&self, idx: u32) -> Option<i32> {
        if self.etype != ElemType::Integer || idx >= self.count { None }
        else { Some( unsafe { alsa::snd_ctl_elem_value_get_integer(self.ptr, idx as c_uint) } as i32) }
    }

    pub fn set_integer(&mut self, idx: u32, val: i32) -> Option<()> {
        if self.etype != ElemType::Integer || idx >= self.count { None }
        else { unsafe { alsa::snd_ctl_elem_value_set_integer(self.ptr, idx as c_uint, val as c_long) }; Some(()) }
    }

    pub fn get_integer64(&self, idx: u32) -> Option<i64> {
        if self.etype != ElemType::Integer64 || idx >= self.count { None }
        else { Some( unsafe { alsa::snd_ctl_elem_value_get_integer64(self.ptr, idx as c_uint) } as i64) }
    }

    pub fn set_integer64(&mut self, idx: u32, val: i64) -> Option<()> {
        if self.etype != ElemType::Integer || idx >= self.count { None }
        else { unsafe { alsa::snd_ctl_elem_value_set_integer64(self.ptr, idx as c_uint, val) }; Some(()) }
    }

    pub fn get_enumerated(&self, idx: u32) -> Option<u32> {
        if self.etype != ElemType::Enumerated || idx >= self.count { None }
        else { Some( unsafe { alsa::snd_ctl_elem_value_get_enumerated(self.ptr, idx as c_uint) } as u32) }
    }

    pub fn set_enumerated(&mut self, idx: u32, val: u32) -> Option<()> {
        if self.etype != ElemType::Enumerated || idx >= self.count { None }
        else { unsafe { alsa::snd_ctl_elem_value_set_enumerated(self.ptr, idx as c_uint, val as c_uint) }; Some(()) }
    }

    pub fn get_byte(&self, idx: u32) -> Option<u8> {
        if self.etype != ElemType::Bytes || idx >= self.count { None }
        else { Some( unsafe { alsa::snd_ctl_elem_value_get_byte(self.ptr, idx as c_uint) } as u8) }
    }

    pub fn set_byte(&mut self, idx: u32, val: u8) -> Option<()> {
        if self.etype != ElemType::Bytes || idx >= self.count { None }
        else { unsafe { alsa::snd_ctl_elem_value_set_byte(self.ptr, idx as c_uint, val) }; Some(()) }
    }

    pub fn get_bytes(&self) -> Option<&[u8]> {
        if self.etype != ElemType::Bytes { None }
        else { Some( unsafe { ::std::slice::from_raw_parts(
            alsa::snd_ctl_elem_value_get_bytes(self.ptr) as *const u8, self.count as usize) } ) }
    }

    pub fn set_bytes(&mut self, val: &[u8]) -> Option<()> {
        if self.etype != ElemType::Bytes || val.len() != self.count as usize { None }

        // Note: the alsa-lib function definition is broken. First, the pointer is declared as mut even
        // though it's const, and second, there is a "value" missing between "elem" and "set_bytes".
        else { unsafe { alsa::snd_ctl_elem_set_bytes(self.ptr, val.as_ptr() as *mut c_void, val.len() as size_t) }; Some(()) }
    }

    /// Creates a new ElemValue.
    pub fn new(t: ElemType) -> Result<ElemValue> {
        // See max length in include/uapi/sound/asound.h in linux kernel for these values
        let count = match t {
            ElemType::None => 1,
            ElemType::Boolean => 128,
            ElemType::Integer => 128,
            ElemType::Enumerated => 128,
            ElemType::Bytes => 512,
            ElemType::IEC958 => 1,
            ElemType::Integer64 => 64,
        };
        // if count > maxcount { return Err(Error::new(Some("ElemValue::new - count too large".into()), 1)) }
        let ev = elem_value_new(t, count)?;
        unsafe { alsa::snd_ctl_elem_value_clear(elem_value_ptr(&ev)) };
        Ok(ev)
    }

}

impl fmt::Debug for ElemValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ElemType::*;
        write!(f, "ElemValue({:?}", self.etype)?;
        for a in 0..self.count { match self.etype {
            Boolean => write!(f, ",{:?}", self.get_boolean(a).unwrap()),
            Integer => write!(f, ",{:?}", self.get_integer(a).unwrap()),
            Integer64 => write!(f, ",{:?}", self.get_integer64(a).unwrap()),
            Enumerated => write!(f, ",{:?}", self.get_enumerated(a).unwrap()),
            Bytes => write!(f, ",{:?}", self.get_byte(a).unwrap()),
            _ => Ok(()),
        }?};
        write!(f, ")")
    }
}

/// [snd_ctl_elem_info_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___control.html) wrapper
pub struct ElemInfo(*mut alsa::snd_ctl_elem_info_t);

pub fn elem_info_ptr(a: &ElemInfo) -> *mut alsa::snd_ctl_elem_info_t { a.0 }

impl Drop for ElemInfo {
    fn drop(&mut self) { unsafe { alsa::snd_ctl_elem_info_free(self.0) }; }
}

pub fn elem_info_new() -> Result<ElemInfo> {
    let mut p = ptr::null_mut();
    acheck!(snd_ctl_elem_info_malloc(&mut p)).map(|_| ElemInfo(p))
}

impl ElemInfo {
    pub fn get_type(&self) -> ElemType { ElemType::from_c_int(
        unsafe { alsa::snd_ctl_elem_info_get_type(self.0) } as c_int, "snd_ctl_elem_info_get_type").unwrap() }
    pub fn get_count(&self) -> u32 { unsafe { alsa::snd_ctl_elem_info_get_count(self.0) as u32 } }
}

//
// Non-allocating version of ElemId
//

/// [snd_ctl_elem_id_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___control.html) wrapper
pub struct ElemId(UnsafeCell<[u8; ELEM_ID_SIZE]>);

pub fn elem_id_new() -> Result<ElemId> {
    assert!(unsafe { alsa::snd_ctl_elem_id_sizeof() } as usize <= ELEM_ID_SIZE);
    Ok(ElemId(UnsafeCell::new(unsafe { mem::zeroed() })))
}

#[inline]
pub fn elem_id_ptr(a: &ElemId) -> *mut alsa::snd_ctl_elem_id_t { a.0.get() as *mut _ as *mut alsa::snd_ctl_elem_id_t }

unsafe impl Send for ElemId {}

impl Clone for ElemId {
    fn clone(&self) -> Self {
        ElemId(UnsafeCell::new(unsafe { *self.0.get() }))
    }
}

//
// Allocating version of ElemId
//

/*

/// [snd_ctl_elem_id_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___control.html) wrapper
pub struct ElemId(*mut alsa::snd_ctl_elem_id_t);

impl Drop for ElemId {
    fn drop(&mut self) { unsafe { alsa::snd_ctl_elem_id_free(self.0) }; }
}

pub fn elem_id_new() -> Result<ElemId> {
    let mut p = ptr::null_mut();
    acheck!(snd_ctl_elem_id_malloc(&mut p)).map(|_| ElemId(p))
}

pub fn elem_id_ptr(a: &ElemId) -> *mut alsa::snd_ctl_elem_id_t { a.0 }

*/

impl ElemId {
    pub fn get_name(&self) -> Result<&str> {
        from_const("snd_hctl_elem_id_get_name", unsafe { alsa::snd_ctl_elem_id_get_name(elem_id_ptr(self)) })}
    pub fn get_device(&self) -> u32 { unsafe { alsa::snd_ctl_elem_id_get_device(elem_id_ptr(self)) as u32 }}
    pub fn get_subdevice(&self) -> u32 { unsafe { alsa::snd_ctl_elem_id_get_subdevice(elem_id_ptr(self)) as u32 }}
    pub fn get_numid(&self) -> u32 { unsafe { alsa::snd_ctl_elem_id_get_numid(elem_id_ptr(self)) as u32 }}
    pub fn get_index(&self) -> u32 { unsafe { alsa::snd_ctl_elem_id_get_index(elem_id_ptr(self)) as u32 }}
    pub fn get_interface(&self) -> ElemIface { ElemIface::from_c_int(
        unsafe { alsa::snd_ctl_elem_id_get_interface(elem_id_ptr(self)) } as c_int, "snd_ctl_elem_id_get_interface").unwrap() }

    pub fn set_device(&mut self, v: u32) { unsafe { alsa::snd_ctl_elem_id_set_device(elem_id_ptr(self), v) }}
    pub fn set_subdevice(&mut self, v: u32) { unsafe { alsa::snd_ctl_elem_id_set_subdevice(elem_id_ptr(self), v) }}
    pub fn set_numid(&mut self, v: u32) { unsafe { alsa::snd_ctl_elem_id_set_numid(elem_id_ptr(self), v) }}
    pub fn set_index(&mut self, v: u32) { unsafe { alsa::snd_ctl_elem_id_set_index(elem_id_ptr(self), v) }}
    pub fn set_interface(&mut self, v: ElemIface) { unsafe { alsa::snd_ctl_elem_id_set_interface(elem_id_ptr(self), v as u32) }}
    pub fn set_name(&mut self, v: &CStr) { unsafe { alsa::snd_ctl_elem_id_set_name(elem_id_ptr(self), v.as_ptr()) }}

    /// Creates a new ElemId.
    ///
    /// To ensure safety (i e make sure we never have an invalid interface enum), we need to supply it to the "new" function.
    pub fn new(iface: ElemIface) -> Self {
        let mut r = elem_id_new().unwrap();
        r.set_interface(iface);
        r
    }
}

impl cmp::Eq for ElemId {}

impl cmp::PartialEq for ElemId {
    fn eq(&self, a: &ElemId) -> bool {
        self.get_numid() == a.get_numid() && self.get_interface() == a.get_interface() &&
        self.get_index() == a.get_index() && self.get_device() == a.get_device() &&
        self.get_subdevice() == a.get_subdevice() && self.get_name().ok() == a.get_name().ok()
    }
}

impl fmt::Debug for ElemId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let index = self.get_index();
        let device = self.get_device();
        let subdevice = self.get_subdevice();

        write!(f, "ElemId(#{}, {:?}, {:?}", self.get_numid(), self.get_interface(), self.get_name())?;
        if index > 0 { write!(f, ", index={}", index)? };
        if device > 0 || subdevice > 0 { write!(f, ", device={}", device)? };
        if subdevice > 0 { write!(f, ", subdevice={}", device)? };
        write!(f, ")")
    }
}

/// [snd_ctl_elem_list_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___control.html) wrapper
pub struct ElemList(*mut alsa::snd_ctl_elem_list_t);

impl Drop for ElemList {
    fn drop(&mut self) {
        unsafe { alsa::snd_ctl_elem_list_free_space(self.0) };
        unsafe { alsa::snd_ctl_elem_list_free(self.0) };
    }
}

fn elem_list_new<F: FnOnce(&ElemList) -> Result<u32>>(f: F) -> Result<ElemList> {
    let mut p = ptr::null_mut();
    let list = acheck!(snd_ctl_elem_list_malloc(&mut p)).map(|_| ElemList(p))?;
    let count = f(&list)?;
    if count > 0 {
        acheck!(snd_ctl_elem_list_alloc_space(list.0, count))?;
    }
    Ok(list)
}

impl ElemList {
    #[inline]
    fn ensure_valid_index(&self, index: u32) -> Result<()> {
        if index >= self.get_used() {
            Err(Error::new("snd_ctl_elem_list_*", libc::EINVAL))
        } else {
            Ok(())
        }
    }

    pub(crate) fn get_count(&self) -> u32 { unsafe { alsa::snd_ctl_elem_list_get_count(self.0) } }
    pub fn get_used(&self) -> u32 { unsafe { alsa::snd_ctl_elem_list_get_used(self.0) } }
    pub fn get_id(&self, index: u32) -> Result<ElemId> {
        self.ensure_valid_index(index)?;
        let elem_id = elem_id_new()?;
        unsafe { alsa::snd_ctl_elem_list_get_id(self.0, index, elem_id_ptr(&elem_id)) };
        Ok(elem_id)
    }
    pub fn get_numid(&self, index: u32) -> Result<u32> { self.ensure_valid_index(index)?; Ok(unsafe { alsa::snd_ctl_elem_list_get_numid(self.0, index) }) }
    pub fn get_interface(&self, index: u32) -> Result<ElemIface> {
        self.ensure_valid_index(index)?;
        ElemIface::from_c_int(unsafe { alsa::snd_ctl_elem_list_get_interface(self.0, index) } as c_int, "snd_ctl_elem_list_get_interface")
    }
    pub fn get_device(&self, index: u32) -> Result<u32> { self.ensure_valid_index(index)?; Ok(unsafe { alsa::snd_ctl_elem_list_get_device(self.0, index) }) }
    pub fn get_subdevice(&self, index: u32) -> Result<u32> { self.ensure_valid_index(index)?; Ok(unsafe { alsa::snd_ctl_elem_list_get_subdevice(self.0, index) }) }
    pub fn get_name(&self, index: u32) -> Result<&str> {
        self.ensure_valid_index(index)?;
        from_const("snd_ctl_elem_list_get_name", unsafe { alsa::snd_ctl_elem_list_get_name(self.0, index) })
    }
    pub fn get_index(&self, index: u32) -> Result<u32> { self.ensure_valid_index(index)?; Ok(unsafe { alsa::snd_ctl_elem_list_get_index(self.0, index) }) }
}

/// [snd_ctl_event_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___control.html) wrapper
pub struct Event(*mut alsa::snd_ctl_event_t);

impl Drop for Event {
    fn drop(&mut self) { unsafe { alsa::snd_ctl_event_free(self.0) }; }
}

pub fn event_new() -> Result<Event> {
    let mut p = ptr::null_mut();
    acheck!(snd_ctl_event_malloc(&mut p)).map(|_| Event(p))
}

impl Event {
    pub fn get_mask(&self) -> EventMask { EventMask(unsafe { alsa::snd_ctl_event_elem_get_mask(self.0) as u32 })}
    pub fn get_id(&self) -> ElemId {
        let r = elem_id_new().unwrap();
        unsafe { alsa::snd_ctl_event_elem_get_id(self.0, elem_id_ptr(&r)) };
        r
    }
}


/// [SND_CTL_EVENT_MASK_XXX](http://www.alsa-project.org/alsa-doc/alsa-lib/group___control.html) bitmask
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EventMask(pub u32);

impl EventMask {
   pub fn remove(&self) -> bool { return self.0 & 0xffffffff == 0xffffffff }
   pub fn value(&self) -> bool { return (!self.remove()) && (self.0 & (1 << 0) != 0); }
   pub fn info(&self) -> bool { return (!self.remove()) && (self.0 & (1 << 1) != 0); }
   pub fn add(&self) -> bool { return (!self.remove()) && (self.0 & (1 << 2) != 0); }
   pub fn tlv(&self) -> bool { return (!self.remove()) && (self.0 & (1 << 3) != 0); }
}

#[test]
fn print_sizeof() {
    let elemid = unsafe { alsa::snd_ctl_elem_id_sizeof() } as usize;
    let elemvalue = unsafe { alsa::snd_ctl_elem_value_sizeof() } as usize;
    let eleminfo = unsafe { alsa::snd_ctl_elem_info_sizeof() } as usize;

    assert!(elemid <= ELEM_ID_SIZE);
//    assert!(elemvalue <= ELEM_VALUE_SIZE);
//    assert!(eleminfo <= ELEM_INFO_SIZE);

    println!("Elem id: {}, Elem value: {}, Elem info: {}", elemid, elemvalue, eleminfo);
}
