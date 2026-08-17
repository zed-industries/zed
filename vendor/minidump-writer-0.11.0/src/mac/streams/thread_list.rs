use {super::*, crate::minidump_cpu::RawContextCPU};

impl MinidumpWriter {
    /// Writes the [`MDStreamType::ThreadListStream`] which is an array of
    /// [`miniduimp_common::format::MINIDUMP_THREAD`]
    pub(crate) fn write_thread_list(
        &mut self,
        buffer: &mut DumpBuf,
        dumper: &TaskDumper,
    ) -> Result<MDRawDirectory, WriterError> {
        let threads = self.threads(dumper);

        let list_header = MemoryWriter::<u32>::alloc_with_val(buffer, threads.len() as u32)?;

        let mut dirent = MDRawDirectory {
            stream_type: MDStreamType::ThreadListStream as u32,
            location: list_header.location(),
        };

        let mut thread_list = MemoryArrayWriter::<MDRawThread>::alloc_array(buffer, threads.len())?;
        dirent.location.data_size += thread_list.location().data_size;

        for (i, tid) in threads.enumerate() {
            let thread = self.write_thread(tid, buffer, dumper)?;
            thread_list.set_value_at(buffer, thread, i)?;
        }

        Ok(dirent)
    }

    fn write_thread(
        &mut self,
        tid: u32,
        buffer: &mut DumpBuf,
        dumper: &TaskDumper,
    ) -> Result<MDRawThread, WriterError> {
        let mut thread = MDRawThread {
            thread_id: tid,
            suspend_count: 0,
            priority_class: 0,
            priority: 0,
            teb: 0,
            stack: MDMemoryDescriptor::default(),
            thread_context: MDLocationDescriptor::default(),
        };

        let thread_state = dumper.read_thread_state(tid)?;

        self.write_stack_from_start_address(thread_state.sp(), &mut thread, buffer, dumper)?;

        let mut cpu: RawContextCPU = Default::default();
        Self::fill_cpu_context(&thread_state, &mut cpu);
        let cpu_section = MemoryWriter::alloc_with_val(buffer, cpu)?;
        thread.thread_context = cpu_section.location();
        Ok(thread)
    }

    fn write_stack_from_start_address(
        &mut self,
        start: u64,
        thread: &mut MDRawThread,
        buffer: &mut DumpBuf,
        dumper: &TaskDumper,
    ) -> Result<(), WriterError> {
        thread.stack.start_of_memory_range = start;
        thread.stack.memory.data_size = 0;
        thread.stack.memory.rva = buffer.position() as u32;

        let stack_size = self.calculate_stack_size(start, dumper);

        // In some situations the stack address for the thread can come back 0.
        // In these cases we skip over the threads in question and stuff the
        // stack with a clearly borked value.
        //
        // In other cases, notably a stack overflow, we might fail to read the
        // stack eg. InvalidAddress in which case we use a different borked
        // value to indicate the different failure
        let stack_location = if stack_size != 0 {
            dumper
                .read_task_memory(start, stack_size)
                .map(|stack_buffer| {
                    let stack_location = MDLocationDescriptor {
                        data_size: stack_buffer.len() as u32,
                        rva: buffer.position() as u32,
                    };
                    buffer.write_all(&stack_buffer);
                    stack_location
                })
                .ok()
        } else {
            None
        };

        thread.stack.memory = stack_location.unwrap_or_else(|| {
            let borked = if stack_size == 0 {
                0xdeadbeef
            } else {
                0xdeaddead
            };

            thread.stack.start_of_memory_range = borked;

            let stack_location = MDLocationDescriptor {
                data_size: 16,
                rva: buffer.position() as u32,
            };
            buffer.write_all(&borked.to_ne_bytes());
            buffer.write_all(&borked.to_ne_bytes());
            stack_location
        });

        // Add the stack memory as a raw block of memory, this is written to
        // the minidump as part of the memory list stream
        self.memory_blocks.push(thread.stack);
        Ok(())
    }

    fn calculate_stack_size(&self, start_address: u64, dumper: &TaskDumper) -> usize {
        if start_address == 0 {
            return 0;
        }

        let mut region = if let Ok(region) = dumper.get_vm_region(start_address) {
            region
        } else {
            return 0;
        };

        // Failure or stack corruption, since mach_vm_region had to go
        // higher in the process address space to find a valid region.
        if start_address < region.range.start {
            return 0;
        }

        let root_range_start = region.range.start;
        let mut stack_size = region.range.end - region.range.start;

        // If the user tag is VM_MEMORY_STACK, look for more readable regions with
        // the same tag placed immediately above the computed stack region. Under
        // some circumstances, the stack for thread 0 winds up broken up into
        // multiple distinct abutting regions. This can happen for several reasons,
        // including user code that calls setrlimit(RLIMIT_STACK, ...) or changes
        // the access on stack pages by calling mprotect.
        if region.info.user_tag == mach2::vm_statistics::VM_MEMORY_STACK {
            loop {
                let proposed_next_region_base = region.range.end;

                region = if let Ok(reg) = dumper.get_vm_region(region.range.end) {
                    reg
                } else {
                    break;
                };

                if region.range.start != proposed_next_region_base
                    || region.info.user_tag != mach2::vm_statistics::VM_MEMORY_STACK
                    || (region.info.protection & mach2::vm_prot::VM_PROT_READ) == 0
                {
                    break;
                }

                stack_size += region.range.end - region.range.start;
            }
        }

        (root_range_start + stack_size - start_address) as usize
    }

    pub(crate) fn fill_cpu_context(
        thread_state: &crate::mac::mach::ThreadState,
        out: &mut RawContextCPU,
    ) {
        let ts = thread_state.arch_state();

        cfg_if::cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                out.context_flags = format::ContextFlagsCpu::CONTEXT_AMD64.bits();

                out.rax = ts.__rax;
                out.rbx = ts.__rbx;
                out.rcx = ts.__rcx;
                out.rdx = ts.__rdx;
                out.rdi = ts.__rdi;
                out.rsi = ts.__rsi;
                out.rbp = ts.__rbp;
                out.rsp = ts.__rsp;
                out.r8 = ts.__r8;
                out.r9 = ts.__r9;
                out.r10 = ts.__r10;
                out.r11 = ts.__r11;
                out.r12 = ts.__r12;
                out.r13 = ts.__r13;
                out.r14 = ts.__r14;
                out.r15 = ts.__r15;
                out.rip = ts.__rip;
                // according to AMD's software developer guide, bits above 18 are
                // not used in the flags register.  Since the minidump format
                // specifies 32 bits for the flags register, we can truncate safely
                // with no loss.
                out.eflags = ts.__rflags as _;
                out.cs = ts.__cs as u16;
                out.fs = ts.__fs as u16;
                out.gs = ts.__gs as u16;
            } else if #[cfg(target_arch = "aarch64")] {
                // This is kind of a lie as we don't actually include the full float state..?
                out.context_flags = format::ContextFlagsArm64Old::CONTEXT_ARM64_OLD_FULL.bits() as u64;

                out.cpsr = ts.cpsr;
                out.iregs[..29].copy_from_slice(&ts.x[..29]);
                out.iregs[29] = ts.fp;
                out.iregs[30] = ts.lr;
                out.sp = ts.sp;
                out.pc = ts.pc;
            } else {
                compile_error!("unsupported target arch");
            }
        }
    }
}
