//! Symbol versioning
//!
//! Implementation of the GNU symbol versioning extension according to
//! [LSB Core Specification - Symbol Versioning][lsb-symver].
//!
//! # Examples
//!
//! List the dependencies of an ELF file that have [version needed][lsb-verneed] information along
//! with the versions needed for each dependency.
//! ```rust
//! use goblin::error::Error;
//!
//! pub fn show_verneed(bytes: &[u8]) -> Result<(), Error> {
//!     let binary = goblin::elf::Elf::parse(&bytes)?;
//!
//!     if let Some(verneed) = binary.verneed {
//!         for need_file in verneed.iter() {
//!             println!(
//!                 "Depend on {:?} with version(s):",
//!                 binary.dynstrtab.get_at(need_file.vn_file)
//!             );
//!             for need_ver in need_file.iter() {
//!                 println!("{:?}", binary.dynstrtab.get_at(need_ver.vna_name));
//!             }
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! List the [version defined][lsb-verdef] information of an ELF file, effectively listing the version
//! defined by this ELF file.
//! ```rust
//! use goblin::error::Error;
//!
//! pub fn show_verdef(bytes: &[u8]) -> Result<(), Error> {
//!     let binary = goblin::elf::Elf::parse(&bytes)?;
//!
//!     if let Some(verdef) = &binary.verdef {
//!         for def in verdef.iter() {
//!             for (n, aux) in def.iter().enumerate() {
//!                 let name = binary.dynstrtab.get_at(aux.vda_name);
//!                 match n {
//!                     0 => print!("Name: {:?}", name),
//!                     1 => print!(" Parent: {:?}", name),
//!                     _ => print!(", {:?}", name),
//!                 }
//!             }
//!             print!("\n");
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! [lsb-symver]: https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/symversion.html
//! [lsb-verneed]: https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/symversion.html#SYMVERRQMTS
//! [lsb-verdef]: https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/symversion.html#SYMVERDEFS

use crate::container;
use crate::elf::section_header::{SectionHeader, SHT_GNU_VERDEF, SHT_GNU_VERNEED, SHT_GNU_VERSYM};
use crate::error::Result;
use core::iter::FusedIterator;
use scroll::Pread;

/******************
 *  ELF Constants *
 ******************/

// Versym constants.

/// Constant describing a local symbol, see [`Versym::is_local`].
pub const VER_NDX_LOCAL: u16 = 0;
/// Constant describing a global symbol, see [`Versym::is_global`].
pub const VER_NDX_GLOBAL: u16 = 1;
/// Bitmask to check hidden bit, see [`Versym::is_hidden`].
pub const VERSYM_HIDDEN: u16 = 0x8000;
/// Bitmask to get version information, see [`Versym::version`].
pub const VERSYM_VERSION: u16 = 0x7fff;

// Verdef constants.

/// Bitmask to check `base` flag in [`Verdef::vd_flags`].
pub const VER_FLG_BASE: u16 = 0x1;
/// Bitmask to check `weak` flag in [`Verdef::vd_flags`].
pub const VER_FLG_WEAK: u16 = 0x2;
/// Bitmask to check `info` flag in [`Verdef::vd_flags`].
pub const VER_FLG_INFO: u16 = 0x4;

/********************
 *  ELF Structures  *
 ********************/

/// An ELF `Symbol Version` entry.
///
/// https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/symversion.html#SYMVERTBL
#[repr(C)]
#[derive(Debug, Pread)]
struct ElfVersym {
    vs_val: u16,
}

/// An ELF `Version Definition` entry Elfxx_Verdef.
///
/// https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/symversion.html#VERDEFENTRIES
#[repr(C)]
#[derive(Debug, Pread)]
struct ElfVerdef {
    vd_version: u16,
    vd_flags: u16,
    vd_ndx: u16,
    vd_cnt: u16,
    vd_hash: u32,
    vd_aux: u32,
    vd_next: u32,
}

/// An ELF `Version Definition Auxiliary` entry Elfxx_Verdaux.
///
/// https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/symversion.html#VERDEFEXTS
#[repr(C)]
#[derive(Debug, Pread)]
struct ElfVerdaux {
    vda_name: u32,
    vda_next: u32,
}

