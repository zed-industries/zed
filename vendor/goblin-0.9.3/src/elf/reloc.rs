//! # Relocation computations
//!
//! The following notation is used to describe relocation computations
//! specific to x86_64 ELF.
//!
//!  * A: The addend used to compute the value of the relocatable field.
//!  * B: The base address at which a shared object is loaded into memory
//!       during execution. Generally, a shared object file is built with a
//!       base virtual address of 0. However, the execution address of the
//!       shared object is different.
//!  * G: The offset into the global offset table at which the address of
//!       the relocation entry's symbol resides during execution.
//!  * GOT: The address of the global offset table.
//!  * L: The section offset or address of the procedure linkage table entry
//!       for a symbol.
//!  * P: The section offset or address of the storage unit being relocated,
//!       computed using r_offset.
//!  * S: The value of the symbol whose index resides in the relocation entry.
//!  * Z: The size of the symbol whose index resides in the relocation entry.
//!
//! Below are some common x86_64 relocation computations you might find useful:
//!
//! | Relocation                | Value | Size      | Formula           |
//! |:--------------------------|:------|:----------|:------------------|
//! | `R_X86_64_NONE`           | 0     | NONE      | NONE              |
//! | `R_X86_64_64`             | 1     | 64        | S + A             |
//! | `R_X86_64_PC32`           | 2     | 32        | S + A - P         |
//! | `R_X86_64_GOT32`          | 3     | 32        | G + A             |
//! | `R_X86_64_PLT32`          | 4     | 32        | L + A - P         |
//! | `R_X86_64_COPY`           | 5     | NONE      | NONE              |
//! | `R_X86_64_GLOB_DAT`       | 6     | 64        | S                 |
//! | `R_X86_64_JUMP_SLOT`      | 7     | 64        | S                 |
//! | `R_X86_64_RELATIVE`       | 8     | 64        | B + A             |
//! | `R_X86_64_GOTPCREL`       | 9     | 32        | G + GOT + A - P   |
//! | `R_X86_64_32`             | 10    | 32        | S + A             |
//! | `R_X86_64_32S`            | 11    | 32        | S + A             |
//! | `R_X86_64_16`             | 12    | 16        | S + A             |
//! | `R_X86_64_PC16`           | 13    | 16        | S + A - P         |
//! | `R_X86_64_8`              | 14    | 8         | S + A             |
//! | `R_X86_64_PC8`            | 15    | 8         | S + A - P         |
//! | `R_X86_64_DTPMOD64`       | 16    | 64        |                   |
//! | `R_X86_64_DTPOFF64`       | 17    | 64        |                   |
//! | `R_X86_64_TPOFF64`        | 18    | 64        |                   |
//! | `R_X86_64_TLSGD`          | 19    | 32        |                   |
//! | `R_X86_64_TLSLD`          | 20    | 32        |                   |
//! | `R_X86_64_DTPOFF32`       | 21    | 32        |                   |
//! | `R_X86_64_GOTTPOFF`       | 22    | 32        |                   |
//! | `R_X86_64_TPOFF32`        | 23    | 32        |                   |
//! | `R_X86_64_PC64`           | 24    | 64        | S + A - P         |
//! | `R_X86_64_GOTOFF64`       | 25    | 64        | S + A - GOT       |
//! | `R_X86_64_GOTPC32`        | 26    | 32        | GOT + A - P       |
//! | `R_X86_64_SIZE32`         | 32    | 32        | Z + A             |
//! | `R_X86_64_SIZE64`         | 33    | 64        | Z + A             |
//! | `R_X86_64_GOTPC32_TLSDESC`| 34    | 32        |                   |
//! | `R_X86_64_TLSDESC_CALL`   | 35    | NONE      |                   |
//! | `R_X86_64_TLSDESC`        | 36    | 64 × 2    |                   |
//! | `R_X86_64_IRELATIVE`      | 37    | 64        | indirect (B + A)  |
//!
//! TLS information is at <http://people.redhat.com/aoliva/writeups/TLS/RFC-TLSDESC-x86.txt>
//!
//! `R_X86_64_IRELATIVE` is similar to `R_X86_64_RELATIVE` except that
//! the value used in this relocation is the program address returned by the function,
//! which takes no arguments, at the address of the result of the corresponding
//! `R_X86_64_RELATIVE` relocation.
//!
//! Read more <https://docs.oracle.com/cd/E23824_01/html/819-0690/chapter6-54839.html>

include!("constants_relocation.rs");

