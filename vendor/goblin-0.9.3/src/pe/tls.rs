use crate::error;
use alloc::vec::Vec;
use scroll::{Pread, Pwrite, SizeWith};

use crate::pe::data_directories;
use crate::pe::options;
use crate::pe::section_table;
use crate::pe::utils;

/// Indicates 1-byte alignment for Thread Local Storage (TLS) characteristics field in [`ImageTlsDirectory::characteristics`]
pub const TLS_CHARACTERISTICS_ALIGN_1BYTES: u32 = 0x00100000;
/// Indicates 2-byte alignment for Thread Local Storage (TLS) characteristics field in [`ImageTlsDirectory::characteristics`]
pub const TLS_CHARACTERISTICS_ALIGN_2BYTES: u32 = 0x00200000;
/// Indicates 4-byte alignment for Thread Local Storage (TLS) characteristics field in [`ImageTlsDirectory::characteristics`]
pub const TLS_CHARACTERISTICS_ALIGN_4BYTES: u32 = 0x00300000;
/// Indicates 8-byte alignment for Thread Local Storage (TLS) characteristics field in [`ImageTlsDirectory::characteristics`]
pub const TLS_CHARACTERISTICS_ALIGN_8BYTES: u32 = 0x00400000;
/// Indicates 16-byte alignment for Thread Local Storage (TLS) characteristics field in [`ImageTlsDirectory::characteristics`]
pub const TLS_CHARACTERISTICS_ALIGN_16BYTES: u32 = 0x00500000;
/// Indicates 32-byte alignment for Thread Local Storage (TLS) characteristics field in [`ImageTlsDirectory::characteristics`]
pub const TLS_CHARACTERISTICS_ALIGN_32BYTES: u32 = 0x00600000;
/// Indicates 64-byte alignment for Thread Local Storage (TLS) characteristics field in [`ImageTlsDirectory::characteristics`]
pub const TLS_CHARACTERISTICS_ALIGN_64BYTES: u32 = 0x00700000;
/// Indicates 128-byte alignment for Thread Local Storage (TLS) characteristics field in [`ImageTlsDirectory::characteristics`]
pub const TLS_CHARACTERISTICS_ALIGN_128BYTES: u32 = 0x00800000;
/// Indicates 256-byte alignment for Thread Local Storage (TLS) characteristics field in [`ImageTlsDirectory::characteristics`]
pub const TLS_CHARACTERISTICS_ALIGN_256BYTES: u32 = 0x00900000;
/// Indicates 512-byte alignment for Thread Local Storage (TLS) characteristics field in [`ImageTlsDirectory::characteristics`]
pub const TLS_CHARACTERISTICS_ALIGN_512BYTES: u32 = 0x00A00000;
/// Indicates 1024-byte alignment for Thread Local Storage (TLS) characteristics field in [`ImageTlsDirectory::characteristics`]
pub const TLS_CHARACTERISTICS_ALIGN_1024BYTES: u32 = 0x00B00000;
/// Indicates 2048-byte alignment for Thread Local Storage (TLS) characteristics field in [`ImageTlsDirectory::characteristics`]
pub const TLS_CHARACTERISTICS_ALIGN_2048BYTES: u32 = 0x00D00000;
/// Indicates 4096-byte alignment for Thread Local Storage (TLS) characteristics field in [`ImageTlsDirectory::characteristics`]
pub const TLS_CHARACTERISTICS_ALIGN_4096BYTES: u32 = 0x00C00000;
/// Indicates 8192-byte alignment for Thread Local Storage (TLS) characteristics field in [`ImageTlsDirectory::characteristics`]
pub const TLS_CHARACTERISTICS_ALIGN_8192BYTES: u32 = 0x00E00000;
/// Mask for isolating alignment information from the characteristics field in [`ImageTlsDirectory::characteristics`]
pub const TLS_CHARACTERISTICS_ALIGN_MASK: u32 = 0x00F00000;