/// An ELF `Version Need` entry Elfxx_Verneed.
///
/// https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/symversion.html#VERNEEDFIG
#[repr(C)]
#[derive(Debug, Pread)]
struct ElfVerneed {
    vn_version: u16,
    vn_cnt: u16,
    vn_file: u32,
    vn_aux: u32,
    vn_next: u32,
}

/// An ELF `Version Need Auxiliary` entry Elfxx_Vernaux.
///
/// https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/symversion.html#VERNEEDEXTFIG
#[repr(C)]
#[derive(Debug, Pread)]
struct ElfVernaux {
    vna_hash: u32,
    vna_flags: u16,
    vna_other: u16,
    vna_name: u32,
    vna_next: u32,
}

/********************
 *  Symbol Version  *
 ********************/

/// Helper struct to iterate over [Symbol Version][Versym] entries.
#[derive(Debug)]
pub struct VersymSection<'a> {
    bytes: &'a [u8],
    ctx: container::Ctx,
}

impl<'a> VersymSection<'a> {
    pub fn parse(
        bytes: &'a [u8],
        shdrs: &[SectionHeader],
        ctx: container::Ctx,
    ) -> Result<Option<VersymSection<'a>>> {
        // Get fields needed from optional `symbol version` section.
        let (offset, size) =
            if let Some(shdr) = shdrs.iter().find(|shdr| shdr.sh_type == SHT_GNU_VERSYM) {
                (shdr.sh_offset as usize, shdr.sh_size as usize)
            } else {
                return Ok(None);
            };

        // Get a slice of bytes of the `version definition` section content.
        let bytes: &'a [u8] = bytes.pread_with(offset, size)?;

        Ok(Some(VersymSection { bytes, ctx }))
    }

    /// Get an iterator over the [`Versym`] entries.
    #[inline]
    pub fn iter(&'a self) -> VersymIter<'a> {
        self.into_iter()
    }

    /// True if there are no [`Versym`] entries.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Number of [`Versym`] entries.
    #[inline]
    pub fn len(&self) -> usize {
        let entsize = core::mem::size_of::<ElfVersym>();

        self.bytes.len() / entsize
    }

    /// Get [`Versym`] entry at index.
    #[inline]
    pub fn get_at(&self, idx: usize) -> Option<Versym> {
        let entsize = core::mem::size_of::<ElfVersym>();
        let offset = idx.checked_mul(entsize)?;

        self.bytes
            .pread_with::<ElfVersym>(offset, self.ctx.le)
            .ok()
            .map(Versym::from)
    }
}

impl<'a> IntoIterator for &'_ VersymSection<'a> {
    type Item = <VersymIter<'a> as Iterator>::Item;
    type IntoIter = VersymIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        VersymIter {
            bytes: self.bytes,
            offset: 0,
            ctx: self.ctx,
        }
    }
}

/// Iterator over the [`Versym`] entries from the [`SHT_GNU_VERSYM`] section.
pub struct VersymIter<'a> {
    bytes: &'a [u8],
    offset: usize,
    ctx: container::Ctx,
}

impl<'a> Iterator for VersymIter<'a> {
    type Item = Versym;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.bytes.len() {
            None
        } else {
            self.bytes
                .gread_with::<ElfVersym>(&mut self.offset, self.ctx.le)
                .ok()
                .map(Versym::from)
                .or_else(|| {
                    // self.bytes are not a multiple of ElfVersym.
                    // Adjust offset to continue yielding Nones.
                    self.offset = self.bytes.len();
                    None
                })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.bytes.len() - self.offset) / core::mem::size_of::<Self::Item>();
        (len, Some(len))
    }
}

impl ExactSizeIterator for VersymIter<'_> {}

impl FusedIterator for VersymIter<'_> {}

/// An ELF [Symbol Version][lsb-versym] entry.
///
/// [lsb-versym]: https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/symversion.html#SYMVERTBL
#[derive(Debug)]
pub struct Versym {
    pub vs_val: u16,
}