macro_rules! elf_reloc {
    ($size:ident, $isize:ty) => {
        use core::fmt;
        #[cfg(feature = "alloc")]
        use scroll::{Pread, Pwrite, SizeWith};
        #[repr(C)]
        #[derive(Clone, Copy, PartialEq, Default)]
        #[cfg_attr(feature = "alloc", derive(Pread, Pwrite, SizeWith))]
        /// Relocation with an explicit addend
        pub struct Rela {
            /// Address
            pub r_offset: $size,
            /// Relocation type and symbol index
            pub r_info: $size,
            /// Addend
            pub r_addend: $isize,
        }
        #[repr(C)]
        #[derive(Clone, PartialEq, Default)]
        #[cfg_attr(feature = "alloc", derive(Pread, Pwrite, SizeWith))]
        /// Relocation without an addend
        pub struct Rel {
            /// address
            pub r_offset: $size,
            /// relocation type and symbol address
            pub r_info: $size,
        }
        use plain;
        unsafe impl plain::Plain for Rela {}
        unsafe impl plain::Plain for Rel {}

        impl fmt::Debug for Rela {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let sym = r_sym(self.r_info);
                let typ = r_type(self.r_info);
                f.debug_struct("Rela")
                    .field("r_offset", &format_args!("{:x}", self.r_offset))
                    .field("r_info", &format_args!("{:x}", self.r_info))
                    .field("r_addend", &format_args!("{:x}", self.r_addend))
                    .field("r_typ", &typ)
                    .field("r_sym", &sym)
                    .finish()
            }
        }
        impl fmt::Debug for Rel {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let sym = r_sym(self.r_info);
                let typ = r_type(self.r_info);
                f.debug_struct("Rel")
                    .field("r_offset", &format_args!("{:x}", self.r_offset))
                    .field("r_info", &format_args!("{:x}", self.r_info))
                    .field("r_typ", &typ)
                    .field("r_sym", &sym)
                    .finish()
            }
        }
    };
}

macro_rules! elf_rela_std_impl {
    ($size:ident, $isize:ty) => {
        if_alloc! {
            use core::slice;

            if_std! {
                use crate::error::Result;

                use std::fs::File;
                use std::io::{Read, Seek};
                use std::io::SeekFrom::Start;
            }

            impl From<Rela> for Reloc {
                fn from(rela: Rela) -> Self {
                    Reloc {
                        r_offset: u64::from(rela.r_offset),
                        r_addend: Some(i64::from(rela.r_addend)),
                        r_sym: r_sym(rela.r_info) as usize,
                        r_type: r_type(rela.r_info),
                    }
                }
            }

            impl From<Rel> for Reloc {
                fn from(rel: Rel) -> Self {
                    Reloc {
                        r_offset: u64::from(rel.r_offset),
                        r_addend: None,
                        r_sym: r_sym(rel.r_info) as usize,
                        r_type: r_type(rel.r_info),
                    }
                }
            }

            impl From<Reloc> for Rela {
                fn from(rela: Reloc) -> Self {
                    let r_info = r_info(rela.r_sym as $size, $size::from(rela.r_type));
                    Rela {
                        r_offset: rela.r_offset as $size,
                        r_info: r_info,
                        r_addend: rela.r_addend.unwrap_or(0) as $isize,
                    }
                }
            }

            impl From<Reloc> for Rel {
                fn from(rel: Reloc) -> Self {
                    let r_info = r_info(rel.r_sym as $size, $size::from(rel.r_type));
                    Rel {
                        r_offset: rel.r_offset as $size,
                        r_info: r_info,
                    }
                }
            }

            /// Gets the rela entries given a rela pointer and the _size_ of the rela section in the binary,
            /// in bytes.
            /// Assumes the pointer is valid and can safely return a slice of memory pointing to the relas because:
            /// 1. `ptr` points to memory received from the kernel (i.e., it loaded the executable), _or_
            /// 2. The binary has already been mmapped (i.e., it's a `SharedObject`), and hence it's safe to return a slice of that memory.
            /// 3. Or if you obtained the pointer in some other lawful manner
            pub unsafe fn from_raw_rela<'a>(ptr: *const Rela, size: usize) -> &'a [Rela] {
                slice::from_raw_parts(ptr, size / SIZEOF_RELA)
            }

            /// Gets the rel entries given a rel pointer and the _size_ of the rel section in the binary,
            /// in bytes.
            /// Assumes the pointer is valid and can safely return a slice of memory pointing to the rels because:
            /// 1. `ptr` points to memory received from the kernel (i.e., it loaded the executable), _or_
            /// 2. The binary has already been mmapped (i.e., it's a `SharedObject`), and hence it's safe to return a slice of that memory.
            /// 3. Or if you obtained the pointer in some other lawful manner
            pub unsafe fn from_raw_rel<'a>(ptr: *const Rel, size: usize) -> &'a [Rel] {
                slice::from_raw_parts(ptr, size / SIZEOF_REL)
            }

            #[cfg(feature = "std")]
            pub fn from_fd(fd: &mut File, offset: usize, size: usize) -> Result<Vec<Rela>> {
                let count = size / SIZEOF_RELA;
                let mut relocs = vec![Rela::default(); count];
                fd.seek(Start(offset as u64))?;
                unsafe {
                    fd.read_exact(plain::as_mut_bytes(&mut *relocs))?;
                }
                Ok(relocs)
            }
        } // end if_alloc
    };
}

