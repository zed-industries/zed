use {
    super::*,
    minidump_common::format::{MemoryProtection, MemoryState, MemoryType},
    procfs_core::{process::MMPermissions, FromRead},
};

#[derive(Debug, Error, serde::Serialize)]
pub enum SectionMemInfoListError {
    #[error("Failed to write to memory")]
    MemoryWriterError(#[from] MemoryWriterError),
    #[error("Failed to read from procfs")]
    ProcfsError(
        #[from]
        #[serde(serialize_with = "serialize_proc_error")]
        procfs_core::ProcError,
    ),
}

impl MinidumpWriter {
    /// Write a MemoryInfoListStream using information from procfs.
    pub fn write_memory_info_list_stream(
        &mut self,
        buffer: &mut DumpBuf,
    ) -> Result<MDRawDirectory, SectionMemInfoListError> {
        let maps = procfs_core::process::MemoryMaps::from_file(std::path::PathBuf::from(format!(
            "/proc/{}/maps",
            self.blamed_thread
        )))?;

        let list_header = MemoryWriter::alloc_with_val(
            buffer,
            MDMemoryInfoList {
                size_of_header: std::mem::size_of::<MDMemoryInfoList>() as u32,
                size_of_entry: std::mem::size_of::<MDMemoryInfo>() as u32,
                number_of_entries: maps.len() as u64,
            },
        )?;

        let mut dirent = MDRawDirectory {
            stream_type: MDStreamType::MemoryInfoListStream as u32,
            location: list_header.location(),
        };

        let block_list = MemoryArrayWriter::<MDMemoryInfo>::alloc_from_iter(
            buffer,
            maps.iter().map(|mm| MDMemoryInfo {
                base_address: mm.address.0,
                allocation_base: mm.address.0,
                allocation_protection: get_memory_protection(mm.perms).bits(),
                __alignment1: 0,
                region_size: mm.address.1 - mm.address.0,
                state: MemoryState::MEM_COMMIT.bits(),
                protection: get_memory_protection(mm.perms).bits(),
                _type: if mm.perms.contains(MMPermissions::PRIVATE) {
                    MemoryType::MEM_PRIVATE
                } else {
                    MemoryType::MEM_MAPPED
                }
                .bits(),
                __alignment2: 0,
            }),
        )?;

        dirent.location.data_size += block_list.location().data_size;

        Ok(dirent)
    }
}
fn get_memory_protection(permissions: MMPermissions) -> MemoryProtection {
    let read = permissions.contains(MMPermissions::READ);
    let write = permissions.contains(MMPermissions::WRITE);
    let exec = permissions.contains(MMPermissions::EXECUTE);
    match (read, write, exec) {
        (false, false, false) => MemoryProtection::PAGE_NOACCESS,
        (false, false, true) => MemoryProtection::PAGE_EXECUTE,
        (true, false, false) => MemoryProtection::PAGE_READONLY,
        (true, false, true) => MemoryProtection::PAGE_EXECUTE_READ,
        // No support for write-only
        (true | false, true, false) => MemoryProtection::PAGE_READWRITE,
        // No support for execute+write-only
        (true | false, true, true) => MemoryProtection::PAGE_EXECUTE_READWRITE,
    }
}
