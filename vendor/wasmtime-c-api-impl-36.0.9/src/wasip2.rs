#[repr(transparent)]
pub struct wasmtime_wasip2_config_t {
    pub(crate) builder: wasmtime_wasi::WasiCtxBuilder,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_wasip2_config_new() -> Box<wasmtime_wasip2_config_t> {
    Box::new(wasmtime_wasip2_config_t {
        builder: wasmtime_wasi::WasiCtxBuilder::new(),
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_wasip2_config_inherit_stdin(
    config: &mut wasmtime_wasip2_config_t,
) {
    config.builder.inherit_stdin();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_wasip2_config_inherit_stdout(
    config: &mut wasmtime_wasip2_config_t,
) {
    config.builder.inherit_stdout();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_wasip2_config_inherit_stderr(
    config: &mut wasmtime_wasip2_config_t,
) {
    config.builder.inherit_stderr();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_wasip2_config_arg(
    config: &mut wasmtime_wasip2_config_t,
    arg: *const u8,
    arg_len: usize,
) {
    let arg = unsafe { std::slice::from_raw_parts(arg, arg_len) };
    let arg = std::str::from_utf8(arg).expect("valid utf-8");
    config.builder.arg(arg);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_wasip2_config_delete(_: Box<wasmtime_wasip2_config_t>) {}
