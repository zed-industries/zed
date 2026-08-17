use {
    super::{
        auxv::AuxvDumpInfo, mem_reader::CopyFromProcessError, minidump_writer::MinidumpWriter,
        serializers::*,
    },
    crate::{
        mem_writer::{
            write_string_to_location, Buffer, MemoryArrayWriter, MemoryWriter, MemoryWriterError,
        },
        minidump_format::*,
    },
};

type Result<T> = std::result::Result<T, SectionDsoDebugError>;

cfg_if::cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        use goblin::elf::program_header::program_header32::SIZEOF_PHDR;
    } else if #[cfg(target_pointer_width = "64")] {
        use goblin::elf::program_header::program_header64::SIZEOF_PHDR;
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(target_pointer_width = "64", target_arch = "arm"))] {
        type ElfAddr = u64;
    } else if #[cfg(all(target_pointer_width = "64", not(target_arch = "arm")))] {
        type ElfAddr = libc::Elf64_Addr;
    } else if #[cfg(all(target_pointer_width = "32", target_arch = "arm"))] {
        type ElfAddr = u32;
    } else if #[cfg(all(target_pointer_width = "32", not(target_arch = "arm")))] {
        type ElfAddr = libc::Elf32_Addr;
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum SectionDsoDebugError {
    #[error("Failed to write to memory")]
    MemoryWriterError(#[from] MemoryWriterError),
    #[error("Could not find: {0}")]
    CouldNotFind(&'static str),
    #[error("Failed to copy memory from process")]
    CopyFromProcessError(#[from] CopyFromProcessError),
    #[error("Failed to copy memory from process")]
    FromUTF8Error(
        #[from]
        #[serde(serialize_with = "serialize_from_utf8_error")]
        std::string::FromUtf8Error,
    ),
}

// COPY from <link.h>
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct LinkMap {
    /* These first few members are part of the protocol with the debugger.
    This is the same format used in SVR4.  */
    l_addr: ElfAddr, /* Difference between the address in the ELF
                     file and the addresses in memory.  */
    l_name: usize, /* Absolute file name object was found in. WAS: `char*`  */
    l_ld: usize,   /* Dynamic section of the shared object.  WAS: `ElfW(Dyn) *` */
    l_next: usize, /* Chain of loaded objects. WAS: `struct link_map *` */
    l_prev: usize, /* Chain of loaded objects. WAS: `struct link_map *` */
}

// COPY from <link.h>
/// This state value describes the mapping change taking place when
/// the `r_brk' address is called.
#[derive(Debug, Clone, Default)]
#[allow(non_camel_case_types, unused)]
#[repr(C)]
enum RState {
    /// Mapping change is complete.
    #[default]
    RT_CONSISTENT,
    /// Beginning to add a new object.
    RT_ADD,
    /// Beginning to remove an object mapping.
    RT_DELETE,
}

// COPY from <link.h>
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct RDebug {
    r_version: libc::c_int, /* Version number for this protocol.  */
    r_map: usize,           /* Head of the chain of loaded objects. WAS: `struct link_map *` */

    /* This is the address of a function internal to the run-time linker,
    that will always be called when the linker begins to map in a
    library or unmap it, and again when the mapping change is complete.
    The debugger can set a breakpoint at this address if it wants to
    notice shared object mapping changes.  */
    r_brk: ElfAddr,
    r_state: RState,
    r_ldbase: ElfAddr, /* Base address the linker is loaded at.  */
}

pub fn write_dso_debug_stream(
    buffer: &mut Buffer,
    blamed_thread: i32,
    auxv: &AuxvDumpInfo,
) -> Result<MDRawDirectory> {
    let phnum_max =
        auxv.get_program_header_count()
            .ok_or(SectionDsoDebugError::CouldNotFind("AT_PHNUM in auxv"))? as usize;
    let phdr = auxv
        .get_program_header_address()
        .ok_or(SectionDsoDebugError::CouldNotFind("AT_PHDR in auxv"))? as usize;

    let ph = MinidumpWriter::copy_from_process(blamed_thread, phdr, SIZEOF_PHDR * phnum_max)?;
    let program_headers;
    #[cfg(target_pointer_width = "64")]
    {
        program_headers = goblin::elf::program_header::program_header64::ProgramHeader::from_bytes(
            &ph, phnum_max,
        );
    }
    #[cfg(target_pointer_width = "32")]
    {
        program_headers = goblin::elf::program_header::program_header32::ProgramHeader::from_bytes(
            &ph, phnum_max,
        );
    };

    // Assume the program base is at the beginning of the same page as the PHDR
    let mut base = phdr & !0xfff;
    let mut dyn_addr = 0;
    // Search for the program PT_DYNAMIC segment
    for ph in program_headers {
        // Adjust base address with the virtual address of the PT_LOAD segment
        // corresponding to offset 0
        if ph.p_type == goblin::elf::program_header::PT_LOAD && ph.p_offset == 0 {
            base -= ph.p_vaddr as usize;
        }
        if ph.p_type == goblin::elf::program_header::PT_DYNAMIC {
            dyn_addr = ph.p_vaddr;
        }
    }

    if dyn_addr == 0 {
        return Err(SectionDsoDebugError::CouldNotFind(
            "dyn_addr in program headers",
        ));
    }

    dyn_addr += base as ElfAddr;

    let dyn_size = std::mem::size_of::<goblin::elf::Dyn>();
    let mut r_debug = 0usize;
    let mut dynamic_length = 0usize;

    // The dynamic linker makes information available that helps gdb find all
    // DSOs loaded into the program. If this information is indeed available,
    // dump it to a MD_LINUX_DSO_DEBUG stream.
    loop {
        let dyn_data = MinidumpWriter::copy_from_process(
            blamed_thread,
            dyn_addr as usize + dynamic_length,
            dyn_size,
        )?;
        dynamic_length += dyn_size;

        // goblin::elf::Dyn doesn't have padding bytes
        let (head, body, _tail) = unsafe { dyn_data.align_to::<goblin::elf::Dyn>() };
        assert!(head.is_empty(), "Data was not aligned");
        let dyn_struct = &body[0];

        let debug_tag = goblin::elf::dynamic::DT_DEBUG;
        if dyn_struct.d_tag == debug_tag {
            r_debug = dyn_struct.d_val as usize;
        } else if dyn_struct.d_tag == goblin::elf::dynamic::DT_NULL {
            break;
        }
    }

    // The "r_map" field of that r_debug struct contains a linked list of all
    // loaded DSOs.
    // Our list of DSOs potentially is different from the ones in the crashing
    // process. So, we have to be careful to never dereference pointers
    // directly. Instead, we use CopyFromProcess() everywhere.
    // See <link.h> for a more detailed discussion of the how the dynamic
    // loader communicates with debuggers.

    let debug_entry_data =
        MinidumpWriter::copy_from_process(blamed_thread, r_debug, std::mem::size_of::<RDebug>())?;

    // goblin::elf::Dyn doesn't have padding bytes
    let (head, body, _tail) = unsafe { debug_entry_data.align_to::<RDebug>() };
    assert!(head.is_empty(), "Data was not aligned");
    let debug_entry = &body[0];

    // Count the number of loaded DSOs
    let mut dso_vec = Vec::new();
    let mut curr_map = debug_entry.r_map;
    while curr_map != 0 {
        let link_map_data = MinidumpWriter::copy_from_process(
            blamed_thread,
            curr_map,
            std::mem::size_of::<LinkMap>(),
        )?;

        // LinkMap is repr(C) and doesn't have padding bytes, so this should be safe
        let (head, body, _tail) = unsafe { link_map_data.align_to::<LinkMap>() };
        assert!(head.is_empty(), "Data was not aligned");
        let map = &body[0];

        curr_map = map.l_next;
        dso_vec.push(map.clone());
    }

    let mut linkmap_rva = u32::MAX;
    if !dso_vec.is_empty() {
        // If we have at least one DSO, create an array of MDRawLinkMap
        // entries in the minidump file.
        let mut linkmap = MemoryArrayWriter::<MDRawLinkMap>::alloc_array(buffer, dso_vec.len())?;
        linkmap_rva = linkmap.location().rva;

        // Iterate over DSOs and write their information to mini dump
        for (idx, map) in dso_vec.iter().enumerate() {
            let mut filename = String::new();
            if map.l_name > 0 {
                let filename_data =
                    MinidumpWriter::copy_from_process(blamed_thread, map.l_name, 256)?;

                // C - string is NULL-terminated
                if let Some(name) = filename_data.splitn(2, |x| *x == b'\0').next() {
                    filename = String::from_utf8(name.to_vec())?;
                }
            }
            let location = write_string_to_location(buffer, &filename)?;
            let entry = MDRawLinkMap {
                addr: map.l_addr,
                name: location.rva,
                ld: map.l_ld as ElfAddr,
            };

            linkmap.set_value_at(buffer, entry, idx)?;
        }
    }

    // Write MD_LINUX_DSO_DEBUG record
    let debug = MDRawDebug {
        version: debug_entry.r_version as u32,
        map: linkmap_rva,
        dso_count: dso_vec.len() as u32,
        brk: debug_entry.r_brk,
        ldbase: debug_entry.r_ldbase,
        dynamic: dyn_addr,
    };
    let debug_loc = MemoryWriter::<MDRawDebug>::alloc_with_val(buffer, debug)?;

    let mut dirent = MDRawDirectory {
        stream_type: MDStreamType::LinuxDsoDebug as u32,
        location: debug_loc.location(),
    };

    dirent.location.data_size += dynamic_length as u32;
    let dso_debug_data =
        MinidumpWriter::copy_from_process(blamed_thread, dyn_addr as usize, dynamic_length)?;
    MemoryArrayWriter::write_bytes(buffer, &dso_debug_data);

    Ok(dirent)
}
