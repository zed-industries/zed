#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]
#![cfg_attr(not(target_os = "macos"), allow(unused))]

fn main() {
    println!("cargo::rustc-check-cfg=cfg(gles)");

    #[cfg(all(target_os = "windows", feature = "windows-manifest"))]
    embed_resource();
}

#[cfg(all(target_os = "windows", feature = "windows-manifest"))]
fn embed_resource() {
    let manifest = std::path::Path::new("resources/windows/gpui.manifest.xml");
    let rc_file = std::path::Path::new("resources/windows/gpui.rc");
    println!("cargo:rerun-if-changed={}", manifest.display());
    println!("cargo:rerun-if-changed={}", rc_file.display());
    embed_resource::compile(rc_file, embed_resource::NONE)
        .manifest_required()
        .unwrap();
}
