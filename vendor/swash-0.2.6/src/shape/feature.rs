//! Feature constants.

use super::internal::{raw_tag, RawTag};

// Default tag used in various places.
pub const _DFLT: RawTag = raw_tag(b"DFLT");

// Substitution features.
pub const CCMP: RawTag = raw_tag(b"ccmp");
pub const LOCL: RawTag = raw_tag(b"locl");
pub const RVRN: RawTag = raw_tag(b"rvrn");
pub const LIGA: RawTag = raw_tag(b"liga");
pub const CLIG: RawTag = raw_tag(b"clig");
pub const RLIG: RawTag = raw_tag(b"rlig");
pub const _DLIG: RawTag = raw_tag(b"dlig");
pub const CALT: RawTag = raw_tag(b"calt");
pub const LJMO: RawTag = raw_tag(b"ljmo");
pub const VJMO: RawTag = raw_tag(b"vjmo");
pub const TJMO: RawTag = raw_tag(b"tjmo");
pub const NUKT: RawTag = raw_tag(b"nukt");
pub const AKHN: RawTag = raw_tag(b"akhn");
pub const RKRF: RawTag = raw_tag(b"rkrf");
pub const HALF: RawTag = raw_tag(b"half");
pub const HALN: RawTag = raw_tag(b"haln");
pub const VATU: RawTag = raw_tag(b"vatu");
pub const CJCT: RawTag = raw_tag(b"cjct");
pub const ISOL: RawTag = raw_tag(b"isol");
pub const INIT: RawTag = raw_tag(b"init");
pub const MEDI: RawTag = raw_tag(b"medi");
pub const MED2: RawTag = raw_tag(b"med2");
pub const FINA: RawTag = raw_tag(b"fina");
pub const FIN2: RawTag = raw_tag(b"fin2");
pub const FIN3: RawTag = raw_tag(b"fin3");
pub const MSET: RawTag = raw_tag(b"mset");
pub const RPHF: RawTag = raw_tag(b"rphf");
pub const PREF: RawTag = raw_tag(b"pref");
pub const ABVF: RawTag = raw_tag(b"abvf");
pub const BLWF: RawTag = raw_tag(b"blwf");
pub const PSTF: RawTag = raw_tag(b"pstf");
pub const PRES: RawTag = raw_tag(b"pres");
pub const ABVS: RawTag = raw_tag(b"abvs");
pub const BLWS: RawTag = raw_tag(b"blws");
pub const PSTS: RawTag = raw_tag(b"psts");
pub const RCLT: RawTag = raw_tag(b"rclt");
pub const VERT: RawTag = raw_tag(b"vert");
pub const VRT2: RawTag = raw_tag(b"vrt2");
pub const RTLM: RawTag = raw_tag(b"rtlm");

// Positioning features.
pub const KERN: RawTag = raw_tag(b"kern");
pub const DIST: RawTag = raw_tag(b"dist");
pub const ABVM: RawTag = raw_tag(b"abvm");
pub const BLWM: RawTag = raw_tag(b"blwm");
pub const CURS: RawTag = raw_tag(b"curs");
pub const MARK: RawTag = raw_tag(b"mark");
pub const MKMK: RawTag = raw_tag(b"mkmk");

// Arabic joining masks.
pub const ISOL_MASK: u8 = 1;
pub const INIT_MASK: u8 = 2;
pub const MEDI_MASK: u8 = 4;
pub const FINA_MASK: u8 = 8;
pub const MED2_MASK: u8 = 16;
pub const FIN2_MASK: u8 = 32;
pub const FIN3_MASK: u8 = 64;
pub const NONE_MASK: u8 = 0;

// Hangul jamo masks.
pub const LJMO_MASK: u8 = 1;
pub const VJMO_MASK: u8 = 2;
pub const TJMO_MASK: u8 = 4;
