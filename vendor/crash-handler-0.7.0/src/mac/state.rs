use super::ffi::*;
use crate::CrashEventResult;
use crate::Error;
use std::mem;

#[repr(u32)]
enum MessageIds {
    /// Message ID telling the handler thread to signal a crash w/optional exception information
    SignalCrash = 0,
    /// Message ID telling the handler thread to quit.
    Shutdown = 2,
    /// Taken from `mach_exc` in `/usr/include/mach/exc.defs`.
    Exception = 2405,
    ExceptionStateIdentity = 2407,
}

impl TryFrom<u32> for MessageIds {
    type Error = u32;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        Ok(match val {
            0 => Self::SignalCrash,
            2 => Self::Shutdown,
            2405 => Self::Exception,
            2407 => Self::ExceptionStateIdentity,
            unknown => return Err(unknown),
        })
    }
}

/// The exceptions that we want to handle, we note the ~equivalent signal next to each
const EXCEPTION_MASK: et::exception_mask_t = et::EXC_MASK_BAD_ACCESS // SIGSEGV/SIGBUS
    | et::EXC_MASK_BAD_INSTRUCTION // SIGILL
    | et::EXC_MASK_ARITHMETIC // SIGFPE
    | et::EXC_MASK_BREAKPOINT // SIGTRAP
    | et::EXC_MASK_CRASH // technically this is a catch all mask for the previous masks, but no reason not to spell out exactly what we handle
    | et::EXC_MASK_RESOURCE // exception thrown when a resource limit is exceeded
    | et::EXC_MASK_GUARD // exception thrown when an action that is guarded on a resource is attempted
    ;

static HANDLER: parking_lot::RwLock<Option<HandlerInner>> = parking_lot::const_rwlock(None);

#[inline]
pub(crate) fn kern_ret(func: impl FnOnce() -> kern_return_t) -> Result<(), Error> {
    let res = func();

    if res == KERN_SUCCESS {
        Ok(())
    } else {
        Err(std::io::Error::from_raw_os_error(res).into())
    }
}

struct PreviousPort {
    /// The exception the port is masking
    mask: et::exception_mask_t,
    /// The port that is sent messages when the exception occurs
    port: mach_port_t,
    /// The way in which the exception is raised
    behavior: et::exception_behavior_t,
    /// The CPU context included with the exception
    flavor: ts::thread_state_flavor_t,
}

struct PreviousPorts {
    count: usize,
    ports: [PreviousPort; EXC_TYPES_COUNT],
}

type UserSignal = std::sync::Arc<(parking_lot::Mutex<Option<bool>>, parking_lot::Condvar)>;

struct AllocatedPort {
    port: mach_port_t,
}

impl Drop for AllocatedPort {
    fn drop(&mut self) {
        unsafe {
            mp::mach_port_deallocate(mach_task_self(), self.port);
        }
    }
}

pub(super) struct HandlerInner {
    pub(super) crash_event: Box<dyn crate::CrashEvent>,
    handler_port: AllocatedPort,
    user_signal: UserSignal,
    handler_thread: std::thread::JoinHandle<()>,
    previous_abort_action: libc::sigaction,
    previous: PreviousPorts,
}

impl HandlerInner {
    /// Restores the previously registered signal handler and exception ports
    ///
    /// SAFETY: syscalls
    unsafe fn uninstall(&self) -> Result<(), Error> {
        unsafe {
            super::signal::restore_abort_handler(self.previous_abort_action);

            let current_task = mach_task_self();

            // Restore the previous ports
            for pp in &self.previous.ports[..self.previous.count] {
                kern_ret(|| {
                    task_set_exception_ports(current_task, pp.mask, pp.port, pp.behavior, pp.flavor)
                })?;
            }
        }

        Ok(())
    }

    /// SAFETY: syscalls
    unsafe fn shutdown(self, is_handler_thread: bool) -> Result<(), Error> {
        unsafe {
            self.uninstall()?;

            let mut exc_msg: UserException = mem::zeroed();
            exc_msg.header.msgh_id = MessageIds::Shutdown as i32;

            if self.send_message(exc_msg) {
                // We don't really care if there was some error in the thread, note
                // that we check the thread in case we're being uninstalled from
                // the handler thread itself
                if !is_handler_thread {
                    let _res = self.handler_thread.join();
                }
            }
        }

        Ok(())
    }

