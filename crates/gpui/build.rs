#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]

fn main() {
    println!("cargo::rustc-check-cfg=cfg(gles)");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os == "windows" {
        #[cfg(feature = "windows-manifest")]
        embed_resource();
    }
}

#[cfg(feature = "windows-manifest")]
fn embed_resource() {
    let manifest = std::path::Path::new("resources/windows/gpui.manifest.xml");
    let rc_file = std::path::Path::new("resources/windows/gpui.rc");
    println!("cargo:rerun-if-changed={}", manifest.display());
    println!("cargo:rerun-if-changed={}", rc_file.display());
    embed_resource::compile(rc_file, embed_resource::NONE)
        .manifest_required()
        .unwrap();
}
