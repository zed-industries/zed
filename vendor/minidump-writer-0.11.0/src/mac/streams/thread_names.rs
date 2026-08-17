use super::*;

impl MinidumpWriter {
    /// Writes the [`MDStreamType::ThreadNamesStream`] which is an array of
    /// [`miniduimp_common::format::MINIDUMP_THREAD`]
    pub(crate) fn write_thread_names(
        &mut self,
        buffer: &mut DumpBuf,
        dumper: &TaskDumper,
    ) -> Result<MDRawDirectory, WriterError> {
        let threads = self.threads(dumper);

        let list_header = MemoryWriter::<u32>::alloc_with_val(buffer, threads.len() as u32)?;

        let mut dirent = MDRawDirectory {
            stream_type: MDStreamType::ThreadNamesStream as u32,
            location: list_header.location(),
        };

        let mut names = MemoryArrayWriter::<MDRawThreadName>::alloc_array(buffer, threads.len())?;
        dirent.location.data_size += names.location().data_size;

        for (i, tid) in threads.enumerate() {
            // It's unfortunate if we can't grab a thread name, but it's also
            // not a critical failure
            let name_loc = match Self::write_thread_name(buffer, dumper, tid) {
                Ok(loc) => loc,
                Err(err) => {
                    log::warn!("failed to write thread name for thread {tid}: {err}");
                    write_string_to_location(buffer, "")?
                }
            };

            let thread = MDRawThreadName {
                thread_id: tid,
                thread_name_rva: name_loc.rva.into(),
            };

            names.set_value_at(buffer, thread, i)?;
        }

        Ok(dirent)
    }

    /// Attempts to retrieve and write the threadname, returning the threa names
    /// location if successful
    fn write_thread_name(
        buffer: &mut Buffer,
        dumper: &TaskDumper,
        tid: u32,
    ) -> Result<MDLocationDescriptor, WriterError> {
        let thread_info: libc::proc_threadinfo = dumper.thread_info(tid)?;

        let name = std::str::from_utf8(
            // SAFETY: This is an initialized block of static size
            unsafe {
                std::slice::from_raw_parts(
                    thread_info.pth_name.as_ptr().cast(),
                    thread_info.pth_name.len(),
                )
            },
        )
        .unwrap_or_default();

        // Ignore the null terminator
        let tname = match name.find('\0') {
            Some(i) => &name[..i],
            None => name,
        };

        Ok(write_string_to_location(buffer, tname)?)
    }
}

/// As noted in `usr/include/mach/thread_info.h`, the `THREAD_EXTENDED_INFO`
/// return is exactly the same as `proc_pidinfo(..., proc_threadinfo)`
impl mach::ThreadInfo for libc::proc_threadinfo {
    /// `THREAD_EXTENDED_INFO`
    const FLAVOR: u32 = 5;
}
