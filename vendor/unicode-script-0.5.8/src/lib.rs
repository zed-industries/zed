//! This crate exposes the Unicode `Script` and `Script_Extension`
//! properties from [UAX #24](http://www.unicode.org/reports/tr24/)

#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "bench", feature(test))]

mod tables;

use core::convert::{TryFrom, TryInto};
use core::fmt;
use core::u64;
pub use tables::script_extensions;
use tables::{get_script, get_script_extension, NEXT_SCRIPT};
pub use tables::{Script, UNICODE_VERSION};

impl Script {
    /// Get the full name of a script.
    pub fn full_name(self) -> &'static str {
        self.inner_full_name()
    }

    /// Attempts to parse script name from the provided string.
    /// Returns `None` if the provided string does not represent a valid
    /// script full name.
    pub fn from_full_name(input: &str) -> Option<Self> {
        Self::inner_from_full_name(input)
    }

    /// Get the four-character short name of a script.
    pub fn short_name(self) -> &'static str {
        self.inner_short_name()
    }

    /// Attempts to parse script name from the provided string.
    /// Returns `None` if the provided string does not represent a valid
    /// script four-character short name.
    pub fn from_short_name(input: &str) -> Option<Self> {
        Self::inner_from_short_name(input)
    }

    /// The 4-byte iso15924 tag as a big-endian `u32`
    pub fn as_iso15924_tag(self) -> u32 {
        let arr: [u8; 4] = self.inner_short_name().as_bytes().try_into().unwrap();
        u32::from_be_bytes(arr)
    }

    /// Is this script "Recommended" according to
    /// [UAX #31](www.unicode.org/reports/tr31/#Table_Recommended_Scripts)?
    pub fn is_recommended(self) -> bool {
        use Script::*;
        match self {
            Common | Inherited | Arabic | Armenian | Bengali | Bopomofo | Cyrillic | Devanagari
            | Ethiopic | Georgian | Greek | Gujarati | Gurmukhi | Han | Hangul | Hebrew
            | Hiragana | Kannada | Katakana | Khmer | Lao | Latin | Malayalam | Myanmar | Oriya
            | Sinhala | Tamil | Telugu | Thaana | Thai | Tibetan => true,
            _ => false,
        }
    }
}

impl From<Script> for ScriptExtension {
    fn from(script: Script) -> Self {
        if script == Script::Common {
            ScriptExtension::new_common()
        } else if script == Script::Inherited {
            ScriptExtension::new_inherited()
        } else if script == Script::Unknown {
            ScriptExtension::new_unknown()
        } else {
            let mut first = 0;
            let mut second = 0;
            let mut third = 0;
            let bit = script as u8;
            // Find out which field it's in, and set the appropriate bit there
            if bit < 64 {
                first = 1 << bit as u64;
            } else if bit < 128 {
                // offset by 64 since `bit` is an absolute number,
                // not relative to the chunk
                second = 1 << (bit - 64) as u64;
            } else {
                third = 1 << (bit - 128) as u32;
            }
            ScriptExtension::new(first, second, third)
        }
    }
}

impl TryFrom<ScriptExtension> for Script {
    type Error = ();
    fn try_from(ext: ScriptExtension) -> Result<Self, ()> {
        if ext.is_common_or_inherited() {
            if ext.common {
                Ok(Script::Common)
            } else {
                Ok(Script::Inherited)
            }
        } else if ext.is_empty() {
            Ok(Script::Unknown)
        } else {
            // filled elements will have set ones
            let fo = ext.first.count_ones();
            let so = ext.second.count_ones();
            let to = ext.third.count_ones();
            // only one bit set, in the first chunk
            if fo == 1 && so == 0 && to == 0 {
                // use trailing_zeroes() to figure out which bit it is
                Ok(Script::for_integer(ext.first.trailing_zeros() as u8))
            // only one bit set, in the second chunk
            } else if fo == 0 && so == 1 && to == 0 {
                Ok(Script::for_integer(64 + ext.second.trailing_zeros() as u8))
            // only one bit set, in the third chunk
            } else if fo == 0 && so == 0 && to == 1 {
                Ok(Script::for_integer(128 + ext.third.trailing_zeros() as u8))
            } else {
                Err(())
            }
        }
    }
}

impl Default for Script {
    fn default() -> Self {
        Script::Common
    }
}

