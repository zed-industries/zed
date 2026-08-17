//! Mixer API - Simple Mixer API for mixer control
//!
use std::ffi::{CStr, CString};
use std::{ptr, mem, fmt, ops};
use libc::{c_long, c_int, c_uint, c_short, pollfd};
use crate::poll;

use crate::alsa;
use super::Round;
use super::error::*;

const SELEM_ID_SIZE: usize = 64;

/// wraps [snd_mixer_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___mixer.html)
#[derive(Debug)]
pub struct Mixer(*mut alsa::snd_mixer_t);

unsafe impl Send for Mixer {}

impl Mixer {
    /// Opens a mixer and attaches it to a card identified by its name (like hw:0) and loads the
    /// mixer after registering a Selem.
    pub fn new(name: &str, nonblock: bool) -> Result<Mixer> {
        let mut mixer = Mixer::open(nonblock)?;
        mixer.attach(&CString::new(name).unwrap())?;
        Selem::register(&mut mixer)?;
        mixer.load()?;
        Ok(mixer)
    }

    /// Creates a Selem by looking for a specific selem by name given a mixer (of a card)
    pub fn find_selem(&self, id: &SelemId) -> Option<Selem> {
        let selem = unsafe { alsa::snd_mixer_find_selem(self.0, id.as_ptr()) };

        if selem.is_null() { None }
        else { Some(Selem(Elem {handle: selem, _mixer: self})) }
    }

    pub fn open(nonblock: bool) -> Result<Mixer> {
        let mut r = ptr::null_mut();
        let flags = if nonblock { 1 } else { 0 }; // FIXME: alsa::SND_CTL_NONBLOCK does not exist in alsa-sys
        acheck!(snd_mixer_open(&mut r, flags)).map(|_| Mixer(r))
    }

    pub fn attach(&mut self, name: &CStr) -> Result<()> {
        acheck!(snd_mixer_attach(self.0, name.as_ptr())).map(|_| ())
    }

    pub fn load(&mut self) -> Result<()> {
        acheck!(snd_mixer_load(self.0)).map(|_| ())
    }

    pub fn iter(&self) -> Iter {
        Iter {
            last_handle: ptr::null_mut(),
            mixer: self
        }
    }

    pub fn handle_events(&self) -> Result<u32> {
        acheck!(snd_mixer_handle_events(self.0)).map(|x| x as u32)
    }

    pub fn wait(&self, timeout_ms: Option<u32>) -> Result<()> {
        acheck!(snd_mixer_wait(self.0, timeout_ms.map(|x| x as c_int).unwrap_or(-1))).map(|_| ()) }
}

/// Closes mixer and frees used resources
impl Drop for Mixer {
    fn drop(&mut self) {
        unsafe { alsa::snd_mixer_close(self.0) };
    }
}


impl poll::Descriptors for Mixer {
    fn count(&self) -> usize {
        unsafe { alsa::snd_mixer_poll_descriptors_count(self.0) as usize }
    }
    fn fill(&self, p: &mut [pollfd]) -> Result<usize> {
        let z = unsafe { alsa::snd_mixer_poll_descriptors(self.0, p.as_mut_ptr(), p.len() as c_uint) };
        from_code("snd_mixer_poll_descriptors", z).map(|_| z as usize)
    }
    fn revents(&self, p: &[pollfd]) -> Result<poll::Flags> {
        let mut r = 0;
        let z = unsafe { alsa::snd_mixer_poll_descriptors_revents(self.0, p.as_ptr() as *mut pollfd, p.len() as c_uint, &mut r) };
        from_code("snd_mixer_poll_descriptors_revents", z).map(|_| poll::Flags::from_bits_truncate(r as c_short))
    }
}


/// Wrapper for a mB (millibel) value.
///
/// Despite some ALSA functions named "dB", they actually take mB values instead.
/// This is a wrapper type to help with those calculations. Its interior is the
/// actual mB value.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MilliBel(pub i64);

