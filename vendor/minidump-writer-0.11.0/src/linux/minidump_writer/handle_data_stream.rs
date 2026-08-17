use {
    super::*,
    crate::mem_writer::MemoryWriter,
    std::{
        ffi::{CString, OsString},
        fs::{self, DirEntry},
        mem::{self},
        os::unix::prelude::OsStrExt,
        path::{Path, PathBuf},
    },
};

fn file_stat(path: &Path) -> Option<libc::stat> {
    let c_path = CString::new(path.as_os_str().as_bytes()).ok()?;
    let mut stat = unsafe { std::mem::zeroed::<libc::stat>() };
    let result = unsafe { libc::stat(c_path.as_ptr(), &mut stat) };

    if result == 0 {
        Some(stat)
    } else {
        None
    }
}

fn direntry_to_descriptor(buffer: &mut DumpBuf, entry: &DirEntry) -> Option<MDRawHandleDescriptor> {
    let handle = filename_to_fd(&entry.file_name())?;
    let realpath = fs::read_link(entry.path()).ok()?;
    let path_rva = write_string_to_location(buffer, realpath.to_string_lossy().as_ref()).ok()?;
    let stat = file_stat(&entry.path())?;

    // TODO: We store the contents of `st_mode` into the `attributes` field, but
    // we could also store a human-readable string of the file type inside
    // `type_name_rva`. We might move this missing information (and
    // more) inside a custom `MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE` blob.
    // That would make this conversion loss-less.
    Some(MDRawHandleDescriptor {
        handle,
        type_name_rva: 0,
        object_name_rva: path_rva.rva,
        attributes: stat.st_mode,
        granted_access: 0,
        handle_count: 0,
        pointer_count: 0,
    })
}

fn filename_to_fd(filename: &OsString) -> Option<u64> {
    let filename = filename.to_string_lossy();
    filename.parse::<u64>().ok()
}

#[derive(Debug, Error, serde::Serialize)]
pub enum SectionHandleDataStreamError {
    #[error("Failed to access file")]
    IOError(
        #[from]
        #[serde(serialize_with = "serialize_io_error")]
        std::io::Error,
    ),
    #[error("Failed to write to memory")]
    MemoryWriterError(#[from] MemoryWriterError),
    #[error("Failed integer conversion")]
    TryFromIntError(
        #[from]
        #[serde(skip)]
        std::num::TryFromIntError,
    ),
}

impl MinidumpWriter {
    pub fn write_handle_data_stream(
        &mut self,
        buffer: &mut DumpBuf,
    ) -> Result<MDRawDirectory, SectionHandleDataStreamError> {
        let proc_fd_path = PathBuf::from(format!("/proc/{}/fd", self.process_id));
        let proc_fd_iter = fs::read_dir(proc_fd_path)?;
        let descriptors: Vec<_> = proc_fd_iter
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| direntry_to_descriptor(buffer, &entry))
            .collect();
        let number_of_descriptors = descriptors.len() as u32;

        let stream_header = MemoryWriter::<MDRawHandleDataStream>::alloc_with_val(
            buffer,
            MDRawHandleDataStream {
                size_of_header: mem::size_of::<MDRawHandleDataStream>() as u32,
                size_of_descriptor: mem::size_of::<MDRawHandleDescriptor>() as u32,
                number_of_descriptors,
                reserved: 0,
            },
        )?;

        let mut dirent = MDRawDirectory {
            stream_type: MDStreamType::HandleDataStream as u32,
            location: stream_header.location(),
        };

        let descriptor_list =
            MemoryArrayWriter::<MDRawHandleDescriptor>::alloc_from_iter(buffer, descriptors)?;

        dirent.location.data_size += descriptor_list.location().data_size;
        Ok(dirent)
    }
}
