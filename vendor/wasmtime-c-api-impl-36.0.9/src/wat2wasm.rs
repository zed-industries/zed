use crate::{bad_utf8, handle_result, wasm_byte_vec_t, wasmtime_error_t};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_wat2wasm(
    wat: *const u8,
    wat_len: usize,
    ret: &mut wasm_byte_vec_t,
) -> Option<Box<wasmtime_error_t>> {
    let wat = crate::slice_from_raw_parts(wat, wat_len);
    let wat = match std::str::from_utf8(wat) {
        Ok(s) => s,
        Err(_) => return bad_utf8(),
    };
    handle_result(wat::parse_str(wat).map_err(|e| e.into()), |bytes| {
        ret.set_buffer(bytes)
    })
}
