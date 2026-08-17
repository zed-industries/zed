use crate::{wasm_frame_vec_t, wasm_instance_t, wasm_name_t, wasm_store_t};
use anyhow::{Error, anyhow};
use std::cell::OnceCell;
use wasmtime::{Trap, WasmBacktrace};

#[repr(C)]
pub struct wasm_trap_t {
    pub(crate) error: Error,
}

// This is currently only needed for the `wasm_trap_copy` API in the C API.
//
// For now the impl here is "fake it til you make it" since this is losing
// context by only cloning the error string.
impl Clone for wasm_trap_t {
    fn clone(&self) -> wasm_trap_t {
        wasm_trap_t {
            error: anyhow!("{:?}", self.error),
        }
    }
}

wasmtime_c_api_macros::declare_ref!(wasm_trap_t);

impl wasm_trap_t {
    pub(crate) fn new(error: Error) -> wasm_trap_t {
        wasm_trap_t { error }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct wasm_frame_t<'a> {
    trace: &'a WasmBacktrace,
    idx: usize,
    func_name: OnceCell<Option<wasm_name_t>>,
    module_name: OnceCell<Option<wasm_name_t>>,
}

wasmtime_c_api_macros::declare_own!(wasm_frame_t);

pub type wasm_message_t = wasm_name_t;

#[unsafe(no_mangle)]
pub extern "C" fn wasm_trap_new(
    _store: &wasm_store_t,
    message: &wasm_message_t,
) -> Box<wasm_trap_t> {
    let message = message.as_slice();
    if message[message.len() - 1] != 0 {
        panic!("wasm_trap_new message stringz expected");
    }
    let message = String::from_utf8_lossy(&message[..message.len() - 1]);
    Box::new(wasm_trap_t {
        error: Error::msg(message.into_owned()),
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_trap_new(message: *const u8, len: usize) -> Box<wasm_trap_t> {
    let bytes = crate::slice_from_raw_parts(message, len);
    let message = String::from_utf8_lossy(&bytes);
    Box::new(wasm_trap_t {
        error: Error::msg(message.into_owned()),
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_trap_new_code(code: u8) -> Box<wasm_trap_t> {
    let trap = Trap::from_u8(code).unwrap();
    Box::new(wasm_trap_t {
        error: Error::new(trap),
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_trap_message(trap: &wasm_trap_t, out: &mut wasm_message_t) {
    let mut buffer = Vec::new();
    buffer.extend_from_slice(format!("{:?}", trap.error).as_bytes());
    buffer.reserve_exact(1);
    buffer.push(0);
    out.set_buffer(buffer);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_trap_origin(raw: &wasm_trap_t) -> Option<Box<wasm_frame_t<'_>>> {
    let trace = match raw.error.downcast_ref::<WasmBacktrace>() {
        Some(trap) => trap,
        None => return None,
    };
    if trace.frames().len() > 0 {
        Some(Box::new(wasm_frame_t {
            trace,
            idx: 0,
            func_name: OnceCell::new(),
            module_name: OnceCell::new(),
        }))
    } else {
        None
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_trap_trace<'a>(raw: &'a wasm_trap_t, out: &mut wasm_frame_vec_t<'a>) {
    error_trace(&raw.error, out)
}

pub(crate) fn error_trace<'a>(error: &'a Error, out: &mut wasm_frame_vec_t<'a>) {
    let trace = match error.downcast_ref::<WasmBacktrace>() {
        Some(trap) => trap,
        None => return out.set_buffer(Vec::new()),
    };
    let vec = (0..trace.frames().len())
        .map(|idx| {
            Some(Box::new(wasm_frame_t {
                trace,
                idx,
                func_name: OnceCell::new(),
                module_name: OnceCell::new(),
            }))
        })
        .collect();
    out.set_buffer(vec);
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_trap_code(raw: &wasm_trap_t, code: &mut u8) -> bool {
    let trap = match raw.error.downcast_ref::<Trap>() {
        Some(trap) => trap,
        None => return false,
    };
    *code = *trap as u8;
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_frame_func_index(frame: &wasm_frame_t<'_>) -> u32 {
    frame.trace.frames()[frame.idx].func_index()
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_frame_func_name<'a>(
    frame: &'a wasm_frame_t<'_>,
) -> Option<&'a wasm_name_t> {
    frame
        .func_name
        .get_or_init(|| {
            frame.trace.frames()[frame.idx]
                .func_name()
                .map(|s| wasm_name_t::from(s.to_string().into_bytes()))
        })
        .as_ref()
}

#[unsafe(no_mangle)]
pub extern "C" fn wasmtime_frame_module_name<'a>(
    frame: &'a wasm_frame_t<'_>,
) -> Option<&'a wasm_name_t> {
    frame
        .module_name
        .get_or_init(|| {
            frame.trace.frames()[frame.idx]
                .module()
                .name()
                .map(|s| wasm_name_t::from(s.to_string().into_bytes()))
        })
        .as_ref()
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_frame_func_offset(frame: &wasm_frame_t<'_>) -> usize {
    frame.trace.frames()[frame.idx]
        .func_offset()
        .unwrap_or(usize::MAX)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_frame_instance(_arg1: *const wasm_frame_t<'_>) -> *mut wasm_instance_t {
    unimplemented!("wasm_frame_instance")
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_frame_module_offset(frame: &wasm_frame_t<'_>) -> usize {
    frame.trace.frames()[frame.idx]
        .module_offset()
        .unwrap_or(usize::MAX)
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_frame_copy<'a>(frame: &wasm_frame_t<'a>) -> Box<wasm_frame_t<'a>> {
    Box::new(frame.clone())
}
