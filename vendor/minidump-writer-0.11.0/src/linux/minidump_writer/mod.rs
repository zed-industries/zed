use {
    super::{
        app_memory::AppMemoryList,
        auxv::AuxvDumpInfo,
        crash_context::CrashContext,
        dso_debug,
        dumper_cpu_info::CpuInfoError,
        maps_reader::{MappingInfo, MappingList, MapsReaderError},
        mem_reader::CopyFromProcessError,
        module_reader,
        serializers::*,
        thread_info::{ThreadInfo, ThreadInfoError},
        Pid,
    },
    crate::{
        dir_section::{DirSection, DumpBuf},
        mem_writer::{
            write_string_to_location, Buffer, MemoryArrayWriter, MemoryWriter, MemoryWriterError,
        },
        minidump_format::*,
        serializers::*,
    },
    error_graph::{ErrorList, WriteErrorList},
    errors::{ContinueProcessError, InitError, StopProcessError, WriterError},
    failspot::failspot,
    nix::{
        errno::Errno,
        sys::{ptrace, signal, wait},
    },
    procfs_core::{
        process::{MMPermissions, ProcState, Stat},
        FromRead,
    },
    std::{
        io::{Seek, Write},
        path,
        time::{Duration, Instant},
    },
    thiserror::Error,
};

#[cfg(target_os = "android")]
use super::android::late_process_mappings;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use super::thread_info;

pub use super::auxv::{AuxvType, DirectAuxvDumpInfo};

pub mod app_memory;
pub mod errors;
pub mod exception_stream;
pub mod handle_data_stream;
pub mod mappings;
pub mod memory_info_list_stream;
pub mod memory_list_stream;
pub mod systeminfo_stream;
pub mod thread_list_stream;
pub mod thread_names_stream;

/// The default timeout after a `SIGSTOP` after which minidump writing proceeds
/// regardless of the process state
pub const STOP_TIMEOUT: Duration = Duration::from_millis(100);

#[cfg(target_pointer_width = "32")]
pub const AT_SYSINFO_EHDR: u32 = 33;
#[cfg(target_pointer_width = "64")]
pub const AT_SYSINFO_EHDR: u64 = 33;

#[derive(Debug)]
pub struct MinidumpWriterConfig {
    process_id: Pid,
    blamed_thread: Pid,
    minidump_size_limit: Option<u64>,
    skip_stacks_if_mapping_unreferenced: bool,
    principal_mapping_address: Option<usize>,
    user_mapping_list: MappingList,
    app_memory: AppMemoryList,
    memory_blocks: Vec<MDMemoryDescriptor>,
    principal_mapping: Option<MappingInfo>,
    sanitize_stack: bool,
    crash_context: Option<CrashContext>,
    crashing_thread_context: CrashingThreadContext,
    stop_timeout: Duration,
    direct_auxv_dump_info: Option<DirectAuxvDumpInfo>,
}

#[derive(Debug)]
pub struct MinidumpWriter {
    pub process_id: Pid,
    threads_suspended: bool,
    pub threads: Vec<Thread>,
    pub auxv: AuxvDumpInfo,
    pub mappings: Vec<MappingInfo>,
    pub page_size: usize,
    pub sanitize_stack: bool,
    pub minidump_size_limit: Option<u64>,
    pub user_mapping_list: MappingList,
    pub crashing_thread_context: CrashingThreadContext,
    stop_timeout: Duration,
    pub skip_stacks_if_mapping_unreferenced: bool,
    principal_mapping_address: Option<usize>,
    pub principal_mapping: Option<MappingInfo>,
    pub blamed_thread: Pid,
    pub crash_context: Option<CrashContext>,
    pub app_memory: AppMemoryList,
    pub memory_blocks: Vec<MDMemoryDescriptor>,
}

#[derive(Debug, Clone)]
pub struct Thread {
    pub tid: Pid,
    pub name: Option<String>,
}

#[derive(Debug, Default)]
pub enum CrashingThreadContext {
    #[default]
    None,
    CrashContext(MDLocationDescriptor),
    CrashContextPlusAddress((MDLocationDescriptor, usize)),
}

impl MinidumpWriterConfig {
    pub fn new(process_id: Pid, blamed_thread: Pid) -> Self {
        Self {
            process_id,
            blamed_thread,
            minidump_size_limit: Default::default(),
            skip_stacks_if_mapping_unreferenced: Default::default(),
            principal_mapping_address: Default::default(),
            user_mapping_list: Default::default(),
            app_memory: Default::default(),
            memory_blocks: Default::default(),
            principal_mapping: Default::default(),
            sanitize_stack: Default::default(),
            crash_context: Default::default(),
            crashing_thread_context: Default::default(),
            stop_timeout: STOP_TIMEOUT,
            direct_auxv_dump_info: Default::default(),
        }
    }