    /// SAFETY: syscalls
    unsafe fn send_message(&self, mut msg: UserException) -> bool {
        unsafe {
            msg.header.msgh_size = mem::size_of_val(&msg) as u32;
            msg.header.msgh_remote_port = self.handler_port.port;

            // Reset the condition variable in case a user signal was already raised
            {
                let (lock, _cvar) = &*self.user_signal;
                *lock.lock() = None;
            }

            if msg::mach_msg(
                (&mut msg.header) as *mut _,
                msg::MACH_SEND_MSG,
                msg.header.msgh_size,
                0,
                0,
                msg::MACH_MSG_TIMEOUT_NONE,
                MACH_PORT_NULL,
            ) != KERN_SUCCESS
            {
                return false;
            }

            if msg.header.msgh_id != MessageIds::SignalCrash as i32 {
                true
            } else {
                // Wait on the handler thread to signal the user callback has finished
                // with the exception
                let (lock, cvar) = &*self.user_signal;
                let mut processed = lock.lock();
                if processed.is_none() {
                    cvar.wait(&mut processed);
                }

                processed.unwrap_or(false)
            }
        }
    }
}

/// The thread that is actually handling the exception port.
static HANDLER_THREAD: parking_lot::Mutex<Option<mach_port_t>> = parking_lot::const_mutex(None);

/// Creates a new `mach_port` and installs it as the new task (process) exception
/// port so that any exceptions not handled by a thread specific exception port
/// are sent to it, as well as a signal handler for `SIGABRT` as it is not an
/// exception on macos.
///
/// This spawns a message loop thread that waits on messages to the exception port.
///
/// # Errors
///
/// - A handler has already been installed, we only allow one
/// - Any of the various syscalls that are made fail
pub(super) fn attach(crash_event: Box<dyn crate::CrashEvent>) -> Result<(), Error> {
    let mut lock = HANDLER.write();

    if lock.is_some() {
        return Err(Error::HandlerAlreadyInstalled);
    }

    // SAFETY: this is basically just a lot of syscalls we're doing
    unsafe {
        let current_task = mach_task_self();

        let mut handler_port = MACH_PORT_NULL;

        // Create a receive right so that we can actually receive exception messages on the port
        kern_ret(|| {
            mp::mach_port_allocate(
                current_task,
                port::MACH_PORT_RIGHT_RECEIVE,
                &mut handler_port,
            )
        })?;

        let handler_port = AllocatedPort { port: handler_port };

        // Add send right
        kern_ret(|| {
            mp::mach_port_insert_right(
                current_task,
                handler_port.port,
                handler_port.port,
                msg::MACH_MSG_TYPE_MAKE_SEND,
            )
        })?;

        let previous_abort_action = super::signal::install_abort_handler()?;

        let mut count = EXC_TYPES_COUNT as u32;
        let mut masks = [0; EXC_TYPES_COUNT];
        let mut ports = [0; EXC_TYPES_COUNT];
        let mut behaviors = [0; EXC_TYPES_COUNT];
        let mut flavors = [0; EXC_TYPES_COUNT];

        let behavior =
            // Send a catch_exception_raise message including the identity.
            et::EXCEPTION_DEFAULT |
            // Send 64-bit code and subcode in the exception header.
            //
            // Without this flag the code and subcode in the exception will be
            // 32-bits, losing information for several types of exception
            // * `EXC_BAD_ACCESS` - the address of the bad access is stored in the subcode
            // * `EXC_RESOURCE` - the details of the resource exception are stored
            // using the full 64-bits of the code
            // * `EXC_GUARD` - the details of the guard exception are stored
            // in the full 64-bits of the code, and the full 64-bits of the subcode
            // _can_ be used depending on the guard type
            et::MACH_EXCEPTION_CODES;

        // Swap the exception ports so that we use our own
        kern_ret(|| {
            task_swap_exception_ports(
                current_task,
                EXCEPTION_MASK,
                handler_port.port,
                behavior as _,
                THREAD_STATE_NONE,
                masks.as_mut_ptr(),
                &mut count,
                ports.as_mut_ptr(),
                behaviors.as_mut_ptr(),
                flavors.as_mut_ptr(),
            )
        })?;

        let mut previous: PreviousPorts = std::mem::zeroed();
        previous.count = count as usize;
        for i in 0..previous.count {
            previous.ports[i] = PreviousPort {
                mask: masks[i],
                port: ports[i],
                behavior: behaviors[i],
                flavor: flavors[i],
            };
        }

        let user_signal =
            std::sync::Arc::new((parking_lot::Mutex::new(None), parking_lot::Condvar::new()));
        let us = user_signal.clone();

        let port = handler_port.port;

        // Spawn a thread that will handle the actual exception/user messages sent
        // to the exception port we've just created
        let handler_thread = std::thread::spawn(move || {
            *HANDLER_THREAD.lock() = Some(mach_thread_self());

            exception_handler(port, us);

            *HANDLER_THREAD.lock() = None;
        });

        *lock = Some(HandlerInner {
            crash_event,
            handler_port,
            user_signal,
            handler_thread,
            previous_abort_action,
            previous,
        });
    }

    Ok(())
}