impl From<char> for Script {
    fn from(o: char) -> Self {
        o.script()
    }
}

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// A value for the `Script_Extension` property
///
/// [`ScriptExtension`] is one or more [`Script`]
///
/// This is essentially an optimized version of `Vec<Script>` that uses bitfields
pub struct ScriptExtension {
    // A bitset for the first 64 scripts
    first: u64,
    // A bitset for the scripts 65-128
    second: u64,
    // A bitset for scripts after 128
    third: u64,
    // Both Common and Inherited are represented by all used bits being set,
    // this flag lets us distinguish the two.
    common: bool,
}

impl ScriptExtension {
    // We don't use the complete u64 of `third`, so the "all" value is not just u32::MAX
    // Instead, we take the number of the next (unused) script bit, subtract 128 to bring
    // it in the range of `third`, create a u64 with just that bit set, and subtract 1
    // to create one with all the lower bits set.
    const THIRD_MAX: u64 = ((1 << (NEXT_SCRIPT - 128)) - 1);

    pub(crate) const fn new(first: u64, second: u64, third: u64) -> Self {
        ScriptExtension {
            first,
            second,
            third,
            common: false,
        }
    }

    pub(crate) const fn new_common() -> Self {
        ScriptExtension {
            first: u64::MAX,
            second: u64::MAX,
            third: Self::THIRD_MAX,
            common: true,
        }
    }

    pub(crate) const fn new_inherited() -> Self {
        ScriptExtension {
            first: u64::MAX,
            second: u64::MAX,
            third: Self::THIRD_MAX,
            common: false,
        }
    }

    pub(crate) const fn new_unknown() -> Self {
        ScriptExtension {
            first: 0,
            second: 0,
            third: 0,
            common: false,
        }
    }

    const fn is_common_or_inherited(self) -> bool {
        (self.first == u64::MAX) & (self.second == u64::MAX) & (self.third == Self::THIRD_MAX)
    }

    /// Checks if the script extension is Common
    pub const fn is_common(self) -> bool {
        self.is_common_or_inherited() & self.common
    }

    /// Checks if the script extension is Inherited
    pub const fn is_inherited(self) -> bool {
        self.is_common_or_inherited() & !self.common
    }

    /// Checks if the script extension is empty (unknown)
    pub const fn is_empty(self) -> bool {
        (self.first == 0) & (self.second == 0) & (self.third == 0)
    }

    /// Returns the number of scripts in the script extension
    pub fn len(self) -> usize {
        if self.is_common_or_inherited() {
            1
        } else {
            (self.first.count_ones() + self.second.count_ones() + self.third.count_ones()) as usize
        }
    }

    /// Intersect this `ScriptExtension` with another `ScriptExtension`. Produces `Unknown` if things
    /// do not intersect. This is equivalent to [`ScriptExtension::intersection`] but it stores the result
    /// in `self`
    ///
    /// "Common" (`Zyyy`) and "Inherited" (`Zinh`) are considered as intersecting
    /// everything, the intersection of `Common` and `Inherited` is `Inherited`
    pub fn intersect_with(&mut self, other: Self) {
        *self = self.intersection(other)
    }

    /// Find the intersection between two ScriptExtensions. Returns Unknown if things
    /// do not intersect.
    ///
    /// "Common" (`Zyyy`) and "Inherited" (`Zinh`) are considered as intersecting
    /// everything, the intersection of `Common` and `Inherited` is `Inherited`
    pub const fn intersection(self, other: Self) -> Self {
        let first = self.first & other.first;
        let second = self.second & other.second;
        let third = self.third & other.third;
        let common = self.common & other.common;
        ScriptExtension {
            first,
            second,
            third,
            common,
        }
    }

    /// Find the union between two ScriptExtensions.
    ///
    /// "Common" (`Zyyy`) and "Inherited" (`Zinh`) are considered as intersecting
    /// everything, the union of `Common` and `Inherited` is `Common`
    pub const fn union(self, other: Self) -> Self {
        let first = self.first | other.first;
        let second = self.second | other.second;
        let third = self.third | other.third;
        let common = self.common | other.common;
        ScriptExtension {
            first,
            second,
            third,
            common,
        }
    }

    /// Check if this ScriptExtension contains the given script
    ///
    /// Should be used with specific scripts only, this will
    /// return `true` if `self` is not `Unknown` and `script` is
    /// `Common` or `Inherited`
    pub fn contains_script(self, script: Script) -> bool {
        !self.intersection(script.into()).is_empty()
    }

    /// Get the intersection of script extensions of all characters
    /// in a string.
    pub fn for_str(x: &str) -> Self {
        let mut ext = ScriptExtension::default();
        for ch in x.chars() {
            ext.intersect_with(ch.into());
        }
        ext
    }

    /// Iterate over the scripts in this script extension
    ///
    /// Will never yield Script::Unknown
    pub fn iter(self) -> ScriptIterator {
        ScriptIterator { ext: self }
    }
}