pub mod reloc32 {

    pub use crate::elf::reloc::*;

    elf_reloc!(u32, i32);

    pub const SIZEOF_RELA: usize = 4 + 4 + 4;
    pub const SIZEOF_REL: usize = 4 + 4;

    #[inline(always)]
    pub fn r_sym(info: u32) -> u32 {
        info >> 8
    }

    #[inline(always)]
    pub fn r_type(info: u32) -> u32 {
        info & 0xff
    }

    #[inline(always)]
    pub fn r_info(sym: u32, typ: u32) -> u32 {
        (sym << 8) + (typ & 0xff)
    }

    elf_rela_std_impl!(u32, i32);
}

pub mod reloc64 {
    pub use crate::elf::reloc::*;

    elf_reloc!(u64, i64);

    pub const SIZEOF_RELA: usize = 8 + 8 + 8;
    pub const SIZEOF_REL: usize = 8 + 8;

    #[inline(always)]
    pub fn r_sym(info: u64) -> u32 {
        (info >> 32) as u32
    }

    #[inline(always)]
    pub fn r_type(info: u64) -> u32 {
        (info & 0xffff_ffff) as u32
    }

    #[inline(always)]
    pub fn r_info(sym: u64, typ: u64) -> u64 {
        (sym << 32) + typ
    }

    elf_rela_std_impl!(u64, i64);
}