    pub fn set_minidump_size_limit(&mut self, limit: u64) -> &mut Self {
        self.minidump_size_limit = Some(limit);
        self
    }

    pub fn set_user_mapping_list(&mut self, user_mapping_list: MappingList) -> &mut Self {
        self.user_mapping_list = user_mapping_list;
        self
    }

    pub fn set_principal_mapping_address(&mut self, principal_mapping_address: usize) -> &mut Self {
        self.principal_mapping_address = Some(principal_mapping_address);
        self
    }

    pub fn set_app_memory(&mut self, app_memory: AppMemoryList) -> &mut Self {
        self.app_memory = app_memory;
        self
    }

    pub fn set_crash_context(&mut self, crash_context: CrashContext) -> &mut Self {
        self.crash_context = Some(crash_context);
        self
    }

    pub fn skip_stacks_if_mapping_unreferenced(&mut self) -> &mut Self {
        self.skip_stacks_if_mapping_unreferenced = true; // Off by default
        self
    }

    pub fn sanitize_stack(&mut self) -> &mut Self {
        self.sanitize_stack = true; // Off by default
        self
    }

    /// Sets the timeout after `SIGSTOP` is sent to the process, if the process
    /// has not stopped by the time the timeout has reached, we proceed with
    /// minidump generation
    pub fn stop_timeout(&mut self, duration: Duration) -> &mut Self {
        self.stop_timeout = duration;
        self
    }

    /// Directly set important Auxv info determined by the crashing process
    ///
    /// Since `/proc/{pid}/auxv` can sometimes be inaccessible, the calling process should prefer to transfer this
    /// information directly using the Linux `getauxval()` call (if possible).
    ///
    /// Any field that is set to `0` will be considered unset. In that case, minidump-writer might try other techniques
    /// to obtain it (like reading `/proc/{pid}/auxv`).
    pub fn set_direct_auxv_dump_info(
        &mut self,
        direct_auxv_dump_info: DirectAuxvDumpInfo,
    ) -> &mut Self {
        self.direct_auxv_dump_info = Some(direct_auxv_dump_info);
        self
    }
    /// Generates a minidump and writes to the destination provided. Returns the in-memory
    /// version of the minidump as well.
    pub fn write(self, destination: &mut (impl Write + Seek)) -> Result<Vec<u8>, WriterError> {
        let mut soft_errors = ErrorList::default();

        let mut writer = self.build();
        writer.init(soft_errors.subwriter(WriterError::InitErrors))?;

        let mut buffer = Buffer::with_capacity(0);
        writer.write_dump(&mut buffer, destination, soft_errors)?;
        Ok(buffer.into())
    }
    /// Allows testing code to inspect the pre-output state of the MinidumpWriter
    pub fn build_for_testing(
        self,
        soft_errors: impl WriteErrorList<InitError>,
    ) -> Result<MinidumpWriter, InitError> {
        let mut writer = self.build();
        writer.init(soft_errors)?;
        Ok(writer)
    }
    fn build(self) -> MinidumpWriter {
        let auxv = self
            .direct_auxv_dump_info
            .map(AuxvDumpInfo::from)
            .unwrap_or_default();

        MinidumpWriter {
            process_id: self.process_id,
            threads_suspended: Default::default(),
            threads: Default::default(),
            auxv,
            mappings: Default::default(),
            page_size: Default::default(),
            sanitize_stack: self.sanitize_stack,
            minidump_size_limit: self.minidump_size_limit,
            user_mapping_list: self.user_mapping_list,
            crashing_thread_context: self.crashing_thread_context,
            stop_timeout: self.stop_timeout,
            skip_stacks_if_mapping_unreferenced: self.skip_stacks_if_mapping_unreferenced,
            principal_mapping_address: self.principal_mapping_address,
            principal_mapping: self.principal_mapping,
            blamed_thread: self.blamed_thread,
            crash_context: self.crash_context,
            app_memory: self.app_memory,
            memory_blocks: self.memory_blocks,
        }
    }
}

