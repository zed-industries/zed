use {
    crate::{
        mem_writer::{Buffer, MemoryArrayWriter, MemoryWriterError},
        minidump_format::MDRawDirectory,
        serializers::*,
    },
    std::io::{Error, Seek, Write},
};

pub type DumpBuf = Buffer;

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum FileWriterError {
    #[error("IO error")]
    IOError(
        #[from]
        #[serde(serialize_with = "serialize_io_error")]
        Error,
    ),
    #[error("Failed to write to memory")]
    MemoryWriterError(#[from] MemoryWriterError),
}

/// Utility that wraps writing minidump directory entries to an I/O stream, generally
/// a [`std::fs::File`].
#[derive(Debug)]
pub struct DirSection<'a, W>
where
    W: Write + Seek,
{
    curr_idx: usize,
    section: MemoryArrayWriter<MDRawDirectory>,
    /// If we have to append to some file, we have to know where we currently are
    destination_start_offset: u64,
    destination: &'a mut W,
    last_position_written_to_file: u64,
}

impl<'a, W> DirSection<'a, W>
where
    W: Write + Seek,
{
    pub fn new(
        buffer: &mut DumpBuf,
        index_length: u32,
        destination: &'a mut W,
    ) -> std::result::Result<Self, FileWriterError> {
        let dir_section =
            MemoryArrayWriter::<MDRawDirectory>::alloc_array(buffer, index_length as usize)?;

        Ok(Self {
            curr_idx: 0,
            section: dir_section,
            destination_start_offset: destination.stream_position()?,
            destination,
            last_position_written_to_file: 0,
        })
    }

    #[inline]
    pub fn position(&self) -> u32 {
        self.section.position
    }

    pub fn dump_dir_entry(
        &mut self,
        buffer: &mut DumpBuf,
        dirent: MDRawDirectory,
    ) -> std::result::Result<(), FileWriterError> {
        self.section.set_value_at(buffer, dirent, self.curr_idx)?;

        // Now write it to file

        // First get all the positions
        let curr_file_pos = self.destination.stream_position()?;
        let idx_pos = self.section.location_of_index(self.curr_idx);
        self.curr_idx += 1;

        self.destination.seek(std::io::SeekFrom::Start(
            self.destination_start_offset + idx_pos.rva as u64,
        ))?;
        let start = idx_pos.rva as usize;
        let end = (idx_pos.rva + idx_pos.data_size) as usize;
        self.destination.write_all(&buffer[start..end])?;

        // Reset file-position
        self.destination
            .seek(std::io::SeekFrom::Start(curr_file_pos))?;

        Ok(())
    }

    /// Writes 2 things to file:
    /// 1. The given dirent into the dir section in the header (if any is given)
    /// 2. Everything in the in-memory buffer that was added since the last call to this function
    pub fn write_to_file(
        &mut self,
        buffer: &mut DumpBuf,
        dirent: Option<MDRawDirectory>,
    ) -> std::result::Result<(), FileWriterError> {
        if let Some(dirent) = dirent {
            self.dump_dir_entry(buffer, dirent)?;
        }

        let start_pos = self.last_position_written_to_file as usize;
        self.destination.write_all(&buffer[start_pos..])?;
        self.last_position_written_to_file = buffer.position();
        Ok(())
    }
}