impl MilliBel {
    pub fn to_db(self) -> f32 { (self.0 as f32) / 100.0 }
    pub fn from_db(db: f32) -> Self { MilliBel((db * 100.0) as i64) }
}

impl ops::Deref for MilliBel {
    type Target = i64;
    fn deref(&self) -> &i64 { &self.0 }
}

impl ops::Add for MilliBel {
    type Output = MilliBel;
    fn add(self, rhs: Self) -> Self { MilliBel(self.0 + rhs.0) }
}

impl ops::AddAssign for MilliBel {
    fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0 }
}

impl ops::Sub for MilliBel {
    type Output = MilliBel;
    fn sub(self, rhs: Self) -> Self { MilliBel(self.0 - rhs.0) }
}

impl ops::SubAssign for MilliBel {
    fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0 }
}

/// Wraps [snd_mixer_elem_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___mixer.html)
#[derive(Copy, Clone, Debug)]
pub struct Elem<'a>{
    handle: *mut alsa::snd_mixer_elem_t,
    _mixer: &'a Mixer
}

/// Iterator for all elements of mixer
#[derive(Copy, Clone)]
pub struct Iter<'a>{
    last_handle: *mut alsa::snd_mixer_elem_t,
    mixer: &'a Mixer
}

impl<'a> Iterator for Iter<'a> {
    type Item = Elem<'a>;

    fn next(&mut self) -> Option<Elem<'a>> {
        let elem = if self.last_handle.is_null() {
            unsafe { alsa::snd_mixer_first_elem(self.mixer.0) }
        } else {
            unsafe { alsa::snd_mixer_elem_next(self.last_handle) }
        };

        if elem.is_null() {
            None
        } else {
            self.last_handle = elem;
            Some(Elem { handle: elem, _mixer: self.mixer})
        }
    }

}

/// Wrapper for [snd_mixer_selem_id_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___simple_mixer.html)
/// No allocation (uses fixed array)
// #[derive(Copy, Clone, Debug)]
pub struct SelemId([u8; SELEM_ID_SIZE]);

impl SelemId {

    pub fn new(name: &str, index: u32) -> SelemId {
        let mut s = SelemId::empty();
        s.set_name(&CString::new(name).unwrap());
        s.set_index(index);
        s
    }

    /// Returns an empty (zeroed) SelemId. This id is not a usable id and need to be initialized
    /// like `SelemId::new()` does
    pub fn empty() -> SelemId {
        assert!(unsafe { alsa::snd_mixer_selem_id_sizeof() } as usize <= SELEM_ID_SIZE);
        // Create empty selem_id and fill from mixer
        SelemId(unsafe { mem::zeroed() })
    }

    /// Convert SelemId into ``*mut snd_mixer_selem_id_t` that the alsa call needs.
    /// See [snd_mixer_selem_id_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___simple_mixer.html)
    #[inline]
    fn as_ptr(&self) -> *mut alsa::snd_mixer_selem_id_t {
        self.0.as_ptr() as *const _ as *mut alsa::snd_mixer_selem_id_t
    }

    pub fn get_name(&self) -> Result<&str> {
        let c = unsafe { alsa::snd_mixer_selem_id_get_name(self.as_ptr()) };
        from_const("snd_mixer_selem_id_get_name", c)
    }

    pub fn get_index(&self) -> u32 {
        unsafe { alsa::snd_mixer_selem_id_get_index(self.as_ptr()) }
    }

    pub fn set_name(&mut self, name: &CStr) {
        unsafe { alsa::snd_mixer_selem_id_set_name(self.as_ptr(), name.as_ptr()) };
    }

    pub fn set_index(&mut self, index: u32) {
        unsafe { alsa::snd_mixer_selem_id_set_index(self.as_ptr(), index) };
    }

}

/// Wraps an Elem as a Selem
// #[derive(Copy, Clone)]
pub struct Selem<'a>(Elem<'a>);