impl MinidumpWriter {
    // TODO: late_init for chromeos and android
    fn init(&mut self, mut soft_errors: impl WriteErrorList<InitError>) -> Result<(), InitError> {
        if self.process_id == std::process::id() as i32 {
            return Err(InitError::CannotPtraceSameProcess);
        }

        // Stopping the process is best-effort.
        if let Err(e) = self.stop_process(self.stop_timeout) {
            soft_errors.push(InitError::StopProcessFailed(e));
        }

        // Even if we completely fail to fill in any additional Auxv info, we can still press
        // forward.
        if let Err(e) = self.auxv.try_filling_missing_info(
            self.process_id,
            soft_errors.subwriter(InitError::FillMissingAuxvInfoErrors),
        ) {
            soft_errors.push(InitError::FillMissingAuxvInfoFailed(e));
        }

        // If we completely fail to enumerate any threads... Some information is still better than
        // no information!
        if let Err(e) =
            self.enumerate_threads(soft_errors.subwriter(InitError::EnumerateThreadsErrors))
        {
            soft_errors.push(InitError::EnumerateThreadsFailed(Box::new(e)));
        }

        // Same with mappings -- Some information is still better than no information!
        if let Err(e) = self.enumerate_mappings() {
            soft_errors.push(InitError::EnumerateMappingsFailed(Box::new(e)));
        }

        self.page_size = nix::unistd::sysconf(nix::unistd::SysconfVar::PAGE_SIZE)?
            .expect("page size apparently unlimited: doesn't make sense.")
            as usize;

        let threads_count = self.threads.len();

        self.suspend_threads(soft_errors.subwriter(InitError::SuspendThreadsErrors));

        if self.threads.is_empty() {
            soft_errors.push(InitError::SuspendNoThreadsLeft(threads_count));
        }

        #[cfg(target_os = "android")]
        {
            late_process_mappings(self.process_id, &mut self.mappings)?;
        }

        if self.skip_stacks_if_mapping_unreferenced {
            if let Some(address) = self.principal_mapping_address {
                self.principal_mapping = self.find_mapping_no_bias(address).cloned();
            }

            if !self.crash_thread_references_principal_mapping() {
                soft_errors.push(InitError::PrincipalMappingNotReferenced);
            }
        }

        Ok(())
    }
    /// Generates a minidump and writes to the destination provided. Returns the in-memory
    /// version of the minidump as well.
    fn write_dump(
        &mut self,
        buffer: &mut DumpBuf,
        destination: &mut (impl Write + Seek),
        mut soft_errors: ErrorList<WriterError>,
    ) -> Result<(), WriterError> {
        // A minidump file contains a number of tagged streams. This is the number
        // of streams which we write.
        let num_writers = 18u32;

        let mut header_section = MemoryWriter::<MDRawHeader>::alloc(buffer)?;

        let mut dir_section = DirSection::new(buffer, num_writers, destination)?;

        let header = MDRawHeader {
            signature: MD_HEADER_SIGNATURE,
            version: MD_HEADER_VERSION,
            stream_count: num_writers,
            //   header.get()->stream_directory_rva = dir.position();
            stream_directory_rva: dir_section.position(),
            checksum: 0, /* Can be 0.  In fact, that's all that's
                          * been found in minidump files. */
            time_date_stamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs() as u32, // TODO: This is not Y2038 safe, but thats how its currently defined as
            flags: 0,
        };
        header_section.set_value(buffer, header)?;

        // Ensure the header gets flushed. If we crash somewhere below,
        // we should have a mostly-intact dump
        dir_section.write_to_file(buffer, None)?;

        let dirent = self.write_thread_list_stream(buffer)?;
        dir_section.write_to_file(buffer, Some(dirent))?;

        let dirent = self.write_mappings(buffer)?;
        dir_section.write_to_file(buffer, Some(dirent))?;

        self.write_app_memory(buffer)?;
        dir_section.write_to_file(buffer, None)?;

        let dirent = self.write_memory_list_stream(buffer)?;
        dir_section.write_to_file(buffer, Some(dirent))?;

        let dirent = self.write_exception_stream(buffer)?;
        dir_section.write_to_file(buffer, Some(dirent))?;

        let dirent = systeminfo_stream::write(
            buffer,
            soft_errors.subwriter(WriterError::WriteSystemInfoErrors),
        )?;
        dir_section.write_to_file(buffer, Some(dirent))?;

        let dirent = self.write_memory_info_list_stream(buffer)?;
        dir_section.write_to_file(buffer, Some(dirent))?;

        let dirent = match write_file(buffer, "/proc/cpuinfo") {
            Ok(location) => MDRawDirectory {
                stream_type: MDStreamType::LinuxCpuInfo as u32,
                location,
            },
            Err(e) => {
                soft_errors.push(WriterError::WriteCpuInfoFailed(e));
                Default::default()
            }
        };
        dir_section.write_to_file(buffer, Some(dirent))?;

        let dirent = match write_file(buffer, &format!("/proc/{}/status", self.blamed_thread)) {
            Ok(location) => MDRawDirectory {
                stream_type: MDStreamType::LinuxProcStatus as u32,
                location,
            },
            Err(e) => {
                soft_errors.push(WriterError::WriteThreadProcStatusFailed(e));
                Default::default()
            }
        };
        dir_section.write_to_file(buffer, Some(dirent))?;

        let dirent = match write_file(buffer, "/etc/lsb-release")
            .or_else(|_| write_file(buffer, "/etc/os-release"))
        {
            Ok(location) => MDRawDirectory {
                stream_type: MDStreamType::LinuxLsbRelease as u32,
                location,
            },
            Err(e) => {
                soft_errors.push(WriterError::WriteOsReleaseInfoFailed(e));
                Default::default()
            }
        };
        dir_section.write_to_file(buffer, Some(dirent))?;

        let dirent = match write_file(buffer, &format!("/proc/{}/cmdline", self.blamed_thread)) {
            Ok(location) => MDRawDirectory {
                stream_type: MDStreamType::LinuxCmdLine as u32,
                location,
            },
            Err(e) => {
                soft_errors.push(WriterError::WriteCommandLineFailed(e));
                Default::default()
            }
        };
        dir_section.write_to_file(buffer, Some(dirent))?;

        let dirent = match write_file(buffer, &format!("/proc/{}/environ", self.blamed_thread)) {
            Ok(location) => MDRawDirectory {
                stream_type: MDStreamType::LinuxEnviron as u32,
                location,
            },
            Err(e) => {
                soft_errors.push(WriterError::WriteEnvironmentFailed(e));
                Default::default()
            }
        };
        dir_section.write_to_file(buffer, Some(dirent))?;

        let dirent = match write_file(buffer, &format!("/proc/{}/auxv", self.blamed_thread)) {
            Ok(location) => MDRawDirectory {
                stream_type: MDStreamType::LinuxAuxv as u32,
                location,
            },
            Err(e) => {
                soft_errors.push(WriterError::WriteAuxvFailed(e));
                Default::default()
            }
        };
        dir_section.write_to_file(buffer, Some(dirent))?;

        let dirent = match write_file(buffer, &format!("/proc/{}/maps", self.blamed_thread)) {
            Ok(location) => MDRawDirectory {
                stream_type: MDStreamType::LinuxMaps as u32,
                location,
            },
            Err(e) => {
                soft_errors.push(WriterError::WriteMapsFailed(e));
                Default::default()
            }
        };
        dir_section.write_to_file(buffer, Some(dirent))?;

        let dirent = match dso_debug::write_dso_debug_stream(buffer, self.process_id, &self.auxv) {
            Ok(dirent) => dirent,
            Err(e) => {
                soft_errors.push(WriterError::WriteDSODebugStreamFailed(e));
                Default::default()
            }
        };
        dir_section.write_to_file(buffer, Some(dirent))?;

        let dirent = match write_file(buffer, &format!("/proc/{}/limits", self.blamed_thread)) {
            Ok(location) => MDRawDirectory {
                stream_type: MDStreamType::MozLinuxLimits as u32,
                location,
            },
            Err(e) => {
                soft_errors.push(WriterError::WriteLimitsFailed(e));
                Default::default()
            }
        };
        dir_section.write_to_file(buffer, Some(dirent))?;

        let dirent = self.write_thread_names_stream(buffer)?;
        dir_section.write_to_file(buffer, Some(dirent))?;

        let dirent = match self.write_handle_data_stream(buffer) {
            Ok(dirent) => dirent,
            Err(e) => {
                soft_errors.push(WriterError::WriteHandleDataStreamFailed(e));
                Default::default()
            }
        };
        dir_section.write_to_file(buffer, Some(dirent))?;

        // If this fails, there's really nothing we can do about that (other than ignore it).
        let dirent = write_soft_errors(buffer, soft_errors)
            .map(|location| MDRawDirectory {
                stream_type: MDStreamType::MozSoftErrors as u32,
                location,
            })
            .unwrap_or_default();
        dir_section.write_to_file(buffer, Some(dirent))?;

        // If you add more directory entries, don't forget to update num_writers, above.
        Ok(())
    }

