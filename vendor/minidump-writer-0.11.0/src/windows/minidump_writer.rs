#![allow(unsafe_code)]

use {
    super::{
        errors::Error,
        ffi::{
            capture_context, CloseHandle, GetCurrentProcess, GetCurrentThreadId, GetThreadContext,
            MiniDumpWriteDump, MinidumpType, OpenProcess, OpenThread, ResumeThread, SuspendThread,
            EXCEPTION_POINTERS, EXCEPTION_RECORD, FALSE, HANDLE, MINIDUMP_EXCEPTION_INFORMATION,
            MINIDUMP_USER_STREAM, MINIDUMP_USER_STREAM_INFORMATION, PROCESS_ALL_ACCESS,
            STATUS_NONCONTINUABLE_EXCEPTION, THREAD_GET_CONTEXT, THREAD_QUERY_INFORMATION,
            THREAD_SUSPEND_RESUME,
        },
    },
    minidump_common::format::{BreakpadInfoValid, MINIDUMP_BREAKPAD_INFO, MINIDUMP_STREAM_TYPE},
    scroll::Pwrite,
    std::os::windows::io::AsRawHandle,
};

pub struct MinidumpWriter {
    /// Optional exception information
    exc_info: Option<MINIDUMP_EXCEPTION_INFORMATION>,
    /// Handle to the crashing process, which could be ourselves
    crashing_process: HANDLE,
    /// The id of the process we are dumping
    pid: u32,
    /// The id of the 'crashing' thread
    tid: u32,
    /// The exception code for the dump
    #[allow(dead_code)]
    exception_code: i32,
    /// Whether we are dumping the current process or not
    is_external_process: bool,
}

impl MinidumpWriter {
    /// Creates a minidump of the current process, optionally including an
    /// exception code and the CPU context of the specified thread. If no thread
    /// is specified the current thread CPU context is used.
    ///
    /// Note that it is inherently unreliable to dump the currently running
    /// process, at least in the event of an actual exception. It is recommended
    /// to dump from an external process if possible via [`Self::dump_crash_context`]
    ///
    /// # Errors
    ///
    /// In addition to the errors described in [`Self::dump_crash_context`], this
    /// function can also fail if `thread_id` is specified and we are unable to
    /// acquire the thread's context
    pub fn dump_local_context(
        exception_code: Option<i32>,
        thread_id: Option<u32>,
        minidump_type: Option<MinidumpType>,
        destination: &mut std::fs::File,
    ) -> Result<(), Error> {
        let exception_code = exception_code.unwrap_or(STATUS_NONCONTINUABLE_EXCEPTION);

        // SAFETY: syscalls, while this encompasses most of the function, the user
        // has no invariants to uphold so the entire function is not marked unsafe
        unsafe {
            let mut exception_context = if let Some(tid) = thread_id {
                let mut ec = std::mem::MaybeUninit::uninit();

                // We need to suspend the thread to get its context, which would be bad
                // if it's the current thread, so we check it early before regrets happen
                if tid == GetCurrentThreadId() {
                    capture_context(ec.as_mut_ptr());
                } else {
                    // We _could_ just fallback to the current thread if we can't get the
                    // thread handle, but probably better for this to fail with a specific
                    // error so that the caller can do that themselves if they want to
                    // https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthread
                    let thread_handle = OpenThread(
                        THREAD_GET_CONTEXT | THREAD_QUERY_INFORMATION | THREAD_SUSPEND_RESUME, // desired access rights, we only need to get the context, which also requires suspension
                        FALSE, // inherit handles
                        tid,   // thread id
                    );

                    if thread_handle == 0 {
                        return Err(Error::ThreadOpen(std::io::Error::last_os_error()));
                    }

                    struct OwnedHandle(HANDLE);

                    impl Drop for OwnedHandle {
                        fn drop(&mut self) {
                            // SAFETY: syscall
                            unsafe { CloseHandle(self.0) };
                        }
                    }

                    let thread_handle = OwnedHandle(thread_handle);

                    // As noted in the GetThreadContext docs, we have to suspend the thread before we can get its context
                    // https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-suspendthread
                    if SuspendThread(thread_handle.0) == u32::MAX {
                        return Err(Error::ThreadSuspend(std::io::Error::last_os_error()));
                    }

                    // https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadcontext
                    if GetThreadContext(thread_handle.0, ec.as_mut_ptr()) == 0 {
                        // Try to be a good citizen and resume the thread
                        ResumeThread(thread_handle.0);

                        return Err(Error::ThreadContext(std::io::Error::last_os_error()));
                    }

                    // _presumably_ this will not fail if SuspendThread succeeded, but if it does
                    // there's really not much we can do about it, thus we don't bother checking the
                    // return value
                    ResumeThread(thread_handle.0);
                }

                ec.assume_init()
            } else {
                let mut ec = std::mem::MaybeUninit::uninit();
                capture_context(ec.as_mut_ptr());
                ec.assume_init()
            };

            let mut exception_record: EXCEPTION_RECORD = std::mem::zeroed();

            let exception_ptrs = EXCEPTION_POINTERS {
                ExceptionRecord: &mut exception_record,
                ContextRecord: &mut exception_context,
            };

            exception_record.ExceptionCode = exception_code;

            let cc = crash_context::CrashContext {
                exception_pointers: (&exception_ptrs as *const EXCEPTION_POINTERS).cast(),
                process_id: std::process::id(),
                thread_id: thread_id.unwrap_or_else(|| GetCurrentThreadId()),
                exception_code,
            };

            Self::dump_crash_context(&cc, minidump_type, destination)
        }
    }

