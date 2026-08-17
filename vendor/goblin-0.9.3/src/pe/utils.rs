use crate::error;
use alloc::string::ToString;
use alloc::vec::Vec;
use scroll::Pread;

use super::options;
use super::section_table;

use crate::pe::data_directories::DataDirectory;
use core::cmp;

use log::debug;

pub fn is_in_range(rva: usize, r1: usize, r2: usize) -> bool {
    r1 <= rva && rva < r2
}

// reference: Peter Ferrie. Reliable algorithm to extract overlay of a PE. https://bit.ly/2vBX2bR
#[inline]
fn aligned_pointer_to_raw_data(pointer_to_raw_data: usize) -> usize {
    const PHYSICAL_ALIGN: usize = 0x1ff;
    pointer_to_raw_data & !PHYSICAL_ALIGN
}

#[inline]
fn section_read_size(section: &section_table::SectionTable, file_alignment: u32) -> usize {
    fn round_size(size: usize) -> usize {
        const PAGE_MASK: usize = 0xfff;
        (size + PAGE_MASK) & !PAGE_MASK
    }

    // Paraphrased from https://reverseengineering.stackexchange.com/a/4326 (by Peter Ferrie).
    //
    // Handles the corner cases such as mis-aligned pointers (round down) and sizes (round up)
    // Further rounding corner cases:
    // - the physical pointer should be rounded down to a multiple of 512, regardless of the value in the header
    // - the read size is rounded up by using a combination of the file alignment and 4kb
    // - the virtual size is always rounded up to a multiple of 4kb, regardless of the value in the header.
    //
    // Reference C implementation:
    //
    // long pointerToRaw = section.get(POINTER_TO_RAW_DATA);
    // long alignedpointerToRaw = pointerToRaw & ~0x1ff;
    // long sizeOfRaw = section.get(SIZE_OF_RAW_DATA);
    // long readsize = ((pointerToRaw + sizeOfRaw) + filealign - 1) & ~(filealign - 1)) - alignedpointerToRaw;
    // readsize = min(readsize, (sizeOfRaw + 0xfff) & ~0xfff);
    // long virtsize = section.get(VIRTUAL_SIZE);
    //
    // if (virtsize)
    // {
    //     readsize = min(readsize, (virtsize + 0xfff) & ~0xfff);
    // }

    let file_alignment = file_alignment as usize;
    let size_of_raw_data = section.size_of_raw_data as usize;
    let virtual_size = section.virtual_size as usize;
    let read_size = {
        let read_size =
            ((section.pointer_to_raw_data as usize + size_of_raw_data + file_alignment - 1)
                & !(file_alignment - 1))
                - aligned_pointer_to_raw_data(section.pointer_to_raw_data as usize);
        cmp::min(read_size, round_size(size_of_raw_data))
    };

    if virtual_size == 0 {
        read_size
    } else if read_size == 0 {
        virtual_size
    } else {
        cmp::min(read_size, round_size(virtual_size))
    }
}

fn rva2offset(rva: usize, section: &section_table::SectionTable) -> usize {
    (rva - section.virtual_address as usize)
        + aligned_pointer_to_raw_data(section.pointer_to_raw_data as usize)
}

fn is_in_section(rva: usize, section: &section_table::SectionTable, file_alignment: u32) -> bool {
    let section_rva = section.virtual_address as usize;
    is_in_range(
        rva,
        section_rva,
        section_rva + section_read_size(section, file_alignment),
    )
}

pub fn find_offset(
    rva: usize,
    sections: &[section_table::SectionTable],
    file_alignment: u32,
    opts: &options::ParseOptions,
) -> Option<usize> {
    if opts.resolve_rva {
        if file_alignment == 0 || file_alignment & (file_alignment - 1) != 0 {
            return None;
        }
        for (i, section) in sections.iter().enumerate() {
            debug!(
                "Checking {} for {:#x} ∈ {:#x}..{:#x}",
                section.name().unwrap_or(""),
                rva,
                section.virtual_address,
                section.virtual_address + section.virtual_size
            );
            if is_in_section(rva, &section, file_alignment) {
                let offset = rva2offset(rva, &section);
                debug!(
                    "Found in section {}({}), remapped into offset {:#x}",
                    section.name().unwrap_or(""),
                    i,
                    offset
                );
                return Some(offset);
            }
        }
        None
    } else {
        Some(rva)
    }
}

pub fn find_offset_or(
    rva: usize,
    sections: &[section_table::SectionTable],
    file_alignment: u32,
    opts: &options::ParseOptions,
    msg: &str,
) -> error::Result<usize> {
    find_offset(rva, sections, file_alignment, opts)
        .ok_or_else(|| error::Error::Malformed(msg.to_string()))
}

pub fn try_name<'a>(
    bytes: &'a [u8],
    rva: usize,
    sections: &[section_table::SectionTable],
    file_alignment: u32,
    opts: &options::ParseOptions,
) -> error::Result<&'a str> {
    match find_offset(rva, sections, file_alignment, opts) {
        Some(offset) => Ok(bytes.pread::<&str>(offset)?),
        None => Err(error::Error::Malformed(format!(
            "Cannot find name from rva {:#x} in sections: {:?}",
            rva, sections
        ))),
    }
}

pub fn get_data<'a, T>(
    bytes: &'a [u8],
    sections: &[section_table::SectionTable],
    directory: DataDirectory,
    file_alignment: u32,
) -> error::Result<T>
where
    T: scroll::ctx::TryFromCtx<'a, scroll::Endian, Error = scroll::Error>,
{
    get_data_with_opts(
        bytes,
        sections,
        directory,
        file_alignment,
        &options::ParseOptions::default(),
    )
}

pub fn get_data_with_opts<'a, T>(
    bytes: &'a [u8],
    sections: &[section_table::SectionTable],
    directory: DataDirectory,
    file_alignment: u32,
    opts: &options::ParseOptions,
) -> error::Result<T>
where
    T: scroll::ctx::TryFromCtx<'a, scroll::Endian, Error = scroll::Error>,
{
    let rva = directory.virtual_address as usize;
    let offset = find_offset(rva, sections, file_alignment, opts)
        .ok_or_else(|| error::Error::Malformed(directory.virtual_address.to_string()))?;
    let result: T = bytes.pread_with(offset, scroll::LE)?;
    Ok(result)
}

pub(crate) fn pad(length: usize, alignment: Option<usize>) -> Option<Vec<u8>> {
    match alignment {
        Some(alignment) => {
            let overhang = length % alignment;
            if overhang != 0 {
                let repeat = alignment - overhang;
                Some(vec![0u8; repeat])
            } else {
                None
            }
        }
        None => None,
    }
}