impl<'a> Selem<'a> {
    /// Creates a Selem by wrapping `elem`.
    pub fn new(elem: Elem<'a>) -> Option<Selem<'a>> {
        if unsafe { alsa::snd_mixer_elem_get_type(elem.handle) } == alsa::SND_MIXER_ELEM_SIMPLE
           { Some(Selem(elem)) } else { None }
    }

    /// TODO: This function might change to support regopt and to return the mixer class
    pub fn register(mixer: &mut Mixer) -> Result<()> {
        acheck!(snd_mixer_selem_register(mixer.0, ptr::null_mut(), ptr::null_mut())).map(|_| ())
    }

    pub fn get_id(&self) -> SelemId {
        let id = SelemId::empty();
        unsafe { alsa::snd_mixer_selem_get_id(self.handle, id.as_ptr()) };
        id
    }

    pub fn has_capture_volume(&self) -> bool {
        unsafe { alsa::snd_mixer_selem_has_capture_volume(self.handle) > 0 }
    }

    pub fn has_capture_switch(&self) -> bool {
        unsafe { alsa::snd_mixer_selem_has_capture_switch(self.handle) > 0 }
    }

    pub fn has_playback_volume(&self) -> bool {
        unsafe { alsa::snd_mixer_selem_has_playback_volume(self.handle) > 0 }
    }

    pub fn has_playback_switch(&self) -> bool {
        unsafe { alsa::snd_mixer_selem_has_playback_switch(self.handle) > 0 }
    }

    pub fn can_capture(&self) -> bool {
        self.has_capture_volume() || self.has_capture_switch()
    }

    pub fn can_playback(&self) -> bool {
        self.has_playback_volume() || self.has_playback_switch()
    }

    pub fn has_volume(&self) -> bool {
        self.has_capture_volume() || self.has_playback_volume()
    }

    /// returns range for capture volume as (min, max) values
    pub fn get_capture_volume_range(&self) -> (i64, i64) {
        let mut min: c_long = 0;
        let mut max: c_long = 0;
        unsafe { alsa::snd_mixer_selem_get_capture_volume_range(self.handle, &mut min, &mut max) };
        (min as i64, max as i64)
    }

    /// returns (min, max) values.
    pub fn get_capture_db_range(&self) -> (MilliBel, MilliBel) {
        let mut min: c_long = 0;
        let mut max: c_long = 0;
        unsafe { alsa::snd_mixer_selem_get_capture_dB_range(self.handle, &mut min, &mut max) };
        (MilliBel(min as i64), MilliBel(max as i64))
    }

    /// returns (min, max) values.
    pub fn get_playback_volume_range(&self) -> (i64, i64) {
        let mut min: c_long = 0;
        let mut max: c_long = 0;
        unsafe { alsa::snd_mixer_selem_get_playback_volume_range(self.handle, &mut min, &mut max) };
        (min as i64, max as i64)
    }

    /// returns (min, max) values.
    pub fn get_playback_db_range(&self) -> (MilliBel, MilliBel) {
        let mut min: c_long = 0;
        let mut max: c_long = 0;
        unsafe { alsa::snd_mixer_selem_get_playback_dB_range(self.handle, &mut min, &mut max) };
        (MilliBel(min as i64), MilliBel(max as i64))
    }

    pub fn is_capture_mono(&self) -> bool {
        unsafe { alsa::snd_mixer_selem_is_capture_mono(self.handle) == 1 }
    }

    pub fn is_playback_mono(&self) -> bool {
        unsafe { alsa::snd_mixer_selem_is_playback_mono(self.handle) == 1 }
    }

    pub fn has_capture_channel(&self, channel: SelemChannelId) -> bool {
        unsafe { alsa::snd_mixer_selem_has_capture_channel(self.handle, channel as i32) > 0 }
    }

    pub fn has_playback_channel(&self, channel: SelemChannelId) -> bool {
        unsafe { alsa::snd_mixer_selem_has_playback_channel(self.handle, channel as i32) > 0 }
    }

    /// Gets name from snd_mixer_selem_channel_name
    pub fn channel_name(channel: SelemChannelId) -> Result<&'static str> {
        let c = unsafe { alsa::snd_mixer_selem_channel_name(channel as i32) };
        from_const("snd_mixer_selem_channel_name", c)
    }

