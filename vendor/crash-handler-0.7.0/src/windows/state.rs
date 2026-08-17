#![allow(non_camel_case_types, clippy::exit)]

use super::ExceptionCode;
use crate::Error;

type LPTOP_LEVEL_EXCEPTION_FILTER = Option<
    unsafe extern "system" fn(exceptioninfo: *const crash_context::EXCEPTION_POINTERS) -> i32,
>;

type PVECTORED_EXCEPTION_HANDLER = Option<
    unsafe extern "system" fn(exceptioninfo: *const crash_context::EXCEPTION_POINTERS) -> i32,
>;

unsafe extern "system" {
    fn GetCurrentThreadId() -> u32;
    fn SetUnhandledExceptionFilter(
        filter: LPTOP_LEVEL_EXCEPTION_FILTER,
    ) -> LPTOP_LEVEL_EXCEPTION_FILTER;
    fn AddVectoredExceptionHandler(
        first_handler: u32,
        handler: PVECTORED_EXCEPTION_HANDLER,
    ) -> *mut core::ffi::c_void;
    fn RemoveVectoredExceptionHandler(handle: *mut core::ffi::c_void) -> u32;
}

struct VehHandler(std::ptr::NonNull<libc::c_void>);
unsafe impl Send for VehHandler {}
unsafe impl Sync for VehHandler {}

unsafe extern "C" {
    /// MSVCRT has its own error handling function for invalid parameters to crt functions
    /// (eg printf) which instead of returning error codes from the function itself,
    /// like one would want, call a handler if specified, or, worse, throw up a dialog
    /// if in a GUI!
    ///
    /// [Invalid Parameter Handler](https://docs.microsoft.com/en-us/cpp/c-runtime-library/reference/set-invalid-parameter-handler-set-thread-local-invalid-parameter-handler?view=msvc-170)
    fn _set_invalid_parameter_handler(
        new_handler: Option<_invalid_parameter_handler>,
    ) -> Option<_invalid_parameter_handler>;
    /// It also has a separate error handling function when calling pure virtuals
    /// because why not?
    ///
    /// [Purecall Handler](https://docs.microsoft.com/en-us/cpp/c-runtime-library/reference/get-purecall-handler-set-purecall-handler?view=msvc-170)
    fn _set_purecall_handler(new_handler: Option<_purecall_handler>) -> Option<_purecall_handler>;
    // This is only available in the debug CRT
    // fn _invalid_parameter(
    //     expression: *const u16,
    //     function: *const u16,
    //     file: *const u16,
    //     line: u32,
    //     reserved: usize,
    // );
    fn _invalid_parameter_noinfo_noreturn() -> !;
    fn _invoke_watson() -> !;
}

type _invalid_parameter_handler = unsafe extern "C" fn(
    expression: *const u16,
    function: *const u16,
    file: *const u16,
    line: u32,
    _reserved: usize,
);
type _purecall_handler = unsafe extern "C" fn();

pub(super) static HANDLER: parking_lot::Mutex<Option<HandlerInner>> =
    parking_lot::const_mutex(None);

pub(super) struct HandlerInner {
    pub(super) user_handler: Box<dyn crate::CrashEvent>,
    /// The previously installed filter before this handler installed its own
    previous_filter: LPTOP_LEVEL_EXCEPTION_FILTER,
    /// The previously installed invalid parameter handler
    previous_iph: Option<_invalid_parameter_handler>,
    /// The previously installed purecall handler
    previous_pch: Option<_purecall_handler>,
    /// The previously installed SIGABRT handler
    previous_abort_handler: Option<libc::sighandler_t>,
    /// The handle of our own vectored exception handler
    veh_handle: Option<VehHandler>,
}

impl HandlerInner {
    pub(crate) fn new(user_handler: Box<dyn crate::CrashEvent>) -> Self {
        // Note that breakpad has flags so the user can choose which error handlers
        // to install, but for now we just install all of them

        // SAFETY: syscalls
        unsafe {
            let previous_filter = SetUnhandledExceptionFilter(Some(handle_exception));
            let previous_iph = _set_invalid_parameter_handler(Some(handle_invalid_parameter));
            let previous_pch = _set_purecall_handler(Some(handle_pure_virtual_call));
            let previous_abort_handler = super::signal::install_abort_handler().ok();
            let veh_handle = AddVectoredExceptionHandler(1, Some(vectored_handle_exception));
            let veh_handle = std::ptr::NonNull::new(veh_handle).map(VehHandler);

            Self {
                user_handler,
                previous_filter,
                previous_iph,
                previous_pch,
                previous_abort_handler,
                veh_handle,
            }
        }
    }