pub(super) fn detach(is_handler_thread: bool) {
    let mut lock = HANDLER.write();
    if let Some(handler) = lock.take() {
        // user can't really do anything if something fails at this point, but
        // should have a clean way of surfacing the error happened
        // SAFETY: syscalls
        let _result = unsafe { handler.shutdown(is_handler_thread) };
    }
}

#[repr(C)]
struct UserException {
    header: msg::mach_msg_header_t,
    body: msg::mach_msg_body_t,
    crash_thread: msg::mach_msg_port_descriptor_t,
    flags: u32,
    exception_kind: u32,
    exception_code: u64,
    exception_subcode: u64,
}

const FLAG_HAS_EXCEPTION: u32 = 0x1;
const FLAG_HAS_SUBCODE: u32 = 0x2;

pub(super) fn simulate_exception(info: Option<crash_context::ExceptionInfo>) -> bool {
    let lock = HANDLER.read();
    if let Some(handler) = &*lock {
        // SAFETY: ExceptionMessage is POD and send_message is syscalls
        unsafe {
            let (flags, exception_kind, exception_code, exception_subcode) = if let Some(exc) = info
            {
                (
                    FLAG_HAS_EXCEPTION
                        | if exc.subcode.is_some() {
                            FLAG_HAS_SUBCODE
                        } else {
                            0
                        },
                    exc.kind,
                    exc.code,
                    exc.subcode.unwrap_or_default(),
                )
            } else {
                (0, 0, 0, 0)
            };

            let exc_msg = UserException {
                header: msg::mach_msg_header_t {
                    msgh_bits: msg::MACH_MSG_TYPE_COPY_SEND | msg::MACH_MSGH_BITS_COMPLEX,
                    msgh_size: std::mem::size_of::<UserException>() as u32,
                    msgh_remote_port: port::MACH_PORT_NULL,
                    msgh_local_port: port::MACH_PORT_NULL,
                    msgh_voucher_port: port::MACH_PORT_NULL,
                    msgh_id: 0,
                },
                body: msg::mach_msg_body_t {
                    msgh_descriptor_count: 1,
                },
                crash_thread: msg::mach_msg_port_descriptor_t::new(
                    mach_thread_self(),
                    msg::MACH_MSG_TYPE_COPY_SEND,
                ),
                flags,
                exception_kind,
                exception_code,
                exception_subcode,
            };

            handler.send_message(exc_msg)
        }
    } else {
        false
    }
}

#[inline]
fn call_user_callback(cc: &crash_context::CrashContext) -> CrashEventResult {
    let lock = HANDLER.read();
    if let Some(handler) = &*lock {
        handler.crash_event.on_crash(cc)
    } else {
        CrashEventResult::Handled(false)
    }
}

