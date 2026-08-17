use {
    super::*,
    format::{BreakpadInfoValid, MINIDUMP_BREAKPAD_INFO as BreakpadInfo},
};

impl MinidumpWriter {
    /// Writes the [`BreakpadInfo`] stream.
    ///
    /// For MacOS the primary use of this stream is to differentiate between
    /// the thread that actually raised an exception, and the thread on which
    /// the exception port was listening, so that the exception port (handler)
    /// thread can be deprioritized/ignored when analyzing the minidump.
    pub(crate) fn write_breakpad_info(
        &mut self,
        buffer: &mut DumpBuf,
        _dumper: &TaskDumper,
    ) -> Result<MDRawDirectory, WriterError> {
        let bp_section = MemoryWriter::<BreakpadInfo>::alloc_with_val(
            buffer,
            BreakpadInfo {
                validity: BreakpadInfoValid::DumpThreadId.bits()
                    | BreakpadInfoValid::RequestingThreadId.bits(),
                // The thread where the exception port handled the exception, might
                // be useful to ignore/deprioritize when processing the minidump
                dump_thread_id: self.handler_thread,
                // The actual thread where the exception was thrown
                requesting_thread_id: self.crash_context.as_ref().map(|cc| cc.thread).unwrap_or(0),
            },
        )?;

        Ok(MDRawDirectory {
            stream_type: MDStreamType::BreakpadInfoStream as u32,
            location: bp_section.location(),
        })
    }
}