    /// Sets the handlers to the previous handlers that were registered when the
    /// specified handler was attached
    pub(crate) fn restore_previous_handlers(&mut self) {
        // SAFETY: syscalls
        unsafe {
            if let Some(ah) = self.previous_abort_handler {
                super::signal::restore_abort_handler(ah);
            }
            SetUnhandledExceptionFilter(self.previous_filter);
            _set_invalid_parameter_handler(self.previous_iph);
            _set_purecall_handler(self.previous_pch);
            if let Some(handler) = self.veh_handle.take() {
                RemoveVectoredExceptionHandler(handler.0.as_ptr());
            }
        }
    }
}

impl Drop for HandlerInner {
    fn drop(&mut self) {
        self.restore_previous_handlers();
    }
}

pub(super) fn attach(on_crash: Box<dyn crate::CrashEvent>) -> Result<(), Error> {
    let mut lock = HANDLER.lock();

    if lock.is_some() {
        return Err(Error::HandlerAlreadyInstalled);
    }

    *lock = Some(HandlerInner::new(on_crash));
    Ok(())
}

pub(super) fn detach() {
    let mut lock = HANDLER.lock();
    // The previous handlers are restored on drop
    lock.take();
}

pub(super) unsafe fn simulate_exception(exception_code: Option<i32>) -> crate::CrashEventResult {
    let lock = HANDLER.lock();
    if let Some(handler) = &*lock {
        unsafe {
            // https://github.com/chromium/crashpad/blob/fca8871ca3fb721d3afab370ca790122f9333bfd/util/win/exception_codes.h#L32
            let exception_code = exception_code.unwrap_or(ExceptionCode::User as i32);
            let mut exception_record = crash_context::EXCEPTION_RECORD {
                ExceptionCode: exception_code,
                ..std::mem::zeroed()
            };
            let mut exception_context = std::mem::MaybeUninit::zeroed();

            crash_context::capture_context(exception_context.as_mut_ptr());

            let mut exception_context = exception_context.assume_init();

            let exception_ptrs = crash_context::EXCEPTION_POINTERS {
                ExceptionRecord: &mut exception_record,
                ContextRecord: &mut exception_context,
            };

            let cc = crash_context::CrashContext {
                exception_pointers: (&exception_ptrs as *const crash_context::EXCEPTION_POINTERS)
                    .cast(),
                process_id: std::process::id(),
                thread_id: GetCurrentThreadId(),
                exception_code,
            };

            handler.user_handler.on_crash(&cc)
        }
    } else {
        crate::CrashEventResult::Handled(false)
    }
}

/// While handling any exceptions, especially when calling user code, we restore
/// and previously registered handlers
/// Note this keeps the `HANDLER` lock for the duration of the scope
struct AutoHandler<'scope> {
    lock: parking_lot::MutexGuard<'scope, Option<HandlerInner>>,
}

impl<'scope> AutoHandler<'scope> {
    fn new(mut lock: parking_lot::MutexGuard<'scope, Option<HandlerInner>>) -> Option<Self> {
        if let Some(hi) = &mut *lock {
            // In case another exception occurs while this handler is doing its thing,
            // it should be delivered to the previous filter.
            hi.restore_previous_handlers();
        }

        if lock.is_some() {
            Some(Self { lock })
        } else {
            None
        }
    }
}

/// Sets the handlers back to our internal ones
fn set_handlers() {
    unsafe {
        SetUnhandledExceptionFilter(Some(handle_exception));
        _set_invalid_parameter_handler(Some(handle_invalid_parameter));
        _set_purecall_handler(Some(handle_pure_virtual_call));
    }
}

impl std::ops::Deref for AutoHandler<'_> {
    type Target = HandlerInner;

    fn deref(&self) -> &Self::Target {
        self.lock.as_ref().unwrap()
    }
}

impl Drop for AutoHandler<'_> {
    fn drop(&mut self) {
        // Restore our handlers
        set_handlers();
    }
}

/// The handler is not entered, and the OS continues searching for an exception handler.
const EXCEPTION_CONTINUE_SEARCH: i32 = 0;
/// Enter the exception handler.
pub(super) const EXCEPTION_EXECUTE_HANDLER: i32 = 1;

