use crate::{wasm_frame_vec_t, wasm_name_t};
use anyhow::{Error, Result, anyhow};

#[repr(C)]
pub struct wasmtime_error_t {
    error: Error,
}

wasmtime_c_api_macros::declare_own!(wasmtime_error_t);

impl From<Error> for wasmtime_error_t {
    fn from(error: Error) -> wasmtime_error_t {
        wasmtime_error_t { error }
    }
}

impl From<wasmtime_error_t> for Error {
    fn from(cerr: wasmtime_error_t) -> Error {
        cerr.error
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_error_new(
    msg: *const std::ffi::c_char,
) -> Option<Box<wasmtime_error_t>> {
    let msg_bytes = unsafe { std::ffi::CStr::from_ptr(msg).to_bytes() };
    let msg_string = String::from_utf8_lossy(msg_bytes).into_owned();
    Some(Box::new(wasmtime_error_t::from(anyhow!(msg_string))))
}

pub(crate) fn handle_result<T>(
    result: Result<T>,
    ok: impl FnOnce(T),
) -> Option<Box<wasmtime_error_t>> {
    match result {
        Ok(value) => {
            ok(value);
            None
        }
        Err(error) => Some(Box::new(wasmtime_error_t { error })),
    }
}

pub(crate) fn bad_utf8() -> Option<Box<wasmtime_error_t>> {
    Some(Box::new(wasmtime_error_t {
        error: anyhow!("input was not valid utf-8"),
    }))
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_error_message(error: &wasmtime_error_t, message: &mut wasm_name_t) {
    message.set_buffer(format!("{:?}", error.error).into_bytes());
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_error_exit_status(raw: &wasmtime_error_t, status: &mut i32) -> bool {
    #[cfg(feature = "wasi")]
    if let Some(exit) = raw.error.downcast_ref::<wasmtime_wasi::I32Exit>() {
        *status = exit.0;
        return true;
    }

    // Squash unused warnings in wasi-disabled builds.
    drop((raw, status));

    false
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_error_wasm_trace<'a>(
    raw: &'a wasmtime_error_t,
    out: &mut wasm_frame_vec_t<'a>,
) {
    crate::trap::error_trace(&raw.error, out)
}
