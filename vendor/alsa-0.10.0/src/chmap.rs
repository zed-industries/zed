use crate::alsa;
use std::{fmt, mem, slice};
use super::error::*;

alsa_enum!(
    /// [SND_CHMAP_TYPE_xxx](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m.html) constants
    ChmapType, ALL_CHMAP_TYPES[4],

    None = SND_CHMAP_TYPE_NONE,
    Fixed = SND_CHMAP_TYPE_FIXED,
    Var = SND_CHMAP_TYPE_VAR,
    Paired = SND_CHMAP_TYPE_PAIRED,
);

alsa_enum!(
    /// [SND_CHMAP_xxx](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m.html) constants
    ChmapPosition, ALL_CHMAP_POSITIONS[37],

    Unknown = SND_CHMAP_UNKNOWN,
    NA = SND_CHMAP_NA,
    Mono = SND_CHMAP_MONO,
    FL = SND_CHMAP_FL,
    FR = SND_CHMAP_FR,
    RL = SND_CHMAP_RL,
    RR = SND_CHMAP_RR,
    FC = SND_CHMAP_FC,
    LFE = SND_CHMAP_LFE,
    SL = SND_CHMAP_SL,
    SR = SND_CHMAP_SR,
    RC = SND_CHMAP_RC,
    FLC = SND_CHMAP_FLC,
    FRC = SND_CHMAP_FRC,
    RLC = SND_CHMAP_RLC,
    RRC = SND_CHMAP_RRC,
    FLW = SND_CHMAP_FLW,
    FRW = SND_CHMAP_FRW,
    FLH = SND_CHMAP_FLH,
    FCH = SND_CHMAP_FCH,
    FRH = SND_CHMAP_FRH,
    TC = SND_CHMAP_TC,
    TFL = SND_CHMAP_TFL,
    TFR = SND_CHMAP_TFR,
    TFC = SND_CHMAP_TFC,
    TRL = SND_CHMAP_TRL,
    TRR = SND_CHMAP_TRR,
    TRC = SND_CHMAP_TRC,
    TFLC = SND_CHMAP_TFLC,
    TFRC = SND_CHMAP_TFRC,
    TSL = SND_CHMAP_TSL,
    TSR = SND_CHMAP_TSR,
    LLFE = SND_CHMAP_LLFE,
    RLFE = SND_CHMAP_RLFE,
    BC = SND_CHMAP_BC,
    BLC = SND_CHMAP_BLC,
    BRC = SND_CHMAP_BRC,
);

impl fmt::Display for ChmapPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe { alsa::snd_pcm_chmap_long_name(*self as libc::c_uint) };
        let s = from_const("snd_pcm_chmap_long_name", s)?;
        write!(f, "{}", s)
    }
}


/// [snd_pcm_chmap_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m.html) wrapper
pub struct Chmap(*mut alsa::snd_pcm_chmap_t, bool);

impl Drop for Chmap {
    fn drop(&mut self) { if self.1 { unsafe { libc::free(self.0 as *mut libc::c_void) }}}
}

impl Chmap {
    fn set_channels(&mut self, c: libc::c_uint) { unsafe { (*self.0) .channels = c }}
    fn as_slice_mut(&mut self) -> &mut [libc::c_uint] {
        unsafe { slice::from_raw_parts_mut((*self.0).pos.as_mut_ptr(), (*self.0).channels as usize) }
    }
    fn as_slice(&self) -> &[libc::c_uint] {
        unsafe { slice::from_raw_parts((*self.0).pos.as_ptr(), (*self.0).channels as usize) }
    }
}

impl fmt::Display for Chmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf: Vec<libc::c_char> = vec![0; 512];
        acheck!(snd_pcm_chmap_print(self.0, buf.len() as libc::size_t, buf.as_mut_ptr()))?;
        let s = from_const("snd_pcm_chmap_print", buf.as_mut_ptr())?;
        write!(f, "{}", s)
    }
}

impl<'a> From<&'a [ChmapPosition]> for Chmap {
    fn from(a: &'a [ChmapPosition]) -> Chmap {
        let p = unsafe { libc::malloc((mem::size_of::<alsa::snd_pcm_chmap_t>() + mem::size_of::<libc::c_uint>() * a.len()) as libc::size_t) };
        if p.is_null() { panic!("Out of memory") }
        let mut r = Chmap(p as *mut alsa::snd_pcm_chmap_t, true);
        r.set_channels(a.len() as libc::c_uint);
        for (i,v) in r.as_slice_mut().iter_mut().enumerate() { *v = a[i] as libc::c_uint }
        r
    }
}

impl<'a> From<&'a Chmap> for Vec<ChmapPosition> {
    fn from(a: &'a Chmap) -> Vec<ChmapPosition> {
        a.as_slice().iter().map(|&v| ChmapPosition::from_c_int(v as libc::c_int, "").unwrap()).collect()
    }
}

pub fn chmap_new(a: *mut alsa::snd_pcm_chmap_t) -> Chmap { Chmap(a, true) }
pub fn chmap_handle(a: &Chmap) -> *mut alsa::snd_pcm_chmap_t { a.0 }


/// Iterator over available channel maps - see [snd_pcm_chmap_query_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m.html)
pub struct ChmapsQuery(*mut *mut alsa::snd_pcm_chmap_query_t, isize);

impl Drop for ChmapsQuery {
    fn drop(&mut self) { unsafe { alsa::snd_pcm_free_chmaps(self.0) }}
}

pub fn chmaps_query_new(a: *mut *mut alsa::snd_pcm_chmap_query_t) -> ChmapsQuery { ChmapsQuery(a, 0) }

impl Iterator for ChmapsQuery {
    type Item = (ChmapType, Chmap);
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_null() { return None; }
        let p = unsafe { *self.0.offset(self.1) };
        if p.is_null() { return None; }
        self.1 += 1;
        let t = ChmapType::from_c_int(unsafe { (*p).type_ } as libc::c_int, "snd_pcm_query_chmaps").unwrap();
        let m = Chmap(unsafe { &mut (*p).map }, false);
        Some((t, m))
    }
}


#[test]
fn chmap_for_first_pcm() {
    use super::*;
    use std::ffi::CString;
    use crate::device_name::HintIter;
    let i = HintIter::new(None, &*CString::new("pcm").unwrap()).unwrap();
    for p in i.map(|n| n.name.unwrap()) {
        println!("Chmaps for {:?}:", p);
        match PCM::open(&CString::new(p).unwrap(),  Direction::Playback, false) {
            Ok(a) => for c in a.query_chmaps() {
               println!("  {:?}, {}", c.0, c.1);
            },
            Err(a) => println!("  {}", a) // It's okay to have entries in the name hint array that can't be opened
        }
    }
}