impl Versym {
    /// Returns true if the symbol is local and not available outside the object according to
    /// [`VER_NDX_LOCAL`].
    #[inline]
    pub fn is_local(&self) -> bool {
        self.vs_val == VER_NDX_LOCAL
    }

    /// Returns true if the symbol is defined in this object and globally available according
    /// to [`VER_NDX_GLOBAL`].
    #[inline]
    pub fn is_global(&self) -> bool {
        self.vs_val == VER_NDX_GLOBAL
    }

    /// Returns true if the `hidden` bit is set according to the [`VERSYM_HIDDEN`] bitmask.
    #[inline]
    pub fn is_hidden(&self) -> bool {
        (self.vs_val & VERSYM_HIDDEN) == VERSYM_HIDDEN
    }

    /// Returns the symbol version index according to the [`VERSYM_VERSION`] bitmask.
    #[inline]
    pub fn version(&self) -> u16 {
        self.vs_val & VERSYM_VERSION
    }
}

impl From<ElfVersym> for Versym {
    fn from(ElfVersym { vs_val }: ElfVersym) -> Self {
        Versym { vs_val }
    }
}

/************************
 *  Version Definition  *
 ************************/

/// Helper struct to iterate over [Version Definition][Verdef] and [Version Definition
/// Auxiliary][Verdaux] entries.
#[derive(Debug)]
pub struct VerdefSection<'a> {
    /// String table used to resolve version strings.
    bytes: &'a [u8],
    count: usize,
    ctx: container::Ctx,
}

impl<'a> VerdefSection<'a> {
    pub fn parse(
        bytes: &'a [u8],
        shdrs: &[SectionHeader],
        ctx: container::Ctx,
    ) -> Result<Option<VerdefSection<'a>>> {
        // Get fields needed from optional `version definition` section.
        let (offset, size, count) =
            if let Some(shdr) = shdrs.iter().find(|shdr| shdr.sh_type == SHT_GNU_VERDEF) {
                (
                    shdr.sh_offset as usize,
                    shdr.sh_size as usize,
                    shdr.sh_info as usize, // Encodes the number of ElfVerdef entries.
                )
            } else {
                return Ok(None);
            };

        // Get a slice of bytes of the `version definition` section content.
        let bytes: &'a [u8] = bytes.pread_with(offset, size)?;

        Ok(Some(VerdefSection { bytes, count, ctx }))
    }

    /// Get an iterator over the [`Verdef`] entries.
    #[inline]
    pub fn iter(&'a self) -> VerdefIter<'a> {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'_ VerdefSection<'a> {
    type Item = <VerdefIter<'a> as Iterator>::Item;
    type IntoIter = VerdefIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        VerdefIter {
            bytes: self.bytes,
            count: self.count,
            index: 0,
            offset: 0,
            ctx: self.ctx,
        }
    }
}

/// Iterator over the [`Verdef`] entries from the [`SHT_GNU_VERDEF`] section.
pub struct VerdefIter<'a> {
    bytes: &'a [u8],
    count: usize,
    index: usize,
    offset: usize,
    ctx: container::Ctx,
}

