//! Thin wrapper around `embed_resource::compile()`.


extern crate embed_resource;


use std::borrow::Cow;
use std::path::Path;
use std::ffi::OsStr;
use std::env;


fn main() {
    env::set_var("TARGET",
                 if cfg!(target_arch = "x86_64") {
                     "x86_64"
                 } else if cfg!(target_arch = "aarch64") {
                     "aarch64"
                 } else {
                     "irrelevant"
                 });
    #[cfg(target_os = "windows")]
    env::set_var("HOST", env::var_os("TARGET").unwrap());

    env::set_var("OUT_DIR", ".");

    let mut args = env::args_os();
    let argv0 = args.next().map(Cow::from).unwrap_or(Cow::from(OsStr::new("rust-embed-resource")));
    let resource = args.next().unwrap_or_else(|| panic!("usage: {} resource [include-dir]", Path::new(&*argv0).display()));
    let include_dir = args.next();
    embed_resource::compile(&resource, embed_resource::ParamsMacrosAndIncludeDirs(["VERSION=\"0.5.0\""], include_dir.as_ref())).manifest_required().unwrap();
    embed_resource::compile_for(&resource,
                                ["embed_resource", "embed_resource-installer"],
                                embed_resource::ParamsIncludeDirs(include_dir))
        .manifest_required()
        .unwrap();
}