use crate::CrashEventResult;

/// Called on the exception thread when an unhandled exception occurs.
/// Signals the exception handler thread to handle the exception.
pub(super) unsafe extern "system" fn handle_exception(
    except_info: *const crash_context::EXCEPTION_POINTERS,
) -> i32 {
    let _jump = unsafe {
        let lock = HANDLER.lock();
        if let Some(current_handler) = AutoHandler::new(lock) {
            let code = (*(*except_info).ExceptionRecord).ExceptionCode;

            match current_handler.user_handler.on_crash(&crate::CrashContext {
                exception_pointers: except_info.cast(),
                process_id: std::process::id(),
                thread_id: GetCurrentThreadId(),
                exception_code: code as _,
            }) {
                CrashEventResult::Handled(true) => {
                    // The handler fully handled the exception.  Returning
                    // EXCEPTION_EXECUTE_HANDLER indicates this to the system, and usually
                    // results in the application being terminated.
                    //
                    // Note: If the application was launched from within the Cygwin
                    // environment, returning EXCEPTION_EXECUTE_HANDLER seems to cause the
                    // application to be restarted.
                    return EXCEPTION_EXECUTE_HANDLER;
                }
                CrashEventResult::Handled(false) => {
                    // There was an exception, it was a breakpoint or something else ignored
                    // above, or it was passed to the handler, which decided not to handle it.
                    // Give the previous handler a chance to do something with the exception.
                    // If there is no previous handler, return EXCEPTION_CONTINUE_SEARCH,
                    // which will allow a debugger or native "crashed" dialog to handle the
                    // exception.
                    return if let Some(previous) = current_handler.previous_filter {
                        previous(except_info)
                    } else {
                        EXCEPTION_CONTINUE_SEARCH
                    };
                }
                #[cfg(target_arch = "x86_64")]
                CrashEventResult::Jump { jmp_buf, value } => (jmp_buf, value),
            }
        } else {
            return EXCEPTION_CONTINUE_SEARCH;
        }
    };

    #[cfg(target_arch = "x86_64")]
    unsafe {
        super::jmp::longjmp(_jump.0, _jump.1)
    };
}

const STATUS_HEAP_CORRUPTION: u32 = 0xc0000374;

/// Called on the exception thread when an exception occurs.
/// Gets to act before other exception handlers.
pub(super) unsafe extern "system" fn vectored_handle_exception(
    except_info: *const crash_context::EXCEPTION_POINTERS,
) -> i32 {
    unsafe {
        let exception_code = (*(*except_info).ExceptionRecord).ExceptionCode as u32;
        if exception_code == STATUS_HEAP_CORRUPTION {
            handle_exception(except_info)
        } else {
            EXCEPTION_CONTINUE_SEARCH
        }
    }
}