impl<'a> Iterator for VerdefIter<'a> {
    type Item = Verdef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            None
        } else {
            self.index += 1;

            let do_next = |iter: &mut Self| {
                let ElfVerdef {
                    vd_version,
                    vd_flags,
                    vd_ndx,
                    vd_cnt,
                    vd_hash,
                    vd_aux,
                    vd_next,
                } = iter.bytes.pread_with(iter.offset, iter.ctx.le).ok()?;

                // Validate offset to first ElfVerdaux entry.
                let offset = iter.offset.checked_add(vd_aux as usize)?;

                // Validate if offset is valid index into bytes slice.
                if offset >= iter.bytes.len() {
                    return None;
                }

                // Get a slice of bytes starting with the first ElfVerdaux entry.
                let bytes: &'a [u8] = &iter.bytes[offset..];

                // Bump the offset to the next ElfVerdef entry.
                iter.offset = iter.offset.checked_add(vd_next as usize)?;

                // Start yielding None on the next call if there is no next offset.
                if vd_next == 0 {
                    iter.index = iter.count;
                }

                Some(Verdef {
                    vd_version,
                    vd_flags,
                    vd_ndx,
                    vd_cnt,
                    vd_hash,
                    vd_aux,
                    vd_next,
                    bytes,
                    ctx: iter.ctx,
                })
            };

            do_next(self).or_else(|| {
                // Adjust current index to count in case of an error.
                self.index = self.count;
                None
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.count - self.index;
        (0, Some(len))
    }
}

impl ExactSizeIterator for VerdefIter<'_> {}

impl FusedIterator for VerdefIter<'_> {}

/// An ELF [Version Definition][lsb-verdef] entry .
///
/// [lsb-verdef]: https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/symversion.html#VERDEFENTRIES
#[derive(Debug)]
pub struct Verdef<'a> {
    /// Version revision. This field shall be set to 1.
    pub vd_version: u16,
    /// Version information flag bitmask.
    pub vd_flags: u16,
    /// Version index numeric value referencing the SHT_GNU_versym section.
    pub vd_ndx: u16,
    /// Number of associated verdaux array entries.
    pub vd_cnt: u16,
    /// Version name hash value (ELF hash function).
    pub vd_hash: u32,
    /// Offset in bytes to a corresponding entry in an array of Elfxx_Verdaux structures.
    pub vd_aux: u32,
    /// Offset to the next verdef entry, in bytes.
    pub vd_next: u32,

    bytes: &'a [u8],
    ctx: container::Ctx,
}

impl<'a> Verdef<'a> {
    /// Get an iterator over the [`Verdaux`] entries of this [`Verdef`] entry.
    #[inline]
    pub fn iter(&'a self) -> VerdauxIter<'a> {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'_ Verdef<'a> {
    type Item = <VerdauxIter<'a> as Iterator>::Item;
    type IntoIter = VerdauxIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        VerdauxIter {
            bytes: self.bytes,
            count: self.vd_cnt,
            index: 0,
            offset: 0,
            ctx: self.ctx,
        }
    }
}

/// Iterator over the [`Verdaux`] entries for an specific [`Verdef`] entry.
pub struct VerdauxIter<'a> {
    bytes: &'a [u8],
    count: u16,
    index: u16,
    offset: usize,
    ctx: container::Ctx,
}

impl<'a> Iterator for VerdauxIter<'a> {
    type Item = Verdaux;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            None
        } else {
            self.index += 1;

            let do_next = |iter: &mut Self| {
                let ElfVerdaux { vda_name, vda_next } =
                    iter.bytes.pread_with(iter.offset, iter.ctx.le).ok()?;

                // Bump the offset to the next ElfVerdaux entry.
                iter.offset = iter.offset.checked_add(vda_next as usize)?;

                // Start yielding None on the next call if there is no next offset.
                if vda_next == 0 {
                    iter.index = iter.count;
                }

                Some(Verdaux {
                    vda_name: vda_name as usize,
                    vda_next,
                })
            };

            do_next(self).or_else(|| {
                // Adjust current index to count in case of an error.
                self.index = self.count;
                None
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = usize::from(self.count - self.index);
        (0, Some(len))
    }
}

impl ExactSizeIterator for VerdauxIter<'_> {}

impl FusedIterator for VerdauxIter<'_> {}

/// An ELF [Version Definition Auxiliary][lsb-verdaux] entry.
///
/// [lsb-verdaux]: https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/symversion.html#VERDEFEXTS
#[derive(Debug)]
pub struct Verdaux {
    /// Offset to the version or dependency name string in the section header, in bytes.
    pub vda_name: usize,
    /// Offset to the next verdaux entry, in bytes.
    pub vda_next: u32,
}

/**************************
 *  Version Requirements  *
 **************************/

/// Helper struct to iterate over [Version Needed][Verneed] and [Version Needed
/// Auxiliary][Vernaux] entries.
#[derive(Debug)]
pub struct VerneedSection<'a> {
    bytes: &'a [u8],
    count: usize,
    ctx: container::Ctx,
}

impl<'a> VerneedSection<'a> {
    /// Try to parse the optional [`SHT_GNU_VERNEED`] section.
    pub fn parse(
        bytes: &'a [u8],
        shdrs: &[SectionHeader],
        ctx: container::Ctx,
    ) -> Result<Option<VerneedSection<'a>>> {
        // Get fields needed from optional `version needed` section.
        let (offset, size, count) =
            if let Some(shdr) = shdrs.iter().find(|shdr| shdr.sh_type == SHT_GNU_VERNEED) {
                (
                    shdr.sh_offset as usize,
                    shdr.sh_size as usize,
                    shdr.sh_info as usize, // Encodes the number of ElfVerneed entries.
                )
            } else {
                return Ok(None);
            };