/// Message loop thread. Simply waits for messages to the port, which will either
/// be exceptions sent by the kernel, or messages sent by the exception handler
/// that this message loop is servicing.
unsafe fn exception_handler(port: mach_port_t, us: UserSignal) {
    unsafe {
        let mut request: ExceptionMessage = mem::zeroed();

        loop {
            request.header.local_port = port;
            request.header.size = mem::size_of_val(&request) as _;

            let kret = msg::mach_msg(
                ((&mut request.header) as *mut MachMsgHeader).cast(),
                msg::MACH_RCV_MSG | msg::MACH_RCV_LARGE,
                0,
                mem::size_of_val(&request) as u32,
                port,
                msg::MACH_MSG_TIMEOUT_NONE,
                MACH_PORT_NULL,
            );

            if kret != KERN_SUCCESS {
                eprintln!("mach_msg failed with {} ({0:x})", kret);
                libc::abort();
            }

            match MessageIds::try_from(request.header.id) {
                Ok(MessageIds::Exception | MessageIds::ExceptionStateIdentity) => {
                    // When forking a child process with the exception handler installed,
                    // if the child crashes, it will send the exception back to the parent
                    // process.  The check for task == self_task() ensures that only
                    // exceptions that occur in the parent process are caught and
                    // processed.  If the exception was not caused by this task, we
                    // still need to call into the exception server and have it return
                    // KERN_FAILURE (see catch_exception_raise) in order for the kernel
                    // to move onto the host exception handler for the child task
                    let ret_code = if request.task.name == mach_task_self() {
                        let _ss = ScopedSuspend::new();

                        let subcode = (request.code_count > 1).then_some(request.code[1]);

                        let exc_info = crash_context::ExceptionInfo {
                            kind: request.exception,
                            code: request.code[0],
                            subcode,
                        };

                        // Check if the exception is non-fatal, if it is we don't report it
                        // and importantly _don't_ detach the exception handler like we
                        // do for fatal exceptions
                        if !is_exception_non_fatal(exc_info, request.task.name) {
                            let cc = crash_context::CrashContext {
                                thread: request.thread.name,
                                task: request.task.name,
                                handler_thread: mach_thread_self(),
                                exception: Some(exc_info),
                            };

                            let ret_code =
                                if let CrashEventResult::Handled(true) = call_user_callback(&cc) {
                                    KERN_SUCCESS
                                } else {
                                    mach2::kern_return::KERN_FAILURE
                                };

                            // Restores the previous exception ports, in most cases
                            // this will be the default for the OS, which will kill this
                            // process when we reply that we've handled the exception
                            detach(true);

                            ret_code
                        } else {
                            KERN_SUCCESS
                        }
                    } else {
                        KERN_SUCCESS
                    };

                    // This magic incantation to send a reply back to the kernel was
                    // derived from the exc_server generated by
                    // 'mig -v /usr/include/mach/mach_exc.defs', or you can look at
                    // https://github.com/doadam/xnu-4570.1.46/blob/2ad7fbf85ff567495a572cd4583961ffd8525083/BUILD/obj/RELEASE_X86_64/osfmk/RELEASE/mach/exc_server.c#L491-L520
                    let mut reply: ExceptionRaiseReply = mem::zeroed();
                    reply.header.bits = msg::MACH_MSGH_BITS(
                        request.header.bits & msg::MACH_MSGH_BITS_REMOTE_MASK,
                        0,
                    );
                    reply.header.size = mem::size_of_val(&reply) as u32;
                    reply.header.remote_port = request.header.remote_port;
                    reply.header.local_port = MACH_PORT_NULL;
                    reply.header.id = request.header.id + 100;
                    reply.ndr = NDR_record;
                    reply.ret_code = ret_code;

                    msg::mach_msg(
                        ((&mut reply.header) as *mut MachMsgHeader).cast(),
                        msg::MACH_SEND_MSG,
                        mem::size_of_val(&reply) as u32,
                        0,
                        MACH_PORT_NULL,
                        msg::MACH_MSG_TIMEOUT_NONE,
                        MACH_PORT_NULL,
                    );
                }
                Ok(MessageIds::Shutdown) => return,
                Ok(MessageIds::SignalCrash) => {
                    let res = {
                        let _ss = ScopedSuspend::new();

                        let user_exception: &UserException = std::mem::transmute(&request);

                        let exception = if user_exception.flags & FLAG_HAS_EXCEPTION != 0 {
                            Some(crash_context::ExceptionInfo {
                                kind: user_exception.exception_kind,
                                code: user_exception.exception_code,
                                subcode: (user_exception.flags & FLAG_HAS_SUBCODE != 0)
                                    .then_some(user_exception.exception_subcode),
                            })
                        } else {
                            None
                        };

                        // Reconstruct a crash context from the message we received
                        let cc = crash_context::CrashContext {
                            task: mach_task_self(),
                            thread: user_exception.crash_thread.name,
                            handler_thread: mach_thread_self(),
                            exception,
                        };

                        call_user_callback(&cc)
                    };

                    {
                        let (lock, cvar) = &*us;
                        let mut processed = lock.lock();
                        *processed = Some(matches!(res, CrashEventResult::Handled(true)));
                        cvar.notify_one();
                    }
                }
                Err(unknown) => unreachable!("received unknown message {unknown}"),
            }
        }
    }
}

