use super::*;

#[derive(Debug, Error, serde::Serialize)]
pub enum SectionAppMemoryError {
    #[error("Failed to copy memory from process")]
    CopyFromProcessError(#[from] CopyFromProcessError),
    #[error("Failed to write to memory")]
    MemoryWriterError(#[from] MemoryWriterError),
}

impl MinidumpWriter {
    /// Write application-provided memory regions.
    pub fn write_app_memory(&mut self, buffer: &mut DumpBuf) -> Result<(), SectionAppMemoryError> {
        let blamed_thread = self.blamed_thread;
        for app_memory in &self.app_memory {
            let data_copy =
                Self::copy_from_process(blamed_thread, app_memory.ptr, app_memory.length)?;

            let section = MemoryArrayWriter::write_bytes(buffer, &data_copy);
            let desc = MDMemoryDescriptor {
                start_of_memory_range: app_memory.ptr as u64,
                memory: section.location(),
            };
            self.memory_blocks.push(desc);
        }
        Ok(())
    }
}