        // Get a slice of bytes of the `version needed` section content.
        let bytes: &'a [u8] = bytes.pread_with(offset, size)?;

        Ok(Some(VerneedSection { bytes, count, ctx }))
    }

    /// Get an iterator over the [`Verneed`] entries.
    #[inline]
    pub fn iter(&'a self) -> VerneedIter<'a> {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'_ VerneedSection<'a> {
    type Item = <VerneedIter<'a> as Iterator>::Item;
    type IntoIter = VerneedIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        VerneedIter {
            bytes: self.bytes,
            count: self.count,
            index: 0,
            offset: 0,
            ctx: self.ctx,
        }
    }
}

/// Iterator over the [`Verneed`] entries from the [`SHT_GNU_VERNEED`] section.
pub struct VerneedIter<'a> {
    bytes: &'a [u8],
    count: usize,
    index: usize,
    offset: usize,
    ctx: container::Ctx,
}

impl<'a> Iterator for VerneedIter<'a> {
    type Item = Verneed<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            None
        } else {
            self.index += 1;

            let do_next = |iter: &mut Self| {
                let ElfVerneed {
                    vn_version,
                    vn_cnt,
                    vn_file,
                    vn_aux,
                    vn_next,
                } = iter.bytes.pread_with(iter.offset, iter.ctx.le).ok()?;

                // Validate offset to first ElfVernaux entry.
                let offset = iter.offset.checked_add(vn_aux as usize)?;

                // Validate if offset is valid index into bytes slice.
                if offset >= iter.bytes.len() {
                    return None;
                }

                // Get a slice of bytes starting with the first ElfVernaux entry.
                let bytes: &'a [u8] = &iter.bytes[offset..];

                // Bump the offset to the next ElfVerneed entry.
                iter.offset = iter.offset.checked_add(vn_next as usize)?;

                // Start yielding None on the next call if there is no next offset.
                if vn_next == 0 {
                    iter.index = iter.count;
                }

                Some(Verneed {
                    vn_version,
                    vn_cnt,
                    vn_file: vn_file as usize,
                    vn_aux,
                    vn_next,
                    bytes,
                    ctx: iter.ctx,
                })
            };

            do_next(self).or_else(|| {
                // Adjust current index to count in case of an error.
                self.index = self.count;
                None
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.count - self.index;
        (0, Some(len))
    }
}

impl ExactSizeIterator for VerneedIter<'_> {}

impl FusedIterator for VerneedIter<'_> {}

/// An ELF [Version Need][lsb-verneed] entry .
///
/// [lsb-verneed]: https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/symversion.html#VERNEEDFIG
#[derive(Debug)]
pub struct Verneed<'a> {
    /// Version of structure. This value is currently set to 1, and will be reset if the versioning
    /// implementation is incompatibly altered.
    pub vn_version: u16,
    /// Number of associated verneed array entries.
    pub vn_cnt: u16,
    /// Offset to the file name string in the section header, in bytes.
    pub vn_file: usize,
    /// Offset to a corresponding entry in the vernaux array, in bytes.
    pub vn_aux: u32,
    /// Offset to the next verneed entry, in bytes.
    pub vn_next: u32,

    bytes: &'a [u8],
    ctx: container::Ctx,
}

impl<'a> Verneed<'a> {
    /// Get an iterator over the [`Vernaux`] entries of this [`Verneed`] entry.
    #[inline]
    pub fn iter(&'a self) -> VernauxIter<'a> {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'_ Verneed<'a> {
    type Item = <VernauxIter<'a> as Iterator>::Item;
    type IntoIter = VernauxIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        VernauxIter {
            bytes: self.bytes,
            count: self.vn_cnt,
            index: 0,
            offset: 0,
            ctx: self.ctx,
        }
    }
}

/// Iterator over the [`Vernaux`] entries for an specific [`Verneed`] entry.
pub struct VernauxIter<'a> {
    bytes: &'a [u8],
    count: u16,
    index: u16,
    offset: usize,
    ctx: container::Ctx,
}

impl<'a> Iterator for VernauxIter<'a> {
    type Item = Vernaux;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            None
        } else {
            self.index += 1;

            let do_next = |iter: &mut Self| {
                let ElfVernaux {
                    vna_hash,
                    vna_flags,
                    vna_other,
                    vna_name,
                    vna_next,
                } = iter.bytes.pread_with(iter.offset, iter.ctx.le).ok()?;

                // Bump the offset to the next ElfVernaux entry.
                iter.offset = iter.offset.checked_add(vna_next as usize)?;

                // Start yielding None on the next call if there is no next offset.
                if vna_next == 0 {
                    iter.index = iter.count;
                }

                Some(Vernaux {
                    vna_hash,
                    vna_flags,
                    vna_other,
                    vna_name: vna_name as usize,
                    vna_next,
                })
            };

            do_next(self).or_else(|| {
                // Adjust current index to count in case of an error.
                self.index = self.count;
                None
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = usize::from(self.count - self.index);
        (0, Some(len))
    }
}

impl ExactSizeIterator for VernauxIter<'_> {}

impl FusedIterator for VernauxIter<'_> {}

/// An ELF [Version Need Auxiliary][lsb-vernaux] entry.
///
/// [lsb-vernaux]: https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/symversion.html#VERNEEDEXTFIG
#[derive(Debug)]
pub struct Vernaux {
    /// Dependency name hash value (ELF hash function).
    pub vna_hash: u32,
    /// Dependency information flag bitmask.
    pub vna_flags: u16,
    /// Object file version identifier used in the .gnu.version symbol version array. Bit number 15
    /// controls whether or not the object is hidden; if this bit is set, the object cannot be used
    /// and the static linker will ignore the symbol's presence in the object.
    pub vna_other: u16,
    /// Offset to the dependency name string in the section header, in bytes.
    pub vna_name: usize,
    /// Offset to the next vernaux entry, in bytes.
    pub vna_next: u32,
}

#[cfg(test)]
mod test {
    use super::{ElfVerdaux, ElfVerdef, ElfVernaux, ElfVerneed, ElfVersym};
    use super::{Versym, VERSYM_HIDDEN, VER_NDX_GLOBAL, VER_NDX_LOCAL};
    use core::mem::size_of;

    #[test]
    fn check_size() {
        assert_eq!(2, size_of::<ElfVersym>());
        assert_eq!(20, size_of::<ElfVerdef>());
        assert_eq!(8, size_of::<ElfVerdaux>());
        assert_eq!(16, size_of::<ElfVerneed>());
        assert_eq!(16, size_of::<ElfVernaux>());
    }

    #[test]
    fn check_versym() {
        let local = Versym {
            vs_val: VER_NDX_LOCAL,
        };
        assert_eq!(true, local.is_local());
        assert_eq!(false, local.is_global());
        assert_eq!(false, local.is_hidden());
        assert_eq!(VER_NDX_LOCAL, local.version());

        let global = Versym {
            vs_val: VER_NDX_GLOBAL,
        };
        assert_eq!(false, global.is_local());
        assert_eq!(true, global.is_global());
        assert_eq!(false, global.is_hidden());
        assert_eq!(VER_NDX_GLOBAL, global.version());

        let hidden = Versym {
            vs_val: VERSYM_HIDDEN,
        };
        assert_eq!(false, hidden.is_local());
        assert_eq!(false, hidden.is_global());
        assert_eq!(true, hidden.is_hidden());
        assert_eq!(0, hidden.version());

        let hidden = Versym {
            vs_val: VERSYM_HIDDEN | 0x123,
        };
        assert_eq!(false, hidden.is_local());
        assert_eq!(false, hidden.is_global());
        assert_eq!(true, hidden.is_hidden());
        assert_eq!(0x123, hidden.version());
    }
}