    /// Writes a minidump for the context described by [`crash_context::CrashContext`].
    ///
    /// # Errors
    ///
    /// Fails if the process specified in the context is not the local process
    /// and we are unable to open it due to eg. security reasons, or we fail to
    /// write the minidump, which can be due to a host of issues with both acquiring
    /// the process information as well as writing the actual minidump contents to disk
    ///
    /// # Safety
    ///
    /// If [`crash_context::CrashContext::exception_pointers`] is specified, it
    /// is the responsibility of the caller to ensure that the pointer is valid
    /// for the duration of this function call.
    pub fn dump_crash_context(
        crash_context: &crash_context::CrashContext,
        minidump_type: Option<MinidumpType>,
        destination: &mut std::fs::File,
    ) -> Result<(), Error> {
        let pid = crash_context.process_id;

        // SAFETY: syscalls
        let (crashing_process, is_external_process) = unsafe {
            if pid != std::process::id() {
                let proc = OpenProcess(
                    PROCESS_ALL_ACCESS, // desired access
                    FALSE,              // inherit handles
                    pid,                // pid
                );

                if proc == 0 {
                    return Err(std::io::Error::last_os_error().into());
                }

                (proc, true)
            } else {
                (GetCurrentProcess(), false)
            }
        };

        let pid = crash_context.process_id;
        let tid = crash_context.thread_id;
        let exception_code = crash_context.exception_code;

        let exc_info = (!crash_context.exception_pointers.is_null()).then_some(
            // https://docs.microsoft.com/en-us/windows/win32/api/minidumpapiset/ns-minidumpapiset-minidump_exception_information
            MINIDUMP_EXCEPTION_INFORMATION {
                ThreadId: crash_context.thread_id,
                // This is a mut pointer for some reason...I don't _think_ it is
                // actually mut in practice...?
                ExceptionPointers: crash_context.exception_pointers as *mut _,
                // The `EXCEPTION_POINTERS` contained in crash context is a pointer into the
                // memory of the process that crashed, as it contains an `EXCEPTION_RECORD`
                // record which is an internally linked list, so in the case that we are
                // dumping a process other than the current one, we need to tell
                // `MiniDumpWriteDump` that the pointers come from an external process so that
                // it can use eg `ReadProcessMemory` to get the contextual information from
                // the crash, rather than from the current process
                ClientPointers: i32::from(is_external_process),
            },
        );

        let mdw = Self {
            exc_info,
            crashing_process,
            pid,
            tid,
            exception_code,
            is_external_process,
        };

        mdw.dump(minidump_type, destination)
    }