struct ScopedSuspend;

impl ScopedSuspend {
    fn new() -> Self {
        // SAFETY: syscalls
        unsafe {
            let mut threads_for_task = std::ptr::null_mut();
            let mut thread_count = 0;

            if task::task_threads(mach_task_self(), &mut threads_for_task, &mut thread_count)
                != KERN_SUCCESS
            {
                return Self;
            }

            let this_thread = mach_thread_self();
            let threads = std::slice::from_raw_parts(threads_for_task, thread_count as usize);

            // suspend all of the threads except for this one
            for thread in threads {
                if *thread != this_thread {
                    // We try to suspend all threads as a best effort, it's not fatal
                    // if we can't
                    mach2::thread_act::thread_suspend(*thread);
                }
            }
        }

        Self
    }
}

impl Drop for ScopedSuspend {
    fn drop(&mut self) {
        // SAFETY: syscalls
        unsafe {
            let mut threads_for_task = std::ptr::null_mut();
            let mut thread_count = 0;

            if task::task_threads(mach_task_self(), &mut threads_for_task, &mut thread_count)
                != KERN_SUCCESS
            {
                return;
            }

            let this_thread = mach_thread_self();
            let threads = std::slice::from_raw_parts(threads_for_task, thread_count as usize);

            // resume all of the threads except for this one
            for thread in threads {
                if *thread != this_thread {
                    mach2::thread_act::thread_resume(*thread);
                }
            }
        }
    }
}