    pub fn get_playback_volume(&self, channel: SelemChannelId) -> Result<i64> {
        let mut value: c_long = 0;
        acheck!(snd_mixer_selem_get_playback_volume(self.handle, channel as i32, &mut value)).and_then(|_| Ok(value as i64))
    }

    /// returns volume in millibels.
    pub fn get_playback_vol_db(&self, channel: SelemChannelId) -> Result<MilliBel> {
        self.get_playback_volume(channel)
            .and_then(|volume| self.ask_playback_vol_db(volume))
    }

    /// Asks alsa to convert playback volume to millibels.
    pub fn ask_playback_vol_db(&self, volume: i64) -> Result<MilliBel> {
        let mut decibel_value: c_long = 0;
        acheck!(snd_mixer_selem_ask_playback_vol_dB(self.handle, volume as c_long, &mut decibel_value))
            .map(|_| MilliBel(decibel_value as i64))
    }

    // Asks alsa to convert millibels to playback volume.
    pub fn ask_playback_db_vol(&self, db: MilliBel, dir: Round) -> Result<i64> {
        let mut raw_volume: c_long = 0;
        acheck!(snd_mixer_selem_ask_playback_dB_vol(self.handle, db.0 as c_long, dir as c_int, &mut raw_volume))
            .map(|_| raw_volume as i64)
    }

    pub fn get_capture_volume(&self, channel: SelemChannelId) -> Result<i64> {
        let mut value: c_long = 0;
        acheck!(snd_mixer_selem_get_capture_volume(self.handle, channel as i32, &mut value)).map(|_| value as i64)
    }

    /// returns volume in millibels.
    pub fn get_capture_vol_db(&self, channel: SelemChannelId) -> Result<MilliBel> {
        self.get_capture_volume(channel)
            .and_then(|volume| self.ask_capture_vol_db(volume))
    }

    /// Asks alsa to convert capture volume to millibels
    pub fn ask_capture_vol_db(&self, volume: i64) -> Result<MilliBel> {
        let mut decibel_value: c_long = 0;
        acheck!(snd_mixer_selem_ask_capture_vol_dB (self.handle, volume as c_long, &mut decibel_value))
            .map(|_| MilliBel(decibel_value as i64))
    }

    // Asks alsa to convert millibels to capture volume.
    pub fn ask_capture_db_vol(&self, db: MilliBel, dir: Round) -> Result<i64> {
        let mut raw_volume: c_long = 0;
        acheck!(snd_mixer_selem_ask_capture_dB_vol(self.handle, db.0 as c_long, dir as c_int, &mut raw_volume))
            .map(|_| raw_volume as i64)
    }

    pub fn set_playback_volume(&self, channel: SelemChannelId, value: i64) -> Result<()> {
        acheck!(snd_mixer_selem_set_playback_volume(self.handle, channel as i32, value as c_long)).map(|_| ())
    }

    pub fn set_playback_volume_range(&self, min: i64, max: i64) -> Result<()> {
        acheck!(snd_mixer_selem_set_playback_volume_range(self.handle, min as c_long, max as c_long)).map(|_| ())
    }

    pub fn set_playback_volume_all(&self, value: i64) -> Result<()> {
        acheck!(snd_mixer_selem_set_playback_volume_all(self.handle, value as c_long)).map(|_| ())
    }

    pub fn set_playback_db(&self, channel: SelemChannelId, value: MilliBel, dir: Round) -> Result<()> {
        acheck!(snd_mixer_selem_set_playback_dB(self.handle, channel as i32, *value as c_long, dir as c_int)).map(|_| ())
    }

    pub fn set_capture_db(&self, channel: SelemChannelId, value: MilliBel, dir: Round) -> Result<()> {
        acheck!(snd_mixer_selem_set_capture_dB(self.handle, channel as i32, *value as c_long, dir as c_int)).map(|_| ())
    }