    /// Writes a minidump to the specified file
    fn dump(
        mut self,
        minidump_type: Option<MinidumpType>,
        destination: &mut std::fs::File,
    ) -> Result<(), Error> {
        let exc_info = self.exc_info.take();

        let mut user_streams = Vec::with_capacity(1);

        let mut breakpad_info = self.fill_breakpad_stream();

        if let Some(bp_info) = &mut breakpad_info {
            user_streams.push(MINIDUMP_USER_STREAM {
                Type: MINIDUMP_STREAM_TYPE::BreakpadInfoStream as u32,
                BufferSize: bp_info.len() as u32,
                // Again with the mut pointer
                Buffer: bp_info.as_mut_ptr().cast(),
            });
        }

        let user_stream_infos = MINIDUMP_USER_STREAM_INFORMATION {
            UserStreamCount: user_streams.len() as u32,
            UserStreamArray: user_streams.as_mut_ptr(),
        };

        // Write the actual minidump
        // https://docs.microsoft.com/en-us/windows/win32/api/minidumpapiset/nf-minidumpapiset-minidumpwritedump
        // SAFETY: syscall
        let ret = unsafe {
            MiniDumpWriteDump(
                self.crashing_process, // HANDLE to the process with the crash we want to capture
                self.pid,              // process id
                destination.as_raw_handle() as HANDLE, // file to write the minidump to
                minidump_type.unwrap_or(MinidumpType::Normal),
                exc_info
                    .as_ref()
                    .map_or(std::ptr::null(), |ei| ei as *const _), // exceptionparam - the actual exception information
                &user_stream_infos, // user streams
                std::ptr::null(),   // callback, unused
            )
        };

        if ret == 0 {
            Err(std::io::Error::last_os_error().into())
        } else {
            Ok(())
        }
    }

    /// Create an MDRawBreakpadInfo stream to the minidump, to provide additional
    /// information about the exception handler to the Breakpad processor.
    /// The information will help the processor determine which threads are
    /// relevant. The Breakpad processor does not require this information but
    /// can function better with Breakpad-generated dumps when it is present.
    /// The native debugger is not harmed by the presence of this information.
    ///
    /// This info is only relevant for in-process dumping
    fn fill_breakpad_stream(&self) -> Option<[u8; 12]> {
        if self.is_external_process {
            return None;
        }

        let mut breakpad_info = [0u8; 12];

        let bp_info = MINIDUMP_BREAKPAD_INFO {
            validity: BreakpadInfoValid::DumpThreadId.bits()
                | BreakpadInfoValid::RequestingThreadId.bits(),
            dump_thread_id: self.tid,
            // SAFETY: syscall
            requesting_thread_id: unsafe { GetCurrentThreadId() },
        };

        // TODO: derive Pwrite for MINIDUMP_BREAKPAD_INFO
        // https://github.com/rust-minidump/rust-minidump/pull/534
        let mut offset = 0;
        breakpad_info.gwrite(bp_info.validity, &mut offset).ok()?;
        breakpad_info
            .gwrite(bp_info.dump_thread_id, &mut offset)
            .ok()?;
        breakpad_info
            .gwrite(bp_info.requesting_thread_id, &mut offset)
            .ok()?;

        Some(breakpad_info)
    }
}

impl Drop for MinidumpWriter {
    fn drop(&mut self) {
        // Note we close the handle regardless of whether it is the local handle
        // or an external one, as noted in the docs
        //
        // > The pseudo handle need not be closed when it is no longer needed.
        // > Calling the CloseHandle function with a pseudo handle has no effect.
        // SAFETY: syscall
        unsafe { CloseHandle(self.crashing_process) };
    }
}
