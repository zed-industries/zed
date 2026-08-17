use alloc::{borrow::Cow, string::String};

use parking_lot::Mutex;
use windows::Win32::{Foundation, System::Diagnostics::Debug};

// This is a mutex as opposed to an atomic as we need to completely
// lock everyone out until we have registered or unregistered the
// exception handler, otherwise really nasty races could happen.
//
// By routing all the registration through these functions we can guarantee
// there is either 1 or 0 exception handlers registered, not multiple.
static EXCEPTION_HANDLER_COUNT: Mutex<usize> = Mutex::new(0);

pub fn register_exception_handler() {
    let mut count_guard = EXCEPTION_HANDLER_COUNT.lock();
    if *count_guard == 0 {
        unsafe { Debug::AddVectoredExceptionHandler(0, Some(output_debug_string_handler)) };
    }
    *count_guard += 1;
}

pub fn unregister_exception_handler() {
    let mut count_guard = EXCEPTION_HANDLER_COUNT.lock();
    if *count_guard == 1 {
        unsafe { Debug::RemoveVectoredExceptionHandler(output_debug_string_handler as *mut _) };
    }
    *count_guard -= 1;
}

const MESSAGE_PREFIXES: &[(&str, log::Level)] = &[
    ("CORRUPTION", log::Level::Error),
    ("ERROR", log::Level::Error),
    ("WARNING", log::Level::Warn),
    // We intentionally suppress "INFO" messages down to debug
    // so that users are not innundated with info messages from the runtime.
    ("INFO", log::Level::Debug),
    ("MESSAGE", log::Level::Trace),
];

unsafe extern "system" fn output_debug_string_handler(
    exception_info: *mut Debug::EXCEPTION_POINTERS,
) -> i32 {
    // See https://stackoverflow.com/a/41480827
    let record = unsafe { &*(*exception_info).ExceptionRecord };
    if record.NumberParameters != 2 {
        return Debug::EXCEPTION_CONTINUE_SEARCH;
    }
    let message = match record.ExceptionCode {
        Foundation::DBG_PRINTEXCEPTION_C => {
            String::from_utf8_lossy(bytemuck::cast_slice(&record.ExceptionInformation))
        }
        Foundation::DBG_PRINTEXCEPTION_WIDE_C => Cow::Owned(String::from_utf16_lossy(
            bytemuck::cast_slice(&record.ExceptionInformation),
        )),
        _ => return Debug::EXCEPTION_CONTINUE_SEARCH,
    };

    let message = match message.strip_prefix("D3D12 ") {
        Some(msg) => msg
            .trim_end_matches("\n\0")
            .trim_end_matches("[ STATE_CREATION WARNING #0: UNKNOWN]"),
        None => return Debug::EXCEPTION_CONTINUE_SEARCH,
    };

    let (message, level) = match MESSAGE_PREFIXES
        .iter()
        .find(|&&(prefix, _)| message.starts_with(prefix))
    {
        Some(&(prefix, level)) => (&message[prefix.len() + 2..], level),
        None => (message, log::Level::Debug),
    };

    if level == log::Level::Warn && message.contains("#82") {
        // This is are useless spammy warnings (#820, #821):
        // "The application did not pass any clear value to resource creation"
        return Debug::EXCEPTION_CONTINUE_SEARCH;
    }

    if level == log::Level::Warn && message.contains("DRAW_EMPTY_SCISSOR_RECTANGLE") {
        // This is normal, WebGPU allows passing empty scissor rectangles.
        return Debug::EXCEPTION_CONTINUE_SEARCH;
    }

    let _ = std::panic::catch_unwind(|| {
        log::log!(level, "{message}");
    });

    #[cfg(feature = "validation_canary")]
    if cfg!(debug_assertions) && level == log::Level::Error {
        use alloc::string::ToString as _;

        // Set canary and continue
        crate::VALIDATION_CANARY.add(message.to_string());
    }

    Debug::EXCEPTION_CONTINUE_EXECUTION
}
