// Derived from code in LLVM, which is:
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Default implementation of [crate::ObjectReader] that uses the `object` crate.

use std::{io, mem::offset_of};

use object::{Object, ObjectSymbol, pe::ImportObjectHeader, xcoff};

use crate::coff::is_any_arm64;
use crate::coff_import_file;

fn is_archive_symbol(sym: &object::read::Symbol<'_, '_>) -> bool {
    // FIXME Use a better equivalent of LLVM's SymbolRef::SF_FormatSpecific
    if sym.kind() == object::SymbolKind::File || sym.kind() == object::SymbolKind::Section {
        return false;
    }
    if !sym.is_global() {
        return false;
    }
    if sym.is_undefined() {
        return false;
    }
    true
}

pub fn get_native_object_symbols(
    buf: &[u8],
    f: &mut dyn FnMut(&[u8]) -> io::Result<()>,
) -> io::Result<bool> {
    // FIXME match what LLVM does

    match object::File::parse(buf) {
        Ok(file) => {
            for sym in file.symbols() {
                if !is_archive_symbol(&sym) {
                    continue;
                }
                f(sym.name_bytes().expect("FIXME"))?;
            }
            Ok(true)
        }
        Err(_) => {
            let mut offset = 0;
            // Try to handle this as a COFF import library.
            if ImportObjectHeader::parse(buf, &mut offset).is_ok() {
                coff_import_file::get_short_import_symbol(buf, f).or(Ok(false))
            } else {
                Ok(false)
            }
        }
    }
}

pub fn is_ec_object(obj: &[u8]) -> bool {
    match object::FileKind::parse(obj) {
        Ok(object::FileKind::Coff) => {
            u16::from_le_bytes([obj[0], obj[1]]) != object::pe::IMAGE_FILE_MACHINE_ARM64
        }
        Ok(object::FileKind::CoffImport) => {
            // COFF Import Header is:
            // sig1: u16
            // sig2: u16
            // version: u16
            // machine: u16
            u16::from_le_bytes([obj[6], obj[7]]) != object::pe::IMAGE_FILE_MACHINE_ARM64
        }
        _ => false,
    }
}

pub fn is_any_arm64_coff(obj: &[u8]) -> bool {
    match object::FileKind::parse(obj) {
        Ok(object::FileKind::Coff) => u16::from_le_bytes([obj[0], obj[1]])
            .try_into()
            .is_ok_and(is_any_arm64),
        Ok(object::FileKind::CoffImport) => {
            // COFF Import Header is:
            // sig1: u16
            // sig2: u16
            // version: u16
            // machine: u16
            u16::from_le_bytes([obj[6], obj[7]])
                .try_into()
                .is_ok_and(is_any_arm64)
        }
        _ => false,
    }
}

pub fn is_64_bit_symbolic_file(obj: &[u8]) -> bool {
    object::FileKind::parse(obj).is_ok_and(|kind| match kind {
        object::FileKind::Elf64
        | object::FileKind::MachO64
        | object::FileKind::Pe64
        | object::FileKind::Xcoff64
        | object::FileKind::MachOFat64 => true,
        object::FileKind::Elf32
        | object::FileKind::MachO32
        | object::FileKind::Pe32
        | object::FileKind::Xcoff32
        | object::FileKind::MachOFat32
        | object::FileKind::Coff
        | object::FileKind::CoffBig
        | object::FileKind::CoffImport => false,
        _ => panic!("Unexpected file kind"),
    })
}

// Log2 of PAGESIZE(4096) on an AIX system.
const LOG2_OF_AIXPAGE_SIZE: u32 = 12;

// In the AIX big archive format, since the data content follows the member file
// name, if the name ends on an odd byte, an extra byte will be added for
// padding. This ensures that the data within the member file starts at an even
// byte.
const MIN_BIG_ARCHIVE_MEM_DATA_ALIGN: u32 = 2;

fn get_aux_max_alignment<AuxiliaryHeader: object::read::xcoff::AuxHeader>(
    aux_header_size: u16,
    aux_header: Option<&AuxiliaryHeader>,
    log_2_of_max_align: u32,
    offset_of_modtype: u16,
) -> u32 {
    // If the member doesn't have an auxiliary header, it isn't a loadable object
    // and so it just needs aligning at the minimum value.
    let Some(aux_header) = aux_header else {
        return MIN_BIG_ARCHIVE_MEM_DATA_ALIGN;
    };

    // If the auxiliary header does not have both MaxAlignOfData and
    // MaxAlignOfText field, it is not a loadable shared object file, so align at
    // the minimum value. The 'ModuleType' member is located right after
    // 'MaxAlignOfData' in the AuxiliaryHeader.
    if aux_header_size < offset_of_modtype {
        return MIN_BIG_ARCHIVE_MEM_DATA_ALIGN;
    }

    // If the XCOFF object file does not have a loader section, it is not
    // loadable, so align at the minimum value.
    if aux_header.o_snloader() == 0 {
        return MIN_BIG_ARCHIVE_MEM_DATA_ALIGN;
    }

    // The content of the loadable member file needs to be aligned at MAX(maximum
    // alignment of .text, maximum alignment of .data) if there are both fields.
    // If the desired alignment is > PAGESIZE, 32-bit members are aligned on a
    // word boundary, while 64-bit members are aligned on a PAGESIZE(2^12=4096)
    // boundary.
    let log_2_of_align = u32::from(std::cmp::max(
        aux_header.o_algntext(),
        aux_header.o_algndata(),
    ));
    1 << (if log_2_of_align > LOG2_OF_AIXPAGE_SIZE {
        log_2_of_max_align
    } else {
        log_2_of_align
    })
}

// AIX big archives may contain shared object members. The AIX OS requires these
// members to be aligned if they are 64-bit and recommends it for 32-bit
// members. This ensures that when these members are loaded they are aligned in
// memory.
pub fn get_member_alignment(obj: &[u8]) -> u32 {
    use object::read::xcoff::FileHeader;

    // If the desired alignment is > PAGESIZE, 32-bit members are aligned on a
    // word boundary, while 64-bit members are aligned on a PAGESIZE boundary.
    match object::FileKind::parse(obj) {
        Ok(object::FileKind::Xcoff64) => {
            let mut offset = 0;
            let Ok(header) = xcoff::FileHeader64::parse(obj, &mut offset) else {
                return MIN_BIG_ARCHIVE_MEM_DATA_ALIGN;
            };
            let Ok(aux_header) = header.aux_header(obj, &mut offset) else {
                return MIN_BIG_ARCHIVE_MEM_DATA_ALIGN;
            };
            get_aux_max_alignment(
                header.f_opthdr(),
                aux_header,
                LOG2_OF_AIXPAGE_SIZE,
                offset_of!(object::xcoff::AuxHeader64, o_modtype)
                    .try_into()
                    .unwrap(),
            )
        }
        Ok(object::FileKind::Xcoff32) => {
            let mut offset = 0;
            let Ok(header) = object::xcoff::FileHeader32::parse(obj, &mut offset) else {
                return MIN_BIG_ARCHIVE_MEM_DATA_ALIGN;
            };
            let Ok(aux_header) = header.aux_header(obj, &mut offset) else {
                return MIN_BIG_ARCHIVE_MEM_DATA_ALIGN;
            };
            get_aux_max_alignment(
                header.f_opthdr(),
                aux_header,
                2,
                offset_of!(object::xcoff::AuxHeader32, o_modtype)
                    .try_into()
                    .unwrap(),
            )
        }
        _ => MIN_BIG_ARCHIVE_MEM_DATA_ALIGN,
    }
}