impl Default for ScriptExtension {
    fn default() -> Self {
        ScriptExtension::new_common()
    }
}

impl From<char> for ScriptExtension {
    fn from(o: char) -> Self {
        o.script_extension()
    }
}

impl From<&'_ str> for ScriptExtension {
    fn from(o: &'_ str) -> Self {
        Self::for_str(o)
    }
}

impl fmt::Debug for ScriptExtension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScriptExtension(")?;
        fmt::Display::fmt(self, f)?;
        write!(f, ")")
    }
}

impl fmt::Display for ScriptExtension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_common() {
            write!(f, "Common")?;
        } else if self.is_inherited() {
            write!(f, "Inherited")?;
        } else if self.is_empty() {
            write!(f, "Unknown")?;
        } else {
            let mut first = true;
            for script in self.iter() {
                if first {
                    first = false;
                } else {
                    write!(f, " + ")?;
                }
                script.full_name().fmt(f)?;
            }
        }
        Ok(())
    }
}

/// Extension trait on `char` for calculating script properties
pub trait UnicodeScript {
    /// Get the script for a given character
    fn script(&self) -> Script;
    /// Get the Script_Extension for a given character
    fn script_extension(&self) -> ScriptExtension;
}

impl UnicodeScript for char {
    fn script(&self) -> Script {
        get_script(*self).unwrap_or(Script::Unknown)
    }

    fn script_extension(&self) -> ScriptExtension {
        get_script_extension(*self).unwrap_or_else(|| self.script().into())
    }
}

/// Iterator over scripts in a [ScriptExtension].
///
/// Can be obtained via [ScriptExtension::iter()]
pub struct ScriptIterator {
    ext: ScriptExtension,
}

impl Iterator for ScriptIterator {
    type Item = Script;