    pub fn set_playback_db_all(&self, value: MilliBel, dir: Round) -> Result<()> {
        acheck!(snd_mixer_selem_set_playback_dB_all(self.handle, *value as c_long, dir as c_int)).map(|_| ())
    }

    pub fn set_capture_db_all(&self, value: MilliBel, dir: Round) -> Result<()> {
        acheck!(snd_mixer_selem_set_capture_dB_all(self.handle, *value as c_long, dir as c_int)).map(|_| ())
    }

    pub fn set_capture_volume(&self, channel: SelemChannelId, value: i64) -> Result<()> {
        acheck!(snd_mixer_selem_set_capture_volume(self.handle, channel as i32, value as c_long)).map(|_| ())
    }

    pub fn set_capture_volume_range(&self, min: i64, max: i64) -> Result<()> {
        acheck!(snd_mixer_selem_set_capture_volume_range(self.handle, min as c_long, max as c_long)).map(|_| ())
    }

    pub fn set_capture_volume_all(&self, value: i64) -> Result<()> {
        acheck!(snd_mixer_selem_set_capture_volume_all(self.handle, value as c_long)).map(|_| ())
    }

    pub fn set_playback_switch(&self, channel: SelemChannelId, value: i32) -> Result<()> {
        acheck!(snd_mixer_selem_set_playback_switch(self.handle, channel as i32, value)).map(|_| ())
    }

    pub fn set_playback_switch_all(&self, value: i32) -> Result<()> {
        acheck!(snd_mixer_selem_set_playback_switch_all(self.handle, value)).map(|_| ())
    }

    pub fn set_capture_switch(&self, channel: SelemChannelId, value: i32) -> Result<()> {
        acheck!(snd_mixer_selem_set_capture_switch(self.handle, channel as i32, value)).map(|_| ())
    }

    pub fn set_capture_switch_all(&self, value: i32) -> Result<()> {
        acheck!(snd_mixer_selem_set_capture_switch_all(self.handle, value)).map(|_| ())
    }

    pub fn get_playback_switch(&self, channel: SelemChannelId) -> Result<i32> {
        let mut value: i32 = 0;
        acheck!(snd_mixer_selem_get_playback_switch(self.handle, channel as i32, &mut value)).map(|_| value)
    }

    pub fn get_capture_switch(&self, channel: SelemChannelId) -> Result<i32> {
        let mut value: i32 = 0;
        acheck!(snd_mixer_selem_get_capture_switch(self.handle, channel as i32, &mut value)).map(|_| value)
    }

    pub fn is_enumerated(&self) -> bool {
        unsafe { alsa::snd_mixer_selem_is_enumerated(self.handle) == 1 }
    }

    pub fn is_enum_playback(&self) -> bool {
        unsafe { alsa::snd_mixer_selem_is_enum_playback(self.handle) == 1 }
    }

    pub fn is_enum_capture(&self) -> bool {
        unsafe { alsa::snd_mixer_selem_is_enum_capture(self.handle) == 1 }
    }

    pub fn get_enum_items(&self) -> Result<u32> {
        acheck!(snd_mixer_selem_get_enum_items(self.handle)).map(|v| v as u32)
    }

    pub fn get_enum_item_name(&self, idx: u32) -> Result<String> {
        let mut temp = [0 as ::libc::c_char; 128];
        acheck!(snd_mixer_selem_get_enum_item_name(self.handle, idx, temp.len()-1, temp.as_mut_ptr()))
            .and_then(|_| from_const("snd_mixer_selem_get_enum_item_name", temp.as_ptr()))
            .map(|v| v.into())
    }

    /// Enumerates over valid Enum values
    pub fn iter_enum(&self) -> Result<IterEnum> {
        Ok(IterEnum(self, 0, self.get_enum_items()?))
    }

    pub fn get_enum_item(&self, channel: SelemChannelId) -> Result<u32> {
        let mut temp = 0;
        acheck!(snd_mixer_selem_get_enum_item(self.handle, channel as i32, &mut temp))
            .map(|_| temp)
    }

