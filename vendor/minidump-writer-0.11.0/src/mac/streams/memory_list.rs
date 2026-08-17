use super::*;

impl MinidumpWriter {
    /// Writes the [`MDStreamType::MemoryListStream`]. The memory blocks that are
    /// written into this stream are the raw thread contexts that were retrieved
    /// and added by [`Self::write_thread_list`]
    pub(crate) fn write_memory_list(
        &mut self,
        buffer: &mut DumpBuf,
        dumper: &TaskDumper,
    ) -> Result<MDRawDirectory, WriterError> {
        // Include some memory around the instruction pointer if the crash was
        // due to an exception
        if let Some(cc) = &self.crash_context {
            if cc.exception.is_some() {
                const IP_MEM_SIZE: u64 = 256;

                let get_ip_block = |tid| -> Option<std::ops::Range<u64>> {
                    let thread_state = dumper.read_thread_state(tid).ok()?;

                    let ip = thread_state.pc();

                    // Bound it to the upper and lower bounds of the region
                    // it's contained within. If it's not in a known memory region,
                    // don't bother trying to write it.
                    let region = dumper.get_vm_region(ip).ok()?;

                    if ip < region.range.start || ip > region.range.end {
                        return None;
                    }

                    // Try to get IP_MEM_SIZE / 2 bytes before and after the IP, but
                    // settle for whatever's available.
                    let start = std::cmp::max(region.range.start, ip - IP_MEM_SIZE / 2);
                    let end = std::cmp::min(ip + IP_MEM_SIZE / 2, region.range.end);

                    Some(start..end)
                };

                if let Some(ip_range) = get_ip_block(cc.thread) {
                    let size = ip_range.end - ip_range.start;
                    let stack_buffer =
                        dumper.read_task_memory(ip_range.start as _, size as usize)?;
                    let ip_location = MDLocationDescriptor {
                        data_size: size as u32,
                        rva: buffer.position() as u32,
                    };
                    buffer.write_all(&stack_buffer);

                    self.memory_blocks.push(MDMemoryDescriptor {
                        start_of_memory_range: ip_range.start,
                        memory: ip_location,
                    });
                }
            }
        }

        let list_header =
            MemoryWriter::<u32>::alloc_with_val(buffer, self.memory_blocks.len() as u32)?;

        let mut dirent = MDRawDirectory {
            stream_type: MDStreamType::MemoryListStream as u32,
            location: list_header.location(),
        };

        let block_list =
            MemoryArrayWriter::<MDMemoryDescriptor>::alloc_from_array(buffer, &self.memory_blocks)?;

        dirent.location.data_size += block_list.location().data_size;
        Ok(dirent)
    }
}