//////////////////////////////
// Generic Reloc
/////////////////////////////
if_alloc! {
    use scroll::{ctx, Pread};
    use scroll::ctx::SizeWith;
    use core::fmt;
    use core::result;
    use crate::container::{Ctx, Container};
    use alloc::vec::Vec;

    #[derive(Clone, Copy, PartialEq, Default)]
    /// A unified ELF relocation structure
    pub struct Reloc {
        /// Address
        pub r_offset: u64,
        /// Addend
        pub r_addend: Option<i64>,
        /// The index into the corresponding symbol table - either dynamic or regular
        pub r_sym: usize,
        /// The relocation type
        pub r_type: u32,
    }

    impl Reloc {
        pub fn size(is_rela: bool, ctx: Ctx) -> usize {
            use scroll::ctx::SizeWith;
            Reloc::size_with(&(is_rela, ctx))
        }
    }

    type RelocCtx = (bool, Ctx);

    impl ctx::SizeWith<RelocCtx> for Reloc {
        fn size_with( &(is_rela, Ctx { container, .. }): &RelocCtx) -> usize {
            match container {
                Container::Little => {
                    if is_rela { reloc32::SIZEOF_RELA } else { reloc32::SIZEOF_REL }
                },
                Container::Big => {
                    if is_rela { reloc64::SIZEOF_RELA } else { reloc64::SIZEOF_REL }
                }
            }
        }
    }

    impl<'a> ctx::TryFromCtx<'a, RelocCtx> for Reloc {
        type Error = crate::error::Error;
        fn try_from_ctx(bytes: &'a [u8], (is_rela, Ctx { container, le }): RelocCtx) -> result::Result<(Self, usize), Self::Error> {
            use scroll::Pread;
            let reloc = match container {
                Container::Little => {
                    if is_rela {
                        (bytes.pread_with::<reloc32::Rela>(0, le)?.into(), reloc32::SIZEOF_RELA)
                    } else {
                        (bytes.pread_with::<reloc32::Rel>(0, le)?.into(), reloc32::SIZEOF_REL)
                    }
                },
                Container::Big => {
                    if is_rela {
                        (bytes.pread_with::<reloc64::Rela>(0, le)?.into(), reloc64::SIZEOF_RELA)
                    } else {
                        (bytes.pread_with::<reloc64::Rel>(0, le)?.into(), reloc64::SIZEOF_REL)
                    }
                }
            };
            Ok(reloc)
        }
    }

    impl ctx::TryIntoCtx<RelocCtx> for Reloc {
        type Error = crate::error::Error;
        /// Writes the relocation into `bytes`
        fn try_into_ctx(self, bytes: &mut [u8], (is_rela, Ctx {container, le}): RelocCtx) -> result::Result<usize, Self::Error> {
            use scroll::Pwrite;
            match container {
                Container::Little => {
                    if is_rela {
                        let rela: reloc32::Rela = self.into();
                        Ok(bytes.pwrite_with(rela, 0, le)?)
                    } else {
                        let rel: reloc32::Rel = self.into();
                        Ok(bytes.pwrite_with(rel, 0, le)?)
                    }
                },
                Container::Big => {
                    if is_rela {
                        let rela: reloc64::Rela = self.into();
                        Ok(bytes.pwrite_with(rela, 0, le)?)
                    } else {
                        let rel: reloc64::Rel = self.into();
                        Ok(bytes.pwrite_with(rel, 0, le)?)
                    }
                },
            }
        }
    }

    impl ctx::IntoCtx<(bool, Ctx)> for Reloc {
        /// Writes the relocation into `bytes`
        fn into_ctx(self, bytes: &mut [u8], ctx: RelocCtx) {
            use scroll::Pwrite;
            bytes.pwrite_with(self, 0, ctx).unwrap();
        }
    }

    impl fmt::Debug for Reloc {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("Reloc")
                .field("r_offset", &format_args!("{:x}", self.r_offset))
                .field("r_addend", &format_args!("{:x}", self.r_addend.unwrap_or(0)))
                .field("r_sym", &self.r_sym)
                .field("r_type", &self.r_type)
                .finish()
        }
    }

    #[derive(Default)]
    /// An ELF section containing relocations, allowing lazy iteration over symbols.
    pub struct RelocSection<'a> {
        bytes: &'a [u8],
        count: usize,
        ctx: RelocCtx,
        start: usize,
        end: usize,
    }

    impl<'a> fmt::Debug for RelocSection<'a> {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            let len = self.bytes.len();
            fmt.debug_struct("RelocSection")
                .field("bytes", &len)
                .field("range", &format!("{:#x}..{:#x}", self.start, self.end))
                .field("count", &self.count)
                .field("Relocations", &self.to_vec())
                .finish()
        }
    }

    impl<'a> RelocSection<'a> {
        #[cfg(feature = "endian_fd")]
        /// Parse a REL or RELA section of size `filesz` from `offset`.
        pub fn parse(bytes: &'a [u8], offset: usize, filesz: usize, is_rela: bool, ctx: Ctx) -> crate::error::Result<RelocSection<'a>> {
            // TODO: better error message when too large (see symtab implementation)
            let bytes = if filesz != 0 {
                bytes.pread_with::<&'a [u8]>(offset, filesz)?
            } else {
                &[]
            };

            Ok(RelocSection {
                bytes: bytes,
                count: filesz / Reloc::size(is_rela, ctx),
                ctx: (is_rela, ctx),
                start: offset,
                end: offset + filesz,
            })
        }

        /// Try to parse a single relocation from the binary, at `index`.
        #[inline]
        pub fn get(&self, index: usize) -> Option<Reloc> {
            if index >= self.count {
                None
            } else {
                Some(self.bytes.pread_with(index * Reloc::size_with(&self.ctx), self.ctx).unwrap())
            }
        }

        /// The number of relocations in the section.
        #[inline]
        pub fn len(&self) -> usize {
            self.count
        }

        /// Returns true if section has no relocations.
        #[inline]
        pub fn is_empty(&self) -> bool {
            self.count == 0
        }

        /// Iterate over all relocations.
        pub fn iter(&self) -> RelocIterator<'a> {
            self.into_iter()
        }

        /// Parse all relocations into a vector.
        pub fn to_vec(&self) -> Vec<Reloc> {
            self.iter().collect()
        }
    }

    impl<'a, 'b> IntoIterator for &'b RelocSection<'a> {
        type Item = <RelocIterator<'a> as Iterator>::Item;
        type IntoIter = RelocIterator<'a>;

        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            RelocIterator {
                bytes: self.bytes,
                offset: 0,
                index: 0,
                count: self.count,
                ctx: self.ctx,
            }
        }
    }

    pub struct RelocIterator<'a> {
        bytes: &'a [u8],
        offset: usize,
        index: usize,
        count: usize,
        ctx: RelocCtx,
    }

    impl<'a> fmt::Debug for RelocIterator<'a> {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.debug_struct("RelocIterator")
                .field("bytes", &"<... redacted ...>")
                .field("offset", &self.offset)
                .field("index", &self.index)
                .field("count", &self.count)
                .field("ctx", &self.ctx)
                .finish()
        }
    }

    impl<'a> Iterator for RelocIterator<'a> {
        type Item = Reloc;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= self.count {
                None
            } else {
                self.index += 1;
                Some(self.bytes.gread_with(&mut self.offset, self.ctx).unwrap())
            }
        }
    }

    impl<'a> ExactSizeIterator for RelocIterator<'a> {
        #[inline]
        fn len(&self) -> usize {
            self.count - self.index
        }
    }
} // end if_alloc