    pub fn set_enum_item(&self, channel: SelemChannelId, idx: u32) -> Result<()> {
        acheck!(snd_mixer_selem_set_enum_item(self.handle, channel as i32, idx))
            .map(|_| ())
    }
}

impl<'a> ops::Deref for Selem<'a> {
    type Target = Elem<'a>;

    /// returns the elem of this selem
    fn deref(&self) -> &Elem<'a> {
        &self.0
    }
}

pub struct IterEnum<'a>(&'a Selem<'a>, u32, u32);

impl<'a> Iterator for IterEnum<'a> {
    type Item = Result<String>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 { None }
        else { self.1 += 1; Some(self.0.get_enum_item_name(self.1-1)) }
    }
}

alsa_enum!(
    /// Wrapper for [SND_MIXER_SCHN_*](http://www.alsa-project.org/alsa-doc/alsa-lib/group___simple_mixer.html) constants
    SelemChannelId, ALL_SELEM_CHANNEL_ID[11],

    Unknown     = SND_MIXER_SCHN_UNKNOWN,
    FrontLeft   = SND_MIXER_SCHN_FRONT_LEFT,
    FrontRight  = SND_MIXER_SCHN_FRONT_RIGHT,
    RearLeft    = SND_MIXER_SCHN_REAR_LEFT,
    RearRight   = SND_MIXER_SCHN_REAR_RIGHT,
    FrontCenter = SND_MIXER_SCHN_FRONT_CENTER,
    Woofer      = SND_MIXER_SCHN_WOOFER,
    SideLeft    = SND_MIXER_SCHN_SIDE_LEFT,
    SideRight   = SND_MIXER_SCHN_SIDE_RIGHT,
    RearCenter  = SND_MIXER_SCHN_REAR_CENTER,
    Last        = SND_MIXER_SCHN_LAST,
);

impl SelemChannelId {
    pub fn mono() -> SelemChannelId { SelemChannelId::FrontLeft }
}

impl fmt::Display for SelemChannelId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Selem::channel_name(*self).unwrap())
    }
}

#[test]
fn print_mixer_of_cards() {
    use super::card;

    for card in card::Iter::new().map(|c| c.unwrap()) {
        println!("Card #{}: {} ({})", card.get_index(), card.get_name().unwrap(), card.get_longname().unwrap());

        let mixer = Mixer::new(&format!("hw:{}", card.get_index()), false).unwrap();
        for selem in mixer.iter().filter_map(|e| Selem::new(e)) {

            let sid = selem.get_id();
            println!("\tMixer element {},{}:", sid.get_name().unwrap(), sid.get_index());

            if selem.has_volume() {
                print!("\t  Volume limits: ");
                if selem.has_capture_volume() {
                    let (vmin, vmax) = selem.get_capture_volume_range();
                    let (mbmin, mbmax) = selem.get_capture_db_range();
                    print!("Capture = {} - {}", vmin, vmax);
                    print!(" ({} dB - {} dB)", mbmin.to_db(), mbmax.to_db());
                }
                if selem.has_playback_volume() {
                    let (vmin, vmax) = selem.get_playback_volume_range();
                    let (mbmin, mbmax) = selem.get_playback_db_range();
                    print!("Playback = {} - {}", vmin, vmax);
                    print!(" ({} dB - {} dB)", mbmin.to_db(), mbmax.to_db());
                }
                println!();
            }

            if selem.is_enumerated() {
                print!("\t  Valid values: ");
                for v in selem.iter_enum().unwrap() { print!("{}, ", v.unwrap()) };
                print!("\n\t  Current values: ");
                for v in SelemChannelId::all().iter().filter_map(|&v| selem.get_enum_item(v).ok()) {
                    print!("{}, ", selem.get_enum_item_name(v).unwrap());
                }
                println!();
            }

            if selem.can_capture() {
                print!("\t  Capture channels: ");
                if selem.is_capture_mono() {
                    print!("Mono");
                } else {
                    for channel in SelemChannelId::all() {
                        if selem.has_capture_channel(*channel) { print!("{}, ", channel) };
                    }
                }
                println!();
                print!("\t  Capture volumes: ");
                for channel in SelemChannelId::all() {
                    if selem.has_capture_channel(*channel) { print!("{}: {} ({} dB), ", channel,
                        match selem.get_capture_volume(*channel) {Ok(v) => format!("{}", v), Err(_) => "n/a".to_string()},
                        match selem.get_capture_vol_db(*channel) {Ok(v) => format!("{}", v.to_db()), Err(_) => "n/a".to_string()}
                    );}
                }
                println!();
            }

            if selem.can_playback() {
                print!("\t  Playback channels: ");
                if selem.is_playback_mono() {
                    print!("Mono");
                } else {
                    for channel in SelemChannelId::all() {
                        if selem.has_playback_channel(*channel) { print!("{}, ", channel) };
                    }
                }
                println!();
                if selem.has_playback_volume() {
                    print!("\t  Playback volumes: ");
                    for channel in SelemChannelId::all() {
                        if selem.has_playback_channel(*channel) { print!("{}: {} / {}dB, ",
                            channel,
                            match selem.get_playback_volume(*channel) {Ok(v) => format!("{}", v), Err(_) => "n/a".to_string()},
                            match selem.get_playback_vol_db(*channel) {Ok(v) => format!("{}", v.to_db()), Err(_) => "n/a".to_string()}
                        );}
                    }
                    println!();
                }
            }
        }
    }
}