/// Determines if the specified exception is non-fatal.
///
/// `EXC_RESOURCE` exceptions are thrown when a resource limit (hard or soft)
/// is surpassed, and importantly for our scenario, are often non-fatal, meaning
/// we should _not_ notify the user callback that a crash has occurred
fn is_exception_non_fatal(exc_info: crash_context::ExceptionInfo, task: mt::task_t) -> bool {
    use crash_context::{
        ipc::pid_for_task,
        resource::{self as res, ResourceException as Re},
    };

    // We want to clearly see the different variants, even if they end up with
    // the same result
    #[allow(clippy::match_same_arms)]
    match exc_info.resource_exception() {
        // CPU exceptions have, currently 2 flavors, fata and non-fatal
        Some(Re::Cpu(cpu_exc)) => !cpu_exc.is_fatal,
        // Wakeups exceptions
        Some(Re::Wakeups(wu_exc)) if wu_exc.flavor == res::WakeupsFlavor::Monitor => {
            // Unlike the other resource exceptions kinds, we need to call into
            // the task to determine if this exception is actually fatal. Since
            // it is by default not fatal, failure to retrieve the task's pid
            // or calling proc_get_wakemon_params will consider the exception
            // non-fatal
            let mut pid = 0;
            // SAFETY: syscall
            if unsafe { pid_for_task(task, &mut pid) } != KERN_SUCCESS {
                return true;
            }

            // The SDK doesn’t have `proc_get_wakemon_params` to link against,
            // even with weak import, so we need need to look it up by name
            // before invoking it
            // SAFETY: syscalls
            unsafe {
                let mut dl_info = std::mem::MaybeUninit::uninit();
                if libc::dladdr(libc::proc_pidinfo as *const _, dl_info.as_mut_ptr()) == 0 {
                    // We failed to find the lib that contains proc_pidinfo, which
                    // is the same lib that contains proc_get_wakemon_params
                    return true;
                }

                let dl_info = dl_info.assume_init();

                let dl_handle = libc::dlopen(
                    dl_info.dli_fname,
                    libc::RTLD_LAZY | libc::RTLD_LOCAL | libc::RTLD_NOLOAD,
                );
                if dl_handle.is_null() {
                    return true;
                }

                type ProcGetWakemonParams = unsafe extern "C" fn(
                    pid: libc::pid_t,
                    rate_hz: *mut i32,
                    flags: *mut i32,
                ) -> i32;

                let proc_get_wakemon_params =
                    libc::dlsym(dl_handle, c"proc_get_wakemon_params".as_ptr().cast());
                if proc_get_wakemon_params.is_null() {
                    return true;
                }

                let proc_get_wakemon_params: ProcGetWakemonParams =
                    std::mem::transmute(proc_get_wakemon_params);

                let mut rate = 0;
                let mut flags = 0;
                if proc_get_wakemon_params(pid, &mut rate, &mut flags) < 0 {
                    return true;
                }

                // Configure the task so that violations are fatal. <include/sys/resource.h>
                const WAKEMON_MAKE_FATAL: i32 = 0x10;

                (flags & WAKEMON_MAKE_FATAL) == 0
            }
        }
        // Memory resource exceptions are never fatal
        Some(Re::Memory(mem_exc)) if mem_exc.flavor == res::MemoryFlavor::HighWatermark => true,
        // I/O resource exeptions are never fatal
        Some(Re::Io(_)) => true,
        // Thread resource exceptions are not possible (at least currently) in production kernels
        Some(Re::Threads(_)) => false,
        // Port resource exceptions are always fatal
        Some(Re::Ports(_)) => false,
        // non resource exceptions are always fatal
        None => false,
        // TODO: print out details on the unknown exception?
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crash_context::{resource::*, *};

    #[inline]
    fn res_exc(kind: ResourceKind, flavor: u8) -> ExceptionInfo {
        ExceptionInfo {
            kind: et::EXC_RESOURCE as _,
            code: (((kind as u64 & 0x7) << 61) | ((flavor as u64 & 0x7) << 58)),
            subcode: None,
        }
    }

    /// Ensures that the resource exceptions can be non-fatal and all other
    /// exceptions we handle are always fatal
    #[test]
    fn check_resource_non_fatal() {
        // SAFETY: syscall
        let task = unsafe { mach_task_self() };

        assert!(is_exception_non_fatal(
            res_exc(ResourceKind::Cpu, CpuFlavor::Monitor as u8),
            task
        ));
        assert!(!is_exception_non_fatal(
            res_exc(ResourceKind::Cpu, CpuFlavor::MonitorFatal as u8),
            task
        ));

        // This assumes that WAKEMON_MAKE_FATAL is not set for this process. The
        // default is for WAKEMON_MAKE_FATAL to not be set, there’s no public API to
        // enable it, and nothing in this process should have enabled it.
        assert!(is_exception_non_fatal(
            res_exc(ResourceKind::Wakeups, WakeupsFlavor::Monitor as u8),
            task
        ));

        assert!(is_exception_non_fatal(
            res_exc(ResourceKind::Memory, MemoryFlavor::HighWatermark as u8),
            task
        ));

        assert!(!is_exception_non_fatal(
            ExceptionInfo {
                kind: et::EXC_CRASH as _,
                code: 0xb100001,
                subcode: None,
            },
            task
        ));
        assert!(!is_exception_non_fatal(
            ExceptionInfo {
                kind: et::EXC_CRASH as _,
                code: 0x0b00000,
                subcode: None,
            },
            task
        ));
        assert!(!is_exception_non_fatal(
            ExceptionInfo {
                kind: et::EXC_CRASH as _,
                code: 0x6000000,
                subcode: None,
            },
            task
        ));
        assert!(!is_exception_non_fatal(
            ExceptionInfo {
                kind: et::EXC_BAD_ACCESS as _,
                code: mach2::kern_return::KERN_INVALID_ADDRESS as _,
                subcode: None,
            },
            task
        ));
        assert!(!is_exception_non_fatal(
            ExceptionInfo {
                kind: et::EXC_BAD_INSTRUCTION as _,
                code: 1,
                subcode: None,
            },
            task
        ));
        assert!(!is_exception_non_fatal(
            ExceptionInfo {
                kind: et::EXC_ARITHMETIC as _,
                code: 1,
                subcode: None,
            },
            task
        ));
        assert!(!is_exception_non_fatal(
            ExceptionInfo {
                kind: et::EXC_BREAKPOINT as _,
                code: 2,
                subcode: None,
            },
            task
        ));
        assert!(!is_exception_non_fatal(
            ExceptionInfo {
                kind: 0,
                code: 0,
                subcode: None,
            },
            task
        ));
    }
}