    fn crash_thread_references_principal_mapping(&self) -> bool {
        if self.crash_context.is_none() || self.principal_mapping.is_none() {
            return false;
        }

        let low_addr = self
            .principal_mapping
            .as_ref()
            .unwrap()
            .system_mapping_info
            .start_address;
        let high_addr = self
            .principal_mapping
            .as_ref()
            .unwrap()
            .system_mapping_info
            .end_address;

        let pc = self
            .crash_context
            .as_ref()
            .unwrap()
            .get_instruction_pointer();
        let stack_pointer = self.crash_context.as_ref().unwrap().get_stack_pointer();

        if pc >= low_addr && pc < high_addr {
            return true;
        }

        let (valid_stack_pointer, stack_len) = match self.get_stack_info(stack_pointer) {
            Ok(x) => x,
            Err(_) => {
                return false;
            }
        };

        let stack_copy = match MinidumpWriter::copy_from_process(
            self.blamed_thread,
            valid_stack_pointer,
            stack_len,
        ) {
            Ok(x) => x,
            Err(_) => {
                return false;
            }
        };

        let sp_offset = stack_pointer.saturating_sub(valid_stack_pointer);
        self.principal_mapping
            .as_ref()
            .unwrap()
            .stack_has_pointer_to_mapping(&stack_copy, sp_offset)
    }