/// Represents the TLS directory `IMAGE_TLS_DIRECTORY64`.
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, Default, Pread, Pwrite, SizeWith)]
pub struct ImageTlsDirectory {
    /// The starting address of the TLS raw data.
    // NOTE: `u32` for 32-bit binaries, `u64` for 64-bit binaries.
    pub start_address_of_raw_data: u64,
    /// The ending address of the TLS raw data.
    // NOTE: `u32` for 32-bit binaries, `u64` for 64-bit binaries.
    pub end_address_of_raw_data: u64,
    /// The address of the TLS index.
    // NOTE: `u32` for 32-bit binaries, `u64` for 64-bit binaries.
    pub address_of_index: u64,
    /// The address of the TLS callback functions.
    ///
    /// Terminated by a null pointer.
    // NOTE: `u32` for 32-bit binaries, `u64` for 64-bit binaries.
    pub address_of_callbacks: u64,
    /// The size of the zero fill.
    pub size_of_zero_fill: u32,
    /// The characteristics of the TLS.
    ///
    /// Contains one or more bitflags of:
    ///
    /// - [`TLS_CHARACTERISTICS_ALIGN_1BYTES`]
    /// - [`TLS_CHARACTERISTICS_ALIGN_2BYTES`]
    /// - [`TLS_CHARACTERISTICS_ALIGN_4BYTES`]
    /// - [`TLS_CHARACTERISTICS_ALIGN_8BYTES`]
    /// - [`TLS_CHARACTERISTICS_ALIGN_16BYTES`]
    /// - [`TLS_CHARACTERISTICS_ALIGN_32BYTES`]
    /// - [`TLS_CHARACTERISTICS_ALIGN_64BYTES`]
    /// - [`TLS_CHARACTERISTICS_ALIGN_128BYTES`]
    /// - [`TLS_CHARACTERISTICS_ALIGN_256BYTES`]
    /// - [`TLS_CHARACTERISTICS_ALIGN_512BYTES`]
    /// - [`TLS_CHARACTERISTICS_ALIGN_1024BYTES`]
    /// - [`TLS_CHARACTERISTICS_ALIGN_2048BYTES`]
    /// - [`TLS_CHARACTERISTICS_ALIGN_4096BYTES`]
    /// - [`TLS_CHARACTERISTICS_ALIGN_8192BYTES`]
    /// - [`TLS_CHARACTERISTICS_ALIGN_MASK`]
    pub characteristics: u32,
}

/// TLS information.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TlsData<'a> {
    /// TLS directory.
    pub image_tls_directory: ImageTlsDirectory,
    /// Raw data of the TLS.
    pub raw_data: Option<&'a [u8]>,
    /// TLS index.
    pub slot: Option<u32>,
    /// TLS callbacks.
    pub callbacks: Vec<u64>,
}

impl ImageTlsDirectory {
    pub fn parse<T: Sized>(
        bytes: &[u8],
        dd: data_directories::DataDirectory,
        sections: &[section_table::SectionTable],
        file_alignment: u32,
    ) -> error::Result<Self> {
        Self::parse_with_opts::<T>(
            bytes,
            dd,
            sections,
            file_alignment,
            &options::ParseOptions::default(),
        )
    }

    pub fn parse_with_opts<T: Sized>(
        bytes: &[u8],
        dd: data_directories::DataDirectory,
        sections: &[section_table::SectionTable],
        file_alignment: u32,
        opts: &options::ParseOptions,
    ) -> error::Result<Self> {
        let rva = dd.virtual_address as usize;
        let mut offset =
            utils::find_offset(rva, sections, file_alignment, opts).ok_or_else(|| {
                error::Error::Malformed(format!(
                    "Cannot map ImageTlsDirectory rva {:#x} into offset",
                    rva
                ))
            })?;

        let is_64 = core::mem::size_of::<T>() == 8;

        let start_address_of_raw_data = if is_64 {
            bytes.gread_with::<u64>(&mut offset, scroll::LE)?
        } else {
            bytes.gread_with::<u32>(&mut offset, scroll::LE)? as u64
        };
        let end_address_of_raw_data = if is_64 {
            bytes.gread_with::<u64>(&mut offset, scroll::LE)?
        } else {
            bytes.gread_with::<u32>(&mut offset, scroll::LE)? as u64
        };
        let address_of_index = if is_64 {
            bytes.gread_with::<u64>(&mut offset, scroll::LE)?
        } else {
            bytes.gread_with::<u32>(&mut offset, scroll::LE)? as u64
        };
        let address_of_callbacks = if is_64 {
            bytes.gread_with::<u64>(&mut offset, scroll::LE)?
        } else {
            bytes.gread_with::<u32>(&mut offset, scroll::LE)? as u64
        };
        let size_of_zero_fill = bytes.gread_with::<u32>(&mut offset, scroll::LE)?;
        let characteristics = bytes.gread_with::<u32>(&mut offset, scroll::LE)?;

        let itd = Self {
            start_address_of_raw_data,
            end_address_of_raw_data,
            address_of_index,
            address_of_callbacks,
            size_of_zero_fill,
            characteristics,
        };

        Ok(itd)
    }
}

impl<'a> TlsData<'a> {
    pub fn parse<T: Sized>(
        bytes: &'a [u8],
        image_base: usize,
        dd: &data_directories::DataDirectory,
        sections: &[section_table::SectionTable],
        file_alignment: u32,
    ) -> error::Result<Option<Self>> {
        Self::parse_with_opts::<T>(
            bytes,
            image_base,
            dd,
            sections,
            file_alignment,
            &options::ParseOptions::default(),
        )
    }