/// Handler for invalid parameters to CRT functions, this is not an exception so
/// the context (shouldn't be) isn't compromised
///
/// As noted [here](https://docs.microsoft.com/en-us/cpp/c-runtime-library/reference/set-invalid-parameter-handler-set-thread-local-invalid-parameter-handler?view=msvc-170#remarks)
/// the parameters to this function are useless when not linked against the debug
/// CRT, and rust std itself is only ever linked aginst the [non-debug CRT](https://github.com/rust-lang/rust/issues/39016)
/// and you can't really link both the regular and debug CRT in the same application
/// as that results in sadness, so this function just ignores the parameters,
/// unlike the original Breakpad code.
#[unsafe(no_mangle)]
unsafe extern "C" fn handle_invalid_parameter(
    expression: *const u16,
    function: *const u16,
    file: *const u16,
    line: u32,
    reserved: usize,
) {
    let _jump = unsafe {
        let lock = HANDLER.lock();
        if let Some(current_handler) = AutoHandler::new(lock) {
            // Make up an exception record for the current thread and CPU context
            // to make it possible for the crash processor to classify these
            // as do regular crashes, and to make it humane for developers to
            // analyze them.
            let exception_code = ExceptionCode::InvalidParameter as i32;
            let mut exception_record = crash_context::EXCEPTION_RECORD {
                ExceptionCode: exception_code,
                ..std::mem::zeroed()
            };
            let mut exception_context = std::mem::MaybeUninit::zeroed();

            crash_context::capture_context(exception_context.as_mut_ptr());

            let mut exception_context = exception_context.assume_init();

            let exception_ptrs = crash_context::EXCEPTION_POINTERS {
                ExceptionRecord: &mut exception_record,
                ContextRecord: &mut exception_context,
            };

            match current_handler.user_handler.on_crash(&crate::CrashContext {
                exception_pointers: (&exception_ptrs as *const crash_context::EXCEPTION_POINTERS)
                    .cast(),
                process_id: std::process::id(),
                thread_id: GetCurrentThreadId(),
                exception_code,
            }) {
                CrashEventResult::Handled(true) => return,
                CrashEventResult::Handled(false) => {
                    if let Some(prev_iph) = current_handler.previous_iph {
                        prev_iph(expression, function, file, line, reserved);
                    } else {
                        // If there's no previous handler, pass the exception back in to the
                        // invalid parameter handler's core.  That's the routine that called this
                        // function, but now, since this function is no longer registered (and in
                        // fact, no function at all is registered), this will result in the
                        // default code path being taken: _CRT_DEBUGGER_HOOK and _invoke_watson.
                        // Use _invalid_parameter where it exists (in _DEBUG builds) as it passes
                        // more information through.  In non-debug builds, it is not available,
                        // so fall back to using _invalid_parameter_noinfo.  See invarg.c in the
                        // CRT source.

                        // _invalid_parameter is only available in the debug CRT
                        _invoke_watson();
                        // if expression.is_null() && function.is_null() && file.is_null() {
                        //     _invalid_parameter_noinfo();
                        // } else {
                        //     _invalid_parameter(expression, function, file, line, reserved);
                        // }
                    }

                    // The handler either took care of the invalid parameter problem itself,
                    // or passed it on to another handler.  "Swallow" it by exiting, paralleling
                    // the behavior of "swallowing" exceptions.
                    std::process::exit(0);
                }
                #[cfg(target_arch = "x86_64")]
                CrashEventResult::Jump { jmp_buf, value } => (jmp_buf, value),
            }
        } else {
            _invoke_watson();
        }
    };

    #[cfg(target_arch = "x86_64")]
    unsafe {
        super::jmp::longjmp(_jump.0, _jump.1)
    };
}

/// Handler for pure virtual function calls, this is not an exception so the
/// context (shouldn't be) isn't compromised
#[unsafe(no_mangle)]
unsafe extern "C" fn handle_pure_virtual_call() {
    let _jump = unsafe {
        let lock = HANDLER.lock();
        if let Some(current_handler) = AutoHandler::new(lock) {
            // Make up an exception record for the current thread and CPU context
            // to make it possible for the crash processor to classify these
            // as do regular crashes, and to make it humane for developers to
            // analyze them.
            let exception_code = ExceptionCode::Purecall as i32;
            let mut exception_record = crash_context::EXCEPTION_RECORD {
                ExceptionCode: exception_code,
                ..std::mem::zeroed()
            };
            let mut exception_context = std::mem::MaybeUninit::zeroed();

            crash_context::capture_context(exception_context.as_mut_ptr());

            let mut exception_context = exception_context.assume_init();

            let exception_ptrs = crash_context::EXCEPTION_POINTERS {
                ExceptionRecord: &mut exception_record,
                ContextRecord: &mut exception_context,
            };

            match current_handler.user_handler.on_crash(&crate::CrashContext {
                exception_pointers: (&exception_ptrs as *const crash_context::EXCEPTION_POINTERS)
                    .cast(),
                process_id: std::process::id(),
                thread_id: GetCurrentThreadId(),
                exception_code,
            }) {
                CrashEventResult::Handled(true) => {
                    // The handler either took care of the invalid parameter problem itself,
                    // or passed it on to another handler. "Swallow" it by exiting, paralleling
                    // the behavior of "swallowing" exceptions.
                    std::process::exit(0);
                }
                CrashEventResult::Handled(false) => {
                    if let Some(pch) = current_handler.previous_pch {
                        // The handler didn't fully handle the exception.  Give it to the
                        // previous purecall handler.
                        pch();
                    }

                    // If there's no previous handler, return and let _purecall handle it.
                    // This will just throw up an assertion dialog.
                    return;
                }
                #[cfg(target_arch = "x86_64")]
                CrashEventResult::Jump { jmp_buf, value } => (jmp_buf, value),
            }
        } else {
            return;
        }
    };

    #[cfg(target_arch = "x86_64")]
    unsafe {
        super::jmp::longjmp(_jump.0, _jump.1)
    };
}