    /// Suspends a thread by attaching to it.
    fn suspend_thread(child: Pid) -> Result<(), WriterError> {
        use WriterError::PtraceAttachError as AttachErr;

        let pid = nix::unistd::Pid::from_raw(child);
        // This may fail if the thread has just died or debugged.
        ptrace::attach(pid).map_err(|e| AttachErr(child, e))?;
        loop {
            match wait::waitpid(pid, Some(wait::WaitPidFlag::__WALL)) {
                Ok(status) => {
                    let wait::WaitStatus::Stopped(_, status) = status else {
                        return Err(WriterError::WaitPidError(
                            child,
                            nix::errno::Errno::UnknownErrno,
                        ));
                    };

                    // Any signal will stop the thread, make sure it is SIGSTOP. Otherwise, this
                    // signal will be delivered after PTRACE_DETACH, and the thread will enter
                    // the "T (stopped)" state.
                    if status == nix::sys::signal::SIGSTOP {
                        break;
                    }

                    // Signals other than SIGSTOP that are received need to be reinjected,
                    // or they will otherwise get lost.
                    if let Err(err) = ptrace::cont(pid, status) {
                        return Err(WriterError::WaitPidError(child, err));
                    }
                }
                Err(Errno::EINTR) => continue,
                Err(e) => {
                    ptrace_detach(child)?;
                    return Err(WriterError::WaitPidError(child, e));
                }
            }
        }
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            // On x86, the stack pointer is NULL or -1, when executing trusted code in
            // the seccomp sandbox. Not only does this cause difficulties down the line
            // when trying to dump the thread's stack, it also results in the minidumps
            // containing information about the trusted threads. This information is
            // generally completely meaningless and just pollutes the minidumps.
            // We thus test the stack pointer and exclude any threads that are part of
            // the seccomp sandbox's trusted code.
            let skip_thread;
            let regs = thread_info::ThreadInfo::getregs(pid.into());
            if let Ok(regs) = regs {
                #[cfg(target_arch = "x86_64")]
                {
                    skip_thread = regs.rsp == 0;
                }
                #[cfg(target_arch = "x86")]
                {
                    skip_thread = regs.esp == 0;
                }
            } else {
                skip_thread = true;
            }
            if skip_thread {
                ptrace_detach(child)?;
                return Err(WriterError::DetachSkippedThread(child));
            }
        }
        Ok(())
    }

    /// Resumes a thread by detaching from it.
    fn resume_thread(child: Pid) -> Result<(), WriterError> {
        ptrace_detach(child)
    }

    fn suspend_threads(&mut self, mut soft_errors: impl WriteErrorList<WriterError>) {
        // Iterate over all threads and try to suspend them.
        // If the thread either disappeared before we could attach to it, or if
        // it was part of the seccomp sandbox's trusted code, it is OK to
        // silently drop it from the minidump.
        self.threads.retain(|x| match Self::suspend_thread(x.tid) {
            Ok(()) => true,
            Err(e) => {
                soft_errors.push(e);
                false
            }
        });

        self.threads_suspended = true;

        failspot::failspot!(<crate::FailSpotName>::SuspendThreads soft_errors.push(WriterError::PtraceAttachError(1234, nix::Error::EPERM)))
    }

    fn resume_threads(&mut self, mut soft_errors: impl WriteErrorList<WriterError>) {
        if self.threads_suspended {
            for thread in &self.threads {
                match Self::resume_thread(thread.tid) {
                    Ok(()) => (),
                    Err(e) => {
                        soft_errors.push(e);
                    }
                }
            }
        }
        self.threads_suspended = false;
    }

    /// Send SIGSTOP to the process so that we can get a consistent state.
    ///
    /// This will block waiting for the process to stop until `timeout` has passed.
    fn stop_process(&mut self, timeout: Duration) -> Result<(), StopProcessError> {
        failspot!(StopProcess bail(nix::Error::EPERM));

        signal::kill(
            nix::unistd::Pid::from_raw(self.process_id),
            Some(signal::SIGSTOP),
        )?;

        // Something like waitpid for non-child processes would be better, but we have no such
        // tool, so we poll the status.
        const POLL_INTERVAL: Duration = Duration::from_millis(1);
        let proc_file = format!("/proc/{}/stat", self.process_id);
        let end = Instant::now() + timeout;

        loop {
            if let Ok(ProcState::Stopped) = Stat::from_file(&proc_file)?.state() {
                return Ok(());
            }

            std::thread::sleep(POLL_INTERVAL);
            if Instant::now() > end {
                return Err(StopProcessError::Timeout);
            }
        }
    }

    /// Send SIGCONT to the process to continue.
    ///
    /// Unlike `stop_process`, this function does not wait for the process to continue.
    fn continue_process(&mut self) -> Result<(), ContinueProcessError> {
        signal::kill(
            nix::unistd::Pid::from_raw(self.process_id),
            Some(signal::SIGCONT),
        )?;
        Ok(())
    }

    /// Parse /proc/$pid/task to list all the threads of the process identified by
    /// pid.
    fn enumerate_threads(
        &mut self,
        mut soft_errors: impl WriteErrorList<InitError>,
    ) -> Result<(), InitError> {
        let pid = self.process_id;
        let filename = format!("/proc/{pid}/task");
        let task_path = path::PathBuf::from(&filename);
        if !task_path.is_dir() {
            return Err(InitError::ProcPidTaskNotDirectory(filename));
        }

        for entry in std::fs::read_dir(task_path).map_err(|e| InitError::IOError(filename, e))? {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    soft_errors.push(InitError::ReadProcessThreadEntryFailed(e));
                    continue;
                }
            };
            let file_name = entry.file_name();
            let tid = match file_name.to_str().and_then(|name| name.parse::<Pid>().ok()) {
                Some(tid) => tid,
                None => {
                    soft_errors.push(InitError::ProcessTaskEntryNotTid(file_name));
                    continue;
                }
            };

            // Read the thread-name (if there is any)
            let name_result = failspot!(if ThreadName {
                Err(std::io::Error::other(
                    "testing requested failure reading thread name",
                ))
            } else {
                std::fs::read_to_string(format!("/proc/{pid}/task/{tid}/comm"))
            });

            let name = match name_result {
                Ok(name) => Some(name.trim_end().to_string()),
                Err(e) => {
                    soft_errors.push(InitError::ReadThreadNameFailed(e));
                    None
                }
            };

            self.threads.push(Thread { tid, name });
        }

        Ok(())
    }

    fn enumerate_mappings(&mut self) -> Result<(), InitError> {
        // linux_gate_loc is the beginning of the kernel's mapping of
        // linux-gate.so in the process.  It doesn't actually show up in the
        // maps list as a filename, but it can be found using the AT_SYSINFO_EHDR
        // aux vector entry, which gives the information necessary to special
        // case its entry when creating the list of mappings.
        // See http://www.trilithium.com/johan/2005/08/linux-gate/ for more
        // information.
        let maps_path = format!("/proc/{}/maps", self.process_id);
        let maps_file =
            std::fs::File::open(&maps_path).map_err(|e| InitError::IOError(maps_path, e))?;

        let maps = procfs_core::process::MemoryMaps::from_read(maps_file)
            .map_err(InitError::ReadProcessMapFileFailed)?;

        self.mappings = MappingInfo::aggregate(maps, self.auxv.get_linux_gate_address())
            .map_err(InitError::AggregateMappingsFailed)?;

        // Although the initial executable is usually the first mapping, it's not
        // guaranteed (see http://crosbug.com/25355); therefore, try to use the
        // actual entry point to find the mapping.
        if let Some(entry_point_loc) = self
            .auxv
            .get_entry_address()
            .map(|u| usize::try_from(u).unwrap())
        {
            // If this module contains the entry-point, and it's not already the first
            // one, then we need to make it be first.  This is because the minidump
            // format assumes the first module is the one that corresponds to the main
            // executable (as codified in
            // processor/minidump.cc:MinidumpModuleList::GetMainModule()).
            if let Some(entry_mapping_idx) = self.mappings.iter().position(|mapping| {
                (mapping.start_address..mapping.start_address + mapping.size)
                    .contains(&entry_point_loc)
            }) {
                self.mappings.swap(0, entry_mapping_idx);
            }
        }
        Ok(())
    }

    /// Read thread info from /proc/$pid/status.
    /// Fill out the |tgid|, |ppid| and |pid| members of |info|. If unavailable,
    /// these members are set to -1. Returns true if all three members are
    /// available.
    pub fn get_thread_info_by_index(&self, index: usize) -> Result<ThreadInfo, ThreadInfoError> {
        if index > self.threads.len() {
            return Err(ThreadInfoError::IndexOutOfBounds(index, self.threads.len()));
        }

        ThreadInfo::create(self.process_id, self.threads[index].tid)
    }

    // Returns a valid stack pointer and the mapping that contains the stack.
    // The stack pointer will usually point within this mapping, but it might
    // not in case of stack overflows, hence the returned pointer might be
    // different from the one that was passed in.
    pub fn get_stack_info(&self, int_stack_pointer: usize) -> Result<(usize, usize), WriterError> {
        // Round the stack pointer to the nearest page, this will cause us to
        // capture data below the stack pointer which might still be relevant.
        let mut stack_pointer = int_stack_pointer & !(self.page_size - 1);
        let mut mapping = self.find_mapping(stack_pointer);

        // The guard page has been 1 MiB in size since kernel 4.12, older
        // kernels used a 4 KiB one instead. Note the saturating add, as 32-bit
        // processes can have a stack pointer within 1MiB of usize::MAX
        let guard_page_max_addr = stack_pointer.saturating_add(1024 * 1024);

        // If we found no mapping, or the mapping we found has no permissions
        // then we might have hit a guard page, try looking for a mapping in
        // addresses past the stack pointer. Stack grows towards lower addresses
        // on the platforms we care about so the stack should appear after the
        // guard page.
        while !Self::may_be_stack(mapping) && (stack_pointer <= guard_page_max_addr) {
            stack_pointer += self.page_size;
            mapping = self.find_mapping(stack_pointer);
        }

        mapping
            .map(|mapping| {
                let valid_stack_pointer = if mapping.contains_address(stack_pointer) {
                    stack_pointer
                } else {
                    mapping.start_address
                };

                let stack_len = mapping.size - (valid_stack_pointer - mapping.start_address);
                (valid_stack_pointer, stack_len)
            })
            .ok_or(WriterError::NoStackPointerMapping)
    }

    fn may_be_stack(mapping: Option<&MappingInfo>) -> bool {
        if let Some(mapping) = mapping {
            return mapping
                .permissions
                .intersects(MMPermissions::READ | MMPermissions::WRITE);
        }

        false
    }

    pub fn sanitize_stack_copy(
        &self,
        stack_copy: &mut [u8],
        stack_pointer: usize,
        sp_offset: usize,
    ) -> Result<(), WriterError> {
        // We optimize the search for containing mappings in three ways:
        // 1) We expect that pointers into the stack mapping will be common, so
        //    we cache that address range.
        // 2) The last referenced mapping is a reasonable predictor for the next
        //    referenced mapping, so we test that first.
        // 3) We precompute a bitfield based upon bits 32:32-n of the start and
        //    stop addresses, and use that to short circuit any values that can
        //    not be pointers. (n=11)
        let defaced;
        #[cfg(target_pointer_width = "64")]
        {
            defaced = 0x0defaced0defacedusize.to_ne_bytes();
        }
        #[cfg(target_pointer_width = "32")]
        {
            defaced = 0x0defacedusize.to_ne_bytes();
        };
        // the bitfield length is 2^test_bits long.
        let test_bits = 11;
        // byte length of the corresponding array.
        let array_size: usize = 1 << (test_bits - 3);
        let array_mask = array_size - 1;
        // The amount to right shift pointers by. This captures the top bits
        // on 32 bit architectures. On 64 bit architectures this would be
        // uninformative so we take the same range of bits.
        let shift = 32 - 11;
        // let MappingInfo* last_hit_mapping = nullptr;
        // let MappingInfo* hit_mapping = nullptr;
        let stack_mapping = self.find_mapping_no_bias(stack_pointer);
        let mut last_hit_mapping: Option<&MappingInfo> = None;
        // The magnitude below which integers are considered to be to be
        // 'small', and not constitute a PII risk. These are included to
        // avoid eliding useful register values.
        let small_int_magnitude: isize = 4096;

        let mut could_hit_mapping = vec![0; array_size];
        // Initialize the bitfield such that if the (pointer >> shift)'th
        // bit, modulo the bitfield size, is not set then there does not
        // exist a mapping in mappings that would contain that pointer.
        for mapping in &self.mappings {
            if !mapping.is_executable() {
                continue;
            }
            // For each mapping, work out the (unmodulo'ed) range of bits to
            // set.
            let mut start = mapping.start_address;
            let mut end = start + mapping.size;
            start >>= shift;
            end >>= shift;
            for bit in start..=end {
                // Set each bit in the range, applying the modulus.
                could_hit_mapping[(bit >> 3) & array_mask] |= 1 << (bit & 7);
            }
        }

        // Zero memory that is below the current stack pointer.
        let offset =
            (sp_offset + std::mem::size_of::<usize>() - 1) & !(std::mem::size_of::<usize>() - 1);
        for x in &mut stack_copy[0..offset] {
            *x = 0;
        }
        let mut chunks = stack_copy[offset..].chunks_exact_mut(std::mem::size_of::<usize>());

        // Apply sanitization to each complete pointer-aligned word in the
        // stack.
        for sp in &mut chunks {
            let addr = usize::from_ne_bytes(sp.to_vec().as_slice().try_into()?);
            let addr_signed = isize::from_ne_bytes(sp.to_vec().as_slice().try_into()?);

            if addr <= small_int_magnitude as usize && addr_signed >= -small_int_magnitude {
                continue;
            }

            if let Some(stack_map) = stack_mapping {
                if stack_map.contains_address(addr) {
                    continue;
                }
            }
            if let Some(last_hit) = last_hit_mapping {
                if last_hit.contains_address(addr) {
                    continue;
                }
            }

            let test = addr >> shift;
            if could_hit_mapping[(test >> 3) & array_mask] & (1 << (test & 7)) != 0 {
                if let Some(hit_mapping) = self.find_mapping_no_bias(addr) {
                    if hit_mapping.is_executable() {
                        last_hit_mapping = Some(hit_mapping);
                        continue;
                    }
                }
            }
            sp.copy_from_slice(&defaced);
        }
        // Zero any partial word at the top of the stack, if alignment is
        // such that that is required.
        for sp in chunks.into_remainder() {
            *sp = 0;
        }
        Ok(())
    }

    // Find the mapping which the given memory address falls in.
    pub fn find_mapping(&self, address: usize) -> Option<&MappingInfo> {
        self.mappings
            .iter()
            .find(|map| address >= map.start_address && address - map.start_address < map.size)
    }

    // Find the mapping which the given memory address falls in. Uses the
    // unadjusted mapping address range from the kernel, rather than the
    // biased range.
    pub fn find_mapping_no_bias(&self, address: usize) -> Option<&MappingInfo> {
        self.mappings.iter().find(|map| {
            address >= map.system_mapping_info.start_address
                && address < map.system_mapping_info.end_address
        })
    }

    pub fn from_process_memory_for_index<T: module_reader::ReadFromModule>(
        &mut self,
        idx: usize,
    ) -> Result<T, WriterError> {
        assert!(idx < self.mappings.len());

        Self::from_process_memory_for_mapping(&self.mappings[idx], self.process_id)
    }

    pub fn from_process_memory_for_mapping<T: module_reader::ReadFromModule>(
        mapping: &MappingInfo,
        pid: Pid,
    ) -> Result<T, WriterError> {
        Ok(T::read_from_module(
            module_reader::ProcessReader::new(pid, mapping.start_address).into(),
        )?)
    }
}