    fn next(&mut self) -> Option<Script> {
        if self.ext.is_common_or_inherited() {
            let common = self.ext.common;
            self.ext = ScriptExtension::new_unknown();
            if common {
                Some(Script::Common)
            } else {
                Some(Script::Inherited)
            }
        // Are there bits left in the first chunk?
        } else if self.ext.first != 0 {
            // Find the next bit
            let bit = self.ext.first.trailing_zeros();
            // unset just that bit
            self.ext.first &= !(1 << bit);
            Some(Script::for_integer(bit as u8))
        // Are there bits left in the second chunk?
        } else if self.ext.second != 0 {
            let bit = self.ext.second.trailing_zeros();
            self.ext.second &= !(1 << bit);
            Some(Script::for_integer(64 + bit as u8))
        // Are there bits left in the third chunk?
        } else if self.ext.third != 0 {
            let bit = self.ext.third.trailing_zeros();
            self.ext.third &= !(1 << bit);
            Some(Script::for_integer(128 + bit as u8))
        } else {
            // Script::Unknown
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::HashSet;
    use std::convert::TryInto;

    #[cfg(feature = "bench")]
    use test::bench::Bencher;
    #[cfg(feature = "bench")]
    extern crate test;

    #[test]
    fn test_conversion() {
        let mut seen_scripts = HashSet::new();
        let mut seen_exts = HashSet::new();
        for bit in 0..NEXT_SCRIPT {
            let script = Script::for_integer(bit);
            let ext = script.into();
            if seen_scripts.contains(&script) {
                panic!("Found script {:?} twice!", script)
            }
            if seen_exts.contains(&ext) {
                panic!("Found extension {:?} twice!", ext)
            }
            seen_scripts.insert(script);
            seen_exts.insert(ext);
            assert_eq!(script as u8, bit);
            assert!(!ScriptExtension::new_common().intersection(ext).is_empty());
            assert!(!ScriptExtension::new_inherited()
                .intersection(ext)
                .is_empty());
            assert!(ScriptExtension::new_unknown().intersection(ext).is_empty());
            assert_eq!(ext.iter().collect::<Vec<_>>(), vec![script]);
            assert_eq!(Ok(script), ext.try_into());
        }
    }

    #[test]
    fn test_specific() {
        let s = "सवव मानवी व्यद्क् जन्मतःच स्वतींत्र आहेत व त्ाींना समान प्रवतष्ठा व समान अविकार आहेत. त्ाींना ववचारशद्क् व सवविे कबुद्द्धलाभलेली आहे. व त्ाींनी एकमेकाींशी बींिुत्वाचाभावनेने आचरण करावे.";
        let ext = ScriptExtension::for_str(s);
        assert_eq!(ext, script_extensions::DEVA);
        println!(
            "{:?}",
            script_extensions::DEVA_DOGR_GUJR_GURU_KHOJ_KTHI_MAHJ_MODI_SIND_TAKR_TIRH
        );
        println!(
            "{:?}",
            ext.intersection(
                script_extensions::DEVA_DOGR_GUJR_GURU_KHOJ_KTHI_MAHJ_MODI_SIND_TAKR_TIRH
            )
        );
        assert!(!ext
            .intersection(script_extensions::DEVA_DOGR_GUJR_GURU_KHOJ_KTHI_MAHJ_MODI_SIND_TAKR_TIRH)
            .is_empty());

        let u = ext.union(Script::Dogra.into());
        assert_eq!(
            u.intersection(
                script_extensions::DEVA_DOGR_GUJR_GURU_KHOJ_KTHI_MAHJ_MODI_SIND_TAKR_TIRH
            ),
            u
        );
    }

    #[test]
    fn test_specific_ext() {
        let ext = script_extensions::DEVA_DOGR_GUJR_GURU_KHOJ_KTHI_MAHJ_MODI_SIND_TAKR_TIRH;

        let all: HashSet<_> = ext.iter().collect();

        for bit in 0..NEXT_SCRIPT {
            let script = Script::for_integer(bit);

            if all.contains(&script) {
                assert!(ext.contains_script(script))
            } else {
                assert!(!ext.contains_script(script))
            }
        }

        assert!(ext.contains_script(Script::Devanagari));
        assert!(ext.contains_script(Script::Dogra));
        assert!(ext.contains_script(Script::Gujarati));
        assert!(ext.contains_script(Script::Gurmukhi));
        assert!(ext.contains_script(Script::Khojki));
        assert!(ext.contains_script(Script::Kaithi));
        assert!(ext.contains_script(Script::Mahajani));
        assert!(ext.contains_script(Script::Modi));
        assert!(ext.contains_script(Script::Khudawadi));
        assert!(ext.contains_script(Script::Takri));
        assert!(ext.contains_script(Script::Tirhuta));

        let scr: Result<Script, _> = ext.try_into();
        assert!(scr.is_err());
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn bench_script_intersection(b: &mut Bencher) {
        b.iter(|| {
            let script = test::black_box(Script::Devanagari);
            let ext = test::black_box(script_extensions::BENG_DEVA_DOGR_GONG_GONM_GRAN_GUJR_GURU_KNDA_MAHJ_MLYM_NAND_ONAO_ORYA_SIND_SINH_SYLO_TAKR_TAML_TELU_TIRH);
            test::black_box(ext.intersection(script.into()));
        })
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn bench_ext_to_script(b: &mut Bencher) {
        let ext: ScriptExtension = Script::Devanagari.into();
        b.iter(|| {
            let ext = test::black_box(ext);
            let script: Result<Script, _> = ext.try_into();
            let _ = test::black_box(script);
        })
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn bench_script_to_ext(b: &mut Bencher) {
        b.iter(|| {
            let script = test::black_box(Script::Devanagari);
            let ext: ScriptExtension = script.into();
            test::black_box(ext);
        })
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn bench_ext_intersection(b: &mut Bencher) {
        b.iter(|| {
            let e1 = test::black_box(script_extensions::ARAB_GARA_NKOO_ROHG_SYRC_THAA_YEZI);
            let e2 = test::black_box(script_extensions::BENG_DEVA_DOGR_GONG_GONM_GRAN_GUJR_GURU_KNDA_MAHJ_MLYM_NAND_ONAO_ORYA_SIND_SINH_SYLO_TAKR_TAML_TELU_TIRH);
            test::black_box(e2.intersection(e1));
        })
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn bench_to_vec(b: &mut Bencher) {
        b.iter(|| {
            let ext = test::black_box(script_extensions::BENG_DEVA_DOGR_GONG_GONM_GRAN_GUJR_GURU_KNDA_MAHJ_MLYM_NAND_ONAO_ORYA_SIND_SINH_SYLO_TAKR_TAML_TELU_TIRH);
            test::black_box(ext.iter().collect::<Vec<_>>());
        })
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn bench_string_ext(b: &mut Bencher) {
        b.iter(|| {
            let s = test::black_box("सवव मानवी व्यद्क् जन्मतःच स्वतींत्र आहेत व त्ाींना समान प्रवतष्ठा व समान अविकार आहेत. त्ाींना ववचारशद्क् व सवविे कबुद्द्धलाभलेली आहे. व त्ाींनी एकमेकाींशी बींिुत्वाचाभावनेने आचरण करावे.");
            test::black_box(ScriptExtension::for_str(s));
        })
    }
}