    pub fn parse_with_opts<T: Sized>(
        bytes: &'a [u8],
        image_base: usize,
        dd: &data_directories::DataDirectory,
        sections: &[section_table::SectionTable],
        file_alignment: u32,
        opts: &options::ParseOptions,
    ) -> error::Result<Option<Self>> {
        let mut raw_data = None;
        let mut slot = None;
        let mut callbacks = Vec::new();

        let is_64 = core::mem::size_of::<T>() == 8;

        let itd =
            ImageTlsDirectory::parse_with_opts::<T>(bytes, *dd, sections, file_alignment, opts)?;

        // Parse the raw data if any
        if itd.end_address_of_raw_data != 0 && itd.start_address_of_raw_data != 0 {
            if itd.start_address_of_raw_data > itd.end_address_of_raw_data {
                return Err(error::Error::Malformed(format!(
                    "tls start_address_of_raw_data ({:#x}) is greater than end_address_of_raw_data ({:#x})",
                    itd.start_address_of_raw_data,
                    itd.end_address_of_raw_data
                )));
            }

            if (itd.start_address_of_raw_data as usize) < image_base {
                return Err(error::Error::Malformed(format!(
                    "tls start_address_of_raw_data ({:#x}) is less than image base ({:#x})",
                    itd.start_address_of_raw_data, image_base
                )));
            }

            // VA to RVA
            let rva = itd.start_address_of_raw_data as usize - image_base;
            let size = itd.end_address_of_raw_data - itd.start_address_of_raw_data;
            let offset =
                utils::find_offset(rva, sections, file_alignment, opts).ok_or_else(|| {
                    error::Error::Malformed(format!(
                        "cannot map tls start_address_of_raw_data rva ({:#x}) into offset",
                        rva
                    ))
                })?;
            raw_data = Some(&bytes[offset..offset + size as usize]);
        }

        // Parse the index if any
        if itd.address_of_index != 0 {
            if (itd.address_of_index as usize) < image_base {
                return Err(error::Error::Malformed(format!(
                    "tls address_of_index ({:#x}) is less than image base ({:#x})",
                    itd.address_of_index, image_base
                )));
            }

            // VA to RVA
            let rva = itd.address_of_index as usize - image_base;
            let offset = utils::find_offset(rva, sections, file_alignment, opts);
            slot = offset.and_then(|x| bytes.pread_with::<u32>(x, scroll::LE).ok());
        }

        // Parse the callbacks if any
        if itd.address_of_callbacks != 0 {
            if (itd.address_of_callbacks as usize) < image_base {
                return Err(error::Error::Malformed(format!(
                    "tls address_of_callbacks ({:#x}) is less than image base ({:#x})",
                    itd.address_of_callbacks, image_base
                )));
            }

            // VA to RVA
            let rva = itd.address_of_callbacks as usize - image_base;
            let offset =
                utils::find_offset(rva, sections, file_alignment, opts).ok_or_else(|| {
                    error::Error::Malformed(format!(
                        "cannot map tls address_of_callbacks rva ({:#x}) into offset",
                        rva
                    ))
                })?;
            let mut i = 0;
            // Read the callbacks until we find a null terminator
            loop {
                let callback: u64 = if is_64 {
                    bytes.pread_with::<u64>(offset + i * 8, scroll::LE)?
                } else {
                    bytes.pread_with::<u32>(offset + i * 4, scroll::LE)? as u64
                };
                if callback == 0 {
                    break;
                }
                if callback < image_base as u64 {
                    return Err(error::Error::Malformed(format!(
                        "tls callback ({:#x}) is less than image base ({:#x})",
                        callback, image_base
                    )));
                }
                // Each callback is an VA so convert it to RVA
                // For x86 compatibility, `usize` is 32-bit, `u64` is 64-bit.
                // Therefore upcast to u64 first, then downcast the whole var to u32.
                let callback_rva = (callback - image_base as u64) as usize;
                // Check if the callback is in the image
                if utils::find_offset(callback_rva, sections, file_alignment, opts).is_none() {
                    return Err(error::Error::Malformed(format!(
                        "cannot map tls callback ({:#x})",
                        callback
                    )));
                }
                callbacks.push(callback);
                i += 1;
            }
        }

        Ok(Some(TlsData {
            image_tls_directory: itd,
            raw_data,
            slot,
            callbacks,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::TLS_CHARACTERISTICS_ALIGN_4BYTES;

    const SPECIAL_IMPORT_FORWARDER_TLS: &[u8] =
        include_bytes!("../../tests/bins/pe/special_import_forwarder_tls.exe.bin");
    const LLD_TLS_SLOT_VIRTONLY_BIN64: &[u8] =
        include_bytes!("../../tests/bins/pe/lld_tls_slot_virtonly.exe.bin");
    const LLD_MALFORMED_TLS_CALLBACKS_BIN64: &[u8] =
        include_bytes!("../../tests/bins/pe/lld_malformed_tls_callbacks_64.exe.bin");
    const LLD_WITH_TLS_BIN64: &[u8] = include_bytes!("../../tests/bins/pe/lld_with_tls_64.exe.bin");
    const LLD_NO_TLS_BIN64: &[u8] = include_bytes!("../../tests/bins/pe/lld_no_tls_64.exe.bin");

    /// Binary without TLS directory
    #[test]
    fn parse_no_tls() {
        let binary = crate::pe::PE::parse(LLD_NO_TLS_BIN64).expect("Unable to parse binary");
        assert_eq!(binary.tls_data.is_none(), true);
    }

    #[test]
    fn parse_with_tls() {
        let binary = crate::pe::PE::parse(LLD_WITH_TLS_BIN64).expect("Unable to parse binary");
        assert_eq!(binary.tls_data.is_some(), true);
        let tls_data = binary.tls_data.unwrap();
        let dir = tls_data.image_tls_directory;

        assert_eq!(tls_data.callbacks, vec![0x140001000]);
        assert_eq!(dir.address_of_callbacks, 0x140001020);

        let raw_data_expect: &[u8] = &[0, 0, 0, 0];
        assert_eq!(
            tls_data
                .raw_data
                .as_ref()
                .map(|x| x.len() == 4)
                .unwrap_or(false),
            true
        );
        assert_eq!(tls_data.raw_data, Some(raw_data_expect));
        assert_eq!(dir.start_address_of_raw_data, 0x140001060);
        assert_eq!(dir.end_address_of_raw_data, 0x140001064);

        assert_eq!(tls_data.slot, Some(0xCCCCCCCC));
        assert_eq!(dir.address_of_index, 0x140001034);

        assert_eq!(dir.size_of_zero_fill, 0x0);
        assert_eq!(dir.characteristics, TLS_CHARACTERISTICS_ALIGN_4BYTES);
    }

    /// Contains two valid callbacks, but null-terminator is (intentionally, for test)
    /// malformed with 8-bytes `08 07 06 05 04 03 02 01` (LE).
    #[test]
    fn parse_malformed_tls_callbacks() {
        let binary = crate::pe::PE::parse(LLD_MALFORMED_TLS_CALLBACKS_BIN64);
        match binary {
            Ok(_) => unreachable!("Malformed TLS callbacks should fail to parse"),
            Err(crate::error::Error::Malformed(msg)) => {
                assert_eq!(msg, "cannot map tls callback (0x807060504030201)");
            }
            Err(err) => unreachable!("Unexpected error: {err:?}"),
        }
    }

    /// Binaries compiled with a valid TLS index may generate an binary that
    /// its TLS directory contains `AddressOfIndex` field that only
    /// present in virtual address when mapped to virtual memory.
    ///
    /// Issue: <https://github.com/m4b/goblin/issues/424>
    #[test]
    fn parse_tls_slot_nonexist_in_raw() {
        let binary =
            crate::pe::PE::parse(LLD_TLS_SLOT_VIRTONLY_BIN64).expect("Unable to parse binary");
        assert_eq!(binary.tls_data.is_some(), true);
        let tls_data = binary.tls_data.unwrap();
        assert_eq!(tls_data.slot, None);
    }

    /// So-called "special import forwarder TLS" means the address of callbacks points
    /// to the VA of FT (first thunk, aka address table) within an associated import descriptor.
    ///
    /// This forwarder allows a exported symbol in the external DLL to be called as main
    /// executables TLS callbacks ealier than DLLs TLS callbacks callouts.
    ///
    /// When the image is mapped to memory for execution:
    ///
    /// 1. Windows loader loads up the depencency specified in the descriptor (`abcd.dll`)
    /// 2. Windows loader resolves import symbol and writes absolute address of import symbol (`abcd.dll!ORDINAL 00001`)
    /// 3. Once entire dependencies are resolved, Windows loader then calls out the chain of TLS callbacks
    ///    specified in the `AddressOfCallbacks` in TLS directory.
    /// 4. The imported symbol (`abcd.dll!ORDINAL 00001`) is called as an TLS callback.
    ///
    /// This executable cannot be created by any combinations of compiler or linker options. Instead, it requires manual or
    /// automated process of modifying artifact binary.
    #[test]
    fn parse_special_import_fowarder_tls() {
        let binary = crate::pe::PE::parse(SPECIAL_IMPORT_FORWARDER_TLS);
        match binary {
            Ok(_) => unreachable!("Special import forwarder TLS should fail to parse"),
            Err(crate::error::Error::Malformed(msg)) => {
                assert_eq!(msg, "cannot map tls callback (0x800000000000c8c6)")
            }
            Err(err) => unreachable!("Unexpected error: {err:?}"),
        }
    }
}