impl Drop for MinidumpWriter {
    fn drop(&mut self) {
        // Always try to resume all threads (e.g. in case of error)
        self.resume_threads(error_graph::strategy::DontCare);
        // Always allow the process to continue.
        let _ = self.continue_process();
    }
}

/// PTRACE_DETACH the given pid.
///
/// This handles special errno cases (ESRCH) which we won't consider errors.
fn ptrace_detach(child: Pid) -> Result<(), WriterError> {
    let pid = nix::unistd::Pid::from_raw(child);
    ptrace::detach(pid, None).or_else(|e| {
        // errno is set to ESRCH if the pid no longer exists, but we don't want to error in that
        // case.
        if e == nix::Error::ESRCH {
            Ok(())
        } else {
            Err(WriterError::PtraceDetachError(child, e))
        }
    })
}

fn write_file(
    buffer: &mut DumpBuf,
    filename: &str,
) -> std::result::Result<MDLocationDescriptor, MemoryWriterError> {
    let content = std::fs::read(filename)?;

    let section = MemoryArrayWriter::write_bytes(buffer, &content);
    Ok(section.location())
}

fn write_soft_errors(
    buffer: &mut DumpBuf,
    soft_errors: ErrorList<WriterError>,
) -> Result<MDLocationDescriptor, WriterError> {
    let soft_errors_json_str =
        serde_json::to_string_pretty(&soft_errors).map_err(WriterError::ConvertToJsonFailed)?;
    let section = MemoryArrayWriter::write_bytes(buffer, soft_errors_json_str.as_bytes());
    Ok(section.location())
}
