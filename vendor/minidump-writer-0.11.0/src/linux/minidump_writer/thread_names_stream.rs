use super::*;

#[derive(Debug, Error, serde::Serialize)]
pub enum SectionThreadNamesError {
    #[error("Failed integer conversion")]
    TryFromIntError(
        #[from]
        #[serde(skip)]
        std::num::TryFromIntError,
    ),
    #[error("Failed to write to memory")]
    MemoryWriterError(#[from] MemoryWriterError),
    #[error("Failed to write to memory buffer")]
    IOError(
        #[from]
        #[serde(serialize_with = "serialize_io_error")]
        std::io::Error,
    ),
}

impl MinidumpWriter {
    pub fn write_thread_names_stream(
        &self,
        buffer: &mut DumpBuf,
    ) -> Result<MDRawDirectory, SectionThreadNamesError> {
        // Only count threads that have a name
        let num_threads = self.threads.iter().filter(|t| t.name.is_some()).count();
        // Memory looks like this:
        // <num_threads><thread_1><thread_2>...

        let list_header = MemoryWriter::<u32>::alloc_with_val(buffer, num_threads as u32)?;

        let mut dirent = MDRawDirectory {
            stream_type: MDStreamType::ThreadNamesStream as u32,
            location: list_header.location(),
        };

        let mut thread_list =
            MemoryArrayWriter::<MDRawThreadName>::alloc_array(buffer, num_threads)?;
        dirent.location.data_size += thread_list.location().data_size;

        for (idx, item) in self.threads.iter().enumerate() {
            if let Some(name) = &item.name {
                let pos = write_string_to_location(buffer, name)?;
                let thread = MDRawThreadName {
                    thread_id: item.tid.try_into()?,
                    thread_name_rva: pos.rva.into(),
                };
                thread_list.set_value_at(buffer, thread, idx)?;
            }
        }
        Ok(dirent)
    }
}