#[test]
#[ignore]
fn get_and_set_playback_volume() {
    let mixer = Mixer::new("hw:1", false).unwrap();
    let selem = mixer.find_selem(&SelemId::new("Master", 0)).unwrap();

    let (rmin, rmax) = selem.get_playback_volume_range();
    let mut channel = SelemChannelId::mono();
    for c in SelemChannelId::all().iter() {
        if selem.has_playback_channel(*c) { channel = *c; break }
    }
    println!("Testing on {} with limits {}-{} on channel {}", selem.get_id().get_name().unwrap(), rmin, rmax, channel);

    let old: i64 = selem.get_playback_volume(channel).unwrap();
    let new: i64 = rmax / 2;
    assert_ne!(new, old);

    println!("Changing volume of {} from {} to {}", channel, old, new);
    selem.set_playback_volume(channel, new).unwrap();
    let mut result: i64 = selem.get_playback_volume(channel).unwrap();
    assert_eq!(new, result);

    // return volume to old value
    selem.set_playback_volume(channel, old).unwrap();
    result = selem.get_playback_volume(channel).unwrap();
    assert_eq!(old, result);
}

#[test]
#[ignore]
fn get_and_set_capture_volume() {
    let mixer = Mixer::new("hw:1", false).unwrap();
    let selem = mixer.find_selem(&SelemId::new("Capture", 0)).unwrap();

    let (rmin, rmax) = selem.get_capture_volume_range();
    let mut channel = SelemChannelId::mono();
    for c in SelemChannelId::all().iter() {
        if selem.has_playback_channel(*c) { channel = *c; break }
    }
    println!("Testing on {} with limits {}-{} on channel {}", selem.get_id().get_name().unwrap(), rmin, rmax, channel);

    let old: i64 = selem.get_capture_volume(channel).unwrap();
    let new: i64 = rmax / 2;
    assert_ne!(new, old);

    println!("Changing volume of {} from {} to {}", channel, old, new);
    selem.set_capture_volume(channel, new).unwrap();
    let mut result: i64 = selem.get_capture_volume(channel).unwrap();
    assert_eq!(new, result);

    // return volume to old value
    selem.set_capture_volume(channel, old).unwrap();
    result = selem.get_capture_volume(channel).unwrap();
    assert_eq!(old, result);
}


#[test]
fn print_sizeof() {
    let selemid = unsafe { alsa::snd_mixer_selem_id_sizeof() } as usize;

    assert!(selemid <= SELEM_ID_SIZE);
    println!("Selem id: {}", selemid);
}
